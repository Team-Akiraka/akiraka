use druid::{Color, ImageBuf, Size};
use druid::piet::ImageFormat;
use image::imageops::FilterType;
use crate::Asset;

pub fn color_as_hex_string(color: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", color.as_rgba8().0, color.as_rgba8().1, color.as_rgba8().2).parse().unwrap()
}

pub fn load_image(path: &str, size: Size, filter: FilterType) -> ImageBuf {
    let raw_img = Asset::get(path).unwrap().data;
    let img_data = image::load_from_memory(&raw_img).unwrap();
    let img_data = img_data.resize(size.width as u32, size.height as u32, filter);
    let rgb_img = img_data.to_rgba8();
    let img_size = rgb_img.dimensions();
    let img_buf = ImageBuf::from_raw(
        rgb_img.to_vec(),
        ImageFormat::RgbaPremul,
        img_size.0 as usize,
        img_size.1 as usize
    );
    img_buf
}