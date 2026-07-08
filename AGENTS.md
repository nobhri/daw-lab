# AGENTS.md

## Project

daw-lab is a learning project for building a DAW audio engine from scratch in Rust.

The primary goal is learning how audio engines work, not building a production-ready DAW.

## Rules

- Use Rust stable.
- All code, comments, commit messages, logs, and docs must be in English.
- Keep the MVP as a single crate.
- Prefer small, testable changes.
- Implement only the requested step.
- Do not jump ahead.
- Do not add GUI, MIDI, VST/plugin hosting, or project file persistence unless explicitly requested.
- Hardware-dependent audio I/O must not be required in CI.

## Architecture

- Logic layer: Clock, Transport, Mixer, WAV I/O.
- Audio layer: CPAL streams, callbacks, and buffer handling.
- Logic layer must be testable with `cargo test`.
- Audio layer may require manual testing.

## Required checks

Run before finishing:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Git Workflow

- Before making or moving changes, always run `git status --short --branch`.
- Before editing, confirm whether the current branch matches the current task, whether it contains work from a previous session, whether there are existing uncommitted changes, and which files are safe to edit, stage, or commit.
- Use a new feature branch for each independent task or session. Do not continue work on a branch from a previous session unless the user explicitly asks to amend that branch.
- If the current branch is from a previous task, keep current changes safe with `git stash`, switch to latest `main`, pull with `git pull --ff-only origin main`, create a new feature branch, then re-apply the changes.
- Before committing, always run `git status --short --branch` and inspect the diff.
- Before committing, inspect both unstaged and staged changes with `git diff` and `git diff --cached`.
- Stage only files related to the requested task. Do not use broad staging when the worktree contains unrelated changes.
- Use feature branches. Never commit directly to `main`.
- Use `main` as the default PR base unless the user specifies another branch.
- Open draft PRs by default.
- Never force push.
- For phase completion, prefer annotated tags, for example `phase-1-complete`.

## PR Workflow

- PR descriptions should include:
  - what changed
  - why it changed
  - how it was verified
  - any remaining risk or follow-up
- For documentation-only changes, state that no runtime validation was needed.
- For Terraform changes, include the Terraform commands that were run and whether Azure resources were applied or destroyed.
- When using `gh` for PR work, do not treat sandboxed authentication or network
  failures as the final state. GitHub API commands such as `gh auth status`,
  `gh pr list`, `gh pr view`, and `gh pr create` may need normal local
  environment access to read keychain credentials and reach the API. If a
  sandboxed `gh` command reports an invalid token or API connectivity failure,
  retry the same `gh` command with the required permission instead of asking
  the user to re-authenticate first.

  ## Learning Workflow

- Offer code-reading prompts when they help the user understand the next Terraform concept.
- When a real error occurs, capture the error pattern and fix in README or a session retrospective if it is likely to help later.
- At the end of each learning session, offer to create or update a short retrospective under `docs/sessions/`.
- Name retrospective files as `YYYY-MM-DD-NN-topic.md`, where `NN` is a two-digit sequence number for that date, for example `2026-07-08-01-phase-1-networking.md`.
- Do not read all retrospectives by default. Read the latest one only when the user asks to continue from the previous session or when context is unclear.
