pub mod ascii;
pub mod cache;
pub mod config;
pub mod modules;
pub mod sha256;

pub use ascii::apply_neofetch_colors;
pub use config::{
    default_icon_for_module, distro_icon, BorderCharsConfig, BorderImageConfig, Config,
};
pub use modules::update::{codename_for_version, format_version_with_codename, CURRENT_VERSION};
pub use modules::{all_modules, module_by_name, module_from_config};

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
    fn is_slow(&self) -> bool {
        false
    }
}

pub fn detect_all(modules: &[Box<dyn Module>]) -> Vec<Option<Info>> {
    if !modules.iter().any(|m| m.is_slow()) {
        return modules.iter().map(|m| m.detect()).collect();
    }

    std::thread::scope(|s| {
        let mut slow_handles = Vec::new();
        for (i, m) in modules.iter().enumerate() {
            if m.is_slow() {
                slow_handles.push((i, s.spawn(|| m.detect())));
            }
        }

        let mut results: Vec<Option<Info>> = Vec::with_capacity(modules.len());
        for m in modules {
            if !m.is_slow() {
                results.push(m.detect());
            } else {
                results.push(None);
            }
        }

        for (i, handle) in slow_handles {
            results[i] = handle.join().ok().flatten();
        }

        results
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FastModule(&'static str);
    impl Module for FastModule {
        fn name(&self) -> &'static str {
            self.0
        }
        fn detect(&self) -> Option<Info> {
            Some(Info::new(self.0, "fast_val"))
        }
    }

    struct SlowModule(&'static str);
    impl Module for SlowModule {
        fn name(&self) -> &'static str {
            self.0
        }
        fn detect(&self) -> Option<Info> {
            Some(Info::new(self.0, "slow_val"))
        }
        fn is_slow(&self) -> bool {
            true
        }
    }

    struct NoneModule;
    impl Module for NoneModule {
        fn name(&self) -> &'static str {
            "none"
        }
        fn detect(&self) -> Option<Info> {
            None
        }
    }

    #[test]
    fn test_detect_all_empty() {
        let empty: Vec<Box<dyn Module>> = vec![];
        let res = detect_all(&empty);
        assert!(res.is_empty());
    }

    #[test]
    fn test_detect_all_only_fast() {
        let modules: Vec<Box<dyn Module>> = vec![
            Box::new(FastModule("m1")),
            Box::new(NoneModule),
            Box::new(FastModule("m2")),
        ];
        let res = detect_all(&modules);
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].as_ref().unwrap().value, "fast_val");
        assert!(res[1].is_none());
        assert_eq!(res[2].as_ref().unwrap().value, "fast_val");
    }

    #[test]
    fn test_detect_all_mixed_preserves_order() {
        let modules: Vec<Box<dyn Module>> = vec![
            Box::new(FastModule("m1")),
            Box::new(SlowModule("s1")),
            Box::new(FastModule("m2")),
            Box::new(SlowModule("s2")),
        ];
        let res = detect_all(&modules);
        assert_eq!(res.len(), 4);
        assert_eq!(res[0].as_ref().unwrap().label, "m1");
        assert_eq!(res[1].as_ref().unwrap().label, "s1");
        assert_eq!(res[2].as_ref().unwrap().label, "m2");
        assert_eq!(res[3].as_ref().unwrap().label, "s2");
    }

    #[test]
    fn test_slow_module_overrides() {
        assert!(!modules::os::Os.is_slow());
        assert!(modules::cpu_usage::CpuUsage.is_slow());
        assert!(modules::wifi::Wifi.is_slow());
        assert!(modules::sound::Sound.is_slow());
        assert!(modules::media::Media.is_slow());
        assert!(modules::bluetooth::BluetoothModule.is_slow());
    }
}
