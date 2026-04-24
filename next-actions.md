
# Next Actions

## Active

- Define objective residual-risk acceptability criteria for the hazard baseline
- Review GitHub Actions Node.js 20 deprecation warning and update action versions when replacements are available
- Extend generated testing to multi-step adversarial command sequences and regression seeds
- Refine the outward-facing explainer for release preparation and final audience fit
- Execute the containerized Rust verification path and record container evidence
- Decide whether a later container baseline should include Kani or stay CI-only for proofs

## Waiting

- None

## Later

- Review and refine the project charter.
- Add definitions of done for the first three artifacts.

## Recently Completed

- Drafted first hazard analysis and linked safety invariants for therapy-control logic.
- Defined the initial therapy state machine and pure Rust transition-function interface.
- Created first traceable software requirements and risk controls baseline.
- Stood up the Rust workspace, unit-test harness, and CI skeleton.
- Specified initial Kani proof targets and assumptions.
- Added scenario tests for full command sequences: power on, monitor, arm, deliver, complete, fault, reset, and power off.
- Added a Kani CI execution path and local/CI Kani runbook.
- Pushed the CI workflow and recorded passing Kani proof evidence from GitHub Actions.
- Added generated adversarial tests for invalid command sequences and interlock combinations.
- Drafted the top-level portfolio explainer and business case.
- Added a containerized Rust verification baseline and runbook.
- Created initial repository structure.
