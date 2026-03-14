SYSTEM_PROMPT = """\
You are an expert systems programmer specializing in C++ to Rust migration.
You are converting the V8 JavaScript engine from C++ to Rust.

Rules:
1. Produce COMPLETE, COMPILABLE Rust code. Never use placeholder comments like
   "// TODO", "unimplemented!()", or "todo!()".
2. Convert C++ classes to Rust structs + impl blocks + traits where appropriate.
3. Convert C++ templates to Rust generics with trait bounds.
4. Convert raw pointers to safe Rust types (Box, Arc, Rc, references) where possible.
   Use unsafe only when strictly necessary and document why.
5. Convert C++ exceptions and error codes to Rust Result/Option types.
6. Convert #define macros to const, const fn, or macro_rules! as appropriate.
7. Use standard Rust naming: snake_case for functions/variables, PascalCase for types,
   SCREAMING_SNAKE_CASE for constants.
8. Preserve the logical structure and algorithmic intent of the original code.
9. Add appropriate derives (Debug, Clone, PartialEq, etc.) to structs and enums.
10. Use Rust's module system correctly (pub, pub(crate), etc.).
11. Return ONLY raw Rust code — no markdown code fences, no explanations, no commentary.\
"""


def module_conversion_prompt(
    module_name: str,
    source_files: list[dict],
    dependency_context: str = "",
    crate_name: str = "",
    type_inventory: str = "",
    prior_batch_context: str = "",
) -> str:
    crate = crate_name or f"v8_{module_name.replace('-', '_')}"

    files_section = ""
    for f in source_files:
        files_section += f"\n--- File: {f['path']} ---\n{f['content']}\n"

    dep_section = ""
    if dependency_context:
        dep_section = f"""
## Already-Converted Dependencies

The following Rust types and functions are available from dependency crates.
Use them directly — do not redefine them.

{dependency_context}
"""

    type_section = ""
    if type_inventory:
        type_section = f"""
## Module Type Inventory

The following C++ types, enums, and constants are defined across ALL files in
the `{module_name}` module (not just the files in this batch). Use this
inventory to ensure consistent type usage. When you encounter a reference to
one of these types, use the same Rust name consistently. Define shared types
in the file where they are most naturally used, and `use` them in other files.

```cpp
{type_inventory}
```
"""

    prior_section = ""
    if prior_batch_context:
        prior_section = f"""
## Already Converted in This Module (prior batches)

The following Rust types, functions, and constants have already been generated
by earlier batches of this same module. Do NOT redefine them — instead, import
them with `use crate::<module>::<item>`. Ensure your code is compatible with
these existing definitions.

```rust
{prior_batch_context}
```
"""

    return f"""\
Convert the following C++ files from V8's `{module_name}` module to Rust.
The output will be part of the `{crate}` crate.
{dep_section}{type_section}{prior_section}
## C++ Source Files
{files_section}

## Output Format

For each input C++ file, produce the corresponding Rust file.
Separate each file with a marker line like this:

// === FILE: <filename>.rs ===

Then write the complete Rust code for that file.
Every function must have a real implementation — not a placeholder.
If complex logic cannot be directly translated, provide the closest safe Rust
equivalent with unsafe blocks where truly necessary.\
"""


def fix_errors_prompt(
    rust_code: str,
    errors: str,
    original_cpp: str = "",
) -> str:
    cpp_section = ""
    if original_cpp:
        cpp_section = f"""
## Original C++ Source (for reference)

{original_cpp}
"""

    return f"""\
Fix the following Rust code based on the compiler errors.

## Current Rust Code

{rust_code}

## Compiler Errors

{errors}
{cpp_section}
Return the COMPLETE fixed Rust code. Do not omit any functions or types.
Return ONLY the fixed Rust code — no markdown fences, no explanations.\
"""


def analysis_prompt(file_path: str, content: str) -> str:
    return f"""\
Analyze this C++ file from the V8 JavaScript engine and return a JSON object.

File: {file_path}

```cpp
{content}
```

Return ONLY a JSON object (no markdown fences) with this structure:
{{
    "file": "{file_path}",
    "purpose": "<brief description>",
    "namespaces": ["<namespace names>"],
    "classes": [
        {{
            "name": "<class name>",
            "bases": ["<base class names>"],
            "methods": ["<method names>"],
            "is_template": false
        }}
    ],
    "functions": [
        {{
            "name": "<function name>",
            "return_type": "<return type>",
            "is_template": false
        }}
    ],
    "includes": ["<include paths>"],
    "macros": ["<macro names>"],
    "typedefs": ["<typedef/using names>"]
}}\
"""
