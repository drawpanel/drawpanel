use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::Rc,
};

use geo::{coord, point, Coordinate, EuclideanDistance, Point};

use crate::{
    binder::{Binder, Draw, DrawLineOpts, EventType},
    elem::{Elem, Status},
};

pub enum Mode {
    EditMoving, // default
    Creating,
    EditResizing(u8),
    Deleting,
}

pub struct Drawpanel {
    elems: Vec<Box<dyn Elem>>,
    hover_index: i32,
    drag_vertex: i32,
    prev_coord: Coordinate,
    mode: Mode,
}

impl Drawpanel {
    pub fn new(mut binder: impl Binder) -> Rc<RefCell<Self>> {
        let drawpanel = Rc::new(RefCell::new(Drawpanel {
            elems: Vec::new(),
            hover_index: -1,
            drag_vertex: -1,
            mode: Mode::EditMoving,
            prev_coord: Coordinate::default(),
        }));

        binder.init(Rc::clone(&drawpanel));

        drawpanel
    }

    pub fn trigger_draw(&mut self, draw: Box<dyn Draw>) {
        for (i, elem) in self.elems.iter().enumerate() {
            elem.draw(
                draw.borrow(),
                if i as i32 == self.hover_index {
                    if let Mode::EditResizing(darg_point_index) = self.mode {
                        Status::Resizing(darg_point_index)
                    } else {
                        Status::Hover
                    }
                } else {
                    Status::Default
                },
            );
        }
    }

    pub fn trigger_event(&mut self, event_type: EventType, mouse_coord: Coordinate) {
        let mouse_point = point!(mouse_coord);
        let hover_index = &mut self.hover_index;
        let drag_vertex = &mut self.drag_vertex;
        let elems = &mut self.elems;
        match event_type {
            EventType::Move => {
                *hover_index.borrow_mut() = -1;
                let len = elems.len();
                for (i, elem) in elems.iter().rev().enumerate() {
                    if elem.hover_condition(mouse_point) {
                        *hover_index.borrow_mut() = (len - i - 1) as i32;
                        break;
                    }
                }
            }
            EventType::Push => {
                self.prev_coord = mouse_coord;
                let idx = *hover_index.borrow_mut();
                let elem = elems.get_mut(idx as usize);
                if let Some(elem) = elem {
                    let vertex = elem.get_vertex();
                    for (i, coord) in vertex.iter().enumerate() {
                        let point = Point::new(coord.x, coord.y);
                        if mouse_point.euclidean_distance(&point) < 10. {
                            self.mode = Mode::EditResizing(i as u8);
                            *drag_vertex.borrow_mut() = i as i32;
                        }
                    }
                }
            }
            EventType::Released => {
                self.mode = Mode::EditMoving;
            }
            EventType::Drag => match self.mode {
                Mode::Creating => {
                    let top = elems.last_mut();
                    if let Some(elem) = top {
                        elem.creating(self.prev_coord, mouse_coord);
                    }
                }
                Mode::EditMoving => {
                    let idx = *hover_index.borrow_mut();
                    let elem = elems.get_mut(idx as usize);
                    if let Some(elem) = elem {
                        elem.edit_moving(self.prev_coord, mouse_coord);
                        self.prev_coord = mouse_coord;
                    }
                }
                Mode::EditResizing(_) => {
                    let idx = *hover_index.borrow_mut();
                    let elem = elems.get_mut(idx as usize);
                    if let Some(elem) = elem {
                        elem.edit_resizing(self.prev_coord, mouse_coord, *drag_vertex);
                    }
                }
                Mode::Deleting => {}
            },
        };
    }

    pub fn append(&mut self, elem: impl Elem + 'static) {
        self.set_status(Mode::Creating);
        self.elems.push(Box::new(elem));
    }

    pub fn elems(&self) -> &Vec<Box<dyn Elem>> {
        &self.elems
    }

    pub fn set_status(&mut self, mode: Mode) {
        *self.mode.borrow_mut() = mode;
    }
}
