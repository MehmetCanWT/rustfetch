use crate::{Info, Module};
use std::fs;
use std::process::Command;

pub struct Wifi;

impl Module for Wifi {
    fn name(&self) -> &'static str {
        "wifi"
    }

    fn detect(&self) -> Option<Info> {
        let details = get_wifi_details()?;
        let mut parts = Vec::new();
        if let Some(dbm) = details.dbm {
            if let Some(sig) = details.signal_pct {
                parts.push(format!("{dbm} dBm, {sig}%"));
            } else {
                parts.push(format!("{dbm} dBm"));
            }
        } else if let Some(sig) = details.signal_pct {
            parts.push(format!("{sig}%"));
        }

        let sig_str = if parts.is_empty() {
            String::new()
        } else {
            format!(" ({})", parts.join(", "))
        };

        let band_str = details.band.map(|b| format!(" [{b}]")).unwrap_or_default();

        let val = format!("{}{sig_str}{band_str}", details.ssid);
        Some(Info::new("Wi-Fi", val))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WifiDetails {
    pub ssid: String,
    pub signal_pct: Option<u8>,
    pub dbm: Option<i32>,
    pub band: Option<String>,
}

pub fn get_wifi_info() -> Option<(String, Option<u8>)> {
    let details = get_wifi_details()?;
    Some((details.ssid, details.signal_pct))
}

pub fn get_wifi_details() -> Option<WifiDetails> {
    let iface = find_wireless_interface()?;
    if let Ok(output) = Command::new("iw").args(["dev", &iface, "link"]).output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            if let Some(details) = parse_iw_link_details(&text) {
                return Some(details);
            }
        }
    }

    if let Ok(output) = Command::new("nmcli")
        .args(["-t", "-f", "active,ssid,signal,freq", "dev", "wifi"])
        .output()
    {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            if let Some(details) = parse_nmcli_dev_wifi(&text) {
                return Some(details);
            }
        }
    }

    if let Ok(output) = Command::new("iwgetid").args(["-r", &iface]).output() {
        if output.status.success() {
            let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ssid.is_empty() {
                return Some(WifiDetails {
                    ssid,
                    signal_pct: None,
                    dbm: None,
                    band: None,
                });
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
    let details = parse_iw_link_details(text)?;
    Some((details.ssid, details.signal_pct))
}

pub fn parse_iw_link_details(text: &str) -> Option<WifiDetails> {
    if !text.contains("Connected to") {
        return None;
    }

    let mut ssid = None;
    let mut signal_pct = None;
    let mut dbm_val = None;
    let mut band = None;

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
                    signal_pct = Some(pct);
                    dbm_val = Some(dbm);
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("freq:") {
            if let Some(freq_str) = rest.split_whitespace().next() {
                if let Ok(freq) = freq_str.parse::<f64>() {
                    if freq >= 5925.0 {
                        band = Some("6 GHz".to_string());
                    } else if freq >= 4900.0 {
                        band = Some("5 GHz".to_string());
                    } else if freq >= 2400.0 {
                        band = Some("2.4 GHz".to_string());
                    }
                }
            }
        }
    }

    ssid.map(|s| WifiDetails {
        ssid: s,
        signal_pct,
        dbm: dbm_val,
        band,
    })
}

pub fn parse_nmcli_dev_wifi(text: &str) -> Option<WifiDetails> {
    for line in text.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 4 {
            let active = parts[0].trim().to_lowercase();
            // yes, evet, ja, oui, etc. or non-empty/non-no
            if active == "yes" || active == "evet" || active == "ja" || active == "true" {
                let ssid = parts[1].trim().to_string();
                if ssid.is_empty() {
                    continue;
                }
                let signal_pct = parts[2].trim().parse::<u8>().ok();
                let freq_num = parts[3]
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<f64>().ok());
                let band = freq_num.map(|f| {
                    if f >= 5925.0 {
                        "6 GHz".to_string()
                    } else if f >= 4900.0 {
                        "5 GHz".to_string()
                    } else {
                        "2.4 GHz".to_string()
                    }
                });
                return Some(WifiDetails {
                    ssid,
                    signal_pct,
                    dbm: None,
                    band,
                });
            }
        }
    }
    None
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
    fn test_parse_iw_link_details_with_freq() {
        let sample = "Connected to 3c:64:cf:72:29:e6 (on wlp0s20f3)\n\tSSID: MyNetwork 5G\n\tfreq: 5200.0\n\tsignal: -48 dBm\n";
        let details = parse_iw_link_details(sample).unwrap();
        assert_eq!(details.ssid, "MyNetwork 5G");
        assert_eq!(details.signal_pct, Some(100)); // (-48 + 100) * 2 = 104 -> clamp to 100
        assert_eq!(details.dbm, Some(-48));
        assert_eq!(details.band, Some("5 GHz".to_string()));
    }

    #[test]
    fn test_parse_iw_link_disconnected() {
        assert_eq!(parse_iw_link_output("Not connected."), None);
        assert_eq!(parse_iw_link_details("Not connected."), None);
    }

    #[test]
    fn test_parse_nmcli_dev_wifi() {
        let sample = "hayır:OtherNet:60:2437 MHz\nevet:HomeFiber:95:5200 MHz\n";
        let details = parse_nmcli_dev_wifi(sample).unwrap();
        assert_eq!(details.ssid, "HomeFiber");
        assert_eq!(details.signal_pct, Some(95));
        assert_eq!(details.band, Some("5 GHz".to_string()));
    }
}
