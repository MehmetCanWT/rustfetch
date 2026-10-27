use std::ffi::CString;
use std::mem::MaybeUninit;

use crate::{Info, Module};

pub struct Disk {
    pub mount_point: Option<String>,
}

impl Default for Disk {
    fn default() -> Self {
        Self::new()
    }
}

impl Disk {
    pub const fn new() -> Self {
        Self { mount_point: None }
    }

    pub fn with_mount(mount_point: impl Into<String>) -> Self {
        Self {
            mount_point: Some(mount_point.into()),
        }
    }
}

impl Module for Disk {
    fn name(&self) -> &'static str {
        "disk"
    }

    fn detect(&self) -> Option<Info> {
        if let Some(mount) = &self.mount_point {
            let trimmed = mount.trim();
            if !trimmed.is_empty() && trimmed != "/" {
                let (total, free) = get_disk_usage(trimmed)?;
                let value = format_disk_usage(total, free)?;
                return Some(Info::new(format!("Disk ({trimmed})"), value));
            }
        }
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

/// Reads and formats disk usage for the root filesystem (`/`),
/// falling back to `/sysroot` or `/var` on immutable ostree/composefs systems (e.g. Bazzite, Silverblue)
/// where `/` is a tiny read-only composefs mount (< 1 GiB).
pub fn read_disk() -> Option<String> {
    let one_gib = 1024 * 1024 * 1024;
    let (mut total_bytes, mut free_bytes) = get_disk_usage("/")?;
    if total_bytes < one_gib {
        for fallback in ["/sysroot", "/var"] {
            if let Some((t, f)) = get_disk_usage(fallback) {
                if t >= one_gib {
                    total_bytes = t;
                    free_bytes = f;
                    break;
                }
            }
        }
    }
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

        let module = Disk::new();
        assert_eq!(module.name(), "disk");
        let detected = module.detect();
        assert!(detected.is_some());
        let info = detected.unwrap();
        assert_eq!(info.label, "Disk (/)");
        assert!(info.value.contains("GiB"));
        assert!(info.value.contains('%'));

        let custom_mount = Disk::with_mount("/tmp");
        if let Some(tmp_info) = custom_mount.detect() {
            assert_eq!(tmp_info.label, "Disk (/tmp)");
        }
    }

    #[test]
    fn test_statvfs_nonexistent_path() {
        let usage = get_disk_usage("/nonexistent_path_for_rustfetch_test_12345");
        assert_eq!(usage, None);
    }
}
