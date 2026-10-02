# ASCII Animations & GIF Frame Player

RustFetch supports playing continuous, frame-by-frame ASCII animations and converting GIF or video sequences into high-speed, zero-flicker terminal animations with full ANSI TrueColor support.

---

## ⚡ Quick Start

Play an animation using `--ascii-anim` (or `--anim`):

```bash
# Play a multi-frame animation file at 15 FPS
rustfetch --ascii-anim animation.txt --fps 15

# Play frames from a directory at 24 FPS
rustfetch --ascii-anim ~/.config/rustfetch/anims/nyan_cat/ --fps 24

# Play exactly 60 frames and stop, cleanly freezing the final frame
rustfetch --ascii-anim animation.txt --frames 60

# Continuous loop with instant exit on any keypress (Default)
rustfetch --ascii-anim animation.txt
```

---

## 📂 Supported Animation Formats

### 1. Single-File Multi-Frame Format (Delimited)
All frames are stored sequentially in a single `.txt` or `.ascii` file. Any of the following delimiter lines can be used to separate frames:
- `===FRAME===` or `===`
- `---FRAME---` or `---`
- `[frame]`
- VT100 Form Feed character (`\x0c`)

**Example `heart.txt`:**
```text
  /\_/\
 ( o.o )  RustFetch
  > ^ <
===FRAME===
  /\_/\
 ( -.- )  RustFetch
  > ^ <
===FRAME===
  /\_/\
 ( ^.^ )  RustFetch
  > ^ <
```

### 2. Directory Format (Individual Frame Files)
Frames can be stored as individual text files inside a folder:
```text
flame_anim/
├── frame_001.txt
├── frame_002.txt
├── frame_003.txt
└── ...
```

> [!TIP]
> **Natural Numerical Sorting:**
> RustFetch uses natural sorting for directory filenames (`frame_2.txt` precedes `frame_10.txt`), preventing the standard alphabetical ordering bug where `10` sorts before `2`.

---

## 🎞️ Converting Any GIF or Video into an ASCII Animation

You can convert any GIF or short video into an ASCII animation playable by RustFetch using `ffmpeg` and `chafa`:

### Step 1: Install `ffmpeg` and `chafa`
```bash
# Arch Linux / CachyOS
sudo pacman -S ffmpeg chafa

# Ubuntu / Debian
sudo apt install ffmpeg chafa

# Fedora
sudo dnf install ffmpeg chafa
```

### Step 2: Extract Frames from GIF as PNG
```bash
mkdir /tmp/gif_frames
ffmpeg -i input.gif -vf "fps=15,scale=40:-1" /tmp/gif_frames/frame_%04d.png
```

### Step 3: Convert PNG Frames to ANSI ASCII Files
```bash
mkdir -p ~/.config/rustfetch/anims/my_anim

for img in /tmp/gif_frames/*.png; do
    fname=$(basename "$img" .png)
    # Convert using 24-bit TrueColor symbols
    chafa --format=symbols --size=36x18 "$img" > ~/.config/rustfetch/anims/my_anim/"$fname.txt"
done

rm -rf /tmp/gif_frames
```

### Step 4: Play with RustFetch
```bash
rustfetch --ascii-anim ~/.config/rustfetch/anims/my_anim/ --fps 15
```

---

## ⌨️ Terminal Ergonomics & Zero-Flicker Architecture

1. **`exit_on_key` (Typing Handover):**
   - The animation runs continuously when your shell opens.
   - The moment you type any key (Space, Enter, Esc, q, Ctrl+C), the loop breaks immediately without consuming your keystroke.
   - Normal terminal mode (`termios`) and cursor visibility are restored via an RAII drop guard.
2. **`--hold` Flag:**
   - Setting `--hold` prevents premature exit on keypress, requiring an explicit `Ctrl+C` or `q` to quit.
3. **Double-Buffered Zero-Flicker Rendering:**
   - Instead of clearing the entire terminal window between frames (which causes flickering), RustFetch moves the cursor to the home position (`\x1b[H`) and flushes the entire frame in a single atomic buffer write.

---

## 📝 Configuration in `config.toml`

```toml
[general.logo.animation]
enabled = true
path = "~/.config/rustfetch/anims/my_anim"
fps = 20.0
frames = 120           # Number of frames before stopping (omit for infinite)
exit_on_key = true     # Exit immediately on keypress
```
