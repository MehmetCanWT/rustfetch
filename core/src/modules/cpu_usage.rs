use crate::{Info, Module};
use std::fs;
use std::time::Duration;

pub struct CpuUsage;

impl Module for CpuUsage {
    fn name(&self) -> &'static str {
        "cpu_usage"
    }

    fn detect(&self) -> Option<Info> {
        let (t1, i1) = read_proc_stat()?;
        std::thread::sleep(Duration::from_millis(50));
        let (t2, i2) = read_proc_stat()?;

        let total_delta = t2.saturating_sub(t1);
        let idle_delta = i2.saturating_sub(i1);

        if total_delta == 0 {
            return None;
        }

        let used = total_delta.saturating_sub(idle_delta);
        let usage = ((used as f64 / total_delta as f64) * 100.0).round() as u32;
        let usage = usage.min(100);

        Some(Info::new("CPU Usage", format!("{usage}%")))
    }
}

pub fn parse_proc_stat_line(line: &str) -> Option<(u64, u64)> {
    if !line.starts_with("cpu ") {
        return None;
    }
    let vals: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();
    if vals.len() < 4 {
        return None;
    }
    let idle = vals[3] + vals.get(4).unwrap_or(&0); // idle + iowait
    let total: u64 = vals.iter().sum();
    Some((total, idle))
}

fn read_proc_stat() -> Option<(u64, u64)> {
    let stat = fs::read_to_string("/proc/stat").ok()?;
    let first_line = stat.lines().next()?;
    parse_proc_stat_line(first_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_usage_name() {
        assert_eq!(CpuUsage.name(), "cpu_usage");
    }

    #[test]
    fn test_parse_proc_stat_line() {
        let line = "cpu  2255 34 2290 22625563 6290 127 456 0 0 0";
        let (total, idle) = parse_proc_stat_line(line).unwrap();
        assert_eq!(idle, 22625563 + 6290);
        assert!(total > idle);
    }

    #[test]
    fn test_parse_proc_stat_invalid() {
        assert!(parse_proc_stat_line("cpu0 100 200").is_none());
        assert!(parse_proc_stat_line("intr 12345").is_none());
        assert!(parse_proc_stat_line("").is_none());
    }
}
