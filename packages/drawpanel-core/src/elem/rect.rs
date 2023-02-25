use crate::{
    binder::{Draw, DrawCircleOpts, DrawRectOpts},
    draw_wrap::DrawWrap,
};

use super::{Elem, IElem, Status};
use educe::Educe;
use geo::{coord, point, Coordinate, EuclideanDistance, Intersects, Point};

#[derive(Debug, Clone, Educe)]
#[educe(Default)]
pub struct Rect {
    pub lt_coord: Coordinate, // left top coord
    pub width: f64,
    pub height: f64,
    #[educe(Default = 3.)]
    pub line_size: f64,
    #[educe(Default = 0xff0000)]
    pub line_color: u32,
    pub fill_color: Option<u32>,
    pub line_style: LineStyle,
}

impl IElem for Rect {}

impl Elem for Rect {
    fn draw(&self, draw: &DrawWrap<'_>, status: Status) {
        let line_color = self.line_color;
        let line_size = self.line_size;
        let fill_color = self.fill_color;
        let line_style = self.line_style.clone();
        let drag_coords = self.get_vertex();

        match status {
            Status::Hover => {
                draw.draw_rect(DrawRectOpts {
                    left_top_coord: self.lt_coord,
                    width: self.width,
                    height: self.height,
                    line_size: line_size + 2.,
                    line_color,
                    fill_color,
                    line_style,
                });

                let lt = drag_coords.get(0).unwrap();
                let tr = drag_coords.get(1).unwrap();
                let br = drag_coords.get(2).unwrap();
                let bl = drag_coords.get(3).unwrap();
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *lt,
                    r: line_size as f64 + 2.,
                    line_size: 0.,
                    line_color,
                    fill_color: 0,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *tr,
                    r: line_size as f64 + 2.,
                    line_size: 0.,
                    line_color,
                    fill_color: 0,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *br,
                    r: line_size as f64 + 2.,
                    line_size: 0.,
                    line_color,
                    fill_color: 0,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *bl,
                    r: line_size as f64 + 2.,
                    line_size: 0.,
                    line_color,
                    fill_color: 0,
                });
            }
            Status::Resizing(index) => {
                draw.draw_rect(DrawRectOpts {
                    left_top_coord: self.lt_coord,
                    width: self.width,
                    height: self.height,
                    line_size,
                    line_color,
                    fill_color,
                    line_style,
                });

                let lt = drag_coords.get(0).unwrap();
                let tr = drag_coords.get(1).unwrap();
                let br = drag_coords.get(2).unwrap();
                let bl = drag_coords.get(3).unwrap();
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *lt,
                    r: line_size as f64 + 2.,
                    line_size: 0.,
                    line_color,
                    fill_color: 0,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *tr,
                    r: line_size as f64 + 2.,
                    line_size: 0.,
                    line_color,
                    fill_color: 0,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *br,
                    r: line_size as f64 + 2.,
                    line_size: 0.,
                    line_color,
                    fill_color: 0,
                });
                draw.draw_circle(DrawCircleOpts {
                    center_coord: *bl,
                    r: line_size as f64 + 2.,
                    line_size: 0.,
                    line_color,
                    fill_color: 0,
                });
            }
            _ => {
                draw.draw_rect(DrawRectOpts {
                    left_top_coord: self.lt_coord,
                    width: self.width,
                    height: self.height,
                    line_size,
                    line_color,
                    fill_color,
                    line_style,
                });
            }
        }
    }

    fn get_vertex(&self) -> Vec<Coordinate> {
        let mut tl = coord! {x: self.lt_coord.x, y: self.lt_coord.y};
        let mut tr = coord! {x: 0., y: 0.};
        let mut br = coord! {x: self.lt_coord.x + self.width, y: self.lt_coord.y + self.height};
        let mut bl = coord! {x: 0., y: 0.};

        let mut t = 0.;
        if tl.x > br.x {
            t = tl.x;
            tl.x = br.x;
            br.x = t;
        }
        if tl.y > br.y {
            t = tl.y;
            tl.y = br.y;
            br.y = t;
        }

        bl.x = tl.x;
        bl.y = br.y;

        tr.x = br.x;
        tr.y = tl.y;

        Vec::from([tl, tr, br, bl])
    }

    fn creating(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        let mut tfrom = from_coord.clone();
        let mut tend = end_coord.clone();

        if tfrom.x > tend.x {
            let t = tfrom.x;
            tfrom.x = tend.x;
            tend.x = t;
        }

        if tfrom.y > tend.y {
            let t = tfrom.y;
            tfrom.y = tend.y;
            tend.y = t;
        }

        self.lt_coord = tfrom;
        self.width = tend.x - tfrom.x;
        self.height = tend.y - tfrom.y;
    }

    fn edit_moving(&mut self, from_coord: Coordinate, end_coord: Coordinate) {
        let x_dif = end_coord.x - from_coord.x;
        let y_dif = end_coord.y - from_coord.y;

        self.lt_coord.x += x_dif;
        self.lt_coord.y += y_dif;
    }

    fn edit_resizing(&mut self, from_coord: Coordinate, end_coord: Coordinate, drag_vertex: i32) {
        match drag_vertex {
            0 => {
                self.width += self.lt_coord.x - end_coord.x;
                self.height += self.lt_coord.y - end_coord.y;
                self.lt_coord = end_coord;
            }
            1 => {
                self.width = end_coord.x - self.lt_coord.x;
                self.height += self.lt_coord.y - end_coord.y;
                self.lt_coord.y = end_coord.y;
            }
            2 => {
                self.width = end_coord.x - self.lt_coord.x;
                self.height = end_coord.y - self.lt_coord.y;
            }
            3 => {
                self.height = end_coord.y - self.lt_coord.y;
                self.width += self.lt_coord.x - end_coord.x;
                self.lt_coord.x = end_coord.x;
            }
            _ => (),
        }
        if self.width < 0. {
            self.width = 0.;
            self.lt_coord.x = end_coord.x;
        }
        if self.height < 0. {
            self.height = 0.;
            self.lt_coord.y = end_coord.y;
        }
    }

    fn hover_condition(&self, mouse_point: Point) -> bool {
        let vertex = self.get_vertex();
        geo::Rect::new(vertex[0], vertex[2]).intersects(&mouse_point)
            || point! {vertex[0]}.euclidean_distance(&mouse_point) < 10.
            || point! {vertex[1]}.euclidean_distance(&mouse_point) < 10.
            || point! {vertex[2]}.euclidean_distance(&mouse_point) < 10.
            || point! {vertex[3]}.euclidean_distance(&mouse_point) < 10.
    }

    fn elem_type(&self) -> String {
        "rect".to_string()
    }

    fn export(&self) -> String {
        format!(
            "{},{},{},{}",
            self.lt_coord.x, self.lt_coord.y, self.width, self.height
        )
    }

    fn import(&self, content: &str) -> Box<dyn IElem> {
        let mut coords = content.split(',');
        let lt_x = coords.next().unwrap().parse::<f64>().unwrap();
        let lt_y = coords.next().unwrap().parse::<f64>().unwrap();
        let width = coords.next().unwrap().parse::<f64>().unwrap();
        let height = coords.next().unwrap().parse::<f64>().unwrap();

        Box::new(Rect {
            lt_coord: coord! {x: lt_x, y: lt_y},
            width,
            height,
            ..Default::default()
        })
    }
}

#[derive(Debug, Clone, Educe)]
#[educe(Default)]
pub enum LineStyle {
    #[educe(Default)]
    Solid,
    Dotted,
}
