use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

use drawpanel_core::{
    binder::{
        Binder, Draw, DrawCircleOpts, DrawLineOpts, DrawRectOpts, EventMouseButton, EventRect,
        EventType, EventZoom, HookEvent, IDraw, IHookEvent,
    },
    drawpanel::Drawpanel,
    elem::{Elem, IElem},
    panel::Panel,
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
    // panel: Option<Rc<RefCell<Panel>>>,
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
                _ => {
                    inp.redraw();
                    frame.redraw();
                    false
                }
            }
        });

        FltkBinder {
            frame,
            input,
            // panel: None,
        }
    }
}

impl Binder for FltkBinder {
    fn init(&mut self, panel: Rc<RefCell<Panel>>) {
        self.frame.draw({
            let drawpanel = Rc::clone(&panel);
            move |frm| {
                (*drawpanel).borrow_mut().trigger_draw();
            }
        });

        self.frame.handle({
            let mut input = self.input.clone();
            let drawpanel = Rc::clone(&panel);
            move |frm, e| {
                let (x, y) = app::event_coords();
                let is_double = app::event_clicks();
                let mouse_coord = coord! {
                    x: x as f64,
                    y: y as f64
                };
                let mouse_button = match app::event_mouse_button() {
                    app::MouseButton::Left => EventMouseButton::Left,
                    app::MouseButton::Middle => EventMouseButton::Middle,
                    app::MouseButton::Right => EventMouseButton::Right,
                    _ => EventMouseButton::None,
                };
                match e {
                    Event::Move => {
                        (*drawpanel)
                            .borrow_mut()
                            .trigger_event(EventType::Move(mouse_button), mouse_coord);
                        input.redraw();
                        true
                    }
                    Event::Push => {
                        (*drawpanel)
                            .borrow_mut()
                            .trigger_event(EventType::Push(EventMouseButton::None), mouse_coord);
                        true
                    }
                    Event::Drag => {
                        (*drawpanel)
                            .borrow_mut()
                            .trigger_event(EventType::Drag(mouse_button), mouse_coord);
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
                                .trigger_event(EventType::Released(mouse_button), mouse_coord);
                        }
                        true
                    }
                    Event::ZoomGesture | Event::MouseWheel => {
                        (*drawpanel).borrow_mut().trigger_event(
                            EventType::Zoom(match app::event_dy() {
                                app::MouseWheel::Up => EventZoom::Grow,
                                app::MouseWheel::Down => EventZoom::Dwindle,
                                _ => EventZoom::None,
                            }),
                            mouse_coord,
                        );
                        true
                    }
                    _ => false,
                }
            }
        });

        // self.panel = Some(panel.clone());
    }

    fn draw(&self) -> Box<dyn IDraw> {
        Box::new(FltkDraw)
    }

    fn hook_event(&self) -> Box<dyn IHookEvent> {
        Box::new(FltkHookEvent {
            input: self.input.clone(),
            frame: self.frame.clone(),
        })
    }
}

#[derive(Debug)]
struct FltkDraw;

impl IDraw for FltkDraw {}

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

#[derive(Debug)]
struct FltkHookEvent {
    input: input::MultilineInput,
    frame: frame::Frame,
    // panel: Rc<RefCell<Panel>>,
}

impl IHookEvent for FltkHookEvent {}

impl HookEvent for FltkHookEvent {
    // fn begin_create(&mut self, elem: &Box<dyn Elem>) {
    //     println!("before_create");
    // }
    // fn doing_create(&mut self, elem: &mut Box<dyn Elem>, mouse_coord: Coordinate) {
    //     println!("doing_create");
    // }
    // fn end_create(&mut self, elem: &mut Box<dyn Elem>) {
    //     println!("after_create");
    // }

    fn begin_edit_state(&mut self, elem: &mut Box<dyn IElem>, event_rect: EventRect) {
        self.input.set_value(elem.get_content());
        elem.set_content("");
        if elem.need_input() {
            self.input.set_pos(
                (event_rect.coord.x + 3.) as i32,
                (event_rect.coord.y + 3.) as i32,
            );
            self.input.set_size(
                (event_rect.width - 6.) as i32,
                (event_rect.height - 6.) as i32,
            );
            self.input.take_focus().unwrap();
        }
        println!("edit_state");
    }

    fn end_edit_state(&mut self, elem: &mut Box<dyn IElem>, mouse_coord: Coordinate) {
        if elem.need_input() {
            let value = self.input.value();
            elem.set_content(&value);
            self.input.set_size(0, 0);
            self.input.set_value("");
            println!("edit_end");
        }
    }

    fn flush(&mut self) {
        self.frame.redraw();
    }
}
