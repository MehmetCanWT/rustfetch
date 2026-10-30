use std::mem;

pub fn get_winsize_cells() -> Option<(u16, u16)> {
    // SAFETY: TIOCGWINSZ ioctl only queries the terminal emulator window dimensions
    // and writes to our locally allocated libc::winsize struct.
    unsafe {
        let mut ws: libc::winsize = mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0
            && ws.ws_col > 0
            && ws.ws_row > 0
        {
            return Some((ws.ws_col, ws.ws_row));
        }
    }
    None
}

pub fn get_cell_size() -> Option<(u16, u16)> {
    // SAFETY: TIOCGWINSZ ioctl safely queries the terminal emulator window dimensions in pixels.
    unsafe {
        let mut ws: libc::winsize = mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0
            && ws.ws_col > 0
            && ws.ws_row > 0
            && ws.ws_xpixel > 0
            && ws.ws_ypixel > 0
        {
            return Some((ws.ws_xpixel / ws.ws_col, ws.ws_ypixel / ws.ws_row));
        }
    }
    None
}

/// Returns (cell_w, cell_h) in pixels, defaulting to (10, 20) if terminal does not report pixel sizes.
pub fn cell_size_or_default() -> (u16, u16) {
    get_cell_size().unwrap_or((10, 20))
}

pub fn supports_kitty() -> bool {
    use std::io::IsTerminal;
    if !std::io::stdout().is_terminal() {
        return false;
    }
    if std::env::var("NO_COLOR").is_ok() || std::env::var("TERM").as_deref() == Ok("dumb") {
        return false;
    }
    if std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("KITTY_PID").is_ok()
        || std::env::var("GHOSTTY_RESOURCES_DIR").is_ok()
        || std::env::var("WEZTERM_PANE").is_ok()
        || std::env::var("RIO_VERSION").is_ok()
    {
        return true;
    }
    if let Ok(term) = std::env::var("TERM") {
        let term_lower = term.to_lowercase();
        if term_lower.contains("kitty")
            || term_lower.contains("ghostty")
            || term_lower.contains("wayst")
        {
            return true;
        }
    }
    if let Ok(prog) = std::env::var("TERM_PROGRAM") {
        let p = prog.to_lowercase();
        if p.contains("ghostty") || p.contains("wezterm") || p.contains("rio") {
            return true;
        }
    }
    false
}

pub fn supports_sixel() -> bool {
    use std::io::IsTerminal;
    if !std::io::stdout().is_terminal() {
        return false;
    }
    if std::env::var("NO_COLOR").is_ok() || std::env::var("TERM").as_deref() == Ok("dumb") {
        return false;
    }
    if std::env::var("SIXEL_SUPPORT").is_ok()
        || std::env::var("FOOT_SERVER_PATH").is_ok()
        || std::env::var("CONTOUR_VERSION").is_ok()
    {
        return true;
    }
    if let Ok(term) = std::env::var("TERM") {
        let t = term.to_lowercase();
        if t.contains("foot")
            || t.contains("mlterm")
            || t.contains("yaft")
            || t.contains("sixel")
            || t.contains("chafa")
        {
            return true;
        }
    }
    if let Ok(prog) = std::env::var("TERM_PROGRAM") {
        let p = prog.to_lowercase();
        if p.contains("foot")
            || p.contains("wezterm")
            || p.contains("contour")
            || p.contains("mlterm")
        {
            return true;
        }
    }
    false
}

pub fn supports_iterm() -> bool {
    use std::io::IsTerminal;
    if !std::io::stdout().is_terminal() {
        return false;
    }
    if std::env::var("NO_COLOR").is_ok() || std::env::var("TERM").as_deref() == Ok("dumb") {
        return false;
    }
    if std::env::var("ITERM_SESSION_ID").is_ok()
        || std::env::var("ITERM_SUPPORT").is_ok()
        || std::env::var("KONSOLE_VERSION").is_ok()
        || std::env::var("TABBY_CONFIG_DIRECTORY").is_ok()
        || std::env::var("WEZTERM_PANE").is_ok()
    {
        return true;
    }
    if std::env::var("LC_TERMINAL").as_deref() == Ok("iTerm2") {
        return true;
    }
    if let Ok(prog) = std::env::var("TERM_PROGRAM") {
        let p = prog.to_lowercase();
        if p.contains("iterm")
            || p.contains("wezterm")
            || p.contains("warp")
            || p.contains("tabby")
            || p.contains("mintty")
            || p.contains("konsole")
        {
            return true;
        }
    }
    if let Ok(term) = std::env::var("TERM") {
        let t = term.to_lowercase();
        if t.contains("iterm") || t.contains("wezterm") || t.contains("konsole") {
            return true;
        }
    }
    false
}

pub fn is_tmux() -> bool {
    if std::env::var("TMUX").is_ok() {
        return true;
    }
    if let Ok(term) = std::env::var("TERM") {
        if term.starts_with("tmux") || term.starts_with("screen") {
            return true;
        }
    }
    false
}

pub fn wrap_tmux_passthrough(seq: &str) -> String {
    if !is_tmux() {
        return seq.to_string();
    }
    wrap_tmux_passthrough_always(seq)
}

pub fn wrap_tmux_passthrough_always(seq: &str) -> String {
    let mut out = String::with_capacity(seq.len() + 32);
    out.push_str("\x1bPtmux;");
    for ch in seq.chars() {
        if ch == '\x1b' {
            out.push_str("\x1b\x1b");
        } else {
            out.push(ch);
        }
    }
    out.push_str("\x1b\\");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_tmux_passthrough_always() {
        let kitty_cmd = "\x1b_Ga=d,d=a\x1b\\";
        let wrapped = wrap_tmux_passthrough_always(kitty_cmd);
        assert_eq!(wrapped, "\x1bPtmux;\x1b\x1b_Ga=d,d=a\x1b\x1b\\\x1b\\");

        let sixel_cmd = "\x1bPq\"1;1;2;1\x1b\\";
        let wrapped_sixel = wrap_tmux_passthrough_always(sixel_cmd);
        assert_eq!(
            wrapped_sixel,
            "\x1bPtmux;\x1b\x1bPq\"1;1;2;1\x1b\x1b\\\x1b\\"
        );
    }
}
