use std::{cell::RefCell, rc::Rc};

use geo::Coordinate;

use crate::drawpanel::Drawpanel;

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
    pub r: u32,
    pub line_size: u32,
    pub line_color: u32,
    pub fill_color: u32,
}

pub trait Binder {
    fn init(&mut self, drawpanel: Rc<RefCell<Drawpanel>>);
    // fn draw(&self) -> Box<dyn Draw>;
}

pub trait Draw {
    fn draw_line(&self, opts: DrawLineOpts);
    fn draw_rect(&self, opts: DrawRectOpts);
    fn draw_circle(&self, opts: DrawCircleOpts);
}
