use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget};
use druid::piet::{Brush, IntoBrush};
use druid::Value::Color;

pub struct TitleBar {
}

impl TitleBar {
    pub fn new() -> Self {
        Self {
        }
    }
}

// https://www.pauljmiller.com/posts/druid-widget-tutorial.html
impl<T> Widget<T> for TitleBar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        return Size::new(0.0, 0.0);
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let rgb = druid::Color::rgb(255.0, 255.0, 0.0);
        let rect = Rect::from_origin_size(Point::ORIGIN, (16.0, 16.0));
        // ctx.fill(rect);
    }
}