# Use CPAL for audio I/O

- Status: Accepted
- Date: 2026-07-08

## Context

The project needs direct access to system audio devices while keeping the audio
engine itself in Rust. Hardware audio is introduced only in steps that require
live playback or recording.

## Decision

Use `cpal` for audio device discovery, stream creation, and audio callbacks.
Keep hardware-independent logic outside the CPAL layer so it can be tested in
CI without an audio device.

## Consequences

The project can learn callback-based audio I/O through a cross-platform Rust
API. Device-dependent behavior still requires manual testing, and platform
build dependencies may differ.
