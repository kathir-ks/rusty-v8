## Why

cpp2rust has been proven on exactly one V8 module (bigint): it transpiles, `cargo check` passes, and no `todo!()` leaks. That is a single anecdote, not evidence that cpp2rust is a *standard* tool. To trust it on the rest of V8 we need to transpile the engine's other major modules and **prove behavioral equivalence**, not just compilation. The natural way to do that is to slice V8's well-known pipeline (source text → scanner → parser → AST → bytecode → interpreter execution) into discrete layers, give each layer a fixed input and a known-correct expected output (ideally lifted from V8's own test corpus), and require the transpiled Rust to reproduce that output. This turns "it compiles" into "it behaves," and makes cpp2rust's maturity measurable per layer.

## What Changes

- Define a **canonical layer taxonomy** for V8: 5–10 validation layers spanning input JS → scanner/lexer → parser → AST → bytecode (Ignition) → interpreter execution, with an explicit mapping from V8 source modules/directories to layers, and rules for grouping modules that must be validated together.
- Add a **per-layer transpile + validation harness**: each layer declares its V8 source set (compile_commands subset), its input fixtures, and its expected outputs; the harness transpiles the layer with cpp2rust, builds the Rust, runs it against the inputs, and asserts the outputs match.
- Establish a **golden-fixture corpus**: a versioned, layer-organized store of (input, expected-output) pairs, sourced from real V8 examples/tests where viable, with a documented format per layer and a provenance note for each fixture.
- Add **differential (oracle) validation**: for layers where a reference output exists (V8's own output, or the original C++ compiled and run), compare transpiled-Rust output against the oracle on identical input, with explicit normalization/tolerance rules so equivalence is well-defined.
- Add a **validation scorecard**: aggregate per-layer pass rate, fixture count, and coverage into a single report so progress toward "standard tool" is trackable and regressions are caught.

## Capabilities

### New Capabilities
- `v8-layer-taxonomy`: The canonical decomposition of V8 into 5–10 validation layers and the authoritative mapping from V8 source modules/directories to those layers, including layer ordering and grouping rules.
- `layer-test-harness`: The standardized per-layer test-suite contract and runner — how a layer declares its source set, inputs, and expected outputs, and how the harness transpiles, builds, runs, and judges each layer.
- `golden-fixture-corpus`: The format, sourcing rules (including reuse of real V8 test inputs), storage layout, and provenance tracking for the (input, expected-output) fixtures each layer validates against.
- `differential-validation`: The oracle-comparison contract — how transpiled Rust output is compared against a reference (V8 or original C++) for equivalence, including normalization and tolerance rules.
- `validation-reporting`: The per-layer and aggregate scorecard contract — pass rate, fixture coverage, and regression signaling that quantify cpp2rust maturity across V8.

### Modified Capabilities
<!-- None. The validation framework consumes the existing transpiler capabilities (v8-plugin,
     phase1-emitter, etc.) unchanged; it adds new spec-level behavior rather than altering theirs. -->

## Impact

- **New code**: a validation harness package (likely `cpp2rust/validation/` or a top-level `v8-layers/` suite) plus a fixture corpus directory; layer manifests (one per layer) declaring source sets and fixtures.
- **Existing code**: consumes the current cpp2rust transpile pipeline (`cpp2rust.cli transpile --plugin v8 ...`) and the `v8` plugin as-is; may add per-module `compile_commands.json` generation for modules beyond bigint.
- **Dependencies**: needs access to V8 source for the targeted modules and, for differential validation, a way to obtain reference outputs (V8 binary/test outputs or compiled original C++). Rust toolchain (already required) is reused to build per-layer crates.
- **Existing specs**: builds on `v8-plugin`, `phase1-emitter`, `v8-bigint-benchmark`, and `cpp-extractor` without changing their requirements.
