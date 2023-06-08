#![windows_subsystem = "windows"]
mod widget;
mod theme;

use rust_embed::RustEmbed;
use druid::widget::{Flex, Label};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Screen, UnitPoint, Widget, WidgetExt, WindowDesc, WindowState};
use crate::widget::{bottom_bar, window};
use crate::widget::window::WindowWidget;

// const VERTICAL_WIDGET_SPACING: f64 = 20.0;
// const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Akiraka - Internal build");

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct Asset;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    global_search_bar_input: String
}

fn main() {
    let scr_rect = Screen::get_monitors().get(0).unwrap().virtual_work_rect();
    let main_window = WindowDesc::new(WindowWidget::new(build_root_widget()))
        .title(WINDOW_TITLE)
        .with_min_size((600.0, 400.0))
        .window_size((600.0, 400.0))
        .set_position(((scr_rect.width() / 2.0) - 300.0, (scr_rect.height() / 2.0) - 200.0))
        .set_window_state(WindowState::Restored)
        .show_titlebar(false);

    let initial_state = AppState {
        global_search_bar_input: "".into()
    };

    AppLauncher::with_window(main_window)
        .configure_env(|_env, _state| {
            // TODO: 环境
            theme::theme::init(_env);
        })
        .launch(initial_state)
        .expect("Failed to launch application");
}

#[allow(unused_variables)]
fn build_root_widget() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, env: &Env| format!("Hello {}!", data.global_search_bar_input));
    // let text_box = TextBox::new()
    //     .with_placeholder("Who are we greeting?")
    //     .fix_width(TEXT_BOX_WIDTH)
    //     .lens(AppState::name);

    let layout = Flex::column()
        // .with_spacer(VERTICAL_WIDGET_SPACING)
        // .with_child(label)
        .with_child(bottom_bar::build());
        // .with_child(text_box);

    layout
        .with_spacer(window::TITLE_BAR_HEIGHT)
        .align_horizontal(UnitPoint::BOTTOM)
    // Align::centered(layout.align_vertical(UnitPoint::new(0.0, 0.0)))
}