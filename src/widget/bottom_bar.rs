use druid::{Insets, Widget, WidgetExt};
use druid::widget::{Flex};
use crate::{AppState};
use crate::theme::theme;
use crate::widget::button::Button;
use crate::widget::clear_button::ClearButton;
use crate::widget::primary_button::PrimaryButton;

pub fn build() -> impl Widget<AppState> {
    let launch_button = ClearButton::new("Launch")
        .fix_width(160.0)
        .fix_height(crate::widget::window::TITLE_BAR_HEIGHT);

    let bar = Flex::column()
        .with_child(launch_button)
        .align_right()
        .padding(Insets::new(8.0, 6.0, 8.0, 6.0))
        .fix_height(56.0)
        .background(theme::COLOR_BACKGROUND_DARK)
        .border(theme::COLOR_BORDER_LIGHT, 1.0)
        .expand_width();
    bar
}