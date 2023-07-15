use std::collections::HashMap;
use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetPod};

pub struct Child<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T> Child<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> Child<T> {
        Child {
            inner: WidgetPod::new(Box::new(inner))
        }
    }

    fn widget_mut(&mut self) -> Option<&mut WidgetPod<T, Box<dyn Widget<T>>>> {
        Some(&mut self.inner)
    }

    fn widget(&self) -> Option<&WidgetPod<T, Box<dyn Widget<T>>>> {
        Some(&self.inner)
    }
}

impl<T: Data> Widget<T> for Child<T> {
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
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}

pub struct Tabs<T> {
    children: HashMap<String, Child<T>>,
    selected: String,
    inner_size: Size
}

impl<T> Tabs<T> {
    pub fn new() -> Tabs<T> {
        Tabs {
            children: HashMap::new(),
            selected: String::new(),
            inner_size: Size::ZERO
        }
    }
}

impl<T: Data> Widget<T> for Tabs<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.layout(ctx, bc, data, env);
        }
        self.inner_size = bc.max();
        self.inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.paint(ctx, data, env);
        }
    }
}