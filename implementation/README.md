# Implementation Artifacts

This folder contains implementation artifacts for the therapy-control proof slice.

Current crate:

- `pulse_guard_core`: pure Rust transition-function library with unit tests and initial Kani proof harnesses guarded by `cfg(kani)`.

Related reproducibility artifact:

- `/Dockerfile`: pinned Rust verification container baseline
