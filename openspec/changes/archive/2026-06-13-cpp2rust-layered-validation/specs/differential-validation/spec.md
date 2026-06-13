## ADDED Requirements

### Requirement: Oracle comparison on identical input
For layers where a reference oracle exists (V8's own output, or the original C++ compiled and run), the system SHALL run the transpiled Rust and the oracle on identical input and assert their outputs are equivalent. A layer using differential validation MUST fail if the oracle and the transpiled Rust diverge on any input.

#### Scenario: Equivalent outputs pass
- **WHEN** the oracle and the transpiled Rust are run on the same input and produce equivalent outputs under the layer's equivalence rule
- **THEN** the differential check for that input passes

#### Scenario: Divergence fails
- **WHEN** the oracle and the transpiled Rust produce non-equivalent outputs for the same input
- **THEN** the differential check fails and reports the diverging input and the diff

### Requirement: Normalization and tolerance rules
The system SHALL define, per layer, explicit normalization and tolerance rules (e.g., stripping memory addresses, pointer ids, or non-deterministic ordering) applied before comparison, so that equivalence is well-defined and not defeated by incidental non-determinism.

#### Scenario: Normalization applied before compare
- **WHEN** outputs contain layer-declared incidental variation (addresses, ids, ordering)
- **THEN** the declared normalization is applied to both sides before comparison and incidental differences do not cause a failure

#### Scenario: Substantive difference survives normalization
- **WHEN** a difference remains after normalization
- **THEN** it is treated as a real divergence and fails the differential check

### Requirement: Oracle availability is explicit
The system SHALL record, per layer, whether a differential oracle is available. Layers without an available oracle MUST fall back to golden-fixture comparison and MUST NOT be silently reported as differentially validated.

#### Scenario: No oracle falls back to golden
- **WHEN** a layer has no available oracle
- **THEN** the layer is validated against golden fixtures and its report states that no differential oracle was used
