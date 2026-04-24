# Formal Property Baseline

## Purpose

This artifact maps the first Kani proof targets to hazards, risk controls, requirements, and implementation scope.

## Property Map

| Property ID | Kani Harness | Statement | Trace |
|---|---|---|---|
| PROP-001 | `inv_001_inhibit_blocks_delivery_request` | For all modeled states and commands, `therapy_delivery_requested` is false when `therapy_inhibit` is true. | HAZ-001; RC-002; SWR-001; INV-001 |
| PROP-002 | `inv_002_delivery_requires_armed_deliver_command` | If a transition requests therapy, the previous state was `Armed` and the command was `DeliverTherapy`. | HAZ-002; RC-003; SWR-002, SWR-007; INV-002 |
| PROP-003 | `inv_003_delivery_requires_valid_detection` | For all modeled states and commands, therapy is not requested when valid therapy detection is false. | HAZ-003; RC-004; SWR-003; INV-003 |
| PROP-004 | `inv_004_delivery_requires_connection_readiness_and_self_test` | If a transition requests therapy, patient connection, hardware readiness, and self-test preconditions are true. | HAZ-004, HAZ-006; RC-005; SWR-004, SWR-005; INV-004 |
| PROP-005 | `inv_005_faulted_never_requests_or_enables_therapy` | From `Faulted`, a transition never enables or requests therapy. | HAZ-005; RC-007, RC-008; SWR-008, SWR-009; INV-005 |

## Current Status

The harnesses are embedded in `implementation/pulse_guard_core/src/lib.rs` under `cfg(kani)`. They are not run by default `cargo test`.

## Claim Boundary

Passing these properties would support only a narrow claim: the modeled one-step transition function satisfies selected delivery-gating invariants over bounded state and command inputs. It would not prove the complete device, clinical algorithm, hardware interface, runtime scheduler, compiler, or operator workflow.
