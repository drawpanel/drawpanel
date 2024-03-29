use std::{
    cell::RefCell,
    collections::HashSet,
    rc::{Rc, Weak},
};

use crate::{
    binder::Binder,
    elem::{self, IElem},
    panel::Panel,
};

#[derive(Debug)]
pub enum Mode {
    EditMoving, // default
    Creating(Option<Box<dyn IElem>>),
    EditResizing(u8),
    Deleting,
    EditState,
    Select,
}

pub struct Drawpanel {
    panel: Rc<RefCell<Panel>>,
}

impl Drawpanel {
    pub fn new(mut binder: impl Binder) -> Self {
        let region = binder.region();
        let panel = Rc::new(RefCell::new(Panel::new(
            region.min().x,
            region.min().y,
            region.width(),
            region.height(),
            vec![
                Box::new(elem::pen::Pen::default()) as Box<dyn IElem>,
                Box::new(elem::line::Line::default()) as Box<dyn IElem>,
                Box::new(elem::rect::Rect::default()) as Box<dyn IElem>,
                Box::new(elem::text::Text::default()) as Box<dyn IElem>,
            ],
        )));
        let drawpanel = Drawpanel {
            panel: panel.clone(),
        };

        binder.init(Rc::downgrade(&panel));
        drawpanel
            .panel
            .borrow_mut()
            .set_draw(binder.draw(Rc::downgrade(&panel)));
        drawpanel
            .panel
            .borrow_mut()
            .set_hook_event(binder.hook_event());

        drawpanel
    }

    pub fn flush(&mut self) {
        let mut panel = self.panel.borrow_mut();
        if let Some(hook_event) = &mut panel.hook_event {
            hook_event.flush();
        }
    }

    pub fn set_scale(&mut self, val: f64, x: f64, y: f64) {
        let mut panel = (*self.panel).borrow_mut();
        panel.set_scale(val, x, y)
    }

    pub fn scale(&self) -> f64 {
        let panel = (*self.panel).borrow();
        panel.scale
    }

    // pub fn scale_mut(&mut self) -> &mut f64 {
    //     let mut panel = (*self.panel).borrow_mut();
    //     let scale = panel.scale.borrow_mut();

    //     scale
    // }

    // pub fn elems(&self) -> &Vec<Box<dyn Elem>> {
    //     &self.panel.elems
    // }

    pub fn set_mode(&mut self, mode: Mode) {
        let mut panel = (*self.panel).borrow_mut();
        panel.mode = mode;
    }

    // pub fn mode(&self) -> &Mode {
    //     &self.borrow().panel.mode
    // }

    // pub fn mode_mut(&mut self) -> &mut Mode {
    //     &mut self.borrow_mut().panel.mode
    // }

    pub fn export(&self) -> String {
        return self.panel.borrow().export();
    }

    pub fn import(&mut self, data: &str) {
        let mut panel = (*self.panel).borrow_mut();
        panel.import(data);
    }

    pub fn panel(&self) -> Weak<RefCell<Panel>> {
        Rc::downgrade(&self.panel)
    }
}
