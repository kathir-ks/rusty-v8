# dependency-wirer Specification

## Purpose

Build the cross-crate dependency graph from `#include` directives, emit correct `[dependencies]` in each crate's Cargo.toml, break dependency cycles, and keep the workspace members list in sync.

## Requirements

### Requirement: Build #include graph from extracted files
The dependency wirer SHALL construct a directed graph where each node is a crate and each edge represents a cross-crate dependency inferred from `#include` directives in extracted files.

#### Scenario: Include from another crate creates dependency edge
- **WHEN** `src/bigint/bigint.cc` includes `src/base/bits.h` and `src/base/` maps to `v8-base`
- **THEN** `v8-bigint` has a dependency edge to `v8-base`

#### Scenario: Intra-crate includes create no edge
- **WHEN** `src/bigint/bigint.cc` includes `src/bigint/bigint-internal.h`
- **THEN** no new dependency edge is created (same crate)

### Requirement: Emit correct [dependencies] in each crate's Cargo.toml
For each crate, the wirer SHALL emit a `[dependencies]` section listing all crates it depends on, using `path = "../<crate-name>"` for workspace-local crates.

#### Scenario: v8-bigint Cargo.toml depends on v8-base
- **WHEN** the include graph shows `v8-bigint` depends on `v8-base`
- **THEN** `output/transpiled/crates/v8-bigint/Cargo.toml` contains `v8-base = { path = "../v8-base" }`

### Requirement: Detect and break dependency cycles
The wirer SHALL detect cycles in the include graph (which map to circular crate dependencies, illegal in Cargo) and resolve them by merging the cyclic crates into a single crate.

#### Scenario: Cyclic headers merged into one crate
- **WHEN** `src/foo/a.h` includes `src/bar/b.h` and `src/bar/b.h` includes `src/foo/a.h`
- **THEN** `v8-foo` and `v8-bar` are merged into a single `v8-foo-bar` crate with a warning emitted to stderr

### Requirement: workspace Cargo.toml `members` list kept in sync
The top-level `output/transpiled/Cargo.toml` SHALL list all generated crates in its `members` array, with no stale or missing entries.

#### Scenario: New crate appears in workspace members
- **WHEN** a new module is added to the V8 codebase and transpiled
- **THEN** the workspace `Cargo.toml` is regenerated with the new crate in `members`
