//! Built-in CLI utilities: discovery (`--list-*`), diagnostics (`--doctor`),
//! cache management (`--clean-cache`), self-updater (`--self-update`),
//! and interactive configuration wizard (`--setup` / `--tui`).

use crate::layout::{self, BOLD, DIM, RESET};
use crate::logos;
use rustfetch_core::cache::Cache;
use rustfetch_core::config::{default_icon_for_module, Config};
use rustfetch_core::modules::module_by_name;
use rustfetch_core::modules::update::{
    check_latest_release, codename_for_version, is_newer_version, CURRENT_VERSION,
};
use std::io::Write;
use std::process::Command;

pub const MODULE_DESCRIPTIONS: &[(&str, &str)] = &[
    ("os", "Operating system distribution and architecture"),
    ("os_age", "Operating system installation age (statx btime)"),
    ("host", "System hardware model / motherboard product name"),
    ("board", "Motherboard vendor and model"),
    ("kernel", "Linux kernel release version"),
    ("uptime", "System uptime formatted in days/hours/minutes"),
    ("packages", "Installed package counts across all managers"),
    ("shell", "Current user shell and version"),
    (
        "desktop",
        "Desktop environment, window manager, and display server",
    ),
    ("font", "System UI font family and size"),
    ("terminal", "Active terminal emulator"),
    ("cpu", "CPU model, core/thread count, and max frequency"),
    (
        "cpu_usage",
        "Real-time aggregate CPU utilization percentage",
    ),
    ("temp", "CPU / hardware thermal sensor temperature"),
    (
        "gpu",
        "Graphics processor (priority dGPU with iGPU fallback)",
    ),
    (
        "dgpu",
        "Dedicated discrete GPU only (e.g. NVIDIA / AMD dGPU)",
    ),
    (
        "igpu",
        "Integrated GPU only (e.g. Intel UHD / AMD Radeon iGPU)",
    ),
    ("gpu_driver", "Active kernel GPU driver and module version"),
    ("memory", "RAM usage, total capacity, and percentage"),
    ("swap", "Swap memory usage and total capacity"),
    (
        "disk",
        "Filesystem usage (supports custom mount `path = \"/home\"`)",
    ),
    ("battery", "Battery charge percentage and status"),
    (
        "power_profile",
        "ACPI platform power profile and CPU scaling governor",
    ),
    ("brightness", "Display backlight brightness percentage"),
    (
        "display",
        "Connected monitor model, resolution, and refresh rate",
    ),
    ("sound", "Audio sink volume and mute state"),
    ("media", "Currently playing MPRIS media track"),
    (
        "bluetooth",
        "Connected Bluetooth devices and battery levels",
    ),
    ("wifi", "Connected Wi-Fi SSID, signal quality, and band"),
    (
        "net_io",
        "Active network interface cumulative RX / TX traffic",
    ),
    ("local_ip", "Primary LAN IPv4 address and interface"),
    ("locale", "System language and character encoding"),
    ("processes", "Load average and running/total process counts"),
    (
        "init",
        "PID 1 init system (systemd, openrc, runit, dinit, s6)",
    ),
    ("container", "Container or VM runtime environment"),
    (
        "update",
        "GitHub release update notifier with cat breed codename",
    ),
    ("custom", "Custom user-defined static label and value row"),
    ("break", "Blank vertical spacer line"),
    ("divider", "Horizontal section divider rule"),
];

pub const PRESET_DESCRIPTIONS: &[(&str, &str)] = &[
    (
        "default",
        "Balanced full system overview with Nerd Font icons and color palette",
    ),
    (
        "minimal",
        "Ultra-clean text-only layout without logo, icons, or palette",
    ),
    ("clean", "Icon-only key column with zero separator noise"),
    (
        "dots",
        "Bullet-separated icon-only telemetry with dot color row",
    ),
    (
        "card",
        "Centered rounded border card with sectioned dividers",
    ),
    ("brackets", "Centered bracket-framed box layout"),
    (
        "neofetch",
        "Classic Neofetch-inspired layout with 2-row solid color blocks",
    ),
    (
        "retro",
        "Rounded frame with Pac-Man color symbols and progress bars",
    ),
    (
        "modern",
        "Centered layout with arrow separators and inline progress bars",
    ),
    (
        "compact",
        "Essential 7-module summary for small terminal windows",
    ),
];

pub const THEME_NAMES: &[&str] = &[
    "catppuccin",
    "nord",
    "dracula",
    "gruvbox",
    "tokyonight",
    "rosepine",
    "solarized",
    "monokai",
    "cyberpunk",
];

pub fn run_list_modules() {
    println!(
        "{BOLD}Available RustFetch Modules ({}):{RESET}\n",
        MODULE_DESCRIPTIONS.len()
    );
    for &(name, desc) in MODULE_DESCRIPTIONS {
        let icon = default_icon_for_module(name).unwrap_or(" ");
        let preview = if matches!(name, "update" | "media" | "custom" | "break" | "divider") {
            String::new()
        } else {
            module_by_name(name)
                .and_then(|m| m.detect())
                .map(|info| format!(" {DIM}=> {}{RESET}", info.value))
                .unwrap_or_default()
        };
        println!("  \x1b[36m{icon}\x1b[0m  {BOLD}{name:<15}{RESET} {desc}{preview}");
    }
}

pub fn run_list_themes() {
    println!(
        "{BOLD}Available Built-in Color Themes ({}):{RESET}\n",
        THEME_NAMES.len()
    );
    for &name in THEME_NAMES {
        if let Some(t) = rustfetch_render::get_theme(name) {
            let (pr, pg, pb) = t.primary;
            let (sr, sg, sb) = t.secondary;
            let (br, bg, bb) = t.border;
            println!(
                "  {BOLD}{name:<14}{RESET} \x1b[48;2;{pr};{pg};{pb}m   \x1b[0m Primary (#{pr:02x}{pg:02x}{pb:02x})   \x1b[48;2;{sr};{sg};{sb}m   \x1b[0m Secondary (#{sr:02x}{sg:02x}{sb:02x})   \x1b[48;2;{br};{bg};{bb}m   \x1b[0m Border (#{br:02x}{bg:02x}{bb:02x})"
            );
        }
    }
    println!("\n{DIM}Usage: rustfetch --theme <THEME>  or  set `theme = \"catppuccin\"` in ~/.config/rustfetch/config.toml{RESET}");
}

pub fn run_list_presets() {
    println!(
        "{BOLD}Available Layout Presets ({}):{RESET}\n",
        PRESET_DESCRIPTIONS.len()
    );
    for &(name, desc) in PRESET_DESCRIPTIONS {
        println!("  \x1b[32m●\x1b[0m  {BOLD}{name:<12}{RESET} {desc}");
    }
    println!("\n{DIM}Usage: rustfetch --preset <PRESET>{RESET}");
}

pub fn run_list_logos() {
    let logos = logos::list_available_logos();
    println!("{BOLD}Available Built-in Logos ({}):{RESET}\n", logos.len());
    for &(key, label) in logos {
        let sample = logos::get_logo(key);
        println!(
            "  {}██{RESET}  {BOLD}{key:<16}{RESET} {label}",
            sample.color
        );
    }
    println!("\n{DIM}Usage: rustfetch -l <LOGO>  or  rustfetch --3d -l <LOGO>{RESET}");
}

pub fn run_clean_cache() {
    let dir = Cache::cache_dir();
    let count = Cache::clear_all();
    println!(
        "\x1b[32m✓\x1b[0m Cleared {count} cached entry/entries from {}",
        dir.display()
    );
}

pub fn run_doctor() {
    let ver = CURRENT_VERSION;
    let code = codename_for_version(ver);
    println!("{BOLD}RustFetch System & Terminal Diagnostics{RESET}");
    println!("────────────────────────────────────────────────────────");
    println!("  {BOLD}Version:{RESET}         v{ver} \"{code}\"");
    println!("  {BOLD}Architecture:{RESET}    {}", std::env::consts::ARCH);

    let (cols, rows) = layout::terminal_size();
    let term_env = std::env::var("TERM").unwrap_or_else(|_| "(unset)".to_string());
    let colorterm = std::env::var("COLORTERM").unwrap_or_else(|_| "(unset)".to_string());
    let kitty = rustfetch_render::supports_kitty();
    let sixel = rustfetch_render::supports_sixel();
    let iterm = rustfetch_render::supports_iterm();
    let (cw, ch) = rustfetch_render::cell_size_or_default();

    println!("  {BOLD}Terminal:{RESET}        TERM={term_env}, COLORTERM={colorterm}");
    println!("  {BOLD}Grid Size:{RESET}       {cols} cols × {rows} rows (cell: {cw}×{ch} px)");
    println!(
        "  {BOLD}Graphics:{RESET}        Kitty={}  Sixel={}  iTerm2={}  TrueColor={}",
        if kitty { "\x1b[32myes\x1b[0m" } else { "no" },
        if sixel { "\x1b[32myes\x1b[0m" } else { "no" },
        if iterm { "\x1b[32myes\x1b[0m" } else { "no" },
        if colorterm.contains("truecolor") || colorterm.contains("24bit") {
            "\x1b[32myes\x1b[0m"
        } else {
            "auto"
        }
    );

    let cfg_path = Config::config_path();
    let cfg_status = if cfg_path.is_file() {
        match Config::load_from(&cfg_path) {
            Ok(_) => format!("\x1b[32mvalid\x1b[0m ({})", cfg_path.display()),
            Err(e) => format!("\x1b[31merror\x1b[0m ({e})"),
        }
    } else {
        format!("not created yet (will use {})", cfg_path.display())
    };
    println!("  {BOLD}Config File:{RESET}     {cfg_status}");

    let cache_dir = Cache::cache_dir();
    let cache_count = std::fs::read_dir(&cache_dir)
        .map(|it| it.flatten().count())
        .unwrap_or(0);
    println!(
        "  {BOLD}Cache Dir:{RESET}       {} ({cache_count} active files)",
        cache_dir.display()
    );

    if let Some((latest, latest_code)) = check_latest_release() {
        if is_newer_version(ver, &latest) {
            println!(
                "  {BOLD}Update Status:{RESET}   \x1b[33mUpdate available: v{latest} \"{latest_code}\"\x1b[0m (run `rustfetch --self-update`)"
            );
        } else {
            println!(
                "  {BOLD}Update Status:{RESET}   \x1b[32mUp to date\x1b[0m (v{ver} \"{code}\")"
            );
        }
    } else {
        println!("  {BOLD}Update Status:{RESET}   {DIM}Offline / GitHub unreachable{RESET}");
    }
    println!("────────────────────────────────────────────────────────");
}

pub fn build_release_tarball_url(version: &str, arch: &str, is_musl: bool) -> String {
    let ver = version.trim().trim_start_matches(['v', 'V']);
    let libc_tag = if is_musl && arch == "x86_64" {
        "linux-musl"
    } else {
        "linux-gnu"
    };
    format!(
        "https://github.com/MehmetCanWT/rustfetch/releases/download/v{ver}/rustfetch-v{ver}-{libc_tag}-{arch}.tar.gz"
    )
}

pub fn run_self_update() {
    let current = CURRENT_VERSION;
    let cur_code = codename_for_version(current);
    println!(
        "{BOLD}==> Checking GitHub for RustFetch updates (current: v{current} \"{cur_code}\")...{RESET}"
    );

    let Some((latest, latest_code)) = check_latest_release() else {
        eprintln!(
            "Could not reach GitHub to check for updates. Please check your internet connection."
        );
        std::process::exit(1);
    };

    if !is_newer_version(current, &latest) {
        println!("\x1b[32m✓\x1b[0m RustFetch is already up to date (v{current} \"{cur_code}\").");
        return;
    }

    println!(
        "==> Updating RustFetch: {BOLD}v{current} \"{cur_code}\" -> v{latest} \"{latest_code}\"{RESET}"
    );
    let cfg_path = Config::config_path();
    if cfg_path.is_file() {
        println!(
            "\x1b[32m✓\x1b[0m Preserving existing user configuration at {}",
            cfg_path.display()
        );
    }

    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        other => {
            eprintln!("Unsupported prebuilt architecture '{other}'. Running source installer...");
            run_installer_fallback();
            return;
        }
    };

    let is_musl = std::path::Path::new("/etc/alpine-release").exists();
    let url = build_release_tarball_url(&latest, arch, is_musl);

    let tmp_dir = std::env::temp_dir().join(format!("rustfetch-update-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);
    if std::fs::create_dir_all(&tmp_dir).is_err() {
        eprintln!("Failed to create temporary update directory.");
        std::process::exit(1);
    }

    let archive_path = tmp_dir.join("rustfetch.tar.gz");
    println!("==> Downloading {url}...");
    let dl_ok = Command::new("curl")
        .args(["-fsSL", &url, "-o"])
        .arg(&archive_path)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !dl_ok {
        let _ = std::fs::remove_dir_all(&tmp_dir);
        println!("==> Prebuilt release archive not found; building latest from source (preserving config.toml)...");
        run_installer_fallback();
        return;
    }

    let extract_ok = Command::new("tar")
        .args(["-xzf"])
        .arg(&archive_path)
        .arg("-C")
        .arg(&tmp_dir)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !extract_ok {
        let _ = std::fs::remove_dir_all(&tmp_dir);
        eprintln!("Failed to extract downloaded archive.");
        std::process::exit(1);
    }

    let target_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_default();
            std::path::PathBuf::from(home).join(".local/bin")
        });

    let extracted_bin = find_file_named(&tmp_dir, "rustfetch");
    if let Some(bin_src) = extracted_bin {
        let dest_rustfetch = target_dir.join("rustfetch");
        let dest_rfetch = target_dir.join("rfetch");
        if install_binary_atomic(&bin_src, &dest_rustfetch).is_ok() {
            let _ = install_binary_atomic(&bin_src, &dest_rfetch);
            let _ = Cache::clear_all();
            let _ = std::fs::remove_dir_all(&tmp_dir);
            println!(
                "\x1b[32m✓\x1b[0m Successfully updated to {BOLD}RustFetch v{latest} \"{latest_code}\"{RESET} in {}!",
                target_dir.display()
            );
            return;
        }
    }

    let _ = std::fs::remove_dir_all(&tmp_dir);
    run_installer_fallback();
}

fn install_binary_atomic(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    let tmp_dst = dst.with_extension("new");
    std::fs::copy(src, &tmp_dst)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&tmp_dst, std::fs::Permissions::from_mode(0o755));
    }
    std::fs::rename(&tmp_dst, dst)
}

fn find_file_named(dir: &std::path::Path, filename: &str) -> Option<std::path::PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.file_name().and_then(|s| s.to_str()) == Some(filename) {
            return Some(path);
        } else if path.is_dir() {
            if let Some(found) = find_file_named(&path, filename) {
                return Some(found);
            }
        }
    }
    None
}

fn run_installer_fallback() {
    let status = Command::new("bash")
        .args([
            "-c",
            "curl -fsSL https://raw.githubusercontent.com/MehmetCanWT/rustfetch/main/install.sh | NONINTERACTIVE=1 bash",
        ])
        .status();
    let _ = Cache::clear_all();
    if status.map(|s| s.success()).unwrap_or(false) {
        println!("\x1b[32m✓\x1b[0m Update completed! Your existing config.toml was preserved.");
    } else {
        eprintln!("Update failed.");
        std::process::exit(1);
    }
}

pub fn run_setup_tui() {
    use crossterm::{
        cursor::{Hide, MoveTo, Show},
        event::{read, Event, KeyCode, KeyModifiers},
        execute, queue,
        terminal::{
            disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    };
    use std::io::IsTerminal;

    let mut cfg = Config::load();

    if !std::io::stdout().is_terminal() || !std::io::stdin().is_terminal() {
        eprintln!("Interactive setup (--setup / --tui) requires an interactive terminal (TTY).");
        return;
    }

    struct SetupGuard;
    impl Drop for SetupGuard {
        fn drop(&mut self) {
            let _ = disable_raw_mode();
            let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
        }
    }

    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen, Hide);
    let _ = enable_raw_mode();
    let _guard = SetupGuard;

    let themes = [
        "none",
        "catppuccin",
        "nord",
        "dracula",
        "gruvbox",
        "tokyonight",
        "rosepine",
        "solarized",
        "monokai",
        "cyberpunk",
    ];
    let border_styles = [
        "rounded", "single", "double", "thick", "dashed", "block", "ascii", "brackets",
    ];
    let logos_list = [
        "auto",
        "cat",
        "bengal",
        "turkish_van",
        "turkish_angora",
        "arch",
        "fedora",
        "bazzite",
        "nobara",
        "cachyos",
        "nixos",
        "ubuntu",
        "debian",
    ];

    let mut selected_row = 0usize;
    let total_rows = 8usize;
    let mut status_msg =
        String::from("Use ↑/↓ to select, ←/→ or Enter to toggle, [S] to Save, [Q] to Quit");

    loop {
        let _ = queue!(stdout, Clear(ClearType::All), MoveTo(0, 0));

        let theme_val = cfg.general.theme.as_deref().unwrap_or("none");
        let border_style_val = cfg.general.border_style.as_deref().unwrap_or("rounded");
        let mode_3d = cfg.general.logo.three_d.enabled;

        let items = [
            ("1. Color Theme", theme_val.to_string()),
            ("2. Logo / Mascot", cfg.general.logo.distro.clone()),
            (
                "3. Display Mode",
                if mode_3d {
                    "3D Animated".to_string()
                } else {
                    "Standard 2D".to_string()
                },
            ),
            (
                "4. Outer Border Box",
                if cfg.general.border {
                    "Enabled"
                } else {
                    "Disabled"
                }
                .to_string(),
            ),
            ("5. Border Style", border_style_val.to_string()),
            (
                "6. Nerd Font Icons",
                if cfg.general.icons {
                    "Enabled"
                } else {
                    "Disabled"
                }
                .to_string(),
            ),
            (
                "7. Center Output",
                if cfg.general.center {
                    "Enabled"
                } else {
                    "Disabled"
                }
                .to_string(),
            ),
            (
                "8. GitHub Update Check",
                if cfg.general.check_updates {
                    "Enabled"
                } else {
                    "Disabled"
                }
                .to_string(),
            ),
        ];

        print!("  \x1b[1;36m╭──────────────────────────────────────────────────────────────╮\x1b[0m\r\n");
        print!("  \x1b[1;36m│\x1b[0m  {BOLD}RustFetch Interactive Configuration Wizard (--setup){RESET}        \x1b[1;36m│\x1b[0m\r\n");
        print!("  \x1b[1;36m│\x1b[0m  {DIM}Existing settings are preserved; a .bak backup is kept.{RESET}   \x1b[1;36m│\x1b[0m\r\n");
        print!("  \x1b[1;36m╰──────────────────────────────────────────────────────────────╯\x1b[0m\r\n\r\n");

        for (idx, (label, val)) in items.iter().enumerate() {
            if idx == selected_row {
                print!("  \x1b[1;32m❯ {label:<26}\x1b[0m \x1b[1;33m◀ {val} ▶\x1b[0m\r\n");
            } else {
                print!("    {label:<26} {DIM}{val}{RESET}\r\n");
            }
        }

        let preview_logo = logos::get_logo(if cfg.general.logo.distro == "auto" {
            "cat"
        } else {
            &cfg.general.logo.distro
        });
        let preview_lines = preview_logo.colored_lines();
        print!(
            "\r\n  {BOLD}Live Logo Preview ({}):{RESET}\r\n",
            cfg.general.logo.distro
        );
        for line in preview_lines.iter().take(8) {
            print!("    {line}\r\n");
        }

        print!("\r\n  \x1b[36mℹ\x1b[0m {status_msg}\r\n");
        let _ = stdout.flush();

        let Ok(event) = read() else { break };
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::Up | KeyCode::Char('k') => {
                    selected_row = if selected_row == 0 {
                        total_rows - 1
                    } else {
                        selected_row - 1
                    };
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    selected_row = (selected_row + 1) % total_rows;
                }
                KeyCode::Right
                | KeyCode::Left
                | KeyCode::Enter
                | KeyCode::Char(' ')
                | KeyCode::Char('l')
                | KeyCode::Char('h') => {
                    let forward = !matches!(key.code, KeyCode::Left | KeyCode::Char('h'));
                    match selected_row {
                        0 => {
                            let cur = cfg.general.theme.as_deref().unwrap_or("none");
                            let pos = themes.iter().position(|&t| t == cur).unwrap_or(0);
                            let next = if forward {
                                (pos + 1) % themes.len()
                            } else if pos == 0 {
                                themes.len() - 1
                            } else {
                                pos - 1
                            };
                            if themes[next] == "none" {
                                cfg.general.theme = None;
                            } else {
                                cfg.general.theme = Some(themes[next].to_string());
                            }
                        }
                        1 => {
                            let cur = cfg.general.logo.distro.as_str();
                            let pos = logos_list.iter().position(|&l| l == cur).unwrap_or(0);
                            let next = if forward {
                                (pos + 1) % logos_list.len()
                            } else if pos == 0 {
                                logos_list.len() - 1
                            } else {
                                pos - 1
                            };
                            cfg.general.logo.distro = logos_list[next].to_string();
                        }
                        2 => {
                            cfg.general.logo.three_d.enabled = !cfg.general.logo.three_d.enabled;
                        }
                        3 => {
                            cfg.general.border = !cfg.general.border;
                        }
                        4 => {
                            let cur = cfg.general.border_style.as_deref().unwrap_or("rounded");
                            let pos = border_styles.iter().position(|&s| s == cur).unwrap_or(0);
                            let next = if forward {
                                (pos + 1) % border_styles.len()
                            } else if pos == 0 {
                                border_styles.len() - 1
                            } else {
                                pos - 1
                            };
                            cfg.general.border_style = Some(border_styles[next].to_string());
                        }
                        5 => {
                            cfg.general.icons = !cfg.general.icons;
                        }
                        6 => {
                            cfg.general.center = !cfg.general.center;
                        }
                        7 => {
                            cfg.general.check_updates = !cfg.general.check_updates;
                        }
                        _ => {}
                    }
                }
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    let path = Config::config_path();
                    if let Some(parent) = path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    let _ = Config::backup_if_exists(&path);
                    if let Ok(toml_str) = cfg.to_toml_string() {
                        if std::fs::write(&path, toml_str).is_ok() {
                            status_msg = format!(
                                "✓ Saved to {} (previous config backed up to .bak)!",
                                path.display()
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_release_tarball_url() {
        assert_eq!(
            build_release_tarball_url("v0.1.2", "x86_64", false),
            "https://github.com/MehmetCanWT/rustfetch/releases/download/v0.1.2/rustfetch-v0.1.2-linux-gnu-x86_64.tar.gz"
        );
        assert_eq!(
            build_release_tarball_url("0.2.0", "x86_64", true),
            "https://github.com/MehmetCanWT/rustfetch/releases/download/v0.2.0/rustfetch-v0.2.0-linux-musl-x86_64.tar.gz"
        );
        assert_eq!(
            build_release_tarball_url("0.2.0", "aarch64", false),
            "https://github.com/MehmetCanWT/rustfetch/releases/download/v0.2.0/rustfetch-v0.2.0-linux-gnu-aarch64.tar.gz"
        );
    }

    #[test]
    fn test_module_and_preset_descriptions_are_valid() {
        for &(name, desc) in MODULE_DESCRIPTIONS {
            assert!(!desc.is_empty());
            assert!(
                module_by_name(name).is_some(),
                "Module '{name}' in MODULE_DESCRIPTIONS must resolve via module_by_name"
            );
        }
        for &(preset, desc) in PRESET_DESCRIPTIONS {
            assert!(!desc.is_empty());
            assert!(
                Config::from_preset(preset).is_some(),
                "Preset '{preset}' in PRESET_DESCRIPTIONS must resolve via Config::from_preset"
            );
        }
    }

    #[test]
    fn test_install_binary_atomic_replaces_existing_file() {
        let dir =
            std::env::temp_dir().join(format!("rustfetch_atomic_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("src_bin");
        let dst = dir.join("rustfetch");
        std::fs::write(&src, b"new-binary").unwrap();
        std::fs::write(&dst, b"old-binary").unwrap();

        install_binary_atomic(&src, &dst).unwrap();
        assert_eq!(std::fs::read(&dst).unwrap(), b"new-binary");
        assert!(!dst.with_extension("new").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
