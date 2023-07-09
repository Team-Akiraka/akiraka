#[allow(dead_code)]

use std::borrow::ToOwned;
use std::collections::HashMap;
use std::ptr::null;
use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, LocalizedString, Menu, PaintCtx, RenderContext, Size, UnitPoint, UpdateCtx, Vec2, Widget, WidgetExt, WidgetPod};
use druid::keyboard_types::Key::Clear;
use druid::widget::{Axis, CrossAxisAlignment, Flex, FlexParams, Label, List};
use crate::{animations, AppState, Asset};
use crate::theme::theme;
use crate::widget::button::Button;
use crate::widget::clear_button::ClearButton;
use crate::widget::side_bar_selection::SideBarSelection;

pub const ID: &str = "SETTINGS_PAGE";

const ANIMATION_TIME: f64 = 0.3;
static mut SELECTED: u64 = 0;

struct Child<AppState> {
    inner: WidgetPod<AppState, Box<dyn Widget<AppState>>>
}

impl Child<AppState> {
    fn new(inner: WidgetPod<AppState, Box<dyn Widget<AppState>>>) -> Child<AppState> {
        Child {
            inner
        }
    }

    fn widget_mut(&mut self) -> Option<&mut WidgetPod<AppState, Box<dyn Widget<AppState>>>> {
        Some(&mut self.inner)
    }

    fn widget(&self) -> Option<&WidgetPod<AppState, Box<dyn Widget<AppState>>>> {
        Some(&self.inner)
    }
}

struct PagedWidget<AppState> {
    pages: HashMap<u64, Child<AppState>>,
    current_id: u64,
    inner_size: Size,
    offset: f64,
    t: f64
}

impl PagedWidget<AppState> {
    fn new(pages: HashMap<u64, Child<AppState>>, offset: f64) -> PagedWidget<AppState> {
        PagedWidget {
            pages,
            current_id: unsafe { SELECTED },
            inner_size: Size::ZERO,
            offset,
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

impl Widget<AppState> for PagedWidget<AppState> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
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

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        // if self.detect_scene_change() {
        //     self.t = 0.0;
        //     ctx.request_anim_frame();
        // }

        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppState, data: &AppState, env: &Env) {
        // if self.detect_scene_change() {
        //     self.t = 0.0;
        //     ctx.request_anim_frame();
        // }
        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        println!("{:?}", bc);
        let w = ctx.window().get_size().width - self.offset;
        let h = ctx.window().get_size().height;
        let child_bc = BoxConstraints::new(
            Size::new(
                if bc.min().width > w { w } else { bc.min().width },
                bc.max().height),
            Size::new(
                if bc.max().width > w { w } else { bc.max().width },
                bc.max().height)
        );

        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.layout(ctx, &child_bc, data, env);
        }

        let size = Size::new(if bc.min().width > ctx.window().get_size().width {
            ctx.window().get_size().width
        } else {
            bc.min().width
        }, ctx.window().get_size().height);

        self.inner_size = size;
        self.inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        if self.detect_scene_change() {
            self.t = 1.0;
            ctx.window().request_anim_frame();
        }

        let rect = ctx.size().to_rect();
        ctx.fill(rect, &Color::TRANSPARENT);

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

fn build_settings() -> impl Widget<AppState> {
    let game = Flex::column()
        .with_child(Label::new(LocalizedString::new("None")).with_text_size(16.0).align_left())
        .padding(Insets::uniform_xy(12.0, 12.0))
        .background(theme::COLOR_BACKGROUND_LIGHT)
        .border(theme::COLOR_BORDER_DARK, 1.0)
        .rounded(10.0)
        .expand_width()
        .align_left();

    let mut body = Flex::column()
        .with_child(game)
        .padding(Insets::new(4.0, 4.0, 32.0, 4.0));

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}

fn build_game() -> impl Widget<AppState> {
    let java = Flex::column()
        .with_child(Label::new(LocalizedString::new("Java Runtime")).with_text_size(16.0).align_left())
        .with_spacer(12.0)
        .with_child(
            List::new(|| {
            Label::new("114514").align_left()
        })
            .lens(AppState::java)
        )
        .with_spacer(12.0)
        .with_child(ClearButton::new("Test").fix_height(28.0).align_left())

        .with_spacer(4.0)
        .padding(Insets::uniform_xy(12.0, 8.0))
        .background(theme::COLOR_BACKGROUND_LIGHT)
        .border(theme::COLOR_BORDER_DARK, 1.0)
        .rounded(10.0)
        .expand_width()
        .align_left();

    let mut body = Flex::column()
        .with_child(java)
        .padding(Insets::new(4.0, 4.0, 32.0, 4.0));

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}

fn build_left() -> impl Widget<AppState> {
    let title = Label::new("Settings")
        .with_text_size(24.0)
        .fix_width(32.0)
        .expand_width()
        .padding(Insets::uniform_xy(12.0, 4.0));

    // let mut buttons: HashMap<u64, &SideBarSelection<AppState>> = HashMap::new();

    let common_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/settings.svg").unwrap().data).unwrap().parse().unwrap(), "Common", 0);
    let game_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/play.svg").unwrap().data).unwrap().parse().unwrap(), "Game", 2);
    let network_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/download.svg").unwrap().data).unwrap().parse().unwrap(), "Download", 1);
    let multiplayer_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/network.svg").unwrap().data).unwrap().parse().unwrap(), "Multiplayer", 3);
    let about_button = SideBarSelection::new(std::str::from_utf8(&Asset::get("icon/info.svg").unwrap().data).unwrap().parse().unwrap(), "About", 4);

    let common_button = common_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 0;
        }
        ctx.request_layout();
    });

    let game_button = game_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 1;
        }
        ctx.request_layout();
    });

    let network_button = network_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 2;
        }
        ctx.request_layout();
    });

    let multiplayer_button = multiplayer_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 3;
        }
        ctx.request_layout();
    });

    let about_button = about_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 4;
        }
        ctx.request_layout();
    });

    let common_button = common_button
        .fix_height(32.0)
        .expand_width();

    let game_button = game_button
        .fix_height(32.0)
        .expand_width();

    let network_button = network_button
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
        .with_child(game_button)
        .with_spacer(4.0)
        .with_child(network_button)
        .with_spacer(4.0)
        .with_child(multiplayer_button)
        .with_spacer(4.0)
        .with_child(about_button)
        .with_spacer(4.0)
        .fix_width(128.0)
        .padding(Insets::uniform_xy(8.0, 0.0))
        .align_horizontal(UnitPoint::CENTER);

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}

fn build_right() -> impl Widget<AppState> {
    let mut children = HashMap::new();
    children.insert(0, Child::new(WidgetPod::new(Box::new(build_settings()))));
    children.insert(1, Child::new(WidgetPod::new(Box::new(build_game()))));

    let paged = PagedWidget::new(children, 128.0)
        .expand();

    paged
}

pub fn build() -> impl Widget<AppState> {
    let body = Flex::row()
        .with_child(build_left())
        .with_child(build_right());

    body
        .align_vertical(UnitPoint::TOP)
}