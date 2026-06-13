# V8 layered validation corpus

This tree is the data side of the cpp2rust layered validation harness
(`cpp2rust/validation/`). It slices V8 into a canonical set of validation layers
and gives each one a fixed input corpus and known-correct expected outputs, so
that "cpp2rust transpiled this module" can be upgraded to "the transpiled module
*behaves* correctly," measured per layer.

## Layout

```
v8-layers/
  taxonomy.yaml          # the 5–10 canonical layers + pinned V8 revision
  module_map.yaml        # authoritative V8 source-path -> layer mapping
  FORMATS.md             # per-layer input / expected-output formats
  baseline.json          # last-recorded scorecard, for regression detection
  <layer>/
    manifest.yaml        # this layer's source set, fixtures, oracle, groups
    fixtures/
      <name>.in              # input artifact
      <name>.expected        # known-correct expected output
      <name>.provenance.yaml # v8-derived vs authored (+ source ref)
```

## Running

```bash
# Whole suite (transpile + no-todo + build + run/judge where runnable)
python3 -m cpp2rust.cli validate

# One layer, skip the cargo build (fast)
python3 -m cpp2rust.cli validate --layer scanner --no-build

# Machine-readable scorecard
python3 -m cpp2rust.cli validate --json

# Re-baseline after intentional changes
python3 -m cpp2rust.cli validate --update-baseline
```

Scorecard statuses: `PASS` (every stage + fixture passed), `FAIL` (hard failure),
`BUILT` (transpiled + built clean, no behavioral runner yet), `PEND` (could not
start — e.g. no compile_commands shim), `----` (no manifest / unimplemented).

## Adding a new layer

1. **Confirm the layer exists in `taxonomy.yaml`.** If it is a genuinely new
   pipeline stage, add it there (keeping the layer count in 5–10 and order
   indices contiguous). Map its V8 source dirs in `module_map.yaml`.
2. **Create `<layer>/manifest.yaml`** with the three mandatory elements — the
   loader rejects a manifest missing any of them:
   - a **source set**: either `compile_commands: <path>` (preferred; lets the
     layer actually transpile) or `source_set: [<files>]`;
   - an **expected-output format**: `output_format: <name>` (see `FORMATS.md`);
   - **inputs**: a `fixtures_dir` (default `fixtures`) that exists.
   Optionally declare an `oracle` (e.g. `d8 --print-bytecode`) with `normalize`
   rules, and `groups` for modules that must validate together.
3. **Add fixtures** under `fixtures/` as `<name>.in` + `<name>.expected`, plus a
   `<name>.provenance.yaml` (`source: v8` with a `ref`, or `source: authored`).
   Prefer real V8 inputs/outputs where viable; record the decision in the
   manifest's `reuse_real_v8` / `reuse_note`.
4. **Run** `python3 -m cpp2rust.cli validate --layer <layer>` and iterate until
   the layer reaches the state you expect (`BUILT` until a runner exists, `PASS`
   once fixtures judge green).
5. **Re-baseline** with `--update-baseline` once the new state is intentional.

## Differential oracles

A layer with an available oracle is judged against the oracle's fresh output on
the same input (after normalization). When the oracle binary is absent (e.g. no
`d8` in this environment), the harness falls back to the checked-in golden
`.expected` and reports `golden(oracle-unavailable)` — it never silently claims a
differential pass. See `cpp2rust/validation/oracle.py`.
