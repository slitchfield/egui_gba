#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod arm7tdmi;
mod gba_emu;
mod util;
pub use app::EmulatorApp;
