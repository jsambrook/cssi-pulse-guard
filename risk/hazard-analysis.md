# Hazard Analysis and Risk-Control Baseline

## Scope

This baseline covers the software-controlled therapy state machine for a plausible pulse-therapy controller. It is a portfolio artifact, not a complete medical-device risk-management file.

The analysis focuses on software hazards that can be represented at the pure transition-function boundary:

- therapy requested when an inhibit interlock is active
- therapy requested before the controller is armed
- therapy requested without a valid therapy indication
- therapy requested when patient connection, hardware readiness, or self-test conditions are not satisfied
- failure to enter or remain in a safe state after detected faults

Out of scope for this first baseline: clinical validity, hardware energy delivery, sensing accuracy, alarm audibility, cybersecurity, manufacturing controls, and post-market feedback handling.

## Standards Basis

- ISO 14971:2019 is treated as the risk-management process reference. The official ISO page states the 2019 edition was reviewed and confirmed in 2025.
- IEC 62304:2006 with Amendment 1:2015 is treated as the medical-device software lifecycle reference for software requirements, architecture, implementation, verification, and traceability expectations.
- FDA design-control expectations are treated as lifecycle context, not as a claim of regulatory submission readiness.

## Severity and Probability Scale

| Value | Severity | Probability |
|---|---|---|
| 1 | negligible or no patient harm | improbable in normal or foreseeable use |
| 2 | transient discomfort or delayed workflow | remote |
| 3 | reversible injury or clinically meaningful delay | occasional |
| 4 | serious injury requiring intervention | probable without controls |
| 5 | death or irreversible serious injury | frequent without controls |

Initial risk priority is estimated as severity x probability. Residual values are reviewed against the project-specific acceptability criteria in `risk-acceptability.md`.

## Hazard Baseline

| Hazard ID | Hazardous Situation | Foreseeable Cause | Harm | Initial SxP | Residual SxP | Acceptance | Risk Controls | Linked Requirements | Linked Invariants | Residual Rationale |
|---|---|---|---|---:|---:|---|---|---|---|---|
| HAZ-001 | Therapy is requested while an inhibit interlock is active | command accepted without checking inhibit signal; stale interlock state | serious unintended therapy | 5x3=15 | 5x1=5 | Conditionally acceptable | RC-001, RC-002, RC-006 | SWR-001, SWR-006, SWR-010 | INV-001, INV-004 | transition logic blocks delivery and clears armed state when inhibit is active; unit, scenario, generated, and Kani evidence exist |
| HAZ-002 | Therapy is requested before the controller is armed | direct command path bypasses arming state | serious unintended therapy | 5x2=10 | 5x1=5 | Conditionally acceptable | RC-001, RC-003, RC-006 | SWR-002, SWR-007, SWR-010 | INV-002, INV-004 | delivery request is only possible from `Armed`; unit, generated, and Kani evidence exist |
| HAZ-003 | Therapy is requested without a valid therapy indication | false command accepted without rhythm/therapy-detection evidence | inappropriate therapy or delayed correct treatment | 5x3=15 | 5x1=5 | Conditionally acceptable | RC-001, RC-004, RC-006 | SWR-003, SWR-006, SWR-010 | INV-003, INV-004 | arming and delivery require a valid detection flag; residual acceptance depends on the explicit non-claim that clinical detection is out of scope |
| HAZ-004 | Therapy is requested when the patient connection is absent | disconnected lead/pad status ignored | ineffective therapy; delayed treatment | 4x3=12 | 4x1=4 | Acceptable | RC-001, RC-005, RC-006 | SWR-004, SWR-006, SWR-010 | INV-004 | arming and delivery require patient connection present; unit, scenario, generated, and Kani evidence exist |
| HAZ-005 | Therapy remains enabled after a detected fault | fault command does not latch safe state | unintended or uncontrolled therapy | 5x2=10 | 5x1=5 | Conditionally acceptable | RC-007, RC-008 | SWR-008, SWR-009, SWR-010 | INV-005 | fault transition disables delivery and latches `Faulted` until reset preconditions pass; unit, scenario, generated, and Kani evidence exist |
| HAZ-006 | Therapy is requested when hardware readiness or self-test status is failed | readiness and diagnostic status not included in gating logic | ineffective or uncontrolled therapy | 5x2=10 | 5x1=5 | Conditionally acceptable | RC-001, RC-005, RC-007 | SWR-005, SWR-006, SWR-010 | INV-004, INV-005 | arming, delivery, and fault reset require hardware-ready and self-test-passed inputs; acceptance is limited to the software proof slice |

## Risk Controls

| Control ID | Control | Control Type | Verification Approach |
|---|---|---|---|
| RC-001 | Implement a single pure transition function that gates all state changes and therapy requests. | software architecture | design review; unit tests; Kani harness review |
| RC-002 | Block therapy request whenever therapy inhibit is active. | protective measure | unit test; Kani invariant `INV-001` |
| RC-003 | Permit therapy request only from the `Armed` state. | protective measure | unit test; Kani invariant `INV-002` |
| RC-004 | Require valid therapy detection before arming and before therapy request. | protective measure | unit test; Kani invariant `INV-003` |
| RC-005 | Require patient-connected, hardware-ready, and self-test-passed inputs before arming, delivery, and fault reset. | protective measure | unit test; Kani invariant `INV-004` |
| RC-006 | Drop back to monitoring when an armed delivery attempt is blocked by a safety interlock. | inherent safe-state design | unit test; scenario test |
| RC-007 | Latch detected faults into `Faulted` with therapy output disabled. | protective measure | unit test; Kani invariant `INV-005` |
| RC-008 | Permit fault reset only when readiness and self-test preconditions are satisfied. | protective measure | unit test; scenario test |

## Open Review Items

The current proof slice does not model these areas, but the public release package should still state the boundary clearly:

- Usability: operator arming, confirmation, and alarm-response hazards are out of scope because there is no user-interface model in the current proof boundary.
- Hardware interface: sensor, actuator, and hardware-abstraction failure modes are out of scope because the current proof boundary stops at the pure Rust transition function.
- Cybersecurity: command injection, spoofing, and authentication failures are out of scope because there is no communications or trust-boundary model in the current proof boundary.

Future work should add hazard entries if the repository expands into those areas, and severity-5 residual acceptance should be revisited if the proof boundary broadens.
