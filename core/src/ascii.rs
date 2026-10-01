use std::fs;
use std::io;
use std::path::Path;

/// Natural sort key chunk.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum KeyChunk {
    Num(u64),
    Str(String),
}

/// Computes a natural sorting key for a filename (e.g. "frame_2.txt" < "frame_10.txt").
fn natural_sort_key(s: &str) -> Vec<KeyChunk> {
    let mut chunks = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            let mut num_str = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    num_str.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            let n = num_str.parse::<u64>().unwrap_or(0);
            chunks.push(KeyChunk::Num(n));
        } else {
            let mut text = String::new();
            while let Some(&t) = chars.peek() {
                if !t.is_ascii_digit() {
                    text.push(t.to_ascii_lowercase());
                    chars.next();
                } else {
                    break;
                }
            }
            chunks.push(KeyChunk::Str(text));
        }
    }

    chunks
}

/// Reads a static ASCII art file into lines.
pub fn load_ascii_lines(path: &Path) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

/// Determines whether a line acts as a frame separator in multi-frame ASCII animation files.
pub fn is_frame_delimiter(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }
    trimmed.starts_with("===")
        || trimmed.starts_with("---")
        || trimmed == "\x0c"
        || trimmed == "[frame]"
        || trimmed == "===FRAME==="
        || trimmed == "---FRAME---"
}

/// Parses multi-frame ASCII content from a single string.
pub fn parse_ascii_frames_from_str(content: &str) -> Vec<Vec<String>> {
    // If form-feed (\x0c) exists, split by form-feed first
    if content.contains('\x0c') {
        let parts: Vec<&str> = content.split('\x0c').collect();
        let frames: Vec<Vec<String>> = parts
            .into_iter()
            .map(|part| part.lines().map(|s| s.to_string()).collect())
            .filter(|lines: &Vec<String>| !lines.is_empty())
            .collect();
        if !frames.is_empty() {
            return frames;
        }
    }

    let mut frames = Vec::new();
    let mut current_frame = Vec::new();
    let mut has_delimiters = false;

    for line in content.lines() {
        if is_frame_delimiter(line) {
            has_delimiters = true;
            if !current_frame.is_empty() {
                frames.push(std::mem::take(&mut current_frame));
            }
        } else {
            current_frame.push(line.to_string());
        }
    }

    if !current_frame.is_empty() {
        frames.push(current_frame);
    }

    // If no delimiters were encountered, return the single frame (or empty if file was empty)
    if !has_delimiters && frames.is_empty() {
        return vec![content.lines().map(|s| s.to_string()).collect()];
    }

    frames
}

/// Loads ASCII animation frames from either a single multi-frame file or a directory of frame files.
pub fn load_ascii_frames(path: &Path) -> io::Result<Vec<Vec<String>>> {
    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                if let Ok(ft) = e.file_type() {
                    ft.is_file()
                } else {
                    false
                }
            })
            .collect();

        if entries.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("No animation frames found in directory: {}", path.display()),
            ));
        }

        // Sort entries naturally (e.g. frame_1.txt before frame_10.txt)
        entries.sort_by_key(|e| natural_sort_key(&e.file_name().to_string_lossy()));

        let mut frames = Vec::with_capacity(entries.len());
        for entry in entries {
            let lines = load_ascii_lines(&entry.path())?;
            if !lines.is_empty() {
                frames.push(lines);
            }
        }

        if frames.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "All frame files in directory were empty: {}",
                    path.display()
                ),
            ));
        }

        Ok(frames)
    } else {
        let content = fs::read_to_string(path)?;
        let frames = parse_ascii_frames_from_str(&content);
        if frames.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("No valid ASCII frames parsed from: {}", path.display()),
            ));
        }
        Ok(frames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_sort_order() {
        let mut list = vec![
            "frame_10.txt",
            "frame_1.txt",
            "frame_2.txt",
            "frame_20.txt",
            "frame_03.txt",
        ];
        list.sort_by_key(|a| natural_sort_key(a));
        assert_eq!(
            list,
            vec![
                "frame_1.txt",
                "frame_2.txt",
                "frame_03.txt",
                "frame_10.txt",
                "frame_20.txt"
            ]
        );
    }

    #[test]
    fn test_parse_ascii_frames_delimiter() {
        let content = "Frame 1 Line 1\nFrame 1 Line 2\n===FRAME===\nFrame 2 Line 1\n===FRAME===\nFrame 3 Line 1";
        let frames = parse_ascii_frames_from_str(content);
        assert_eq!(frames.len(), 3);
        assert_eq!(frames[0], vec!["Frame 1 Line 1", "Frame 1 Line 2"]);
        assert_eq!(frames[1], vec!["Frame 2 Line 1"]);
        assert_eq!(frames[2], vec!["Frame 3 Line 1"]);
    }

    #[test]
    fn test_parse_ascii_frames_form_feed() {
        let content = "Page 1\nLine 2\x0cPage 2\nLine 2";
        let frames = parse_ascii_frames_from_str(content);
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0], vec!["Page 1", "Line 2"]);
        assert_eq!(frames[1], vec!["Page 2", "Line 2"]);
    }

    #[test]
    fn test_parse_single_frame() {
        let content = "Single Frame\nLine 2";
        let frames = parse_ascii_frames_from_str(content);
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0], vec!["Single Frame", "Line 2"]);
    }

    #[test]
    fn test_load_ascii_frames_dir() {
        let temp = std::env::temp_dir().join("rustfetch_test_anim_dir");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        fs::write(temp.join("02.txt"), "Frame 2\n").unwrap();
        fs::write(temp.join("01.txt"), "Frame 1\n").unwrap();
        fs::write(temp.join("10.txt"), "Frame 10\n").unwrap();

        let frames = load_ascii_frames(&temp).unwrap();
        assert_eq!(frames.len(), 3);
        assert_eq!(frames[0], vec!["Frame 1"]);
        assert_eq!(frames[1], vec!["Frame 2"]);
        assert_eq!(frames[2], vec!["Frame 10"]);

        let _ = fs::remove_dir_all(&temp);
    }
}
