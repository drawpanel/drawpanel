use std::{any::Any, borrow::Borrow, cell::RefCell, rc::Rc};

use drawpanel_bind_egui::{EguiBinder, EguiHookEvent};
use drawpanel_core::{
    binder::{EventMouseButton, EventRect, EventType, EventZoom},
    drawpanel::{Drawpanel, Mode},
};
use eframe::emath;
use egui::{Color32, Frame, PointerButton, Pos2, Rect, Sense, Shape, Stroke};
use geo::coord;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    drawpanel: Drawpanel,
    my_string: String,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        setup_custom_fonts(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        Self {
            drawpanel: Drawpanel::new(EguiBinder::new()),
            my_string: "Hello World!".to_string(),
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        let panel = self.drawpanel.panel();

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Draw", |ui| {
                    if ui.button("Pen").clicked() {
                        self.drawpanel.set_mode(Mode::Creating(Some(Box::new(
                            drawpanel_core::elem::pen::Pen::default(),
                        ))));
                        println!("Pen");
                        ui.close_menu();
                    }
                    if ui.button("Line").clicked() {
                        self.drawpanel.set_mode(Mode::Creating(Some(Box::new(
                            drawpanel_core::elem::line::Line::default(),
                        ))));
                        println!("Line");
                        ui.close_menu();
                    }
                    if ui.button("Rect").clicked() {
                        self.drawpanel.set_mode(Mode::Creating(Some(Box::new(
                            drawpanel_core::elem::rect::Rect::default(),
                        ))));
                        println!("Rect");
                        ui.close_menu();
                    }
                    if ui.button("Text").clicked() {
                        self.drawpanel.set_mode(Mode::Creating(Some(Box::new(
                            drawpanel_core::elem::text::Text::default(),
                        ))));
                        println!("Text");
                        ui.close_menu();
                    }
                });
            });
        });

        // egui::Window::new("Drawpanel")
        //     .open(&mut true)
        //     .default_size(egui::vec2(512.0, 512.0))
        //     .vscroll(false)
        //     .title_bar(false)
        //     .show(ctx, |ui| {
        //         let response = ui.add(egui::TextEdit::singleline(&mut self.my_string));
        //         if response.changed() {
        //             println!("my_string: {:?}", self.my_string);
        //         }
        //     });

        egui::CentralPanel::default().show(&ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                let (mut response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

                let panel = panel.upgrade();
                let panel = panel.unwrap().clone();
                let mut panel = panel.borrow_mut();

                // panel.trigger_event2(
                //     EventType::None,
                //     coord! { x: 0.0, y: 0.0 },
                //     Box::new(ui),
                // );

                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    if response.dragged_by(PointerButton::Primary) {
                        if response.drag_started() {
                            // println!("Drag Started {:?}", pointer_pos);
                            panel.trigger_event(
                                EventType::Push(EventMouseButton::Left),
                                coord! {
                                    x: pointer_pos.x as f64,
                                    y: pointer_pos.y as f64
                                },
                            );
                        }
                    }
                    if response.drag_released() {
                        // println!("Drag Released {:?}", pointer_pos);
                        panel.trigger_event(
                            EventType::Released(EventMouseButton::Left),
                            coord! {
                                x: pointer_pos.x as f64,
                                y: pointer_pos.y as f64
                            },
                        );
                    }
                }

                if response.dragged() {
                    if let Some(pointer_pos) = response.hover_pos() {
                        panel.trigger_event(
                            EventType::Drag(EventMouseButton::Left),
                            coord! {
                                x: pointer_pos.x as f64,
                                y: pointer_pos.y as f64
                            },
                        );
                    }
                } else if let Some(pointer_pos) = response.hover_pos() {
                    panel.trigger_event(
                        EventType::Move(EventMouseButton::Left),
                        coord! {
                            x: pointer_pos.x as f64,
                            y: pointer_pos.y as f64
                        },
                    );
                }
                if let Some(pointer_pos) = response.hover_pos() {
                    ui.ctx().input().events.iter().for_each(|event| {
                        if let egui::Event::Scroll(v) = event {
                            panel.trigger_event(
                                EventType::Zoom(if v.y > 0.0 {
                                    EventZoom::Grow
                                } else {
                                    EventZoom::Dwindle
                                }),
                                coord! {
                                    x: pointer_pos.x as f64,
                                    y: pointer_pos.y as f64
                                },
                            );
                        }
                    });
                }

                if response.double_clicked_by(PointerButton::Primary) {
                    if let Some(pointer_pos) = response.hover_pos() {
                        panel.trigger_event(
                            EventType::Dblclick,
                            coord! {
                                x: pointer_pos.x as f64,
                                y: pointer_pos.y as f64
                            },
                        );
                    }
                }

                // 绘图
                let shapes: Box<RefCell<Option<Vec<egui::Shape>>>> = panel
                    .trigger_draw2(Box::new(ctx.clone()))
                    .downcast()
                    .unwrap();
                if let Some(shapes) = shapes.borrow_mut().as_mut() {
                    painter.extend(shapes.clone());
                }

                // 输入框处理
                if let Some(hook_event) = panel.hook_event.as_mut() {
                    let hook: EguiHookEvent = *hook_event.get_state().downcast().unwrap();
                    if let Some(input_rect) = hook.input_rect {
                        self.my_string = hook.input_text.unwrap();
                        println!("input_rect: {:?}", input_rect);
                        let input_box = ui.put(
                            egui::Rect::from_min_size(
                                egui::pos2(
                                    (input_rect.coord.x) as f32,
                                    (input_rect.coord.y) as f32,
                                ),
                                egui::vec2(input_rect.width as f32, input_rect.height as f32),
                            ),
                            egui::TextEdit::multiline(&mut self.my_string).desired_rows(1),
                        );
                        input_box.request_focus();

                        hook_event.set_state(Box::new(EguiHookEvent {
                            input_rect: Some(input_rect),
                            input_text: Some(self.my_string.clone()),
                        }));
                    }
                }

                response
            });
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();
    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/SmileySans.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
