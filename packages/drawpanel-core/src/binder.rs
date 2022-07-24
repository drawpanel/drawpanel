use std::{cell::RefCell, rc::Rc};

use geo::Coordinate;

use crate::{drawpanel::Drawpanel, elem::Elem};

pub enum EventType {
    Move,
    Push,
    Released,
    Drag,
}

pub struct DrawLineOpts {
    pub from_coord: Coordinate,
    pub end_coord: Coordinate,
    pub line_size: u32,
    pub line_color: u32,
}

pub struct DrawRectOpts {
    pub left_top_coord: Coordinate,
    pub width: f64,
    pub height: f64,
    pub line_size: u32,
    pub line_color: u32,
    pub fill_color: u32,
}

pub struct DrawCircleOpts {
    pub center_coord: Coordinate,
    pub r: f64,
    pub line_size: u32,
    pub line_color: u32,
    pub fill_color: u32,
}

pub struct DrawTextOpts<'a> {
    pub left_top_coord: Coordinate,
    pub width: f64,
    pub height: f64,
    pub content: &'a str,
    pub font_size: u32,
    pub font_space: u32,
    pub font_color: u32,
    pub background_color: u32,
    pub border_size: u32,
    pub border_color: u32,
}

pub trait Binder {
    fn init(&mut self, drawpanel: Rc<RefCell<Drawpanel>>);
    // fn draw(&self) -> Box<dyn Draw>;
    fn hook_event(&self) -> Box<dyn HookEvent>;
}

pub trait Draw {
    fn draw_line(&self, opts: DrawLineOpts);
    fn draw_rect(&self, opts: DrawRectOpts);
    fn draw_circle(&self, opts: DrawCircleOpts);
    fn draw_text(&self, opts: DrawTextOpts);
}

pub trait HookEvent {
    fn before_create(&mut self, elem: &mut Box<dyn Elem>) {}
    fn creating(&mut self, elem: &mut Box<dyn Elem>, mouse_coord: Coordinate) {}
    fn after_create(&mut self, elem: &mut Box<dyn Elem>) {}
}
