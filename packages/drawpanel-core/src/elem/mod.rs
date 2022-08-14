pub mod line;
pub mod pen;
pub mod rect;
pub mod text;

use geo::{Coordinate, Point};

use crate::draw_wrap::DrawWrap;

pub enum Status {
    Default,
    Hover,
    Creating,
    Resizing(u8),
}

pub trait Elem {
    fn draw(&self, draw: &DrawWrap, status: Status);
    fn get_vertex(&self) -> Vec<Coordinate<f64>>;
    fn get_content(&self) -> &str {
        ""
    }
    fn set_content(&mut self, content: &str) {}
    fn need_input(&self) -> bool {
        false
    }
    fn creating(&mut self, from_coord: Coordinate, end_coord: Coordinate);
    fn edit_moving(&mut self, from_coord: Coordinate, end_coord: Coordinate);
    fn edit_resizing(&mut self, from_coord: Coordinate, end_coord: Coordinate, drag_vertex: i32);
    fn hover_condition(&self, mouse_point: Point) -> bool;
}
