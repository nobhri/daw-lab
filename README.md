# daw-lab

A learning project for building a DAW audio engine from scratch in Rust.

The primary goal is to understand how DAW audio engines work at a fundamental level by building one piece at a time. This project is not intended to become a production-ready DAW.

## MVP direction

- CLI-only
- WAV generation and playback
- Sample-based clock
- Generated click track
- Two-track recording
- Mixdown to WAV

## Tech stack

- Rust stable
- `hound` for WAV I/O
- `cpal` for audio I/O
- GitHub Actions for CI

## Development principle

Defer complexity until the pain is real.

## Current status

Steps 0 through 3 are complete. Step 4, an offline mixer, is next.

See the [MVP roadmap](docs/roadmap.md) for the full build plan.

## Quick start

Generate the default sine-wave WAV file:

```bash
cargo run -- generate
```

Play it through the default audio output device:

```bash
cargo run -- play
```

Generate the default 120 BPM click track:

```bash
cargo run -- click
```

This writes a four-second mono WAV file to `output/click_120.wav`. Listen to it
with:

```bash
cargo run -- play output/click_120.wav
```

Run the hardware-independent test suite:

```bash
cargo test
```

Audio playback requires a system audio device. See the
[manual testing guide](docs/dev/manual-testing.md) for details.

## Documentation

- [MVP roadmap](docs/roadmap.md)
- [Learning backlog](docs/learning-backlog.md)
- [Architecture decisions](docs/adr/README.md)
- [Contribution workflow](docs/dev/contributing.md)
- [Learning session notes](docs/sessions/)
