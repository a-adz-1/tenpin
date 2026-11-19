//! Step timing. Positions are in sixteenth notes (beats * 4).

/// Ticks per step: divisible by every roll count (1-4 hits per step).
pub const TICKS: i64 = 12;

/// Where step `k` starts: odd steps are pushed late by up to half a step at full shuffle.
pub fn fire_time(k: i64, shuffle: f64) -> f64 {
    k as f64 + if k & 1 == 1 { shuffle * 0.5 } else { 0.0 }
}

/// Continuous position in ticks: step k starts at k * TICKS, and each step's ticks are spread
/// evenly over its length, which shuffle stretches or squeezes, so rolls stay inside their step.
fn ticks(x: f64, shuffle: f64) -> f64 {
    let mut j = x.floor() as i64;
    if fire_time(j, shuffle) > x {
        j -= 1;
    }
    let start = fire_time(j, shuffle);
    (j as f64 + (x - start) / (fire_time(j + 1, shuffle) - start)) * TICKS as f64
}

/// Roll value for 16th-note triplets: a hit every 2/3 of a step on a grid that runs across steps,
/// so two triplet steps in a row give three even hits.
pub const TRIPLET: u8 = 5;

/// Whether a step with this roll value plays at tick `tick` of absolute step `k`.
pub fn plays(roll: u8, k: i64, tick: i64) -> bool {
    if roll == TRIPLET {
        (k * TICKS + tick).rem_euclid(TICKS * 2 / 3) == 0
    } else {
        tick % (TICKS / roll.clamp(1, 4) as i64) == 0
    }
}

/// The (step, tick) that starts in the interval (prev, now], if any.
pub fn fired(prev: f64, now: f64, shuffle: f64) -> Option<(i64, i64)> {
    let n = ticks(now, shuffle).floor() as i64;
    ((ticks(prev, shuffle).floor() as i64) < n).then(|| (n.div_euclid(TICKS), n.rem_euclid(TICKS)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Walk a timeline in small increments and collect (step, tick, time).
    fn walk(shuffle: f64) -> Vec<(i64, i64, f64)> {
        let (mut prev, mut out) = (-1e-6, vec![]);
        for i in 0..=40000 {
            let now = i as f64 / 10000.0;
            if let Some((k, tick)) = fired(prev, now, shuffle) {
                out.push((k, tick, now));
            }
            prev = now;
        }
        out
    }

    #[test]
    fn every_tick_fires_once_in_order() {
        for shuffle in [0.0, 0.6, 1.0] {
            let got = walk(shuffle);
            assert_eq!(
                got.len(),
                4 * TICKS as usize + 1,
                "shuffle {shuffle}: {:?}",
                got.iter().map(|g| (g.0, g.1)).collect::<Vec<_>>()
            );
            for (n, &(k, tick, _)) in got.iter().enumerate() {
                assert_eq!((k, tick), (n as i64 / TICKS, n as i64 % TICKS));
            }
        }
    }

    #[test]
    fn shuffle_delays_odd_steps_and_rolls_fit_inside() {
        let got = walk(1.0);
        let at = |k, tick| got.iter().find(|g| (g.0, g.1) == (k, tick)).unwrap().2;
        assert!((at(1, 0) - 1.5).abs() < 1e-3 && (at(2, 0) - 2.0).abs() < 1e-3);
        // A 2-hit roll on the shortened odd step lands halfway through it.
        assert!((at(1, TICKS / 2) - 1.75).abs() < 1e-3);
    }

    #[test]
    fn two_triplet_steps_play_three_even_hits() {
        let hits: Vec<i64> = (2..4)
            .flat_map(|k| (0..TICKS).filter(move |&t| plays(TRIPLET, k, t)).map(move |t| k * TICKS + t))
            .collect();
        assert_eq!(hits, vec![24, 32, 40]);
        assert_eq!((0..TICKS).filter(|&t| plays(3, 0, t)).count(), 3);
    }
}
