use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, HasRawWindowHandle, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, RawWindowHandle, Rect, RenderContext, Size, UpdateCtx, Widget};
use druid::widget::{Button};
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{SendMessageW, ReleaseCapture, WM_SYSCOMMAND, SC_MOVE, HTCAPTION};
use winapi::um::winuser::SendMessageA;

#[allow(dead_code)]
struct TitleBarState {
}

pub struct TitleBar<T> {
    size: Size,
    dragging: bool,
    fill: Color,
    exit_button: Button<T>
}

impl<T: druid::Data> TitleBar<T> {
    pub fn new() -> Self {
        let exit_button = Button::new("X");
        Self {
            size: Size::new(0.0, 0.0),
            dragging: false,
            fill: Color::rgba(1.0, 1.0, 1.0, 1.0),
            exit_button
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
        match event {
            Event::MouseDown(mouse_event) => {
                self.dragging = true;
            }
            Event::MouseUp(mouse_event) => {
                self.dragging = false;
            }
            Event::MouseMove(mouse_event) => {
                let pos = mouse_event.pos;
                if self.dragging {
                    // TODO: 跨平台
                    #[cfg(target_os = "windows")]
                    #[allow(unsafe_code)]
                    if let RawWindowHandle::Win32(handle) = ctx.window().raw_window_handle() {
                        unsafe {
                            ReleaseCapture();
                            SendMessageA(handle.hwnd as HWND, WM_SYSCOMMAND, SC_MOVE + (HTCAPTION as usize), 0);
                        }
                    }
                }
            }
            _ => {}
        }
        // self.exit_button.event(ctx, event, data, env);
    }

    #[allow(unused_variables)]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        // self.exit_button.lifecycle(ctx, event, data, env);
    }

    #[allow(unused_variables)]
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        // self.exit_button.update(ctx, old_data, data, env);
    }

    #[allow(unused_variables)]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.size.width = ctx.window().get_size().width;
        let self_child = Size::new(bc.max().width, self.size.height);

        // self.exit_button.layout(ctx, bc, data, env);

        return self_child;
    }

    #[allow(unused_variables)]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, self.size);
        ctx.fill(rect, &self.fill);

        ctx.with_save(|ctx| {
            // self.exit_button.paint(ctx, data, env);
        })
    }
}