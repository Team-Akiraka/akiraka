use druid::{BoxConstraints, Color, Data, Env, Event, EventCtx, HasRawWindowHandle, ImageBuf, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, RawWindowHandle, Rect, RenderContext, Size, UpdateCtx, Widget, WidgetExt};
use druid::widget::{Button, Flex, Image, SizedBox};
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{ReleaseCapture, WM_SYSCOMMAND, SC_MOVE, HTCAPTION, GWL_STYLE, SendMessageA, WS_MAXIMIZEBOX, GetWindowLongW, SetWindowLongW};
use crate::widget::{icon_button};

#[allow(dead_code)]
struct TitleBarState {
}

pub struct TitleBar<T> {
    size: Size,
    fill: Color,
    col: SizedBox<T>
}

impl<T: druid::Data> TitleBar<T> {
    pub fn new() -> Self {
        // Button
        let exit_button = icon_button::IconButton::new(Image::new(ImageBuf::empty()), Size::new(40.0, 40.0));
        let col = Flex::column()
            .with_child(exit_button)
            .align_right()
            .fix_height(40.0);

        Self {
            size: Size::new(0.0, 0.0),
            fill: Color::rgba(1.0, 1.0, 1.0, 1.0),
            col
        }
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }
}

// https://www.pauljmiller.com/posts/druid-widget-tutorial.html
impl<T: Data> Widget<T> for TitleBar<T> {
    #[allow(unused_variables)]
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.col.event(ctx, event, data, env);
        match event {
            Event::MouseDown(mouse_event) => {
                let pos = mouse_event.pos;
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

    #[allow(unused_variables)]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.col.lifecycle(ctx, event, data, env);
    }

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        #[cfg(target_os = "windows")]
        #[allow(unsafe_code)]
        if let RawWindowHandle::Win32(handle) = ctx.window().raw_window_handle() {
            unsafe {
                SetWindowLongW(handle.hwnd as HWND, GWL_STYLE, GetWindowLongW(handle.hwnd as HWND, GWL_STYLE) & !WS_MAXIMIZEBOX as i32);
            }
        }
        self.col.update(ctx, old_data, data, env);
    }

    #[allow(unused_variables)]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.size.width = ctx.window().get_size().width;
        let self_child = Size::new(bc.max().width, self.size.height);

        self.col.layout(ctx, bc, data, env);

        self_child
    }

    #[allow(unused_variables)]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, self.size);
        ctx.fill(rect, &self.fill);

        ctx.with_save(|ctx| {
            self.col.paint(ctx, data, env);
        })
    }
}