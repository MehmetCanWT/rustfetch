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
    parse_terminal(
        std::env::var("TERM_PROGRAM").ok().as_deref(),
        std::env::var("KITTY_WINDOW_ID").ok().as_deref(),
        std::env::var("TERMINAL_EMULATOR").ok().as_deref(),
        std::env::var("TERM").ok().as_deref(),
    )
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

/// Helper function to parse terminal using an environment lookup closure.
pub fn parse_terminal_env<F: Fn(&str) -> Option<String>>(get_var: F) -> Option<String> {
    parse_terminal(
        get_var("TERM_PROGRAM").as_deref(),
        get_var("KITTY_WINDOW_ID").as_deref(),
        get_var("TERMINAL_EMULATOR").as_deref(),
        get_var("TERM").as_deref(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_terminal_term_program_precedence() {
        // TERM_PROGRAM should take precedence over all other variables
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
        // In Kitty on Fedora Linux, KITTY_WINDOW_ID is set (e.g., "1")
        let result = parse_terminal(None, Some("1"), None, Some("xterm-kitty"));
        assert_eq!(result.as_deref(), Some("kitty"));
    }

    #[test]
    fn test_parse_terminal_emulator() {
        // JetBrains embedded terminal sets TERMINAL_EMULATOR
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
        // Generic fallback to TERM on Fedora Linux
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
    fn test_parse_terminal_env_helper() {
        let env_map = |key: &str| match key {
            "KITTY_WINDOW_ID" => Some("42".to_string()),
            "TERM" => Some("xterm-kitty".to_string()),
            _ => None,
        };
        assert_eq!(parse_terminal_env(env_map).as_deref(), Some("kitty"));
    }
}
