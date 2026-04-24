
# Decision Log

## Initial Decision

### Decision

Use the CSSI Development Method project structure with the `embedded-medical-device-software` profile.

### Context

The project was created from an initial project idea and needs a consistent development process for software-like artifact development.

### Rationale

This profile best matches the expected artifacts and evidence needed for the project.

### Consequences

The project starts with a structured artifact map, journal, and folders tailored to the selected profile.

### Revisit When

Revisit if the project begins producing artifacts that do not fit the selected profile.

## Pure Transition-Function Boundary

### Decision

Implement the first therapy-control slice as a dependency-free Rust library centered on a pure `transition(current_state, inputs)` function.

### Context

The charter calls for traceable hazards, requirements, design artifacts, Rust implementation, conventional tests, and narrow Kani-based formal verification of safety invariants.

### Rationale

A pure transition function gives the project a small, reviewable proof boundary that can be linked directly to risk controls, software requirements, unit tests, and Kani harnesses.

### Consequences

The first implementation intentionally excludes timing, hardware energy delivery, clinical detection, alarms, operator UI, and communication security. Those concerns must be added as separate artifacts and interfaces rather than implied by this proof slice.

### Revisit When

Revisit when the project adds a hardware abstraction, event timing model, operator workflow, or integration layer.

## Residual Risk Acceptance For The Portfolio Slice

### Decision

Use a project-specific residual-risk acceptance matrix for the current therapy-control proof slice, with residual scores `1-4` acceptable, `5-9` conditionally acceptable with explicit rationale and evidence, and `10-25` not acceptable for the baseline release.

### Context

The hazard analysis baseline had provisional residual rationale but no objective acceptance criteria, which made the risk story weaker than the code and verification story.

### Rationale

This repository needs a defensible way to distinguish accepted low residual risk from conditionally accepted risk that is only tolerable because the proof slice is narrow and non-claims are explicit.

### Consequences

Severity-5 residual hazards now require explicit review notes even when conditionally acceptable. If the repository expands claims or scope, the current acceptability assessment must be revisited rather than silently inherited.

### Revisit When

Revisit when hardware, timing, alarms, usability, cybersecurity, or broader product claims enter scope.
