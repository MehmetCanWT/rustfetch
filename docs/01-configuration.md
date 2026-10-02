# Configuration & Quick Start

RustFetch is configured via a standard TOML file located according to the XDG Base Directory specification. It also features automatic, non-destructive migration from existing Fastfetch configurations.

---

## 📍 Configuration File Locations

RustFetch resolves configuration files in the following strict order of precedence:

1. **CLI Flag Override:** Passed via `-c` or `--config <PATH>`:
   ```bash
   rustfetch --config ~/.config/rustfetch/work.toml
   ```
2. **XDG Standard Path:** `$XDG_CONFIG_HOME/rustfetch/config.toml` (defaults to `~/.config/rustfetch/config.toml`).
3. **Internal Default Fallback:** If no configuration file exists, RustFetch runs using optimized built-in defaults with zero mandatory setup.

### Generating a Default Configuration File

To scaffold a well-commented, complete `config.toml` in your configuration directory:
```bash
rustfetch --generate-config
```

To print the active configuration to stdout:
```bash
rustfetch --dump-config
```

---

## 🔄 Automatic Fastfetch Migration

If you are migrating from **Fastfetch**, RustFetch can automatically import your module layout, ordering, custom icons, and visual preferences:

```bash
rustfetch --import-fastfetch
```

### How Fastfetch Migration Works:
- Inspects `~/.config/fastfetch/config.jsonc` in a strictly read-only manner.
- Strips JSONC comments (`//` and `/* */`) and trailing commas.
- Maps Fastfetch module keys (`OS`, `Kernel`, `Memory`, `Disk`, `Battery`, etc.) to RustFetch collectors.
- Preserves custom icons, labels, and formatting rules.
- Exports the translated configuration to `~/.config/rustfetch/config.toml`. Your original Fastfetch setup remains completely untouched.

---

## 🧱 Configuration Structure (`config.toml`)

A typical configuration file consists of a `[general]` table and an ordered array of `[[modules]]`:

```toml
# Top-level preset override (optional)
# preset = "card"

[general]
separator = " : "          # Delimiter between label/icon and telemetry value
padding = 1                # Horizontal margin from terminal left edge
center = false             # Horizontally center output in terminal window
icons = true               # Enable or disable Nerd Font glyphs globally
icon_only = false          # Hide text labels and display icons only
key_type = "both"          # "both", "icon", "title", or "none"

# Box Container Settings
border = false             # Wrap telemetry lines in a border container
border_style = "rounded"   # "ascii", "light", "heavy", "double", "rounded", "brackets", "dots", "block"
border_color = "magenta"   # Color name ("blue", "cyan", "magenta") or hex ("#bd93f9")
border_title = "System"    # Optional title embedded in top border line
box_padding = 1            # Inner horizontal spacing between border and content

# Image Border / Decorative Frame
# border_image = "~/.config/rustfetch/frames/ornament.png"
# border_image_width = 24
# border_image_position = "frame" # left, right, top, bottom, or frame

# Theme and Gradients
theme = "catppuccin-mocha" # Built-in color theme
# gradient = ["#f38ba8", "#cba6f7", "#89b4fa"] # Linear text gradient

[general.colors]
enabled = true             # Display terminal color palette preview
symbol = "●"               # Palette symbol: "●", "■", "󰮯", etc.
block = false              # Use colored background blocks instead of glyphs
position = "bottom"        # "bottom", "top", or "both"
rows = 1                   # 1 (8 colors) or 2 (16 colors including bright variants)

[general.logo]
enabled = true             # Display logo/graphic
distro = "auto"            # "auto" detects host OS, or override ("arch", "fedora", "ubuntu", etc.)
protocol = "auto"          # Image protocol: "auto", "kitty", "halfblock", "sixel", "iterm"
image_width_cols = 42      # Width of graphic column in terminal cells
auto_color = true          # Adapt accent colors to image dominant color

[[modules]]
name = "os"

[[modules]]
name = "kernel"

[[modules]]
name = "uptime"

[[modules]]
name = "break"             # Renders as internal horizontal divider inside boxes

[[modules]]
name = "cpu"

[[modules]]
name = "gpu"

[[modules]]
name = "memory"
bar = true
bar_width = 12
```

---

## 🛠️ CLI Flag Overrides

CLI flags always take precedence over configuration file settings:

```text
  -c, --config <PATH>          Path to custom TOML config file
      --preset <NAME>          Apply built-in layout preset [card, dots, clean, neofetch, brackets, retro, minimal, modern, compact, default]
      --theme <NAME>           Apply color theme [catppuccin-mocha, dracula, tokyo-night, gruvbox, nord, rose-pine, etc.]
      --border                 Enable decorative box container
      --border-style <STYLE>   Container style [ascii, light, heavy, double, rounded, brackets, dots, block]
      --border-color <COLOR>   Container color (name or hex, e.g. magenta, #bd93f9)
      --border-title <TITLE>   Title embedded in top border line
      --border-image <PATH>    Path to image file used as a decorative border/frame
      --border-image-width <N> Column width for border image
      --border-image-position <POSITION>  left, right, top, bottom, or frame
      --icon-only              Hide labels and display only glyphs + values
      --separator <SEP>        Custom delimiter between label/icon and value
  -l, --logo <NAME>            Override distro logo
  -i, --image <PATH>           Render image logo via Kitty protocol / halfblocks
      --no-logo                Disable logo completely
      --3d                     Run in real-time 3D animated mode
      --ascii <PATH>           Use custom static ASCII art file
      --ascii-anim <PATH>      Play multi-frame ASCII animation sequence
      --live                   Run in real-time continuous monitor mode
      --json                   Output structured JSON telemetry
      --benchmark              Profile microsecond execution latency
```
