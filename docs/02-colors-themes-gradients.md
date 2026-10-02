# Colors, Themes & Gradients

RustFetch features an authentic, 24-bit TrueColor rendering pipeline that operates independently of terminal palette remappings, alongside 13 built-in ricing themes, multi-stop linear text gradients, and flexible palette previews.

---

## 🎨 Built-in Color Themes

RustFetch includes 13 production themes curated from the most popular Linux desktop ricing aesthetics:

| Theme Identifier | Description | Accent Color | Border Color |
|---|---|---|---|
| `catppuccin-mocha` | Dark soothing pastel theme | Lavender `#cba6f7` | Mauve `#cba6f7` |
| `catppuccin-latte` | Light pastel variant | Lavender `#7287fd` | Mauve `#8839ef` |
| `catppuccin-macchiato` | Medium dark pastel variant | Lavender `#b7bdf8` | Mauve `#c6a0f6` |
| `catppuccin-frappe` | Muted dark pastel variant | Lavender `#babbf1` | Mauve `#ca9ee6` |
| `dracula` | Classic dark theme with vibrant accents | Purple `#bd93f9` | Pink `#ff79c6` |
| `tokyo-night` | Neon Tokyo night aesthetic | Blue `#7aa2f7` | Magenta `#bb9af7` |
| `gruvbox` | Retro warm groove theme | Yellow `#fabd2f` | Orange `#fe8019` |
| `nord` | Arctic, north-bluish palette | Frost Cyan `#88c0d0` | Frost Blue `#81a1c1` |
| `rose-pine` | Soho vibes with natural rose tones | Rose `#ebbcba` | Iris `#c4a7e7` |
| `cyberpunk` | High-contrast neon cyber aesthetic | Yellow `#fcee0a` | Neon Pink `#ff007f` |
| `monokai` | Iconic code editor theme | Green `#a6e22e` | Magenta `#f92672` |
| `onedark` | Atom/Neovim deep dark theme | Blue `#61afef` | Purple `#c678dd` |
| `solarized-dark` | Precision solarized palette | Cyan `#2aa198` | Blue `#268bd2` |

### Using a Theme
```bash
# Apply a theme via CLI
rustfetch --theme catppuccin-mocha

# Or combine with box borders
rustfetch --theme dracula --border --border-style rounded
```

In `config.toml`:
```toml
[general]
theme = "tokyo-night"
```

When a theme is active, it automatically sets default colors for module keys, header titles, and border outlines unless explicitly overridden.

---

## 🌈 24-bit TrueColor & Text Gradients

RustFetch supports full 24-bit TrueColor RGB escape codes (`\x1b[38;2;R;G;Bm`). Any module color, border color, or header color can be specified as a standard 6-digit or 3-digit HEX code (`#bd93f9`, `#ff79c6`, `#fff`).

### Multi-Stop Linear Gradients
You can apply a continuous linear gradient across the output header and module labels by setting the `gradient` array in `config.toml`:

```toml
[general]
gradient = ["#f38ba8", "#cba6f7", "#89b4fa"]
```

RustFetch performs linear RGB interpolation across each line of text:
```
  t = current_char_index / total_characters
  R(t) = R_start * (1 - t) + R_end * t
```

---

## 🎛️ Color Palette Customization

RustFetch allows configuring how the terminal color preview is displayed:

```toml
[general.colors]
enabled = true             # Toggle color preview
symbol = "●"               # Symbol glyph: "●", "■", "󰮯", "◆", ""
block = false              # If true, outputs colored background blocks ("  ")
position = "bottom"        # "bottom", "top", or "both"
rows = 1                   # 1 (8 standard colors) or 2 (16 colors with bright variants)
# custom = ["#1e1e2e", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7"]
```

### Visual Preview Examples:

- **1-Row Dots (`symbol = "●"`, `rows = 1`):**
  ```text
  ● ● ● ● ● ● ● ●
  ```
- **2-Row Blocks (`block = true`, `rows = 2`):**
  ```text
  ████ ████ ████ ████ ████ ████ ████ ████
  ████ ████ ████ ████ ████ ████ ████ ████
  ```
- **Custom Color List:** Providing a `custom` array overrides standard ANSI palette colors with exact TrueColor swatches of your choice.
