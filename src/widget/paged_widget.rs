use std::collections::HashMap;
use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Vec2, Widget, WidgetPod};
use druid::piet::ImageFormat;
use druid::platform_menus::common::undo;
use crate::ui::{download_page, hello_page, instances_page, settings_page};
use crate::{animations, AppState};

struct Child<AppState> {
    inner: WidgetPod<AppState, Box<dyn Widget<AppState>>>
}


impl Child<AppState> {
    fn new(inner: WidgetPod<AppState, Box<dyn Widget<AppState>>>) -> Child<AppState> {
        Child {
            inner
        }
    }

    fn widget_mut(&mut self) -> Option<&mut WidgetPod<AppState, Box<dyn Widget<AppState>>>> {
        Some(&mut self.inner)
    }

    fn widget(&self) -> Option<&WidgetPod<AppState, Box<dyn Widget<AppState>>>> {
        Some(&self.inner)
    }
}

const ANIMATION_TIME: f64 = 0.5;

pub struct PagedWidget<AppState> {
    children: HashMap<String, Child<AppState>>,
    current_id: String,
    inner_size: Size,
    t: f64
}

impl PagedWidget<AppState> {
    pub fn new() -> PagedWidget<AppState> {
        let mut children = HashMap::new();
        // Add Children
        children.insert(hello_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(hello_page::build()))));
        children.insert(instances_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(instances_page::build()))));
        children.insert(download_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(download_page::build()))));
        children.insert(settings_page::ID.parse().unwrap(), Child::new(WidgetPod::new(Box::new(settings_page::build()))));

        PagedWidget {
            children,
            current_id: hello_page::ID.parse().unwrap(),
            inner_size: Size::ZERO,
            t: 1.0
        }
    }

    fn detect_scene_change(&mut self) -> bool {
        if self.current_id.as_str() != unsafe { crate::PAGE_ID } {
            self.current_id = String::from(unsafe { crate::PAGE_ID });
            true
        } else {
            false
        }
    }
}

impl Widget<AppState> for PagedWidget<AppState> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
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

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if self.current_id.as_str() != unsafe { crate::PAGE_ID } {
            self.current_id = String::from(unsafe { crate::PAGE_ID });
        }

        for x in self.children.values_mut().filter_map(|x| x.widget_mut()) {
            x.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppState, data: &AppState, env: &Env) {
        let x = self.children.get_mut(&self.current_id);
        if x.is_some() {
            x.unwrap().inner.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        let x = self.children.get_mut(&self.current_id);
        if x.is_some() {
            bc.constrain(x.unwrap().inner.layout(ctx, bc, data, env));
        }
        self.inner_size = bc.max();
        self.inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {

        let x = self.children.get_mut(&self.current_id);
        if x.is_some() {
            let s = if self.t / ANIMATION_TIME < 1.0 {
                let s = self.t / ANIMATION_TIME;

                animations::expo::ease_out(s)
            } else {
                1.0
            };
            let s0 = s.clone();
            let s = s / 4.0 + 0.75;
            let w = ctx.window().get_size().width / 2.0 - self.inner_size.width * s / 2.0;
            let h = ctx.window().get_size().height / 2.0 - self.inner_size.height * s / 2.0;
            ctx.transform(Affine::scale(s)
                .then_translate(Vec2::new(w, h)));

            x.unwrap().inner.paint(ctx, data, env);
            // ctx.render_ctx.fill(self.inner_size.to_rect(), &Color::rgba(1.0,1.0, 1.0, 1.0 - s0.powf(8.0)));
        }
    }
}