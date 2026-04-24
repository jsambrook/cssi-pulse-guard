# Release Readiness Checklist

## Baseline Gates

- [x] Project charter and scope statement exist
- [x] Hazard analysis and safety invariants exist
- [x] Objective residual-risk acceptability criteria exist
- [x] Traceable software requirements exist
- [x] Architecture and proof boundary are documented
- [x] Rust implementation and tests exist
- [x] Scenario tests and generated adversarial tests exist
- [x] Kani proof evidence is recorded in CI
- [x] Containerized verification evidence is recorded in CI
- [x] Release and tagging strategy is documented
- [x] Top-level explainer and business case exist

## Public-Release Gates

- [x] Final audience-fit review for README and business-case language
- [x] Review whether additional hazard notes are needed for usability, hardware-interface, and cybersecurity risks outside the proof slice
- [ ] Confirm the final public release tag boundary
- [ ] Confirm the release notes or summary package are adequate for Microsoft Office use

## Operating Rules

- Do not broaden the proof claims beyond what the evidence supports.
- Do not move the proof boundary without updating hazards, requirements, design, tests, and evidence.
- Do not tag a public release until the remaining public-release gates are reviewed.
