mod layout;
mod logos;
mod tools;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use rustfetch_core::config::Config;
use rustfetch_core::modules::{all_modules, module_by_name, module_from_config};
use rustfetch_core::{default_icon_for_module, detect_all, Module};
use serde::Serialize;
use std::io::Write;

use layout::{color_code, InfoLine, LogoBlock};

fn cli_version_string() -> &'static str {
    static VERSION_STR: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    VERSION_STR.get_or_init(|| {
        let ver = rustfetch_core::CURRENT_VERSION;
        let code = rustfetch_core::codename_for_version(ver);
        format!("{ver} \"{code}\"")
    })
}

#[derive(Parser, Debug)]
#[command(
    name = "rustfetch",
    version = cli_version_string(),
    about = "Blazing-fast system information fetch tool written in Rust"
)]
struct Cli {
    #[arg(long, hide = true)]
    check_updates_bg: bool,

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

    #[arg(
        long,
        help = "Output system information as Waybar / Polybar single-line JSON"
    )]
    waybar: bool,

    #[arg(long, value_name = "FORMAT", help = "Output format (e.g. waybar)")]
    format: Option<String>,

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

    #[arg(long, alias = "remove-hook")]
    uninstall_hook: bool,

    #[arg(
        long,
        help = "Disable checking GitHub repository for RustFetch updates"
    )]
    no_update_check: bool,

    #[arg(long, help = "List all available system information modules")]
    list_modules: bool,

    #[arg(long, help = "List all built-in color themes with previews")]
    list_themes: bool,

    #[arg(long, help = "List all layout presets")]
    list_presets: bool,

    #[arg(long, help = "List all built-in OS and cat breed mascot logos")]
    list_logos: bool,

    #[arg(long, help = "Clear cached module data (~/.cache/rustfetch)")]
    clean_cache: bool,

    #[arg(long, help = "Run system, terminal graphics, and config diagnostics")]
    doctor: bool,

    #[arg(
        long,
        alias = "update",
        help = "Update RustFetch to the latest GitHub release while preserving config.toml"
    )]
    self_update: bool,

    #[arg(
        long,
        alias = "tui",
        alias = "configure",
        help = "Launch interactive configuration wizard with live preview"
    )]
    setup: bool,

    #[arg(long, hide = true)]
    keep_logo: bool,
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
            r#"# RustFetch — Shell startup integration
if [[ $- == *i* ]]; then
    _rustfetch_init_hist=""
    _rustfetch_last_size="$(stty size 2>/dev/null)"
    _rustfetch_prompt_cmd() {
        if [ -z "$_rustfetch_init_hist" ]; then
            _rustfetch_init_hist=$HISTCMD
        elif [ "$HISTCMD" -gt "$_rustfetch_init_hist" ]; then
            trap - WINCH
        fi
    }
    PROMPT_COMMAND+=(_rustfetch_prompt_cmd)
    _rustfetch_on_resize() {
        if [ -z "$_rustfetch_init_hist" ] || [ "$HISTCMD" -le "$_rustfetch_init_hist" ]; then
            if [ -z "$READLINE_LINE" ]; then
                local cur_size
                while :; do
                    cur_size="$(stty size 2>/dev/null)"
                    if [ -z "$cur_size" ] || [ "$cur_size" = "$_rustfetch_last_size" ]; then
                        break
                    fi
                    _rustfetch_last_size="$cur_size"
                    rustfetch --keep-logo --no-update-check
                done
                local _rf_ps1="${PS1@P}"
                printf '%s' "${_rf_ps1//[$'\001\002']/}"
            fi
        else
            trap - WINCH
        fi
    }
    trap '_rustfetch_on_resize' WINCH
    rustfetch
fi
"#,
        ),
        "zsh" => Some(
            r#"# RustFetch — Shell startup integration
if [[ -o interactive ]]; then
    _rustfetch_startup=1
    _rustfetch_last_size="$(stty size 2>/dev/null)"
    _rustfetch_preexec() {
        _rustfetch_startup=0
    }
    autoload -Uz add-zsh-hook 2>/dev/null && add-zsh-hook preexec _rustfetch_preexec 2>/dev/null
    TRAPWINCH() {
        if [[ "$_rustfetch_startup" == "1" && -z "$BUFFER" ]]; then
            local cur_size
            while true; do
                cur_size="$(stty size 2>/dev/null)"
                if [[ -z "$cur_size" || "$cur_size" == "$_rustfetch_last_size" ]]; then
                    break
                fi
                _rustfetch_last_size="$cur_size"
                rustfetch --keep-logo --no-update-check
            done
            zle && zle reset-prompt 2>/dev/null
        fi
    }
    rustfetch
fi
"#,
        ),
        "fish" => Some(
            r#"# RustFetch — Shell startup integration
if status is-interactive
    set -g _rustfetch_startup 1
    set -g _rustfetch_last_size (stty size 2>/dev/null)
    function _rustfetch_preexec --on-event fish_preexec
        set -g _rustfetch_startup 0
    end
    function _rustfetch_on_resize --on-signal WINCH
        if test "$_rustfetch_startup" = "1" -a -z (commandline)
            set -l cur_size (stty size 2>/dev/null)
            if test -n "$cur_size" -a "$cur_size" != "$_rustfetch_last_size"
                set -g _rustfetch_last_size "$cur_size"
                rustfetch --keep-logo --no-update-check
                commandline -f repaint 2>/dev/null
            end
        end
    end
    rustfetch
end
"#,
        ),
        _ => None,
    }
}

fn get_shell_target_file(home: &std::path::Path) -> (&'static str, std::path::PathBuf) {
    let shell_path = std::env::var("SHELL").unwrap_or_default();
    let is_zsh = shell_path.ends_with("/zsh") || shell_path == "zsh";
    let is_fish = shell_path.ends_with("/fish") || shell_path == "fish";

    if is_zsh {
        ("zsh", home.join(".zshrc"))
    } else if is_fish {
        ("fish", home.join(".config/fish/config.fish"))
    } else {
        ("bash", home.join(".bashrc"))
    }
}

pub(crate) fn strip_rustfetch_hook(content: &str) -> String {
    let mut result = Vec::new();
    let mut in_rustfetch_block = false;
    let mut block_depth = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("# RustFetch") {
            in_rustfetch_block = true;
            block_depth = 0;
            continue;
        }

        if in_rustfetch_block {
            if trimmed.starts_with("if ") || trimmed.starts_with("function ") {
                block_depth += 1;
            }
            if trimmed == "fi" || trimmed == "end" || (block_depth > 0 && trimmed.ends_with("; fi"))
            {
                if block_depth > 0 {
                    block_depth -= 1;
                }
                if block_depth == 0 {
                    in_rustfetch_block = false;
                    continue;
                }
            }
            continue;
        }

        if trimmed.contains("_rustfetch")
            || trimmed.contains("TRAPWINCH")
            || trimmed == "rustfetch"
            || trimmed == "rfetch"
        {
            continue;
        }

        result.push(line);
    }

    let mut cleaned_lines = Vec::new();
    let mut prev_empty = false;
    for line in result {
        let is_empty = line.trim().is_empty();
        if is_empty && prev_empty {
            continue;
        }
        cleaned_lines.push(line);
        prev_empty = is_empty;
    }

    let mut output = cleaned_lines.join("\n");
    if !output.is_empty() && !output.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn install_shell_hook() {
    let home = match std::env::var("HOME") {
        Ok(h) => std::path::PathBuf::from(h),
        Err(_) => {
            eprintln!("Error: HOME environment variable not set.");
            return;
        }
    };

    let (shell_name, target_file) = get_shell_target_file(&home);

    let script = match shell_init_script(shell_name) {
        Some(s) => s,
        None => return,
    };

    let existing = std::fs::read_to_string(&target_file).unwrap_or_default();
    let cleaned = strip_rustfetch_hook(&existing);

    let mut new_content = cleaned;
    if !new_content.is_empty() && !new_content.ends_with('\n') {
        new_content.push('\n');
    }
    new_content.push_str(script);

    if let Some(parent) = target_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match std::fs::write(&target_file, new_content) {
        Ok(()) => println!(
            "Successfully installed RustFetch startup hook to {}",
            target_file.display()
        ),
        Err(e) => eprintln!("Failed to write to {}: {e}", target_file.display()),
    }
}

fn uninstall_shell_hook() {
    let home = match std::env::var("HOME") {
        Ok(h) => std::path::PathBuf::from(h),
        Err(_) => {
            eprintln!("Error: HOME environment variable not set.");
            return;
        }
    };

    let (_, target_file) = get_shell_target_file(&home);
    if !target_file.exists() {
        println!("No shell configuration found at {}", target_file.display());
        return;
    }

    let existing = std::fs::read_to_string(&target_file).unwrap_or_default();
    let cleaned = strip_rustfetch_hook(&existing);

    match std::fs::write(&target_file, cleaned) {
        Ok(()) => println!(
            "Successfully removed RustFetch hook from {}",
            target_file.display()
        ),
        Err(e) => eprintln!("Failed to write to {}: {e}", target_file.display()),
    }
}

pub fn run() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let cli = Cli::parse();

    if cli.check_updates_bg {
        rustfetch_core::modules::update::refresh_update_cache();
        return;
    }

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

    if cli.uninstall_hook {
        uninstall_shell_hook();
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
        if let Some(bak) = Config::backup_if_exists(&path) {
            println!("Backed up existing configuration to {}", bak.display());
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
                            if let Some(bak) = Config::backup_if_exists(&path) {
                                println!("Backed up existing configuration to {}", bak.display());
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

    if cli.list_modules {
        tools::run_list_modules();
        return;
    }

    if cli.list_themes {
        tools::run_list_themes();
        return;
    }

    if cli.list_presets {
        tools::run_list_presets();
        return;
    }

    if cli.list_logos {
        tools::run_list_logos();
        return;
    }

    if cli.clean_cache {
        tools::run_clean_cache();
        return;
    }

    if cli.doctor {
        tools::run_doctor();
        return;
    }

    if cli.self_update {
        tools::run_self_update();
        return;
    }

    if cli.setup {
        tools::run_setup_tui();
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

    let (mut modules, mut configs): (
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
            .filter_map(|mc| module_from_config(mc).map(|m| (m, mc.clone())))
            .unzip()
    };

    if cli.no_update_check {
        config.general.check_updates = false;
    }

    if !config.general.check_updates {
        let mut filtered_mods = Vec::with_capacity(modules.len());
        let mut filtered_cfgs = Vec::with_capacity(configs.len());
        for (m, c) in modules.into_iter().zip(configs) {
            if c.name != "update" {
                filtered_mods.push(m);
                filtered_cfgs.push(c);
            }
        }
        modules = filtered_mods;
        configs = filtered_cfgs;
    } else if !configs.iter().any(|mc| mc.name == "update") {
        if let Some(m) = module_by_name("update") {
            modules.push(m);
            configs.push(rustfetch_core::config::ModuleConfig::new("update"));
        }
    }

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

    let is_3d = is_3d_requested && is_interactive && !is_dumb && !cli.keep_logo;
    let is_anim = is_anim_requested && is_interactive && !is_dumb && !cli.keep_logo;
    let is_live = (cli.live || config.general.live) && is_interactive && !is_dumb && !cli.keep_logo;
    let is_waybar = cli.waybar
        || cli
            .format
            .as_deref()
            .map(|f| f.eq_ignore_ascii_case("waybar"))
            .unwrap_or(false);
    let is_json = cli.json
        || cli
            .format
            .as_deref()
            .map(|f| f.eq_ignore_ascii_case("json"))
            .unwrap_or(false);

    if cli.benchmark {
        run_benchmark(&modules);
    } else if is_waybar {
        run_waybar(&config, &modules, &configs);
    } else if is_json {
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
    let max_attempts = if cli.keep_logo { 4 } else { 1 };
    for _ in 0..max_attempts {
        if cli.keep_logo {
            let start = std::time::Instant::now();
            let mut last_size = layout::terminal_size();
            let mut stable_ms = 0u64;
            while stable_ms < 45 && start.elapsed() < std::time::Duration::from_millis(300) {
                std::thread::sleep(std::time::Duration::from_millis(15));
                let cur = layout::terminal_size();
                if cur == last_size {
                    stable_ms += 15;
                } else {
                    last_size = cur;
                    stable_ms = 0;
                }
            }
        }
        let size_before = layout::terminal_size();
        run_once_inner(config, cli, modules, configs);
        if !cli.keep_logo || layout::terminal_size() == size_before {
            break;
        }
    }
}

fn begin_keep_logo_frame(keep_logo: bool) {
    if keep_logo && layout::colors_enabled() {
        print!("\x1b[?2026h");
        if terminal_supports_kitty() {
            rustfetch_render::clear_kitty_images();
        }
        print!("\x1b[2J\x1b[3J\x1b[H");
    }
}

fn end_keep_logo_frame(keep_logo: bool) {
    if keep_logo && layout::colors_enabled() {
        print!("\x1b[?2026l");
        let _ = std::io::stdout().flush();
    }
}

fn run_once_inner(
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
    if !cli.no_logo && !cli.keep_logo && terminal_supports_kitty() {
        rustfetch_render::clear_kitty_images();
    }
    let mut resolved_image_path = None;
    let mut dynamic_color = None;
    let mut image_palette = None;

    let custom_ascii_path = cli
        .ascii
        .as_deref()
        .or(config.general.logo.ascii_path.as_deref());

    let is_3d_requested = cli.three_d
        || config.general.logo.three_d.enabled
        || config.general.three_d.unwrap_or(false);

    let show_logo = config.general.logo.enabled && !cli.no_logo;
    if (!is_3d_requested || cli.image.is_some())
        && (custom_ascii_path.is_none() || cli.image.is_some())
        && (show_logo || config.general.logo.auto_color || config.general.colors.image_palette)
    {
        if config.general.logo.random_image && cli.image.is_none() {
            let dir = config
                .general
                .logo
                .image_dir
                .as_deref()
                .unwrap_or("~/.config/rustfetch/logos");
            resolved_image_path = pick_random_image(dir, cli.keep_logo)
                .or_else(|| pick_random_image("~/.config/fastfetch/logos", cli.keep_logo));
        }
        if resolved_image_path.is_none() {
            resolved_image_path = config.general.logo.image_path.clone();
        }

        if let Some(img_path) = &resolved_image_path {
            let expanded_img = tilde_expand(img_path).to_string();
            resolved_image_path = Some(expanded_img.clone());
            let (dom, pal) = get_image_color_and_palette(
                &expanded_img,
                config.general.logo.auto_color,
                config.general.colors.image_palette,
            );
            dynamic_color = dom;
            image_palette = pal;
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
        let (block, color) =
            build_ascii_logo_block(config, custom_ascii_path, None, dynamic_color.as_deref());
        (Some(block), color)
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
            .map(|p| tilde_expand(p).to_string());
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
        if info_lines.len() > max_modules {
            let preserve_update = info_lines
                .last()
                .is_some_and(|l| l.value.starts_with("Update available"));
            if preserve_update {
                let update_line = info_lines.pop();
                info_lines.truncate(max_modules.saturating_sub(1));
                if let Some(u) = update_line {
                    info_lines.push(u);
                }
            } else {
                info_lines.truncate(max_modules);
            }
        }
    }

    if is_compact_term && cli.image.is_none() && cli.ascii.is_none() {
        begin_keep_logo_frame(cli.keep_logo);
        layout::render(None, &info_lines, &render_opts);
        end_keep_logo_frame(cli.keep_logo);
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
                } else if target_cols + min_card_w > avail_for_both {
                    target_cols = avail_for_both.saturating_sub(min_card_w).max(min_image_w);
                }
            } else if effective_logo_pos == "top" || effective_logo_pos == "bottom" {
                target_cols = config_img_w
                    .min(term_cols.saturating_sub(pad_val * 2 + 2))
                    .max(min_image_w);
            }

            render_opts.logo_position = effective_logo_pos;

            let card_rows = info_lines.len()
                + if header.is_some() { 2 } else { 0 }
                + if render_opts.palette_lines.is_some() {
                    2
                } else {
                    0
                }
                + if config.general.border { 2 } else { 0 };
            let default_max_rows = if effective_logo_pos == "top" || effective_logo_pos == "bottom"
            {
                (term_rows / 2).clamp(8, 18)
            } else {
                card_rows
                    .clamp(16, 23)
                    .min(term_rows.saturating_sub(3).max(6))
            };
            let max_rows_val = cli
                .height
                .unwrap_or(default_max_rows)
                .min(term_rows.saturating_sub(2).max(1));
            let max_image_rows = Some(max_rows_val);

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
            let use_iterm = protocol == "iterm"
                || (protocol == "auto"
                    && !use_kitty
                    && !use_sixel
                    && rustfetch_render::supports_iterm());

            if effective_logo_pos == "behind" && terminal_supports_kitty() {
                let card_w = max_right_w
                    .max(header_w)
                    .max(target_cols)
                    .min(term_cols.saturating_sub(pad_val * 2 + 1));
                let card_h = (info_lines.len() + 4).min(max_allowed_rows);
                begin_keep_logo_frame(cli.keep_logo);
                let _ = rustfetch_render::render_background(image_path, card_w, card_h, -1);
                layout::render(None, &info_lines, &render_opts);
                end_keep_logo_frame(cli.keep_logo);
                return;
            }

            let (est_cols, est_rows) =
                rustfetch_render::image_display_dimensions(image_path, target_cols, max_image_rows)
                    .unwrap_or((target_cols, max_rows_val));

            let bounded_card_w = if effective_logo_pos == "top" || effective_logo_pos == "bottom" {
                needed_right_w.min(term_cols.saturating_sub(pad_val * 2 + 1))
            } else {
                needed_right_w
                    .min(term_cols.saturating_sub(est_cols + layout::LOGO_GAP + pad_val * 2 + 1))
            };

            let reserve_rows = est_rows.max(card_rows).min(term_rows.saturating_sub(1));

            if use_kitty {
                let total_fetch_w = if effective_logo_pos == "top" || effective_logo_pos == "bottom"
                {
                    est_cols.max(bounded_card_w)
                } else {
                    est_cols + layout::LOGO_GAP + bounded_card_w
                };
                let h_pad = layout::compute_h_pad(
                    total_fetch_w,
                    config.general.padding,
                    config.general.center,
                );

                let pre_scroll = if effective_logo_pos == "top" || effective_logo_pos == "bottom" {
                    0
                } else {
                    reserve_rows
                };

                if let Some((actual_cols, actual_lines)) = rustfetch_render::render_image_positioned(
                    image_path,
                    target_cols,
                    max_image_rows,
                    pre_scroll,
                    h_pad,
                    cli.keep_logo,
                ) {
                    layout::render_with_image(
                        actual_lines,
                        actual_cols,
                        h_pad,
                        &info_lines,
                        &render_opts,
                    );
                    print!("\x1b[?2026l");
                    let _ = std::io::stdout().flush();
                    return;
                }
                if cli.keep_logo {
                    return;
                }
            } else if use_sixel {
                let total_fetch_w = if effective_logo_pos == "top" || effective_logo_pos == "bottom"
                {
                    est_cols.max(bounded_card_w)
                } else {
                    est_cols + layout::LOGO_GAP + bounded_card_w
                };
                let h_pad = layout::compute_h_pad(
                    total_fetch_w,
                    config.general.padding,
                    config.general.center,
                );

                begin_keep_logo_frame(cli.keep_logo);
                if effective_logo_pos != "top" && effective_logo_pos != "bottom" && reserve_rows > 1
                {
                    print!(
                        "{}\x1b[{}A\r",
                        "\n".repeat(reserve_rows - 1),
                        reserve_rows - 1
                    );
                }

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
                    end_keep_logo_frame(cli.keep_logo);
                    return;
                }
                end_keep_logo_frame(cli.keep_logo);
            } else if use_iterm {
                let total_fetch_w = if effective_logo_pos == "top" || effective_logo_pos == "bottom"
                {
                    est_cols.max(bounded_card_w)
                } else {
                    est_cols + layout::LOGO_GAP + bounded_card_w
                };
                let h_pad = layout::compute_h_pad(
                    total_fetch_w,
                    config.general.padding,
                    config.general.center,
                );

                begin_keep_logo_frame(cli.keep_logo);
                if effective_logo_pos != "top" && effective_logo_pos != "bottom" && reserve_rows > 1
                {
                    print!(
                        "{}\x1b[{}A\r",
                        "\n".repeat(reserve_rows - 1),
                        reserve_rows - 1
                    );
                }

                if h_pad > 0 {
                    print!("{}", " ".repeat(h_pad));
                    let _ = std::io::stdout().flush();
                }

                if let Some((actual_cols, actual_lines)) =
                    rustfetch_render::render_iterm_ex(image_path, target_cols, max_image_rows)
                {
                    layout::render_with_image(
                        actual_lines,
                        actual_cols,
                        h_pad,
                        &info_lines,
                        &render_opts,
                    );
                    end_keep_logo_frame(cli.keep_logo);
                    return;
                }
                end_keep_logo_frame(cli.keep_logo);
            }

            if let Some(lines) = if protocol == "braille" {
                rustfetch_render::render_braille_ex(image_path, target_cols, max_image_rows, true)
            } else {
                rustfetch_render::render_halfblock_ex(image_path, target_cols, max_image_rows, true)
            } {
                let w = lines
                    .first()
                    .map(|l| layout::strip_ansi_width(l))
                    .unwrap_or(target_cols);
                let logo_block = LogoBlock { lines, width: w };
                begin_keep_logo_frame(cli.keep_logo);
                layout::render(Some(&logo_block), &info_lines, &render_opts);
                end_keep_logo_frame(cli.keep_logo);
                return;
            }
        }
    }

    begin_keep_logo_frame(cli.keep_logo);
    layout::render(logo_block.as_ref(), &info_lines, &render_opts);
    end_keep_logo_frame(cli.keep_logo);
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

#[derive(Serialize)]
struct WaybarOutput {
    text: String,
    tooltip: String,
    class: &'static str,
}

fn generate_waybar_json(
    config: &Config,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) -> String {
    let info_lines = gather_info_lines(config, modules, configs, None, None);
    let header = build_header();
    let render_opts = layout::RenderOptions {
        header: header.as_deref(),
        header_color: None,
        separator: &config.general.separator,
        padding: 0,
        center: false,
        border: false,
        border_style: None,
        border_color: None,
        border_chars: None,
        border_title: None,
        box_padding: 0,
        palette_lines: None,
        palette_position: "none",
        icon_only: false,
        key_type: None,
        border_image: None,
        border_image_position: "none",
        border_images: &[],
        logo_position: "none",
    };

    let right_lines = layout::build_right_lines(&info_lines, &render_opts);
    let tooltip_lines: Vec<String> = right_lines
        .into_iter()
        .map(|l| layout::strip_ansi(&l))
        .collect();
    let tooltip = tooltip_lines.join("\n").trim().to_string();

    let raw_text = if let Some(os_line) = info_lines
        .iter()
        .find(|l| l.label.eq_ignore_ascii_case("os"))
    {
        let clean_val = layout::strip_ansi(&os_line.value);
        if let Some(icon) = &os_line.icon {
            format!("{icon} {clean_val}")
        } else {
            clean_val
        }
    } else if let Some(first_line) = info_lines
        .iter()
        .find(|l| !l.value.is_empty() && !l.label.starts_with("---DIVIDER"))
    {
        let clean_val = layout::strip_ansi(&first_line.value);
        if let Some(icon) = &first_line.icon {
            format!("{icon} {clean_val}")
        } else {
            clean_val
        }
    } else {
        "rustfetch".to_string()
    };
    let text = layout::strip_ansi(&raw_text).trim().to_string();

    let output = WaybarOutput {
        text,
        tooltip,
        class: "rustfetch",
    };

    serde_json::to_string(&output).unwrap_or_default()
}

fn run_waybar(
    config: &Config,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) {
    let json_str = generate_waybar_json(config, modules, configs);
    if !json_str.is_empty() {
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

    if let Some(pos) = val.find("°C") {
        let prefix = &val[..pos];
        let num_str: String = prefix
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let num_str: String = num_str.chars().rev().collect();
        if let Ok(temp_c) = num_str.parse::<f64>() {
            return Some((temp_c.round() as u8).min(100));
        }
    }

    None
}

pub fn render_sparkline(samples: &[u8], colored: bool) -> String {
    const TICKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    if samples.is_empty() {
        return String::new();
    }
    let mut out = String::with_capacity(samples.len() * 16);
    for &pct in samples {
        let clamped = pct.min(100);
        let idx = ((clamped as usize) * 7) / 100;
        if colored {
            let col = if clamped < 60 {
                "\x1b[38;2;102;217;160m"
            } else if clamped < 85 {
                "\x1b[38;2;249;175;79m"
            } else {
                "\x1b[38;2;243;139;168m"
            };
            out.push_str(col);
            out.push(TICKS[idx]);
        } else {
            out.push(TICKS[idx]);
        }
    }
    if colored {
        out.push_str("\x1b[0m");
    }
    out
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
            "\x1b[38;2;102;217;160m"
        } else if percent < 85 {
            "\x1b[38;2;249;175;79m"
        } else {
            "\x1b[38;2;243;139;168m"
        }
    } else if percent < 20 {
        "\x1b[38;2;243;139;168m"
    } else if percent < 50 {
        "\x1b[38;2;249;175;79m"
    } else {
        "\x1b[38;2;102;217;160m"
    };

    let dim_track = if colored {
        "\x1b[38;2;108;112;134m"
    } else {
        ""
    };
    let reset = if colored { "\x1b[0m" } else { "" };
    format!(
        "{}[{}{}{}{}]",
        color_code,
        "■".repeat(elapsed),
        dim_track,
        "-".repeat(remaining),
        color_code
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
    let collected: Vec<(InfoLine, bool)> = results
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
                return Some((
                    InfoLine {
                        label,
                        value: String::new(),
                        icon: None,
                        color: String::new(),
                    },
                    false,
                ));
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

            if mc.name == "update" && layout::colors_enabled() {
                value = format!(
                    "\x1b]8;;https://github.com/MehmetCanWT/rustfetch/releases/latest\x07{value}\x1b]8;;\x07"
                );
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
            let has_explicit_color = mc.color.is_some() || mc.name == "update";
            let color_str = mc
                .color
                .as_deref()
                .or(if mc.name == "update" {
                    Some("bright_yellow")
                } else {
                    None
                })
                .or(dynamic_color)
                .or(theme_color.as_deref())
                .or(distro_color)
                .unwrap_or("blue");
            let color = color_code(color_str);
            Some((
                InfoLine {
                    label,
                    value,
                    icon,
                    color,
                },
                has_explicit_color,
            ))
        })
        .collect();

    let grad_rgb: Vec<(u8, u8, u8)> = config
        .general
        .gradient
        .as_ref()
        .map(|list| {
            list.iter()
                .filter_map(|s| rustfetch_render::parse_color(s))
                .collect()
        })
        .unwrap_or_default();

    let grad_target_count = collected
        .iter()
        .filter(|(line, explicit)| !explicit && !line.label.starts_with("---DIVIDER"))
        .count();

    let mut grad_idx = 0usize;
    collected
        .into_iter()
        .map(|(mut line, explicit)| {
            if !grad_rgb.is_empty() && !explicit && !line.label.starts_with("---DIVIDER") {
                let t = if grad_target_count <= 1 {
                    0.0
                } else {
                    grad_idx as f32 / (grad_target_count - 1) as f32
                };
                grad_idx += 1;
                let (r, g, b) = rustfetch_render::interpolate_gradient(&grad_rgb, t);
                line.color = color_code(&format!("#{:02x}{:02x}{:02x}", r, g, b));
            }
            line
        })
        .collect()
}

fn build_right_lines(info_lines: &[InfoLine], opts: &layout::RenderOptions) -> Vec<String> {
    layout::build_right_lines(info_lines, opts)
}

#[cfg(unix)]
static SIGINT_RECEIVED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(unix)]
extern "C" fn handle_sigint(_: libc::c_int) {
    SIGINT_RECEIVED.store(true, std::sync::atomic::Ordering::SeqCst);
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
        SIGINT_RECEIVED.store(false, std::sync::atomic::Ordering::SeqCst);
        #[cfg(unix)]
        let mut orig_termios = None;
        #[cfg(unix)]
        unsafe {
            let mut orig: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(libc::STDIN_FILENO, &mut orig) == 0 {
                orig_termios = Some(orig);
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
    let max_w = term_cols.saturating_sub(1).max(1);
    let max_r = term_rows.saturating_sub(1).max(1);
    let mut rows_written = 0usize;
    let mut frame_buf = String::with_capacity(term_cols * term_rows * 4 + 64);
    frame_buf.push_str("\x1b[H");

    let push_bounded = |buf: &mut String, row: String, rows_written: &mut usize| {
        if *rows_written >= max_r {
            return;
        }
        if layout::strip_ansi_width(&row) > max_w {
            buf.push_str(&layout::truncate_ansi(&row, max_w));
        } else {
            buf.push_str(&row);
        }
        buf.push_str("\x1b[K\r\n");
        *rows_written += 1;
    };

    if layout_stacked {
        let left_pad = if config.general.center {
            " ".repeat(term_cols.saturating_sub(left_width) / 2)
        } else {
            " ".repeat(config.general.padding)
        };
        for line in left_lines {
            push_bounded(
                &mut frame_buf,
                format!("{left_pad}{line}"),
                &mut rows_written,
            );
        }
        push_bounded(&mut frame_buf, String::new(), &mut rows_written);
        let info_pad = if config.general.center {
            " ".repeat(term_cols.saturating_sub(right_width) / 2)
        } else {
            " ".repeat(config.general.padding)
        };
        for line in right_lines {
            push_bounded(
                &mut frame_buf,
                format!("{info_pad}{line}"),
                &mut rows_written,
            );
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
            let mut row = String::with_capacity(total_content_w + 64);
            row.push_str(&left_pad);
            if let Some(line) = left_lines.get(r) {
                row.push_str(line);
                if pad_short_left_lines {
                    let line_w = layout::strip_ansi_width(line);
                    if line_w < left_width {
                        row.push_str(&" ".repeat(left_width - line_w));
                    }
                }
            } else {
                row.push_str(&" ".repeat(left_width));
            }
            row.push_str(&gap);
            if let Some(right_line) = right_lines.get(r) {
                row.push_str(right_line);
            }
            push_bounded(&mut frame_buf, row, &mut rows_written);
        }
    }
    frame_buf.push_str("\x1b[J");
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
            let expanded = tilde_expand(ascii_p);
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
    renderer.set_light_direction(&config.general.logo.three_d.light);
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
            .filter_map(|mc| module_from_config(&mc).map(|m| (m, mc)))
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

    let exit_on_key = !cli.hold;
    let mut prev_term_size = layout::terminal_size();

    loop {
        #[cfg(unix)]
        if SIGINT_RECEIVED.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
        if let Some(limit) = max_frames {
            if frame_idx >= limit {
                break;
            }
        }

        let (term_cols, term_rows) = layout::terminal_size();
        if (term_cols, term_rows) != prev_term_size {
            prev_term_size = (term_cols, term_rows);
            let _ = stdout.write_all(b"\x1b[2J\x1b[H");
            right_lines = build_right_lines(&info_lines, &render_opts);
        }

        // Receive non-blocking background telemetry updates
        while let Ok(new_info) = telemetry_rx.try_recv() {
            info_lines = new_info;
            right_lines = build_right_lines(&info_lines, &render_opts);
        }

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
                .min(term_rows.saturating_sub(4).max(8))
        } else {
            let ideal_h = (canvas_cols * 3 / 5).clamp(18, 24);
            cli.height
                .or(config.general.logo.three_d.height)
                .unwrap_or(ideal_h.max(right_lines.len().min(24)))
                .min(term_rows.saturating_sub(2).max(8))
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
    drop(_guard);

    // Fallback immediately from 3D canvas to static 2D ASCII logo of the same distro
    print!("\x1b[2J\x1b[H");
    let _ = std::io::stdout().flush();

    let _wrap_guard = LineWrapGuard::new();
    let (logo_block, _) = build_ascii_logo_block(
        config,
        custom_ascii_path,
        Some(&distro_name),
        cli.color.as_deref(),
    );

    layout::render(Some(&logo_block), &info_lines, &render_opts);
}

fn build_ascii_logo_block(
    config: &Config,
    custom_ascii_path: Option<&str>,
    distro_override: Option<&str>,
    dynamic_color: Option<&str>,
) -> (LogoBlock, Option<&'static str>) {
    if let Some(raw) = custom_ascii_path
        .and_then(|ascii_p| {
            let expanded = tilde_expand(ascii_p);
            std::fs::read_to_string(expanded.as_ref()).ok()
        })
        .filter(|raw| !raw.is_empty())
    {
        let processed = if let Some(colors) = &config.general.logo.colors {
            let codes: Vec<String> = colors.iter().map(|c| layout::color_code(c)).collect();
            let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
            rustfetch_core::apply_neofetch_colors(&raw, &refs)
        } else {
            raw
        };
        let lines: Vec<String> = processed.lines().map(|s| s.to_string()).collect();
        let width = lines
            .iter()
            .map(|l| layout::strip_ansi_width(l))
            .max()
            .unwrap_or(0);
        return (LogoBlock { lines, width }, None);
    }

    let distro_name = distro_override.map(str::to_string).unwrap_or_else(|| {
        if config.general.logo.distro == "auto" {
            detect_distro_name()
        } else {
            config.general.logo.distro.clone()
        }
    });
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
    } else if let Some(dc) = dynamic_color {
        let dc_code = layout::color_code(dc);
        logo.colored_lines_with_custom(Some(&dc_code), None)
    } else {
        logo.colored_lines()
    };
    (
        LogoBlock {
            lines: colored_lines,
            width: logo.width,
        },
        Some(logo.color),
    )
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

    let expanded = tilde_expand(path_str);
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
        #[cfg(unix)]
        if SIGINT_RECEIVED.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
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

    let mut live_config = config.clone();
    let mut live_configs = configs.to_vec();
    for mc in &mut live_configs {
        if matches!(
            mc.name.as_str(),
            "memory" | "disk" | "swap" | "battery" | "cpu_usage" | "temp"
        ) && mc.bar.is_none()
        {
            mc.bar = Some(true);
        }
    }

    const THEMES: &[&str] = &[
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
    let mut theme_idx = live_config
        .general
        .theme
        .as_deref()
        .and_then(|t| THEMES.iter().position(|&x| x.eq_ignore_ascii_case(t)))
        .unwrap_or(0);

    let mut cpu_history: Vec<u8> = Vec::with_capacity(12);
    let mut mem_history: Vec<u8> = Vec::with_capacity(12);

    loop {
        if let Some(pct) = rustfetch_core::modules::cpu_usage::CpuUsage
            .detect()
            .and_then(|i| extract_percentage(&i.value))
        {
            if cpu_history.len() >= 12 {
                cpu_history.remove(0);
            }
            cpu_history.push(pct);
        }
        if let Some(pct) = rustfetch_core::modules::memory::Memory
            .detect()
            .and_then(|i| extract_percentage(&i.value))
        {
            if mem_history.len() >= 12 {
                mem_history.remove(0);
            }
            mem_history.push(pct);
        }

        let _ = queue!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        run_once(&live_config, cli, modules, &live_configs);

        let colored = layout::colors_enabled();
        let cpu_spark = render_sparkline(&cpu_history, colored);
        let mem_spark = render_sparkline(&mem_history, colored);
        let active_theme = live_config.general.theme.as_deref().unwrap_or("default");
        print!(
            "\r\n  \x1b[2mLive Graphs:\x1b[0m CPU {cpu_spark}   RAM {mem_spark}   \x1b[2m|  [t] Theme ({active_theme})  [b] Border  [q] Quit\x1b[0m\r\n"
        );
        let _ = stdout.flush();

        if let Ok(ready) = poll(Duration::from_millis(1000)) {
            if ready {
                if let Ok(event) = read() {
                    match event {
                        Event::Key(key)
                            if key.code == KeyCode::Char('q')
                                || key.code == KeyCode::Char('Q')
                                || key.code == KeyCode::Esc
                                || (key.code == KeyCode::Char('c')
                                    && key
                                        .modifiers
                                        .contains(crossterm::event::KeyModifiers::CONTROL)) =>
                        {
                            break;
                        }
                        Event::Key(key)
                            if key.code == KeyCode::Char('t') || key.code == KeyCode::Char('T') =>
                        {
                            theme_idx = (theme_idx + 1) % THEMES.len();
                            let next_theme = THEMES[theme_idx];
                            live_config.general.theme = Some(next_theme.to_string());
                            if let Some(t) = rustfetch_render::get_theme(next_theme) {
                                live_config.general.border_color = Some(format!(
                                    "#{:02x}{:02x}{:02x}",
                                    t.border.0, t.border.1, t.border.2
                                ));
                            }
                        }
                        Event::Key(key)
                            if key.code == KeyCode::Char('b') || key.code == KeyCode::Char('B') =>
                        {
                            live_config.general.border = !live_config.general.border;
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
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .ok()?;
    let hostname = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| {
            std::fs::read_to_string("/etc/hostname")
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })?;
    Some(format!("{user}@{hostname}"))
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

pub fn tilde_expand(path: &str) -> std::borrow::Cow<'_, str> {
    if path == "~" {
        if let Ok(home) = std::env::var("HOME") {
            return std::borrow::Cow::Owned(home);
        }
    } else if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            let home_trimmed = home.trim_end_matches('/');
            return std::borrow::Cow::Owned(format!("{home_trimmed}/{stripped}"));
        }
    }
    std::borrow::Cow::Borrowed(path)
}

pub fn filter_candidate_images<'a>(files: &'a [String], recent: &[String]) -> Vec<&'a String> {
    let n = files.len();
    let exclude_count = match n {
        0 | 1 => 0,
        2 => 1,
        3 => 2,
        _ => 3,
    };

    let mut relevant_recent: Vec<&str> = Vec::new();
    for r in recent {
        let r_str = r.as_str();
        let r_name = std::path::Path::new(r_str).file_name();
        let matches = files.iter().any(|f| {
            f.as_str() == r_str
                || (r_name.is_some() && std::path::Path::new(f.as_str()).file_name() == r_name)
        });
        if matches && !relevant_recent.contains(&r_str) {
            relevant_recent.push(r_str);
        }
    }

    let excluded: Vec<&str> = relevant_recent.into_iter().take(exclude_count).collect();

    let candidates: Vec<&String> = files
        .iter()
        .filter(|f| {
            let f_str = f.as_str();
            let f_name = std::path::Path::new(f_str).file_name();
            !excluded.iter().any(|&ex| {
                ex == f_str || (f_name.is_some() && std::path::Path::new(ex).file_name() == f_name)
            })
        })
        .collect();

    if candidates.is_empty() {
        files.iter().collect()
    } else {
        candidates
    }
}

pub fn update_recent_history(recent: &[String], chosen: &str) -> Vec<String> {
    let mut updated = Vec::with_capacity(3);
    updated.push(chosen.to_string());
    for r in recent {
        if r != chosen && updated.len() < 3 {
            updated.push(r.clone());
        }
    }
    updated
}

fn pick_random_image(dir_path: &str, keep_last: bool) -> Option<String> {
    use std::fs;
    let base_dir = std::env::var("XDG_CACHE_HOME")
        .unwrap_or_else(|_| format!("{}/.cache", std::env::var("HOME").unwrap_or_default()));
    let cache_dir = std::path::PathBuf::from(base_dir).join("rustfetch");
    let last_logo_file = cache_dir.join("last_logo");
    let recent_images_file = cache_dir.join("recent_images");

    let expanded = tilde_expand(dir_path);
    let expanded_path = std::path::Path::new(expanded.as_ref());
    if !expanded_path.is_dir() {
        return None;
    }

    let entries = fs::read_dir(expanded_path).ok()?;
    let mut files: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let Some(name_str) = file_name.to_str() else {
            continue;
        };
        let is_image_ext = std::path::Path::new(name_str)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                ext.eq_ignore_ascii_case("png")
                    || ext.eq_ignore_ascii_case("jpg")
                    || ext.eq_ignore_ascii_case("jpeg")
                    || ext.eq_ignore_ascii_case("gif")
                    || ext.eq_ignore_ascii_case("webp")
            })
            .unwrap_or(false);
        if !is_image_ext {
            continue;
        }
        let is_file = entry
            .file_type()
            .map(|ft| ft.is_file() || (ft.is_symlink() && entry.path().is_file()))
            .unwrap_or(false);
        if is_file {
            if let Some(s) = entry.path().to_str() {
                files.push(s.to_string());
            }
        }
    }
    if files.is_empty() {
        return None;
    }
    files.sort();

    let mut recent: Vec<String> = fs::read_to_string(&recent_images_file)
        .ok()
        .map(|content| {
            content
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .take(3)
                .collect()
        })
        .unwrap_or_default();

    if recent.is_empty() {
        if let Ok(last) = fs::read_to_string(&last_logo_file) {
            let trimmed = last.trim();
            if !trimmed.is_empty() {
                recent.push(trimmed.to_string());
            }
        }
    }

    if keep_last {
        if let Some(first) = recent.first() {
            let first_name = std::path::Path::new(first).file_name();
            if let Some(matched) = files.iter().find(|f| {
                f.as_str() == first.as_str()
                    || (first_name.is_some()
                        && std::path::Path::new(f.as_str()).file_name() == first_name)
            }) {
                return Some(matched.clone());
            }
        }
    }

    let candidates = filter_candidate_images(&files, &recent);

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let chosen = candidates[(nanos as usize) % candidates.len()].clone();

    let updated_recent = update_recent_history(&recent, &chosen);

    let _ = fs::create_dir_all(&cache_dir);
    let _ = fs::write(&last_logo_file, chosen.as_bytes());
    let _ = fs::write(&recent_images_file, updated_recent.join("\n").as_bytes());

    Some(chosen)
}

#[allow(clippy::type_complexity)]
fn get_image_color_and_palette(
    image_path: &str,
    auto_color: bool,
    want_palette: bool,
) -> (Option<String>, Option<Vec<(u8, u8, u8)>>) {
    let color_path = format!("{image_path}.color");
    let mut cached_dom = None;
    let mut cached_pal = None;

    if let Ok(content) = std::fs::read_to_string(&color_path) {
        let mut lines = content.lines();
        if let Some(first) = lines.next().map(str::trim) {
            if first.starts_with('#') && (first.len() == 7 || first.len() == 4) {
                cached_dom = Some(first.to_string());
            }
        }
        if let Some(second) = lines.next().map(str::trim) {
            let parsed: Vec<(u8, u8, u8)> = second
                .split(',')
                .filter_map(|s| rustfetch_render::parse_color(s.trim()))
                .collect();
            if !parsed.is_empty() {
                cached_pal = Some(parsed);
            }
        }
    }

    let need_dom = auto_color && cached_dom.is_none();
    let need_pal = want_palette && cached_pal.is_none();

    if need_dom || need_pal {
        if let Some((dom, pal)) = rustfetch_render::extract_dominant_and_palette(image_path, 16) {
            let final_dom = cached_dom.or(Some(dom));
            let final_pal = cached_pal.or(pal);
            if let Some(ref d) = final_dom {
                let mut out = d.clone();
                if let Some(ref p) = final_pal {
                    let pal_hex: Vec<String> = p
                        .iter()
                        .map(|(r, g, b)| format!("#{:02x}{:02x}{:02x}", r, g, b))
                        .collect();
                    if !pal_hex.is_empty() {
                        out.push('\n');
                        out.push_str(&pal_hex.join(","));
                    }
                }
                let _ = std::fs::write(&color_path, out.as_bytes());
            }
            return (
                if auto_color { final_dom } else { None },
                if want_palette { final_pal } else { None },
            );
        }
    }

    (
        if auto_color { cached_dom } else { None },
        if want_palette { cached_pal } else { None },
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_rustfetch_hook_old_bash() {
        let input = r#"# .bashrc
export PATH="$HOME/.local/bin:$PATH"

# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
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

alias ll='ls -la'
"#;

        let cleaned = strip_rustfetch_hook(input);
        assert!(!cleaned.contains("_rustfetch"));
        assert!(!cleaned.contains("WINCH"));
        assert!(!cleaned.contains("clear"));
        assert!(cleaned.contains(r#"export PATH="$HOME/.local/bin:$PATH""#));
        assert!(cleaned.contains("alias ll='ls -la'"));
    }

    #[test]
    fn test_strip_rustfetch_hook_new_bash() {
        let input = r#"# .bashrc
export PATH="$HOME/.local/bin:$PATH"

# RustFetch — Shell startup integration
if [[ $- == *i* ]]; then
    rustfetch
fi

alias ll='ls -la'
"#;

        let cleaned = strip_rustfetch_hook(input);
        assert!(!cleaned.contains("RustFetch"));
        assert!(cleaned.contains(r#"export PATH="$HOME/.local/bin:$PATH""#));
        assert!(cleaned.contains("alias ll='ls -la'"));
    }

    #[test]
    fn test_strip_rustfetch_hook_zsh_and_fish() {
        let zsh_input = r#"# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
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
"#;
        let cleaned_zsh = strip_rustfetch_hook(zsh_input);
        assert!(cleaned_zsh.trim().is_empty());

        let fish_input = r#"# RustFetch — Automatic responsive redraw for tiling WMs (MangoWM, Hyprland, Sway, etc.)
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
"#;
        let cleaned_fish = strip_rustfetch_hook(fish_input);
        assert!(cleaned_fish.trim().is_empty());
    }

    #[test]
    fn test_tilde_expand() {
        assert_eq!(tilde_expand("foo/bar"), "foo/bar");
        assert_eq!(tilde_expand("/etc/os-release"), "/etc/os-release");
        if let Ok(home) = std::env::var("HOME") {
            let trimmed = home.trim_end_matches('/');
            assert_eq!(tilde_expand("~"), home);
            assert_eq!(tilde_expand("~/foo/bar"), format!("{trimmed}/foo/bar"));
        }
    }

    #[test]
    fn test_pick_random_image() {
        let tmp = std::env::temp_dir().join(format!("rustfetch_img_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        // Non-images that should be ignored:
        std::fs::write(tmp.join("png"), b"not an image").unwrap();
        std::fs::write(tmp.join(".png"), b"hidden file").unwrap();
        std::fs::write(tmp.join("readme.txt"), b"text").unwrap();
        assert!(pick_random_image(tmp.to_str().unwrap(), false).is_none());

        // Valid image
        std::fs::write(tmp.join("logo.png"), b"pngdata").unwrap();
        let picked = pick_random_image(tmp.to_str().unwrap(), false);
        assert!(picked.is_some());
        assert!(picked.unwrap().ends_with("logo.png"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_candidate_filtering_rules() {
        // N = 1: picks the single image even if it was recently used
        let files_n1 = vec!["img1.png".to_string()];
        let recent_n1 = vec!["img1.png".to_string()];
        let c_n1 = filter_candidate_images(&files_n1, &recent_n1);
        assert_eq!(c_n1, vec![&files_n1[0]]);

        // N = 2: excludes the last 1 used image, picks the other
        let files_n2 = vec!["img1.png".to_string(), "img2.png".to_string()];
        let recent_n2_a = vec!["img1.png".to_string()];
        let c_n2_a = filter_candidate_images(&files_n2, &recent_n2_a);
        assert_eq!(c_n2_a, vec![&files_n2[1]]); // picks img2

        let recent_n2_b = vec!["img2.png".to_string(), "img1.png".to_string()];
        let c_n2_b = filter_candidate_images(&files_n2, &recent_n2_b);
        assert_eq!(c_n2_b, vec![&files_n2[0]]); // picks img1

        // N = 3: excludes the last 2 used images, picks the 3rd one
        let files_n3 = vec![
            "img1.png".to_string(),
            "img2.png".to_string(),
            "img3.png".to_string(),
        ];
        let recent_n3 = vec!["img2.png".to_string(), "img1.png".to_string()];
        let c_n3 = filter_candidate_images(&files_n3, &recent_n3);
        assert_eq!(c_n3, vec![&files_n3[2]]); // picks img3

        let recent_n3_cycle = vec![
            "img3.png".to_string(),
            "img2.png".to_string(),
            "img1.png".to_string(),
        ];
        let c_n3_cycle = filter_candidate_images(&files_n3, &recent_n3_cycle);
        assert_eq!(c_n3_cycle, vec![&files_n3[0]]); // picks img1

        // N = 4+: excludes the last 3 used images, candidates are the remaining images
        let files_n5 = vec![
            "img1.png".to_string(),
            "img2.png".to_string(),
            "img3.png".to_string(),
            "img4.png".to_string(),
            "img5.png".to_string(),
        ];
        let recent_n5 = vec![
            "img3.png".to_string(),
            "img2.png".to_string(),
            "img1.png".to_string(),
        ];
        let c_n5 = filter_candidate_images(&files_n5, &recent_n5);
        assert_eq!(c_n5, vec![&files_n5[3], &files_n5[4]]); // only img4 and img5
        assert!(!c_n5.iter().any(|&c| recent_n5.contains(c)));

        // Foreign / deleted paths in history must not consume exclusion slots
        let recent_with_deleted = vec![
            "/deleted/old.png".to_string(),
            "img2.png".to_string(),
            "img1.png".to_string(),
        ];
        let c_foreign = filter_candidate_images(&files_n3, &recent_with_deleted);
        assert_eq!(c_foreign, vec![&files_n3[2]]); // correctly excludes img2 and img1, picks img3

        // Duplicate entries in recent history do not burn multiple exclusion slots
        let recent_dups = vec![
            "img2.png".to_string(),
            "img2.png".to_string(),
            "img1.png".to_string(),
        ];
        let c_dups = filter_candidate_images(&files_n3, &recent_dups);
        assert_eq!(c_dups, vec![&files_n3[2]]); // still excludes img2 and img1

        // History update keeps max 3 entries with chosen at front and deduplicates
        let h1 = update_recent_history(
            &["img2.png".to_string(), "img1.png".to_string()],
            "img3.png",
        );
        assert_eq!(h1, vec!["img3.png", "img2.png", "img1.png"]);

        let h2 = update_recent_history(&h1, "img4.png");
        assert_eq!(h2, vec!["img4.png", "img3.png", "img2.png"]);

        let h3 = update_recent_history(&h2, "img3.png");
        assert_eq!(h3, vec!["img3.png", "img4.png", "img2.png"]);
    }

    #[test]
    fn test_generate_waybar_json() {
        let config = Config::default();
        let (modules, configs): (Vec<_>, Vec<_>) = config
            .modules
            .iter()
            .filter_map(|mc| module_from_config(mc).map(|m| (m, mc.clone())))
            .unzip();
        let json_str = generate_waybar_json(&config, &modules, &configs);
        assert!(!json_str.is_empty());
        assert!(
            !json_str.contains('\n'),
            "Waybar output must be a single line"
        );
        let parsed: serde_json::Value = serde_json::from_str(&json_str).expect("Valid JSON");
        assert_eq!(parsed["class"], "rustfetch");
        assert!(parsed.get("text").is_some());
        assert!(parsed.get("tooltip").is_some());
        let tooltip = parsed["tooltip"].as_str().unwrap();
        assert!(
            !tooltip.contains("\x1b["),
            "ANSI escape codes must be stripped from tooltip"
        );
        let text = parsed["text"].as_str().unwrap();
        assert!(
            !text.contains("\x1b["),
            "ANSI escape codes must be stripped from text"
        );
        assert!(!text.is_empty());
    }
}
