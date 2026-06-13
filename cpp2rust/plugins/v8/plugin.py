"""V8 plugin for cpp2rust.

Provides V8-specific type mappings, module layout, and pattern overrides.
"""

from __future__ import annotations

from typing import Any, Dict, List, Optional

from cpp2rust.core.plugin import CppPlugin


# V8-specific type mappings (base::, v8:: types that aren't in stdlib)
_V8_TYPE_MAP: Dict[str, str] = {
    "base::Mutex": "std::sync::Mutex<()>",
    "base::MutexGuard": "std::sync::MutexGuard<'static, ()>",
    "base::SharedMutex": "std::sync::RwLock<()>",
    "base::Optional": "Option",
    "base::Vector": "&[T]",
    "base::OwnedVector": "Vec",
    "base::TimeDelta": "std::time::Duration",
    "base::TimeTicks": "std::time::Instant",
    "base::Thread": "std::thread::JoinHandle<()>",
    "base::OnceType": "std::sync::Once",
}

# V8 module → crate name mapping
_MODULE_MAP: Dict[str, str] = {
    "base": "v8-base",
    "bigint": "v8-bigint",
    "codegen": "v8-codegen",
    "compiler": "v8-compiler",
    "execution": "v8-execution",
    "heap": "v8-heap",
    "objects": "v8-objects",
    "runtime": "v8-runtime",
    "wasm": "v8-wasm",
}


class V8Plugin(CppPlugin):
    """Plugin for the V8 JavaScript engine."""

    def map_type(self, cpp_type: str) -> Optional[str]:
        return _V8_TYPE_MAP.get(cpp_type.strip())

    def map_macro(self, macro_name: str) -> Optional[str]:
        _macros = {
            "DCHECK": "debug_assert!",
            "DCHECK_EQ": "debug_assert_eq!",
            "DCHECK_NE": "debug_assert_ne!",
            "DCHECK_LT": "debug_assert!(< )",
            "DCHECK_LE": "debug_assert!(<= )",
            "DCHECK_GT": "debug_assert!(> )",
            "DCHECK_GE": "debug_assert!(>= )",
            "CHECK": "assert!",
            "CHECK_EQ": "assert_eq!",
            "CHECK_NE": "assert_ne!",
            "UNREACHABLE": "unreachable!()",
            "FATAL": "panic!",
        }
        return _macros.get(macro_name)

    def crate_for_file(self, file_path: str) -> Optional[str]:
        path_lower = file_path.lower()
        for module, crate in _MODULE_MAP.items():
            if f"/{module}/" in path_lower or path_lower.endswith(f"/{module}"):
                return crate
        if "/v8/" in path_lower or "v8/" in path_lower:
            return "v8-core"
        return None

    def skip_patterns(self) -> List[str]:
        return [
            "*/test/*",
            "*/tests/*",
            "*/testing/*",
            "*_test.cc",
            "*_unittest.cc",
            "*/third_party/*",
            "*/tools/*",
            "*/build/*",
        ]

    def extra_clang_args(self) -> List[str]:
        return [
            "-DV8_ENABLE_CHECKS",
            "-DV8_OS_LINUX",
            "-std=c++20",
        ]

    def prelude_lines(self) -> List[str]:
        return [
            "#![allow(non_snake_case)]",
            "#![allow(dead_code)]",
            "#![allow(unused_variables)]",
            "#![allow(unused_imports)]",
            "#![allow(non_camel_case_types)]",
        ]
