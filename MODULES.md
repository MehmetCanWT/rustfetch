# 📦 RustFetch — Telemetry Modules & Customization Guide

<div align="center">

![Rust](https://img.shields.io/badge/Language-Rust%202021-DEA584?style=for-the-badge&logo=rust)
![Modules](https://img.shields.io/badge/Modules-27%20Telemetry%20Collectors-4D9375?style=for-the-badge)
![Architecture](https://img.shields.io/badge/Architecture-Zero--Fork%20Kernel-61afef?style=for-the-badge)
![Performance](https://img.shields.io/badge/Latency-%3C5ms%20Full%20Fetch-E5C07B?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-98c379?style=for-the-badge)

**A high-performance Linux system information fetcher powered by pure kernel virtual filesystem parsing.**

[Quick Reference](#-master-telemetry-matrix) • [Configuration Syntax](#-configuration-syntax--attributes) • [Custom Icons Guide](#-custom-icons--dynamic-theming-guide) • [Module Catalog](#-detailed-module-catalog) • [Showcase Presets](#-showcase-presets)

</div>

---

## ⚡ Zero-Fork Architecture

Unlike traditional fetch scripts that spawn dozens of subshells (`bash -c`, `grep`, `awk`, `sed`, `cut`, `lspci`, `xrandr`), **RustFetch** gathers telemetry with zero process overhead:

- 🔬 **Kernel Direct:** Reads directly from `/proc` and `/sys` virtual filesystems into memory buffers.
- ⚡ **Native C Calls:** Invokes standard `libc` functions (`uname`, `statvfs`) directly via FFI.
- 🚀 **Hardware EDID Parsing:** Inspects raw 128-byte EDID binary headers from sysfs DRM connectors without X11 or Wayland display server round-trips.
- 🕒 **TTL Caching:** Aggregated package manager scans are cached under `$XDG_CACHE_HOME/rustfetch/` with a 2-hour TTL to eliminate disk I/O bottlenecks.
- ⏱️ **Microsecond Latency:** Individual collectors execute in **10 µs to 80 µs**, completing a full system fetch in **~4–6 milliseconds**.

---

## 📊 Master Telemetry Matrix

| Icon | Module | Category | Primary Data Source | Progress Bar | Status |
| :---: | :--- | :--- | :--- | :---: | :---: |
| `` | [`os`](#os) | System | `/etc/os-release` | No | Enabled |
| `` | [`host`](#host) | System | `/sys/devices/virtual/dmi/id/` | No | Enabled |
| `` | [`board`](#board) | System | `/sys/devices/virtual/dmi/id/board_name` | No | Enabled |
| `` | [`kernel`](#kernel) | System | `libc::uname` | No | Enabled |
| `` | [`uptime`](#uptime) | System | `/proc/uptime` | No | Enabled |
| `` | [`packages`](#packages) | System | Pure FS traversal (2h TTL cache) | No | Enabled |
| `` | [`cpu`](#cpu) | Hardware | `/proc/cpuinfo` & `/sys/devices/system/cpu` | No | Enabled |
| `󰾲` | [`gpu`](#gpu) | Hardware | `/sys/bus/pci/devices` & `pci.ids` | No | Enabled |
| `` | [`memory`](#memory) | Hardware | `/proc/meminfo` | Yes | Enabled |
| `󰓡` | [`swap`](#swap) | Hardware | `/proc/meminfo` | Yes | Enabled |
| `` | [`disk`](#disk) | Hardware | `libc::statvfs` | Yes | Enabled |
| `` | [`battery`](#battery) | Hardware | `/sys/class/power_supply/BAT*` | Yes | Enabled |
| `󰃠` | [`brightness`](#brightness) | Hardware | `/sys/class/backlight/` sysfs | Yes | Optional |
| `󰓅` | [`cpu_usage`](#cpu_usage) | Hardware | `/proc/stat` jiffies delta | Yes | Optional |
| `` | [`temp`](#temp) | Hardware | `/sys/class/hwmon/` | No | Optional |
| `󰍹` | [`display`](#display) | Hardware | Sysfs DRM & EDID Timing Descriptors | No | Optional |
| `` | [`desktop`](#desktop) | Environment | `$XDG_CURRENT_DESKTOP`, Wayland/X11 sockets | No | Enabled |
| `` | [`terminal`](#terminal) | Environment | `/proc/$PPID/stat` ancestry tree | No | Enabled |
| `` | [`shell`](#shell) | Environment | `$SHELL` & binary inspection | No | Enabled |
| `` | [`font`](#font) | Environment | `~/.config/gtk-3.0/settings.ini` | No | Enabled |
| `` | [`locale`](#locale) | Environment | `$LC_ALL`, `$LC_MESSAGES`, `$LANG` | No | Enabled |
| `󰖩` | [`wifi`](#wifi) | Network | Active sysfs link (`iw dev link` / `nmcli`) | No | Optional |
| `󰂯` | [`bluetooth`](#bluetooth) | Hardware | `bluetoothctl` & sysfs power_supply | No | Optional |
| `` | [`local_ip`](#local_ip) | Network | Local network interface query | No | Enabled |
| `` | [`sound`](#sound) | Multimedia | WirePlumber (`wpctl`) / PulseAudio | Yes | Optional |
| `󰝚` | [`media`](#media) | Multimedia | MPRIS D-Bus (`playerctl metadata`) | No | Optional |
| `` | [`processes`](#processes) | System | `/proc/loadavg` & PID count | No | Optional |
| `●` | [`colors`](#colors) | Layout | ANSI 16 / Dynamic Image TrueColor | — | Enabled |
| `` | [`custom`](#custom) | Layout | Static or user-defined text | No | Optional |
| ` ` | [`break`](#break) | Layout | Blank vertical spacing row | — | Optional |

---

## 🛠️ Configuration Syntax & Attributes

Every telemetry module is declared as an element of the `[[modules]]` table in `~/.config/rustfetch/config.toml`:

```toml
[[modules]]
name = "os"                 # (Required) Unique module identifier
text = "Operating System"   # (Optional) Custom label text (Aliases: label, title, key)
logo = ">_"                 # (Optional) Custom prefix icon (Aliases: icon, symbol, prefix)
color = "cyan"              # (Optional) Label/icon accent color (Name or Hex #7aa2f7)
format = "{value}"          # (Optional) Custom format template
bar = true                  # (Optional) Render visual bar (for percentage modules)
bar_width = 10              # (Optional) Progress bar character width (default: 10)
```

### Supported Module Attributes

| Field | Type | Default | Description |
| :--- | :---: | :---: | :--- |
| `name` | `string` | *(Required)* | The module identifier (e.g. `"memory"`, `"cpu"`). |
| `text` / `label` | `string` | Module default | Left-side label text displayed before the separator. |
| `logo` / `icon` | `string` | Built-in glyph | Prefix icon displayed before the label text. |
| `color` | `string` | `"auto"` | Terminal ANSI name (`"red"`, `"green"`, `"cyan"`, `"magenta"`, etc.) or Hex code (`"#7aa2f7"`). |
| `format` | `string` | `"{value}"` | Value formatting template. Supports `{value}` or `{}`. |
| `bar` | `boolean` | `false` | When `true`, appends a visual progress meter `[■■■■■-----]` on percentage-based collectors. |
| `bar_width` | `integer` | `10` | The character width of the progress bar. |
| `value` | `string` | `None` | Static text value (used specifically by the `custom` module). |

### Logo & 3D Animation Settings (`[general.logo]` and `[general.logo.three_d]`)

```toml
[general]
3d = true                    # Toggle animated 3D ASCII relief mode

[general.logo]
enabled = true
distro = "auto"              # Auto-detect or specify "arch", "fedora", "gentoo", etc.
image_width_cols = 60        # Width of 3D canvas or rendered image (default: 60)
protocol = "auto"            # "auto", "kitty", or "halfblock"

[general.logo.three_d]
enabled = true               # Enable 3D relief engine
speed = 1.0                  # Rotation speed multiplier
rotate_x = true              # Enable rotation on X axis
rotate_y = true              # Enable rotation on Y axis
size = 1.25                  # Scale factor for 3D model
depth = 1.0                  # Depth extrusion scale
width = 60                   # Canvas column width (alias: image_width_cols)
shading_mode = "ascii"       # "ascii" (default), "blocks", or "sextants"
# outer_color = "#29a8e0"    # Optional 24-bit TrueColor hex/name override
# inner_color = "#ffffff"
```

> [!TIP]
> **Serde Aliases for Total Freedom:**
> - You can use `text`, `label`, `title`, or `key` interchangeably.
> - You can use `logo`, `icon`, `symbol`, or `prefix` interchangeably.
> - RustFetch parses all of them seamlessly without configuration errors.

---

## 🎨 Custom Icons & Dynamic Theming Guide

RustFetch allows you to customize module prefixes with any character: ASCII symbols, Nerd Font glyphs, or emojis. Understanding how the terminal renders each type ensures a cohesive visual theme:

### 1. Vector Glyphs & Nerd Fonts (Dynamic Palette)
```toml
logo = ""   # Linux icon
logo = ""   # CPU chip
logo = ""   # Memory RAM
logo = ""   # Clock uptime
```
- **How they render:** These characters are vector outlines supplied by fonts like *JetBrains Mono Nerd Font*, *FiraCode Nerd Font*, or *MesloLGS*.
- **Color behavior:** They accept ANSI TrueColor escape sequences (`\x1b[38;2;R;G;Bm`).
- **Dynamic Theming:** When using `auto_color = true` or loading image palettes, vector glyphs automatically tint to match your wallpaper's dominant accent color.

### 2. ASCII & Minimalist Glyphs (Universal & Bulletproof)
```toml
logo = ">_"   # Terminal prompt
logo = "::"   # Namespace delimiter
logo = "->"   # Arrow indicator
logo = "#"    # Hash anchor
```
- **How they render:** Standard 7-bit ASCII characters available on 100% of computers and TTY consoles without special fonts.
- **Color behavior:** Dynamically tinted by the active theme palette.
- **Recommendation:** Perfect for minimalist configs or environments where Nerd Fonts are not installed.

### 3. Bitmap Color Emojis (Fixed OS Colors)
```toml
logo = "💻"   # Laptop
logo = "🐧"   # Penguin
logo = "📦"   # Package
logo = "⚡"   # Lightning bolt
```
- **How they render:** These are pre-rendered bitmap images embedded in operating system emoji fonts (e.g. *Noto Color Emoji*, *Apple Color Emoji*).
- **Color behavior:** Terminal escape sequences **cannot** re-color or tint bitmap emojis. A blue penguin `🐧` or yellow lightning `⚡` remains its fixed color regardless of your theme or image palette.

> [!NOTE]
> **Aesthetic Advice:** If you want your fetch output to dynamically match your desktop wallpapers (`auto_color = true`), choose **Vector Nerd Font glyphs** or **ASCII characters**.

---

## 📂 Detailed Module Catalog

### ⚡ Hardware & Sensors

#### `cpu`
Directly parses `/proc/cpuinfo` to detect processor model, logical/physical core counts, and maximum clock speeds while stripping redundant marketing noise (`(R)`, `(TM)`, `with Radeon Graphics`).
```toml
[[modules]]
name = "cpu"
text = "Processor"
logo = ""
```
```text
 Processor : 11th Gen Intel Core i5-11400H @ 2.70GHz (12)
```

#### `gpu`
Zero-subprocess graphics detection. Traverses `/sys/bus/pci/devices/*/class` to locate VGA display controllers (`0x030000`, `0x038000`, `0x030200`) and resolves vendor and model names directly from `/usr/share/hwdata/pci.ids` in microsecond time.
```toml
[[modules]]
name = "gpu"
logo = "󰾲"
```
```text
󰾲 GPU : NVIDIA GeForce RTX 3050 Mobile, Intel UHD Graphics
```

#### `memory`
Calculates accurate RAM consumption (`MemTotal`, `MemAvailable`, `Buffers`, `Cached`) directly from `/proc/meminfo`. Supports inline progress bar meters.
```toml
[[modules]]
name = "memory"
text = "Memory"
logo = ""
bar = true
bar_width = 10
```
```text
 Memory : 9.8 GiB / 15.4 GiB (63%) [■■■■■■----]
```

#### `swap`
Monitors swap file or zram partition usage from `/proc/meminfo`. If no swap is enabled on the host, the module cleanly and silently hides itself without cluttering output.
```toml
[[modules]]
name = "swap"
logo = "󰓡"
bar = true
```
```text
󰓡 Swap : 866.3 MiB / 8.0 GiB (10%) [■---------]
```

#### `disk`
Inspects partition space and usage using pure `libc::statvfs` calls on the root filesystem `/` or specified mount point.
```toml
[[modules]]
name = "disk"
text = "Disk (/)"
logo = ""
bar = true
bar_width = 12
```
```text
 Disk (/) : 313.5 GiB / 928.9 GiB (34%) [■■■■--------]
```

#### `temp`
Discovers system thermal sensors by scanning `/sys/class/hwmon/hwmon*/temp*_input` against known CPU thermal drivers (`coretemp`, `k10temp`, `zenpower`, `cpu_thermal`), falling back to ACPI thermal zones.
```toml
[[modules]]
name = "temp"
logo = ""
```
```text
 Temperature : 48°C
```

#### `battery`
Inspects `/sys/class/power_supply/BAT*/capacity`, charging state, battery health (`charge_full` vs `charge_full_design`), and battery cycle count (`cycle_count`). Features intelligent reversed progress bar thresholds (green at high percentage, yellow at medium, red when low).
```toml
[[modules]]
name = "battery"
text = "Battery"
logo = ""
bar = true
bar_width = 10
```
```text
 Battery : SMP - 84% [AC Connected] (Health: 83%, 144 cycles) [■■■■■■■■--]
```

#### `brightness`
Zero-subprocess screen brightness detection via `/sys/class/backlight/*/brightness` and `max_brightness`. Automatically calculates display backlight percentage and supports inline progress bar meters.
```toml
[[modules]]
name = "brightness"
text = "Brightness"
logo = "󰃠"
bar = true
bar_width = 10
```
```text
󰃠 Brightness : 75% [■■■■■■■---]
```

#### `cpu_usage`
High-precision active CPU load measurement calculated from `/proc/stat` jiffies delta over a subtle 50ms sample. Renders real-time multi-core utilization percentage with inline progress bar meters.
```toml
[[modules]]
name = "cpu_usage"
text = "CPU Usage"
logo = "󰓅"
bar = true
bar_width = 10
```
```text
󰓅 CPU Usage : 18% [■■--------]
```

#### `display`
Pure Rust sysfs DRM connector parsing (`/sys/class/drm/card*-*/modes`) and 128-byte EDID binary header parsing to calculate monitor names, physical resolutions, and refresh rates (e.g. `144Hz`) without `xrandr`.
```toml
[[modules]]
name = "display"
logo = "󰍹"
```
```text
󰍹 Display : 1920x1080 @ 144Hz (eDP-1)
```

---

### 🐧 System & Kernel

#### `os`
Inspects `/etc/os-release` to report the official distribution name, version, and architecture.
```toml
[[modules]]
name = "os"
logo = ""
```
```text
 OS : Fedora Linux 44 (KDE Plasma Desktop Edition)
```

#### `kernel`
Calls `libc::uname` to read the active Linux kernel release version.
```toml
[[modules]]
name = "kernel"
logo = ""
```
```text
 Kernel : 7.2.7-200.fc44.x86_64
```

#### `uptime`
Parses system uptime seconds from `/proc/uptime` and formats into natural human-readable units (days, hours, minutes).
```toml
[[modules]]
name = "uptime"
logo = ""
```
```text
 Uptime : 8 hours, 24 mins
```

#### `packages`
High-speed package counting across all installed package managers. Results are persisted in `$XDG_CACHE_HOME/rustfetch/` with a 2-hour TTL cache:
- `pacman` (`/var/lib/pacman/local/`)
- `dpkg` (`/var/lib/dpkg/status`)
- `rpm` (SQLite `/var/lib/rpm/rpmdb.sqlite` or BDB `/var/lib/rpm/Packages`)
- `flatpak` (`/var/lib/flatpak/app/`)
- `apk` (Alpine `/lib/apk/db/installed`)
- `xbps` (Void `/var/db/xbps/pkgdb-0.38.cpub`)
- `nix` (`/nix/var/nix/profiles/default`)
- `emerge` (`/var/db/pkg`)
- `snap` (`/var/lib/snapd/snaps`)
```toml
[[modules]]
name = "packages"
logo = ""
```
```text
 Packages : 4 (flatpak), 2728 (rpm)
```

#### `host` & `board`
Reads DMI hardware table data from `/sys/devices/virtual/dmi/id/product_name` and `board_name`.
```toml
[[modules]]
name = "host"
logo = ""

[[modules]]
name = "board"
logo = ""
```
```text
 Host  : Acer Nitro AN515-57
 Board : TGL Scala_TLS
```

#### `processes`
Reads `/proc/loadavg` to display the total running process count alongside the 1-minute CPU load average.
```toml
[[modules]]
name = "processes"
logo = ""
```
```text
 Processes : 342 (load: 0.42)
```

---

### 🖥️ Desktop & Environment

#### `desktop`
Resolves active Desktop Environment (`$XDG_CURRENT_DESKTOP`, `$DESKTOP_SESSION`) and Window Manager (Wayland socket / X11 root window inspection).
```toml
[[modules]]
name = "desktop"
logo = ""
```
```text
 Desktop : KDE (KWin Wayland)
```

#### `terminal`
Ascends the parent process tree via `/proc/$PPID/stat` to reliably pinpoint the true host terminal emulator, handling nested subshells and multiplexers gracefully.
```toml
[[modules]]
name = "terminal"
logo = ""
```
```text
 Terminal : ghostty
```

#### `shell`
Identifies the user's login shell from `$SHELL` and queries its version.
```toml
[[modules]]
name = "shell"
logo = ""
```
```text
 Shell : bash 5.2.32
```

#### `font`
Discovers system GTK or interface font configuration from `~/.config/gtk-3.0/settings.ini` or desktop configuration keys.
```toml
[[modules]]
name = "font"
logo = ""
```
```text
 Font : Noto Sans, 10 [GTK]
```

#### `locale`
Identifies active system language and character encoding from environment variables (`$LC_ALL`, `$LC_MESSAGES`, `$LANG`).
```toml
[[modules]]
name = "locale"
logo = ""
```
```text
 Locale : tr_TR.UTF-8
```

---

### 🌐 Network & Connectivity

#### `wifi`
Queries active wireless network SSID, signal quality in dBm and percentage, and Wi-Fi frequency band (`2.4 GHz`, `5 GHz`, `6 GHz`) using direct `iw dev <iface> link` queries with graceful `nmcli` fallback without initiating slow network scans.
```toml
[[modules]]
name = "wifi"
logo = "󰖩"
```
```text
󰖩 Wi-Fi : HomeNetwork (-52 dBm, 76%) [5 GHz]
```

#### `bluetooth`
Discovers active Bluetooth status and connected peripheral devices (wireless headphones, mice, keyboards, game controllers). Displays device names and battery percentages parsed via `bluetoothctl` and sysfs HID power supply interfaces (`/sys/class/power_supply/hid-*`).
```toml
[[modules]]
name = "bluetooth"
logo = "󰂯"
```
```text
󰂯 Bluetooth : MX Master 3S (85%), WH-1000XM4 (90%)
```

#### `local_ip`
Detects the private IPv4 address assigned to the machine's primary active routing interface.
```toml
[[modules]]
name = "local_ip"
logo = ""
```
```text
 Local IP : 192.168.1.200
```

---

### 🎵 Audio & Multimedia

#### `sound`
Reads volume percentage and mute status from WirePlumber (`wpctl`) or PulseAudio.
```toml
[[modules]]
name = "sound"
logo = ""
bar = true
```
```text
 Sound : 100% [■■■■■■■■■■]
```

#### `media`
Connects to MPRIS D-Bus via `playerctl metadata` to report the currently playing track.
> [!NOTE]
> **Zero Overhead:** This module is completely disabled unless explicitly declared in your `config.toml`, ensuring default runs stay below 5ms.
```toml
[[modules]]
name = "media"
logo = "󰝚"
```
```text
󰝚 Media : Daft Punk - Around the World
```

---

### 🎨 Layout & Formatting

#### `colors`
Renders an ANSI color palette preview test strip. Supports both standard terminal colors and dynamic TrueColor image palette extraction.
```toml
[[modules]]
name = "colors"
```
- **Configuration under `[general.colors]`**:
  - `symbol = "●"`: Palette symbol (circles `●`, squares `■`, or hashes `#`).
  - `block = true`: Renders dense solid background blocks (`███`).
  - `image_palette = true`: When an image is rendered with `auto_color = true`, RustFetch clusters the image's top 16 color tones using Euclidean distance and renders the palette with the image's vibrant colors!

#### `custom`
Inserts arbitrary user text, annotations, or environment variables.
```toml
[[modules]]
name = "custom"
text = "Role"
value = "Production Server"
logo = ">_"
color = "magenta"
```
```text
>_ Role : Production Server
```

#### `break`
Inserts a clean, empty spacer row between sections to visually balance the layout.
```toml
[[modules]]
name = "break"
```

---

## 🎨 Showcase Presets

### 1. Minimalist Hacker (`>_` ASCII Style)
```toml
[general]
separator = " :"
center = false
icons = true

[[modules]]
name = "os"
text = "os"
logo = ">_"

[[modules]]
name = "kernel"
text = "kernel"
logo = ">_"

[[modules]]
name = "uptime"
text = "uptime"
logo = ">_"

[[modules]]
name = "cpu"
text = "cpu"
logo = ">_"

[[modules]]
name = "memory"
text = "ram"
logo = ">_"
bar = true
bar_width = 8

[[modules]]
name = "colors"
```

### 2. Comprehensive Hardware Dashboard
```toml
[general]
separator = " :"
padding = 2
border = true
center = true

[general.colors]
enabled = true
symbol = "■"
image_palette = true

[[modules]]
name = "os"
[[modules]]
name = "kernel"
[[modules]]
name = "uptime"
[[modules]]
name = "break"
[[modules]]
name = "cpu"
[[modules]]
name = "cpu_usage"
bar = true
bar_width = 10
[[modules]]
name = "gpu"
[[modules]]
name = "temp"
[[modules]]
name = "memory"
bar = true
bar_width = 12
[[modules]]
name = "swap"
bar = true
[[modules]]
name = "disk"
bar = true
bar_width = 12
[[modules]]
name = "battery"
[[modules]]
name = "brightness"
bar = true
bar_width = 10
[[modules]]
name = "break"
[[modules]]
name = "wifi"
[[modules]]
name = "bluetooth"
[[modules]]
name = "local_ip"
[[modules]]
name = "colors"
```
