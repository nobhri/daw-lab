# Learning backlog

This document tracks concepts that need another focused learning pass. Unlike a
session retrospective, it is a living list of unresolved topics rather than a
chronological record.

## How to use this backlog

Each topic should include:

- a status of `Open` or `Completed`;
- where the topic was introduced;
- a return trigger that identifies when it becomes relevant again;
- focused questions rather than a broad subject name;
- observable completion evidence;
- links to related source files and session notes; and
- a milestone tag or commit that preserves the simplest relevant code state.

Do not block unrelated progress solely because an item is open. Review the
backlog before starting work that matches an item's return trigger. When a topic
is understood, keep it in this file and change its status to `Completed` so the
learning path remains visible.

Milestone tags identify implementation states, not levels of understanding.
When a milestone does not have a tag yet, record its commit and note the pending
tag rather than creating a tag as part of unrelated work.

## Sample clock and playback callback

Status: Open

Introduced in: Step 2

Return trigger: Before connecting generated click samples to the CPAL output
callback.

### Questions

- Trace the mono-to-stereo playback test one expression at a time.
- Explain why `channel.min(input_channels - 1)` duplicates mono input into
  both stereo output channels.
- Explain how `get`, `copied`, and `unwrap_or(0.0)` safely produce silence
  after the input ends.
- Trace the Clock position across two separate output buffers.
- Relate the playback completion condition to the length of interleaved input.

### Completion evidence

- Predict the relevant playback test results before running them.
- Trace one worked buffer example without referring to session notes.
- Explain when the Clock advances and why it counts frames rather than
  interleaved sample values.
- Explain the completion condition using both mono and stereo examples.

### Related material

- `src/clock.rs`
- `src/playback.rs`
- `docs/sessions/2026-07-12-02-step-2-sample-clock.md`
- `docs/sessions/2026-07-13-01-step-2-clock-code-reading.md`
- Step 2 merge commit: `e7fa6b0`
- Planned milestone tag: `step-2-complete` (not created yet)
