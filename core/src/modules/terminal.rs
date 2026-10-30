use crate::{Info, Module};

pub struct Terminal;

impl Module for Terminal {
    fn name(&self) -> &'static str {
        "terminal"
    }

    fn detect(&self) -> Option<Info> {
        let terminal = read_terminal()?;
        Some(Info::new("Terminal", terminal))
    }
}

fn read_terminal() -> Option<String> {
    if let Ok(p) = std::env::var("TERM_PROGRAM") {
        let trimmed = p.trim();
        if !trimmed.is_empty() {
            return Some(format_terminal_name(trimmed));
        }
    }
    if std::env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        return Some("Ghostty".to_string());
    }
    if std::env::var("KITTY_WINDOW_ID").is_ok() || std::env::var("KITTY_PID").is_ok() {
        return Some("kitty".to_string());
    }
    if std::env::var("ALACRITTY_WINDOW_ID").is_ok() || std::env::var("ALACRITTY_SOCKET").is_ok() {
        return Some("Alacritty".to_string());
    }
    if std::env::var("WEZTERM_PANE").is_ok() || std::env::var("WEZTERM_EXECUTABLE").is_ok() {
        return Some("WezTerm".to_string());
    }
    if std::env::var("FOOT_SERVER_PATH").is_ok() {
        return Some("foot".to_string());
    }
    if std::env::var("KONSOLE_VERSION").is_ok() {
        return Some("Konsole".to_string());
    }
    if std::env::var("PTYXIS_VERSION").is_ok() {
        return Some("Ptyxis".to_string());
    }
    if std::env::var("KGX_VERSION").is_ok() {
        return Some("GNOME Console".to_string());
    }
    if std::env::var("WT_SESSION").is_ok() || std::env::var("WT_PROFILE_ID").is_ok() {
        return Some("Windows Terminal".to_string());
    }
    if std::env::var("TILIX_ID").is_ok() {
        return Some("Tilix".to_string());
    }
    if std::env::var("BLACKBOX_VERSION").is_ok() {
        return Some("BlackBox".to_string());
    }
    if std::env::var("RIO_VERSION").is_ok() {
        return Some("Rio".to_string());
    }
    if std::env::var("CONTOUR_VERSION").is_ok() {
        return Some("Contour".to_string());
    }
    if std::env::var("GUAKE_PID").is_ok() {
        return Some("Guake".to_string());
    }
    if std::env::var("TERMINOLOGY").is_ok() {
        return Some("Terminology".to_string());
    }
    if std::env::var("TABBY_CONFIG_DIRECTORY").is_ok() {
        return Some("Tabby".to_string());
    }
    if std::env::var("HYPER_TERM").is_ok() {
        return Some("Hyper".to_string());
    }
    if std::env::var("WARP_IS_LOCAL_SHELL_SESSION").is_ok() {
        return Some("Warp".to_string());
    }
    if std::env::var("VTE_VERSION").is_ok() {
        return Some("GNOME Terminal".to_string());
    }
    if let Ok(emu) = std::env::var("TERMINAL_EMULATOR") {
        let trimmed = emu.trim();
        if !trimmed.is_empty() {
            return Some(format_terminal_name(trimmed));
        }
    }
    #[cfg(target_os = "linux")]
    if let Some(proc_name) = detect_terminal_from_process() {
        return Some(proc_name);
    }
    parse_terminal(None, None, None, std::env::var("TERM").ok().as_deref())
}

pub fn format_terminal_name(raw: &str) -> String {
    match raw.to_lowercase().as_str() {
        "vscode" => "Visual Studio Code".to_string(),
        "iterm.app" => "iTerm2".to_string(),
        "apple_terminal" => "Apple Terminal".to_string(),
        "ghostty" => "Ghostty".to_string(),
        "wezterm" => "WezTerm".to_string(),
        "warpterminal" => "Warp".to_string(),
        "alacritty" => "Alacritty".to_string(),
        "kitty" => "kitty".to_string(),
        "foot" => "foot".to_string(),
        "konsole" => "Konsole".to_string(),
        "tabby" => "Tabby".to_string(),
        _ => raw.to_string(),
    }
}

#[cfg(target_os = "linux")]
fn detect_terminal_from_process() -> Option<String> {
    use std::fs;
    let mut pid = unsafe { libc::getppid() };
    for _ in 0..6 {
        if pid <= 1 {
            break;
        }
        let comm_path = format!("/proc/{pid}/comm");
        if let Ok(comm) = fs::read_to_string(&comm_path) {
            let name = comm.trim();
            let lower = name.to_lowercase();
            let matched = match lower.as_str() {
                "ptyxis" | "ptyxis-agent" => Some("Ptyxis"),
                "gnome-terminal" | "gnome-terminal-server" => Some("GNOME Terminal"),
                "kgx" => Some("GNOME Console"),
                "konsole" => Some("Konsole"),
                "kitty" => Some("kitty"),
                "alacritty" => Some("Alacritty"),
                "ghostty" => Some("Ghostty"),
                "foot" | "footclient" => Some("foot"),
                "wezterm-gui" | "wezterm" => Some("WezTerm"),
                "xfce4-terminal" => Some("XFCE Terminal"),
                "mate-terminal" => Some("MATE Terminal"),
                "terminator" => Some("Terminator"),
                "tilix" => Some("Tilix"),
                "blackbox" => Some("BlackBox"),
                "rio" => Some("Rio"),
                "contour" => Some("Contour"),
                "urxvt" | "rxvt-unicode" => Some("urxvt"),
                "xterm" => Some("xterm"),
                "st" | "st-256color" => Some("st"),
                "hyper" => Some("Hyper"),
                "tabby" => Some("Tabby"),
                "warp" | "warp-terminal" => Some("Warp"),
                "qterminal" => Some("QTerminal"),
                "lxterminal" => Some("LXTerminal"),
                "guake" => Some("Guake"),
                "yakuake" => Some("Yakuake"),
                "tilda" => Some("Tilda"),
                _ => None,
            };
            if let Some(m) = matched {
                return Some(m.to_string());
            }
        }
        let status_path = format!("/proc/{pid}/status");
        if let Ok(status) = fs::read_to_string(&status_path) {
            let mut next_pid = None;
            for line in status.lines() {
                if let Some(rest) = line.strip_prefix("PPid:\t") {
                    if let Ok(p) = rest.trim().parse::<libc::pid_t>() {
                        next_pid = Some(p);
                        break;
                    }
                }
            }
            match next_pid {
                Some(p) if p != pid => pid = p,
                _ => break,
            }
        } else {
            break;
        }
    }
    None
}

pub fn parse_terminal(
    term_program: Option<&str>,
    kitty_window_id: Option<&str>,
    terminal_emulator: Option<&str>,
    term: Option<&str>,
) -> Option<String> {
    let check = |s: Option<&str>| s.map(str::trim).filter(|s| !s.is_empty()).map(String::from);

    check(term_program)
        .or_else(|| check(kitty_window_id).map(|_| "kitty".to_string()))
        .or_else(|| check(terminal_emulator))
        .or_else(|| {
            term.map(str::trim)
                .filter(|s| !s.is_empty() && *s != "dumb")
                .map(String::from)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_terminal_term_program_precedence() {
        let result = parse_terminal(
            Some("ghostty"),
            Some("123"),
            Some("alacritty"),
            Some("xterm-256color"),
        );
        assert_eq!(result.as_deref(), Some("ghostty"));

        let result_wezterm = parse_terminal(Some("WezTerm"), None, None, Some("xterm-256color"));
        assert_eq!(result_wezterm.as_deref(), Some("WezTerm"));
    }

    #[test]
    fn test_parse_terminal_kitty_window_id() {
        let result = parse_terminal(None, Some("1"), None, Some("xterm-kitty"));
        assert_eq!(result.as_deref(), Some("kitty"));
    }

    #[test]
    fn test_parse_terminal_emulator() {
        let result = parse_terminal(
            None,
            None,
            Some("JetBrains-JediTerm"),
            Some("xterm-256color"),
        );
        assert_eq!(result.as_deref(), Some("JetBrains-JediTerm"));
    }

    #[test]
    fn test_parse_terminal_term_fallback() {
        let result = parse_terminal(None, None, None, Some("xterm-256color"));
        assert_eq!(result.as_deref(), Some("xterm-256color"));
    }

    #[test]
    fn test_parse_terminal_ignores_dumb_and_empty() {
        assert_eq!(parse_terminal(None, None, None, Some("dumb")), None);
        assert_eq!(
            parse_terminal(Some("   "), Some(""), Some(""), Some("dumb")),
            None
        );
        assert_eq!(parse_terminal(None, None, None, None), None);
    }

    #[test]
    fn test_format_terminal_name() {
        assert_eq!(format_terminal_name("vscode"), "Visual Studio Code");
        assert_eq!(format_terminal_name("iTerm.app"), "iTerm2");
        assert_eq!(format_terminal_name("ghostty"), "Ghostty");
        assert_eq!(format_terminal_name("wezterm"), "WezTerm");
        assert_eq!(format_terminal_name("warpterminal"), "Warp");
        assert_eq!(format_terminal_name("custom_term"), "custom_term");
    }
}
