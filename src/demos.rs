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
        // Trap hats: 8ths broken up by 16th-triplet runs ('t'), 32nd and 32nd-triplet rolls and the
        // odd 64th buzz, with velocity swells and pitch climbs; the 808 plays a tune.
        name: "Hip hop: trap rolls 1",
        bpm: 140.0,
        shuffle: 0.0,
        kit: [
            s("kick_house"),
            s("snare_trap"),
            s("clap_808"),
            // Short and bright so fast rolls stay separate hits instead of smearing.
            Kit { decay: 0.07, pan: 0.1, ..hat("hat_closed_808") },
            Kit { decay: 0.3, pan: 0.1, ..hat("hat_open_808") },
            s("ride_bell"),
            Kit { decay: 3.0, ..s("crash_bright") },
            s("snap_1"),
            Kit { decay: 2.5, ..s("sub_808_f") },
            s("perc_cowbell_808"),
        ],
        patterns: [
            &[
                lane(KICK, "x.........x....."),
                lane(SNARE, "........x......."),
                lane(CLAP, "........x......."),
                Lane {
                    roll: "......tt....2233",
                    pitch: &[(13, 2), (14, 4), (15, 7)],
                    ..lane(HAT, "o.o.o.oox.o.--ox")
                },
                Lane { pitch: &[(6, 0), (10, 3), (14, -2)], ..lane(LOW, "x.....x...x...x.") },
            ],
            // A whole beat of triplets swelling up, then a run diving down.
            &[
                lane(KICK, "x.....x...x....."),
                lane(SNARE, "........x......."),
                lane(CLAP, "........x......."),
                Lane {
                    roll: "....tttt....tttt",
                    pitch: &[(12, 3), (13, 1), (14, -2), (15, -5)],
                    ..lane(HAT, "o.o.--oox.o.xoo-")
                },
                lane(OPEN, "...........o...."),
                Lane { pitch: &[(0, 0), (6, 5), (10, 3), (12, 0)], ..lane(LOW, "x.....x...x.x...") },
            ],
            // Half-time hats that stutter into a buzz.
            &[
                lane(KICK, "x.......x.x....."),
                lane(SNARE, "........x......."),
                Lane {
                    roll: "..........23tt34",
                    pitch: &[(11, 3), (14, 5), (15, 9)],
                    ..lane(HAT, "o...o...o.-o--ox")
                },
                lane(PERC, "..-.......-....."),
                Lane { pitch: &[(0, -5), (8, -5), (10, -2)], ..lane(LOW, "x.......x.x.....") },
            ],
            // Fill: snare rolls rising into the drop over hats that speed up.
            &[
                lane(KICK, "x..............."),
                Lane {
                    roll: "....1...22223344",
                    pitch: &[(12, 2), (13, 4), (14, 7), (15, 12)],
                    ..lane(SNARE, "....x...oooooxxx")
                },
                Lane { roll: "........tttt3344", ..lane(HAT, "-.-.-.-.--------") },
                Lane { pitch: &[(0, 0)], ..lane(LOW, "x...............") },
            ],
        ],
    },
    Demo {
        name: "Hip hop: trap rolls 2",
        bpm: 145.0,
        shuffle: 0.0,
        kit: [
            s("kick_house"),
            s("snare_trap"),
            s("clap_808"),
            Kit { decay: 0.07, pan: 0.1, ..hat("hat_closed_808") },
            Kit { decay: 0.3, pan: 0.1, ..hat("hat_open_808") },
            s("ride_bell"),
            Kit { decay: 3.0, ..s("crash_bright") },
            s("snap_1"),
            Kit { decay: 2.5, ..s("sub_808_f") },
            s("perc_cowbell_808"),
        ],
        // Snare rolls in 8th-note triplets: a group of three, then three more a fourth lower,
        // with one hit chopped into a fast stutter.
        patterns: [
            &[
                lane(KICK, "x......x..x....."),
                lane(SNARE, "........x......."),
                lane(CLAP, "........x......."),
                Lane { roll: "..........tt....", ..lane(HAT, "o.o.o.o.o.oo-.o.") },
                Lane { pitch: &[(7, 3), (10, 0)], ..lane(LOW, "x......x..x.....") },
            ],
            &[
                lane(KICK, "x......x........"),
                Lane {
                    roll: "........T33.TTT.",
                    pitch: &[(12, -5), (13, -5), (14, -5)],
                    ..lane(SNARE, "........x-o.xox.")
                },
                lane(CLAP, "........x......."),
                lane(HAT, "o.o.o.o.o...o..."),
                Lane { pitch: &[(7, 3)], ..lane(LOW, "x......x........") },
            ],
            // Flipped: low group first, then the high group stutters up into the next bar.
            &[
                lane(KICK, "x.....x...x....."),
                Lane {
                    roll: "........TTT.T33.",
                    pitch: &[(8, -5), (9, -5), (10, -5), (14, 2)],
                    ..lane(SNARE, "........-o-.x-ox")
                },
                lane(CLAP, "........x......."),
                Lane { roll: "....tt..........", ..lane(HAT, "o.o.oo..o.o.o.o.") },
                Lane { pitch: &[(6, 5), (10, 3)], ..lane(LOW, "x.....x...x.....") },
            ],
            // Fill: a whole bar of triplet groups, getting louder.
            &[
                lane(KICK, "x..............."),
                Lane {
                    roll: "TTT.TTT.T33.TTT.",
                    pitch: &[(4, -5), (5, -5), (6, -5), (8, 2), (9, 2), (10, 2), (12, -3), (13, -3), (14, -3)],
                    ..lane(SNARE, "---.ooo.o-o.xxx.")
                },
                lane(HAT, "o.o.o.o........."),
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
        // Liquid two-step: a round kick on 1 and the "and" of 3, a bright tuned-up snare with ghost
        // notes around it, and shaker, tambourine and hats rolling underneath.
        name: "Drum & bass: liquid",
        bpm: 174.0,
        shuffle: 0.1,
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
                lane(CLAP, ".......-.-.....-"),
                lane(HAT, "-.o.-.o.-.o.-.o."),
                lane(TOM_HI, "..o...o...o...o."),
                lane(PERC, "-o-o-o-o-o-o-o-o"),
            ],
            &[
                lane(KICK, "x.x.......x....."),
                lane(SNARE, "....x.......x..."),
                lane(CLAP, "..-....-.-.-...-"),
                lane(HAT, "-.o.-.o.-.o.-oo."),
                lane(OPEN, "..............o."),
                lane(TOM_HI, "..o...o...o...o."),
                lane(PERC, "-o-o-o-o-o-o-o-o"),
            ],
            &[
                lane(KICK, "x.........x....."),
                lane(SNARE, "....x.......x..."),
                lane(CLAP, ".......-.-....-."),
                lane(RIDE, "o.-.o.-.o.-.o.-."),
                lane(CRASH, "x..............."),
                lane(TOM_HI, "..o...o...o...o."),
                lane(PERC, "-o-o-o-o-o-o-o-o"),
            ],
            // Fill: ghosts tumble into a doubled snare and a floor tom.
            &[
                lane(KICK, "x.........x....."),
                Lane { roll: "..............22", ..lane(SNARE, "....x.......x.ox") },
                lane(CLAP, ".......-.-.--.-."),
                lane(HAT, "-.o.-.o.-.o....."),
                lane(LOW, "...........o...."),
                lane(PERC, "-o-o-o-o-o-o-o-o"),
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
        // Triplets every 16 ticks from beat 3, with the second one chopped into 4-tick hits.
        assert_eq!(hits, vec![96, 108, 112, 116, 120, 124, 128, 144, 160, 176]);
    }
}
