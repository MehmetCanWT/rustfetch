use crate::{Info, Module};
use std::fs;
use std::process::Command;

pub struct BluetoothModule;

impl Module for BluetoothModule {
    fn name(&self) -> &'static str {
        "bluetooth"
    }

    fn detect(&self) -> Option<Info> {
        let mut devices = Vec::new();

        // 1. Try bluetoothctl
        if let Ok(output) = Command::new("bluetoothctl")
            .args(["devices", "Connected"])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Some((mac, name)) = parse_bluetoothctl_device_line(line) {
                        let battery = get_device_battery(&mac);
                        if let Some(pct) = battery {
                            devices.push(format!("{name} ({pct}%)"));
                        } else {
                            devices.push(name);
                        }
                    }
                }
            }
        }

        // 2. Try sysfs power_supply for peripheral Bluetooth/wireless devices (mice, keyboards, controllers)
        if let Ok(dir) = fs::read_dir("/sys/class/power_supply") {
            for entry in dir.flatten() {
                let path = entry.path();
                let fname = entry.file_name().to_string_lossy().into_owned();
                if fname.starts_with("hid-") || fname.starts_with("hidpp") {
                    let model = fs::read_to_string(path.join("model_name"))
                        .ok()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .unwrap_or(fname);
                    let cap = fs::read_to_string(path.join("capacity"))
                        .ok()
                        .and_then(|c| c.trim().parse::<u8>().ok());
                    let desc = if let Some(pct) = cap {
                        format!("{model} ({pct}%)")
                    } else {
                        model
                    };
                    if !devices.contains(&desc) {
                        devices.push(desc);
                    }
                }
            }
        }

        if devices.is_empty() {
            None
        } else {
            Some(Info::new("Bluetooth", devices.join(", ")))
        }
    }
}

pub fn parse_bluetoothctl_device_line(line: &str) -> Option<(String, String)> {
    // Format: "Device 70:26:05:11:22:33 Sony WH-1000XM5"
    let trimmed = line.trim();
    if !trimmed.starts_with("Device ") {
        return None;
    }
    let rest = trimmed.strip_prefix("Device ")?.trim();
    let (mac, name) = rest.split_once(' ')?;
    let mac = mac.trim().to_string();
    let name = name.trim().to_string();
    if mac.is_empty() || name.is_empty() {
        return None;
    }
    Some((mac, name))
}

pub fn parse_bluetoothctl_info_battery(output: &str) -> Option<u8> {
    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Battery Percentage:") {
            let rest = rest.trim();
            // Could be "0x50 (80)" or "80%" or "80"
            if let Some(open) = rest.find('(') {
                if let Some(close) = rest.find(')') {
                    let inner = &rest[open + 1..close];
                    if let Ok(pct) = inner.trim().parse::<u8>() {
                        return Some(pct.min(100));
                    }
                }
            }
            let cleaned = rest.trim_end_matches('%').trim();
            if let Ok(pct) = cleaned.parse::<u8>() {
                return Some(pct.min(100));
            }
        }
    }
    None
}

fn get_device_battery(mac: &str) -> Option<u8> {
    let output = Command::new("bluetoothctl")
        .args(["info", mac])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    parse_bluetoothctl_info_battery(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_module_name() {
        assert_eq!(BluetoothModule.name(), "bluetooth");
    }

    #[test]
    fn test_parse_bluetoothctl_device_line() {
        let line = "Device 70:26:05:AA:BB:CC Sony WH-1000XM5";
        let (mac, name) = parse_bluetoothctl_device_line(line).unwrap();
        assert_eq!(mac, "70:26:05:AA:BB:CC");
        assert_eq!(name, "Sony WH-1000XM5");
    }

    #[test]
    fn test_parse_bluetoothctl_info_battery_parens() {
        let info = "Device 70:26:05:AA:BB:CC\n\tName: Earbuds\n\tConnected: yes\n\tBattery Percentage: 0x55 (85)\n";
        assert_eq!(parse_bluetoothctl_info_battery(info), Some(85));
    }

    #[test]
    fn test_parse_bluetoothctl_info_battery_percent() {
        let info = "Device 70:26:05:AA:BB:CC\n\tBattery Percentage: 90%\n";
        assert_eq!(parse_bluetoothctl_info_battery(info), Some(90));
    }

    #[test]
    fn test_parse_bluetoothctl_info_no_battery() {
        let info = "Device 70:26:05:AA:BB:CC\n\tName: Keyboard\n\tConnected: yes\n";
        assert_eq!(parse_bluetoothctl_info_battery(info), None);
    }
}
