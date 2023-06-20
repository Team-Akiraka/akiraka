use std::any::Any;
use std::collections::HashMap;
use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetPod};
use crate::ui::{hello_page, settings_page};

struct Child<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T> Child<T> {
    fn new(inner: WidgetPod<T, Box<dyn Widget<T>>>) -> Child<T> {
        Child {
            inner
        }
    }

    fn widget_mut(&mut self) -> Option<&mut WidgetPod<T, Box<dyn Widget<T>>>> {
        Some(&mut self.inner)
    }
    fn widget(&self) -> Option<&WidgetPod<T, Box<dyn Widget<T>>>> {
        Some(&self.inner)
    }
}

pub struct PagedWidget<T> {
    children: HashMap<String, Child<T>>,
    current_id: String,
}

impl<T: Data> PagedWidget<T> {
    pub fn new() -> PagedWidget<T> {
        let mut children = HashMap::new();
        // Add Children
        children.insert(hello_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(hello_page::build()))));
        children.insert(settings_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(settings_page::build()))));
        PagedWidget {
            children,
            current_id: hello_page::ID.parse().unwrap()
        }
    }
}

impl<T: Data> Widget<T> for PagedWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        let x = self.children.get_mut(&String::from(unsafe { crate::PAGE_ID }));
        if x.is_some() {
            x.unwrap().inner.event(ctx, event, data, env);
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
            bc.constrain(x.layout(ctx, bc, data, env));
        }
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let x = self.children.get_mut(&String::from(unsafe { crate::PAGE_ID }));
        if x.is_some() {
            x.unwrap().inner.paint(ctx, data, env);
        }
    }
}