use crate::{Info, Module};
use std::fs;
use std::path::Path;

pub struct GpuDriver;

impl Module for GpuDriver {
    fn name(&self) -> &'static str {
        "gpu_driver"
    }

    fn detect(&self) -> Option<Info> {
        let value = detect_gpu_driver()?;
        Some(Info::new("GPU Driver", value))
    }
}

pub fn detect_gpu_driver() -> Option<String> {
    if let Ok(nv_ver) = fs::read_to_string("/proc/driver/nvidia/version") {
        if let Some(parsed) = parse_nvidia_proc_version(&nv_ver) {
            return Some(parsed);
        }
    }

    let drm_dir = Path::new("/sys/class/drm");
    let mut drivers = Vec::new();
    if let Ok(entries) = fs::read_dir(drm_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();
            if !name_str.starts_with("card") || name_str.contains('-') {
                continue;
            }
            let driver_link = entry.path().join("device/driver");
            if let Ok(target) = fs::read_link(&driver_link) {
                if let Some(drv) = target.file_name().and_then(|s| s.to_str()) {
                    let drv_str = drv.trim().to_string();
                    if !drv_str.is_empty() && !drivers.contains(&drv_str) {
                        drivers.push(drv_str);
                    }
                }
            }
        }
    }

    if drivers.is_empty() {
        return None;
    }

    let primary = &drivers[0];
    let mod_ver = fs::read_to_string(format!("/sys/module/{primary}/version"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let joined = drivers.join(", ");
    if let Some(ver) = mod_ver {
        Some(format!("{joined} ({ver})"))
    } else {
        Some(joined)
    }
}

pub fn parse_nvidia_proc_version(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.contains("Kernel Module") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, w) in parts.iter().enumerate() {
                if *w == "Module" {
                    if let Some(ver) = parts.get(i + 1) {
                        if ver.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                            return Some(format!("nvidia {ver}"));
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_driver_module_name() {
        assert_eq!(GpuDriver.name(), "gpu_driver");
    }

    #[test]
    fn test_parse_nvidia_proc_version() {
        let sample = "NVRM version: NVIDIA UNIX x86_64 Kernel Module  565.77  Wed Nov 27 23:33:08 UTC 2024\nGCC version:  gcc version 14.2.1\n";
        assert_eq!(
            parse_nvidia_proc_version(sample),
            Some("nvidia 565.77".to_string())
        );
    }
}
