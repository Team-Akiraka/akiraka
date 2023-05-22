use druid::{Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, theme, UpdateCtx, Widget};
use druid::widget::Image;

pub struct IconButton {
    image: Image,
    size: Size,
    pressing: bool
}

impl IconButton {
    pub fn new(image: Image, size: Size) -> Self {
        Self {
            image,
            size,
            pressing: false
        }
    }
}

impl<T: Data> Widget<T> for IconButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(m_event) => {
                self.pressing = true;
                // ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(m_event) => {
                self.pressing = false;
                // ctx.set_active(false);
                ctx.request_paint();
            }
            Event::MouseMove(m_event) => {
                println!("{} : {}", ctx.is_active(), self.pressing);
            }
            _ => {}
        }
        self.image.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
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
        let active = ctx.is_active();
        ctx.with_save(move |ctx| {
            let rect = Rect::from_origin_size(Point::ORIGIN, self.size);
            ctx.fill(rect, &env.get(if self.pressing {
                theme::BACKGROUND_DARK
            } else {
                theme::PLACEHOLDER_COLOR
            }));

            ctx.transform(Affine::scale(0.75));
            self.image.paint(ctx, data, env);
        })
    }
}