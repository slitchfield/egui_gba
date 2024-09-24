#![warn(clippy::all, rust_2018_idioms)]

mod arm7tdmi;
mod gba_emu;
mod app;
pub use app::EmulatorApp;
