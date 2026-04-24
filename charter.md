
# Project Charter

## Purpose

Create an outward-facing, AI-led portfolio project for Common Sense Systems, Inc. that demonstrates credible medical-device software development practice, verification and validation discipline, and recursive AI-assisted artifact improvement across the lifecycle.

## Audience / User

Hiring managers, prospective medical-device clients, quality and regulatory reviewers, engineering peers, talent-network evaluators such as Toptal and Upwork, and internal Common Sense Systems stakeholders.

## Current Condition

The project owner has a long history in medical-device development but has been out of the field for about a year and needs a current, compelling public portfolio artifact that demonstrates present capability in software engineering, V&V, and safety-oriented development practice.

## Desired Condition

A release-ready public repository that shows a disciplined, AI-led development system producing traceable requirements, risks, design artifacts, code, tests, formal properties, verification evidence, and business-facing portfolio documentation for a plausible medical-device software example.

## Core Problem

There is currently no single public artifact that credibly demonstrates Common Sense Systems' current capability to deliver AI-assisted medical-device software development with traceability, risk thinking, formal methods, and verification evidence that serious buyers and reviewers can inspect.

## Theory of Improvement

An AI-led, recursively improving workflow will generate and refine hazards, risk controls, requirements, design decisions, Rust implementation, adversarial tests, formal verification properties, CI evidence, and portfolio-facing documentation under human review, producing a stronger and more scalable demonstration than a manually assembled example alone.

## Useful First Version

A small but credible therapy-control state-machine project in Rust with traceable hazards and risk controls, selected IEC 62304 and ISO 14971 aligned artifacts, conventional automated tests, AI-assisted adversarial testing, narrow Kani-based formal verification of safety invariants, CI/CD automation, containerized reproducibility, and a top-level explainer describing the technical and business case for the portfolio project.

## Initial Artifacts

- Project charter describing the AI-led portfolio objective and proof boundary
- Hazard and risk-control starter set for unintended therapy delivery and related safety concerns
- Initial software requirements for therapy-control state transitions and interlocks
- Architecture note describing the pure Rust transition function and proof boundary
- Rust therapy-control state-machine skeleton
- Conventional automated test strategy and starter scenarios
- Formal property list linked to hazards and requirements
- Initial Kani proof harness plan for shock-inhibit and armed-state invariants
- CI/CD and containerization plan
- Top-level explainer of the technical and business case for the portfolio project

## Success Criteria

- The repository credibly demonstrates AI-led medical-device software development practice
- The repository includes traceable hazards, risk controls, requirements, design artifacts, code, tests, and evidence
- Selected safety invariants are checked with Kani against pure transition logic
- CI/CD can build, test, and report verification status reproducibly
- The project is suitable to show prospective clients, reviewers, and hiring evaluators after release preparation
- The proof boundary and non-claims are explicit and professionally defensible

## Constraints

- AI-led project with recursive artifact improvement as a hard requirement
- Alignment with current IEC 62304 expectations
- Alignment with current ISO 14971 risk-management expectations
- Alignment with FDA design controls expectations
- Rust as the primary implementation language
- Kani as the initial formal verification tool
- GitHub Actions based CI/CD
- Containerized reproducibility via Docker or equivalent
- Clear traceability from hazards to risk controls, requirements, tests, formal properties, and evidence
- Public-release readiness after private incubation
- Claims about formal verification must remain narrow, honest, and defensible
- Documentation deliverables should be suitable for Microsoft Office formats

## Non-Goals

- Claiming full formal verification of a complete medical device
- Claiming clinical validation or hardware validation from software-only evidence
- Treating AI outputs as unreviewed authoritative safety evidence
- Building a production medical device ready for market submission in the first iteration
