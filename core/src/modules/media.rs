use crate::{Info, Module};
use std::process::Command;

pub struct Media;

impl Module for Media {
    fn name(&self) -> &'static str {
        "media"
    }

    fn detect(&self) -> Option<Info> {
        get_mpris_media()
    }
}

pub fn parse_playerctl_output(stdout: &str) -> Option<String> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() || trimmed == "-" {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn get_mpris_media() -> Option<Info> {
    if let Ok(output) = Command::new("playerctl")
        .args(["metadata", "--format", "{{artist}} - {{title}}"])
        .output()
    {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            if let Some(val) = parse_playerctl_output(&text) {
                return Some(Info::new("Media", val));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_module_name() {
        assert_eq!(Media.name(), "media");
    }

    #[test]
    fn test_parse_playerctl_output() {
        assert_eq!(
            parse_playerctl_output("Queen - Bohemian Rhapsody\n"),
            Some("Queen - Bohemian Rhapsody".into())
        );
        assert_eq!(parse_playerctl_output(""), None);
        assert_eq!(parse_playerctl_output("   "), None);
        assert_eq!(parse_playerctl_output("-"), None);
    }
}
