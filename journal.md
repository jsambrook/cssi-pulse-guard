
# Project Journal

## Initial Entry

### Focus

Create the initial project structure using the CSSI Development Method.

### Intent

Establish enough project context, files, and next actions to begin disciplined artifact development.

### Work Done

- Created standalone project repository.
- Selected profile: `embedded-medical-device-software`.
- Created charter, artifact map, decision log, next actions, and folder structure.

### Evidence / Test

The repo structure exists, git is initialized, and an initial commit should be present after creation.

### Result

Ready for the first artifact-development step.

### Next Action

Draft the first hazard analysis and linked safety invariants for therapy-control logic

## 2026-04-24 - First Traceable Therapy-Control Baseline

### Focus

Move the project from scaffolded artifacts to a traceable first implementation slice.

### Intent

Create a narrow but credible baseline that links hazards, risk controls, requirements, design, Rust code, tests, and formal-property plans.

### Work Done

- Reviewed the charter, artifact map, and active next actions.
- Identified hazard analysis plus linked safety invariants as the highest-value next step because it drives requirements, design, implementation, and verification.
- Drafted `risk/hazard-analysis.md` and `risk/safety-invariants.md`.
- Drafted `requirements/software-requirements.md`.
- Drafted `design/therapy-state-machine.md`.
- Added the `pulse_guard_core` Rust crate with a pure therapy-control transition function.
- Added unit tests for inhibit behavior, arming preconditions, therapy request gating, fault latching, and fault reset.
- Added initial Kani proof harnesses under `cfg(kani)`.
- Added `verification/formal-properties.md`, `verification/kani-proof-plan.md`, and `.github/workflows/ci.yml`.
- Recorded local verification evidence in `evidence/local-verification-2026-04-24.md`.

### Evidence / Test

- `cargo fmt --all --check`: pass
- `cargo test --workspace --all-targets`: pass, 6 tests
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo kani -p pulse_guard_core`: not run because local Cargo does not have the `kani` subcommand installed

### Result

The repository now has a first end-to-end traceable therapy-control slice suitable for review and refinement.

### Next Action

Add scenario tests for full command sequences and decide whether Kani should be installed locally or run first in CI.

## 2026-04-24 - Scenario Test Baseline

### Focus

Add conventional scenario coverage for the existing therapy-control state-machine baseline.

### Intent

Strengthen the charter-required V&V evidence by exercising readable multi-step command sequences in addition to focused unit tests and planned Kani properties.

### Work Done

- Reviewed the charter, artifact map, next actions, state-machine implementation, verification plan, and local evidence.
- Identified starter scenario testing as the highest-value next step because the implementation had unit tests and formal harnesses but no multi-step executable scenarios.
- Added `verification/test-strategy.md` with unit, scenario, Kani, and future fault-injection levels.
- Added `implementation/pulse_guard_core/tests/scenarios.rs` with five scenario tests:
  - nominal power-on, monitoring, arming, therapy request, and completion
  - blocked arming until detection returns
  - inhibit during armed state with disarm and no therapy request
  - fault latching and readiness-gated reset
  - power-off from active state with therapy outputs disabled
- Updated local verification evidence and project-control artifacts.

### Evidence / Test

- `cargo fmt --all --check`: pass
- `cargo test --workspace --all-targets`: pass, 11 tests
- `cargo clippy --workspace --all-targets -- -D warnings`: pass

### Result

The project now has a conventional scenario-test baseline linked to the therapy-control requirements and hazard controls.

### Next Action

Add a Kani execution path in CI or document local Kani installation steps, then add adversarial/generated tests for invalid command sequences and interlock combinations.

## 2026-04-24 - Release and Tagging Strategy

### Focus

Document a lifecycle tagging strategy before committing the first traceable development baseline.

### Intent

Make project history reviewable through coherent commits and annotated milestone tags that align with artifact maturity, verification status, and explicit proof-boundary claims.

### Work Done

- Added `process/release-and-tagging.md`.
- Added `process/README.md`.
- Updated `artifact-map.md` to include release and tagging process documentation.

### Evidence / Test

Process-only change. Verification checks should be run before committing the baseline.

### Result

The project has a documented tagging strategy for lifecycle baselines such as `v0.3.0-scenario-vv-baseline`, `v0.4.0-kani-proof-baseline`, and `v1.0.0-public-portfolio-release`.

### Next Action

Commit the current baseline and tag it as the first scenario V&V lifecycle baseline after verification passes.

## 2026-04-24 - Kani CI Execution Path

### Focus

Add a CI path for executing the existing Kani proof harnesses.

### Intent

Advance the charter requirement that selected safety invariants are checked with Kani and that CI/CD can report verification status reproducibly.

### Work Done

- Reviewed the active artifact gaps and selected Kani CI execution as the highest-value next step.
- Checked current Kani documentation and GitHub Action usage guidance.
- Added a `kani` job to `.github/workflows/ci.yml`.
- Pinned the CI job to Kani `0.66.0`.
- Added `verification/kani-runbook.md` for CI and local proof execution.
- Updated `verification/kani-proof-plan.md`, `evidence/local-verification-2026-04-24.md`, `artifact-map.md`, and `next-actions.md`.

### Evidence / Test

Local Rust verification should be run after this change. Kani proof output is not yet available because the GitHub Actions workflow has not run in this environment.

### Result

The project now has a documented Kani execution path in CI, but proof status remains pending until the workflow is pushed and the CI result is recorded.

### Next Action

Push the Kani CI workflow, review the GitHub Actions result, and record proof evidence or any failure details in `evidence/`.

## 2026-04-24 - Kani CI Proof Evidence

### Focus

Execute the Kani CI path and preserve proof evidence.

### Intent

Close the gap between having Kani harnesses and having recorded verification evidence for selected safety invariants.

### Work Done

- Pushed `main` and the existing `v0.3.0-scenario-vv-baseline` tag to GitHub.
- Waited for GitHub Actions run `24899095207`.
- Reviewed CI run status and Kani proof logs.
- Added `evidence/ci-verification-2026-04-24.md`.
- Updated `artifact-map.md`, `next-actions.md`, `verification/kani-proof-plan.md`, and local evidence notes.

### Evidence / Test

- GitHub Actions workflow `CI`: pass
- Commit: `5f19d05f1907cf1772cfb4d8030ff3f4869955f4`
- Rust checks: pass
- Kani proofs: pass
- Kani summary: `0 of 52 failed`
- Manual harness summary: 5 successfully verified harnesses, 0 failures, 5 total

### Result

The project now has CI-recorded Kani proof evidence for the five initial therapy-control safety invariants.

### Next Action

Commit the evidence record and tag the lifecycle baseline as `v0.4.0-kani-proof-baseline`, then continue with adversarial/generated testing or containerized reproducibility.

## 2026-04-24 - Generated Adversarial Test Baseline

### Focus

Add generated adversarial tests for invalid command combinations and hostile interlock states.

### Intent

Advance the charter's AI-led adversarial-testing requirement with a reproducible generated test slice that complements unit, scenario, and Kani evidence.

### Work Done

- Reviewed the current artifacts and identified adversarial/generated testing as the highest-value next step after recorded Kani proof evidence.
- Added `implementation/pulse_guard_core/tests/adversarial.rs`.
- Added `verification/adversarial-test-strategy.md`.
- Updated `verification/test-strategy.md`, local verification evidence, `artifact-map.md`, and `next-actions.md`.

### Evidence / Test

- `cargo fmt --all --check`: pass
- `cargo test --workspace --all-targets`: pass, 14 tests
- `cargo clippy --workspace --all-targets -- -D warnings`: pass

### Result

The project now includes generated adversarial regression coverage over invalid command/state combinations and hostile delivery-precondition combinations.

### Next Action

Commit this adversarial test baseline, then either extend generation to multi-step command sequences or add the containerized reproducibility baseline.
