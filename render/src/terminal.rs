use std::mem;

pub fn get_winsize_cells() -> Option<(u16, u16)> {
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
    if std::env::var("KITTY_WINDOW_ID").is_ok() || std::env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        return true;
    }
    if let Ok(term) = std::env::var("TERM") {
        let term_lower = term.to_lowercase();
        if term_lower.contains("kitty") || term_lower.contains("ghostty") {
            return true;
        }
    }
    if let Ok(prog) = std::env::var("TERM_PROGRAM") {
        let p = prog.to_lowercase();
        if p.contains("ghostty") || p.contains("wezterm") {
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
    if std::env::var("SIXEL_SUPPORT").is_ok() {
        return true;
    }
    if let Ok(term) = std::env::var("TERM") {
        let t = term.to_lowercase();
        if t.contains("foot") || t.contains("mlterm") || t.contains("yaft") || t.contains("sixel") {
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
