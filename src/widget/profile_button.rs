
use druid::widget::{Flex, Image, Label};
use druid::{Affine, BoxConstraints, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, Point, RenderContext, Size, TextAlignment, theme, UpdateCtx, Vec2, Widget, WidgetExt, WidgetPod};
use image::imageops::FilterType;
use crate::util;

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct ProfileButton<T> {
    icon: Image,
    layout: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T: Data> ProfileButton<T> {
    pub fn new() -> ProfileButton<T> {
        let user_name = Label::new("Unknown User").with_text_size(15.0).with_text_alignment(TextAlignment::Start).expand_width().fix_height(18.0);
        let user_type = Label::new("Unknown Type").with_text_size(12.0).with_text_alignment(TextAlignment::Start).expand_width().fix_height(13.0);

        ProfileButton {
            icon: Image::new(util::load_image("icon/steve_head.png", Size::new(64.0, 64.0), FilterType::Nearest)),
            layout: WidgetPod::new(Box::new(Flex::column()
                .with_child(user_name)
                .with_child(user_type)
                .align_left()))
        }
    }
}

impl<T: Data> Widget<T> for ProfileButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(event) => {
                if !ctx.is_disabled() && event.button == MouseButton::Left {
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
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }

        self.layout.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.layout.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());

        let icon_bc = bc.loosen();
        let icon_size = self.icon.layout(ctx, &icon_bc, data, env);

        let layout_bc = bc.shrink(padding).shrink_max_width_to(bc.min().width - icon_size.width).loosen();
        self.layout.set_origin(ctx, Point::new(icon_size.width + 20.0, 0.0));
        self.layout.layout(ctx, &layout_bc, data, env);
        bc.min()
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
            ctx.transform(Affine::scale(0.6).then_translate(Vec2::new(16.0 - 40.0, 9.0)));
            self.icon.paint(ctx, data, env)
        });

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(Vec2::new( -40.0, 0.0)));
            self.layout.paint(ctx, data, env);
        });
    }
}