//! The sample library: the built-in synthesized kit plus the WAVs in `samples/`, embedded in the
//! plugin so it works without any files next to it.
use crate::kit::{self, Sample};
use std::sync::{Arc, OnceLock};

/// Embedded samples. Channels save their sample as an index into the library, so only ever append
/// to this list: inserting would change the sound of saved projects.
const WAVS: &[(&str, &[u8])] = &[
    ("clap_808", include_bytes!("../samples/clap_808.wav")),
    ("clap_909", include_bytes!("../samples/clap_909.wav")),
    ("clap_group", include_bytes!("../samples/clap_group.wav")),
    ("clap_solo", include_bytes!("../samples/clap_solo.wav")),
    ("crash_bright", include_bytes!("../samples/crash_bright.wav")),
    ("crash_china", include_bytes!("../samples/crash_china.wav")),
    ("crash_rock", include_bytes!("../samples/crash_rock.wav")),
    ("hat_closed_808", include_bytes!("../samples/hat_closed_808.wav")),
    ("hat_closed_909", include_bytes!("../samples/hat_closed_909.wav")),
    ("hat_closed_boombap", include_bytes!("../samples/hat_closed_boombap.wav")),
    ("hat_closed_clean", include_bytes!("../samples/hat_closed_clean.wav")),
    ("hat_closed_crisp", include_bytes!("../samples/hat_closed_crisp.wav")),
    ("hat_closed_rock", include_bytes!("../samples/hat_closed_rock.wav")),
    ("hat_closed_shank", include_bytes!("../samples/hat_closed_shank.wav")),
    ("hat_closed_tight", include_bytes!("../samples/hat_closed_tight.wav")),
    ("hat_closed_vintage", include_bytes!("../samples/hat_closed_vintage.wav")),
    ("hat_open_808", include_bytes!("../samples/hat_open_808.wav")),
    ("hat_open_909", include_bytes!("../samples/hat_open_909.wav")),
    ("hat_open_acoustic", include_bytes!("../samples/hat_open_acoustic.wav")),
    ("hat_open_rock", include_bytes!("../samples/hat_open_rock.wav")),
    ("hat_pedal_rock", include_bytes!("../samples/hat_pedal_rock.wav")),
    ("hat_pedal_vintage", include_bytes!("../samples/hat_pedal_vintage.wav")),
    ("hat_pedal_zildjian", include_bytes!("../samples/hat_pedal_zildjian.wav")),
    ("kick_808_boom", include_bytes!("../samples/kick_808_boom.wav")),
    ("kick_808_deep", include_bytes!("../samples/kick_808_deep.wav")),
    ("kick_808_long", include_bytes!("../samples/kick_808_long.wav")),
    ("kick_909_long", include_bytes!("../samples/kick_909_long.wav")),
    ("kick_909_punchy", include_bytes!("../samples/kick_909_punchy.wav")),
    ("kick_boombap", include_bytes!("../samples/kick_boombap.wav")),
    ("kick_dusty", include_bytes!("../samples/kick_dusty.wav")),
    ("kick_heavy", include_bytes!("../samples/kick_heavy.wav")),
    ("kick_house", include_bytes!("../samples/kick_house.wav")),
    ("kick_jazz", include_bytes!("../samples/kick_jazz.wav")),
    ("kick_rock", include_bytes!("../samples/kick_rock.wav")),
    ("kick_tech", include_bytes!("../samples/kick_tech.wav")),
    ("perc_bongo_hi", include_bytes!("../samples/perc_bongo_hi.wav")),
    ("perc_bongo_lo", include_bytes!("../samples/perc_bongo_lo.wav")),
    ("perc_clave", include_bytes!("../samples/perc_clave.wav")),
    ("perc_clave_808", include_bytes!("../samples/perc_clave_808.wav")),
    ("perc_conga_low", include_bytes!("../samples/perc_conga_low.wav")),
    ("perc_conga_mute", include_bytes!("../samples/perc_conga_mute.wav")),
    ("perc_conga_open", include_bytes!("../samples/perc_conga_open.wav")),
    ("perc_cowbell", include_bytes!("../samples/perc_cowbell.wav")),
    ("perc_cowbell_808", include_bytes!("../samples/perc_cowbell_808.wav")),
    ("perc_shaker", include_bytes!("../samples/perc_shaker.wav")),
    ("perc_shaker_up", include_bytes!("../samples/perc_shaker_up.wav")),
    ("perc_tambourine", include_bytes!("../samples/perc_tambourine.wav")),
    ("perc_tambourine_hit", include_bytes!("../samples/perc_tambourine_hit.wav")),
    ("perc_woodblock", include_bytes!("../samples/perc_woodblock.wav")),
    ("ride_bell", include_bytes!("../samples/ride_bell.wav")),
    ("ride_rock", include_bytes!("../samples/ride_rock.wav")),
    ("rim_808", include_bytes!("../samples/rim_808.wav")),
    ("rim_crossstick_jazz", include_bytes!("../samples/rim_crossstick_jazz.wav")),
    ("rim_sidestick", include_bytes!("../samples/rim_sidestick.wav")),
    ("snap_1", include_bytes!("../samples/snap_1.wav")),
    ("snap_2", include_bytes!("../samples/snap_2.wav")),
    ("snare_808", include_bytes!("../samples/snare_808.wav")),
    ("snare_909", include_bytes!("../samples/snare_909.wav")),
    ("snare_909_tight", include_bytes!("../samples/snare_909_tight.wav")),
    ("snare_boombap", include_bytes!("../samples/snare_boombap.wav")),
    ("snare_boombap_soft", include_bytes!("../samples/snare_boombap_soft.wav")),
    ("snare_dnb", include_bytes!("../samples/snare_dnb.wav")),
    ("snare_dnb_2", include_bytes!("../samples/snare_dnb_2.wav")),
    ("snare_dusty", include_bytes!("../samples/snare_dusty.wav")),
    ("snare_jazz", include_bytes!("../samples/snare_jazz.wav")),
    ("snare_rock", include_bytes!("../samples/snare_rock.wav")),
    ("snare_rock_rimshot", include_bytes!("../samples/snare_rock_rimshot.wav")),
    ("snare_trap", include_bytes!("../samples/snare_trap.wav")),
    ("sub_808_c_drive", include_bytes!("../samples/sub_808_c_drive.wav")),
    ("sub_808_f", include_bytes!("../samples/sub_808_f.wav")),
    ("tom_808_hi", include_bytes!("../samples/tom_808_hi.wav")),
    ("tom_808_low", include_bytes!("../samples/tom_808_low.wav")),
    ("tom_808_mid", include_bytes!("../samples/tom_808_mid.wav")),
    ("tom_rock_floor", include_bytes!("../samples/tom_rock_floor.wav")),
    ("tom_rock_hi", include_bytes!("../samples/tom_rock_hi.wav")),
    ("tom_rock_mid", include_bytes!("../samples/tom_rock_mid.wav")),
];

/// Category prefixes, in menu order. A sample's category is the prefix its name starts with.
pub const CATEGORIES: &[&str] = &[
    "kick",
    "sub",
    "snare",
    "rim",
    "clap",
    "snap",
    "hat_closed",
    "hat_open",
    "hat_pedal",
    "ride",
    "crash",
    "tom",
    "perc",
];

pub fn len() -> usize {
    kit::NAMES.len() + WAVS.len()
}

pub fn name(i: usize) -> &'static str {
    kit::NAMES
        .get(i)
        .copied()
        .unwrap_or_else(|| WAVS[(i - kit::NAMES.len()).min(WAVS.len() - 1)].0)
}

pub fn find(name_: &str) -> Option<usize> {
    (0..len()).find(|&i| name(i) == name_)
}

pub fn category(name: &str) -> &'static str {
    CATEGORIES
        .iter()
        .find(|c| name.starts_with(&format!("{c}_")))
        .copied()
        .unwrap_or("perc")
}

/// A short label for menus and strips: the name minus its category, e.g. "808 long".
pub fn label(i: usize) -> String {
    let n = name(i);
    n.strip_prefix(category(n))
        .unwrap_or(n)
        .trim_start_matches('_')
        .replace('_', " ")
}

/// Every sample, decoded on first use.
pub fn all() -> &'static [Arc<Sample>] {
    static LIB: OnceLock<Vec<Arc<Sample>>> = OnceLock::new();
    LIB.get_or_init(|| {
        let synth = (0..kit::NAMES.len()).map(kit::builtin);
        let wavs = WAVS.iter().map(|(name, bytes)| {
            let reader = hound::WavReader::new(std::io::Cursor::new(*bytes)).expect(name);
            kit::decode(reader, name.to_string()).expect(name)
        });
        synth.chain(wavs).map(Arc::new).collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_unique_categorised_and_decode() {
        assert_eq!(all().len(), len());
        for i in 0..len() {
            assert_eq!(find(name(i)), Some(i), "duplicate {}", name(i));
            assert!(
                name(i).starts_with(category(name(i))),
                "no category: {}",
                name(i)
            );
            assert!(!all()[i].data.is_empty(), "empty: {}", name(i));
        }
    }
}
