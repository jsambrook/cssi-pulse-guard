
# Next Actions

## Active

- Draft the top-level portfolio explainer and business case
- Add a containerized reproducibility baseline
- Define objective residual-risk acceptability criteria for the hazard baseline
- Review GitHub Actions Node.js 20 deprecation warning and update action versions when replacements are available
- Extend generated testing to multi-step adversarial command sequences and regression seeds

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
- Created initial repository structure.
