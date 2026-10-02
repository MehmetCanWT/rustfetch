use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub modules: Vec<ModuleConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct ColorsConfig {
    pub enabled: bool,
    pub symbol: String,
    pub block: bool,
    pub image_palette: bool,
    pub position: String,
    pub rows: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<String>>,
}

impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            symbol: "●".to_string(),
            block: false,
            image_palette: true,
            position: "bottom".to_string(),
            rows: 1,
            custom: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct BorderCharsConfig {
    pub top_left: String,
    pub top: String,
    pub top_right: String,
    pub right: String,
    pub bottom_right: String,
    pub bottom: String,
    pub bottom_left: String,
    pub left: String,
    pub divider_left: String,
    pub divider: String,
    pub divider_right: String,
}

impl Default for BorderCharsConfig {
    fn default() -> Self {
        Self::from_style("rounded")
    }
}

impl BorderCharsConfig {
    pub fn from_style(style: &str) -> Self {
        match style.to_lowercase().as_str() {
            "single" => Self {
                top_left: "┌".to_string(),
                top: "─".to_string(),
                top_right: "┐".to_string(),
                right: "│".to_string(),
                bottom_right: "┘".to_string(),
                bottom: "─".to_string(),
                bottom_left: "└".to_string(),
                left: "│".to_string(),
                divider_left: "├".to_string(),
                divider: "─".to_string(),
                divider_right: "┤".to_string(),
            },
            "double" => Self {
                top_left: "╔".to_string(),
                top: "═".to_string(),
                top_right: "╗".to_string(),
                right: "║".to_string(),
                bottom_right: "╝".to_string(),
                bottom: "═".to_string(),
                bottom_left: "╚".to_string(),
                left: "║".to_string(),
                divider_left: "╠".to_string(),
                divider: "═".to_string(),
                divider_right: "╣".to_string(),
            },
            "thick" | "heavy" => Self {
                top_left: "┏".to_string(),
                top: "━".to_string(),
                top_right: "┓".to_string(),
                right: "┃".to_string(),
                bottom_right: "┛".to_string(),
                bottom: "━".to_string(),
                bottom_left: "┗".to_string(),
                left: "┃".to_string(),
                divider_left: "┣".to_string(),
                divider: "━".to_string(),
                divider_right: "┫".to_string(),
            },
            "dashed" => Self {
                top_left: "┌".to_string(),
                top: "╌".to_string(),
                top_right: "┐".to_string(),
                right: "┆".to_string(),
                bottom_right: "┘".to_string(),
                bottom: "╌".to_string(),
                bottom_left: "└".to_string(),
                left: "┆".to_string(),
                divider_left: "├".to_string(),
                divider: "╌".to_string(),
                divider_right: "┤".to_string(),
            },
            "brackets" => Self {
                top_left: "[".to_string(),
                top: "─".to_string(),
                top_right: "]".to_string(),
                right: "│".to_string(),
                bottom_right: "]".to_string(),
                bottom: "─".to_string(),
                bottom_left: "[".to_string(),
                left: "│".to_string(),
                divider_left: "[".to_string(),
                divider: "─".to_string(),
                divider_right: "]".to_string(),
            },
            "ascii" => Self {
                top_left: "+".to_string(),
                top: "-".to_string(),
                top_right: "+".to_string(),
                right: "|".to_string(),
                bottom_right: "+".to_string(),
                bottom: "-".to_string(),
                bottom_left: "+".to_string(),
                left: "|".to_string(),
                divider_left: "+".to_string(),
                divider: "-".to_string(),
                divider_right: "+".to_string(),
            },
            _ => Self {
                top_left: "╭".to_string(),
                top: "─".to_string(),
                top_right: "╮".to_string(),
                right: "│".to_string(),
                bottom_right: "╯".to_string(),
                bottom: "─".to_string(),
                bottom_left: "╰".to_string(),
                left: "│".to_string(),
                divider_left: "├".to_string(),
                divider: "─".to_string(),
                divider_right: "┤".to_string(),
            },
        }
    }

    pub fn from_str_compact(s: &str) -> Option<Self> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() >= 11 {
            Some(Self {
                top_left: chars[0].to_string(),
                top: chars[1].to_string(),
                top_right: chars[2].to_string(),
                right: chars[3].to_string(),
                bottom_right: chars[4].to_string(),
                bottom: chars[5].to_string(),
                bottom_left: chars[6].to_string(),
                left: chars[7].to_string(),
                divider_left: chars[8].to_string(),
                divider: chars[9].to_string(),
                divider_right: chars[10].to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct ThreeDConfig {
    pub enabled: bool,
    pub speed: f32,
    pub rotate_x: bool,
    pub rotate_y: bool,
    pub size: f32,
    pub depth: f32,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "image_width_cols",
        alias = "cols"
    )]
    pub width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "rows")]
    pub height: Option<usize>,
    pub shading_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shading: Option<String>,
    pub light: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_on_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_color: Option<String>,
}

impl Default for ThreeDConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            speed: 1.0,
            rotate_x: true,
            rotate_y: true,
            size: 1.0,
            depth: 1.0,
            width: Some(42),
            height: Some(24),
            shading_mode: "ascii".to_string(),
            shading: None,
            light: "top-left".to_string(),
            frames: None,
            exit_on_key: Some(true),
            outer_color: None,
            inner_color: None,
        }
    }
}

fn deserialize_three_d<'de, D>(deserializer: D) -> Result<ThreeDConfig, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Helper {
        Bool(bool),
        Config(ThreeDConfig),
    }

    match Helper::deserialize(deserializer)? {
        Helper::Bool(b) => Ok(ThreeDConfig {
            enabled: b,
            ..ThreeDConfig::default()
        }),
        Helper::Config(cfg) => Ok(cfg),
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub separator: String,
    pub padding: usize,
    pub center: bool,
    pub icons: bool,
    pub icon_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    pub border: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_chars: Option<BorderCharsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_title: Option<String>,
    pub box_padding: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_image_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    pub colors: ColorsConfig,
    pub logo: LogoConfig,
    #[serde(alias = "3d", default)]
    pub three_d: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct AnimationConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub fps: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_on_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<usize>,
    pub infinite: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            path: None,
            fps: 15.0,
            exit_on_key: Some(true),
            frames: None,
            infinite: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct LogoConfig {
    pub enabled: bool,
    pub distro: String,
    #[serde(skip_serializing_if = "Option::is_none", alias = "ascii")]
    pub ascii_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<String>>,
    pub image_path: Option<String>,
    pub image_width_cols: usize,
    pub image_dir: Option<String>,
    pub random_image: bool,
    pub auto_color: bool,
    pub protocol: String,
    #[serde(alias = "3d", default, deserialize_with = "deserialize_three_d")]
    pub three_d: ThreeDConfig,
    #[serde(alias = "anim", default)]
    pub animation: AnimationConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ModuleConfig {
    pub name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "text",
        alias = "title",
        alias = "key"
    )]
    pub label: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "logo",
        alias = "symbol",
        alias = "prefix"
    )]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_width: Option<usize>,
}

impl Default for LogoConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            distro: "auto".to_string(),
            ascii_path: None,
            colors: None,
            image_path: None,
            image_width_cols: 42,
            image_dir: None,
            random_image: false,
            auto_color: true,
            protocol: "auto".to_string(),
            three_d: ThreeDConfig::default(),
            animation: AnimationConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            separator: ":".to_string(),
            padding: 1,
            center: false,
            icons: true,
            icon_only: false,
            key_type: None,
            border: false,
            border_style: None,
            border_color: None,
            border_chars: None,
            border_title: None,
            box_padding: 1,
            border_image: None,
            border_image_width: None,
            gradient: None,
            theme: None,
            colors: ColorsConfig::default(),
            logo: LogoConfig::default(),
            three_d: None,
        }
    }
}

impl ModuleConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: None,
            icon: None,
            color: None,
            value: None,
            format: None,
            bar: None,
            bar_width: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn with_bar(mut self, bar: bool) -> Self {
        self.bar = Some(bar);
        self
    }
}

pub fn make_progress_bar(percent: u8, width: usize, colored: bool) -> String {
    let width = if width == 0 { 10 } else { width };
    let percent = percent.min(100);
    let elapsed = ((percent as usize * width) + 50) / 100;
    let remaining = width.saturating_sub(elapsed);

    let color_code = if !colored {
        ""
    } else if percent < 60 {
        "\x1b[32m"
    } else if percent < 85 {
        "\x1b[33m"
    } else {
        "\x1b[31m"
    };

    let reset = if colored { "\x1b[0m" } else { "" };
    format!(
        "{}[{}{}] ({}%){}",
        color_code,
        "■".repeat(elapsed),
        "-".repeat(remaining),
        percent,
        reset
    )
}

pub fn default_icon_for_module(name: &str) -> Option<&'static str> {
    match name {
        "os" => Some("󰌽"),
        "host" => Some("󰌢"),
        "board" => Some("󰘚"),
        "kernel" => Some("󰒋"),
        "uptime" => Some("󱑂"),
        "packages" => Some("󰏖"),
        "shell" => Some("󰞷"),
        "desktop" => Some("󰨇"),
        "font" => Some("󰛖"),
        "terminal" => Some("󰆍"),
        "cpu" => Some("󰍛"),
        "gpu" => Some("󰢮"),
        "memory" => Some("󰘚"),
        "swap" => Some("󰓡"),
        "disk" => Some("󰋊"),
        "battery" => Some("󰂁"),
        "brightness" => Some("󰃠"),
        "cpu_usage" => Some("󰓅"),
        "bluetooth" => Some("󰂯"),
        "local_ip" => Some("󰩟"),
        "locale" => Some("󰗊"),
        "temp" => Some("󰔏"),
        "sound" => Some("󰕾"),
        "media" => Some("󰝚"),
        "display" => Some("󰍹"),
        "wifi" => Some("󰖩"),
        "processes" => Some("󰒋"),
        "custom" => Some("󰅐"),
        _ => None,
    }
}

pub fn distro_icon(distro_name: &str) -> &'static str {
    let lower = distro_name.to_lowercase();
    if lower.contains("arch") {
        "󰣇"
    } else if lower.contains("ubuntu") {
        "󰕈"
    } else if lower.contains("fedora") {
        "󰣛"
    } else if lower.contains("debian") {
        "󰣚"
    } else if lower.contains("nixos") {
        "󱄅"
    } else if lower.contains("gentoo") {
        "󰣨"
    } else if lower.contains("manjaro") {
        "󱘊"
    } else if lower.contains("mint") {
        "󰣭"
    } else if lower.contains("alpine") {
        "󰣠"
    } else if lower.contains("suse") {
        "󰣡"
    } else if lower.contains("kali") {
        "󰣳"
    } else if lower.contains("pop") {
        "󰣩"
    } else if lower.contains("endeavour") {
        "󰣫"
    } else if lower.contains("artix") {
        "󰣢"
    } else if lower.contains("void") {
        "󰣲"
    } else if lower.contains("cachy") {
        "󰣇"
    } else if lower.contains("darwin") || lower.contains("macos") || lower.contains("apple") {
        "󰀵"
    } else if lower.contains("windows") {
        "󰍲"
    } else if lower.contains("android") {
        "󰀲"
    } else if lower.contains("freebsd") || lower.contains("bsd") {
        "󰣶"
    } else {
        "󰌽"
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            modules: vec![
                ModuleConfig::new("os"),
                ModuleConfig::new("host"),
                ModuleConfig::new("board"),
                ModuleConfig::new("kernel"),
                ModuleConfig::new("uptime"),
                ModuleConfig::new("packages"),
                ModuleConfig::new("shell"),
                ModuleConfig::new("desktop"),
                ModuleConfig::new("font"),
                ModuleConfig::new("terminal"),
                ModuleConfig::new("cpu"),
                ModuleConfig::new("gpu"),
                ModuleConfig::new("memory"),
                ModuleConfig::new("swap"),
                ModuleConfig::new("disk"),
                ModuleConfig::new("battery"),
                ModuleConfig::new("local_ip"),
                ModuleConfig::new("locale"),
            ],
        }
    }
}

impl Config {
    pub fn from_preset(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "minimal" => {
                let mut c = Config::default();
                c.general.icons = false;
                c.general.separator = " :".to_string();
                c.general.colors.enabled = false;
                c.general.logo.enabled = false;
                c.modules = vec![
                    ModuleConfig::new("os"),
                    ModuleConfig::new("kernel"),
                    ModuleConfig::new("uptime"),
                    ModuleConfig::new("shell"),
                    ModuleConfig::new("cpu"),
                    ModuleConfig::new("memory"),
                    ModuleConfig::new("disk"),
                ];
                Some(c)
            }
            "clean" => {
                let mut c = Config::default();
                c.general.icon_only = true;
                c.general.separator = "".to_string();
                c.general.padding = 1;
                c.general.colors.enabled = false;
                c.modules = vec![
                    ModuleConfig::new("os"),
                    ModuleConfig::new("kernel"),
                    ModuleConfig::new("desktop"),
                    ModuleConfig::new("shell"),
                    ModuleConfig::new("terminal"),
                    ModuleConfig::new("cpu"),
                    ModuleConfig::new("gpu"),
                    ModuleConfig::new("memory"),
                    ModuleConfig::new("uptime"),
                ];
                Some(c)
            }
            "dots" => {
                let mut c = Config::default();
                c.general.icon_only = true;
                c.general.separator = "•".to_string();
                c.general.padding = 1;
                c.general.colors.enabled = true;
                c.general.colors.symbol = "●".to_string();
                c.modules = vec![
                    ModuleConfig::new("os"),
                    ModuleConfig::new("kernel"),
                    ModuleConfig::new("uptime"),
                    ModuleConfig::new("shell"),
                    ModuleConfig::new("terminal"),
                    ModuleConfig::new("cpu"),
                    ModuleConfig::new("gpu"),
                    ModuleConfig::new("memory"),
                    ModuleConfig::new("disk"),
                ];
                Some(c)
            }
            "card" | "modern-card" => {
                let mut c = Config::default();
                c.general.border = true;
                c.general.border_style = Some("rounded".to_string());
                c.general.padding = 2;
                c.general.center = true;
                c.general.colors.enabled = true;
                c.general.colors.position = "bottom".to_string();
                c.modules = vec![
                    ModuleConfig::new("os"),
                    ModuleConfig::new("host"),
                    ModuleConfig::new("kernel"),
                    ModuleConfig::new("uptime"),
                    ModuleConfig::new("break"),
                    ModuleConfig::new("shell"),
                    ModuleConfig::new("terminal"),
                    ModuleConfig::new("desktop"),
                    ModuleConfig::new("break"),
                    ModuleConfig::new("cpu"),
                    ModuleConfig::new("gpu"),
                    ModuleConfig::new("memory"),
                    ModuleConfig::new("disk"),
                ];
                Some(c)
            }
            "brackets" => {
                let mut c = Config::default();
                c.general.border = true;
                c.general.border_style = Some("brackets".to_string());
                c.general.padding = 2;
                c.general.center = true;
                Some(c)
            }
            "neofetch" => {
                let mut c = Config::default();
                c.general.separator = ":".to_string();
                c.general.padding = 1;
                c.general.colors.enabled = true;
                c.general.colors.block = true;
                c.general.colors.rows = 2;
                c.general.colors.position = "bottom".to_string();
                Some(c)
            }
            "retro" => {
                let mut c = Config::default();
                c.general.border = true;
                c.general.border_style = Some("rounded".to_string());
                c.general.colors.symbol = "󰮯".to_string();
                for m in &mut c.modules {
                    if matches!(m.name.as_str(), "memory" | "disk" | "battery") {
                        m.bar = Some(true);
                        m.bar_width = Some(10);
                    }
                }
                Some(c)
            }
            "modern" => {
                let mut c = Config::default();
                c.general.center = true;
                c.general.separator = " ->".to_string();
                c.general.colors.block = true;
                for m in &mut c.modules {
                    if matches!(m.name.as_str(), "memory" | "swap" | "disk" | "battery") {
                        m.bar = Some(true);
                        m.bar_width = Some(10);
                    }
                }
                Some(c)
            }
            "compact" => {
                let mut c = Config::default();
                c.general.padding = 1;
                c.general.colors.enabled = false;
                c.modules = vec![
                    ModuleConfig::new("os"),
                    ModuleConfig::new("host"),
                    ModuleConfig::new("kernel"),
                    ModuleConfig::new("uptime"),
                    ModuleConfig::new("shell"),
                    ModuleConfig::new("cpu"),
                    ModuleConfig::new("memory"),
                ];
                Some(c)
            }
            "default" => Some(Config::default()),
            _ => None,
        }
    }

    pub fn config_path() -> PathBuf {
        Self::config_path_from_env(|var| std::env::var(var).ok())
    }

    fn config_path_from_env(get_env: impl Fn(&str) -> Option<String>) -> PathBuf {
        if let Some(xdg) = get_env("XDG_CONFIG_HOME") {
            let trimmed = xdg.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed).join("rustfetch").join("config.toml");
            }
        }
        if let Some(home) = get_env("HOME") {
            let trimmed = home.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed)
                    .join(".config")
                    .join("rustfetch")
                    .join("config.toml");
            }
        }
        PathBuf::from("~/.config/rustfetch/config.toml")
    }

    pub fn find_fastfetch_config() -> Option<PathBuf> {
        let check_dir = |dir: PathBuf| {
            let jsonc = dir.join("fastfetch").join("config.jsonc");
            if jsonc.is_file() {
                return Some(jsonc);
            }
            let json = dir.join("fastfetch").join("config.json");
            if json.is_file() {
                return Some(json);
            }
            None
        };

        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            let trimmed = xdg.trim();
            if !trimmed.is_empty() {
                if let Some(p) = check_dir(PathBuf::from(trimmed)) {
                    return Some(p);
                }
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            let trimmed = home.trim();
            if !trimmed.is_empty() {
                if let Some(p) = check_dir(PathBuf::from(trimmed).join(".config")) {
                    return Some(p);
                }
            }
        }
        None
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if !path.exists() {
            if let Some(ff_path) = Self::find_fastfetch_config() {
                if let Ok(content) = std::fs::read_to_string(&ff_path) {
                    if let Some(imported) = Self::import_from_fastfetch(&content) {
                        if let Ok(toml_str) = toml::to_string_pretty(&imported) {
                            if let Some(parent) = path.parent() {
                                let _ = std::fs::create_dir_all(parent);
                            }
                            let _ = std::fs::write(&path, &toml_str);
                        }
                        return imported;
                    }
                }
            }
            let default_cfg = Self::default();
            if let Ok(toml_str) = toml::to_string_pretty(&default_cfg) {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&path, &toml_str);
            }
            return default_cfg;
        }
        Self::load_from(&path).unwrap_or_else(|err| {
            eprintln!("{err}");
            Self::default()
        })
    }

    pub fn load_from(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file '{}': {}", path.display(), e))?;
        Self::from_toml(&content)
            .map_err(|e| format!("Failed to parse config file '{}': {}", path.display(), e))
    }

    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    pub fn to_default_toml() -> String {
        toml::to_string_pretty(&Self::default())
            .expect("default config must be serializable to TOML")
    }

    pub fn to_toml_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    pub fn import_from_fastfetch(content: &str) -> Option<Self> {
        let stripped = strip_jsonc(content);
        let val: serde_json::Value = serde_json::from_str(&stripped).ok()?;

        let mut config = Self::default();

        if let Some(sep) = val.pointer("/display/separator").and_then(|s| s.as_str()) {
            config.general.separator = sep.trim_end().to_string();
        }

        if let Some(pad) = val
            .pointer("/display/key/paddingLeft")
            .and_then(|p| p.as_u64())
        {
            config.general.padding = pad as usize;
        }

        let mut center_detected = false;
        if let Some(logo) = val.get("logo").and_then(|l| l.as_object()) {
            if let Some(ltype) = logo.get("type").and_then(|t| t.as_str()) {
                if ltype == "none" {
                    config.general.logo.enabled = false;
                }
            }

            if let Some(pad_left) = logo
                .get("padding")
                .and_then(|p| p.get("left"))
                .and_then(|v| v.as_i64())
            {
                if pad_left > 0 {
                    center_detected = true;
                }
            }

            if let Some(width) = logo.get("width").and_then(|w| w.as_u64()) {
                if (15..=60).contains(&width) {
                    config.general.logo.image_width_cols = width.min(45) as usize;
                }
            }

            if let Some(src) = logo.get("source").and_then(|s| s.as_str()) {
                if !src.is_empty() {
                    let path = PathBuf::from(src);
                    if let Some(parent) = path.parent() {
                        let parent_str = parent.to_string_lossy();
                        if parent_str.ends_with("/logos") || parent_str.contains("/logos/") {
                            config.general.logo.image_dir = Some(parent_str.to_string());
                            config.general.logo.random_image = true;
                            config.general.logo.auto_color = true;
                        } else if path.is_dir() {
                            config.general.logo.image_dir = Some(src.to_string());
                            config.general.logo.random_image = true;
                            config.general.logo.auto_color = true;
                        } else {
                            config.general.logo.image_path = Some(src.to_string());
                            config.general.logo.auto_color = true;
                        }
                    } else {
                        config.general.logo.image_path = Some(src.to_string());
                    }
                }
            }
        }

        if config.general.logo.image_dir.is_none() && config.general.logo.image_path.is_none() {
            if let Ok(home) = std::env::var("HOME") {
                let ff_logos = PathBuf::from(home).join(".config/fastfetch/logos");
                if ff_logos.is_dir() {
                    config.general.logo.image_dir = Some("~/.config/fastfetch/logos".to_string());
                    config.general.logo.random_image = true;
                    config.general.logo.auto_color = true;
                }
            }
        }

        if let Some(modules_arr) = val.get("modules").and_then(|m| m.as_array()) {
            let mut converted = Vec::new();
            let mut seen = std::collections::HashSet::new();

            for item in modules_arr {
                if let Some(mod_obj) = item.as_object() {
                    let mod_type = mod_obj.get("type").and_then(|t| t.as_str()).unwrap_or("");
                    if mod_type == "custom" {
                        if let Some(fmt) = mod_obj.get("format").and_then(|f| f.as_str()) {
                            if fmt.contains('\n') {
                                center_detected = true;
                            }
                        }
                        continue;
                    }

                    if let Some(mapped_name) = map_fastfetch_type(mod_type) {
                        if seen.insert(mapped_name) {
                            let mut mc = ModuleConfig::new(mapped_name);
                            if let Some(icon) = mod_obj.get("keyIcon").and_then(|i| i.as_str()) {
                                if !icon.trim().is_empty() {
                                    mc.icon = Some(icon.to_string());
                                }
                            }
                            if let Some(key) = mod_obj.get("key").and_then(|k| k.as_str()) {
                                if !key.trim().is_empty() {
                                    mc.label = Some(key.to_string());
                                }
                            }
                            converted.push(mc);
                        }
                    }
                } else if let Some(mod_name) = item.as_str() {
                    if let Some(mapped_name) = map_fastfetch_type(mod_name) {
                        if seen.insert(mapped_name) {
                            converted.push(ModuleConfig::new(mapped_name));
                        }
                    }
                }
            }

            if !converted.is_empty() {
                config.modules = converted;
            }
        }

        config.general.center = center_detected;

        Some(config)
    }
}

fn map_fastfetch_type(ff_type: &str) -> Option<&'static str> {
    match ff_type {
        "os" => Some("os"),
        "host" => Some("host"),
        "board" => Some("board"),
        "kernel" => Some("kernel"),
        "uptime" => Some("uptime"),
        "packages" => Some("packages"),
        "shell" => Some("shell"),
        "de" | "wm" => Some("desktop"),
        "font" | "terminalfont" => Some("font"),
        "terminal" => Some("terminal"),
        "cpu" => Some("cpu"),
        "gpu" => Some("gpu"),
        "memory" => Some("memory"),
        "swap" => Some("swap"),
        "disk" => Some("disk"),
        "battery" | "poweradapter" => Some("battery"),
        "localip" | "local_ip" => Some("local_ip"),
        "locale" => Some("locale"),
        _ => None,
    }
}

pub fn strip_jsonc(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(n);
    let mut i = 0;
    let mut in_str = false;
    let mut escape = false;

    while i < n {
        let c = chars[i];
        if in_str {
            out.push(c);
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_str = false;
            }
        } else if c == '"' {
            in_str = true;
            out.push(c);
        } else if c == '/' && i + 1 < n {
            if chars[i + 1] == '/' {
                i += 2;
                while i < n && chars[i] != '\n' {
                    i += 1;
                }
                if i < n {
                    out.push('\n');
                }
            } else if chars[i + 1] == '*' {
                i += 2;
                while i + 1 < n && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                i += 1;
            } else {
                out.push(c);
            }
        } else {
            out.push(c);
        }
        i += 1;
    }

    let out_chars: Vec<char> = out.chars().collect();
    let len = out_chars.len();
    let mut final_out = String::with_capacity(len);
    let mut j = 0;
    in_str = false;
    escape = false;

    while j < len {
        let c = out_chars[j];
        if in_str {
            final_out.push(c);
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_str = false;
            }
        } else if c == '"' {
            in_str = true;
            final_out.push(c);
        } else if c == ',' {
            let mut k = j + 1;
            while k < len && out_chars[k].is_whitespace() {
                k += 1;
            }
            if k < len && (out_chars[k] == '}' || out_chars[k] == ']') {
                // skip trailing comma
            } else {
                final_out.push(c);
            }
        } else {
            final_out.push(c);
        }
        j += 1;
    }

    final_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_all_modules() {
        let config = Config::default();
        assert_eq!(config.modules.len(), 18);

        let names: Vec<&str> = config.modules.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "os", "host", "board", "kernel", "uptime", "packages", "shell", "desktop", "font",
                "terminal", "cpu", "gpu", "memory", "swap", "disk", "battery", "local_ip",
                "locale"
            ]
        );

        assert_eq!(config.general.separator, ":");
        assert_eq!(config.general.padding, 1);
        assert!(config.general.logo.enabled);
        assert_eq!(config.general.logo.distro, "auto");

        for m in &config.modules {
            assert!(m.label.is_none());
            assert!(m.icon.is_none());
            assert!(m.color.is_none());
        }
    }

    #[test]
    fn test_parse_toml_string() {
        let toml_str = r#"
[general]
separator = " ->"
padding = 4

[general.logo]
enabled = false
distro = "fedora"

[[modules]]
name = "os"
label = "Distro"
icon = ""
color = "blue"

[[modules]]
name = "cpu"
icon = "󰍛"
"#;

        let config: Config = Config::from_toml(toml_str).expect("failed to parse valid TOML");
        assert_eq!(config.general.separator, " ->");
        assert_eq!(config.general.padding, 4);
        assert!(!config.general.logo.enabled);
        assert_eq!(config.general.logo.distro, "fedora");

        assert_eq!(config.modules.len(), 2);
        assert_eq!(config.modules[0].name, "os");
        assert_eq!(config.modules[0].label.as_deref(), Some("Distro"));
        assert_eq!(config.modules[0].icon.as_deref(), Some(""));
        assert_eq!(config.modules[0].color.as_deref(), Some("blue"));

        assert_eq!(config.modules[1].name, "cpu");
        assert_eq!(config.modules[1].label, None);
        assert_eq!(config.modules[1].icon.as_deref(), Some("󰍛"));
        assert_eq!(config.modules[1].color, None);
    }

    #[test]
    fn test_parse_partial_toml_uses_defaults() {
        let toml_str = r#"
[general]
separator = " >"
"#;
        let config: Config = Config::from_toml(toml_str).expect("failed to parse partial TOML");
        assert_eq!(config.general.separator, " >");
        assert_eq!(config.general.padding, 1);
        assert!(config.general.logo.enabled);
        assert_eq!(config.general.logo.distro, "auto");
        assert_eq!(config.modules.len(), 18);
        assert_eq!(config.modules[0].name, "os");
    }

    #[test]
    fn test_invalid_toml_gives_error() {
        let bad_syntax = "general = [unclosed bracket";
        let err = Config::from_toml(bad_syntax);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(
            err_msg.contains("line") || err_msg.contains("error"),
            "Error message should mention location or error: {err_msg}"
        );

        let bad_type = r#"
[general]
separator = 12345
"#;
        let err2 = Config::from_toml(bad_type);
        assert!(err2.is_err());
        let err2_msg = err2.unwrap_err().to_string();
        assert!(
            err2_msg.contains("invalid type") || err2_msg.contains("expected a string"),
            "Error message should explain what was expected: {err2_msg}"
        );
    }

    #[test]
    fn test_to_default_toml_round_trips() {
        let default_config = Config::default();
        let toml_str = Config::to_default_toml();

        assert!(!toml_str.is_empty());
        assert!(toml_str.contains("separator = \":\""));
        assert!(toml_str.contains("padding = 1"));
        assert!(toml_str.contains("enabled = true"));
        assert!(toml_str.contains("distro = \"auto\""));
        assert!(toml_str.contains("name = \"os\""));
        assert!(toml_str.contains("name = \"kernel\""));

        let deserialized: Config =
            Config::from_toml(&toml_str).expect("round-trip deserialization failed");
        assert_eq!(deserialized, default_config);
    }

    #[test]
    fn test_config_path_precedence() {
        let path1 = Config::config_path_from_env(|var| match var {
            "XDG_CONFIG_HOME" => Some("/custom/config".into()),
            "HOME" => Some("/home/user".into()),
            _ => None,
        });
        assert_eq!(path1, PathBuf::from("/custom/config/rustfetch/config.toml"));

        let path2 = Config::config_path_from_env(|var| match var {
            "XDG_CONFIG_HOME" => Some("   ".into()),
            "HOME" => Some("/home/user".into()),
            _ => None,
        });
        assert_eq!(
            path2,
            PathBuf::from("/home/user/.config/rustfetch/config.toml")
        );

        let path3 = Config::config_path_from_env(|var| match var {
            "XDG_CONFIG_HOME" => None,
            "HOME" => Some("/home/user".into()),
            _ => None,
        });
        assert_eq!(
            path3,
            PathBuf::from("/home/user/.config/rustfetch/config.toml")
        );

        let path4 = Config::config_path_from_env(|_| None);
        assert_eq!(path4, PathBuf::from("~/.config/rustfetch/config.toml"));
    }

    #[test]
    fn test_load_from_file_and_errors() {
        let temp_dir = std::env::temp_dir().join(format!("rustfetch_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);

        let valid_file = temp_dir.join("valid.toml");
        std::fs::write(
            &valid_file,
            "[general]\npadding = 3\nseparator = \" -> \"\n",
        )
        .expect("write file");

        let loaded = Config::load_from(&valid_file).expect("load_from valid file");
        assert_eq!(loaded.general.padding, 3);
        assert_eq!(loaded.general.separator, " -> ");

        let invalid_file = temp_dir.join("invalid.toml");
        std::fs::write(&invalid_file, "[general]\npadding = \"not_a_number\"\n")
            .expect("write file");

        let err = Config::load_from(&invalid_file).unwrap_err();
        assert!(err.contains("Failed to parse config file"));
        assert!(err.contains("line"));

        let nonexistent = temp_dir.join("nonexistent.toml");
        let not_found_err = Config::load_from(&nonexistent).unwrap_err();
        assert!(not_found_err.contains("Failed to read config file"));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_strip_jsonc_comments_and_trailing_commas() {
        let input = r#"{
            // This is a line comment
            "name": "https://example.com/test//not_comment",
            /* This is a
               block comment */
            "items": [
                1,
                2,
                3, // trailing comma here
            ],
            "enabled": true,
        }"#;

        let cleaned = strip_jsonc(input);
        assert!(!cleaned.contains("This is a line comment"));
        assert!(!cleaned.contains("block comment"));
        assert!(cleaned.contains("https://example.com/test//not_comment"));

        let parsed: serde_json::Value =
            serde_json::from_str(&cleaned).expect("should parse valid json");
        assert_eq!(parsed["name"], "https://example.com/test//not_comment");
        assert_eq!(parsed["items"].as_array().unwrap().len(), 3);
        assert_eq!(parsed["enabled"], true);
    }

    #[test]
    fn test_import_from_fastfetch() {
        let ff_jsonc = r#"{
            "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/master/doc/json_schema.json",
            "logo": {
                "type": "kitty",
                "source": "~/.config/fastfetch/logos/current.png",
                "width": 60,
                "padding": {
                    "left": 47
                }
            },
            "display": {
                "separator": ": "
            },
            "modules": [
                {
                    "type": "custom",
                    "format": "\n\n\n\n\n\n"
                },
                {
                    "type": "title",
                    "keyIcon": ""
                },
                {
                    "type": "separator"
                },
                {
                    "type": "os",
                    "keyIcon": ""
                },
                {
                    "type": "cpu",
                    "keyIcon": ""
                },
                {
                    "type": "de",
                    "keyIcon": ""
                },
                {
                    "type": "wm",
                    "keyIcon": ""
                },
                {
                    "type": "localip",
                    "keyIcon": "󰩟"
                }
            ]
        }"#;

        let config =
            Config::import_from_fastfetch(ff_jsonc).expect("should import fastfetch config");
        assert_eq!(config.general.separator, ":");
        assert!(config.general.center);
        assert!(config.general.logo.random_image);
        assert_eq!(
            config.general.logo.image_dir.as_deref(),
            Some("~/.config/fastfetch/logos")
        );

        let mod_names: Vec<&str> = config.modules.iter().map(|m| m.name.as_str()).collect();
        // de and wm should be deduplicated to a single desktop module
        assert_eq!(mod_names, vec!["os", "cpu", "desktop", "local_ip"]);

        let os_mod = config.modules.iter().find(|m| m.name == "os").unwrap();
        assert_eq!(os_mod.icon.as_deref(), Some(""));

        let cpu_mod = config.modules.iter().find(|m| m.name == "cpu").unwrap();
        assert_eq!(cpu_mod.icon.as_deref(), Some(""));

        let desktop_mod = config.modules.iter().find(|m| m.name == "desktop").unwrap();
        assert_eq!(desktop_mod.icon.as_deref(), Some(""));

        let local_ip_mod = config
            .modules
            .iter()
            .find(|m| m.name == "local_ip")
            .unwrap();
        assert_eq!(local_ip_mod.icon.as_deref(), Some("󰩟"));
    }

    #[test]
    fn test_make_progress_bar() {
        let bar = make_progress_bar(50, 10, false);
        assert_eq!(bar, "[■■■■■-----] (50%)");

        let bar_full = make_progress_bar(100, 10, false);
        assert_eq!(bar_full, "[■■■■■■■■■■] (100%)");

        let bar_empty = make_progress_bar(0, 10, false);
        assert_eq!(bar_empty, "[----------] (0%)");
    }

    #[test]
    fn test_from_preset() {
        let minimal = Config::from_preset("minimal").unwrap();
        assert!(!minimal.general.icons);
        assert!(!minimal.general.logo.enabled);

        let card = Config::from_preset("card").unwrap();
        assert!(card.general.border);

        let modern = Config::from_preset("modern").unwrap();
        assert!(modern.general.colors.block);

        let compact = Config::from_preset("compact").unwrap();
        assert_eq!(compact.modules.len(), 7);

        assert!(Config::from_preset("nonexistent").is_none());
    }

    #[test]
    fn test_module_config_text_and_logo_aliases() {
        let toml_str = r#"
        [[modules]]
        name = "os"
        text = "System"
        logo = ">_"

        [[modules]]
        name = "cpu"
        title = "Processor"
        symbol = ""
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.modules[0].label.as_deref(), Some("System"));
        assert_eq!(config.modules[0].icon.as_deref(), Some(">_"));
        assert_eq!(config.modules[1].label.as_deref(), Some("Processor"));
        assert_eq!(config.modules[1].icon.as_deref(), Some(""));
    }

    #[test]
    fn test_three_d_config_boolean_and_table() {
        let toml_bool = r#"
        [general.logo]
        3d = true
        "#;
        let cfg1: Config = toml::from_str(toml_bool).unwrap();
        assert!(cfg1.general.logo.three_d.enabled);

        let toml_table = r##"
        [general.logo.three_d]
        enabled = true
        speed = 2.0
        width = 60
        outer_color = "#3b82f6"
        "##;
        let cfg2: Config = toml::from_str(toml_table).unwrap();
        assert!(cfg2.general.logo.three_d.enabled);
        assert_eq!(cfg2.general.logo.three_d.speed, 2.0);
        assert_eq!(cfg2.general.logo.three_d.width, Some(60));
        assert_eq!(
            cfg2.general.logo.three_d.outer_color.as_deref(),
            Some("#3b82f6")
        );

        let toml_gen = r#"
        [general]
        3d = true
        "#;
        let cfg3: Config = toml::from_str(toml_gen).unwrap();
        assert_eq!(cfg3.general.three_d, Some(true));
    }
}
