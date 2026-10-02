use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct Cache {
    dir: PathBuf,
}

impl Cache {
    pub fn new(module_name: &str) -> Self {
        let base_dir = std::env::var("XDG_CACHE_HOME")
            .unwrap_or_else(|_| format!("{}/.cache", std::env::var("HOME").unwrap_or_default()));
        let dir = PathBuf::from(base_dir).join("rustfetch");
        let _ = fs::create_dir_all(&dir);
        Self {
            dir: dir.join(module_name),
        }
    }

    pub fn read(&self, ttl: Duration) -> Option<String> {
        let modified = fs::metadata(&self.dir).ok()?.modified().ok()?;
        if SystemTime::now().duration_since(modified).ok()? < ttl {
            fs::read_to_string(&self.dir).ok()
        } else {
            None
        }
    }

    pub fn write(&self, value: &str) {
        // Write-then-rename prevents another RustFetch process from observing a
        // partially written cache entry.
        let temp_path = self
            .dir
            .with_extension(format!("{}.tmp", std::process::id()));
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
        assert!(!cache
            .dir
            .with_extension(format!("{}.tmp", std::process::id()))
            .exists());
        fs::remove_dir_all(root).unwrap();
    }
}
