mod layout;
mod logos;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use rustfetch_core::config::Config;
use rustfetch_core::modules::{all_modules, module_by_name};
use rustfetch_core::{default_icon_for_module, detect_all, Module};
use serde::Serialize;
use std::io::Write;

use layout::{color_code, InfoLine, LogoBlock, BOLD, DIM, RESET};

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
    json: bool,

    #[arg(long, value_name = "PRESET")]
    preset: Option<String>,

    #[arg(long)]
    benchmark: bool,

    #[arg(long, value_enum, value_name = "SHELL")]
    completions: Option<Shell>,
}

fn main() {
    let cli = Cli::parse();

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
                "Unknown preset: {preset_name}. Available presets: default, minimal, card, modern, compact"
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

    let is_3d = cli.three_d
        || config.general.logo.three_d.enabled
        || config.general.three_d.unwrap_or(false);

    if cli.benchmark {
        run_benchmark(&modules);
    } else if cli.json {
        run_json(&config, &modules, &configs);
    } else if is_3d {
        run_3d(&config, &cli, &modules, &configs);
    } else if cli.live {
        run_live(&config, &cli, &modules, &configs);
    } else {
        run_once(&config, &cli, &modules, &configs);
    }
}

fn run_once(
    config: &Config,
    cli: &Cli,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) {
    let mut resolved_image_path = None;
    let mut dynamic_color = None;
    let mut image_palette = None;

    let show_logo = config.general.logo.enabled && !cli.no_logo;
    if show_logo {
        if config.general.logo.random_image {
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
            dynamic_color = get_image_color(img_path, config.general.logo.auto_color);
            if config.general.colors.image_palette {
                image_palette = rustfetch_render::extract_color_palette(img_path, 16);
            }
        }
    }

    let palette_lines = if config.general.colors.enabled {
        Some(layout::build_color_palette(
            &config.general.colors.symbol,
            config.general.colors.block,
            image_palette.as_deref(),
        ))
    } else {
        None
    };

    let info_lines = gather_info_lines(config, modules, configs, dynamic_color.as_deref());

    let header = build_header();

    let render_opts = layout::RenderOptions {
        header: header.as_deref(),
        separator: &config.general.separator,
        padding: config.general.padding,
        center: config.general.center,
        border: config.general.border,
        palette_lines: palette_lines.as_deref(),
    };

    if show_logo {
        if let Some(image_path) = &resolved_image_path {
            let mut target_cols = config.general.logo.image_width_cols;

            if cli.live {
                if let Ok((term_cols, _)) = crossterm::terminal::size() {
                    let max_cols = (term_cols as usize) / 2;
                    if target_cols > max_cols && max_cols > 5 {
                        target_cols = max_cols;
                    }
                }
            }

            let protocol = config.general.logo.protocol.to_lowercase();
            let use_kitty =
                protocol == "kitty" || (protocol == "auto" && terminal_supports_kitty());

            if !use_kitty {
                if let Some(lines) = rustfetch_render::render_halfblock(image_path, target_cols) {
                    let logo_block = LogoBlock {
                        lines,
                        width: target_cols,
                    };
                    layout::render(Some(&logo_block), &info_lines, &render_opts);
                    return;
                }
            } else {
                let right_preview =
                    layout::format_info_lines(&info_lines, &config.general.separator);
                let max_right_w = right_preview
                    .iter()
                    .map(|l| layout::strip_ansi_width(l))
                    .max()
                    .unwrap_or(0);
                let header_w = header
                    .as_ref()
                    .map(|h| unicode_width::UnicodeWidthStr::width(h.as_str()))
                    .unwrap_or(0);
                let total_fetch_w = target_cols + layout::LOGO_GAP + max_right_w.max(header_w);
                let h_pad = layout::compute_h_pad(
                    total_fetch_w,
                    config.general.padding,
                    config.general.center,
                );

                if h_pad > 0 {
                    print!("{}", " ".repeat(h_pad));
                    let _ = std::io::stdout().flush();
                }

                if let Some(image_lines) = rustfetch_render::render_image(image_path, target_cols) {
                    layout::render_with_image(
                        image_lines,
                        target_cols,
                        h_pad,
                        &info_lines,
                        &render_opts,
                    );
                    return;
                }

                if let Some(lines) = rustfetch_render::render_halfblock(image_path, target_cols) {
                    let logo_block = LogoBlock {
                        lines,
                        width: target_cols,
                    };
                    layout::render(Some(&logo_block), &info_lines, &render_opts);
                    return;
                }
            }
        }
    }

    let logo_block = if show_logo {
        let distro_name = if config.general.logo.distro == "auto" {
            detect_distro_name()
        } else {
            config.general.logo.distro.clone()
        };
        let logo = logos::get_logo(&distro_name);
        Some(LogoBlock {
            lines: logo.colored_lines(),
            width: logo.width,
        })
    } else {
        None
    };

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
        if mc.name == "break" {
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
            mc.icon
                .clone()
                .or_else(|| default_icon_for_module(&mc.name).map(str::to_string))
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
) -> Vec<InfoLine> {
    let results = detect_all(modules);
    results
        .into_iter()
        .zip(configs.iter())
        .filter_map(|(result, mc)| {
            let info = result?;
            let label = if mc.name == "break" {
                String::new()
            } else if mc.name == "custom" {
                mc.label.clone().unwrap_or_default()
            } else {
                mc.label.clone().unwrap_or(info.label)
            };

            let mut value = if mc.name == "break" {
                String::new()
            } else if mc.name == "custom" {
                mc.value.clone().unwrap_or_default()
            } else {
                info.value
            };

            if let Some(fmt) = &mc.format {
                value = fmt.replace("{value}", &value).replace("{}", &value);
            }

            let icon = if config.general.icons && mc.name != "break" {
                mc.icon
                    .clone()
                    .or_else(|| default_icon_for_module(&mc.name).map(str::to_string))
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

            let color_str = dynamic_color
                .or(mc.color.as_deref())
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
    let mut right_lines: Vec<String> = Vec::new();

    if let Some(h) = opts.header {
        right_lines.push(format!("{BOLD}{}{h}{RESET}", color_code("cyan")));
        right_lines.push(format!(
            "{DIM}{}{RESET}",
            "─".repeat(unicode_width::UnicodeWidthStr::width(h))
        ));
    }

    right_lines.extend(layout::format_info_lines(info_lines, opts.separator));

    if let Some(pal) = opts.palette_lines {
        right_lines.push(String::new());
        right_lines.extend(pal.iter().cloned());
    }

    if opts.border {
        right_lines = layout::wrap_in_box(&right_lines);
    }

    right_lines
}

fn run_3d(
    config: &Config,
    cli: &Cli,
    modules: &[Box<dyn Module>],
    configs: &[rustfetch_core::config::ModuleConfig],
) {
    use crossterm::{
        cursor::{Hide, Show},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    };
    use std::time::{Duration, Instant};

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

    let colored_logo_lines = logo.colored_lines();
    let model = rustfetch_render::three_d::Model3D::from_ascii_lines(
        &colored_logo_lines,
        size_scale,
        depth_scale,
        shading_mode,
    );

    let (distro_outer, distro_inner) =
        rustfetch_render::three_d::distro_3d_colors(&distro_name);

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
    let render_opts = layout::RenderOptions {
        header: header.as_deref(),
        separator: &config.general.separator,
        padding: config.general.padding,
        center: config.general.center,
        border: config.general.border,
        palette_lines: palette_lines.as_deref(),
    };

    let mut info_lines = gather_info_lines(config, modules, configs, None);
    let mut right_lines = build_right_lines(&info_lines, &render_opts);

    // RAII guard to guarantee raw mode is disabled and cursor restored on exit or panic
    struct TermGuard;
    impl Drop for TermGuard {
        fn drop(&mut self) {
            let _ = crossterm::terminal::disable_raw_mode();
            let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        }
    }
    let _guard = TermGuard;

    let mut stdout = std::io::stdout();
    let _ = enable_raw_mode();
    let _ = execute!(
        stdout,
        Hide,
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    );
    let _ = stdout.flush();

    let mut angle_x = 0.0f32;
    let mut angle_y = 0.0f32;
    let mut frame_idx = 0usize;
    let mut last_metric_refresh = Instant::now();

    let max_frames = cli.frames.or(config.general.logo.three_d.frames);

    loop {
        if let Some(limit) = max_frames {
            if frame_idx >= limit {
                break;
            }
        }

        // Re-gather dynamic telemetry every 1 second
        if last_metric_refresh.elapsed() >= Duration::from_millis(1000) {
            info_lines = gather_info_lines(config, modules, configs, None);
            right_lines = build_right_lines(&info_lines, &render_opts);
            last_metric_refresh = Instant::now();
        }

        let env_cols = std::env::var("COLUMNS").ok().and_then(|s| s.parse().ok());
        let env_rows = std::env::var("LINES").ok().and_then(|s| s.parse().ok());

        let (term_cols, term_rows) = match (env_cols, env_rows) {
            (Some(c), Some(r)) => (c, r),
            (Some(c), None) => (c, crossterm::terminal::size().map(|(_, r)| r as usize).unwrap_or(24)),
            (None, Some(r)) => (crossterm::terminal::size().map(|(c, _)| c as usize).unwrap_or(80), r),
            (None, None) => crossterm::terminal::size()
                .map(|(c, r)| (c as usize, r as usize))
                .unwrap_or((80, 24)),
        };

        // Compute layout
        let right_width = right_lines
            .iter()
            .map(|l| layout::strip_ansi_width(l))
            .max()
            .unwrap_or(30);

        let target_width = cli
            .width
            .or(config.general.logo.three_d.width)
            .unwrap_or(config.general.logo.image_width_cols)
            .max(30);

        let needed_cols = target_width + layout::LOGO_GAP + right_width;
        let layout_stacked = term_cols < needed_cols;

        let canvas_cols = if layout_stacked {
            target_width.min(term_cols.max(20))
        } else {
            target_width
        };

        let canvas_rows = if layout_stacked {
            let max_stacked_h = (canvas_cols * 3 / 5).clamp(16, 36);
            cli.height
                .or(config.general.logo.three_d.height)
                .unwrap_or(max_stacked_h)
        } else {
            let ideal_h = (canvas_cols * 3 / 5).max(36);
            cli.height
                .or(config.general.logo.three_d.height)
                .unwrap_or(ideal_h.max(right_lines.len()))
        };

        // Render 3D Logo
        let logo_3d = renderer.render_frame(&model, angle_x, angle_y, canvas_cols, canvas_rows);

        // Update rotation
        if rotate_x {
            angle_x += 0.04 * speed;
        }
        if rotate_y {
            angle_y += 0.06 * speed;
        }

        // Compose frame into single buffer with \x1b[H
        let mut frame_buf = String::with_capacity(term_cols * term_rows * 4 + 64);
        frame_buf.push_str("\x1b[H");

        if layout_stacked {
            let left_pad = if config.general.center {
                " ".repeat(term_cols.saturating_sub(canvas_cols) / 2)
            } else {
                " ".repeat(config.general.padding)
            };
            for line in &logo_3d {
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
            for line in &right_lines {
                frame_buf.push_str(&info_pad);
                frame_buf.push_str(line);
                frame_buf.push_str("\x1b[K\r\n");
            }
        } else {
            let total_content_w = canvas_cols + layout::LOGO_GAP + right_width;
            let left_pad = if config.general.center {
                " ".repeat(term_cols.saturating_sub(total_content_w) / 2)
            } else {
                " ".repeat(config.general.padding)
            };
            let total_rows = canvas_rows.max(right_lines.len());
            let gap = " ".repeat(layout::LOGO_GAP);

            for r in 0..total_rows {
                frame_buf.push_str(&left_pad);
                if r < logo_3d.len() {
                    frame_buf.push_str(&logo_3d[r]);
                } else {
                    frame_buf.push_str(&" ".repeat(canvas_cols));
                }
                frame_buf.push_str(&gap);
                if r < right_lines.len() {
                    frame_buf.push_str(&right_lines[r]);
                }
                frame_buf.push_str("\x1b[K\r\n");
            }
        }

        let _ = stdout.write_all(frame_buf.as_bytes());
        let _ = stdout.flush();

        frame_idx += 1;

        // Poll input: only exit on Ctrl+C (0x03) or 'q' / 'Q'.
        // Consume and drain all other typed keys so the 3D logo keeps spinning continuously!
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = std::io::stdin().as_raw_fd();
            let mut pfd = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };
            let ret = unsafe { libc::poll(&mut pfd, 1, 33) };
            if ret > 0 && (pfd.revents & libc::POLLIN) != 0 {
                let mut buf = [0u8; 128];
                let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
                if n > 0 {
                    let bytes = &buf[..n as usize];
                    if bytes.iter().any(|&b| b == 3 || b == b'q' || b == b'Q') {
                        break;
                    }
                }
            }
        }
        #[cfg(not(unix))]
        {
            if let Ok(ready) = crossterm::event::poll(Duration::from_millis(33)) {
                if ready {
                    if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                        if (key.code == crossterm::event::KeyCode::Char('c')
                            && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL))
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

    let _ = disable_raw_mode();
    let _ = execute!(stdout, Show);
    let _ = stdout.flush();
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

    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen, Hide);
    let _ = enable_raw_mode();

    loop {
        let _ = queue!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        run_once(config, cli, modules, configs);
        let _ = stdout.flush();

        if let Ok(ready) = poll(Duration::from_millis(1000)) {
            if ready {
                if let Ok(event) = read() {
                    match event {
                        Event::Key(key) => {
                            if key.code == KeyCode::Char('q')
                                || key.code == KeyCode::Esc
                                || (key.code == KeyCode::Char('c')
                                    && key
                                        .modifiers
                                        .contains(crossterm::event::KeyModifiers::CONTROL))
                            {
                                break;
                            }
                        }
                        Event::Resize(_, _) => {}
                        _ => {}
                    }
                }
            }
        }
    }

    let _ = disable_raw_mode();
    let _ = execute!(stdout, Show, LeaveAlternateScreen);
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
    if std::env::var("KITTY_WINDOW_ID").is_ok() || std::env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        return true;
    }
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("kitty") || term.contains("ghostty") {
            return true;
        }
    }
    if let Ok(prog) = std::env::var("TERM_PROGRAM") {
        let p = prog.to_lowercase();
        if p.contains("ghostty") || p.contains("wezterm") {
            return true;
        }
    }
    false
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
