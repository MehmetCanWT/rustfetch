//! Helpers for image discovery, random selection with LRU cycling, and color extraction.

use rustfetch_core::cache::Cache;

pub fn tilde_expand(path: &str) -> std::borrow::Cow<'_, str> {
    if path == "~" {
        if let Ok(home) = std::env::var("HOME") {
            return std::borrow::Cow::Owned(home);
        }
    } else if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            let home_trimmed = home.trim_end_matches('/');
            return std::borrow::Cow::Owned(format!("{home_trimmed}/{stripped}"));
        }
    }
    std::borrow::Cow::Borrowed(path)
}

pub fn filter_candidate_images<'a>(files: &'a [String], recent: &[String]) -> Vec<&'a String> {
    let n = files.len();
    let exclude_count = match n {
        0 | 1 => 0,
        2 => 1,
        3 => 2,
        _ => 3,
    };

    let mut relevant_recent: Vec<&str> = Vec::new();
    for r in recent {
        let r_str = r.as_str();
        let r_name = std::path::Path::new(r_str).file_name();
        let matches = files.iter().any(|f| {
            f.as_str() == r_str
                || (r_name.is_some() && std::path::Path::new(f.as_str()).file_name() == r_name)
        });
        if matches && !relevant_recent.contains(&r_str) {
            relevant_recent.push(r_str);
        }
    }

    let excluded: Vec<&str> = relevant_recent.into_iter().take(exclude_count).collect();

    let candidates: Vec<&String> = files
        .iter()
        .filter(|f| {
            let f_str = f.as_str();
            let f_name = std::path::Path::new(f_str).file_name();
            !excluded.iter().any(|&ex| {
                ex == f_str || (f_name.is_some() && std::path::Path::new(ex).file_name() == f_name)
            })
        })
        .collect();

    if candidates.is_empty() {
        files.iter().collect()
    } else {
        candidates
    }
}

pub fn update_recent_history(recent: &[String], chosen: &str) -> Vec<String> {
    let mut updated = Vec::with_capacity(3);
    updated.push(chosen.to_string());
    for r in recent {
        if r != chosen && updated.len() < 3 {
            updated.push(r.clone());
        }
    }
    updated
}

pub fn pick_random_image(dir_path: &str, keep_last: bool) -> Option<String> {
    use std::fs;
    let cache_dir = Cache::cache_dir();
    let last_logo_file = cache_dir.join("last_logo");
    let recent_images_file = cache_dir.join("recent_images");

    let expanded = tilde_expand(dir_path);
    let expanded_path = std::path::Path::new(expanded.as_ref());
    if !expanded_path.is_dir() {
        return None;
    }

    let entries = fs::read_dir(expanded_path).ok()?;
    let mut files: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let Some(name_str) = file_name.to_str() else {
            continue;
        };
        let is_image_ext = std::path::Path::new(name_str)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                ext.eq_ignore_ascii_case("png")
                    || ext.eq_ignore_ascii_case("jpg")
                    || ext.eq_ignore_ascii_case("jpeg")
                    || ext.eq_ignore_ascii_case("gif")
                    || ext.eq_ignore_ascii_case("webp")
            })
            .unwrap_or(false);
        if !is_image_ext {
            continue;
        }
        let is_file = entry
            .file_type()
            .map(|ft| ft.is_file() || (ft.is_symlink() && entry.path().is_file()))
            .unwrap_or(false);
        if is_file {
            if let Some(s) = entry.path().to_str() {
                files.push(s.to_string());
            }
        }
    }
    if files.is_empty() {
        return None;
    }
    files.sort();

    let mut recent: Vec<String> = fs::read_to_string(&recent_images_file)
        .ok()
        .map(|content| {
            content
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .take(3)
                .collect()
        })
        .unwrap_or_default();

    if recent.is_empty() {
        if let Ok(last) = fs::read_to_string(&last_logo_file) {
            let trimmed = last.trim();
            if !trimmed.is_empty() {
                recent.push(trimmed.to_string());
            }
        }
    }

    if keep_last {
        if let Some(first) = recent.first() {
            let first_name = std::path::Path::new(first).file_name();
            if let Some(matched) = files.iter().find(|f| {
                f.as_str() == first.as_str()
                    || (first_name.is_some()
                        && std::path::Path::new(f.as_str()).file_name() == first_name)
            }) {
                return Some(matched.clone());
            }
        }
    }

    let candidates = filter_candidate_images(&files, &recent);

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let chosen = candidates[(nanos as usize) % candidates.len()].clone();

    let updated_recent = update_recent_history(&recent, &chosen);

    let _ = fs::create_dir_all(&cache_dir);
    let _ = fs::write(&last_logo_file, chosen.as_bytes());
    let _ = fs::write(&recent_images_file, updated_recent.join("\n").as_bytes());

    Some(chosen)
}

#[allow(clippy::type_complexity)]
pub fn get_image_color_and_palette(
    image_path: &str,
    auto_color: bool,
    want_palette: bool,
) -> (Option<String>, Option<Vec<(u8, u8, u8)>>) {
    let color_path = format!("{image_path}.color");
    let mut cached_dom = None;
    let mut cached_pal = None;

    if let Ok(content) = std::fs::read_to_string(&color_path) {
        let mut lines = content.lines();
        if let Some(first) = lines.next().map(str::trim) {
            if first.starts_with('#') && (first.len() == 7 || first.len() == 4) {
                cached_dom = Some(first.to_string());
            }
        }
        if let Some(second) = lines.next().map(str::trim) {
            let parsed: Vec<(u8, u8, u8)> = second
                .split(',')
                .filter_map(|s| rustfetch_render::parse_color(s.trim()))
                .collect();
            if !parsed.is_empty() {
                cached_pal = Some(parsed);
            }
        }
    }

    let need_dom = auto_color && cached_dom.is_none();
    let need_pal = want_palette && cached_pal.is_none();

    if need_dom || need_pal {
        if let Some((dom, pal)) = rustfetch_render::extract_dominant_and_palette(image_path, 16) {
            let final_dom = cached_dom.or(Some(dom));
            let final_pal = cached_pal.or(pal);
            if let Some(ref d) = final_dom {
                let mut out = d.clone();
                if let Some(ref p) = final_pal {
                    let pal_hex: Vec<String> = p
                        .iter()
                        .map(|(r, g, b)| format!("#{:02x}{:02x}{:02x}", r, g, b))
                        .collect();
                    if !pal_hex.is_empty() {
                        out.push('\n');
                        out.push_str(&pal_hex.join(","));
                    }
                }
                let _ = std::fs::write(&color_path, out.as_bytes());
            }
            return (
                if auto_color { final_dom } else { None },
                if want_palette { final_pal } else { None },
            );
        }
    }

    (
        if auto_color { cached_dom } else { None },
        if want_palette { cached_pal } else { None },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilde_expand() {
        assert_eq!(tilde_expand("foo/bar"), "foo/bar");
        assert_eq!(tilde_expand("/etc/os-release"), "/etc/os-release");
        if let Ok(home) = std::env::var("HOME") {
            let trimmed = home.trim_end_matches('/');
            assert_eq!(tilde_expand("~"), home);
            assert_eq!(tilde_expand("~/foo/bar"), format!("{trimmed}/foo/bar"));
        }
    }

    #[test]
    fn test_pick_random_image() {
        let tmp = std::env::temp_dir().join(format!("rustfetch_img_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        // Non-images that should be ignored:
        std::fs::write(tmp.join("png"), b"not an image").unwrap();
        std::fs::write(tmp.join(".png"), b"hidden file").unwrap();
        std::fs::write(tmp.join("readme.txt"), b"text").unwrap();
        assert!(pick_random_image(tmp.to_str().unwrap(), false).is_none());

        // Valid image
        std::fs::write(tmp.join("logo.png"), b"pngdata").unwrap();
        let picked = pick_random_image(tmp.to_str().unwrap(), false);
        assert!(picked.is_some());
        assert!(picked.unwrap().ends_with("logo.png"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_candidate_filtering_rules() {
        // N = 1: picks the single image even if it was recently used
        let files_n1 = vec!["img1.png".to_string()];
        let recent_n1 = vec!["img1.png".to_string()];
        let c_n1 = filter_candidate_images(&files_n1, &recent_n1);
        assert_eq!(c_n1, vec![&files_n1[0]]);

        // N = 2: excludes the last 1 used image, picks the other
        let files_n2 = vec!["img1.png".to_string(), "img2.png".to_string()];
        let recent_n2_a = vec!["img1.png".to_string()];
        let c_n2_a = filter_candidate_images(&files_n2, &recent_n2_a);
        assert_eq!(c_n2_a, vec![&files_n2[1]]); // picks img2

        let recent_n2_b = vec!["img2.png".to_string(), "img1.png".to_string()];
        let c_n2_b = filter_candidate_images(&files_n2, &recent_n2_b);
        assert_eq!(c_n2_b, vec![&files_n2[0]]); // picks img1

        // N = 3: excludes the last 2 used images, picks the 3rd one
        let files_n3 = vec![
            "img1.png".to_string(),
            "img2.png".to_string(),
            "img3.png".to_string(),
        ];
        let recent_n3 = vec!["img2.png".to_string(), "img1.png".to_string()];
        let c_n3 = filter_candidate_images(&files_n3, &recent_n3);
        assert_eq!(c_n3, vec![&files_n3[2]]); // picks img3

        let recent_n3_cycle = vec![
            "img3.png".to_string(),
            "img2.png".to_string(),
            "img1.png".to_string(),
        ];
        let c_n3_cycle = filter_candidate_images(&files_n3, &recent_n3_cycle);
        assert_eq!(c_n3_cycle, vec![&files_n3[0]]); // picks img1

        // N = 4+: excludes the last 3 used images, candidates are the remaining images
        let files_n5 = vec![
            "img1.png".to_string(),
            "img2.png".to_string(),
            "img3.png".to_string(),
            "img4.png".to_string(),
            "img5.png".to_string(),
        ];
        let recent_n5 = vec![
            "img3.png".to_string(),
            "img2.png".to_string(),
            "img1.png".to_string(),
        ];
        let c_n5 = filter_candidate_images(&files_n5, &recent_n5);
        assert_eq!(c_n5, vec![&files_n5[3], &files_n5[4]]); // only img4 and img5
        assert!(!c_n5.iter().any(|&c| recent_n5.contains(c)));

        // Foreign / deleted paths in history must not consume exclusion slots
        let recent_with_deleted = vec![
            "/deleted/old.png".to_string(),
            "img2.png".to_string(),
            "img1.png".to_string(),
        ];
        let c_foreign = filter_candidate_images(&files_n3, &recent_with_deleted);
        assert_eq!(c_foreign, vec![&files_n3[2]]); // correctly excludes img2 and img1, picks img3

        // Duplicate entries in recent history do not burn multiple exclusion slots
        let recent_dups = vec![
            "img2.png".to_string(),
            "img2.png".to_string(),
            "img1.png".to_string(),
        ];
        let c_dups = filter_candidate_images(&files_n3, &recent_dups);
        assert_eq!(c_dups, vec![&files_n3[2]]); // still excludes img2 and img1

        // History update keeps max 3 entries with chosen at front and deduplicates
        let h1 = update_recent_history(
            &["img2.png".to_string(), "img1.png".to_string()],
            "img3.png",
        );
        assert_eq!(h1, vec!["img3.png", "img2.png", "img1.png"]);

        let h2 = update_recent_history(&h1, "img4.png");
        assert_eq!(h2, vec!["img4.png", "img3.png", "img2.png"]);

        let h3 = update_recent_history(&h2, "img3.png");
        assert_eq!(h3, vec!["img3.png", "img4.png", "img2.png"]);
    }
}
