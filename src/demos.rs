//! Demo beats, written as text so they read like a drum chart.
//! Velocity lanes: '.' off, '-' soft, 'o' medium, 'x' hard. Roll lanes: '.' or '1'-'4' hits, 't' for 16th triplets,
//! 'T' for 8th triplets.
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
                        .and_then(|r| match r {
                            't' => Some(crate::seq::TRIPLET as u32),
                            'T' => Some(crate::seq::TRIPLET8 as u32),
                            _ => r.to_digit(10),
                        })
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
            hat("hat_closed_rock"),
            hat("hat_open_rock"),
            s("ride_rock"),
            Kit { decay: 4.0, ..s("crash_rock") },
            s("tom_rock_hi"),
            s("tom_rock_floor"),
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
            hat("hat_closed_rock"),
            hat("hat_open_rock"),
            s("ride_rock"),
            Kit { decay: 4.0, ..s("crash_rock") },
            s("tom_rock_hi"),
            s("tom_rock_floor"),
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
            s("kick_909_punchy"),
            s("snare_909"),
            s("clap_909"),
            hat("hat_closed_909"),
            hat("hat_open_909"),
            s("ride_bell"),
            Kit { decay: 4.0, ..s("crash_bright") },
            s("tom_808_hi"),
            s("tom_808_low"),
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
            s("clap_group"),
            hat("hat_closed_boombap"),
            hat("hat_open_acoustic"),
            s("ride_rock"),
            Kit { decay: 3.0, ..s("crash_rock") },
            s("snap_1"),
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
        // Trap at 140: even 8th-note hats with rolls only into the snare and at the bar end, at the
        // speeds trap really uses (32nds, 16th triplets 't', bouncing 8th triplets 'T'). The 808
        // doubles the kick and carries the tune.
        name: "Hip hop: trap rolls 1",
        bpm: 140.0,
        shuffle: 0.0,
        kit: [
            s("kick_house"),
            s("snare_trap"),
            Kit { level: -3.0, ..s("clap_808") },
            Kit { decay: 0.3, pan: 0.1, ..hat("hat_closed_808") },
            Kit { level: -4.0, decay: 0.4, pan: 0.1, ..hat("hat_open_808") },
            s("ride_bell"),
            Kit { decay: 3.0, ..s("crash_bright") },
            s("snap_1"),
            Kit { decay: 2.5, ..s("sub_808_f") },
            s("perc_cowbell_808"),
        ],
        patterns: [
            // A triplet into the snare and 32nds to end the bar.
            &[
                lane(KICK, "x......x..x....."),
                lane(SNARE, "........x......."),
                lane(CLAP, "........x......."),
                Lane { roll: "......tt......22", ..lane(HAT, "o.o.o.ooo.o.o.oo") },
                Lane { pitch: &[(10, -2)], ..lane(LOW, "x......x..x.....") },
            ],
            // The second half bounces on 8th-note triplets that drop in pitch.
            &[
                lane(KICK, "x.........x..x.."),
                lane(SNARE, "........x......."),
                lane(CLAP, "........x......."),
                Lane {
                    roll: "........TTTTTTTT",
                    pitch: &[(12, -2), (13, -3), (14, -5)],
                    ..lane(HAT, "o.o.o.oooooooooo")
                },
                Lane { pitch: &[(10, -2), (13, 3)], ..lane(LOW, "x.........x..x..") },
            ],
            // Sparser hats, an open hat, and a run that climbs into the next bar.
            &[
                lane(KICK, "x..x......x....."),
                lane(SNARE, "........x......."),
                lane(CLAP, "........x......."),
                Lane {
                    roll: "............tt22",
                    pitch: &[(14, 2), (15, 4)],
                    ..lane(HAT, "o...o...o.o.oooo")
                },
                lane(OPEN, "......o........."),
                Lane { pitch: &[(10, -5)], ..lane(LOW, "x..x......x.....") },
            ],
            // Fill: a 32nd-note snare roll rising into the drop.
            &[
                lane(KICK, "x.........x....."),
                Lane {
                    roll: "............2222",
                    pitch: &[(12, 0), (13, 2), (14, 4), (15, 7)],
                    ..lane(SNARE, "........x...-oox")
                },
                lane(CLAP, "........x......."),
                lane(HAT, "o.o.o.o.o.o.o.o."),
                Lane { pitch: &[(10, -2)], ..lane(LOW, "x.........x.....") },
            ],
        ],
    },
    Demo {
        name: "Hip hop: trap rolls 2",
        bpm: 140.0,
        shuffle: 0.0,
        kit: [
            s("kick_house"),
            s("snare_trap"),
            s("clap_808"),
            Kit { decay: 0.3, pan: 0.1, ..hat("hat_closed_808") },
            Kit { decay: 0.3, pan: 0.1, ..hat("hat_open_808") },
            s("ride_bell"),
            Kit { decay: 3.0, ..s("crash_bright") },
            s("snap_1"),
            Kit { decay: 2.5, ..s("sub_808_f") },
            s("perc_cowbell_808"),
        ],
        // Trap rolls 1's groove with snare rolls in 8th-note triplets: a group of three,
        // then three more a fourth lower, with a stutter driving into the last hit.
        patterns: [
            &[
                lane(KICK, "x.........x....."),
                Lane { roll: "............TTT.", ..lane(SNARE, "........x...-ox.") },
                lane(CLAP, "........x......."),
                Lane { roll: "......tt........", ..lane(HAT, "o.o.o.oox.o.o.o.") },
                Lane { pitch: &[(6, 0), (10, 3), (14, -2)], ..lane(LOW, "x.....x...x...x.") },
            ],
            &[
                lane(KICK, "x.........x....."),
                Lane {
                    roll: "........TT3.TTT.",
                    pitch: &[(12, -5), (13, -5), (14, -5)],
                    ..lane(SNARE, "........xo-.xox.")
                },
                lane(CLAP, "........x......."),
                Lane { roll: "......tt........", ..lane(HAT, "o.o.o.oox.o.o.o.") },
                Lane { pitch: &[(6, 0), (10, 3), (14, -2)], ..lane(LOW, "x.....x...x...x.") },
            ],
            // Flipped: the low group first, then the high group stutters into the next bar.
            &[
                lane(KICK, "x.....x...x....."),
                Lane {
                    roll: "........TTT.TT3.",
                    pitch: &[(8, -5), (9, -5), (10, -5)],
                    ..lane(SNARE, "........oo-.xo-.")
                },
                lane(CLAP, "........x......."),
                Lane { roll: "....tttt........", ..lane(HAT, "o.o.--oox.o.o.o.") },
                Lane { pitch: &[(0, 0), (6, 5), (10, 3), (12, 0)], ..lane(LOW, "x.....x...x.x...") },
            ],
            // Fill: a bar of triplet groups stepping down and back, getting louder.
            &[
                lane(KICK, "x..............."),
                Lane {
                    roll: "TTT.TTT.TT3.TTT.",
                    pitch: &[(4, -5), (5, -5), (6, -5), (12, -5), (13, -5), (14, -5)],
                    ..lane(SNARE, "---.ooo.oo-.xxx.")
                },
                lane(HAT, "o.o.o.o.o.o.o.o."),
                Lane { pitch: &[(0, 0)], ..lane(LOW, "x...............") },
            ],
        ],
    },
    Demo {
        name: "House",
        bpm: 124.0,
        shuffle: 0.2,
        kit: [
            s("kick_909_punchy"),
            s("snare_909"),
            s("clap_909"),
            hat("hat_closed_909"),
            hat("hat_open_909"),
            s("ride_bell"),
            Kit { decay: 4.0, ..s("crash_bright") },
            s("perc_conga_open"),
            s("perc_conga_low"),
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
        name: "Drum & bass 1",
        bpm: 174.0,
        shuffle: 0.0,
        kit: [
            s("kick_heavy"),
            s("snare_dnb"),
            s("clap_909"),
            hat("hat_closed_rock"),
            hat("hat_open_rock"),
            s("ride_rock"),
            Kit { decay: 4.0, ..s("crash_rock") },
            s("tom_rock_hi"),
            s("tom_rock_floor"),
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
        // Liquid, after a High Contrast style groove: two-step kick, a big crack on 2 and 4,
        // ghost snares around it and busy 16th hats that lean on the beat.
        name: "Drum & bass 2: liquid",
        bpm: 173.0,
        shuffle: 0.0,
        kit: [
            Kit { level: 3.0, decay: 0.3, ..s("kick_tech") },
            Kit { level: 3.0, pitch: 2.0, ..s("snare_dnb") },
            // Ghost snares, a little softer and off to one side.
            Kit { pitch: 2.0, level: -5.0, pan: -0.15, ..s("snare_dnb_2") },
            Kit { decay: 0.08, pan: 0.25, ..hat("hat_closed_crisp") },
            Kit { decay: 0.35, pan: 0.25, ..hat("hat_open_acoustic") },
            Kit { level: -6.0, pan: -0.3, ..s("ride_rock") },
            Kit { level: -3.0, decay: 4.0, ..s("crash_bright") },
            Kit { level: -4.0, pan: 0.35, ..s("perc_tambourine") },
            s("tom_rock_floor"),
            Kit { level: -5.0, decay: 0.15, pan: -0.3, ..s("perc_shaker") },
        ],
        patterns: [
            &[
                lane(KICK, "x.........x....."),
                lane(SNARE, "....x.......x..."),
                lane(CLAP, "..-....-o.....-."),
                lane(HAT, "o-oox-o-x-o.x-o-"),
            ],
            &[
                lane(KICK, "x.........x....."),
                lane(SNARE, "....x.......x..."),
                lane(CLAP, "..-...--o-....-."),
                lane(HAT, "o-oox-oox-o.x-oo"),
                lane(TOM_HI, "..-...-...-...-."),
            ],
            &[
                lane(KICK, "x.......-.x....."),
                lane(SNARE, "....x.......x..."),
                lane(CLAP, "..-....-o.....-."),
                lane(HAT, "o-oox-o-x-o.x-o-"),
                lane(CRASH, "x..............."),
                lane(PERC, "-o-o-o-o-o-o-o-o"),
            ],
            // Fill: ghosts tumble into a doubled snare.
            &[
                lane(KICK, "x.........x....."),
                Lane { roll: "..............22", ..lane(SNARE, "....x.......x.ox") },
                lane(CLAP, "..-...--o-.--.-."),
                lane(HAT, "o-oox-o-x-o....."),
                lane(OPEN, "..........o....."),
            ],
        ],
    },
    Demo {
        name: "Funk",
        bpm: 100.0,
        shuffle: 0.3,
        kit: [
            s("kick_boombap"),
            s("snare_jazz"),
            s("rim_sidestick"),
            hat("hat_closed_rock"),
            hat("hat_open_rock"),
            s("ride_bell"),
            Kit { decay: 3.0, ..s("crash_rock") },
            s("perc_cowbell_808"),
            s("tom_rock_floor"),
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
        // Slow and deep: a long subby kick on every beat, claps on 2 and 4, offbeat hats
        // and low congas answering the kick.
        name: "Deep house: slow",
        bpm: 108.0,
        shuffle: 0.1,
        kit: [
            Kit { decay: 1.2, ..s("kick_909_long") },
            Kit { level: -4.0, ..s("snare_909") },
            Kit { level: -2.0, ..s("clap_909") },
            Kit { decay: 0.1, pan: 0.2, ..hat("hat_closed_909") },
            Kit { decay: 0.25, pan: 0.2, ..hat("hat_open_909") },
            Kit { level: -8.0, pan: -0.3, ..s("ride_bell") },
            Kit { level: -4.0, decay: 4.0, ..s("crash_bright") },
            Kit { level: -6.0, pan: -0.3, ..s("rim_808") },
            Kit { level: -3.0, pan: 0.25, ..s("perc_conga_low") },
            Kit { level: -8.0, pan: -0.2, ..s("perc_shaker") },
        ],
        patterns: [
            &[
                lane(KICK, "x...x...x...x..."),
                lane(CLAP, "....x.......x..."),
                lane(OPEN, "..o...o...o...o."),
                lane(LOW, ".........-...-.."),
            ],
            &[
                lane(KICK, "x...x...x...x..."),
                lane(CLAP, "....x.......x..."),
                lane(OPEN, "..o...o...o...o."),
                lane(HAT, "-.......-......."),
                lane(LOW, ".......-.o...-.."),
                lane(TOM_HI, "...-.......-...."),
            ],
            &[
                lane(KICK, "x...x...x...x..."),
                lane(CLAP, "....x.......x..."),
                lane(SNARE, "..............-."),
                lane(OPEN, "..o...o...o...o."),
                lane(PERC, "-.-.-.-.-.-.-.-."),
                lane(LOW, ".........-...-.."),
                lane(CRASH, "x..............."),
            ],
            // Breakdown: the kick drops out after the one.
            &[
                lane(KICK, "x..............."),
                lane(CLAP, "....x.......x..."),
                lane(OPEN, "..o...o...o...o."),
                lane(RIDE, "o...o...o...o..."),
                lane(LOW, ".........-.o.-.."),
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

    #[test]
    fn trap_2_snare_roll_lands_on_triplets_with_a_stutter() {
        use crate::seq::{plays, TICKS};
        let d = ALL.iter().find(|d| d.name == "Hip hop: trap rolls 2").unwrap();
        let steps = d.steps();
        let hits: Vec<i64> = (0..STEPS as i64)
            .flat_map(|k| (0..TICKS).map(move |t| (k, t)))
            .filter(|&(k, t)| {
                let s = steps[pattern_index(1, SNARE, k as usize)];
                s.vel > 0 && plays(s.roll, k, t)
            })
            .map(|(k, t)| k * TICKS + t)
            .collect();
        // Triplets every 16 ticks from beat 3, with 4-tick hits driving into the third.
        assert_eq!(hits, vec![96, 112, 120, 124, 128, 144, 160, 176]);
    }
}
