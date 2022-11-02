#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![windows_subsystem = "windows"]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::epaint::Vec2;
    use harptabber_gui::GUIApp;

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1000.0, 800.0)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "harmonica tab transposer",
        native_options,
        Box::new(|cc| Box::new(GUIApp::new(cc))),
    );
}
