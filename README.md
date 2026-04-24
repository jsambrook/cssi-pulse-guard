
# CSSI Pulse Guard

CSSI Pulse Guard is an outward-facing portfolio project for Common Sense Systems, Inc. It demonstrates a narrow, defensible slice of AI-assisted medical-device software development practice: risk-informed requirements, traceable design, Rust implementation, conventional verification, Kani-based formal checks, CI evidence, and explicit non-claims.

The modeled product slice is a plausible therapy-control state machine. It is not a complete device. The current proof boundary is a pure Rust transition function that decides state changes and therapy-request outputs under interlock, detection, readiness, and fault conditions.

## Why This Repository Exists

This repository is meant to answer a specific question for technical reviewers, prospective clients, and hiring evaluators:

Can Common Sense Systems show current, disciplined capability in safety-oriented software development with traceability and verification evidence that a serious reviewer can inspect?

The answer here is intentionally narrow. The repository does not claim full device development, clinical validation, or submission readiness. It shows a credible first proof slice with artifacts that line up from hazard to requirement to design to code to tests to proof evidence.

## Current Baseline

The repository currently includes:

- hazard analysis and linked safety invariants for therapy-control logic
- traceable software requirements and a state-machine design note
- a dependency-light Rust crate implementing the pure transition function
- unit tests, scenario tests, and generated adversarial regression tests
- Kani proof harnesses with passing CI-recorded proof evidence for five initial invariants
- GitHub Actions CI for Rust checks and Kani proofs
- lifecycle controls including a journal, decision log, artifact map, next actions, and release/tagging strategy

## What Has Been Verified

The current verification evidence supports these narrow claims:

- therapy request remains gated by arming, inhibit, detection, connection, readiness, and self-test conditions
- `FaultDetected` drives the controller into a faulted safe state with therapy disabled
- `PowerOff` dominates and clears therapy-related outputs
- five selected one-step safety invariants have been checked with Kani in CI

See:

- [hazard-analysis.md](risk/hazard-analysis.md)
- [software-requirements.md](requirements/software-requirements.md)
- [therapy-state-machine.md](design/therapy-state-machine.md)
- [ci-verification-2026-04-24.md](evidence/ci-verification-2026-04-24.md)
- [release/public-portfolio-summary.md](release/public-portfolio-summary.md)

## Proof Boundary and Non-Claims

This repository does not claim:

- full formal verification of a complete medical device
- clinical validation
- hardware validation or energy-delivery verification
- real-time scheduling correctness
- cybersecurity assurance
- market-submission readiness

The current proof boundary is the pure Rust transition function in [lib.rs](implementation/pulse_guard_core/src/lib.rs).

## Repository Guide

- Start with the [project charter](charter.md) for scope and constraints.
- Use the [artifact map](artifact-map.md) to see expected deliverables and status.
- Review [verification/README.md](verification/README.md) and [evidence/README.md](evidence/README.md) for the current verification and evidence trail.
- Read the external-facing [business-case.md](business-case.md) for why this project matters commercially and professionally.
- Use the [release package](release/README.md) for a concise public-release view.

## Current Priorities

- keep CI dependencies current as GitHub Actions platform requirements change
