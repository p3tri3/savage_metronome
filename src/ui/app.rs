// The main application struct that holds the UI state and domain model.
use crate::audio::engine::start_metronome_thread;
use crate::domain::metronome::Metronome;
use crate::domain::tempo::calculate_tap_tempo;
use crate::presets::preset::{MetronomePreset, NOTE_NAMES, calculate_pitch};
use crate::presets::storage::{load_preset, save_preset};
use eframe::egui;

#[cfg(not(feature = "audio"))]
use crate::audio::mock as audio_crate;
#[cfg(feature = "audio")]
use rodio as audio_crate;

use audio_crate::{OutputStream, OutputStreamBuilder, mixer::Mixer};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct MetronomeApp {
    state: Arc<Mutex<Metronome>>,
    tap_times: Vec<Instant>,
    _output_stream: Option<OutputStream>,
    mixer: Option<Mixer>,
    thread_stop: Arc<AtomicBool>,
}

impl Default for MetronomeApp {
    fn default() -> Self {
        Self::new()
    }
}

impl MetronomeApp {
    pub fn new() -> Self {
        let (_output_stream, mixer) = match OutputStreamBuilder::open_default_stream() {
            Ok(stream) => {
                let mixer = stream.mixer().clone();
                (Some(stream), Some(mixer))
            }
            Err(_) => (None, None),
        };

        let preset = load_preset().unwrap_or_default();
        let metronome_state = Metronome::from(preset);

        Self {
            state: Arc::new(Mutex::new(metronome_state)),
            tap_times: Vec::new(),
            _output_stream,
            mixer,
            thread_stop: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl eframe::App for MetronomeApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();

        if state.is_running {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Savage Metronome");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Start").clicked() && !state.is_running {
                    state.is_running = true;
                    if let Some(mixer) = self.mixer.clone() {
                        let stop = Arc::new(AtomicBool::new(false));
                        self.thread_stop = stop.clone();
                        start_metronome_thread(self.state.clone(), mixer, stop);
                    }
                }
                if ui.button("Stop").clicked() {
                    self.thread_stop.store(true, Ordering::Relaxed);
                    state.is_running = false;
                }
            });

            ui.separator();

            ui.heading("Tempo");
            ui.horizontal(|ui| {
                if ui.button("-10").clicked() {
                    state.bpm = (state.bpm - 10.0).max(20.0);
                }
                if ui.button("-1").clicked() {
                    state.bpm = (state.bpm - 1.0).max(20.0);
                }
                ui.add(
                    egui::DragValue::new(&mut state.bpm)
                        .speed(1.0)
                        .range(20.0..=300.0)
                        .prefix("BPM: "),
                );
                if ui.button("+1").clicked() {
                    state.bpm = (state.bpm + 1.0).min(300.0);
                }
                if ui.button("+10").clicked() {
                    state.bpm = (state.bpm + 10.0).min(300.0);
                }
            });
            if ui.button("Tap Tempo").clicked() {
                self.tap_times.push(Instant::now());
                if self.tap_times.len() > 8 {
                    self.tap_times.remove(0);
                }
                if let Some(new_bpm) = calculate_tap_tempo(&self.tap_times) {
                    state.bpm = new_bpm.clamp(20.0, 300.0);
                }
            }
            ui.separator();

            ui.heading("Visualization");
            ui.horizontal(|ui| {
                ui.radio_value(&mut state.visual_enabled, true, "On");
                ui.radio_value(&mut state.visual_enabled, false, "Off");
                let size = egui::Vec2::new(20.0, 20.0);
                let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                ui.painter().rect_stroke(
                    rect,
                    0.0,
                    egui::Stroke::new(1.0, egui::Color32::BLACK),
                    egui::StrokeKind::Outside,
                );
                if state.visual_enabled
                    && state.is_running
                    && let Some(last) = state.last_beat
                    && last.elapsed().as_millis() < 100
                {
                    ui.painter().rect_filled(rect, 0.0, egui::Color32::BLACK);
                }
            });
            ui.separator();

            ui.heading("Tuning");
            let mut pitch_changed = false;

            // Reference Pitch
            ui.horizontal(|ui| {
                ui.label("Ref. Pitch:");
                if ui.button("-1.0").clicked() {
                    state.tuning.reference_pitch -= 1.0;
                    pitch_changed = true;
                }
                if ui.button("-0.1").clicked() {
                    state.tuning.reference_pitch -= 0.1;
                    pitch_changed = true;
                }

                ui.label(format!("{:.1} Hz", state.tuning.reference_pitch));

                if ui.button("+0.1").clicked() {
                    state.tuning.reference_pitch += 0.1;
                    pitch_changed = true;
                }
                if ui.button("+1.0").clicked() {
                    state.tuning.reference_pitch += 1.0;
                    pitch_changed = true;
                }
            });

            // Octave
            ui.horizontal(|ui| {
                ui.label("Octave:");
                egui::ComboBox::from_id_salt("octave_combo")
                    .selected_text(format!("{}", state.tuning.octave))
                    .show_ui(ui, |ui| {
                        for o in 0..=8 {
                            if ui
                                .selectable_value(&mut state.tuning.octave, o, format!("{}", o))
                                .clicked()
                            {
                                pitch_changed = true;
                            }
                        }
                    });
            });

            // Note
            ui.horizontal(|ui| {
                ui.label("Note:");
                egui::ComboBox::from_id_salt("note_combo")
                    .selected_text(NOTE_NAMES[state.tuning.note_index])
                    .show_ui(ui, |ui| {
                        for (i, name) in NOTE_NAMES.iter().enumerate() {
                            if ui
                                .selectable_value(&mut state.tuning.note_index, i, *name)
                                .clicked()
                            {
                                pitch_changed = true;
                            }
                        }
                    });
            });

            if pitch_changed {
                state.pitch_hz = calculate_pitch(&state.tuning);
            }

            ui.label(format!("Current freq: {:.2} Hz", state.pitch_hz));

            ui.separator();

            ui.add(egui::Slider::new(&mut state.volume, 0.0..=1.0).text("Volume"));

            let max_duration = 60.0 / state.bpm;
            ui.add(
                egui::Slider::new(&mut state.beep_duration, 0.005..=max_duration)
                    .text("Beep Duration")
                    .suffix(" s")
                    .max_decimals(3),
            );

            ui.separator();

            ui.heading("Presets");
            ui.horizontal(|ui| {
                if ui.button("Save").clicked()
                    && let Err(e) = save_preset(&state.to_preset())
                {
                    eprintln!("Failed to save preset: {}", e);
                }
                if ui.button("Load").clicked() {
                    if let Ok(preset) = load_preset() {
                        *state = preset.into();
                    } else {
                        eprintln!("Failed to load preset.");
                    }
                }
                if ui.button("Reset").clicked() {
                    *state = MetronomePreset::default().into();
                }
            });
        });
    }
}
