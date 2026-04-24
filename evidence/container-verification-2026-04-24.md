# Container Verification Evidence - 2026-04-24

## Scope

GitHub Actions verification for the containerized Rust verification baseline.

## CI Run

| Field | Value |
|---|---|
| Workflow | CI |
| Event | push |
| Run ID | 24903332050 |
| Run URL | https://github.com/jsambrook/cssi-pulse-guard/actions/runs/24903332050 |
| Commit | `fe57f1158fe106cd79b5916a5856c3d7ad647754` |
| Created | 2026-04-24T17:38:43Z |
| Completed | 2026-04-24T17:39:44Z |
| Conclusion | success |

## Container Job

| Field | Value |
|---|---|
| Job | Containerized Rust verification |
| Job ID | 72926360613 |
| Result | Pass |
| Image build result | Pass |
| Container run result | Pass |

## Container Details

The CI job:

- built the repository `Dockerfile`
- produced image `cssi-pulse-guard-rust-verification`
- ran the container default command `./container/run-rust-verification.sh`

The job log showed:

- base image: `rust:1.95.0-bookworm`
- exported image digest: `sha256:67406140cc0c44c1374749df565b89526d54157ad5910c250bb72acba2fcc100`

## Verification Performed Inside The Container

The container run completed:

- `cargo fmt --all --check`
- `cargo test --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`

The container log reported:

- 6 unit tests passed
- 4 adversarial/generated tests passed
- 5 scenario tests passed

## Interpretation

This evidence supports a narrow reproducibility claim: the repository's conventional Rust verification bar can be executed successfully in a pinned container environment through GitHub Actions for commit `fe57f1158fe106cd79b5916a5856c3d7ad647754`.

This evidence does not claim:

- containerized Kani proof execution
- local container execution on the current workstation
- full system reproducibility beyond the current conventional Rust verification bar

## Warnings

GitHub Actions reported a Node.js 20 deprecation warning for workflow actions. The warning did not fail the run.
