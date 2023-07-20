use crate::util::color_as_hex_string;
use druid::widget::{Svg, SvgData};
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget,
};

pub struct Icon {
    icon: Svg,
    data: String,
}

impl Icon {
    pub fn new(data: String) -> Icon {
        Self {
            icon: Svg::new(
                data.clone()
                    .replace("{color}", "#000000")
                    .parse::<SvgData>()
                    .unwrap(),
            ),
            data,
        }
    }
}

impl<T: Data> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.icon.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.icon.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let icon_size = self.icon.layout(ctx, bc, data, env);
        let icon_size = bc.constrain(Size::new(icon_size.width, icon_size.height));
        icon_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        ctx.with_save(|ctx| {
            let svg_data = self
                .data
                .replace(
                    "{color}",
                    color_as_hex_string(Color::from(env.get(crate::theme::theme::COLOR_TEXT)))
                        .as_str(),
                )
                .parse::<SvgData>()
                .unwrap();
            self.icon = Svg::new(svg_data);
            self.icon.paint(ctx, data, env);
        });
    }
}
