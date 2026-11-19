//! Demo beats, written as text so they read like a drum chart.
//! Velocity lanes: '.' off, '-' soft, 'o' medium, 'x' hard. Roll lanes: '.' or '1'-'4' hits.
use crate::{pattern_index, Step, CHANNELS, PATTERNS, STEPS};

#[derive(Clone, Copy)]
pub struct Kit {
    pub sample: &'static str,
    pub level: f32,
    pub pitch: f32,
    pub decay: f32,
    pub tone: f32,
    pub pan: f32,
    pub choke: bool,
}

pub const K: Kit = Kit {
    sample: "",
    level: 0.0,
    pitch: 0.0,
    decay: 2.0,
    tone: 1.0,
    pan: 0.0,
    choke: false,
};

pub struct Lane {
    pub ch: usize,
    pub vel: &'static str,
    pub roll: &'static str,
    pub pitch: &'static [(usize, i8)],
}

pub const fn lane(ch: usize, vel: &'static str) -> Lane {
    Lane {
        ch,
        vel,
        roll: "",
        pitch: &[],
    }
}

pub struct Demo {
    pub name: &'static str,
    pub bpm: f32,
    pub shuffle: f32,
    pub kit: [Kit; CHANNELS],
    pub patterns: [&'static [Lane]; PATTERNS],
}

impl Demo {
    pub fn steps(&self) -> Vec<Step> {
        let mut out = vec![Step::default(); PATTERNS * CHANNELS * STEPS];
        for (p, lanes) in self.patterns.iter().enumerate() {
            for l in lanes.iter() {
                for (s, c) in l.vel.chars().take(STEPS).enumerate() {
                    let step = &mut out[pattern_index(p, l.ch, s)];
                    step.vel = match c {
                        '-' => 1,
                        'o' => 2,
                        'x' => 3,
                        _ => 0,
                    };
                    step.roll = l
                        .roll
                        .chars()
                        .nth(s)
                        .and_then(|r| r.to_digit(10))
                        .unwrap_or(1) as u8;
                }
                for &(s, semis) in l.pitch {
                    out[pattern_index(p, l.ch, s)].pitch = semis;
                }
            }
        }
        out
    }
}

const fn synth(sample: &'static str) -> Kit {
    Kit { sample, ..K }
}

const fn s(sample: &'static str) -> Kit {
    Kit { sample, ..K }
}

/// Closed and open hats on channels 4 and 5 choke each other.
const fn hat(sample: &'static str) -> Kit {
    Kit { sample, choke: true, ..K }
}

// Every demo uses the same channel layout:
// 1 kick, 2 snare, 3 clap, 4 closed hat, 5 open hat, 6 ride, 7 crash, 8 hi tom, 9 low tom or 808, 10 perc.
const KICK: usize = 0;
const SNARE: usize = 1;
const CLAP: usize = 2;
const HAT: usize = 3;
const OPEN: usize = 4;
const RIDE: usize = 5;
const CRASH: usize = 6;
const TOM_HI: usize = 7;
const LOW: usize = 8;
const PERC: usize = 9;

pub const ALL: &[Demo] = &[
    Demo {
        name: "Rock: straight eights",
        bpm: 120.0,
        shuffle: 0.0,
        kit: [
            s("kick_rock"),
            s("snare_rock"),
            s("rim_sidestick"),
            hat("hat_closed_acoustic"),
            hat("hat_open_acoustic"),
            s("ride_acoustic"),
            Kit { decay: 4.0, ..s("crash_acoustic") },
            s("tom_acoustic_hi"),
            s("tom_acoustic_floor"),
            s("perc_tambourine"),
        ],
        patterns: [
            &[
                lane(KICK, "x.....x.x......."),
                lane(SNARE, "....x.......x..."),
                lane(HAT, "x.o.x.o.x.o.x.o."),
                lane(CRASH, "x..............."),
            ],
            &[
                lane(KICK, "x.....x.x.x....."),
                lane(SNARE, "....x.......x..-"),
                lane(HAT, "x.o.x.o.x.o.x..."),
                lane(OPEN, "..............o."),
                lane(PERC, "....o.......o..."),
            ],
            // Half-time on the ride.
            &[
                lane(KICK, "x.....x...x....."),
                lane(SNARE, "........x......."),
                lane(RIDE, "x.o.x.o.x.o.x.o."),
                lane(CRASH, "x..............."),
            ],
            // Fill down the toms.
            &[
                lane(KICK, "x.......x......."),
                lane(SNARE, "....x...xo......"),
                lane(HAT, "x.o.x.o........."),
                lane(TOM_HI, "..........xo...."),
                lane(LOW, "............xoxx"),
            ],
        ],
    },
    Demo {
        // Steps are eighth notes here: half tempo with shuffle gives the triplet swing.
        name: "Rock: shuffle",
        bpm: 58.0,
        shuffle: 0.66,
        kit: [
            s("kick_rock"),
            s("snare_rock"),
            s("rim_sidestick"),
            hat("hat_closed_acoustic"),
            hat("hat_open_acoustic"),
            s("ride_acoustic"),
            Kit { decay: 4.0, ..s("crash_acoustic") },
            s("tom_acoustic_hi"),
            s("tom_acoustic_floor"),
            s("perc_tambourine"),
        ],
        patterns: [
            &[
                lane(KICK, "x...x...x...x.x."),
                lane(SNARE, "..x...x...x...x."),
                lane(HAT, "x-x-x-x-x-x-x-x-"),
            ],
            &[
                lane(KICK, "x...x.x.x...x.x."),
                lane(SNARE, "..x...x-..x...xo"),
                lane(RIDE, "x-x-x-x-x-x-x-x-"),
                lane(CRASH, "x..............."),
            ],
            &[
                lane(KICK, "x.......x......."),
                lane(SNARE, "..x...x...x...x."),
                lane(HAT, "x-x-x-x-x-x-x-x-"),
                lane(PERC, "..o...o...o...o."),
            ],
            &[
                lane(KICK, "x...x...x......."),
                lane(SNARE, "..x...x...xoxo.."),
                lane(TOM_HI, "............xo.."),
                lane(LOW, "..............xx"),
                lane(HAT, "x-x-x-x-........"),
            ],
        ],
    },
    Demo {
        name: "Trance",
        bpm: 138.0,
        shuffle: 0.0,
        kit: [
            s("kick_909"),
            s("snare_909"),
            s("clap_909"),
            hat("hat_closed_909"),
            hat("hat_open_909"),
            s("ride_909"),
            Kit { decay: 4.0, ..s("crash_909") },
            s("tom_909_hi"),
            s("tom_909_lo"),
            s("perc_shaker"),
        ],
        patterns: [
            &[
                lane(KICK, "x...x...x...x..."),
                lane(CLAP, "....x.......x..."),
                lane(HAT, "o-.-o-.-o-.-o-.-"),
                lane(OPEN, "..x...x...x...x."),
            ],
            &[
                lane(KICK, "x...x...x...x..."),
                lane(CLAP, "....x.......x..."),
                lane(HAT, "o-.-o-.-o-.-o-.-"),
                lane(OPEN, "..x...x...x...x."),
                lane(RIDE, "-.o.-.o.-.o.-.o."),
                lane(PERC, "-o-o-o-o-o-o-o-o"),
                lane(CRASH, "x..............."),
            ],
            // Breakdown: no kick.
            &[
                lane(CLAP, "....x.......x..."),
                lane(OPEN, "..o...o...o...o."),
                lane(PERC, "-o-o-o-o-o-o-o-o"),
                lane(CRASH, "x..............."),
            ],
            // Build: the snare rolls get faster and rise in pitch.
            &[
                lane(KICK, "x...x...x...x..."),
                Lane {
                    roll: "1.1.1.1.22223344",
                    pitch: &[(8, 2), (9, 3), (10, 4), (11, 5), (12, 7), (13, 8), (14, 10), (15, 12)],
                    ..lane(SNARE, "o.o.o.o.oooooxxx")
                },
                lane(HAT, "o-.-o-.-o-.-o-.-"),
            ],
        ],
    },
    Demo {
        name: "Hip hop: boom bap",
        bpm: 90.0,
        shuffle: 0.55,
        kit: [
            s("kick_boombap"),
            s("snare_boombap"),
            s("clap_hiphop"),
            hat("hat_closed_dusty"),
            hat("hat_open_dusty"),
            s("ride_acoustic"),
            Kit { decay: 3.0, ..s("crash_acoustic") },
            s("perc_snap"),
            s("perc_tambourine"),
            s("rim_sidestick"),
        ],
        patterns: [
            &[
                lane(KICK, "x......x..x....."),
                lane(SNARE, "....x.......x..."),
                lane(HAT, "o.-.o.-.o.-.o.-."),
            ],
            &[
                lane(KICK, "x.....xx..x...x."),
                lane(SNARE, "....x.......x.-."),
                lane(HAT, "o.-.o.-.o.-.o.o."),
                lane(OPEN, "..............o."),
                lane(TOM_HI, "....o.......o..."),
            ],
            &[
                lane(KICK, "x.........x....."),
                lane(SNARE, "....x.......x..."),
                lane(PERC, "o.......o......."),
                lane(LOW, "..-...-...-...-."),
            ],
            &[
                lane(KICK, "x......x..x..x.."),
                lane(SNARE, "....x.......x.xx"),
                lane(HAT, "o.-.o.-.o.-....."),
                lane(CRASH, "x..............."),
            ],
        ],
    },
    Demo {
        // Hat rolls switch speed (2, 3 and 4 hits a step) and climb in pitch; the 808 plays a tune.
        name: "Hip hop: trap rolls",
        bpm: 140.0,
        shuffle: 0.0,
        kit: [
            s("kick_808_punchy"),
            s("snare_trap"),
            s("clap_808"),
            hat("hat_closed_808"),
            hat("hat_open_808"),
            s("ride_909"),
            Kit { decay: 3.0, ..s("crash_909") },
            s("perc_snap"),
            Kit { decay: 2.5, ..s("sub_808") },
            s("perc_cowbell"),
        ],
        patterns: [
            &[
                lane(KICK, "x.........x....."),
                lane(SNARE, "........x......."),
                lane(CLAP, "........x......."),
                Lane {
                    roll: "1111112211113334",
                    pitch: &[(12, 1), (13, 3), (14, 5), (15, 7)],
                    ..lane(HAT, "o-o-o-ooo-o-oooo")
                },
                Lane { pitch: &[(6, 0), (10, 3), (14, -2)], ..lane(LOW, "x.....x...x...x.") },
            ],
            &[
                lane(KICK, "x.....x...x....."),
                lane(SNARE, "........x......."),
                lane(CLAP, "........x......."),
                Lane {
                    roll: "1.1.3.1.1.1.2.4.",
                    pitch: &[(4, -2), (14, 5)],
                    ..lane(HAT, "o.o.o.o.o.o.o.o.")
                },
                lane(OPEN, "...........o...."),
                Lane { pitch: &[(0, 0), (6, 5), (10, 3), (12, 0)], ..lane(LOW, "x.....x...x.x...") },
            ],
            // Half-speed hats with triplet bursts.
            &[
                lane(KICK, "x.......x.x....."),
                lane(SNARE, "........x......."),
                Lane {
                    roll: "1...3...1...3.4.",
                    pitch: &[(4, 2), (12, -3), (14, -5)],
                    ..lane(HAT, "o...o...o...o.o.")
                },
                lane(PERC, "..-.......-....."),
                Lane { pitch: &[(0, -5), (8, -5), (10, -2)], ..lane(LOW, "x.......x.x.....") },
            ],
            // Fill: snare rolls rising into the drop.
            &[
                lane(KICK, "x..............."),
                Lane {
                    roll: "....1...22223344",
                    pitch: &[(12, 2), (13, 4), (14, 7), (15, 12)],
                    ..lane(SNARE, "....x...oooooxxx")
                },
                Lane { roll: "4444444444444444", ..lane(HAT, "-.-.-.-.-.-.-.-.") },
                Lane { pitch: &[(0, 0)], ..lane(LOW, "x...............") },
            ],
        ],
    },
    Demo {
        name: "House",
        bpm: 124.0,
        shuffle: 0.2,
        kit: [
            s("kick_909"),
            s("snare_909"),
            s("clap_909"),
            hat("hat_closed_909"),
            hat("hat_open_909"),
            s("ride_909"),
            Kit { decay: 4.0, ..s("crash_909") },
            s("perc_conga_hi"),
            s("perc_conga_lo"),
            s("perc_shaker"),
        ],
        patterns: [
            &[
                lane(KICK, "x...x...x...x..."),
                lane(CLAP, "....x.......x..."),
                lane(HAT, "-.-.-.-.-.-.-.-."),
                lane(OPEN, "..o...o...o...o."),
            ],
            &[
                lane(KICK, "x...x...x...x..."),
                lane(CLAP, "....x.......x..."),
                lane(HAT, "-o-.-o-.-o-.-o-."),
                lane(OPEN, "..o...o...o...o."),
                lane(PERC, "o-o-o-o-o-o-o-o-"),
                lane(TOM_HI, "...o.....o...o.."),
                lane(LOW, "o.....o.....o..."),
            ],
            &[
                lane(KICK, "x...x...x...x..."),
                lane(SNARE, "....-.......-..."),
                lane(CLAP, "....x.......x..."),
                lane(RIDE, "o.-.o.-.o.-.o.-."),
                lane(OPEN, "..o...o...o...o."),
            ],
            &[
                lane(KICK, "x...x...x...x..."),
                Lane { roll: "........11223344", ..lane(CLAP, "....x...xxxxxxxx") },
                lane(OPEN, "..o...o...o...o."),
                lane(CRASH, "x..............."),
            ],
        ],
    },
    Demo {
        name: "Drum & bass",
        bpm: 174.0,
        shuffle: 0.0,
        kit: [
            s("kick_dnb"),
            s("snare_dnb"),
            s("clap_909"),
            hat("hat_closed_acoustic"),
            hat("hat_open_acoustic"),
            s("ride_acoustic"),
            Kit { decay: 4.0, ..s("crash_acoustic") },
            s("tom_acoustic_hi"),
            s("tom_acoustic_floor"),
            s("perc_shaker"),
        ],
        patterns: [
            &[
                lane(KICK, "x.........x....."),
                lane(SNARE, "....x..-.-..x..-"),
                lane(HAT, "o.o.o.o.o.o.o.o."),
            ],
            &[
                lane(KICK, "x.x.......x....."),
                lane(SNARE, "....x..-.x..x..."),
                lane(RIDE, "o.-.o.-.o.-.o.-."),
                lane(PERC, "-o-o-o-o-o-o-o-o"),
                lane(CRASH, "x..............."),
            ],
            &[
                lane(KICK, "x.........x.x..."),
                lane(SNARE, "....x.......x..."),
                Lane { roll: "1.1.1.1.1.1.2.2.", ..lane(HAT, "o.o.o.o.o.o.o.o.") },
                lane(OPEN, ".......o........"),
            ],
            &[
                lane(KICK, "x.........x....."),
                lane(SNARE, "....x..-.xxo.ooo"),
                lane(TOM_HI, "...........x...."),
                lane(LOW, "............x..."),
                lane(HAT, "o.o.o.o.o......."),
            ],
        ],
    },
    Demo {
        name: "Funk",
        bpm: 100.0,
        shuffle: 0.3,
        kit: [
            s("kick_boombap"),
            s("snare_funk"),
            s("rim_sidestick"),
            hat("hat_closed_acoustic"),
            hat("hat_open_acoustic"),
            s("ride_bell"),
            Kit { decay: 3.0, ..s("crash_acoustic") },
            s("perc_cowbell"),
            s("tom_acoustic_floor"),
            s("perc_tambourine"),
        ],
        patterns: [
            &[
                lane(KICK, "x..x..x...x..x.."),
                lane(SNARE, "....x..-.-..x..-"),
                lane(HAT, "xoxoxo.oxoxoxoxo"),
                lane(OPEN, "......o........."),
            ],
            &[
                lane(KICK, "x..x..x...xx.x.."),
                lane(SNARE, "-...x..-.-..x.-o"),
                lane(HAT, "xoxoxo.oxoxoxo.o"),
                lane(OPEN, "......o.......o."),
                lane(PERC, "....o.......o..."),
            ],
            &[
                lane(KICK, "x.....x...x....."),
                lane(SNARE, "....x..-.-..x..."),
                lane(RIDE, "x.o.x.o.x.o.x.o."),
                lane(TOM_HI, "x..x..x...x..x.."),
            ],
            &[
                lane(KICK, "x..x..x........."),
                lane(SNARE, "....x..-..xoxo.."),
                lane(LOW, "..............xx"),
                lane(HAT, "xoxoxo.o........"),
                lane(CRASH, "x..............."),
            ],
        ],
    },
    Demo {
        name: "Tenpin synth kit",
        bpm: 120.0,
        shuffle: 0.0,
        kit: [
            synth("kick_tenpin"),
            synth("snare_tenpin"),
            synth("clap_tenpin"),
            hat("hat_closed_tenpin"),
            hat("hat_open_tenpin"),
            synth("rim_tenpin"),
            synth("crash_tenpin"),
            synth("tom_tenpin_hi"),
            synth("tom_tenpin_lo"),
            synth("perc_tenpin_cowbell"),
        ],
        patterns: [
            &[
                lane(KICK, "x...x...x...x..."),
                lane(SNARE, "....x.......x..."),
                lane(HAT, "o.-.o.-.o.-.o..."),
                lane(OPEN, "..............o."),
            ],
            &[
                lane(KICK, "x...x...x...x.x."),
                lane(SNARE, "....x.......x..."),
                lane(CLAP, "....x.......x..x"),
                Lane { roll: "1111111111112234", ..lane(HAT, "o.-.o.-.o.-.oooo") },
            ],
            &[
                lane(KICK, "x.....x...x....."),
                lane(SNARE, "....x.......x..."),
                lane(RIDE, "..-..-..-..-.-.-"),
                lane(PERC, "x..x..x...x..x.."),
                lane(HAT, "-o-o-o-o-o-o-o-o"),
            ],
            &[
                lane(KICK, "x..............."),
                lane(SNARE, "....x...o.o.xxxx"),
                Lane { pitch: &[(8, 5), (10, 3), (12, 0), (13, -3), (14, -5), (15, -7)], ..lane(LOW, "........x.x.xxxx") },
                lane(CRASH, "x..............."),
            ],
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demos_use_real_samples_and_full_lanes() {
        for d in ALL {
            for k in &d.kit {
                assert!(crate::library::find(k.sample).is_some(), "{}: no sample {}", d.name, k.sample);
            }
            for lanes in d.patterns {
                for l in lanes.iter() {
                    assert_eq!(l.vel.len(), STEPS, "{}: lane {}", d.name, l.ch);
                    assert!(l.roll.is_empty() || l.roll.len() == STEPS, "{}: roll {}", d.name, l.ch);
                }
            }
        }
    }
}
