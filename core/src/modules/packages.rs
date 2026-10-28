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

        let mut rpm_counted = false;
        if let Some(count) = count_rpm_sqlite() {
            if count > 0 {
                packages.push(format!("{count} (rpm)"));
            }
            rpm_counted = true;
        }

        if !rpm_counted {
            if let Some(count) = count_rpm_fallback() {
                if count > 0 {
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

        let mut brew_paths = vec![std::path::PathBuf::from(
            "/home/linuxbrew/.linuxbrew/Cellar",
        )];
        if let Ok(home) = std::env::var("HOME") {
            brew_paths.push(std::path::PathBuf::from(format!(
                "{home}/.linuxbrew/Cellar"
            )));
        }
        let brew_count = count_brew_cellars(&brew_paths);
        if brew_count > 0 {
            packages.push(format!("{brew_count} (brew)"));
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

pub fn count_brew_cellars(paths: &[std::path::PathBuf]) -> usize {
    let mut seen = Vec::new();
    let mut total = 0;
    for path in paths {
        let Ok(canon) = fs::canonicalize(path) else {
            continue;
        };
        if seen.contains(&canon) {
            continue;
        }
        seen.push(canon.clone());
        if let Ok(entries) = fs::read_dir(&canon) {
            total += entries
                .flatten()
                .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
                .count();
        }
    }
    total
}

fn count_rpm_sqlite() -> Option<usize> {
    const CANDIDATES: &[&str] = &[
        "/usr/share/rpm/rpmdb.sqlite",
        "/var/lib/rpm/rpmdb.sqlite",
        "/usr/lib/sysimage/rpm/rpmdb.sqlite",
    ];

    let db_path = CANDIDATES
        .iter()
        .map(std::path::Path::new)
        .find(|p| p.is_file())?;

    let c_path = std::ffi::CString::new(db_path.to_str()?).ok()?;

    type Sqlite3OpenV2 = unsafe extern "C" fn(
        filename: *const libc::c_char,
        pp_db: *mut *mut libc::c_void,
        flags: libc::c_int,
        z_vfs: *const libc::c_char,
    ) -> libc::c_int;
    type Sqlite3PrepareV2 = unsafe extern "C" fn(
        db: *mut libc::c_void,
        z_sql: *const libc::c_char,
        n_byte: libc::c_int,
        pp_stmt: *mut *mut libc::c_void,
        pz_tail: *mut *const libc::c_char,
    ) -> libc::c_int;
    type Sqlite3Step = unsafe extern "C" fn(p_stmt: *mut libc::c_void) -> libc::c_int;
    type Sqlite3ColumnInt64 =
        unsafe extern "C" fn(p_stmt: *mut libc::c_void, i_col: libc::c_int) -> i64;
    type Sqlite3Finalize = unsafe extern "C" fn(p_stmt: *mut libc::c_void) -> libc::c_int;
    type Sqlite3Close = unsafe extern "C" fn(p_db: *mut libc::c_void) -> libc::c_int;

    const SQLITE_OK: libc::c_int = 0;
    const SQLITE_ROW: libc::c_int = 100;
    const SQLITE_OPEN_READONLY: libc::c_int = 0x00000001;

    let lib = unsafe {
        let lib0 = libc::dlopen(c"libsqlite3.so.0".as_ptr(), libc::RTLD_LAZY);
        if !lib0.is_null() {
            lib0
        } else {
            libc::dlopen(c"libsqlite3.so".as_ptr(), libc::RTLD_LAZY)
        }
    };
    if lib.is_null() {
        return None;
    }

    unsafe {
        let sym_open = libc::dlsym(lib, c"sqlite3_open_v2".as_ptr());
        let sym_prep = libc::dlsym(lib, c"sqlite3_prepare_v2".as_ptr());
        let sym_step = libc::dlsym(lib, c"sqlite3_step".as_ptr());
        let sym_col = libc::dlsym(lib, c"sqlite3_column_int64".as_ptr());
        let sym_fin = libc::dlsym(lib, c"sqlite3_finalize".as_ptr());
        let sym_close = libc::dlsym(lib, c"sqlite3_close".as_ptr());

        if sym_open.is_null()
            || sym_prep.is_null()
            || sym_step.is_null()
            || sym_col.is_null()
            || sym_fin.is_null()
            || sym_close.is_null()
        {
            libc::dlclose(lib);
            return None;
        }

        let fn_open: Sqlite3OpenV2 = std::mem::transmute(sym_open);
        let fn_prep: Sqlite3PrepareV2 = std::mem::transmute(sym_prep);
        let fn_step: Sqlite3Step = std::mem::transmute(sym_step);
        let fn_col: Sqlite3ColumnInt64 = std::mem::transmute(sym_col);
        let fn_fin: Sqlite3Finalize = std::mem::transmute(sym_fin);
        let fn_close: Sqlite3Close = std::mem::transmute(sym_close);

        let mut db: *mut libc::c_void = std::ptr::null_mut();

        if fn_open(
            c_path.as_ptr(),
            &mut db,
            SQLITE_OPEN_READONLY,
            std::ptr::null(),
        ) != SQLITE_OK
            || db.is_null()
        {
            if !db.is_null() {
                fn_close(db);
            }
            libc::dlclose(lib);
            return None;
        }

        let mut count: Option<usize> = None;
        for sql in [
            c"SELECT count(*) FROM Packages".as_ptr(),
            c"SELECT count(*) FROM Sigmd5".as_ptr(),
        ] {
            let mut stmt: *mut libc::c_void = std::ptr::null_mut();
            if fn_prep(db, sql, -1, &mut stmt, std::ptr::null_mut()) == SQLITE_OK && !stmt.is_null()
            {
                if fn_step(stmt) == SQLITE_ROW {
                    let n = fn_col(stmt, 0);
                    if n >= 0 {
                        count = Some(n as usize);
                    }
                }
                fn_fin(stmt);
            }
            if count.is_some() {
                break;
            }
        }

        fn_close(db);
        libc::dlclose(lib);
        count
    }
}

pub fn count_rpm_from_cache_or_exec<F>(
    rpmdb_path: Option<&std::path::Path>,
    cache: &Cache,
    exec_rpm: F,
) -> Option<usize>
where
    F: FnOnce() -> Option<usize>,
{
    let mtime_stamp = rpmdb_path.and_then(|p| {
        fs::metadata(p)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(std::time::SystemTime::UNIX_EPOCH).ok())
            .map(|d| format!("{}:{}", d.as_secs(), d.subsec_nanos()))
    });

    if let (Some(ref stamp), Some(cached)) = (&mtime_stamp, cache.read_stale()) {
        if let Some((cached_stamp, count_str)) = cached.trim().split_once(' ') {
            if cached_stamp == stamp {
                if let Ok(count) = count_str.parse::<usize>() {
                    return Some(count);
                }
            }
        }
    }

    let count = exec_rpm()?;
    if let Some(ref stamp) = mtime_stamp {
        cache.write(&format!("{stamp} {count}"));
    }
    Some(count)
}

pub fn count_rpm_fallback() -> Option<usize> {
    const CANDIDATES: &[&str] = &[
        "/var/lib/rpm/rpmdb.sqlite",
        "/usr/share/rpm/rpmdb.sqlite",
        "/usr/lib/sysimage/rpm/rpmdb.sqlite",
        "/var/lib/rpm/Packages",
        "/var/lib/rpm",
    ];

    let rpmdb_path = CANDIDATES
        .iter()
        .map(std::path::Path::new)
        .find(|p| p.exists());

    let cache = Cache::new("rpm_cache");

    count_rpm_from_cache_or_exec(rpmdb_path, &cache, || {
        if std::path::Path::new("/var/lib/rpm").exists()
            || std::path::Path::new("/usr/bin/rpm").exists()
        {
            if let Ok(output) = Command::new("rpm").arg("-qa").output() {
                let count = count_nonempty_lines(&String::from_utf8_lossy(&output.stdout));
                if output.status.success() && count > 0 {
                    return Some(count);
                }
            }
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_brew_cellars_deduplicates_symlinks() {
        let tmp = std::env::temp_dir().join(format!("rustfetch_brew_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let cellar = tmp.join("Cellar");
        fs::create_dir_all(cellar.join("ripgrep")).unwrap();
        fs::create_dir_all(cellar.join("fd")).unwrap();
        fs::write(cellar.join("README"), b"ignore").unwrap();

        let alias = tmp.join("CellarAlias");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&cellar, &alias).unwrap();

        let count = count_brew_cellars(&[cellar.clone(), alias, cellar]);
        let _ = fs::remove_dir_all(&tmp);
        assert_eq!(count, 2);
    }

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

    #[test]
    fn test_count_rpm_sqlite() {
        if std::path::Path::new("/usr/share/rpm/rpmdb.sqlite").exists()
            || std::path::Path::new("/var/lib/rpm/rpmdb.sqlite").exists()
        {
            let count = count_rpm_sqlite();
            assert!(count.is_some());
            assert!(count.unwrap() > 0);
        }
    }

    #[test]
    fn test_rpm_fallback_cache_with_mtime() {
        let tmp =
            std::env::temp_dir().join(format!("rustfetch_rpm_cache_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let fake_rpmdb = tmp.join("rpmdb.sqlite");
        fs::write(&fake_rpmdb, b"fake-db").unwrap();

        let cache_file = tmp.join("rpm_cache");
        let cache = Cache::for_path(cache_file);

        let mut exec_count = 0;
        let count1 = count_rpm_from_cache_or_exec(Some(&fake_rpmdb), &cache, || {
            exec_count += 1;
            Some(42)
        });
        assert_eq!(count1, Some(42));
        assert_eq!(exec_count, 1);

        // Second call with same mtime should read from cache without executing closure
        let count2 = count_rpm_from_cache_or_exec(Some(&fake_rpmdb), &cache, || {
            exec_count += 1;
            Some(999)
        });
        assert_eq!(count2, Some(42));
        assert_eq!(exec_count, 1);

        // Third call after updating mtime should invalidate cache and re-execute closure
        std::thread::sleep(std::time::Duration::from_millis(50));
        fs::write(&fake_rpmdb, b"updated-fake-db").unwrap();
        let count3 = count_rpm_from_cache_or_exec(Some(&fake_rpmdb), &cache, || {
            exec_count += 1;
            Some(100)
        });
        assert_eq!(count3, Some(100));
        assert_eq!(exec_count, 2);

        let _ = fs::remove_dir_all(&tmp);
    }
}
