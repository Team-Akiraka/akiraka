use crate::ui::bottom_bar;
use crate::widget::window;
use druid::widget::{Flex, Label, Widget};
use druid::{Data, Insets, UnitPoint, WidgetExt};

pub const ID: &str = "HELLO_PAGE";

pub fn build<T: Data>() -> impl Widget<T> {
    let hello = Label::new("Hello")
        .with_text_size(48.0)
        .align_left()
        .padding(Insets::uniform_xy(32.0, 0.0));
    let name = Label::new("Arrokoth233")
        .with_text_size(18.0)
        .align_left()
        .padding(Insets::uniform_xy(32.0, 0.0));

    let body = Flex::column()
        .with_flex_spacer(1.0)
        .with_child(hello)
        .with_child(name)
        .with_spacer(16.0)
        .with_spacer(bottom_bar::BOTTOM_BAR_HEIGHT)
        .with_spacer(window::TITLE_BAR_HEIGHT)
        .expand_width();

    body.align_vertical(UnitPoint::TOP)
}
