use crate::{Info, Module};
use std::fs;
use std::process::Command;

pub struct Wifi;

impl Module for Wifi {
    fn name(&self) -> &'static str {
        "wifi"
    }

    fn detect(&self) -> Option<Info> {
        let (ssid, signal) = get_wifi_info()?;
        let val = if let Some(sig) = signal {
            format!("{ssid} ({sig}%)")
        } else {
            ssid
        };
        Some(Info::new("Wi-Fi", val))
    }
}

pub fn get_wifi_info() -> Option<(String, Option<u8>)> {
    let iface = find_wireless_interface()?;
    if let Ok(output) = Command::new("iw").args(["dev", &iface, "link"]).output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            if let Some((ssid, sig)) = parse_iw_link_output(&text) {
                return Some((ssid, sig));
            }
        }
    }

    if let Ok(output) = Command::new("iwgetid").args(["-r", &iface]).output() {
        if output.status.success() {
            let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ssid.is_empty() {
                return Some((ssid, None));
            }
        }
    }

    None
}

pub fn find_wireless_interface() -> Option<String> {
    let net_dir = fs::read_dir("/sys/class/net").ok()?;
    for entry in net_dir.flatten() {
        let p = entry.path();
        let fname = entry.file_name();
        let fname_str = fname.to_string_lossy();
        if p.join("wireless").exists() || p.join("phy80211").exists() || fname_str.starts_with("wl")
        {
            let state = fs::read_to_string(p.join("operstate")).unwrap_or_default();
            if state.trim() == "up" {
                return Some(fname_str.to_string());
            }
        }
    }

    // Fallback: any wireless interface
    let net_dir = fs::read_dir("/sys/class/net").ok()?;
    for entry in net_dir.flatten() {
        let fname = entry.file_name();
        let fname_str = fname.to_string_lossy();
        if fname_str.starts_with("wl") {
            return Some(fname_str.to_string());
        }
    }

    None
}

pub fn parse_iw_link_output(text: &str) -> Option<(String, Option<u8>)> {
    if !text.contains("Connected to") {
        return None;
    }

    let mut ssid = None;
    let mut signal = None;

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("SSID:") {
            let s = rest.trim();
            if !s.is_empty() {
                ssid = Some(s.to_string());
            }
        } else if let Some(rest) = trimmed.strip_prefix("signal:") {
            if let Some(dbm_str) = rest.split_whitespace().next() {
                if let Ok(dbm) = dbm_str.parse::<i32>() {
                    // Standard dBm to percentage mapping: -50 dBm is 100%, -100 dBm is 0%
                    let pct = (2 * (dbm + 100)).clamp(0, 100) as u8;
                    signal = Some(pct);
                }
            }
        }
    }

    ssid.map(|s| (s, signal))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wifi_module_name() {
        assert_eq!(Wifi.name(), "wifi");
    }

    #[test]
    fn test_parse_iw_link_output() {
        let sample = "Connected to 3c:64:cf:72:29:e6 (on wlp0s20f3)\n\tSSID: MyNetwork 5G\n\tsignal: -45 dBm\n";
        let res = parse_iw_link_output(sample);
        assert_eq!(res, Some(("MyNetwork 5G".to_string(), Some(100))));
    }

    #[test]
    fn test_parse_iw_link_disconnected() {
        assert_eq!(parse_iw_link_output("Not connected."), None);
    }
}
