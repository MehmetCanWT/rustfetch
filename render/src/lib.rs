mod color;
mod image;
mod kitty;
pub mod sixel;
mod terminal;
pub mod three_d;

pub use color::{
    apply_gradient_to_text, extract_color_palette, extract_dominant_color, get_theme,
    interpolate_color, interpolate_gradient, parse_color, Theme,
};
pub use image::{
    apply_image_background_to_lines, apply_image_background_to_lines_ex, fit_dimensions,
    render_background, render_background_ex, render_halfblock, render_halfblock_ex,
    BackgroundOptions,
};
pub use kitty::{clear_kitty_images, print_kitty, print_kitty_ex};
pub use sixel::{encode_sixel, render_sixel_ex};
pub use terminal::{cell_size_or_default, get_cell_size, supports_kitty, supports_sixel};

pub fn render_image_ex(
    path: &str,
    target_cols: usize,
    max_rows: Option<usize>,
) -> Option<(usize, usize)> {
    image::render_ex(path, target_cols, max_rows)
}
