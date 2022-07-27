use std::{cell::RefCell, rc::Rc};

use drawpanel_core::{
    binder::{Binder, Draw, DrawCircleOpts, DrawLineOpts, DrawRectOpts, EventType, HookEvent},
    drawpanel::Drawpanel,
    elem::{text::Text, Elem},
};
use fltk::{
    app,
    draw::{self, LineStyle},
    enums::{Align, CallbackTrigger, Color, Event, Font, FrameType},
    frame::{self, Frame},
    input,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
    window,
};
use geo::{coord, Coordinate};

pub struct FltkBinder {
    frame: Frame,
    input: input::MultilineInput,
}

impl FltkBinder {
    pub fn new(frame: Frame, mut win: window::DoubleWindow) -> Self {
        let mut input = input::MultilineInput::default();
        win.add(&input);
        input.set_frame(FrameType::NoBox);
        input.set_align(Align::Center);

        input.handle({
            let mut frame = frame.clone();
            move |inp, e| match e {
                Event::KeyDown => {
                    inp.redraw();
                    frame.redraw();
                    true
                }
                _ => false,
            }
        });

        FltkBinder { frame, input }
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
            let mut input = self.input.clone();
            let drawpanel = Rc::clone(&drawpanel);
            move |frm, e| {
                let (x, y) = app::event_coords();
                let is_double = app::event_clicks();
                let mouse_coord = coord! {
                    x: x as f64,
                    y: y as f64
                };
                match e {
                    Event::Move => {
                        (*drawpanel)
                            .borrow_mut()
                            .trigger_event(EventType::Move, mouse_coord);
                        input.redraw();
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
                        if is_double {
                            (*drawpanel)
                                .borrow_mut()
                                .trigger_event(EventType::Dblclick, mouse_coord);
                        } else {
                            (*drawpanel)
                                .borrow_mut()
                                .trigger_event(EventType::Released, mouse_coord);
                        }
                        frm.redraw();
                        true
                    }
                    _ => false,
                }
            }
        });
    }

    fn hook_event(&self) -> Box<dyn drawpanel_core::binder::HookEvent> {
        Box::new(FltkHookEvent {
            input: self.input.clone(),
            frame: self.frame.clone(),
        })
    }

    fn draw(&self) -> Box<dyn Draw> {
        Box::new(FltkDraw)
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

    fn draw_text(&self, opts: drawpanel_core::binder::DrawTextOpts) {
        draw::set_draw_color(Color::from_hex(opts.font_color));
        draw::set_font(Font::Screen, opts.font_size as i32);
        draw::draw_text2(
            &opts.content,
            opts.left_top_coord.x as i32,
            opts.left_top_coord.y as i32,
            opts.width as i32,
            opts.height as i32,
            Align::Center,
        )
    }
}

struct FltkHookEvent {
    input: input::MultilineInput,
    frame: frame::Frame,
}

impl HookEvent for FltkHookEvent {
    fn before_create(&mut self, elem: &mut Box<dyn Elem>) {}
    fn creating(&mut self, elem: &mut Box<dyn Elem>, mouse_coord: Coordinate) {
        if elem.need_input() {
            let ver = elem.get_vertex();
            let left_top = ver.get(0).unwrap();
            let right_bottom = ver.get(2).unwrap();
            self.input
                .set_pos((left_top.x + 3.) as i32, (left_top.y + 3.) as i32);
            self.input.set_size(
                (right_bottom.x - left_top.x - 6.) as i32,
                (right_bottom.y - left_top.y - 6.) as i32,
            );
            self.input.take_focus().unwrap();
        }
    }
    fn after_create(&mut self, elem: &mut Box<dyn Elem>) {
        if elem.need_input() {
            let value = self.input.value();
            elem.set_content(&value);
            self.input.set_size(0, 0);
            self.input.set_value("");
        }
    }

    fn edit_state(&mut self, elem: &mut Box<dyn Elem>, mouse_coord: Coordinate) {
        self.input.set_value(elem.get_content());
        elem.set_content("");
        self.creating(elem, mouse_coord);
    }

    fn edit_end(&mut self, elem: &mut Box<dyn Elem>) {
        self.after_create(elem)
    }

    fn flush(&mut self) {
        self.frame.redraw();
    }
}
