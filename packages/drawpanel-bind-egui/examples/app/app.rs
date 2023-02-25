use std::{any::Any, borrow::Borrow, cell::RefCell, rc::Rc};

use drawpanel_bind_egui::EguiBinder;
use drawpanel_core::{
    binder::{EventMouseButton, EventType},
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
                    if ui.button("Line").clicked() {
                        self.drawpanel.set_mode(Mode::Creating(Some(Box::new(
                            drawpanel_core::elem::line::Line::default(),
                        ))));
                        println!("Line");
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(&ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                let (mut response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

                let panel = panel.upgrade();
                let panel = panel.unwrap().clone();
                let mut panel = panel.borrow_mut();

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
                        println!("elems {:?}", panel.elems);
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

                let shapes: Box<RefCell<Option<Vec<egui::Shape>>>> =
                    panel.trigger_draw().downcast().unwrap();
                if let Some(shapes) = shapes.borrow_mut().as_mut() {
                    painter.extend(shapes.clone());
                }

                response
            });
        });
        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });

        //     ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         *value += 1.0;
        //     }

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to(
        //                 "eframe",
        //                 "https://github.com/emilk/egui/tree/master/crates/eframe",
        //             );
        //             ui.label(".");
        //         });
        //     });
        // });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // The central panel the region left after adding TopPanel's and SidePanel's

        //     ui.heading("eframe template");
        //     ui.hyperlink("https://github.com/emilk/eframe_template");
        //     ui.add(egui::github_link_file!(
        //         "https://github.com/emilk/eframe_template/blob/master/",
        //         "Source code."
        //     ));
        //     egui::warn_if_debug_build(ui);
        // });

        // if true {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally choose either panels OR windows.");
        //     });
        // }
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
