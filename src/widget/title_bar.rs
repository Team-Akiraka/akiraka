use druid::{BoxConstraints, Color, Data, Env, Event, EventCtx, HasRawWindowHandle, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, Point, RawWindowHandle, Rect, RenderContext, Size, UpdateCtx, Widget};
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetWindowLongW, GWL_STYLE, HTCAPTION, ReleaseCapture, SC_MOVE, SendMessageA, SetWindowLongW, WM_SYSCOMMAND, WS_MAXIMIZEBOX};

struct TitleBarButton {
    size: f64
}

impl TitleBarButton {
    pub fn new(size: f64) -> Self {
        Self {
            size
        }
    }
}

impl<T: Data> Widget<T> for TitleBarButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        Size::new(self.size, self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
    }
}

pub struct TitleBar {
    fill: Color,
    height: f64
}

impl TitleBar {
    pub fn new(height: f64) -> Self {
        Self {
            fill: Color::rgb(0.5, 0.6, 1.0),
            height
        }
    }
}

impl<T: Data> Widget<T> for TitleBar {
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
        let rect = Rect::from_origin_size(Point::ORIGIN, Size::new(ctx.window().get_size().width, self.height));
        ctx.fill(rect, &self.fill);
    }
}