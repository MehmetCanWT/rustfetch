mod layout;
mod logos;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use rustfetch_core::config::Config;
use rustfetch_core::modules::{all_modules, module_by_name};
use rustfetch_core::{default_icon_for_module, detect_all, Module};
use serde::Serialize;
use std::io::Write;

use layout::{color_code, InfoLine, LogoBlock};

#[derive(Parser, Debug)]
#[command(
    name = "rustfetch",
    version,
    about = "Blazing-fast system information fetch tool written in Rust"
)]
struct Cli {
    #[arg(long)]
    print_default_config: bool,

    #[arg(long)]
    dump_config: bool,

    #[arg(long)]
    generate_config: bool,

    #[arg(long)]
    import_fastfetch: bool,

    #[arg(short = 'c', long, value_name = "PATH")]
    config: Option<String>,

    #[arg(short = 'l', long, value_name = "DISTRO")]
    logo: Option<String>,

    #[arg(short = 'i', long, value_name = "PATH")]
    image: Option<String>,

    #[arg(long, value_name = "COLOR")]
    color: Option<String>,

    #[arg(long)]
    center: bool,

    #[arg(long)]
    no_logo: bool,

    #[arg(
        short = 'C',
        long,
        help = "Clear terminal screen and scrollback buffer before displaying"
    )]
    clear: bool,

    #[arg(long)]
    live: bool,

    #[arg(long = "3d")]
    three_d: bool,

    #[arg(long, value_name = "FLOAT")]
    speed: Option<f32>,

    #[arg(long)]
    rotate_x: bool,

    #[arg(long)]
    rotate_y: bool,

    #[arg(long, value_name = "FLOAT")]
    size: Option<f32>,

    #[arg(long, value_name = "FLOAT")]
    depth: Option<f32>,

    #[arg(long, value_name = "MODE")]
    shading_mode: Option<String>,

    #[arg(long, value_name = "CHARS")]
    shading: Option<String>,

    #[arg(long, value_name = "COLS", alias = "image-width-cols")]
    width: Option<usize>,

    #[arg(long, value_name = "ROWS")]
    height: Option<usize>,

    #[arg(long, value_name = "N")]
    frames: Option<usize>,

    #[arg(long)]
    infinite: bool,

    #[arg(long)]
    hold: bool,

    #[arg(long)]
    exit_on_key: bool,

    #[arg(long)]
    json: bool,

    #[arg(long, value_name = "PATH")]
    ascii: Option<String>,

    #[arg(long, alias = "anim", value_name = "PATH")]
    ascii_anim: Option<String>,

    #[arg(long, value_name = "FLOAT")]
    fps: Option<f64>,

    #[arg(long, value_name = "PRESET")]
    preset: Option<String>,

    #[arg(long)]
    benchmark: bool,

    #[arg(long)]
    border: bool,

    #[arg(long, value_name = "STYLE")]
    border_style: Option<String>,

    #[arg(long, value_name = "COLOR")]
    border_color: Option<String>,

    #[arg(long, value_name = "TITLE")]
    border_title: Option<String>,

    #[arg(long)]
    icon_only: bool,

    #[arg(long, value_name = "SEP")]
    separator: Option<String>,

    #[arg(long, value_name = "THEME")]
    theme: Option<String>,

    #[arg(long, value_name = "PATH")]
    border_image: Option<String>,

    #[arg(long, value_name = "COLS")]
    border_image_width: Option<usize>,

    #[arg(long, value_name = "ROWS")]
    border_image_height: Option<usize>,

    #[arg(long, value_name = "COLS")]
    border_image_offset_x: Option<i32>,

    #[arg(long, value_name = "ROWS")]
    border_image_offset_y: Option<i32>,

    #[arg(long, value_name = "INDEX")]
    border_image_z_index: Option<i32>,

    #[arg(long, value_name = "POSITION")]
    border_image_position: Option<String>,

    #[arg(long, value_name = "PROTOCOL")]
    border_image_protocol: Option<String>,

    #[arg(long, value_name = "BOOL")]
    border_image_preserve_aspect_ratio: Option<bool>,

    #[arg(long, value_name = "BOOL")]
    border_image_text_on_top: Option<bool>,

    #[arg(long, value_name = "POSITION")]
    logo_position: Option<String>,

    #[arg(long, value_name = "PROTOCOL")]
    protocol: Option<String>,

    #[arg(long, value_enum, value_name = "SHELL")]
    completions: Option<Shell>,

    #[arg(long, value_name = "SHELL")]
    init: Option<String>,

    #[arg(long)]
    install_hook: bool,
}

fn resolve_distro_logo(distro_name: &str, padding: usize) -> logos::Logo {
    let logo = logos::get_logo(distro_name);
    let (term_w, term_h) = layout::terminal_size();
    if term_w < logo.width + layout::LOGO_GAP + 45 + padding * 2 || term_h < logo.lines.len() + 4 {
        let small_logo = logos::get_logo(&format!("{distro_name}_small"));
        if small_logo.width < logo.width || small_logo.lines.len() < logo.lines.len() {
            if term_w < small_logo.width + layout::LOGO_GAP + 30 + padding * 2
                || term_h < small_logo.lines.len() + 3
            {
                return logos::Logo {
                    lines: Vec::new(),
                    color: logo.color,
                    colors: logo.colors,
                    width: 0,
                };
            }
            return small_logo;
        }
    }
    logo
}

fn shell_init_script(shell: &str) -> Option<&'static str> {
    match shell.to_lowercase().as_str() {
        "bash" => Some(
            r#"# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
if [[ $- == *i* ]]; then
    _rustfetch_last_size=""
    _rustfetch_on_resize() {
        local cur_size
        cur_size="$(tput cols 2>/dev/null)x$(tput lines 2>/dev/null)"
        if [ -n "$cur_size" ] && [ "$cur_size" != "$_rustfetch_last_size" ]; then
            _rustfetch_last_size="$cur_size"
            if [ -z "$READLINE_LINE" ]; then
                clear
                rustfetch
            fi
        fi
    }
    trap '_rustfetch_on_resize' WINCH
    _rustfetch_last_size="$(tput cols 2>/dev/null)x$(tput lines 2>/dev/null)"
    rustfetch
fi
"#,
        ),
        "zsh" => Some(
            r#"# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
if [[ -o interactive ]]; then
    _rustfetch_last_size=""
    TRAPWINCH() {
        local cur_size="${COLUMNS}x${LINES}"
        if [[ -n "$cur_size" && "$cur_size" != "$_rustfetch_last_size" ]]; then
            _rustfetch_last_size="$cur_size"
            if [[ -z "$BUFFER" ]]; then
                clear
                rustfetch
                zle && zle reset-prompt 2>/dev/null
            fi
        fi
    }
    _rustfetch_last_size="${COLUMNS}x${LINES}"
    rustfetch
fi
"#,
        ),
        "fish" => Some(
            r#"# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
if status is-interactive
    set -g _rustfetch_last_size ""
    function _rustfetch_on_resize --on-signal WINCH
        set -l cur_size (tput cols 2>/dev/null)x(tput lines 2>/dev/null)
        if test -n "$cur_size" -a "$cur_size" != "$_rustfetch_last_size"
            set -g _rustfetch_last_size "$cur_size"
            if test -z (commandline)
                clear
                rustfetch
                commandline -f repaint 2>/dev/null
            end
        end
    end
    set -g _rustfetch_last_size (tput cols 2>/dev/null)x(tput lines 2>/dev/null)
    rustfetch
end
"#,
        ),
        _ => None,
    }
}

fn install_shell_hook() {
    let home = match std::env::var("HOME") {
        Ok(h) => std::path::PathBuf::from(h),
        Err(_) => {
            eprintln!("Error: HOME environment variable not set.");
            return;
        }
    };

    let shell_path = std::env::var("SHELL").unwrap_or_default();
    let is_zsh = shell_path.ends_with("/zsh") || shell_path == "zsh";
    let is_fish = shell_path.ends_with("/fish") || shell_path == "fish";

    let (shell_name, target_file) = if is_zsh {
        ("zsh", home.join(".zshrc"))
    } else if is_fish {
        ("fish", home.join(".config/fish/config.fish"))
    } else {
        ("bash", home.join(".bashrc"))
    };

    let script = match shell_init_script(shell_name) {
        Some(s) => s,
        None => return,
    };

    let existing = std::fs::read_to_string(&target_file).unwrap_or_default();
    if existing.contains("_rustfetch_on_resize") || existing.contains("TRAPWINCH() {") {
        println!(
            "RustFetch responsive hook is already installed in {}",
            target_file.display()
        );
        return;
    }

    let cleaned: Vec<&str> = existing
        .lines()
        .filter(|line| line.trim() != "rustfetch" && line.trim() != "rfetch")
        .collect();
    let mut new_content = cleaned.join("\n");
    if !new_content.is_empty() && !new_content.ends_with('\n') {
        new_content.push('\n');
    }
    new_content.push('\n');
    new_content.push_str(script);

    if let Some(parent) = target_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match std::fs::write(&target_file, new_content) {
        Ok(()) => println!(
            "Successfully installed RustFetch responsive hook to {}",
            target_file.display()
        ),
        Err(e) => eprintln!("Failed to write to {}: {e}", target_file.display()),
    }
}

pub fn run() {
    let cli = Cli::parse();

    if let Some(shell) = &cli.init {
        match shell_init_script(shell) {
            Some(script) => print!("{script}"),
            None => {
                eprintln!("Unknown shell: {shell}. Supported shells: bash, zsh, fish");
                std::process::exit(1);
            }
        }
        return;
    }

    if cli.install_hook {
        install_shell_hook();
        return;
    }

    if cli.print_default_config || cli.dump_config {
        print!("{}", Config::to_default_toml());
        return;
    }

    if cli.generate_config {
        let path = Config::config_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let default_toml = Config::to_default_toml();
        match std::fs::write(&path, &default_toml) {
            Ok(()) => println!("Configuration generated at {}", path.display()),
            Err(e) => eprintln!("Failed to write configuration: {e}"),
        }
        return;
    }

    if cli.import_fastfetch {
        let path = Config::config_path();
        if let Some(ff_path) = Config::find_fastfetch_config() {
            match std::fs::read_to_string(&ff_path) {
                Ok(content) => match Config::import_from_fastfetch(&content) {
                    Some(imported) => match imported.to_toml_string() {
                        Ok(toml_str) => {
                            if let Some(parent) = path.parent() {
                                let _ = std::fs::create_dir_all(parent);
                            }
                            match std::fs::write(&path, &toml_str) {
                                Ok(()) => println!(
                                    "Successfully imported Fastfetch config to {}",
                                    path.display()
                                ),
                                Err(e) => eprintln!("Failed to save imported config: {e}"),
                            }
                        }
                        Err(e) => eprintln!("Failed to serialize imported config: {e}"),
                    },
                    None => eprintln!(
                        "Could not parse Fastfetch config from {}",
                        ff_path.display()
                    ),
                },
                Err(e) => eprintln!(
                    "Could not read Fastfetch config from {}: {e}",
                    ff_path.display()
                ),
            }
        } else {
            eprintln!("No Fastfetch configuration file found to import.");
        }
        return;
    }

    if let Some(shell) = cli.completions {
        let mut cmd = Cli::command();
        generate(shell, &mut cmd, "rustfetch", &mut std::io::stdout());
        return;
    }

    let mut config = if let Some(preset_name) = &cli.preset {
        Config::from_preset(preset_name).unwrap_or_else(|| {
            eprintln!(
                "Unknown preset: {preset_name}. Available presets: default, minimal, clean, dots, card, brackets, neofetch, retro, modern, compact"
            );
            std::process::exit(1);
        })
    } else if let Some(path) = &cli.config {
        Config::load_from(std::path::Path::new(path)).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        })
    } else {
        Config::load()
    };

    if let Some(logo) = &cli.logo {
        config.general.logo.distro = logo.clone();
        config.general.logo.image_path = None;
        config.general.logo.random_image = false;
    }
    if let Some(img) = &cli.image {
        config.general.logo.image_path = Some(img.clone());
        config.general.logo.random_image = false;
    }
    if cli.center {
        config.general.center = true;
    }
    if let Some(color) = &cli.color {
        for mc in &mut config.modules {
            mc.color = Some(color.clone());
        }
    }
    if cli.border {
        config.general.border = true;
    }
    if let Some(bs) = &cli.border_style {
        config.general.border = true;
        config.general.border_style = Some(bs.clone());
    }
    if let Some(bc) = &cli.border_color {
        config.general.border_color = Some(bc.clone());
    }
    if let Some(bt) = &cli.border_title {
        config.general.border_title = Some(bt.clone());
    }
    if cli.icon_only {
        config.general.icon_only = true;
    }
    if let Some(sep) = &cli.separator {
        config.general.separator = sep.clone();
    }
    if let Some(t) = &cli.theme {
        config.general.theme = Some(t.clone());
    }
    if let Some(bi) = &cli.border_image {
        config.general.border_image = Some(bi.clone());
    }
    if let Some(bw) = cli.border_image_width {
        config.general.border_image_width = Some(bw);
    }
    if let Some(bh) = cli.border_image_height {
        config.general.border_image_height = Some(bh);
    }
    if let Some(ox) = cli.border_image_offset_x {
        config.general.border_image_offset_x = Some(ox);
    }
    if let Some(oy) = cli.border_image_offset_y {
        config.general.border_image_offset_y = Some(oy);
    }
    if let Some(zi) = cli.border_image_z_index {
        config.general.border_image_z_index = Some(zi);
    }
    if let Some(position) = &cli.border_image_position {
        config.general.border_image_position = position.clone();
    }
    if let Some(proto) = &cli.border_image_protocol {
        config.general.border_image_protocol = Some(proto.clone());
    }
    if let Some(par) = cli.border_image_preserve_aspect_ratio {
        config.general.border_image_preserve_aspect_ratio = Some(par);
    }
    if let Some(tot) = cli.border_image_text_on_top {
        config.general.border_image_text_on_top = Some(tot);
    }
    if let Some(theme_name) = &config.general.theme {
        if let Some(t) = rustfetch_render::get_theme(theme_name) {
            if config.general.border_color.is_none() {
                config.general.border_color = Some(format!(
                    "#{:02x}{:02x}{:02x}",
                    t.border.0, t.border.1, t.border.2
                ));
            }
        }
    }

    let (modules, configs): (
        Vec<Box<dyn Module>>,
        Vec<rustfetch_core::config::ModuleConfig>,
    ) = if config.modules.is_empty() {
        let all = all_modules();
        let cfgs = all
            .iter()
            .map(|m| rustfetch_core::config::ModuleConfig::new(m.name()))
            .collect();
        (all, cfgs)
    } else {
        config
            .modules
            .iter()
            .filter_map(|mc| module_by_name(&mc.name).map(|m| (m, mc.clone())))
            .unzip()
    };

    use std::io::IsTerminal;

    let is_interactive = std::io::stdout().is_terminal() && std::io::stdin().is_terminal();
    let is_dumb = std::env::var("TERM")
        .map(|t| t == "dumb" || t.is_empty())
        .unwrap_or(false);

    // CRITICAL SYSTEM SAFETY GUARD:
    // Animated 3D mode and Live monitor mode MUST NEVER run in non-interactive,
    // piped, redirected, or headless environments (e.g. SDDM/GDM session startup scripts,
    // systemd services, cron jobs, pipes, or background subshells).
    // Running interactive raw-mode animation loops in non-TTY environments hangs
    // display managers and session scripts indefinitely!
    let is_3d_requested = cli.three_d
        || config.general.logo.three_d.enabled
        || config.general.three_d.unwrap_or(false);

    let is_anim_requested = cli.ascii_anim.is_some() || config.general.logo.animation.enabled;

    let is_3d = is_3d_requested && is_interactive && !is_dumb;
    let is_anim = is_anim_requested && is_interactive && !is_dumb;
    let is_live = (cli.live || config.general.live) && is_interactive && !is_dumb;

    if cli.benchmark {
        run_benchmark(&modules);
    } else if cli.json {
        run_json(&config, &modules, &configs);
    } else if is_3d {
        run_3d(&config, &cli, &modules, &configs);
    } else if is_anim {
        run_ascii_animation(&config, &cli, &modules, &configs);
    } else if is_live {
        run_live(&config, &cli, &modules, &configs);
    } else {
        run_once(&config, &cli, &modules, &configs);
    }
}

struct LineWrapGuard {
    active: bool,
}

impl LineWrapGuard {
    fn new() -> Self {
        use std::io::IsTerminal;
        let active = std::io::stdout().is_terminal() && layout::colors_enabled();
        if active {
            print!("\x1b[?7l");
            let _ = std::io::stdout().flush();
        }
        Self { active }
    }
}

impl Drop for LineWrapGuard {
    fn drop(&mut self) {
        if self.active {
            print!("\x1b[?7h");
            let _ = std::io::stdout().flush();
        }
    }
}

fn run_once(
    config: &Config,
    cli: &Cli,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) {
    let _wrap_guard = LineWrapGuard::new();
    if (cli.clear || config.general.clear_screen.unwrap_or(false)) && layout::colors_enabled() {
        print!("\x1b[2J\x1b[3J\x1b[H");
        let _ = std::io::stdout().flush();
    }
    if !cli.no_logo && terminal_supports_kitty() {
        rustfetch_render::clear_kitty_images();
    }
    let mut resolved_image_path = None;
    let mut dynamic_color = None;
    let mut image_palette = None;

    let custom_ascii_path = cli
        .ascii
        .as_deref()
        .or(config.general.logo.ascii_path.as_deref());

    let show_logo = config.general.logo.enabled && !cli.no_logo;
    if (custom_ascii_path.is_none() || cli.image.is_some())
        && (show_logo || config.general.logo.auto_color || config.general.colors.image_palette)
    {
        if config.general.logo.random_image && cli.image.is_none() {
            let dir = config
                .general
                .logo
                .image_dir
                .as_deref()
                .unwrap_or("~/.config/rustfetch/logos");
            resolved_image_path =
                pick_random_image(dir).or_else(|| pick_random_image("~/.config/fastfetch/logos"));
        }
        if resolved_image_path.is_none() {
            resolved_image_path = config.general.logo.image_path.clone();
        }

        if let Some(img_path) = &resolved_image_path {
            let expanded_img = shellexpand::tilde(img_path).to_string();
            resolved_image_path = Some(expanded_img.clone());
            dynamic_color = get_image_color(&expanded_img, config.general.logo.auto_color);
            if config.general.colors.image_palette {
                image_palette = rustfetch_render::extract_color_palette(&expanded_img, 16);
            }
        }
    }

    if let Some(c) = &cli.color {
        dynamic_color = Some(c.clone());
    } else if let Some(t) = &cli.theme {
        if let Some(th) = rustfetch_render::get_theme(t) {
            dynamic_color = Some(format!(
                "#{:02x}{:02x}{:02x}",
                th.primary.0, th.primary.1, th.primary.2
            ));
        }
    } else if dynamic_color.is_none() {
        if let Some(t_name) = &config.general.theme {
            if let Some(t) = rustfetch_render::get_theme(t_name) {
                dynamic_color = Some(format!(
                    "#{:02x}{:02x}{:02x}",
                    t.primary.0, t.primary.1, t.primary.2
                ));
            }
        }
    }

    let palette_lines = if config.general.colors.enabled {
        let theme_palette = config.general.theme.as_deref().and_then(|t_name| {
            rustfetch_render::get_theme(t_name).map(|t| {
                vec![
                    t.background,
                    t.border,
                    t.primary,
                    t.secondary,
                    t.accent,
                    t.text,
                ]
            })
        });

        let custom_colors: Option<Vec<(u8, u8, u8)>> = config
            .general
            .colors
            .custom
            .as_ref()
            .map(|list| {
                list.iter()
                    .filter_map(|s| rustfetch_render::parse_color(s))
                    .collect()
            })
            .or(image_palette)
            .or(theme_palette);

        let mut lines = layout::build_color_palette(
            &config.general.colors.symbol,
            config.general.colors.block,
            custom_colors.as_deref(),
        );

        if config.general.colors.rows == 1 && lines.len() > 1 {
            lines.truncate(1);
        }

        Some(lines)
    } else {
        None
    };

    let (logo_block, distro_color) = if show_logo && resolved_image_path.is_none() {
        let custom_block = custom_ascii_path
            .and_then(|ascii_p| {
                let expanded = shellexpand::tilde(ascii_p);
                std::fs::read_to_string(expanded.as_ref()).ok()
            })
            .filter(|raw| !raw.is_empty())
            .map(|mut processed| {
                if let Some(colors) = &config.general.logo.colors {
                    let codes: Vec<String> = colors.iter().map(|c| layout::color_code(c)).collect();
                    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
                    processed = rustfetch_core::apply_neofetch_colors(&processed, &refs);
                }
                let lines: Vec<String> = processed.lines().map(|s| s.to_string()).collect();
                let width = lines
                    .iter()
                    .map(|l| layout::strip_ansi_width(l))
                    .max()
                    .unwrap_or(0);
                (Some(LogoBlock { lines, width }), None)
            });

        custom_block.unwrap_or_else(|| {
            let distro_name = if config.general.logo.distro == "auto" {
                detect_distro_name()
            } else {
                config.general.logo.distro.clone()
            };
            let logo = resolve_distro_logo(&distro_name, config.general.padding);
            let logo_colors = config.general.logo.colors.as_ref().map(|colors| {
                colors
                    .iter()
                    .map(|c| layout::color_code(c))
                    .collect::<Vec<String>>()
            });
            let colored_lines = if let Some(colors) = &logo_colors {
                let refs: Vec<&str> = colors.iter().map(|s| s.as_str()).collect();
                logo.colored_lines_with_custom(None, Some(&refs))
            } else if let Some(ref dc) = dynamic_color {
                let dc_code = layout::color_code(dc);
                logo.colored_lines_with_custom(Some(&dc_code), None)
            } else {
                logo.colored_lines()
            };
            (
                Some(LogoBlock {
                    lines: colored_lines,
                    width: logo.width,
                }),
                Some(logo.color),
            )
        })
    } else {
        (None, None)
    };

    let header_color = dynamic_color.as_deref().or(distro_color);

    let mut info_lines = gather_info_lines(
        config,
        modules,
        configs,
        dynamic_color.as_deref(),
        distro_color,
    );

    let header = build_header();

    let active_border_images = config.general.active_border_images();

    let border_image_lines = if active_border_images.is_empty() {
        let border_image_path = cli
            .border_image
            .as_deref()
            .or(config.general.border_image.as_deref())
            .map(|p| shellexpand::tilde(p).to_string());
        let border_image_width = cli
            .border_image_width
            .or(config.general.border_image_width)
            .unwrap_or(24);
        let border_image_height = cli
            .border_image_height
            .or(config.general.border_image_height);
        let preserve_aspect_ratio = cli
            .border_image_preserve_aspect_ratio
            .or(config.general.border_image_preserve_aspect_ratio)
            .unwrap_or(true);

        border_image_path.as_deref().and_then(|p| {
            rustfetch_render::render_halfblock_ex(
                p,
                border_image_width,
                border_image_height,
                preserve_aspect_ratio,
            )
        })
    } else {
        None
    };

    let logo_pos = cli
        .logo_position
        .as_deref()
        .or(config.general.logo.position.as_deref())
        .unwrap_or("left");

    let mut render_opts = layout::RenderOptions {
        header: header.as_deref(),
        header_color,
        separator: &config.general.separator,
        padding: config.general.padding,
        center: config.general.center,
        border: config.general.border,
        border_style: config.general.border_style.as_deref(),
        border_color: config.general.border_color.as_deref(),
        border_chars: config.general.border_chars.as_ref(),
        border_title: config.general.border_title.as_deref(),
        box_padding: config.general.box_padding,
        palette_lines: palette_lines.as_deref(),
        palette_position: &config.general.colors.position,
        icon_only: config.general.icon_only,
        key_type: config.general.key_type.as_deref(),
        border_image: border_image_lines.as_deref(),
        border_image_position: &config.general.border_image_position,
        border_images: &active_border_images,
        logo_position: logo_pos,
    };

    let (term_cols, term_rows) = layout::terminal_size();
    let is_compact_term = term_rows < 18 || term_cols < 65;
    let max_allowed_rows = term_rows.saturating_sub(2).max(6);
    if term_rows < 22 {
        render_opts.palette_lines = None;
    }
    if term_rows < 24 {
        let overhead = (if config.general.border { 2 } else { 0 })
            + (if header.is_some() { 2 } else { 0 })
            + (if render_opts.palette_lines.is_some() {
                2
            } else {
                0
            });
        let max_modules = max_allowed_rows.saturating_sub(overhead).max(3);
        info_lines.truncate(max_modules);
    }

    if is_compact_term && cli.image.is_none() && cli.ascii.is_none() {
        layout::render(None, &info_lines, &render_opts);
        return;
    }

    if show_logo {
        if let Some(image_path) = &resolved_image_path {
            let config_img_w = cli.width.unwrap_or(config.general.logo.image_width_cols);
            let mut target_cols = config_img_w;

            let right_preview = layout::format_info_lines(
                &info_lines,
                &config.general.separator,
                config.general.icon_only,
                config.general.key_type.as_deref(),
            );
            let max_right_w = right_preview
                .iter()
                .map(|l| layout::strip_ansi_width(l))
                .max()
                .unwrap_or(0);
            let header_w = header
                .as_ref()
                .map(|h| unicode_width::UnicodeWidthStr::width(h.as_str()))
                .unwrap_or(0);
            let needed_right_w =
                max_right_w.max(header_w) + if config.general.border { 4 } else { 0 };

            let pad_val = config.general.padding;
            let mut effective_logo_pos = logo_pos;

            // Responsive layout adaptation for tiling window managers (Hyprland, MangoWM, etc.)
            let min_card_w = 32;
            let min_image_w = 16;

            if effective_logo_pos == "left" || effective_logo_pos == "right" {
                let avail_for_both = term_cols.saturating_sub(layout::LOGO_GAP + pad_val * 2 + 1);
                if avail_for_both < min_image_w + min_card_w {
                    if term_cols >= 32 {
                        effective_logo_pos = "top";
                        target_cols = config_img_w
                            .min(term_cols.saturating_sub(pad_val * 2 + 2))
                            .max(min_image_w);
                    }
                } else {
                    let total_needed = target_cols + needed_right_w;
                    if total_needed > avail_for_both {
                        let card_allocated = needed_right_w
                            .clamp(min_card_w, avail_for_both.saturating_sub(min_image_w));
                        let max_img_avail = avail_for_both.saturating_sub(card_allocated);
                        target_cols = config_img_w.min(max_img_avail).max(min_image_w);
                    }
                }
            } else if effective_logo_pos == "top" || effective_logo_pos == "bottom" {
                target_cols = config_img_w
                    .min(term_cols.saturating_sub(pad_val * 2 + 2))
                    .max(min_image_w);
            }

            render_opts.logo_position = effective_logo_pos;

            let max_image_rows = Some(
                if effective_logo_pos == "top" || effective_logo_pos == "bottom" {
                    (term_rows.saturating_sub(info_lines.len() + 4) / 2).clamp(4, 16)
                } else {
                    max_allowed_rows.min(28)
                },
            );

            if cli.live {
                if let Ok((cols, _)) = crossterm::terminal::size() {
                    let max_cols = (cols as usize) / 2;
                    if target_cols > max_cols && max_cols > 5 {
                        target_cols = max_cols;
                    }
                }
            }

            let protocol = cli
                .protocol
                .as_deref()
                .unwrap_or(&config.general.logo.protocol)
                .to_lowercase();
            let use_kitty =
                protocol == "kitty" || (protocol == "auto" && terminal_supports_kitty());
            let use_sixel = protocol == "sixel"
                || (protocol == "auto" && !use_kitty && rustfetch_render::supports_sixel());

            if effective_logo_pos == "behind" && terminal_supports_kitty() {
                let card_w = max_right_w
                    .max(header_w)
                    .max(target_cols)
                    .min(term_cols.saturating_sub(pad_val * 2 + 1));
                let card_h = (info_lines.len() + 4).min(max_allowed_rows);
                let _ = rustfetch_render::render_background(image_path, card_w, card_h, -1);
                layout::render(None, &info_lines, &render_opts);
                return;
            }

            let bounded_card_w = if effective_logo_pos == "top" || effective_logo_pos == "bottom" {
                needed_right_w.min(term_cols.saturating_sub(pad_val * 2 + 1))
            } else {
                needed_right_w
                    .min(term_cols.saturating_sub(target_cols + layout::LOGO_GAP + pad_val * 2 + 1))
            };

            if use_kitty {
                let total_fetch_w = if effective_logo_pos == "top" || effective_logo_pos == "bottom"
                {
                    target_cols.max(bounded_card_w)
                } else {
                    target_cols + layout::LOGO_GAP + bounded_card_w
                };
                let h_pad = layout::compute_h_pad(
                    total_fetch_w,
                    config.general.padding,
                    config.general.center,
                );

                if h_pad > 0 {
                    print!("{}", " ".repeat(h_pad));
                    let _ = std::io::stdout().flush();
                }

                if let Some((actual_cols, actual_lines)) =
                    rustfetch_render::render_image_ex(image_path, target_cols, max_image_rows)
                {
                    layout::render_with_image(
                        actual_lines,
                        actual_cols,
                        h_pad,
                        &info_lines,
                        &render_opts,
                    );
                    return;
                }
            } else if use_sixel {
                let total_fetch_w = if effective_logo_pos == "top" || effective_logo_pos == "bottom"
                {
                    target_cols.max(bounded_card_w)
                } else {
                    target_cols + layout::LOGO_GAP + bounded_card_w
                };
                let h_pad = layout::compute_h_pad(
                    total_fetch_w,
                    config.general.padding,
                    config.general.center,
                );

                if h_pad > 0 {
                    print!("{}", " ".repeat(h_pad));
                    let _ = std::io::stdout().flush();
                }

                if let Some((actual_cols, actual_lines)) =
                    rustfetch_render::render_sixel_ex(image_path, target_cols, max_image_rows)
                {
                    layout::render_with_image(
                        actual_lines,
                        actual_cols,
                        h_pad,
                        &info_lines,
                        &render_opts,
                    );
                    return;
                }
            }

            if let Some(lines) =
                rustfetch_render::render_halfblock_ex(image_path, target_cols, max_image_rows, true)
            {
                let w = lines
                    .first()
                    .map(|l| layout::strip_ansi_width(l))
                    .unwrap_or(target_cols);
                let logo_block = LogoBlock { lines, width: w };
                layout::render(Some(&logo_block), &info_lines, &render_opts);
                return;
            }
        }
    }

    layout::render(logo_block.as_ref(), &info_lines, &render_opts);
}

#[derive(Serialize)]
struct JsonModuleOutput {
    name: String,
    label: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
}

fn run_json(
    config: &Config,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) {
    let results = detect_all(modules);
    let mut outputs = Vec::new();

    for (result, mc) in results.into_iter().zip(configs.iter()) {
        if mc.name == "break" || mc.name == "divider" {
            continue;
        }
        let (label, mut value) = if mc.name == "custom" {
            (
                mc.label.clone().unwrap_or_else(|| "custom".to_string()),
                mc.value.clone().unwrap_or_default(),
            )
        } else if let Some(info) = result {
            (mc.label.clone().unwrap_or(info.label), info.value)
        } else {
            continue;
        };

        if let Some(fmt) = &mc.format {
            value = fmt.replace("{value}", &value).replace("{}", &value);
        }

        let icon = if config.general.icons {
            if let Some(user_icon) = &mc.icon {
                Some(user_icon.clone())
            } else if mc.name == "os" {
                Some(rustfetch_core::distro_icon(&value).to_string())
            } else {
                default_icon_for_module(&mc.name).map(str::to_string)
            }
        } else {
            None
        };

        outputs.push(JsonModuleOutput {
            name: mc.name.clone(),
            label,
            value,
            icon,
        });
    }

    if let Ok(json_str) = serde_json::to_string_pretty(&outputs) {
        println!("{json_str}");
    }
}

fn extract_percentage(val: &str) -> Option<u8> {
    if let Some(pos) = val.find('%') {
        let prefix = &val[..pos];
        let num_str: String = prefix
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let num_str: String = num_str.chars().rev().collect();
        if let Ok(p) = num_str.parse::<u8>() {
            return Some(p.min(100));
        }
    }

    if let Some((left, right)) = val.split_once('/') {
        let parse_num = |s: &str| -> Option<f64> {
            let mut words = s.split_whitespace();
            let n = words.next()?.parse::<f64>().ok()?;
            let unit = words.next().map(|u| u.to_lowercase()).unwrap_or_default();
            let mult = if unit.starts_with("gib") {
                1024.0 * 1024.0
            } else if unit.starts_with("mib") {
                1024.0
            } else if unit.starts_with("tib") {
                1024.0 * 1024.0 * 1024.0
            } else {
                1.0
            };
            Some(n * mult)
        };
        if let (Some(used), Some(total)) = (parse_num(left.trim()), parse_num(right.trim())) {
            if total > 0.0 {
                let pct = ((used / total) * 100.0).round();
                return Some((pct as u8).min(100));
            }
        }
    }

    None
}

fn format_bar(percent: u8, width: usize, colored: bool, invert_color: bool) -> String {
    let width = if width == 0 { 10 } else { width };
    let percent = percent.min(100);
    let elapsed = ((percent as usize * width) + 50) / 100;
    let remaining = width.saturating_sub(elapsed);

    let color_code = if !colored {
        ""
    } else if !invert_color {
        if percent < 60 {
            "\x1b[32m"
        } else if percent < 85 {
            "\x1b[33m"
        } else {
            "\x1b[31m"
        }
    } else if percent < 20 {
        "\x1b[31m"
    } else if percent < 50 {
        "\x1b[33m"
    } else {
        "\x1b[32m"
    };

    let reset = if colored { "\x1b[0m" } else { "" };
    format!(
        "{}[{}{}]",
        color_code,
        "■".repeat(elapsed),
        "-".repeat(remaining)
    ) + reset
}

fn gather_info_lines(
    config: &Config,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
    dynamic_color: Option<&str>,
    distro_color: Option<&str>,
) -> Vec<InfoLine> {
    let results = detect_all(modules);
    results
        .into_iter()
        .zip(configs.iter())
        .filter_map(|(result, mc)| {
            let is_divider = mc.name == "break" || mc.name == "divider";
            if is_divider {
                let label = if let Some(lbl) = &mc.label {
                    if !lbl.is_empty() {
                        format!("---DIVIDER:{lbl}---")
                    } else {
                        "---DIVIDER---".to_string()
                    }
                } else {
                    "---DIVIDER---".to_string()
                };
                return Some(InfoLine {
                    label,
                    value: String::new(),
                    icon: None,
                    color: String::new(),
                });
            }

            let info = result?;
            let label = if mc.name == "custom" {
                mc.label.clone().unwrap_or_default()
            } else {
                mc.label.clone().unwrap_or(info.label)
            };

            let mut value = if mc.name == "custom" {
                mc.value.clone().unwrap_or_default()
            } else {
                info.value
            };

            if let Some(fmt) = &mc.format {
                value = fmt.replace("{value}", &value).replace("{}", &value);
            }

            let icon = if config.general.icons {
                if let Some(user_icon) = &mc.icon {
                    Some(user_icon.clone())
                } else if mc.name == "os" {
                    Some(rustfetch_core::distro_icon(&value).to_string())
                } else {
                    default_icon_for_module(&mc.name).map(str::to_string)
                }
            } else {
                None
            };

            if mc.bar.unwrap_or(false) && !value.is_empty() {
                if let Some(pct) = extract_percentage(&value) {
                    let width = mc.bar_width.unwrap_or(10);
                    let invert = mc.name == "battery";
                    let bar = format_bar(pct, width, true, invert);
                    if value.contains('%') {
                        if let Some(pct_idx) = value.rfind('(') {
                            value = format!("{}{} {}", &value[..pct_idx], bar, &value[pct_idx..]);
                        } else {
                            value = format!("{} {}", value, bar);
                        }
                    } else {
                        value = format!("{} {} ({}%)", value, bar, pct);
                    }
                }
            }

            let theme_color = config
                .general
                .theme
                .as_deref()
                .and_then(rustfetch_render::get_theme)
                .map(|t| format!("#{:02x}{:02x}{:02x}", t.primary.0, t.primary.1, t.primary.2));
            let color_str = mc
                .color
                .as_deref()
                .or(dynamic_color)
                .or(theme_color.as_deref())
                .or(distro_color)
                .unwrap_or("blue");
            let color = color_code(color_str);
            Some(InfoLine {
                label,
                value,
                icon,
                color,
            })
        })
        .collect()
}

fn build_right_lines(info_lines: &[InfoLine], opts: &layout::RenderOptions) -> Vec<String> {
    layout::build_right_lines(info_lines, opts)
}

#[cfg(unix)]
static mut ORIG_TERMIOS: Option<libc::termios> = None;

#[cfg(unix)]
extern "C" fn handle_sigint(_: libc::c_int) {
    unsafe {
        if let Some(ref orig) = ORIG_TERMIOS {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, orig);
        }
        libc::write(
            libc::STDOUT_FILENO,
            b"\x1b[?1002l\x1b[?1006l\x1b[?25h\n".as_ptr() as *const _,
            23,
        );
        libc::_exit(0);
    }
}

struct TermGuard {
    #[cfg(unix)]
    orig_termios: Option<libc::termios>,
}

impl TermGuard {
    fn enter() -> Self {
        use crossterm::{
            cursor::{Hide, MoveTo},
            execute,
            terminal::{Clear, ClearType},
        };
        #[cfg(unix)]
        let mut orig_termios = None;
        #[cfg(unix)]
        unsafe {
            let mut orig: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(libc::STDIN_FILENO, &mut orig) == 0 {
                orig_termios = Some(orig);
                ORIG_TERMIOS = Some(orig);
                let mut raw = orig;
                // Retain ISIG so Ctrl+C sends SIGINT natively without buffering
                raw.c_lflag &= !(libc::ICANON | libc::ECHO);
                raw.c_cc[libc::VMIN] = 0;
                raw.c_cc[libc::VTIME] = 0;
                libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &raw);
            }
            libc::signal(
                libc::SIGINT,
                handle_sigint as *const () as libc::sighandler_t,
            );
            libc::signal(
                libc::SIGTERM,
                handle_sigint as *const () as libc::sighandler_t,
            );
        }
        #[cfg(not(unix))]
        let _ = crossterm::terminal::enable_raw_mode();

        let mut stdout = std::io::stdout();
        let _ = execute!(stdout, Hide, Clear(ClearType::All), MoveTo(0, 0));
        let _ = stdout.flush();

        Self {
            #[cfg(unix)]
            orig_termios,
        }
    }
}

impl Drop for TermGuard {
    fn drop(&mut self) {
        #[cfg(unix)]
        unsafe {
            if let Some(ref orig) = self.orig_termios {
                libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, orig);
            }
            libc::signal(libc::SIGINT, libc::SIG_DFL);
            libc::signal(libc::SIGTERM, libc::SIG_DFL);
        }
        #[cfg(not(unix))]
        {
            let _ = crossterm::terminal::disable_raw_mode();
        }
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        let _ = std::io::stdout().flush();
    }
}

fn compose_animated_frame(
    config: &Config,
    left_lines: &[String],
    (left_width, left_rows): (usize, usize),
    pad_short_left_lines: bool,
    (right_lines, right_width): (&[String], usize),
    (term_cols, term_rows): (usize, usize),
) -> String {
    let layout_stacked = term_cols < left_width + layout::LOGO_GAP + right_width;
    let mut frame_buf = String::with_capacity(term_cols * term_rows * 4 + 64);
    frame_buf.push_str("\x1b[H");

    if layout_stacked {
        let left_pad = if config.general.center {
            " ".repeat(term_cols.saturating_sub(left_width) / 2)
        } else {
            " ".repeat(config.general.padding)
        };
        for line in left_lines {
            frame_buf.push_str(&left_pad);
            frame_buf.push_str(line);
            frame_buf.push_str("\x1b[K\r\n");
        }
        frame_buf.push_str("\x1b[K\r\n");
        let info_pad = if config.general.center {
            " ".repeat(term_cols.saturating_sub(right_width) / 2)
        } else {
            " ".repeat(config.general.padding)
        };
        for line in right_lines {
            frame_buf.push_str(&info_pad);
            frame_buf.push_str(line);
            frame_buf.push_str("\x1b[K\r\n");
        }
    } else {
        let total_content_w = left_width + layout::LOGO_GAP + right_width;
        let left_pad = if config.general.center {
            " ".repeat(term_cols.saturating_sub(total_content_w) / 2)
        } else {
            " ".repeat(config.general.padding)
        };
        let total_rows = left_rows.max(right_lines.len());
        let gap = " ".repeat(layout::LOGO_GAP);

        for r in 0..total_rows {
            frame_buf.push_str(&left_pad);
            if let Some(line) = left_lines.get(r) {
                frame_buf.push_str(line);
                if pad_short_left_lines {
                    let line_w = layout::strip_ansi_width(line);
                    if line_w < left_width {
                        frame_buf.push_str(&" ".repeat(left_width - line_w));
                    }
                }
            } else {
                frame_buf.push_str(&" ".repeat(left_width));
            }
            frame_buf.push_str(&gap);
            if let Some(right_line) = right_lines.get(r) {
                frame_buf.push_str(right_line);
            }
            frame_buf.push_str("\x1b[K\r\n");
        }
    }
    frame_buf
}

fn run_3d(
    config: &Config,
    cli: &Cli,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) {
    use std::time::Duration;

    let distro_name = if config.general.logo.distro == "auto" {
        detect_distro_name()
    } else {
        config.general.logo.distro.clone()
    };
    let logo = logos::get_logo(&distro_name);

    let shading_mode_str = cli
        .shading_mode
        .as_deref()
        .unwrap_or(&config.general.logo.three_d.shading_mode);
    let shading_mode = rustfetch_render::three_d::ShadingMode::from_str_loose(shading_mode_str);

    let speed = cli.speed.unwrap_or(config.general.logo.three_d.speed);
    let size_scale = cli.size.unwrap_or(config.general.logo.three_d.size);
    let depth_scale = cli.depth.unwrap_or(config.general.logo.three_d.depth);

    let (rotate_x, rotate_y) = if cli.rotate_x && !cli.rotate_y {
        (true, false)
    } else if cli.rotate_y && !cli.rotate_x {
        (false, true)
    } else {
        (
            config.general.logo.three_d.rotate_x,
            config.general.logo.three_d.rotate_y,
        )
    };

    let shading_chars = cli
        .shading
        .as_deref()
        .or(config.general.logo.three_d.shading.as_deref());

    let custom_ascii_path = cli
        .ascii
        .as_deref()
        .or(config.general.logo.ascii_path.as_deref());

    let (model, distro_outer, distro_inner) = custom_ascii_path
        .and_then(|ascii_p| {
            let expanded = shellexpand::tilde(ascii_p);
            rustfetch_core::ascii::load_ascii_lines(std::path::Path::new(expanded.as_ref())).ok()
        })
        .filter(|lines| !lines.is_empty())
        .map(|custom_lines| {
            let m = rustfetch_render::three_d::Model3D::from_ascii_lines(
                &custom_lines,
                size_scale,
                depth_scale,
                shading_mode,
            );
            (m, "\x1b[38;2;81;190;240m", "\x1b[38;2;255;255;255m")
        })
        .unwrap_or_else(|| {
            let colored_logo_lines = logo.colored_lines();
            let m = rustfetch_render::three_d::Model3D::from_ascii_lines(
                &colored_logo_lines,
                size_scale,
                depth_scale,
                shading_mode,
            );
            let (d_outer, d_inner) = rustfetch_render::three_d::distro_3d_colors(&distro_name);
            (m, d_outer, d_inner)
        });

    let mut renderer = rustfetch_render::three_d::Renderer3D::new(shading_mode, shading_chars);
    renderer.size_scale = size_scale;
    renderer.speed = speed;
    renderer.outer_color = config
        .general
        .logo
        .three_d
        .outer_color
        .as_deref()
        .map(rustfetch_render::three_d::parse_color_to_ansi)
        .unwrap_or_else(|| distro_outer.to_string());
    renderer.inner_color = config
        .general
        .logo
        .three_d
        .inner_color
        .as_deref()
        .map(rustfetch_render::three_d::parse_color_to_ansi)
        .unwrap_or_else(|| distro_inner.to_string());

    let palette_lines = if config.general.colors.enabled {
        Some(layout::build_color_palette(
            &config.general.colors.symbol,
            config.general.colors.block,
            None,
        ))
    } else {
        None
    };

    let header = build_header();
    let header_color = cli
        .color
        .as_deref()
        .or(config.general.logo.three_d.outer_color.as_deref())
        .or(Some(distro_outer));

    let render_opts = layout::RenderOptions {
        header: header.as_deref(),
        header_color,
        separator: &config.general.separator,
        padding: config.general.padding,
        center: config.general.center,
        border: config.general.border,
        border_style: config.general.border_style.as_deref(),
        border_color: config.general.border_color.as_deref(),
        border_chars: config.general.border_chars.as_ref(),
        border_title: config.general.border_title.as_deref(),
        box_padding: config.general.box_padding,
        palette_lines: palette_lines.as_deref(),
        palette_position: &config.general.colors.position,
        icon_only: config.general.icon_only,
        key_type: config.general.key_type.as_deref(),
        border_image: None,
        border_image_position: &config.general.border_image_position,
        border_images: &[],
        logo_position: "left",
    };

    let mut info_lines = gather_info_lines(config, modules, configs, None, Some(distro_outer));
    let mut right_lines = build_right_lines(&info_lines, &render_opts);

    let _guard = TermGuard::enter();
    let mut stdout = std::io::stdout();

    let mut angle_x = 0.0f32;
    let mut angle_y = 0.0f32;
    let mut frame_idx = 0usize;

    let (telemetry_tx, telemetry_rx) = std::sync::mpsc::channel();
    let bg_config = config.clone();
    let bg_configs = configs.to_vec();
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let running_bg = running.clone();

    let _bg_handle = std::thread::spawn(move || {
        let (bg_modules, bg_configs): (Vec<Box<dyn Module>>, Vec<_>) = bg_configs
            .into_iter()
            .filter_map(|mc| module_by_name(&mc.name).map(|m| (m, mc)))
            .unzip();
        while running_bg.load(std::sync::atomic::Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(1000));
            if !running_bg.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            let new_info = gather_info_lines(
                &bg_config,
                &bg_modules,
                &bg_configs,
                None,
                Some(distro_outer),
            );
            if telemetry_tx.send(new_info).is_err() {
                break;
            }
        }
    });

    let mut speed_multiplier = 1.0f32;
    let mut paused = false;
    let target_fps = cli.fps.unwrap_or(30.0).clamp(1.0, 120.0);
    let poll_timeout = ((1000.0 / target_fps) as i32).max(1);

    let max_frames = if cli.infinite || config.general.logo.three_d.infinite.unwrap_or(false) {
        None
    } else {
        cli.frames.or(config.general.logo.three_d.frames)
    };

    let exit_on_key = !cli.hold
        && !config.general.logo.three_d.hold.unwrap_or(false)
        && (cli.exit_on_key || config.general.logo.three_d.exit_on_key.unwrap_or(true));

    loop {
        if let Some(limit) = max_frames {
            if frame_idx >= limit {
                break;
            }
        }

        // Receive non-blocking background telemetry updates
        while let Ok(new_info) = telemetry_rx.try_recv() {
            info_lines = new_info;
            right_lines = build_right_lines(&info_lines, &render_opts);
        }

        let (term_cols, term_rows) = layout::terminal_size();

        // Compute layout
        let right_width = right_lines
            .iter()
            .map(|l| layout::strip_ansi_width(l))
            .max()
            .unwrap_or(30);

        let target_width = cli
            .width
            .or(config.general.logo.three_d.width)
            .unwrap_or(config.general.logo.image_width_cols.min(42))
            .max(25);

        let needed_cols = target_width + layout::LOGO_GAP + right_width;
        let layout_stacked = term_cols < needed_cols;

        let canvas_cols = if layout_stacked {
            target_width.min(term_cols.max(20))
        } else {
            target_width
        };

        let canvas_rows = if layout_stacked {
            let max_stacked_h = (canvas_cols * 3 / 5).clamp(14, 24);
            cli.height
                .or(config.general.logo.three_d.height)
                .unwrap_or(max_stacked_h)
        } else {
            let ideal_h = (canvas_cols * 3 / 5).clamp(18, 24);
            cli.height
                .or(config.general.logo.three_d.height)
                .unwrap_or(ideal_h.max(right_lines.len().min(24)))
        };

        // Render 3D Logo
        let logo_3d = renderer.render_frame(&model, angle_x, angle_y, canvas_cols, canvas_rows);

        // Update rotation
        if !paused {
            if rotate_x {
                angle_x += 0.04 * speed * speed_multiplier;
            }
            if rotate_y {
                angle_y += 0.06 * speed * speed_multiplier;
            }
        }

        let frame_buf = compose_animated_frame(
            config,
            &logo_3d,
            (canvas_cols, canvas_rows),
            false,
            (&right_lines, right_width),
            (term_cols, term_rows),
        );

        let _ = stdout.write_all(frame_buf.as_bytes());
        let _ = stdout.flush();

        frame_idx += 1;

        // Poll input: exit on keypress or if hold mode is active, handle controls
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = std::io::stdin().as_raw_fd();
            let mut pfd = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };
            let ret = unsafe { libc::poll(&mut pfd, 1, poll_timeout) };
            if ret > 0 && (pfd.revents & libc::POLLIN) != 0 {
                let mut avail: libc::c_int = 0;
                unsafe { libc::ioctl(fd, libc::FIONREAD, &mut avail) };
                if avail > 0 {
                    if exit_on_key {
                        // User started typing: leave all bytes in stdin for the shell and exit immediately!
                        running.store(false, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }

                    let mut ibuf = [0u8; 128];
                    let to_read = (avail as usize).min(ibuf.len());
                    let n =
                        unsafe { libc::read(fd, ibuf.as_mut_ptr() as *mut libc::c_void, to_read) };
                    if n > 0 {
                        let bytes = &ibuf[..n as usize];
                        let mut should_exit = false;
                        for &b in bytes {
                            match b {
                                b'q' | b'Q' | 3 => {
                                    should_exit = true;
                                    break;
                                }
                                b'w' | b'W' => angle_x += 0.2,
                                b's' | b'S' => angle_x -= 0.2,
                                b'a' | b'A' => angle_y -= 0.2,
                                b'd' | b'D' => angle_y += 0.2,
                                b'+' | b'=' => {
                                    speed_multiplier = (speed_multiplier * 1.25).min(5.0)
                                }
                                b'-' | b'_' => speed_multiplier = (speed_multiplier * 0.8).max(0.2),
                                b' ' => paused = !paused,
                                _ => {}
                            }
                        }
                        if should_exit {
                            running.store(false, std::sync::atomic::Ordering::Relaxed);
                            break;
                        }
                    }
                }
            }
        }
        #[cfg(not(unix))]
        {
            if let Ok(ready) = crossterm::event::poll(Duration::from_millis(poll_timeout as u64)) {
                if ready {
                    if exit_on_key {
                        let _ = crossterm::event::read();
                        running.store(false, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }
                    if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                        match key.code {
                            crossterm::event::KeyCode::Char('q')
                            | crossterm::event::KeyCode::Char('Q') => {
                                running.store(false, std::sync::atomic::Ordering::Relaxed);
                                break;
                            }
                            crossterm::event::KeyCode::Char('c')
                                if key
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
                            {
                                running.store(false, std::sync::atomic::Ordering::Relaxed);
                                break;
                            }
                            crossterm::event::KeyCode::Char('w')
                            | crossterm::event::KeyCode::Char('W') => angle_x += 0.2,
                            crossterm::event::KeyCode::Char('s')
                            | crossterm::event::KeyCode::Char('S') => angle_x -= 0.2,
                            crossterm::event::KeyCode::Char('a')
                            | crossterm::event::KeyCode::Char('A') => angle_y -= 0.2,
                            crossterm::event::KeyCode::Char('d')
                            | crossterm::event::KeyCode::Char('D') => angle_y += 0.2,
                            crossterm::event::KeyCode::Char('+')
                            | crossterm::event::KeyCode::Char('=') => {
                                speed_multiplier = (speed_multiplier * 1.25).min(5.0)
                            }
                            crossterm::event::KeyCode::Char('-')
                            | crossterm::event::KeyCode::Char('_') => {
                                speed_multiplier = (speed_multiplier * 0.8).max(0.2)
                            }
                            crossterm::event::KeyCode::Char(' ') => paused = !paused,
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    running.store(false, std::sync::atomic::Ordering::Relaxed);
}

fn run_ascii_animation(
    config: &Config,
    cli: &Cli,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) {
    use std::time::{Duration, Instant};

    let anim_path = cli
        .ascii_anim
        .as_deref()
        .or(config.general.logo.animation.path.as_deref())
        .or(cli.ascii.as_deref())
        .or(config.general.logo.ascii_path.as_deref());

    let path_str = match anim_path {
        Some(p) => p,
        None => {
            eprintln!("Error: No ASCII animation path specified. Use --ascii-anim <PATH> or set general.logo.animation.path in config.toml");
            return;
        }
    };

    let expanded = shellexpand::tilde(path_str);
    let p = std::path::Path::new(expanded.as_ref());
    let frames = match rustfetch_core::ascii::load_ascii_frames(p) {
        Ok(f) if !f.is_empty() => f,
        Ok(_) => {
            eprintln!("Error: No ASCII frames found in {}", p.display());
            return;
        }
        Err(e) => {
            eprintln!("Error reading ASCII frames from {}: {e}", p.display());
            return;
        }
    };

    let fps = cli
        .fps
        .unwrap_or(config.general.logo.animation.fps)
        .clamp(1.0, 60.0);
    let frame_interval = Duration::from_secs_f64(1.0 / fps);

    let palette_lines = if config.general.colors.enabled {
        Some(layout::build_color_palette(
            &config.general.colors.symbol,
            config.general.colors.block,
            None,
        ))
    } else {
        None
    };

    let distro_name = if config.general.logo.distro == "auto" {
        detect_distro_name()
    } else {
        config.general.logo.distro.clone()
    };
    let anim_distro_color = logos::get_logo(&distro_name).color;

    let header = build_header();
    let header_color = cli.color.as_deref().or(Some(anim_distro_color));

    let render_opts = layout::RenderOptions {
        header: header.as_deref(),
        header_color,
        separator: &config.general.separator,
        padding: config.general.padding,
        center: config.general.center,
        border: config.general.border,
        border_style: config.general.border_style.as_deref(),
        border_color: config.general.border_color.as_deref(),
        border_chars: config.general.border_chars.as_ref(),
        border_title: config.general.border_title.as_deref(),
        box_padding: config.general.box_padding,
        palette_lines: palette_lines.as_deref(),
        palette_position: &config.general.colors.position,
        icon_only: config.general.icon_only,
        key_type: config.general.key_type.as_deref(),
        border_image: None,
        border_image_position: &config.general.border_image_position,
        border_images: &[],
        logo_position: "left",
    };

    let mut info_lines = gather_info_lines(config, modules, configs, None, Some(anim_distro_color));
    let mut right_lines = build_right_lines(&info_lines, &render_opts);

    let _guard = TermGuard::enter();
    let mut stdout = std::io::stdout();

    let mut frame_idx = 0usize;
    let mut last_metric_refresh = Instant::now();

    let max_frames = if cli.infinite || config.general.logo.animation.infinite {
        None
    } else {
        cli.frames.or(config.general.logo.animation.frames)
    };

    let exit_on_key = !cli.hold
        && !config.general.logo.animation.hold.unwrap_or(false)
        && (cli.exit_on_key || config.general.logo.animation.exit_on_key.unwrap_or(true));

    loop {
        if let Some(limit) = max_frames {
            if frame_idx >= limit {
                break;
            }
        }

        if last_metric_refresh.elapsed() >= Duration::from_millis(1000) {
            info_lines = gather_info_lines(config, modules, configs, None, Some(anim_distro_color));
            right_lines = build_right_lines(&info_lines, &render_opts);
            last_metric_refresh = Instant::now();
        }

        let (term_cols, term_rows) = layout::terminal_size();

        let current_frame = &frames[frame_idx % frames.len()];
        let frame_width = current_frame
            .iter()
            .map(|l| layout::strip_ansi_width(l))
            .max()
            .unwrap_or(0);

        let right_width = right_lines
            .iter()
            .map(|l| layout::strip_ansi_width(l))
            .max()
            .unwrap_or(30);

        let frame_buf = compose_animated_frame(
            config,
            current_frame,
            (frame_width, current_frame.len()),
            true,
            (&right_lines, right_width),
            (term_cols, term_rows),
        );

        let _ = stdout.write_all(frame_buf.as_bytes());
        let _ = stdout.flush();

        frame_idx += 1;

        let wait_ms = (frame_interval.as_millis().max(1) as i32).min(1000);

        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = std::io::stdin().as_raw_fd();
            let mut pfd = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };
            let ret = unsafe { libc::poll(&mut pfd, 1, wait_ms) };
            if ret > 0 && (pfd.revents & libc::POLLIN) != 0 {
                let mut avail: libc::c_int = 0;
                unsafe { libc::ioctl(fd, libc::FIONREAD, &mut avail) };
                if avail > 0 {
                    if exit_on_key {
                        break;
                    }
                    let mut ibuf = [0u8; 128];
                    let to_read = (avail as usize).min(ibuf.len());
                    let n =
                        unsafe { libc::read(fd, ibuf.as_mut_ptr() as *mut libc::c_void, to_read) };
                    if n > 0 {
                        let bytes = &ibuf[..n as usize];
                        if bytes.iter().any(|&b| b == b'q' || b == b'Q' || b == 3) {
                            break;
                        }
                    }
                }
            }
        }

        #[cfg(not(unix))]
        {
            if let Ok(ready) = crossterm::event::poll(Duration::from_millis(wait_ms as u64)) {
                if ready {
                    if exit_on_key {
                        let _ = crossterm::event::read();
                        break;
                    }
                    if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                        if (key.code == crossterm::event::KeyCode::Char('c')
                            && key
                                .modifiers
                                .contains(crossterm::event::KeyModifiers::CONTROL))
                            || key.code == crossterm::event::KeyCode::Char('q')
                            || key.code == crossterm::event::KeyCode::Char('Q')
                        {
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn run_live(
    config: &Config,
    cli: &Cli,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) {
    use crossterm::{
        cursor::{Hide, MoveTo, Show},
        event::{poll, read, Event, KeyCode},
        execute, queue,
        terminal::{
            disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    };
    use std::time::Duration;

    struct LiveGuard;
    impl Drop for LiveGuard {
        fn drop(&mut self) {
            let _ = disable_raw_mode();
            let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
        }
    }

    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen, Hide);
    let _ = enable_raw_mode();
    let _guard = LiveGuard;

    loop {
        let _ = queue!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        run_once(config, cli, modules, configs);
        let _ = stdout.flush();

        if let Ok(ready) = poll(Duration::from_millis(1000)) {
            if ready {
                if let Ok(event) = read() {
                    match event {
                        Event::Key(key)
                            if key.code == KeyCode::Char('q')
                                || key.code == KeyCode::Esc
                                || (key.code == KeyCode::Char('c')
                                    && key
                                        .modifiers
                                        .contains(crossterm::event::KeyModifiers::CONTROL)) =>
                        {
                            break;
                        }
                        Event::Resize(_, _) => {}
                        _ => {}
                    }
                }
            }
        }
    }
}

fn build_header() -> Option<String> {
    let user = std::env::var("USER").ok()?;
    let hostname = std::fs::read_to_string("/etc/hostname").ok()?;
    Some(format!("{user}@{}", hostname.trim()))
}

fn detect_distro_name() -> String {
    std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find_map(|line| line.strip_prefix("ID="))
                .map(|val| val.trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "linux".to_string())
}

fn terminal_supports_kitty() -> bool {
    rustfetch_render::supports_kitty()
}

fn pick_random_image(dir_path: &str) -> Option<String> {
    use std::fs;
    let expanded = shellexpand::tilde(dir_path);
    let files: Vec<String> = fs::read_dir(expanded.as_ref())
        .ok()?
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension()?.to_str()?.to_lowercase();
                if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif" | "webp") {
                    return path.to_str().map(String::from);
                }
            }
            None
        })
        .collect();
    if files.is_empty() {
        return None;
    }

    let base_dir = std::env::var("XDG_CACHE_HOME")
        .unwrap_or_else(|_| format!("{}/.cache", std::env::var("HOME").unwrap_or_default()));
    let cache_dir = std::path::PathBuf::from(base_dir).join("rustfetch");
    let last_logo_file = cache_dir.join("last_logo");
    let last_picked = fs::read_to_string(&last_logo_file).ok();

    let mut candidates: Vec<&String> = if files.len() > 1 {
        files
            .iter()
            .filter(|f| last_picked.as_deref() != Some(f.as_str()))
            .collect()
    } else {
        Vec::new()
    };
    if candidates.is_empty() {
        candidates = files.iter().collect();
    }

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let chosen = candidates[(nanos as usize) % candidates.len()].clone();

    let _ = fs::create_dir_all(&cache_dir);
    let _ = fs::write(&last_logo_file, chosen.as_bytes());

    Some(chosen)
}

fn get_image_color(image_path: &str, auto_color: bool) -> Option<String> {
    let color_path = format!("{image_path}.color");
    if let Ok(content) = std::fs::read_to_string(&color_path) {
        let hex = content.trim();
        if hex.starts_with('#') && (hex.len() == 7 || hex.len() == 4) {
            return Some(hex.to_string());
        }
    }

    if auto_color {
        if let Some(color) = rustfetch_render::extract_dominant_color(image_path) {
            let _ = std::fs::write(&color_path, color.as_bytes());
            return Some(color);
        }
    }

    None
}

fn run_benchmark(modules: &[Box<dyn Module>]) {
    use std::time::Instant;

    println!("\x1b[1m\x1b[36mRustFetch Module Benchmark\x1b[0m");
    println!("──────────────────────────────────────────────");

    let total_start = Instant::now();
    let mut module_timings = Vec::new();

    for m in modules {
        let start = Instant::now();
        let res = m.detect();
        let elapsed = start.elapsed();
        module_timings.push((m.name(), elapsed, res.is_some()));
    }
    let total_time = total_start.elapsed();

    for (name, dur, detected) in &module_timings {
        let us = dur.as_micros();
        let ms = dur.as_secs_f64() * 1000.0;
        let status = if *detected { "✓" } else { "-" };
        let bar_len = ((us as f64 / 1500.0) * 12.0).clamp(1.0, 24.0) as usize;
        let bar = "■".repeat(bar_len);

        if us < 1000 {
            println!(
                "  [{status}] {:<12} : \x1b[32m{:>6} µs\x1b[0m  \x1b[90m{}\x1b[0m",
                name, us, bar
            );
        } else {
            println!(
                "  [{status}] {:<12} : \x1b[33m{:>6.2} ms\x1b[0m  \x1b[90m{}\x1b[0m",
                name, ms, bar
            );
        }
    }

    println!("──────────────────────────────────────────────");
    let total_us = total_time.as_micros();
    let total_ms = total_time.as_secs_f64() * 1000.0;
    println!(
        "  \x1b[1mTotal Time   : \x1b[36m{:.2} ms ({} µs)\x1b[0m ⚡",
        total_ms, total_us
    );
    println!();
}
