use crate::{Info, Module};
use std::fs;
use std::path::Path;

pub struct BrightnessModule;

impl Module for BrightnessModule {
    fn name(&self) -> &'static str {
        "brightness"
    }

    fn detect(&self) -> Option<Info> {
        let dir = fs::read_dir("/sys/class/backlight").ok()?;
        for entry in dir.flatten() {
            let path = entry.path();
            if let Some(info) = detect_from_backlight_dir(&path) {
                return Some(info);
            }
        }
        None
    }
}

fn detect_from_backlight_dir(path: &Path) -> Option<Info> {
    let cur: f64 = fs::read_to_string(path.join("brightness"))
        .ok()?
        .trim()
        .parse()
        .ok()?;
    let max: f64 = fs::read_to_string(path.join("max_brightness"))
        .ok()?
        .trim()
        .parse()
        .ok()?;

    if max <= 0.0 {
        return None;
    }

    let pct = ((cur / max) * 100.0).round() as u32;
    Some(Info::new("Brightness", format!("{pct}%")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness_module_name() {
        assert_eq!(BrightnessModule.name(), "brightness");
    }

    #[test]
    fn test_detect_from_backlight_dir_valid() {
        let temp = std::env::temp_dir().join("rustfetch_test_brightness_valid");
        let _ = fs::remove_dir_all(&temp);
        let _ = fs::create_dir_all(&temp);

        fs::write(temp.join("brightness"), "14400\n").unwrap();
        fs::write(temp.join("max_brightness"), "19200\n").unwrap();

        let info = detect_from_backlight_dir(&temp).unwrap();
        assert_eq!(info.label, "Brightness");
        assert_eq!(info.value, "75%");

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_detect_from_backlight_dir_zero_max() {
        let temp = std::env::temp_dir().join("rustfetch_test_brightness_zero");
        let _ = fs::remove_dir_all(&temp);
        let _ = fs::create_dir_all(&temp);

        fs::write(temp.join("brightness"), "0\n").unwrap();
        fs::write(temp.join("max_brightness"), "0\n").unwrap();

        assert!(detect_from_backlight_dir(&temp).is_none());

        let _ = fs::remove_dir_all(&temp);
    }
}
