use druid::{Data, Insets, WidgetExt};
use druid::widget::{Widget, Flex};
use crate::{Asset};
use crate::theme::theme;
use crate::ui::{download_page, settings_page};
use crate::widget::icon_clear_button::IconClearButton;
use crate::widget::launch_button::LaunchButton;
use crate::widget::profile_button::ProfileButton;

pub const BOTTOM_BAR_HEIGHT: f64 = 56.0;

pub fn build<T: Data>() -> impl Widget<T> {
    let profile_button = ProfileButton::new()
        .fix_width(160.0)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let settings_button = IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/settings.svg").unwrap().data).unwrap().parse::<String>().unwrap()
    )
        .fix_width(crate::widget::window::TITLE_BAR_HEIGHT)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let settings_button = settings_button.on_click(|ctx, _data, _env| {
        unsafe {
            crate::PAGE_ID = settings_page::ID;
        }
        ctx.request_layout();
        ctx.request_update();
        ctx.request_paint();
    });

    let download_button = IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/download.svg").unwrap().data).unwrap().parse::<String>().unwrap()
    )
        .fix_width(crate::widget::window::TITLE_BAR_HEIGHT)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let download_button = download_button.on_click(|ctx, _data, _env| {
        unsafe {
            crate::PAGE_ID = download_page::ID;
        }
        ctx.request_paint();
    });

    let misc_button = IconClearButton::new(
        std::str::from_utf8(&Asset::get("icon/list.svg").unwrap().data).unwrap().parse::<String>().unwrap()
    )
        .fix_width(crate::widget::window::TITLE_BAR_HEIGHT)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let launch_button = LaunchButton::new(
        std::str::from_utf8(&Asset::get("icon/play_slim.svg").unwrap().data).unwrap().parse::<String>().unwrap(),
        "Launch"
    )
        .fix_width(160.0)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let bar = Flex::row()
        .with_child(profile_button)
        .with_flex_spacer(1.0)
        .with_child(settings_button)
        .with_spacer(8.0)
        .with_child(download_button)
        .with_spacer(8.0)
        .with_child(misc_button)
        .with_flex_spacer(1.0)
        .with_child(launch_button)
        .center()
        .padding(Insets::new(12.0, 6.0, 12.0, 6.0))
        .fix_height(BOTTOM_BAR_HEIGHT)
        .background(theme::COLOR_BACKGROUND_DARK)
        .border(theme::COLOR_BORDER_LIGHT, 1.0)
        .expand_width();
    bar
}