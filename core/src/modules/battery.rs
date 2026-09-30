use crate::{Info, Module};
use std::fs;

pub struct BatteryModule;

impl Module for BatteryModule {
    fn name(&self) -> &'static str {
        "battery"
    }

    fn detect(&self) -> Option<Info> {
        let dir = fs::read_dir("/sys/class/power_supply").ok()?;
        for entry in dir.flatten() {
            let path = entry.path();
            if fs::read_to_string(path.join("type"))
                .is_ok_and(|t| t.trim().eq_ignore_ascii_case("Battery"))
            {
                let name = entry.file_name().to_string_lossy().into_owned();
                let capacity = fs::read_to_string(path.join("capacity"))
                    .ok()?
                    .trim()
                    .to_string();
                let status = fs::read_to_string(path.join("status"))
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                let manu = fs::read_to_string(path.join("manufacturer"))
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                let ac_status = match status.to_lowercase().as_str() {
                    "charging" | "full" => "AC Connected",
                    _ => &status,
                };
                let value = if manu.is_empty() {
                    format!("{capacity}% [{ac_status}]")
                } else {
                    format!("{manu} - {capacity}% [{ac_status}]")
                };

                return Some(Info::new(format!("Battery ({name})"), value));
            }
        }
        None
    }
}
