use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
};

use geo::{
    coord, point, ChamberlainDuquetteArea, Coordinate, EuclideanDistance, Intersects, Point,
};

use crate::{
    binder::{Binder, Draw, DrawLineOpts, DrawRectOpts, EventType, HookEvent},
    elem::{rect::Rect, Elem, Status},
};

// #[derive(Debug, Clone, Copy)]
pub enum Mode {
    EditMoving, // default
    Creating(Option<Box<dyn Elem>>),
    EditResizing(u8),
    Deleting,
    EditState,
    Select,
}

pub struct Drawpanel {
    elems: Vec<Box<dyn Elem>>,
    select_box: Option<Rect>,
    selects: HashSet<u32>,
    hover_index: i32,
    drag_vertex: i32,
    prev_coord: Coordinate,
    mode: Mode,
    hook_event: Box<dyn HookEvent>,
    draw: Box<dyn Draw>,
}

impl Drawpanel {
    pub fn new(mut binder: impl Binder) -> Rc<RefCell<Self>> {
        let drawpanel = Rc::new(RefCell::new(Drawpanel {
            elems: Vec::new(),
            hover_index: -1,
            drag_vertex: -1,
            mode: Mode::EditMoving,
            select_box: None,
            selects: HashSet::new(),
            prev_coord: Coordinate::default(),
            hook_event: binder.hook_event(),
            draw: binder.draw(),
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

    pub fn trigger_event(&mut self, event_type: EventType, mouse_coord: Coordinate) {
        let mouse_point = point!(mouse_coord);
        let hover_index = &mut self.hover_index;
        let drag_vertex = &mut self.drag_vertex;
        let elems = &mut self.elems;
        match event_type {
            EventType::Move => {
                if let Mode::EditState = self.mode {
                } else {
                    *hover_index.borrow_mut() = -1;
                    let len = elems.len();
                    for (i, elem) in elems.iter().rev().enumerate() {
                        if elem.hover_condition(mouse_point) {
                            *hover_index.borrow_mut() = (len - i - 1) as i32;
                            break;
                        }
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
                        if let Some(select_box) = &self.select_box {
                            let select_box_ver = select_box.get_vertex();
                            let tl = select_box_ver.get(0).unwrap();
                            let br = select_box_ver.get(2).unwrap();
                            let box_rect = geo::Rect::new(*tl, *br);
                            if box_rect
                                .to_polygon()
                                .euclidean_distance(&geo::Point::from(mouse_coord))
                                > 0.
                            {
                                self.selects.clear();
                                self.select_box = None;
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
                            (*self.selects.borrow_mut()).clear();
                        } else if let Some(select_box) = self.select_box.borrow_mut() {
                            if select_box.hover_condition(mouse_point) {
                                let mut ver = Vec::from_iter(self.selects.iter());
                                ver.sort();
                                for select in ver.iter().rev() {
                                    elems.remove(**select as usize);
                                }

                                self.selects.clear();
                                self.select_box = None;
                                self.hook_event.flush();
                            }
                        }
                    }
                    Mode::EditState => {
                        let elem = self.elems.last_mut();
                        self.hook_event.after_create(elem.unwrap());
                        self.mode = Mode::EditMoving;
                    }
                    Mode::Select => {
                        self.selects.clear();
                        self.select_box = Some(Rect::default());
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
                    if self.selects.is_empty() {
                        let idx = *hover_index.borrow_mut();
                        let elem = elems.get_mut(idx as usize);
                        if let Some(elem) = elem {
                            elem.edit_moving(self.prev_coord, mouse_coord);
                            self.prev_coord = mouse_coord;
                        }
                    } else {
                        for idx in self.selects.iter() {
                            let elem = elems.get_mut((*idx) as usize);
                            if let Some(elem) = elem {
                                elem.edit_moving(self.prev_coord, mouse_coord);
                            }
                        }
                        let select_box = self.select_box.as_mut().unwrap();
                        select_box.edit_moving(self.prev_coord, mouse_coord);
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
                Mode::EditState => {}
                Mode::Select => {
                    if let Some(select_box) = self.select_box.borrow_mut() {
                        select_box.creating(self.prev_coord, mouse_coord);
                    }
                }
            },
            EventType::Dblclick => {
                let idx = *hover_index.borrow_mut();
                let elem = elems.get_mut(idx as usize);
                if let Some(elem) = elem {
                    self.mode = Mode::EditState;
                    self.hook_event.edit_state(elem, mouse_coord);
                }
            }
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
