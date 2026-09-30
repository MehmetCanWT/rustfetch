use crate::{Info, Module};
use std::fs;
use std::path::Path;

pub struct Temp;

impl Module for Temp {
    fn name(&self) -> &'static str {
        "temp"
    }

    fn detect(&self) -> Option<Info> {
        let temp = detect_cpu_temp()?;
        Some(Info::new("Temperature", format!("{temp}°C")))
    }
}

pub fn detect_cpu_temp() -> Option<u32> {
    detect_from_hwmon().or_else(detect_from_thermal_zones)
}

fn detect_from_hwmon() -> Option<u32> {
    let hwmon_dir = Path::new("/sys/class/hwmon");
    let entries = fs::read_dir(hwmon_dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        let name = fs::read_to_string(path.join("name"))
            .unwrap_or_default()
            .trim()
            .to_lowercase();

        if matches!(
            name.as_str(),
            "coretemp" | "k10temp" | "zenpower" | "cpu_thermal" | "soc_thermal"
        ) {
            if let Ok(dir_iter) = fs::read_dir(&path) {
                for temp_entry in dir_iter.flatten() {
                    let fname = temp_entry.file_name();
                    let fname_str = fname.to_string_lossy();
                    if fname_str.starts_with("temp") && fname_str.ends_with("_label") {
                        let label = fs::read_to_string(temp_entry.path()).unwrap_or_default();
                        let trimmed = label.trim();
                        if trimmed.starts_with("Package id")
                            || trimmed == "Tdie"
                            || trimmed == "Tctl"
                        {
                            let input_file = path.join(fname_str.replace("_label", "_input"));
                            if let Some(mdeg) = read_temp_file(&input_file) {
                                return Some(mdeg / 1000);
                            }
                        }
                    }
                }
            }

            let temp1 = path.join("temp1_input");
            if let Some(mdeg) = read_temp_file(&temp1) {
                return Some(mdeg / 1000);
            }
        }
    }
    None
}

fn detect_from_thermal_zones() -> Option<u32> {
    let thermal_dir = Path::new("/sys/class/thermal");
    let entries = fs::read_dir(thermal_dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        let tz_type = fs::read_to_string(path.join("type"))
            .unwrap_or_default()
            .trim()
            .to_lowercase();

        if matches!(
            tz_type.as_str(),
            "x86_pkg_temp" | "tcpu" | "cpu-thermal" | "soc-thermal" | "acpitz"
        ) {
            if let Some(mdeg) = read_temp_file(&path.join("temp")) {
                return Some(mdeg / 1000);
            }
        }
    }
    None
}

fn read_temp_file(path: &Path) -> Option<u32> {
    let content = fs::read_to_string(path).ok()?;
    let val: u32 = content.trim().parse().ok()?;
    if val > 0 && val < 150_000 {
        Some(val)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_module_name() {
        assert_eq!(Temp.name(), "temp");
    }

    #[test]
    fn test_read_temp_file_edge_cases() {
        assert!(read_temp_file(Path::new("/nonexistent")).is_none());
    }
}
