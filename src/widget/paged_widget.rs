use std::collections::HashMap;
use druid::{Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Vec2, Widget, WidgetPod};
use crate::ui::{download_page, hello_page, settings_page};
use crate::{animations};

struct Child<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>
}

#[allow(unused)]
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

const ANIMATION_TIME: f64 = 0.6;

#[allow(unused)]
pub struct PagedWidget<T> {
    children: HashMap<String, Child<T>>,
    current_id: String,
    last_id: String,
    inner_size: Size,
    t: f64
}

impl<T: Data> PagedWidget<T> {
    pub fn new() -> PagedWidget<T> {
        let mut children = HashMap::new();
        // Add Children
        children.insert(hello_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(hello_page::build()))));
        children.insert(settings_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(settings_page::build()))));
        children.insert(download_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(download_page::build()))));

        PagedWidget {
            children,
            current_id: hello_page::ID.parse().unwrap(),
            last_id: String::new(),
            inner_size: Size::ZERO,
            t: 1.0
        }
    }

    fn detect_scene_change(&mut self) -> bool {
        if self.current_id.as_str() != unsafe { crate::PAGE_ID } {
            self.last_id = self.current_id.clone();
            self.current_id = String::from(unsafe { crate::PAGE_ID });
            true
        } else {
            false
        }
    }
}

impl<T: Data> Widget<T> for PagedWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if self.detect_scene_change() {
            self.t = 0.0;
            ctx.request_anim_frame();
        }

        let x = self.children.get_mut(&self.current_id);
        if x.is_some() {
            x.unwrap().inner.event(ctx, event, data, env);
        }

        match event {
            Event::WindowConnected => {
                self.current_id = hello_page::ID.parse().unwrap();
                self.t = 0.0;
                ctx.request_anim_frame();
            }
            Event::AnimFrame(interval) => {
                self.t += (*interval as f64) * 1e-9;
                if self.t <= ANIMATION_TIME {
                    ctx.request_anim_frame();
                    ctx.request_paint();
                } else {
                    ctx.request_paint();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if self.current_id.as_str() != unsafe { crate::PAGE_ID } {
            self.current_id = String::from(unsafe { crate::PAGE_ID });
        }

        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            bc.constrain(x.layout(ctx, bc, data, env));
        }
        self.inner_size = bc.max();
        self.inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        // let c1 = 1.70158;
        // let c3 = c1 + 1.0;

        let x = self.children.get_mut(&self.current_id);
        if x.is_some() {
            let s = if self.t / ANIMATION_TIME < 1.0 {
                let s = self.t / ANIMATION_TIME;
                // 1.0 + c3 * (s - 1.0).powf(3.0) + c1 * (s - 1.0).powf(2.0)

                // if s < 0.5 {
                //     16.0 * s.powf(5.0)
                // } else {
                //     1.0 - (-2.0 * s + 2.0).powf(5.0) / 2.0
                // }
                // 1.0 - (-2.0 * s + 2.0).powf(5.0) / 2.0
                // animations::ease_in_sine(s)
                animations::elastic::ease_out(s)
            } else {
                1.0
            };
            let s = s / 2.0 + 0.5;
            // TODO: Easing
            let w = ctx.window().get_size().width / 2.0 - self.inner_size.width * s / 2.0;
            let h = ctx.window().get_size().height / 2.0 - self.inner_size.height * s / 2.0;
            ctx.transform(Affine::scale(s)
                .then_translate(Vec2::new(w, h)));
            x.unwrap().inner.paint(ctx, data, env);
        }
    }
}