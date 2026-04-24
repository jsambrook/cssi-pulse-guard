
# Artifact Map

## Artifact Types Expected

- Rust state-machine code
- software requirements
- risk controls and traceability
- hazard analysis and safety invariants
- architecture and design documentation
- unit, integration, scenario, fault-injection, and regression tests
- Kani formal verification harnesses and property definitions
- formal verification report
- verification and validation evidence package
- AI workflow and recursive-improvement process documentation
- release and tagging process documentation
- GitHub Actions CI/CD pipeline
- Docker or container specification
- top-level portfolio explainer and business-case documentation
- Microsoft Office compatible final deliverable documents

## Core Artifacts

| Artifact | Purpose | Status |
|---|---|---|
| Project charter describing the AI-led portfolio objective and proof boundary | Initial artifact for the project | Baseline exists |
| Hazard and risk-control starter set for unintended therapy delivery and related safety concerns | Initial artifact for the project | Baseline drafted |
| Residual-risk acceptability criteria for the therapy-control proof slice | Provide objective acceptance rules for current hazard dispositions | Baseline drafted and applied |
| Initial software requirements for therapy-control state transitions and interlocks | Initial artifact for the project | Baseline drafted |
| Architecture note describing the pure Rust transition function and proof boundary | Initial artifact for the project | Baseline drafted |
| Rust therapy-control state-machine skeleton | Initial artifact for the project | Implemented with unit tests |
| Conventional automated test strategy and starter scenarios | Initial artifact for the project | Strategy, starter scenarios, and generated adversarial tests implemented |
| Formal property list linked to hazards and requirements | Initial artifact for the project | Baseline drafted |
| Initial Kani proof harness plan for shock-inhibit and armed-state invariants | Initial artifact for the project | CI proof evidence recorded for five initial invariants |
| CI/CD and containerization plan | Initial artifact for the project | Rust and Kani CI jobs pass; Rust verification container baseline drafted |
| Top-level explainer of the technical and business case for the portfolio project | Initial artifact for the project | Baseline drafted in README and business-case document |
| Release and tagging strategy | Preserve meaningful lifecycle baselines over time | Baseline drafted |

## Supporting Artifacts

| Artifact | Purpose | Status |
|---|---|---|
| Journal | Capture learning and continuity over time | Active |
| Decision log | Record important choices and consequences | Active |
| Next actions | Maintain a short list of concrete next steps | Active |
