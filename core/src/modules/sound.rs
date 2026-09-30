use crate::{Info, Module};
use std::process::Command;

pub struct Sound;

impl Module for Sound {
    fn name(&self) -> &'static str {
        "sound"
    }

    fn detect(&self) -> Option<Info> {
        if let Some(vol) = get_wpctl_volume() {
            return Some(Info::new("Sound", vol));
        }
        if let Some(card) = get_asound_card() {
            return Some(Info::new("Sound", card));
        }
        None
    }
}

pub fn parse_wpctl_output(output: &str) -> Option<String> {
    let line = output.lines().next()?;
    let rest = line.strip_prefix("Volume:")?.trim();
    let mut parts = rest.split_whitespace();
    let vol_float: f64 = parts.next()?.parse().ok()?;
    let pct = (vol_float * 100.0).round() as u32;
    if rest.contains("[MUTED]") {
        Some(format!("{pct}% [MUTED]"))
    } else {
        Some(format!("{pct}%"))
    }
}

fn get_wpctl_volume() -> Option<String> {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()
        .ok()?;
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        parse_wpctl_output(&text)
    } else {
        None
    }
}

fn get_asound_card() -> Option<String> {
    let content = std::fs::read_to_string("/proc/asound/cards").ok()?;
    for line in content.lines() {
        if let Some((_, right)) = line.split_once("]:") {
            let desc = right.split('-').nth(1).unwrap_or(right).trim();
            if !desc.is_empty() {
                return Some(desc.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_module_name() {
        assert_eq!(Sound.name(), "sound");
    }

    #[test]
    fn test_parse_wpctl_output() {
        assert_eq!(parse_wpctl_output("Volume: 0.65"), Some("65%".into()));
        assert_eq!(
            parse_wpctl_output("Volume: 1.00 [MUTED]"),
            Some("100% [MUTED]".into())
        );
        assert_eq!(parse_wpctl_output("Invalid"), None);
    }
}
