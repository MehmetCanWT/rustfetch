use crate::{Info, Module};
use std::fs;

pub struct FontModule;

impl Module for FontModule {
    fn name(&self) -> &'static str {
        "font"
    }

    fn detect(&self) -> Option<Info> {
        let config_dir = std::env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap_or_default()));

        if let Ok(content) = fs::read_to_string(format!("{config_dir}/kdeglobals")) {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("font=") {
                    let parts: Vec<&str> = rest.split(',').collect();
                    if !parts.is_empty() {
                        let name = parts[0];
                        let val = if parts.len() > 1 {
                            format!("{name} ({}pt) [Qt]", parts[1])
                        } else {
                            format!("{name} [Qt]")
                        };
                        return Some(Info::new("Font", val));
                    }
                }
            }
        }

        if let Ok(content) = fs::read_to_string(format!("{config_dir}/gtk-3.0/settings.ini")) {
            for line in content.lines() {
                if let Some(font) = line.strip_prefix("gtk-font-name=") {
                    return Some(Info::new("Font", format!("{font} [GTK]")));
                }
            }
        }

        None
    }
}
