use std::{cell::RefCell, rc::Rc};

use drawpanel_core::{
    binder::{Binder, Draw, DrawCircleOpts, DrawLineOpts, DrawRectOpts, EventType},
    drawpanel::Drawpanel,
};
use fltk::{
    app,
    draw::{self, LineStyle},
    enums::{Color, Event, FrameType},
    frame::Frame,
    prelude::{WidgetBase, WidgetExt},
};
use geo::coord;

pub struct FltkBinder {
    frame: Frame,
}

impl FltkBinder {
    pub fn new(frame: Frame) -> Self {
        FltkBinder { frame }
    }
}

impl Binder for FltkBinder {
    fn init(&mut self, drawpanel: Rc<RefCell<Drawpanel>>) {
        self.frame.draw({
            let drawpanel = Rc::clone(&drawpanel);
            move |frm| {
                (*drawpanel).borrow_mut().trigger_draw(Box::new(FltkDraw));
            }
        });

        self.frame.handle({
            let drawpanel = Rc::clone(&drawpanel);
            move |frm, e| {
                let (x, y) = app::event_coords();
                let mouse_coord = coord! {
                    x: x as f64,
                    y: y as f64
                };
                match e {
                    Event::Move => {
                        (*drawpanel)
                            .borrow_mut()
                            .trigger_event(EventType::Move, mouse_coord);
                        frm.redraw();
                        true
                    }
                    Event::Push => {
                        (*drawpanel)
                            .borrow_mut()
                            .trigger_event(EventType::Push, mouse_coord);
                        frm.redraw();
                        true
                    }
                    Event::Drag => {
                        (*drawpanel)
                            .borrow_mut()
                            .trigger_event(EventType::Drag, mouse_coord);
                        frm.redraw();
                        true
                    }
                    Event::Released => {
                        (*drawpanel)
                            .borrow_mut()
                            .trigger_event(EventType::Released, mouse_coord);
                        frm.redraw();
                        true
                    }
                    _ => false,
                }
            }
        });
    }
}

struct FltkDraw;

impl Draw for FltkDraw {
    fn draw_line(&self, opts: DrawLineOpts) {
        draw::set_draw_color(Color::from_hex(opts.line_color));
        draw::set_line_style(LineStyle::Solid, opts.line_size as i32);
        draw::draw_line(
            opts.from_coord.x as i32,
            opts.from_coord.y as i32,
            opts.end_coord.x as i32,
            opts.end_coord.y as i32,
        );
    }

    fn draw_rect(&self, opts: DrawRectOpts) {
        draw::set_draw_color(Color::from_hex(opts.line_color));
        draw::set_line_style(LineStyle::Solid, opts.line_size as i32);
        draw::draw_rect(
            opts.left_top_coord.x as i32,
            opts.left_top_coord.y as i32,
            opts.width as i32,
            opts.height as i32,
        );
    }

    fn draw_circle(&self, opts: DrawCircleOpts) {
        draw::set_draw_color(Color::from_hex(opts.line_color));
        draw::set_line_style(LineStyle::Solid, opts.line_size as i32);
        let size = (opts.r * 2.) as i32;
        draw::draw_box(
            FrameType::OFlatFrame,
            (opts.center_coord.x - opts.r) as i32 + 1,
            (opts.center_coord.y - opts.r) as i32 + 1,
            size - 1,
            size - 1,
            Color::from_hex(opts.fill_color),
        );
    }
}
