# CLAUDE.md

Guidance for **Claude Code** in this repository.

## Canonical instructions

The repository-wide, stable agent rules live in **AGENTS.md**.
Treat **AGENTS.md as the source of truth** for architecture, invariants, and validation.

## Quick start

```bash
cargo build
cargo run --release
cargo fmt          # run before finishing any change
```

## Claude-specific preferences (keep lightweight)

- Small diffs: plan first, then one focused change at a time.
- Always run `cargo fmt` before wrapping up.
- If touching behavior: add or adjust tests in the same commit.
- No speculative refactors; scope changes tightly to the task.
- Must compile in both modes: `cargo test` **and** `cargo test --no-default-features`
  (the second disables rodio and uses the mock audio backend — keeps tests headless).
- `cargo clippy -- -D warnings` must pass in both modes before wrapping up.
- Edition 2024 supports `let_chains`: `if cond && let Pat = expr { }` — use this
  instead of nested `if`/`if let` blocks (clippy enforces it as `collapsible_if`).
- Do not add `pub mod` for stub files until the module is actually implemented;
  empty stubs in the module tree create false impressions and unused-code warnings.
- Mock backend: `open_default_stream()` always returns `Err` — `MetronomeApp::mixer`
  is always `None` in headless tests; test audio-path logic via `engine.rs` directly.

## Handy test invocations

```bash
cargo test
cargo test --no-default-features          # mock audio backend
cargo test --test preset_integration      # JSON save/load round-trip
cargo test --test metronome_flow          # full start/stop flow (mock)
cargo test tempo::tests                   # tap-tempo unit tests
```
