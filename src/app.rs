use crate::egui_priv::*;
use std::collections::HashMap;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    palette: HashMap<String, egui::Color32>,
    faces: HashMap<String, String>, // face name: palette color
}

/*impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            palette: HashMap::new()
        }
    }
}*/

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }*/

        let mut new: Self = Default::default();
        new.palette.insert("Red".to_string(), egui::Color32::RED);
        new.palette
            .insert("Green".to_string(), egui::Color32::GREEN);
        new.palette
            .insert("White".to_string(), egui::Color32::WHITE);
        new.palette.insert("Blue".to_string(), egui::Color32::BLUE);
        new.faces.insert("R".to_string(), "Red".to_string());
        new.faces.insert("U".to_string(), "White".to_string());
        new.faces.insert("F".to_string(), "Green".to_string());
        new
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            //egui::Grid::new("colors").min_col_width(0.0).show(ui, |ui| {
            for face in ["R", "U", "F"] {
                // i can't make a mutable view &mut [u8; 3] of a Color so i have to do this
                let color = self.palette[&self.faces[face]];
                if color_button(ui, color, false).clicked() {}
                //Ray::ray_to_color_mut(prefs)[ray] = color.into();
                ui.label(face);
                ui.end_row();
            }
            //});
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
