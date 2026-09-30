pub use crate::modules::memory::format_kb;
use crate::{Info, Module};

pub struct Swap;

impl Module for Swap {
    fn name(&self) -> &'static str {
        "swap"
    }

    fn detect(&self) -> Option<Info> {
        let content = std::fs::read_to_string("/proc/meminfo").ok()?;
        let value = parse_swap(&content)?;
        Some(Info::new("Swap", value))
    }
}

pub fn parse_swap(content: &str) -> Option<String> {
    let mut swap_total: Option<u64> = None;
    let mut swap_free: Option<u64> = None;

    for line in content.lines() {
        if let Some((key, rest)) = line.split_once(':') {
            match key.trim() {
                "SwapTotal" => swap_total = rest.split_whitespace().next()?.parse().ok(),
                "SwapFree" => swap_free = rest.split_whitespace().next()?.parse().ok(),
                _ => {}
            }
            if swap_total.is_some() && swap_free.is_some() {
                break;
            }
        }
    }

    let total = swap_total?;
    if total == 0 {
        return None;
    }

    let free = swap_free?;
    let used = total.checked_sub(free)?;

    Some(format!("{} / {}", format_kb(used), format_kb(total)))
}

/// Alias for [`parse_swap`].
pub fn parse_meminfo(content: &str) -> Option<String> {
    parse_swap(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Realistic Fedora Linux /proc/meminfo fixture with active swap.
    /// SwapTotal: 8388608 kB (~8.0 GiB)
    /// SwapFree:  7340032 kB (used = 1048576 kB, ~1.0 GiB)
    const FEDORA_SWAP_MEMINFO: &str = r#"MemTotal:       16133420 kB
MemFree:         2092500 kB
MemAvailable:    7780664 kB
Buffers:            4184 kB
Cached:          6128348 kB
SwapCached:            0 kB
Active:          6605884 kB
Inactive:        5555404 kB
SwapTotal:       8388608 kB
SwapFree:        7340032 kB
Zswap:                 0 kB
Zswapped:              0 kB
Dirty:             13244 kB
Writeback:             0 kB
AnonPages:       6515228 kB
Mapped:          1167996 kB
Shmem:            577396 kB
"#;

    /// Realistic Fedora Linux /proc/meminfo fixture with no swap configured.
    const FEDORA_NO_SWAP_MEMINFO: &str = r#"MemTotal:       16133420 kB
MemFree:         2092500 kB
MemAvailable:    7780664 kB
SwapTotal:             0 kB
SwapFree:              0 kB
"#;

    #[test]
    fn test_parse_swap_configured() {
        let result = parse_swap(FEDORA_SWAP_MEMINFO);
        assert_eq!(result.as_deref(), Some("1.0 GiB / 8.0 GiB"));
    }

    #[test]
    fn test_parse_swap_zero_returns_none() {
        let result = parse_swap(FEDORA_NO_SWAP_MEMINFO);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_swap_empty_and_invalid() {
        assert_eq!(parse_swap(""), None);
        assert_eq!(parse_swap("SwapTotal: 8388608 kB"), None); // missing SwapFree
        assert_eq!(parse_swap("SwapFree: 7340032 kB"), None); // missing SwapTotal
                                                              // SwapFree > SwapTotal safely returns None without panicking
        let invalid = "SwapTotal: 1000 kB\nSwapFree: 2000 kB\n";
        assert_eq!(parse_swap(invalid), None);
    }

    #[test]
    fn test_swap_module_name() {
        let swap = Swap;
        assert_eq!(swap.name(), "swap");
    }

    #[test]
    fn test_swap_detect() {
        let swap = Swap;
        if let Some(info) = swap.detect() {
            assert_eq!(info.label, "Swap");
            assert!(info.value.contains(" / "));
        }
    }
}
