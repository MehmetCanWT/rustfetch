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

        let wm_name = detect_wm_name(&de);

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

fn detect_wm_name(de: &str) -> String {
    if let Some(wm) = get_wm_from_env(de) {
        return wm;
    }
    get_wm_name_from_proc()
}

pub fn get_wm_from_env(de: &str) -> Option<String> {
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return Some("Hyprland".to_string());
    }
    if env::var("SWAYSOCK").is_ok() {
        return Some("Sway".to_string());
    }
    if env::var("I3SOCK").is_ok() {
        return Some("i3".to_string());
    }
    if env::var("RIVER_RUNTIME_DIR").is_ok() {
        return Some("River".to_string());
    }

    let session = env::var("DESKTOP_SESSION").unwrap_or_default();
    get_wm_from_names(de, &session)
}

fn match_wm_name(s: &str) -> Option<&'static str> {
    let lower = s.to_lowercase();
    if lower.contains("kde") || lower.contains("plasma") {
        Some("KWin")
    } else if lower.contains("gnome") {
        Some("Mutter")
    } else if lower.contains("cinnamon") || lower.contains("muffin") {
        Some("Muffin")
    } else if lower.contains("mate") || lower.contains("marco") {
        Some("Marco")
    } else if lower.contains("hyprland") {
        Some("Hyprland")
    } else if lower.contains("sway") {
        Some("Sway")
    } else if lower.contains("mangowm") || lower.contains("mango") {
        Some("MangoWM")
    } else if lower.contains("niri") {
        Some("Niri")
    } else if lower.contains("river") {
        Some("River")
    } else if lower.contains("dwl") {
        Some("dwl")
    } else if lower.contains("wayfire") {
        Some("Wayfire")
    } else if lower.contains("labwc") {
        Some("labwc")
    } else if lower.contains("cosmic") {
        Some("COSMIC")
    } else if lower.contains("xfce") || lower.contains("xfwm") {
        Some("Xfwm4")
    } else if lower.contains("i3") {
        Some("i3")
    } else if lower.contains("bspwm") {
        Some("bspwm")
    } else if lower.contains("awesome") {
        Some("awesome")
    } else if lower.contains("qtile") {
        Some("Qtile")
    } else if lower.contains("openbox") {
        Some("Openbox")
    } else if lower.contains("dwm") {
        Some("dwm")
    } else {
        None
    }
}

pub fn get_wm_from_names(de: &str, session: &str) -> Option<String> {
    match_wm_name(de)
        .or_else(|| match_wm_name(session))
        .map(|s| s.to_string())
}

fn get_wm_name_from_proc() -> String {
    if let Ok(dir) = std::fs::read_dir("/proc") {
        for entry in dir.flatten() {
            let fname = entry.file_name();
            let Some(pid_str) = fname.to_str() else {
                continue;
            };
            if !pid_str.bytes().all(|b| b.is_ascii_digit()) {
                continue;
            }
            if let Ok(comm) = std::fs::read_to_string(format!("/proc/{pid_str}/comm")) {
                let name = match comm.trim() {
                    "kwin_wayland" | "kwin_x11" => "KWin",
                    "mutter" | "gnome-shell" => "Mutter",
                    "cinnamon" | "muffin" => "Muffin",
                    "marco" => "Marco",
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

    #[test]
    fn test_get_wm_from_names() {
        assert_eq!(get_wm_from_names("KDE", ""), Some("KWin".to_string()));
        assert_eq!(get_wm_from_names("GNOME", ""), Some("Mutter".to_string()));
        assert_eq!(
            get_wm_from_names("Hyprland", ""),
            Some("Hyprland".to_string())
        );
        assert_eq!(get_wm_from_names("sway", ""), Some("Sway".to_string()));
        assert_eq!(
            get_wm_from_names("MangoWM", ""),
            Some("MangoWM".to_string())
        );
        assert_eq!(
            get_wm_from_names("Cinnamon", ""),
            Some("Muffin".to_string())
        );
        assert_eq!(get_wm_from_names("MATE", ""), Some("Marco".to_string()));
        assert_eq!(
            get_wm_from_names("", "plasma.desktop"),
            Some("KWin".to_string())
        );
        assert_eq!(get_wm_from_names("custom_nonexistent", "other"), None);
    }
}
