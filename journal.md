
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
