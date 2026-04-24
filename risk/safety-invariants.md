# Safety Invariants

These invariants define the first formal proof targets for the pure therapy-control transition function.

| Invariant ID | Statement | Linked Hazards | Linked Requirements | Initial Verification |
|---|---|---|---|---|
| INV-001 | A transition must never request therapy when `therapy_inhibit` is true. | HAZ-001 | SWR-001, SWR-010 | Kani proof target; unit test |
| INV-002 | A transition must never request therapy unless the previous state is `Armed` and the command is `DeliverTherapy`. | HAZ-002 | SWR-002, SWR-007, SWR-010 | Kani proof target; unit test |
| INV-003 | A transition must never request therapy unless `valid_therapy_detection` is true. | HAZ-003 | SWR-003, SWR-010 | Kani proof target; unit test |
| INV-004 | A transition must never request therapy unless patient connection, hardware readiness, and self-test preconditions are all true. | HAZ-004, HAZ-006 | SWR-004, SWR-005, SWR-010 | Kani proof target; unit test |
| INV-005 | A transition into or within `Faulted` must not enable or request therapy. | HAZ-005, HAZ-006 | SWR-008, SWR-009, SWR-010 | Kani proof target; unit test |

## Proof Boundary

The invariants apply only to the deterministic Rust transition function. They do not prove correctness of clinical detection algorithms, hardware sensors, operator workflow, interrupt timing, compiler behavior, operating-system scheduling, hardware energy delivery, or communication security.
