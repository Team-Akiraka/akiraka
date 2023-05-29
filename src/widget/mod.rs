use rust_embed::RustEmbed;

pub mod title_bar;
pub mod icon_button;
pub mod window;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Asset;