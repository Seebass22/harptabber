#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![windows_subsystem = "windows"]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use harptabber_gui::GUIApp;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "harmonica tab transposer",
        native_options,
        Box::new(|cc| Box::new(GUIApp::new(cc))),
    );
}
