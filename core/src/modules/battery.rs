use crate::{Info, Module};
use std::fs;
use std::path::Path;

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
                return detect_from_dir(&path, &name);
            }
        }
        None
    }
}

fn detect_from_dir(path: &Path, name: &str) -> Option<Info> {
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

    let cycles = fs::read_to_string(path.join("cycle_count"))
        .ok()
        .and_then(|c| c.trim().parse::<u32>().ok())
        .filter(|&c| c > 0);

    let full = fs::read_to_string(path.join("charge_full"))
        .or_else(|_| fs::read_to_string(path.join("energy_full")))
        .ok()
        .and_then(|v| v.trim().parse::<f64>().ok());
    let design = fs::read_to_string(path.join("charge_full_design"))
        .or_else(|_| fs::read_to_string(path.join("energy_full_design")))
        .ok()
        .and_then(|v| v.trim().parse::<f64>().ok());
    let health = match (full, design) {
        (Some(f), Some(d)) if d > 0.0 => {
            let h = (f / d * 100.0).round() as u32;
            Some(h.min(100))
        }
        _ => None,
    };

    let mut extra = Vec::new();
    if let Some(h) = health {
        extra.push(format!("Health: {h}%"));
    }
    if let Some(c) = cycles {
        extra.push(format!("{c} cycles"));
    }

    let extra_str = if extra.is_empty() {
        String::new()
    } else {
        format!(" ({})", extra.join(", "))
    };

    let value = if manu.is_empty() {
        format!("{capacity}% [{ac_status}]{extra_str}")
    } else {
        format!("{manu} - {capacity}% [{ac_status}]{extra_str}")
    };

    Some(Info::new(format!("Battery ({name})"), value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_module_name() {
        assert_eq!(BatteryModule.name(), "battery");
    }

    #[test]
    fn test_detect_from_dir_full() {
        let temp = std::env::temp_dir().join("rustfetch_test_battery_full");
        let _ = fs::remove_dir_all(&temp);
        let _ = fs::create_dir_all(&temp);

        fs::write(temp.join("capacity"), "95\n").unwrap();
        fs::write(temp.join("status"), "Full\n").unwrap();
        fs::write(temp.join("manufacturer"), "SMP\n").unwrap();
        fs::write(temp.join("cycle_count"), "120\n").unwrap();
        fs::write(temp.join("charge_full"), "4000\n").unwrap();
        fs::write(temp.join("charge_full_design"), "5000\n").unwrap();

        let info = detect_from_dir(&temp, "BAT0").unwrap();
        assert_eq!(info.label, "Battery (BAT0)");
        assert_eq!(
            info.value,
            "SMP - 95% [AC Connected] (Health: 80%, 120 cycles)"
        );

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_detect_from_dir_no_extra() {
        let temp = std::env::temp_dir().join("rustfetch_test_battery_simple");
        let _ = fs::remove_dir_all(&temp);
        let _ = fs::create_dir_all(&temp);

        fs::write(temp.join("capacity"), "42\n").unwrap();
        fs::write(temp.join("status"), "Discharging\n").unwrap();

        let info = detect_from_dir(&temp, "BAT1").unwrap();
        assert_eq!(info.label, "Battery (BAT1)");
        assert_eq!(info.value, "42% [Discharging]");

        let _ = fs::remove_dir_all(&temp);
    }
}
