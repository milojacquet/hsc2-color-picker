#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod egui_priv;
pub use app::TemplateApp;
pub use egui_priv::*;
