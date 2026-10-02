# RustFetch Documentation Hub

RustFetch is a high-performance (~4ms), memory-safe, and deeply customizable Linux & Unix system information fetcher written in pure Rust.

This directory contains comprehensive, modular technical guides detailing every subsystem, ricing capability, and telemetry collector available in RustFetch.

---

## 📚 Documentation Index

| Guide | Description |
|---|---|
| [**01. Configuration & Quick Start**](01-configuration.md) | File hierarchy, Fastfetch automatic migration, configuration schema, and CLI options. |
| [**02. Colors, Themes & Gradients**](02-colors-themes-gradients.md) | 13 built-in color themes, 24-bit TrueColor RGB, linear text gradients, and palette configuration. |
| [**03. Boxes, Borders & Image Frames**](03-boxes-and-borders.md) | 8 container styles, custom 11-char `border_chars`, titles, dividers, and **Image Borders** (`border_image`). |
| [**04. Custom ASCII Art**](04-ascii-art.md) | Loading custom 2D/3D ASCII art files and Neofetch `$1..$6` / `${c1}..${c6}` color variable replacement. |
| [**05. Images & Terminal Graphics Protocols**](05-images-and-protocols.md) | Kitty graphics protocol, Sixel, iTerm2, ANSI TrueColor half-blocks, wallpaper galleries, and auto-color. |
| [**06. 3D Terminal Graphics Engine**](06-3d-terminal-engine.md) | Real-time 3D ASCII mesh extrusion, relief heightmaps, Lambertian diffuse & Blinn-Phong specular lighting. |
| [**07. ASCII Animations & GIF Frame Player**](07-ascii-animations-gifs.md) | Multi-frame ASCII playback, converting GIFs to ASCII, zero-flicker double-buffered rendering, and `exit_on_key`. |
| [**08. Telemetry Modules & Nerd Font v3 Icons**](08-modules-and-icons.md) | All 28+ hardware & OS modules, Nerd Font v3 Material Design symbols, distro-aware glyphs, and progress bars. |
| [**09. Layout Presets Showcase**](09-presets-showcase.md) | Built-in presets (`card`, `dots`, `clean`, `neofetch`, `brackets`, `retro`, `minimal`, `modern`, `compact`). |

---

## ⚡ Quick Command Reference

```bash
# Standard fetch with auto-detected distro logo and Nerd Font v3 glyphs
rustfetch

# Rounded box card layout with embedded dividers and palette dots
rustfetch --preset card

# Minimalist dot layout with icon-only telemetry
rustfetch --preset dots

# Render with an image border / decorative frame alongside telemetry
rustfetch --border-image ~/.config/rustfetch/frames/vintage.png --border-image-width 24

# Dual-image mode: portrait logo on the left + decorative image border on the right
rustfetch --image ~/Pictures/avatar.png --border-image ~/Pictures/frame.png

# Double-line magenta border box with Dracula color theme
rustfetch --border --border-style double --border-color "#bd93f9" --theme dracula

# Continuous autonomous 3D animated relief logo
rustfetch --3d --speed 1.5 --shading-mode blocks

# Play frame-by-frame ASCII animation from a directory
rustfetch --ascii-anim ~/.config/rustfetch/anims/flame/ --fps 20

# Generate standard default configuration file
rustfetch --generate-config
```
