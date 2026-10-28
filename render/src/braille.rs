use std::fmt::Write as _;

use fast_image_resize as fr;
use image::ImageReader;

// Braille dot offsets: (dy, dx, bit)
// Dot 1: (0,0), Dot 2: (1,0), Dot 3: (2,0), Dot 4: (0,1), Dot 5: (1,1), Dot 6: (2,1), Dot 7: (3,0), Dot 8: (3,1)
const BRAILLE_MAP: [(usize, usize, u8); 8] = [
    (0, 0, 0x01), // Dot 1
    (1, 0, 0x02), // Dot 2
    (2, 0, 0x04), // Dot 3
    (0, 1, 0x08), // Dot 4
    (1, 1, 0x10), // Dot 5
    (2, 1, 0x20), // Dot 6
    (3, 0, 0x40), // Dot 7
    (3, 1, 0x80), // Dot 8
];

pub fn render_braille_ex(
    path: &str,
    target_cols: usize,
    target_rows: Option<usize>,
    preserve_aspect_ratio: bool,
) -> Option<Vec<String>> {
    let img_reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    let img = img_reader.decode().ok()?.to_rgba8();

    let orig_width = img.width();
    let orig_height = img.height();
    if orig_width == 0 || orig_height == 0 || target_cols == 0 {
        return None;
    }

    let img_ratio = orig_width as f64 / orig_height as f64;

    // In braille, 1 cell = 2 dots wide x 4 dots tall.
    // Standard terminal cells are ~1:2 (w:h).
    // With 2 columns and 4 rows per cell, each dot is 1:1 square.
    let (pixel_cols, pixel_rows) = match target_rows {
        Some(rows) if rows > 0 => {
            let max_c = target_cols * 2;
            let max_r_px = rows * 4;
            if preserve_aspect_ratio {
                let from_w = (max_c as f64 / img_ratio).round() as usize;
                if from_w <= max_r_px {
                    (max_c, from_w.max(4))
                } else {
                    let from_h = (max_r_px as f64 * img_ratio).round() as usize;
                    (from_h.max(2), max_r_px)
                }
            } else {
                (max_c, max_r_px.max(4))
            }
        }
        _ => {
            let px_c = target_cols * 2;
            let px_h = ((px_c as f64 / img_ratio).round() as usize).max(4);
            (px_c, px_h)
        }
    };

    let mut pixel_cols = if pixel_cols % 2 != 0 {
        pixel_cols + 1
    } else {
        pixel_cols
    };
    let mut pixel_rows = if pixel_rows % 4 != 0 {
        pixel_rows + (4 - pixel_rows % 4)
    } else {
        pixel_rows
    };

    let max_cols_px = target_cols * 2;
    if pixel_cols > max_cols_px {
        pixel_cols = max_cols_px;
    }
    if let Some(mr) = target_rows {
        let max_rows_px = mr * 4;
        if pixel_rows > max_rows_px {
            pixel_rows = max_rows_px;
        }
    }

    let raw = crate::image::resize_rgba(
        img.into_raw(),
        orig_width,
        orig_height,
        pixel_cols as u32,
        pixel_rows as u32,
        fr::FilterType::Lanczos3,
    )?;

    let width = pixel_cols;
    let height = pixel_rows;
    let total_pixels = width * height;

    // Detect if the image has any transparent pixels (cutout / logo format)
    let img_has_alpha = raw.as_chunks::<4>().0.iter().any(|c| c[3] < 30);

    let img_avg_lum = if !img_has_alpha && total_pixels > 0 {
        let total_lum: u64 = raw
            .as_chunks::<4>()
            .0
            .iter()
            .map(|c| (c[0] as u64 * 299 + c[1] as u64 * 587 + c[2] as u64 * 114) / 1000)
            .sum();
        (total_lum / total_pixels as u64) as u32
    } else {
        0
    };

    let mut lines = Vec::with_capacity(height / 4);

    for by in (0..height).step_by(4) {
        let mut line = String::with_capacity((width / 2) * 20);
        let mut cur_fg: Option<(u8, u8, u8)> = None;

        for bx in (0..width).step_by(2) {
            let mut pixels = [(0u8, 0u8, 0u8, 0u8); 8];
            let mut total_lum = 0u32;
            let mut min_lum = 255u32;
            let mut max_lum = 0u32;

            for (i, &(dy, dx, _)) in BRAILLE_MAP.iter().enumerate() {
                let y = by + dy;
                let x = bx + dx;
                let idx = (y * width + x) * 4;
                let r = raw[idx];
                let g = raw[idx + 1];
                let b = raw[idx + 2];
                let a = raw[idx + 3];
                pixels[i] = (r, g, b, a);
                let lum = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
                total_lum += lum;
                if lum < min_lum {
                    min_lum = lum;
                }
                if lum > max_lum {
                    max_lum = lum;
                }
            }

            let mut pattern = 0u8;
            let mut on_r = 0u32;
            let mut on_g = 0u32;
            let mut on_b = 0u32;
            let mut on_count = 0usize;

            if img_has_alpha {
                // Transparency-based dot mask for cutouts and logos:
                // preserves all non-transparent pixels as solid shape regardless of color darkness
                for (i, &(_, _, bit)) in BRAILLE_MAP.iter().enumerate() {
                    let (r, g, b, a) = pixels[i];
                    if a >= 30 {
                        pattern |= bit;
                        on_r += r as u32;
                        on_g += g as u32;
                        on_b += b as u32;
                        on_count += 1;
                    }
                }
            } else {
                // Luminance-based dot threshold for opaque images
                let avg_lum = total_lum / 8;
                let threshold = if max_lum - min_lum < 16 {
                    img_avg_lum.saturating_add(15) // Clean background cutoff
                } else {
                    avg_lum.max(25) // Edge contrast cutoff
                };

                for (i, &(_, _, bit)) in BRAILLE_MAP.iter().enumerate() {
                    let (r, g, b, _) = pixels[i];
                    let lum = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
                    if lum >= threshold {
                        pattern |= bit;
                        on_r += r as u32;
                        on_g += g as u32;
                        on_b += b as u32;
                        on_count += 1;
                    }
                }
            }

            if on_count == 0 || pattern == 0 {
                if cur_fg.is_some() {
                    line.push_str("\x1b[39m");
                    cur_fg = None;
                }
                line.push(' ');
            } else {
                let fg_r = (on_r / on_count as u32) as u8;
                let fg_g = (on_g / on_count as u32) as u8;
                let fg_b = (on_b / on_count as u32) as u8;

                if cur_fg != Some((fg_r, fg_g, fg_b)) {
                    let _ = write!(line, "\x1b[38;2;{fg_r};{fg_g};{fg_b}m");
                    cur_fg = Some((fg_r, fg_g, fg_b));
                }

                let ch = char::from_u32(0x2800 + pattern as u32).unwrap_or(' ');
                line.push(ch);
            }
        }

        if cur_fg.is_some() {
            line.push_str("\x1b[0m");
        }
        lines.push(line);
    }

    Some(lines)
}

pub fn render_braille(path: &str, target_cols: usize) -> Option<Vec<String>> {
    render_braille_ex(path, target_cols, None, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_braille_arch_logo() {
        let image = format!("{}/../images/arch.png", env!("CARGO_MANIFEST_DIR"));
        let lines = render_braille(&image, 10).expect("braille rendering must succeed");
        assert!(!lines.is_empty());
        assert!(lines.iter().any(|line| line.contains("\x1b[38;2;")));
    }

    #[test]
    fn test_render_braille_dark_transparent_logo() {
        // Create an in-memory 8x8 image with transparent border and dark blue center (RGB: 0, 0, 100)
        let mut img_buf = image::RgbaImage::new(8, 8);
        for y in 0..8 {
            for x in 0..8 {
                if (2..6).contains(&x) && (2..6).contains(&y) {
                    img_buf.put_pixel(x, y, image::Rgba([0, 0, 100, 255]));
                } else {
                    img_buf.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));
                }
            }
        }
        let tmp =
            std::env::temp_dir().join(format!("test_braille_dark_{}.png", std::process::id()));
        img_buf.save(&tmp).unwrap();

        let lines = render_braille(tmp.to_str().unwrap(), 4).expect("must render");
        let _ = std::fs::remove_file(&tmp);

        // Dark transparent logo center must produce braille dots, not be completely empty
        let non_space = lines
            .iter()
            .any(|l| l.chars().any(|c| ('\u{2801}'..='\u{28ff}').contains(&c)));
        assert!(
            non_space,
            "Dark transparent pixels must produce solid braille dots"
        );
    }
}
