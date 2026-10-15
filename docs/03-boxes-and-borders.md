# Boxes, Borders & Image Frames

RustFetch provides a flexible container and framing engine that supports both traditional text-based box drawing containers (with 8 styles and arbitrary Unicode character sets) and modern **rasterized image borders** (`border_image`) using high-resolution TrueColor ANSI half-blocks.

---

## 🖼️ Image Borders & Decorative Frames (`border_image`)

In modern terminal ricing (such as setups showcased on `r/unixporn`), users often frame their telemetry using an ornate decorative illustration or vintage corner bracket graphic.

RustFetch natively supports using any image file (`.png`, `.jpg`, `.webp`) as a border or frame directly within the terminal layout.

### Quick Start: Image Border

```bash
# Render a decorative frame image alongside telemetry
rustfetch --border-image ~/.config/rustfetch/frames/vintage_corner.png --border-image-width 24

# Dual-image mode: portrait photo on the left + decorative image frame on the right
rustfetch --image ~/Pictures/avatar.png --border-image ~/Pictures/frame.png --border-image-width 20

# Combine with minimalist icon-only mode
rustfetch --border-image ~/Pictures/frame.png --icon-only
```

### Configuration in `config.toml`

#### Single Border Image (Grouped Setup)
```toml
[general.border_image]
path = "~/.config/rustfetch/frames/vintage_corner.png"
position = "behind"        # Default: "behind" ("behind", "background", "left", "right", "top", "bottom")
width = 24                 # Width in terminal columns (default: 24)
height = 15                # Height in rows (optional, default: auto)
offset_x = 0               # Horizontal shift in columns
offset_y = 0               # Vertical shift in rows
z_index = -1               # Kitty z-index (-1 renders directly behind text)
protocol = "auto"          # "auto", "kitty", or "halfblock"
```

#### Legacy Flat Configuration (Supported for backwards compatibility)
```toml
[general]
border_image = "~/.config/rustfetch/frames/vintage_corner.png"
border_image_position = "behind"
border_image_width = 24
```

#### Multi-Image Composition (`[[general.border_images]]`)
You can define arbitrarily many border images layered simultaneously:

```toml
[general]
border = true
border_style = "rounded"

# High-definition native backdrop behind the telemetry modules
[[general.border_images]]
path = "~/Pictures/wallpapers/card_backdrop.png"
position = "background"
z_index = -1
protocol = "auto"         # Uses Kitty GPU text-underlay if supported, else ANSI TrueColor fallback

# Decorative ornament on the right side
[[general.border_images]]
path = "~/.config/rustfetch/frames/ornament.png"
position = "right"
width = 20
```

### Border Image Placement

`border_image_position` (or `position` in `[[general.border_images]]`) controls where the image is composed relative to the module card:

| Position | Result |
|---|---|
| `background` / `behind` | **High-Definition Card Backdrop:** Renders directly behind the modules as a background card. In Kitty/Ghostty/WezTerm, rendered natively via GPU text-underlay (`z=-1, C=1`) with zero pixelation. In standard terminals, rendered via TrueColor ANSI background coloring. |
| `left` | Decorative column before the module card. |
| `right` | Decorative column after the module card. |
| `top` | Decorative rows above the complete module card. |
| `bottom` | Decorative rows below the complete module card. |
| `frame` / `both` | Reuses the image on both sides of the module card. |

```bash
# Render a high-definition background card directly behind telemetry
rustfetch --no-logo --border --border-image images/arch.png --border-image-position background

# Adjust dimensions and offsets
rustfetch --border --border-image ~/Pictures/art.png --border-image-position background \
  --border-image-width 60 --border-image-height 20 --border-image-offset-x 2

# Force half-block ANSI background fallback mode
rustfetch --border --border-image images/arch.png --border-image-position background \
  --border-image-protocol halfblock
```

### How the High-Definition Image Border Engine Works:
1. **Kitty Graphics Protocol Text-Underlay (`z=-1, C=1`):**
   In modern GPU-accelerated terminal emulators (Kitty, Ghostty, WezTerm), RustFetch transmits the image using the Kitty Graphics Protocol with `z=-1` (layer under text cells) and `C=1` (cursor anchor). The terminal GPU draws the original, crisp high-resolution image behind the character cells at monitor native resolution. The telemetry modules then print directly on top with transparent backgrounds, completely eliminating half-block pixelation.
2. **TrueColor ANSI Cell Background Fallback:**
   For terminal emulators that do not support Kitty graphics (e.g. Alacritty, Linux TTY), RustFetch samples the image pixels and injects 24-bit TrueColor background ANSI codes (`\x1b[48;2;R;G;Bm`) into each character cell, so telemetry text visibly sits on top of the image colors without corrupting the layout.
3. **TrueColor Half-Block Rasterization for Frames:**
   For `left`, `right`, `top`, `bottom`, and `frame` placements, images are rasterized into dual-pixel half-block elements (`▀` / `▄`) with alpha transparency preservation.
4. **Multi-Image Compositing:**
   Multiple border images defined in `[[general.border_images]]` are composed sequentially: side/top frames wrap around the card first, and background backdrops are rendered underneath the complete composition.

---

## 📦 Text Box Containers (`--border`)

To encapsulate telemetry output within a structured container, enable the box engine:

```bash
# Enable default rounded card container
rustfetch --border --border-style rounded

# Heavy box container in bright cyan
rustfetch --border --border-style heavy --border-color cyan

# Double-line box container with an embedded title
rustfetch --border --border-style double --border-color "#bd93f9" --border-title "System Specs"
```

---

## 🎨 8 Built-in Border Styles

| Style Name | Top Corners | Top/Bottom Line | Side Lines | Bottom Corners | Divider Line |
|---|:---:|:---:|:---:|:---:|:---:|
| `rounded` (default) | `╭` `╮` | `─` | `│` | `╰` `╯` | `├` `─` `┤` |
| `double` | `╔` `╗` | `═` | `║` | `╚` `╝` | `╠` `═` `╣` |
| `heavy` | `┏` `┓` | `━` | `┃` | `┗` `┛` | `┣` `━` `┫` |
| `light` | `┌` `┐` | `─` | `│` | `└` `┘` | `├` `─` `┤` |
| `ascii` | `+` `+` | `-` | `\|` | `+` `+` | `+` `-` `+` |
| `brackets` | `⎡` `⎤` | ` ` | ` ` | `⎣` `⎦` | ` ` `─` ` ` |
| `dots` | `·` `·` | `·` | `·` | `·` `·` | `·` `·` `·` |
| `block` | `█` `█` | `▀` | `█` | `█` `█` | `█` `▄` `█` |

---

## ✏️ Custom Character Sets (`border_chars`)

You can define completely custom box-drawing glyphs using an 11-character string or a detailed configuration object.

### String Syntax (11 Characters)
Order: `TL` (Top-Left), `T` (Top), `TR` (Top-Right), `L` (Left), `R` (Right), `BL` (Bottom-Left), `B` (Bottom), `BR` (Bottom-Right), `DL` (Divider-Left), `D` (Divider), `DR` (Divider-Right).

```toml
[general]
border = true
border_chars = "┌─┐││└─┘├─┤"
```

### Table Object Syntax
```toml
[general]
border = true

[general.border_chars]
top_left = "╭"
top = "─"
top_right = "╮"
left = "│"
right = "│"
bottom_left = "╰"
bottom = "─"
bottom_right = "╯"
divider_left = "├"
divider = "─"
divider_right = "┤"
```

---

## 🏷️ Embedded Titles & Section Dividers

### 1. Top Border Title (`border_title`)
Setting `border_title` embeds your title directly into the top border line:
```toml
[general]
border = true
border_style = "rounded"
border_title = "Hardware"
```
```text
╭── Hardware ────────────────────────────────╮
│ 󰍛  CPU     : AMD Ryzen 7 7840U             │
│ 󰘚  Memory  : 8.4 GiB / 31.2 GiB            │
╰────────────────────────────────────────────╯
```

### 2. Section Dividers (`name = "break"` / `name = "divider"`)
Adding a `break` module inside `[[modules]]` generates an authentic internal divider line that connects seamlessly with the outer border:

```toml
[[modules]]
name = "os"

[[modules]]
name = "kernel"

[[modules]]
name = "break"             # Creates: ├────────────────────────────────────────────┤
label = "Specs"            # Optional: ├── Specs ──────────────────────────────────┤

[[modules]]
name = "cpu"

[[modules]]
name = "memory"
```
```text
╭────────────────────────────────────────────╮
│ 󰣇  OS      : Arch Linux x86_64             │
│ 󰒋  Kernel  : Linux 6.13.0                  │
├── Specs ───────────────────────────────────┤
│ 󰍛  CPU     : AMD Ryzen 7 7840U             │
│ 󰘚  Memory  : 8.4 GiB / 31.2 GiB            │
╰────────────────────────────────────────────╯
```

---

## 📏 Box Padding (`box_padding`)

Control the inner horizontal spacing between the border lines and the telemetry content:

```toml
[general]
box_padding = 2            # Adds 2 spaces of internal padding on left and right
```
