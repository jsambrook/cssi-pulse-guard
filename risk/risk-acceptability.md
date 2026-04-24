# Residual Risk Acceptability Criteria

## Purpose

This artifact defines objective residual-risk acceptability criteria for the current CSSI Pulse Guard therapy-control proof slice.

The criteria apply only to the software hazard baseline documented in `hazard-analysis.md`. They are a portfolio artifact, not a complete medical-device risk-management procedure.

## Scope

These criteria apply to residual software risk after the currently modeled controls are implemented and verified for the pure transition-function boundary.

They do not cover:

- clinical benefit-risk analysis
- hardware energy-delivery behavior
- usability validation
- alarm effectiveness
- cybersecurity risk
- production deployment controls

## Rating Basis

Residual risk uses the severity and probability scales already defined in `hazard-analysis.md`.

Residual risk priority is estimated as:

```text
severity x residual probability
```

## Acceptance Bands

| Residual Score | Band | Interpretation | Required Disposition |
|---:|---|---|---|
| 1-4 | Acceptable | Residual risk is low enough for this portfolio proof slice when controls and verification evidence are present. | May be accepted with documented rationale and traceability. |
| 5-9 | Conditionally acceptable | Residual risk may be acceptable for this portfolio slice only if verification evidence is strong, proof-boundary limits are explicit, and no practical additional in-scope control is obvious. | Requires explicit rationale, linked evidence, and review note. |
| 10-25 | Not acceptable for baseline release | Residual risk remains too high for the current repository baseline. | Additional controls, narrower claims, or scope reduction required before acceptance. |

## Additional Acceptance Rules

Residual risk is not considered acceptable unless all of the following are true:

- linked risk controls are implemented in the repository
- linked verification activities exist and have passed, or any gap is explicitly recorded
- the proof boundary and non-claims remain aligned with the available evidence
- no obvious, in-scope, lower-cost control has been skipped without rationale

Any hazard with severity `5` requires explicit review even if the residual score falls in the conditionally acceptable band.

## Portfolio-Specific Interpretation

For this repository, conditional acceptability is allowed because the artifact is an outward-facing proof slice rather than a market-submission package. That allowance depends on explicit non-claims and on keeping the scope narrow enough that reviewers can see what has and has not been verified.

This is not permission to carry high residual risk silently. It is permission to preserve a narrowly scoped portfolio baseline while documenting what still requires future controls or broader system evidence.

## Review Trigger Conditions

Revisit the acceptability assessment when any of the following occur:

- a new hazard is added
- a control or verification result changes
- the proof boundary expands beyond the pure transition function
- the repository begins making stronger business or technical claims
- hardware, timing, alarm, UI, or cybersecurity behavior enters scope
