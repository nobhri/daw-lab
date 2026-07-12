# Keep the MVP in a single crate

- Status: Accepted
- Date: 2026-07-08

## Context

The project is intended to teach audio-engine fundamentals one small step at a
time. Multiple crates would add workspace structure before the code has clear
boundaries that need independent packaging.

## Decision

Keep the MVP as one Rust crate. Separate logic and hardware audio by modules
and dependencies rather than by crate boundaries.

## Consequences

The codebase stays easy to navigate and test during the MVP. A workspace can be
introduced later if concrete ownership, dependency, or build-time needs emerge.
