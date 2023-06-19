use std::collections::HashMap;
use std::ffi::OsStr;
use std::iter::Map;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use druid::{BoxConstraints, Data, Env, Event, EventCtx, HasRawWindowHandle, InternalEvent, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, RawWindowHandle, Rect, RenderContext, Size, UpdateCtx, Vec2, Widget, WidgetExt, WidgetPod};
use std::ops::Fn;use lazy_static::lazy_static;
use winapi::shared::minwindef::LPARAM;
use crate::{AppState, Empty};
use crate::theme::theme;
use crate::ui::{bottom_bar, hello_page};
use crate::widget::title_bar::TitleBar;

pub const TITLE_BAR_HEIGHT: f64 = 44.0;

pub struct WindowWidget<T> {
    title_bar: WidgetPod<T, Box<dyn Widget<T>>>,
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    bottom_bar: WidgetPod<T, Box<dyn Widget<T>>>,
    // pages: HashMap<String, WidgetPod<T, Box<dyn Widget<T>>>>,
    pages: HashMap<String, Box<dyn Fn() -> dyn Widget<T>>>,
    page: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T: Data> WindowWidget<T> where TitleBar<AppState>: Widget<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> Self {
        let bottom_bar = bottom_bar::build();
        let inner = WidgetPod::<T, Box<dyn Widget<T>>>::new(Box::new(inner));
        let mut pages = HashMap::<String, Box<dyn Fn() -> dyn Widget<T>>>::new();
        Self {
            title_bar: WidgetPod::new(Box::new(TitleBar::new(TITLE_BAR_HEIGHT))),
            inner,
            bottom_bar: WidgetPod::new(Box::new(bottom_bar)),
            pages,
            page: WidgetPod::new(Box::new(Empty {}))
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
                // TODO: 跨平台
                #[cfg(target_os = "windows")]
                #[allow(unsafe_code)]
                {
                    use winapi::shared::windef::HWND;
                    use winapi::shared::windef::{HICON, HWND__};
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

        self.page.event(ctx, event, data, env);
        self.title_bar.event(ctx, event, data, env);
        self.inner.event(ctx, event, data, env);
        self.bottom_bar.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.page.lifecycle(ctx, event, data, env);
        self.title_bar.lifecycle(ctx, event, data, env);
        self.inner.lifecycle(ctx, event, data, env);
        self.bottom_bar.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.page.update(ctx, data, env);
        self.title_bar.update(ctx, data, env);
        self.inner.update(ctx, data, env);
        self.bottom_bar.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.page.layout(ctx, bc, data, env);

        let title_bar_bc = bc.loosen();
        self.title_bar.layout(ctx, &title_bar_bc, data, env);

        let bottom_bar_bc = bc.loosen();
        self.bottom_bar.set_origin(ctx, Point::new(0.0, ctx.window().get_size().height - bottom_bar::BOTTOM_BAR_HEIGHT));
        self.bottom_bar.layout(ctx, &bottom_bar_bc, data, env);

        let mut size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, (0.0, TITLE_BAR_HEIGHT).into());
        size.height += 0.0;

        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let page_id = unsafe { crate::PAGE_ID };
        if self.pages.contains_key(page_id) {
        }
        self.page.paint(ctx, data, env);

        self.title_bar.paint(ctx, data, env);
        self.inner.paint(ctx, data, env);
        self.bottom_bar.paint(ctx, data, env);
        let rect = Rect::new(0.0, 0.0, ctx.window().get_size().width, ctx.window().get_size().height);
        ctx.stroke(rect, &env.get(theme::COLOR_BORDER_LIGHT), 2.0);
    }
}