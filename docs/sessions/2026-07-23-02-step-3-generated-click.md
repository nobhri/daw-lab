# Step 3 Generated Click Track

Date: 2026-07-23

## Goal

Implement the hardware-independent portion of Step 3: generate a short click
waveform from BPM and an absolute sample position, render it to WAV, and expose
the renderer through the CLI.

## Starting Point

Steps 0 through 2 were complete, and the previous session had defined the Step
3 scope. The worktree also contained an accidental untracked
`docs/sessions/.md` file with only a heading. It was removed before the
implementation began.

The existing Step 2 callback questions remain open, but they did not block this
session because generated click samples were not connected directly to CPAL.

## Implementation

A new hardware-independent `src/click.rs` module now:

- converts BPM to a whole-sample beat interval;
- uses an absolute sample position to locate the current position within a
  beat;
- emits a 1,000 Hz cosine click for the first 10 milliseconds of each beat;
- applies a linear fade-out envelope;
- returns silence for the rest of the beat; and
- renders a fixed four-second click track.

The beat interval calculation is:

```text
sample frames per beat = sample rate * 60 / BPM
```

The result is rounded to the nearest whole sample frame. At 44,100 Hz and 120
BPM, this produces 22,050 frames per beat.

The cosine begins at an amplitude of `1.0`, so the click starts clearly at
sample position zero. Multiplying it by an envelope that falls from `1.0`
toward zero shortens the sound and keeps the generated samples within the
floating-point WAV range.

## CLI and WAV Output

The new command:

```bash
cargo run -- click
```

renders:

```text
output/click_120.wav
```

The file is a four-second, mono, 44,100 Hz floating-point WAV. It can be played
through the existing playback path:

```bash
cargo run -- play output/click_120.wav
```

Manual playback confirmed that the generated click track is audible.

## Tests

The new unit tests cover:

- the 22,050-frame interval at 120 BPM and 44,100 Hz;
- click starts at sample positions zero and 22,050;
- silence between clicks;
- reduced amplitude near the end of the fade-out;
- the `-1.0..=1.0` sample bound; and
- the 176,400-sample length of a four-second render.

The complete suite increased from 11 to 18 tests.

## Documentation

The README documents generation and playback commands. The roadmap marks Step
3 complete. The learning backlog also records that the existing
`step-2-complete` tag is present.

## Verification

The implementation passed:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All 18 tests passed. The generated file was also inspected as mono, 44,100 Hz
WAVE audio and manually heard through the existing playback command.

## Next Learning Session

The next step is a focused code-reading session before Step 4 implementation.
It should trace:

1. the BPM-to-sample interval calculation;
2. modulo-based beat positioning;
3. click duration and the fade-out envelope;
4. the iterator path from absolute sample positions to the rendered vector;
5. the path from the `click` CLI command to the shared WAV writer; and
6. how each unit test demonstrates one behavior.

These questions are tracked as an open topic in
`docs/learning-backlog.md`. Direct CPAL integration remains outside Step 3.

## Outcome

Step 3 is complete as a small, testable logic-layer feature. The project can
generate and play a click track without using a pre-rendered click sample, and
the next session has a defined code-reading path before moving to the offline
mixer.
