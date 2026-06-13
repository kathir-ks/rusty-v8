"""Default plugin for cpp2rust — used when no --plugin is specified.

Maps standard C++ stdlib types to idiomatic Rust equivalents.
"""

from __future__ import annotations

from typing import Any, Dict, List, Optional

from cpp2rust.core.plugin import CppPlugin


class DefaultPlugin(CppPlugin):
    """Default plugin with standard C++ stdlib mappings."""

    _TYPE_MAP: Dict[str, str] = {
        "std::string": "String",
        "std::string_view": "&str",
        "std::wstring": "Vec<u16>",
        "std::vector": "Vec",        # generic arg handled by type_mapper
        "std::map": "std::collections::BTreeMap",
        "std::unordered_map": "std::collections::HashMap",
        "std::set": "std::collections::BTreeSet",
        "std::unordered_set": "std::collections::HashSet",
        "std::pair": "()",           # handled by type_mapper
        "std::optional": "Option",
        "std::unique_ptr": "Box",
        "std::shared_ptr": "std::sync::Arc",
        "std::weak_ptr": "std::sync::Weak",
        "std::function": "Box<dyn Fn>",
        "std::nullptr_t": "*mut ()",
        "int8_t": "i8",
        "int16_t": "i16",
        "int32_t": "i32",
        "int64_t": "i64",
        "uint8_t": "u8",
        "uint16_t": "u16",
        "uint32_t": "u32",
        "uint64_t": "u64",
        "size_t": "usize",
        "ptrdiff_t": "isize",
        "intptr_t": "isize",
        "uintptr_t": "usize",
    }

    def map_type(self, cpp_type: str) -> Optional[str]:
        return self._TYPE_MAP.get(cpp_type.strip())
