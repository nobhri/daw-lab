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

## MVP Build Plan

The MVP is built in small steps. Each step should produce something runnable or testable.

### Step 0: Offline sine WAV generator

Generate a simple WAV file without using audio hardware.

Goal:
- Learn basic Rust project structure, tests, and WAV writing.
- Avoid `cpal` and hardware-dependent audio I/O at the beginning.

Output:
- `output/sine_440.wav`

Scope:
- Generate a 1-second 440 Hz sine wave at 44100 Hz.
- Write it as a mono 32-bit float WAV file using `hound`.
- Add unit tests for sample generation logic.

Out of scope:
- Audio playback
- `cpal`
- Recording
- Mixer
- Transport
- GUI

### Step 1: WAV playback via cpal

Play a WAV file through the system audio device.

Goal:
- Learn hardware audio output through `cpal`.
- Keep playback separate from WAV generation logic.

### Step 2: Sample-accurate Clock

Introduce a Clock that tracks time in samples instead of milliseconds.

Goal:
- Drive playback position from sample time.

### Step 3: Generated click track

Generate a click/metronome signal from BPM and sample position.

Goal:
- Learn BPM-to-sample math.
- Avoid using a pre-rendered click WAV.

### Step 4: Offline mixer

Mix two generated or loaded audio sources sample-by-sample and export to WAV.

Goal:
- Learn summing, clipping, and simple gain handling without hardware I/O.

### Step 5: Live input recording

Record audio input through `cpal`.

Goal:
- Introduce hardware-dependent input only after WAV, Clock, and Mixer logic exist.

### Step 6: Two-track record/playback/export

Combine click, live input, playback, and mixdown into a minimal CLI workflow.

Example:

```bash
cargo run -- record --bpm 120 --bars 4
```