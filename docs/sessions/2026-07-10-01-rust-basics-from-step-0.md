# Rust Basics From Step 0

Date: 2026-07-10

## Goal

Review the Step 0 offline sine WAV generator as a Rust learning exercise.

The session focused on understanding the project structure, Rust syntax, test
layout, and the audio concepts already present in the code. No implementation
changes were requested.

## Code Read

Files discussed:

- `Cargo.toml`
- `src/main.rs`
- `src/lib.rs`
- `docs/sessions/2026-07-08-01-step-0-offline-sine.md`

The current project is a small Rust package with:

- `src/main.rs` as the binary entry point.
- `src/lib.rs` as the library crate containing testable logic.
- `hound` as the WAV read/write dependency.

## Rust Syntax Notes

`#[...]` syntax is not a comment. It is an attribute that attaches metadata or
behavior to an item. For example, `#[test]` marks a function as a test, and
`#[cfg(test)]` compiles a module only when running tests.

Rust comments use:

- `//` for line comments.
- `/* ... */` for block comments.
- `///` for documentation comments on items.
- `//!` for documentation comments on a module or crate.

`use` imports are used to bring names into scope so code can use shorter paths.
For example, `use std::path::Path;` allows `Path` instead of
`std::path::Path`. It is not required for every type because common types such
as `Vec`, `Option`, and `Result` are available through Rust's prelude.

## Project Structure Notes

`Cargo.toml` is the project manifest. It defines the package metadata and
dependencies, similar in role to Python project configuration files.

`src/main.rs` calls the public function exposed by the library crate:

```rust
fn main() -> hound::Result<()> {
    daw_lab::write_default_sine_wav("output/sine_440.wav")
}
```

`src/lib.rs` contains the core logic:

- Constants for sample rate, frequency, and duration.
- `generate_sine_wave`, which returns a `Vec<f32>`.
- `write_float_wav`, which writes samples to a WAV file with `hound`.
- `write_default_sine_wav`, which connects default sine generation to WAV
  writing.

This split keeps the executable entry point small and makes the logic layer easy
to test.

## Audio Concept Notes

`generate_sine_wave` returns `Vec<f32>`, which is closer to Python's
`list[float]` than to a dictionary. Each element is one audio sample value.

The sample vector only stores amplitude values. The position of each value on
the time axis is determined by the sample rate:

- `samples[0]` is at `0 / sample_rate` seconds.
- `samples[1]` is at `1 / sample_rate` seconds.
- `samples[n]` is at `n / sample_rate` seconds.

The WAV file combines the sample data with metadata such as sample rate,
channel count, bit depth, and sample format.

Changing `frequency_hz` changes pitch:

- Doubling frequency raises the pitch by one octave.
- Halving frequency lowers the pitch by one octave.

The current architecture can later support effects by transforming the sample
vector before writing or playback. For example, a distortion function could
modify the amplitude values before passing them to `write_float_wav`. Frequency
analysis with FFT could support spectrum analysis, while EQ could be built with
frequency-domain processing or time-domain filters such as IIR or FIR filters.

## Iterator Notes

The sine generator uses an iterator chain:

```rust
(0..sample_count)
    .map(|sample_index| {
        let time_seconds = sample_index as f32 / sample_rate as f32;
        (TAU * frequency_hz * time_seconds).sin()
    })
    .collect()
```

This is equivalent in meaning to a Python loop that appends each computed sample
to a list:

```python
samples = []

for sample_index in range(sample_count):
    time_seconds = sample_index / sample_rate
    sample = math.sin(TAU * frequency_hz * time_seconds)
    samples.append(sample)
```

`collect()` gathers the iterator output into the function's return type,
`Vec<f32>`.

## Test Notes

The current tests live in `src/lib.rs` under a `#[cfg(test)] mod tests` module.
This is a standard Rust unit test pattern, not just something that happens to
work.

The existing tests verify:

- The generated sample count is `44_100` for 1 second at 44.1 kHz.
- Every generated sample stays within the sine wave amplitude range
  `[-1.0, 1.0]`.
- The first sample is approximately zero because `sin(0) = 0`.

Rust also supports integration tests in a top-level `tests/` directory. That is
useful when testing the crate from the outside through its public API, CLI
behavior, or file I/O behavior.

For the current Step 0 code, colocated unit tests are a good fit because the
logic is small, pure, and easy to test directly.

## Possible Future Tests

Additional tests that could be useful later:

- Different durations produce the expected sample counts.
- Different sample rates produce the expected sample counts.
- `frequency_hz = 0.0` produces silence.
- `duration_seconds = 0.0` produces an empty vector.
- Generated sample values match known expected values within a tolerance.
- `write_float_wav` creates parent directories.
- A written WAV can be read back with the expected sample rate, channel count,
  sample format, and sample count.
- `write_default_sine_wav` creates a valid default WAV in a temporary path.

Hardware-dependent playback tests should remain out of CI.

## Outcome

The session clarified the relationship between Rust syntax, project structure,
unit testing, and basic digital audio representation. No source code behavior
changed.
