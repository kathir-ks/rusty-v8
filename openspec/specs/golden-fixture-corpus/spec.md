# golden-fixture-corpus Specification

## Purpose

Maintain a corpus of (input, expected-output) fixtures per validation layer in a documented, layer-appropriate format, with provenance tracking and a preference for reusing real V8 test inputs over hand-authored ones.

## Requirements

### Requirement: Fixture format per layer
The corpus SHALL store each fixture as an (input, expected-output) pair in a documented, layer-appropriate format (e.g., JS source text in, token stream / AST dump / bytecode listing out). Each layer's expected-output format MUST be documented so a fixture can be authored and judged without reading harness source.

#### Scenario: Fixture is a self-describing pair
- **WHEN** a fixture is loaded for a layer
- **THEN** it exposes a distinct input artifact and a distinct expected-output artifact in the format documented for that layer

#### Scenario: Format is documented
- **WHEN** a contributor inspects a layer's corpus
- **THEN** the layer provides a written description of its input and expected-output formats sufficient to author a new fixture

### Requirement: Provenance tracking
Every fixture SHALL record its provenance — whether it was lifted from a real V8 test/example or authored for this suite — including a source reference when derived from V8. Fixtures derived from V8 sources MUST be distinguishable from hand-authored ones.

#### Scenario: V8-sourced fixture cites origin
- **WHEN** a fixture is derived from a real V8 example or test
- **THEN** its provenance record names the originating V8 source location or test

#### Scenario: Authored fixture is marked
- **WHEN** a fixture is authored specifically for this suite
- **THEN** its provenance record marks it as authored rather than V8-derived

### Requirement: Reuse real V8 inputs where viable
The corpus SHALL prefer real V8 test inputs and expected outputs where they can be obtained and are viable for a layer, falling back to authored fixtures only where reuse is not feasible. The decision MUST be recorded per layer.

#### Scenario: Reuse recorded
- **WHEN** a layer's fixtures are assembled
- **THEN** the layer records whether real V8 inputs were reused or why authored fixtures were used instead
