use std::borrow::ToOwned;
use std::collections::HashMap;
use druid::{BoxConstraints, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UnitPoint, UpdateCtx, Widget, WidgetExt, WidgetPod};
use druid::piet::TextStorage;
use druid::widget::{Axis, CrossAxisAlignment, Flex, FlexParams, Label, Tabs, TabsEdge, TabsTransition};
use crate::{AppState, Asset};
use crate::widget::side_bar_selection::SideBarSelection;

pub const ID: &str = "SETTINGS_PAGE";
static mut SELECTED: u64 = 0;

const ANIMATION_TIME: f64 = 0.3;

struct Child<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>
}

#[allow(unused)]
impl<T> Child<T> {
    fn new(inner: WidgetPod<T, Box<dyn Widget<T>>>) -> Child<T> {
        Child {
            inner
        }
    }

    fn widget_mut(&mut self) -> Option<&mut WidgetPod<T, Box<dyn Widget<T>>>> {
        Some(&mut self.inner)
    }

    fn widget(&self) -> Option<&WidgetPod<T, Box<dyn Widget<T>>>> {
        Some(&self.inner)
    }
}

struct PagedWidget<T> {
    pages: HashMap<u64, Child<T>>
}

impl<T: Data> PagedWidget<T> {
    fn new(pages: HashMap<u64, Child<T>>) -> PagedWidget<T> {
        PagedWidget {
            pages
        }
    }
}

impl<T: Data> Widget<T> for PagedWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.layout(ctx, bc, data, env);
        }
        bc.min()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.paint(ctx, data, env);
        }
    }
}

fn build_left<T: Data>() -> impl Widget<T> {
    let title = Label::new("Settings")
        .with_text_size(24.0)
        .fix_width(32.0)
        .expand_width()
        .padding(Insets::uniform_xy(12.0, 4.0));

    let mut buttons: HashMap<u64, &SideBarSelection<T>> = HashMap::new();

    let mut common_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/settings.svg").unwrap().data).unwrap().parse().unwrap(), "Common", 0);
    common_button.click_event = |btn| {
        // println!("{:?}", buttons.len());
    };
    buttons.insert(0, &common_button);

    let mut network_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/download.svg").unwrap().data).unwrap().parse().unwrap(), "Download", 1);
    let mut game_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/play.svg").unwrap().data).unwrap().parse().unwrap(), "Game", 2);
    let mut multiplayer_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/network.svg").unwrap().data).unwrap().parse().unwrap(), "Multiplayer", 3);
    let mut about_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/info.svg").unwrap().data).unwrap().parse().unwrap(), "About", 4);

    network_button.click_event = |btn| {
    };
    buttons.insert(1, &network_button);

    game_button.click_event = |btn| {
    };
    buttons.insert(2, &game_button);

    multiplayer_button.click_event = |btn| {
    };
    buttons.insert(3, &multiplayer_button);

    about_button.click_event = |btn| {
    };
    buttons.insert(4, &about_button);

    let common_button = common_button
        .fix_height(32.0)
        .expand_width();

    let network_button = network_button
        .fix_height(32.0)
        .expand_width();

    let game_button = game_button
        .fix_height(32.0)
        .expand_width();

    let multiplayer_button = multiplayer_button
        .fix_height(32.0)
        .expand_width();

    let about_button = about_button
        .fix_height(32.0)
        .expand_width();

    // let game_button = game_button
    //     .fix_height(32.0)
    //     .expand_width();

    let left = Flex::column()
        .with_child(title)
        .with_spacer(8.0)
        .with_child(common_button)
        .with_spacer(4.0)
        .with_child(network_button)
        .with_spacer(4.0)
        .with_child(game_button)
        .with_spacer(4.0)
        .with_child(multiplayer_button)
        .with_spacer(4.0)
        .with_child(about_button)
        .with_spacer(4.0)
        .fix_width(160.0)
        .padding(Insets::uniform_xy(8.0, 8.0));

    let paged = PagedWidget::new(HashMap::new());

    let body = Flex::row()
        .with_child(left)
        .with_flex_child(paged, FlexParams::new(1.0, CrossAxisAlignment::Center));

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