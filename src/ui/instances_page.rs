#[allow(dead_code)]

use std::borrow::ToOwned;
use std::collections::HashMap;
use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UnitPoint, UpdateCtx, Vec2, Widget, WidgetExt, WidgetPod};
use druid::widget::{Flex, Label};
use crate::{animations, Asset};
use crate::widget::side_bar_selection::SideBarSelection;

pub const ID: &str = "INSTANCES_PAGE";

pub fn build<T: Data>() -> impl Widget<T> {
    let title = Label::new("Instances")
        .with_text_size(24.0)
        .fix_width(32.0)
        .expand_width()
        .padding(Insets::uniform_xy(12.0, 4.0));

    let body = Flex::column()
        .with_child(title)
        .fix_width(160.0)
        .padding(Insets::uniform_xy(8.0, 0.0));

    let body = druid::widget::Scroll::new(body);

    body
        .align_vertical(UnitPoint::TOP)
        .align_left()
}