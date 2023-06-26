use druid::{Data, Insets, UnitPoint, Widget, WidgetExt};
use druid::widget::{CrossAxisAlignment, Flex, FlexParams, Label, Tabs};
use crate::{Asset};
use crate::widget::side_bar_selection::SideBarSelection;

pub const ID: &str = "SETTINGS_PAGE";

fn build_left<T: Data>() -> impl Widget<T> {
    // let title = Label::new("Settings")
    //     .with_text_size(24.0)
    //     .fix_width(32.0)
    //     .expand_width()
    //     .padding(Insets::uniform_xy(12.0, 4.0));
    //
    // let mut common_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/settings.svg").unwrap().data).unwrap().parse().unwrap(), "Common");
    // let mut network_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/download.svg").unwrap().data).unwrap().parse().unwrap(), "Download");
    // let mut game_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/play.svg").unwrap().data).unwrap().parse().unwrap(), "Game");
    // let mut multiplayer_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/network.svg").unwrap().data).unwrap().parse().unwrap(), "Multiplayer");
    // let mut about_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/info.svg").unwrap().data).unwrap().parse().unwrap(), "About");
    //
    // let common_button = common_button
    //     .fix_height(32.0)
    //     .expand_width();
    //
    // let network_button = network_button
    //     .fix_height(32.0)
    //     .expand_width();
    //
    // let game_button = game_button
    //     .fix_height(32.0)
    //     .expand_width();
    //
    // let multiplayer_button = multiplayer_button
    //     .fix_height(32.0)
    //     .expand_width();
    //
    // let about_button = about_button
    //     .fix_height(32.0)
    //     .expand_width();
    //
    // // let game_button = game_button
    // //     .fix_height(32.0)
    // //     .expand_width();
    //
    // let body = Flex::column()
    //     .with_child(title)
    //     .with_spacer(8.0)
    //     .with_child(common_button)
    //     .with_spacer(4.0)
    //     .with_child(network_button)
    //     .with_spacer(4.0)
    //     .with_child(game_button)
    //     .with_spacer(4.0)
    //     .with_child(multiplayer_button)
    //     .with_spacer(4.0)
    //     .with_child(about_button)
    //     .with_spacer(4.0)
    //     .fix_width(160.0)
    //     .padding(Insets::uniform_xy(8.0, 8.0));
    //
    // let body = druid::widget::Scroll::new(body);
    //
    // body
    //     .align_vertical(UnitPoint::TOP)
    //     .align_left()
    let tab = Tabs::new()
        .with_tab("Test", Label::new("114514"))
        .with_tab("Test1", Label::new("114514"))
        .with_tab("Test2", Label::new("114514"))
        .padding(Insets::uniform(12.0))
        .fix_size(600.0, 480.0);

    // let body = Flex::column()
    //     .with_child(tab.expand());

    // body
    tab
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