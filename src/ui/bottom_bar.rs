use std::collections::HashMap;
use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LensExt, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, Point, RenderContext, Size, UpdateCtx, Vec2, WidgetExt, WidgetPod};
use druid::widget::{Widget, Flex, Radio, RadioGroup, Svg, SvgData, LineBreaking};
use crate::{animations, Asset};
use crate::theme::theme;
use crate::ui::{download_page, hello_page, instances_page, settings_page};
use crate::util::color_as_hex_string;
use crate::widget::launch_button::LaunchButton;
use crate::widget::profile_button::ProfileButton;

pub const BOTTOM_BAR_HEIGHT: f64 = 56.0;
pub const BOTTOM_BAR_HEIGHT_NAV: f64 = 40.0;

const ANIMATION_TIME: f64 = 0.5;
static mut SELECTED: u64 = 0;

const ICON_INSETS: Insets = Insets::uniform_xy(8., 2.);

struct IconClearButton {
    icon: Svg,
    data: String,
    id: String,
    activated: bool
}

impl IconClearButton {
    pub fn new(data: String, id: String) -> IconClearButton {
        Self {
            icon: Svg::new(data.clone().replace("{color}", "#000000").parse::<SvgData>().unwrap()),
            data,
            id,
            activated: false
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
        let icon_size = self.icon.layout(ctx, bc, data, env);
        let button_size =
            bc.constrain(Size::new(
                icon_size.width,
                icon_size.height
            ));
        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        unsafe {
            if crate::PAGE_ID == self.id {
                self.activated = true;
            } else {
                self.activated = false;
            }
        }
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
        } else if self.activated {
            env.get(theme::COLOR_CLEAR_BUTTON_ACTIVE)
        } else {
            env.get(theme::COLOR_CLEAR_BUTTON)
        };

        let border_color = if is_active {
            env.get(theme::COLOR_CLEAR_BUTTON_BORDER_ACTIVE)
        } else if is_hot {
            env.get(theme::COLOR_CLEAR_BUTTON_BORDER_HOT)
        } else if self.activated {
            env.get(theme::COLOR_CLEAR_BUTTON_BORDER_ACTIVE)
        } else {
            env.get(theme::COLOR_CLEAR_BUTTON_BORDER)
        };

        ctx.fill(rounded_rect, &bg_gradient);

        ctx.stroke(rounded_rect, &border_color, stroke_width);

        ctx.with_save(|ctx| {
            let svg_data = self.data.replace("{color}", color_as_hex_string(Color::from(env.get(crate::theme::theme::COLOR_TEXT))).as_str()).parse::<SvgData>().unwrap();
            self.icon = Svg::new(svg_data);
            self.icon.paint(ctx, data, env);
        });
    }
}

struct Child<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    height: f64
}

impl<T> Child<T> {
    fn new(inner: WidgetPod<T, Box<dyn Widget<T>>>, height: f64) -> Child<T> {
        Child {
            inner,
            height
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
    last_height: f64,
    inner_size: Size,
    t: f64
}

impl<T: Data> PagedWidget<T> {
    fn new(pages: HashMap<u64, Child<T>>) -> PagedWidget<T> {
        PagedWidget {
            pages,
            current_id: unsafe { SELECTED },
            last_height: 0.0,
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
        if self.detect_scene_change() {
            self.t = 0.0;
            ctx.window().request_anim_frame();
        }

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
                    ctx.request_layout();
                    ctx.request_paint();
                } else {
                    ctx.request_paint();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if self.detect_scene_change() {
            self.t = 0.0;
            ctx.window().request_anim_frame();
        }

        for x in self.pages.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        if self.detect_scene_change() {
            self.t = 0.0;
            ctx.window().request_anim_frame();
        }
        let x = self.pages.get_mut(&self.current_id);
        if x.is_some() {
            let x = x.unwrap().widget_mut().unwrap();
            x.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        if self.detect_scene_change() {
            self.t = 0.0;
            ctx.window().request_anim_frame();
        }

        let x = self.pages.get_mut(&self.current_id);
        if x.is_some() {
            let x = x.unwrap().widget_mut().unwrap();
            x.set_origin(ctx, Point::new(0.0, self.last_height));
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
            self.t = 0.0;
            ctx.window().request_anim_frame();
        }

        let x = self.pages.get_mut(&self.current_id);
        if x.is_some() {
            let x = x.unwrap();
            self.last_height = BOTTOM_BAR_HEIGHT - x.height;
            let s = if self.t / ANIMATION_TIME < 1.0 {
                let s = self.t / ANIMATION_TIME;

                animations::expo::ease_out(s)
            } else {
                1.0
            };
            let s = s / 4.0 + 0.75;
            let w = ctx.window().get_size().width / 2.0 - self.inner_size.width * s / 2.0;
            let h = ctx.window().get_size().height / 2.0 - self.inner_size.height * s / 2.0;

            let rect = ctx.size().to_rect();
            ctx.transform(Affine::translate(Vec2::new(0.0, (1.0 - s) * x.height + self.last_height)));
            ctx.stroke(rect, &env.get(theme::COLOR_BORDER_DARK), 1.0);
            ctx.fill(rect, &env.get(theme::COLOR_BACKGROUND_LIGHT));

            ctx.transform(Affine::translate(Vec2::new(0.0, -self.last_height)));
            x.inner.paint(ctx, data, env);
        }
    }
}

pub fn build_main<T: Data>() -> impl Widget<T> {
    let profile_button = ProfileButton::new()
        .fix_width(160.0)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    // List
    let list_button = crate::widget::icon_clear_button::IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/list.svg").unwrap().data).unwrap().parse::<String>().unwrap()
    );
        // .fix_width(crate::widget::window::TITLE_BAR_HEIGHT)
        // .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let list_button = list_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 1;
            crate::PAGE_ID = instances_page::ID;
        }
        ctx.request_anim_frame();
    });

    // Download
    let download_button = crate::widget::icon_clear_button::IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/download.svg").unwrap().data).unwrap().parse::<String>().unwrap()
    );
        // .fix_width(crate::widget::window::TITLE_BAR_HEIGHT)
        // .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let download_button = download_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 1;
            crate::PAGE_ID = download_page::ID;
        }
        ctx.request_anim_frame();
    });

    // Settings
    let settings_button = crate::widget::icon_clear_button::IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/settings.svg").unwrap().data).unwrap().parse::<String>().unwrap()
    );
        // .fix_width(crate::widget::window::TITLE_BAR_HEIGHT)
        // .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let settings_button = settings_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 1;
            crate::PAGE_ID = settings_page::ID;
        }
        ctx.request_anim_frame();
    });

    let launch_button = LaunchButton::new(
        std::str::from_utf8(&Asset::get("icon/play_slim.svg").unwrap().data).unwrap().parse::<String>().unwrap(),
        "Launch"
    )
        .fix_width(160.0)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let bar = Flex::row()
        .with_child(profile_button)
        .with_flex_spacer(1.0)
        .with_child(list_button)
        .with_spacer(8.0)
        .with_child(download_button)
        .with_spacer(8.0)
        .with_child(settings_button)
        .with_flex_spacer(1.0)
        .with_child(launch_button)
        .center()
        .padding(Insets::new(12.0, 6.0, 12.0, 6.0))
        .fix_height(BOTTOM_BAR_HEIGHT)
        .expand_width();
    bar
}

pub fn build_nav<T: Data>() -> impl Widget<T> {
    // Home
    let home_button = IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/home.svg").unwrap().data).unwrap().parse::<String>().unwrap(),
        hello_page::ID.parse().unwrap()
    );

    let home_button = home_button.on_click(|ctx, _data, _env| {
        unsafe {
            SELECTED = 0;
            crate::PAGE_ID = hello_page::ID;
        }
        ctx.request_anim_frame();
    });
    // List
    let list_button = IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/list.svg").unwrap().data).unwrap().parse::<String>().unwrap(),
        instances_page::ID.parse().unwrap()
    );

    let list_button = list_button.on_click(|ctx, _data, _env| {
        unsafe {
            crate::PAGE_ID = instances_page::ID;
        }
        ctx.request_anim_frame();
    });

    // Download
    let download_button = IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/download.svg").unwrap().data).unwrap().parse::<String>().unwrap(),
        download_page::ID.parse().unwrap()
    );

    let download_button = download_button.on_click(|ctx, _data, _env| {
        unsafe {
            crate::PAGE_ID = download_page::ID;
        }
        ctx.request_anim_frame();
    });

    // Settings
    let settings_button = IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/settings.svg").unwrap().data).unwrap().parse::<String>().unwrap(),
        settings_page::ID.parse().unwrap()
    );

    let settings_button = settings_button.on_click(|ctx, _data, _env| {
        unsafe {
            crate::PAGE_ID = settings_page::ID;
        }
        ctx.request_anim_frame();
    });

    let bar = Flex::row()
        .with_child(home_button)
        .with_spacer(8.0)
        .with_child(list_button)
        .with_spacer(8.0)
        .with_child(download_button)
        .with_spacer(8.0)
        .with_child(settings_button)
        .with_flex_spacer(1.0)
        .center()
        .padding(Insets::new(12.0, 4.0, 12.0, 4.0))
        .fix_height(BOTTOM_BAR_HEIGHT_NAV)
        .expand_width();
    bar
}

pub fn build<T: Data>() -> impl Widget<T> {
    let mut pages = HashMap::<u64, Child<T>>::new();
    pages.insert(0, Child::new(WidgetPod::new(Box::new(build_main())), BOTTOM_BAR_HEIGHT));
    pages.insert(1, Child::new(WidgetPod::new(Box::new(build_nav())), BOTTOM_BAR_HEIGHT_NAV));

    PagedWidget::new(pages)
        .expand_width()
}