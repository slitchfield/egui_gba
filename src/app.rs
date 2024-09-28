use egui::Color32;
use egui::RichText;

use std::future::Future;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::gba_emu::Gbaemu;

type PathBytesChannel = (Sender<(String, Vec<u8>)>, Receiver<(String, Vec<u8>)>);

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EmulatorApp {
    #[serde(skip)]
    device: Gbaemu,
    #[serde(skip)]
    bios_channel: PathBytesChannel,
    #[serde(skip)]
    rom_channel: PathBytesChannel,
}

impl Default for EmulatorApp {
    fn default() -> Self {
        Self {
            device: Gbaemu::default(),
            bios_channel: channel(),
            rom_channel: channel(),
        }
    }
}

impl EmulatorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for EmulatorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.device.advance_mem_cursor()
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.device.regress_mem_cursor()
        }

        // Quit if q is pressed
        if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Tick clock if space is pressed
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.device
                .tick_clock(1)
                .expect("Error thrown when executing system clock tick")
        }

        if ctx.input(|i| i.key_pressed(egui::Key::R)) {
            self.device.reset()
        }

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

                //egui::widgets::global_dark_light_mode_buttons(ui);
                ctx.set_visuals(egui::style::Visuals::dark());
            });
        });
        const STATE_WIDTH: f32 = 300f32;
        egui::SidePanel::left("processor_state")
            .show_separator_line(true)
            .resizable(false)
            .default_width(STATE_WIDTH)
            .show(ctx, |ui| {
                ui.heading("Processor State:");
                ui.add(
                    egui::TextEdit::multiline(&mut self.device.get_core_state())
                        .font(egui::TextStyle::Monospace),
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("egui GBA Emulator");

            ui.horizontal(|ui| {
                ui.label("Controls");
            });

            if let Ok((path, bytes)) = self.bios_channel.1.try_recv() {
                self.device
                    .load_bios_rom(path, &bytes)
                    .expect("Could not load bios");
            }

            if ui.button("Load BIOS File").clicked() {
                let sender = self.bios_channel.0.clone();
                let task = rfd::AsyncFileDialog::new().pick_file();
                let ctx = ui.ctx().clone();
                execute(async move {
                    let file = task.await;
                    if let Some(file) = file {
                        let path = file.file_name();
                        let bytes = file.read().await;
                        let _ = sender.send((path, bytes));
                        ctx.request_repaint();
                    }
                })
            }

            if let Ok((path, bytes)) = self.rom_channel.1.try_recv() {
                self.device
                    .load_rom(path, &bytes)
                    .expect("Could not load rom");
            }

            if ui.button("Open ROM File").clicked() {
                let sender = self.rom_channel.0.clone();
                let task = rfd::AsyncFileDialog::new().pick_file();
                let ctx = ui.ctx().clone();
                execute(async move {
                    let file = task.await;
                    if let Some(file) = file {
                        let path = file.file_name();
                        let bytes = file.read().await;
                        let _ = sender.send((path, bytes));
                        ctx.request_repaint();
                    }
                })
            }

            ui.separator();
            ui.label(RichText::new("Execution View").color(Color32::GREEN));
            ui.add(
                egui::TextEdit::multiline(&mut self.device.get_execution_state())
                    .font(egui::TextStyle::Monospace)
                    .desired_width(640.0),
            );

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(self.device.get_status());
                ui.separator();
            });
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
