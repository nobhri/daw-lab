# Contribution workflow

## Branches

- Start each independent task or session from the latest `main`.
- Use a dedicated feature branch and never commit directly to `main`.
- Use `main` as the default pull request base.
- Never force push.
- Prefer an annotated tag for a completed phase, for example
  `phase-1-complete`.

## Before committing

Run:

```bash
git status --short --branch
git diff
git diff --cached
```

Stage only files related to the task. Then run the required project checks:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Documentation-only changes do not require runtime validation.

## Pull requests

Open draft pull requests by default. The description should include:

- what changed
- why it changed
- how it was verified
- any remaining risk or follow-up

For documentation-only changes, state that no runtime validation was needed.

If a sandboxed GitHub CLI command reports an invalid token or an API connection
failure, retry it with normal local environment access before concluding that
authentication has failed.
