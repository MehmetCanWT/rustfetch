use fast_image_resize as fr;
use fast_image_resize::Image;
use image::{AnimationDecoder, ImageReader};
use std::fmt::Write as _;
use std::fs::File;
use std::io::BufReader;
use std::num::NonZeroU32;

use crate::{kitty, terminal};

pub fn render(path: &str, target_cols: usize) -> Option<usize> {
    let (cell_w, cell_h) = terminal::get_cell_size().unwrap_or((10, 20));
    let target_px_width = target_cols as u32 * cell_w as u32;

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
            let ratio = orig_height as f64 / orig_width as f64;
            let target_px_height = (target_px_width as f64 * ratio).round() as u32;

            if target_px_width == 0 || target_px_height == 0 {
                return None;
            }

            let mut resized_buffers = Vec::new();
            let mut resizer =
                fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Bilinear));

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

            let lines = (target_px_height as f32 / cell_h as f32).ceil() as usize;
            kitty::print_kitty_animation(
                &kitty_frames,
                target_px_width,
                target_px_height,
                target_cols as u32,
                lines as u32,
            );
            return Some(lines);
        }
    }

    let img = img_reader.decode().ok()?;
    let rgba_img = img.to_rgba8();

    let orig_width = rgba_img.width();
    let orig_height = rgba_img.height();
    let ratio = orig_height as f64 / orig_width as f64;
    let target_px_height = (target_px_width as f64 * ratio).round() as u32;

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

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Bilinear));
    resizer
        .resize(&src_image.view(), &mut dst_image.view_mut())
        .ok()?;

    let lines = (target_px_height as f32 / cell_h as f32).ceil() as usize;
    kitty::print_kitty(
        dst_image.buffer(),
        target_px_width,
        target_px_height,
        target_cols as u32,
        lines as u32,
    );

    Some(lines)
}

pub fn render_halfblock(path: &str, target_cols: usize) -> Option<Vec<String>> {
    let img_reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    let img = img_reader.decode().ok()?.to_rgba8();

    let orig_width = img.width();
    let orig_height = img.height();
    if orig_width == 0 || orig_height == 0 || target_cols == 0 {
        return None;
    }

    let ratio = orig_height as f64 / orig_width as f64;
    let target_px_height = ((target_cols as f64 * ratio).round() as u32).max(2);
    let target_px_height = if !target_px_height.is_multiple_of(2) {
        target_px_height + 1
    } else {
        target_px_height
    };

    let src_width = NonZeroU32::new(orig_width)?;
    let src_height = NonZeroU32::new(orig_height)?;
    let src_image =
        Image::from_vec_u8(src_width, src_height, img.into_raw(), fr::PixelType::U8x4).ok()?;

    let dst_width = NonZeroU32::new(target_cols as u32)?;
    let dst_height = NonZeroU32::new(target_px_height)?;
    let mut dst_image = Image::new(dst_width, dst_height, fr::PixelType::U8x4);

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Bilinear));
    resizer
        .resize(&src_image.view(), &mut dst_image.view_mut())
        .ok()?;

    let raw = dst_image.into_vec();
    let width = target_cols;
    let height = target_px_height as usize;
    let mut lines = Vec::with_capacity(height / 2);

    for y in (0..height).step_by(2) {
        // A TrueColor half-block can contain two SGR sequences, the glyph, and
        // a reset. Reserving once avoids one allocation per source pixel.
        let mut line = String::with_capacity(width * 40);
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
                line.push(' ');
            } else if a1 < 30 {
                let _ = write!(line, "\x1b[38;2;{r2};{g2};{b2}m▄\x1b[0m");
            } else if a2 < 30 {
                let _ = write!(line, "\x1b[38;2;{r1};{g1};{b1}m▀\x1b[0m");
            } else {
                let _ = write!(line, "\x1b[38;2;{r1};{g1};{b1}m\x1b[48;2;{r2};{g2};{b2}m▀");
            }
        }
        line.push_str("\x1b[0m");
        lines.push(line);
    }

    Some(lines)
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
}
