//! Samples: the built-in synthesized kit and WAV loading.
use std::f32::consts::TAU;
use std::path::Path;

pub struct Sample {
    pub name: String,
    pub data: Vec<f32>,
    pub rate: f32,
}

const SR: f32 = 48000.0;
/// Library names of the built-in sounds, in channel order.
pub const NAMES: [&str; 10] = [
    "kick_tenpin",
    "snare_tenpin",
    "clap_tenpin",
    "tom_tenpin_lo",
    "tom_tenpin_hi",
    "rim_tenpin",
    "perc_tenpin_cowbell",
    "hat_closed_tenpin",
    "hat_open_tenpin",
    "crash_tenpin",
];

/// The built-in sound for channel `ch`, so the plugin plays something before any WAV is loaded.
pub fn builtin(ch: usize) -> Sample {
    let mut seed = 0x9e37_79b9u32 ^ ch as u32;
    let mut noise = move || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed as f32 / u32::MAX as f32 * 2.0 - 1.0
    };
    // Six detuned squares make the metallic body of hats and cymbals.
    let metal = |t: f32| {
        [205.3, 304.4, 369.6, 522.7, 540.0, 800.0]
            .iter()
            .map(|f: &f32| (f * 1.7 * t).fract().round() * 2.0 - 1.0)
            .sum::<f32>()
            / 6.0
    };
    let secs = [0.5, 0.3, 0.35, 0.6, 0.5, 0.08, 0.4, 0.15, 0.8, 2.0][ch];
    let (mut phase, mut last) = (0.0f32, 0.0f32);
    let mut data: Vec<f32> = (0..(secs * SR) as usize)
        .map(|i| {
            let t = i as f32 / SR;
            let mut sweep = |f: f32| {
                phase += f / SR;
                (TAU * phase).sin()
            };
            let x = match ch {
                0 => sweep(45.0 + 120.0 * (-t * 30.0).exp()) * (-t * 7.0).exp(),
                1 => sweep(185.0) * (-t * 20.0).exp() * 0.5 + noise() * (-t * 14.0).exp() * 0.6,
                2 => {
                    let bursts: f32 = [0.0, 0.01, 0.02]
                        .iter()
                        .filter(|&&s| t >= s)
                        .map(|s| (-(t - s) * 200.0).exp())
                        .sum();
                    noise() * (bursts + 0.6 * (-(t - 0.02).max(0.0) * 12.0).exp())
                }
                3 | 4 => {
                    let f0 = if ch == 3 { 110.0 } else { 165.0 };
                    sweep(f0 * (1.0 + 0.5 * (-t * 20.0).exp())) * (-t * 8.0).exp()
                }
                5 => sweep(1700.0) * (-t * 60.0).exp() + noise() * (-t * 100.0).exp() * 0.3,
                6 => {
                    ((TAU * 540.0 * t).sin() + (TAU * 800.0 * t).sin()).clamp(-0.8, 0.8)
                        * (-t * 12.0).exp()
                }
                7 => (metal(t) + noise() * 0.3) * (-t * 40.0).exp(),
                8 => (metal(t) + noise() * 0.3) * (-t * 5.0).exp(),
                _ => (metal(t) * 0.5 + noise()) * (-t * 2.5).exp(),
            };
            // Hats and cymbals get a first-difference high-pass to take the body out.
            if ch >= 7 {
                let y = x - last;
                last = x;
                y
            } else {
                x
            }
        })
        .collect();
    let peak = data.iter().fold(0.0f32, |m, x| m.max(x.abs())).max(1e-9);
    data.iter_mut().for_each(|x| *x *= 0.9 / peak);
    Sample {
        name: NAMES[ch].into(),
        data,
        rate: SR,
    }
}

pub fn load_wav(path: &Path) -> Result<Sample, hound::Error> {
    let name = path
        .file_stem()
        .map_or("?".into(), |s| s.to_string_lossy().into_owned());
    decode(hound::WavReader::open(path)?, name)
}

pub fn decode<R: std::io::Read>(
    mut r: hound::WavReader<R>,
    name: String,
) -> Result<Sample, hound::Error> {
    let spec = r.spec();
    let raw: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => r.samples::<f32>().collect::<Result<_, _>>()?,
        hound::SampleFormat::Int => {
            let scale = 1.0 / (1u64 << (spec.bits_per_sample - 1)) as f32;
            r.samples::<i32>()
                .map(|s| s.map(|v| v as f32 * scale))
                .collect::<Result<_, _>>()?
        }
    };
    let n = spec.channels.max(1) as usize;
    // shortcut: multichannel files are mixed to mono; keep stereo if wide samples matter.
    let data = raw
        .chunks(n)
        .map(|f| f.iter().sum::<f32>() / n as f32)
        .collect();
    Ok(Sample {
        name,
        data,
        rate: spec.sample_rate as f32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_kit_is_normalised_and_audible() {
        for ch in 0..10 {
            let s = builtin(ch);
            let peak = s.data.iter().fold(0.0f32, |m, x| m.max(x.abs()));
            assert!(
                (peak - 0.9).abs() < 1e-3 && s.data.iter().all(|x| x.is_finite()),
                "{ch}"
            );
        }
    }

    #[test]
    fn stereo_16bit_wav_loads_as_mono() {
        let path = std::env::temp_dir().join("tenpin_test.wav");
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut w = hound::WavWriter::create(&path, spec).unwrap();
        for v in [16384i16, 0, -32768, -32768] {
            w.write_sample(v).unwrap();
        }
        w.finalize().unwrap();
        let s = load_wav(&path).unwrap();
        assert_eq!(
            (s.data, s.rate, s.name.as_str()),
            (vec![0.25, -1.0], 44100.0, "tenpin_test")
        );
    }
}
