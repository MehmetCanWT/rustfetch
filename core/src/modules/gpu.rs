use crate::{Info, Module};
use std::fs;

pub struct GpuModule;

impl Module for GpuModule {
    fn name(&self) -> &'static str {
        "gpu"
    }

    fn detect(&self) -> Option<Info> {
        let mut pci_devices = Vec::new();
        if let Ok(dir) = fs::read_dir("/sys/bus/pci/devices") {
            for entry in dir.flatten() {
                let path = entry.path();
                let class = fs::read_to_string(path.join("class")).unwrap_or_default();
                let class = class.trim();
                if class.starts_with("0x0300")
                    || class.starts_with("0x0380")
                    || class.starts_with("0x0302")
                {
                    let vendor = fs::read_to_string(path.join("vendor")).unwrap_or_default();
                    let device = fs::read_to_string(path.join("device")).unwrap_or_default();
                    let v = vendor.trim().trim_start_matches("0x");
                    let d = device.trim().trim_start_matches("0x");
                    if !v.is_empty() && !d.is_empty() {
                        pci_devices.push((v.to_string(), d.to_string()));
                    }
                }
            }
        }

        let mut gpus = Vec::new();
        if !pci_devices.is_empty() {
            let pci_content = [
                "/usr/share/hwdata/pci.ids",
                "/usr/share/misc/pci.ids",
                "/usr/local/share/hwdata/pci.ids",
            ]
            .iter()
            .find_map(|p| fs::read_to_string(p).ok())
            .unwrap_or_default();

            for (vendor, device) in pci_devices {
                if let Some(name) = lookup_pci_in_content(&pci_content, &vendor, &device) {
                    gpus.push(name);
                }
            }
        }

        if gpus.is_empty() {
            if let Ok(output) = std::process::Command::new("lspci").output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.contains("VGA compatible controller") || line.contains("3D controller")
                    {
                        if let Some((_, name)) = line.split_once(": ") {
                            let clean = name.split(" [").next().unwrap_or(name).trim();
                            gpus.push(clean.to_string());
                        }
                    }
                }
            }
        }

        if gpus.is_empty() {
            return None;
        }

        Some(Info::new("GPU", gpus.join(", ")))
    }
}

fn lookup_pci_in_content(content: &str, vendor_id: &str, device_id: &str) -> Option<String> {
    if content.is_empty() {
        return None;
    }
    let mut current_vendor = String::new();
    let mut in_vendor = false;

    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if !line.starts_with('\t') {
            if line.len() >= 4 && line[..4].eq_ignore_ascii_case(vendor_id) {
                in_vendor = true;
                current_vendor = line[4..]
                    .replace(" Corporation", "")
                    .replace(" Advanced Micro Devices, Inc. [AMD/ATI]", "AMD")
                    .replace(" Advanced Micro Devices, Inc. [AMD]", "AMD")
                    .trim()
                    .to_string();
            } else {
                in_vendor = false;
            }
        } else if in_vendor && !line.starts_with("\t\t") {
            let line = &line[1..];
            if line.len() >= 4 && line[..4].eq_ignore_ascii_case(device_id) {
                let dev_name = line[4..].trim();
                let dev_clean = if let (Some(s), Some(e)) = (dev_name.find('['), dev_name.find(']'))
                {
                    if s < e {
                        &dev_name[s + 1..e]
                    } else {
                        dev_name
                    }
                } else {
                    dev_name
                };
                return Some(format!("{current_vendor} {dev_clean}"));
            }
        }
    }
    None
}
