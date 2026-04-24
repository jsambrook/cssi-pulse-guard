# Kani Proof Plan

## Initial Target

Crate: `pulse_guard_core`

Proof boundary: the pure `transition(current, inputs)` function and data types in `implementation/pulse_guard_core/src/lib.rs`.

## Planned Command

```sh
cargo kani -p pulse_guard_core
```

If the local environment does not have Kani installed, run conventional Rust tests first:

```sh
cargo test
```

## Review Checklist

- Confirm each proof harness maps to one or more safety invariants.
- Confirm nondeterministic input generation covers every modeled state and command.
- Confirm proof results are recorded in `evidence/` with tool version, command, date, and outcome.
- Confirm failures update either the implementation, requirements, or hazard analysis before being closed.

## Known Limits

- No time model exists yet.
- No hardware abstraction exists yet.
- No clinical detection algorithm exists yet.
- No proof has been run in CI yet.
