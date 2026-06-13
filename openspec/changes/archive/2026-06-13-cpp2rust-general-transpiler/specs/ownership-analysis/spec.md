## ADDED Requirements

### Requirement: Classify every raw pointer in Phase 1 output
The ownership analysis pass SHALL assign one of five classifications to every `*mut T` pointer in the IR: OWNING, BORROW_IMM, BORROW_MUT, ARRAY, or ALIASED.

#### Scenario: Single-owner pointer classified as OWNING
- **WHEN** a pointer is allocated with `new` at one site, never aliased, and freed at one site
- **THEN** the pointer is classified OWNING → rewritten to `Box<T>` in Phase 2 output

#### Scenario: Never-stored parameter classified as BORROW
- **WHEN** a function parameter pointer is never stored into a struct field or returned
- **THEN** it is classified BORROW_IMM or BORROW_MUT → rewritten to `&T` or `&mut T`

#### Scenario: Pointer with arithmetic classified as ARRAY
- **WHEN** `ptr + offset` or `ptr[i]` access patterns are detected
- **THEN** the pointer is classified ARRAY → eligible for `&[T]` or `Vec<T>` rewrite

#### Scenario: Aliased-mutable pointer stays unsafe
- **WHEN** two code paths both perform mutable writes through the same pointer
- **THEN** the pointer is classified ALIASED and is NOT rewritten — it stays `*mut T` in an unsafe block

### Requirement: Constraint propagation resolves transitive ownership
The analysis SHALL propagate ownership constraints through the call graph: if a pointer is passed to a function that stores it into a OWNING field, the pointer at the call site is also classified OWNING.

#### Scenario: Pointer passed to owning-storing function
- **WHEN** `fn store(self_: *mut Container, item: *mut Item)` stores `item` into `self_.owned_item`
- **THEN** all call sites passing a pointer as `item` are classified OWNING for that pointer

### Requirement: Phase 2 rewrite is conservative — no false positives
The ownership analysis SHALL only rewrite pointers where all constraints agree on a safe classification. Any ambiguity defaults to keeping the pointer as `*mut T` in unsafe.

#### Scenario: Ambiguous pointer stays unsafe
- **WHEN** a pointer is classified BORROW on 3 paths and OWNING on 1 path
- **THEN** the classification is ALIASED and the pointer is not rewritten

### Requirement: Phase 2 output still compiles
After ownership rewriting, the Phase 2 output SHALL pass `cargo build` with zero errors. If a rewrite would cause a borrow-checker error, the rewrite is rolled back for that pointer.

#### Scenario: Failed borrow-checker rewrite rolled back
- **WHEN** rewriting a pointer to `&mut T` would cause a borrow checker conflict
- **THEN** the pointer is reverted to `*mut T` in an unsafe block and a `// PHASE2_REVERT` comment is emitted
