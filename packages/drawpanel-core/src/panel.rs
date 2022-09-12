use std::{borrow::BorrowMut, cell::RefCell, collections::HashSet, rc::Rc, time::Instant};

use crate::{
    binder::{
        Draw, DrawCircleOpts, DrawLineOpts, DrawRectOpts, DrawTextOpts, EventRect, EventType,
        EventZoom, HookEvent,
    },
    draw_wrap::DrawWrap,
    drawpanel::Mode,
    elem::{line::Line, rect::Rect, Elem, Status},
};

use geo::{coord, point, Coordinate, EuclideanDistance, Intersects, Point};

// #[derive(Debug)]
pub struct Panel {
    pub raw_lt_coord: Coordinate,
    pub lt_coord: Coordinate,
    pub width: f64,
    pub height: f64,
    pub scale: f64,
    pub scale_coord: Option<Coordinate>,
    pub elems: Vec<Box<dyn Elem>>,
    pub hover_index: isize,
    pub drag_vertex: isize,
    pub mode: Mode,
    pub prev_coord: Coordinate,
    pub draw: Box<dyn Draw>,
    pub hook_event: Box<dyn HookEvent>,
    pub select_box: Option<Rect>,
    pub selects: HashSet<u32>,
}

impl Panel {
    pub fn new(
        draw: Box<dyn Draw>,
        hook_event: Box<dyn HookEvent>,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    ) -> Panel {
        Panel {
            raw_lt_coord: coord! { x: x, y: y },
            lt_coord: coord! { x: x, y: y },
            width: w,
            height: h,
            scale: 1.,
            scale_coord: None,
            hover_index: -1,
            drag_vertex: -1,
            mode: Mode::EditMoving,
            prev_coord: coord! { x: 0., y:0. },
            elems: vec![],
            draw,
            hook_event,

            select_box: None,
            selects: HashSet::new(),
        }
    }

    pub fn trigger_draw(&self) {
        let draw = &self.draw;
        draw.draw_rect(DrawRectOpts {
            left_top_coord: self.lt_coord,
            width: self.width * self.scale,
            height: self.height * self.scale,
            line_size: 3,
            line_color: 0x000000,
            fill_color: 0,
        });
        if let Some(scale_coord) = &self.scale_coord {
            draw.draw_circle(DrawCircleOpts {
                center_coord: *scale_coord,
                r: 5.,
                line_size: 0,
                line_color: 0x000000,
                fill_color: 0x000000,
            })
        }
        let draw = DrawWrap::new(&draw, self);
        for (i, elem) in self.elems.iter().enumerate() {
            elem.draw(
                &draw,
                if i == (self.hover_index as usize) {
                    if let Mode::EditResizing(darg_point_index) = self.mode {
                        Status::Resizing(darg_point_index)
                    } else {
                        Status::Hover
                    }
                } else if let Mode::Creating(_) = self.mode {
                    Status::Creating
                } else if self.selects.contains(&(i as u32)) {
                    Status::Hover
                } else {
                    Status::Default
                },
            );
        }
        if let Some(select_box) = &self.select_box {
            select_box.draw(&draw, Status::Creating)
        }
    }

    pub fn trigger_event(&mut self, event_type: EventType, inp_mouse_coord: Coordinate) {
        let relative_coord = self.relative_coord(inp_mouse_coord);
        let mouse_point = point!(relative_coord);
        let hover_index = &mut self.hover_index;
        let drag_vertex = &mut self.drag_vertex;

        match event_type {
            EventType::Move => {
                if let Mode::EditState = self.mode {
                } else {
                    *hover_index.borrow_mut() = -1;
                    let len = self.elems.len();
                    for (i, elem) in self.elems.iter().rev().enumerate() {
                        if elem.hover_condition(mouse_point) {
                            *hover_index = (len - i - 1) as isize;
                            break;
                        }
                    }
                }
            }
            EventType::Push => {
                self.prev_coord = relative_coord;
                let idx = *hover_index;

                match &mut self.mode {
                    Mode::EditMoving => {
                        let elem = self.elems.get_mut(idx as usize);
                        if let Some(elem) = elem {
                            let vertex = elem.get_vertex();
                            for (i, coord) in vertex.iter().enumerate() {
                                let point = Point::new(coord.x, coord.y);
                                if mouse_point.euclidean_distance(&point) < 10. {
                                    self.mode = Mode::EditResizing(i as u8);
                                    *drag_vertex = i as isize;
                                }
                            }
                        }
                        if let Some(select_box) = &self.select_box {
                            let select_box_ver = select_box.get_vertex();
                            let tl = select_box_ver.get(0).unwrap();
                            let br = select_box_ver.get(2).unwrap();
                            let box_rect = geo::Rect::new(*tl, *br);
                            if box_rect
                                .to_polygon()
                                .euclidean_distance(&geo::Point::from(relative_coord))
                                > 0.
                            {
                                self.selects.clear();
                                self.select_box = None;
                            }
                        }
                    }
                    Mode::Creating(elem) => {
                        if let Some(elem) = elem.take() {
                            self.hook_event.begin_create(&elem, relative_coord);
                            self.elems.push(elem);
                        } else {
                            let elem = self.elems.last_mut();
                            self.hook_event.end_create(elem.unwrap(), relative_coord);
                            self.mode = Mode::EditMoving;
                        }
                    }
                    Mode::EditResizing(_) => {}
                    Mode::Deleting => {
                        if idx > -1 {
                            self.elems.remove(idx as usize);
                            self.selects.clear();
                            self.select_box = None;
                        } else if let Some(select_box) = self.select_box.borrow_mut() {
                            if select_box.hover_condition(mouse_point) {
                                let mut ver = Vec::from_iter(self.selects.iter());
                                ver.sort();
                                for select in ver.iter().rev() {
                                    self.elems.remove(**select as usize);
                                }

                                self.selects.clear();
                                self.select_box = None;
                                self.hook_event.flush();
                            }
                        }
                    }
                    Mode::EditState => {
                        let elem = self.elems.last_mut();
                        self.mode = Mode::EditMoving;
                        self.hook_event
                            .end_edit_state(elem.unwrap(), relative_coord);
                    }
                    Mode::Select => {
                        self.selects.clear();
                        self.select_box = Some(Rect::default());
                    }
                }
            }
            EventType::Released => match self.mode {
                Mode::EditMoving => {}
                Mode::Creating(_) => {
                    let elem = self.elems.last().unwrap();
                    if elem.need_input() {
                        let vec = elem.get_vertex();
                        let event_rect = self.calc_event_rect(vec);
                        let elem = self.elems.last_mut().unwrap();
                        self.hook_event.end_create(elem, relative_coord);
                        self.hook_event.begin_edit_state(elem, event_rect);
                        self.mode = Mode::EditState;
                    }
                }
                Mode::EditResizing(_) => {
                    self.mode = Mode::EditMoving;
                }
                Mode::Deleting => {}
                Mode::EditState => {}
                Mode::Select => {
                    let select_box = self.select_box.borrow_mut().as_ref().unwrap();
                    let select_box_ver = select_box.get_vertex();
                    let tl: Coordinate<f64> = *select_box_ver.get(0).unwrap();
                    let br: Coordinate<f64> = *select_box_ver.get(2).unwrap();
                    let box_rect = geo::Rect::new(tl, br);
                    for (i, elem) in self.elems.iter().enumerate() {
                        let ver = elem.get_vertex();
                        let mut is_select = true;
                        for coord in ver {
                            if !box_rect.intersects(&geo::Point::from(coord)) {
                                is_select = false;
                            }
                        }
                        if is_select {
                            self.selects.insert(i as u32);
                        }
                    }
                    self.mode = Mode::EditMoving;
                }
            },
            EventType::Drag => match self.mode {
                Mode::Creating(_) => {
                    let top = self.elems.last_mut();
                    if let Some(elem) = top {
                        elem.creating(self.prev_coord, relative_coord);
                        self.hook_event.doing_create(elem, relative_coord);
                    }
                }
                Mode::EditMoving => {
                    if self.selects.is_empty() {
                        let idx = *hover_index;
                        let elem = self.elems.get_mut(idx as usize);
                        if let Some(elem) = elem {
                            elem.edit_moving(self.prev_coord, relative_coord);
                            self.prev_coord = relative_coord;
                        }
                    } else {
                        for idx in self.selects.iter() {
                            let elem = self.elems.get_mut((*idx) as usize);
                            if let Some(elem) = elem {
                                elem.edit_moving(self.prev_coord, relative_coord);
                            }
                        }
                        let select_box = self.select_box.as_mut().unwrap();
                        select_box.edit_moving(self.prev_coord, relative_coord);
                        self.prev_coord = relative_coord;
                    }
                }
                Mode::EditResizing(_) => {
                    let idx = *hover_index;
                    let elem = self.elems.get_mut(idx as usize);
                    if let Some(elem) = elem {
                        elem.edit_resizing(self.prev_coord, relative_coord, *drag_vertex as i32);
                    }
                }
                Mode::Deleting => {}
                Mode::EditState => {}
                Mode::Select => {
                    if let Some(select_box) = &mut self.select_box {
                        select_box.creating(self.prev_coord, relative_coord);
                    }
                }
            },
            EventType::Dblclick => {
                let idx = *hover_index;
                if idx > -1 {
                    let vec = self.elems.get(idx as usize).unwrap().get_vertex();
                    let event_rect = self.calc_event_rect(vec);

                    let elem = self.elems.get_mut(idx as usize).unwrap();
                    self.mode = Mode::EditState;
                    self.hook_event.begin_edit_state(elem, event_rect);
                }
            }
            EventType::Zoom(zoom) => match zoom {
                EventZoom::None => {}
                EventZoom::Up => {
                    self.set_scale(self.scale + 0.01, inp_mouse_coord.x, inp_mouse_coord.y);
                }
                EventZoom::Down => {
                    self.set_scale(self.scale - 0.01, inp_mouse_coord.x, inp_mouse_coord.y);
                }
            },
        };
        self.hook_event.flush();
    }

    fn calc_event_rect(&self, ver: Vec<Coordinate>) -> EventRect {
        let left_top = ver.get(0).unwrap();
        let right_bottom = ver.get(2).unwrap();
        let w_h = *right_bottom - *left_top;
        EventRect {
            coord: self.absolute_coord(*left_top),
            width: w_h.x * self.scale,
            height: w_h.y * self.scale,
        }
    }

    pub fn set_scale(&mut self, val: f64, x: f64, y: f64) {
        let zoom = self.scale - val;
        let scale_coord = coord! { x:x, y:y  };

        self.scale = val;
        self.scale_coord = Some(scale_coord);
        self.lt_coord.x = self.lt_coord.x * (1. - zoom) + scale_coord.x * zoom;
        self.lt_coord.y = self.lt_coord.y * (1. - zoom) + scale_coord.y * zoom;
        self.hook_event.flush();
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn scale_mut(&mut self, val: f64) {
        self.scale = val;
    }

    pub fn relative_coord(&self, coord: Coordinate) -> Coordinate {
        let relative_coord = coord - self.lt_coord;
        let relative_coord = relative_coord / self.scale;
        relative_coord
    }

    pub fn absolute_coord(&self, coord: Coordinate) -> Coordinate {
        coord! {
            x: (self.lt_coord.x + coord.x) * self.scale + self.lt_coord.x * (1. - self.scale),
            y: (self.lt_coord.y + coord.y) * self.scale + self.lt_coord.y * (1. - self.scale),
        }
    }
}
