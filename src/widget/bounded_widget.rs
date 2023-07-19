use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetPod};

pub struct BoundedWidget<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    inner_size: Size
}

impl<T: Data> BoundedWidget<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> BoundedWidget<T> {
        BoundedWidget {
            inner: WidgetPod::new(Box::new(inner)),
            inner_size: Size::ZERO
        }
    }
}

impl<T: Data> Widget<T> for BoundedWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let wnd_size = ctx.window().get_size();
        self.inner_size = self.inner.layout(ctx, &BoxConstraints::new(
            Size::new(bc.min().width.min(wnd_size.width), bc.min().height.min(wnd_size.height)),
            Size::new(bc.max().width.min(wnd_size.width), bc.max().height.min(wnd_size.height))
        ), data, env);
        self.inner_size = Size::new(self.inner_size.width.min(wnd_size.width), self.inner_size.height.min(wnd_size.height));
        self.inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}