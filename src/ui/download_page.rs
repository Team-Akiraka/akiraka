use druid::{Data, Insets, LocalizedString, UnitPoint, Widget, WidgetExt};
use druid::widget::{Axis, Flex, Label, Tabs, TabsTransition};
use crate::{AppState, Empty};

pub const ID: &str = "DOWNLOAD_PAGE";

fn build_selection(name: LocalizedString<AppState>) -> impl Widget<AppState> {
    Label::new(name)
}

fn build_center() -> impl Widget<AppState> {
    let tabs = Tabs::new()
        .with_transition(TabsTransition::Instant)
        .with_axis(Axis::Horizontal)
        .with_tab("114514", Label::new("114514"))
        .with_tab("1412", Label::new("123413123"))
        .with_tab("1412", Label::new("123413123"))
        .with_tab("1412", Label::new("123413123"))
        .with_tab("1412", Label::new("123413123"))
        .with_tab("1412", Label::new("123413123"))
        .with_tab("1412", Label::new("123413123"))
        .with_tab("1412", Label::new("123413123"))
        .with_tab("1412", Label::new("123413123"))
        .with_tab("1412", Label::new("123413123"))
        .expand_width();
    // let layout = Flex::row()
        // .with_child(build_selection(LocalizedString::new("Minecraft")));

    tabs
    // tabs.expand_width()
}

pub fn build() -> impl Widget<AppState> {
    let title = Label::new("Download")
        .with_text_size(24.0)
        .fix_width(32.0)
        .expand_width()
        .padding(Insets::uniform_xy(16.0, 4.0));

    let body = Flex::column()
        .with_child(title)
        .with_spacer(4.0)
        .with_child(build_center())
        .fix_width(160.0)
        .padding(Insets::uniform_xy(0.0, 0.0));

    body
        .expand_width()
        .align_vertical(UnitPoint::TOP)
        .align_left()
}