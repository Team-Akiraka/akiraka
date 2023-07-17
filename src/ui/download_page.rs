use druid::{BoxConstraints, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, LocalizedString, PaintCtx, Size, UnitPoint, UpdateCtx, Widget, WidgetExt, WidgetPod};
use druid::widget::{Axis, Flex, Label, List};
use crate::{AppState, Empty};
use crate::widget::tabs::Tabs;

pub const ID: &str = "DOWNLOAD_PAGE";
pub static mut IS_LOADING: bool = false;

struct GameInstance<T> {
    version_name: String,
    version_type: String,
    layout: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T: Data> GameInstance<T> {
    pub fn new(version_name: String, version_type: String) -> GameInstance<T> {
        let layout = Flex::row();
        GameInstance {
            version_name,
            version_type,
            layout: WidgetPod::new(Box::new(layout))
        }
    }
}

impl<T: Data> Widget<T> for GameInstance<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.layout.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.layout.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.layout.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.layout.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.layout.paint(ctx, data, env);
    }
}

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