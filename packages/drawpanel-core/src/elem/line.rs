use std::rc::Rc;

use crate::binder::{Binder, Draw, DrawCircleOpts, DrawLineOpts};

use super::{Elem, Status};

use geo::{coord, Coordinate, EuclideanDistance, Point};

#[derive(Debug, Copy, Clone, Default)]
pub struct Line {
    pub from_coord: Coordinate,
    pub end_coord: Coordinate,
}

impl Elem for Line {
    fn draw(&self, draw: &Box<dyn Draw>, status: Status, scale: f64) {
        let line_color = 0xff0000;
        match status {
            Status::Hover => {
                draw.draw_line(DrawLineOpts {
                    from_coord: self.from_coord,
                    end_coord: self.end_coord,
                    line_size: 5,
                    line_color,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: self.from_coord,
                    r: 5.,
                    line_size: 1,
                    line_color,
                    fill_color: 0,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: self.end_coord,
                    r: 5.,
                    line_size: 1,
                    line_color,
                    fill_color: 0,
                });
            }
            Status::Resizing(darg_point_index) => {
                draw.draw_line(DrawLineOpts {
                    from_coord: self.from_coord,
                    end_coord: self.end_coord,
                    line_size: 3,
                    line_color,
                });

                draw.draw_circle(DrawCircleOpts {
                    center_coord: self.from_coord,
                    r: 5.,
                    line_size: 1,
                    line_color,
                    fill_color: 0,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: self.end_coord,
                    r: 5.,
                    line_size: 1,
                    line_color,
                    fill_color: 0,
                });
            }
            _ => {
                let center = coord! {x: 100., y: 100.};
                let mut from = self.from_coord.clone();
                let mut end = self.end_coord.clone();
                // from.x = (from.x * scale);
                // from.y = from.y * scale;
                // end.x = end.x * scale;
                // end.y = end.y * scale;
                // println!(
                //     "from:{:?},end:{:?},line:{:?}",
                //     from,
                //     end,
                //     (line.dx(), line.dy())
                // );

                draw.draw_line(DrawLineOpts {
                    from_coord: from,
                    end_coord: end,
                    line_size: 3, //((3 as f64) * scale) as u32,
                    line_color,
                });
            }
        }
    }

    fn get_vertex(&self) -> Vec<Coordinate> {
        vec![self.from_coord, self.end_coord]
    }

    fn creating(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        self.from_coord = from_coord;
        self.end_coord = end_coord;
    }

    fn edit_moving(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        let x_dif = end_coord.x - from_coord.x;
        let y_dif = end_coord.y - from_coord.y;

        self.from_coord.x = self.from_coord.x + x_dif;
        self.from_coord.y = self.from_coord.y + y_dif;
        self.end_coord.x = self.end_coord.x + x_dif;
        self.end_coord.y = self.end_coord.y + y_dif;
    }

    fn edit_resizing(&mut self, from_coord: Coordinate, end_coord: Coordinate, drag_vertex: i32) {
        match drag_vertex {
            0 => {
                self.from_coord = end_coord;
            }
            1 => {
                self.end_coord = end_coord;
            }
            _ => (),
        }
    }

    fn hover_condition(&self, mouse_point: Point) -> bool {
        let t_line = geo::Line::new(self.from_coord, self.end_coord);
        mouse_point.euclidean_distance(&t_line) < 10.
    }
}
