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
- **GitHub Update Notification (`update` Module)**: Automatically checks the GitHub repository for newer releases (with a 6-hour disk cache TTL for zero startup latency) and displays `󰚰 Update : Update available (v0.1.2 -> vX.Y.Z "Codename")` at the bottom of the modules list when an update is available. Can be disabled via `check_updates = false` in `config.toml` or `--no-update-check` on the CLI.
- **Cat Breed Version Codenames**: Every semantic version now carries an official cat breed codename displayed in `rustfetch --version`, update notifications, and GitHub Releases.
- **Bazzite & Immutable Distro Support**: Added embedded Bazzite ASCII logos (`bazzite` and `bazzite_small`), Nerd Font icon mapping, Homebrew (`brew`) package counting in `packages`, 3D engine Bazzite/Nobara colors, and `rpm-ostree` / `brew` support in `install.sh`.
- **Clean Distro-Native Release Artifacts**: Replaced generic `*-unknown-*` target triples with clean platform/distro package names (`arch`, `fedora`, `debian`, `nixos`, `linux-gnu`, `linux-musl`) and native extensions (`.pkg.tar.zst`, `.rpm`, `.deb`, `.tar.gz`).
- **Multi-Stop Text Gradients**: Wired `general.gradient` in `config.toml` to smoothly interpolate TrueColor gradients across module labels and icons.
- **3D Directional Lighting Control**: Wired `general.logo.three_d.light` (`top-left`, `top-right`, `front`, `top`, `bottom`) into `Renderer3D`.

### 🐛 Bug Fixes
- **Immutable / Ostree Root Disk Reporting**: Fixed `Disk (/)` showing `0.0 GiB / 0.0 GiB (100%)` on Bazzite and Fedora Silverblue/Kinoite (`composefs` root) by falling back to `/sysroot` and `/var`.
- **Header Hostname Detection**: Fixed missing hostname (`user@`) on systems with an empty `/etc/hostname` by reading `/proc/sys/kernel/hostname` first.
- **Desktop & Window Manager Deduplication**: Fixed duplicate WM strings such as `Hyprland (Hyprland Wayland)`, added detection for `MangoWM`, `Niri`, `River`, `COSMIC`, `Wayfire`, `dwl`, and `labwc`, and restricted `/proc` scanning to numeric PID directories.
- **GTK Font Formatting**: Normalized extra commas and spaces in `gtk-font-name` output.

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
