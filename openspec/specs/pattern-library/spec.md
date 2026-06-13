# pattern-library Specification

## Purpose

Provide a YAML-defined pattern library that matches C++ idioms the structural converter cannot handle and rewrites them to Rust, applied before the raw-text fallback, with built-in coverage of common V8 bigint constructs.

## Requirements

### Requirement: Patterns defined in YAML files under cpp2rust/patterns/
The pattern library SHALL consist of YAML files, each containing a list of pattern entries. Pattern files are loaded at transpiler startup and merged into a single lookup table keyed by pattern name.

#### Scenario: Pattern file loaded successfully
- **WHEN** `cpp2rust/patterns/bit_ops.yaml` contains a valid pattern entry for `bit_cast`
- **THEN** the transpiler's pattern lookup table contains a `bit_cast` entry after startup

#### Scenario: Malformed pattern file raises error at startup
- **WHEN** a YAML pattern file has invalid syntax
- **THEN** the transpiler raises a configuration error before processing any files

### Requirement: Pattern entry fields
Each pattern entry SHALL have: `name` (string, unique), `cpp_pattern` (string with `{T}`, `{expr}`, `{args}` placeholders), `rust_emit` (string with matching placeholders), `needs_unsafe` (bool), and optionally `requires` (list of Rust use-paths to inject).

#### Scenario: bit_cast pattern entry
- **WHEN** the pattern entry is `{name: bit_cast, cpp_pattern: "base::bit_cast<{T}>({expr})", rust_emit: "std::mem::transmute::<_, {T}>({expr})", needs_unsafe: true}`
- **THEN** the entry is valid and loadable

### Requirement: Pattern matching applied before raw fallback
In `stmt_converter`, after all structured CursorKind handlers fail, the pattern library SHALL be consulted. Only if no pattern matches does the extractor produce `IRRawCppExpr`.

#### Scenario: bit_cast expression matched by pattern
- **WHEN** a Clang AST contains a call to `base::bit_cast<uint64_t>(value)` that `stmt_converter` cannot structurally handle
- **THEN** the pattern library matches `bit_cast` and produces an `IRCastExpr` with `transmute` semantics rather than `IRRawCppExpr`

#### Scenario: Unmatched expression falls through to raw
- **WHEN** a Clang cursor kind is not handled structurally AND no pattern matches
- **THEN** an `IRRawCppExpr` is produced with the original C++ source text

### Requirement: Built-in pattern files cover V8 bigint gaps
The pattern library SHALL ship with entries covering: `bit_cast`, `__builtin_clz`/`__builtin_ctz`, `std::make_unique`, `std::make_shared`, `reinterpret_cast` of pointers, `static_cast` of numerics, `alignof`, and `sizeof` of types.

#### Scenario: sizeof type resolved via pattern
- **WHEN** `sizeof(digit_t)` appears in an expression context
- **THEN** the pattern library matches and emits `std::mem::size_of::<digit_t>()`

#### Scenario: make_unique resolved via pattern
- **WHEN** `std::make_unique<BoundedPageAllocator>(args...)` appears
- **THEN** the pattern library emits `Box::new(BoundedPageAllocator::new(args...))`
