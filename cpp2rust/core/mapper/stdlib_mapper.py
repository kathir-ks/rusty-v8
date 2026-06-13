"""C++ standard library function call → Rust equivalent mapper.

Maps common C++ std library method calls and free functions to their
idiomatic Rust equivalents.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple


@dataclass
class StdlibMapping:
    """A mapping from a C++ std call to Rust."""
    rust_call: str              # Rust replacement (with {0}, {1}, ... for args)
    is_method: bool = True      # True if Rust version is a method call
    needs_import: str = ""      # Import path if needed
    is_unsafe: bool = False
    comment: str = ""


class StdlibMapper:
    """Maps C++ standard library function/method calls to Rust equivalents."""

    def __init__(self):
        self._init_method_maps()
        self._init_free_function_maps()
        self._init_stream_maps()

    def _init_method_maps(self):
        """C++ method calls on std types → Rust method calls."""
        # key = (receiver_type_pattern, method_name)
        # value = StdlibMapping
        self.method_map: Dict[Tuple[str, str], StdlibMapping] = {
            # ─── vector ──────────────────────────────────────────────
            ("vector", "push_back"): StdlibMapping("{obj}.push({0})"),
            ("vector", "emplace_back"): StdlibMapping("{obj}.push({0})"),
            ("vector", "pop_back"): StdlibMapping("{obj}.pop()"),
            ("vector", "size"): StdlibMapping("{obj}.len()"),
            ("vector", "empty"): StdlibMapping("{obj}.is_empty()"),
            ("vector", "clear"): StdlibMapping("{obj}.clear()"),
            ("vector", "reserve"): StdlibMapping("{obj}.reserve({0})"),
            ("vector", "resize"): StdlibMapping("{obj}.resize({0}, {1})"),
            ("vector", "capacity"): StdlibMapping("{obj}.capacity()"),
            ("vector", "front"): StdlibMapping("{obj}.first().unwrap()"),
            ("vector", "back"): StdlibMapping("{obj}.last().unwrap()"),
            ("vector", "data"): StdlibMapping("{obj}.as_ptr()"),
            ("vector", "begin"): StdlibMapping("{obj}.iter()"),
            ("vector", "end"): StdlibMapping("/* end */"),
            ("vector", "at"): StdlibMapping("{obj}[{0}]"),
            ("vector", "insert"): StdlibMapping("{obj}.insert({0}, {1})"),
            ("vector", "erase"): StdlibMapping("{obj}.remove({0})"),
            ("vector", "shrink_to_fit"): StdlibMapping("{obj}.shrink_to_fit()"),

            # ─── string ──────────────────────────────────────────────
            ("string", "c_str"): StdlibMapping("{obj}.as_ptr()"),
            ("string", "length"): StdlibMapping("{obj}.len()"),
            ("string", "size"): StdlibMapping("{obj}.len()"),
            ("string", "empty"): StdlibMapping("{obj}.is_empty()"),
            ("string", "clear"): StdlibMapping("{obj}.clear()"),
            ("string", "substr"): StdlibMapping("&{obj}[{0}..{0}+{1}]"),
            ("string", "find"): StdlibMapping("{obj}.find({0})"),
            ("string", "rfind"): StdlibMapping("{obj}.rfind({0})"),
            ("string", "append"): StdlibMapping("{obj}.push_str({0})"),
            ("string", "push_back"): StdlibMapping("{obj}.push({0})"),
            ("string", "compare"): StdlibMapping("{obj}.cmp({0})"),
            ("string", "starts_with"): StdlibMapping("{obj}.starts_with({0})"),
            ("string", "ends_with"): StdlibMapping("{obj}.ends_with({0})"),
            ("string", "replace"): StdlibMapping("{obj}.replace({0}, {1})"),
            ("string", "data"): StdlibMapping("{obj}.as_bytes()"),

            # ─── map / unordered_map ─────────────────────────────────
            ("map", "find"): StdlibMapping("{obj}.get({0})"),
            ("map", "count"): StdlibMapping("if {obj}.contains_key({0}) {{ 1 }} else {{ 0 }}"),
            ("map", "contains"): StdlibMapping("{obj}.contains_key({0})"),
            ("map", "insert"): StdlibMapping("{obj}.insert({0}, {1})"),
            ("map", "emplace"): StdlibMapping("{obj}.insert({0}, {1})"),
            ("map", "erase"): StdlibMapping("{obj}.remove({0})"),
            ("map", "size"): StdlibMapping("{obj}.len()"),
            ("map", "empty"): StdlibMapping("{obj}.is_empty()"),
            ("map", "clear"): StdlibMapping("{obj}.clear()"),
            ("map", "at"): StdlibMapping("{obj}[{0}]"),
            ("map", "begin"): StdlibMapping("{obj}.iter()"),
            ("unordered_map", "find"): StdlibMapping("{obj}.get({0})"),
            ("unordered_map", "count"): StdlibMapping("if {obj}.contains_key({0}) {{ 1 }} else {{ 0 }}"),
            ("unordered_map", "insert"): StdlibMapping("{obj}.insert({0}, {1})"),
            ("unordered_map", "emplace"): StdlibMapping("{obj}.insert({0}, {1})"),
            ("unordered_map", "erase"): StdlibMapping("{obj}.remove({0})"),
            ("unordered_map", "size"): StdlibMapping("{obj}.len()"),
            ("unordered_map", "empty"): StdlibMapping("{obj}.is_empty()"),

            # ─── set / unordered_set ─────────────────────────────────
            ("set", "insert"): StdlibMapping("{obj}.insert({0})"),
            ("set", "find"): StdlibMapping("{obj}.get({0})"),
            ("set", "count"): StdlibMapping("if {obj}.contains({0}) {{ 1 }} else {{ 0 }}"),
            ("set", "contains"): StdlibMapping("{obj}.contains({0})"),
            ("set", "erase"): StdlibMapping("{obj}.remove({0})"),
            ("set", "size"): StdlibMapping("{obj}.len()"),
            ("set", "empty"): StdlibMapping("{obj}.is_empty()"),
            ("unordered_set", "insert"): StdlibMapping("{obj}.insert({0})"),
            ("unordered_set", "contains"): StdlibMapping("{obj}.contains({0})"),
            ("unordered_set", "erase"): StdlibMapping("{obj}.remove({0})"),
            ("unordered_set", "size"): StdlibMapping("{obj}.len()"),

            # ─── unique_ptr / shared_ptr ─────────────────────────────
            ("unique_ptr", "get"): StdlibMapping("{obj}.as_ref()"),
            ("unique_ptr", "release"): StdlibMapping("Box::into_raw({obj})"),
            ("unique_ptr", "reset"): StdlibMapping("*{obj} = {0}"),
            ("shared_ptr", "get"): StdlibMapping("Arc::as_ptr(&{obj})"),
            ("shared_ptr", "use_count"): StdlibMapping("Arc::strong_count(&{obj})"),
            ("shared_ptr", "reset"): StdlibMapping("*{obj} = Arc::new({0})"),
            ("weak_ptr", "lock"): StdlibMapping("{obj}.upgrade()"),
            ("weak_ptr", "expired"): StdlibMapping("{obj}.strong_count() == 0"),

            # ─── optional ────────────────────────────────────────────
            ("optional", "has_value"): StdlibMapping("{obj}.is_some()"),
            ("optional", "value"): StdlibMapping("{obj}.unwrap()"),
            ("optional", "value_or"): StdlibMapping("{obj}.unwrap_or({0})"),
            ("optional", "reset"): StdlibMapping("{obj} = None"),
            ("optional", "emplace"): StdlibMapping("{obj} = Some({0})"),

            # ─── mutex ───────────────────────────────────────────────
            ("mutex", "lock"): StdlibMapping("{obj}.lock().unwrap()"),
            ("mutex", "try_lock"): StdlibMapping("{obj}.try_lock()"),
            ("mutex", "unlock"): StdlibMapping("/* unlock is automatic via drop */"),

            # ─── atomic ──────────────────────────────────────────────
            ("atomic", "load"): StdlibMapping("{obj}.load(std::sync::atomic::Ordering::SeqCst)"),
            ("atomic", "store"): StdlibMapping("{obj}.store({0}, std::sync::atomic::Ordering::SeqCst)"),
            ("atomic", "exchange"): StdlibMapping("{obj}.swap({0}, std::sync::atomic::Ordering::SeqCst)"),
            ("atomic", "compare_exchange"): StdlibMapping(
                "{obj}.compare_exchange({0}, {1}, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst)"
            ),
            ("atomic", "fetch_add"): StdlibMapping("{obj}.fetch_add({0}, std::sync::atomic::Ordering::SeqCst)"),
            ("atomic", "fetch_sub"): StdlibMapping("{obj}.fetch_sub({0}, std::sync::atomic::Ordering::SeqCst)"),
        }

    def _init_free_function_maps(self):
        """C++ std:: free functions → Rust equivalents."""
        self.free_fn_map: Dict[str, StdlibMapping] = {
            # Move/Forward semantics
            "std::move": StdlibMapping("{0}", is_method=False,
                                       comment="Rust moves by default"),
            "std::forward": StdlibMapping("{0}", is_method=False),

            # Smart pointer construction
            "std::make_unique": StdlibMapping("Box::new({0})", is_method=False),
            "std::make_shared": StdlibMapping("Arc::new({0})", is_method=False,
                                              needs_import="std::sync::Arc"),

            # Algorithms
            "std::sort": StdlibMapping("{0}.sort()", is_method=False),
            "std::stable_sort": StdlibMapping("{0}.sort()", is_method=False),
            "std::reverse": StdlibMapping("{0}.reverse()", is_method=False),
            "std::find": StdlibMapping("{0}.iter().find(|&&x| x == {1})", is_method=False),
            "std::count": StdlibMapping("{0}.iter().filter(|&&x| x == {1}).count()", is_method=False),
            "std::fill": StdlibMapping("{0}.fill({1})", is_method=False),
            "std::copy": StdlibMapping("{2}.copy_from_slice(&{0}[..{1}])", is_method=False),
            "std::transform": StdlibMapping("{0}.iter().map({1}).collect()", is_method=False),
            "std::accumulate": StdlibMapping("{0}.iter().fold({2}, {3})", is_method=False),
            "std::any_of": StdlibMapping("{0}.iter().any({1})", is_method=False),
            "std::all_of": StdlibMapping("{0}.iter().all({1})", is_method=False),
            "std::none_of": StdlibMapping("{0}.iter().all(|x| !{1}(x))", is_method=False),
            "std::lower_bound": StdlibMapping("{0}.partition_point(|x| x < &{1})", is_method=False),
            "std::upper_bound": StdlibMapping("{0}.partition_point(|x| x <= &{1})", is_method=False),
            "std::binary_search": StdlibMapping("{0}.binary_search(&{1})", is_method=False),
            "std::unique": StdlibMapping("{0}.dedup()", is_method=False),
            "std::remove_if": StdlibMapping("{0}.retain(|x| !{1}(x))", is_method=False),
            "std::for_each": StdlibMapping("{0}.iter().for_each({1})", is_method=False),

            # Math
            "std::min": StdlibMapping("{0}.min({1})", is_method=False),
            "std::max": StdlibMapping("{0}.max({1})", is_method=False),
            "std::abs": StdlibMapping("{0}.abs()", is_method=False),
            "std::clamp": StdlibMapping("{0}.clamp({1}, {2})", is_method=False),
            "std::swap": StdlibMapping("std::mem::swap(&mut {0}, &mut {1})", is_method=False,
                                       needs_import="std::mem"),
            "std::ceil": StdlibMapping("{0}.ceil()", is_method=False),
            "std::floor": StdlibMapping("{0}.floor()", is_method=False),
            "std::round": StdlibMapping("{0}.round()", is_method=False),
            "std::sqrt": StdlibMapping("{0}.sqrt()", is_method=False),
            "std::pow": StdlibMapping("{0}.powf({1})", is_method=False),
            "std::log": StdlibMapping("{0}.ln()", is_method=False),
            "std::log2": StdlibMapping("{0}.log2()", is_method=False),
            "std::log10": StdlibMapping("{0}.log10()", is_method=False),
            "std::isnan": StdlibMapping("{0}.is_nan()", is_method=False),
            "std::isinf": StdlibMapping("{0}.is_infinite()", is_method=False),
            "std::isfinite": StdlibMapping("{0}.is_finite()", is_method=False),

            # String conversion
            "std::to_string": StdlibMapping("{0}.to_string()", is_method=False),
            "std::stoi": StdlibMapping("{0}.parse::<i32>().unwrap()", is_method=False),
            "std::stol": StdlibMapping("{0}.parse::<i64>().unwrap()", is_method=False),
            "std::stof": StdlibMapping("{0}.parse::<f32>().unwrap()", is_method=False),
            "std::stod": StdlibMapping("{0}.parse::<f64>().unwrap()", is_method=False),

            # Memory
            "memcpy": StdlibMapping(
                "unsafe {{ std::ptr::copy_nonoverlapping({1} as *const u8, {0} as *mut u8, {2}) }}",
                is_method=False, is_unsafe=True),
            "memmove": StdlibMapping(
                "unsafe {{ std::ptr::copy({1} as *const u8, {0} as *mut u8, {2}) }}",
                is_method=False, is_unsafe=True),
            "memset": StdlibMapping(
                "unsafe {{ std::ptr::write_bytes({0} as *mut u8, {1} as u8, {2}) }}",
                is_method=False, is_unsafe=True),
            "memcmp": StdlibMapping(
                "unsafe {{ libc::memcmp({0}, {1}, {2}) }}",
                is_method=False, is_unsafe=True, needs_import="libc"),
            "malloc": StdlibMapping(
                "unsafe {{ std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked({0}, 8)) }}",
                is_method=False, is_unsafe=True),
            "free": StdlibMapping(
                "unsafe {{ std::alloc::dealloc({0} as *mut u8, std::alloc::Layout::from_size_align_unchecked(0, 8)) }}",
                is_method=False, is_unsafe=True),
            "calloc": StdlibMapping(
                "unsafe {{ std::alloc::alloc_zeroed(std::alloc::Layout::from_size_align_unchecked({0} * {1}, 8)) }}",
                is_method=False, is_unsafe=True),

            # IO
            "printf": StdlibMapping("print!({0})", is_method=False),
            "fprintf": StdlibMapping("write!({0}, {1})", is_method=False),
            "snprintf": StdlibMapping("write!({0}, {1})", is_method=False),

            # Assertions
            "assert": StdlibMapping("assert!({0})", is_method=False),
            "static_assert": StdlibMapping("const _: () = assert!({0})", is_method=False),

            # Threading
            "std::this_thread::sleep_for": StdlibMapping(
                "std::thread::sleep({0})", is_method=False),
            "std::this_thread::yield_now": StdlibMapping(
                "std::thread::yield_now()", is_method=False),
        }

    def _init_stream_maps(self):
        """C++ stream operations → Rust print/write macros."""
        self.stream_targets: Dict[str, str] = {
            "std::cout": "print!",
            "std::cerr": "eprint!",
            "std::endl": r'"\n"',
            "std::flush": "",
        }

    def map_method_call(self, receiver_type: str, method_name: str,
                        receiver_expr: str, args: List[str]) -> Optional[str]:
        """Map a C++ method call to Rust.

        Args:
            receiver_type: The C++ type of the receiver (e.g., "std::vector<int>")
            method_name: The method being called (e.g., "push_back")
            receiver_expr: The Rust expression for the receiver (e.g., "vec")
            args: List of Rust argument expressions

        Returns:
            Rust expression string, or None if no mapping found.
        """
        # Normalize receiver type to base container name
        base_type = self._extract_base_type(receiver_type)

        key = (base_type, method_name)
        mapping = self.method_map.get(key)
        if not mapping:
            return None

        return self._apply_mapping(mapping, receiver_expr, args)

    def map_free_function(self, func_name: str, args: List[str]) -> Optional[str]:
        """Map a C++ free function call to Rust.

        Args:
            func_name: Fully qualified function name (e.g., "std::sort")
            args: List of Rust argument expressions

        Returns:
            Rust expression string, or None if no mapping found.
        """
        mapping = self.free_fn_map.get(func_name)
        if not mapping:
            return None

        return self._apply_mapping(mapping, "", args)

    def get_import_for_method(self, receiver_type: str, method_name: str) -> Optional[str]:
        """Get the Rust import needed for a mapped method call."""
        base_type = self._extract_base_type(receiver_type)
        key = (base_type, method_name)
        mapping = self.method_map.get(key)
        return mapping.needs_import if mapping else None

    def get_import_for_function(self, func_name: str) -> Optional[str]:
        """Get the Rust import needed for a mapped free function."""
        mapping = self.free_fn_map.get(func_name)
        return mapping.needs_import if mapping else None

    def is_known_method(self, receiver_type: str, method_name: str) -> bool:
        """Check if we have a mapping for this method call."""
        base_type = self._extract_base_type(receiver_type)
        return (base_type, method_name) in self.method_map

    def is_known_function(self, func_name: str) -> bool:
        """Check if we have a mapping for this free function."""
        return func_name in self.free_fn_map

    # ─── Helpers ─────────────────────────────────────────────────────────

    def _extract_base_type(self, cpp_type: str) -> str:
        """Extract the base container type name from a C++ type string.
        e.g., "std::vector<int>" → "vector", "std::unordered_map<K,V>" → "unordered_map"
        """
        t = cpp_type.strip()
        # Remove std:: prefix
        if t.startswith("std::"):
            t = t[5:]
        # Remove template args
        angle = t.find("<")
        if angle >= 0:
            t = t[:angle]
        return t.strip()

    def _apply_mapping(self, mapping: StdlibMapping,
                       receiver: str, args: List[str]) -> str:
        """Apply a mapping template with receiver and args."""
        result = mapping.rust_call

        # Replace {obj} with receiver
        result = result.replace("{obj}", receiver)

        # Replace {0}, {1}, ... with args
        for i, arg in enumerate(args):
            result = result.replace(f"{{{i}}}", arg)

        return result
