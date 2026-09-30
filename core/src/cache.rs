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
        let _ = fs::write(&self.dir, value);
    }
}
