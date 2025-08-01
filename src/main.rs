use eframe::egui;
use log::info;
use std::path::PathBuf;

mod app;
mod data_models;
mod excel_exporter;
mod xml_parser;
mod analytics;

use app::EsubpoenaApp;

fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    env_logger::init();
    info!("Starting eSubpoena Tolls Tool");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        resizable: true,
        transparent: false,
        decorated: true,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "eSubpoena Tolls Tool",
        options,
        Box::new(|_cc| {
            Box::new(EsubpoenaApp::new())
        }),
    )
} 