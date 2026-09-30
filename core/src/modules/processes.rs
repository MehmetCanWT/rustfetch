use crate::{Info, Module};
use std::fs;

pub struct Processes;

impl Module for Processes {
    fn name(&self) -> &'static str {
        "processes"
    }

    fn detect(&self) -> Option<Info> {
        let count = detect_process_count()?;
        Some(Info::new("Processes", count))
    }
}

pub fn detect_process_count() -> Option<String> {
    if let Ok(loadavg) = fs::read_to_string("/proc/loadavg") {
        if let Some(parsed) = parse_loadavg_processes(&loadavg) {
            return Some(parsed);
        }
    }

    // Fallback: count PID directories in /proc
    if let Ok(entries) = fs::read_dir("/proc") {
        let pids = entries
            .flatten()
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .chars()
                    .all(|c| c.is_ascii_digit())
            })
            .count();
        if pids > 0 {
            return Some(pids.to_string());
        }
    }

    None
}

pub fn parse_loadavg_processes(content: &str) -> Option<String> {
    // 4.26 3.21 2.92 11/1294 31653
    let mut parts = content.split_whitespace();
    let slash_field = parts.nth(3)?;
    let (running, total) = slash_field.split_once('/')?;
    let r: u32 = running.parse().ok()?;
    let t: u32 = total.parse().ok()?;
    if t > 0 {
        Some(format!("{t} (running: {r})"))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processes_module_name() {
        assert_eq!(Processes.name(), "processes");
    }

    #[test]
    fn test_parse_loadavg_processes() {
        let val = parse_loadavg_processes("0.52 0.58 0.59 2/450 12345");
        assert_eq!(val, Some("450 (running: 2)".to_string()));
    }

    #[test]
    fn test_parse_loadavg_invalid() {
        assert_eq!(parse_loadavg_processes("invalid loadavg string"), None);
    }
}
