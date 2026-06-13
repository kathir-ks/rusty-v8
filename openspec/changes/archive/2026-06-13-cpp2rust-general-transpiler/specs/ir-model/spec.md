## ADDED Requirements

### Requirement: IR covers all C++ expression kinds
The IR SHALL include node types for every C++ expression kind required for V8 and general C++ codebases: literals, name references, binary/unary operators, function calls, member access, array subscript, casts (static/reinterpret/const/functional), ternary, new/delete, sizeof/alignof, lambdas, `this`, aggregate initialization, and a raw-text fallback node.

#### Scenario: Binary expression roundtrip
- **WHEN** `IRBinaryExpr(op=ADD, left=IRNameRef("a"), right=IRLiteral(1))` is serialized and deserialized
- **THEN** the reconstructed node is equal to the original

#### Scenario: Raw fallback node carries cpp_source
- **WHEN** an unhandled AST node is encountered
- **THEN** an `IRRawCppExpr(cpp_source="...", default_type="i32")` is produced, carrying the original C++ source text

### Requirement: IR covers all C++ statement kinds
The IR SHALL include node types for: variable declarations, expression statements, return, if/else, switch→match, C-style for, range-based for, while, do-while, break, continue, throw, try/catch, unsafe blocks, and a raw-text fallback statement.

#### Scenario: For loop IR node
- **WHEN** `for (int i = 0; i < n; i++)` is converted
- **THEN** an `IRForLoop` is produced with `init=IRVarDecl`, `condition=IRBinaryExpr(LT)`, `increment=IRUnaryExpr(POST_INC)`

#### Scenario: Raw fallback statement compiles
- **WHEN** `IRRawCppStmt(cpp_source="complex_macro();")` is emitted by Phase 1
- **THEN** the emitter produces `{ let _ = Default::default(); /* UNTRANSLATED: complex_macro(); */ };` which is valid Rust syntax

### Requirement: IR is JSON-serializable and deserializable
All IR node types SHALL be serializable to JSON via `ir_to_dict()` and reconstructable via `ir_from_dict()` without loss of information.

#### Scenario: Full function IR roundtrip
- **WHEN** an `IRFunction` with a non-trivial body is serialized to JSON and deserialized
- **THEN** the result is structurally identical to the original

### Requirement: IRType captures all C++ type qualifiers
`IRType` SHALL carry: constness, reference/mutable-reference, pointer/mutable-pointer, optional (nullable pointer), generic arguments, array size, and lifetime annotation fields.

#### Scenario: Const reference type
- **WHEN** a C++ parameter `const Foo&` is extracted
- **THEN** `IRType` has `is_const=True`, `is_reference=True`, `is_mut_reference=False`
