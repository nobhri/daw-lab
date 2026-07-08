# Step 0: Offline Sine WAV Generator

Date: 2026-07-08

## Goal

Implement the first MVP build step: generate a WAV file offline without using audio hardware.

## What Changed

- Added `hound` for WAV writing.
- Added sine-wave sample generation in the logic layer.
- Added a binary entry point that writes `output/sine_440.wav`.
- Generated signal settings:
  - Frequency: 440 Hz
  - Duration: 1 second
  - Sample rate: 44100 Hz
  - Channels: mono
  - WAV sample format: 32-bit float

## Automated Testing

Added unit tests for:

- Expected sample count.
- Amplitude staying within `[-1.0, 1.0]`.
- First sample being approximately zero.

Checks run:

```bash
cargo fmt
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All checks passed.

## Manual Testing Recommendation

Run:

```bash
cargo run
```

Then confirm that `output/sine_440.wav` exists and can be opened or played in an audio player. The expected result is a steady 1-second 440 Hz tone.

Manual result:

- `cargo run` generated the WAV file.
- `afplay output/sine_440.wav` played successfully on macOS.
- The generated 440 Hz tone was audible.

Optional WAV inspection:

- Sample rate should be `44100`.
- Channel count should be `1`.
- Format should be 32-bit float.
- Duration should be about 1 second.

No hardware-dependent testing is required for this step.

## Notes

This step intentionally did not add playback, recording, `cpal`, transport, clock, mixer, GUI, MIDI, plugin hosting, or project persistence.
