use std::fmt::{Formatter, LowerHex};
use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, HasRawWindowHandle, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, LocalizedString, MouseButton, PaintCtx, Point, RawWindowHandle, Rect, RenderContext, Size, TextAlignment, UpdateCtx, Vec2, Widget, WidgetExt, WidgetPod, WindowState};
use druid::widget::{Align, LensWrap, Svg, SvgData, TextBox};
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetWindowLongW, GWL_STYLE, HTCAPTION, ReleaseCapture, SC_MOVE, SendMessageA, SetWindowLongW, WM_SYSCOMMAND, WS_MAXIMIZEBOX};
use crate::app_state_derived_lenses::{global_search_bar_input};
use crate::{AppState, Asset};
use crate::theme::theme;

struct DraggableArea {
    height: f64
}

impl DraggableArea {
    pub fn new(height: f64) -> Self {
        Self {
            height
        }
    }
}

#[allow(unused_variables)]
impl<T> Widget<T> for DraggableArea {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                // TODO: 跨平台
                #[cfg(target_os = "windows")]
                #[allow(unsafe_code)]
                if let RawWindowHandle::Win32(handle) = ctx.window().raw_window_handle() {
                    unsafe {
                        SetWindowLongW(handle.hwnd as HWND, GWL_STYLE, GetWindowLongW(handle.hwnd as HWND, GWL_STYLE) & !WS_MAXIMIZEBOX as i32);
                        ReleaseCapture();
                        SendMessageA(handle.hwnd as HWND, WM_SYSCOMMAND, SC_MOVE + (HTCAPTION as usize), 0);
                    }
                }
                ctx.request_paint();
            }
            Event::MouseUp(mouse_event) => {
            }
            Event::MouseMove(mouse_event) => {
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        Size::new(bc.max().width, self.height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, ctx.size());
        ctx.fill(rect, &Color::rgba8(255, 255,255, 0));
    }
}

fn color_as_hex_string(color: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", color.as_rgba8().0, color.as_rgba8().1, color.as_rgba8().2).parse().unwrap()
}

struct TitleBarButton {
    size: f64,
    icon: Svg,
    data: String,
}

impl TitleBarButton {
    pub fn new(size: f64, data: String) -> Self {
        Self {
            size,
            icon: Svg::new(data.clone().replace("{color}", "#000000").parse::<SvgData>().unwrap()),
            data,
        }
    }
}

#[allow(unused_variables)]
impl<T: Data> Widget<T> for TitleBarButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.icon.event(ctx, event, data, env);
        match event {
            Event::MouseDown(event) => {
                if event.button == MouseButton::Left && !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(event) => {
                if event.button == MouseButton::Left && ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.icon.lifecycle(ctx, event, data, env);
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.icon.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.constrain(self.icon.layout(ctx, bc, data, env));
        Size::new(self.size, self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, Size::new(self.size, self.size));
        if ctx.is_active() {
            ctx.fill(rect, &env.get(theme::COLOR_TITLE_BAR_BUTTON_ACTIVE));
        } else if ctx.is_hot() {
            ctx.fill(rect, &env.get(theme::COLOR_TITLE_BAR_BUTTON_HOT));
        } else {
            ctx.fill(rect, &env.get(theme::COLOR_TITLE_BAR_BUTTON));
        }

        ctx.with_save(|ctx| {
            ctx.transform(Affine::scale(0.5).then_translate(Vec2::new(12.0 - 1.0, 12.0 - 1.0)));
            // println!("{:?}", color_as_hex_string(Color::from(env.get(theme::COLOR_TEXT))));
            self.icon = Svg::new(self.data.replace("{color}", color_as_hex_string(Color::from(env.get(theme::COLOR_TEXT))).as_str()).parse::<SvgData>().unwrap());
            self.icon.paint(ctx, data, env);
        });
    }
}

pub struct TitleBar<T> {
    height: f64,
    draggable_area: WidgetPod<T, Box<dyn Widget<T>>>,
    exit_button: WidgetPod<T, Box<dyn Widget<T>>>,
    minimize_button: WidgetPod<T, Box<dyn Widget<T>>>,
    search_bar: WidgetPod<T, Box<dyn Widget<T>>>
}

#[allow(unused_variables)]
impl<T: Data> TitleBar<T> where LensWrap<AppState, String, global_search_bar_input, TextBox<String>>: Widget<T> {
    pub fn new(height: f64) -> Self {
        // let raw_img = Asset::get("close.png").unwrap().data;
        // let img_data = image::load_from_memory(&raw_img).unwrap();
        // let rgb_img = img_data.to_rgba8();
        // let img_size = rgb_img.dimensions();
        // let img_buf = ImageBuf::from_raw(
        //     rgb_img.to_vec(),
        //     ImageFormat::RgbaPremul,
        //     img_size.0 as usize,
        //     img_size.1 as usize
        // );
        // let exit_button = TitleBarButton::new(height, Image::new(img_buf));

        // let svg = std::str::from_utf8(&Asset::get("icon/close.svg").unwrap().data).unwrap().replace("{color}", "#000000").parse::<SvgData>().unwrap();
        let svg = std::str::from_utf8(&Asset::get("icon/close.svg").unwrap().data).unwrap().parse::<String>().unwrap();
        let exit_button = TitleBarButton::new(height, svg.clone())
            .on_click(|ctx, t: &mut T, env| {
                ctx.window().clone().close();
            });

        // let svg = std::str::from_utf8(&Asset::get("icon/minimize.svg").unwrap().data).unwrap().replace("{color}", "#000000").parse::<SvgData>().unwrap();
        let svg = std::str::from_utf8(&Asset::get("icon/minimize.svg").unwrap().data).unwrap().parse::<String>().unwrap();
        let minimize_button = TitleBarButton::new(height, svg.clone())
            .on_click(|ctx, t: &mut T, env| {
                ctx.window().clone().set_window_state(WindowState::Minimized);
            });

        // TODO: 搜索栏
        let search_bar = TextBox::new()
            // .with_placeholder(LocalizedString::new("Type here for a global search"))
            .with_placeholder(LocalizedString::new("Coming soon!"))
            .with_text_size(13.0)
            .with_text_alignment(TextAlignment::Start)
            .lens(AppState::global_search_bar_input)
            .padding(Insets::uniform(8.0));

        Self {
            height,
            draggable_area: WidgetPod::new(Box::new(DraggableArea::new(height))),
            exit_button: WidgetPod::new(Box::new(exit_button)),
            minimize_button: WidgetPod::new(Box::new(minimize_button)),
            search_bar: WidgetPod::new(Box::new(search_bar))
        }
    }
}

#[allow(unused_variables)]
impl<T: Data> Widget<T> for TitleBar<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.exit_button.event(ctx, event, data, env);
        self.minimize_button.event(ctx, event, data, env);
        self.search_bar.event(ctx, event, data, env);

        if !(self.exit_button.is_hot() || self.minimize_button.is_hot() || self.search_bar.is_hot()) {
            self.draggable_area.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.draggable_area.lifecycle(ctx, event, data, env);
        self.exit_button.lifecycle(ctx, event, data, env);
        self.minimize_button.lifecycle(ctx, event, data, env);
        self.search_bar.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.draggable_area.update(ctx, data, env);
        self.exit_button.update(ctx, data, env);
        self.minimize_button.update(ctx, data, env);
        self.search_bar.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.draggable_area.set_origin(ctx, Point::ORIGIN);
        bc.constrain(self.draggable_area.layout(ctx, bc, data, env));

        self.exit_button.set_origin(ctx, Point::new(bc.max().width - self.height, 0.0));
        bc.constrain(self.exit_button.layout(ctx, bc, data, env));
        self.minimize_button.set_origin(ctx, Point::new(bc.max().width - self.height * 2.0, 0.0));
        bc.constrain(self.minimize_button.layout(ctx, bc, data, env));
        self.search_bar.set_origin(ctx, Point::new(bc.max().width / 2.0 - 160.0, 0.0));
        bc.constrain(self.search_bar.layout(ctx,
                                            &BoxConstraints::new(
                                                Size::new(320.0, self.height),
                                                Size::new(320.0, self.height))
                                            , data, env));

        Size::new(bc.max().width, self.height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, Size::new(ctx.window().get_size().width, self.height));
        ctx.fill(rect, &env.get(theme::COLOR_PRIMARY_TITLE_BAR));

        self.draggable_area.paint(ctx, data, env);
        self.exit_button.paint(ctx, data, env);
        self.minimize_button.paint(ctx, data, env);
        self.search_bar.paint(ctx, data, env);
    }
}