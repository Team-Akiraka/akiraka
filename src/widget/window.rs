use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use druid::{BoxConstraints, Data, Env, Event, EventCtx, HasRawWindowHandle, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RawWindowHandle, Rect, RenderContext, Size, UpdateCtx, Widget, WidgetPod};
use lazy_static::lazy_static;
use winapi::shared::minwindef::LPARAM;
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os = "windows")]
use winapi::shared::windef::{HICON, HWND__};
#[cfg(target_os = "windows")]
use winapi::um::libloaderapi::GetModuleHandleW;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{ICON_BIG, ICON_SMALL, IDI_APPLICATION, IMAGE_ICON, LoadImageW, LR_DEFAULTSIZE, LR_SHARED, LR_VGACOLOR, SendMessageW, WM_SETICON};
use crate::AppState;
use crate::theme::theme;
use crate::widget::title_bar::TitleBar;

pub const TITLE_BAR_HEIGHT: f64 = 44.0;

pub struct WindowWidget<T> {
    title_bar: WidgetPod<T, Box<dyn Widget<T>>>,
    inner: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T: Data> WindowWidget<T> where TitleBar<AppState>: druid::Widget<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> Self {
        Self {
            title_bar: WidgetPod::new(Box::new(TitleBar::new(TITLE_BAR_HEIGHT))),
            inner: WidgetPod::new(Box::new(inner)),
        }
    }
}

#[allow(unused_variables)]
impl<T: Data> Widget<T> for WindowWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::WindowConnected => {
                // TODO: 跨平台
                #[cfg(target_os = "windows")]
                #[allow(unsafe_code)]
                {
                    // use winapi::um::winuser::{GetWindowLongW, GWL_STYLE, HTCAPTION, ReleaseCapture, SC_MOVE, SendMessageA, SetWindowLongW, WM_SYSCOMMAND, WS_MAXIMIZEBOX};
                    if let RawWindowHandle::Win32(handle) = ctx.window().raw_window_handle() {
                        unsafe {
                            let hwnd = handle.hwnd as HWND;


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
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.title_bar.lifecycle(ctx, event, data, env);
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.title_bar.update(ctx, data, env);
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.title_bar.layout(ctx, bc, data, env);

        let mut size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, (0.0, TITLE_BAR_HEIGHT).into());
        size.height += 0.0;
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.title_bar.paint(ctx, data, env);
        self.inner.paint(ctx, data, env);
        let rect = Rect::new(0.0, 0.0, ctx.window().get_size().width, ctx.window().get_size().height);
        ctx.stroke(rect, &env.get(theme::COLOR_BORDER_LIGHT), 2.0);
    }
}