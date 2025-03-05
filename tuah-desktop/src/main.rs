mod capture;
mod gui;
mod signaling;
mod utils;

use eframe::egui;
use gui::TuahApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hawk-Tuah",
        options,
        Box::new(|_cc| Ok(Box::<TuahApp>::default())),
    )
}
