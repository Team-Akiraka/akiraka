use rust_embed::RustEmbed;

pub mod title_bar;
pub mod icon_button;
pub mod window;
pub mod bottom_bar;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Asset;