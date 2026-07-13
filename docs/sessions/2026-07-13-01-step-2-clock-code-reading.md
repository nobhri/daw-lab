# Step 2 Clock Code Reading

Date: 2026-07-13

## Goal

Read the current sample-clock and playback code closely enough to understand
why the Clock exists, how CPAL requests audio, and how WAV samples move into an
output buffer before starting Step 3.

This was a code-reading session only. No Rust implementation was changed, and
Step 3 click generation was not started.

## Starting Understanding

Before this session, the Clock was understood as a prerequisite for keeping a
future click track, multiple playback tracks, and recording on a common
timeline. The remaining uncertainty was how that architectural idea connected
to the current CPAL callback and buffer-writing code.

The earlier single-WAV playback implementation could use a local array index as
an implicit playback position. Step 2 gives that position an explicit meaning:
the current sample-frame position on the audio timeline.

## Clock Responsibility

The current `Clock` stores only:

- the sample rate, which defines how sample positions convert to seconds;
- the current position in sample frames.

It does not measure wall-clock time and does not make CPAL request audio. The
audio device and CPAL determine when output buffers are needed. The Clock moves
as the callback successfully consumes input sample frames.

The Clock is intentionally a small logic-layer concept. Future BPM, click,
transport, mixer, and recording responsibilities do not all need to become
Clock fields. Those components can use a shared sample position while retaining
separate responsibilities.

## CPAL Callback Model

The session used a serving-tray analogy for the output callback:

- CPAL supplies an empty tray: the mutable output buffer;
- each bowl on the tray represents one sample frame;
- a stereo frame has left and right sample values;
- the callback fills the tray using the WAV data and Clock position;
- after the callback returns, CPAL uses the filled buffer for playback;
- CPAL calls the same callback again when another output buffer is needed.

This clarified that the application does not poll the device and decide when to
send individual samples. Instead, it registers a callback in advance and fills
each buffer when CPAL invokes that callback.

The buffer size is not assumed to be 512 frames. That number was used only as
an example. The callback processes however many frames are represented by the
output slice supplied for that invocation.

## Callback, Closure, and Ownership

A callback describes the role of a function that another component calls
later. A closure is the Rust construct used here to implement that callback.
The closure combines executable code with the surrounding state that the code
needs.

The `move` keyword does not move the Clock directly into the `Device`.
Instead, it makes the closure capture the required local values by value. The
closure, containing the audio data and its own Clock state, is then passed to
`device.build_output_stream` as the data callback.

The resulting lifetime relationship is approximately:

```text
play_wav
  -> Stream
    -> callback closure
      -> AudioData
      -> Clock
      -> completion state and sender
```

This allows the same Clock state to survive across repeated callback
invocations after the local `build_stream` call has returned.

The output buffer is different. Its type is `&mut [T]`, so the callback only
borrows CPAL's buffer for the duration of one invocation and may modify its
contents. It does not take ownership of the buffer.

One Rust detail noted during the discussion is that `Clock` currently
implements `Copy`. A move closure therefore captures a copied Clock value,
while non-`Copy` values such as `AudioData` are moved into the closure. The
important runtime fact is that subsequent callback invocations mutate the
Clock stored inside the closure.

## Stream Construction and Runtime Processing

Two different times in the program must remain distinct.

During stream construction:

1. `play_wav` reads the WAV and selects a device configuration.
2. The project-defined generic `build_stream<T>` function is called.
3. `build_stream` creates the Clock and callback closure.
4. The closure is registered through CPAL's
   `device.build_output_stream` method.
5. A `cpal::Stream` is returned.

During playback:

1. `stream.play()` starts the stream.
2. CPAL invokes the registered closure with a mutable output buffer.
3. The closure calls `write_output_data`.
4. `write_output_data` fills the borrowed buffer from the current WAV and
   Clock positions.
5. The closure checks whether playback has finished.
6. CPAL invokes the same closure again when it needs another buffer.

`write_output_data` does not produce a completed buffer first and pass it into
`build_output_stream`. Instead, `build_output_stream` receives the closure in
advance, and that closure calls `write_output_data` later at runtime.

## Samples, Frames, Channels, and Buffers

The code-reading discussion distinguished four terms:

- A sample value is one channel's amplitude at one point in time.
- A sample frame contains the values for every channel at one point in time.
- A buffer contains multiple sample frames.
- The Clock counts sample frames, not individual interleaved channel values.

Stereo WAV data is stored in interleaved order:

```text
L0, R0, L1, R1, L2, R2, ...
```

It is not stored as one nested list for the left channel and another for the
right channel. Therefore, the starting vector index for a Clock position is:

```text
input_start = Clock position * input channel count
```

For example, Clock position 2 in stereo begins at vector index 4, containing
`L2`, followed by `R2` at index 5.

CPAL's output slice is also processed as interleaved frames. Calling
`chunks_mut(output_channels)` divides the flat slice into one mutable slice per
output frame. The Clock advances once after a valid input frame is consumed,
regardless of how many channel values that frame contains.

## Generic Output Sample Type

The `T` in `build_stream<T>` is a generic type parameter rather than a simple
alias. A call such as `build_stream::<f32>` makes `T` equal to `f32`, while
`build_stream::<i16>` makes it equal to `i16`.

The output callback consequently receives `&mut [T]`, matching the sample
format supported by the selected device configuration. The trait bounds ensure
that CPAL can use `T` as a sized sample type and that the decoded `f32` WAV
values can be converted into it.

This topic has been introduced but does not need to be mastered before Step 3.

## Worked Data Trace

The following example helped connect Clock positions to interleaved vector
indices:

```text
input              = [0.1, 0.2, 0.3, 0.4]
input channels      = 2
output channels     = 2
starting Clock      = 1
output              = [0.0, 0.0, 0.0, 0.0]
```

The input contains two stereo frames:

```text
frame 0 = [L: 0.1, R: 0.2]
frame 1 = [L: 0.3, R: 0.4]
```

At Clock position 1, `input_start` is `1 * 2 = 2`. The first output frame is
therefore filled with `[0.3, 0.4]`, and the Clock advances to 2.

The next `input_start` is `2 * 2 = 4`, which is at the end of the input vector.
Safe `get` calls return no value, and `unwrap_or(0.0)` fills the second output
frame with silence. The Clock does not advance for that frame because no input
frame was consumed.

The final state is:

```text
output = [0.3, 0.4, 0.0, 0.0]
Clock  = 2
```

## Current Understanding

The architectural motivation is now reasonably clear:

- a single WAV can be played using an implicit array cursor;
- a DAW needs an explicit sample-frame timeline that multiple components can
  share;
- CPAL controls when output work is requested;
- a registered closure retains the WAV data and Clock between requests;
- each callback invocation fills a borrowed buffer frame by frame;
- the Clock position selects the current input frame.

The remaining gap is not primarily the high-level design. It is gaining enough
fluency to simulate the nested `write_output_data` loops directly from the Rust
expressions without relying on an explanation.

## Before Step 3

One more focused code-reading pass should cover:

1. Trace the mono-to-stereo playback test one expression at a time.
2. Explain why `channel.min(input_channels - 1)` duplicates mono input into
   both stereo output channels.
3. Explain how `get`, `copied`, and `unwrap_or(0.0)` produce silence safely
   after the input ends.
4. Trace the Clock across two separate output buffers.
5. Relate the callback completion condition to interleaved input length.

Once those points can be predicted from the code, Step 3 can introduce BPM to
sample-interval calculations without carrying unresolved confusion about the
existing playback path.
