use druid::{Color, Env, Key, RoundedRectRadii, theme};

// 颜色
pub const COLOR_WINDOW_BACKGROUND: Key<Color> = theme::WINDOW_BACKGROUND_COLOR;

pub const COLOR_PRIMARY_TITLE_BAR: Key<Color> = Key::new("team.akiraka.theme.color.primary.titlebar");
pub const COLOR_PRIMARY_LIGHT: Key<Color> = theme::PRIMARY_LIGHT;
pub const COLOR_PRIMARY_DARK: Key<Color> = theme::PRIMARY_DARK;

pub const COLOR_BACKGROUND_LIGHT: Key<Color> = theme::BACKGROUND_LIGHT;
pub const COLOR_BACKGROUND_DARK: Key<Color> = theme::BACKGROUND_DARK;

pub const COLOR_FOREGROUND_LIGHT: Key<Color> = theme::FOREGROUND_LIGHT;
pub const COLOR_FOREGROUND_DARK: Key<Color> = theme::FOREGROUND_DARK;

pub const COLOR_TEXT: Key<Color> = theme::TEXT_COLOR;
pub const COLOR_DISABLED_TEXT: Key<Color> = theme::DISABLED_TEXT_COLOR;

pub const COLOR_CLEAR_BUTTON: Key<Color> = Key::new("team.akiraka.theme.color.clear.button");
pub const COLOR_CLEAR_BUTTON_HOT: Key<Color> = Key::new("team.akiraka.theme.color.clear.button.hot");
pub const COLOR_CLEAR_BUTTON_ACTIVE: Key<Color> = Key::new("team.akiraka.theme.color.clear.button.active");

pub const COLOR_BORDER_LIGHT: Key<Color> = theme::BORDER_LIGHT;
pub const COLOR_BORDER_DARK: Key<Color> = theme::BORDER_DARK;

pub const COLOR_CURSOR: Key<Color> = theme::CURSOR_COLOR;

// 数值
pub const RADIUS_TEXTBOX_BORDER: Key<RoundedRectRadii> = theme::TEXTBOX_BORDER_RADIUS;

pub fn init(env: &mut Env) {
    env.set(COLOR_WINDOW_BACKGROUND, Color::rgba8(243, 243, 243, 255));

    env.set(COLOR_PRIMARY_TITLE_BAR, Color::rgba8(47, 255, 106, 0));
    env.set(COLOR_PRIMARY_LIGHT, Color::rgba8(29, 155, 48, 255));
    env.set(COLOR_PRIMARY_DARK, Color::rgba8(24, 129, 40, 255));

    env.set(COLOR_BACKGROUND_LIGHT, Color::rgba8(243, 243, 243, 255));
    env.set(COLOR_BACKGROUND_DARK, Color::rgba8(233, 233, 233, 255));

    env.set(COLOR_FOREGROUND_LIGHT, Color::rgba8(255, 255, 255, 255));
    env.set(COLOR_FOREGROUND_DARK, Color::rgba8(255, 255, 255, 255));

    env.set(COLOR_TEXT, Color::rgba8(14, 14, 14, 255));
    env.set(COLOR_DISABLED_TEXT, Color::rgba8(72, 72, 72, 255));

    env.set(COLOR_CLEAR_BUTTON, Color::rgba8(0, 0, 0, 0));
    env.set(COLOR_CLEAR_BUTTON_HOT, Color::rgba8(0, 0, 0, 63));
    env.set(COLOR_CLEAR_BUTTON_ACTIVE, Color::rgba8(0, 0, 0, 31));

    env.set(COLOR_BORDER_LIGHT, Color::rgba8(163, 163,163, 255));
    env.set(COLOR_BORDER_DARK, Color::rgba8(58, 58, 58, 255));

    env.set(COLOR_CURSOR, Color::rgba8(14, 14, 14, 255));

    env.set(RADIUS_TEXTBOX_BORDER, 8.0);
}