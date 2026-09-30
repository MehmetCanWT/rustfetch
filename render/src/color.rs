use fast_image_resize as fr;
use fast_image_resize::Image;
use image::ImageReader;
use std::collections::HashMap;
use std::num::NonZeroU32;

fn load_and_sample_image(path: &str) -> Option<HashMap<(u8, u8, u8), u32>> {
    let img_reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    let img = img_reader.decode().ok()?;
    let rgba_img = img.to_rgba8();

    let orig_w = rgba_img.width();
    let orig_h = rgba_img.height();
    if orig_w == 0 || orig_h == 0 {
        return None;
    }

    let composited_raw: Vec<u8> = rgba_img
        .pixels()
        .flat_map(|pixel| {
            let a = pixel[3] as f32 / 255.0;
            [
                (pixel[0] as f32 * a + 20.0 * (1.0 - a)).round() as u8,
                (pixel[1] as f32 * a + 20.0 * (1.0 - a)).round() as u8,
                (pixel[2] as f32 * a + 20.0 * (1.0 - a)).round() as u8,
                255,
            ]
        })
        .collect();

    let src_w = NonZeroU32::new(orig_w)?;
    let src_h = NonZeroU32::new(orig_h)?;
    let src_image = Image::from_vec_u8(src_w, src_h, composited_raw, fr::PixelType::U8x4).ok()?;

    let target_w = NonZeroU32::new(60)?;
    let target_h = NonZeroU32::new(60)?;
    let mut dst_image = Image::new(target_w, target_h, fr::PixelType::U8x4);

    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Bilinear));
    resizer
        .resize(&src_image.view(), &mut dst_image.view_mut())
        .ok()?;

    let buffer = dst_image.buffer();
    let mut color_counts: HashMap<(u8, u8, u8), u32> = HashMap::new();
    for &[r, g, b, _] in buffer.as_chunks::<4>().0 {
        let qr = (r / 4) * 4;
        let qg = (g / 4) * 4;
        let qb = (b / 4) * 4;
        *color_counts.entry((qr, qg, qb)).or_insert(0) += 1;
    }
    Some(color_counts)
}

fn ensure_visible(c: (u8, u8, u8)) -> (u8, u8, u8) {
    let lum = 0.299 * c.0 as f64 + 0.587 * c.1 as f64 + 0.114 * c.2 as f64;
    if lum < 85.0 {
        let factor = 110.0 / lum.max(1.0);
        (
            ((c.0 as f64 * factor).round() as u32).min(255) as u8,
            ((c.1 as f64 * factor).round() as u32).min(255) as u8,
            ((c.2 as f64 * factor).round() as u32).min(255) as u8,
        )
    } else {
        c
    }
}

pub fn extract_dominant_color(path: &str) -> Option<String> {
    let color_counts = load_and_sample_image(path)?;

    let mut candidates: Vec<(f64, (u8, u8, u8))> = Vec::new();
    for (&(r, g, b), &count) in &color_counts {
        let r_f = r as f64 / 255.0;
        let g_f = g as f64 / 255.0;
        let b_f = b as f64 / 255.0;

        let max = r_f.max(g_f).max(b_f);
        let min = r_f.min(g_f).min(b_f);
        let delta = max - min;

        let v = max;
        let s = if max > 0.0 { delta / max } else { 0.0 };

        if s > 0.25 && v > 0.35 && v < 0.98 {
            let lum = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) / 255.0;
            let score = (count as f64).powf(0.6)
                * s.powf(1.6)
                * v.powf(0.8)
                * (1.0 - (lum - 0.6).abs() * 0.7);
            candidates.push((score, (r, g, b)));
        }
    }

    candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    if let Some(&(_, (mut r, mut g, mut b))) = candidates.first() {
        let lum = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64;
        if lum < 110.0 {
            let factor = 135.0 / lum.max(1.0);
            r = ((r as f64 * factor).round() as u32).min(255) as u8;
            g = ((g as f64 * factor).round() as u32).min(255) as u8;
            b = ((b as f64 * factor).round() as u32).min(255) as u8;
        }
        return Some(format!("#{:02x}{:02x}{:02x}", r, g, b));
    }

    Some("#5485b6".to_string())
}

pub fn extract_color_palette(path: &str, count: usize) -> Option<Vec<(u8, u8, u8)>> {
    let color_counts = load_and_sample_image(path)?;
    if color_counts.is_empty() {
        return None;
    }

    let mut scored_colors: Vec<(f64, (u8, u8, u8))> = Vec::new();
    for (&(r, g, b), &cnt) in &color_counts {
        let r_f = r as f64 / 255.0;
        let g_f = g as f64 / 255.0;
        let b_f = b as f64 / 255.0;

        let max = r_f.max(g_f).max(b_f);
        let min = r_f.min(g_f).min(b_f);
        let delta = max - min;
        let s = if max > 0.0 { delta / max } else { 0.0 };
        let v = max;
        let lum = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) / 255.0;

        if v > 0.12 && lum < 0.96 {
            let score = (cnt as f64).powf(0.5) * (1.0 + s * 1.5) * (1.0 - (lum - 0.55).abs() * 0.5);
            scored_colors.push((score, (r, g, b)));
        }
    }

    scored_colors.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    if scored_colors.is_empty() {
        return None;
    }

    let mut palette: Vec<(u8, u8, u8)> = Vec::with_capacity(count);
    palette.push(ensure_visible(scored_colors[0].1));

    let color_dist = |c1: (u8, u8, u8), c2: (u8, u8, u8)| -> f64 {
        let dr = c1.0 as f64 - c2.0 as f64;
        let dg = c1.1 as f64 - c2.1 as f64;
        let db = c1.2 as f64 - c2.2 as f64;
        (2.0 * dr * dr + 4.0 * dg * dg + 3.0 * db * db).sqrt()
    };

    while palette.len() < count {
        let mut best_candidate = None;
        let mut best_min_dist = -1.0;

        for &(_, candidate) in scored_colors.iter().take(250) {
            let visible = ensure_visible(candidate);
            let min_dist = palette
                .iter()
                .map(|&p| color_dist(p, visible))
                .fold(f64::INFINITY, f64::min);

            if min_dist > best_min_dist && min_dist > 35.0 {
                best_min_dist = min_dist;
                best_candidate = Some(visible);
            }
        }

        if let Some(cand) = best_candidate {
            palette.push(cand);
        } else {
            break;
        }
    }

    let mut i = 0;
    while palette.len() < count && i < palette.len() {
        let base = palette[i];
        let lightened = (
            ((base.0 as f64 * 1.35).min(255.0)) as u8,
            ((base.1 as f64 * 1.35).min(255.0)) as u8,
            ((base.2 as f64 * 1.35).min(255.0)) as u8,
        );
        if !palette.contains(&lightened) {
            palette.push(lightened);
        }
        i += 1;
    }

    Some(palette)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_dominant_color_nonexistent() {
        assert!(extract_dominant_color("nonexistent_image_12345.png").is_none());
    }

    #[test]
    fn test_extract_dominant_color_real_image() {
        if let Some(color) = extract_dominant_color("../scratch/test2.jpg") {
            assert!(color.starts_with('#'));
            assert_eq!(color.len(), 7);
        }
    }

    #[test]
    fn test_extract_palette_real_image() {
        if let Some(pal) = extract_color_palette("../scratch/test2.jpg", 16) {
            assert!(!pal.is_empty());
            assert!(pal.len() <= 16);
        }
    }
}
