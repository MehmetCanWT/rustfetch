# Layout Presets Showcase

RustFetch includes a library of built-in layout presets inspired by popular Unix ricing styles. Presets allow you to transform the entire layout, border structure, and spacing with a single flag, while remaining fully customizable through `config.toml`.

---

## 🚀 Using Presets

To apply a preset temporarily on the command line:

```bash
# Minimalist dot-separated layout
rustfetch --preset dots

# Rounded card box with section dividers
rustfetch --preset card

# Classic Neofetch style with color blocks
rustfetch --preset neofetch

# Balanced corner bracket card
rustfetch --preset brackets

# High-speed clean layout without containers
rustfetch --preset clean

# Retro terminal style with progress meters
rustfetch --preset retro
```

To make a preset your default, add it to `~/.config/rustfetch/config.toml`:
```toml
# config.toml
preset = "card"
```

---

## 🎨 Built-in Presets Catalog

### 1. `card` (Modern Rounded Container)
Encapsulates system telemetry inside an elegant rounded Unicode box (`╭───╮`, `│   │`, `╰───╯`), using section divider lines (`├───┤`) to organize hardware and desktop information.
- **Attributes:** `border = true`, `border_style = "rounded"`, `break` dividers, bottom color dots.
- **Preview:**
  ```text
  ╭───────────────────────────────────────────────╮
  │ mehmetcan@fedora                              │
  │ ────────────────                              │
  │ 󰣛  OS      : Fedora Linux 41 (Workstation)    │
  │ 󰌢  Host    : ThinkPad T14                     │
  │ 󰒋  Kernel  : Linux 6.13.0                     │
  ├───────────────────────────────────────────────┤
  │ 󰞷  Shell   : bash                             │
  │ 󰆍  Terminal: kitty                            │
  ├───────────────────────────────────────────────┤
  │ 󰍛  CPU     : Intel i5-11400H                  │
  │ 󰘚  Memory  : 8.4 GiB / 31.2 GiB               │
  │                                               │
  │ ● ● ● ● ● ● ● ●                               │
  ╰───────────────────────────────────────────────╯
  ```

---

### 2. `dots` (Minimalist Dot Separators)
For users who favor clean typography over border containers. Uses a subtle `•` dot separator and displays vibrant bottom palette dots.
- **Attributes:** `icon_only = true`, `separator = " • "`, `symbol = "●"`.
- **Preview:**
  ```text
  󰣇 • Arch Linux x86_64
  󰒋 • Linux 6.13.0
  󱑂 • 14 hours, 35 mins
  󰞷 • zsh 5.9
  󰍛 • AMD Ryzen 9 7950X
  󰘚 • 14.2 GiB / 64.0 GiB
  
  ● ● ● ● ● ● ● ●
  ```

---

### 3. `clean` (Ultra-Clean Icon-Only)
Zero visual noise: removes labels, containers, and palettes, presenting only the essential glyph and system value.
- **Attributes:** `icon_only = true`, `padding = 1`, `colors.enabled = false`.
- **Preview:**
  ```text
  󰣇  Arch Linux x86_64
  󰒋  Linux 6.13.0
  󰍛  AMD Ryzen 7 7840U
  󰘚  8.4 GiB / 31.2 GiB
  󱑂  5 hours, 20 mins
  ```

---

### 4. `neofetch` (Classic Terminal Aesthetic)
Emulates the iconic Neofetch format with `user@hostname`, dashed underline, colon separators, and a 16-color two-row block palette.
- **Attributes:** `separator = ":"`, `colors.block = true`, `colors.rows = 2`.
- **Preview:**
  ```text
  mehmetcan@archlinux
  -------------------
  OS: Arch Linux x86_64
  Host: B650 AORUS ELITE AX
  Kernel: 6.13.0-arch1-1
  Uptime: 2 days, 4 hours
  Packages: 1245 (pacman)
  Shell: zsh 5.9
  Terminal: kitty
  CPU: AMD Ryzen 7 7800X3D (16) @ 5.050GHz
  GPU: NVIDIA GeForce RTX 4080 SUPER
  Memory: 11420MiB / 31892MiB
  
  ████ ████ ████ ████ ████ ████ ████ ████
  ████ ████ ████ ████ ████ ████ ████ ████
  ```

---

### 5. `brackets` (Corner Bracket Accents)
Frames telemetry between top and bottom corner brackets (`⎡ ... ⎤` / `⎣ ... ⎦`), giving a modern HUD appearance.
- **Attributes:** `border_style = "brackets"`, `padding = 2`, `center = true`.
- **Preview:**
  ```text
  ⎡                                               ⎤
    󰣇  OS      : Fedora Linux 41                   
    󰒋  Kernel  : Linux 6.13.0                     
    󰍛  CPU     : AMD Ryzen 5 7600X                 
    󰘚  Memory  : 6.1 GiB / 32.0 GiB               
  ⎣                                               ⎦
  ```

---

### 6. `retro` (Retrowave Container & Progress Bars)
Features rounded borders, embedded progress bars for memory, swap, and disk, and a Pacman palette glyph (`󰮯`).
- **Attributes:** `border_style = "rounded"`, `bar = true`, `colors.symbol = "󰮯"`.

---

### 7. `minimal` (Server MOTD & Scripting)
A stripped-down, uncolored hardware summary that executes in <1ms, ideal for headless servers or automated login banners.
- **Attributes:** `icons = false`, `colors.enabled = false`, `logo.enabled = false`.

---

### 8. `modern` (Progress Banners)
Displays horizontal arrow separators (`OS -> Fedora`) and embedded progress meters for hardware telemetry.

---

### 9. `compact` (Terse Two-Letter Keys)
Two-letter abbreviation keys (`os`, `kr`, `up`, `pk`, `cp`, `gp`, `mm`) for compact or small terminal panes.

---

## 🛠️ Overriding Preset Properties

You can use any preset as a foundation and selectively override specific properties in your `config.toml`:

```toml
# Base layout
preset = "card"

[general]
# Override card's border with a custom double line and title:
border_style = "double"
border_color = "cyan"
border_title = "Workstation"
theme = "dracula"
```
RustFetch loads the base preset first, then applies your custom definitions on top.
