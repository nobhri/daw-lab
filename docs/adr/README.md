# Architecture decision records

Architecture decision records (ADRs) preserve decisions that affect the shape
or scope of the project, including why each choice was made and its tradeoffs.

Create an ADR when a decision is both significant and difficult to infer from
the code. Do not create placeholder ADRs for decisions that have not been made.
Use the next four-digit sequence number and one of these statuses: Proposed,
Accepted, Superseded, or Rejected.

## Decisions

- [0001: Use CPAL for audio I/O](0001-use-cpal-for-audio-io.md)
- [0002: Keep the MVP in a single crate](0002-single-crate-for-mvp.md)
- [0003: Exclude project-file persistence from the MVP](0003-no-song-file-for-mvp.md)
