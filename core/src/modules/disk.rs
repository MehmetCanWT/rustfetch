use std::ffi::CString;
use std::mem::MaybeUninit;

use crate::{Info, Module};

pub struct Disk;

impl Module for Disk {
    fn name(&self) -> &'static str {
        "disk"
    }

    fn detect(&self) -> Option<Info> {
        let value = read_disk()?;
        Some(Info::new("Disk (/)", value))
    }
}

pub fn format_disk_usage(total_bytes: u64, free_bytes: u64) -> Option<String> {
    if total_bytes == 0 || free_bytes > total_bytes {
        return None;
    }

    let used_bytes = total_bytes - free_bytes;
    let gib = 1024.0 * 1024.0 * 1024.0;
    let used_gib = used_bytes as f64 / gib;
    let total_gib = total_bytes as f64 / gib;
    let pct = (used_bytes as f64 / total_bytes as f64 * 100.0).round() as u64;

    Some(format!("{used_gib:.1} GiB / {total_gib:.1} GiB ({pct}%)"))
}

pub fn parse_disk_usage(total_bytes: u64, free_bytes: u64) -> Option<String> {
    format_disk_usage(total_bytes, free_bytes)
}

pub fn parse_disk(total_bytes: u64, free_bytes: u64) -> Option<String> {
    format_disk_usage(total_bytes, free_bytes)
}

/// Query the filesystem using `libc::statvfs` for the given path.
///
/// Returns `Some((total_bytes, free_bytes))` on success, or `None` if `statvfs` fails
/// or if block sizes/calculations are invalid.
pub fn get_disk_usage(path: &str) -> Option<(u64, u64)> {
    let c_path = CString::new(path).ok()?;
    let mut stat = MaybeUninit::<libc::statvfs>::zeroed();

    // Call libc::statvfs safely
    let ret = unsafe { libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };
    if ret != 0 {
        return None;
    }

    let stat = unsafe { stat.assume_init() };

    let block_size = if stat.f_frsize > 0 {
        stat.f_frsize
    } else if stat.f_bsize > 0 {
        stat.f_bsize
    } else {
        return None;
    };

    let total_bytes = stat.f_blocks.checked_mul(block_size)?;
    let free_bytes = stat.f_bfree.checked_mul(block_size)?;

    Some((total_bytes, free_bytes))
}

/// Reads and formats disk usage for the root filesystem (`/`).
pub fn read_disk() -> Option<String> {
    let (total_bytes, free_bytes) = get_disk_usage("/")?;
    format_disk_usage(total_bytes, free_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_disk_usage_prompt_example() {
        // 45.2 GiB used / 237.9 GiB total (19%)
        let total_bytes = 255_443_180_052;
        let used_bytes = 48_533_130_445;
        let free_bytes = total_bytes - used_bytes;

        let formatted = format_disk_usage(total_bytes, free_bytes);
        assert_eq!(formatted, Some("45.2 GiB / 237.9 GiB (19%)".to_string()));

        // Verify parse_disk and parse_disk_usage aliases
        assert_eq!(parse_disk(total_bytes, free_bytes), formatted);
        assert_eq!(parse_disk_usage(total_bytes, free_bytes), formatted);
    }

    #[test]
    fn test_format_disk_usage_fedora_workstation() {
        // Realistic Fedora Linux installation fixture: 512 GB SSD with ~476.8 GiB usable
        // e.g. 105.4 GiB used out of 476.8 GiB (22%)
        let gib = 1024 * 1024 * 1024;
        let total_bytes = (476.8 * gib as f64).round() as u64;
        let used_bytes = (105.4 * gib as f64).round() as u64;
        let free_bytes = total_bytes - used_bytes;

        let result = format_disk_usage(total_bytes, free_bytes);
        assert_eq!(result, Some("105.4 GiB / 476.8 GiB (22%)".to_string()));
    }

    #[test]
    fn test_format_disk_usage_edge_cases() {
        // Zero total bytes should return None
        assert_eq!(format_disk_usage(0, 0), None);

        // Free bytes > total bytes should return None
        assert_eq!(format_disk_usage(100, 200), None);

        // 0% used
        let gib = 1024 * 1024 * 1024;
        assert_eq!(
            format_disk_usage(100 * gib, 100 * gib),
            Some("0.0 GiB / 100.0 GiB (0%)".to_string())
        );

        // 100% used
        assert_eq!(
            format_disk_usage(100 * gib, 0),
            Some("100.0 GiB / 100.0 GiB (100%)".to_string())
        );
    }

    #[test]
    fn test_statvfs_and_detect_live() {
        let usage = get_disk_usage("/");
        assert!(
            usage.is_some(),
            "get_disk_usage('/') should succeed on Linux"
        );
        let (total, free) = usage.unwrap();
        assert!(total > 0);
        assert!(total >= free);

        let module = Disk;
        assert_eq!(module.name(), "disk");
        let detected = module.detect();
        assert!(detected.is_some());
        let info = detected.unwrap();
        assert_eq!(info.label, "Disk (/)");
        assert!(info.value.contains("GiB"));
        assert!(info.value.contains('%'));
    }

    #[test]
    fn test_statvfs_nonexistent_path() {
        let usage = get_disk_usage("/nonexistent_path_for_rustfetch_test_12345");
        assert_eq!(usage, None);
    }
}
