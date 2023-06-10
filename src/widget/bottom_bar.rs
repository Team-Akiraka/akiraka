use druid::{Insets, Widget, WidgetExt};
use druid::widget::{Flex};
use crate::{AppState, Asset};
use crate::theme::theme;
use crate::widget::button::Button;
use crate::widget::clear_button::ClearButton;
use crate::widget::icon_clear_button::IconClearButton;
use crate::widget::launch_button::LaunchButton;
use crate::widget::primary_button::PrimaryButton;

pub fn build() -> impl Widget<AppState> {
    let launch_button = LaunchButton::new(
        std::str::from_utf8(&Asset::get("icon/play.svg").unwrap().data).unwrap().parse::<String>().unwrap(),
        "Launch"
    )
        .fix_width(160.0)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);
    // let launch_button = IconClearButton::new(std::str::from_utf8(&Asset::get("icon/play.svg").unwrap().data).unwrap().parse::<String>().unwrap());

    let bar = Flex::column()
        .with_child(launch_button)
        .align_right()
        .padding(Insets::new(12.0, 6.0, 12.0, 6.0))
        .fix_height(56.0)
        .background(theme::COLOR_BACKGROUND_DARK)
        .border(theme::COLOR_BORDER_LIGHT, 1.0)
        .expand_width();
    bar
}