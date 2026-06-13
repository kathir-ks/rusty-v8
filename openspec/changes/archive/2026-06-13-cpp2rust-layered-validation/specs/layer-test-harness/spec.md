## ADDED Requirements

### Requirement: Layer manifest declares its test contract
Each layer SHALL be described by a manifest that declares its V8 source set (or a reference to a compile_commands subset), its input fixtures, and its expected outputs. The harness MUST refuse to run a layer whose manifest is missing any of these three elements.

#### Scenario: Complete manifest runs
- **WHEN** a layer manifest declares source set, inputs, and expected outputs
- **THEN** the harness accepts the layer and proceeds to transpile-build-run-judge

#### Scenario: Incomplete manifest is rejected
- **WHEN** a layer manifest omits its source set, its inputs, or its expected outputs
- **THEN** the harness reports the layer as misconfigured and does not mark it as passed

### Requirement: Transpile-build-run-judge cycle
The harness SHALL, for each layer, transpile the declared source set with cpp2rust, build the resulting Rust, run it against each input fixture, and compare the produced output against the expected output. A layer SHALL be reported as passing only if every stage of this cycle succeeds for every fixture.

#### Scenario: Full cycle pass
- **WHEN** the layer transpiles, the Rust builds, and every fixture's output matches its expected output
- **THEN** the harness reports the layer as PASS

#### Scenario: Build failure fails the layer
- **WHEN** the transpiled Rust for a layer fails to build
- **THEN** the harness reports the layer as FAIL with the build stage identified, and does not run downstream fixtures

#### Scenario: Output mismatch fails the layer
- **WHEN** at least one fixture's produced output does not match its expected output
- **THEN** the harness reports the layer as FAIL and identifies the failing fixture(s)

### Requirement: Always-compiles invariant is enforced
The harness SHALL enforce the cpp2rust always-compiles invariant for every transpiled layer by verifying the output contains no `todo!()`. A layer whose transpiled output contains `todo!()` MUST be reported as FAIL regardless of fixture results.

#### Scenario: todo!() leak fails the layer
- **WHEN** the transpiled Rust for a layer contains a `todo!()` token
- **THEN** the harness reports the layer as FAIL citing the invariant violation
