## Context

cpp2rust today has one proof point: the V8 `bigint` module transpiles to ~3,122 lines of Rust, `cargo check` passes with zero errors, and no `todo!()` leaks (the always-compiles invariant holds). That validates *compilation*, not *behavior*, and on a single module. The engine is much larger, and the value proposition — "a standard C++→Rust tool" — only holds if transpiled code does what the original did.

V8 has a famously well-layered pipeline: source text → scanner/lexer → preparser/parser → AST → bytecode generation (Ignition) → bytecode interpreter execution, with supporting runtime/heap/builtins underneath. That structure is a gift: each stage has a relatively clean input and a serializable output (token stream, AST dump, bytecode listing, interpreter result), which makes per-layer golden testing natural. V8 also ships its own test corpus (cctest, unittests, mjsunit, test262 inputs) we can mine for real (input, expected-output) pairs.

This change builds a validation framework *on top of* the existing transpiler. It does not modify the transpiler's requirements; it consumes `cpp2rust.cli transpile --plugin v8` and the `v8` plugin as they are.

## Goals / Non-Goals

**Goals:**
- Slice V8 into 5–10 named validation layers with an authoritative module→layer manifest.
- Give every targeted layer a fixed input corpus and known-correct expected outputs, preferring real V8 test data.
- Run a uniform transpile → build → run → judge cycle per layer and report pass/fail behaviorally, not just by compilation.
- Where a reference oracle exists, validate by differential comparison against V8 / original C++, with explicit normalization.
- Produce a scorecard that turns "is cpp2rust mature?" into a single trackable number, with regression detection.

**Non-Goals:**
- Full V8 functional parity or running real JS end-to-end through fully-transpiled V8. We validate layers/groups, not a working browser engine.
- Performance benchmarking (covered separately by `v8-bigint-benchmark`); this is correctness validation.
- Changing the transpiler's emission strategy or the always-compiles invariant — we enforce it, we don't redefine it.
- Transpiling V8's GC/heap internals to bit-exact behavior; layers depending on nondeterministic heap state use normalization or are deferred.

## Decisions

**D1 — Layer taxonomy (proposed 7 layers).** Source-intake → Scanner → Parser → AST → Bytecode-gen → Interpreter → Runtime-support. Seven sits in the requested 5–10 band, matches V8's actual module boundaries (`scanner`, `parser`, `ast`, `interpreter`/`compiler` for bytecode, `interpreter` execution, plus a runtime/util bucket), and lets early layers (scanner, parser) be validated before the harder execution layers. *Alternative considered:* a coarse 3-layer (frontend/midend/backend) split — rejected as too coarse to localize failures. *Alternative:* per-directory 1:1 layers (~20+) — rejected as too fine to give standardized fixture formats.

**D2 — Manifest-driven, declarative layers.** Each layer is a checked-in manifest (source set / compile_commands subset, fixture dir, expected-output format, oracle availability). The harness reads manifests; nothing is inferred at runtime. *Why:* reproducibility and reviewability — a layer's contract is a file, not code. *Alternative:* code-defined layers — rejected; harder to diff and audit.

**D3 — Golden fixtures first, differential where viable.** Default judging is golden-file comparison (deterministic, no V8 build needed). Differential/oracle comparison is layered on only where we can obtain a reference output cheaply (V8's own dumps like `--print-bytecode`, or original C++ compiled and run). *Why:* golden files give immediate coverage; oracles raise confidence but cost a V8 build/toolchain. Recording oracle-availability per layer keeps the report honest (a golden-only layer is not claimed as differentially validated).

**D4 — Reuse V8's own outputs as oracles.** V8 exposes stage dumps (`--print-bytecode`, scanner/parser test expectations in unittests) that are ideal expected outputs. Where viable we lift these directly. *Why:* hand-authoring expected ASTs/bytecode is error-prone; V8's own output is authoritative. Provenance is recorded so derived fixtures are traceable.

**D5 — Normalization layer before compare.** Outputs carry incidental nondeterminism (pointer addresses, object ids, hash-ordering). Each layer declares normalization rules applied to both sides before equivalence is judged. *Why:* without it, every run spuriously fails; with over-broad normalization, real bugs hide — so rules are explicit and per-layer, reviewable in the manifest.

**D6 — Harness location and shape.** A new `cpp2rust/validation/` package (runner, manifest loader, normalizer, reporter) plus a top-level `v8-layers/` corpus tree (one dir per layer holding manifest + fixtures). Reuses the existing `run_check` no-`todo!()` logic. *Why:* keeps validation code with the tool, keeps bulky fixtures out of the Python package.

**D7 — Scorecard as the headline artifact.** Output is a single report (machine-readable + human summary) covering every taxonomy layer (including unimplemented ones, marked), per-layer metrics, an aggregate maturity fraction, and regression diff against a stored baseline. *Why:* the user's stated goal is making cpp2rust "a more standard tool" — that needs one number that moves.

## Risks / Trade-offs

- **Obtaining V8 source / reference outputs is heavy** → Start with layers whose fixtures need only V8 stage dumps or small source subsets (scanner, parser); defer layers needing a full V8 build. Record oracle-unavailable layers honestly as golden-only.
- **Transpiled execution-layer Rust may not actually run** (Phase-1 unsafe emission compiles but may not be behaviorally correct) → Expected and *desired*: that is exactly the signal this suite exists to surface. Build-pass with run-fail is a valid, informative scorecard state, not a harness bug.
- **Normalization hides real divergences** → Keep normalization rules minimal, per-layer, and reviewed in-manifest; prefer failing loud over normalizing broadly.
- **Fixture corpus rots as V8 evolves** → Pin fixtures to a recorded V8 revision in provenance; treat a V8 bump as a deliberate corpus refresh, not silent drift.
- **Scope creep toward "run all of V8"** → Non-Goals fence this: we validate layers/groups against fixtures, not a working engine.

## Migration Plan

Additive only — no existing behavior changes. Land in order: (1) taxonomy + manifest schema, (2) harness runner against one already-working layer to prove the cycle (reuse bigint as the smoke test), (3) fixture corpus + normalization for scanner/parser (cheap oracles), (4) differential oracles where available, (5) scorecard + baseline. Rollback is deletion of `cpp2rust/validation/` and `v8-layers/`; nothing else depends on them.

## Open Questions

- Which exact V8 revision do we pin fixtures to, and is V8 source already vendored locally or must it be fetched?
- For execution-layer oracles, is building V8 (or a stage-dump-capable `d8`) acceptable in this environment, or should those layers stay golden-only initially?
- Do we want the scorecard wired into CI now, or run on demand until the corpus stabilizes?
