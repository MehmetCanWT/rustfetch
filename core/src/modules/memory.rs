use crate::{Info, Module};

pub struct Memory;

impl Module for Memory {
    fn name(&self) -> &'static str {
        "memory"
    }

    fn detect(&self) -> Option<Info> {
        let content = read_meminfo()?;
        let value = parse_meminfo(&content)?;
        Some(Info::new("Memory", value))
    }
}

fn read_meminfo() -> Option<String> {
    std::fs::read_to_string("/proc/meminfo").ok()
}

pub fn format_kb(kb: u64) -> String {
    const KIB_PER_MIB: f64 = 1024.0;
    const KIB_PER_GIB: f64 = 1024.0 * 1024.0;

    let kb_f = kb as f64;
    if kb_f >= KIB_PER_GIB {
        format!("{:.1} GiB", kb_f / KIB_PER_GIB)
    } else {
        format!("{:.1} MiB", kb_f / KIB_PER_MIB)
    }
}

fn parse_kb_line(rest: &str) -> Option<u64> {
    rest.split_whitespace().next()?.parse().ok()
}

pub fn parse_meminfo(content: &str) -> Option<String> {
    let mut mem_total = None;
    let mut mem_available = None;

    for line in content.lines() {
        if let Some((key, rest)) = line.split_once(':') {
            match key.trim() {
                "MemTotal" => mem_total = parse_kb_line(rest),
                "MemAvailable" => mem_available = parse_kb_line(rest),
                _ => {}
            }
            if mem_total.is_some() && mem_available.is_some() {
                break;
            }
        }
    }

    let total = mem_total?;
    let available = mem_available?;
    let mut used = total.checked_sub(available)?;

    if let Ok(arc_content) = std::fs::read_to_string("/proc/spl/kstat/zfs/arcstats") {
        for line in arc_content.lines() {
            let mut parts = line.split_whitespace();
            if parts.next() == Some("size") {
                if let Some(arc_kb) = parts
                    .nth(1)
                    .and_then(|v| v.parse::<u64>().ok())
                    .map(|b| b / 1024)
                {
                    used = used.saturating_sub(arc_kb);
                }
            }
        }
    }

    Some(format!("{} / {}", format_kb(used), format_kb(total)))
}

pub fn parse_memory(content: &str) -> Option<String> {
    parse_meminfo(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    /// Realistic Fedora Linux /proc/meminfo fixture.
    /// MemTotal: 16252928 kB (~15.5 GiB)
    /// MemAvailable: 12897485 kB (used = 3355443 kB, ~3.2 GiB)
    const FEDORA_MEMINFO: &str = r#"MemTotal:       16252928 kB
MemFree:         2092500 kB
MemAvailable:   12897485 kB
Buffers:            4184 kB
Cached:          6128348 kB
SwapCached:            0 kB
Active:          6605884 kB
Inactive:        5555404 kB
Active(anon):    6605884 kB
Inactive(anon):        0 kB
Active(file):          0 kB
Inactive(file):  5555404 kB
Unevictable:      485868 kB
Mlocked:             136 kB
SwapTotal:       8388604 kB
SwapFree:        8388604 kB
Zswap:                 0 kB
Zswapped:              0 kB
Dirty:             13244 kB
Writeback:             0 kB
AnonPages:       6515228 kB
Mapped:          1167996 kB
Shmem:            577396 kB
KReclaimable:     477664 kB
Slab:             486212 kB
SReclaimable:     229556 kB
SUnreclaim:       256656 kB
KernelStack:       18080 kB
PageTables:        60304 kB
SecPageTables:      2056 kB
"#;

    #[test]
    fn test_parse_meminfo_fedora() {
        let result = parse_meminfo(FEDORA_MEMINFO);
        assert_snapshot!(result.unwrap(), @"3.2 GiB / 15.5 GiB");
    }

    #[test]
    fn test_format_kb() {
        assert_eq!(format_kb(1048576), "1.0 GiB");
        assert_eq!(format_kb(16252928), "15.5 GiB");
        assert_eq!(format_kb(3355443), "3.2 GiB");
        assert_eq!(format_kb(524288), "512.0 MiB");
        assert_eq!(format_kb(1024), "1.0 MiB");
        assert_eq!(format_kb(0), "0.0 MiB");
    }

    #[test]
    fn test_parse_meminfo_empty_and_invalid() {
        assert_eq!(parse_meminfo(""), None);
        assert_eq!(parse_meminfo("MemTotal: 16252928 kB"), None);
        assert_eq!(parse_meminfo("MemAvailable: 12897485 kB"), None);
        // MemAvailable > MemTotal should safely return None without panicking
        let invalid = "MemTotal: 1000 kB\nMemAvailable: 2000 kB\n";
        assert_eq!(parse_meminfo(invalid), None);
    }

    #[test]
    fn test_memory_module_name() {
        let mem = Memory;
        assert_eq!(mem.name(), "memory");
    }

    #[test]
    fn test_memory_detect() {
        let mem = Memory;
        if let Some(info) = mem.detect() {
            assert_eq!(info.label, "Memory");
            assert!(info.value.contains(" / "));
        }
    }
}
