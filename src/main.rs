use eframe::egui;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([380.0, 380.0])
            .with_min_inner_size([320.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Savage Metronome",
        options,
        Box::new(|_cc| Box::new(MetronomeApp::new())),
    )
}

const NOTE_NAMES: [&str; 12] = [
    "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B",
];

struct SharedState {
    bpm: f32,
    volume: f32,
    pitch_hz: f32,
    beep_duration: f32,
    is_running: bool,
    visual_enabled: bool,
    last_beat: Option<Instant>,
}

struct TuningState {
    reference_pitch: f32,
    octave: u8,
    note_index: usize,
}

struct MetronomeApp {
    state: Arc<Mutex<SharedState>>,
    tuning: TuningState,
    tap_times: Vec<Instant>,
    // Keep the output stream alive as long as the app is running
    _output_stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
}

impl MetronomeApp {
    fn new() -> Self {
        let (_output_stream, stream_handle) = OutputStream::try_default()
            .ok()
            .map(|(stream, handle)| (Some(stream), Some(handle)))
            .unwrap_or((None, None));

        let tuning = TuningState {
            reference_pitch: 440.0,
            octave: 6,
            note_index: 0, // C
        };

        // Calculate initial pitch
        let pitch_hz = Self::calculate_pitch(&tuning);

        Self {
            state: Arc::new(Mutex::new(SharedState {
                bpm: 100.0,
                volume: 0.5,
                pitch_hz,
                beep_duration: 0.005,
                is_running: false,
                visual_enabled: true,
                last_beat: None,
            })),
            tuning,
            tap_times: Vec::new(),
            _output_stream,
            stream_handle,
        }
    }

    fn calculate_pitch(tuning: &TuningState) -> f32 {
        // MIDI note number: C0 = 12.
        // n = (octave + 1) * 12 + note_index
        let n = (tuning.octave as i32 + 1) * 12 + tuning.note_index as i32;
        // A4 is MIDI 69.
        // f = f_ref * 2^((n - 69) / 12)
        tuning.reference_pitch * 2.0_f32.powf((n - 69) as f32 / 12.0)
    }

    fn start_metronome(&mut self) {
        let mut state = self.state.lock().unwrap();
        if state.is_running {
            return;
        }
        state.is_running = true;
        drop(state); // Drop lock before spawning thread

        let state_ref = self.state.clone();
        // Clone the handle if it exists. If audio failed to init, the thread will just exit or do nothing.
        let stream_handle = match &self.stream_handle {
            Some(handle) => handle.clone(),
            None => return, // No audio device
        };

        thread::spawn(move || {
            let mut next_tick = Instant::now();
            
            loop {
                let (bpm, volume, pitch, beep_duration, is_running) = {
                    let state = state_ref.lock().unwrap();
                    (state.bpm, state.volume, state.pitch_hz, state.beep_duration, state.is_running)
                };

                if !is_running {
                    break;
                }

                if Instant::now() >= next_tick {
                    // Play sound
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    
                    // Anti-pop: Duration approx beep_duration, but exact multiple of period
                    // Limit duration to slightly less than interval to prevent overlap
                    let interval = 60.0 / bpm;
                    let max_dur = interval * 0.95; // Leave a tiny gap just in case
                    let target = beep_duration.min(max_dur);
                    
                    let cycles = (target * pitch).round().max(1.0);
                    let duration = Duration::from_secs_f32(cycles / pitch);

                    let source = rodio::source::SineWave::new(pitch)
                        .take_duration(duration)
                        .amplify(volume);
                    sink.append(source);
                    sink.detach();

                    // Update last_beat timestamp for visualization
                    if let Ok(mut state) = state_ref.lock() {
                        state.last_beat = Some(Instant::now());
                    }

                    next_tick += Duration::from_secs_f32(interval);
                    
                    // If we fell behind too much (e.g. system sleep), reset to now
                    if next_tick < Instant::now() {
                        next_tick = Instant::now() + Duration::from_secs_f32(interval);
                    }
                }

                let now = Instant::now();
                if next_tick > now {
                    let sleep_duration = next_tick - now;
                    let max_sleep = Duration::from_millis(20);
                    thread::sleep(sleep_duration.min(max_sleep));
                }
            }
        });
    }

    fn calculate_tap_tempo(&mut self) {
        if self.tap_times.len() < 2 {
            return;
        }

        let mut intervals = Vec::new();
        for w in self.tap_times.windows(2) {
            intervals.push(w[1].duration_since(w[0]).as_secs_f32());
        }

        let avg = intervals.iter().sum::<f32>() / intervals.len() as f32;
        let new_bpm = 60.0 / avg;
        
        self.state.lock().unwrap().bpm = new_bpm;
    }
}

impl eframe::App for MetronomeApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Request repaint if running to animate visualization
        if self.state.lock().unwrap().is_running {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Visualization Section
            ui.heading("Visualization");
            
            let (visual_enabled, is_running, last_beat) = {
                let state = self.state.lock().unwrap();
                (state.visual_enabled, state.is_running, state.last_beat)
            };

            ui.horizontal(|ui| {
                ui.label("Visual Mode:");
                let mut visual_enabled_mut = visual_enabled;
                if ui.radio_value(&mut visual_enabled_mut, true, "On").clicked() {
                    self.state.lock().unwrap().visual_enabled = true;
                }
                if ui.radio_value(&mut visual_enabled_mut, false, "Off").clicked() {
                    self.state.lock().unwrap().visual_enabled = false;
                }
                 // Visualization Box
                let size = egui::Vec2::new(20.0, 20.0);
                let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                
                // Draw black border
                ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::BLACK));

                // Draw fill if active
                if visual_enabled && is_running {
                    if let Some(last) = last_beat {
                         let elapsed = last.elapsed();
                         if elapsed.as_millis() < 100 {
                             ui.painter().rect_filled(rect, 0.0, egui::Color32::BLACK);
                         }
                    }
                }
            });

            ui.separator();

            ui.heading("Tempo");
            
            let mut state = self.state.lock().unwrap();

            ui.horizontal(|ui| {
                if ui.button("-10").clicked() {
                    state.bpm -= 10.0;
                }
                if ui.button("-1").clicked() {
                    state.bpm -= 1.0;
                }

                ui.add(
                    egui::DragValue::new(&mut state.bpm)
                        .speed(1.0)
                        .clamp_range(20.0..=300.0)
                        .prefix("BPM: "),
                );

                if ui.button("+1").clicked() {
                    state.bpm += 1.0;
                }
                if ui.button("+10").clicked() {
                    state.bpm += 10.0;
                }
            });

            if ui.button("Tap Tempo").clicked() {
                self.tap_times.push(Instant::now());
                if self.tap_times.len() > 8 {
                    self.tap_times.remove(0);
                }
                drop(state);
                self.calculate_tap_tempo();
                state = self.state.lock().unwrap();
            }

            ui.separator();
            ui.heading("Tuning");

            let mut pitch_changed = false;

            // Reference Pitch
            ui.horizontal(|ui| {
                ui.label("Ref. Pitch:");
                if ui.button("-1.0").clicked() {
                    self.tuning.reference_pitch -= 1.0;
                    pitch_changed = true;
                }
                if ui.button("-0.1").clicked() {
                    self.tuning.reference_pitch -= 0.1;
                    pitch_changed = true;
                }
                
                ui.label(format!("{:.1} Hz", self.tuning.reference_pitch));

                if ui.button("+0.1").clicked() {
                    self.tuning.reference_pitch += 0.1;
                    pitch_changed = true;
                }
                if ui.button("+1.0").clicked() {
                    self.tuning.reference_pitch += 1.0;
                    pitch_changed = true;
                }
            });

            // Octave
            ui.horizontal(|ui| {
                ui.label("Octave:");
                egui::ComboBox::from_id_source("octave_combo")
                    .selected_text(format!("{}", self.tuning.octave))
                    .show_ui(ui, |ui| {
                        for o in 0..=8 {
                             if ui.selectable_value(&mut self.tuning.octave, o, format!("{}", o)).clicked() {
                                 pitch_changed = true;
                             }
                        }
                    });
            });

            // Note
            ui.horizontal(|ui| {
                ui.label("Note:");
                egui::ComboBox::from_id_source("note_combo")
                    .selected_text(NOTE_NAMES[self.tuning.note_index])
                    .show_ui(ui, |ui| {
                        for (i, name) in NOTE_NAMES.iter().enumerate() {
                             if ui.selectable_value(&mut self.tuning.note_index, i, *name).clicked() {
                                 pitch_changed = true;
                             }
                        }
                    });
            });

            if pitch_changed {
                state.pitch_hz = Self::calculate_pitch(&self.tuning);
            }
            
            ui.label(format!("Current freq: {:.2} Hz", state.pitch_hz));

            ui.separator();
            
            ui.add(
                egui::Slider::new(&mut state.volume, 0.0..=1.0)
                    .text("Volume"),
            );

            let max_duration = 60.0 / state.bpm;
            ui.add(
                egui::Slider::new(&mut state.beep_duration, 0.005..=max_duration)
                    .text("Beep Duration")
                    .suffix(" s")
                    .max_decimals(3),
            );

            ui.separator();

            if ui.button("Start").clicked() {
                 drop(state);
                 self.start_metronome();
                 state = self.state.lock().unwrap();
            }

            if ui.button("Stop").clicked() {
                state.is_running = false;
            }
        });
    }
}
