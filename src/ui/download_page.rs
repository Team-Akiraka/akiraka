use druid::{Data, Insets, LocalizedString, UnitPoint, Widget, WidgetExt};
use druid::widget::{Axis, Flex, Label, List};
use crate::{AppState, Empty};
use crate::widget::tabs::Tabs;

pub const ID: &str = "DOWNLOAD_PAGE";

fn build_minecraft() -> impl Widget<AppState> {
    let list = List::<String>::new(|| {
        Label::new("")
            .on_added(|widget, ctx, data: &String, env| {
                widget.set_text(data.clone());
            })
    })
        .with_spacing(0.0)
        .expand_width()
        .lens(AppState::minecraft_versions)
        .on_added(|widget, ctx, data, env| {
        });

    let layout = Flex::column()
        .with_child(list);

    layout
        .padding(Insets::uniform(8.0))
}

fn build_center() -> impl Widget<AppState> {
    let tabs = Tabs::new()
        .with_child("Minecraft".parse().unwrap(), build_minecraft())
        .with_child("Resources".parse().unwrap(), Label::new("Resources"))
        // .with_child("3".parse().unwrap(), Label::new("1145141919810"))
        // .with_child("4".parse().unwrap(), Label::new("1145141919810"))
        // .with_child("5".parse().unwrap(), Label::new("1145141919810"))
        // .with_child("6".parse().unwrap(), Label::new("1145141919810"))
        .with_selected("Minecraft".parse().unwrap())
        .padding(Insets::uniform(8.0));

    tabs
}

pub fn build() -> impl Widget<AppState> {
    // let title = Label::new("Download")
    //     .with_text_size(24.0)
    //     .fix_width(32.0)
    //     .expand_width()
    //     .padding(Insets::uniform_xy(16.0, 4.0));

    let body = Flex::column()
        // .with_child(title)
        // .with_spacer(4.0)
        .with_child(build_center())
        .fix_width(160.0)
        .padding(Insets::uniform_xy(0.0, 0.0));

    body
        .expand_width()
        .align_vertical(UnitPoint::TOP)
        .align_left()
}