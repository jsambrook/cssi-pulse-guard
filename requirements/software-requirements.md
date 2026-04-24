# Software Requirements Baseline

## Scope

These requirements apply to the first therapy-control state-machine slice. They are intentionally narrow and trace to the hazards, risk controls, and invariants in `risk/`.

## Assumptions

- The transition function receives already-debounced boolean inputs for patient connection, therapy inhibit, therapy detection, hardware readiness, and self-test result.
- The transition function does not perform clinical signal processing.
- The transition function is deterministic and has no hidden global state.
- Therapy request is modeled as a one-cycle command output, not as physical energy delivery.

## State-Machine Requirements

| Requirement ID | Requirement | Rationale | Trace |
|---|---|---|---|
| SWR-001 | The software shall not request therapy when `therapy_inhibit` is true. | Prevent therapy during inhibit conditions. | HAZ-001; RC-002; INV-001 |
| SWR-002 | The software shall request therapy only when the previous state is `Armed`. | Prevent direct delivery from non-armed states. | HAZ-002; RC-003; INV-002 |
| SWR-003 | The software shall require `valid_therapy_detection` before entering `Armed` and before requesting therapy. | Prevent therapy without a valid indication. | HAZ-003; RC-004; INV-003 |
| SWR-004 | The software shall require `patient_connected` before entering `Monitoring`, entering `Armed`, or requesting therapy. | Prevent ineffective therapy due to absent patient connection. | HAZ-004; RC-005; INV-004 |
| SWR-005 | The software shall require `hardware_ready` and `self_test_passed` before entering `Standby` from `Off`, entering `Armed`, requesting therapy, or resetting a fault. | Prevent therapy when diagnostics or readiness are failed. | HAZ-006; RC-005; INV-004 |
| SWR-006 | The software shall remain in or return to a non-delivery state when a requested arming or delivery transition fails a safety precondition. | Keep the controller in a safe state on blocked transitions. | HAZ-001, HAZ-003, HAZ-004, HAZ-006; RC-006 |
| SWR-007 | The software shall require the `DeliverTherapy` command and all delivery preconditions before asserting `therapy_delivery_requested`. | Keep delivery request explicit and gated. | HAZ-001, HAZ-002, HAZ-003, HAZ-004, HAZ-006; RC-001, RC-003 |
| SWR-008 | The software shall transition to `Faulted` on `FaultDetected` from any state. | Provide deterministic fault response. | HAZ-005; RC-007; INV-005 |
| SWR-009 | The software shall not leave `Faulted` except through `PowerOff` or `ResetFault` with readiness and self-test preconditions satisfied. | Preserve fault latch behavior. | HAZ-005, HAZ-006; RC-008; INV-005 |
| SWR-010 | The software shall compute `therapy_output_enabled` and `therapy_delivery_requested` exclusively inside the pure transition function. | Keep a single proof and verification boundary. | HAZ-001 through HAZ-006; RC-001; INV-001 through INV-005 |

## Non-Functional Requirements

| Requirement ID | Requirement | Rationale | Trace |
|---|---|---|---|
| NFR-001 | The transition function shall be pure, deterministic, and free of I/O. | Enables unit testing and bounded formal analysis. | RC-001 |
| NFR-002 | The core state-machine crate shall have no third-party runtime dependencies in the first baseline. | Reduces SOUP burden for the proof slice. | RC-001 |
| NFR-003 | Safety-related enum and struct types shall derive `Copy`, `Clone`, `Debug`, `Eq`, and `PartialEq` where practical. | Supports clear testing and proof harness construction. | RC-001 |

## Open Requirement Work

- Add event timing requirements for therapy delivery pulse duration once the model includes time.
- Add operator-confirmation requirements once an operator interface exists.
- Add alarm and fault-reporting requirements once the output interface is expanded.
