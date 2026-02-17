use eframe::egui;
use metronome::ui::app::MetronomeApp;

// The entry point of the application.
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([380.0, 420.0])
            .with_min_inner_size([320.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Savage Metronome",
        options,
        Box::new(|_cc| Ok(Box::new(MetronomeApp::new()))),
    )
}
