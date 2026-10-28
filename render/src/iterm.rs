use base64::prelude::*;
use std::io::Write;

use crate::terminal::wrap_tmux_passthrough;

/// Encodes image file bytes into an iTerm2 inline image OSC 1337 escape sequence.
pub fn encode_iterm(image_bytes: &[u8], cols: usize, rows: usize) -> String {
    let encoded = BASE64_STANDARD.encode(image_bytes);
    format!(
        "\x1b]1337;File=inline=1;width={cols};height={rows};preserveAspectRatio=1:{encoded}\x07"
    )
}

/// Renders an image using the iTerm2 OSC 1337 inline protocol.
pub fn render_iterm_ex(
    path: &str,
    target_cols: usize,
    max_rows: Option<usize>,
) -> Option<(usize, usize)> {
    if target_cols == 0 {
        return None;
    }
    let file_bytes = std::fs::read(path).ok()?;
    if file_bytes.is_empty() {
        return None;
    }

    let (orig_width, orig_height) = image::image_dimensions(path).ok()?;
    if orig_width == 0 || orig_height == 0 {
        return None;
    }

    let (fit_cols, fit_rows) =
        crate::image::fit_cell_dimensions(orig_width, orig_height, target_cols, max_rows);

    let payload = encode_iterm(&file_bytes, fit_cols, fit_rows);
    print!("{}", wrap_tmux_passthrough(&payload));
    let _ = std::io::stdout().flush();

    Some((fit_cols, fit_rows))
}

pub fn render_iterm(path: &str, target_cols: usize) -> Option<(usize, usize)> {
    render_iterm_ex(path, target_cols, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_iterm_format() {
        let dummy_data = b"hello image";
        let seq = encode_iterm(dummy_data, 20, 10);
        let expected_b64 = BASE64_STANDARD.encode(dummy_data);
        assert_eq!(
            seq,
            format!("\x1b]1337;File=inline=1;width=20;height=10;preserveAspectRatio=1:{expected_b64}\x07")
        );
    }

    #[test]
    fn test_iterm_tmux_passthrough() {
        let dummy_data = b"test";
        let seq = encode_iterm(dummy_data, 5, 2);
        let wrapped = crate::terminal::wrap_tmux_passthrough_always(&seq);
        assert!(wrapped.starts_with("\x1bPtmux;\x1b\x1b]1337;File="));
        assert!(wrapped.ends_with("\x07\x1b\\"));
    }
}
