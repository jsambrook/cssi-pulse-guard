# CSSI Pulse Guard Public Portfolio Summary

## What This Project Is

CSSI Pulse Guard is an AI-assisted portfolio project that demonstrates a narrow but credible slice of medical-device software development practice. It shows how Common Sense Systems approaches traceability, risk thinking, Rust implementation, automated verification, formal proofs, and release discipline.

The modeled software is a therapy-control state machine. It is intentionally not a full device. The repository is designed to be inspectable by hiring managers, prospective clients, and technical reviewers who want evidence instead of claims.

## What The Repository Demonstrates

- hazards, safety invariants, and residual-risk acceptability criteria
- software requirements and a state-machine design note
- a small Rust implementation of the pure transition-function boundary
- unit tests, scenario tests, and generated adversarial regression tests
- Kani proof harnesses with CI-recorded proof evidence
- containerized CI verification evidence
- release and tagging discipline with explicit non-claims

## What Has Been Verified

The current evidence supports these narrow claims:

- therapy request remains gated by arming, inhibit, detection, connection, readiness, and self-test conditions
- `FaultDetected` drives the controller into a faulted safe state with therapy disabled
- `PowerOff` dominates and clears therapy-related outputs
- five selected one-step safety invariants have been checked with Kani in CI
- the conventional Rust verification path runs in a pinned container in CI

## What This Project Does Not Claim

- full formal verification of a complete medical device
- clinical validation
- hardware validation or energy-delivery validation
- cybersecurity assurance
- market submission readiness

## Why It Matters

The value of the repository is not the specific state machine alone. The value is the evidence trail that shows how requirements, risks, code, tests, proofs, and CI fit together under review.

This makes the project useful as a portfolio artifact for technical review, hiring, and client discussions.

## Evidence Pointers

- [charter.md](../charter.md)
- [artifact-map.md](../artifact-map.md)
- [risk/hazard-analysis.md](../risk/hazard-analysis.md)
- [requirements/software-requirements.md](../requirements/software-requirements.md)
- [design/therapy-state-machine.md](../design/therapy-state-machine.md)
- [evidence/ci-verification-2026-04-24.md](../evidence/ci-verification-2026-04-24.md)
- [evidence/container-verification-2026-04-24.md](../evidence/container-verification-2026-04-24.md)

## Current Release Posture

The repository is not yet tagged as `v1.0.0-public-portfolio-release`. The current best description is:

- a release-prepared private baseline with strong technical evidence
- a portfolio artifact that is approaching public release readiness
- a project that still has a few audience-fit and risk-review items to close before final release
