
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
