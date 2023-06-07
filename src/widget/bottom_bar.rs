use druid::{Widget, WidgetExt};
use druid::widget::Flex;
use crate::AppState;
use crate::theme::theme;

pub fn build() -> impl Widget<AppState> {
    let bar = Flex::column()
        .fix_height(48.0)
        .background(theme::COLOR_BACKGROUND_DARK)
        .border(theme::COLOR_BORDER_LIGHT, 1.0)
        .expand_width();
    bar
}