use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";

pub fn colors_enabled() -> bool {
    if std::env::var("FORCE_COLOR")
        .map(|v| v != "0")
        .unwrap_or(false)
        || std::env::var("CLICOLOR_FORCE")
            .map(|v| v != "0")
            .unwrap_or(false)
    {
        return true;
    }
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }
    if let Ok(term) = std::env::var("TERM") {
        if term == "dumb" {
            return false;
        }
    }
    true
}

pub fn is_linux_console() -> bool {
    std::env::var("TERM").as_deref() == Ok("linux")
}

pub fn rgb_to_ansi(r: u8, g: u8, b: u8) -> &'static str {
    const BASIC: &[(u8, u8, u8, &str)] = &[
        (0, 0, 0, "\x1b[30m"),
        (205, 0, 0, "\x1b[31m"),
        (0, 205, 0, "\x1b[32m"),
        (205, 205, 0, "\x1b[33m"),
        (0, 0, 238, "\x1b[34m"),
        (205, 0, 205, "\x1b[35m"),
        (0, 205, 205, "\x1b[36m"),
        (229, 229, 229, "\x1b[37m"),
        (127, 127, 127, "\x1b[90m"),
        (255, 0, 0, "\x1b[91m"),
        (0, 255, 0, "\x1b[92m"),
        (255, 255, 0, "\x1b[93m"),
        (92, 92, 255, "\x1b[94m"),
        (255, 0, 255, "\x1b[95m"),
        (0, 255, 255, "\x1b[96m"),
        (255, 255, 255, "\x1b[97m"),
    ];

    let mut best_dist = u32::MAX;
    let mut best_code = "\x1b[37m";

    for &(br, bg, bb, code) in BASIC {
        let dr = (r as i32) - (br as i32);
        let dg = (g as i32) - (bg as i32);
        let db = (b as i32) - (bb as i32);
        let dist = (dr * dr + dg * dg + db * db) as u32;
        if dist < best_dist {
            best_dist = dist;
            best_code = code;
        }
    }
    best_code
}

pub fn color_code(name: &str) -> String {
    if name.starts_with("\x1b[") {
        return name.to_string();
    }
    if name.starts_with('#') {
        let hex = name.trim_start_matches('#');
        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let (r, g, b) = if hex.len() == 6 {
                ((rgb >> 16) & 0xFF, (rgb >> 8) & 0xFF, rgb & 0xFF)
            } else if hex.len() == 3 {
                let r = (rgb >> 8) & 0xF;
                let g = (rgb >> 4) & 0xF;
                let b = rgb & 0xF;
                (r | (r << 4), g | (g << 4), b | (b << 4))
            } else {
                (255, 255, 255)
            };
            if is_linux_console() {
                return rgb_to_ansi(r as u8, g as u8, b as u8).to_string();
            }
            return format!("\x1b[38;2;{};{};{}m", r, g, b);
        }
    }

    let code = match name.to_lowercase().as_str() {
        "black" => "\x1b[38;2;20;20;20m",
        "red" => "\x1b[38;2;239;83;80m",
        "green" => "\x1b[38;2;76;175;80m",
        "yellow" => "\x1b[38;2;255;193;7m",
        "blue" => "\x1b[38;2;66;165;245m",
        "magenta" => "\x1b[38;2;186;104;200m",
        "cyan" => "\x1b[38;2;38;198;218m",
        "white" => "\x1b[38;2;245;245;245m",
        "bright_black" | "gray" | "grey" => "\x1b[38;2;140;140;140m",
        "bright_red" => "\x1b[38;2;255;82;82m",
        "bright_green" => "\x1b[38;2;105;240;174m",
        "bright_yellow" => "\x1b[38;2;255;215;64m",
        "bright_blue" => "\x1b[38;2;68;138;255m",
        "bright_magenta" => "\x1b[38;2;224;64;251m",
        "bright_cyan" => "\x1b[38;2;24;255;255m",
        "bright_white" => "\x1b[38;2;255;255;255m",
        _ => "\x1b[38;2;245;245;245m",
    };
    code.to_string()
}

pub struct InfoLine {
    pub label: String,
    pub value: String,
    pub icon: Option<String>,
    pub color: String,
}

pub struct LogoBlock {
    pub lines: Vec<String>,
    pub width: usize,
}

pub fn terminal_size() -> (usize, usize) {
    let mut cols = 0;
    let mut rows = 0;
    if let Ok(c) = std::env::var("COLUMNS") {
        if let Ok(w) = c.parse::<usize>() {
            cols = w;
        }
    }
    if let Ok(r) = std::env::var("LINES") {
        if let Ok(h) = r.parse::<usize>() {
            rows = h;
        }
    }
    if cols > 0 && rows > 0 {
        return (cols, rows);
    }
    if let Ok((w, h)) = crossterm::terminal::size() {
        if w > 0 && cols == 0 {
            cols = w as usize;
        }
        if h > 0 && rows == 0 {
            rows = h as usize;
        }
    }
    #[cfg(unix)]
    {
        if cols == 0 || rows == 0 {
            for &fd in &[libc::STDOUT_FILENO, libc::STDERR_FILENO, libc::STDIN_FILENO] {
                let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
                if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) } == 0 {
                    if cols == 0 && ws.ws_col > 0 {
                        cols = ws.ws_col as usize;
                    }
                    if rows == 0 && ws.ws_row > 0 {
                        rows = ws.ws_row as usize;
                    }
                    if cols > 0 && rows > 0 {
                        break;
                    }
                }
            }
        }
    }
    (
        if cols > 0 { cols } else { 80 },
        if rows > 0 { rows } else { 24 },
    )
}

#[allow(dead_code)]
pub fn terminal_height() -> usize {
    terminal_size().1
}

pub fn terminal_width() -> usize {
    terminal_size().0
}

pub const LOGO_GAP: usize = 3;
const MIN_LOGO_WIDTH: usize = 50;

pub fn compute_h_pad(total_content_cols: usize, padding: usize, center: bool) -> usize {
    if center {
        terminal_width().saturating_sub(total_content_cols) / 2
    } else {
        padding
    }
}

pub fn build_color_palette(
    symbol: &str,
    block: bool,
    custom_palette: Option<&[(u8, u8, u8)]>,
) -> Vec<String> {
    let mut lines = Vec::with_capacity(2);
    if let Some(colors) = custom_palette.filter(|c| !c.is_empty()) {
        let sym = if symbol.is_empty() { "●" } else { symbol };
        let half = colors.len().min(8);
        let mut r1 = String::new();
        let mut r2 = String::new();
        for &(r, g, b) in &colors[..half] {
            if block {
                r1.push_str(&format!("\x1b[48;2;{r};{g};{b}m   "));
            } else {
                r1.push_str(&format!("\x1b[38;2;{r};{g};{b}m{sym} "));
            }
        }
        r1.push_str(RESET);
        lines.push(r1);

        if colors.len() > half {
            let second_half = &colors[half..colors.len().min(half * 2)];
            for &(r, g, b) in second_half {
                if block {
                    r2.push_str(&format!("\x1b[48;2;{r};{g};{b}m   "));
                } else {
                    r2.push_str(&format!("\x1b[38;2;{r};{g};{b}m{sym} "));
                }
            }
            r2.push_str(RESET);
            lines.push(r2);
        }
        return lines;
    }

    if block {
        let mut r1 = String::new();
        let mut r2 = String::new();
        for i in 40..=47 {
            r1.push_str(&format!("\x1b[{}m   ", i));
        }
        r1.push_str(RESET);
        for i in 100..=107 {
            r2.push_str(&format!("\x1b[{}m   ", i));
        }
        r2.push_str(RESET);
        lines.push(r1);
        lines.push(r2);
    } else {
        let sym = if symbol.is_empty() { "●" } else { symbol };
        let mut r1 = String::new();
        let mut r2 = String::new();
        for i in 30..=37 {
            r1.push_str(&format!("\x1b[{}m{} ", i, sym));
        }
        r1.push_str(RESET);
        for i in 90..=97 {
            r2.push_str(&format!("\x1b[{}m{} ", i, sym));
        }
        r2.push_str(RESET);
        lines.push(r1);
        lines.push(r2);
    }
    lines
}

#[derive(Clone, Copy)]
pub struct RenderOptions<'a> {
    pub header: Option<&'a str>,
    pub header_color: Option<&'a str>,
    pub separator: &'a str,
    pub padding: usize,
    pub center: bool,
    pub border: bool,
    pub border_style: Option<&'a str>,
    pub border_color: Option<&'a str>,
    pub border_chars: Option<&'a rustfetch_core::BorderCharsConfig>,
    pub border_title: Option<&'a str>,
    pub box_padding: usize,
    pub palette_lines: Option<&'a [String]>,
    pub palette_position: &'a str,
    pub icon_only: bool,
    pub key_type: Option<&'a str>,
    pub border_image: Option<&'a [String]>,
    pub border_image_position: &'a str,
    pub border_images: &'a [rustfetch_core::BorderImageConfig],
    pub logo_position: &'a str,
}

pub fn truncate_ansi(s: &str, max_width: usize) -> String {
    let cur_w = strip_ansi_width(s);
    if cur_w <= max_width {
        return s.to_string();
    }
    if max_width == 0 {
        return String::new();
    }
    if max_width == 1 {
        return "…".to_string();
    }

    let target_w = max_width.saturating_sub(1);
    let mut out = String::with_capacity(s.len());
    let mut current_w = 0;

    enum State {
        Ground,
        Escape,
        Csi,
        Osc,
        Apc,
        OscEsc,
    }

    let mut state = State::Ground;
    let mut has_color = false;

    for ch in s.chars() {
        match state {
            State::Ground => {
                if ch == '\x1b' {
                    state = State::Escape;
                    out.push(ch);
                    has_color = true;
                } else {
                    let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0);
                    if current_w + ch_w > target_w {
                        break;
                    }
                    out.push(ch);
                    current_w += ch_w;
                }
            }
            State::Escape => {
                out.push(ch);
                match ch {
                    '[' => state = State::Csi,
                    ']' => state = State::Osc,
                    '_' | 'P' | '^' => state = State::Apc,
                    _ => state = State::Ground,
                }
            }
            State::Csi => {
                out.push(ch);
                if ('@'..='~').contains(&ch) {
                    state = State::Ground;
                }
            }
            State::Osc | State::Apc => {
                out.push(ch);
                if ch == '\x07' {
                    state = State::Ground;
                } else if ch == '\x1b' {
                    state = State::OscEsc;
                }
            }
            State::OscEsc => {
                out.push(ch);
                if ch == '\\' {
                    state = State::Ground;
                } else if ch == '\x1b' {
                    // repeat escape
                } else {
                    state = State::Osc;
                }
            }
        }
    }

    if current_w < target_w {
        out.push_str(&" ".repeat(target_w - current_w));
    }
    out.push('…');
    if has_color {
        out.push_str(RESET);
    }
    out
}

#[allow(dead_code)]
pub fn wrap_in_box_advanced(
    lines: &[String],
    style: Option<&str>,
    color: Option<&str>,
    custom_chars: Option<&rustfetch_core::BorderCharsConfig>,
    title: Option<&str>,
    box_padding: usize,
) -> Vec<String> {
    wrap_in_box_bounded(lines, style, color, custom_chars, title, box_padding, None)
}

pub fn wrap_in_box_bounded(
    lines: &[String],
    style: Option<&str>,
    color: Option<&str>,
    custom_chars: Option<&rustfetch_core::BorderCharsConfig>,
    title: Option<&str>,
    box_padding: usize,
    max_box_width: Option<usize>,
) -> Vec<String> {
    if lines.is_empty() {
        return Vec::new();
    }

    let default_chars = rustfetch_core::BorderCharsConfig::from_style(style.unwrap_or("rounded"));
    let chars = custom_chars.unwrap_or(&default_chars);

    let border_color_code = color.map(color_code).unwrap_or_default();
    let reset_border = if color.is_some() { RESET } else { "" };

    let content_lines: Vec<&String> = lines
        .iter()
        .filter(|l| !l.starts_with("---DIVIDER"))
        .collect();

    let mut max_w = content_lines
        .iter()
        .map(|l| strip_ansi_width(l))
        .max()
        .unwrap_or(0);

    if let Some(t) = title.filter(|t| !t.is_empty()) {
        max_w = max_w.max(UnicodeWidthStr::width(strip_ansi(t).as_str()) + 4);
    }
    for line in lines {
        if line.starts_with("---DIVIDER:") {
            let section_title = line
                .strip_prefix("---DIVIDER:")
                .unwrap_or("")
                .trim_end_matches("---");
            let st_w = UnicodeWidthStr::width(strip_ansi(section_title).as_str());
            max_w = max_w.max(st_w + 4);
        }
    }

    let mut pad_len = box_padding.max(1);

    if let Some(mbw) = max_box_width {
        if mbw < 15 {
            pad_len = 0;
        } else if mbw < 35 && pad_len > 1 {
            pad_len = 1;
        }
        let border_overhead = 2 + (pad_len * 2);
        let avail_content_w = mbw.saturating_sub(border_overhead);
        if max_w > avail_content_w {
            max_w = avail_content_w;
        }
    }

    let pad_str = " ".repeat(pad_len);
    let inner_w = max_w + (pad_len * 2);

    let mut boxed = Vec::with_capacity(lines.len() + 2);

    // Top border line
    if let Some(t) = title.filter(|t| !t.is_empty()) {
        let title_clean = strip_ansi(t);
        let title_w = UnicodeWidthStr::width(title_clean.as_str());
        let top_rep = chars.top.as_str();
        if inner_w > title_w + 4 {
            let remain = inner_w - (title_w + 4);
            boxed.push(format!(
                "{border_color_code}{tl}{top}{top} {BOLD}{t}{RESET}{border_color_code} {rem}{tr}{reset_border}",
                tl = chars.top_left,
                top = top_rep,
                rem = top_rep.repeat(remain),
                tr = chars.top_right
            ));
        } else if inner_w > 4 {
            let avail_t = inner_w.saturating_sub(4);
            let trunc_t = truncate_ansi(t, avail_t);
            let trunc_w = strip_ansi_width(&trunc_t);
            let remain = inner_w.saturating_sub(trunc_w + 4);
            boxed.push(format!(
                "{border_color_code}{tl}{top}{top} {BOLD}{trunc_t}{RESET}{border_color_code} {rem}{tr}{reset_border}",
                tl = chars.top_left,
                top = top_rep,
                rem = top_rep.repeat(remain),
                tr = chars.top_right
            ));
        } else {
            boxed.push(format!(
                "{border_color_code}{tl}{rep}{tr}{reset_border}",
                tl = chars.top_left,
                rep = chars.top.repeat(inner_w),
                tr = chars.top_right
            ));
        }
    } else {
        boxed.push(format!(
            "{border_color_code}{tl}{rep}{tr}{reset_border}",
            tl = chars.top_left,
            rep = chars.top.repeat(inner_w),
            tr = chars.top_right
        ));
    }

    // Inner lines
    for line in lines {
        if line.starts_with("---DIVIDER:") {
            let section_title = line
                .strip_prefix("---DIVIDER:")
                .unwrap_or("")
                .trim_end_matches("---");
            let st_clean = strip_ansi(section_title);
            let st_w = UnicodeWidthStr::width(st_clean.as_str());
            let div_rep = chars.divider.as_str();
            if inner_w > st_w + 2 {
                let remain = inner_w - (st_w + 2);
                let left_side = remain / 2;
                let right_side = remain - left_side;
                boxed.push(format!(
                    "{border_color_code}{dl}{left_r} {BOLD}{section_title}{RESET}{border_color_code} {right_r}{dr}{reset_border}",
                    dl = chars.divider_left,
                    left_r = div_rep.repeat(left_side),
                    right_r = div_rep.repeat(right_side),
                    dr = chars.divider_right
                ));
            } else if inner_w > 4 {
                let trunc_st = truncate_ansi(section_title, inner_w.saturating_sub(2));
                let trunc_w = strip_ansi_width(&trunc_st);
                let remain = inner_w.saturating_sub(trunc_w + 2);
                let left_side = remain / 2;
                let right_side = remain - left_side;
                boxed.push(format!(
                    "{border_color_code}{dl}{left_r} {BOLD}{trunc_st}{RESET}{border_color_code} {right_r}{dr}{reset_border}",
                    dl = chars.divider_left,
                    left_r = div_rep.repeat(left_side),
                    right_r = div_rep.repeat(right_side),
                    dr = chars.divider_right
                ));
            } else {
                boxed.push(format!(
                    "{border_color_code}{dl}{rep}{dr}{reset_border}",
                    dl = chars.divider_left,
                    rep = chars.divider.repeat(inner_w),
                    dr = chars.divider_right
                ));
            }
        } else if line == "---DIVIDER---" {
            boxed.push(format!(
                "{border_color_code}{dl}{rep}{dr}{reset_border}",
                dl = chars.divider_left,
                rep = chars.divider.repeat(inner_w),
                dr = chars.divider_right
            ));
        } else {
            let processed_line = if strip_ansi_width(line) > max_w {
                truncate_ansi(line, max_w)
            } else {
                line.clone()
            };
            let visual_w = strip_ansi_width(&processed_line);
            let line_pad = " ".repeat(max_w.saturating_sub(visual_w));
            boxed.push(format!(
                "{border_color_code}{l}{reset_border}{pad_str}{processed_line}{line_pad}{pad_str}{border_color_code}{r}{reset_border}",
                l = chars.left,
                r = chars.right
            ));
        }
    }

    // Bottom border line
    boxed.push(format!(
        "{border_color_code}{bl}{rep}{br}{reset_border}",
        bl = chars.bottom_left,
        rep = chars.bottom.repeat(inner_w),
        br = chars.bottom_right
    ));

    boxed
}

pub fn build_right_lines(info_lines: &[InfoLine], opts: &RenderOptions) -> Vec<String> {
    build_right_lines_bounded(info_lines, opts, None)
}

pub fn build_right_lines_bounded(
    info_lines: &[InfoLine],
    opts: &RenderOptions,
    max_card_width: Option<usize>,
) -> Vec<String> {
    let mut right_lines: Vec<String> = Vec::new();

    if opts.palette_position == "top" || opts.palette_position == "both" {
        if let Some(pal) = opts.palette_lines {
            right_lines.extend(pal.iter().cloned());
            right_lines.push(String::new());
        }
    }

    if let Some(h) = opts.header {
        let col = opts.header_color.unwrap_or("cyan");
        let h_str = if let Some(mcw) = max_card_width {
            let max_h = if opts.border {
                mcw.saturating_sub(opts.box_padding.max(1) * 2 + 2)
            } else {
                mcw
            };
            if UnicodeWidthStr::width(h) > max_h && max_h > 0 {
                truncate_ansi(h, max_h)
            } else {
                h.to_string()
            }
        } else {
            h.to_string()
        };
        let h_w = strip_ansi_width(&h_str);
        right_lines.push(format!("{BOLD}{}{h_str}{RESET}", color_code(col)));
        right_lines.push(format!("{DIM}{}{RESET}", "─".repeat(h_w)));
    }

    right_lines.extend(format_info_lines(
        info_lines,
        opts.separator,
        opts.icon_only,
        opts.key_type,
    ));

    if opts.palette_position == "bottom" || opts.palette_position == "both" {
        if let Some(pal) = opts.palette_lines {
            right_lines.push(String::new());
            right_lines.extend(pal.iter().cloned());
        }
    }

    if opts.border {
        right_lines = wrap_in_box_bounded(
            &right_lines,
            opts.border_style,
            opts.border_color,
            opts.border_chars,
            opts.border_title,
            opts.box_padding,
            max_card_width,
        );
    } else {
        let mut content_w = right_lines
            .iter()
            .filter(|l| !l.starts_with("---DIVIDER"))
            .map(|l| strip_ansi_width(l))
            .max()
            .unwrap_or(24)
            .max(20);

        if let Some(mcw) = max_card_width {
            content_w = content_w.min(mcw);
        }

        right_lines = right_lines
            .into_iter()
            .map(|line| {
                if line.starts_with("---DIVIDER:") {
                    let section_title = line
                        .strip_prefix("---DIVIDER:")
                        .unwrap_or("")
                        .trim_end_matches("---");
                    let st_clean = strip_ansi(section_title);
                    let st_w = UnicodeWidthStr::width(st_clean.as_str());
                    if st_w > 0 && content_w > st_w + 4 {
                        let remain = content_w.saturating_sub(st_w + 4);
                        format!(
                            "{DIM}── {RESET}{BOLD}{section_title}{RESET} {DIM}{}{RESET}",
                            "─".repeat(remain)
                        )
                    } else if st_w > 0 {
                        format!("{BOLD}{section_title}{RESET}")
                    } else {
                        String::new()
                    }
                } else if line == "---DIVIDER---" {
                    String::new()
                } else {
                    if let Some(mcw) = max_card_width {
                        if strip_ansi_width(&line) > mcw {
                            truncate_ansi(&line, mcw)
                        } else {
                            line
                        }
                    } else {
                        line
                    }
                }
            })
            .collect();
    }

    if !opts.border_images.is_empty() {
        // Compose non-background border images first (frames, left, right, top, bottom)
        for img in opts.border_images {
            if !img.is_background() {
                let expanded = shellexpand::tilde(&img.path);
                let target_w = img.width.unwrap_or(24);
                if let Some(rasterized) = rustfetch_render::render_halfblock_ex(
                    expanded.as_ref(),
                    target_w,
                    img.height,
                    img.preserve_aspect_ratio,
                ) {
                    right_lines =
                        compose_with_border_image(&right_lines, &rasterized, &img.position);
                }
            }
        }

        // For background images, if not using Kitty (e.g. halfblock/fallback), apply TrueColor ANSI background
        for img in opts.border_images {
            if img.is_background() && !should_use_kitty(img) {
                let expanded = shellexpand::tilde(&img.path);
                let card_w = right_lines
                    .iter()
                    .map(|l| strip_ansi_width(l))
                    .max()
                    .unwrap_or(0);
                let card_h = right_lines.len();
                let target_cols = img.width.unwrap_or(card_w).max(1);
                let target_rows = img.height.unwrap_or(card_h).max(1);
                if let Some(bg_applied) = rustfetch_render::apply_image_background_to_lines_ex(
                    &right_lines,
                    expanded.as_ref(),
                    target_cols,
                    target_rows,
                    img.preserve_aspect_ratio,
                    img.offset_x,
                    img.offset_y,
                ) {
                    right_lines = bg_applied;
                }
            }
        }
    } else if let Some(border_img) = opts.border_image.filter(|b| !b.is_empty()) {
        return compose_with_border_image(&right_lines, border_img, opts.border_image_position);
    }

    right_lines
}

pub fn should_use_kitty(img_config: &rustfetch_core::BorderImageConfig) -> bool {
    let proto = img_config.protocol.to_ascii_lowercase();
    proto == "kitty" || (proto == "auto" && rustfetch_render::supports_kitty())
}

pub fn emit_kitty_background(
    img_config: &rustfetch_core::BorderImageConfig,
    col_offset: usize,
    card_w: usize,
    card_h: usize,
) -> Option<usize> {
    let expanded = shellexpand::tilde(&img_config.path).to_string();
    let z_index = img_config.effective_z_index();

    let opts = rustfetch_render::BackgroundOptions {
        col_offset,
        card_w,
        card_h,
        target_cols: img_config.width,
        target_rows: img_config.height,
        offset_x: img_config.offset_x,
        offset_y: img_config.offset_y,
        z_index,
        preserve_aspect_ratio: img_config.preserve_aspect_ratio,
    };

    rustfetch_render::render_background_ex(&expanded, &opts)
}

pub fn emit_all_kitty_backgrounds(
    border_images: &[rustfetch_core::BorderImageConfig],
    col_offset: usize,
    card_w: usize,
    card_h: usize,
) -> usize {
    let mut max_lines = card_h;
    for img in border_images {
        if img.is_background() && should_use_kitty(img) {
            if let Some(bottom_row) = emit_kitty_background(img, col_offset, card_w, card_h) {
                max_lines = max_lines.max(bottom_row);
            }
        }
    }
    max_lines
}

/// Places a rasterized decoration around the telemetry card. `left` preserves
/// the original layout; `right`, `top`, `bottom`, and `frame` support the
/// common ricing compositions without requiring duplicate image files.
pub fn compose_with_border_image(
    content: &[String],
    border_image: &[String],
    position: &str,
) -> Vec<String> {
    match position.to_ascii_lowercase().as_str() {
        "top" => {
            let mut combined = Vec::with_capacity(border_image.len() + content.len());
            combined.extend(border_image.iter().cloned());
            combined.extend(content.iter().cloned());
            combined
        }
        "bottom" => {
            let mut combined = Vec::with_capacity(border_image.len() + content.len());
            combined.extend(content.iter().cloned());
            combined.extend(border_image.iter().cloned());
            combined
        }
        "right" => compose_image_columns(content, border_image, false),
        "frame" | "both" => compose_image_frame(content, border_image),
        "left" => compose_image_columns(content, border_image, true),
        "background" | "behind" | "bg" => content.to_vec(),
        _ => compose_image_columns(content, border_image, true),
    }
}

fn compose_image_columns(
    content: &[String],
    border_image: &[String],
    image_on_left: bool,
) -> Vec<String> {
    let image_width = border_image
        .iter()
        .map(|line| strip_ansi_width(line))
        .max()
        .unwrap_or(0);
    let content_width = content
        .iter()
        .map(|line| strip_ansi_width(line))
        .max()
        .unwrap_or(0);
    let total_rows = border_image.len().max(content.len());
    let mut combined = Vec::with_capacity(total_rows);

    for index in 0..total_rows {
        let image = border_image.get(index).map(String::as_str).unwrap_or("");
        let content_line = content.get(index).map(String::as_str).unwrap_or("");
        let image_pad = " ".repeat(image_width.saturating_sub(strip_ansi_width(image)));
        let content_pad = " ".repeat(content_width.saturating_sub(strip_ansi_width(content_line)));
        if image_on_left {
            combined.push(format!("{image}{image_pad}   {content_line}"));
        } else {
            combined.push(format!("{content_line}{content_pad}   {image}"));
        }
    }
    combined
}

fn compose_image_frame(content: &[String], border_image: &[String]) -> Vec<String> {
    let image_width = border_image
        .iter()
        .map(|line| strip_ansi_width(line))
        .max()
        .unwrap_or(0);
    let content_width = content
        .iter()
        .map(|line| strip_ansi_width(line))
        .max()
        .unwrap_or(0);
    let total_rows = border_image.len().max(content.len());
    let mut combined = Vec::with_capacity(total_rows);

    for index in 0..total_rows {
        let image = border_image.get(index).map(String::as_str).unwrap_or("");
        let content_line = content.get(index).map(String::as_str).unwrap_or("");
        let image_pad = " ".repeat(image_width.saturating_sub(strip_ansi_width(image)));
        let content_pad = " ".repeat(content_width.saturating_sub(strip_ansi_width(content_line)));
        combined.push(format!(
            "{image}{image_pad}   {content_line}{content_pad}   {image}"
        ));
    }
    combined
}

pub fn render(logo: Option<&LogoBlock>, info_lines: &[InfoLine], opts: &RenderOptions) {
    let term_w = terminal_width();
    let pad = " ".repeat(opts.padding);

    if opts.logo_position == "none" || opts.logo_position == "hidden" || term_w < 35 {
        render_without_logo(info_lines, &pad, term_w, opts);
        return;
    }

    if let Some(logo) = logo {
        let is_vertical = opts.logo_position == "top" || opts.logo_position == "bottom";
        let needed_side_by_side = logo.width + LOGO_GAP + 34 + opts.padding * 2;
        if is_vertical {
            if term_w >= logo.width.min(MIN_LOGO_WIDTH) || term_w >= 35 {
                render_with_logo(logo, info_lines, &pad, term_w, opts);
            } else {
                render_without_logo(info_lines, &pad, term_w, opts);
            }
        } else if term_w >= needed_side_by_side {
            render_with_logo(logo, info_lines, &pad, term_w, opts);
        } else if term_w >= 45 {
            let mut top_opts = *opts;
            top_opts.logo_position = "top";
            render_with_logo(logo, info_lines, &pad, term_w, &top_opts);
        } else {
            render_without_logo(info_lines, &pad, term_w, opts);
        }
    } else {
        render_without_logo(info_lines, &pad, term_w, opts);
    }
}

pub fn format_info_lines(
    info_lines: &[InfoLine],
    separator: &str,
    icon_only: bool,
    key_type: Option<&str>,
) -> Vec<String> {
    let show_icon = key_type != Some("title");
    let show_label = !icon_only && key_type != Some("icon");

    let label_width = if show_label {
        info_lines
            .iter()
            .filter(|l| {
                !l.label.is_empty() && !l.value.is_empty() && !l.label.starts_with("---DIVIDER")
            })
            .map(|l| {
                let icon_w = if show_icon {
                    l.icon
                        .as_deref()
                        .map(|i| UnicodeWidthStr::width(i) + 1)
                        .unwrap_or(0)
                } else {
                    0
                };
                UnicodeWidthStr::width(l.label.as_str()) + icon_w
            })
            .max()
            .unwrap_or(0)
    } else {
        0
    };

    let sep_trimmed = separator.trim();
    let is_sep_empty = sep_trimmed.is_empty();

    info_lines
        .iter()
        .map(|line| {
            if line.label.starts_with("---DIVIDER") {
                return line.label.clone();
            }

            if line.label.is_empty() && line.icon.is_none() {
                if line.value.is_empty() {
                    return String::new();
                } else {
                    return format!("{}{}{}", line.color, line.value, RESET);
                }
            } else if line.value.is_empty() {
                let icon_str = if show_icon {
                    line.icon
                        .as_deref()
                        .map(|i| format!("{i} "))
                        .unwrap_or_default()
                } else {
                    String::new()
                };
                return format!("{BOLD}{}{icon_str}{}{RESET}", line.color, line.label);
            }

            let icon_str = if show_icon {
                line.icon
                    .as_deref()
                    .map(|i| format!("{i} "))
                    .unwrap_or_default()
            } else {
                String::new()
            };

            if !show_label || line.label.is_empty() {
                let icon_display = if show_icon && line.icon.is_some() {
                    format!(
                        "{BOLD}{}{}{RESET}",
                        line.color,
                        line.icon.as_deref().unwrap_or("")
                    )
                } else {
                    String::new()
                };

                if is_sep_empty {
                    if icon_display.is_empty() {
                        line.value.clone()
                    } else {
                        format!("{icon_display}  {}", line.value)
                    }
                } else if icon_display.is_empty() {
                    line.value.clone()
                } else {
                    format!("{icon_display} {sep_trimmed} {}", line.value)
                }
            } else {
                let icon_w = if show_icon {
                    line.icon
                        .as_deref()
                        .map(|i| UnicodeWidthStr::width(i) + 1)
                        .unwrap_or(0)
                } else {
                    0
                };
                let cur_label_w = UnicodeWidthStr::width(line.label.as_str()) + icon_w;
                let label_pad = " ".repeat(label_width.saturating_sub(cur_label_w));

                if is_sep_empty {
                    format!(
                        "{BOLD}{color}{icon_str}{label}{RESET}{label_pad}  {value}",
                        color = line.color,
                        label = line.label,
                        value = line.value
                    )
                } else {
                    format!(
                        "{BOLD}{color}{icon_str}{label}{RESET}{label_pad} {sep_trimmed} {value}",
                        color = line.color,
                        label = line.label,
                        value = line.value
                    )
                }
            }
        })
        .collect()
}

pub fn render_with_image(
    image_lines: usize,
    image_cols: usize,
    h_pad: usize,
    info_lines: &[InfoLine],
    opts: &RenderOptions,
) {
    let term_w = terminal_width();
    let is_vertical = opts.logo_position == "top" || opts.logo_position == "bottom";
    let max_card_w = if is_vertical {
        term_w.saturating_sub(h_pad * 2 + 1)
    } else {
        term_w.saturating_sub(h_pad + image_cols + LOGO_GAP + 1)
    };
    let mut right_lines = build_right_lines_bounded(info_lines, opts, Some(max_card_w));

    if opts.logo_position == "top" {
        let pad_str = " ".repeat(h_pad);
        let max_w = right_lines
            .iter()
            .map(|l| strip_ansi_width(l))
            .max()
            .unwrap_or(0);
        let bg_lines =
            emit_all_kitty_backgrounds(opts.border_images, h_pad, max_w, right_lines.len());
        println!();
        for line in &right_lines {
            println!("{pad_str}{line}");
        }
        for _ in right_lines.len()..bg_lines {
            println!();
        }
        return;
    }

    if opts.center && image_lines > right_lines.len() {
        let v_offset = (image_lines - right_lines.len()) / 2;
        let mut padded = Vec::with_capacity(image_lines);
        padded.resize(v_offset, String::new());
        padded.extend(right_lines);
        right_lines = padded;
    }

    if image_lines > 1 {
        print!("\r\x1b[{}A", image_lines - 1);
    } else {
        print!("\r");
    }

    let right_indent = h_pad + image_cols + LOGO_GAP;

    let card_w = right_lines
        .iter()
        .map(|l| strip_ansi_width(l))
        .max()
        .unwrap_or(0);
    let card_h = right_lines.len();
    let bg_lines = emit_all_kitty_backgrounds(opts.border_images, right_indent, card_w, card_h);
    let total_lines = image_lines.max(right_lines.len()).max(bg_lines);

    for i in 0..total_lines {
        if i < right_lines.len() && !right_lines[i].is_empty() {
            print!("\x1b[{}C{}\r\n", right_indent, right_lines[i]);
        } else {
            print!("\r\n");
        }
    }
}

fn render_with_logo(
    logo: &LogoBlock,
    info_lines: &[InfoLine],
    pad: &str,
    term_w: usize,
    opts: &RenderOptions,
) {
    let is_vertical = opts.logo_position == "top" || opts.logo_position == "bottom";
    let avail_card_w = if is_vertical {
        term_w.saturating_sub(pad.len() * 2 + 1)
    } else {
        term_w.saturating_sub(pad.len() + logo.width + LOGO_GAP + 1)
    };
    let mut right_lines = build_right_lines_bounded(info_lines, opts, Some(avail_card_w));
    let disable_colors = !colors_enabled();

    if opts.logo_position == "top" {
        let logo_pad = if opts.center && term_w > logo.width {
            " ".repeat((term_w.saturating_sub(logo.width)) / 2)
        } else {
            pad.to_string()
        };
        for line in &logo.lines {
            let clean = if disable_colors {
                strip_ansi(line)
            } else {
                line.clone()
            };
            println!("{logo_pad}{clean}");
        }
        let max_w = right_lines
            .iter()
            .map(|l| strip_ansi_width(l))
            .max()
            .unwrap_or(0);
        let modules_pad = if opts.center && term_w > max_w {
            " ".repeat((term_w.saturating_sub(max_w)) / 2)
        } else {
            pad.to_string()
        };
        let bg_lines = emit_all_kitty_backgrounds(
            opts.border_images,
            modules_pad.len(),
            max_w,
            right_lines.len(),
        );
        println!();
        for line in &right_lines {
            let clean = if disable_colors {
                strip_ansi(line)
            } else {
                line.clone()
            };
            println!("{modules_pad}{clean}");
        }
        for _ in right_lines.len()..bg_lines {
            println!();
        }
        return;
    }

    if opts.logo_position == "bottom" {
        let max_w = right_lines
            .iter()
            .map(|l| strip_ansi_width(l))
            .max()
            .unwrap_or(0);
        let modules_pad = if opts.center && term_w > max_w {
            " ".repeat((term_w.saturating_sub(max_w)) / 2)
        } else {
            pad.to_string()
        };
        let bg_lines = emit_all_kitty_backgrounds(
            opts.border_images,
            modules_pad.len(),
            max_w,
            right_lines.len(),
        );
        for line in &right_lines {
            let clean = if disable_colors {
                strip_ansi(line)
            } else {
                line.clone()
            };
            println!("{modules_pad}{clean}");
        }
        for _ in right_lines.len()..bg_lines {
            println!();
        }
        println!();
        let logo_pad = if opts.center && term_w > logo.width {
            " ".repeat((term_w.saturating_sub(logo.width)) / 2)
        } else {
            pad.to_string()
        };
        for line in &logo.lines {
            let clean = if disable_colors {
                strip_ansi(line)
            } else {
                line.clone()
            };
            println!("{logo_pad}{clean}");
        }
        return;
    }

    if opts.logo_position == "right" {
        if opts.center && logo.lines.len() > right_lines.len() {
            let v_offset = (logo.lines.len() - right_lines.len()) / 2;
            let mut padded = Vec::with_capacity(logo.lines.len());
            padded.resize(v_offset, String::new());
            padded.extend(right_lines);
            right_lines = padded;
        }

        let max_right_w = right_lines
            .iter()
            .map(|l| strip_ansi_width(l))
            .max()
            .unwrap_or(0);
        let total_w = max_right_w + LOGO_GAP + logo.width;
        let h_pad_str = if opts.center && term_w > total_w {
            " ".repeat((term_w.saturating_sub(total_w)) / 2)
        } else {
            pad.to_string()
        };

        let bg_lines = emit_all_kitty_backgrounds(
            opts.border_images,
            h_pad_str.len(),
            max_right_w,
            right_lines.len(),
        );

        let total_lines = logo.lines.len().max(right_lines.len()).max(bg_lines);
        let logo_v_offset = if right_lines.len() > logo.lines.len() {
            (right_lines.len() - logo.lines.len()) / 2
        } else {
            0
        };
        let gap = " ".repeat(LOGO_GAP);

        for i in 0..total_lines {
            let right_line = right_lines.get(i).map(|s| s.as_str()).unwrap_or("");
            let right_clean = if disable_colors {
                strip_ansi(right_line)
            } else {
                right_line.to_string()
            };
            let right_w = strip_ansi_width(&right_clean);
            let right_trailing = " ".repeat(max_right_w.saturating_sub(right_w));

            let logo_idx = if i >= logo_v_offset && i < logo_v_offset + logo.lines.len() {
                Some(i - logo_v_offset)
            } else {
                None
            };
            if let Some(l_str) = logo_idx.and_then(|idx| logo.lines.get(idx)) {
                let logo_to_print = if disable_colors {
                    strip_ansi(l_str)
                } else {
                    l_str.to_string()
                };
                print!("{h_pad_str}{right_clean}{right_trailing}{gap}{logo_to_print}\r\n");
            } else {
                print!("{h_pad_str}{right_clean}\r\n");
            }
        }
        return;
    }

    // Default "left" position
    if opts.center && logo.lines.len() > right_lines.len() {
        let v_offset = (logo.lines.len() - right_lines.len()) / 2;
        let mut padded = Vec::with_capacity(logo.lines.len());
        padded.resize(v_offset, String::new());
        padded.extend(right_lines);
        right_lines = padded;
    }

    let max_right_w = right_lines
        .iter()
        .map(|l| strip_ansi_width(l))
        .max()
        .unwrap_or(0);
    let total_w = logo.width + LOGO_GAP + max_right_w;
    let h_pad_str = if opts.center && term_w > total_w {
        " ".repeat((term_w - total_w) / 2)
    } else {
        pad.to_string()
    };

    let col_offset = h_pad_str.len() + logo.width + LOGO_GAP;
    let bg_lines = emit_all_kitty_backgrounds(
        opts.border_images,
        col_offset,
        max_right_w,
        right_lines.len(),
    );

    let total_lines = logo.lines.len().max(right_lines.len()).max(bg_lines);
    let logo_v_offset = if right_lines.len() > logo.lines.len() {
        (right_lines.len() - logo.lines.len()) / 2
    } else {
        0
    };
    let logo_pad_str = " ".repeat(logo.width);
    let gap = " ".repeat(LOGO_GAP);

    for i in 0..total_lines {
        let right_line = right_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let right_clean = if disable_colors {
            strip_ansi(right_line)
        } else {
            right_line.to_string()
        };
        let logo_idx = if i >= logo_v_offset && i < logo_v_offset + logo.lines.len() {
            Some(i - logo_v_offset)
        } else {
            None
        };
        if let Some(l_str) = logo_idx.and_then(|idx| logo.lines.get(idx)) {
            let logo_to_print = if disable_colors {
                strip_ansi(l_str)
            } else {
                l_str.to_string()
            };
            let logo_visual_w = strip_ansi_width(&logo_to_print);
            let logo_trailing = " ".repeat(logo.width.saturating_sub(logo_visual_w));
            print!("{h_pad_str}{logo_to_print}{logo_trailing}{gap}{right_clean}\r\n");
        } else {
            print!("{h_pad_str}{logo_pad_str}{gap}{right_clean}\r\n");
        }
    }
}

fn render_without_logo(info_lines: &[InfoLine], pad: &str, term_w: usize, opts: &RenderOptions) {
    let avail_card_w = term_w.saturating_sub(pad.len() * 2 + 1);
    let right_lines = build_right_lines_bounded(info_lines, opts, Some(avail_card_w));

    let max_w = right_lines
        .iter()
        .map(|l| strip_ansi_width(l))
        .max()
        .unwrap_or(0);
    let h_pad_str = if opts.center && term_w > max_w {
        " ".repeat((term_w - max_w) / 2)
    } else {
        pad.to_string()
    };

    let col_offset = h_pad_str.len();
    let bg_lines =
        emit_all_kitty_backgrounds(opts.border_images, col_offset, max_w, right_lines.len());
    let total_lines = right_lines.len().max(bg_lines);

    let disable_colors = !colors_enabled();
    for i in 0..total_lines {
        if i < right_lines.len() {
            let display_line = if disable_colors {
                strip_ansi(&right_lines[i])
            } else {
                right_lines[i].clone()
            };
            print!("{h_pad_str}{display_line}\r\n");
        } else {
            print!("\r\n");
        }
    }
}

pub fn strip_ansi(s: &str) -> String {
    enum State {
        Ground,
        Escape,
        Csi,
        Osc,
        Apc,
        OscEsc,
    }

    let mut state = State::Ground;
    let mut clean = String::with_capacity(s.len());

    for ch in s.chars() {
        match state {
            State::Ground => {
                if ch == '\x1b' {
                    state = State::Escape;
                } else {
                    clean.push(ch);
                }
            }
            State::Escape => {
                match ch {
                    '[' => state = State::Csi,
                    ']' => state = State::Osc,
                    '_' | 'P' | '^' => state = State::Apc,
                    _ => {
                        // 2-byte escape sequence (e.g. \x1b7, \x1b8, \x1bM, \x1bc)
                        state = State::Ground;
                    }
                }
            }
            State::Csi => {
                if ('@'..='~').contains(&ch) {
                    state = State::Ground;
                }
            }
            State::Osc | State::Apc => {
                if ch == '\x07' {
                    state = State::Ground;
                } else if ch == '\x1b' {
                    state = State::OscEsc;
                }
            }
            State::OscEsc => {
                if ch == '\\' {
                    state = State::Ground;
                } else if ch == '\x1b' {
                    // repeat escape
                } else {
                    state = State::Osc;
                }
            }
        }
    }
    clean
}

pub fn strip_ansi_width(s: &str) -> usize {
    UnicodeWidthStr::width(strip_ansi(s).as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_code() {
        assert_eq!(color_code("red"), "\x1b[38;2;239;83;80m");
    }

    #[test]
    fn test_colors_enabled() {
        std::env::set_var("FORCE_COLOR", "1");
        assert!(colors_enabled());
        std::env::remove_var("FORCE_COLOR");
    }

    #[test]
    fn test_strip_ansi_width() {
        assert_eq!(strip_ansi_width("\x1b[31mhello\x1b[0m"), 5);
        assert_eq!(strip_ansi_width("\x1b7hello\x1b8"), 5);
        assert_eq!(strip_ansi_width("\x1b_Ga=T,f=100;ABCDEF\x1b\\world"), 5);
        assert_eq!(
            strip_ansi_width("\x1b]8;;https://example.com\x07link\x1b]8;;\x07"),
            4
        );
    }

    #[test]
    fn test_terminal_width() {
        assert!(terminal_width() > 0);
    }

    #[test]
    fn test_terminal_size() {
        let (w, h) = terminal_size();
        assert!(w > 0);
        assert!(h > 0);
    }

    #[test]
    fn test_build_color_palette() {
        let dots = build_color_palette("●", false, None);
        assert_eq!(dots.len(), 2);
        assert!(dots[0].contains("●"));

        let blocks = build_color_palette("", true, None);
        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].contains("\x1b[40m"));

        let custom = vec![(255, 100, 50), (100, 200, 255)];
        let custom_dots = build_color_palette("●", false, Some(&custom));
        assert_eq!(custom_dots.len(), 1);
        assert!(custom_dots[0].contains("\x1b[38;2;255;100;50m"));
    }

    #[test]
    fn test_format_custom_and_break_lines() {
        let lines = vec![
            InfoLine {
                label: "OS".into(),
                value: "Linux".into(),
                icon: Some("".into()),
                color: "\x1b[34m".into(),
            },
            InfoLine {
                label: "".into(),
                value: "".into(),
                icon: None,
                color: "".into(),
            },
            InfoLine {
                label: "Custom".into(),
                value: "Val".into(),
                icon: None,
                color: "\x1b[32m".into(),
            },
            InfoLine {
                label: "".into(),
                value: "Raw Text".into(),
                icon: None,
                color: "\x1b[33m".into(),
            },
        ];
        let formatted = format_info_lines(&lines, ":", false, None);
        assert_eq!(formatted.len(), 4);
        assert!(formatted[0].contains("OS"));
        assert!(formatted[1].is_empty()); // break line is empty
        assert!(formatted[2].contains("Custom"));
        assert!(formatted[3].contains("Raw Text"));

        // Test icon_only mode
        let icon_only_formatted = format_info_lines(&lines, "•", true, None);
        assert!(icon_only_formatted[0].contains("•"));
        assert!(!icon_only_formatted[0].contains("OS")); // label omitted
        assert!(icon_only_formatted[0].contains("Linux"));
    }

    #[test]
    fn test_wrap_in_box() {
        let lines = vec!["Hello".to_string(), "World!".to_string()];
        let boxed = wrap_in_box_advanced(&lines, None, None, None, None, 1);
        assert_eq!(boxed.len(), 4);
        assert!(boxed[0].starts_with('╭'));
        assert!(boxed[0].ends_with('╮'));
        assert!(boxed[1].contains("Hello"));
        assert!(boxed[2].contains("World!"));
        assert!(boxed[3].starts_with('╰'));
        assert!(boxed[3].ends_with('╯'));
    }

    #[test]
    fn test_wrap_in_box_advanced_with_title_and_divider() {
        let lines = vec![
            "---DIVIDER:System---".to_string(),
            "OS: Linux".to_string(),
            "---DIVIDER---".to_string(),
            "CPU: 8 cores".to_string(),
        ];
        let boxed =
            wrap_in_box_advanced(&lines, Some("double"), Some("cyan"), None, Some("Specs"), 1);
        assert_eq!(boxed.len(), 6);
        assert!(boxed[0].contains("Specs"));
        assert!(boxed[0].contains('╔'));
        assert!(boxed[1].contains("System"));
        assert!(boxed[1].contains('╠'));
        assert!(boxed[2].contains("OS: Linux"));
        assert!(boxed[3].contains('╠'));
        assert!(boxed[4].contains("CPU: 8 cores"));
        assert!(boxed[5].contains('╚'));

        let w0 = strip_ansi_width(&boxed[0]);
        for (idx, b_line) in boxed.iter().enumerate() {
            assert_eq!(strip_ansi_width(b_line), w0, "Line {} width mismatch", idx);
        }
    }

    #[test]
    fn test_build_right_lines_header_color() {
        let info = vec![InfoLine {
            label: "OS".into(),
            value: "Linux".into(),
            icon: None,
            color: "\x1b[34m".into(),
        }];
        let opts = RenderOptions {
            header: Some("user@host"),
            header_color: Some("#ff007f"),
            separator: ":",
            padding: 1,
            center: false,
            border: false,
            border_style: None,
            border_color: None,
            border_chars: None,
            border_title: None,
            box_padding: 1,
            palette_lines: None,
            palette_position: "bottom",
            icon_only: false,
            key_type: None,
            border_image: None,
            border_image_position: "left",
            border_images: &[],
            logo_position: "left",
        };
        let lines = build_right_lines(&info, &opts);
        assert_eq!(lines.len(), 3);
        // Header line must contain the 24-bit TrueColor for #ff007f (255, 0, 127)
        assert!(lines[0].contains("\x1b[38;2;255;0;127m"));
        assert!(lines[0].contains("user@host"));
    }

    #[test]
    fn test_build_right_lines_with_border_image() {
        let info = vec![InfoLine {
            icon: None,
            label: "OS".to_string(),
            value: "Linux".to_string(),
            color: "".to_string(),
        }];
        let border_lines = vec!["[IMG_TOP]".to_string(), "[IMG_BOT]".to_string()];
        let opts = RenderOptions {
            header: None,
            header_color: None,
            separator: ": ",
            padding: 1,
            center: false,
            border: false,
            border_style: None,
            border_color: None,
            border_chars: None,
            border_title: None,
            box_padding: 1,
            palette_lines: None,
            palette_position: "bottom",
            icon_only: false,
            key_type: None,
            border_image: Some(&border_lines),
            border_image_position: "left",
            border_images: &[],
            logo_position: "left",
        };
        let lines = build_right_lines(&info, &opts);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("[IMG_TOP]"));
        let clean0 = strip_ansi(&lines[0]);
        assert!(clean0.contains("OS"));
        assert!(clean0.contains("Linux"));
        assert!(lines[1].starts_with("[IMG_BOT]"));
    }

    #[test]
    fn test_compose_with_border_image_positions() {
        let content = vec!["CPU: 8 cores".to_string(), "RAM: 16 GiB".to_string()];
        let image = vec!["<top>".to_string(), "<bottom>".to_string()];

        let left = compose_with_border_image(&content, &image, "left");
        assert!(left[0].starts_with("<top>"));
        assert!(left[0].contains("CPU: 8 cores"));

        let right = compose_with_border_image(&content, &image, "right");
        assert!(right[0].starts_with("CPU: 8 cores"));
        assert!(right[0].ends_with("<top>"));

        let frame = compose_with_border_image(&content, &image, "frame");
        assert!(frame[0].starts_with("<top>"));
        assert!(frame[0].ends_with("<top>"));

        let top = compose_with_border_image(&content, &image, "top");
        assert_eq!(top, ["<top>", "<bottom>", "CPU: 8 cores", "RAM: 16 GiB"]);

        let bottom = compose_with_border_image(&content, &image, "bottom");
        assert_eq!(bottom, ["CPU: 8 cores", "RAM: 16 GiB", "<top>", "<bottom>"]);

        let bg = compose_with_border_image(&content, &image, "background");
        assert_eq!(bg, content);
    }

    #[test]
    fn test_build_right_lines_multi_border_images_fallback() {
        let image_path = format!("{}/../images/arch.png", env!("CARGO_MANIFEST_DIR"));
        let border_configs = vec![rustfetch_core::BorderImageConfig {
            path: image_path.clone(),
            position: "background".to_string(),
            width: Some(30),
            height: Some(4),
            offset_x: None,
            offset_y: None,
            z_index: -1,
            protocol: "halfblock".to_string(),
            preserve_aspect_ratio: true,
            text_on_top: true,
        }];

        let info = vec![InfoLine {
            icon: None,
            label: "OS".to_string(),
            value: "Linux".to_string(),
            color: "".to_string(),
        }];

        let opts = RenderOptions {
            header: None,
            header_color: None,
            separator: ": ",
            padding: 1,
            center: false,
            border: true,
            border_style: Some("rounded"),
            border_color: None,
            border_chars: None,
            border_title: None,
            box_padding: 1,
            palette_lines: None,
            palette_position: "bottom",
            icon_only: false,
            key_type: None,
            border_image: None,
            border_image_position: "left",
            border_images: &border_configs,
            logo_position: "left",
        };

        let lines = build_right_lines(&info, &opts);
        assert_eq!(lines.len(), 4);
        for line in &lines {
            assert!(line.contains("\x1b[48;2;"));
            assert!(line.ends_with("\x1b[0m"));
        }
        let stripped0 = strip_ansi(&lines[1]);
        assert!(stripped0.contains("OS"));
        assert!(stripped0.contains("Linux"));
    }

    #[test]
    fn test_render_with_logo_positions() {
        let logo = LogoBlock {
            lines: vec!["/---\\ ".to_string(), "\\---/ ".to_string()],
            width: 6,
        };
        let info = vec![InfoLine {
            icon: None,
            label: "Host".to_string(),
            value: "TestBox".to_string(),
            color: "".to_string(),
        }];

        for pos in &["left", "right", "top", "bottom", "hidden"] {
            let opts = RenderOptions {
                header: None,
                header_color: None,
                separator: ": ",
                padding: 1,
                center: false,
                border: false,
                border_style: None,
                border_color: None,
                border_chars: None,
                border_title: None,
                box_padding: 1,
                palette_lines: None,
                palette_position: "bottom",
                icon_only: false,
                key_type: None,
                border_image: None,
                border_image_position: "left",
                border_images: &[],
                logo_position: pos,
            };
            // Ensure no panics during rendering
            render(Some(&logo), &info, &opts);
        }
    }

    #[test]
    fn test_truncate_ansi() {
        assert_eq!(truncate_ansi("hello world", 15), "hello world");
        assert_eq!(truncate_ansi("hello world", 5), "hell…");
        assert_eq!(strip_ansi_width(&truncate_ansi("hello world", 5)), 5);

        let colored = "\x1b[31mhello world\x1b[0m";
        let trunc_col = truncate_ansi(colored, 6);
        assert!(trunc_col.contains("…"));
        assert!(trunc_col.ends_with("\x1b[0m"));
        assert_eq!(strip_ansi_width(&trunc_col), 6);
    }

    #[test]
    fn test_wrap_in_box_bounded_responsive() {
        let lines = vec![
            "OS: Arch Linux x86_64".to_string(),
            "CPU: 12th Gen Intel(R) Core(TM) i5-12400F (12) @ 4.40 GHz".to_string(),
        ];
        let max_w = 40;
        let boxed = wrap_in_box_bounded(
            &lines,
            Some("rounded"),
            None,
            None,
            Some("Info"),
            1,
            Some(max_w),
        );
        assert!(boxed.len() >= 4);
        for line in &boxed {
            assert!(
                strip_ansi_width(line) <= max_w,
                "Line exceeded max_w: {}",
                line
            );
        }
        let w0 = strip_ansi_width(&boxed[0]);
        for (idx, line) in boxed.iter().enumerate() {
            assert_eq!(strip_ansi_width(line), w0, "Row {} width differs", idx);
        }
    }
}
