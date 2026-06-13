## 1. Layer taxonomy & manifest schema

- [x] 1.1 Define the canonical 7-layer taxonomy (source-intake, scanner, parser, ast, bytecode-gen, interpreter, runtime-support) in a machine-readable `v8-layers/taxonomy.yaml` with id, description, and order index per layer
- [x] 1.2 Define the module→layer manifest schema (source set / compile_commands subset, fixture dir, expected-output format, oracle availability, grouping)
- [x] 1.3 Author the authoritative module→layer mapping for the targeted V8 directories; mark unmapped modules explicitly
- [x] 1.4 Define layer grouping rules so interdependent modules (e.g., parser + ast) validate as one unit
- [x] 1.5 Write a manifest loader/validator that rejects layers missing source set, inputs, or expected outputs

## 2. Harness runner core

- [x] 2.1 Scaffold `cpp2rust/validation/` package (runner, manifest loader, normalizer, reporter)
- [x] 2.2 Implement per-layer transpile step invoking the existing `cpp2rust.cli transpile --plugin v8` with the layer's source set
- [x] 2.3 Implement the build step (`cargo build`/`cargo check`) on the transpiled layer crate
- [x] 2.4 Reuse `run_check` no-`todo!()` logic to enforce the always-compiles invariant per layer
- [x] 2.5 Implement the run step that executes the built Rust against each input fixture
- [x] 2.6 Implement the judge step comparing produced vs expected output; report PASS only if every stage passes for every fixture
- [x] 2.7 Smoke-test the full cycle against the already-working `bigint` layer

## 3. Golden fixture corpus

- [x] 3.1 Define the per-layer fixture format (input artifact + expected-output artifact) and document each layer's formats
- [x] 3.2 Implement provenance tracking (V8-derived vs authored, with source reference)
- [x] 3.3 Assemble scanner-layer fixtures, preferring real V8 inputs/expected outputs where viable
- [x] 3.4 Assemble parser/ast-layer fixtures, recording reuse-vs-authored decision per layer
- [x] 3.5 Record the pinned V8 revision in fixture provenance

## 4. Differential validation

- [x] 4.1 Implement the oracle-comparison path: run transpiled Rust and oracle on identical input and assert equivalence
- [x] 4.2 Implement per-layer normalization rules (strip addresses, ids, nondeterministic ordering) applied before compare
- [x] 4.3 Wire V8 stage dumps (e.g., `--print-bytecode`, scanner/parser unittest expectations) as oracles where viable
- [x] 4.4 Record per-layer oracle availability; fall back to golden comparison and report when no oracle exists

## 5. Validation reporting

- [x] 5.1 Implement the per-layer scorecard (pass/fail, fixtures-run, fixtures-passed, differential-used) covering every taxonomy layer including unimplemented ones
- [x] 5.2 Compute the aggregate maturity metric (fraction of layers passing, total fixtures passing)
- [x] 5.3 Implement baseline storage and regression signaling (distinguish regressions from never-passing layers)
- [x] 5.4 Emit both machine-readable and human-readable report output

## 6. Integration & docs

- [x] 6.1 Add a `cpp2rust.cli validate` (or equivalent) entry point to run the full layer suite
- [x] 6.2 Document how to add a new layer (manifest + fixtures) end to end
- [x] 6.3 Run the suite across implemented layers and record the initial scorecard as the baseline
