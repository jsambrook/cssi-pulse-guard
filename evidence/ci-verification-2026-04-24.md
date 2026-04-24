# CI Verification Evidence - 2026-04-24

## Scope

GitHub Actions verification for the therapy-control state-machine baseline with Kani proof execution.

## CI Run

| Field | Value |
|---|---|
| Workflow | CI |
| Event | push |
| Run ID | 24899095207 |
| Run URL | https://github.com/jsambrook/cssi-pulse-guard/actions/runs/24899095207 |
| Commit | `5f19d05f1907cf1772cfb4d8030ff3f4869955f4` |
| Created | 2026-04-24T16:01:02Z |
| Completed | 2026-04-24T16:01:43Z |
| Conclusion | success |

## Jobs

| Job | Result | Evidence |
|---|---|---|
| Rust checks | Pass | Formatting, unit/scenario tests, and clippy completed successfully. |
| Kani proofs | Pass | Kani completed successfully with 5 verified harnesses and 0 harness failures. |

## Rust Check Details

The `Rust checks` job ran:

- `cargo fmt --all --check`
- `cargo test --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`

The test job reported:

- 6 unit tests passed
- 5 scenario tests passed

## Kani Proof Details

The `Kani proofs` job used:

- GitHub Action: `model-checking/kani-github-action@v1`
- Kani version input: `0.66.0`
- Command: `cargo-kani -p pulse_guard_core`
- Runner image: `ubuntu-24.04`
- Rust toolchain observed in log: `rustc 1.95.0 (59807616e 2026-04-14)`

Kani result summary from the job log:

- `0 of 52 failed`
- `VERIFICATION:- SUCCESSFUL`
- `Complete - 5 successfully verified harnesses, 0 failures, 5 total.`

Verified harnesses:

- `inv_001_inhibit_blocks_delivery_request`
- `inv_002_delivery_requires_armed_deliver_command`
- `inv_003_delivery_requires_valid_detection`
- `inv_004_delivery_requires_connection_readiness_and_self_test`
- `inv_005_faulted_never_requests_or_enables_therapy`

## Warnings

GitHub Actions reported a Node.js 20 deprecation warning for actions used by the workflow. The warning did not fail the run. The project should review action versions before GitHub's Node.js 20 removal on 2026-09-16.

## Claim Boundary

This evidence supports a narrow claim: selected one-step safety invariants for the pure Rust therapy-control transition function were checked by Kani in CI for commit `5f19d05f1907cf1772cfb4d8030ff3f4869955f4`.

This evidence does not claim full formal verification, clinical validation, hardware validation, real-time correctness, cybersecurity assurance, or production medical-device readiness.
