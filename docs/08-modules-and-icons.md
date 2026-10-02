# Telemetry Modules & Nerd Font v3 Icons

RustFetch gathers system telemetry across **28+ kernel, hardware, desktop, and network collectors** with zero subshell overhead.

All default icons are modernized using **Nerd Font v3 (Material Design Symbols)** glyphs, replacing broken legacy characters. Furthermore, RustFetch detects your Linux distribution and automatically assigns an authentic, distribution-specific OS glyph.

---

## 🎨 Nerd Font v3 Icon Set

### Distro-Aware Operating System Glyphs

The `os` module queries `/etc/os-release` and automatically renders your distribution's authentic Nerd Font glyph:

| Distribution | Glyph | Nerd Font Identifier | Unicode |
|---|:---:|---|---|
| **Arch Linux / CachyOS** | `󰣇` | `nf-md-arch` | `U+F08C7` |
| **Fedora Linux** | `󰣛` | `nf-md-fedora` | `U+F08DB` |
| **Ubuntu** | `󰕈` | `nf-md-ubuntu` | `U+F0548` |
| **Debian** | `󰣚` | `nf-md-debian` | `U+F08DA` |
| **NixOS** | `󱄅` | `nf-md-nix` | `U+F1105` |
| **Gentoo** | `󰣨` | `nf-md-gentoo` | `U+F08E8` |
| **Manjaro** | `󱘊` | `nf-md-manjaro` | `U+F160A` |
| **macOS** | `󰀵` | `nf-md-apple` | `U+F0035` |
| **Windows** | `󰍲` | `nf-md-microsoft_windows` | `U+F0372` |
| **Generic Linux / BSD** | `󰌽` | `nf-md-linux` | `U+F033D` |

---

### Hardware & System Module Glyphs

| Module | Default Icon | Meaning | Description |
|---|:---:|---|---|
| `cpu` | `󰍛` | Processor | Integrated circuit chip |
| `gpu` | `󰘚` | Graphics Card | Expansion card / GPU |
| `memory` | `󰘚` | Memory (RAM) | Memory module / RAM stick |
| `swap` | `󰓡` | Swap Space | Paged memory buffer |
| `disk` | `󰋊` | Storage Drive | Hard drive / SSD disk |
| `battery` | `󰂁` | Battery Power | Battery charge indicator |
| `shell` | `󰞷` | User Shell | Terminal prompt (`>_`) |
| `terminal` | `󰆍` | Terminal Emulator | Terminal window |
| `terminal_font` | `󰛖` | Terminal Font | Typography font glyph |
| `uptime` | `󱑂` | System Uptime | Clock timer |
| `packages` | `󰏖` | Package Manager | Package parcel box |
| `host` | `󰌢` | Computer Hardware | Laptop / Desktop device |
| `wifi` | `󰖩` | Wireless Network | Wi-Fi radio wave |
| `local_ip` | `󰩩` | Local IP | Network interface |
| `public_ip` | `󰩩` | Public IP | Internet gateway |
| `de` / `wm` | `󰧨` | Desktop Environment | Desktop window tiling |
| `theme` | `󰔎` | Theme | Paint palette |
| `icons` | `󰀻` | Icon Theme | Grid of icons |
| `media` | `󰝚` | Media Player | Music note |
| `sound` | `󰕾` | Audio Volume | Speaker volume |
| `temp` | `󰔏` | Temperature | Thermometer sensor |
| `brightness` | `󰃠` | Display Backlight | Sun / Brightness |
| `display` | `󰍹` | Monitor Display | Video display screen |

---

## 📋 Telemetry Module Catalog

### 1. System & Kernel Modules
- **`os`**: Operating system name, edition, and architecture (`Arch Linux x86_64`).
- **`kernel`**: Running Linux kernel release (`Linux 6.13.0-arch1-1`).
- **`host`**: Motherboard or laptop model from DMI sysfs (`ThinkPad T14 Gen 3`).
- **`board`**: Motherboard model name.
- **`uptime`**: System uptime formatted into days, hours, and minutes.
- **`packages`**: Package manager inventory (pacman, flatpak, nix, apt, dnf, rpm, apk, xbps, emerge). Pure filesystem scan with a 2-hour TTL cache.

### 2. Hardware & Sensor Modules
- **`cpu`**: Processor brand, model, core count, and frequency (`AMD Ryzen 7 7840U @ 5.10GHz`).
- **`gpu`**: Dedicated and integrated graphics cards scanned from PCI bus.
- **`memory`**: Memory usage, total capacity, and percentage bar (`8.4 GiB / 31.2 GiB (27%)`).
- **`swap`**: Swap space allocation and utilization.
- **`disk`**: Filesystem mount point capacity and percentage usage (`statvfs`).
- **`battery`**: Battery charge level, charging status, health percentage, and cycle count.
- **`brightness`**: Display backlight brightness percentage from `/sys/class/backlight`.
- **`temp`**: Hardware thermal monitoring sensors (`hwmon`).

### 3. Desktop & Environment Modules
- **`de` / `wm`**: Active desktop environment (GNOME, KDE Plasma, XFCE) or window manager (Hyprland, Sway, i3, niri).
- **`shell`**: Login shell binary and version (`zsh 5.9`).
- **`terminal`**: Terminal emulator parsed from parent process tree (`kitty`, `alacritty`, `foot`).
- **`terminal_font`**: Active terminal font name and point size.
- **`theme`**, **`icons`**, **`font`**, **`cursor`**: Desktop styling settings.

---

## ⚙️ Module Customization Options

Modules are configured under `[[modules]]` in `config.toml`:

```toml
[[modules]]
name = "os"
icon = "󰣇"
label = "OS"
color = "#1793d1"
format = "{icon} {value}"

[[modules]]
name = "memory"
label = "RAM"
bar = true
bar_width = 12
bar_style = "blocks"    # "blocks", "braille", "ascii"

[[modules]]
# Horizontal Divider Line (creates '├── Hardware ──┤' inside boxes)
name = "break"
label = "Hardware"

[[modules]]
name = "cpu"
label = "CPU"

[[modules]]
# Custom Shell Command
name = "custom"
command = "curl -s ifconfig.me"
icon = "󰩩"
label = "Public IP"
color = "yellow"
```

---

## 🎯 Key Display Formatting Options

RustFetch provides flexible key display modes:

### 1. `icon_only` Mode
Hides text labels and displays only the icon and value:
```bash
rustfetch --icon-only
```
```text
󰣇  Arch Linux x86_64
󰒋  Linux 6.13.0
󰍛  AMD Ryzen 7 7840U
󰘚  8.4 GiB / 31.2 GiB
```

### 2. `key_type` Option
- `key_type = "both"`: Displays icon and label (`󰍛 CPU: ...`)
- `key_type = "icon"`: Displays icon only (`󰍛 ...`)
- `key_type = "title"`: Displays label only (`CPU: ...`)
- `key_type = "none"`: Displays value only.

### 3. Custom Separator (`separator`)
```toml
[general]
separator = " • "
```
```text
󰣇 • Arch Linux x86_64
󰒋 • Linux 6.13.0
󰍛 • AMD Ryzen 7 7840U
```
