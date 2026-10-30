use crate::{Info, Module};
use std::path::Path;

pub struct Shell;

impl Module for Shell {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn detect(&self) -> Option<Info> {
        let shell = if let Ok(path) = std::env::var("SHELL") {
            parse_shell(&path)
        } else {
            None
        };
        let shell = shell.or_else(|| {
            let ppid = std::fs::read_to_string("/proc/self/status")
                .ok()?
                .lines()
                .find_map(|l| l.strip_prefix("PPid:"))
                .and_then(|p| p.trim().parse::<u32>().ok())?;
            let comm = std::fs::read_to_string(format!("/proc/{ppid}/comm")).ok()?;
            let trimmed = comm.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        })?;
        Some(Info::new("Shell", shell))
    }
}

pub fn parse_shell(path: &str) -> Option<String> {
    let trimmed = path.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return None;
    }
    let file_name = Path::new(trimmed).file_name()?.to_str()?;
    if file_name.is_empty() {
        return None;
    }
    Some(file_name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shell_fixtures() {
        // Standard Fedora Linux shell paths
        assert_eq!(parse_shell("/bin/bash").as_deref(), Some("bash"));
        assert_eq!(parse_shell("/usr/bin/bash").as_deref(), Some("bash"));
        assert_eq!(parse_shell("/usr/bin/zsh").as_deref(), Some("zsh"));
        assert_eq!(parse_shell("/usr/bin/fish").as_deref(), Some("fish"));
        assert_eq!(parse_shell("/bin/sh").as_deref(), Some("sh"));
        assert_eq!(parse_shell("/usr/local/bin/nu").as_deref(), Some("nu"));
    }

    #[test]
    fn test_parse_shell_trailing_slashes_and_whitespace() {
        assert_eq!(parse_shell("/bin/bash/").as_deref(), Some("bash"));
        assert_eq!(parse_shell("  /usr/bin/fish  ").as_deref(), Some("fish"));
    }

    #[test]
    fn test_parse_shell_just_name() {
        assert_eq!(parse_shell("bash").as_deref(), Some("bash"));
        assert_eq!(parse_shell("zsh").as_deref(), Some("zsh"));
    }

    #[test]
    fn test_parse_shell_invalid() {
        assert_eq!(parse_shell(""), None);
        assert_eq!(parse_shell("   "), None);
        assert_eq!(parse_shell("/"), None);
        assert_eq!(parse_shell("///"), None);
    }
}
