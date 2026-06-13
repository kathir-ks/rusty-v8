## ADDED Requirements

### Requirement: Plugin interface defined as Python base class
The plugin system SHALL define a `CppPlugin` base class in `cpp2rust/core/plugin.py` with overridable methods: `map_type`, `map_macro`, `prelude_lines`, `crate_for_file`, and `extra_patterns`.

#### Scenario: Default plugin maps no types
- **WHEN** `CppPlugin().map_type("SomeUnknownType")` is called
- **THEN** `None` is returned (no mapping)

#### Scenario: Plugin loaded by name from plugins/ directory
- **WHEN** `--plugin v8` is passed on the CLI
- **THEN** `cpp2rust/plugins/v8/plugin.py` is imported and its `V8Plugin` class is instantiated

### Requirement: YAML config loaded alongside plugin class
Each plugin directory SHALL contain a `config.yaml` file specifying: `type_mappings` (dict), `macro_mappings` (dict), `module_layout` (list of path→crate rules), `prelude_file` (path), and `skip_patterns` (list of globs).

#### Scenario: Type mapping from config applied
- **WHEN** `config.yaml` contains `type_mappings: {digit_t: u64}`
- **THEN** every C++ type `digit_t` is emitted as `u64` in Rust

#### Scenario: Skip pattern excludes test files
- **WHEN** `config.yaml` contains `skip_patterns: ["**/*_unittest*", "**/test/**"]`
- **THEN** files matching those patterns are excluded from transpilation

### Requirement: Plugin extra_patterns extend the pattern library
A plugin's `extra_patterns()` method SHALL return additional YAML pattern entries that are merged into the global pattern table with higher priority than built-in patterns.

#### Scenario: V8 plugin overrides DCHECK macro
- **WHEN** the V8 plugin returns a pattern entry for `DCHECK(x)` → `debug_assert!(x)`
- **THEN** all `DCHECK(...)` macro calls are translated to `debug_assert!` rather than falling through to `IRRawCppExpr`

### Requirement: Prelude injected into each crate's lib.rs
The plugin's prelude (either from `prelude_file` or `prelude_lines()`) SHALL be prepended to each crate's `lib.rs` or `prelude.rs` and re-exported with `pub use prelude::*`.

#### Scenario: V8 plugin prelude contains digit_t
- **WHEN** the V8 plugin's prelude defines `pub type digit_t = u64;`
- **THEN** all generated crates can reference `digit_t` without a full qualified path

### Requirement: No-plugin mode uses sensible defaults
When `--plugin` is not specified, the transpiler SHALL use a `DefaultPlugin` that applies standard C++ stdlib mappings (`std::string` → `String`, `std::vector<T>` → `Vec<T>`, etc.) and no codebase-specific prelude.

#### Scenario: Default plugin maps std::string
- **WHEN** no plugin is specified and a C++ parameter `std::string name` is transpiled
- **THEN** the Rust parameter is `name: String`
