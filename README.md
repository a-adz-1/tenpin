# Tenpin

A ten-channel sample drum machine with a step sequencer. Runs as a **VST3** or **CLAP** plugin
in any DAW, or on its own as a **standalone app**.

## Features

- Ten channels, each with its own sample, level, pan, pitch, decay, tone and start point, plus
  mute, solo and choke (choke channels cut each other off, like a closed hat stopping an open one)
- A 16-step sequencer with four patterns (A–D), three velocities per step, shuffle and 1–16 steps
- Per-step **rolls** (1–4 hits inside a step, for hi-hat rolls) and per-step **pitch**
- A built-in sample library and synthesized kit, or load your own WAVs
- Follows the DAW's transport, or press **Run** to play on its own clock
- Every knob and button is a DAW parameter you can automate, including each channel's sample
  and the current pattern (automate the pattern to arrange a song)
- MIDI notes C1–A1 (36–45) play channels 1–10
- **Save / Load** `.tenpin` files: jam in the standalone app, then load the same file in your DAW
- Demo beats: rock, trance, boom bap, trap rolls, house, drum & bass, funk

## Building

Needs Rust (stable). On Linux you also need the ALSA, JACK, X11 and GL development headers.

```bash
cargo xtask bundle tenpin --release
```

This writes `tenpin.vst3`, `tenpin.clap` and the standalone `tenpin` binary to `target/bundled/`.
On Linux, copy the plugins to `~/.vst3/` and `~/.clap/`.

```bash
cargo test
```

## Using it

- Drag knobs up and down (hold Shift for fine control), scroll over them, or double-click to reset
- Click a channel's sample name to pick from the library or load a WAV
- **Select** a channel, then edit its steps along the bottom:
  - **VEL**: click steps on and off at the chosen dynamic (soft, medium, hard)
  - **ROLL**: click a step to cycle 1–4 hits inside it
  - **PITCH**: drag a step up or down to retune that hit
  - Right-click a step to reset it
- **Hide knobs** swaps the channel strips for a grid of every channel's steps

## Licence

Tenpin is free software under the GNU General Public License v3.0 or later (see `LICENSE`).
It's built with [nih-plug](https://github.com/robbert-vdh/nih-plug), whose VST3 bindings are
GPLv3. Bundled samples are public domain or CC0 unless `samples/SOURCES.md` says otherwise.
