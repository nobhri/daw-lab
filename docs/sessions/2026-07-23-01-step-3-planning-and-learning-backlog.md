# Step 3 Planning and Learning Backlog

Date: 2026-07-23

## Goal

Decide whether to begin Step 3 despite an incomplete understanding of parts of
the Step 2 playback path, define a small Step 3 implementation plan, and create
a durable way to revisit unresolved learning topics.

This was a planning and documentation session. No Rust implementation or
runtime behavior was changed.

## Starting Point

Steps 0 through 2 were implemented. The latest code-reading session had
improved the high-level understanding of the sample Clock and CPAL callback,
but several expression-level details in `src/playback.rs` still required more
practice.

The previous session suggested completing another playback trace before
starting Step 3. During this session, that recommendation was reconsidered
because the first part of click generation can be implemented as
hardware-independent logic without modifying the CPAL callback.

The repository had milestone tags for Step 0 and Step 1:

```text
step-0-complete
step-1-complete
```

Step 2 did not yet have a milestone tag. Its implementation was merged in
commit `e7fa6b0`.

## Decision: Proceed with Step 3

Step 3 can begin before every Step 2 callback detail is fully understood.
Learning does not need to progress in a strict sequence when the next change
has a clear boundary and remains independently testable.

The initial Step 3 work will focus on:

- converting BPM to a sample-frame interval;
- generating a click from an absolute sample position;
- testing timing and signal generation without audio hardware; and
- rendering a short WAV file for manual listening.

Directly connecting generated click samples to the CPAL callback is outside
this initial scope. That integration is the point at which the unresolved Step
2 callback questions become immediately relevant again.

This separates two learning concerns:

```text
BPM and sample-position math
  -> generated click logic
  -> revisit callback questions
  -> CPAL integration
```

## Step 3 Plan

A dedicated implementation plan was added at
`docs/plans/step-3-generated-click-track.md`.

The plan defines:

- the goal and explicit non-goals;
- fixed initial values for tempo, duration, click frequency, and click length;
- focused unit-test cases;
- a CLI path that renders `output/click_120.wav`;
- required automated and manual validation; and
- learning checkpoints for the timing calculations.

The plan intentionally excludes transport controls, time signatures, tempo
changes, accented beats, and direct CPAL playback. These can be considered only
after the requested Step 3 foundation is complete.

## Problem with Retrospectives Alone

Session retrospectives provide useful chronological history, but unresolved
questions can become difficult to find as more sessions are added. A
retrospective records what happened during a particular session; it is not an
ideal active queue for topics that still need attention.

The project therefore now distinguishes three kinds of documentation:

- the roadmap tracks implementation progress;
- session retrospectives preserve chronological learning history; and
- the learning backlog tracks unresolved concepts by topic.

## Learning Backlog

The new `docs/learning-backlog.md` is a living document. Each topic records:

- whether it is open or completed;
- where it was introduced;
- a return trigger;
- focused questions;
- observable completion evidence;
- related source files and session notes; and
- a milestone tag or commit for the relevant code state.

Open topics do not automatically block unrelated implementation. They should be
reviewed when their return trigger matches upcoming work. Completed topics
remain in the document so that the learning path is preserved.

The initial topic captures the remaining Step 2 questions about:

- mono-to-stereo indexing;
- safe silence after input ends;
- Clock movement across callback buffers; and
- the playback completion condition.

Its return trigger is:

> Before connecting generated click samples to the CPAL output callback.

## Milestone Tags as Learning Anchors

Milestone tags can preserve a simpler implementation state for later study.
They represent the code at the end of a step, not proof that the code was fully
understood at that time.

A tagged version can be inspected without switching the current branch:

```bash
git show step-1-complete:src/playback.rs
```

This makes it possible to compare the simpler historical code with the current
implementation. The learning backlog should reference the relevant tag. If a
tag has not been created yet, it should reference the commit and explicitly
record the pending tag instead of creating one during unrelated work.

For Step 2, the backlog currently records:

```text
merge commit: e7fa6b0
planned tag: step-2-complete
```

No tag was created during this session.

## Workflow Updates

`AGENTS.md` now requires unresolved learning topics to be tracked separately
from retrospectives. It also defines the minimum fields for each topic and the
rule that milestone tags identify implementation states rather than levels of
understanding.

The README links to the learning backlog, and the Step 3 roadmap entry links to
the detailed implementation plan.

## Verification

The documentation changes passed:

```text
git diff --check
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The test suite completed with 11 passing tests and no failures.

## Outcome

Step 3 is ready to begin as a small, hardware-independent learning task.
Incomplete Step 2 knowledge has not been ignored; it has been converted into a
discoverable backlog topic with a concrete return trigger and completion
criteria.

This approach allows forward progress while preserving an explicit route back
to the simpler Step 2 code when CPAL integration makes that knowledge necessary.
