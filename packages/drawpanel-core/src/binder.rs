use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

use geo::Coordinate;

use crate::{
    drawpanel::Drawpanel,
    elem::{Elem, IElem},
    panel::Panel,
};

#[derive(Debug)]
pub enum EventZoom {
    None,
    Grow,
    Dwindle,
}

#[derive(Debug)]
pub enum EventMouseButton {
    None,
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
pub enum EventType {
    Move(EventMouseButton),
    Push(EventMouseButton), // Click, Dblclick, Mouseup(Left\Right\Mid), Mousedown
    Dblclick,
    Released(EventMouseButton),
    Drag(EventMouseButton),
    Zoom(EventZoom),
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
    pub fill_color: Option<u32>,
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
}

pub trait Binder {
    fn init(&mut self, panel: Rc<RefCell<Panel>>);
    fn draw(&self) -> Box<dyn IDraw>;
    fn hook_event(&self) -> Box<dyn IHookEvent>;
}

pub trait IDraw: Draw + Debug {}

pub trait Draw {
    fn draw_line(&self, opts: DrawLineOpts);
    fn draw_rect(&self, opts: DrawRectOpts);
    fn draw_circle(&self, opts: DrawCircleOpts);
    fn draw_text(&self, opts: DrawTextOpts);
}

pub trait IHookEvent: HookEvent + Debug {}

pub trait HookEvent {
    fn begin_create(
        &mut self,
        elem: &Box<dyn IElem>,
        mouse_coord: Coordinate,
        // panel: Rc<RefCell<Panel>>,
    ) {
    }
    fn doing_create(
        &mut self,
        elem: &mut Box<dyn IElem>,
        mouse_coord: Coordinate,
        // panel: Rc<RefCell<Panel>>,
    ) {
    }
    fn end_create(
        &mut self,
        elem: &mut Box<dyn IElem>,
        mouse_coord: Coordinate,
        // panel: Rc<RefCell<Panel>>,
    ) {
    }
    fn begin_edit_state(&mut self, elem: &mut Box<dyn IElem>, event_rect: EventRect) {}
    fn end_edit_state(
        &mut self,
        elem: &mut Box<dyn IElem>,
        mouse_coord: Coordinate,
        // panel: Rc<RefCell<Panel>>,
    ) {
    }
    fn flush(&mut self) {}
}

pub struct EventRect {
    pub coord: Coordinate,
    pub width: f64,
    pub height: f64,
}
