use std::{default, rc::Rc};

use crate::binder::{Binder, Draw, DrawCircleOpts, DrawLineOpts};

use super::{Elem, Status};

use geo::{coord, Coordinate, EuclideanDistance, Line, LineString, Point, Polygon};

#[derive(Debug, Clone)]
pub struct Pen {
    coords: Vec<Coordinate>,
    // last_coord: Option<Coordinate>,
}

impl Default for Pen {
    fn default() -> Self {
        Pen {
            coords: vec![Coordinate::default()],
        }
    }
}

impl Elem for Pen {
    fn draw(&self, draw: &Box<dyn Draw>, status: Status, scale: f64) {
        let line_color = 0xff0000;
        match status {
            Status::Hover => {
                for (i, coord) in self.coords.iter().enumerate() {
                    if i > 1 {
                        let prev = self.coords.get(i - 1).unwrap();
                        draw.draw_line(DrawLineOpts {
                            from_coord: *prev,
                            end_coord: *coord,
                            line_size: 8,
                            line_color,
                        });
                        draw.draw_circle(DrawCircleOpts {
                            center_coord: *prev,
                            r: 4.,
                            line_size: 0,
                            line_color,
                            fill_color: line_color,
                        });
                    }
                }
                // draw.draw_circle(DrawCircleOpts {
                //     center_coord: *self.coords.first().unwrap(),
                //     r: 5,
                //     line_size: 1,
                //     line_color,
                //     fill_color: 0,
                // });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *self.coords.last().unwrap(),
                    r: 8.,
                    line_size: 1,
                    line_color: 0,
                    fill_color: 0,
                });
            }
            Status::Resizing(darg_point_index) => {
                for (i, coord) in self.coords.iter().enumerate() {
                    if i > 1 {
                        let prev = self.coords.get(i - 1).unwrap();
                        draw.draw_line(DrawLineOpts {
                            from_coord: *prev,
                            end_coord: *coord,
                            line_size: 4,
                            line_color,
                        });
                        draw.draw_circle(DrawCircleOpts {
                            center_coord: *coord,
                            r: 2.,
                            line_size: 0,
                            line_color,
                            fill_color: line_color,
                        });
                    }
                }
                // draw.draw_circle(DrawCircleOpts {
                //     center_coord: *self.coords.first().unwrap(),
                //     r: if darg_point_index == 1 { 6 } else { 5 },
                //     line_size: 1,
                //     line_color,
                //     fill_color: 0,
                // });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *self.coords.last().unwrap(),
                    r: 8.,
                    line_size: 1,
                    line_color: 0,
                    fill_color: 0,
                });
            }
            _ => {
                for (i, coord) in self.coords.iter().enumerate() {
                    if i > 1 {
                        let prev = self.coords.get(i - 1).unwrap();
                        draw.draw_line(DrawLineOpts {
                            from_coord: *prev,
                            end_coord: *coord,
                            line_size: 4,
                            line_color,
                        });
                        draw.draw_circle(DrawCircleOpts {
                            center_coord: *prev,
                            r: 2.,
                            line_size: 0,
                            line_color,
                            fill_color: line_color,
                        });
                    }
                }
            }
        }
    }

    fn get_vertex(&self) -> Vec<Coordinate> {
        vec![*self.coords.first().unwrap(), *self.coords.last().unwrap()]
    }

    fn creating(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        if self.coords.len() == 1 {
            (*self.coords.get_mut(0).unwrap()) = from_coord;
        }
        let last = self.coords.last().unwrap();
        if Point::new(last.x, last.y).euclidean_distance(&Point::new(end_coord.x, end_coord.y)) > 1.
        {
            self.coords.push(end_coord);
        }
        // self.last_coord = Some(end_coord);
    }

    fn edit_moving(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        let x_dif = end_coord.x - from_coord.x;
        let y_dif = end_coord.y - from_coord.y;

        for coord in self.coords.iter_mut() {
            coord.x += x_dif;
            coord.y += y_dif;
        }
    }

    fn edit_resizing(&mut self, from_coord: Coordinate, end_coord: Coordinate, drag_vertex: i32) {
        match drag_vertex {
            0 => {}
            1 => {
                self.creating(from_coord, end_coord);
            }
            _ => (),
        }
    }

    fn hover_condition(&self, mouse_point: Point) -> bool {
        LineString::new(self.coords.clone()).euclidean_distance(&mouse_point) < 10.
    }
}
