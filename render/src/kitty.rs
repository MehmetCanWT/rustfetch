use base64::prelude::*;

pub fn print_kitty(rgba_data: &[u8], width: u32, height: u32) {
    let encoded = BASE64_STANDARD.encode(rgba_data);
    let chunks: Vec<&str> = encoded
        .as_bytes()
        .chunks(4096)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let m = if i == chunks.len() - 1 { 0 } else { 1 };
        if i == 0 {
            print!(
                "\x1b_Ga=T,q=2,f=32,s={},v={},m={};{}\x1b\\",
                width, height, m, chunk
            );
        } else {
            print!("\x1b_Gm={},q=2;{}\x1b\\", m, chunk);
        }
    }
}

pub struct KittyFrame<'a> {
    pub rgba_data: &'a [u8],
    pub delay_ms: u32,
}

pub fn print_kitty_animation(frames: &[KittyFrame], width: u32, height: u32) {
    if frames.is_empty() {
        return;
    }

    let image_id = 1;

    for (frame_idx, frame) in frames.iter().enumerate() {
        let encoded = BASE64_STANDARD.encode(frame.rgba_data);
        let chunks: Vec<&str> = encoded
            .as_bytes()
            .chunks(4096)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect();

        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            let m = if chunk_idx == chunks.len() - 1 { 0 } else { 1 };

            if chunk_idx == 0 {
                let a = if frame_idx == 0 { 'T' } else { 'f' };
                print!(
                    "\x1b_Ga={},q=2,i={},f=32,s={},v={},z={},m={};{}\x1b\\",
                    a, image_id, width, height, frame.delay_ms, m, chunk
                );
            } else if frame_idx == 0 {
                print!("\x1b_Gm={},q=2;{}\x1b\\", m, chunk);
            } else {
                print!("\x1b_Ga=f,q=2,i={},m={};{}\x1b\\", image_id, m, chunk);
            }
        }
    }

    print!("\x1b_Ga=a,q=2,i={},s=3,v=1\x1b\\", image_id);
}
