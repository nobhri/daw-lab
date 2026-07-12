# Step 2: Sample-Accurate Clock

Date: 2026-07-12

## Goal

Introduce a hardware-independent Clock that represents the current audio
timeline position in samples, then use it as the playback position in the
existing CPAL output path.

The scope was limited to Step 2. BPM calculations, click generation,
transport controls, and other Step 3 or later features were not added.

## Starting Point

Steps 0 and 1 were complete. WAV playback already tracked its position with a
local `frame_index`, but that integer did not express its role as part of the
audio engine's timeline.

Before Step 2 work began, the current Step 1 commit was marked with the
annotated `step-1-complete` tag and the tag was pushed to `origin`.

The Step 2 work was started from the latest `main` on the dedicated
`codex-step-2-sample-clock` branch.

## Clock Model

The Clock in this project is not the CPU clock, an OS timer, or a component
that periodically pushes audio to the device. CPAL and the audio device drive
real-time processing by requesting output buffers.

The Clock has a narrower responsibility:

```text
processed sample frames + sample rate
  -> exact position on the audio timeline
  -> position in seconds when needed
```

It stores:

- the fixed sample rate as `u32`;
- the current sample position as `u64`.

It can advance by an integer number of samples and convert its current sample
position to seconds as `f64`.

For interleaved multichannel audio, the current implementation advances once
per sample frame, not once per channel value. One stereo frame contains two
stored values, but occupies only one position on the audio timeline.

At 44,100 Hz:

```text
position 0       -> 0 seconds
position 22,050  -> 0.5 seconds
position 44,100  -> 1 second
```

## Implementation

A public `clock` module was added to the library. Its `Clock` type owns the
sample rate and current position while exposing read-only accessors and an
`advance` operation.

The playback callback now creates a Clock with the WAV sample rate and passes
it to `write_output_data`. Each successfully consumed input frame advances the
Clock by one. The same Clock is captured by the CPAL callback, so its position
continues across repeated output-buffer requests.

The Clock stops advancing after the source data ends. The existing playback
completion check now derives its input position from the Clock, and remaining
output frames are filled with silence as before.

No audio-device access is required to test the Clock itself.

## Tests and Verification

The Clock tests cover:

- starting at sample zero;
- advancing by one sample;
- advancing across multiple operations;
- converting one second at 44,100 Hz;
- converting a fractional second at 48,000 Hz.

Playback tests cover:

- copying mono input to each stereo output channel;
- writing silence after the input ends;
- retaining Clock position across two output buffers.

All required automated checks passed:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The test suite completed with 11 passing tests and no failures.

Manual playback testing also passed. The generated WAV played successfully
through the audio device after the Clock integration, confirming that the
existing playback path still works on hardware.

## CI Compatibility Review

The CI workflow was reviewed explicitly before finishing because Step 1 had
introduced a Linux build dependency that was available locally but initially
missing from GitHub Actions. CPAL requires the ALSA development package to
compile on Linux, even when CI does not open an audio device. The current
workflow already installs `libasound2-dev` for that reason.

Step 2 does not require a CI change:

- `Cargo.toml` and `Cargo.lock` have no Step 2 changes;
- no new system package, OS API, Rust component, feature flag, or environment
  variable was introduced;
- the Clock is hardware-independent Rust logic;
- the playback tests exercise buffer handling without constructing a CPAL
  device or stream;
- CI already runs the same formatting, Clippy, and test commands required by
  the project.

Manual audio playback remains outside CI by design. The existing Ubuntu job
can compile the CPAL layer and run all Step 2 tests without audio hardware.

## Learning Focus for the Next Session

The implementation has been completed, but a detailed code-reading session is
still planned. Useful questions to explore are:

1. Why are the Clock fields private while their values have public accessors?
2. Why is the sample position stored as `u64` while seconds are returned as
   `f64`?
3. Why does a stereo output frame advance the Clock by one rather than two?
4. Why does the Clock retain its position between CPAL callback invocations?
5. Why is `input_channels` multiplied by the Clock position when indexing the
   interleaved input vector and checking for completion?
6. What is the distinction between a sample value, a sample frame, a CPAL
   buffer, sample rate, and musical tempo?

A useful trace is mono input `[0.25, -0.5]` rendered to stereo in two separate
callback-sized buffers. The expected Clock positions are zero before the
first buffer, one after the first buffer, and two after the second buffer.

## Outcome

Step 2 made playback position an explicit logic-layer concept instead of an
anonymous callback index. The audio timeline can now be expressed in exact
integer sample positions and converted to seconds without using wall-clock
time.

This provides the timing foundation for Step 3, where BPM can be converted to
sample intervals and click events can be scheduled at exact positions. Step 3
should begin only after the current Clock code has been read and the remaining
questions have been discussed in a separate session.
