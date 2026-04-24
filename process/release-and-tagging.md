# Release and Tagging Strategy

## Purpose

This strategy defines how CSSI Pulse Guard will use Git commits and tags to preserve meaningful lifecycle baselines. The goal is to make the repository reviewable over time as an AI-led medical-device software portfolio project, with traceable artifacts, verification evidence, and honest proof-boundary claims.

## Commit Strategy

Commits should represent coherent development increments. A good commit leaves the repository in a reviewable state and should normally include any artifact updates needed to explain the code or verification changes.

Examples of coherent commits:

- add a hazard-analysis baseline and linked safety invariants
- add a requirements/design/code/test slice for a state-machine feature
- add verification evidence for a completed test or proof activity
- refine the process or release documentation

Avoid commits that update code without the corresponding traceability or evidence artifacts when those artifacts are affected.

## Tag Strategy

Use annotated milestone tags for lifecycle baselines. Tags are not required for every work session or every commit.

Tag format:

```text
vMAJOR.MINOR.PATCH-qualifier
```

Examples:

```text
v0.1.0-charter-scaffold
v0.2.0-traceable-state-machine
v0.3.0-scenario-vv-baseline
v0.4.0-kani-proof-baseline
v0.5.0-containerized-ci-baseline
v1.0.0-public-portfolio-release
```

## Version Meaning

| Version Segment | Meaning Before Public Release |
|---|---|
| `v0.x.0` | Meaningful lifecycle baseline before the first public portfolio release. |
| `v0.x.y` | Correction or small refinement to an existing baseline. |
| `v1.0.0` | First public release-ready portfolio baseline. |
| `v1.x.y` | Public-release improvements, corrections, or additional evidence packages. |

## Tag Qualification Rules

A lifecycle tag should be created only when:

- the repository has a coherent artifact/code/evidence baseline
- the verification bar documented for that baseline has been run or explicitly recorded as unavailable
- project-control artifacts such as `artifact-map.md`, `next-actions.md`, and `journal.md` reflect the baseline
- non-claims and proof-boundary limits are clear

## Milestone Threshold

Not every good commit deserves a tag. A commit should be tagged only when it crosses a lifecycle boundary that changes what the repository can credibly claim.

Use this decision rule:

- commit every coherent development increment
- tag only when the increment creates a new externally meaningful baseline

A baseline is externally meaningful when at least one of these is true:

- it adds a new artifact class that materially improves reviewability, such as the first hazard baseline, the first traceable implementation slice, the first formal-proof evidence, or the first containerized reproducibility path
- it changes the verification claim the repository can honestly make
- it makes the repository substantially more usable for an external audience, such as a release-ready explainer or a public portfolio baseline
- it establishes a stable checkpoint that future work is likely to build on rather than immediately replace

An increment is usually not tag-worthy when it only:

- refines an existing artifact without changing the repository's external claim
- adds depth inside an already-established baseline without creating a new lifecycle boundary
- fixes small issues after a tag-worthy baseline unless the fixes justify a patch tag
- records routine maintenance, housekeeping, or process clarifications

## Working Rule For This Project

Use the following default rule during day-to-day work:

- if the commit answers "what new thing can a serious reviewer now verify or understand that they could not before?" with a materially different answer, create a new lifecycle tag
- if the answer is "the same baseline, but better refined," make a commit without a new tag

Applied examples from this repository:

- `v0.3.0-scenario-vv-baseline` was tag-worthy because it created the first conventional scenario-test baseline
- `v0.4.0-kani-proof-baseline` was tag-worthy because it created the first recorded formal-proof evidence baseline
- the generated adversarial regression tests were a strong development increment, but not automatically a new tag because they extended an existing verification baseline rather than creating a new artifact class or external proof claim
- the portfolio explainer and business-case draft may become tag-worthy later if refined into a release-quality external baseline, but the initial draft alone does not yet force a new milestone

## Tie-Breaker Rule

If it is unclear whether to tag, do not tag immediately. Commit the work first, continue until the milestone boundary is clearer, and then tag the commit that best represents the stable baseline.

This bias is intentional. It is better to have slightly fewer, cleaner tags than too many tags that do not represent meaningfully different lifecycle checkpoints.

## Annotated Tag Message Content

Use annotated tags:

```sh
git tag -a v0.3.0-scenario-vv-baseline -m "v0.3.0 scenario V&V baseline"
```

The tag message should summarize:

- artifact scope
- verification performed
- known gaps
- explicit non-claims when safety or formal verification is involved

## Current Planned Milestones

| Tag | Intended Baseline |
|---|---|
| `v0.1.0-charter-scaffold` | Initial CSSI development-method scaffold with charter, artifact map, journal, and folder structure. |
| `v0.2.0-traceable-state-machine` | First traceable hazards, requirements, design, Rust transition function, unit tests, and Kani harness plan. |
| `v0.3.0-scenario-vv-baseline` | Adds conventional scenario tests and test-strategy traceability for the therapy-control slice. |
| `v0.4.0-kani-proof-baseline` | Kani execution path exists and proof results are recorded as evidence. |
| `v0.5.0-containerized-ci-baseline` | CI and containerized reproducibility are documented and verified. |
| `v1.0.0-public-portfolio-release` | Release-ready public portfolio baseline with business-facing explainer, traceability, verification evidence, and explicit non-claims. |

## Retagging Policy

Do not move published tags except to correct an obvious local mistake before sharing. If a tagged baseline needs correction after it has been shared, create a patch tag such as `v0.3.1-scenario-vv-baseline`.

## Evidence Expectations

Each milestone tag should be supported by evidence in `evidence/` or by a documented reason that evidence could not yet be generated. For example, if Kani is unavailable locally, that gap should be recorded rather than implied as complete.
