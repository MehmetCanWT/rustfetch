use fast_image_resize as fr;
use fast_image_resize::Image;
use image::{AnimationDecoder, ImageReader};
use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufReader, Write};
use std::num::NonZeroU32;

use crate::{kitty, terminal};

pub fn render(path: &str, target_cols: usize) -> Option<usize> {
    render_ex(path, target_cols, None).map(|(_, rows)| rows)
}

pub fn render_ex(
    path: &str,
    target_cols: usize,
    max_rows: Option<usize>,
) -> Option<(usize, usize)> {
    let (cell_w, cell_h) = terminal::get_cell_size().unwrap_or((10, 20));

    let img_reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    if img_reader.format() == Some(image::ImageFormat::Gif) {
        let file = File::open(path).ok()?;
        if let Ok(decoder) = image::codecs::gif::GifDecoder::new(BufReader::new(file)) {
            let frames = decoder.into_frames().collect_frames().ok()?;
            if frames.is_empty() {
                return None;
            }

            let orig_width = frames[0].buffer().width();
            let orig_height = frames[0].buffer().height();
            if orig_width == 0 || orig_height == 0 || target_cols == 0 {
                return None;
            }

            let (fit_cols, fit_rows) = if let Some(max_r) = max_rows {
                fit_dimensions(
                    orig_width,
                    orig_height,
                    Some(target_cols),
                    Some(max_r),
                    target_cols,
                    max_r,
                    true,
                )
            } else {
                fit_dimensions(
                    orig_width,
                    orig_height,
                    Some(target_cols),
                    None,
                    target_cols,
                    24,
                    true,
                )
            };
            let fit_cols = fit_cols.max(1);
            let fit_rows = fit_rows.max(1);

            let target_px_width = fit_cols as u32 * cell_w as u32;
            let target_px_height = fit_rows as u32 * cell_h as u32;

            if target_px_width == 0 || target_px_height == 0 {
                return None;
            }

            let mut resized_buffers = Vec::new();
            let mut resizer =
                fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));

            for frame in &frames {
                let rgba_img = frame.buffer();
                let src_width = NonZeroU32::new(orig_width)?;
                let src_height = NonZeroU32::new(orig_height)?;
                let src_image = Image::from_vec_u8(
                    src_width,
                    src_height,
                    rgba_img.as_raw().clone(),
                    fr::PixelType::U8x4,
                )
                .ok()?;

                let dst_width = NonZeroU32::new(target_px_width)?;
                let dst_height = NonZeroU32::new(target_px_height)?;
                let mut dst_image = Image::new(dst_width, dst_height, fr::PixelType::U8x4);

                resizer
                    .resize(&src_image.view(), &mut dst_image.view_mut())
                    .ok()?;

                let (num, den) = frame.delay().numer_denom_ms();
                let delay_ms = num.checked_div(den).unwrap_or(0);
                resized_buffers.push((dst_image.into_vec(), delay_ms));
            }

            let kitty_frames: Vec<_> = resized_buffers
                .iter()
                .map(|(buf, delay)| kitty::KittyFrame {
                    rgba_data: buf,
                    delay_ms: *delay,
                })
                .collect();

            kitty::print_kitty_animation(
                &kitty_frames,
                target_px_width,
                target_px_height,
                fit_cols as u32,
                fit_rows as u32,
            );
            return Some((fit_cols, fit_rows));
        }
    }

    let img = img_reader.decode().ok()?;
    let rgba_img = img.to_rgba8();

    let orig_width = rgba_img.width();
    let orig_height = rgba_img.height();
    if orig_width == 0 || orig_height == 0 || target_cols == 0 {
        return None;
    }

    let (fit_cols, fit_rows) = if let Some(max_r) = max_rows {
        fit_dimensions(
            orig_width,
            orig_height,
            Some(target_cols),
            Some(max_r),
            target_cols,
            max_r,
            true,
        )
    } else {
        fit_dimensions(
            orig_width,
            orig_height,
            Some(target_cols),
            None,
            target_cols,
            24,
            true,
        )
    };
    let fit_cols = fit_cols.max(1);
    let fit_rows = fit_rows.max(1);

    let target_px_width = fit_cols as u32 * cell_w as u32;
    let target_px_height = fit_rows as u32 * cell_h as u32;

    if target_px_width == 0 || target_px_height == 0 {
        return None;
    }

    let src_width = NonZeroU32::new(orig_width)?;
    let src_height = NonZeroU32::new(orig_height)?;
    let src_image = Image::from_vec_u8(
        src_width,
        src_height,
        rgba_img.into_raw(),
        fr::PixelType::U8x4,
    )
    .ok()?;

    let dst_width = NonZeroU32::new(target_px_width)?;
    let dst_height = NonZeroU32::new(target_px_height)?;
    let mut dst_image = Image::new(dst_width, dst_height, fr::PixelType::U8x4);

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer
        .resize(&src_image.view(), &mut dst_image.view_mut())
        .ok()?;

    kitty::print_kitty(
        dst_image.buffer(),
        target_px_width,
        target_px_height,
        fit_cols as u32,
        fit_rows as u32,
    );

    Some((fit_cols, fit_rows))
}

/// Computes target (cols, rows) in terminal character cells while preserving image aspect ratio.
/// Takes into account the terminal cell aspect ratio (typically 1:2, where 1 col is half as wide as 1 row is tall).
pub fn fit_dimensions(
    orig_w: u32,
    orig_h: u32,
    target_cols: Option<usize>,
    target_rows: Option<usize>,
    container_cols: usize,
    container_rows: usize,
    preserve_aspect_ratio: bool,
) -> (usize, usize) {
    if orig_w == 0 || orig_h == 0 {
        return (
            target_cols.unwrap_or(container_cols).max(1),
            target_rows.unwrap_or(container_rows).max(1),
        );
    }

    if !preserve_aspect_ratio {
        return (
            target_cols.unwrap_or(container_cols).max(1),
            target_rows.unwrap_or(container_rows).max(1),
        );
    }

    let (cell_w, cell_h) = terminal::cell_size_or_default();
    let cell_ratio = cell_w as f64 / cell_h as f64; // typically ~0.5
    let img_ratio = orig_w as f64 / orig_h as f64;
    // How many terminal columns correspond to 1 terminal row for this image:
    let natural_cols_per_row = (img_ratio / cell_ratio).max(0.1);

    match (target_cols, target_rows) {
        (Some(c), None) => {
            let cols = c.max(1);
            let rows = (cols as f64 / natural_cols_per_row).round().max(1.0) as usize;
            (cols, rows)
        }
        (None, Some(r)) => {
            let rows = r.max(1);
            let cols = (rows as f64 * natural_cols_per_row).round().max(1.0) as usize;
            (cols, rows)
        }
        (Some(c), Some(r)) => {
            let box_c = c.max(1) as f64;
            let box_r = r.max(1) as f64;
            if box_c / natural_cols_per_row <= box_r {
                let cols = box_c as usize;
                let rows = (box_c / natural_cols_per_row).round().max(1.0) as usize;
                (cols, rows)
            } else {
                let rows = box_r as usize;
                let cols = (box_r * natural_cols_per_row).round().max(1.0) as usize;
                (cols, rows)
            }
        }
        (None, None) => {
            let box_c = container_cols.max(1) as f64;
            let box_r = container_rows.max(1) as f64;
            if box_c / natural_cols_per_row <= box_r {
                let cols = box_c as usize;
                let rows = (box_c / natural_cols_per_row).round().max(1.0) as usize;
                (cols, rows)
            } else {
                let rows = box_r as usize;
                let cols = (box_r * natural_cols_per_row).round().max(1.0) as usize;
                (cols, rows)
            }
        }
    }
}

pub fn render_halfblock_ex(
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

    // In halfblocks, each cell character has 2 vertical pixels (1 column = 1 px wide, 1 row = 2 px high).
    // Because a terminal cell is ~10px wide x 20px high, each half-block pixel is ~10px x 10px (square!).
    let (pixel_cols, pixel_rows) = match target_rows {
        Some(rows) if rows > 0 => {
            let max_c = target_cols;
            let max_r_px = rows * 2;
            if preserve_aspect_ratio {
                let from_w = (max_c as f64 / img_ratio).round() as usize;
                if from_w <= max_r_px {
                    (max_c, from_w.max(2))
                } else {
                    let from_h = (max_r_px as f64 * img_ratio).round() as usize;
                    (from_h.max(1), max_r_px)
                }
            } else {
                (max_c, max_r_px.max(2))
            }
        }
        _ => {
            let px_h = ((target_cols as f64 / img_ratio).round() as usize).max(2);
            (target_cols, px_h)
        }
    };

    let pixel_rows = if !pixel_rows.is_multiple_of(2) {
        pixel_rows + 1
    } else {
        pixel_rows
    };

    let src_width = NonZeroU32::new(orig_width)?;
    let src_height = NonZeroU32::new(orig_height)?;
    let src_image =
        Image::from_vec_u8(src_width, src_height, img.into_raw(), fr::PixelType::U8x4).ok()?;

    let dst_width = NonZeroU32::new(pixel_cols as u32)?;
    let dst_height = NonZeroU32::new(pixel_rows as u32)?;
    let mut dst_image = Image::new(dst_width, dst_height, fr::PixelType::U8x4);

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer
        .resize(&src_image.view(), &mut dst_image.view_mut())
        .ok()?;

    let raw = dst_image.into_vec();
    let width = pixel_cols;
    let height = pixel_rows;
    let mut lines = Vec::with_capacity(height / 2);

    for y in (0..height).step_by(2) {
        let mut line = String::with_capacity(width * 20);
        let mut cur_fg: Option<(u8, u8, u8)> = None;
        let mut cur_bg: Option<(u8, u8, u8)> = None;

        for x in 0..width {
            let idx_top = (y * width + x) * 4;
            let idx_bot = ((y + 1) * width + x) * 4;

            let (r1, g1, b1, a1) = (
                raw[idx_top],
                raw[idx_top + 1],
                raw[idx_top + 2],
                raw[idx_top + 3],
            );
            let (r2, g2, b2, a2) = (
                raw[idx_bot],
                raw[idx_bot + 1],
                raw[idx_bot + 2],
                raw[idx_bot + 3],
            );

            if a1 < 30 && a2 < 30 {
                if cur_bg.is_some() {
                    line.push_str("\x1b[49m");
                    cur_bg = None;
                }
                line.push(' ');
            } else if a1 < 30 {
                if cur_bg.is_some() {
                    line.push_str("\x1b[49m");
                    cur_bg = None;
                }
                if cur_fg != Some((r2, g2, b2)) {
                    let _ = write!(line, "\x1b[38;2;{r2};{g2};{b2}m");
                    cur_fg = Some((r2, g2, b2));
                }
                line.push('▄');
            } else if a2 < 30 {
                if cur_bg.is_some() {
                    line.push_str("\x1b[49m");
                    cur_bg = None;
                }
                if cur_fg != Some((r1, g1, b1)) {
                    let _ = write!(line, "\x1b[38;2;{r1};{g1};{b1}m");
                    cur_fg = Some((r1, g1, b1));
                }
                line.push('▀');
            } else {
                if cur_fg != Some((r1, g1, b1)) {
                    let _ = write!(line, "\x1b[38;2;{r1};{g1};{b1}m");
                    cur_fg = Some((r1, g1, b1));
                }
                if cur_bg != Some((r2, g2, b2)) {
                    let _ = write!(line, "\x1b[48;2;{r2};{g2};{b2}m");
                    cur_bg = Some((r2, g2, b2));
                }
                line.push('▀');
            }
        }
        if cur_fg.is_some() || cur_bg.is_some() {
            line.push_str("\x1b[0m");
        }
        lines.push(line);
    }

    Some(lines)
}

pub fn render_halfblock(path: &str, target_cols: usize) -> Option<Vec<String>> {
    render_halfblock_ex(path, target_cols, None, true)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundOptions {
    pub col_offset: usize,
    pub card_w: usize,
    pub card_h: usize,
    pub target_cols: Option<usize>,
    pub target_rows: Option<usize>,
    pub offset_x: Option<i32>,
    pub offset_y: Option<i32>,
    pub z_index: i32,
    pub preserve_aspect_ratio: bool,
}

pub fn render_background_ex(path: &str, opts: &BackgroundOptions) -> Option<usize> {
    let img_reader = match ImageReader::open(path) {
        Ok(r) => match r.with_guessed_format() {
            Ok(r) => r,
            Err(_) => return None,
        },
        Err(_) => return None,
    };

    let img = match img_reader.decode() {
        Ok(i) => i,
        Err(_) => return None,
    };
    let rgba_img = img.to_rgba8();
    let orig_width = rgba_img.width();
    let orig_height = rgba_img.height();
    if orig_width == 0 || orig_height == 0 || opts.card_w == 0 || opts.card_h == 0 {
        return None;
    }

    let (fit_cols, fit_rows) = if opts.preserve_aspect_ratio {
        fit_dimensions(
            orig_width,
            orig_height,
            opts.target_cols,
            opts.target_rows,
            opts.card_w,
            opts.card_h,
            true,
        )
    } else {
        (
            opts.target_cols.unwrap_or(opts.card_w).max(1),
            opts.target_rows.unwrap_or(opts.card_h).max(1),
        )
    };

    // Calculate exact center alignment within the card:
    let center_pad_x = (opts.card_w as i32 - fit_cols as i32) / 2;
    let center_pad_y = (opts.card_h as i32 - fit_rows as i32) / 2;

    let eff_col =
        (opts.col_offset as i32 + center_pad_x + opts.offset_x.unwrap_or(0)).max(0) as usize;
    let eff_row = (center_pad_y + opts.offset_y.unwrap_or(0)).max(0) as usize;

    let max_px_w = 1920u32;
    let max_px_h = 1080u32;
    let (buf, w, h) = if orig_width > max_px_w || orig_height > max_px_h {
        let ratio_w = max_px_w as f64 / orig_width as f64;
        let ratio_h = max_px_h as f64 / orig_height as f64;
        let scale = ratio_w.min(ratio_h);
        let dst_w = ((orig_width as f64 * scale).round() as u32).max(1);
        let dst_h = ((orig_height as f64 * scale).round() as u32).max(1);

        if let (Some(sw), Some(sh), Some(dw), Some(dh)) = (
            NonZeroU32::new(orig_width),
            NonZeroU32::new(orig_height),
            NonZeroU32::new(dst_w),
            NonZeroU32::new(dst_h),
        ) {
            match Image::from_vec_u8(sw, sh, rgba_img.into_raw(), fr::PixelType::U8x4) {
                Ok(src_image) => {
                    let mut dst_image = Image::new(dw, dh, fr::PixelType::U8x4);
                    let mut resizer =
                        fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
                    if resizer
                        .resize(&src_image.view(), &mut dst_image.view_mut())
                        .is_ok()
                    {
                        (dst_image.into_vec(), dst_w, dst_h)
                    } else {
                        return None;
                    }
                }
                Err(_) => return None,
            }
        } else {
            (rgba_img.into_raw(), orig_width, orig_height)
        }
    } else {
        (rgba_img.into_raw(), orig_width, orig_height)
    };

    print!("\x1b7");
    if eff_col > 0 {
        print!("\x1b[{}C", eff_col);
    }
    if eff_row > 0 {
        print!("\x1b[{}B", eff_row);
    }

    kitty::print_kitty_ex(
        &buf,
        w,
        h,
        fit_cols as u32,
        fit_rows as u32,
        opts.z_index,
        true,
    );

    print!("\x1b8");
    let _ = std::io::stdout().flush();
    Some(eff_row + fit_rows)
}

pub fn render_background(path: &str, target_cols: usize, target_rows: usize, z_index: i32) -> bool {
    let opts = BackgroundOptions {
        col_offset: 0,
        card_w: target_cols,
        card_h: target_rows,
        target_cols: Some(target_cols),
        target_rows: Some(target_rows),
        offset_x: None,
        offset_y: None,
        z_index,
        preserve_aspect_ratio: true,
    };
    render_background_ex(path, &opts).is_some()
}

pub fn apply_image_background_to_lines(
    lines: &[String],
    path: &str,
    target_cols: usize,
    target_rows: usize,
) -> Option<Vec<String>> {
    apply_image_background_to_lines_ex(lines, path, target_cols, target_rows, true, None, None)
}

pub fn apply_image_background_to_lines_ex(
    lines: &[String],
    path: &str,
    target_cols: usize,
    target_rows: usize,
    preserve_aspect_ratio: bool,
    offset_x: Option<i32>,
    offset_y: Option<i32>,
) -> Option<Vec<String>> {
    let img_reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    let img = img_reader.decode().ok()?.to_rgba8();

    let orig_width = img.width();
    let orig_height = img.height();
    if orig_width == 0 || orig_height == 0 || target_cols == 0 || target_rows == 0 {
        return None;
    }

    let (fit_cols, fit_rows) = if preserve_aspect_ratio {
        fit_dimensions(
            orig_width,
            orig_height,
            Some(target_cols),
            Some(target_rows),
            target_cols,
            target_rows,
            true,
        )
    } else {
        (target_cols, target_rows)
    };

    let src_w = NonZeroU32::new(orig_width)?;
    let src_h = NonZeroU32::new(orig_height)?;
    let src_image = Image::from_vec_u8(src_w, src_h, img.into_raw(), fr::PixelType::U8x4).ok()?;

    let dst_w = NonZeroU32::new(fit_cols as u32)?;
    let dst_h = NonZeroU32::new(fit_rows as u32)?;
    let mut dst_image = Image::new(dst_w, dst_h, fr::PixelType::U8x4);

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
    resizer
        .resize(&src_image.view(), &mut dst_image.view_mut())
        .ok()?;
    let raw = dst_image.into_vec();
    let mut result = Vec::with_capacity(target_rows);

    let center_pad_x = (target_cols as i32 - fit_cols as i32) / 2;
    let center_pad_y = (target_rows as i32 - fit_rows as i32) / 2;
    let start_col = (center_pad_x + offset_x.unwrap_or(0)).max(0) as usize;
    let start_row = (center_pad_y + offset_y.unwrap_or(0)).max(0) as usize;

    for row in 0..target_rows {
        let content_line = lines.get(row).map(|s| s.as_str()).unwrap_or("");
        let mut row_pixels = Vec::with_capacity(target_cols);
        for col in 0..target_cols {
            if col >= start_col
                && col < start_col + fit_cols
                && row >= start_row
                && row < start_row + fit_rows
            {
                let idx = ((row - start_row) * fit_cols + (col - start_col)) * 4;
                row_pixels.push((raw[idx], raw[idx + 1], raw[idx + 2], raw[idx + 3]));
            } else {
                row_pixels.push((0, 0, 0, 0));
            }
        }
        result.push(apply_row_background(content_line, &row_pixels, target_cols));
    }

    Some(result)
}

fn apply_row_background(
    content: &str,
    row_pixels: &[(u8, u8, u8, u8)],
    target_width: usize,
) -> String {
    use unicode_width::UnicodeWidthChar;

    let mut out = String::with_capacity(content.len() + target_width * 20);
    let mut col = 0usize;
    let mut cur_bg: Option<(u8, u8, u8)> = None;
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            let mut seq = String::from('\x1b');
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '[' {
                    seq.push(chars.next().unwrap());
                    while let Some(&c) = chars.peek() {
                        seq.push(chars.next().unwrap());
                        if c.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
            }
            out.push_str(&seq);
            if seq == "\x1b[0m" || seq == "\x1b[m" {
                cur_bg = None;
            }
            continue;
        }

        let ch_width = ch.width().unwrap_or(1);
        if col < target_width {
            if let Some(&(r, g, b, a)) = row_pixels.get(col) {
                if a > 20 {
                    if cur_bg != Some((r, g, b)) {
                        let _ = write!(out, "\x1b[48;2;{r};{g};{b}m");
                        cur_bg = Some((r, g, b));
                    }
                } else if cur_bg.is_some() {
                    out.push_str("\x1b[49m");
                    cur_bg = None;
                }
            }
        }
        out.push(ch);
        col += ch_width;
    }

    while col < target_width {
        if let Some(&(r, g, b, a)) = row_pixels.get(col) {
            if a > 20 {
                if cur_bg != Some((r, g, b)) {
                    let _ = write!(out, "\x1b[48;2;{r};{g};{b}m");
                    cur_bg = Some((r, g, b));
                }
                out.push(' ');
            } else {
                if cur_bg.is_some() {
                    out.push_str("\x1b[49m");
                    cur_bg = None;
                }
                out.push(' ');
            }
        } else {
            out.push(' ');
        }
        col += 1;
    }
    out.push_str("\x1b[0m");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_halfblock_rasterizes_a_repository_image() {
        let image = format!("{}/../images/arch.png", env!("CARGO_MANIFEST_DIR"));
        let lines = render_halfblock(&image, 4).expect("repository image must render");

        assert!(!lines.is_empty());
        assert!(lines.iter().any(|line| line.contains("\x1b[")));
    }

    #[test]
    fn test_apply_image_background_to_lines() {
        let image = format!("{}/../images/arch.png", env!("CARGO_MANIFEST_DIR"));
        let input_lines = vec![
            "\x1b[34mOS:\x1b[0m Arch Linux".to_string(),
            "\x1b[32mHost:\x1b[0m Laptop".to_string(),
        ];
        let bg_lines = apply_image_background_to_lines(&input_lines, &image, 30, 4)
            .expect("should apply background");
        assert_eq!(bg_lines.len(), 4);
        for line in &bg_lines {
            assert!(line.contains("\x1b[48;2;"));
            assert!(line.ends_with("\x1b[0m"));
        }
        fn strip_ansi(s: &str) -> String {
            let mut in_escape = false;
            let mut clean = String::new();
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

        let stripped0 = strip_ansi(&bg_lines[0]);
        assert!(stripped0.contains("OS: Arch Linux"));
        let stripped1 = strip_ansi(&bg_lines[1]);
        assert!(stripped1.contains("Host: Laptop"));
    }

    #[test]
    fn test_render_background_valid() {
        let image = format!("{}/../images/arch.png", env!("CARGO_MANIFEST_DIR"));
        assert!(render_background(&image, 40, 10, -1));
    }

    #[test]
    fn test_render_ex_respects_max_rows() {
        let image = format!("{}/../images/arch.png", env!("CARGO_MANIFEST_DIR"));
        if let Some((cols, rows)) = render_ex(&image, 40, Some(10)) {
            assert!(rows <= 10, "Rendered rows {} should be <= 10", rows);
            assert!(cols <= 40, "Rendered cols {} should be <= 40", cols);
        }
    }
}
