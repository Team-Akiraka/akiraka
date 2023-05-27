use druid::{Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, theme, UpdateCtx, Widget};
use druid::widget::{Image};

pub struct IconButton {
    image: Image,
    size: Size
}

#[allow(unused)]
impl IconButton {
    pub fn new(image: Image, size: Size) -> Self {
        Self {
            image,
            size
        }
    }
}

impl<T: Data> Widget<T> for IconButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                }
                ctx.set_active(false);
            }
            _ => (),
        }
        self.image.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.image.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.image.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.image.layout(ctx, bc, data, env);
        self.size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let rect = Rect::from_origin_size(Point::ORIGIN, self.size);
        ctx.fill(rect, &env.get(if is_active {
            theme::BACKGROUND_DARK
        } else {
            theme::PLACEHOLDER_COLOR
        }));
        ctx.with_save(move |ctx| {

            ctx.transform(Affine::scale(0.75));
            self.image.paint(ctx, data, env);
        })
    }
}