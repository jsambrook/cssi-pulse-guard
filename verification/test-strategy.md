# Test Strategy Baseline

## Purpose

This strategy defines the first conventional automated tests for the therapy-control state-machine slice. It complements the Kani proof plan by checking representative command sequences and fault paths that reviewers can read as operational scenarios.

## Test Levels

| Level | Location | Purpose | Current Status |
|---|---|---|---|
| Unit tests | `implementation/pulse_guard_core/src/lib.rs` | Check focused transition rules and individual safety interlocks. | Implemented |
| Scenario tests | `implementation/pulse_guard_core/tests/scenarios.rs` | Check multi-step command sequences across normal, blocked, inhibit, fault, reset, and power-off behavior. | Implemented |
| Kani proofs | `implementation/pulse_guard_core/src/lib.rs` under `cfg(kani)` | Check selected one-step safety invariants over modeled state and command inputs. | Harnesses drafted; not executed locally |
| Fault-injection tests | Future test module | Exercise expanded fault inputs and recovery paths as the model grows. | Not started |

## Scenario Coverage

| Scenario ID | Test | Intent | Trace |
|---|---|---|---|
| SCN-001 | `nominal_sequence_powers_on_monitors_arms_delivers_and_completes` | Verify the nominal path from `Off` through therapy request and completion. | SWR-002, SWR-003, SWR-004, SWR-005, SWR-007 |
| SCN-002 | `blocked_arming_sequence_stays_in_monitoring_until_preconditions_recover` | Verify arming remains blocked until therapy detection returns. | HAZ-003; SWR-003, SWR-006 |
| SCN-003 | `inhibit_during_armed_sequence_disarms_without_requesting_therapy` | Verify inhibit prevents delivery after arming and drops to monitoring. | HAZ-001; SWR-001, SWR-006, SWR-010 |
| SCN-004 | `fault_sequence_latches_fault_then_resets_only_after_readiness_returns` | Verify fault latching and readiness-gated reset behavior. | HAZ-005, HAZ-006; SWR-008, SWR-009 |
| SCN-005 | `power_off_from_any_active_sequence_disables_outputs_and_clears_fault_latch` | Verify power-off disables therapy-related outputs from an active state. | SWR-010 |

## Evidence Expectations

Each verification evidence entry should include:

- command used to run the tests
- number of unit and scenario tests passed
- toolchain version when available
- unresolved gaps, especially Kani execution and containerized reproducibility

## Current Limits

- Scenario tests are deterministic examples, not exhaustive proofs.
- No timing, alarm, UI, hardware abstraction, or clinical detection behavior is represented.
- No adversarial test-generation workflow has been added yet.
