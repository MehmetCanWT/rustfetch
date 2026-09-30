# RustFetch — Telemetry Modules Guide

This document provides a comprehensive reference for all telemetry collectors and formatting modules available in **RustFetch**.

---

## Table of Contents

- [Overview & Architecture](#overview--architecture)
- [Module Configuration Reference](#module-configuration-reference)
- [Custom Icons and Color Styling Guide](#custom-icons-and-color-styling-guide)
- [Available Modules](#available-modules)
  - [Hardware Telemetry](#hardware-telemetry)
    - [cpu](#cpu)
    - [temp](#temp)
    - [gpu](#gpu)
    - [memory](#memory)
    - [swap](#swap)
    - [disk](#disk)
    - [battery](#battery)
    - [display](#display)
  - [System & Kernel](#system--kernel)
    - [os](#os)
    - [kernel](#kernel)
    - [uptime](#uptime)
    - [packages](#packages)
    - [host](#host)
    - [board](#board)
    - [processes](#processes)
  - [Desktop & Environment](#desktop--environment)
    - [desktop](#desktop)
    - [terminal](#terminal)
    - [shell](#shell)
    - [font](#font)
    - [locale](#locale)
  - [Network & Connectivity](#network--connectivity)
    - [wifi](#wifi)
    - [local_ip](#local_ip)
  - [Audio & Multimedia](#audio--multimedia)
    - [sound](#sound)
    - [media](#media)
  - [Layout & Formatting](#layout--formatting)
    - [colors](#colors)
    - [custom](#custom)
    - [break](#break)
- [Example Configuration](#example-configuration)

---

## Overview & Architecture

RustFetch is built around a **zero-fork kernel telemetry architecture**:
- **Zero Subprocesses**: Telemetry is parsed directly from Linux virtual filesystems (`/proc` and `/sys`) and libc system calls (`uname`, `statvfs`). Subprocesses like `lspci`, `xrandr`, `awk`, or `grep` are never spawned.
- **TTL Disk Caching**: Heavy package manager scans are cached in `$XDG_CACHE_HOME/rustfetch/` with a 2-hour TTL.
- **Microsecond Execution**: Most modules execute in **10 to 80 microseconds**, achieving full fetch runs in **~4–6 milliseconds**.

---

## Module Configuration Reference

Each module is defined in `~/.config/rustfetch/config.toml` under `[[modules]]`:

```toml
[[modules]]
name = "os"                 # (Required) Module identifier
text = "System"             # (Optional) Custom label text (alias: label, title, key)
logo = ">_"                 # (Optional) Custom icon / symbol (alias: icon, symbol, prefix)
color = "cyan"              # (Optional) Label/icon color: name or hex (#7aa2f7)
format = "{value}"          # (Optional) Output format string ({value} or {})
bar = true                  # (Optional) Enable visual progress bar (percentage modules)
bar_width = 10              # (Optional) Progress bar width in characters (default: 10)
```

> 💡 **Convenient Aliases:**
> - Both `text = "..."` and `label = "..."` work interchangeably.
> - `logo = "..."`, `icon = "..."`, and `symbol = "..."` all configure the module's prefix icon.

---

## Custom Icons and Color Styling Guide

In RustFetch, you can assign any custom character, ASCII string, or Nerd Font glyph as a module prefix using `logo` (or `icon`). However, whether an icon **dynamically adapts to your theme's color palette** depends on the character format:

### 1. Color-Adaptive Icons (Monochrome / Vector)
These characters belong to the terminal's monochrome vector glyph sets. They are rendered directly through ANSI TrueColor (`\x1b[38;2;r;g;bm`) escape sequences. When `auto_color = true` or when `--color` is specified, **they immediately inherit the active accent color**:

- **Text & ASCII Symbols:**
  - `logo = ">_"`
  - `logo = "::"`
  - `logo = "->"`
  - `logo = "[$]"`
  - `logo = "=>"`
  - `logo = "#"`
- **Nerd Font & FontAwesome Glyphs:**
  - `logo = ""` (Linux)
  - `logo = ""` (CPU)
  - `logo = ""` (RAM / Memory)
  - `logo = ""` (Laptop / Host)
  - `logo = ""` (Desktop)
  - `logo = ""` (Terminal / Shell)
  - `logo = ""` (Uptime / Clock)
  - `logo = ""` (Packages)
  - `logo = ""` (Kernel / Settings)
  - `logo = ""` (Network / Local IP)
- **Unicode Geometric Shapes:**
  - `logo = "●"`, `logo = "■"`, `logo = "▲"`, `logo = "◆"`, `logo = "⚡"`

### 2. Fixed-Color Glyphs (Multi-Color Emojis)
These characters originate from the operating system's color emoji libraries (Noto Color Emoji, Apple Color Emoji, Twemoji, etc.) as embedded bitmap or SVG pictures.

- **Examples:** `💻`, `🐧`, `📦`, `🕒`, `📁`, `🚀`, `💾`, `🔊`
- **Why they do not adapt:** Terminal ANSI color codes cannot tint multi-color bitmap emojis. Regardless of the theme or accent color, they will always render in their fixed predefined colors and will not match dynamic themes.

> 🎯 **Recommendation:** For a seamless, cohesive aesthetic that dynamically adapts to image colors (`auto_color = true`), choose ASCII symbols (`>_`, `::`) or vector Nerd Font glyphs.

---

## Available Modules

### Hardware Telemetry

#### `cpu`
- **Description:** Reports the CPU processor model name, total physical/logical cores, and maximum clock frequency.
- **Icon:** ``
- **Data Source:** Direct parsing of `/proc/cpuinfo`.
- **Formatting:** Cleans redundant vendor prefixes (e.g. `Intel(R) Core(TM)`, `11th Gen`, `with Radeon Graphics`).
- **Example Output:** `11th Gen Intel(R) Core(TM) i5-11400H @ 2.70GHz (12)`

#### `temp`
- **Description:** Detects current processor thermal temperature in degrees Celsius (°C).
- **Icon:** ``
- **Data Source:** Iterates `/sys/class/hwmon/hwmon*/temp*_input` matching thermal drivers (`coretemp`, `k10temp`, `zenpower`, `cpu_thermal`), with fallback to `/sys/class/thermal/thermal_zone*`.
- **Example Output:** `52°C`

#### `gpu`
- **Description:** Identifies dedicated and integrated graphics cards.
- **Icon:** `󰾲`
- **Data Source:** Pure filesystem scan of `/sys/bus/pci/devices/*/class` (matching VGA display class `0x030000`, `0x038000`, `0x030200`). Vendor and device IDs are resolved directly via `/usr/share/hwdata/pci.ids` without spawning `lspci`.
- **Example Output:** `NVIDIA GeForce RTX 3050 Mobile, Intel UHD Graphics`

#### `memory`
- **Description:** Reports used and total system RAM with percentage calculation. Supports visual progress bars.
- **Icon:** ``
- **Data Source:** Direct parsing of `MemTotal`, `MemAvailable`, `MemFree`, `Buffers`, and `Cached` from `/proc/meminfo`.
- **Bar Support:** Yes (`bar = true`, `bar_width = 10`).
- **Example Output:** `8.1 GiB / 15.4 GiB (52%) [■■■■■-----]`

#### `swap`
- **Description:** Reports used and total swap memory. Automatically hidden if no swap space is configured.
- **Icon:** `󰓡`
- **Data Source:** `SwapTotal` and `SwapFree` from `/proc/meminfo`.
- **Bar Support:** Yes (`bar = true`).
- **Example Output:** `1.5 GiB / 8.0 GiB (18%)`

#### `disk`
- **Description:** Filesystem disk usage and capacity for root (`/`) or mounted partitions.
- **Icon:** ``
- **Data Source:** `libc::statvfs` on root `/` (or configured mount point).
- **Bar Support:** Yes (`bar = true`).
- **Example Output:** `313.3 GiB / 928.9 GiB (34%) [■■■-------]`

#### `battery`
- **Description:** Reports battery charge percentage, charging state (`Charging`, `Discharging`, `Full`), and AC power connection status.
- **Icon:** ``
- **Data Source:** `/sys/class/power_supply/BAT*/capacity` and `status`.
- **Bar Support:** Yes (`bar = true`, with inverted color thresholds: green at high capacity).
- **Example Output:** `100% [Full] [AC Connected]`

#### `display`
- **Description:** Detects connected external and internal monitors, active screen resolutions, and refresh rates.
- **Icon:** `󰍹`
- **Data Source:** Pure Rust parsing of sysfs DRM connectors (`/sys/class/drm/card*-*/modes`) and 128-byte EDID Detailed Timing Descriptors (`/sys/class/drm/card*-*/edid`) to compute pixel clock and refresh rate (e.g. 144Hz) without `xrandr`.
- **Example Output:** `1920x1080 @ 144Hz`

---

### System & Kernel

#### `os`
- **Description:** Operating system distribution name, release version, and machine architecture.
- **Icon:** ``
- **Data Source:** `/etc/os-release` (`PRETTY_NAME` or `NAME` + `VERSION_ID`).
- **Example Output:** `Fedora Linux 41 (Workstation Edition) x86_64`

#### `kernel`
- **Description:** Running Linux kernel release version and build architecture.
- **Icon:** ``
- **Data Source:** System call `libc::uname`.
- **Example Output:** `6.13.1-200.fc41.x86_64`

#### `uptime`
- **Description:** Total system uptime formatted human-readably in days, hours, and minutes.
- **Icon:** ``
- **Data Source:** Direct reading of `/proc/uptime`.
- **Example Output:** `5 hours, 12 mins`

#### `packages`
- **Description:** Aggregated count of installed software packages across package managers.
- **Icon:** `󰏖`
- **Data Source:** Pure filesystem inspection with 2-hour TTL cache:
  - `pacman`: `/var/lib/pacman/local` directory entries
  - `dpkg`: `/var/lib/dpkg/status`
  - `rpm`: SQLite `/var/lib/rpm/rpmdb.sqlite` or BDB `/var/lib/rpm/Packages`
  - `flatpak`: User and system `/var/lib/flatpak/app/`
  - `apk`: Alpine `/lib/apk/db/installed`
  - `xbps`: Void `/var/db/xbps/pkgdb-0.38.cpub`
  - `nix`: NixOS `/nix/var/nix/profiles/default`
  - `emerge`: Gentoo `/var/db/pkg`
  - `snap`: `/var/lib/snapd/snaps`
- **Example Output:** `2418 (rpm), 12 (flatpak)`

#### `host`
- **Description:** Computer manufacturer, product model, and chassis name.
- **Icon:** `󰌢`
- **Data Source:** DMI sysfs files (`/sys/devices/virtual/dmi/id/product_name` and `sys_vendor`).
- **Example Output:** `Acer Nitro AN515-57`

#### `board`
- **Description:** Motherboard model name.
- **Icon:** `󰌢`
- **Data Source:** `/sys/devices/virtual/dmi/id/board_name`.
- **Example Output:** `TGL Scala_TLS`

#### `processes`
- **Description:** Total active process count and 1-minute load average.
- **Icon:** ``
- **Data Source:** Parsed directly from `/proc/loadavg` and `/proc` PID scan.
- **Example Output:** `342 (load: 0.42)`

---

### Desktop & Environment

#### `desktop`
- **Description:** Desktop Environment (DE) and active Window Manager (WM).
- **Icon:** ``
- **Data Source:** Environment variables (`$XDG_CURRENT_DESKTOP`, `$DESKTOP_SESSION`) and Wayland/X11 socket inspection.
- **Example Output:** `GNOME 47.2 (Wayland)` or `KDE (KWin Wayland)`

#### `terminal`
- **Description:** Currently running terminal emulator application name.
- **Icon:** ``
- **Data Source:** Inspects parent process hierarchy through `/proc/$PPID/stat` up the process tree, with fallback to `$TERM_PROGRAM` and `$TERM`.
- **Example Output:** `alacritty`, `kitty`, `ghostty`, or `wezterm`

#### `shell`
- **Description:** User login shell name and version.
- **Icon:** ``
- **Data Source:** Environment variable `$SHELL`.
- **Example Output:** `bash 5.2.32` or `zsh 5.9`

#### `font`
- **Description:** System default GTK or desktop interface font and size.
- **Icon:** ``
- **Data Source:** Parsed from `~/.config/gtk-3.0/settings.ini` or desktop font settings.
- **Example Output:** `Noto Sans, 10 [GTK]`

#### `locale`
- **Description:** System language and character encoding locale.
- **Icon:** ``
- **Data Source:** `$LC_ALL`, `$LC_MESSAGES`, or `$LANG`.
- **Example Output:** `en_US.UTF-8` or `tr_TR.UTF-8`

---

### Network & Connectivity

#### `wifi`
- **Description:** Wireless network interface SSID and signal quality percentage.
- **Icon:** `󰖩`
- **Data Source:** Active wireless interface link status via `iw dev <iface> link` or `iwgetid -r` (reads current connection state without initiating a slow air scan).
- **Example Output:** `MyHomeNetwork (70dBm - 60%)`

#### `local_ip`
- **Description:** Private IPv4 address assigned to the primary active network interface.
- **Icon:** `󰩟`
- **Data Source:** Network interface query.
- **Example Output:** `192.168.1.150`

---

### Audio & Multimedia

#### `sound`
- **Description:** Default audio sink volume percentage and mute status.
- **Icon:** ``
- **Data Source:** WirePlumber (`wpctl get-volume @DEFAULT_AUDIO_SINK@`) with fallback to PulseAudio or `/proc/asound/cards`.
- **Example Output:** `100%` or `Muted`

#### `media`
- **Description:** Currently active media player playback track (Artist - Title).
- **Icon:** `󰎆`
- **Data Source:** MPRIS D-Bus interface via `playerctl metadata`.
- **Performance Note:** **Disabled by default**. The media collector only runs when explicitly declared in `[[modules]]` in `config.toml`, ensuring zero overhead during standard terminal opens.
- **Example Output:** `Daft Punk - Around the World`

---

### Layout & Formatting

#### `colors`
- **Description:** Terminal 8- or 16-color palette test strip. Supports both standard terminal ANSI colors and dynamic TrueColor image palette extraction.
- **Icon:** None
- **Configuration Options:**
  - `symbol = "●"`: Palette symbol (e.g. circles `●`, squares `■`, or hashes `#`).
  - `block = true`: Renders dense solid background blocks (`███`).
  - `image_palette = true` (Default: `true`): When an image is displayed and `auto_color = true`, RustFetch automatically extracts the 16 primary color tones from the image and renders the palette circles using the image's vibrant colors! When no image is displayed, it falls back to the terminal's 16 ANSI system colors.
- **Example Output:** `● ● ● ● ● ● ● ●`

#### `custom`
- **Description:** Arbitrary static or dynamically formatted text line with custom label and color.
- **Configuration Options:**
  - `label`: Left-side label text.
  - `value`: Right-side value text.
  - `color`: Accent color.
- **Example:**
  ```toml
  [[modules]]
  name = "custom"
  label = "Host"
  value = "Production Workstation"
  color = "magenta"
  ```

#### `break`
- **Description:** Inserts an empty blank spacing line between module sections for visual balance.
- **Configuration Options:**
  ```toml
  [[modules]]
  name = "break"
  ```

---

## Example Configuration

Here is a full `~/.config/rustfetch/config.toml` utilizing modules with custom formatting and progress bars:

```toml
[general]
separator = ":"
padding = 1
center = true
icons = true
border = false

[general.colors]
enabled = true
symbol = "●"
block = false

[general.logo]
enabled = true
distro = "auto"
protocol = "auto"

[[modules]]
name = "os"
label = "OS"

[[modules]]
name = "kernel"
label = "Kernel"

[[modules]]
name = "uptime"
label = "Uptime"

[[modules]]
name = "packages"
label = "Packages"

[[modules]]
name = "break"

[[modules]]
name = "display"
label = "Display"

[[modules]]
name = "cpu"
label = "CPU"

[[modules]]
name = "gpu"
label = "GPU"

[[modules]]
name = "memory"
label = "Memory"
bar = true
bar_width = 10

[[modules]]
name = "disk"
label = "Disk (/)"
bar = true
bar_width = 10

[[modules]]
name = "break"

[[modules]]
name = "wifi"
label = "Wi-Fi"

[[modules]]
name = "processes"
label = "Processes"

[[modules]]
name = "colors"
```
