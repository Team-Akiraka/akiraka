use druid::{BoxConstraints, Color, Data, Env, Event, EventCtx, HasRawWindowHandle, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, Point, RawWindowHandle, Rect, RenderContext, Size, UpdateCtx, Widget, WidgetPod};
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetWindowLongW, GWL_STYLE, HTCAPTION, ReleaseCapture, SC_MOVE, SendMessageA, SetWindowLongW, WM_SYSCOMMAND, WS_MAXIMIZEBOX};

struct TitleBarButton {
    size: f64,
    fill: Color
}

impl TitleBarButton {
    pub fn new(size: f64) -> Self {
        Self {
            size,
            fill: Color::rgb(0.5, 0.0, 1.0)
        }
    }
}

#[allow(unused_variables)]
impl<T: Data> Widget<T> for TitleBarButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
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
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        Size::new(self.size, self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, Size::new(self.size, self.size));
        if ctx.is_active() {
            self.fill = Color::rgb8(255, 0, 0);
        } else if ctx.is_hot() {
            self.fill = Color::rgb8(0, 255, 0);
        } else {
            self.fill = Color::rgb8(0, 0, 255);
        }
        ctx.fill(rect, &self.fill);
    }
}

pub struct TitleBar<T> {
    fill: Color,
    height: f64,
    exit_button: WidgetPod<T, Box<dyn Widget<T>>>,
    minimize_button: WidgetPod<T, Box<dyn Widget<T>>>
}

#[allow(unused_variables)]
impl<T: Data> TitleBar<T> {
    pub fn new(height: f64, fill: Color) -> Self {
        let exit_button = TitleBarButton::new(height);
        let minimize_button = TitleBarButton::new(height);

        Self {
            fill,
            height,
            exit_button: WidgetPod::new(Box::new(exit_button)),
            minimize_button: WidgetPod::new(Box::new(minimize_button))
        }
    }
}

#[allow(unused_variables)]
impl<T: Data> Widget<T> for TitleBar<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                if !(self.exit_button.is_hot() || self.minimize_button.is_hot()) {
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
                }
            }
            Event::MouseUp(mouse_event) => {
            }
            Event::MouseMove(mouse_event) => {
            }
            _ => {}
        }

        self.exit_button.event(ctx, event, data, env);
        self.minimize_button.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.exit_button.lifecycle(ctx, event, data, env);
        self.minimize_button.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.exit_button.update(ctx, data, env);
        self.minimize_button.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.exit_button.set_origin(ctx, Point::new(bc.max().width - self.height, 0.0));
        bc.constrain(self.exit_button.layout(ctx, bc, data, env));
        self.minimize_button.set_origin(ctx, Point::new(bc.max().width - self.height * 2.0, 0.0));
        bc.constrain(self.minimize_button.layout(ctx, bc, data, env));

        Size::new(bc.max().width, self.height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, Size::new(ctx.window().get_size().width, self.height));
        ctx.fill(rect, &self.fill);

        self.exit_button.paint(ctx, data, env);
        self.minimize_button.paint(ctx, data, env);
    }
}