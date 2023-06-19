use druid::{Data, Insets, UnitPoint, Widget, WidgetExt};
use druid::widget::{Align, Flex, FlexParams, Label};
use crate::ui;
use crate::widget::{window};
use crate::ui::{bottom_bar};

pub const ID: &str = "SETTINGS_PAGE";

pub fn build<T: Data>() -> Box<dyn Widget<T>> {
    let body = Flex::column()
        .expand_width();

    Box::new(body
        .align_vertical(UnitPoint::TOP))
}