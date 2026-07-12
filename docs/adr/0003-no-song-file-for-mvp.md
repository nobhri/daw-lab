# Exclude project-file persistence from the MVP

- Status: Accepted
- Date: 2026-07-08

## Context

Saving and loading DAW projects introduces a data model, serialization format,
versioning, and compatibility concerns. These do not help answer the MVP's main
questions about clocks, audio callbacks, mixing, and recording.

## Decision

Do not implement a song or project file format during the MVP. Use explicit CLI
inputs and WAV files for the workflows built in the planned steps.

## Consequences

The MVP remains focused on the audio engine and cannot resume an editable
session from a saved project. Persistence can be reconsidered after the core
record, playback, and export workflow exists.
