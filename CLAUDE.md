# CLAUDE.md

Guidance for **Claude Code** in this repository.

## Canonical instructions

The repository-wide, stable agent rules live in **AGENTS.md**.  
Treat **AGENTS.md as the source of truth** for architecture, invariants, and validation.

## Claude-specific preferences (keep lightweight)

- Prefer a short plan + incremental edits (small diffs).
- When touching logic that affects behavior, add/adjust tests in the same change.
- Avoid speculative refactors; keep changes scoped to the task.

## Handy test invocations

```bash
cargo test
cargo test --no-default-features
cargo test --test preset_integration
cargo test tempo::tests
```
