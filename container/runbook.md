# Container Reproducibility Runbook

## Purpose

This runbook defines the first containerized reproducibility baseline for CSSI Pulse Guard.

The current container scope is the conventional Rust verification bar:

- `cargo fmt --all --check`
- `cargo test --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`

Kani is not included in this first container baseline. Kani proof execution remains available through GitHub Actions and the dedicated Kani runbook.

## Container Specification

- Container file: `Dockerfile`
- Base image: `rust:1.95.0-bookworm`
- Rust components installed in container: `clippy`, `rustfmt`
- Default command: `./container/run-rust-verification.sh`

## Build Command

```sh
docker build -t cssi-pulse-guard-rust-verification .
```

## Run Command

```sh
docker run --rm cssi-pulse-guard-rust-verification
```

## Expected Result

A successful run demonstrates that the conventional Rust verification bar can be executed in a pinned container environment without depending on the host Rust toolchain.

## CI Execution

GitHub Actions also executes the container path in `.github/workflows/ci.yml`.

The CI job:

- builds the repository `Dockerfile`
- runs the resulting container image
- treats a passing container run as reproducibility evidence for the conventional Rust verification bar

After a successful CI run, record the workflow run reference in `evidence/`.

CI evidence for commit `fe57f1158fe106cd79b5916a5856c3d7ad647754` is recorded in `evidence/container-verification-2026-04-24.md`.

## Current Limits

- The local development environment used for this update does not have Docker or Podman installed, so the container path has not been executed locally.
- Kani is not part of this first container baseline.
- The container is aimed at reproducible verification, not interactive development.
