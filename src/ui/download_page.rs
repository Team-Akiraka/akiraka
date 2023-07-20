use crate::theme::theme;
use crate::widget::bounded_widget::BoundedWidget;
use crate::widget::tabs::Tabs;
use crate::{AppState, Empty};
use druid::widget::{Flex, Label, List, Scroll};
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, UnitPoint, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

pub const ID: &str = "DOWNLOAD_PAGE";
// pub static mut IS_LOADING: bool = false;

struct GameInstance<T> {
    pub(crate) version_name: String,
    pub(crate) version_type: String,
    layout: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> GameInstance<T> {
    pub fn new(version_name: String, version_type: String) -> GameInstance<T> {
        GameInstance {
            version_name,
            version_type,
            layout: WidgetPod::new(Box::new(Empty {})),
        }
    }

    pub fn init_data(&mut self, version_name: String, version_type: String) {
        let layout = Flex::column()
            .with_child(Label::new(version_name).with_text_size(14.0).align_left())
            .with_child(Label::new(version_type).with_text_size(12.0).align_left())
            .center()
            .align_left();

        self.layout = WidgetPod::new(Box::new(layout));
    }
}

impl<T: Data> Widget<T> for GameInstance<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.layout.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.layout.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.layout.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.layout.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_hot = ctx.is_hot();
        let rect = ctx.size().to_rect().to_rounded_rect(12.0);

        if is_hot {
            ctx.fill(rect, &env.get(theme::COLOR_CLEAR_BUTTON_ACTIVE));
        }

        self.layout.paint(ctx, data, env);
    }
}

fn build_minecraft() -> impl Widget<AppState> {
    let list = List::<String>::new(|| {
        GameInstance::new(String::new(), String::new())
            .on_added(|widget, ctx, data: &String, _env| {
                widget.init_data(String::from(data), String::from(data));
                ctx.request_paint();
            })
            .expand_width()
            .fix_height(48.0)
            .align_left()
    })
    .with_spacing(0.0)
    .lens(AppState::minecraft_versions);

    let layout = Flex::column().with_child(list);

    layout
}

fn build_center() -> impl Widget<AppState> {
    // rust_embed::RustEmbed
    let tabs = Tabs::new()
        .with_child(
            "Minecraft".parse().unwrap(),
            BoundedWidget::new(
                Scroll::new(build_minecraft())
                    .vertical()
                    .expand_height()
                    .padding(Insets::new(8.0, 8.0, 8.0, 124.0)),
            ),
        )
        .with_child("Resources".parse().unwrap(), Label::new("Resources"))
        .with_selected("Minecraft".parse().unwrap())
        .padding(Insets::uniform(8.0));

    tabs
}

pub fn build() -> impl Widget<AppState> {
    let body = Flex::column()
        .with_child(build_center())
        .padding(Insets::uniform_xy(0.0, 0.0));

    body.expand().align_vertical(UnitPoint::TOP).align_left()
}
