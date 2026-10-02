use crate::{cache::Cache, Info, Module};
use std::fs;
use std::process::Command;
use std::time::Duration;

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str {
        "packages"
    }

    fn detect(&self) -> Option<Info> {
        let cache = Cache::new("packages");
        if let Some(cached_val) = cache.read(Duration::from_secs(2 * 3600)) {
            return Some(Info::new("Packages", cached_val));
        }

        let mut packages = Vec::new();

        let flatpak_count = fs::read_dir("/var/lib/flatpak/app")
            .map(|d| d.count())
            .unwrap_or(0)
            + std::env::var("HOME")
                .ok()
                .and_then(|h| fs::read_dir(format!("{h}/.local/share/flatpak/app")).ok())
                .map(|d| d.count())
                .unwrap_or(0);
        if flatpak_count > 0 {
            packages.push(format!("{flatpak_count} (flatpak)"));
        }

        if std::path::Path::new("/var/lib/rpm").exists()
            || std::path::Path::new("/usr/bin/rpm").exists()
        {
            if let Ok(output) = Command::new("rpm").arg("-qa").output() {
                let count = count_nonempty_lines(&String::from_utf8_lossy(&output.stdout));
                if output.status.success() && count > 0 {
                    packages.push(format!("{count} (rpm)"));
                }
            }
        }

        let dpkg_count = fs::read_dir("/var/lib/dpkg/info")
            .map(|d| {
                d.flatten()
                    .filter(|e| e.file_name().to_string_lossy().ends_with(".list"))
                    .count()
            })
            .unwrap_or(0);
        if dpkg_count > 0 {
            packages.push(format!("{dpkg_count} (dpkg)"));
        }

        let pacman_count = fs::read_dir("/var/lib/pacman/local")
            .map(|d| {
                d.flatten()
                    .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
                    .count()
            })
            .unwrap_or(0);
        if pacman_count > 0 {
            packages.push(format!("{pacman_count} (pacman)"));
        }

        if let Ok(apk_str) = fs::read_to_string("/lib/apk/db/installed") {
            let count = count_apk_packages(&apk_str);
            if count > 0 {
                packages.push(format!("{count} (apk)"));
            }
        }

        if let Ok(xbps_str) = fs::read_to_string("/var/db/xbps/pkgdb-0.38.plist") {
            let count = count_xbps_packages(&xbps_str);
            if count > 0 {
                packages.push(format!("{count} (xbps)"));
            }
        }

        let emerge_count = fs::read_dir("/var/db/pkg")
            .map(|d| {
                d.flatten()
                    .filter_map(|cat| fs::read_dir(cat.path()).ok())
                    .map(|entries| entries.count())
                    .sum::<usize>()
            })
            .unwrap_or(0);
        if emerge_count > 0 {
            packages.push(format!("{emerge_count} (emerge)"));
        }

        let nix_count = count_nix_packages();
        if nix_count > 0 {
            packages.push(format!("{nix_count} (nix)"));
        }

        let snap_count = fs::read_dir("/var/lib/snapd/snaps")
            .map(|d| {
                d.flatten()
                    .filter(|e| e.file_name().to_string_lossy().ends_with(".snap"))
                    .count()
            })
            .unwrap_or(0);
        if snap_count > 0 {
            packages.push(format!("{snap_count} (snap)"));
        }

        if packages.is_empty() {
            return None;
        }

        let val = packages.join(", ");
        cache.write(&val);

        Some(Info::new("Packages", val))
    }
}

pub fn count_apk_packages(content: &str) -> usize {
    content
        .lines()
        .filter(|line| line.starts_with("P:"))
        .count()
}

pub fn count_xbps_packages(content: &str) -> usize {
    content.matches("<key>pkgver</key>").count()
}

pub fn count_nonempty_lines(content: &str) -> usize {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

pub fn count_nix_packages() -> usize {
    let mut count = 0;
    if let Ok(entries) = fs::read_dir("/run/current-system/sw/bin") {
        count += entries.count();
    }
    if let Ok(home) = std::env::var("HOME") {
        if let Ok(entries) = fs::read_dir(format!("{home}/.nix-profile/bin")) {
            count += entries.count();
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apk_packages() {
        let sample = "C:Q1xyz\nP:musl\nV:1.2.4\nP:busybox\nV:1.36.1\n";
        assert_eq!(count_apk_packages(sample), 2);
        assert_eq!(count_apk_packages(""), 0);
    }

    #[test]
    fn test_count_xbps_packages() {
        let sample = r#"
        <dict>
            <key>pkgver</key>
            <string>bash-5.2.026_1</string>
            <key>pkgver</key>
            <string>coreutils-9.4_1</string>
        </dict>
        "#;
        assert_eq!(count_xbps_packages(sample), 2);
        assert_eq!(count_xbps_packages("<dict></dict>"), 0);
    }

    #[test]
    fn test_packages_module_name() {
        assert_eq!(PackagesModule.name(), "packages");
    }

    #[test]
    fn test_count_nonempty_lines() {
        assert_eq!(count_nonempty_lines("package-a\n\n package-b \n"), 2);
    }
}
