use crate::{Info, Module};
use std::fs;
use std::process::Command;

pub struct Wifi;

impl Module for Wifi {
    fn name(&self) -> &'static str {
        "wifi"
    }

    fn is_slow(&self) -> bool {
        true
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
        .args([
            "-t",
            "-f",
            "active,ssid,signal,freq",
            "dev",
            "wifi",
            "list",
            "--rescan",
            "no",
        ])
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
    let mut fallback = None;
    for entry in fs::read_dir("/sys/class/net").ok()?.flatten() {
        let p = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if p.join("wireless").exists() || p.join("phy80211").exists() || name.starts_with("wl") {
            if fs::read_to_string(p.join("operstate"))
                .map(|s| s.trim() == "up")
                .unwrap_or(false)
            {
                return Some(name);
            }
            if fallback.is_none() {
                fallback = Some(name);
            }
        }
    }
    fallback
}

fn freq_to_band(freq: f64) -> Option<String> {
    if freq >= 5925.0 {
        Some("6 GHz".to_string())
    } else if freq >= 4900.0 {
        Some("5 GHz".to_string())
    } else if freq >= 2400.0 {
        Some("2.4 GHz".to_string())
    } else {
        None
    }
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
            if let Some(dbm) = rest
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<i32>().ok())
            {
                // Standard dBm to percentage mapping: -50 dBm is 100%, -100 dBm is 0%
                signal_pct = Some((2 * (dbm + 100)).clamp(0, 100) as u8);
                dbm_val = Some(dbm);
            }
        } else if let Some(rest) = trimmed.strip_prefix("freq:") {
            band = rest
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<f64>().ok())
                .and_then(freq_to_band);
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
        let Some((active_raw, rest)) = line.split_once(':') else {
            continue;
        };
        let Some((rest, freq_raw)) = rest.rsplit_once(':') else {
            continue;
        };
        let Some((ssid_raw, sig_raw)) = rest.rsplit_once(':') else {
            continue;
        };
        let active = active_raw.trim().to_lowercase();
        if matches!(active.as_str(), "yes" | "evet" | "ja" | "true") {
            let ssid = ssid_raw.trim().replace("\\:", ":");
            if ssid.is_empty() {
                continue;
            }
            let signal_pct = sig_raw.trim().parse::<u8>().ok();
            let band = freq_raw
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<f64>().ok())
                .and_then(freq_to_band);
            return Some(WifiDetails {
                ssid,
                signal_pct,
                dbm: None,
                band,
            });
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
        assert_eq!(parse_iw_link_details("Not connected."), None);
    }

    #[test]
    fn test_parse_nmcli_dev_wifi() {
        let sample = "hayır:OtherNet:60:2437 MHz\nevet:Home\\:Fiber:95:5200 MHz\n";
        let details = parse_nmcli_dev_wifi(sample).unwrap();
        assert_eq!(details.ssid, "Home:Fiber");
        assert_eq!(details.signal_pct, Some(95));
        assert_eq!(details.band, Some("5 GHz".to_string()));
    }
}
