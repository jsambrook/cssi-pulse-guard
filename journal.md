
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

## 2026-04-24 - Portfolio Explainer Baseline

### Focus

Turn the repository's top-level entry point into a credible outward-facing explainer.

### Intent

Close the biggest remaining public-facing gap by making the repository understandable to prospective clients, hiring evaluators, and technical reviewers without requiring them to reconstruct the project from internal artifact files.

### Work Done

- Reviewed the current outward-facing artifacts and identified the missing explainer/business-case baseline as the highest-value next step.
- Rewrote `README.md` as the top-level portfolio explainer.
- Added `business-case.md` for external commercial and hiring audiences.
- Updated `artifact-map.md` and `next-actions.md` to reflect the new baseline.

### Evidence / Test

Documentation change only. No behavior or verification logic changed.

### Result

The repository now opens with a clear explanation of scope, proof boundary, current evidence, non-claims, and commercial purpose.

### Next Action

Commit the explainer baseline, then move to containerized reproducibility or residual-risk acceptability criteria.

## 2026-04-24 - Tagging Threshold Clarification

### Focus

Make the lifecycle tagging strategy easier to apply consistently during future development.

### Intent

Turn the existing tagging strategy into an operational rule that clearly distinguishes ordinary coherent commits from tag-worthy lifecycle baselines.

### Work Done

- Expanded `process/release-and-tagging.md` with an explicit milestone threshold.
- Added a working rule for deciding whether a commit changes the repository's external claim.
- Added repository-specific examples showing why some recent commits were tagged and others were not.
- Added a tie-breaker rule to prefer fewer, cleaner tags when the milestone boundary is unclear.

### Evidence / Test

Process-only change. No code or verification behavior changed.

### Result

The repository now has a more explicit and repeatable rule for deciding when a lifecycle tag should be created.

### Next Action

Commit the tagging-threshold clarification, then continue with containerized reproducibility or residual-risk acceptability criteria.

## 2026-04-24 - Container Reproducibility Baseline

### Focus

Add the first containerized reproducibility path for the repository.

### Intent

Advance the charter requirement for containerized reproducibility by making the conventional Rust verification bar runnable in a pinned container environment.

### Work Done

- Reviewed the current artifact gaps and selected containerized reproducibility as the highest-value next step.
- Confirmed the local development environment does not currently have `docker` or `podman`.
- Added `Dockerfile` pinned to `rust:1.95.0-bookworm`.
- Added `.dockerignore`.
- Added `container/run-rust-verification.sh`.
- Added `container/runbook.md` and `container/README.md`.
- Updated `artifact-map.md`, `implementation/README.md`, `next-actions.md`, and local evidence notes.

### Evidence / Test

- `cargo fmt --all --check`: pass
- `cargo test --workspace --all-targets`: pass, 14 tests
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `docker --version`: unavailable in local environment
- `podman --version`: unavailable in local environment

### Result

The repository now has a pinned container baseline for the conventional Rust verification bar, but container execution evidence is still pending because no local container engine is available in this environment.

### Next Action

Commit the container baseline, then execute it and record evidence once a container engine is available, or move next to residual-risk acceptability criteria.

## 2026-04-24 - Container CI Execution Path

### Focus

Turn the drafted container baseline into an executable CI path.

### Intent

Close the gap between having a Dockerfile and having actual evidence that the repository's conventional Rust verification bar runs reproducibly in a pinned container environment.

### Work Done

- Reviewed the remaining gaps and selected container execution evidence as the highest-value next step.
- Added a `container-rust` job to `.github/workflows/ci.yml`.
- Updated `container/runbook.md` to describe CI execution and evidence expectations.
- Updated `artifact-map.md` and `next-actions.md` to reflect that the container path now exists in CI but is not yet evidenced.

### Evidence / Test

Local Rust verification should be run after this change. Container execution evidence is not yet available until the GitHub Actions workflow runs and its result is recorded.

### Result

The repository now has a CI execution path for the containerized Rust verification baseline.

### Next Action

Commit and push the container CI path, then record the container verification result from GitHub Actions.

## 2026-04-24 - Container CI Verification Evidence

### Focus

Execute the new container CI path and preserve the resulting reproducibility evidence.

### Intent

Turn the repository's container baseline from a drafted capability into a verified lifecycle checkpoint with recorded evidence.

### Work Done

- Pushed commit `fe57f11` to `main`.
- Waited for GitHub Actions run `24903332050`.
- Confirmed successful completion of the `Containerized Rust verification` job along with the Rust and Kani jobs.
- Added `evidence/container-verification-2026-04-24.md`.
- Updated `artifact-map.md`, `next-actions.md`, `evidence/local-verification-2026-04-24.md`, and `container/runbook.md`.

### Evidence / Test

- GitHub Actions workflow `CI`: pass
- Commit: `fe57f1158fe106cd79b5916a5856c3d7ad647754`
- Containerized Rust verification job: pass
- Container built successfully and ran `./container/run-rust-verification.sh`
- Container run completed formatting, tests, and clippy successfully

### Result

The repository now has recorded CI evidence that the conventional Rust verification bar runs successfully in a pinned container environment.

### Next Action

Commit the container evidence record and tag the lifecycle baseline as `v0.5.0-containerized-ci-baseline`, then continue with out-of-scope hazard review notes or broader generated exploration.

## 2026-04-24 - GitHub Actions Checkout Upgrade

### Focus

Remove the Node.js 20 deprecation warning by moving workflow checkout steps to the Node 24-compatible `actions/checkout@v5` line.

### Intent

Keep the CI pipeline current and avoid carrying a warning that is already known to be tied to the existing checkout action version.

### Work Done

- Reviewed the project artifacts and selected the GitHub Actions deprecation warning as the highest-value remaining maintenance gap.
- Updated all checkout steps in `.github/workflows/ci.yml` from `actions/checkout@v4` to `actions/checkout@v5`.
- Updated `next-actions.md` to track verification of the warning removal.

### Evidence / Test

Workflow-only change. The updated workflow should be verified by the next GitHub Actions run after push.

### Result

The workflow now uses the Node 24-compatible checkout action line recommended by the upstream release notes.

### Next Action

Commit and push the workflow update, then confirm the next CI run no longer reports the Node.js 20 deprecation warning.

## 2026-04-24 - GitHub Actions Warning Cleared

### Focus

Verify that the checkout upgrade removed the Node.js 20 deprecation warning from the latest CI run.

### Intent

Close the maintenance item by confirming the repository's workflow now uses the Node 24-compatible checkout action line without reintroducing the deprecation warning.

### Work Done

- Pushed commit `340e42e` to `main`.
- Waited for GitHub Actions run `24903548750`.
- Confirmed all three jobs passed.
- Verified the workflow used `actions/checkout@v5` in the Rust and container jobs.
- Filtered the logs and confirmed the prior Node.js 20 deprecation warning text was no longer present.
- Added `evidence/ci-maintenance-2026-04-24.md`.
- Updated `next-actions.md` to remove the deprecation warning item from active work and replace it with a future monitoring note.

### Evidence / Test

- GitHub Actions workflow `CI`: pass
- Commit: `340e42eda2289413674318627bce789ce604edb6`
- Rust checks: pass
- Containerized Rust verification: pass
- Kani proofs: pass
- `actions/checkout@v5` observed in log
- No Node.js 20 deprecation warning text observed in the filtered job logs

### Result

The checkout-action maintenance item is closed for the current workflow state.

### Next Action

Continue with hazard review notes for usability, hardware-interface, and cybersecurity risks, or refine the outward-facing explainer for final audience fit.

## 2026-04-24 - Residual Risk Acceptability Baseline

### Focus

Define objective residual-risk acceptance criteria for the current software hazard baseline.

### Intent

Strengthen the repository's risk story by replacing provisional residual rationale with explicit acceptance bands and an applied disposition for each current hazard.

### Work Done

- Reviewed the hazard analysis, requirements baseline, and open project-control actions.
- Identified residual-risk acceptability as the highest-value next step because the repository already had stronger implementation and verification evidence than risk-disposition discipline.
- Added `risk/risk-acceptability.md`.
- Updated `risk/hazard-analysis.md` with residual scores, acceptance status, and evidence-based residual rationale.
- Updated `risk/README.md`, `artifact-map.md`, `decision-log.md`, and `next-actions.md`.

### Evidence / Test

Documentation change only. No implementation behavior changed.

### Result

The repository now has a project-specific residual-risk acceptance frame and an explicit disposition for each current therapy-control software hazard.

### Next Action

Commit the residual-risk baseline, then move next to multi-step adversarial testing, container execution evidence, or expanded out-of-scope hazard review notes.

## 2026-04-24 - Container CI Execution Path

### Focus

Turn the drafted container baseline into an executable CI path.

### Intent

Close the gap between having a Dockerfile and having actual evidence that the repository's conventional Rust verification bar runs reproducibly in a pinned container environment.

### Work Done

- Reviewed the remaining gaps and selected container execution evidence as the highest-value next step.
- Added a `container-rust` job to `.github/workflows/ci.yml`.
- Updated `container/runbook.md` to describe CI execution and evidence expectations.
- Updated `artifact-map.md` and `next-actions.md` to reflect that the container path now exists in CI but is not yet evidenced.

### Evidence / Test

Local Rust verification should be run after this change. Container execution evidence is not yet available until the GitHub Actions workflow runs and its result is recorded.

### Result

The repository now has a CI execution path for the containerized Rust verification baseline.

### Next Action

Commit and push the container CI path, then record the container verification result from GitHub Actions.

## 2026-04-24 - Multi-Step Adversarial Sequence Baseline

### Focus

Extend adversarial regression coverage from one-step matrices to bounded hostile command sequences.

### Intent

Increase verification depth by checking that the therapy-control logic preserves delivery gating, fault latching, and power-off dominance across multi-step hostile paths rather than only one-step combinations.

### Work Done

- Reviewed the remaining gaps and selected multi-step adversarial testing as the highest-value next step.
- Extended `implementation/pulse_guard_core/tests/adversarial.rs` with bounded hostile regression sequences.
- Updated `verification/adversarial-test-strategy.md` and `verification/test-strategy.md`.
- Updated local verification evidence, `artifact-map.md`, and `next-actions.md`.

### Evidence / Test

- `cargo fmt --all --check`: pass
- `cargo test --workspace --all-targets`: pass, 15 tests
- `cargo clippy --workspace --all-targets -- -D warnings`: pass

### Result

The repository now includes bounded multi-step adversarial sequence coverage in addition to the one-step generated matrix tests.

### Next Action

Commit this adversarial sequence baseline, then move to container execution evidence, broader generated sequence exploration, or out-of-scope hazard review notes.
