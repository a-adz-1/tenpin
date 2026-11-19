//! A small Freeverb-style room (8 combs into 4 allpasses per side) for the channels' reverb sends.

const COMBS: [usize; 8] = [1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617];
const ALLPASSES: [usize; 4] = [556, 441, 341, 225];
/// Extra delay on the right side, which is what makes the room stereo.
const SPREAD: usize = 23;
const DAMP: f32 = 0.2;

struct Line {
    buf: Vec<f32>,
    i: usize,
    /// Low-pass state in the comb's feedback path.
    store: f32,
}

impl Line {
    fn new(len: usize) -> Self {
        Self {
            buf: vec![0.0; len.max(1)],
            i: 0,
            store: 0.0,
        }
    }

    fn comb(&mut self, x: f32, feedback: f32) -> f32 {
        let y = self.buf[self.i];
        self.store = y + (self.store - y) * DAMP;
        self.buf[self.i] = x + self.store * feedback;
        self.i = (self.i + 1) % self.buf.len();
        y
    }

    fn allpass(&mut self, x: f32) -> f32 {
        let b = self.buf[self.i];
        self.buf[self.i] = x + b * 0.5;
        self.i = (self.i + 1) % self.buf.len();
        b - x
    }
}

pub struct Reverb {
    /// Combs and allpasses for the left and right outputs.
    sides: [(Vec<Line>, Vec<Line>); 2],
}

impl Reverb {
    /// Allocates the delay lines, so call it outside the audio thread.
    pub fn new(sample_rate: f32) -> Self {
        let side = |extra: usize| {
            let line = |n: &usize| Line::new(((n + extra) as f32 * sample_rate / 44100.0) as usize);
            (COMBS.iter().map(line).collect(), ALLPASSES.iter().map(line).collect())
        };
        Self {
            sides: [side(0), side(SPREAD)],
        }
    }

    pub fn reset(&mut self) {
        for (combs, allpasses) in &mut self.sides {
            for l in combs.iter_mut().chain(allpasses.iter_mut()) {
                l.buf.fill(0.0);
                l.store = 0.0;
            }
        }
    }

    /// One stereo frame of reverb for a mono send. `size` from 0 to 1 sets how long it rings.
    pub fn process(&mut self, x: f32, size: f32) -> (f32, f32) {
        let feedback = 0.7 + 0.28 * size;
        let mut out = [0.0; 2];
        for (o, (combs, allpasses)) in out.iter_mut().zip(&mut self.sides) {
            let mut y: f32 = combs.iter_mut().map(|c| c.comb(x * 0.015, feedback)).sum();
            for a in allpasses.iter_mut() {
                y = a.allpass(y);
            }
            *o = y;
        }
        // Freeverb's own output sits about 10 dB under the dry hit; this makes a full send 6 dB louder.
        (out[0] * 2.0, out[1] * 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Energy of the tail after an impulse, and how long until it falls 60 dB.
    fn ring(size: f32) -> (f32, f32) {
        let sr = 48000.0;
        let mut r = Reverb::new(sr);
        let tail: Vec<f32> = (0..sr as usize * 4)
            .map(|i| {
                let (l, r) = r.process(if i == 0 { 1.0 } else { 0.0 }, size);
                (l * l + r * r) / 2.0
            })
            .collect();
        let peak = tail.chunks(480).map(|c| c.iter().sum::<f32>()).fold(0.0, f32::max);
        let last = tail.chunks(480).rposition(|c| c.iter().sum::<f32>() > peak * 1e-6).unwrap();
        (tail.iter().sum(), last as f32 * 0.01)
    }

    #[test]
    fn rings_longer_with_size_and_dies_away() {
        let (small_energy, small) = ring(0.2);
        let (big_energy, big) = ring(0.9);
        assert!(small > 0.3 && small < big && big < 4.0, "{small} {big}");
        assert!(small_energy > 0.0 && big_energy > small_energy);
    }
}
