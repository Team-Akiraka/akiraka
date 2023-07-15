use std::collections::HashMap;
use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Size, UpdateCtx, Widget, WidgetPod};

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

    pub fn with_child(mut self, name: impl Into<String> + 'static, body: impl Widget<T> + 'static) -> Tabs<T> {
        self.children.insert(name.into(), Child::new(body));
        self
    }

    pub fn with_selected(mut self, name: impl Into<String> + 'static) -> Tabs<T> {
        self.selected = name.into();
        self
    }
}

impl<T: Data> Widget<T> for Tabs<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        let x = self.children.get_mut(self.selected.as_str());
        if x.is_some() {
            let x = x.unwrap().widget_mut().unwrap();
            x.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        let x = self.children.get_mut(self.selected.as_str());
        if x.is_some() {
            let x = x.unwrap().widget_mut().unwrap();
            x.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let x = self.children.get_mut(self.selected.as_str());
        if x.is_some() {
            let x = x.unwrap().widget_mut().unwrap();
            let child_bc = bc.shrink_max_height_to(bc.max().height - 24.0);
            x.set_origin(ctx, Point::new(0.0, 24.0));
            x.layout(ctx, &child_bc, data, env);
        }
        self.inner_size = bc.shrink_max_height_to(bc.max().height - 24.0).max();
        self.inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let x = self.children.get_mut(self.selected.as_str());
        if x.is_some() {
            let x = x.unwrap().widget_mut().unwrap();
            x.paint(ctx, data, env);
        }
    }
}