use std::{cell::RefCell, rc::Rc};
use std::{fmt::Debug, rc::Weak};

use drawpanel_core::{
    binder::{
        Binder, Draw, DrawCircleOpts, DrawLineOpts, DrawRectOpts, EventMouseButton, EventRect,
        EventType, EventZoom, HookEvent, IDraw, IHookEvent,
    },
    drawpanel::Drawpanel,
    elem::{rect::Rect, Elem, IElem},
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
            egui_ctx: RefCell::new(None),
        })
    }

    fn hook_event(&self) -> Box<dyn IHookEvent> {
        Box::new(EguiHookEvent::default())
    }

    fn region(&self) -> geo::Rect<f64> {
        geo::Rect::new(coord!(x: 0.0, y: 0.0), coord!(x: 100.0, y: 100.0))
    }
}

#[derive(Debug)]
struct EguiDraw {
    shapes: RefCell<Option<Vec<egui::Shape>>>,
    egui_ctx: RefCell<Option<egui::Context>>,
}

impl IDraw for EguiDraw {}

impl Draw for EguiDraw {
    fn draw_begin(&self, ctx: Box<dyn std::any::Any>) {
        let mut shapes = self.shapes.borrow_mut();
        *shapes = Some(Vec::new());
        let ctx: Result<Box<egui::Context>, _> = ctx.downcast();
        if let Ok(ctx) = ctx {
            let mut egui_ctx = self.egui_ctx.borrow_mut();
            *egui_ctx = Some(*ctx);
        }
    }
    fn draw_line(&self, opts: DrawLineOpts) {
        let mut shapes = self.shapes.borrow_mut();

        if let Some(shapes) = shapes.as_mut() {
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

    fn draw_text(&self, opts: drawpanel_core::binder::DrawTextOpts) {
        let mut shapes = self.shapes.borrow_mut();

        if let Some(shapes) = shapes.as_mut() {
            let binding = self.egui_ctx.borrow();
            let fonts = binding.as_ref().unwrap().fonts();
            shapes.push(egui::Shape::text(
                &fonts,
                egui::pos2(
                    (opts.left_top_coord.x + opts.width / 2.) as f32,
                    (opts.left_top_coord.y + opts.height / 2.) as f32,
                ),
                egui::Align2::CENTER_CENTER,
                opts.content,
                egui::FontId::new(opts.font_size as f32, egui::FontFamily::default()),
                egui::Color32::RED,
            ));
        }
    }

    fn draw_end(&self) -> Box<dyn std::any::Any> {
        // println!("[DEBUG] draw_end");
        return Box::new(self.shapes.clone());
    }
}

#[derive(Debug, Clone, Default)]
pub struct EguiHookEvent {
    pub input_rect: Option<EventRect>,
    pub input_text: Option<String>,
}

impl EguiHookEvent {}

impl IHookEvent for EguiHookEvent {}

impl HookEvent for EguiHookEvent {
    fn begin_edit_state(&mut self, elem: &mut Box<dyn IElem>, event_rect: EventRect) {
        println!("[DEBUG] begin_edit_state");
        if elem.need_input() {
            self.input_rect = Some(event_rect);
            self.input_text = Some(elem.get_content().to_owned());
            elem.set_content("");
        }
    }

    fn end_edit_state(&mut self, elem: &mut Box<dyn IElem>, mouse_coord: Coordinate) {
        println!("[DEBUG] end_edit_state");
        if elem.need_input() {
            elem.set_content(&self.input_text.as_ref().unwrap().to_owned());
            self.input_rect = None;
            self.input_text = None;
        }
    }

    fn flush(&mut self) {}

    fn get_state(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }

    fn set_state(&mut self, state: Box<dyn std::any::Any>) {
        if let Ok(state) = state.downcast::<Self>() {
            *self = *state;
        }
    }
}
