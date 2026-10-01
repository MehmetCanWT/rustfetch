/// Shading mode determining how sub-cells are sampled and rasterized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShadingMode {
    /// 1x1 sub-cell using standard ASCII characters: .,-~:;=!*#$@
    #[default]
    Ascii,
    /// 2x2 sub-cell using quadrant block elements: ▘▝▀▖▌▞▛▗▚▐▜▄▙▟█
    Blocks,
    /// 2x3 sub-cell using Unicode legacy computing sextant blocks (U+1FB00..U+1FB3B)
    Sextants,
}

impl ShadingMode {
    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "blocks" | "block" | "quadrants" => Self::Blocks,
            "sextants" | "sextant" => Self::Sextants,
            _ => Self::Ascii,
        }
    }

    pub fn sub_grid(&self) -> (usize, usize) {
        match self {
            Self::Ascii => (1, 1),
            Self::Blocks => (2, 2),
            Self::Sextants => (3, 2), // (rows, cols)
        }
    }
}

pub const RAMP_ASCII: &str = ".,-~:;=!*#$@";
pub const RAMP_BLOCKS: &str = "░▒▓█";

/// Quadrant block glyphs indexed by 4-bit bitmask (bit 0 = top-left .. bit 3 = bottom-right).
const QUADRANT_GLYPHS: [&str; 16] = [
    " ", "▘", "▝", "▀", "▖", "▌", "▞", "▛", "▗", "▚", "▐", "▜", "▄", "▙", "▟", "█",
];

/// Precomputed table of 64 Unicode sextant glyphs (U+1FB00..U+1FB3B).
fn build_sextant_table() -> Vec<String> {
    let mut table = Vec::with_capacity(64);
    for mask in 0..64 {
        if mask == 0 {
            table.push(" ".to_string());
        } else if mask == 63 {
            table.push("█".to_string());
        } else if mask == 21 {
            table.push("▌".to_string());
        } else if mask == 42 {
            table.push("▐".to_string());
        } else {
            let mut cp = 0x1FB00 + mask - 1;
            if mask > 21 {
                cp -= 1;
            }
            if mask > 42 {
                cp -= 1;
            }
            let s = std::char::from_u32(cp as u32)
                .map(|c| c.to_string())
                .unwrap_or_else(|| " ".to_string());
            table.push(s);
        }
    }
    table
}

/// Computes ink density / height for a UTF-8 character.
pub fn char_weight_utf8(ch: char) -> f32 {
    let u = ch as u32;

    // Single-byte ASCII
    if u < 0x80 {
        return match ch {
            'M' => 1.00,
            'N' => 0.88,
            'm' | 'W' => 0.76,
            'd' | 'B' | 'Q' | 'R' => 0.66,
            'h' | 'b' | 'p' | 'q' | 'k' => 0.56,
            'y' | 'g' | 'w' => 0.46,
            'o' | 'n' | 'u' | 'v' | 'x' | 'z' => 0.38,
            's' | 'a' | 'e' | 'c' => 0.30,
            '+' | '*' | '#' => 0.22,
            ':' | ';' => 0.18,
            '=' | '~' => 0.22,
            '-' | '_' => 0.14,
            '`' | '^' => 0.08,
            '.' | ',' => 0.10,
            '/' | '\\' | '|' => 0.16,
            '\'' | '"' => 0.06,
            ' ' => 0.0,
            'A'..='Z' => 0.70,
            'a'..='z' => 0.45,
            '0'..='9' => 0.45,
            _ => 0.20,
        };
    }

    // Common Unicode blocks
    match u {
        // Full block █
        0x2588 => 1.00,
        // Shades: Dark ▓, Medium ▒, Light ░
        0x2593 => 0.75,
        0x2592 => 0.50,
        0x2591 => 0.25,
        // Half blocks: ▀ ▄ ▌ ▐
        0x2580 | 0x2584 | 0x258C | 0x2590 => 0.50,
        // 3/4 blocks: ▛ ▜ ▙ ▟
        0x259B | 0x259C | 0x2599 | 0x259F => 0.75,
        // 1/4 blocks: ▖ ▗ ▘ ▝
        0x2596 | 0x2597 | 0x2598 | 0x259D => 0.25,
        // Box drawing characters (0x2500..0x257F)
        0x2500..=0x257F => 0.22,
        // Braille patterns (0x2800..0x28FF)
        0x2800..=0x28FF => {
            let dots = (u & 0xFF).count_ones() as f32;
            (dots / 8.0).max(0.12)
        }
        _ => 0.35,
    }
}

/// A 3D vertex with normal and optional ANSI color string.
#[derive(Debug, Clone)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub nx: f32,
    pub ny: f32,
    pub nz: f32,
    pub color: Option<String>,
    pub is_outer_side: bool,
}

/// A parsed 3D logo model ready for rotation and rasterization.
#[derive(Debug, Clone)]
pub struct Model3D {
    pub points: Vec<Point3D>,
    pub orig_rows: usize,
    pub orig_cols: usize,
    pub has_ansi_colors: bool,
}

impl Model3D {
    /// Constructs a 3D relief model from lines of ASCII art with optional ANSI escape codes.
    pub fn from_ascii_lines(
        lines: &[String],
        size_scale: f32,
        depth_scale: f32,
        shading_mode: ShadingMode,
    ) -> Self {
        let (sub_rows, _sub_cols) = shading_mode.sub_grid();

        // Parse lines into cell characters and active ANSI color strings
        let mut raw_cells: Vec<Vec<char>> = Vec::new();
        let mut raw_colors: Vec<Vec<Option<String>>> = Vec::new();
        let mut max_cols = 0;
        let mut has_ansi = false;

        for line in lines {
            let mut row_chars = Vec::new();
            let mut row_colors = Vec::new();
            let mut cur_color: Option<String> = None;
            let mut chars = line.chars().peekable();

            while let Some(c) = chars.next() {
                if c == '\x1b' && chars.peek() == Some(&'[') {
                    // Collect ANSI sequence
                    let mut seq = String::from("\x1b[");
                    chars.next(); // consume '['
                    while let Some(&next) = chars.peek() {
                        seq.push(next);
                        chars.next();
                        if next.is_ascii_alphabetic() {
                            break;
                        }
                    }
                    if seq.ends_with('m') {
                        if seq == "\x1b[0m" || seq == "\x1b[m" {
                            cur_color = None;
                        } else {
                            // Map ANSI colors to 24-bit TrueColor so Kitty themes cannot override them
                            let tc_seq = match seq.as_str() {
                                "\x1b[34m" | "\x1b[1;34m" => "\x1b[38;2;41;168;224m".to_string(),
                                "\x1b[37m" | "\x1b[1;37m" => "\x1b[38;2;255;255;255m".to_string(),
                                "\x1b[35m" | "\x1b[1;35m" => "\x1b[38;2;254;122;230m".to_string(),
                                "\x1b[36m" | "\x1b[1;36m" => "\x1b[38;2;23;147;209m".to_string(),
                                "\x1b[31m" | "\x1b[1;31m" => "\x1b[38;2;239;68;68m".to_string(),
                                "\x1b[32m" | "\x1b[1;32m" => "\x1b[38;2;34;197;94m".to_string(),
                                "\x1b[33m" | "\x1b[1;33m" => "\x1b[38;2;234;179;8m".to_string(),
                                "\x1b[96m" | "\x1b[1;96m" => "\x1b[38;2;51;162;230m".to_string(),
                                "\x1b[94m" | "\x1b[1;94m" => "\x1b[38;2;60;130;246m".to_string(),
                                "\x1b[95m" | "\x1b[1;95m" => "\x1b[38;2;254;122;230m".to_string(),
                                "\x1b[97m" | "\x1b[1;97m" => "\x1b[38;2;255;255;255m".to_string(),
                                _ => seq,
                            };
                            cur_color = Some(tc_seq);
                            has_ansi = true;
                        }
                    }
                    continue;
                }
                row_chars.push(c);
                row_colors.push(cur_color.clone());
            }

            max_cols = max_cols.max(row_chars.len());
            raw_cells.push(row_chars);
            raw_colors.push(row_colors);
        }

        let num_rows = raw_cells.len();
        let num_cols = max_cols;

        if num_rows == 0 || num_cols == 0 {
            return Self {
                points: Vec::new(),
                orig_rows: 0,
                orig_cols: 0,
                has_ansi_colors: false,
            };
        }

        // Build heightmap
        let mut hmap = vec![vec![0.0f32; num_cols]; num_rows];
        let mut sum = 0.0f32;
        let mut sum2 = 0.0f32;
        let mut count = 0;

        for r in 0..num_rows {
            for c in 0..num_cols {
                if c < raw_cells[r].len() {
                    let w = char_weight_utf8(raw_cells[r][c]);
                    hmap[r][c] = w;
                    if w > 0.0 {
                        sum += w;
                        sum2 += w * w;
                        count += 1;
                    }
                }
            }
        }

        // Auto-scale depth if variance is low
        let mut effective_depth = depth_scale;
        if count > 0 {
            let mean = sum / count as f32;
            let variance = (sum2 / count as f32) - (mean * mean);
            let stddev = variance.max(0.0).sqrt();
            if stddev < 0.25 {
                let boost = 1.0 + 2.0 * ((0.25 - stddev) / 0.25);
                effective_depth *= boost;
            }
        }

        let norm = if num_rows > 0 && num_rows < 30 {
            (28.0f32 / (num_rows as f32)).clamp(1.0, 1.55)
        } else {
            1.0f32
        };
        let sx = 0.07f32 * norm;
        let sy = 0.14f32 * norm;
        let cx = (num_cols as f32 - 1.0) * 0.5;
        let cy = (num_rows as f32 - 1.0) * 0.5;
        let zmax = 0.18f32 * effective_depth * norm;

        // Compute finite difference gradients for normals
        let mut gnx = vec![vec![0.0f32; num_cols]; num_rows];
        let mut gny = vec![vec![0.0f32; num_cols]; num_rows];
        let mut gnz = vec![vec![1.0f32; num_cols]; num_rows];

        for r in 0..num_rows {
            for c in 0..num_cols {
                if hmap[r][c] <= 0.0 {
                    continue;
                }
                let dhdx = if c > 0 && c < num_cols - 1 {
                    (hmap[r][c + 1] - hmap[r][c - 1]) * 0.5
                } else if c == 0 && num_cols > 1 {
                    hmap[r][c + 1] - hmap[r][c]
                } else if c > 0 {
                    hmap[r][c] - hmap[r][c - 1]
                } else {
                    0.0
                } / sx;

                let dhdy = if r > 0 && r < num_rows - 1 {
                    (hmap[r + 1][c] - hmap[r - 1][c]) * 0.5
                } else if r == 0 && num_rows > 1 {
                    hmap[r + 1][c] - hmap[r][c]
                } else if r > 0 {
                    hmap[r][c] - hmap[r - 1][c]
                } else {
                    0.0
                } / sy;

                let nnx = -dhdx;
                let nny = dhdy;
                let nnz = 1.0f32;
                let len = (nnx * nnx + nny * nny + nnz * nnz).sqrt();
                gnx[r][c] = nnx / len;
                gny[r][c] = nny / len;
                gnz[r][c] = nnz / len;
            }
        }

        let z_layers = ((6.0 * size_scale) as usize).max(6);
        let subdiv = ((size_scale * sub_rows as f32) as usize).max(sub_rows);

        let mut points = Vec::new();

        for row in 0..num_rows {
            for col in 0..num_cols {
                let h = hmap[row][col];
                if h <= 0.0 {
                    continue;
                }

                let cell_color = raw_colors[row].get(col).cloned().flatten();

                for sr in 0..subdiv {
                    for sc in 0..subdiv {
                        let frow = row as f32 + (sr as f32 / subdiv as f32);
                        let fcol = col as f32 + (sc as f32 / subdiv as f32);

                        // Bilinear interpolation of height
                        let ih = if sr > 0 || sc > 0 {
                            let fr = sr as f32 / subdiv as f32;
                            let fc = sc as f32 / subdiv as f32;
                            let nr = (row + if sr > 0 { 1 } else { 0 }).min(num_rows - 1);
                            let nc = (col + if sc > 0 { 1 } else { 0 }).min(num_cols - 1);
                            let h00 = hmap[row][col];
                            let h10 = hmap[nr][col];
                            let h01 = hmap[row][nc];
                            let h11 = hmap[nr][nc];
                            h00 * (1.0 - fr) * (1.0 - fc)
                                + h10 * fr * (1.0 - fc)
                                + h01 * (1.0 - fr) * fc
                                + h11 * fr * fc
                        } else {
                            h
                        };

                        if ih <= 0.0 {
                            continue;
                        }

                        let ox = (fcol - cx) * sx;
                        let oy = (cy - frow) * sy;
                        let zr = ih * zmax;

                        // Check if edge cell
                        let mut is_edge = false;
                        for dr in -1i32..=1 {
                            for dc in -1i32..=1 {
                                if dr == 0 && dc == 0 {
                                    continue;
                                }
                                let nr = row as i32 + dr;
                                let nc = col as i32 + dc;
                                if nr < 0
                                    || nr >= num_rows as i32
                                    || nc < 0
                                    || nc >= num_cols as i32
                                    || hmap[nr as usize][nc as usize] <= 0.0
                                {
                                    is_edge = true;
                                    break;
                                }
                            }
                            if is_edge {
                                break;
                            }
                        }

                        let layers = if is_edge || ih < 0.15 { 2 } else { z_layers };

                        for k in 0..layers {
                            let t = ((k as f32) / ((layers - 1) as f32)) - 0.5;
                            let pz = t * 2.0 * zr;

                            let (nx, ny, nz) = if k == 0 {
                                (gnx[row][col], gny[row][col], -gnz[row][col])
                            } else if k == layers - 1 {
                                (gnx[row][col], gny[row][col], gnz[row][col])
                            } else {
                                let mut ex = 0.0f32;
                                let mut ey = 0.0f32;
                                for dr in -1i32..=1 {
                                    for dc in -1i32..=1 {
                                        if dr == 0 && dc == 0 {
                                            continue;
                                        }
                                        let nr = row as i32 + dr;
                                        let nc = col as i32 + dc;
                                        let nh = if nr >= 0
                                            && nr < num_rows as i32
                                            && nc >= 0
                                            && nc < num_cols as i32
                                        {
                                            hmap[nr as usize][nc as usize]
                                        } else {
                                            0.0
                                        };
                                        if nh < h {
                                            ex += dc as f32;
                                            ey += -dr as f32;
                                        }
                                    }
                                }
                                let el = (ex * ex + ey * ey).sqrt();
                                if el > 1e-6 {
                                    ex /= el;
                                    ey /= el;
                                }
                                let tn = (k as f32 / (layers - 1) as f32) * 2.0 - 1.0;
                                let side = (1.0 - tn * tn).max(0.0).sqrt();
                                (ex * side, ey * side, tn)
                            };

                            let is_outer_side = k > 0 && k < layers - 1;

                            points.push(Point3D {
                                x: ox,
                                y: oy,
                                z: pz,
                                nx,
                                ny,
                                nz,
                                color: cell_color.clone(),
                                is_outer_side,
                            });
                        }
                    }
                }
            }
        }

        Self {
            points,
            orig_rows: num_rows,
            orig_cols: num_cols,
            has_ansi_colors: has_ansi,
        }
    }
}

/// Renderer responsible for rotating, lighting, and rasterizing 3D models.
pub struct Renderer3D {
    pub shading_mode: ShadingMode,
    pub shading_chars: Vec<String>,
    pub sextant_table: Vec<String>,
    pub light_dir: (f32, f32, f32),
    pub half_dir: (f32, f32, f32),
    pub size_scale: f32,
    pub speed: f32,
    pub outer_color: String,
    pub inner_color: String,
}

impl Default for Renderer3D {
    fn default() -> Self {
        let (lx, ly, lz) = (0.4082f32, 0.8165f32, -0.4082f32);
        // Blinn-Phong half vector with viewer at (0, 0, -1)
        let vx = 0.0f32;
        let vy = 0.0f32;
        let vz = -1.0f32;
        let hx = lx + vx;
        let hy = ly + vy;
        let hz = lz + vz;
        let hlen = (hx * hx + hy * hy + hz * hz).sqrt();

        Self {
            shading_mode: ShadingMode::Ascii,
            shading_chars: RAMP_ASCII.chars().map(|c| c.to_string()).collect(),
            sextant_table: build_sextant_table(),
            light_dir: (lx, ly, lz),
            half_dir: (hx / hlen, hy / hlen, hz / hlen),
            size_scale: 1.0,
            speed: 1.0,
            outer_color: "\x1b[1;36m".to_string(), // bold cyan
            inner_color: "\x1b[1;37m".to_string(), // bold white
        }
    }
}

/// Returns the (outer_color, inner_color) ANSI escape codes for a distribution.
/// Returns the (outer_color, inner_color) ANSI escape codes for a distribution.
/// Uses 24-bit TrueColor (RGB) escape sequences so that terminal emulators with active color themes
/// (such as Kitty themes, Catppuccin, TokyoNight, etc.) do NOT override the official branding.
pub fn distro_3d_colors(distro: &str) -> (&'static str, &'static str) {
    let d = distro.to_lowercase();
    if d.contains("gentoo") {
        ("\x1b[38;2;254;122;230m", "\x1b[38;2;255;255;255m") // vivid Gentoo pink outer (#fe7ae6), crisp pure white inner
    } else if d.contains("arch") {
        ("\x1b[38;2;23;147;209m", "\x1b[38;2;51;162;230m") // Arch cyan outer (#1793d1), light cyan inner (#33a2e6)
    } else if d.contains("ubuntu") || d.contains("asahi") {
        ("\x1b[38;2;233;84;32m", "\x1b[38;2;255;255;255m") // Ubuntu orange (#e95420), crisp white
    } else if d.contains("debian") {
        ("\x1b[38;2;215;10;83m", "\x1b[38;2;255;255;255m") // Debian red (#d70a53), crisp white
    } else if d.contains("fedora") {
        ("\x1b[38;2;41;168;224m", "\x1b[38;2;255;255;255m") // Fedora blue (#29a8e0), crisp pure white inner (#ffffff)
    } else if d.contains("nixos") {
        ("\x1b[38;2;82;119;195m", "\x1b[38;2;126;186;228m") // NixOS blue (#5277c3), cyan inner (#7ebae4)
    } else if d.contains("void") {
        ("\x1b[38;2;71;128;99m", "\x1b[38;2;104;166;132m") // Void green (#478063)
    } else if d.contains("alpine") {
        ("\x1b[38;2;41;168;224m", "\x1b[38;2;255;255;255m") // Alpine blue, crisp white
    } else if d.contains("opensuse") || d.contains("suse") {
        ("\x1b[38;2;115;186;37m", "\x1b[38;2;255;255;255m") // openSUSE green (#73ba25), white
    } else if d.contains("mint") {
        ("\x1b[38;2;135;189;101m", "\x1b[38;2;255;255;255m") // Linux Mint green (#87bd65), white
    } else if d.contains("manjaro") {
        ("\x1b[38;2;53;191;92m", "\x1b[38;2;255;255;255m") // Manjaro green (#35bf5c), white
    } else if d.contains("macos") || d.contains("darwin") || d.contains("pop") {
        ("\x1b[38;2;51;162;230m", "\x1b[38;2;255;255;255m") // Apple/Pop cyan (#33a2e6), white
    } else {
        ("\x1b[38;2;254;122;230m", "\x1b[38;2;255;255;255m") // Default Tux: vivid pink outer, pure white inner
    }
}

/// Convert an ANSI escape sequence or color name/hex into a TrueColor or formatted escape.
pub fn parse_color_to_ansi(color_str: &str) -> String {
    let s = color_str.trim();
    if s.starts_with("\x1b[") {
        return s.to_string();
    }
    if s.starts_with('#') && s.len() == 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&s[1..3], 16),
            u8::from_str_radix(&s[3..5], 16),
            u8::from_str_radix(&s[5..7], 16),
        ) {
            return format!("\x1b[38;2;{r};{g};{b}m");
        }
    }
    match s.to_lowercase().as_str() {
        "blue" => "\x1b[38;2;41;168;224m".to_string(),
        "white" => "\x1b[38;2;255;255;255m".to_string(),
        "pink" | "magenta" => "\x1b[38;2;254;122;230m".to_string(),
        "cyan" => "\x1b[38;2;23;147;209m".to_string(),
        "red" => "\x1b[38;2;239;68;68m".to_string(),
        "green" => "\x1b[38;2;34;197;94m".to_string(),
        "yellow" => "\x1b[38;2;234;179;8m".to_string(),
        "orange" => "\x1b[38;2;233;84;32m".to_string(),
        _ => s.to_string(),
    }
}

impl Renderer3D {
    pub fn new(shading_mode: ShadingMode, custom_shading: Option<&str>) -> Self {
        let shading_chars = if let Some(chars) = custom_shading {
            chars.chars().map(|c| c.to_string()).collect()
        } else {
            match shading_mode {
                ShadingMode::Ascii => RAMP_ASCII.chars().map(|c| c.to_string()).collect(),
                ShadingMode::Blocks | ShadingMode::Sextants => {
                    RAMP_BLOCKS.chars().map(|c| c.to_string()).collect()
                }
            }
        };
        Self {
            shading_mode,
            shading_chars,
            ..Self::default()
        }
    }

    /// Renders a single frame of the 3D model into an array of terminal strings.
    pub fn render_frame(
        &self,
        model: &Model3D,
        angle_x: f32,
        angle_y: f32,
        canvas_cols: usize,
        canvas_rows: usize,
    ) -> Vec<String> {
        if model.points.is_empty() || canvas_cols == 0 || canvas_rows == 0 {
            return vec![String::new(); canvas_rows];
        }

        let (sub_rows, sub_cols) = self.shading_mode.sub_grid();
        let total_sub_cols = canvas_cols * sub_cols;
        let total_sub_rows = canvas_rows * sub_rows;

        // Buffers: z-buffer (ooz), luminance, color index / ANSI string
        let mut zbuf = vec![vec![0.0f32; total_sub_cols]; total_sub_rows];
        let mut lumbuf = vec![vec![0.0f32; total_sub_cols]; total_sub_rows];
        let mut colorbuf: Vec<Vec<Option<String>>> =
            vec![vec![None; total_sub_cols]; total_sub_rows];
        let mut is_outer_buf = vec![vec![false; total_sub_cols]; total_sub_rows];

        let cos_a = angle_x.cos();
        let sin_a = angle_x.sin();
        let cos_b = angle_y.cos();
        let sin_b = angle_y.sin();

        let (lx, ly, lz) = self.light_dir;
        let (hx, hy, hz) = self.half_dir;

        let k2 = 5.5f32;
        let logo_h = (canvas_cols * 3 / 5).max(canvas_rows.min(canvas_cols * 3 / 5));
        let k1 = 37.0f32 * (logo_h as f32 / 36.0f32) * self.size_scale;
        let k1x2 = k1 * 2.0;

        let half_w = canvas_cols as f32 * 0.5;

        // Compute face-on y extent to align the top of the 3D logo at row 1
        let mut face_up = 0.0f32;
        for pt in &model.points {
            let zc = pt.z + k2;
            if zc > 0.1 {
                let ys = k1 * pt.y / zc;
                if ys > face_up {
                    face_up = ys;
                }
            }
        }
        let fixed_y_center = face_up + 1.0;
        let y_center = if fixed_y_center > 0.0 && fixed_y_center < canvas_rows as f32 {
            fixed_y_center
        } else {
            canvas_rows as f32 * 0.5
        };

        for pt in &model.points {
            // Rotation around X axis
            let y1 = pt.y * cos_a - pt.z * sin_a;
            let z1 = pt.y * sin_a + pt.z * cos_a;

            // Rotation around Y axis
            let x2 = pt.x * cos_b + z1 * sin_b;
            let z2 = -pt.x * sin_b + z1 * cos_b;
            let y2 = y1;

            // Rotate normals
            let ny1 = pt.ny * cos_a - pt.nz * sin_a;
            let nz1 = pt.ny * sin_a + pt.nz * cos_a;
            let nx2 = pt.nx * cos_b + nz1 * sin_b;
            let nz2 = -pt.nx * sin_b + nz1 * cos_b;
            let ny2 = ny1;

            let zc = z2 + k2;
            if zc < 0.1 {
                continue;
            }

            let ooz = 1.0 / zc;
            let xs = ((half_w + k1x2 * x2 * ooz) * sub_cols as f32) as i32;
            let ys = ((y_center - k1 * y2 * ooz) * sub_rows as f32) as i32;

            if xs < 0 || xs >= total_sub_cols as i32 || ys < 0 || ys >= total_sub_rows as i32 {
                continue;
            }

            let ux = xs as usize;
            let uy = ys as usize;

            if ooz > zbuf[uy][ux] {
                // Blinn-Phong Diffuse
                let diff = (nx2 * lx + ny2 * ly + nz2 * lz).max(0.0);

                // Specular
                let spec_dot = (nx2 * hx + ny2 * hy + nz2 * hz).max(0.0);
                let mut spec = spec_dot * spec_dot; // ^2
                spec *= spec; // ^4
                spec *= spec; // ^8

                let l = (0.08 + 0.62 * diff + 0.30 * spec).min(1.0);

                zbuf[uy][ux] = ooz;
                lumbuf[uy][ux] = l;
                colorbuf[uy][ux] = pt.color.clone();
                is_outer_buf[uy][ux] = pt.is_outer_side;
            }
        }

        let smax = self.shading_chars.len().saturating_sub(1);
        let total_sub = sub_rows * sub_cols;
        let mut output_lines = Vec::with_capacity(canvas_rows);

        for row in 0..canvas_rows {
            let mut line = String::with_capacity(canvas_cols * 16);
            let mut prev_color: Option<String> = None;

            for col in 0..canvas_cols {
                let x0 = col * sub_cols;
                let y0 = row * sub_rows;

                let mut mask = 0usize;
                let mut bit = 0;
                let mut n = 0;
                let mut lsum = 0.0f32;
                let mut best_z = 0.0f32;
                let mut cell_color: Option<String> = None;
                let mut cell_is_outer = false;

                for sr in 0..sub_rows {
                    for sc in 0..sub_cols {
                        let z = zbuf[y0 + sr][x0 + sc];
                        if z > 0.0 {
                            mask |= 1 << bit;
                            lsum += lumbuf[y0 + sr][x0 + sc];
                            n += 1;
                            if z > best_z {
                                best_z = z;
                                cell_color = colorbuf[y0 + sr][x0 + sc].clone();
                                cell_is_outer = is_outer_buf[y0 + sr][x0 + sc];
                            }
                        }
                        bit += 1;
                    }
                }

                if n == 0 {
                    if prev_color.is_some() {
                        line.push_str("\x1b[0m");
                        prev_color = None;
                    }
                    line.push(' ');
                    continue;
                }

                let coverage = n as f32 / total_sub as f32;
                let ink = (lsum / n as f32) * coverage;
                let ci = ((ink * smax as f32) + 0.5) as usize;
                let ci = ci.min(smax);

                // Select glyph
                let glyph = if mask != ((1 << total_sub) - 1)
                    && (coverage - ink).abs()
                        <= (((ci + 1) as f32 / self.shading_chars.len() as f32) - ink).abs()
                {
                    match self.shading_mode {
                        ShadingMode::Ascii => &self.shading_chars[ci],
                        ShadingMode::Blocks => QUADRANT_GLYPHS[mask],
                        ShadingMode::Sextants => {
                            if mask < self.sextant_table.len() {
                                &self.sextant_table[mask]
                            } else {
                                &self.shading_chars[ci]
                            }
                        }
                    }
                } else {
                    &self.shading_chars[ci]
                };

                // Color handling: extruded side walls always take outer_color (matching areofyl/fetch);
                // front/back surfaces take cell_color (e.g. white 'f' in Fedora) or inner_color (white/cyan).
                let active_color = if cell_is_outer {
                    self.outer_color.clone()
                } else if let Some(col) = cell_color {
                    col
                } else {
                    self.inner_color.clone()
                };

                if prev_color.as_deref() != Some(&active_color) {
                    line.push_str(&active_color);
                    prev_color = Some(active_color);
                }

                line.push_str(glyph);
            }

            if prev_color.is_some() {
                line.push_str("\x1b[0m");
            }
            output_lines.push(line);
        }

        output_lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_weight_utf8() {
        assert_eq!(char_weight_utf8(' '), 0.0);
        assert!(char_weight_utf8('M') > char_weight_utf8('.'));
        assert_eq!(char_weight_utf8('█'), 1.0);
        assert_eq!(char_weight_utf8('░'), 0.25);
    }

    #[test]
    fn test_model3d_creation() {
        let ascii = vec![
            "  ###  ".to_string(),
            " ####### ".to_string(),
            "  #####  ".to_string(),
        ];
        let model = Model3D::from_ascii_lines(&ascii, 1.0, 1.0, ShadingMode::Ascii);
        assert!(!model.points.is_empty());
        assert_eq!(model.orig_rows, 3);
    }

    #[test]
    fn test_renderer_frame() {
        let ascii = vec![
            "   /\\   ".to_string(),
            "  /  \\  ".to_string(),
            " /____\\ ".to_string(),
        ];
        let model = Model3D::from_ascii_lines(&ascii, 1.0, 1.0, ShadingMode::Ascii);
        let renderer = Renderer3D::default();
        let frame = renderer.render_frame(&model, 0.2, 0.4, 20, 10);
        assert_eq!(frame.len(), 10);
        assert!(frame.iter().any(|line| !line.trim().is_empty()));
    }
}
