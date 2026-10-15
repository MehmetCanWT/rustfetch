use crate::{Info, Module};
use std::fs;
use std::path::Path;

pub struct Container;

impl Module for Container {
    fn name(&self) -> &'static str {
        "container"
    }

    fn detect(&self) -> Option<Info> {
        let name = detect_container()?;
        Some(Info::new("Container", name))
    }
}

pub fn detect_container() -> Option<String> {
    // 1. Direct container marker files
    if Path::new("/.dockerenv").exists() {
        return Some("Docker".to_string());
    }

    if let Ok(content) = fs::read_to_string("/run/systemd/container") {
        let trimmed = content.trim();
        if !trimmed.is_empty() {
            return Some(format_runtime(trimmed));
        }
    }

    // 2. Flatpak / Snap sandboxes
    if Path::new("/.flatpak-info").exists() || std::env::var("FLATPAK_ID").is_ok() {
        return Some("Flatpak".to_string());
    }
    if std::env::var("SNAP").is_ok() {
        return Some("Snap".to_string());
    }

    // 3. WSL2 detection via kernel version string
    if let Ok(ver) = fs::read_to_string("/proc/version") {
        let lower = ver.to_lowercase();
        if lower.contains("microsoft") || lower.contains("wsl") {
            return Some("WSL2".to_string());
        }
    }

    // 4. PID 1 environment container variable
    if let Ok(environ) = fs::read("/proc/1/environ") {
        for entry in environ.split(|&b| b == 0) {
            if let Ok(s) = std::str::from_utf8(entry) {
                if let Some(val) = s.strip_prefix("container=") {
                    let val = val.trim();
                    if !val.is_empty() {
                        return Some(format_runtime(val));
                    }
                }
            }
        }
    }

    // 5. Virtual machine inspection via DMI
    let product = fs::read_to_string("/sys/class/dmi/id/product_name").unwrap_or_default();
    let vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor").unwrap_or_default();
    let combined = format!("{product} {vendor}").to_lowercase();

    if combined.contains("kvm") {
        Some("KVM".to_string())
    } else if combined.contains("qemu") {
        Some("QEMU".to_string())
    } else if combined.contains("virtualbox") || combined.contains("innotek") {
        Some("VirtualBox".to_string())
    } else if combined.contains("vmware") {
        Some("VMware".to_string())
    } else if combined.contains("bochs") {
        Some("Bochs".to_string())
    } else {
        None
    }
}

fn format_runtime(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "docker" => "Docker".to_string(),
        "podman" => "Podman".to_string(),
        "lxc" => "LXC".to_string(),
        "systemd-nspawn" | "nspawn" => "systemd-nspawn".to_string(),
        "kubevirt" => "KubeVirt".to_string(),
        other => {
            let mut chars = other.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_module_name() {
        assert_eq!(Container.name(), "container");
    }

    #[test]
    fn test_format_runtime() {
        assert_eq!(format_runtime("docker"), "Docker");
        assert_eq!(format_runtime("podman"), "Podman");
        assert_eq!(format_runtime("lxc"), "LXC");
        assert_eq!(format_runtime("nspawn"), "systemd-nspawn");
        assert_eq!(format_runtime("custom"), "Custom");
    }
}
