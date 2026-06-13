## ADDED Requirements

### Requirement: Per-layer scorecard
The system SHALL produce a scorecard reporting, for each layer, its pass/fail status, the number of fixtures run, the number passed, and whether differential validation was used. The scorecard MUST cover every layer declared in the taxonomy, marking unimplemented layers explicitly rather than omitting them.

#### Scenario: Every taxonomy layer appears
- **WHEN** the scorecard is generated
- **THEN** it contains one row per layer in the taxonomy, including layers not yet implemented, marked as such

#### Scenario: Per-layer metrics present
- **WHEN** a layer has been run
- **THEN** its scorecard row reports pass/fail, fixtures-run, fixtures-passed, and differential-used

### Requirement: Aggregate maturity metric
The system SHALL compute an aggregate metric summarizing cpp2rust maturity across V8 — at minimum the fraction of taxonomy layers passing and the total fixtures passing — so progress toward "standard tool" is a single trackable number.

#### Scenario: Aggregate computed from layers
- **WHEN** the scorecard is generated
- **THEN** it includes an aggregate showing the fraction of layers passing and total fixtures passing across all layers

### Requirement: Regression signaling
The system SHALL compare a run against a stored baseline and signal regressions when a previously-passing layer or fixture fails. A regression MUST be distinguishable in the report from a layer that has never passed.

#### Scenario: Regression flagged
- **WHEN** a layer that passed in the baseline fails in the current run
- **THEN** the report flags it as a regression, distinct from never-passing layers

#### Scenario: New failure is not a regression
- **WHEN** a layer that has never passed fails again
- **THEN** the report lists it as outstanding rather than as a regression
