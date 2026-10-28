use crate::{cache::Cache, Info, Module};
use std::process::Command;
use std::time::Duration;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CAT_BREEDS: &[&str] = &[
    "Turkish Van",
    "Turkish Angora",
    "Bengal",
    "Maine Coon",
    "Siamese",
    "Ragdoll",
    "Persian",
    "Birman",
    "Chartreux",
    "Korat",
    "Abyssinian",
    "British Shorthair",
    "Scottish Fold",
    "Russian Blue",
    "Norwegian Forest",
    "Siberian",
    "Savannah",
    "Ocicat",
    "Somali",
    "Sphynx",
    "Bombay",
    "Burmese",
    "Manx",
    "Anatolian",
];

/// Returns the official cat breed codename for a given semantic version.
pub fn codename_for_version(version: &str) -> &'static str {
    let Some((major, minor, patch)) = parse_semver(version) else {
        return "Bengal";
    };
    match (major, minor, patch) {
        (0, 1, 0) => "Turkish Van",
        (0, 1, 1) => "Turkish Angora",
        (0, 1, 2) => "Bengal",
        (0, 1, 3) => "Maine Coon",
        (0, 1, 4) => "Siamese",
        (0, 1, 5) => "Ragdoll",
        (0, 1, 6) => "Persian",
        (0, 1, 7) => "Birman",
        (0, 1, 8) => "Chartreux",
        (0, 1, 9) => "Korat",
        (0, 2, 0) => "Abyssinian",
        (0, 2, 1) => "British Shorthair",
        (0, 2, 2) => "Scottish Fold",
        (0, 2, 3) => "Russian Blue",
        (0, 2, 4) => "Norwegian Forest",
        (0, 2, 5) => "Siberian",
        (0, 3, 0) => "Savannah",
        (0, 3, 1) => "Ocicat",
        (0, 3, 2) => "Somali",
        (0, 4, 0) => "Sphynx",
        (1, 0, 0) => "Anatolian",
        _ => {
            let idx =
                (major as usize * 31 + minor as usize * 10 + patch as usize) % CAT_BREEDS.len();
            CAT_BREEDS[idx]
        }
    }
}

/// Formats a version string with its cat breed codename, e.g. `v0.1.2 "Bengal"`.
pub fn format_version_with_codename(version: &str) -> String {
    let clean = version.trim().trim_start_matches(['v', 'V']);
    let codename = codename_for_version(clean);
    format!("v{clean} \"{codename}\"")
}

const GITHUB_CARGO_TOML_URL: &str =
    "https://raw.githubusercontent.com/MehmetCanWT/rustfetch/main/Cargo.toml";
const GITHUB_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/MehmetCanWT/rustfetch/releases/latest";
const UPDATE_CACHE_TTL: Duration = Duration::from_secs(6 * 3600);
const UPDATE_CHECK_THROTTLE: Duration = Duration::from_secs(3600);

pub struct Update;

impl Module for Update {
    fn name(&self) -> &'static str {
        "update"
    }

    fn detect(&self) -> Option<Info> {
        if std::env::var("RUSTFETCH_DISABLE_UPDATE_CHECK")
            .is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        {
            return None;
        }

        let current = CURRENT_VERSION.trim().trim_start_matches(['v', 'V']);

        if let Ok(forced_latest) = std::env::var("RUSTFETCH_LATEST_VERSION") {
            let (latest_raw, remote_code) = split_cached_version_codename(&forced_latest);
            let latest = latest_raw.trim().trim_start_matches(['v', 'V']);
            if is_newer_version(current, latest) {
                return Some(Info::new(
                    "Update",
                    format_update_message_with_codename(current, latest, remote_code),
                ));
            }
            return None;
        }

        let cache = Cache::new("update");
        // Fast path: use fresh cached version (< 6 hours).
        let cached_entry = if let Some(cached) = cache.read(UPDATE_CACHE_TTL) {
            Some(cached.trim().to_string())
        } else {
            // Cache is expired or missing. Never block interactive fetch!
            // Trigger asynchronous background update check (detached process).
            trigger_background_update_check();
            // Fall back to stale cache if available, so existing update notification is not lost
            cache.read_stale().map(|s| s.trim().to_string())
        };

        let cached_entry = cached_entry.unwrap_or_else(|| current.to_string());
        let (latest_raw, remote_codename) = split_cached_version_codename(&cached_entry);
        let latest_clean = latest_raw.trim().trim_start_matches(['v', 'V']);
        if is_newer_version(current, latest_clean) {
            Some(Info::new(
                "Update",
                format_update_message_with_codename(current, latest_clean, remote_codename),
            ))
        } else {
            None
        }
    }
}

/// Checks whether the system has an active default network gateway routing route.
/// Takes ~5 microseconds (pure /proc/net/route read, 0 network packets).
pub fn has_default_gateway() -> bool {
    let Ok(routes) = std::fs::read_to_string("/proc/net/route") else {
        return true; // Fall back to true if sysfs is inaccessible
    };
    for line in routes.lines().skip(1) {
        let mut fields = line.split_whitespace();
        let _iface = fields.next();
        let dest = fields.next();
        let _gateway = fields.next();
        let flags = fields.next();
        if let (Some(dest), Some(flags)) = (dest, flags) {
            if dest == "00000000" {
                if let Ok(f) = u32::from_str_radix(flags, 16) {
                    if f & 0x0001 != 0 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Spawns a detached background process to refresh the update cache without blocking the shell.
pub fn trigger_background_update_check() {
    let throttle_cache = Cache::new("update_throttle");
    if throttle_cache.read(UPDATE_CHECK_THROTTLE).is_some() {
        return;
    }
    throttle_cache.write("1");

    if !has_default_gateway() {
        return;
    }

    if let Ok(exe) = std::env::current_exe() {
        let _ = Command::new(exe)
            .arg("--check-updates-bg")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }
}

/// Executed in the background process to fetch the latest version and update the disk cache.
pub fn refresh_update_cache() {
    if !has_default_gateway() {
        return;
    }
    let cache = Cache::new("update");
    if let Some(fetched) = fetch_latest_github_version() {
        cache.write(&fetched);
    } else {
        let fallback = cache.read_stale().unwrap_or_else(|| {
            CURRENT_VERSION
                .trim()
                .trim_start_matches(['v', 'V'])
                .to_string()
        });
        cache.write(&fallback);
    }
}

pub fn format_update_message(current: &str, latest: &str) -> String {
    let (latest_ver, remote_code) = split_cached_version_codename(latest);
    format_update_message_with_codename(current, latest_ver, remote_code)
}

pub fn format_update_message_with_codename(
    current: &str,
    latest: &str,
    remote_codename: Option<&str>,
) -> String {
    let cur = current.trim().trim_start_matches(['v', 'V']);
    let lat = latest.trim().trim_start_matches(['v', 'V']);
    let lat_code = remote_codename
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| codename_for_version(lat));
    format!("Update available (v{cur} -> v{lat} \"{lat_code}\")")
}

fn split_cached_version_codename(entry: &str) -> (&str, Option<&str>) {
    if let Some((ver, code)) = entry.split_once('|') {
        let code = code.trim();
        (ver.trim(), (!code.is_empty()).then_some(code))
    } else {
        (entry.trim(), None)
    }
}

/// Extracts a cat breed codename from a GitHub Release title if present,
/// e.g. `RustFetch v0.2.0 — "Abyssinian"` -> `Some("Abyssinian")`.
pub fn extract_codename_from_release_title(title: &str) -> Option<String> {
    if let Some(start) = title.find('"') {
        let rest = &title[start + 1..];
        if let Some(end) = rest.find('"') {
            let candidate = rest[..end].trim();
            if !candidate.is_empty() {
                return Some(candidate.to_string());
            }
        }
    }
    for sep in ["—", "–", " - "] {
        if let Some((_, after)) = title.split_once(sep) {
            let candidate = after.trim().trim_matches(['"', '\'', '(', ')']);
            if !candidate.is_empty() && parse_semver(candidate).is_none() {
                return Some(candidate.to_string());
            }
        }
    }
    None
}

pub fn parse_semver(version: &str) -> Option<(u64, u64, u64)> {
    let cleaned = version.trim().trim_start_matches(['v', 'V']);
    let core_part = cleaned.split(['-', '+']).next()?;
    let mut parts = core_part.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next().unwrap_or("0").parse::<u64>().ok()?;
    let patch = parts.next().unwrap_or("0").parse::<u64>().ok()?;
    Some((major, minor, patch))
}

pub fn is_newer_version(current: &str, latest: &str) -> bool {
    let Some(cur) = parse_semver(current) else {
        return false;
    };
    let Some(lat) = parse_semver(latest) else {
        return false;
    };
    lat > cur
}

pub fn parse_cargo_toml_version(content: &str) -> Option<String> {
    let mut in_pkg_section = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_pkg_section = trimmed == "[workspace.package]" || trimmed == "[package]";
            continue;
        }
        if in_pkg_section {
            if let Some((key, val)) = trimmed.split_once('=') {
                if key.trim() == "version" {
                    let v = val
                        .split('#')
                        .next()
                        .unwrap_or(val)
                        .trim()
                        .trim_matches('"')
                        .trim_matches('\'');
                    if parse_semver(v).is_some() {
                        return Some(v.to_string());
                    }
                }
            }
        }
    }
    None
}

pub fn parse_github_release_info(json_str: &str) -> Option<(String, Option<String>)> {
    let val: serde_json::Value = serde_json::from_str(json_str).ok()?;
    let tag = val.get("tag_name")?.as_str()?.trim();
    let cleaned = tag.trim_start_matches(['v', 'V']);
    if parse_semver(cleaned).is_some() {
        let title_codename = val
            .get("name")
            .and_then(|n| n.as_str())
            .and_then(extract_codename_from_release_title);
        Some((cleaned.to_string(), title_codename))
    } else {
        None
    }
}

fn fetch_url(url: &str) -> Option<String> {
    if !has_default_gateway() {
        return None;
    }

    match Command::new("curl")
        .args([
            "-fsSL",
            "--connect-timeout",
            "2",
            "--max-time",
            "4",
            "-H",
            "User-Agent: rustfetch",
            url,
        ])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout).into_owned();
                if !text.trim().is_empty() {
                    return Some(text);
                }
            }
            None
        }
        Err(_) => {
            // Only try wget if curl binary was not found
            if let Ok(output) = Command::new("wget")
                .args(["-qO-", "--timeout=3", "--tries=1", url])
                .output()
            {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout).into_owned();
                    if !text.trim().is_empty() {
                        return Some(text);
                    }
                }
            }
            None
        }
    }
}

pub fn check_latest_release() -> Option<(String, String)> {
    if let Ok(forced) = std::env::var("RUSTFETCH_LATEST_VERSION") {
        let (v, c) = split_cached_version_codename(&forced);
        let clean = v.trim().trim_start_matches(['v', 'V']);
        if parse_semver(clean).is_some() {
            let code = c
                .map(str::to_string)
                .unwrap_or_else(|| codename_for_version(clean).to_string());
            return Some((clean.to_string(), code));
        }
    }
    let raw = fetch_latest_github_version()?;
    let (v, c) = split_cached_version_codename(&raw);
    let clean = v.trim().trim_start_matches(['v', 'V']);
    let code = c
        .map(str::to_string)
        .unwrap_or_else(|| codename_for_version(clean).to_string());
    Some((clean.to_string(), code))
}

pub fn fetch_latest_github_version() -> Option<String> {
    if let Some((r, r_code)) =
        fetch_url(GITHUB_LATEST_RELEASE_URL).and_then(|body| parse_github_release_info(&body))
    {
        return Some(match r_code {
            Some(code) => format!("{r}|{code}"),
            None => r,
        });
    }
    fetch_url(GITHUB_CARGO_TOML_URL).and_then(|body| parse_cargo_toml_version(&body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_module_name() {
        assert_eq!(Update.name(), "update");
    }

    #[test]
    fn test_parse_semver_and_is_newer() {
        assert_eq!(parse_semver("0.1.2"), Some((0, 1, 2)));
        assert_eq!(parse_semver("v1.2.3-beta.1"), Some((1, 2, 3)));
        assert_eq!(parse_semver("invalid"), None);

        assert!(is_newer_version("0.1.2", "0.1.3"));
        assert!(is_newer_version("0.1.2", "v0.2.0"));
        assert!(is_newer_version("0.1.2", "1.0.0"));
        assert!(!is_newer_version("0.1.2", "0.1.2"));
        assert!(!is_newer_version("0.1.2", "0.1.1"));
    }

    #[test]
    fn test_parse_cargo_toml_version() {
        let sample = r#"
[workspace]
members = ["core", "render", "cli"]

[workspace.package]
version = "0.2.1"
edition = "2021"
"#;
        assert_eq!(parse_cargo_toml_version(sample), Some("0.2.1".to_string()));
    }

    #[test]
    fn test_parse_github_release_info() {
        let json = r#"{"tag_name": "v0.3.0", "name": "RustFetch v0.3.0 — \"Van Kedisi\""}"#;
        assert_eq!(
            parse_github_release_info(json),
            Some(("0.3.0".to_string(), Some("Van Kedisi".to_string())))
        );
    }

    #[test]
    fn test_format_update_message() {
        assert_eq!(
            format_update_message("0.1.2", "v0.2.0"),
            "Update available (v0.1.2 -> v0.2.0 \"Abyssinian\")"
        );
        assert_eq!(
            format_update_message("0.1.2", "0.2.0|Tekir"),
            "Update available (v0.1.2 -> v0.2.0 \"Tekir\")"
        );
        assert_eq!(codename_for_version("0.1.0"), "Turkish Van");
        assert_eq!(codename_for_version("0.1.1"), "Turkish Angora");
        assert_eq!(codename_for_version("0.1.2"), "Bengal");
        assert_eq!(codename_for_version("0.1.3"), "Maine Coon");
    }
}
