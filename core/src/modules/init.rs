use crate::{Info, Module};
use std::fs;
use std::path::Path;

pub struct Init;

impl Module for Init {
    fn name(&self) -> &'static str {
        "init"
    }

    fn detect(&self) -> Option<Info> {
        let name = detect_init_name()?;
        Some(Info::new("Init", name))
    }
}

pub fn detect_init_name() -> Option<String> {
    let exe_path = fs::read_link("/proc/1/exe").ok();
    let comm = fs::read_to_string("/proc/1/comm").ok();
    parse_init(exe_path.as_deref(), comm.as_deref())
}

pub fn parse_init(exe_path: Option<&Path>, comm: Option<&str>) -> Option<String> {
    if let Some(path) = exe_path {
        let path_str = path.to_string_lossy();
        if let Some(identified) = identify_init(&path_str) {
            return Some(identified.to_string());
        }
        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            if !file_name.is_empty() && file_name != "init" && file_name != "exe" {
                return Some(file_name.to_string());
            }
        }
    }

    if let Some(c) = comm {
        let trimmed = c.trim();
        if let Some(identified) = identify_init(trimmed) {
            return Some(identified.to_string());
        }
        if !trimmed.is_empty() && trimmed != "exe" {
            return Some(trimmed.to_string());
        }
    }

    if exe_path.is_some() || comm.is_some() {
        Some("init".to_string())
    } else {
        None
    }
}

fn identify_init(s: &str) -> Option<&'static str> {
    let lower = s.to_lowercase();
    if lower.contains("systemd") {
        Some("systemd")
    } else if lower.contains("openrc") {
        Some("OpenRC")
    } else if lower.contains("runit") {
        Some("runit")
    } else if lower.contains("dinit") {
        Some("dinit")
    } else if lower.contains("s6") {
        Some("s6")
    } else if lower.contains("shepherd") {
        Some("GNU Shepherd")
    } else if lower.contains("busybox") {
        Some("BusyBox init")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_init_module_name() {
        assert_eq!(Init.name(), "init");
    }

    #[test]
    fn test_parse_init_systemd() {
        let exe = PathBuf::from("/usr/lib/systemd/systemd");
        assert_eq!(
            parse_init(Some(&exe), Some("systemd")),
            Some("systemd".to_string())
        );
    }

    #[test]
    fn test_parse_init_openrc() {
        let exe = PathBuf::from("/sbin/openrc-init");
        assert_eq!(
            parse_init(Some(&exe), Some("openrc-init")),
            Some("OpenRC".to_string())
        );
    }

    #[test]
    fn test_parse_init_runit() {
        let exe = PathBuf::from("/sbin/runit");
        assert_eq!(
            parse_init(Some(&exe), Some("runit")),
            Some("runit".to_string())
        );
    }

    #[test]
    fn test_parse_init_generic() {
        let exe = PathBuf::from("/sbin/custom_init");
        assert_eq!(
            parse_init(Some(&exe), None),
            Some("custom_init".to_string())
        );
    }

    #[test]
    fn test_parse_init_empty() {
        assert_eq!(parse_init(None, None), None);
    }
}
