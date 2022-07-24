use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::Rc,
};

use geo::{coord, point, Coordinate, EuclideanDistance, Point};

use crate::{
    binder::{Binder, Draw, DrawLineOpts, EventType, HookEvent},
    elem::{Elem, Status},
};

// #[derive(Debug, Clone, Copy)]
pub enum Mode {
    EditMoving, // default
    Creating(Option<Box<dyn Elem>>),
    EditResizing(u8),
    Deleting,
}

pub struct Drawpanel {
    elems: Vec<Box<dyn Elem>>,
    hover_index: i32,
    drag_vertex: i32,
    prev_coord: Coordinate,
    mode: Mode,
    hook_event: Box<dyn HookEvent>,
}

impl Drawpanel {
    pub fn new(mut binder: impl Binder) -> Rc<RefCell<Self>> {
        let drawpanel = Rc::new(RefCell::new(Drawpanel {
            elems: Vec::new(),
            hover_index: -1,
            drag_vertex: -1,
            mode: Mode::EditMoving,
            prev_coord: Coordinate::default(),
            hook_event: binder.hook_event(),
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

                match self.mode.borrow_mut() {
                    Mode::EditMoving => {
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
                    Mode::Creating(elem) => {
                        if let Some(mut elem) = elem.take() {
                            self.hook_event.before_create(&mut elem);
                            self.elems.push(elem);
                        } else {
                            let elem = self.elems.last_mut();
                            self.hook_event.after_create(elem.unwrap());
                            self.mode = Mode::EditMoving;
                        }
                    }
                    Mode::EditResizing(_) => {}
                    Mode::Deleting => {
                        if idx > -1 {
                            elems.remove(idx as usize);
                        }
                    }
                }
            }
            EventType::Released => {
                match self.mode {
                    Mode::EditMoving => {}
                    Mode::Creating(_) => {
                        // let elem = self.elems.last_mut();
                        // self.hook_event.after_create(elem.unwrap());
                    }
                    Mode::EditResizing(_) => {
                        self.mode = Mode::EditMoving;
                    }
                    Mode::Deleting => {}
                }
            }
            EventType::Drag => match self.mode {
                Mode::Creating(_) => {
                    let top = elems.last_mut();
                    if let Some(elem) = top {
                        elem.creating(self.prev_coord, mouse_coord);
                        self.hook_event.creating(elem, mouse_coord);
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
        self.elems.push(Box::new(elem));
    }

    pub fn elems(&self) -> &Vec<Box<dyn Elem>> {
        &self.elems
    }

    pub fn set_mode(&mut self, mode: Mode) {
        *self.mode.borrow_mut() = mode;
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }
}
