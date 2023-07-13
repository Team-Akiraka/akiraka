#![windows_subsystem = "windows"]
#[allow(dead_code)]

mod widget;
mod theme;
mod util;
mod ui;
mod animations;

use std::ops::Add;
use rust_embed::RustEmbed;
use druid::{AppDelegate, AppLauncher, BoxConstraints, Command, Data, DelegateCtx, Env, Event, EventCtx, Handled, im, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, LocalizedString, PaintCtx, Screen, Size, Target, UpdateCtx, Widget, WidgetPod, WindowDesc, WindowId, WindowState};
use druid::im::Vector;
use crate::ui::hello_page;
use crate::widget::{paged_widget};
use crate::widget::window::WindowWidget;

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Akiraka - Internal build");

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Asset;

pub struct Delegate {
    page: WidgetPod<AppState, Box<dyn Widget<AppState>>>
}

impl Delegate {
    pub fn new(root: impl Widget<AppState> + 'static) -> Delegate {
        Delegate {
            page: WidgetPod::new(Box::new(root))
        }
    }

    pub fn switch_page(&mut self, page: impl Widget<AppState> + 'static) {
        self.page = WidgetPod::new(Box::new(page));
    }
}

impl AppDelegate<AppState> for Delegate {
    fn event(&mut self, _ctx: &mut DelegateCtx, _window_id: WindowId, event: Event, _data: &mut AppState, _env: &Env) -> Option<Event> {
        Some(event)
    }

    fn command(&mut self, ctx: &mut DelegateCtx, target: Target, cmd: &Command, data: &mut AppState, env: &Env) -> Handled {
        if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {
            if data.file_open_type == "JAVA_FILE_OPEN" {
                let path = file_info.path.as_path();
                data.java.push_back(path.to_str().unwrap().parse().unwrap());

                Handled::Yes
            } else {
                Handled::No
            }
        } else {
            Handled::No
        }
    }
}

struct Empty {
}

impl<T: Data> Widget<T> for Empty {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        Size::ZERO
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &T, _env: &Env) {
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    page_id: String,
    global_search_bar_input: String,
    java: im::Vector<String>,
    file_open_type: String
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

    let mut initial_state = AppState {
        page_id: String::new(),
        global_search_bar_input: String::new(),
        java: im::Vector::<String>::new(),
        file_open_type: String::new()
    };
    initial_state.java.append(Vector::new());

    let root = build_root_widget();
    AppLauncher::with_window(main_window)
        .configure_env(|_env, _state| {
            // TODO: Environment
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