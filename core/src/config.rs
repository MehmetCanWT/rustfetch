use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub modules: Vec<ModuleConfig>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct ConfigRaw {
    general: Option<GeneralConfig>,
    modules: Option<Vec<ModuleConfig>>,
    border: Option<BorderFieldHelper>,
    border_image: Option<BorderImageFieldHelper>,
    border_images: Option<Vec<BorderImageConfig>>,
    colors: Option<ColorsConfig>,
    logo: Option<LogoConfig>,
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = ConfigRaw::deserialize(deserializer)?;
        let mut general = raw.general.unwrap_or_default();

        if let Some(b) = raw.border {
            match b {
                BorderFieldHelper::Bool(en) => general.border = en,
                BorderFieldHelper::Table(tbl) => apply_border_helper(&mut general, *tbl),
            }
        }

        if let Some(bif) = raw.border_image {
            apply_border_image_helper(&mut general, bif);
        }

        if let Some(imgs) = raw.border_images {
            general.border_images = imgs;
        }
        if let Some(cols) = raw.colors {
            general.colors = cols;
        }
        if let Some(l) = raw.logo {
            general.logo = l;
        }

        let modules = raw.modules.unwrap_or_else(|| Config::default().modules);

        Ok(Config { general, modules })
    }
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
        let (tl, t, tr, r, br, b, bl, l, dl, d, dr) = match style.to_lowercase().as_str() {
            "single" | "light" => ("┌", "─", "┐", "│", "┘", "─", "└", "│", "├", "─", "┤"),
            "double" => ("╔", "═", "╗", "║", "╝", "═", "╚", "║", "╠", "═", "╣"),
            "thick" | "heavy" => ("┏", "━", "┓", "┃", "┛", "━", "┗", "┃", "┣", "━", "┫"),
            "dashed" | "dots" => ("┌", "╌", "┐", "┆", "┘", "╌", "└", "┆", "├", "╌", "┤"),
            "brackets" => ("[", "─", "]", "│", "]", "─", "[", "│", "[", "─", "]"),
            "ascii" => ("+", "-", "+", "|", "+", "-", "+", "|", "+", "-", "+"),
            "block" => ("█", "▀", "█", "█", "█", "▄", "█", "█", "█", "▀", "█"),
            _ => ("╭", "─", "╮", "│", "╯", "─", "╰", "│", "├", "─", "┤"),
        };
        Self {
            top_left: tl.to_string(),
            top: t.to_string(),
            top_right: tr.to_string(),
            right: r.to_string(),
            bottom_right: br.to_string(),
            bottom: b.to_string(),
            bottom_left: bl.to_string(),
            left: l.to_string(),
            divider_left: dl.to_string(),
            divider: d.to_string(),
            divider_right: dr.to_string(),
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
    pub infinite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<bool>,
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
            infinite: None,
            hold: None,
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GeneralConfig {
    pub separator: String,
    pub padding: usize,
    pub center: bool,
    pub icons: bool,
    pub icon_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    pub live: bool,
    #[serde(skip_serializing_if = "Option::is_none", alias = "clear")]
    pub clear_screen: Option<bool>,
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
    pub border_image_height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_image_offset_x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_image_offset_y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_image_z_index: Option<i32>,
    pub border_image_position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_image_protocol: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "border_image_preserveAspectRatio"
    )]
    pub border_image_preserve_aspect_ratio: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "border_image_textOnTop",
        alias = "border_image_text_over"
    )]
    pub border_image_text_on_top: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub border_images: Vec<BorderImageConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    pub colors: ColorsConfig,
    pub logo: LogoConfig,
    #[serde(alias = "3d", default)]
    pub three_d: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct BorderImageConfig {
    pub path: String,
    pub position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<i32>,
    pub z_index: i32,
    pub protocol: String,
    #[serde(default = "default_true", alias = "preserveAspectRatio")]
    pub preserve_aspect_ratio: bool,
    #[serde(
        default = "default_true",
        alias = "text_over",
        alias = "textOnTop",
        alias = "textOver"
    )]
    pub text_on_top: bool,
}

fn default_true() -> bool {
    true
}

impl Default for BorderImageConfig {
    fn default() -> Self {
        Self {
            path: String::new(),
            position: "behind".to_string(),
            width: None,
            height: None,
            offset_x: None,
            offset_y: None,
            z_index: -1,
            protocol: "auto".to_string(),
            preserve_aspect_ratio: true,
            text_on_top: true,
        }
    }
}

impl BorderImageConfig {
    pub fn is_background(&self) -> bool {
        let pos = self.position.trim().to_ascii_lowercase();
        pos == "background" || pos == "behind" || pos == "bg"
    }

    pub fn effective_z_index(&self) -> i32 {
        if self.text_on_top {
            -1
        } else if self.z_index <= 0 {
            1
        } else {
            self.z_index
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(default)]
pub struct BorderSectionHelper {
    #[serde(alias = "border")]
    pub enabled: Option<bool>,
    #[serde(alias = "border_style")]
    pub style: Option<String>,
    #[serde(alias = "border_color")]
    pub color: Option<String>,
    #[serde(alias = "border_title")]
    pub title: Option<String>,
    #[serde(alias = "box_padding")]
    pub padding: Option<usize>,
    #[serde(alias = "border_chars")]
    pub chars: Option<BorderCharsConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum BorderFieldHelper {
    Bool(bool),
    Table(Box<BorderSectionHelper>),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(default)]
pub struct BorderImageSectionHelper {
    #[serde(alias = "border_image")]
    pub path: Option<String>,
    #[serde(alias = "border_image_position")]
    pub position: Option<String>,
    #[serde(alias = "border_image_width")]
    pub width: Option<usize>,
    #[serde(alias = "border_image_height")]
    pub height: Option<usize>,
    #[serde(alias = "border_image_offset_x")]
    pub offset_x: Option<i32>,
    #[serde(alias = "border_image_offset_y")]
    pub offset_y: Option<i32>,
    #[serde(alias = "border_image_z_index")]
    pub z_index: Option<i32>,
    #[serde(alias = "border_image_protocol")]
    pub protocol: Option<String>,
    #[serde(
        alias = "border_image_preserve_aspect_ratio",
        alias = "preserve_aspect_ratio",
        alias = "preserveAspectRatio"
    )]
    pub preserve_aspect_ratio: Option<bool>,
    #[serde(
        alias = "border_image_text_on_top",
        alias = "text_on_top",
        alias = "text_over",
        alias = "textOnTop",
        alias = "textOver"
    )]
    pub text_on_top: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum BorderImageFieldHelper {
    Path(String),
    Table(BorderImageSectionHelper),
}

pub fn apply_border_helper(cfg: &mut GeneralConfig, b: BorderSectionHelper) {
    if let Some(en) = b.enabled {
        cfg.border = en;
    }
    if let Some(s) = b.style {
        cfg.border_style = Some(s);
    }
    if let Some(c) = b.color {
        cfg.border_color = Some(c);
    }
    if let Some(t) = b.title {
        cfg.border_title = Some(t);
    }
    if let Some(p) = b.padding {
        cfg.box_padding = p;
    }
    if let Some(ch) = b.chars {
        cfg.border_chars = Some(ch);
    }
}

pub fn apply_border_image_helper(cfg: &mut GeneralConfig, bif: BorderImageFieldHelper) {
    match bif {
        BorderImageFieldHelper::Path(p) => {
            if !p.trim().is_empty() {
                cfg.border_image = Some(p);
            }
        }
        BorderImageFieldHelper::Table(tbl) => {
            if let Some(p) = tbl.path {
                if !p.trim().is_empty() {
                    cfg.border_image = Some(p);
                }
            }
            if let Some(pos) = tbl.position {
                cfg.border_image_position = pos;
            }
            if let Some(w) = tbl.width {
                cfg.border_image_width = Some(w);
            }
            if let Some(h) = tbl.height {
                cfg.border_image_height = Some(h);
            }
            if let Some(ox) = tbl.offset_x {
                cfg.border_image_offset_x = Some(ox);
            }
            if let Some(oy) = tbl.offset_y {
                cfg.border_image_offset_y = Some(oy);
            }
            if let Some(zi) = tbl.z_index {
                cfg.border_image_z_index = Some(zi);
            }
            if let Some(pr) = tbl.protocol {
                cfg.border_image_protocol = Some(pr);
            }
            if let Some(par) = tbl.preserve_aspect_ratio {
                cfg.border_image_preserve_aspect_ratio = Some(par);
            }
            if let Some(tot) = tbl.text_on_top {
                cfg.border_image_text_on_top = Some(tot);
            }
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct GeneralConfigRaw {
    separator: Option<String>,
    padding: Option<usize>,
    center: Option<bool>,
    icons: Option<bool>,
    icon_only: Option<bool>,
    key_type: Option<String>,
    live: Option<bool>,
    #[serde(alias = "clear")]
    clear_screen: Option<bool>,

    border: Option<BorderFieldHelper>,
    border_style: Option<String>,
    border_color: Option<String>,
    border_chars: Option<BorderCharsConfig>,
    border_title: Option<String>,
    box_padding: Option<usize>,

    border_image: Option<BorderImageFieldHelper>,
    border_image_width: Option<usize>,
    border_image_height: Option<usize>,
    border_image_offset_x: Option<i32>,
    border_image_offset_y: Option<i32>,
    border_image_z_index: Option<i32>,
    border_image_position: Option<String>,
    border_image_protocol: Option<String>,
    #[serde(
        alias = "border_image_preserveAspectRatio",
        alias = "border_image_preserve_aspect_ratio"
    )]
    border_image_preserve_aspect_ratio: Option<bool>,
    #[serde(
        alias = "border_image_textOnTop",
        alias = "border_image_text_on_top",
        alias = "border_image_text_over",
        alias = "border_image_textOver"
    )]
    border_image_text_on_top: Option<bool>,

    border_images: Option<Vec<BorderImageConfig>>,
    gradient: Option<Vec<String>>,
    theme: Option<String>,
    colors: Option<ColorsConfig>,
    logo: Option<LogoConfig>,
    #[serde(alias = "3d")]
    three_d: Option<bool>,
}

impl<'de> Deserialize<'de> for GeneralConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = GeneralConfigRaw::deserialize(deserializer)?;
        let mut cfg = GeneralConfig::default();

        if let Some(v) = raw.separator {
            cfg.separator = v;
        }
        if let Some(v) = raw.padding {
            cfg.padding = v;
        }
        if let Some(v) = raw.center {
            cfg.center = v;
        }
        if let Some(v) = raw.icons {
            cfg.icons = v;
        }
        if let Some(v) = raw.icon_only {
            cfg.icon_only = v;
        }
        if let Some(v) = raw.key_type {
            cfg.key_type = Some(v);
        }
        if let Some(v) = raw.live {
            cfg.live = v;
        }
        if let Some(cs) = raw.clear_screen {
            cfg.clear_screen = Some(cs);
        }

        if let Some(bf) = raw.border {
            match bf {
                BorderFieldHelper::Bool(b) => {
                    cfg.border = b;
                }
                BorderFieldHelper::Table(tbl) => {
                    apply_border_helper(&mut cfg, *tbl);
                }
            }
        }
        if let Some(s) = raw.border_style {
            cfg.border_style = Some(s);
        }
        if let Some(c) = raw.border_color {
            cfg.border_color = Some(c);
        }
        if let Some(ch) = raw.border_chars {
            cfg.border_chars = Some(ch);
        }
        if let Some(t) = raw.border_title {
            cfg.border_title = Some(t);
        }
        if let Some(p) = raw.box_padding {
            cfg.box_padding = p;
        }

        if let Some(bif) = raw.border_image {
            apply_border_image_helper(&mut cfg, bif);
        }
        if let Some(pos) = raw.border_image_position {
            cfg.border_image_position = pos;
        }
        if let Some(w) = raw.border_image_width {
            cfg.border_image_width = Some(w);
        }
        if let Some(h) = raw.border_image_height {
            cfg.border_image_height = Some(h);
        }
        if let Some(ox) = raw.border_image_offset_x {
            cfg.border_image_offset_x = Some(ox);
        }
        if let Some(oy) = raw.border_image_offset_y {
            cfg.border_image_offset_y = Some(oy);
        }
        if let Some(zi) = raw.border_image_z_index {
            cfg.border_image_z_index = Some(zi);
        }
        if let Some(pr) = raw.border_image_protocol {
            cfg.border_image_protocol = Some(pr);
        }
        if let Some(par) = raw.border_image_preserve_aspect_ratio {
            cfg.border_image_preserve_aspect_ratio = Some(par);
        }
        if let Some(tot) = raw.border_image_text_on_top {
            cfg.border_image_text_on_top = Some(tot);
        }

        if let Some(images) = raw.border_images {
            cfg.border_images = images;
        }
        if let Some(g) = raw.gradient {
            cfg.gradient = Some(g);
        }
        if let Some(t) = raw.theme {
            cfg.theme = Some(t);
        }
        if let Some(c) = raw.colors {
            cfg.colors = c;
        }
        if let Some(l) = raw.logo {
            cfg.logo = l;
        }
        if let Some(td) = raw.three_d {
            cfg.three_d = Some(td);
        }

        Ok(cfg)
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<bool>,
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
            hold: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct LogoConfig {
    pub enabled: bool,
    pub distro: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
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
            position: None,
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
            live: false,
            clear_screen: None,
            border: false,
            border_style: None,
            border_color: None,
            border_chars: None,
            border_title: None,
            box_padding: 1,
            border_image: None,
            border_image_width: None,
            border_image_height: None,
            border_image_offset_x: None,
            border_image_offset_y: None,
            border_image_z_index: None,
            border_image_position: "behind".to_string(),
            border_image_protocol: None,
            border_image_preserve_aspect_ratio: None,
            border_image_text_on_top: None,
            border_images: Vec::new(),
            gradient: None,
            theme: None,
            colors: ColorsConfig::default(),
            logo: LogoConfig::default(),
            three_d: None,
        }
    }
}

impl GeneralConfig {
    pub fn active_border_images(&self) -> Vec<BorderImageConfig> {
        let mut list = Vec::new();
        if let Some(path) = &self.border_image {
            if !path.trim().is_empty() {
                list.push(BorderImageConfig {
                    path: path.clone(),
                    position: self.border_image_position.clone(),
                    width: self.border_image_width,
                    height: self.border_image_height,
                    offset_x: self.border_image_offset_x,
                    offset_y: self.border_image_offset_y,
                    z_index: self.border_image_z_index.unwrap_or(-1),
                    protocol: self
                        .border_image_protocol
                        .clone()
                        .unwrap_or_else(|| "auto".to_string()),
                    preserve_aspect_ratio: self.border_image_preserve_aspect_ratio.unwrap_or(true),
                    text_on_top: self.border_image_text_on_top.unwrap_or(true),
                });
            }
        }
        for img in &self.border_images {
            if !img.path.trim().is_empty() {
                list.push(img.clone());
            }
        }
        list
    }
}

impl ModuleConfig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }
}

fn modules_from(names: &[&str]) -> Vec<ModuleConfig> {
    names.iter().copied().map(ModuleConfig::new).collect()
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
        "init" | "initsystem" => Some(""),
        "container" | "vm" => Some("󰡨"),
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
            modules: modules_from(&[
                "os", "host", "board", "kernel", "uptime", "packages", "shell", "desktop", "font",
                "terminal", "cpu", "gpu", "memory", "swap", "disk", "battery", "local_ip",
                "locale",
            ]),
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
                c.modules =
                    modules_from(&["os", "kernel", "uptime", "shell", "cpu", "memory", "disk"]);
                Some(c)
            }
            "clean" => {
                let mut c = Config::default();
                c.general.icon_only = true;
                c.general.separator = "".to_string();
                c.general.padding = 1;
                c.general.colors.enabled = false;
                c.modules = modules_from(&[
                    "os", "kernel", "desktop", "shell", "terminal", "cpu", "gpu", "memory",
                    "uptime",
                ]);
                Some(c)
            }
            "dots" => {
                let mut c = Config::default();
                c.general.icon_only = true;
                c.general.separator = "•".to_string();
                c.general.padding = 1;
                c.general.colors.enabled = true;
                c.general.colors.symbol = "●".to_string();
                c.modules = modules_from(&[
                    "os", "kernel", "uptime", "shell", "terminal", "cpu", "gpu", "memory", "disk",
                ]);
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
                c.modules = modules_from(&[
                    "os", "host", "kernel", "uptime", "break", "shell", "terminal", "desktop",
                    "break", "cpu", "gpu", "memory", "disk",
                ]);
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
                c.modules =
                    modules_from(&["os", "host", "kernel", "uptime", "shell", "cpu", "memory"]);
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
            let cfg = Self::find_fastfetch_config()
                .and_then(|ff_path| std::fs::read_to_string(ff_path).ok())
                .and_then(|content| Self::import_from_fastfetch(&content))
                .unwrap_or_default();
            if let Ok(toml_str) = toml::to_string_pretty(&cfg) {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&path, &toml_str);
            }
            return cfg;
        }
        Self::load_from(&path).unwrap_or_else(|err| {
            eprintln!("{err}");
            Self::default()
        })
    }

    pub fn load_from(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file '{}': {}", path.display(), e))?;
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext == "json" || ext == "jsonc")
        {
            if let Some(cfg) = Self::import_from_fastfetch(&content) {
                return Ok(cfg);
            }
        }
        Self::from_toml(&content).or_else(|toml_err| {
            if let Some(cfg) = Self::import_from_fastfetch(&content) {
                Ok(cfg)
            } else {
                Err(format!(
                    "Failed to parse config file '{}': {toml_err}",
                    path.display()
                ))
            }
        })
    }

    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    pub fn to_default_toml() -> String {
        r##"# ==============================================================================
# RustFetch Configuration (config.toml)
# ==============================================================================
# Blazing-fast, highly customizable system information tool written in Rust.
# Edit this file to customize layout, borders, overlays, colors, and modules.
# ==============================================================================

# ------------------------------------------------------------------------------
# 1. General Display & Layout
# ------------------------------------------------------------------------------
[general]
# Separator between module labels and values (e.g. ":", ": ", "  ", " ")
separator = ":"

# Spaces between separator and value
padding = 1

# Center output horizontally in terminal
center = false

# Enable/disable Nerd Font icons next to module labels
icons = true

# Display only icons and values (hide module label text)
icon_only = false

# Run in live interactive system monitor mode (continuous refresh)
live = false

# Preset theme: "catppuccin", "nord", "dracula", "gruvbox", "tokyonight", "rosepine", "solarized", "monokai", "cyberpunk"
# theme = "catppuccin"

# Multi-stop color gradient for text and borders (list of hex codes)
# gradient = ["#89b4fa", "#b4befe", "#cba6f7"]

# ------------------------------------------------------------------------------
# 2. Border & Box Frame
# ------------------------------------------------------------------------------
[general.border]
# Enable or disable the outer border box
enabled = false

# Border frame style: "rounded", "single", "double", "thick", "dashed", "block", "ascii", "brackets"
# style = "rounded"

# Border color: color name ("cyan", "red", "green", "blue", "magenta", "yellow") or hex ("#89b4fa")
# color = "cyan"

# Optional title displayed in the top border frame
# title = "RustFetch"

# Padding (in spaces) between border frame and content
padding = 1

# Custom border characters (overrides style if set)
# [general.border.chars]
# top_left = "╭"
# top = "─"
# top_right = "╮"
# right = "│"
# bottom_right = "╯"
# bottom = "─"
# bottom_left = "╰"
# left = "│"
# divider_left = "├"
# divider = "─"
# divider_right = "┤"

# ------------------------------------------------------------------------------
# 3. Border Image & Background Overlays
# ------------------------------------------------------------------------------
[general.border_image]
# Position options: "behind", "background", "left", "right", "top", "bottom"
# Default is "behind" (render image as background backdrop)
position = "behind"

# Overlay an image behind or alongside the fetch layout
# Supported protocols: "auto", "kitty", "sixel", "iterm", "halfblocks"
# path = "~/.config/rustfetch/background.png"
# width = 30
# height = 18
# offset_x = 0
# offset_y = 0
# z_index = -1
# protocol = "auto"

# Multiple layered border images (optional)
# [[general.border_images]]
# path = "~/.config/rustfetch/badge.png"
# position = "right"
# width = 20
# height = 10
# offset_x = 0
# offset_y = 0
# z_index = 1
# protocol = "auto"

# ------------------------------------------------------------------------------
# 4. Color Palette Bar
# ------------------------------------------------------------------------------
[general.colors]
# Display color test dots/blocks at bottom or top of output
enabled = true

# Character used for color blocks (e.g. "●", "■", "󰝤", "")
symbol = "●"

# Use solid colored rectangular blocks instead of symbols
block = false

# Extract color palette automatically from image logo when image mode is used
image_palette = true

# Position of the palette: "bottom", "top", "none"
position = "bottom"

# Number of rows (1 or 2 rows; 2 rows displays standard + bright colors)
rows = 1

# Custom color palette override (hex codes or ANSI color names)
# custom = ["#1e1e2e", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#94e2d5", "#cdd6f4"]

# ------------------------------------------------------------------------------
# 5. Logo & Visual Artwork
# ------------------------------------------------------------------------------
[general.logo]
# Enable or disable logo display
enabled = true

# Logo placement: "left" (default), "right", "top", "bottom", "behind", "hidden"
# position = "left"

# OS distro logo to display ("auto" detects current OS, or e.g. "arch", "ubuntu", "fedora", "nixos")
distro = "auto"

# Custom ASCII logo file path (overrides built-in distro art)
# ascii_path = "~/.config/rustfetch/ascii_logo.txt"

# Custom colors for multi-color ASCII logos (e.g. ["blue", "white"])
# colors = ["blue", "white"]

# Image logo file path (PNG, JPG, WebP)
# image_path = "~/.config/rustfetch/logo.png"

# Directory to randomly pick an image logo from on each launch
# image_dir = "~/.config/rustfetch/logos"

# Width of image logo in terminal columns
image_width_cols = 42

# Randomly select image from image_dir
random_image = false

# Automatically adapt primary theme colors to match dominant image colors
auto_color = true

# Terminal graphics protocol: "auto", "kitty", "sixel", "iterm", "halfblocks"
protocol = "auto"

# ------------------------------------------------------------------------------
# 6. 3D Spinning ASCII Relief Logo
# ------------------------------------------------------------------------------
[general.logo.three_d]
# Enable real-time animated 3D ASCII relief logo
enabled = false

# Rotation speed multiplier (default: 1.0)
speed = 1.0

# Enable rotation around X axis (pitch / wobble)
rotate_x = true

# Enable rotation around Y axis (horizontal yaw spin)
rotate_y = true

# Model size scaling factor
size = 1.0

# 3D depth extrusion factor
depth = 1.0

# Canvas dimensions in characters
width = 42
height = 24

# Shading mode: "ascii", "dot", "wireframe", "solid", "halfblock", "braille"
shading_mode = "ascii"

# Custom shading characters from darkest to brightest (default: " .:-=+*#%@")
# shading = " .:-=+*#%@"

# Simulated directional light source: "top-left", "top-right", "front", "top"
light = "top-left"

# Stop animation after N frames (default: None = run indefinitely)
# frames = 120

# Loop animation indefinitely
# infinite = true

# Hold terminal open and keep animating until Ctrl+C / 'q' (disables exit-on-key)
# hold = false

# Automatically exit animation on any keypress (perfect for instant shell access)
exit_on_key = true

# Custom outer and inner colors for 3D model (hex or color name)
# outer_color = "#89b4fa"
# inner_color = "#cdd6f4"

# ------------------------------------------------------------------------------
# 7. Multi-frame ASCII Animation
# ------------------------------------------------------------------------------
[general.logo.animation]
# Enable multi-frame ASCII animation
enabled = false

# Path to multi-frame ASCII animation text file (delimited by '---' or form-feed)
# path = "~/.config/rustfetch/anim.txt"

# Frames per second (FPS)
fps = 15.0

# Exit animation on any keypress
exit_on_key = true

# Loop animation indefinitely
infinite = true

# Hold animation open until Ctrl+C / 'q'
# hold = false

# Stop animation after N frames
# frames = 60

# ==============================================================================
# 8. Information Modules
# ==============================================================================
# Configure which modules are shown, their order, custom labels, icons, and formats.
#
# Available modules:
#   os, host, board, kernel, uptime, packages, shell, desktop, font,
#   terminal, cpu, cpu_usage, temp, gpu, memory, swap, disk, battery,
#   brightness, display, sound, wifi, local_ip, locale, processes, media,
#   break (blank spacer line), divider (horizontal rule), custom
#
# Optional per-module fields:
#   label     - Custom display label text (or "" to hide label)
#   icon      - Custom Nerd Font icon
#   color     - Module label color (hex or ANSI name)
#   format    - Value template (e.g. "{value}", "{}")
#   bar       - true/false: display percentage progress bar
#   bar_width - Width of progress bar in characters (default: 10)
#   value     - Static text value (for name = "custom")

[[modules]]
name = "os"

[[modules]]
name = "host"

[[modules]]
name = "board"

[[modules]]
name = "kernel"

[[modules]]
name = "uptime"

[[modules]]
name = "packages"

[[modules]]
name = "shell"

[[modules]]
name = "desktop"

[[modules]]
name = "font"

[[modules]]
name = "terminal"

[[modules]]
name = "cpu"

[[modules]]
name = "gpu"

[[modules]]
name = "memory"

[[modules]]
name = "swap"

[[modules]]
name = "disk"

[[modules]]
name = "battery"

[[modules]]
name = "local_ip"

[[modules]]
name = "locale"
"##
        .to_string()
    }

    pub fn to_toml_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    pub fn import_from_fastfetch(content: &str) -> Option<Self> {
        let stripped = strip_jsonc(content);
        let val: serde_json::Value = serde_json::from_str(&stripped).ok()?;

        let mut config = Self::default();

        if let Some(sep) = val.pointer("/display/separator").and_then(|s| s.as_str()) {
            if sep == " " {
                config.general.separator = " ".to_string();
            } else {
                config.general.separator = sep.trim_end().to_string();
            }
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
                } else if ltype == "kitty" {
                    config.general.logo.protocol = "kitty".to_string();
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
                    let is_file_type = logo.get("type").and_then(|t| t.as_str()) == Some("file");
                    let path = PathBuf::from(src);
                    if is_file_type {
                        config.general.logo.ascii_path = Some(src.to_string());
                    } else if let Some(parent) = path.parent() {
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
                        let mut mc = ModuleConfig::new("custom");
                        if let Some(key) = mod_obj.get("key").and_then(|k| k.as_str()) {
                            mc.label = Some(key.to_string());
                        }
                        if let Some(fmt) = mod_obj.get("format").and_then(|f| f.as_str()) {
                            mc.value = Some(fmt.to_string());
                        }
                        if mc.label.is_some() || mc.value.is_some() {
                            converted.push(mc);
                        }
                        continue;
                    }

                    if let Some(mapped_name) = map_fastfetch_type(mod_type) {
                        let allow_multiple = matches!(
                            mapped_name,
                            "break" | "custom" | "divider" | "packages" | "gpu" | "disk"
                        );
                        if allow_multiple || seen.insert(mapped_name) {
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
                            if let Some(color) = mod_obj.get("keyColor").and_then(|c| c.as_str()) {
                                mc.color = Some(color.to_string());
                            }
                            if let Some(fmt) = mod_obj.get("format").and_then(|f| f.as_str()) {
                                mc.format = Some(fmt.to_string());
                            }
                            converted.push(mc);
                        }
                    }
                } else if let Some(mod_name) = item.as_str() {
                    if let Some(mapped_name) = map_fastfetch_type(mod_name) {
                        let allow_multiple = matches!(
                            mapped_name,
                            "break" | "custom" | "divider" | "packages" | "gpu" | "disk"
                        );
                        if allow_multiple || seen.insert(mapped_name) {
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
        "de" | "wm" | "theme" | "wmtheme" | "icons" | "cursor" => Some("desktop"),
        "font" | "terminalfont" => Some("font"),
        "terminal" => Some("terminal"),
        "cpu" => Some("cpu"),
        "cpu_usage" | "cpuusage" => Some("cpu_usage"),
        "gpu" => Some("gpu"),
        "memory" => Some("memory"),
        "swap" => Some("swap"),
        "disk" => Some("disk"),
        "battery" | "poweradapter" => Some("battery"),
        "localip" | "local_ip" => Some("local_ip"),
        "locale" => Some("locale"),
        "break" | "divider" | "separator" => Some("break"),
        "custom" => Some("custom"),
        "media" | "player" => Some("media"),
        "sound" | "audio" => Some("sound"),
        "wifi" => Some("wifi"),
        "bluetooth" => Some("bluetooth"),
        "brightness" => Some("brightness"),
        "processes" => Some("processes"),
        "temp" | "temperature" => Some("temp"),
        "init" | "initsystem" => Some("init"),
        "container" | "vm" => Some("container"),
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
        // custom and break preserved, de and wm deduplicated to a single desktop module
        assert_eq!(
            mod_names,
            vec!["custom", "break", "os", "cpu", "desktop", "local_ip"]
        );

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

    #[test]
    fn test_border_images_config_and_active_list() {
        let toml_str = r#"
        [general]
        border_image = "legacy_border.png"
        border_image_width = 30
        border_image_position = "left"

        [[general.border_images]]
        path = "card_backdrop.png"
        position = "background"
        width = 50
        height = 20
        offset_x = 2
        offset_y = -1
        z_index = -1
        protocol = "kitty"

        [[general.border_images]]
        path = "side_ornament.png"
        position = "right"
        "#;

        let cfg: Config = toml::from_str(toml_str).unwrap();
        let active = cfg.general.active_border_images();
        assert_eq!(active.len(), 3);

        // Legacy image normalized
        assert_eq!(active[0].path, "legacy_border.png");
        assert_eq!(active[0].position, "left");
        assert_eq!(active[0].width, Some(30));
        assert!(!active[0].is_background());

        // Multi-image 1 (background)
        assert_eq!(active[1].path, "card_backdrop.png");
        assert_eq!(active[1].position, "background");
        assert_eq!(active[1].width, Some(50));
        assert_eq!(active[1].height, Some(20));
        assert_eq!(active[1].offset_x, Some(2));
        assert_eq!(active[1].offset_y, Some(-1));
        assert_eq!(active[1].z_index, -1);
        assert_eq!(active[1].protocol, "kitty");
        assert!(active[1].is_background());

        // Multi-image 2 (right)
        assert_eq!(active[2].path, "side_ornament.png");
        assert_eq!(active[2].position, "right");
        assert_eq!(active[2].z_index, -1);
        assert!(!active[2].is_background());
    }

    #[test]
    fn test_grouped_border_and_border_image_tables() {
        let toml_str = r#"
        [general]
        separator = " => "

        [general.border]
        enabled = true
        style = "double"
        color = "magenta"
        title = "MyBox"
        padding = 2

        [general.border_image]
        path = "wallpaper.png"
        position = "behind"
        width = 40
        height = 20
        offset_x = 1
        offset_y = 2
        z_index = -2
        protocol = "sixel"
        "#;

        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.general.separator, " => ");
        assert!(cfg.general.border);
        assert_eq!(cfg.general.border_style.as_deref(), Some("double"));
        assert_eq!(cfg.general.border_color.as_deref(), Some("magenta"));
        assert_eq!(cfg.general.border_title.as_deref(), Some("MyBox"));
        assert_eq!(cfg.general.box_padding, 2);

        assert_eq!(cfg.general.border_image.as_deref(), Some("wallpaper.png"));
        assert_eq!(cfg.general.border_image_position, "behind");
        assert_eq!(cfg.general.border_image_width, Some(40));
        assert_eq!(cfg.general.border_image_height, Some(20));
        assert_eq!(cfg.general.border_image_offset_x, Some(1));
        assert_eq!(cfg.general.border_image_offset_y, Some(2));
        assert_eq!(cfg.general.border_image_z_index, Some(-2));
        assert_eq!(cfg.general.border_image_protocol.as_deref(), Some("sixel"));

        let active = cfg.general.active_border_images();
        assert_eq!(active.len(), 1);
        assert!(active[0].is_background());
    }

    #[test]
    fn test_top_level_border_and_border_image_tables() {
        let toml_str = r#"
        [general]
        padding = 3

        [border]
        enabled = true
        style = "thick"

        [border_image]
        path = "bg.png"
        "#;

        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.general.padding, 3);
        assert!(cfg.general.border);
        assert_eq!(cfg.general.border_style.as_deref(), Some("thick"));
        assert_eq!(cfg.general.border_image.as_deref(), Some("bg.png"));
        assert_eq!(cfg.general.border_image_position, "behind");
    }

    #[test]
    fn test_border_image_default_position_is_behind() {
        let cfg = Config::default();
        assert_eq!(cfg.general.border_image_position, "behind");
        let bi = BorderImageConfig::default();
        assert_eq!(bi.position, "behind");
        assert!(bi.is_background());
    }
}
