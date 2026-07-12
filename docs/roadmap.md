# MVP roadmap

The MVP is built in small steps. Each step should produce something runnable or
testable.

## Progress

- [x] Step 0: Offline sine WAV generator
- [x] Step 1: WAV playback via CPAL
- [ ] Step 2: Sample-accurate clock
- [ ] Step 3: Generated click track
- [ ] Step 4: Offline mixer
- [ ] Step 5: Live input recording
- [ ] Step 6: Two-track record, playback, and export

## Step 0: Offline sine WAV generator

Generate a one-second, 440 Hz mono 32-bit float WAV file at 44,100 Hz without
using audio hardware. This step introduces Rust project structure, tests, and
WAV writing through `hound`.

Output: `output/sine_440.wav`

## Step 1: WAV playback via CPAL

Play a WAV file through the system audio device. This step introduces hardware
audio output while keeping playback separate from WAV generation logic.

## Step 2: Sample-accurate clock

Introduce a clock that tracks time in samples instead of milliseconds and use
sample time to represent playback position.

## Step 3: Generated click track

Generate a click signal from BPM and sample position. This step introduces
BPM-to-sample math without relying on a pre-rendered click WAV.

## Step 4: Offline mixer

Mix two generated or loaded audio sources sample by sample and export the result
to WAV. This step introduces summing, clipping, and simple gain handling without
hardware I/O.

## Step 5: Live input recording

Record audio input through CPAL. Hardware-dependent input is introduced only
after WAV, clock, and mixer logic exist.

## Step 6: Two-track record, playback, and export

Combine click generation, live input, playback, and mixdown into a minimal CLI
workflow, for example:

```bash
cargo run -- record --bpm 120 --bars 4
```
