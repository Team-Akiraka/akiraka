mod widget;

use std::fmt::format;
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt, Screen, Color, KeyOrValue, Rect};
use crate::widget::TitleBar;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn main() {
    let scr_rect = Screen::get_monitors().get(0).unwrap().virtual_work_rect();
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .with_min_size((600.0, 400.0))
        .window_size((600.0, 400.0))
        .set_position(((scr_rect.width() / 2.0) - 300.0, (scr_rect.height() / 2.0) - 200.0))
        .show_titlebar(false);
    println!("{:?}", (scr_rect.width() / 2.0, scr_rect.height() / 2.0));

    let initial_state = HelloState {
        name: "World".into(),
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, state| {
            // TODO: 环境
        })
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, env: &Env| format!("Hello {}!", data.name));
    // a text_box that modifies `name`.
    let text_box = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    let mut label2 = Label::new(|data: &HelloState, env: &Env| "Are you ok?");
    label2.set_text_color(<druid::Color as Into<Color>>::into(Color::rgb8(255, 0, 0)));

    let mut title_bar = TitleBar::new();

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(title_bar)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(text_box)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(label2);

    // center the two widgets in the available space
    Align::centered(layout)
}