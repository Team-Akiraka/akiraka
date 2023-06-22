#![windows_subsystem = "windows"]
mod widget;
mod theme;
mod util;
mod ui;

use rust_embed::RustEmbed;
use druid::{AppDelegate, AppLauncher, BoxConstraints, Data, DelegateCtx, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, LocalizedString, PaintCtx, Screen, Size, UpdateCtx, Widget, WidgetPod, WindowDesc, WindowId, WindowState};
use crate::ui::hello_page;
use crate::widget::{paged_widget};
use crate::widget::window::WindowWidget;

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Akiraka - Internal build");

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Asset;

pub struct Delegate<T> {
    page: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T> Delegate<T> {
    pub fn new(root: impl Widget<T> + 'static) -> Delegate<T> {
        Delegate {
            page: WidgetPod::new(Box::new(root))
        }
    }

    pub fn switch_page(&mut self, page: impl Widget<T> + 'static) {
        self.page = WidgetPod::new(Box::new(page));
    }
}

impl<T: Data> AppDelegate<T> for Delegate<T> {
    fn event(&mut self, ctx: &mut DelegateCtx, window_id: WindowId, event: Event, data: &mut T, env: &Env) -> Option<Event> {
        Some(event)
    }
}

struct Empty {
}

impl<T: Data> Widget<T> for Empty {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        Size::ZERO
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    page_id: String,
    global_search_bar_input: String
}

pub static mut PAGE_ID: &str = hello_page::ID;

fn main() {
    let scr_rect = Screen::get_monitors().get(0).unwrap().virtual_work_rect();
    let main_window = WindowDesc::new(WindowWidget::new(paged_widget::PagedWidget::new()))
        .title(WINDOW_TITLE)
        .with_min_size((600.0, 400.0))
        .window_size((600.0, 400.0))
        .set_position(((scr_rect.width() / 2.0) - 300.0, (scr_rect.height() / 2.0) - 200.0))
        .set_window_state(WindowState::Restored)
        .show_titlebar(false);

    let initial_state = AppState {
        page_id: String::new(),
        global_search_bar_input: String::new()
    };

    let root = build_root_widget();
    AppLauncher::with_window(main_window)
        .configure_env(|_env, _state| {
            // TODO: 环境
            theme::theme::init(_env);
        })
        .delegate(Delegate::new(root))
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_empty_widget() -> impl Widget<AppState> {
    Empty {
    }
}

#[allow(unused_variables)]
fn build_root_widget() -> impl Widget<AppState> {
    // fn test<T: Widget<AppState>>(_: impl Fn() -> T) {
    // }
    //
    // let x = build_empty_widget;
    // // test(build_empty_widget);
    // test(|| {
    //     Empty {
    //     }
    // });

    // let label = Label::new(|data: &AppState, env: &Env| format!("Hello {}!", data.global_search_bar_input));

    // let text_box = TextBox::new()
    //     .with_placeholder("Who are we greeting?")
    //     .fix_width(TEXT_BOX_WIDTH)
    //     .lens(AppState::name);

    // let layout = Flex::column()
    //     .with_child(main_page::build());
    // .with_spacer(VERTICAL_WIDGET_SPACING)
    // .with_child(label)
    // .with_child(text_box);
    // .with_spacer(window::TITLE_BAR_HEIGHT);
    // .expand();

    // layout
    // .center()
    hello_page::build()
}