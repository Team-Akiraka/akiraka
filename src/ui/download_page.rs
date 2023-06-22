use druid::{Data, UnitPoint, Widget, WidgetExt};
use druid::widget::{Flex};

pub const ID: &str = "DOWNLOAD_PAGE";

#[allow(unused)]
pub fn build<T: Data>() -> impl Widget<T> {
    let body = Flex::column()
        .expand_width();

    body
        .align_vertical(UnitPoint::TOP)
}