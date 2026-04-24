# Adversarial and Generated Test Strategy

## Purpose

This artifact defines the first adversarial and generated test slice for the therapy-control state machine. The goal is to extend verification beyond hand-picked unit and scenario tests by systematically exercising invalid commands and hostile interlock combinations.

## Scope

The generated adversarial suite targets the pure Rust transition function in `pulse_guard_core`.

The suite focuses on:

- invalid command/state combinations
- delivery attempts under hostile interlock combinations
- armed-state drift when delivery preconditions become false
- command dominance for `PowerOff` and `FaultDetected`
- bounded hostile command sequences with changing interlock conditions

## Test Design

The generated test module enumerates:

- 6 controller states
- 10 commands
- 32 boolean input combinations for:
  - `patient_connected`
  - `therapy_inhibit`
  - `valid_therapy_detection`
  - `hardware_ready`
  - `self_test_passed`

This yields 1,920 one-step input combinations per full state-command matrix pass.

The suite also includes bounded regression sequences that combine:

- invalid command ordering
- inhibit activation after arming
- fault latching and blocked fault reset
- readiness and detection churn across multiple steps

## Initial Generated Tests

| Test ID | Test | Intent | Trace |
|---|---|---|---|
| ADV-001 | `generated_input_matrix_preserves_delivery_safety_invariants` | Verify generated hostile inputs do not bypass delivery gating or fault/power dominance rules. | INV-001 through INV-005; SWR-001 through SWR-010 |
| ADV-002 | `invalid_command_matrix_never_requests_therapy` | Verify invalid command/state combinations never request therapy. | SWR-002, SWR-006, SWR-007, SWR-010 |
| ADV-003 | `armed_state_drops_out_of_armed_when_any_delivery_precondition_fails` | Verify the armed state collapses to monitoring when any delivery precondition becomes false. | HAZ-001, HAZ-003, HAZ-004, HAZ-006; SWR-006 |
| ADV-004 | `adversarial_sequences_preserve_delivery_and_fault_safety_properties` | Verify bounded hostile command sequences preserve delivery gating, fault latching, and power-off dominance. | HAZ-001 through HAZ-006; SWR-001 through SWR-010 |

## Relationship To Formal Proof

These tests do not replace Kani. They complement the Kani proofs by:

- checking many concrete combinations under the standard Rust test runner
- protecting against regressions in command handling and blocking behavior
- giving reviewers a readable, conventional evidence trail alongside formal results

## Current Limits

- The suite includes bounded multi-step sequences, but it is not exhaustive sequence exploration.
- Sequence generation is currently seed-based rather than algorithmically enumerated.
- The suite does not model timing, alarms, operator workflow, hardware energy delivery, or cybersecurity behavior.
