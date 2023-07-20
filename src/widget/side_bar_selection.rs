use crate::util::color_as_hex_string;
use druid::widget::{Label, LabelText, Svg, SvgData};
use druid::{
    theme, Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    MouseButton, PaintCtx, Point, RenderContext, Size, UpdateCtx, Vec2, Widget,
    WidgetPod,
};

pub struct SideBarSelection<T> {
    label: WidgetPod<T, Box<dyn Widget<T>>>,
    icon: Svg,
    icon_data: String,
    pub id: u64,
    pub click_event: fn(&SideBarSelection<T>),
    pressing: bool,
}

impl<T: Data> SideBarSelection<T> {
    pub fn new(icon: String, text: impl Into<LabelText<T>>, id: u64) -> SideBarSelection<T> {
        let this = SideBarSelection {
            label: WidgetPod::new(Box::new(Label::new(text).with_text_size(14.0))),
            icon: Svg::new(
                icon.replace("{color}", "#000000")
                    .parse::<SvgData>()
                    .unwrap(),
            ),
            icon_data: icon,
            id,
            click_event: |_| {},
            pressing: false,
        };
        this
    }

    pub fn set_enabled(&mut self, state: bool) {
        self.pressing = state;
    }

    pub fn set_click_event(&mut self, event: fn(&SideBarSelection<T>)) {
        self.click_event = event;
    }

    pub fn update_state(&mut self, id: u64) {
        if self.id == id {
            self.set_enabled(true);
        } else {
            self.set_enabled(false);
        }
    }
}

impl<T: Data> Widget<T> for SideBarSelection<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(event) => {
                if !ctx.is_disabled() && event.button == MouseButton::Left {
                    // ctx.set_active(true);
                    (self.click_event)(self);
                    self.pressing = true;
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                }
                // ctx.set_active(true);
                self.pressing = false;
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env);
        self.icon.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, data, env);
        self.icon.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let padding = Size::new(8.0, 8.0);
        let icon_bc = bc.shrink(padding).loosen();
        let icon_size = self.icon.layout(ctx, &icon_bc, data, env);

        self.label
            .set_origin(ctx, Point::new(icon_size.width + 4.0, 5.5));
        let _label_size = self.label.layout(ctx, bc, data, env);
        bc.min()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        // println!("{}", self.enabled);
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let is_pressing = self.pressing && !ctx.is_disabled();
        let is_hot = ctx.is_hot();
        let size = ctx.size();
        let stroke_width = env.get(theme::BUTTON_BORDER_WIDTH);

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let bg_gradient = if is_pressing {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_ACTIVE)
        } else if is_hot {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_HOT)
        } else if is_active {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_ACTIVE)
        } else {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON)
        };

        let border_color = if is_pressing {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_BORDER_ACTIVE)
        } else if is_hot {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_BORDER_HOT)
        } else if is_active {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_ACTIVE)
        } else {
            env.get(crate::theme::theme::COLOR_CLEAR_BUTTON_BORDER)
        };

        ctx.fill(rounded_rect, &bg_gradient);

        ctx.stroke(rounded_rect, &border_color, stroke_width);

        ctx.with_save(|ctx| {
            let svg_data = self
                .icon_data
                .replace(
                    "{color}",
                    color_as_hex_string(env.get(theme::TEXT_COLOR)).as_str(),
                )
                .parse::<SvgData>()
                .unwrap();
            self.icon = Svg::new(svg_data);
            ctx.transform(Affine::scale(1.0).then_translate(Vec2::new(0.5, 0.0)));
            self.icon.paint(ctx, data, env)
        });

        ctx.with_save(|ctx| {
            self.label.paint(ctx, data, env);
        });
    }
}
