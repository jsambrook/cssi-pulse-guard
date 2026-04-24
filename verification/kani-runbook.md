# Kani Runbook

## Purpose

This runbook defines how to execute the initial Kani proof harnesses for the therapy-control state-machine slice.

The proof boundary remains limited to `pulse_guard_core::transition(current, inputs)` and the selected one-step safety invariants documented in `formal-properties.md`.

## CI Execution

Kani runs in GitHub Actions through `.github/workflows/ci.yml`.

The CI job uses:

- GitHub Action: `model-checking/kani-github-action@v1`
- Kani version: `0.66.0`
- Command: `cargo-kani -p pulse_guard_core`

After the workflow runs, record the result in `evidence/` with:

- workflow URL or run identifier
- commit SHA
- Kani version
- command
- pass/fail status
- any counterexamples or resource-limit failures

## Local Installation

Kani's installation guide currently describes a two-step local install:

```sh
cargo install --locked kani-verifier
cargo kani setup
```

Then run the project proof harnesses:

```sh
cargo kani -p pulse_guard_core
```

## Expected Result

The first expected result is successful proof of:

- `inv_001_inhibit_blocks_delivery_request`
- `inv_002_delivery_requires_armed_deliver_command`
- `inv_003_delivery_requires_valid_detection`
- `inv_004_delivery_requires_connection_readiness_and_self_test`
- `inv_005_faulted_never_requests_or_enables_therapy`

## Failure Handling

If Kani finds a counterexample:

- preserve the Kani output as evidence
- identify the linked invariant, requirement, and hazard
- update either the implementation or the artifact baseline before closing the finding
- do not weaken an invariant without updating the hazard/risk-control rationale

If Kani fails due to installation, toolchain, memory, or timeout:

- record the failure as an environment or tooling issue
- preserve enough log detail to reproduce it
- keep the proof status as not yet verified

## Current Limits

- Kani is not installed in the local development environment used for the first baseline.
- CI proof evidence is not available until the GitHub Actions workflow runs on a pushed commit or pull request.
- These proofs do not cover timing, clinical detection, hardware energy delivery, alarms, operator workflow, communication security, or full-device behavior.
