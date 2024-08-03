#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod audio;
mod mixer;
mod waves;

use crate::mixer::AudioMixer;
use crate::waves::{SineWave, SquareWave, TriangleWave};
use eframe::egui;
use std::sync::{Arc, Mutex};
use tinyaudio::BaseAudioOutputDevice;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    audio_device: Option<Box<dyn BaseAudioOutputDevice>>,
    mixer: Arc<Mutex<AudioMixer>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            audio_device: None,
            mixer: Arc::new(Mutex::new(AudioMixer::new())),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.audio_device.is_none() {
            let mixer_handle = Arc::clone(&self.mixer);
            self.audio_device = audio::init_audio(&mixer_handle);
            {
                let mut mixer = self.mixer.lock().unwrap();
                let _ = &mixer.add_channel(Box::new(SquareWave {
                    frequency: 440.0,
                    amplitude: 0.1,
                    phase: 0.0,
                    sample_rate: 44100.0,
                }));
                let _ = &mixer.add_channel(Box::new(SineWave {
                    frequency: 220.0,
                    amplitude: 0.2,
                    phase: 0.0,
                    sample_rate: 44100.0,
                }));
                // &mixer.add_channel(Box::new(SineWave {
                //     frequency: 442.0,
                //     amplitude: 0.2,
                //     phase: 0.0,
                //     sample_rate: 44100.0,
                // }));
                let _ = mixer.add_channel(Box::new(TriangleWave {
                    frequency: 880.0,
                    amplitude: 0.3,
                    phase: 0.0,
                    sample_rate: 44100.0,
                }));
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, Audio!");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            ui.label("todo".to_string());

            // ui.image(egui::include_image!(
            //     "assets/logo.png"
            // ));
        });
    }
}
