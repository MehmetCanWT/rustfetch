use crate::{Info, Module};
use std::env;

pub struct DesktopModule;

impl Module for DesktopModule {
    fn name(&self) -> &'static str {
        "desktop"
    }

    fn detect(&self) -> Option<Info> {
        let de = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_default();

        let wm_type = env::var("WAYLAND_DISPLAY")
            .map(|_| "Wayland".to_string())
            .or_else(|_| env::var("XDG_SESSION_TYPE"))
            .unwrap_or_default();

        let wm_name = get_wm_name();

        let value = match (de.is_empty(), wm_name.is_empty()) {
            (true, true) => return None,
            (false, true) => de,
            (true, false) => format!("{wm_name} ({wm_type})"),
            (false, false) => format!("{de} ({wm_name} {wm_type})"),
        };

        Some(Info::new("Desktop", value))
    }
}

fn get_wm_name() -> String {
    if let Ok(dir) = std::fs::read_dir("/proc") {
        for entry in dir.flatten() {
            if let Ok(comm) = std::fs::read_to_string(entry.path().join("comm")) {
                let name = match comm.trim() {
                    "kwin_wayland" | "kwin_x11" => "KWin",
                    "mutter" | "gnome-shell" => "Mutter",
                    "sway" => "Sway",
                    "hyprland" | "Hyprland" => "Hyprland",
                    "xfwm4" => "Xfwm4",
                    "openbox" => "Openbox",
                    "i3" => "i3",
                    _ => continue,
                };
                return name.to_string();
            }
        }
    }
    String::new()
}
