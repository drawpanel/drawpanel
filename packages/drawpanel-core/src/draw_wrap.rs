use crate::{
    binder::{Draw, DrawCircleOpts, DrawLineOpts, DrawRectOpts, DrawTextOpts, IDraw},
    panel::Panel,
};

pub struct DrawWrap<'a> {
    draw: &'a Box<dyn IDraw>,
    panel: &'a Panel,
}

impl<'a> DrawWrap<'a> {
    pub fn new(draw: &'a Box<dyn IDraw>, panel: &'a Panel) -> Self {
        Self { draw, panel }
    }
}

impl Draw for DrawWrap<'_> {
    fn draw_line(&self, opts: DrawLineOpts) {
        let scale = self.panel.scale;
        let from_coord = self.panel.absolute_coord(opts.from_coord);
        let end_coord = self.panel.absolute_coord(opts.end_coord);

        self.draw.draw_line(DrawLineOpts {
            from_coord,
            end_coord,
            line_size: opts.line_size * (scale as u32),
            line_color: opts.line_color,
        });
    }

    fn draw_rect(&self, opts: DrawRectOpts) {
        let scale = self.panel.scale;
        let left_top_coord = self.panel.absolute_coord(opts.left_top_coord);

        self.draw.draw_rect(DrawRectOpts {
            left_top_coord,
            width: opts.width * scale,
            height: opts.height * scale,
            line_size: opts.line_size * (scale as u32),
            line_color: opts.line_color,
            fill_color: opts.fill_color,
            line_style: opts.line_style,
        })
    }

    fn draw_circle(&self, opts: DrawCircleOpts) {
        let scale = self.panel.scale;
        let center_coord = self.panel.absolute_coord(opts.center_coord);

        self.draw.draw_circle(DrawCircleOpts {
            line_size: opts.line_size * (scale as u32),
            line_color: opts.line_color,
            fill_color: opts.fill_color,
            center_coord,
            r: opts.r * scale,
        })
    }

    fn draw_text(&self, opts: DrawTextOpts) {
        let scale = self.panel.scale;
        let left_top_coord = self.panel.absolute_coord(opts.left_top_coord);

        self.draw.draw_text(DrawTextOpts {
            left_top_coord,
            width: opts.width * scale,
            height: opts.height * scale,
            content: opts.content,
            font_size: opts.font_size * (scale as u32),
            font_space: opts.font_space * (scale as u32),
            font_color: opts.font_color,
        })
    }
}
