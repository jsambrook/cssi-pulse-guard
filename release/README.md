# Release Package

This folder packages the repository for external review and public-release preparation.

Start here if you want a concise view of the current release posture.

## Contents

- `public-portfolio-summary.md`: reviewer-facing summary of the project, claims, and evidence
- `public-portfolio-summary.pdf`: release-ready PDF asset generated from the Markdown summary
- `build-public-portfolio-summary.sh`: repeatable build script for the PDF asset
- `release-readiness-checklist.md`: release gating checklist for the current baseline

## Current Status

The project is now declared as the first public release baseline. The current baseline is a strong public portfolio state with:

- traceable hazards, requirements, design, implementation, tests, and evidence
- CI-recorded Kani proof results
- CI-recorded containerized verification results
- explicit proof boundaries and non-claims

The remaining work is ongoing maintenance and follow-on improvements, not core proof-slice construction.
