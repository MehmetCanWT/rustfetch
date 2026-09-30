use crate::{Info, Module};
use std::fs;
use std::path::Path;

pub struct Display;

impl Module for Display {
    fn name(&self) -> &'static str {
        "display"
    }

    fn detect(&self) -> Option<Info> {
        let displays = detect_displays();
        if displays.is_empty() {
            None
        } else {
            Some(Info::new("Display", displays.join(", ")))
        }
    }
}

pub fn detect_displays() -> Vec<String> {
    let mut results = Vec::new();
    let drm_path = Path::new("/sys/class/drm");
    let entries = match fs::read_dir(drm_path) {
        Ok(e) => e,
        Err(_) => return results,
    };

    for entry in entries.flatten() {
        let fname = entry.file_name();
        let fname_str = fname.to_string_lossy();
        if !fname_str.starts_with("card") || !fname_str.contains('-') {
            continue;
        }

        let p = entry.path();
        let status = fs::read_to_string(p.join("status"))
            .unwrap_or_default()
            .trim()
            .to_lowercase();
        if status != "connected" {
            continue;
        }

        let connector_name = fname_str.split_once('-').map(|x| x.1).unwrap_or(&fname_str);

        // Try reading EDID Detailed Timing Descriptor
        let mut display_spec = None;
        if let Ok(edid_bytes) = fs::read(p.join("edid")) {
            display_spec = parse_edid_resolution_and_rate(&edid_bytes);
        }

        // Fallback to modes file
        if display_spec.is_none() {
            if let Ok(modes_str) = fs::read_to_string(p.join("modes")) {
                if let Some(first_mode) = modes_str.lines().next() {
                    let trimmed = first_mode.trim();
                    if !trimmed.is_empty() {
                        display_spec = Some(trimmed.to_string());
                    }
                }
            }
        }

        if let Some(spec) = display_spec {
            results.push(format!("{spec} ({connector_name})"));
        }
    }

    results
}

pub fn parse_edid_resolution_and_rate(data: &[u8]) -> Option<String> {
    if data.len() < 72 {
        return None;
    }

    let pixel_clock = (data[54] as u32 | ((data[55] as u32) << 8)) * 10_000;
    if pixel_clock == 0 {
        return None;
    }

    let h_active = data[56] as u32 | (((data[58] as u32) & 0xF0) << 4);
    let h_blank = data[57] as u32 | (((data[58] as u32) & 0x0F) << 8);
    let v_active = data[59] as u32 | (((data[61] as u32) & 0xF0) << 4);
    let v_blank = data[60] as u32 | (((data[61] as u32) & 0x0F) << 8);

    let h_total = h_active + h_blank;
    let v_total = v_active + v_blank;

    if h_active == 0 || v_active == 0 || h_total == 0 || v_total == 0 {
        return None;
    }

    let rate = (pixel_clock as f64 / (h_total * v_total) as f64).round() as u32;
    if rate > 0 {
        Some(format!("{h_active}x{v_active} @ {rate}Hz"))
    } else {
        Some(format!("{h_active}x{v_active}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_module_name() {
        assert_eq!(Display.name(), "display");
    }

    #[test]
    fn test_parse_edid_short_data() {
        assert!(parse_edid_resolution_and_rate(&[0u8; 10]).is_none());
    }

    #[test]
    fn test_parse_edid_valid_synthetic() {
        let mut data = vec![0u8; 128];
        // pixel_clock: 148.50 MHz (14850 * 10_000)
        let clock = 14850u16;
        data[54] = (clock & 0xFF) as u8;
        data[55] = ((clock >> 8) & 0xFF) as u8;
        // 1920 h_active (0x780), h_blank: 280 (0x118), h_total: 2200
        data[56] = 0x80;
        data[57] = 0x18;
        data[58] = (0x7 << 4) | 0x1;
        // 1080 v_active (0x438), v_blank: 45 (0x2D), v_total: 1125
        data[59] = 0x38;
        data[60] = 0x2D;
        data[61] = 0x4 << 4;

        let res = parse_edid_resolution_and_rate(&data);
        assert_eq!(res, Some("1920x1080 @ 60Hz".to_string()));
    }
}
