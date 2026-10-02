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

```toml
[general]
border_image = "~/.config/rustfetch/frames/vintage_corner.png"
border_image_width = 24    # Width in terminal columns (default: 24)
border_image_position = "frame" # "left", "right", "top", "bottom", or "frame"
icon_only = true           # Clean icon-only format next to the frame
```

### Border Image Placement

`border_image_position` controls where the rasterized image is composed relative
to the complete telemetry card. It works with both plain telemetry and a normal
Unicode `border = true` card, so image decoration and text-box borders can be
layered together.

| Position | Result |
|---|---|
| `left` | Backwards-compatible image column before the module card. |
| `right` | Image column after the module card. |
| `top` | Image rows above the complete module card. |
| `bottom` | Image rows below the complete module card. |
| `frame` | Reuses the same image on both sides of the module card. |

```bash
# Put a normal rounded module card between two image decorations.
rustfetch --no-logo --border --border-title "System" \
  --border-image ~/.config/rustfetch/frames/side.png \
  --border-image-position frame
```

### How the Image Border Engine Works:
1. **TrueColor Half-Block Rasterization:**
   RustFetch reads the image file and samples pixel colors with bilinear filtering. Every pair of vertical pixels is mapped into a UTF-8 half-block element (`▀` / `▄`) with 24-bit foreground and background escape codes (`\x1b[38;2;...m\x1b[48;2;...m`).
2. **Alpha Transparency Preservation:**
   Transparent pixels (`alpha < 30`) remain completely transparent (rendered as clean whitespace), allowing the border graphic to blend seamlessly into your terminal's background.
3. **Multi-Column Horizontal Composition:**
   - **Logo + Module Frame:** An image logo remains the primary logo, while the border image decorates the complete module card according to `border_image_position`.
   - **Standalone Frame:** If no logo is enabled (`--no-logo`), image placement still applies directly to the telemetry block.

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
