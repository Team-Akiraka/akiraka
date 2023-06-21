use druid::{Data, Insets, UnitPoint, Widget, WidgetExt};
use druid::widget::{Align, CrossAxisAlignment, Flex, FlexParams, Label, List, RadioGroup};
use crate::{Asset, ui};
use crate::widget::{window};
use crate::ui::{bottom_bar};
use crate::widget::clear_button::ClearButton;
use crate::widget::separator::Separator;
use crate::widget::side_bar_selection::SideBarSelection;

pub const ID: &str = "SETTINGS_PAGE";

fn build_left<T: Data>() -> impl Widget<T> {
    let title = Label::new("Settings")
        .with_text_size(24.0)
        .fix_size(128.0, 32.0)
        .padding(Insets::uniform_xy(12.0, 12.0));

    let about_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/info.svg").unwrap().data).unwrap().parse().unwrap(), "About")
        .fix_size(128.0, 32.0);

    let network_button = ClearButton::new("Network")
        .fix_size(128.0, 32.0);

    let body = Flex::column()
        .with_child(title)
        .with_spacer(4.0)
        .with_child(about_button)
        .with_spacer(4.0)
        .with_child(network_button);

    let body = druid::widget::Scroll::new(body);

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}

fn build_right<T: Data>() -> impl Widget<T> {
    let body = Flex::column();

    body
        .align_vertical(UnitPoint::TOP)
}

pub fn build<T: Data>() -> impl Widget<T> {
    // let title = Label::new("Settings")
    //     .with_text_size(24.0)
    //     .padding(Insets::uniform_xy(24.0, 12.0))
    //     .expand_width();
    //
    // let body = Flex::column()
    //     .with_child(title)
    //     .expand();
    let body = Flex::row()
        .with_child(build_left())
        .with_flex_child(build_right(), FlexParams::new(1.0, CrossAxisAlignment::Center));

    body
        .align_vertical(UnitPoint::TOP)
}