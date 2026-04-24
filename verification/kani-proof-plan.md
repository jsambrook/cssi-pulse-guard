# Kani Proof Plan

## Initial Target

Crate: `pulse_guard_core`

Proof boundary: the pure `transition(current, inputs)` function and data types in `implementation/pulse_guard_core/src/lib.rs`.

## Planned Command

```sh
cargo kani -p pulse_guard_core
```

The GitHub Actions workflow runs the same proof target using `model-checking/kani-github-action@v1` pinned to Kani `0.66.0`.

If the local environment does not have Kani installed, run conventional Rust tests first:

```sh
cargo test
```

## Review Checklist

- Confirm each proof harness maps to one or more safety invariants.
- Confirm nondeterministic input generation covers every modeled state and command.
- Confirm proof results are recorded in `evidence/` with tool version, command, date, and outcome.
- Confirm failures update either the implementation, requirements, or hazard analysis before being closed.

## Current Evidence

CI evidence for commit `5f19d05f1907cf1772cfb4d8030ff3f4869955f4` is recorded in `evidence/ci-verification-2026-04-24.md`.

The Kani CI job reported:

- `0 of 52 failed`
- `VERIFICATION:- SUCCESSFUL`
- `Complete - 5 successfully verified harnesses, 0 failures, 5 total.`

## Known Limits

- No time model exists yet.
- No hardware abstraction exists yet.
- No clinical detection algorithm exists yet.
- Kani evidence covers only the selected one-step transition-function invariants.
