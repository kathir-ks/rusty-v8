# v8-bigint-benchmark Specification

## Purpose

Validate the transpiler end-to-end on V8's bigint module: the `v8-bigint` crate must compile cleanly, produce bit-identical arithmetic results to the C++ original, and run a pure-Rust benchmark harness with no linked C++ code.

## Requirements

### Requirement: v8-bigint crate compiles with zero errors
After running `cpp2rust` on `codebase/src/bigint/` with the V8 plugin, `cargo build -p v8-bigint` SHALL exit with code 0 and zero errors.

#### Scenario: Full build succeeds
- **WHEN** `cpp2rust transpile --plugin v8 --module bigint` completes
- **THEN** `cd output/transpiled && cargo build -p v8-bigint` exits 0

### Requirement: Bigint arithmetic produces correct results
The transpiled Rust bigint functions SHALL produce bit-identical results to the C++ originals for all tested inputs.

#### Scenario: Multiply correctness
- **WHEN** `BigIntMultiply` is called in Rust with the same inputs as the C++ version
- **THEN** the digit arrays of the result are identical

#### Scenario: Add and Subtract correctness
- **WHEN** `BigIntAdd` and `BigIntSubtract` are called with random 64-digit inputs
- **THEN** Rust and C++ results are identical for all tested cases

#### Scenario: ToString correctness
- **WHEN** `BigIntToString` is called on a large bigint in bases 10 and 16
- **THEN** the Rust output string matches the C++ output string exactly

### Requirement: Benchmark harness measures Rust vs C++ performance
A benchmark harness SHALL measure wall-clock time for bigint Multiply, Divide (schoolbook), Add, and ToString operations and report Rust/C++ speedup ratios.

#### Scenario: Benchmark runs end-to-end
- **WHEN** `python3 benchmark/run_bigint_bench.py` is executed
- **THEN** it outputs a table of operations, C++ time (ns), Rust time (ns), and speedup ratio

#### Scenario: Benchmark uses matching input sizes
- **WHEN** the benchmark runs
- **THEN** both C++ and Rust are tested on identical input data (same digit arrays, same sizes: 10, 100, 1000 digits)

### Requirement: No C++ code linked into the benchmark binary
The Rust benchmark binary SHALL be a pure Rust executable with no `extern "C"` calls to V8 C++ code. All bigint logic runs in Rust.

#### Scenario: ldd shows no V8 shared library dependency
- **WHEN** the benchmark binary is inspected with `ldd` or `readelf`
- **THEN** no V8 `.so` or `.a` file appears in its dependencies
