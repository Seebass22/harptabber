#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod syntax_highlight;
pub use app::GUIApp;
