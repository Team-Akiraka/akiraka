use druid::{Data, Insets, UnitPoint, Widget, WidgetExt};
use druid::widget::{Align, Flex, FlexParams, Label};
use crate::ui;
use crate::widget::{window};
use crate::ui::{bottom_bar};

pub const ID: &str = "SETTINGS_PAGE";

pub fn build<T: Data>() -> impl Widget<T> {
    let title = Label::new("Settings")
        .with_text_size(24.0)
        .padding(Insets::uniform_xy(24.0, 12.0))
        .expand_width();

    let body = Flex::column()
        .with_child(title)
        .expand();

    body
        .align_vertical(UnitPoint::TOP)
}