use egui::Align;
use egui::*;
use log::{error, info};
use mongodb::Client;
use std::collections::HashMap;
use tokio::runtime::Runtime;

use crate::collection_list;

/// We derive Deserialize/Serialize so we can &persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    uri: String,

    #[serde(skip)]
    clientMap: HashMap<String, Client>,
    collections: Vec<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            uri: "mongodb://127.0.0.1:27017".to_owned(),
            clientMap: HashMap::new(),
            collections: Vec::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            uri,
            clientMap,
            collections,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::TopBottomPanel::top("connect_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                ui.text_edit_singleline(uri);
                let connect_btn = Button::new("Connect");
                let mut connecting = true;
                if ui.add_enabled(!connecting, connect_btn).clicked() {
                    info!("Connecting: {}", uri.clone());

                    let rt = Runtime::new().unwrap();

                    let urix = uri.clone();
                    rt.spawn(async move {
                        match Client::with_uri_str("mongodb://example.com").await {
                            Ok(c) => {
                                let db = c.database("test");
                                match db.list_collection_names(None).await {
                                    Ok(cc) => {
                                        info!("Connecting OK");
                                        // connecting = false;
                                        // self.collections = cc;
                                        // clientMap.insert(urix.to_owned(), c.clone());
                                    }
                                    Err(e) => {
                                        error!("Error connecting to {}: {}", urix.clone(), e);
                                    }
                                };
                            }
                            Err(e) => {
                                error!("Error connecting: {}: {}", urix.clone(), e);
                            }
                        };
                    });
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut scroll_to =
                collection_list::ScrollTo::new(self.collections.clone(), Some(Align::Center));
            scroll_to.ui(ui);
        });

        // bottom
        egui::TopBottomPanel::bottom("footer_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
                egui::warn_if_debug_build(ui);
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
