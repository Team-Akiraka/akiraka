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

pub const COLOR_CLEAR_BUTTON: Key<Color> = Key::new("team.akiraka.theme.color.clear.button");
pub const COLOR_CLEAR_BUTTON_HOT: Key<Color> = Key::new("team.akiraka.theme.color.clear.button.hot");
pub const COLOR_CLEAR_BUTTON_ACTIVE: Key<Color> = Key::new("team.akiraka.theme.color.clear.button.active");

pub const COLOR_BORDER_LIGHT: Key<Color> = theme::BORDER_LIGHT;
pub const COLOR_BORDER_DARK: Key<Color> = theme::BORDER_DARK;

pub const COLOR_CURSOR: Key<Color> = theme::CURSOR_COLOR;

// 数值
pub const BORDER_WIDTH_WINDOW: Key<f64> = Key::new("team.akiraka.theme.border.width.window");
pub const BORDER_WIDTH_BUTTON: Key<f64> = theme::BUTTON_BORDER_WIDTH;
pub const BORDER_WIDTH_TEXTBOX: Key<f64> = theme::TEXTBOX_BORDER_WIDTH;

pub const RADIUS_TEXTBOX_BORDER: Key<RoundedRectRadii> = theme::TEXTBOX_BORDER_RADIUS;

pub const INSETS_TEXTBOX: Key<Insets> = theme::TEXTBOX_INSETS;

pub fn init(env: &mut Env) {
    env.set(COLOR_WINDOW_BACKGROUND, Color::rgba8(0xf9, 0xf9, 0xf9, 0xff));
    env.set(COLOR_WINDOW_BORDER, Color::rgba8(0xdf, 0xdf,0xdf, 0xff));

    env.set(COLOR_PRIMARY_TITLE_BAR, Color::rgba8(47, 255, 106, 0));
    env.set(COLOR_PRIMARY_LIGHT, Color::rgba8(29, 155, 48, 255));
    env.set(COLOR_PRIMARY_DARK, Color::rgba8(24, 129, 40, 255));

    env.set(COLOR_BACKGROUND_LIGHT, Color::rgba8(0xf9, 0xf9, 0xf9, 0xff));
    env.set(COLOR_BACKGROUND_DARK, Color::rgba8(0xef, 0xef, 0xef, 0xff));

    env.set(COLOR_FOREGROUND_LIGHT, Color::rgba8(255, 255, 255, 255));
    env.set(COLOR_FOREGROUND_DARK, Color::rgba8(255, 255, 255, 255));

    env.set(COLOR_TEXT, Color::rgba8(0x2a, 0x2a, 0x2a, 0xff));
    env.set(COLOR_DISABLED_TEXT, Color::rgba8(0xaf, 0xaf, 0xaf, 0xff));
    env.set(COLOR_PLACEHOLDER_TEXT, Color::rgba8(0x8f, 0x8f, 0x8f, 0xff));

    env.set(COLOR_CLEAR_BUTTON, Color::rgba8(0, 0, 0, 0));
    env.set(COLOR_CLEAR_BUTTON_HOT, Color::rgba8(0, 0, 0, 63));
    env.set(COLOR_CLEAR_BUTTON_ACTIVE, Color::rgba8(0, 0, 0, 31));

    env.set(COLOR_BORDER_LIGHT, Color::rgba8(0xcf, 0xcf,0xcf, 0xff));
    env.set(COLOR_BORDER_DARK, Color::rgba8(0x9f, 0x9f, 0x9f, 0xff));

    env.set(COLOR_CURSOR, Color::rgba8(14, 14, 14, 255));

    // 数值
    env.set(BORDER_WIDTH_WINDOW, 2.0);
    env.set(BORDER_WIDTH_BUTTON, 1.2);
    env.set(BORDER_WIDTH_TEXTBOX, 1.2);

    env.set(RADIUS_TEXTBOX_BORDER, 8.0);

    env.set(INSETS_TEXTBOX, Insets::new(8.0, 4.0, 8.0, 8.0));
}