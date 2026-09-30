use crate::{Info, Module};

pub struct Os;

impl Module for Os {
    fn name(&self) -> &'static str {
        "os"
    }

    fn detect(&self) -> Option<Info> {
        let content = read_os()?;
        let os = parse_os(&content)?;
        Some(Info::new("OS", os))
    }
}

pub fn read_os() -> Option<String> {
    std::fs::read_to_string("/etc/os-release")
        .or_else(|_| std::fs::read_to_string("/usr/lib/os-release"))
        .ok()
}

fn unquote(val: &str) -> &str {
    let s = val.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        if s.len() >= 2 {
            &s[1..s.len() - 1]
        } else {
            s
        }
    } else {
        s
    }
}

pub fn parse_os(content: &str) -> Option<String> {
    let mut pretty_name = None;
    let mut name = None;
    let mut version = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let value = unquote(value);
            match key.trim() {
                "PRETTY_NAME" => pretty_name = Some(value.to_string()),
                "NAME" => name = Some(value.to_string()),
                "VERSION" => version = Some(value.to_string()),
                _ => {}
            }
        }
    }

    if let Some(pretty) = pretty_name.filter(|s| !s.is_empty()) {
        return Some(pretty);
    }

    match (
        name.filter(|s| !s.is_empty()),
        version.filter(|s| !s.is_empty()),
    ) {
        (Some(n), Some(v)) => Some(format!("{n} {v}")),
        (Some(n), None) => Some(n),
        (None, Some(v)) => Some(v),
        (None, None) => None,
    }
}

pub fn parse_os_release(content: &str) -> Option<String> {
    parse_os(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_parse_os_fedora_pretty_name() {
        let fixture = r#"
NAME="Fedora Linux"
VERSION="42 (Workstation Edition)"
ID=fedora
VERSION_ID=42
VERSION_CODENAME=""
PRETTY_NAME="Fedora Linux 42 (Workstation Edition)"
ANSI_COLOR="0;38;2;60;110;180"
LOGO=fedora-logo-icon
CPE_NAME="cpe:/o:fedoraproject:fedora:42"
DEFAULT_HOSTNAME="fedora"
HOME_URL="https://fedoraproject.org/"
DOCUMENTATION_URL="https://docs.fedoraproject.org/en-US/fedora/f42/"
SUPPORT_URL="https://ask.fedoraproject.org/"
BUG_REPORT_URL="https://bugzilla.redhat.com/"
REDHAT_BUGZILLA_PRODUCT="Fedora"
REDHAT_BUGZILLA_PRODUCT_VERSION=42
REDHAT_SUPPORT_PRODUCT="Fedora"
REDHAT_SUPPORT_PRODUCT_VERSION=42
SUPPORT_END=2026-05-13
VARIANT="Workstation Edition"
VARIANT_ID=workstation
"#;
        assert_eq!(
            parse_os(fixture),
            Some("Fedora Linux 42 (Workstation Edition)".to_string())
        );
        assert_eq!(
            parse_os_release(fixture),
            Some("Fedora Linux 42 (Workstation Edition)".to_string())
        );
    }

    #[test]
    fn test_parse_os_fallback_name_and_version() {
        let fixture = r#"
NAME="Fedora Linux"
VERSION="42 (Workstation Edition)"
ID=fedora
VERSION_ID=42
"#;
        assert_eq!(
            parse_os(fixture),
            Some("Fedora Linux 42 (Workstation Edition)".to_string())
        );
    }

    #[test]
    fn test_parse_os_fallback_name_only() {
        let fixture = r#"
NAME="Arch Linux"
ID=arch
"#;
        assert_snapshot!(parse_os(fixture).unwrap(), @"Arch Linux");
    }

    #[test]
    fn test_parse_os_unquoted_and_single_quotes() {
        let fixture_unquoted = "PRETTY_NAME=Alpine Linux v3.19\n";
        assert_eq!(
            parse_os(fixture_unquoted),
            Some("Alpine Linux v3.19".to_string())
        );

        let fixture_single = "PRETTY_NAME='Void Linux'\n";
        assert_snapshot!(parse_os(fixture_single).unwrap(), @"Void Linux");
    }

    #[test]
    fn test_parse_os_empty_and_invalid() {
        assert_eq!(parse_os(""), None);
        assert_eq!(parse_os("# Comment line only\n"), None);
        assert_eq!(parse_os("SOME_OTHER_KEY=value\n"), None);
    }

    #[test]
    fn test_os_detect() {
        let os = Os;
        assert_eq!(os.name(), "os");
        if let Some(info) = os.detect() {
            assert_eq!(info.label, "OS");
            assert!(!info.value.is_empty());
        }
    }
}
