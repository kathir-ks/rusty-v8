## ADDED Requirements

### Requirement: Enumerate all template instantiations via Clang records
The template resolver SHALL scan the extracted AST for all template instantiation records (`TEMPLATE_REF`, `CXX_CONSTRUCT_EXPR` with template args) and build a per-template set of concrete argument tuples.

#### Scenario: Collect Handle<T> instantiations
- **WHEN** V8 source uses `Handle<String>`, `Handle<Map>`, and `Handle<Object>`
- **THEN** the resolver records three instantiations: `{T: String}`, `{T: Map}`, `{T: Object}` for the `Handle` template

### Requirement: Tiered template translation — plugin-generic, structural, monomorphize
Templates SHALL be translated in order: (1) plugin-annotated generics first, (2) structural detection (single-field wrappers, pair-like), (3) monomorphize remaining with concrete names.

#### Scenario: Plugin-annotated generic preserved
- **WHEN** the V8 plugin declares `Handle<T>` as a Rust generic
- **THEN** a single `struct Handle<T>` is emitted, not separate `Handle_String`, `Handle_Map` structs

#### Scenario: Structural one-field wrapper detected
- **WHEN** a template `template<T> struct Wrapper { T value; }` exists
- **THEN** the resolver emits `struct Wrapper<T> { value: T }` without monomorphization

#### Scenario: Unknown template monomorphized
- **WHEN** a template that is neither plugin-annotated nor structurally simple is used with types `Foo` and `Bar`
- **THEN** the resolver emits `struct Tmpl_Foo { ... }` and `struct Tmpl_Bar { ... }` as separate concrete types

### Requirement: Monomorphized names are deterministic and collision-free
Monomorphized type names SHALL be formed as `<TemplateName>_<ArgType1>_<ArgType2>` with namespace separators replaced by `_`. Names SHALL be unique within the emitted workspace.

#### Scenario: Name collision resolved with suffix
- **WHEN** two different template instantiations would produce the same mangled name
- **THEN** a numeric suffix is appended to disambiguate (e.g., `Tmpl_Foo_2`)

### Requirement: Template function bodies instantiated once per concrete type set
For each unique set of concrete template arguments, the template function body SHALL be emitted once with all generic references replaced by the concrete types.

#### Scenario: Function template instantiated for two types
- **WHEN** `template<T> T clamp(T v, T lo, T hi)` is used with `int` and `double`
- **THEN** two Rust functions `clamp_i32` and `clamp_f64` are emitted
