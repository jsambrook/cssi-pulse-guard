# Local Verification Evidence - 2026-04-24

## Scope

Local verification for the first therapy-control state-machine baseline.

## Artifact Version

Working tree after creation of:

- hazard and safety-invariant baseline
- software requirements baseline
- therapy state-machine design
- `pulse_guard_core` Rust crate
- initial Kani harnesses under `cfg(kani)`
- GitHub Actions CI skeleton
- scenario test strategy and command-sequence scenario tests
- generated adversarial test strategy and invalid-command/interlock matrix tests
- containerized Rust verification baseline and runbook

## Commands and Results

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --all --check` | Pass | Formatting clean after applying `cargo fmt --all`. |
| `cargo test --workspace --all-targets` | Pass | 14 tests passed: 6 unit tests, 5 scenario tests, and 3 adversarial/generated tests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass | No clippy warnings. |
| `cargo kani -p pulse_guard_core` | Not run | Local Cargo installation does not include the `kani` subcommand. |

## Open Verification Gaps

- CI proof evidence is now recorded in `ci-verification-2026-04-24.md`.
- Install Kani locally only if local proof execution is needed in addition to CI.
- Extend generated testing from one-step matrices to multi-step adversarial command sequences.
- Execute the Docker container path and record container evidence once a local container engine is available.
