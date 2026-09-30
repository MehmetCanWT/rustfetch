use crate::{Info, Module};

pub struct Kernel;

impl Module for Kernel {
    fn name(&self) -> &'static str {
        "kernel"
    }

    fn detect(&self) -> Option<Info> {
        let content = read_kernel()?;
        let version = parse_kernel(&content)?;
        Some(Info::new("Kernel", version))
    }
}

pub fn read_kernel() -> Option<String> {
    std::fs::read_to_string("/proc/version").ok()
}

pub fn parse_kernel(content: &str) -> Option<String> {
    parse_kernel_version(content)
}

pub fn parse_kernel_version(content: &str) -> Option<String> {
    let mut words = content.split_whitespace();
    if words.next()? == "Linux" && words.next()? == "version" {
        words.next().map(|v| v.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kernel_version_fedora() {
        let fixture = "Linux version 6.8.5-301.fc40.x86_64 (mockbuild@...) (gcc ...) #1 SMP ...";
        assert_eq!(
            parse_kernel(fixture),
            Some("6.8.5-301.fc40.x86_64".to_string())
        );
        assert_eq!(
            parse_kernel_version(fixture),
            Some("6.8.5-301.fc40.x86_64".to_string())
        );
    }

    #[test]
    fn test_parse_kernel_version_fedora_44() {
        let fixture = "Linux version 7.2.7-200.fc44.x86_64 (mockbuild@4bc3738664d84c798c5efe0695a77dbf) (gcc (GCC) 16.2.1 20260819 (Red Hat 16.2.1-2), GNU ld version 2.46.1-1.fc44) #1 SMP PREEMPT_DYNAMIC Mon Sep 21 18:25:57 UTC 2026\n";
        assert_eq!(
            parse_kernel(fixture),
            Some("7.2.7-200.fc44.x86_64".to_string())
        );
    }

    #[test]
    fn test_parse_kernel_version_invalid() {
        assert_eq!(parse_kernel(""), None);
        assert_eq!(parse_kernel("Linux"), None);
        assert_eq!(parse_kernel("Linux version"), None);
        assert_eq!(parse_kernel("Darwin Kernel Version 21.4.0"), None);
        assert_eq!(parse_kernel("Something else entirely"), None);
    }

    #[test]
    fn test_kernel_detect() {
        let kernel = Kernel;
        assert_eq!(kernel.name(), "kernel");
        let detected = kernel.detect();
        assert!(detected.is_some(), "Kernel detect should succeed on Linux");
        let info = detected.unwrap();
        assert_eq!(info.label, "Kernel");
        assert!(!info.value.is_empty());
    }
}
