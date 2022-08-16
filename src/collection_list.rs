use egui::*;
use log::info;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(PartialEq)]
pub struct ScrollTo {
    collections: Vec<String>,
    tack_item_align: Option<Align>,
}

impl Default for ScrollTo {
    fn default() -> Self {
        Self {
            collections: vec!["a".to_string(),"a".to_string()],
            tack_item_align: Some(Align::Center),
        }
    }
}

impl ScrollTo {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("collections");

        ui.separator();

        ScrollArea::vertical()
            .max_width(180.0)
            .max_height(100.0)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for item in 1..=50 {
                        if ui
                            .add(Label::new(format!("This is item {}", item)).sense(Sense::click()))
                            .clicked()
                        {
                            info!("clicked item {}", item);
                        }
                    }
                });
            })
            .inner;

        ui.separator();
    }
}
