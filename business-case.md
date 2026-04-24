# CSSI Pulse Guard Business Case

## Purpose

This document explains why CSSI Pulse Guard exists as a portfolio artifact and why its current scope is commercially useful even though it is intentionally narrower than a full medical-device program.

## Problem Being Solved

Prospective clients, hiring managers, and technical reviewers often need a credible way to evaluate current capability without relying on confidential prior work. In regulated software, vague claims about "experience" are weak. Reviewers want inspectable artifacts, traceability, verification results, and honest language about what is and is not proven.

CSSI Pulse Guard addresses that gap with a public artifact set that shows:

- risk-informed software development discipline
- traceability from hazards to requirements, design, code, tests, and proof evidence
- practical use of AI inside a reviewed engineering workflow rather than as an unexamined authority
- explicit proof boundaries and non-claims that can withstand technical scrutiny

## Why The Scope Is Narrow

The project models a therapy-control state machine instead of a full device because a narrow slice is more defensible than a broad but weak demonstration.

This scope lets reviewers inspect:

- safety gating logic with a plausible medical-device flavor
- a compact Rust implementation that is small enough to read end to end
- conventional and formal verification applied to the same proof boundary
- artifact discipline consistent with IEC 62304 and ISO 14971 expectations

That is a better portfolio signal than a larger repository with shallow traceability or inflated verification claims.

## Intended Audiences

This repository is useful to several audiences for different reasons:

- prospective medical-device clients: it shows how Common Sense Systems approaches safety-oriented software work with traceability and verification discipline
- hiring managers and engineering leaders: it demonstrates current hands-on Rust, testing, CI, and formal-methods capability
- quality and regulatory reviewers: it shows structured artifacts and explicit non-claims instead of marketing language
- talent-network evaluators: it provides a compact, inspectable body of work with technical depth

## Value To Prospective Clients

For clients, the value is not the specific state machine. The value is evidence about how work is done.

This repository shows:

- requirements, design, implementation, and verification kept in visible alignment
- safety claims constrained to what the available evidence actually supports
- AI used to accelerate artifact creation and recursive improvement under human review
- a development style that can scale into more formal project controls when needed

That makes the repository a proxy for delivery quality and engineering judgment.

## Value To Hiring Evaluators

For hiring evaluators, the project demonstrates a current technical baseline rather than historical experience alone.

It shows:

- Rust implementation of a safety-oriented state machine
- unit, scenario, and generated adversarial testing
- Kani proof harnesses and CI-recorded proof results
- documentation discipline and release/tagging hygiene

This is the kind of evidence that lets an evaluator judge depth, not just resume claims.

## AI Use Position

The project is intentionally AI-assisted, but not AI-trusting.

The business value of that position is straightforward:

- AI can accelerate drafting, refinement, traceability maintenance, and test expansion
- human review remains responsible for accepting artifacts and limiting claims
- the repository demonstrates a realistic delivery model for teams that want leverage from AI without collapsing quality controls

That combination is commercially more useful than either fully manual documentation theater or unreviewed AI output.

## What This Repository Does Not Claim

This repository does not claim:

- a complete device development file
- clinical effectiveness
- hardware or system validation
- production release readiness
- regulatory submission readiness

Those limits are part of the value proposition. They show disciplined scope control and defensible communication.

## Next Commercially Meaningful Steps

The next additions with the highest portfolio value are:

- public-release tag decision for the current evidence baseline
- continued verification depth through generated adversarial sequence expansion
- follow-on release preparation for the public portfolio baseline

## Bottom Line

CSSI Pulse Guard is useful because it gives serious reviewers a compact, honest, inspectable demonstration of AI-assisted medical-device software engineering practice. It is not trying to stand in for a full device program. It is trying to prove that Common Sense Systems can build one with discipline.

For a concise external summary, see [release/public-portfolio-summary.md](release/public-portfolio-summary.md).
