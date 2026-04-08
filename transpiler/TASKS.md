# C++ to Rust Transpiler — Task Breakdown

## Overview

Build a Clang AST-based transpiler that converts V8's C++ codebase to idiomatic Rust.
The transpiler uses Python with libclang bindings to parse C++ and emit Rust source code.

**Approach**: Clang AST → JSON IR → Rule-based Rust codegen
**Language**: Python (using libclang for parsing)
**Input**: /home/kathirks_gc/rusty-v8/codebase/ (~5,964 C++ files)
**Output**: Rust workspace with one crate per V8 module

---

## Phase 0: Project Scaffolding

### Task 0.1: Create transpiler directory structure
Create the directory layout for the transpiler project:
```
transpiler/
  __init__.py
  main.py                  # CLI entry point
  config.py                # Configuration (paths, options)
  extractor/               # Phase 1: Clang AST → JSON IR
    __init__.py
    ast_parser.py           # Core Clang AST traversal
    type_collector.py       # Collect all types, classes, enums, typedefs
    function_collector.py   # Collect all function signatures and bodies
    stmt_converter.py       # Convert Clang statement AST to our IR
    include_resolver.py     # Resolve #include dependencies
  ir/                       # Intermediate representation
    __init__.py
    nodes.py                # IR node types (Module, Struct, Function, Enum, etc.)
    type_registry.py        # Global type registry across all translation units
    dependency_graph.py     # Module dependency graph
  mapper/                   # Phase 2: C++ construct → Rust construct mapping
    __init__.py
    type_mapper.py          # C++ type → Rust type mapping
    class_mapper.py         # Class hierarchy → struct/trait mapping
    template_mapper.py      # Template → generic mapping
    stmt_mapper.py          # Statement/expression → Rust mapping
    stdlib_mapper.py        # C++ std:: → Rust std:: mapping
    v8_mapper.py            # V8-specific patterns (Handle, Tagged, Smi)
  codegen/                  # Phase 3: Emit Rust source code
    __init__.py
    rust_emitter.py         # Core Rust source emitter
    module_emitter.py       # mod.rs / lib.rs generation
    cargo_emitter.py        # Cargo.toml generation
    workspace_emitter.py    # Workspace layout
  tests/                    # Unit tests
    __init__.py
    test_extractor.py
    test_mapper.py
    test_codegen.py
    golden/                 # Golden test files (C++ input → expected Rust output)
```

### Task 0.2: Write configuration module
Create `config.py` with:
- Path to V8 codebase root (`../codebase`)
- Path to output directory (`../output/transpiled`)
- Clang include paths needed for V8 headers (the `include/` dir and `src/` dir)
- Clang compilation flags (`-std=c++17`, `-DOFFICIAL_BUILD`, etc.)
- Module-to-crate name mapping (e.g., `base` → `v8_base`, `objects` → `v8_objects`)
- List of V8 source modules (from `codebase/src/` subdirectories)
- Configurable options: target architecture, skip patterns, max file size

### Task 0.3: Write CLI entry point
Create `main.py` with subcommands:
- `extract` — run Clang AST extraction on the codebase
- `analyze` — build type registry and dependency graph
- `transpile` — run the full transpilation pipeline
- `transpile --module <name>` — transpile a single module
- `status` — show progress (how many modules done, errors, etc.)
- `validate` — run cargo check on generated output

---

## Phase 1: Clang AST Extraction

### Task 1.1: Build the core AST parser (`extractor/ast_parser.py`)
Use Python libclang bindings to parse a single C++ translation unit.
For each file, extract a structured representation containing:

**Top-level declarations to extract:**
- Namespace declarations (name, contents)
- Class/struct declarations:
  - Name, qualified name
  - Base classes (with access specifier: public/protected/private)
  - Member fields (name, type, access specifier, is_static, is_const)
  - Member functions (name, return type, parameters, is_virtual, is_pure_virtual,
    is_static, is_const, is_override, access specifier)
  - Constructors (parameters, initializer list)
  - Destructors (is_virtual)
  - Nested types (enums, classes, typedefs)
  - Template parameters (if class template)
- Free functions:
  - Name, qualified name, return type, parameters
  - Template parameters (if function template)
  - Is inline, is static, is constexpr
- Enum declarations:
  - Name, underlying type, is scoped (enum class)
  - Enumerators with values
- Typedef / using declarations:
  - Alias name, underlying type
- Global variables and constants:
  - Name, type, is_const, is_constexpr, initializer value (if simple)
- Template declarations:
  - Template parameters (typename, non-type, template template)
  - The underlying declaration

**Implementation details:**
- Use `clang.cindex.Index` to create a translation unit
- Use `cursor.walk_preorder()` or recursive `cursor.get_children()` traversal
- Filter to only process cursors from the target file (skip system headers)
- Handle `CursorKind.CLASS_DECL`, `CursorKind.STRUCT_DECL`, `CursorKind.FUNCTION_DECL`,
  `CursorKind.CXX_METHOD`, `CursorKind.ENUM_DECL`, `CursorKind.TYPEDEF_DECL`,
  `CursorKind.TYPE_ALIAS_DECL`, `CursorKind.NAMESPACE`, `CursorKind.VAR_DECL`,
  `CursorKind.FIELD_DECL`, `CursorKind.CONSTRUCTOR`, `CursorKind.DESTRUCTOR`,
  `CursorKind.CLASS_TEMPLATE`, `CursorKind.FUNCTION_TEMPLATE`, etc.
- For each cursor, record: spelling, qualified name, location (file:line:col),
  type spelling, canonical type, access specifier
- Must handle forward declarations vs definitions (only extract from definitions)
- Must handle anonymous namespaces, anonymous structs/enums
- Produce output as Python dataclasses that can serialize to JSON

### Task 1.2: Build the statement/expression converter (`extractor/stmt_converter.py`)
Convert Clang statement/expression AST nodes into our intermediate representation.
This is needed for translating function bodies.

**Statement types to handle:**
- CompoundStmt (block `{ ... }`)
- DeclStmt (variable declarations)
- ReturnStmt
- IfStmt, SwitchStmt, CaseStmt, DefaultStmt
- ForStmt, WhileStmt, DoStmt, CXXForRangeStmt
- BreakStmt, ContinueStmt
- ExprStmt (expression as statement)

**Expression types to handle:**
- IntegerLiteral, FloatingLiteral, StringLiteral, CharacterLiteral, CXXBoolLiteralExpr
- DeclRefExpr (variable/function reference)
- CallExpr, CXXMemberCallExpr, CXXOperatorCallExpr
- MemberExpr (field access: `.` and `->`)
- BinaryOperator, UnaryOperator, ConditionalOperator
- CXXStaticCastExpr, CXXDynamicCastExpr, CXXReinterpretCastExpr, CXXConstCastExpr,
  ImplicitCastExpr, CStyleCastExpr
- CXXNewExpr, CXXDeleteExpr
- ArraySubscriptExpr
- CXXConstructExpr, CXXTemporaryObjectExpr
- LambdaExpr
- CXXThisExpr
- CXXNullPtrLiteralExpr
- SizeOfPackExpr, UnaryExprOrTypeTraitExpr (sizeof, alignof)
- InitListExpr (aggregate initialization)
- CXXThrowExpr, CXXTryStmt, CXXCatchStmt

**Implementation approach:**
- Recursive descent through `cursor.get_children()`
- Map each `CursorKind` to an IR node (defined in `ir/nodes.py`)
- Capture the source text as fallback for unhandled nodes (using `cursor.extent`)
- Handle operator overloading by mapping to the underlying function call
- Handle implicit conversions by unwrapping ImplicitCastExpr

### Task 1.3: Build the type collector (`extractor/type_collector.py`)
Traverse the entire codebase and build a global type inventory.

**Responsibilities:**
- Parse every `.h` file in `codebase/include/` and `codebase/src/`
- For each type found, record:
  - Fully qualified name
  - Kind (class, struct, enum, typedef, template)
  - Source file and line
  - Module it belongs to (derived from directory path)
  - Base classes (for class types)
  - Template parameters (for template types)
  - Whether it's a forward declaration or full definition
- Detect and record:
  - The complete class hierarchy (inheritance tree)
  - Abstract classes (those with pure virtual methods)
  - Template instantiations and specializations
  - Type aliases and their resolved types
- Output a `TypeRegistry` object that maps qualified name → type info
- Handle V8-specific type patterns:
  - `Tagged<T>`, `Handle<T>`, `MaybeHandle<T>`, `DirectHandle<T>`
  - Smi (small integer) as a tagged value
  - HeapObject hierarchy: Object → HeapObject → JSReceiver → JSObject → ...

### Task 1.4: Build the include resolver (`extractor/include_resolver.py`)
Resolve `#include` directives to determine inter-file and inter-module dependencies.

**Responsibilities:**
- For each source file, determine which other files it includes
- Map file-level dependencies to module-level dependencies
  (e.g., if `src/compiler/foo.cc` includes `src/objects/bar.h`,
  then module `compiler` depends on module `objects`)
- Handle include paths: V8 uses includes relative to `src/` and `include/`
  (e.g., `#include "src/objects/tagged.h"` or `#include "include/v8.h"`)
- Produce a module dependency graph (DAG with cycles detected)
- Determine topological ordering for conversion (dependencies first)

---

## Phase 2: Intermediate Representation

### Task 2.1: Define IR node types (`ir/nodes.py`)
Create Python dataclasses representing the transpiler's intermediate representation.
These are the bridge between C++ AST and Rust code generation.

**Module-level IR nodes:**
```python
@dataclass
class IRModule:
    name: str                    # e.g., "base"
    crate_name: str              # e.g., "v8_base"
    files: List[IRFile]
    dependencies: List[str]      # crate names this depends on

@dataclass
class IRFile:
    path: str                    # Relative path within the module
    rust_path: str               # Target Rust file path
    items: List[IRItem]          # Top-level items in this file
```

**Type-level IR nodes:**
```python
@dataclass
class IRStruct:
    name: str
    qualified_name: str
    fields: List[IRField]
    methods: List[IRFunction]
    base_classes: List[IRBaseClass]  # For inheritance → composition mapping
    template_params: List[IRTemplateParam]
    is_abstract: bool
    visibility: str              # "pub", "pub(crate)", ""
    derives: List[str]           # #[derive(...)]

@dataclass
class IREnum:
    name: str
    variants: List[IREnumVariant]
    underlying_type: Optional[IRType]
    is_scoped: bool              # C++ enum class → Rust enum

@dataclass
class IRTrait:                   # Generated from abstract C++ classes
    name: str
    methods: List[IRFunction]    # Pure virtual → required, virtual → default impl
    supertraits: List[str]

@dataclass
class IRTypeAlias:
    name: str
    target: IRType

@dataclass
class IRConst:
    name: str
    type: IRType
    value: Optional[str]         # Literal value if known
```

**Function-level IR nodes:**
```python
@dataclass
class IRFunction:
    name: str
    params: List[IRParam]
    return_type: IRType
    body: Optional[IRBlock]      # None for declarations without bodies
    is_method: bool
    self_param: Optional[str]    # "&self", "&mut self", "self"
    is_static: bool
    is_unsafe: bool
    template_params: List[IRTemplateParam]
    visibility: str

@dataclass
class IRParam:
    name: str
    type: IRType
    default_value: Optional[str]
```

**Type IR nodes:**
```python
@dataclass
class IRType:
    name: str                    # Rust type name (after mapping)
    cpp_name: str                # Original C++ type name
    is_reference: bool
    is_mutable: bool
    is_pointer: bool             # Raw pointer (unsafe)
    is_optional: bool            # Nullable pointer → Option<>
    generic_args: List[IRType]   # Template args → generic args
    lifetime: Optional[str]      # 'a, 'static, etc.
```

**Statement/Expression IR nodes:**
```python
@dataclass
class IRBlock:
    statements: List[IRStmt]

@dataclass
class IRVarDecl:
    name: str
    type: IRType
    initializer: Optional[IRExpr]
    is_mutable: bool

@dataclass
class IRReturn:
    value: Optional[IRExpr]

@dataclass
class IRIf:
    condition: IRExpr
    then_block: IRBlock
    else_block: Optional[IRBlock]

@dataclass
class IRForLoop:             # C++ for → Rust loop/for/while
    init: Optional[IRStmt]
    condition: Optional[IRExpr]
    increment: Optional[IRExpr]
    body: IRBlock

@dataclass
class IRForRange:            # C++ range-for → Rust for-in
    variable: str
    iterable: IRExpr
    body: IRBlock

@dataclass
class IRWhile:
    condition: IRExpr
    body: IRBlock

@dataclass
class IRMatch:               # C++ switch → Rust match
    expr: IRExpr
    arms: List[IRMatchArm]

@dataclass
class IRCall:
    function: IRExpr         # What's being called
    args: List[IRExpr]
    is_method_call: bool

@dataclass
class IRFieldAccess:
    object: IRExpr
    field: str

@dataclass
class IRBinaryOp:
    op: str                  # "+", "-", "==", "&&", etc.
    left: IRExpr
    right: IRExpr

@dataclass
class IRUnaryOp:
    op: str                  # "!", "-", "*" (deref), "&" (ref)
    operand: IRExpr

@dataclass
class IRCast:
    expr: IRExpr
    target_type: IRType
    kind: str                # "as", "into", "from", "unsafe_transmute"

@dataclass
class IRIndex:
    object: IRExpr
    index: IRExpr

@dataclass
class IRLiteral:
    value: str
    kind: str                # "int", "float", "string", "bool", "char", "null"

@dataclass
class IRRawCode:             # Fallback: raw C++ that couldn't be parsed
    cpp_source: str
    comment: str             # "TODO: manually translate"
```

### Task 2.2: Build the type registry (`ir/type_registry.py`)
Central registry that stores all types across the entire codebase.

**Responsibilities:**
- Store every type found during extraction (Task 1.3)
- Resolve type aliases to their canonical types
- Build the full inheritance hierarchy as a graph
- Determine which classes need trait extraction (abstract classes with pure virtuals)
- Determine which classes use single vs multiple inheritance
- Map C++ namespaces to Rust module paths
- Detect V8-specific type patterns and annotate them
- Provide lookup methods:
  - `resolve(cpp_qualified_name) → IRType` (fully resolved Rust type)
  - `get_bases(class_name) → List[BaseClass]`
  - `get_virtual_methods(class_name) → List[Method]`
  - `is_abstract(class_name) → bool`
  - `get_module(class_name) → str`

### Task 2.3: Build the dependency graph (`ir/dependency_graph.py`)
Module-level dependency graph for determining conversion order.

**Responsibilities:**
- Build directed graph: module A → module B means A depends on B
- Detect cycles and report them
- Compute topological ordering (dependencies first)
- For cycles, determine which modules should be merged into one crate
- Map each V8 src/ subdirectory to a crate name
- Handle the `include/` directory as a separate `v8_api` crate

---

## Phase 3: C++ to Rust Mapping Rules

### Task 3.1: Build the type mapper (`mapper/type_mapper.py`)
Deterministic mapping of C++ types to Rust types.

**Mappings to implement:**

Primitive types:
- `int` → `i32`, `unsigned int` → `u32`
- `int8_t` → `i8`, `uint8_t` → `u8`, etc.
- `int64_t` → `i64`, `uint64_t` → `u64`
- `size_t` → `usize`, `ssize_t` → `isize`
- `ptrdiff_t` → `isize`
- `intptr_t` → `isize`, `uintptr_t` → `usize`
- `bool` → `bool`
- `char` → `c_char` (or `u8` depending on usage)
- `wchar_t` → `u32`
- `float` → `f32`, `double` → `f64`
- `void` → `()` (return type) or `c_void` (pointer target)

C++ standard library:
- `std::string` → `String`
- `std::string_view` → `&str`
- `std::vector<T>` → `Vec<T>`
- `std::array<T, N>` → `[T; N]`
- `std::unique_ptr<T>` → `Box<T>`
- `std::shared_ptr<T>` → `Arc<T>`
- `std::weak_ptr<T>` → `Weak<T>`
- `std::optional<T>` → `Option<T>`
- `std::pair<A, B>` → `(A, B)`
- `std::tuple<...>` → `(...)`
- `std::unordered_map<K, V>` → `HashMap<K, V>`
- `std::map<K, V>` → `BTreeMap<K, V>`
- `std::unordered_set<T>` → `HashSet<T>`
- `std::set<T>` → `BTreeSet<T>`
- `std::function<R(Args...)>` → `Box<dyn Fn(Args...) -> R>`
- `std::mutex` → `Mutex<()>`
- `std::atomic<T>` → `AtomicT` (e.g., `AtomicI32`, `AtomicBool`)
- `std::thread` → `std::thread::JoinHandle<()>`

Pointer/reference types:
- `const T&` → `&T`
- `T&` → `&mut T`
- `const T*` (non-owning) → `*const T` or `&T`
- `T*` (non-owning) → `*mut T` or `&mut T`
- `T*` (owning, from new) → `Box<T>`
- `nullptr` → `std::ptr::null()` or `None` (for Option)

### Task 3.2: Build the class/struct mapper (`mapper/class_mapper.py`)
Map C++ class hierarchies to Rust structs + traits.

**Rules:**
1. Single inheritance (class Derived : public Base):
   - `struct Derived { base: Base, <own fields> }`
   - Optionally `impl Deref<Target=Base> for Derived`
   - Or flatten fields from base into derived (simpler but loses structure)

2. Abstract class (has pure virtual methods):
   - Extract a `trait` with the pure virtual methods
   - Concrete subclasses `impl TheTrait for ConcreteStruct`

3. Multiple inheritance (class C : public A, public B):
   - Primary base embedded as field
   - Secondary bases become traits
   - `impl ATrait for C` + `impl BTrait for C`

4. Virtual methods (non-pure):
   - Include in trait with default implementation
   - Override = custom impl

5. Access specifiers:
   - `public` → `pub`
   - `protected` → `pub(crate)` (approximate)
   - `private` → no visibility modifier

6. Static members:
   - Static fields → module-level `static` or associated constant
   - Static methods → associated functions (no `self`)

7. Constructors:
   - `new()` or `from()` associated functions
   - Multiple constructors → `new()`, `with_xxx()`, or `From<T>` impls

8. Destructors:
   - Non-trivial destructor → `impl Drop for T`

9. Operator overloading:
   - `operator+` → `impl Add`, `operator==` → `impl PartialEq`, etc.
   - `operator[]` → `impl Index` / `impl IndexMut`
   - `operator()` → `impl Fn/FnMut/FnOnce`

10. Friend declarations:
    - `pub(crate)` visibility for the relevant methods

### Task 3.3: Build the statement mapper (`mapper/stmt_mapper.py`)
Map C++ statements/expressions to Rust equivalents.

**Statement mappings:**
- `if (cond) { ... } else { ... }` → `if cond { ... } else { ... }` (no parens required)
- `switch (x) { case A: ...; break; }` → `match x { A => { ... } }`
  - Fall-through cases: combine patterns `A | B => { ... }`
  - Default → `_ => { ... }`
- `for (int i = 0; i < n; i++)` → `for i in 0..n`
- `for (auto& x : container)` → `for x in &container`
- `while (cond)` → `while cond`
- `do { ... } while (cond)` → `loop { ... if !cond { break; } }`
- `T* p = new T(args)` → `let p = Box::new(T::new(args))`
- `delete p` → drop(p) or just let it go out of scope
- `static_cast<T>(x)` → `x as T` or `T::from(x)`
- `dynamic_cast<T*>(x)` → pattern match or downcast
- `reinterpret_cast<T*>(x)` → `unsafe { std::mem::transmute(x) }` or `x as *const T`
- `const_cast<T*>(x)` → `x as *mut T` (unsafe)
- `throw ExceptionType(msg)` → `return Err(ExceptionType::new(msg))`
- `try { ... } catch (Type& e) { ... }` → Result-based error handling
- `x->member` → `x.member` (Rust auto-derefs)
- `x.member` → `x.member`
- `&x` → `&x` or `&mut x`
- `*ptr` → `unsafe { *ptr }` (for raw pointers) or `*ptr` (for references)
- `sizeof(T)` → `std::mem::size_of::<T>()`
- `alignof(T)` → `std::mem::align_of::<T>()`
- `std::move(x)` → just `x` (Rust moves by default)
- `nullptr` → `None` (for Option) or `std::ptr::null()`
- `this->member` → `self.member`
- `static_assert(cond, msg)` → `const _: () = assert!(cond, msg);` (or just comment)

**Expression mappings:**
- `a && b` → `a && b`
- `a || b` → `a || b`
- `!a` → `!a`
- `a == b` → `a == b`
- `a ? b : c` → `if a { b } else { c }`
- `a++` → `{ let tmp = a; a += 1; tmp }` or just `a += 1` if result unused
- `++a` → `{ a += 1; a }`
- `a << b` → `a << b` (but also could be stream insertion — detect context)
- Lambda `[captures](params) { body }` → closure `|params| { body }`
  - `[=]` capture → move closure
  - `[&]` capture → borrowing closure
  - `[x]` capture → `move |...| { /* x moved */ }`

### Task 3.4: Build the V8-specific mapper (`mapper/v8_mapper.py`)
Handle V8-specific patterns that need special treatment.

**V8 patterns:**
- `Handle<T>` → custom `Handle<T>` Rust type (wraps a GC-managed pointer)
- `MaybeHandle<T>` → `Option<Handle<T>>`
- `DirectHandle<T>` → custom `DirectHandle<T>`
- `Tagged<T>` → custom `Tagged<T>` newtype
- `Smi` → `struct Smi(isize)` with arithmetic ops
- `HeapObject` hierarchy → struct hierarchy with `Tagged` wrapper
- `DCHECK(cond)` → `debug_assert!(cond)`
- `CHECK(cond)` → `assert!(cond)`
- `UNREACHABLE()` → `unreachable!()`
- `USE(x)` → `let _ = x;`
- `V8_WARN_UNUSED_RESULT` → `#[must_use]`
- `V8_INLINE` → `#[inline]`
- `V8_NOINLINE` → `#[inline(never)]`
- `V8_UNLIKELY(cond)` / `V8_LIKELY(cond)` → just `cond` (or use `likely`/`unlikely` intrinsics)
- `DECL_ACCESSORS(name, Type)` → getter/setter methods (extract from macro expansion)
- `base::Mutex` → `std::sync::Mutex`
- `base::MutexGuard` → `std::sync::MutexGuard`
- `base::Optional<T>` → `Option<T>`
- `base::Vector<T>` → `&[T]`
- `MemoryChunk`, `Page` → custom Rust types with unsafe internals
- Architecture-specific code (`#if V8_TARGET_ARCH_X64`) → `#[cfg(target_arch = "x86_64")]`
- Feature flags (`#if V8_ENABLE_WEBASSEMBLY`) → `#[cfg(feature = "webassembly")]`

### Task 3.5: Build the stdlib mapper (`mapper/stdlib_mapper.py`)
Map C++ standard library function calls to Rust equivalents.

**Examples:**
- `std::move(x)` → `x` (Rust moves by default)
- `std::forward<T>(x)` → `x` (no forwarding needed in Rust)
- `std::make_unique<T>(args)` → `Box::new(T::new(args))`
- `std::make_shared<T>(args)` → `Arc::new(T::new(args))`
- `vec.push_back(x)` → `vec.push(x)`
- `vec.emplace_back(args)` → `vec.push(T::new(args))`
- `vec.size()` → `vec.len()`
- `vec.empty()` → `vec.is_empty()`
- `vec.begin()`, `vec.end()` → `vec.iter()`
- `vec.erase(it)` → `vec.remove(idx)`
- `str.c_str()` → `str.as_ptr()`
- `str.length()` → `str.len()`
- `str.substr(pos, len)` → `&str[pos..pos+len]`
- `str.find(sub)` → `str.find(sub)` (returns Option<usize>)
- `std::cout << x` → `print!("{}", x)` or `println!("{}", x)`
- `std::cerr << x` → `eprintln!("{}", x)`
- `std::sort(begin, end)` → `slice.sort()`
- `std::min(a, b)` → `a.min(b)` or `std::cmp::min(a, b)`
- `std::max(a, b)` → `a.max(b)` or `std::cmp::max(a, b)`
- `std::swap(a, b)` → `std::mem::swap(&mut a, &mut b)`
- `assert(cond)` → `assert!(cond)`
- `memcpy(dst, src, n)` → `unsafe { std::ptr::copy_nonoverlapping(src, dst, n) }`
- `memset(ptr, val, n)` → `unsafe { std::ptr::write_bytes(ptr, val, n) }`
- `malloc(n)` → `unsafe { std::alloc::alloc(layout) }`
- `free(ptr)` → `unsafe { std::alloc::dealloc(ptr, layout) }`

### Task 3.6: Build the template mapper (`mapper/template_mapper.py`)
Map C++ templates to Rust generics.

**Rules:**
- `template<typename T> class Foo` → `struct Foo<T>`
- `template<typename T> T func(T x)` → `fn func<T>(x: T) -> T`
- Template specialization → trait specialization or conditional compilation
- CRTP `class D : Base<D>` → trait with `Self` type
- Non-type template params `template<int N>` → const generics `struct Foo<const N: usize>`
- `std::enable_if` / SFINAE → trait bounds
- Template default arguments → Rust default type parameters (limited support)
- Variadic templates → macro_rules! or tuples
- Nested templates `Foo<Bar<T>>` → `Foo<Bar<T>>`

---

## Phase 4: Rust Code Generation

### Task 4.1: Build the Rust source emitter (`codegen/rust_emitter.py`)
Core code emitter that converts IR nodes to Rust source text.

**Responsibilities:**
- Emit struct definitions with proper derives (`#[derive(Debug, Clone, ...)]`)
- Emit enum definitions
- Emit trait definitions
- Emit impl blocks (inherent and trait impls)
- Emit function bodies from IR statement/expression trees
- Handle indentation and formatting consistently
- Emit `use` statements for imports
- Emit type aliases
- Emit const/static declarations
- Emit `unsafe` blocks where needed (raw pointer ops, transmutes, FFI)
- Emit `#[cfg(...)]` attributes for conditional compilation
- Emit doc comments from C++ comments (if available)
- Handle visibility modifiers (`pub`, `pub(crate)`, private)
- Format output with proper Rust conventions (snake_case functions, CamelCase types)

**Name conversion:**
- C++ `CamelCase` class names → keep as-is (Rust convention for types)
- C++ `camelCase` method names → `snake_case`
- C++ `kConstantName` → `CONSTANT_NAME`
- C++ `MACRO_NAME` → `MACRO_NAME` (if const) or `macro_name!` (if macro)
- C++ `m_member` or `member_` → `member`

### Task 4.2: Build the module emitter (`codegen/module_emitter.py`)
Generate Rust module structure (mod.rs, lib.rs).

**Responsibilities:**
- For each V8 module (e.g., `base/`), create a crate directory with:
  - `src/lib.rs` — root module with `pub mod` declarations
  - `src/<file>.rs` — one Rust file per C++ header/source pair
  - Sub-modules for nested directories
- Determine which items are `pub` vs `pub(crate)` vs private
- Generate proper `mod` declarations
- Handle re-exports (`pub use`) where needed
- Merge `.h` and `.cc` file pairs into single `.rs` files

### Task 4.3: Build the Cargo.toml emitter (`codegen/cargo_emitter.py`)
Generate Cargo.toml for each crate and the workspace.

**Responsibilities:**
- Workspace `Cargo.toml` at output root listing all member crates
- Per-crate `Cargo.toml` with:
  - Package name (e.g., `v8-base`)
  - Edition = "2021"
  - Dependencies on other v8 crates (from dependency graph)
  - External dependencies (detected from usage):
    - `bitflags` for flag enums
    - `libc` for C type interop
    - `parking_lot` for mutexes (if needed)
    - `cfg-if` for conditional compilation helpers
  - Features for optional components (e.g., `webassembly`, `inspector`)
  - `#[cfg]` target configurations

---

## Phase 5: Integration and Testing

### Task 5.1: Implement end-to-end transpilation for `base` module
The `base` module is the simplest and most self-contained V8 module.
Use it as the first test case for the full pipeline.

**Steps:**
1. Run the extractor on all files in `codebase/src/base/`
2. Build the type registry for base module types
3. Apply mapping rules to generate IR
4. Emit Rust code into `output/transpiled/v8-base/`
5. Generate Cargo.toml
6. Run `cargo check` and catalog errors
7. Iterate on mapper rules to fix common errors

**Expected files in base:**
- `bits.h` / `bits.cc` → `bits.rs` (bit manipulation utilities)
- `logging.h` / `logging.cc` → `logging.rs`
- `macros.h` → `macros.rs`
- `numbers/` → sub-module
- `platform/` → platform-specific code with cfg attributes
- `strings.h` → `strings.rs`
- `utils/` → utility sub-module
- `vector.h` → `vector.rs`

### Task 5.2: Implement end-to-end transpilation for `objects` module
The `objects` module is the largest (~337 files) and most critical —
it defines V8's entire object hierarchy.

### Task 5.3: Validate with cargo check and iterate
Run `cargo check --workspace` on the generated output.
Parse errors, categorize them, and fix the most common patterns.

### Task 5.4: Write unit tests for mappers
Golden tests: C++ input snippet → expected Rust output.
Test each mapper independently.

---

## Execution Order

The tasks should be executed in this order (dependencies noted):

1. **Task 0.1** — scaffolding (no deps)
2. **Task 0.2** — config (depends on 0.1)
3. **Task 0.3** — CLI (depends on 0.1, 0.2)
4. **Task 2.1** — IR nodes (no deps, but needed by everything)
5. **Task 1.1** — AST parser (depends on 2.1 for IR types)
6. **Task 1.2** — statement converter (depends on 1.1, 2.1)
7. **Task 1.3** — type collector (depends on 1.1)
8. **Task 1.4** — include resolver (depends on 1.1)
9. **Task 2.2** — type registry (depends on 1.3, 2.1)
10. **Task 2.3** — dependency graph (depends on 1.4, 2.1)
11. **Task 3.1** — type mapper (depends on 2.1)
12. **Task 3.2** — class mapper (depends on 2.2, 3.1)
13. **Task 3.3** — statement mapper (depends on 2.1, 3.1)
14. **Task 3.4** — V8 mapper (depends on 3.1, 3.2)
15. **Task 3.5** — stdlib mapper (depends on 3.1)
16. **Task 3.6** — template mapper (depends on 3.1)
17. **Task 4.1** — Rust emitter (depends on 2.1, 3.*)
18. **Task 4.2** — module emitter (depends on 4.1, 2.3)
19. **Task 4.3** — Cargo emitter (depends on 2.3)
20. **Task 5.1** — end-to-end base module (depends on all above)
21. **Task 5.2** — end-to-end objects module (depends on 5.1)
22. **Task 5.3** — validation loop (depends on 5.1)
23. **Task 5.4** — unit tests (can be done in parallel with 5.*)
