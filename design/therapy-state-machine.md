# Therapy State-Machine Design

## Design Intent

The first implementation slice is a pure Rust transition function:

```rust
transition(current_state, inputs) -> TransitionResult
```

This keeps state transition logic, delivery gating, unit tests, and Kani proof harnesses aligned around one deterministic boundary.

## States

| State | Meaning | Therapy Request Allowed |
|---|---|---|
| `Off` | Controller is powered down. | No |
| `Standby` | Controller is powered and diagnostics are acceptable. | No |
| `Monitoring` | Controller is monitoring with patient connection present. | No |
| `Armed` | Controller has a valid therapy detection and all readiness preconditions are satisfied. | Yes, only on `DeliverTherapy` and only if preconditions remain true |
| `Delivering` | A therapy request was issued on the prior transition. | No additional request by state alone |
| `Faulted` | A fault has latched and therapy outputs are disabled. | No |

## Inputs

| Input | Purpose |
|---|---|
| `command` | Operator, system, or fault command for the transition step. |
| `patient_connected` | Indicates required patient connection is present. |
| `therapy_inhibit` | Blocks arming and delivery when active. |
| `valid_therapy_detection` | Indicates therapy is currently justified by upstream detection logic. |
| `hardware_ready` | Indicates hardware readiness is acceptable for arming, delivery, or reset. |
| `self_test_passed` | Indicates software-visible diagnostics are acceptable. |

## Commands

| Command | Intended Use |
|---|---|
| `NoCommand` | Hold or evaluate current state without a requested action. |
| `PowerOn` | Move from `Off` to `Standby` if diagnostics and readiness pass. |
| `PowerOff` | Move to `Off` and disable outputs. |
| `StartMonitoring` | Move from `Standby` to `Monitoring` if patient connection is present. |
| `Arm` | Move from `Monitoring` to `Armed` if all arming preconditions pass. |
| `Disarm` | Move from `Armed` to `Monitoring`. |
| `DeliverTherapy` | Request therapy only from `Armed` when all delivery preconditions pass. |
| `TherapyComplete` | Move from `Delivering` to `Monitoring`. |
| `FaultDetected` | Move from any state to `Faulted`. |
| `ResetFault` | Move from `Faulted` to `Standby` if readiness and self-test pass. |

## Gating Rules

Arming requires:

- current state is `Monitoring`
- command is `Arm`
- `patient_connected` is true
- `therapy_inhibit` is false
- `valid_therapy_detection` is true
- `hardware_ready` is true
- `self_test_passed` is true

Delivery requires:

- current state is `Armed`
- command is `DeliverTherapy`
- all arming preconditions remain true

Fault reset requires:

- current state is `Faulted`
- command is `ResetFault`
- `hardware_ready` is true
- `self_test_passed` is true

## Safe-State Behavior

- `FaultDetected` dominates all other command handling.
- `PowerOff` disables outputs and moves to `Off`.
- Blocked delivery from `Armed` returns to `Monitoring` when a safety interlock has become false.
- `Faulted` never enables or requests therapy.
- The transition result reports a `blocked_reason` for test evidence and review traceability.

## Proof Boundary

Formal checks for the first baseline are limited to one-step safety invariants over the transition function. This is deliberately narrower than proving a complete device controller.
