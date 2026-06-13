## ADDED Requirements

### Requirement: Canonical layer set
The system SHALL define a canonical set of between 5 and 10 named validation layers covering V8's execution pipeline from JavaScript source text through interpreter execution. Each layer SHALL have a unique kebab-case identifier, a one-line description, and a defined position in the pipeline ordering.

#### Scenario: Layers are enumerable and ordered
- **WHEN** the taxonomy is queried for its layers
- **THEN** it returns between 5 and 10 layers, each with a unique id, description, and integer order index with no duplicate order indices

#### Scenario: Pipeline coverage is contiguous
- **WHEN** the layers are sorted by order index
- **THEN** they form a contiguous pipeline beginning at source-text intake and ending at interpreter execution, with no gap that omits scanner, parser, AST, or bytecode stages

### Requirement: Module-to-layer mapping
The system SHALL provide an authoritative mapping from V8 source modules or directories to layers. Every source set targeted for validation MUST resolve to exactly one layer, and the mapping MUST be declared in a machine-readable manifest rather than inferred at runtime.

#### Scenario: Source set resolves to one layer
- **WHEN** a V8 source module path is looked up in the mapping
- **THEN** the mapping returns exactly one owning layer, or an explicit "unassigned" result that fails validation rather than silently defaulting

#### Scenario: Manifest is the source of truth
- **WHEN** the layer of a module is requested
- **THEN** the answer is read from the declared manifest, and a module present in no manifest entry is reported as unmapped

### Requirement: Layer grouping rules
The system SHALL define rules for grouping modules that must be transpiled and validated together (e.g., a parser and its AST nodes), so that interdependent modules are never validated in isolation when isolation would make their output meaningless.

#### Scenario: Interdependent modules grouped
- **WHEN** a layer declares a module group with internal dependencies
- **THEN** the harness treats the group as a single validation unit and does not attempt to validate a member in isolation unless the manifest marks it independently testable
