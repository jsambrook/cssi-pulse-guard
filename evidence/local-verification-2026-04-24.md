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

## Commands and Results

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --all --check` | Pass | Formatting clean after applying `cargo fmt --all`. |
| `cargo test --workspace --all-targets` | Pass | 11 tests passed: 6 unit tests and 5 scenario tests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass | No clippy warnings. |
| `cargo kani -p pulse_guard_core` | Not run | Local Cargo installation does not include the `kani` subcommand. |

## Open Verification Gaps

- Install Kani or add a CI job that runs Kani.
- Capture Kani tool version and proof output once available.
- Add adversarial or generated test cases for invalid command sequences and interlock combinations.
