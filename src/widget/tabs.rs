use std::collections::HashMap;
use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Size, theme, UpdateCtx, Widget, WidgetExt, WidgetPod};
use druid::widget::Label;

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

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}

struct SelectionButton<T> {
    label: WidgetPod<T, Box<dyn Widget<T>>>,
    selected: bool
}

impl<T: Data> SelectionButton<T> {
    pub fn new(name: String) -> SelectionButton<T> {
        SelectionButton {
            label: WidgetPod::new(Box::new( Label::new(name).with_text_size(16.0).center())),
            selected: false
        }
    }

    pub fn selected(&mut self, value: bool) {
        self.selected = value;
    }
}

impl<T: Data> Widget<T> for SelectionButton<T> {
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
        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.label.layout(ctx, &bc.loosen(), data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let is_hot = ctx.is_hot();
        let size = ctx.size();
        let stroke_width = env.get(theme::BUTTON_BORDER_WIDTH);

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let bg_gradient = if is_active {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_ACTIVE)
        } else if is_hot {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_HOT)
        } else {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON)
        };

        let border_color = if is_active {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_BORDER_ACTIVE)
        } else if is_hot {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_BORDER_HOT)
        } else {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_BORDER)
        };

        ctx.fill(rounded_rect, &bg_gradient);

        ctx.stroke(rounded_rect, &border_color, stroke_width);

        ctx.with_save(|ctx| {
            self.label.paint(ctx, data, env);
        });
    }
}

pub struct Tabs<T> {
    children: HashMap<String, Child<T>>,
    selected: String,
    inner_size: Size,
    tabs: Vec<(Child<T>, String)>
}

impl<T: Data> Tabs<T> {
    pub fn new() -> Tabs<T> {
        Tabs {
            children: HashMap::new(),
            selected: String::new(),
            inner_size: Size::ZERO,
            tabs: Vec::new()
        }
    }

    pub fn with_child(mut self, name: String, body: impl Widget<T> + 'static) -> Tabs<T> {
        self.children.insert(name.clone(), Child::new(body));

        let button = SelectionButton::new(name.clone())
            .fix_width(96.0)
            .fix_height(32.0);
        self.tabs.push((Child::new(button), name.clone()));

        self
    }

    pub fn with_selected(mut self, name: String) -> Tabs<T> {
        self.selected = name;
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

        let mut i = 0;
        for x in self.tabs.iter_mut() {
            x.0.widget_mut().unwrap().event(ctx, event, data, env);
            match event {
                Event::MouseUp(_) => {
                    if x.0.inner.is_hot() {
                        // println!("{i}");
                        // println!("{}", self.tabs.get(i).unwrap().0);
                        let x = &x.1;
                        self.selected = String::from(x.clone());
                        ctx.request_anim_frame();
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
        for x in self.tabs.iter_mut() {
            x.0.widget_mut().unwrap().lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        let x = self.children.get_mut(self.selected.as_str());
        if x.is_some() {
            let x = x.unwrap().widget_mut().unwrap();
            x.update(ctx, data, env);
        }
        for x in self.tabs.iter_mut() {
            x.0.widget_mut().unwrap().update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            let child_bc = bc.shrink_max_height_to(bc.max().height - 32.0);
            x.set_origin(ctx, Point::new(0.0, 32.0));
            x.layout(ctx, &child_bc, data, env);
        }
        // let x = self.children.get_mut(self.selected.as_str());
        // if x.is_some() {
        // }
        let mut i = 0.0;
        for x in self.tabs.iter_mut() {
            let x = x.0.widget_mut().unwrap();
            x.set_origin(ctx, Point::new(104.0 * i, 0.0));
            x.layout(ctx, bc, data, env).width;
            i += 1.0;
        }

        let size = bc.shrink_max_height_to(bc.max().height - 32.0).max();
        let size = Size::new(if bc.max().width > ctx.window().get_size().width {
            ctx.window().get_size().width
        } else {
            size.width
        }, ctx.window().get_size().height);
        self.inner_size = size;
        self.inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let mut i = 0;
        for x in self.children.iter_mut() {
            // println!("Indexing {}: {} : {:?}", i, x.0, x.1.inner.id());
            if x.0.as_str() == self.selected.as_str() {
                break;
            }
            i += 1;
        }

        let x = self.children.get_mut(self.selected.as_str());
        if x.is_some() {
            let x = x.unwrap().widget_mut().unwrap();
            x.paint(ctx, data, env);
        }

        let mut j = 0;
        let len = self.tabs.len();
        for x in self.tabs.iter_mut() {
            // let var0 = Vec::from_iter(self.children.iter_mut());
            // let var0 = var0.get(i).unwrap();
            // println!("{} : {:?}", var0.0, var0.1.inner.id());
            // println!("{:?}", x.0.inner.id(), Vec::from(self.children.iter_mut()).get(i).unwrap().0);

            // println!("{} == {}", i, j);
            if i == j {
                // println!("id: {:?}", x.0.inner.id());
                // println!("{j}");
                // I didn't figure out how it works, so plz don't change anything
                let offset = (j) as f64 * 104.0;
                let rect = Rect::new(12.0 + offset, 30.0, 84.0 + offset, 32.0)
                    .to_rounded_rect(2.0);
                ctx.fill(rect, &env.get(crate::theme::theme::COLOR_PRIMARY_LIGHT));
            }
            x.0.widget_mut().unwrap().paint(ctx, data, env);
            j += 1;
        }
    }
}