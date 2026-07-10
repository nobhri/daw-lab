# Step 1 WAV Playback

Date: 2026-07-10

## Goal

Play a WAV file through the system's default audio output device with `cpal`,
while keeping the hardware-independent playback logic testable in CI.

## What Changed

- Added `cpal` for audio output.
- Added WAV decoding for floating-point and integer PCM samples.
- Added output-device configuration at the WAV file's sample rate.
- Added conversion from WAV samples to the device's output sample format.
- Added channel mapping that copies mono input to every output channel.
- Added silence after the source audio reaches its end.
- Added `generate` and `play` CLI commands.
- Added manual testing instructions for hardware playback.

The default Step 0 output path remains `output/sine_440.wav`, so the basic
workflow is:

```bash
cargo run -- generate
cargo run -- play
```

An explicit WAV path can also be provided:

```bash
cargo run -- play path/to/audio.wav
```

## Architecture Notes

The playback implementation is split between logic that can be tested without
audio hardware and the `cpal` stream setup that requires a real output device.

`read_wav` reads WAV metadata and converts samples into normalized `f32`
values. `write_output_data` copies those values into an output callback buffer,
handles channel mapping, and fills the buffer with silence after playback ends.
The `play_wav` function owns the hardware-dependent work: selecting the default
device, choosing a supported configuration, creating the stream, and waiting
for playback to finish.

The implementation does not resample audio. It requests the WAV file's sample
rate from the output device and returns an error when the device does not
support that rate. This keeps Step 1 focused and avoids adding resampling before
it is requested.

## Dependency Compatibility

The latest `cpal` release initially resolved to transitive dependencies that
required Cargo support for Rust edition 2024. The local stable toolchain is
Cargo 1.77, so that dependency graph could not be parsed.

The fix was to use `cpal` 0.15.3 and lock `coreaudio-sys` to 0.2.15. This keeps
the project compatible with the current stable toolchain without introducing a
toolchain upgrade as part of Step 1.

Error pattern:

```text
feature `edition2024` is required
```

When this appears during dependency resolution on an older stable toolchain,
check both the direct dependency version and newly selected transitive versions.

## Verification

Automated checks:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All checks passed. Five unit tests passed, including tests for copying mono
samples to stereo output and producing silence after the input ends.

Manual verification was completed with:

```bash
cargo run -- generate
cargo run -- play
```

Playback through the system audio device worked successfully.

## Remaining Risk and Follow-up

- Hardware playback cannot be exercised in CI.
- Unsupported device sample rates return an error because resampling is out of
  scope for Step 1.
- The next session will focus separately on reading and discussing the Step 1
  code rather than adding more implementation.
