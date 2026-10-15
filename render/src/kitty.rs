use base64::prelude::*;
use std::fmt::Write as _;
use std::io::Write;

pub fn print_kitty_ex(
    rgba_data: &[u8],
    width: u32,
    height: u32,
    cols: u32,
    rows: u32,
    z_index: i32,
    prevent_cursor_move: bool,
) {
    if rgba_data.is_empty() || width == 0 || height == 0 || cols == 0 || rows == 0 {
        return;
    }

    let compressed = miniz_oxide::deflate::compress_to_vec_zlib(rgba_data, 6);
    let use_compression = compressed.len() < rgba_data.len();
    let data_to_encode = if use_compression {
        &compressed[..]
    } else {
        rgba_data
    };

    let encoded = BASE64_STANDARD.encode(data_to_encode);
    let chunks: Vec<&str> = encoded
        .as_bytes()
        .chunks(4096)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let m = if i == chunks.len() - 1 { 0 } else { 1 };
        if i == 0 {
            let mut cmd = format!("\x1b_Ga=T,c={cols},r={rows},q=2,f=32,s={width},v={height}");
            if use_compression {
                cmd.push_str(",o=z");
            }
            if z_index != 0 {
                let _ = write!(cmd, ",z={z_index}");
            }
            if prevent_cursor_move {
                cmd.push_str(",C=1");
            }
            let _ = write!(cmd, ",m={m};{chunk}\x1b\\");
            print!("{cmd}");
        } else {
            print!("\x1b_Gm={m},q=2;{chunk}\x1b\\");
        }
    }
    let _ = std::io::stdout().flush();
}

pub fn print_kitty_png(
    png_bytes: &[u8],
    cols: u32,
    rows: u32,
    z_index: i32,
    prevent_cursor_move: bool,
) {
    if png_bytes.is_empty() || cols == 0 || rows == 0 {
        return;
    }

    let encoded = BASE64_STANDARD.encode(png_bytes);
    let chunks: Vec<&str> = encoded
        .as_bytes()
        .chunks(4096)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let m = if i == chunks.len() - 1 { 0 } else { 1 };
        if i == 0 {
            let mut cmd = format!("\x1b_Ga=T,c={cols},r={rows},q=2,f=100");
            if z_index != 0 {
                let _ = write!(cmd, ",z={z_index}");
            }
            if prevent_cursor_move {
                cmd.push_str(",C=1");
            }
            let _ = write!(cmd, ",m={m};{chunk}\x1b\\");
            print!("{cmd}");
        } else {
            print!("\x1b_Gm={m},q=2;{chunk}\x1b\\");
        }
    }
    let _ = std::io::stdout().flush();
}

pub fn print_kitty(rgba_data: &[u8], width: u32, height: u32, cols: u32, rows: u32) {
    print_kitty_ex(rgba_data, width, height, cols, rows, 0, false);
}

pub struct KittyFrame<'a> {
    pub rgba_data: &'a [u8],
    pub delay_ms: u32,
}

pub fn print_kitty_animation(frames: &[KittyFrame], width: u32, height: u32, cols: u32, rows: u32) {
    if frames.is_empty() {
        return;
    }

    let image_id = 1;

    for (frame_idx, frame) in frames.iter().enumerate() {
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(frame.rgba_data, 6);
        let use_compression = compressed.len() < frame.rgba_data.len();
        let data_to_encode = if use_compression {
            &compressed[..]
        } else {
            frame.rgba_data
        };
        let encoded = BASE64_STANDARD.encode(data_to_encode);
        let chunks: Vec<&str> = encoded
            .as_bytes()
            .chunks(4096)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect();

        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            let m = if chunk_idx == chunks.len() - 1 { 0 } else { 1 };

            if chunk_idx == 0 {
                let a = if frame_idx == 0 { 'T' } else { 'f' };
                let o = if use_compression { ",o=z" } else { "" };
                print!(
                    "\x1b_Ga={},c={},r={},q=2,i={},f=32{},s={},v={},z={},m={};{}\x1b\\",
                    a, cols, rows, image_id, o, width, height, frame.delay_ms, m, chunk
                );
            } else if frame_idx == 0 {
                print!("\x1b_Gm={},q=2;{}\x1b\\", m, chunk);
            } else {
                print!("\x1b_Ga=f,q=2,i={},m={};{}\x1b\\", image_id, m, chunk);
            }
        }
    }

    print!("\x1b_Ga=a,q=2,i={},s=3,v=1\x1b\\", image_id);
    let _ = std::io::stdout().flush();
}

pub fn clear_kitty_images() {
    if !crate::terminal::supports_kitty() {
        return;
    }
    print!("\x1b_Ga=d,d=a\x1b\\");
    let _ = std::io::stdout().flush();
}
