pub mod line;
pub mod rect;

use std::rc::Rc;

use geo::{Coordinate, Point};

use crate::binder::Draw;

pub enum Status {
    Default,
    Hover,
    Resizing(u8),
}

pub trait Elem {
    fn draw(&self, draw: &Box<dyn Draw>, status: Status);
    fn get_vertex(&self) -> Vec<Coordinate<f64>>;
    fn creating(&mut self, from_coord: Coordinate, end_coord: Coordinate);
    fn edit_moving(&mut self, from_coord: Coordinate, end_coord: Coordinate);
    fn edit_resizing(&mut self, from_coord: Coordinate, end_coord: Coordinate, drag_vertex: i32);
    fn hover_condition(&self, mouse_point: Point) -> bool;
}
