# Images & Terminal Graphics Protocols

RustFetch supports rendering high-resolution images, animated GIFs, and custom illustrations directly inside your terminal using modern graphics protocols (Kitty, Sixel, iTerm2) and universal 24-bit TrueColor ANSI half-blocks.

---

## 🚀 Quick Start: Image Mode

```bash
# Render an image using automatic protocol detection
rustfetch --image ~/Pictures/avatar.png

# Specify terminal column width for the image
rustfetch --image ~/Pictures/wallpaper.png --width 50

# Combine an image logo with an image border / decorative frame
rustfetch --image ~/Pictures/portrait.png --border-image ~/Pictures/frame.png --border-image-width 22
```

---

## 🖥️ Supported Graphics Protocols

RustFetch automatically selects the optimal protocol supported by your terminal emulator, or you can force a protocol in `config.toml` or via CLI:

| Protocol | Identification | Supported Terminals | Visual Quality |
|---|---|---|---|
| **Kitty** | `protocol = "kitty"` | Kitty, Ghostty, WezTerm | Native pixel graphics (zero distortion) |
| **Sixel** | `protocol = "sixel"` | Foot, WezTerm, mlterm, xterm (vt340) | 16/256-color raster bitmap |
| **iTerm2** | `protocol = "iterm"` | iTerm2, WezTerm, mintty | Base64-encoded inline graphics |
| **Half-block** | `protocol = "halfblock"` | **All terminals** (Alacritty, TTY, etc.) | High-density Unicode 24-bit TrueColor blocks |

### Protocol Configuration:
```toml
[general.logo]
image_path = "~/Pictures/profile.png"
image_width_cols = 38
protocol = "auto"         # "auto", "kitty", "halfblock", "sixel", "iterm"
```

---

## 🎲 Random Wallpaper Gallery (`image_dir`)

RustFetch can pick a random wallpaper or anime illustration from a designated folder on every terminal launch:

```toml
[general.logo]
enabled = true
image_dir = "~/Pictures/FetchWallpapers"
random_image = true
image_width_cols = 40
protocol = "kitty"
```

Each time you open a terminal or run `rustfetch`, a random `.png`, `.jpg`, or `.webp` file is selected and displayed.

---

## 🎨 Automatic Dominant Color Extraction (`auto_color`)

When `auto_color = true` (the default), RustFetch scans the pixels of your loaded image in pure Rust, identifies the most vibrant dominant accent color using HSV saturation weighting, and automatically paints:
- The username and hostname header (`user@host`).
- Telemetry module icons.
- Border outlines (when border color is unset).

```toml
[general.logo]
auto_color = true          # Accent colors dynamically adapt to the image
```

---

## 🖼️ Dual-Image Composition (Logo + Image Border)

You can combine a primary logo image with a secondary image border (`border_image`):

```bash
rustfetch --image ~/Pictures/photo.png --border-image ~/.config/rustfetch/frames/vintage.png --border-image-width 24
```

1. **Left:** Primary portrait image rendered at `image_width_cols`.
2. **Middle:** Decorative frame / corner ornament rasterized via TrueColor half-blocks.
3. **Right:** System telemetry lines aligned with the frame.
