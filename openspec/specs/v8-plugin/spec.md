# v8-plugin Specification

## Purpose

Provide the V8-specific plugin: type mappings, `Handle<T>` generic wrapper, DCHECK/CHECK macro translations, bigint `Digits`/`RWDigits` prelude structs, and the `src/<subdir>` → `v8-<subdir>` module layout matching V8's crate structure.

## Requirements

### Requirement: V8 plugin maps primitive V8 types
The V8 plugin SHALL map the following C++ types to Rust equivalents in `config.yaml`: `digit_t` → `u64`, `int32_t` / `int64_t` / `uint32_t` / `uint64_t` → Rust fixed-width integers, `bool` → `bool`.

#### Scenario: digit_t mapped to u64
- **WHEN** a function parameter `digit_t x` is transpiled with the V8 plugin
- **THEN** the Rust parameter is `x: u64`

### Requirement: V8 plugin emits Handle<T> as a generic pointer wrapper
The V8 plugin SHALL define `Handle<T>` as a Rust generic struct `pub struct Handle<T> { ptr: *mut T }` in the prelude, rather than monomorphizing each instantiation.

#### Scenario: Handle<String> uses the generic struct
- **WHEN** a C++ field `Handle<String> name_` is transpiled
- **THEN** the Rust field is `name_: Handle<String>` referencing the prelude-defined generic

### Requirement: V8 plugin maps DCHECK and CHECK macros
The V8 plugin's pattern entries SHALL translate: `DCHECK(x)` → `debug_assert!(x)`, `DCHECK_EQ(a,b)` → `debug_assert_eq!(a,b)`, `DCHECK_LT/LE/GT/GE` → appropriate `debug_assert!`, `CHECK(x)` → `assert!(x)`, and all CHECK_* variants to `assert_*` equivalents.

#### Scenario: DCHECK_LT translated
- **WHEN** `DCHECK_LT(i, len)` appears in V8 source
- **THEN** the Rust output contains `debug_assert!(i < len)`

#### Scenario: CHECK_EQ translated
- **WHEN** `CHECK_EQ(result, expected)` appears
- **THEN** the Rust output contains `assert_eq!(result, expected)`

### Requirement: V8 plugin provides Digits and RWDigits structs in prelude
The V8 plugin prelude SHALL define `Digits` (read-only bigint digit slice) and `RWDigits` (writable bigint digit slice) as Rust structs with `unsafe impl Send + Sync`, matching the C++ struct layout for the benchmark.

#### Scenario: Digits struct has len() and index operations
- **WHEN** the V8 plugin prelude is injected into `v8-bigint`
- **THEN** `Digits` has a `len()` method returning `i32` and implements `Index<usize>` and `Index<i32>`

### Requirement: V8 module layout maps src/ subdirectories to crates
The V8 plugin `config.yaml` SHALL specify module layout rules mapping `src/<subdir>/` to `v8-<subdir>` crate names, matching the existing 49-crate workspace structure.

#### Scenario: bigint directory maps to v8-bigint crate
- **WHEN** the plugin processes files under `codebase/src/bigint/`
- **THEN** all output files are placed in `output/transpiled/crates/v8-bigint/`
