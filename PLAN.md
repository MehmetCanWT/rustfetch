# Image-First Linux System Fetch Tool in Rust: Project Architecture & Roadmap

Every phase defines: **Objective → Action Items → Definition of Done**.

## Vision

- Linux-first with universal distribution compatibility.
- Blazing-fast runtime: text mode $\le$ 10 ms, cached images $\le$ 20 ms.
- Key differentiator: **High-fidelity images & animations across any terminal, dynamic resize support, zero-hassle configuration**.
- Focus on quality over quantity: maintain ~20 well-optimized core telemetry modules rather than bloated feature sets.

---

## Phase 0: Scope & Foundation

- **Target Terminals:** Kitty, Ghostty, WezTerm, Foot, Alacritty (TrueColor half-block fallback).
- **Out of Scope:** Windows/macOS, 100+ rarely used modules, competing solely on module counts.
- **Baseline:** Benchmark against Fastfetch with `hyperfine`.
- **Repository Architecture:** Cargo workspace with three crates:
  - `core`: telemetry collectors and sysfs/procfs parsing
  - `render`: Kitty protocol and half-block graphics rendering
  - `cli`: command-line entrypoint and presentation layout
- **Testability Rule:** Every detection module is partitioned into:
  - I/O readers accessing virtual filesystems
  - Pure parsing functions without side effects
  - Fixture-based unit tests from real distributions without requiring virtual machines.
- **Definition of Done:** Repository structure, Cargo workspace, and baseline test suite operational.

---

## Phase 1: Core Telemetry Engine

- `Module` trait: `detect() -> Option<Info>`. Failed detection silently hides the module; panics are strictly forbidden.
- **Core Modules:** `os`, `host`, `kernel`, `uptime`, `shell`, `cpu`, `memory`, `swap`, `disk`, `terminal`, `locale`.
  - Data sources: `/proc`, `/sys`, `/etc/os-release`, `statvfs`, `uname`.
  - Zero external subprocess forks (`pacman`, `lspci`, etc.).
- Modules not declared in configuration are never executed (lazy execution).
- Collectors run concurrently using `std::thread::scope`.
- **Definition of Done:** Accurate, zero-fork text output validated against real hardware metrics.

---

## Phase 2: Configuration & Layout Engine

- **TOML Configuration:** Custom module order, labels, icons, colors, format templates (`"{value}"`), separators, and padding. Works out of the box with zero configuration.
- **CLI Options:** `--dump-config`, `--config <PATH>`, `--preset <NAME>`, `--benchmark`. User-friendly error diagnostics and XDG path compliance.
- **Layout:** Left logo, right info block. Unicode character width calculation using `unicode-width`. Automatic logo suppression on narrow terminal windows.
- **Preset Themes:** Built-in styles (`card`, `minimal`, `modern`, `compact`, `default`).
- **Definition of Done:** Swapping presets or TOML configs completely transforms layout formatting cleanly.

---

## Phase 3: High-Fidelity Image Rendering

### 3a. Terminal Capability Detection
- Check environment indicators (`KITTY_WINDOW_ID`, `GHOSTTY_RESOURCES_DIR`, `TERM_PROGRAM`, `TERM`), followed by protocol queries.
- Determine terminal character cell pixel dimensions (`TIOCGWINSZ`).

### 3b. Protocol Renderers
- Kitty graphics protocol (direct transmit with 4096-byte chunking).
- Universal TrueColor ANSI half-block (`▀` / `▄`) fallback engine.

### 3c. Image Pipeline
- Fast decoding and resizing using `fast_image_resize` before transmission.
- Scaling modes: contain, cover, crop, and stretch.
- Disk caching with SHA-256 keys to avoid repeated decoding.

### 3d. Vertical Alignment & Cursor Management
- Offset cursor appropriately when image height exceeds telemetry line count.

---

## Phase 4: Live Mode & Dynamic Resizing

- **`--live`:** Listens for `SIGWINCH` resize events and updates metrics without flickering.
- Telemetry metrics (uptime, RAM, load) refresh at regular intervals (`q` to exit).

---

## Phase 5: Animated Graphics

- Frame decoding for GIF, APNG, and WebP with frame delays and disposal rules.
- Kitty animation protocol for hardware-accelerated playback.
- Half-block animation loop fallback for non-Kitty terminals.

---

## Phase 6: Universal Hardware & Distribution Expansion

- **GPU:** `/sys/class/drm` and pure PCI scanning via `/usr/share/hwdata/pci.ids`.
- **Package Counters:** Direct filesystem reading for pacman, dpkg, apk, xbps, nix, portage, flatpak, and snap.
- **Display:** Pure Rust EDID Detailed Timing decoding for monitor refresh rates (e.g. `144Hz`).
- **Network & Audio:** Wi-Fi link parameters, private IP detection, and WirePlumber audio volume.
- **ASCII Art:** Complete library of authentic distribution logos.

---

## Phase 7: Micro-Optimizations & Profiling

- Microsecond execution profiling via `--benchmark`.
- Strict compiler optimization flags:
  ```toml
  [profile.release]
  lto = "fat"
  codegen-units = 1
  panic = "abort"
  strip = true
  ```
- Verified hot execution runtimes of ~4 milliseconds.

---

## Phase 8: Testing & Quality Assurance

- Unit tests for all parsers with distribution fixtures.
- Strict linter configuration: `cargo clippy --all-targets --all-features -- -D warnings`.
- Code formatting verification: `cargo fmt --all -- --check`.

---

## Phase 9: Packaging & Deployment

- Automated multi-architecture release pipeline via GitHub Actions.
- Arch Linux AUR PKGBUILD (`packaging/arch/PKGBUILD`).
- Fedora/RHEL RPM Spec (`packaging/rpm/rustfetch.spec`).
- Universal one-line installer (`install.sh`).
- UNIX manual page (`docs/rustfetch.1`).
