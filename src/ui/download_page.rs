use druid::{Data, Insets, LocalizedString, UnitPoint, Widget, WidgetExt};
use druid::widget::{Flex, Label};
use crate::{AppState, Empty};

pub const ID: &str = "DOWNLOAD_PAGE";

fn build_selection(name: LocalizedString<AppState>) -> impl Widget<AppState> {
    Empty {}
}

fn build_center() -> impl Widget<AppState> {
    let layout = Flex::row()
        .with_child(build_selection());

    layout
}

pub fn build() -> impl Widget<AppState> {
    let title = Label::new("Download")
        .with_text_size(24.0)
        .fix_width(32.0)
        .expand_width()
        .padding(Insets::uniform_xy(12.0, 4.0));

    let body = Flex::column()
        .with_child(title)
        .with_spacer(4.0)
        .with_child(build_center())
        .fix_width(160.0)
        .padding(Insets::uniform_xy(8.0, 0.0));

    let body = druid::widget::Scroll::new(body);

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}