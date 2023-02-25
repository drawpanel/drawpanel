use std::{cell::RefCell, rc::Rc};
use std::{fmt::Debug, rc::Weak};

use drawpanel_core::{
    binder::{
        Binder, Draw, DrawCircleOpts, DrawLineOpts, DrawRectOpts, EventMouseButton, EventRect,
        EventType, EventZoom, HookEvent, IDraw, IHookEvent,
    },
    drawpanel::Drawpanel,
    elem::{Elem, IElem},
    panel::Panel,
};
use egui::{Frame, PointerButton, Pos2, Sense};
use geo::{coord, Coordinate};

#[derive(Debug, Clone)]
pub struct EguiBinder {}

impl EguiBinder {
    pub fn new() -> Self {
        EguiBinder {}
    }
}

impl Binder for EguiBinder {
    fn init(&mut self, panel: Weak<RefCell<Panel>>) {}

    fn draw(&self, panel: Weak<RefCell<Panel>>) -> Box<dyn IDraw> {
        Box::new(EguiDraw {
            shapes: RefCell::new(None),
            panel: panel,
        })
    }

    fn hook_event(&self) -> Box<dyn IHookEvent> {
        Box::new(EguiHookEvent { flush: false })
    }

    fn region(&self) -> geo::Rect<f64> {
        geo::Rect::new(coord!(x: 0.0, y: 0.0), coord!(x: 100.0, y: 100.0))
    }
}

#[derive(Debug)]
struct EguiDraw {
    shapes: RefCell<Option<Vec<egui::Shape>>>,
    panel: Weak<RefCell<Panel>>,
}

impl IDraw for EguiDraw {}

impl Draw for EguiDraw {
    fn draw_begin(&self) {
        let mut shapes = self.shapes.borrow_mut();
        *shapes = Some(Vec::new());
        // println!("[DEBUG] draw_begin {:?}", self.shapes);
    }
    fn draw_line(&self, opts: DrawLineOpts) {
        let mut shapes = self.shapes.borrow_mut();

        if let Some(shapes) = shapes.as_mut() {
            println!("draw_line {:?}", opts.line_size);
            shapes.push(egui::Shape::line(
                vec![
                    Pos2::new(opts.from_coord.x as f32, opts.from_coord.y as f32),
                    Pos2::new(opts.end_coord.x as f32, opts.end_coord.y as f32),
                ],
                egui::Stroke::new(opts.line_size as f32, egui::Color32::RED),
            ));
        }
    }

    fn draw_rect(&self, opts: DrawRectOpts) {
        let mut shapes = self.shapes.borrow_mut();

        if let Some(shapes) = shapes.as_mut() {
            shapes.push(egui::Shape::rect_stroke(
                egui::Rect::from_min_size(
                    Pos2::new(opts.left_top_coord.x as f32, opts.left_top_coord.y as f32),
                    egui::Vec2::new(opts.width as f32, opts.height as f32),
                ),
                egui::Rounding::default(),
                egui::Stroke::new(opts.line_size as f32, egui::Color32::RED),
            ));
        }
    }

    fn draw_circle(&self, opts: DrawCircleOpts) {
        let mut shapes = self.shapes.borrow_mut();

        if let Some(shapes) = shapes.as_mut() {
            shapes.push(egui::Shape::circle_filled(
                Pos2::new(opts.center_coord.x as f32, opts.center_coord.y as f32),
                opts.r as f32,
                egui::Color32::RED,
                // egui::Stroke::new(opts.line_size as f32, egui::Color32::RED),
            ));
        }
    }

    fn draw_text(&self, opts: drawpanel_core::binder::DrawTextOpts) {}

    // fn update(&mut self, ctx: Box<dyn std::any::Any>) {
    //     // let shapes = self.shapes.clone();
    //     // let mut shapes = shapes.borrow_mut();
    //     // println!("UPDATE {:?}", shapes);

    //     // shapes.push(egui::Shape::line(
    //     //     vec![Pos2::new(0 as f32, 0 as f32), Pos2::new(1 as f32, 1 as f32)],
    //     //     egui::Stroke::new(1 as f32, egui::Color32::RED),
    //     // ));
    // }
    fn draw_end(&self) -> Box<dyn std::any::Any> {
        // println!("[DEBUG] draw_end");
        return Box::new(self.shapes.clone());
    }
}

#[derive(Debug)]
struct EguiHookEvent {
    flush: bool,
}

impl IHookEvent for EguiHookEvent {}

impl HookEvent for EguiHookEvent {
    fn begin_edit_state(&mut self, elem: &mut Box<dyn IElem>, event_rect: EventRect) {}

    fn end_edit_state(&mut self, elem: &mut Box<dyn IElem>, mouse_coord: Coordinate) {}

    fn flush(&mut self) {
        self.flush = true;
    }
}
