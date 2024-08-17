#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod audio;
mod mixer;
mod waves;

use crate::mixer::AudioMixer;
use crate::waves::{SineWave, SquareWave, TriangleWave};
use eframe::egui;
use egui::{Button, Sense};
use indexmap::IndexMap;
use std::sync::{Arc, Mutex};
use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use tinyaudio::BaseAudioOutputDevice;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([224.0, 328.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Phone Keypad",
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
    dtmf_ordered_map: IndexMap<char, [f32; 2]>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            audio_device: None,
            mixer: Arc::new(Mutex::new(AudioMixer::new())),
            dtmf_ordered_map: create_ordered_dtmf_map(),
        }
    }
}

fn create_ordered_dtmf_map() -> IndexMap<char, [f32; 2]> {
    let mut dtmf_map = IndexMap::new();

    dtmf_map.insert('0', DTMF_TONE[0]);
    dtmf_map.insert('1', DTMF_TONE[1]);
    dtmf_map.insert('2', DTMF_TONE[2]);
    dtmf_map.insert('3', DTMF_TONE[3]);
    dtmf_map.insert('4', DTMF_TONE[4]);
    dtmf_map.insert('5', DTMF_TONE[5]);
    dtmf_map.insert('6', DTMF_TONE[6]);
    dtmf_map.insert('7', DTMF_TONE[7]);
    dtmf_map.insert('8', DTMF_TONE[8]);
    dtmf_map.insert('9', DTMF_TONE[9]);
    dtmf_map.insert('*', DTMF_TONE[10]);
    dtmf_map.insert('#', DTMF_TONE[11]);

    dtmf_map
}

pub const DTMF_TONE: [[f32; 2]; 12] = [
    [941.0, 1336.0], // 0
    [697.0, 1209.0], // 1
    [697.0, 1336.0], // 2
    [697.0, 1477.0], // 3
    [770.0, 1209.0], // 4
    [770.0, 1336.0], // 5
    [770.0, 1477.0], // 6
    [852.0, 1209.0], // 7
    [852.0, 1336.0], // 8
    [852.0, 1477.0], // 9
    [941.0, 1209.0], // *
    [941.0, 1477.0], // #
];

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.audio_device.is_none() {
            let mixer_handle = Arc::clone(&self.mixer);
            self.audio_device = audio::init_audio(&mixer_handle);
            // {
            //     let mut mixer = self.mixer.lock().unwrap();
            // }
        }


        // Start drawing frame elements

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
            );

            ui.heading("Phone Keypad");
            ui.separator();
            {
                // let mut mixer = self.mixer.lock().unwrap();
                // let channels = &mut mixer.channels;
                // let mut channel_to_remove: Option<usize> = None;
                // for (i, c) in channels.iter().enumerate() {
                //     ui.horizontal(|ui| {
                //         if ui.button("X").clicked() {
                //             channel_to_remove = Some(i);
                //         }
                //         ui.label(format!("Channel {} - {:?}", i, c));
                //     });
                // }
                // if let Some(chan_id) = channel_to_remove {
                //     &channels.remove(chan_id);
                // }

                for i in 1..=3 {
                    ui.horizontal(|ui| {
                        for j in 1..=3 {
                            let number = ((i - 1) * 3) + j;
                            let key = format!("{}", number);
                            let button = Button::new(&key).sense(Sense::drag());
                            // let response = &ui.add(button);
                            let response = &ui.add_sized([64.0, 64.0], button);

                            let mut mixer = self.mixer.lock().unwrap();
                            let channels = &mut mixer.channels;
                            if response.drag_started() {
                                // let tones = DTMF_TONE[number];
                                let c = key.chars().next().expect("Empty key string!");
                                let tones = self
                                    .dtmf_ordered_map
                                    .get(&c)
                                    .expect("ordered map doesn't have key!");

                                let _ = &mixer.add_channel(Box::new(SineWave {
                                    frequency: tones[0],
                                    amplitude: 0.1,
                                    phase: 0.0,
                                    sample_rate: 44100.0,
                                }));
                                let _ = &mixer.add_channel(Box::new(SineWave {
                                    frequency: tones[1],
                                    amplitude: 0.1,
                                    phase: 0.0,
                                    sample_rate: 44100.0,
                                }));
                            } else if response.drag_stopped() {
                                &channels.clear();
                            }
                        }
                    });
                }

                ui.horizontal(|ui| {
                    for j in ['*', '0', '#'] {
                        let key = format!("{}", j);
                        let button = Button::new(&key).sense(Sense::drag());
                        let response = &ui.add_sized([64.0, 64.0], button);

                        let mut mixer = self.mixer.lock().unwrap();
                        let channels = &mut mixer.channels;

                        if response.drag_started() {
                            // let tones = DTMF_TONE[number];
                            let c = key.chars().next().expect("Empty key string!");
                            let tones = self
                                .dtmf_ordered_map
                                .get(&c)
                                .expect("ordered map doesn't have key!");

                            let _ = &mixer.add_channel(Box::new(SineWave {
                                frequency: tones[0],
                                amplitude: 0.1,
                                phase: 0.0,
                                sample_rate: 44100.0,
                            }));
                            let _ = &mixer.add_channel(Box::new(SineWave {
                                frequency: tones[1],
                                amplitude: 0.1,
                                phase: 0.0,
                                sample_rate: 44100.0,
                            }));
                        } else if response.drag_stopped() {
                            &channels.clear();
                        }
                    }
                });
            }

            ui.ctx().request_repaint();
        });
    }
}
