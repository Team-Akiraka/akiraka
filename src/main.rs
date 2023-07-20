#![windows_subsystem = "windows"]
mod animations;
mod theme;
mod ui;
mod util;
#[allow(dead_code)]
mod widget;

use crate::ui::hello_page;
use crate::widget::paged_widget;
use crate::widget::window::WindowWidget;
use druid::im::Vector;
use druid::{
    AppDelegate, AppLauncher, BoxConstraints, Command, Data, DelegateCtx, Env, Event, EventCtx,
    Handled, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, LocalizedString, PaintCtx, Screen, Size,
    Target, UpdateCtx, Widget, WidgetPod, WindowDesc, WindowId, WindowState,
};
use rust_embed::RustEmbed;

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Akiraka - Internal build");

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Asset;

pub struct Delegate {
    page: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
}

impl Delegate {
    pub fn new(root: impl Widget<AppState> + 'static) -> Delegate {
        Delegate {
            page: WidgetPod::new(Box::new(root)),
        }
    }
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        _ctx: &mut DelegateCtx,
        _window_id: WindowId,
        event: Event,
        data: &mut AppState,
        _env: &Env,
    ) -> Option<Event> {
        match event {
            Event::WindowConnected => {
                data.java.push_back("java.exe".parse().unwrap());
            }
            _ => {}
        }
        Some(event)
    }

    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
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

struct Empty {}

impl<T: Data> Widget<T> for Empty {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        Size::ZERO
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &T, _env: &Env) {}
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub page_id: String,
    pub global_search_bar_input: String,
    pub java: Vector<String>,
    pub file_open_type: String,
    pub minecraft_versions: Vector<String>,
}

pub static mut PAGE_ID: &str = hello_page::ID;

fn main() {
    let scr_rect = Screen::get_monitors().get(0).unwrap().virtual_work_rect();
    let main_window = WindowDesc::new(WindowWidget::new(paged_widget::PagedWidget::new()))
        .title(WINDOW_TITLE)
        .with_min_size((600.0, 400.0))
        .window_size((600.0, 400.0))
        .set_position((
            (scr_rect.width() / 2.0) - 300.0,
            (scr_rect.height() / 2.0) - 200.0,
        ))
        .set_window_state(WindowState::Restored)
        .show_titlebar(false);

    let mut x = Vector::<String>::new();
    x.push_back("114514".parse().unwrap());

    let mut initial_state = AppState {
        page_id: String::new(),
        global_search_bar_input: String::new(),
        java: Vector::<String>::new(),
        file_open_type: String::new(),
        minecraft_versions: x,
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
    Empty {}
}

#[allow(unused_variables)]
fn build_root_widget() -> impl Widget<AppState> {
    hello_page::build()
}
