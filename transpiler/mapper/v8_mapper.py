"""V8-specific pattern mapper.

Handles V8-specific C++ patterns that need special treatment during transpilation:
Handle<T>, Tagged<T>, Smi, DCHECK/CHECK macros, V8_INLINE, architecture macros, etc.
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set, Tuple


@dataclass
class V8PatternMatch:
    """Result of matching a V8-specific pattern."""
    pattern_name: str
    rust_replacement: str
    needs_unsafe: bool = False
    imports: List[str] = field(default_factory=list)
    comment: str = ""


class V8Mapper:
    """Maps V8-specific C++ patterns to idiomatic Rust equivalents."""

    def __init__(self):
        self._init_handle_patterns()
        self._init_macro_patterns()
        self._init_attribute_patterns()
        self._init_type_patterns()

    # ─── Handle / Tagged System ──────────────────────────────────────────

    def _init_handle_patterns(self):
        """V8's handle system patterns."""
        self.handle_types: Dict[str, str] = {
            "Handle": "Handle",
            "MaybeHandle": "Option<Handle<{T}>>",
            "DirectHandle": "DirectHandle",
            "MaybeDirectHandle": "Option<DirectHandle<{T}>>",
            "IndirectHandle": "IndirectHandle",
            "MaybeObjectHandle": "Option<Handle<Object>>",
        }

        self.tagged_types: Dict[str, str] = {
            "Tagged": "Tagged",
            "MaybeWeak": "MaybeWeak",
            "TaggedMember": "TaggedMember",
        }

    def map_handle_type(self, cpp_type: str) -> Optional[V8PatternMatch]:
        """Map V8 handle types to Rust equivalents."""
        for cpp_handle, rust_handle in self.handle_types.items():
            pattern = rf"^{cpp_handle}<(.+)>$"
            m = re.match(pattern, cpp_type.strip())
            if m:
                inner = m.group(1).strip()
                if "{T}" in rust_handle:
                    rust_type = rust_handle.replace("{T}", inner)
                else:
                    rust_type = f"{rust_handle}<{inner}>"
                return V8PatternMatch(
                    pattern_name=f"handle:{cpp_handle}",
                    rust_replacement=rust_type,
                    imports=[f"crate::handles::{rust_handle.split('<')[0]}"],
                )

        for cpp_tagged, rust_tagged in self.tagged_types.items():
            pattern = rf"^{cpp_tagged}<(.+)>$"
            m = re.match(pattern, cpp_type.strip())
            if m:
                inner = m.group(1).strip()
                return V8PatternMatch(
                    pattern_name=f"tagged:{cpp_tagged}",
                    rust_replacement=f"{rust_tagged}<{inner}>",
                    imports=[f"crate::objects::tagged::{rust_tagged}"],
                )

        return None

    # ─── Macro Patterns ──────────────────────────────────────────────────

    def _init_macro_patterns(self):
        """V8 macro → Rust mapping."""
        self.macro_map: Dict[str, str] = {
            # Assertions
            "DCHECK": "debug_assert!",
            "DCHECK_EQ": "debug_assert_eq!",
            "DCHECK_NE": "debug_assert_ne!",
            "DCHECK_LT": "debug_assert!({0} < {1})",
            "DCHECK_LE": "debug_assert!({0} <= {1})",
            "DCHECK_GT": "debug_assert!({0} > {1})",
            "DCHECK_GE": "debug_assert!({0} >= {1})",
            "DCHECK_NOT_NULL": "debug_assert!(!{0}.is_null())",
            "DCHECK_NULL": "debug_assert!({0}.is_null())",
            "DCHECK_IMPLIES": "debug_assert!(!{0} || {1})",
            "CHECK": "assert!",
            "CHECK_EQ": "assert_eq!",
            "CHECK_NE": "assert_ne!",
            "CHECK_LT": "assert!({0} < {1})",
            "CHECK_LE": "assert!({0} <= {1})",
            "CHECK_GT": "assert!({0} > {1})",
            "CHECK_GE": "assert!({0} >= {1})",
            "CHECK_NOT_NULL": "assert!(!{0}.is_null())",
            "SLOW_DCHECK": "debug_assert!",
            "FATAL": "panic!",

            # Control flow
            "UNREACHABLE": "unreachable!()",
            "UNIMPLEMENTED": "unimplemented!()",

            # Variable usage
            "USE": "let _ =",
            "STATIC_ASSERT": "const _: () = assert!",

            # Printing / Tracing
            "PrintF": "print!",
            "SNPrintF": "write!",
            "V8_Fatal": "panic!",
        }

    def map_macro(self, macro_name: str, args: List[str] = None) -> Optional[V8PatternMatch]:
        """Map a V8 macro invocation to Rust."""
        if macro_name not in self.macro_map:
            return None

        rust = self.macro_map[macro_name]

        if args and "{0}" in rust:
            # Positional argument substitution
            for i, arg in enumerate(args):
                rust = rust.replace(f"{{{i}}}", arg)
        elif args:
            # Standard macro call with args
            if rust.endswith("!"):
                rust = f"{rust}({', '.join(args)})"
            elif rust.endswith("="):
                # USE → let _ = x;
                rust = f"{rust} {args[0]}"

        return V8PatternMatch(
            pattern_name=f"macro:{macro_name}",
            rust_replacement=rust,
        )

    # ─── Attribute Patterns ──────────────────────────────────────────────

    def _init_attribute_patterns(self):
        """V8 attribute macros → Rust attributes."""
        self.attribute_map: Dict[str, str] = {
            "V8_INLINE": "#[inline]",
            "V8_NOINLINE": "#[inline(never)]",
            "V8_WARN_UNUSED_RESULT": "#[must_use]",
            "V8_NODISCARD": "#[must_use]",
            "V8_NOEXCEPT": "",  # Rust functions don't throw by default
            "V8_EXPORT": "pub",
            "V8_EXPORT_PRIVATE": "pub(crate)",
            "V8_LIKELY": "",    # No direct Rust equivalent (hint only)
            "V8_UNLIKELY": "",
            "V8_ASSUME": "",    # unsafe { std::hint::unreachable_unchecked() } too dangerous
            "V8_DEPRECATE_SOON": "#[deprecated]",
            "V8_DEPRECATED": "#[deprecated]",
            "V8_ENUM_DEPRECATED": "#[deprecated]",
            "V8_HAS_ATTRIBUTE_ALWAYS_INLINE": "",
            "V8_HAS_ATTRIBUTE_NOINLINE": "",
            "PRINTF_FORMAT": "",  # Format string checking, not needed in Rust
            "NON_EXPORTED_BASE": "",
        }

    def map_attribute(self, attr_name: str) -> Optional[str]:
        """Map a V8 attribute macro to a Rust attribute."""
        return self.attribute_map.get(attr_name)

    # ─── V8-Specific Type Patterns ───────────────────────────────────────

    def _init_type_patterns(self):
        """V8-specific type mappings beyond what type_mapper handles."""
        self.v8_type_map: Dict[str, str] = {
            # Core V8 types
            "Smi": "Smi",
            "HeapObject": "HeapObject",
            "Map": "V8Map",  # Avoid conflict with std::collections::HashMap
            "Object": "V8Object",
            "JSObject": "JSObject",
            "JSArray": "JSArray",
            "JSFunction": "JSFunction",
            "JSReceiver": "JSReceiver",
            "String": "V8String",  # Avoid conflict with std::string::String
            "Name": "V8Name",
            "Symbol": "V8Symbol",
            "Code": "V8Code",
            "BytecodeArray": "BytecodeArray",
            "SharedFunctionInfo": "SharedFunctionInfo",
            "FeedbackVector": "FeedbackVector",
            "FixedArray": "FixedArray",
            "FixedDoubleArray": "FixedDoubleArray",
            "WeakFixedArray": "WeakFixedArray",

            # V8 internal types
            "Address": "usize",
            "byte": "u8",
            "Tagged_t": "usize",
            "Builtin": "Builtin",
            "Runtime::FunctionId": "RuntimeFunctionId",
            "ExternalReference": "ExternalReference",

            # V8 base types
            "base::Mutex": "std::sync::Mutex<()>",
            "base::MutexGuard": "std::sync::MutexGuard<'_, ()>",
            "base::RecursiveMutex": "parking_lot::ReentrantMutex<()>",
            "base::SharedMutex": "std::sync::RwLock<()>",
            "base::ConditionVariable": "std::sync::Condvar",
            "base::Optional": "Option",
            "base::Vector": "Vec",  # Actually more like &[T] in many cases
            "base::OwnedVector": "Vec",
            "base::SmallVector": "Vec",  # Could use smallvec crate
            "base::TimeDelta": "std::time::Duration",
            "base::TimeTicks": "std::time::Instant",
            "base::ElapsedTimer": "std::time::Instant",

            # Allocation
            "Isolate": "Isolate",
            "LocalIsolate": "LocalIsolate",
            "Heap": "Heap",
            "LocalHeap": "LocalHeap",
            "Zone": "Zone",
            "ZoneObject": "ZoneObject",
            "HandleScope": "HandleScope",
            "HandleScopeData": "HandleScopeData",
        }

        # Patterns that indicate a type needs unsafe
        self.unsafe_types: Set[str] = {
            "Address",
            "MemoryChunk",
            "PageMetadata",
            "CodeRange",
        }

    def map_v8_type(self, cpp_type: str) -> Optional[V8PatternMatch]:
        """Map a V8-specific type to its Rust equivalent."""
        # Check handle types first
        handle_match = self.map_handle_type(cpp_type)
        if handle_match:
            return handle_match

        # Strip namespace prefixes
        clean = cpp_type.strip()
        for prefix in ("v8::internal::", "v8::base::", "v8::"):
            if clean.startswith(prefix):
                clean = clean[len(prefix):]

        if clean in self.v8_type_map:
            return V8PatternMatch(
                pattern_name=f"v8type:{clean}",
                rust_replacement=self.v8_type_map[clean],
                needs_unsafe=clean in self.unsafe_types,
            )

        return None

    # ─── Architecture / Platform Conditional ─────────────────────────────

    # C++ preprocessor define → Rust cfg attribute
    ARCH_CFG: Dict[str, str] = {
        "V8_TARGET_ARCH_X64": '#[cfg(target_arch = "x86_64")]',
        "V8_TARGET_ARCH_IA32": '#[cfg(target_arch = "x86")]',
        "V8_TARGET_ARCH_ARM64": '#[cfg(target_arch = "aarch64")]',
        "V8_TARGET_ARCH_ARM": '#[cfg(target_arch = "arm")]',
        "V8_TARGET_ARCH_MIPS64": '#[cfg(target_arch = "mips64")]',
        "V8_TARGET_ARCH_PPC64": '#[cfg(target_arch = "powerpc64")]',
        "V8_TARGET_ARCH_S390X": '#[cfg(target_arch = "s390x")]',
        "V8_TARGET_ARCH_RISCV64": '#[cfg(target_arch = "riscv64")]',
        "V8_TARGET_ARCH_LOONG64": '#[cfg(target_arch = "loongarch64")]',
    }

    OS_CFG: Dict[str, str] = {
        "V8_OS_LINUX": '#[cfg(target_os = "linux")]',
        "V8_OS_MACOS": '#[cfg(target_os = "macos")]',
        "V8_OS_MACOSX": '#[cfg(target_os = "macos")]',
        "V8_OS_WIN": '#[cfg(target_os = "windows")]',
        "V8_OS_WIN32": '#[cfg(target_os = "windows")]',
        "V8_OS_WIN64": '#[cfg(all(target_os = "windows", target_arch = "x86_64"))]',
        "V8_OS_ANDROID": '#[cfg(target_os = "android")]',
        "V8_OS_FUCHSIA": '#[cfg(target_os = "fuchsia")]',
        "V8_OS_FREEBSD": '#[cfg(target_os = "freebsd")]',
        "V8_OS_OPENBSD": '#[cfg(target_os = "openbsd")]',
        "V8_OS_POSIX": '#[cfg(unix)]',
    }

    FEATURE_CFG: Dict[str, str] = {
        "V8_ENABLE_WEBASSEMBLY": '#[cfg(feature = "webassembly")]',
        "V8_INTL_SUPPORT": '#[cfg(feature = "intl")]',
        "V8_ENABLE_SANDBOX": '#[cfg(feature = "sandbox")]',
        "V8_COMPRESS_POINTERS": '#[cfg(feature = "compress_pointers")]',
        "V8_31BIT_SMIS_ON_64BIT_ARCH": '#[cfg(feature = "smi_31bit")]',
        "V8_SANDBOXED_EXTERNAL_POINTERS": '#[cfg(feature = "sandbox")]',
        "DEBUG": '#[cfg(debug_assertions)]',
        "V8_ENABLE_CHECKS": '#[cfg(debug_assertions)]',
        "OBJECT_PRINT": '#[cfg(feature = "object_print")]',
        "V8_TRACE_MAPS": '#[cfg(feature = "trace_maps")]',
        "V8_ENABLE_CONSERVATIVE_STACK_SCANNING": '#[cfg(feature = "conservative_gc")]',
    }

    def map_ifdef(self, define: str) -> Optional[str]:
        """Map a C++ #ifdef/#if defined() to a Rust #[cfg(...)] attribute."""
        define = define.strip()
        if define in self.ARCH_CFG:
            return self.ARCH_CFG[define]
        if define in self.OS_CFG:
            return self.OS_CFG[define]
        if define in self.FEATURE_CFG:
            return self.FEATURE_CFG[define]
        return None

    def map_arch_directory(self, dir_name: str) -> Optional[str]:
        """Map an architecture directory name to a Rust cfg attribute.
        V8 uses directories like src/codegen/x64/, src/baseline/arm64/, etc.
        """
        arch_dirs: Dict[str, str] = {
            "x64": '#[cfg(target_arch = "x86_64")]',
            "ia32": '#[cfg(target_arch = "x86")]',
            "arm64": '#[cfg(target_arch = "aarch64")]',
            "arm": '#[cfg(target_arch = "arm")]',
            "mips64": '#[cfg(target_arch = "mips64")]',
            "ppc": '#[cfg(target_arch = "powerpc64")]',
            "s390": '#[cfg(target_arch = "s390x")]',
            "riscv": '#[cfg(target_arch = "riscv64")]',
            "riscv64": '#[cfg(target_arch = "riscv64")]',
            "loong64": '#[cfg(target_arch = "loongarch64")]',
        }
        return arch_dirs.get(dir_name)

    # ─── V8 Object Accessor Macros ───────────────────────────────────────

    def expand_accessor_macro(self, macro_name: str, args: List[str]) -> List[str]:
        """Expand V8 object accessor macros into Rust getter/setter method signatures.

        e.g., DECL_ACCESSORS(name, Type) → pub fn name(&self) -> Type, pub fn set_name(&mut self, value: Type)
        """
        result = []

        if macro_name in ("DECL_ACCESSORS", "DECL_GETTER"):
            if len(args) >= 2:
                field_name = args[0].strip()
                field_type = args[1].strip()
                result.append(f"pub fn {field_name}(&self) -> {field_type}")
                if macro_name == "DECL_ACCESSORS":
                    result.append(f"pub fn set_{field_name}(&mut self, value: {field_type})")

        elif macro_name in ("DECL_BOOLEAN_ACCESSORS",):
            if len(args) >= 1:
                field_name = args[0].strip()
                result.append(f"pub fn {field_name}(&self) -> bool")
                result.append(f"pub fn set_{field_name}(&mut self, value: bool)")

        elif macro_name in ("DECL_INT_ACCESSORS",):
            if len(args) >= 1:
                field_name = args[0].strip()
                result.append(f"pub fn {field_name}(&self) -> i32")
                result.append(f"pub fn set_{field_name}(&mut self, value: i32)")

        elif macro_name in ("DECL_UINT16_ACCESSORS",):
            if len(args) >= 1:
                field_name = args[0].strip()
                result.append(f"pub fn {field_name}(&self) -> u16")
                result.append(f"pub fn set_{field_name}(&mut self, value: u16)")

        elif macro_name in ("DECL_RELAXED_GETTER", "DECL_ACQUIRE_GETTER"):
            if len(args) >= 2:
                field_name = args[0].strip()
                field_type = args[1].strip()
                result.append(f"pub fn {field_name}(&self) -> {field_type}")

        return result

    # ─── Name Mangling ───────────────────────────────────────────────────

    def v8_class_to_rust(self, class_name: str) -> str:
        """Convert a V8 class name to avoid Rust standard library conflicts."""
        conflicts = {
            "String": "V8String",
            "Object": "V8Object",
            "Map": "V8Map",
            "Name": "V8Name",
            "Symbol": "V8Symbol",
            "Code": "V8Code",
            "Number": "V8Number",
            "Context": "V8Context",
        }
        return conflicts.get(class_name, class_name)

    def is_v8_gc_type(self, type_name: str) -> bool:
        """Check if a type is a V8 garbage-collected heap object."""
        gc_base_types = {
            "HeapObject", "JSObject", "JSReceiver", "JSArray",
            "JSFunction", "FixedArray", "Code", "BytecodeArray",
            "SharedFunctionInfo", "Map", "String", "Name", "Symbol",
            "Context", "NativeContext", "FeedbackVector", "WeakFixedArray",
            "DescriptorArray", "TransitionArray", "PropertyArray",
            "ScopeInfo", "Script", "ClosureFeedbackCellArray",
        }
        clean = type_name.strip()
        for prefix in ("v8::internal::", "v8::"):
            if clean.startswith(prefix):
                clean = clean[len(prefix):]
        return clean in gc_base_types
