mod color;
mod image;
mod kitty;
mod terminal;
pub mod three_d;

pub use color::{extract_color_palette, extract_dominant_color};
pub use image::render_halfblock;

pub fn render_image(path: &str, target_cols: usize) -> Option<usize> {
    image::render(path, target_cols)
}
