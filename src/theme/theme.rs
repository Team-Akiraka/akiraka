use druid::{Color, Env, Insets, Key, RoundedRectRadii, theme};

// 颜色
pub const COLOR_WINDOW_BACKGROUND: Key<Color> = theme::WINDOW_BACKGROUND_COLOR;
pub const COLOR_WINDOW_BORDER: Key<Color> = Key::new("team.akiraka.theme.color.window.border");

pub const COLOR_PRIMARY_TITLE_BAR: Key<Color> = Key::new("team.akiraka.theme.color.primary.titlebar");
pub const COLOR_PRIMARY_LIGHT: Key<Color> = theme::PRIMARY_LIGHT;
pub const COLOR_PRIMARY_DARK: Key<Color> = theme::PRIMARY_DARK;

pub const COLOR_BACKGROUND_LIGHT: Key<Color> = theme::BACKGROUND_LIGHT;
pub const COLOR_BACKGROUND_DARK: Key<Color> = theme::BACKGROUND_DARK;

pub const COLOR_FOREGROUND_LIGHT: Key<Color> = theme::FOREGROUND_LIGHT;
pub const COLOR_FOREGROUND_DARK: Key<Color> = theme::FOREGROUND_DARK;

pub const COLOR_TEXT: Key<Color> = theme::TEXT_COLOR;
pub const COLOR_DISABLED_TEXT: Key<Color> = theme::DISABLED_TEXT_COLOR;
pub const COLOR_PLACEHOLDER_TEXT: Key<Color> = theme::PLACEHOLDER_COLOR;

pub const COLOR_BUTTON: Key<Color> = Key::new("team.akiraka.theme.color.button");
pub const COLOR_BUTTON_PRIMARY: Key<Color> = Key::new("team.akiraka.theme.color.button.primary");
pub const COLOR_BUTTON_HOT: Key<Color> = Key::new("team.akiraka.theme.color.button.hot");
pub const COLOR_BUTTON_ACTIVE: Key<Color> = Key::new("team.akiraka.theme.color.button.active");
pub const COLOR_BUTTON_DISABLED: Key<Color> = Key::new("team.akiraka.theme.color.button.disabled");
pub const COLOR_BUTTON_BORDER: Key<Color> = Key::new("team.akiraka.theme.color.button.border");
pub const COLOR_BUTTON_BORDER_HOT: Key<Color> = Key::new("team.akiraka.theme.color.button.border.hot");
pub const COLOR_BUTTON_BORDER_ACTIVE: Key<Color> = Key::new("team.akiraka.theme.color.button.border.active");

pub const COLOR_TITLE_BAR_BUTTON: Key<Color> = Key::new("team.akiraka.theme.color.title_bar.button");
pub const COLOR_TITLE_BAR_BUTTON_HOT: Key<Color> = Key::new("team.akiraka.theme.color.title_bar.button.hot");
pub const COLOR_TITLE_BAR_BUTTON_ACTIVE: Key<Color> = Key::new("team.akiraka.theme.color.title_bar.button.active");

pub const COLOR_CLEAR_BUTTON: Key<Color> = Key::new("team.akiraka.theme.color.clear.button");
pub const COLOR_CLEAR_BUTTON_HOT: Key<Color> = Key::new("team.akiraka.theme.color.clear.button.hot");
pub const COLOR_CLEAR_BUTTON_ACTIVE: Key<Color> = Key::new("team.akiraka.theme.color.clear.button.active");
pub const COLOR_CLEAR_BUTTON_BORDER: Key<Color> = Key::new("team.akiraka.theme.color.clear.border.button");
pub const COLOR_CLEAR_BUTTON_BORDER_HOT: Key<Color> = Key::new("team.akiraka.theme.color.clear.button.border.hot");
pub const COLOR_CLEAR_BUTTON_BORDER_ACTIVE: Key<Color> = Key::new("team.akiraka.theme.color.clear.button.border.active");

pub const COLOR_BORDER_LIGHT: Key<Color> = theme::BORDER_LIGHT;
pub const COLOR_BORDER_DARK: Key<Color> = theme::BORDER_DARK;

pub const COLOR_CURSOR: Key<Color> = theme::CURSOR_COLOR;

// 数值
pub const BORDER_WIDTH_WINDOW: Key<f64> = Key::new("team.akiraka.theme.border.width.window");
pub const BORDER_WIDTH_BUTTON: Key<f64> = theme::BUTTON_BORDER_WIDTH;
pub const BORDER_WIDTH_TEXTBOX: Key<f64> = theme::TEXTBOX_BORDER_WIDTH;

pub const RADIUS_BUTTON_BORDER: Key<RoundedRectRadii> = theme::BUTTON_BORDER_RADIUS;
pub const RADIUS_TEXTBOX_BORDER: Key<RoundedRectRadii> = theme::TEXTBOX_BORDER_RADIUS;

pub const INSETS_TEXTBOX: Key<Insets> = theme::TEXTBOX_INSETS;

pub fn init(env: &mut Env) {
    env.set(COLOR_WINDOW_BACKGROUND, Color::rgba8(0xf9, 0xf9, 0xf9, 0xff));
    env.set(COLOR_WINDOW_BORDER, Color::rgba8(0x00, 0x00,0x00, 0x3f));

    env.set(COLOR_PRIMARY_TITLE_BAR, Color::rgba8(0x94, 0xfc, 0xb2, 0x00));
    env.set(COLOR_PRIMARY_LIGHT, Color::rgba8(0x1d, 0x9b, 0x30, 0xff));
    env.set(COLOR_PRIMARY_DARK, Color::rgba8(0x18, 0x81, 0x28, 0xff));

    env.set(COLOR_BACKGROUND_LIGHT, Color::rgba8(0xf9, 0xf9, 0xf9, 0xff));
    env.set(COLOR_BACKGROUND_DARK, Color::rgba8(0xef, 0xef, 0xef, 0xff));

    env.set(COLOR_FOREGROUND_LIGHT, Color::rgba8(255, 255, 255, 255));
    env.set(COLOR_FOREGROUND_DARK, Color::rgba8(255, 255, 255, 255));

    env.set(COLOR_TEXT, Color::rgba8(0x2a, 0x2a, 0x2a, 0xff));
    env.set(COLOR_DISABLED_TEXT, Color::rgba8(0xaf, 0xaf, 0xaf, 0xff));
    env.set(COLOR_PLACEHOLDER_TEXT, Color::rgba8(0x8f, 0x8f, 0x8f, 0xff));

    env.set(COLOR_BUTTON, Color::rgba8(0x2f, 0xff, 0x6a, 0x00));
    env.set(COLOR_BUTTON_PRIMARY, Color::rgba8(0x2f, 0xff, 0x6a, 0x1f));
    env.set(COLOR_BUTTON_HOT, Color::rgba8(0x2f, 0xff, 0x6a, 0x3f));
    env.set(COLOR_BUTTON_ACTIVE, Color::rgba8(0x2f, 0xff, 0x6a, 0x1f));
    env.set(COLOR_BUTTON_DISABLED, Color::rgba8(0x2f, 0xff, 0x6a, 0x1f));
    env.set(COLOR_BUTTON_BORDER, Color::rgba8(0x00, 0x00,0x00, 0x3f));
    env.set(COLOR_BUTTON_BORDER_HOT, Color::rgba8(0x1d, 0x9b, 0x30, 0xff));
    env.set(COLOR_BUTTON_BORDER_ACTIVE, Color::rgba8(0x1d, 0x9b, 0x30, 0x7f));

    env.set(COLOR_TITLE_BAR_BUTTON, Color::rgba8(0x00, 0x00, 0x00, 0x00));
    env.set(COLOR_TITLE_BAR_BUTTON_HOT, Color::rgba8(0x00, 0x00, 0x00, 0x3f));
    env.set(COLOR_TITLE_BAR_BUTTON_ACTIVE, Color::rgba8(0x00, 0x00, 0x00, 0x1f));

    env.set(COLOR_CLEAR_BUTTON, Color::rgba8(0x00, 0x00, 0x00, 0x00));
    env.set(COLOR_CLEAR_BUTTON_HOT, Color::rgba8(0x00, 0x00, 0x00, 0x10));
    env.set(COLOR_CLEAR_BUTTON_ACTIVE, Color::rgba8(0x00, 0x00, 0x00, 0x1f));
    env.set(COLOR_CLEAR_BUTTON_BORDER, Color::rgba8(0x00, 0x00, 0x00, 0x00));
    env.set(COLOR_CLEAR_BUTTON_BORDER_HOT, Color::rgba8(0x00, 0x00, 0x00, 0x1f));
    env.set(COLOR_CLEAR_BUTTON_BORDER_ACTIVE, Color::rgba8(0x00, 0x00, 0x00, 0x1f));

    env.set(COLOR_BORDER_LIGHT, Color::rgba8(0x00, 0x00,0x00, 0x1f));
    env.set(COLOR_BORDER_DARK, Color::rgba8(0x00, 0x00,0x00, 0x3f));

    env.set(COLOR_CURSOR, Color::rgba8(0x2a, 0x2a, 0x2a, 0xff));

    // 数值
    env.set(BORDER_WIDTH_WINDOW, 2.0);
    env.set(BORDER_WIDTH_BUTTON, 1.2);
    env.set(BORDER_WIDTH_TEXTBOX, 1.2);

    env.set(RADIUS_BUTTON_BORDER, 8.0);
    env.set(RADIUS_TEXTBOX_BORDER, 8.0);

    env.set(INSETS_TEXTBOX, Insets::new(8.0, 5.0, 8.0, 5.0));
}