
# Next Actions

## Active

- Refine the outward-facing explainer for release preparation and final audience fit
- Decide whether a later container baseline should include Kani or stay CI-only for proofs
- Add hazard review notes for usability, hardware-interface, and cybersecurity risks outside the current proof slice
- Add regression seeds or broader generated exploration beyond the current bounded adversarial sequences
- Decide whether to add a local container execution evidence record once a workstation container engine is available
- Verify the updated GitHub Actions checkout version removes the Node.js 20 deprecation warning

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
- Extended adversarial testing to bounded multi-step command sequences.
- Drafted the top-level portfolio explainer and business case.
- Added a containerized Rust verification baseline and runbook.
- Added a CI execution path for the containerized Rust verification baseline.
- Executed the containerized Rust verification path in CI and recorded evidence.
- Updated GitHub Actions checkout to the Node 24-compatible `v5` line.
- Defined and applied objective residual-risk acceptability criteria for the current hazard baseline.
- Created initial repository structure.
