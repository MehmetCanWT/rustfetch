use std::fmt::Write as _;
use std::io::Write;

use fast_image_resize as fr;
use image::ImageReader;

use crate::terminal;

/// Map an 8-bit RGBA pixel to a 232-color palette index:
/// 0..215: 6x6x6 RGB cube
/// 216..231: 16-level grayscale ramp
#[inline]
fn quantize_pixel(r: u8, g: u8, b: u8, a: u8) -> Option<u8> {
    if a < 64 {
        return None; // Transparent
    }
    // Grayscale check
    if r == g && g == b {
        let gray_idx = ((r as u32 * 15 + 127) / 255) as u8;
        return Some(216 + gray_idx);
    }
    let r_idx = ((r as u32 * 5 + 127) / 255) as u8;
    let g_idx = ((g as u32 * 5 + 127) / 255) as u8;
    let b_idx = ((b as u32 * 5 + 127) / 255) as u8;
    Some(r_idx * 36 + g_idx * 6 + b_idx)
}

/// Emit run-length encoded Sixel characters.
fn emit_rle(out: &mut String, ch: char, count: usize) {
    if count == 0 {
        return;
    }
    if count == 1 {
        out.push(ch);
    } else if count == 2 {
        out.push(ch);
        out.push(ch);
    } else if count == 3 {
        out.push(ch);
        out.push(ch);
        out.push(ch);
    } else {
        let _ = write!(out, "!{count}{ch}");
    }
}

/// Encodes an RGBA image buffer into a Sixel escape sequence.
pub fn encode_sixel(rgba_data: &[u8], width: u32, height: u32) -> String {
    if rgba_data.is_empty() || width == 0 || height == 0 {
        return String::new();
    }

    let mut out = String::with_capacity((width * height) as usize);

    // DCS q (Sixel graphic intro with aspect ratio 1:1 and dimensions)
    let _ = write!(out, "\x1bPq\"1;1;{width};{height}");

    // Define 6x6x6 color cube (indices 0..215)
    for r in 0..6 {
        let r_pct = (r * 100 + 2) / 5;
        for g in 0..6 {
            let g_pct = (g * 100 + 2) / 5;
            for b in 0..6 {
                let b_pct = (b * 100 + 2) / 5;
                let idx = r * 36 + g * 6 + b;
                let _ = write!(out, "#{idx};2;{r_pct};{g_pct};{b_pct}");
            }
        }
    }

    // Define 16 grayscale levels (indices 216..231)
    for i in 0..16 {
        let pct = (i * 100 + 7) / 15;
        let idx = 216 + i;
        let _ = write!(out, "#{idx};2;{pct};{pct};{pct}");
    }

    // Map all pixels to palette indices
    let total_pixels = (width * height) as usize;
    let mut indexed: Vec<Option<u8>> = Vec::with_capacity(total_pixels);
    for chunk in rgba_data.as_chunks::<4>().0 {
        indexed.push(quantize_pixel(chunk[0], chunk[1], chunk[2], chunk[3]));
    }

    let w = width as usize;
    let h = height as usize;

    // Process image in 6-row bands
    for band_y in (0..h).step_by(6) {
        // Collect which colors appear in this band
        let mut color_mask = [false; 232];
        for dy in 0..6 {
            let y = band_y + dy;
            if y >= h {
                break;
            }
            let row_offset = y * w;
            for x in 0..w {
                if let Some(c) = indexed[row_offset + x] {
                    color_mask[c as usize] = true;
                }
            }
        }

        let mut first_color = true;

        for (color_id, &present) in color_mask.iter().enumerate() {
            if !present {
                continue;
            }

            if !first_color {
                out.push('$'); // Carriage return to start of band
            }
            first_color = false;

            let _ = write!(out, "#{color_id}");

            // Build sixel column values for this color
            let mut line_chars = Vec::with_capacity(w);
            for x in 0..w {
                let mut bits = 0u8;
                for dy in 0..6 {
                    let y = band_y + dy;
                    if y < h && indexed[y * w + x] == Some(color_id as u8) {
                        bits |= 1 << dy;
                    }
                }
                line_chars.push((63 + bits) as char);
            }

            // Strip trailing empty pixels ('?' = ASCII 63)
            let mut end = line_chars.len();
            while end > 0 && line_chars[end - 1] == '?' {
                end -= 1;
            }

            // RLE encode the non-empty segment
            let mut cur_ch = '\0';
            let mut count = 0;
            for &ch in &line_chars[..end] {
                if ch == cur_ch {
                    count += 1;
                } else {
                    emit_rle(&mut out, cur_ch, count);
                    cur_ch = ch;
                    count = 1;
                }
            }
            emit_rle(&mut out, cur_ch, count);
        }

        // Newline (advance down 6 pixels and return to col 0)
        out.push('-');
    }

    // ST (String Terminator)
    out.push_str("\x1b\\");
    out
}

/// Renders an image using the Sixel graphics protocol.
pub fn render_sixel_ex(
    path: &str,
    target_cols: usize,
    max_rows: Option<usize>,
) -> Option<(usize, usize)> {
    let img_reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    let rgba_img = img_reader.decode().ok()?.to_rgba8();

    let orig_width = rgba_img.width();
    let orig_height = rgba_img.height();
    if orig_width == 0 || orig_height == 0 || target_cols == 0 {
        return None;
    }

    let (cell_w, cell_h) = terminal::get_cell_size().unwrap_or((9, 18));
    let cell_w = cell_w.max(1) as u32;
    let cell_h = cell_h.max(1) as u32;

    let (fit_cols, fit_rows) = crate::image::fit_dimensions(
        orig_width,
        orig_height,
        Some(target_cols),
        max_rows,
        target_cols,
        max_rows.unwrap_or(24),
        true,
    );
    let fit_cols = fit_cols.max(1);
    let fit_rows = fit_rows.max(1);

    let target_px_width = (fit_cols as u32) * cell_w;
    let target_px_height = (fit_rows as u32) * cell_h;

    let buf = crate::image::resize_rgba(
        rgba_img.into_raw(),
        orig_width,
        orig_height,
        target_px_width,
        target_px_height,
        fr::FilterType::Lanczos3,
    )?;

    let sixel_payload = encode_sixel(&buf, target_px_width, target_px_height);
    print!("{sixel_payload}");
    let _ = std::io::stdout().flush();

    Some((fit_cols, fit_rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantize_pixel() {
        assert_eq!(quantize_pixel(0, 0, 0, 0), None);
        assert_eq!(quantize_pixel(255, 255, 255, 255), Some(231)); // Grayscale white
        assert_eq!(quantize_pixel(0, 0, 0, 255), Some(216)); // Grayscale black
        assert_eq!(quantize_pixel(255, 0, 0, 255), Some(5 * 36)); // Pure red in cube
    }

    #[test]
    fn test_encode_sixel_basic() {
        let rgba = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 2x1 red and green
        let encoded = encode_sixel(&rgba, 2, 1);
        assert!(encoded.starts_with("\x1bPq\"1;1;2;1"));
        assert!(encoded.ends_with("\x1b\\"));
    }
}
