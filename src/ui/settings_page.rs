#[allow(dead_code)]

use std::borrow::ToOwned;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::ptr::null;
use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, LocalizedString, Menu, MouseButton, PaintCtx, Point, RenderContext, Size, UnitPoint, UpdateCtx, Vec2, Widget, WidgetExt, WidgetPod};
use druid::keyboard_types::Key::Clear;
use druid::widget::{Axis, CrossAxisAlignment, Flex, FlexParams, Label, List, Svg, SvgData};
use crate::{animations, AppState, Asset};
use crate::theme::theme;
use crate::util::color_as_hex_string;
use crate::widget::button::Button;
use crate::widget::clear_button::ClearButton;
use crate::widget::side_bar_selection::SideBarSelection;

pub const ID: &str = "SETTINGS_PAGE";

const ANIMATION_TIME: f64 = 0.3;
static mut SELECTED: u64 = 0;


const ICON_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct IconClearButton {
    icon: Svg,
    data: String,
}

impl IconClearButton {
    pub fn new(data: String) -> IconClearButton {
        Self {
            icon: Svg::new(data.clone().replace("{color}", "#000000").parse::<SvgData>().unwrap()),
            data
        }
    }
}

impl<T: Data> Widget<T> for IconClearButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(event) => {
                if !ctx.is_disabled() && event.button == MouseButton::Left {
                    ctx.set_active(true);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.icon.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.icon.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let icon_bc = bc.shrink(Size::new(16.0, 16.0));
        let icon_size = self.icon.layout(ctx, &icon_bc, data, env);
        let button_size =
            bc.constrain(Size::new(
                icon_size.width,
                icon_size.height
            ));
        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let is_hot = ctx.is_hot();
        let size = ctx.size();
        let stroke_width = env.get(druid::theme::BUTTON_BORDER_WIDTH);

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
                .to_rounded_rect(env.get(druid::theme::BUTTON_BORDER_RADIUS));

        let bg_gradient = if is_active {
            env.get(theme::COLOR_CLEAR_BUTTON_ACTIVE)
        } else if is_hot {
            env.get(theme::COLOR_CLEAR_BUTTON_HOT)
        } else {
            env.get(theme::COLOR_CLEAR_BUTTON)
        };

        let border_color = if is_active {
            env.get(theme::COLOR_CLEAR_BUTTON_BORDER_ACTIVE)
        } else if is_hot {
            env.get(theme::COLOR_CLEAR_BUTTON_BORDER_HOT)
        } else {
            env.get(theme::COLOR_CLEAR_BUTTON_BORDER)
        };

        ctx.fill(rounded_rect, &bg_gradient);

        ctx.stroke(rounded_rect, &border_color, stroke_width);

        ctx.with_save(|ctx| {
            let svg_data = self.data.replace("{color}", color_as_hex_string(Color::from(env.get(theme::COLOR_TEXT))).as_str()).parse::<SvgData>().unwrap();
            self.icon = Svg::new(svg_data);
            self.icon.paint(ctx, data, env);
        });
    }
}

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
        let w = ctx.window().get_size().width - self.offset;
        // let h = ctx.window().get_size().height;
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

struct JavaInstance<T> {
    name: String,
    path: String,
    name_label: WidgetPod<T, Box<dyn Widget<T>>>,
    path_label: WidgetPod<T, Box<dyn Widget<T>>>,
    open: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T: Data> JavaInstance<T> {
    pub fn new(path: String) -> JavaInstance<T> {

        JavaInstance {
            name: path.clone(),
            path: path.clone(),
            name_label: WidgetPod::new(Box::new(Label::new(path.clone()))),
            path_label: WidgetPod::new(Box::new(Label::new(path.clone()))),
            open: WidgetPod::new(Box::new(
                IconClearButton::new(std::str::from_utf8(&Asset::get("icon/folder.svg").unwrap().data).unwrap().parse().unwrap())
                    .align_right()
                    .fix_size(32.0, 32.0)))
        }
    }
}

impl<T: Data> Widget<T> for JavaInstance<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.name_label.event(ctx, event, data, env);
        self.path_label.event(ctx, event, data, env);
        self.open.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }

        self.name_label.lifecycle(ctx, event, data, env);
        self.path_label.lifecycle(ctx, event, data, env);
        self.open.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.name_label.update(ctx, data, env);
        self.path_label.update(ctx, data, env);
        self.open.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let rect = bc.min();
        self.name_label.set_origin(ctx, Point::new(rect.width / 2.0, rect.height / 2.0));
        self.name_label.layout(ctx, bc, data, env);

        self.path_label.layout(ctx, bc, data, env);

        self.open.set_origin(ctx, Point::new(-8.0, 0.0));
        self.open.layout(ctx, bc, data, env);
        bc.min()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let is_hot = ctx.is_hot();
        let size = ctx.size();
        let stroke_width = env.get(druid::theme::BUTTON_BORDER_WIDTH);
        let rect = ctx.size().to_rect().to_rounded_rect(12.0);

        if is_hot {
            ctx.stroke(rect, &env.get(theme::COLOR_CLEAR_BUTTON_BORDER_HOT), 2.0);
        }

        if is_hot {
            ctx.fill(rect, &env.get(theme::COLOR_CLEAR_BUTTON_HOT));
        }

        self.name_label.paint(ctx, data, env);
        self.path_label.paint(ctx, data, env);
        self.open.paint(ctx, data, env);
    }
}

fn build_settings() -> impl Widget<AppState> {
    let game = Flex::column()
        .padding(Insets::uniform_xy(12.0, 12.0))
        .background(theme::COLOR_BACKGROUND_LIGHT)
        .border(theme::COLOR_BORDER_DARK, 1.0)
        .rounded(10.0)
        .expand_width()
        .align_left();

    let mut body = Flex::column()
        .with_child(Label::new(LocalizedString::new("None")).with_text_size(14.0).align_left())
        .with_spacer(8.0)
        .with_child(game)
        .padding(Insets::new(4.0, 4.0, 32.0, 4.0));

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}

fn build_game() -> impl Widget<AppState> {
    let java = Flex::column()
        .with_child(
            List::new(|| {
                JavaInstance::new("C:\\Users\\Arrokoth".parse().unwrap()).expand_width().fix_height(56.0).align_left()
        })
            .lens(AppState::java)
        )
        .with_spacer(12.0)
        .with_child(ClearButton::new("Test").fix_height(28.0).align_left())

        .with_spacer(4.0)
        .padding(Insets::uniform_xy(12.0, 12.0))
        .background(theme::COLOR_BACKGROUND_LIGHT)
        .border(theme::COLOR_BORDER_DARK, 1.0)
        .rounded(10.0)
        .expand_width()
        .align_left();

    let mut body = Flex::column()
        .with_child(Label::new(LocalizedString::new("Java Runtime")).with_text_size(14.0).align_left())
        .with_spacer(8.0)
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