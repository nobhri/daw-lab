# Rust and Audio Foundations From Step 1

Date: 2026-07-12

## Goal

Read the Step 1 WAV playback implementation and build a conceptual model of
the Rust syntax, project structure, and audio pipeline behind it.

This was a code-reading session. No runtime behavior or source code was
changed.

## Questions Covered

The session focused on four groups of questions:

1. When to use `match` instead of `if`.
2. How a WAV file reaches the speakers through a CPAL stream.
3. Where CPAL sits among low- and high-level audio libraries.
4. How Rust crates, modules, files, namespaces, `mod`, `use`, and `pub` fit
   together.

## `match`, `if`, and `if let`

`if` selects behavior from boolean conditions. It is a good fit for questions
such as whether a sample buffer is empty or whether a value exceeds a limit.

`match` compares one value against a complete set of patterns. The Step 1 CLI
matches an `Option<&str>` with these cases:

- no command or `generate`
- `play`
- any unknown command

This makes `match` a natural choice because the code is classifying every
possible shape of one value, not merely checking one boolean condition. Rust
also checks that the match is exhaustive.

`if let` is useful when only one pattern matters. For example,
`if let Some(parent) = path.parent()` performs work only when a parent path is
present and deliberately does nothing for `None`.

## WAV Playback Pipeline

The Step 1 playback path can be summarized as follows:

```text
WAV file
  -> hound parses the WAV container and reads PCM samples
  -> AudioData stores normalized samples in Vec<f32>
  -> the CPAL stream callback copies a short section into output: &mut [T]
  -> CPAL passes the output through the OS audio API
  -> the audio driver sends it to the audio device
  -> the DAC converts sample values into an analog electrical signal
  -> the amplifier and speaker turn the signal into air pressure changes
```

For ordinary PCM WAV files, calling the operation "decoding" is acceptable,
but "parsing the WAV container and reading PCM samples" describes the work more
precisely than it does for compressed formats such as MP3 or AAC.

The important distinction is between stored audio and the real-time output
path:

```text
Vec<f32>
  complete WAV sample data owned by the application
       |
       | frame_index selects the next frames
       v
CPAL output callback
  temporarily borrows output: &mut [T] for a short time slice
       |
       v
OS and device buffers
  are consumed continuously at the configured sample rate
```

A CPAL `Stream` is not the audio data. It connects a device configuration and
callbacks to an OS audio stream. `build_output_stream` prepares that connection
and registers the callbacks. `stream.play()` changes the stream to the running
state, after which the audio backend repeatedly requests more output data.

The callback must meet a recurring deadline. Slow file access, blocking locks,
unbounded allocation, and heavy logging should therefore be kept out of a
real-time callback. Step 1 avoids file access there by loading the WAV into
memory before starting playback.

The current completion message means the final source frame has been written
to a callback buffer. It does not prove that the DAC has physically finished
playing every buffered sample.

## Audio Library Layers

CPAL is low-level within the Rust audio ecosystem, but it is not the bottom
layer. It provides cross-platform device and PCM stream access above native OS
APIs:

```text
Application or custom audio engine
  -> rodio or Kira, when a higher-level engine is wanted
  -> CPAL or SDL2 Audio
  -> Core Audio, WASAPI, ALSA, JACK, or another native backend
  -> driver
  -> audio device and DAC
```

Going below CPAL generally means using different APIs and implementation paths
for macOS, Windows, and Linux. CPAL is therefore a useful boundary for a custom
DAW engine: it exposes output buffers while hiding most platform-specific
device code.

The libraries discussed have different responsibilities:

- CPAL provides device discovery, stream configuration, and PCM callbacks. It
  does not provide a high-level `play_wav` API.
- rodio builds convenient decoding, sources, queues, mixing, and playback on
  top of a lower-level output layer such as CPAL.
- Kira provides game-oriented sound management, tracks, effects, clocks,
  transitions, and spatial audio, with CPAL available as its default output
  backend.
- SDL2 Audio provides callback or queued PCM audio as part of the wider SDL
  multimedia stack. It is broadly parallel to CPAL rather than a layer above
  it.
- hound reads and writes WAV data; Symphonia decodes multiple audio formats.
  These handle file formats rather than being the final device-output layer.

For simple file playback, rodio would usually require less application code.
DAW-Lab uses CPAL because the buffer callback, timing, format conversion, and
eventual mixer design are themselves part of the subject being learned.

## Rust Package and Module Model

The current Cargo package contains a library target and a binary target. In
strict Rust terminology, these are separate crates even though the project is
kept in one package:

```text
Cargo package: daw-lab
  |
  +-- library crate: daw_lab
  |     src/lib.rs                 crate root
  |       +-- WAV generation functions
  |       +-- pub mod playback
  |             src/playback.rs
  |               +-- pub read_wav
  |               +-- pub play_wav
  |               +-- private build_stream
  |               +-- private write_output_data
  |
  +-- binary crate: daw-lab
        src/main.rs                crate root
          +-- main
```

The core rules are:

- A crate is a compilation unit and has a root module.
- A module is a namespace within a crate.
- A file is a physical place to store code; it does not automatically become a
  module merely by existing.
- `mod playback;` adds the `playback` module to the module tree and tells Rust
  to load its implementation from the corresponding file.
- `use` brings an existing path into the current scope under a convenient
  name. It does not create or load a module.
- `pub` makes an item reachable from an allowed outer scope. Every module in a
  public path must also be reachable.
- `crate::foo::bar` starts at the current crate root and follows the module
  path to `bar`.

Because `main.rs` and `lib.rs` are different crate roots, `crate::` inside
`main.rs` refers to the binary crate, not the library. The binary therefore
calls the library as `daw_lab::write_default_sine_wav` and
`daw_lab::playback::play_wav`.

The difference between these paths comes from the module tree, not directly
from whether the implementation happens to be in another file. The generation
function is defined at the library root, while `play_wav` is defined inside the
`playback` module.

`use crate::foo::bar; bar()` and `crate::foo::bar()` resolve to the same item.
The first favors brevity when a name is used repeatedly; the second keeps its
origin visible. `use` can also bring traits into scope, which is why CPAL's
`HostTrait`, `DeviceTrait`, and `StreamTrait` make methods such as
`default_output_device`, `build_output_stream`, and `play` available.

## What Belongs in `lib.rs`

There is no language rule that a particular kind of function must be written
in `lib.rs`. A useful convention is:

- treat `lib.rs` as the crate's root, public entrance, and module map;
- keep small central logic there while the crate is small;
- move a cohesive group of types, functions, internal helpers, and tests into a
  named module when that grouping improves navigation or creates a useful API
  boundary.

The current layout reflects the learning history. The small Step 0 generation
logic remains in `lib.rs`, while the larger Step 1 playback responsibility is
grouped in `playback.rs`. A future `wav` module could become reasonable if the
WAV responsibilities grow, but splitting it now would add structure without
solving a current problem.

Modules should usually represent concepts such as `clock`, `transport`,
`mixer`, `wav`, or `audio`, rather than creating one file per small function.

## Understanding Review

The questions in this session show a good emerging system-level mental model.
In particular, the following ideas are already understood well enough to build
on:

- a WAV file ultimately becomes a sequence of PCM sample values;
- `Vec<f32>` is stored source data, while a CPAL stream is the mechanism that
  feeds short buffers over time;
- CPAL sits above OS-specific APIs and below convenient playback engines;
- choosing CPAL for this project is about controlling and learning the engine
  boundary, not merely making file playback harder;
- `match` is being recognized as value-and-pattern classification rather than
  just an alternative spelling of `if`;
- source files are being connected to architecture and responsibility, rather
  than viewed as arbitrary code containers.

Current Rust fluency is still at a beginner-to-early-intermediate stage. The
syntax can be followed with explanation, but module paths, crate boundaries,
visibility, traits in scope, ownership captured by `move`, and generic bounds
are not yet likely to be recalled or designed independently. That is expected
at this stage and is not a reason to delay the project.

Audio and architecture understanding is ahead of Rust syntax fluency. The
questions consistently connect implementation details to the full system:
buffers, OS APIs, drivers, the DAC, library layers, and module responsibility.
That is a strong fit for the project's learning goal.

The main concepts that remain useful to reinforce are:

- sample versus frame, especially for interleaved multichannel audio;
- stored source data versus short-lived output buffers;
- `build_output_stream` versus `play`, and callback-driven execution;
- callback ownership and why `move` keeps `AudioData` alive;
- crate roots versus child modules and public API paths;
- real-time safety and why ordinary application techniques can be unsafe in an
  audio callback.

These are refinement gaps, not Step 1 blockers.

## Recommended Next Step

Proceed to Step 2, the sample-accurate Clock. No additional Step 1 feature or
refactor is recommended first.

Step 1 has already achieved its intended outcome: it reads PCM WAV data,
creates a CPAL output stream, fills callback buffers, handles simple channel
mapping, passes automated tests, and has been manually verified on hardware.
Adding resampling, a higher-level playback library, more device selection, or a
module reorganization now would jump ahead or add breadth without strengthening
the next core concept.

Step 2 is also the right learning continuation because it makes the existing
`frame_index` idea explicit. A sample-based Clock will connect several concepts
already discussed:

```text
sample rate + processed frame count
  -> exact audio time
  -> playback position
  -> later metronome scheduling and transport behavior
```

Before or during Step 2, three short experiments are recommended. They are
learning exercises, not prerequisite production changes:

1. Manually trace `write_output_data` for mono input `[0.25, -0.5]`, stereo
   output, and two callback invocations. Record how `frame_index`, samples, and
   frames differ.
2. Predict the `match` branch for no command, `generate`, `play`, and an unknown
   command before running each CLI form.
3. Draw the two crate roots and resolve these paths without looking at the
   source: `daw_lab::write_default_sine_wav`,
   `daw_lab::playback::play_wav`, and a hypothetical
   `crate::playback::read_wav` written inside the library crate.

If those traces can be explained in the learner's own words, even without
remembering every Rust keyword, the foundation is sufficient. The next session
should begin Step 2 in a small, logic-only, test-first change and revisit the
relevant Rust syntax as it appears rather than attempting to master all module,
trait, and ownership rules in advance.

## Outcome

The session established a coherent model connecting Rust control flow and
module organization to the complete audio playback path. The recommended path
is to advance to Step 2, using its Clock implementation to reinforce sample,
frame, time, module, and test concepts through concrete code.
