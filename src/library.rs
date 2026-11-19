//! The sample library: the built-in synthesized kit plus the WAVs in `samples/`, embedded in the
//! plugin so it works without any files next to it.
use crate::kit::{self, Sample};
use std::sync::{Arc, OnceLock};

/// Embedded samples. Channels save their sample as an index into the library, so only ever append
/// to this list: inserting would change the sound of saved projects.
const WAVS: &[(&str, &[u8])] = &[
    // @@WAVS@@
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
