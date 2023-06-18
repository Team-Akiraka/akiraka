use druid::Widget;
use crate::util;

pub mod hello_page;
pub mod bottom_bar;
pub mod settings_page;
pub mod download_page;

pub static mut ANIMATION_START_TIME: u128 = 0;

pub fn change_scene<T>(scene: impl Widget<T>) {
    unsafe { ANIMATION_START_TIME = util::get_time(); }
}
