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

        let value = format_desktop(&de, &wm_name, &wm_type)?;

        Some(Info::new("Desktop", value))
    }
}

pub fn format_desktop(de: &str, wm_name: &str, wm_type: &str) -> Option<String> {
    let de = de.trim();
    let wm_name = wm_name.trim();
    let wm_type = wm_type.trim();

    if de.is_empty() && wm_name.is_empty() {
        return None;
    }

    let same_de_wm = !de.is_empty() && !wm_name.is_empty() && de.eq_ignore_ascii_case(wm_name);

    let value = match (
        de.is_empty(),
        wm_name.is_empty() || same_de_wm,
        wm_type.is_empty(),
    ) {
        (false, true, true) => de.to_string(),
        (false, true, false) => format!("{de} ({wm_type})"),
        (true, false, true) => wm_name.to_string(),
        (true, false, false) => format!("{wm_name} ({wm_type})"),
        (false, false, true) => format!("{de} ({wm_name})"),
        (false, false, false) => format!("{de} ({wm_name} {wm_type})"),
        (true, true, _) => return None,
    };

    Some(value)
}

fn get_wm_name() -> String {
    if let Ok(dir) = std::fs::read_dir("/proc") {
        for entry in dir.flatten() {
            let fname = entry.file_name();
            if !fname
                .to_str()
                .is_some_and(|s| s.bytes().all(|b| b.is_ascii_digit()))
            {
                continue;
            }
            if let Ok(comm) = std::fs::read_to_string(entry.path().join("comm")) {
                let name = match comm.trim() {
                    "kwin_wayland" | "kwin_x11" => "KWin",
                    "mutter" | "gnome-shell" => "Mutter",
                    "sway" => "Sway",
                    "hyprland" | "Hyprland" => "Hyprland",
                    "mangowm" | "mango" => "MangoWM",
                    "niri" => "Niri",
                    "river" => "River",
                    "dwl" => "dwl",
                    "wayfire" => "Wayfire",
                    "labwc" => "labwc",
                    "cosmic-comp" => "COSMIC",
                    "xfwm4" => "Xfwm4",
                    "openbox" => "Openbox",
                    "i3" => "i3",
                    "bspwm" => "bspwm",
                    "awesome" => "awesome",
                    "qtile" => "Qtile",
                    "dwm" => "dwm",
                    _ => continue,
                };
                return name.to_string();
            }
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_desktop_deduplicates_de_and_wm() {
        assert_eq!(
            format_desktop("Hyprland", "Hyprland", "Wayland"),
            Some("Hyprland (Wayland)".to_string())
        );
        assert_eq!(
            format_desktop("KDE", "KWin", "Wayland"),
            Some("KDE (KWin Wayland)".to_string())
        );
        assert_eq!(
            format_desktop("MangoWM", "", "Wayland"),
            Some("MangoWM (Wayland)".to_string())
        );
        assert_eq!(format_desktop("", "", "Wayland"), None);
    }
}
