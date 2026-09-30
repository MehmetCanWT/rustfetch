# RustFetch

**RustFetch** is a blazing-fast (~4ms), memory-safe, and highly customizable system information fetch tool for Linux, written entirely in pure Rust.

Designed as a modern, zero-fork alternative to `neofetch` and `fastfetch`, RustFetch matches or beats the native performance of C-based fetchers by querying Linux `sysfs` and `procfs` directly, avoiding expensive subprocess invocations entirely.

```
       /\         mehmetcan@fedora
      /  \        -----------------
     /\   \       OS: Fedora Linux 41 (Workstation Edition) x86_64
    /      \      Host: Katana GF66 11UC (REV:1.0)
   /   ,,   \     Kernel: 6.13.1-200.fc41.x86_64
  /   |  |  -\    Uptime: 4 hours, 12 mins
 /_-''    ''-_\   Packages: 2418 (rpm), 12 (flatpak)
                  Shell: bash 5.2.32
                  Display: 1920x1080 @ 144Hz
                  DE: GNOME 47.2
                  WM: Mutter (Wayland)
                  Terminal: alacritty
                  CPU: 11th Gen Intel i5-11400H (12) @ 4.500GHz
                  GPU: NVIDIA GeForce RTX 3050 Mobile
                  Memory: 3.84 GiB / 15.38 GiB (24%) [■■■-------]
                  Battery: 100% [Full]
                  Wi-Fi: CanHome (70dBm - 60%)
                  Processes: 342 (load: 0.42)
                  Palette: ● ● ● ● ● ● ● ●
```

---

## Key Features

* **Universal Distro Support:** Native ASCII logos and pure-filesystem package counters for Fedora, Arch, Ubuntu, Debian, Alpine, Void, NixOS, Gentoo, openSUSE, Mint, Manjaro, Pop!_OS, EndeavourOS, Kali, SteamOS, Artix, Red Hat / CentOS / Rocky / Alma.
* **Universal Terminal Compatibility:** Seamlessly adapts across GUI emulators (Kitty, Alacritty, Ghostty, Foot, WezTerm), Linux Virtual Consoles (`TERM=linux` automatically downsamples 24-bit TrueColor to 16 ANSI colors), dumb terminals, and standard `NO_COLOR` environments.
* **Microsecond Execution (~4ms):** Reads hardware topology directly from `/sys/bus/pci/devices`, `/sys/class/drm`, and `/proc`, bypassing `lspci`, `xrandr`, and shell sub-processes.
* **Built-in Presets (`--preset`):** Instant layouts including `card` (rounded box container), `minimal` (terse hardware view), `modern` (dot separators and banners), and `compact`.
* **Microsecond Telemetry Profiler (`--benchmark`):** Profile exact microsecond execution latency across all active telemetry modules with visual progress bars.
* **Fastfetch Auto-Migration:** Automatically inspects existing Fastfetch configurations (`~/.config/fastfetch/config.jsonc`) in a read-only manner on first run. Migrates module ordering, custom icons, and logo paths to `~/.config/rustfetch/config.toml` without modifying your Fastfetch setup.
* **Modern Image Rendering:** Full support for the Kitty graphics protocol (`protocol = "kitty"`) and universal TrueColor ANSI half-block rendering (`protocol = "halfblock"`).
* **Native Dominant Color Extraction (`auto_color`):** Automatically extracts vibrant accent colors from images in pure Rust using HSV saturation scoring.
* **Dual Commands (`rustfetch` & `rfetch`):** Both standard and short executable names are installed out of the box with shell completions (Bash, Zsh, Fish) and a UNIX man page (`man rustfetch`).

---

## Performance & Benchmarks

RustFetch is engineered under the philosophy that system information tools should never slow down terminal startup.

### Comparative Benchmark

*Measurements on Intel Core i5-11400H (Fedora Linux, warmed caches, 50-iteration average).*

| Fetch Tool | Language | Execution Time | Architecture Notes |
| :--- | :--- | :--- | :--- |
| **RustFetch** | **Rust** | **~4.2 ms** | Direct `sysfs`/`procfs` parsing, pure Rust EDID decoding, zero subprocesses. |
| **Fastfetch** | C | ~6.0 ms | Highly optimized C, dynamic library linking. |
| **Neofetch** | Bash | ~250.0 ms | Spawns dozens of shell forks (`awk`, `sed`, `grep`, `lspci`). |

### Profiling with `--benchmark`

RustFetch includes a built-in profiler that measures every collector's microsecond footprint:

```bash
rustfetch --benchmark
```

```text
============================================================
              RustFetch Execution Benchmark
============================================================
  os               │    42 µs │ ▓▒░░░░░░░░░░░░░░░░░░░░░░░░░
  host             │    18 µs │ █░░░░░░░░░░░░░░░░░░░░░░░░░
  kernel           │     8 µs │ █░░░░░░░░░░░░░░░░░░░░░░░░░
  uptime           │     9 µs │ █░░░░░░░░░░░░░░░░░░░░░░░░░
  packages         │   480 µs │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
  display          │   120 µs │ ▓▓▓▓▓▓█░░░░░░░░░░░░░░░░░░░
  cpu              │    65 µs │ ▓▓▓█░░░░░░░░░░░░░░░░░░░░░░
  gpu              │    95 µs │ ▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░
  memory           │    22 µs │ █░░░░░░░░░░░░░░░░░░░░░░░░░
  wifi             │   310 µs │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░
  processes        │   180 µs │ ▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░
------------------------------------------------------------
  Total Pipeline   │  1.34 ms
============================================================
```

---

## Installation

### 1. One-Line Install (Recommended)

Universal installer for any Linux distribution. Compiles optimized release binary, sets up symlinks (`rustfetch` and `rfetch`), installs man pages and shell completions:

```bash
curl -fsSL https://raw.githubusercontent.com/MehmetCanWT/rustfetch/main/install.sh | bash
```

### 2. Arch Linux (AUR / PKGBUILD)

```bash
git clone https://github.com/MehmetCanWT/rustfetch.git
cd rustfetch/packaging/arch
makepkg -si
```

### 3. Fedora / RHEL / CentOS

An RPM spec file is included at `packaging/rpm/rustfetch.spec`:

```bash
rpmbuild -ba packaging/rpm/rustfetch.spec
```

### 4. Build from Source (Cargo)

```bash
git clone https://github.com/MehmetCanWT/rustfetch.git
cd rustfetch
cargo build --release -p rustfetch
sudo cp target/release/rustfetch /usr/local/bin/
sudo ln -sf /usr/local/bin/rustfetch /usr/local/bin/rfetch
```

---

## Preset Themes (`--preset`)

RustFetch provides built-in visual presets that can be triggered on the command line or set in your configuration:

| Preset | Command | Description |
| :--- | :--- | :--- |
| **`card`** | `rustfetch --preset card` | Encapsulates output in a rounded Unicode card (`╭───╮`, `│   │`, `╰───╯`) with centered logo. |
| **`minimal`** | `rustfetch --preset minimal` | Clean, logo-free hardware summary ideal for minimalists or server MOTDs. |
| **`modern`** | `rustfetch --preset modern` | Dot-separated labels (`OS ─ Fedora`) with vibrant banner accents. |
| **`compact`** | `rustfetch --preset compact` | Two-character terse labels (`os`, `kr`, `up`, `pk`, `cp`, `gp`, `mm`). |
| **`default`** | `rustfetch --preset default` | Standard aesthetic layout with auto-detected distro ASCII art. |

---

## Command Line Options

```text
Usage: rustfetch [OPTIONS] (or rfetch [OPTIONS])

Options:
      --preset <NAME>          Apply a built-in layout preset [card, minimal, modern, compact, default]
      --benchmark              Run microsecond execution profiler
  -c, --config <PATH>          Path to custom TOML config file
  -i, --image <PATH>           Render image instead of ASCII logo (Kitty protocol / halfblock)
  -l, --logo <NAME>            Override ASCII logo (e.g. arch, fedora, ubuntu, void, alpine, etc.)
      --color <COLOR>          Override primary accent color (name or hex like #ff79c6)
      --align <ALIGN>          Output alignment: left, right, or center
      --center                 Center output horizontally in terminal window
      --no-logo                Disable logo completely
      --live                   Run in real-time monitor mode
      --json                   Output telemetry in structured JSON
      --dump-config            Print active configuration to stdout
      --generate-config        Generate default config file in ~/.config/rustfetch/config.toml
      --import-fastfetch       Import Fastfetch configuration into RustFetch TOML
      --completions <SHELL>    Generate shell completions [bash, zsh, fish]
  -h, --help                   Print help
  -V, --version                Print version
```

---

## Supported Telemetry Modules

> 📖 **Comprehensive Guide:** For full technical details on each module, data sources, progress bar settings, and custom format templates, see the **[Telemetry Modules Guide (MODULES.md)](MODULES.md)**.

| Module | Description | Implementation Strategy |
| :--- | :--- | :--- |
| `os` | Distro name, ID, and version | Parsed from `/etc/os-release` |
| `kernel` | Linux kernel release & architecture | Pure `libc::uname` |
| `uptime` | System uptime in hours and minutes | Direct `/proc/uptime` parsing |
| `packages` | Package manager counts (pacman, dpkg, rpm, flatpak, apk, xbps, nix, emerge, snap) | Pure filesystem traversal with TTL cache |
| `shell` | User login shell & version | Read from `$SHELL` and binary inspection |
| `display` | Connected monitors, resolutions & refresh rates | Pure Rust sysfs DRM & EDID Detailed Timing decoding |
| `de` | Desktop Environment | `$XDG_CURRENT_DESKTOP` and session detection |
| `wm` | Window Manager | Wayland socket or X11 property inspection |
| `terminal` | Terminal emulator | Parent PID inspection via `/proc/$PID/stat` |
| `cpu` | Processor model, cores, frequency & temperature | `/proc/cpuinfo` & `/sys/class/hwmon` |
| `gpu` | Dedicated & integrated graphics cards | PCI class scan in `/sys/bus/pci/devices` via `pci.ids` |
| `memory` | RAM usage, total, and percentage progress bar | `/proc/meminfo` |
| `swap` | Swap space usage & percentage | `/proc/meminfo` |
| `disk` | Filesystem mount usage & capacity | `libc::statvfs` |
| `battery` | Battery state, capacity, and progress bar | `/sys/class/power_supply` |
| `sound` | Audio sink volume & mute status | WirePlumber (`wpctl`) / PulseAudio |
| `wifi` | Wireless network SSID & signal strength | `/sys/class/net` & `iw dev link` / `iwgetid` |
| `processes` | Total running processes & 1-minute load average | `/proc/loadavg` & PID count |
| `media` | Currently playing MPRIS track (artist - title) | Optional module via `playerctl` (disabled by default) |
| `colors` | ANSI color palette preview (dots or blocks) | Terminal color sequence blocks |
| `custom` | Custom static or formatted text | User-defined TOML configuration |
| `break` | Blank spacer line | Formatting spacer |

---

## Configuration

The configuration file is located at `~/.config/rustfetch/config.toml`.

### Example `config.toml`

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
protocol = "auto"          # "auto", "kitty", or "halfblock"
# auto_color = true        # extract accent color from image

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
name = "wifi"
label = "Wi-Fi"

[[modules]]
name = "processes"
label = "Processes"

[[modules]]
name = "colors"
```

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
