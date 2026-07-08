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