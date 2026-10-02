mod color;
mod image;
mod kitty;
mod terminal;
pub mod three_d;

pub use color::{
    apply_gradient_to_text, extract_color_palette, extract_dominant_color, get_theme,
    interpolate_color, interpolate_gradient, parse_color, Theme,
};
pub use image::render_halfblock;

pub fn render_image(path: &str, target_cols: usize) -> Option<usize> {
    image::render(path, target_cols)
}
