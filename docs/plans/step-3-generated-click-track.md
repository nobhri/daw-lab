# Step 3 implementation plan: generated click track

## Goal

Generate a click signal from BPM and an absolute sample position without reading
a pre-rendered click WAV file.

Step 3 should keep the timing and signal-generation logic independent of audio
hardware so that it can be tested with `cargo test`.

## Scope

Step 3 will:

- convert BPM into the number of sample frames per beat;
- determine whether a sample position falls within a click;
- generate a short click waveform at each beat;
- render a short mono click track for listening;
- expose the renderer through a small CLI command; and
- test the timing and signal bounds without audio hardware.

Step 3 will not add:

- direct CPAL playback of generated samples;
- transport controls;
- time signatures or accented beats;
- tempo changes;
- GUI, MIDI, or plugin support; or
- project persistence.

Direct CPAL integration should wait until the related Step 2 callback questions
in the [learning backlog](../learning-backlog.md) have been resolved.

## Initial behavior

Use intentionally small fixed defaults:

- sample rate: 44,100 Hz;
- tempo: 120 BPM;
- duration: 4 seconds;
- click frequency: 1,000 Hz;
- click duration: 10 milliseconds; and
- output: `output/click_120.wav`.

At 120 BPM and 44,100 Hz, one beat spans 22,050 sample frames. The first click
starts at sample position zero, and later clicks start at multiples of 22,050.
The click waveform should use a short fade-out envelope and remain within the
floating-point WAV range of `-1.0..=1.0`.

The first implementation may round the samples-per-beat calculation to a whole
sample frame. The rounding rule should be explicit and covered by tests. More
advanced scheduling for fractional intervals is outside this step.

## Planned changes

### 1. Add click-generation logic

Create `src/click.rs` for hardware-independent click timing and sample
generation. Keep the public API small and based on BPM, sample rate, and sample
position.

Reuse the existing WAV-writing function instead of introducing another output
format or writer.

### 2. Add focused tests

Cover at least:

- 120 BPM at 44,100 Hz produces a 22,050-frame beat interval;
- a click begins at sample position zero;
- another click begins at sample position 22,050;
- positions between clicks are silent after the click duration;
- the fade-out reduces the click amplitude toward its end;
- generated samples remain in `-1.0..=1.0`; and
- a four-second render has the expected number of samples.

### 3. Add a listening path

Add a `click` CLI command that renders the fixed initial behavior:

```bash
cargo run -- click
cargo run -- play output/click_120.wav
```

The generated WAV is a manual-inspection artifact, not an input dependency of
the click generator.

### 4. Update user-facing documentation

Document the new command and its output in the README. Mark Step 3 complete in
the roadmap only after the implementation, tests, and listening path are
finished.

## Validation

Run the required checks:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Manually render the click WAV and play it when an audio device is available.
Hardware playback must not be required in CI.

## Learning checkpoints

During implementation, be able to explain:

- why `60 / BPM` gives seconds per beat;
- how seconds per beat becomes sample frames per beat;
- why the generator uses an absolute sample position;
- how the click duration and fade-out are represented in samples; and
- which calculations involve integer rounding.

Questions that remain unresolved should be added to the learning backlog rather
than expanding Step 3 beyond its stated scope.
