
use std::ptr;
use druid::{BoxConstraints, Color, Data, Env, Event, EventCtx, HasRawWindowHandle, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, RawWindowHandle, Rect, RenderContext, Size, UpdateCtx, Widget, WidgetPod};
use lazy_static::lazy_static;
use crate::{AppState, Empty};
use crate::theme::theme;
use crate::ui::{bottom_bar};
use crate::widget::title_bar::TitleBar;

pub const TITLE_BAR_HEIGHT: f64 = 44.0;

struct Overlay<T> {
    child: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T: Data> Overlay<T> {
    fn new() -> Overlay<T> {
        Overlay {
            child: WidgetPod::new(Box::new(Empty {}))
        }
    }
}

impl<T: Data> Widget<T> for Overlay<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        ctx.window().get_size()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rect = ctx.size().to_rect();
        ctx.fill(rect, &Color::rgba8(0, 0, 0, 127));
    }
}

pub struct WindowWidget<T> {
    title_bar: WidgetPod<T, Box<dyn Widget<T>>>,
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    bottom_bar: WidgetPod<T, Box<dyn Widget<T>>>,
    is_overlay_showing: bool,
    overlay: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T: Data> WindowWidget<T> where TitleBar<AppState>: Widget<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> Self {
        let bottom_bar = bottom_bar::build();
        let inner = WidgetPod::<T, Box<dyn Widget<T>>>::new(Box::new(inner));
        Self {
            title_bar: WidgetPod::new(Box::new(TitleBar::new(TITLE_BAR_HEIGHT))),
            inner,
            bottom_bar: WidgetPod::new(Box::new(bottom_bar)),
            is_overlay_showing: false,
            overlay: WidgetPod::new(Box::new(Overlay::new()))
        }
    }


    pub fn set_inner(&mut self, inner: impl Widget<T> + 'static) {
        self.inner = WidgetPod::new(Box::new(inner));
    }
}

#[allow(unused_variables)]
impl<T: Data> Widget<T> for WindowWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::WindowConnected => {
                // TODO: Multiplatform
                #[cfg(target_os = "windows")]
                #[allow(unsafe_code)]
                {
                    use winapi::shared::windef::HWND;
                    use winapi::shared::windef::{HICON};
                    use winapi::um::libloaderapi::GetModuleHandleW;
                    use winapi::um::winuser::{ICON_BIG, ICON_SMALL, IDI_APPLICATION, IMAGE_ICON, LoadImageW, LR_DEFAULTSIZE, LR_SHARED, LR_VGACOLOR, SendMessageW, WM_SETICON, GetWindowLongW, GWL_STYLE, SetWindowLongW, WS_MAXIMIZEBOX};
                    if let RawWindowHandle::Win32(handle) = ctx.window().raw_window_handle() {
                        unsafe {
                            let hwnd = handle.hwnd as HWND;

                            SetWindowLongW(handle.hwnd as HWND, GWL_STYLE, GetWindowLongW(handle.hwnd as HWND, GWL_STYLE) & !WS_MAXIMIZEBOX as i32);

                            lazy_static! {
                                static ref PROGRAM_ICON: isize = unsafe {
                                    let h_instance = GetModuleHandleW(ptr::null());

                                    LoadImageW(
                                        h_instance,
                                        IDI_APPLICATION,
                                        IMAGE_ICON,
                                        0,
                                        0,
                                        LR_SHARED | LR_DEFAULTSIZE | LR_VGACOLOR
                                    ).cast::<HICON>() as isize
                                };
                            }

                            SendMessageW(
                                hwnd,
                                WM_SETICON,
                                ICON_SMALL as usize,
                                *PROGRAM_ICON
                            );

                            SendMessageW(
                                hwnd,
                                WM_SETICON,
                                ICON_BIG as usize,
                                *PROGRAM_ICON
                            );

                        }
                    }
                }
            }
            _ => {}
        }

        self.title_bar.event(ctx, event, data, env);
        if !self.is_overlay_showing {
            self.inner.event(ctx, event, data, env);
            self.bottom_bar.event(ctx, event, data, env);
        } else {
            self.overlay.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        unsafe { crate::widget::title_bar::SEARCH_ALLOWED = !self.is_overlay_showing; }
        self.title_bar.lifecycle(ctx, event, data, env);
        self.inner.lifecycle(ctx, event, data, env);
        self.bottom_bar.lifecycle(ctx, event, data, env);
        self.overlay.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.title_bar.update(ctx, data, env);
        self.inner.update(ctx, data, env);
        self.bottom_bar.update(ctx, data, env);
        self.overlay.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let title_bar_bc = bc.loosen();
        self.title_bar.layout(ctx, &title_bar_bc, data, env);

        let bottom_bar_bc = bc.loosen();
        self.bottom_bar.set_origin(ctx, Point::new(0.0, ctx.window().get_size().height - bottom_bar::BOTTOM_BAR_HEIGHT));
        self.bottom_bar.layout(ctx, &bottom_bar_bc, data, env);

        let mut size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, (0.0, TITLE_BAR_HEIGHT).into());
        size.height += 0.0;

        self.overlay.layout(ctx, bc, data, env);

        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.title_bar.paint(ctx, data, env);
        self.inner.paint(ctx, data, env);
        self.bottom_bar.paint(ctx, data, env);
        if self.is_overlay_showing {
            self.overlay.paint(ctx, data, env);
        }
        let rect = Rect::new(0.0, 0.0, ctx.window().get_size().width, ctx.window().get_size().height);
        ctx.stroke(rect, &env.get(theme::COLOR_BORDER_LIGHT), 2.0);
    }
}