use std::borrow::ToOwned;
use std::collections::HashMap;
use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UnitPoint, UpdateCtx, Vec2, Widget, WidgetExt, WidgetPod};
use druid::piet::TextStorage;
use druid::widget::{Axis, CrossAxisAlignment, Flex, FlexParams, Label, Tabs, TabsEdge, TabsTransition};
use crate::{animations, AppState, Asset};
use crate::widget::side_bar_selection::SideBarSelection;
use crate::widget::window;

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
    pages: HashMap<u64, Child<T>>,
    current_id: u64,
    inner_size: Size,
    t: f64
}

impl<T: Data> PagedWidget<T> {
    fn new(pages: HashMap<u64, Child<T>>) -> PagedWidget<T> {
        PagedWidget {
            pages,
            current_id: unsafe { SELECTED },
            inner_size: Size::ZERO,
            t: 1.0
        }
    }

    fn detect_scene_change(&mut self) -> bool {
        if self.current_id != unsafe { SELECTED } {
            self.current_id = unsafe { SELECTED };
            true
        } else {
            false
        }
    }
}

impl<T: Data> Widget<T> for PagedWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        // if self.detect_scene_change() {
        //     self.t = 0.0;
        //     ctx.request_anim_frame();
        // }

        let x = self.pages.get_mut(&self.current_id);
        if x.is_some() {
            x.unwrap().inner.event(ctx, event, data, env);
        }

        match event {
            Event::WindowConnected => {
                self.current_id = unsafe { SELECTED };
                self.t = 0.0;
                ctx.request_anim_frame();
            }
            Event::AnimFrame(interval) => {
                self.t += (*interval as f64) * 1e-9;
                if self.t <= ANIMATION_TIME {
                    ctx.request_anim_frame();
                    ctx.request_paint();
                } else {
                    ctx.request_paint();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        // if self.detect_scene_change() {
        //     self.t = 0.0;
        //     ctx.request_anim_frame();
        // }

        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        // if self.detect_scene_change() {
        //     self.t = 0.0;
        //     ctx.request_anim_frame();
        // }
        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.layout(ctx, bc, data, env);
        }
        let size = Size::new(if bc.min().width > ctx.window().get_size().width {
            ctx.window().get_size().width
        } else {
            bc.min().width
        }, ctx.window().get_size().height);
        self.inner_size = size;
        self.inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if self.detect_scene_change() {
            self.t = 1.0;
            ctx.window().request_anim_frame();
        }

        let rect = ctx.size().to_rect();
        ctx.fill(rect, &Color::RED);

        let x = self.pages.get_mut(&self.current_id);
        if x.is_some() {
            let s = if self.t / ANIMATION_TIME < 1.0 {
                let s = self.t / ANIMATION_TIME;

                animations::expo::ease_out(s)
            } else {
                1.0
            };
            let s = s / 4.0 + 0.75;
            let w = ctx.window().get_size().width / 2.0 - self.inner_size.width * s / 2.0;
            let h = ctx.window().get_size().height / 2.0 - self.inner_size.height * s / 2.0;
            ctx.transform(Affine::scale(s)
                .then_translate(Vec2::new(w, h)));
            x.unwrap().inner.paint(ctx, data, env);
        }
    }
}

fn build_left<T: Data>() -> impl Widget<T> {
    let title = Label::new("Settings")
        .with_text_size(24.0)
        .fix_width(32.0)
        .expand_width()
        .padding(Insets::uniform_xy(12.0, 4.0));

    // let mut buttons: HashMap<u64, &SideBarSelection<T>> = HashMap::new();

    let mut common_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/settings.svg").unwrap().data).unwrap().parse().unwrap(), "Common", 0);
    let mut network_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/download.svg").unwrap().data).unwrap().parse().unwrap(), "Download", 1);
    let mut game_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/play.svg").unwrap().data).unwrap().parse().unwrap(), "Game", 2);
    let mut multiplayer_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/network.svg").unwrap().data).unwrap().parse().unwrap(), "Multiplayer", 3);
    let mut about_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/info.svg").unwrap().data).unwrap().parse().unwrap(), "About", 4);

    let mut common_button = common_button.on_click(|ctx, data, env| {
        unsafe {
            SELECTED = 0;
        }
        ctx.request_layout();
    });

    let mut network_button = network_button.on_click(|ctx, data, env| {
        unsafe {
            SELECTED = 1;
        }
        ctx.request_layout();
    });

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

    let body = Flex::column()
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
        .padding(Insets::uniform_xy(8.0, 8.0))
        .align_horizontal(UnitPoint::CENTER);

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}

fn build_right<T: Data>() -> impl Widget<T> {
    fn test<T: Data>() -> impl Widget<T> {
        Label::new("114514").expand()
    }
    fn test1<T: Data>() -> impl Widget<T> {
        Label::new("1145141919810").expand()
    }

    let mut children = HashMap::new();
    children.insert(0, Child::new(WidgetPod::new(Box::new(test()))));
    children.insert(1, Child::new(WidgetPod::new(Box::new(test1()))));

    let paged = PagedWidget::new(children)
        .expand();

    paged
}

pub fn build<T: Data>() -> impl Widget<T> {
    let body = Flex::row()
        .with_child(build_left())
        .with_flex_child(build_right(), FlexParams::new(1.0, CrossAxisAlignment::Center));

    body
        .align_vertical(UnitPoint::TOP)
}