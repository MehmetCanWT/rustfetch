use crate::{Info, Module};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct OsAge;

impl Module for OsAge {
    fn name(&self) -> &'static str {
        "os_age"
    }

    fn detect(&self) -> Option<Info> {
        let birth_secs = detect_os_birth_timestamp()?;
        let now_secs = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        let formatted = format_os_age(birth_secs, now_secs)?;
        Some(Info::new("OS Age", formatted))
    }
}

pub fn format_os_age(birth_secs: u64, now_secs: u64) -> Option<String> {
    if birth_secs == 0 || now_secs < birth_secs {
        return None;
    }
    let elapsed = now_secs - birth_secs;
    let total_days = elapsed / 86_400;
    let hours = (elapsed % 86_400) / 3600;

    if total_days >= 365 {
        let years = total_days / 365;
        let rem_days = total_days % 365;
        let y_unit = if years == 1 { "year" } else { "years" };
        if rem_days == 0 {
            Some(format!("{years} {y_unit}"))
        } else {
            let d_unit = if rem_days == 1 { "day" } else { "days" };
            Some(format!("{years} {y_unit}, {rem_days} {d_unit}"))
        }
    } else if total_days > 0 {
        let d_unit = if total_days == 1 { "day" } else { "days" };
        if hours > 0 && total_days < 30 {
            let h_unit = if hours == 1 { "hour" } else { "hours" };
            Some(format!("{total_days} {d_unit}, {hours} {h_unit}"))
        } else {
            Some(format!("{total_days} {d_unit}"))
        }
    } else if hours > 0 {
        let h_unit = if hours == 1 { "hour" } else { "hours" };
        Some(format!("{hours} {h_unit}"))
    } else {
        Some("< 1 hour".to_string())
    }
}

fn statx_birth_time(path: &str) -> Option<u64> {
    let c_path = CString::new(path).ok()?;
    let mut stx = MaybeUninit::<libc::statx>::zeroed();
    let ret = unsafe {
        libc::statx(
            libc::AT_FDCWD,
            c_path.as_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
            libc::STATX_BTIME,
            stx.as_mut_ptr(),
        )
    };
    if ret != 0 {
        return None;
    }
    let stx = unsafe { stx.assume_init() };
    if (stx.stx_mask & libc::STATX_BTIME) != 0 && stx.stx_btime.tv_sec > 946_684_800 {
        Some(stx.stx_btime.tv_sec as u64)
    } else {
        None
    }
}

pub fn detect_os_birth_timestamp() -> Option<u64> {
    let candidates = [
        "/ostree",
        "/sysroot",
        "/",
        "/var/log/installer",
        "/etc/machine-id",
        "/etc/os-release",
    ];
    let mut oldest: Option<u64> = None;

    for path in candidates {
        if let Some(ts) = statx_birth_time(path) {
            oldest = Some(match oldest {
                Some(cur) => cur.min(ts),
                None => ts,
            });
        }
    }

    if oldest.is_some() {
        return oldest;
    }

    // Fallback to mtime of /etc/machine-id if statx btime is unsupported
    if let Ok(meta) = std::fs::metadata("/etc/machine-id") {
        if let Ok(modified) = meta.modified() {
            if let Ok(dur) = modified.duration_since(UNIX_EPOCH) {
                if dur.as_secs() > 946_684_800 {
                    return Some(dur.as_secs());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_age_module_name() {
        assert_eq!(OsAge.name(), "os_age");
    }

    #[test]
    fn test_format_os_age() {
        let base = 1_700_000_000;
        assert_eq!(format_os_age(0, base), None);
        assert_eq!(format_os_age(base + 100, base), None);
        assert_eq!(
            format_os_age(base, base + 600),
            Some("< 1 hour".to_string())
        );
        assert_eq!(
            format_os_age(base, base + 5 * 3600),
            Some("5 hours".to_string())
        );
        assert_eq!(
            format_os_age(base, base + 14 * 86_400 + 3 * 3600),
            Some("14 days, 3 hours".to_string())
        );
        assert_eq!(
            format_os_age(base, base + 120 * 86_400),
            Some("120 days".to_string())
        );
        assert_eq!(
            format_os_age(base, base + 400 * 86_400),
            Some("1 year, 35 days".to_string())
        );
    }
}
