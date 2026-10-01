use unicode_width::UnicodeWidthStr;

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";

pub fn colors_enabled() -> bool {
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

pub fn terminal_width() -> usize {
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col > 0 {
            return ws.ws_col as usize;
        }
    }
    std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(80)
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

pub struct RenderOptions<'a> {
    pub header: Option<&'a str>,
    pub header_color: Option<&'a str>,
    pub separator: &'a str,
    pub padding: usize,
    pub center: bool,
    pub border: bool,
    pub palette_lines: Option<&'a [String]>,
}

pub fn build_right_lines(info_lines: &[InfoLine], opts: &RenderOptions) -> Vec<String> {
    let mut right_lines: Vec<String> = Vec::new();

    if let Some(h) = opts.header {
        let col = opts.header_color.unwrap_or("cyan");
        right_lines.push(format!("{BOLD}{}{h}{RESET}", color_code(col)));
        right_lines.push(format!(
            "{DIM}{}{RESET}",
            "─".repeat(UnicodeWidthStr::width(h))
        ));
    }

    right_lines.extend(format_info_lines(info_lines, opts.separator));

    if let Some(pal) = opts.palette_lines {
        right_lines.push(String::new());
        right_lines.extend(pal.iter().cloned());
    }

    if opts.border {
        right_lines = wrap_in_box(&right_lines);
    }

    right_lines
}

pub fn wrap_in_box(lines: &[String]) -> Vec<String> {
    if lines.is_empty() {
        return Vec::new();
    }
    let max_w = lines.iter().map(|l| strip_ansi_width(l)).max().unwrap_or(0);
    let inner_w = max_w + 2;
    let mut boxed = Vec::with_capacity(lines.len() + 2);

    boxed.push(format!("╭{}╮", "─".repeat(inner_w)));
    for line in lines {
        let visual_w = strip_ansi_width(line);
        let padding = " ".repeat(max_w.saturating_sub(visual_w));
        boxed.push(format!("│ {line}{padding} │"));
    }
    boxed.push(format!("╰{}╯", "─".repeat(inner_w)));
    boxed
}

pub fn render(logo: Option<&LogoBlock>, info_lines: &[InfoLine], opts: &RenderOptions) {
    let term_w = terminal_width();
    let pad = " ".repeat(opts.padding);

    if let Some(logo) =
        logo.filter(|l| l.width + LOGO_GAP + 20 <= term_w && term_w >= MIN_LOGO_WIDTH)
    {
        render_with_logo(logo, info_lines, &pad, term_w, opts);
    } else {
        render_without_logo(info_lines, &pad, term_w, opts);
    }
}

pub fn format_info_lines(info_lines: &[InfoLine], separator: &str) -> Vec<String> {
    let label_width = info_lines
        .iter()
        .filter(|l| !l.label.is_empty() && !l.value.is_empty())
        .map(|l| {
            let icon_w = l
                .icon
                .as_deref()
                .map(|i| UnicodeWidthStr::width(i) + 1)
                .unwrap_or(0);
            UnicodeWidthStr::width(l.label.as_str()) + icon_w
        })
        .max()
        .unwrap_or(0);

    info_lines
        .iter()
        .map(|line| {
            if line.label.is_empty() && line.icon.is_none() {
                if line.value.is_empty() {
                    return String::new();
                } else {
                    return format!("{}{}{}", line.color, line.value, RESET);
                }
            } else if line.value.is_empty() {
                let icon_str = line
                    .icon
                    .as_deref()
                    .map(|i| format!("{i} "))
                    .unwrap_or_default();
                return format!("{BOLD}{}{icon_str}{}{RESET}", line.color, line.label);
            }

            let icon_str = line
                .icon
                .as_deref()
                .map(|i| format!("{i} "))
                .unwrap_or_default();
            let icon_w = line
                .icon
                .as_deref()
                .map(|i| UnicodeWidthStr::width(i) + 1)
                .unwrap_or(0);
            let cur_label_w = UnicodeWidthStr::width(line.label.as_str()) + icon_w;
            let label_pad = " ".repeat(label_width.saturating_sub(cur_label_w));

            format!(
                "{BOLD}{color}{icon_str}{label}{RESET}{label_pad} {sep} {value}",
                color = line.color,
                label = line.label,
                sep = separator,
                value = line.value
            )
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
    let mut right_lines = build_right_lines(info_lines, opts);

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

    let total_lines = image_lines.max(right_lines.len());
    let right_indent = h_pad + image_cols + LOGO_GAP;

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
    let mut right_lines = build_right_lines(info_lines, opts);

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

    let total_lines = logo.lines.len().max(right_lines.len());
    let logo_v_offset = if right_lines.len() > logo.lines.len() {
        (right_lines.len() - logo.lines.len()) / 2
    } else {
        0
    };
    let logo_pad_str = " ".repeat(logo.width);
    let gap = " ".repeat(LOGO_GAP);

    let disable_colors = !colors_enabled();
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
    let right_lines = build_right_lines(info_lines, opts);

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

    let disable_colors = !colors_enabled();
    for line in right_lines {
        let display_line = if disable_colors {
            strip_ansi(&line)
        } else {
            line
        };
        print!("{h_pad_str}{display_line}\r\n");
    }
}

pub fn strip_ansi(s: &str) -> String {
    let mut in_escape = false;
    let mut clean = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
            continue;
        }
        if in_escape {
            if ch.is_ascii_alphabetic() {
                in_escape = false;
            }
            continue;
        }
        clean.push(ch);
    }
    clean
}

pub fn strip_ansi_width(s: &str) -> usize {
    let mut in_escape = false;
    let mut clean = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
            continue;
        }
        if in_escape {
            if ch.is_ascii_alphabetic() {
                in_escape = false;
            }
            continue;
        }
        clean.push(ch);
    }
    UnicodeWidthStr::width(clean.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_code() {
        assert_eq!(color_code("red"), "\x1b[38;2;239;83;80m");
    }

    #[test]
    fn test_strip_ansi_width() {
        assert_eq!(strip_ansi_width("\x1b[31mhello\x1b[0m"), 5);
    }

    #[test]
    fn test_terminal_width() {
        assert!(terminal_width() > 0);
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
        let formatted = format_info_lines(&lines, ":");
        assert_eq!(formatted.len(), 4);
        assert!(formatted[0].contains("OS"));
        assert!(formatted[1].is_empty()); // break line is empty
        assert!(formatted[2].contains("Custom"));
        assert!(formatted[3].contains("Raw Text"));
    }

    #[test]
    fn test_wrap_in_box() {
        let lines = vec!["Hello".to_string(), "World!".to_string()];
        let boxed = wrap_in_box(&lines);
        assert_eq!(boxed.len(), 4);
        assert!(boxed[0].starts_with('╭'));
        assert!(boxed[0].ends_with('╮'));
        assert!(boxed[1].contains("Hello"));
        assert!(boxed[2].contains("World!"));
        assert!(boxed[3].starts_with('╰'));
        assert!(boxed[3].ends_with('╯'));
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
            palette_lines: None,
        };
        let lines = build_right_lines(&info, &opts);
        assert_eq!(lines.len(), 3);
        // Header line must contain the 24-bit TrueColor for #ff007f (255, 0, 127)
        assert!(lines[0].contains("\x1b[38;2;255;0;127m"));
        assert!(lines[0].contains("user@host"));
    }
}
