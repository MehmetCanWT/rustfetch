use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct Cache {
    dir: PathBuf,
}

impl Cache {
    pub fn cache_dir() -> PathBuf {
        if let Ok(xdg) = std::env::var("XDG_CACHE_HOME") {
            let trimmed = xdg.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed).join("rustfetch");
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            let trimmed = home.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed).join(".cache").join("rustfetch");
            }
        }
        std::env::temp_dir().join("rustfetch-cache")
    }

    pub fn clear_all() -> usize {
        let dir = Self::cache_dir();
        let Ok(entries) = fs::read_dir(&dir) else {
            return 0;
        };
        let mut removed = 0;
        for entry in entries.flatten() {
            if fs::remove_file(entry.path()).is_ok() {
                removed += 1;
            }
        }
        removed
    }

    pub fn new(module_name: &str) -> Self {
        let dir = Self::cache_dir();
        let _ = fs::create_dir_all(&dir);
        Self {
            dir: dir.join(module_name),
        }
    }

    pub fn for_path(path: PathBuf) -> Self {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        Self { dir: path }
    }

    pub fn read(&self, ttl: Duration) -> Option<String> {
        let modified = fs::metadata(&self.dir).ok()?.modified().ok()?;
        if SystemTime::now().duration_since(modified).ok()? < ttl {
            fs::read_to_string(&self.dir).ok()
        } else {
            None
        }
    }

    pub fn read_stale(&self) -> Option<String> {
        fs::read_to_string(&self.dir).ok()
    }

    pub fn write(&self, value: &str) {
        // Write-then-rename prevents another RustFetch process from observing a
        // partially written cache entry.
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let temp_path = self
            .dir
            .with_extension(format!("{}-{}.tmp", std::process::id(), nanos));
        if fs::write(&temp_path, value).is_ok() && fs::rename(&temp_path, &self.dir).is_err() {
            let _ = fs::remove_file(temp_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_write_replaces_the_previous_value() {
        let root =
            std::env::temp_dir().join(format!("rustfetch-cache-test-{}", std::process::id()));
        let cache = Cache {
            dir: root.join("packages"),
        };
        fs::create_dir_all(&root).unwrap();

        cache.write("first");
        cache.write("second");

        assert_eq!(fs::read_to_string(&cache.dir).unwrap(), "second");
        // Ensure no leftover temporary files remain in root
        let tmp_files: Vec<_> = fs::read_dir(&root)
            .unwrap()
            .flatten()
            .filter(|e| e.path().to_string_lossy().contains(".tmp"))
            .collect();
        assert!(
            tmp_files.is_empty(),
            "Leftover temp files found: {:?}",
            tmp_files
        );
        fs::remove_dir_all(root).unwrap();
    }
}
