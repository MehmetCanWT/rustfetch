# Changelog

All notable changes to **RustFetch** are documented in this file.
RustFetch follows [Semantic Versioning](https://semver.org/) paired with **Cat Breed Codenames**.

## Version Codename Registry (Kedi Cinsi Kod Adları)

| Version | Codename | Status |
|---|---|---|
| `v0.1.0` | **Turkish Van** | Released |
| `v0.1.1` | **Turkish Angora** | Released |
| `v0.1.2` | **Bengal** | Current |
| `v0.1.3` | **Maine Coon** | Planned |
| `v0.1.4` | **Siamese** | Planned |
| `v0.1.5` | **Ragdoll** | Planned |
| `v0.2.0` | **Abyssinian** | Planned |
| `v0.3.0` | **Savannah** | Planned |
| `v0.4.0` | **Sphynx** | Planned |
| `v1.0.0` | **Anatolian** | Planned |

---

## [v0.1.2] — "Bengal" (2026-10-27)

### ✨ New Features
- **New Telemetry Modules (`os_age`, `power_profile`, `gpu_driver`, `net_io`)**: Added root filesystem birth age via `statx(STATX_BTIME)`, ACPI platform power profile & scaling governor, bound GPU kernel driver & version, and live network RX/TX throughput.
- **Multi-Disk Mounts & Battery Health**: Added `path = "/home"` (or `disk:/mnt/nvme`) support for multiple disk entries and battery cycle count + design-vs-full health percentage reporting.
- **Interactive TUI Setup (`--setup`), Doctor (`--doctor`), & Self-Updater (`--self-update`)**: Added interactive terminal configuration builder with `.bak` backups, system/terminal capability diagnostics, discovery flags (`--list-modules`, `--list-logos`, `--list-themes`, `--list-presets`), and atomic in-place binary updates.
- **Sixel Graphics, Braille 3D Shading, & Live Sparklines (`--live`)**: Added DEC Sixel encoder, 2x4 Unicode Braille (`U+2800`) 3D rasterization, and real-time CPU/RAM sparkline graphs with hotkeys in `--live` mode.
- **GitHub Update Notification (`update` Module)**: Automatically checks the GitHub repository for newer releases (with a 6-hour disk cache TTL for zero startup latency) and displays `󰚰 Update : Update available (v0.1.2 -> vX.Y.Z "Codename")` with an OSC 8 clickable link when an update is available.
- **Cat Breed Version Codenames**: Every semantic version now carries an official cat breed codename displayed in `rustfetch --version`, update notifications, and GitHub Releases.
- **Bazzite & Immutable Distro Support**: Added embedded Bazzite ASCII logos (`bazzite` and `bazzite_small`), Nerd Font icon mapping, deduplicated Homebrew (`brew`) package counting in `packages`, 3D engine Bazzite/Nobara colors, and `rpm-ostree` / `brew` support in `install.sh`.
- **Clean Distro-Native Release Artifacts**: Replaced generic `*-unknown-*` target triples with clean platform/distro package names (`arch`, `fedora`, `debian`, `nixos`, `linux-gnu`, `linux-musl`) and native extensions (`.pkg.tar.zst`, `.rpm`, `.deb`, `.tar.gz`), plus `flake.nix`.
- **Multi-Stop Text Gradients & 3D Directional Lighting**: Wired `general.gradient` in `config.toml` and `general.logo.three_d.light` (`top-left`, `top-right`, `front`, `top`, `bottom`).

### 🐛 Bug Fixes
- **Kitty Window Resize Sync & Portrait Image Fitting**: Added Kitty Synchronized Output (`?2026h`), image height capping, `.color` palette caching, and `PROMPT_COMMAND` / `stty size` resize hook synchronization so rapid window shrinking/enlarging never overlaps images and text.
- **Instant 3D-to-2D ASCII Fallback**: Pressing any key or running any command in `--3d` mode now clears the 3D frame and immediately renders the matching static 2D ASCII logo of the active distro without consuming keystrokes.
- **Immutable / Ostree Root Disk Reporting**: Fixed `Disk (/)` showing `0.0 GiB / 0.0 GiB (100%)` on Bazzite and Fedora Silverblue/Kinoite (`composefs` root) by falling back to `/sysroot` and `/var`.
- **Header Hostname Detection**: Fixed missing hostname (`user@`) on systems with an empty `/etc/hostname` by reading `/proc/sys/kernel/hostname` first.
- **Desktop & Window Manager Deduplication**: Fixed duplicate WM strings such as `Hyprland (Hyprland Wayland)` and added detection for `MangoWM`, `Niri`, `River`, `COSMIC`, `Wayfire`, `dwl`, and `labwc`.

---

## [v0.1.1] — "Turkish Angora" (2026-10-14)

### ✨ New Features
- Added configurable border image placement (`behind`, `left`, `right`, `top`, `bottom`, `frame`) and multi-image overlay support (`[[general.border_images]]`).
- Added custom box border styles (`rounded`, `single`, `double`, `thick`, `dashed`, `brackets`, `ascii`, `block`) and customizable border character sets.
- Added frame-by-frame multi-frame ASCII animation engine (`--ascii-anim`) and custom ASCII art support (`--ascii`).
- Added comprehensive English documentation hub under `docs/`.

### 🐛 Bug Fixes
- Fixed Kitty graphics protocol cursor positioning and background z-index layering.
- Resolved Clippy warnings and standardized formatting across all workspace crates.

---

## [v0.1.0] — "Turkish Van" (2026-10-01)

### ✨ Initial Release
- Initial public release of **RustFetch** (`rustfetch` and `rfetch`) with three-crate workspace architecture (`rustfetch-core`, `rustfetch-render`, `rustfetch`).
- Zero-fork Linux telemetry modules (`os`, `host`, `board`, `kernel`, `uptime`, `packages`, `shell`, `desktop`, `font`, `terminal`, `cpu`, `cpu_usage`, `temp`, `gpu`, `memory`, `swap`, `disk`, `battery`, `brightness`, `display`, `sound`, `media`, `wifi`, `bluetooth`, `local_ip`, `locale`, `processes`, `init`, `container`).
- Real-time 3D Z-buffer ASCII relief engine (`--3d`) with Blinn-Phong lighting and Unicode quadrant/sextant sub-cell rasterization.
- Kitty Graphics Protocol (static + GIF animations), DEC Sixel, and Unicode Halfblock image rendering.
- Fastfetch JSON/JSONC configuration importer (`--import-fastfetch`) and built-in presets (`default`, `minimal`, `clean`, `dots`, `card`, `brackets`, `neofetch`, `retro`, `modern`, `compact`).
