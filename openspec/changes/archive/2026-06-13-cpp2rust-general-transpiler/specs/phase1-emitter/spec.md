## ADDED Requirements

### Requirement: Every input file produces compilable Rust output
The Phase 1 emitter SHALL produce Rust source for every input C++ file such that `cargo build` on the resulting workspace succeeds with zero errors. `todo!()` is PROHIBITED as a fallback.

#### Scenario: Unhandled expression emits Default::default()
- **WHEN** an `IRRawCppExpr` node is encountered during emission
- **THEN** the emitter produces `{ let _r: <inferred_type> = unsafe { Default::default() }; /* UNTRANSLATED: <cpp_source> */ _r }` which is syntactically valid Rust

#### Scenario: Unhandled statement emits compilable stub
- **WHEN** an `IRRawCppStmt` node is encountered
- **THEN** the emitter produces `unsafe { /* UNTRANSLATED: <cpp_source> */ let _ = 0; }` which is syntactically valid Rust

#### Scenario: Complete bigint module builds
- **WHEN** `cpp2rust` processes `codebase/src/bigint/` with the V8 plugin
- **THEN** `cargo build -p v8-bigint` exits with code 0

### Requirement: Raw pointers used for all pointer types in Phase 1
The Phase 1 emitter SHALL emit `*mut T` for all C++ mutable pointer and reference types, and `*const T` for const pointer types. Ownership inference is deferred to Phase 2.

#### Scenario: C++ T* becomes *mut T
- **WHEN** a function parameter `Digit* result` is emitted
- **THEN** the Rust parameter is `result: *mut Digit`

#### Scenario: C++ const T& becomes *const T
- **WHEN** a function parameter `const Digits& x` is emitted
- **THEN** the Rust parameter is `x: *const Digits` wrapped in unsafe access

### Requirement: Inheritance emitted as struct containment
The Phase 1 emitter SHALL emit C++ single inheritance as Rust struct containment: `struct Derived { base: Base, <own fields> }`.

#### Scenario: Derived struct contains base as first field
- **WHEN** `class ProcessorImpl : public Processor` is emitted
- **THEN** the Rust struct `ProcessorImpl` has `base: Processor` as its first field

### Requirement: Virtual methods emitted as explicit vtable function pointers
The Phase 1 emitter SHALL emit C++ virtual method tables as explicit Rust structs containing function pointers, and virtual calls as unsafe function-pointer invocations through the vtable.

#### Scenario: Abstract class produces vtable struct
- **WHEN** a C++ abstract class `class Platform { virtual void* Allocate(size_t) = 0; }` is emitted
- **THEN** a `PlatformVTable` struct with `allocate: unsafe fn(*mut Platform, usize) -> *mut ()` field is emitted alongside `struct Platform { vtable: *const PlatformVTable }`

### Requirement: Operator overloads emitted as free functions
The Phase 1 emitter SHALL emit C++ operator overloads as free Rust functions with the operator spelled out (e.g., `fn digits_add(lhs: Digits, rhs: Digits)`). Phase 2 may convert these to trait impls.

#### Scenario: operator+ becomes a free function
- **WHEN** `Digits operator+(Digits a, Digits b)` is emitted
- **THEN** the Rust output contains `pub fn digits_operator_add(a: Digits, b: Digits) -> Digits`

### Requirement: `unsafe` blocks wrap all pointer operations
All pointer dereferences, raw casts, and unverified memory accesses SHALL be enclosed in `unsafe { }` blocks in Phase 1 output.

#### Scenario: Pointer dereference is unsafe
- **WHEN** a C++ expression `*ptr = value` is emitted
- **THEN** the Rust output wraps the assignment in `unsafe { *ptr = value; }`
