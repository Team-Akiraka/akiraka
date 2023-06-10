use druid::{Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, Point, RenderContext, Size, TextAlignment, theme, UpdateCtx, Widget, WidgetExt, WidgetPod};
use druid::kurbo::Arc;
use druid::widget::{Click, ControllerHost, Flex, Label, LabelText, Svg, SvgData};

fn color_as_hex_string(color: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", color.as_rgba8().0, color.as_rgba8().1, color.as_rgba8().2).parse().unwrap()
}

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct LaunchButton<T> {
    icon: Svg,
    icon_data: String,
    layout: WidgetPod<T, Box<dyn Widget<T>>>
}

impl<T: Data> LaunchButton<T> {
    pub fn new(icon: String, text: impl Into<LabelText<T>>,) -> LaunchButton<T> {
        let icon_data = icon.replace("{color}", "#000000").parse::<SvgData>().unwrap();
        let label = Label::new(text).with_text_size(15.0).with_text_alignment(TextAlignment::Start).expand_width().fix_height(18.0);
        let subtitle = Label::new("Unknown Instance").with_text_size(12.0).with_text_alignment(TextAlignment::Start).expand_width().fix_height(13.0);;

        LaunchButton {
            icon: Svg::new(icon_data.clone()),
            icon_data: icon,
            layout: WidgetPod::new(Box::new(Flex::column()
                .with_child(label)
                .with_child(subtitle)
                .align_left()))
        }
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }

}

impl<T: Data> Widget<T> for LaunchButton<T> {
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
        // self.label.lifecycle(ctx, event, data, env);
        self.icon.lifecycle(ctx, event, data, env);
        // self.subtitle.lifecycle(ctx, event, data, env);
        self.layout.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        // self.label.update(ctx, old_data, data, env);
        self.icon.update(ctx, old_data, data, env);
        // self.subtitle.update(ctx, old_data, data, env);
        self.layout.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());

        let icon_bc = bc.loosen();
        let icon_size = self.icon.layout(ctx, &icon_bc, data, env);

        let layout_bc = bc.shrink(padding).shrink_max_width_to(bc.min().width - icon_size.width).loosen();
        self.layout.set_origin(ctx, Point::new(icon_size.width - 4.0, -1.0));
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
            let svg_data = self.icon_data.replace("{color}", color_as_hex_string(env.get(theme::TEXT_COLOR)).as_str()).parse::<SvgData>().unwrap();
            self.icon = Svg::new(svg_data);
            ctx.transform(Affine::scale(1.0));
            self.icon.paint(ctx, data, env)
        });

        ctx.with_save(|ctx| {
            self.layout.paint(ctx, data, env);
        });
    }
}