# 🎬 RustFetch ASCII & Animation Guide

RustFetch supports not only built-in distribution ASCII logos and static raster graphics, but also **custom user-defined ASCII artwork** and **high-performance frame-by-frame animated ASCII graphics** rendered in real-time with zero terminal flicker.

---

## 📑 Table of Contents
1. [Custom ASCII Art Usage (2D & 3D)](#1-custom-ascii-art-usage-2d--3d)
2. [Creating Frame-by-Frame ASCII Animations](#2-creating-frame-by-frame-ascii-animations)
   - [Method A: Delimited Single File](#method-a-delimited-single-file)
   - [Method B: Directory of Frame Files](#method-b-directory-of-frame-files)
3. [TrueColor & ANSI Escape Code Support](#3-truecolor--ansi-escape-code-support)
4. [Intelligent Shell Handoff (`exit_on_key`)](#4-intelligent-shell-handoff-exit_on_key)
5. [Configuration Reference (`config.toml`)](#5-configuration-reference-configtoml)
6. [CLI Command Reference](#6-cli-command-reference)
7. [Bundled Showcase Examples](#7-bundled-showcase-examples)

---

## 1. Custom ASCII Art Usage (2D & 3D)

Any ASCII artwork saved as a `.txt` or `.ascii` file can be passed directly to RustFetch.

### A. Static 2D Rendering
```bash
rustfetch --ascii /path/to/my_logo.txt
```
RustFetch automatically measures the glyph bounds, aligns the ASCII art to the left of your system telemetry, and pads the layout cleanly.

### B. Real-Time 3D Mesh Rotation
Convert any static 2D ASCII art into a rotating 3D heightmap relief mesh:
```bash
rustfetch --ascii /path/to/my_logo.txt --3d
```
RustFetch analyzes the glyph ink density of each character (`char_weight_utf8`), constructs a 3D depth field, and applies real-time lighting shaders as it rotates across 3 axes.

---

## 2. Creating Frame-by-Frame ASCII Animations

RustFetch supports two intuitive formats for multi-frame animations:

### Method A: Delimited Single File
Combine all animation frames into a single `.txt` file separated by standard delimiter markers:
- `===FRAME===`
- `===`
- `---FRAME---`
- `---`
- `[frame]`
- Form Feed control character (`\x0c`)

**Example `animation.txt`**:
```text
  ( o.o )
   > ^ <
===FRAME===
  ( -.- )
   > ^ <
===FRAME===
  ( o.- )
   > ^ <
===FRAME===
  ( -.o )
   > ^ <
```

To run:
```bash
rustfetch --ascii-anim animation.txt --fps 10
```

---

### Method B: Directory of Frame Files
Organize individual frame files inside a single folder:
```text
my_animation/
├── 01.txt
├── 02.txt
├── 03.txt
└── 04.txt
```

> [!TIP]
> **Natural Numeric Sorting:** File names like `frame_1.txt`, `frame_2.txt`, ..., `frame_10.txt` are sorted naturally so `frame_10` correctly follows `frame_9` instead of jumping ahead of `frame_2`.

To run:
```bash
rustfetch --ascii-anim my_animation/ --fps 15
```

---

## 3. TrueColor & ANSI Escape Code Support

RustFetch preserves all 24-bit TrueColor (`\x1b[38;2;R;G;Bm`) and 16-color ANSI escape sequences embedded in your ASCII art:
- Terminal emulator color schemes do not distort your explicit TrueColor codes.
- Gradients and per-character shading effects render flawlessly across each frame.

---

## 4. Intelligent Shell Handoff (`exit_on_key`)

Running an interactive animation upon opening your terminal via `~/.bashrc` or `~/.zshrc` typically risks blocking the shell prompt or discarding typed keystrokes.

RustFetch solves this using **non-blocking POSIX stdin polling (`ioctl(FIONREAD)`)**:
- The animation begins playing smoothly as your terminal window opens.
- **The millisecond you press any key (start typing a shell command), the animation loop terminates immediately.**
- The pressed key is **preserved** and delivered directly to your shell without dropping characters.
- The final frame and system telemetry remain cleanly displayed on screen.

> To prevent automatic key handoff and keep the animation running until `Ctrl+C` or `q` is pressed, use `--hold`:
> ```bash
> rustfetch --ascii-anim animation.txt --hold
> ```

---

## 5. Configuration Reference (`config.toml`)

To make your animation or custom ASCII logo permanent, add it to `~/.config/rustfetch/config.toml`:

```toml
[general.logo]
enabled = true
# Static ASCII art path:
ascii_path = "~/.config/rustfetch/my_logo.txt"

# Or animated multi-frame ASCII:
[general.logo.animation]
enabled = true
path = "~/.config/rustfetch/animations/spinner.txt" # or a directory path
fps = 15.0
exit_on_key = true # Immediate handoff to shell on keypress
infinite = true    # Continuous loop
```

---

## 6. CLI Command Reference

| Flag | Description | Default |
| :--- | :--- | :--- |
| `--ascii <PATH>` | Static custom ASCII file (2D or 3D) | None |
| `--ascii-anim <PATH>` / `--anim` | Animated ASCII file or frame directory | None |
| `--fps <FLOAT>` | Frame rate in FPS (1.0 to 60.0) | `15.0` |
| `--frames <N>` | Maximum frames to play before exiting | Infinite |
| `--hold` | Keep playing until manual interrupt (`Ctrl+C` or `q`) | `false` |
| `--exit-on-key` | Immediately hand off control to shell on any keypress | `true` |

---

## 7. Bundled Showcase Examples

Test these bundled assets directly from the repository:

1. **Delimited Single File Spinner (`spinner.txt`)**:
   ```bash
   rustfetch --ascii-anim assets/animations/spinner.txt --fps 15
   ```

2. **Directory Pulse Animation (`pulse/`)**:
   ```bash
   rustfetch --ascii-anim assets/animations/pulse/ --fps 12
   ```

3. **Rotating 3D Relief Mesh from ASCII Art**:
   ```bash
   rustfetch --ascii assets/animations/pulse/01.txt --3d
   ```
