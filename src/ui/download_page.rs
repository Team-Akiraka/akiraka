use druid::{Data, Insets, UnitPoint, Widget, WidgetExt};
use druid::widget::{Flex, Label, Radio, RadioGroup};

pub const ID: &str = "DOWNLOAD_PAGE";

pub fn build<T: Data>() -> impl Widget<T> {
    let title = Label::new("Download")
        .with_text_size(24.0)
        .fix_width(32.0)
        .expand_width()
        .padding(Insets::uniform_xy(12.0, 4.0));

    let body = Flex::column()
        .with_child(title)
        .fix_width(160.0)
        .padding(Insets::uniform_xy(8.0, 8.0));

    let body = druid::widget::Scroll::new(body);

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}