## ADDED Requirements

### Requirement: Parse any C++ codebase via compile_commands.json
The extractor SHALL accept a `compile_commands.json` file as its primary input and parse each translation unit with the exact compiler flags recorded for that file.

#### Scenario: Parse a file with its recorded flags
- **WHEN** `compile_commands.json` contains an entry for `src/bigint/bigint.cc` with flags `["-std=c++17", "-DV8_COMPRESS_POINTERS", "-I./"]`
- **THEN** the extractor parses `src/bigint/bigint.cc` with those exact flags and produces an `ExtractedFile` result

#### Scenario: Missing file is reported, not crashed
- **WHEN** a file listed in `compile_commands.json` does not exist on disk
- **THEN** the extractor records a parse error for that file and continues processing the remaining files

### Requirement: Extract all C++ declaration kinds
The extractor SHALL extract namespaces, classes, structs, enums (plain and scoped), typedefs/using aliases, free functions, class methods (including constructors, destructors, operators), template declarations, and global/static variables.

#### Scenario: Extract a class with methods and fields
- **WHEN** a C++ file contains `class Foo { int x_; void bar(); };`
- **THEN** `ExtractedClass` has `name="Foo"`, one field `x_` of type `int`, and one method `bar`

#### Scenario: Extract nested types
- **WHEN** a class contains a nested enum and a nested struct
- **THEN** the outer `ExtractedClass` has non-empty `nested_enums` and `nested_classes` lists

#### Scenario: Extract template parameters
- **WHEN** a class template `template<typename T, int N>` is encountered
- **THEN** `ExtractedClass.template_params` contains two entries: a type param `T` and a non-type param `N` with value type `int`

### Requirement: Extract function bodies as IR
The extractor SHALL, when `extract_bodies=True`, convert each function body's Clang cursor tree to an `IRBlock` using the `StmtConverter` and store it in `ExtractedFunction.body_ir`.

#### Scenario: Simple function body extracted
- **WHEN** a function `int add(int a, int b) { return a + b; }` is parsed with `extract_bodies=True`
- **THEN** `body_ir` contains an `IRBlock` with one `IRReturnStmt` containing an `IRBinaryExpr` (ADD, a, b)

#### Scenario: Body extraction failure does not abort extraction
- **WHEN** a function body contains an AST node that `StmtConverter` cannot handle
- **THEN** the function is still extracted with `body_ir` containing an `IRRawStmt` or `None`, and extraction of other functions continues

### Requirement: Cache extracted results per file
The extractor SHALL cache each `ExtractedFile` result as JSON on disk, keyed by file path and modification time, and reuse the cached result on subsequent runs if the file has not changed.

#### Scenario: Cache hit avoids re-parsing
- **WHEN** a file has been previously extracted and its mtime has not changed
- **THEN** the cached JSON is loaded instead of re-invoking libclang

#### Scenario: Cache miss on file change
- **WHEN** a file's mtime is newer than its cached JSON
- **THEN** the file is re-parsed and the cache is updated
