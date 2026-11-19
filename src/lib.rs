use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicI32, AtomicU16, Ordering};
use std::sync::{Arc, Mutex, RwLock};

pub mod demos;
mod editor;
pub mod kit;
pub mod library;
pub mod seq;

pub const CHANNELS: usize = 10;
pub const STEPS: usize = 16;
pub const PATTERNS: usize = 4;
/// Gain for the step velocities off, soft, medium, hard.
const VELOCITY: [f32; 4] = [0.0, 0.45, 0.7, 1.0];

#[derive(Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Step {
    /// 0 off, 1-3 soft to hard.
    pub vel: u8,
    /// Hits within the step, 1-4.
    pub roll: u8,
    /// Semitones added to the channel's pitch.
    pub pitch: i8,
}

impl Step {
    pub fn hits(&self) -> i64 {
        self.roll.clamp(1, 4) as i64
    }
}

pub fn pattern_index(pattern: usize, ch: usize, step: usize) -> usize {
    (pattern * CHANNELS + ch) * STEPS + step
}

/// State the editor and the audio thread both touch.
pub struct Shared {
    /// User WAVs that replace a channel's library sample.
    pub user: RwLock<Vec<Option<Arc<kit::Sample>>>>,
    /// The step under the playhead, or -1 when stopped.
    pub step: AtomicI32,
    /// Channels the editor wants played now, one bit each.
    pub audition: AtomicU16,
    /// Each channel's current voice level, for the activity lights.
    pub levels: [AtomicF32; CHANNELS],
    /// A state loaded from a file, waiting for the editor to apply it.
    pub pending_state: Mutex<Option<PluginState>>,
}

#[derive(Params)]
pub struct TenpinParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
    #[persist = "patterns"]
    pub patterns: RwLock<Vec<Step>>,
    /// A WAV replacing each channel's library sample; empty means use the library.
    #[persist = "samples"]
    pub paths: RwLock<Vec<String>>,

    #[id = "master"]
    pub master: FloatParam,
    #[id = "run"]
    pub run: BoolParam,
    #[id = "bpm"]
    pub bpm: FloatParam,
    #[id = "shuffle"]
    pub shuffle: FloatParam,
    #[id = "steps"]
    pub steps: IntParam,
    #[id = "pattern"]
    pub pattern: IntParam,
    #[nested(array, group = "Channel")]
    pub ch: [ChannelParams; CHANNELS],
}

#[derive(Params)]
pub struct ChannelParams {
    #[id = "sample"]
    pub sample: IntParam,
    #[id = "level"]
    pub level: FloatParam,
    #[id = "pan"]
    pub pan: FloatParam,
    #[id = "pitch"]
    pub pitch: FloatParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "start"]
    pub start: FloatParam,
    #[id = "tone"]
    pub tone: FloatParam,
    #[id = "mute"]
    pub mute: BoolParam,
    #[id = "solo"]
    pub solo: BoolParam,
    #[id = "choke"]
    pub choke: BoolParam,
}

fn gain_param(name: String, default_db: f32) -> FloatParam {
    FloatParam::new(
        name,
        util::db_to_gain(default_db),
        FloatRange::Skewed {
            min: util::db_to_gain(-36.0),
            max: util::db_to_gain(6.0),
            factor: FloatRange::gain_skew_factor(-36.0, 6.0),
        },
    )
    .with_unit(" dB")
    .with_value_to_string(formatters::v2s_f32_gain_to_db(1))
    .with_string_to_value(formatters::s2v_f32_gain_to_db())
}

fn pct_param(name: String, default: f32) -> FloatParam {
    FloatParam::new(name, default, FloatRange::Linear { min: 0.0, max: 1.0 })
        .with_unit("%")
        .with_value_to_string(formatters::v2s_f32_percentage(0))
        .with_string_to_value(formatters::s2v_f32_percentage())
}

impl ChannelParams {
    fn new(ch: usize, kit: &demos::Kit) -> Self {
        let n = format!("Ch{}", ch + 1);
        let last = library::len() as i32 - 1;
        Self {
            sample: IntParam::new(
                format!("{n} Sample"),
                library::find(kit.sample).unwrap_or(ch) as i32,
                IntRange::Linear { min: 0, max: last },
            )
            .with_value_to_string(Arc::new(|i| library::name(i.max(0) as usize).to_string()))
            .with_string_to_value(Arc::new(|s| library::find(s.trim()).map(|i| i as i32))),
            level: gain_param(format!("{n} Level"), kit.level),
            pan: FloatParam::new(
                format!("{n} Pan"),
                kit.pan,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_value_to_string(formatters::v2s_f32_panning())
            .with_string_to_value(formatters::s2v_f32_panning()),
            pitch: FloatParam::new(
                format!("{n} Pitch"),
                kit.pitch,
                FloatRange::Linear {
                    min: -24.0,
                    max: 24.0,
                },
            )
            .with_step_size(0.1)
            .with_unit(" st"),
            decay: FloatParam::new(
                format!("{n} Decay"),
                kit.decay,
                FloatRange::Skewed {
                    min: 0.02,
                    max: 8.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" s")
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            start: pct_param(format!("{n} Start"), 0.0),
            tone: pct_param(format!("{n} Tone"), kit.tone),
            mute: BoolParam::new(format!("{n} Mute"), false),
            solo: BoolParam::new(format!("{n} Solo"), false),
            choke: BoolParam::new(format!("{n} Choke"), kit.choke),
        }
    }
}

impl Default for TenpinParams {
    fn default() -> Self {
        let demo = &demos::ALL[0];
        Self {
            editor_state: EguiState::from_size(editor::WIDTH, editor::HEIGHT),
            patterns: RwLock::new(demo.steps()),
            paths: RwLock::new(vec![String::new(); CHANNELS]),
            master: gain_param("Master".into(), -6.0),
            run: BoolParam::new("Run", false),
            bpm: FloatParam::new(
                "BPM",
                demo.bpm,
                FloatRange::Linear {
                    min: 40.0,
                    max: 240.0,
                },
            )
            .with_step_size(0.5)
            .with_value_to_string(formatters::v2s_f32_rounded(1)),
            shuffle: pct_param("Shuffle".into(), demo.shuffle),
            steps: IntParam::new(
                "Steps",
                STEPS as i32,
                IntRange::Linear {
                    min: 1,
                    max: STEPS as i32,
                },
            ),
            pattern: IntParam::new(
                "Pattern",
                0,
                IntRange::Linear {
                    min: 0,
                    max: PATTERNS as i32 - 1,
                },
            )
            .with_value_to_string(Arc::new(|v| ((b'A' + v as u8) as char).to_string()))
            .with_string_to_value(Arc::new(|s| {
                s.trim()
                    .bytes()
                    .next()
                    .map(|c| c.to_ascii_uppercase() as i32 - b'A' as i32)
            })),
            ch: std::array::from_fn(|c| ChannelParams::new(c, &demo.kit[c])),
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Voice {
    active: bool,
    pos: f64,
    rate: f64,
    amp: f32,
    /// Per-sample amplitude multiplier for the decay.
    fall: f32,
    /// One-pole low-pass state for the tone control.
    lp: f32,
}

pub struct Tenpin {
    params: Arc<TenpinParams>,
    shared: Arc<Shared>,
    voices: [Voice; CHANNELS],
    sample_rate: f32,
    /// Last sequencer position in sixteenths, None while stopped.
    prev: Option<f64>,
    /// Position of the Run button's own clock, None while it's off.
    run_pos: Option<f64>,
}

impl Default for Tenpin {
    fn default() -> Self {
        Self {
            params: Arc::new(TenpinParams::default()),
            shared: Arc::new(Shared {
                user: RwLock::new(vec![None; CHANNELS]),
                step: AtomicI32::new(-1),
                audition: AtomicU16::new(0),
                levels: std::array::from_fn(|_| AtomicF32::new(0.0)),
                pending_state: Mutex::new(None),
            }),
            voices: [Voice::default(); CHANNELS],
            sample_rate: 48000.0,
            prev: None,
            run_pos: None,
        }
    }
}

impl Tenpin {
    fn trigger(&mut self, ch: usize, velocity: f32, semitones: f32, sample: &kit::Sample) {
        let p = &self.params.ch[ch];
        if p.choke.value() {
            for (j, v) in self.voices.iter_mut().enumerate() {
                if j != ch && self.params.ch[j].choke.value() {
                    // A 5 ms fade rather than a hard cut, which would click.
                    v.fall = v.fall.min(0.001f32.powf(1.0 / (0.005 * self.sample_rate)));
                }
            }
        }
        let semis = (p.pitch.value() + semitones) as f64;
        self.voices[ch] = Voice {
            active: true,
            pos: p.start.value() as f64 * sample.data.len() as f64,
            rate: (sample.rate / self.sample_rate) as f64 * 2f64.powf(semis / 12.0),
            amp: velocity,
            fall: 0.001f32.powf(1.0 / (p.decay.value() * self.sample_rate)),
            lp: 0.0,
        };
    }
}

/// Reload each channel's user WAV from its saved path. Not for the audio thread.
fn load_samples(params: &TenpinParams, shared: &Shared) {
    let loaded: Vec<_> = params
        .paths
        .read()
        .unwrap()
        .iter()
        .map(|p| {
            (!p.is_empty())
                .then(|| {
                    kit::load_wav(p.as_ref())
                        .inspect_err(|e| nih_error!("could not load {p}: {e}"))
                        .ok()
                })
                .flatten()
                .map(Arc::new)
        })
        .collect();
    *shared.user.write().unwrap() = loaded;
}

impl Plugin for Tenpin {
    const NAME: &'static str = "Tenpin";
    const VENDOR: &'static str = "a-adz-1";
    const URL: &'static str = "";
    const EMAIL: &'static str = "a-adz-1@proton.me";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.shared.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        library::all();
        // Runs again after the host restores state, so saved WAV paths get loaded here.
        load_samples(&self.params, &self.shared);
        true
    }

    fn reset(&mut self) {
        self.voices = [Voice::default(); CHANNELS];
        self.prev = None;
        self.run_pos = None;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let shared = self.shared.clone();
        let params = self.params.clone();
        // If the editor is swapping a sample or editing the pattern this block, play silence
        // rather than wait on the lock.
        let (Ok(user), Ok(patterns)) = (shared.user.try_read(), params.patterns.try_read()) else {
            buffer.as_slice().iter_mut().for_each(|c| c.fill(0.0));
            return ProcessStatus::KeepAlive;
        };
        let lib = library::all();
        let sample_of = |c: usize| lib_or_user(&user, lib, &params, c);

        // The host's transport drives the sequencer while it plays; otherwise the Run button
        // does, at the host's tempo if there is one. The standalone app's transport always
        // claims to be playing, so it only ever uses Run.
        let t = context.transport();
        let standalone = context.plugin_api() == PluginApi::Standalone;
        let sr = self.sample_rate as f64;
        let host_tempo = t.tempo.filter(|_| !standalone);
        let clock = match (t.playing && !standalone, t.pos_beats(), host_tempo) {
            (true, Some(pos), Some(tempo)) => {
                self.run_pos = None;
                Some((pos * 4.0, tempo * 4.0 / 60.0 / sr))
            }
            _ if params.run.value() => {
                let per = host_tempo.unwrap_or(params.bpm.value() as f64) * 4.0 / 60.0 / sr;
                let start = *self.run_pos.get_or_insert(0.0);
                self.run_pos = Some(start + buffer.samples() as f64 * per);
                Some((start, per))
            }
            _ => {
                self.run_pos = None;
                None
            }
        };
        if clock.is_none() {
            self.prev = None;
            shared.step.store(-1, Ordering::Relaxed);
        }

        let shuffle = params.shuffle.value() as f64;
        let steps = params.steps.value() as i64;
        let pattern = params.pattern.value() as usize;
        let master = params.master.value();
        let any_solo = params.ch.iter().any(|p| p.solo.value());
        let mut gain_l = [0.0f32; CHANNELS];
        let mut gain_r = [0.0f32; CHANNELS];
        let mut tone = [0.0f32; CHANNELS];
        for (c, p) in params.ch.iter().enumerate() {
            let silent = p.mute.value() || (any_solo && !p.solo.value());
            let level = if silent {
                0.0
            } else {
                p.level.value() * master
            };
            let angle = (p.pan.value() + 1.0) * std::f32::consts::FRAC_PI_4;
            gain_l[c] = level * angle.cos();
            gain_r[c] = level * angle.sin();
            let cutoff = 200.0 * 100f32.powf(p.tone.value());
            tone[c] = 1.0 - (-std::f32::consts::TAU * cutoff / self.sample_rate).exp();
        }

        let audition = shared.audition.swap(0, Ordering::Relaxed);
        for c in (0..CHANNELS).filter(|c| audition & (1 << c) != 0) {
            self.trigger(c, 1.0, 0.0, sample_of(c));
        }

        let mut next_event = context.next_event();
        for (i, mut frame) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > i as u32 {
                    break;
                }
                // MIDI notes 36-45 (C1 up) play the ten channels.
                if let NoteEvent::NoteOn { note, velocity, .. } = event {
                    let ch = note as usize;
                    if (36..36 + CHANNELS).contains(&ch) {
                        self.trigger(ch - 36, velocity, 0.0, sample_of(ch - 36));
                    }
                }
                next_event = context.next_event();
            }

            if let Some((start, per_sample)) = clock {
                let now = start + i as f64 * per_sample;
                // Looping or seeking makes the position jump: restart from here.
                let prev = match self.prev {
                    Some(p) if p <= now && now - p < 1.0 => p,
                    _ => now - 1e-9,
                };
                if let Some((k, tick)) = seq::fired(prev, now, shuffle) {
                    let step = k.rem_euclid(steps) as usize;
                    if tick == 0 {
                        shared.step.store(step as i32, Ordering::Relaxed);
                    }
                    for c in 0..CHANNELS {
                        let s = patterns[pattern_index(pattern, c, step)];
                        if s.vel > 0 && tick % (seq::TICKS / s.hits()) == 0 {
                            self.trigger(
                                c,
                                VELOCITY[s.vel.min(3) as usize],
                                s.pitch as f32,
                                sample_of(c),
                            );
                        }
                    }
                }
                self.prev = Some(now);
            }

            let (mut l, mut r) = (0.0, 0.0);
            for (c, v) in self.voices.iter_mut().enumerate() {
                if !v.active {
                    continue;
                }
                let d = &sample_of(c).data;
                let at = v.pos as usize;
                if at + 1 >= d.len() || v.amp < 1e-4 {
                    v.active = false;
                    continue;
                }
                let x = d[at] + (d[at + 1] - d[at]) * (v.pos - at as f64) as f32;
                v.lp += tone[c] * (x - v.lp);
                let y = v.lp * v.amp;
                l += y * gain_l[c];
                r += y * gain_r[c];
                v.pos += v.rate;
                v.amp *= v.fall;
            }
            let mut out = frame.iter_mut();
            *out.next().unwrap() = l;
            *out.next().unwrap() = r;
        }
        for (c, v) in self.voices.iter().enumerate() {
            shared.levels[c].store(
                if v.active {
                    v.amp * gain_l[c].max(gain_r[c])
                } else {
                    0.0
                },
                Ordering::Relaxed,
            );
        }
        ProcessStatus::KeepAlive
    }
}

/// The sample a channel plays: its user WAV if it has one, else its library pick.
fn lib_or_user<'a>(
    user: &'a [Option<Arc<kit::Sample>>],
    lib: &'a [Arc<kit::Sample>],
    params: &TenpinParams,
    c: usize,
) -> &'a kit::Sample {
    user[c]
        .as_deref()
        .unwrap_or_else(|| &lib[(params.ch[c].sample.value().max(0) as usize).min(lib.len() - 1)])
}

impl ClapPlugin for Tenpin {
    const CLAP_ID: &'static str = "io.github.a-adz-1.tenpin";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Ten-channel sample drum machine with a step sequencer");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::DrumMachine,
        ClapFeature::Sampler,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for Tenpin {
    const VST3_CLASS_ID: [u8; 16] = *b"TenpinDrumsAdz01";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Drum,
        Vst3SubCategory::Sampler,
    ];
}

nih_export_clap!(Tenpin);
nih_export_vst3!(Tenpin);
