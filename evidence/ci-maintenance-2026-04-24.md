# CI Maintenance Evidence - 2026-04-24

## Scope

GitHub Actions maintenance update for the workflow checkout action version.

## CI Run

| Field | Value |
|---|---|
| Workflow | CI |
| Event | push |
| Run ID | 24903548750 |
| Run URL | https://github.com/jsambrook/cssi-pulse-guard/actions/runs/24903548750 |
| Commit | `340e42eda2289413674318627bce789ce604edb6` |
| Created | 2026-04-24T17:43:47Z |
| Completed | 2026-04-24T17:44:44Z |
| Conclusion | success |

## Maintenance Change

All workflow checkout steps were updated from `actions/checkout@v4` to `actions/checkout@v5`.

## Evidence Summary

The latest CI run completed successfully and the log confirms the workflow used `actions/checkout@v5` in all jobs.

The log filtering performed against the Rust and container jobs did not show the prior Node.js 20 deprecation warning text. The remaining checkout log text was the normal Git hint about default branch naming inside the action environment, not the GitHub Actions Node.js warning.

## Interpretation

This evidence supports the narrow claim that the repository's CI workflow has been updated to the Node 24-compatible checkout action line and the latest run completed successfully after that change.

## Residual Notes

- the workflow still uses `dtolnay/rust-toolchain@stable` and `model-checking/kani-github-action@v1`, which did not trigger the Node.js 20 warning in this run
- future GitHub Actions warnings should be evaluated as part of routine CI maintenance
