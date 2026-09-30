pub mod cache;
pub mod config;
pub mod modules;

pub use config::{default_icon_for_module, make_progress_bar, Config};

#[derive(Debug, Clone)]
pub struct Info {
    pub label: String,
    pub value: String,
}

impl Info {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self) -> Option<Info>;
}

pub fn detect_all(modules: &[Box<dyn Module>]) -> Vec<Option<Info>> {
    std::thread::scope(|s| {
        let handles: Vec<_> = modules.iter().map(|m| s.spawn(|| m.detect())).collect();
        handles
            .into_iter()
            .map(|h| h.join().ok().flatten())
            .collect()
    })
}
