"""Test the always-compiles invariant.

IRRawCppExpr and IRRawCppStmt must NEVER produce todo!() in emitted output.
"""

import pytest
from cpp2rust.core.emit.unsafe_emitter import UnsafeEmitter
from cpp2rust.core.ir.nodes import (
    IRFile, IRFunction, IRBlock, IRRawCppExpr, IRRawCppStmt,
    IRParam, IRType, TypeKind, IRReturnStmt, IRExprStmt, AccessSpecifier,
    IRLiteral, LiteralKind,
)


def make_emitter():
    return UnsafeEmitter()


def make_fn_with(statements, ret=None):
    return IRFunction(
        name="test_fn",
        params=[],
        return_type=IRType(name="i32", cpp_name="int", kind=TypeKind.PRIMITIVE),
        body=IRBlock(statements=statements + ([IRReturnStmt(value=ret)] if ret else [])),
        access=AccessSpecifier.PUBLIC,
    )


def emit(items):
    ir_file = IRFile(path="src/test.rs", cpp_source_files=["test.cpp"], items=items)
    return UnsafeEmitter().emit_file(ir_file)


class TestNoTodo:
    def test_raw_cpp_stmt_no_todo(self):
        fn = make_fn_with([IRRawCppStmt(cpp_source="complex_cpp_call();")])
        src = emit([fn])
        assert "todo!" not in src

    def test_raw_cpp_expr_no_todo(self):
        fn = make_fn_with([], ret=IRRawCppExpr(cpp_source="a + b", default_type="i32"))
        src = emit([fn])
        assert "todo!" not in src

    def test_raw_cpp_stmt_emits_unsafe_block(self):
        fn = make_fn_with([IRRawCppStmt(cpp_source="int x = 5;")])
        src = emit([fn])
        assert "unsafe" in src
        assert "let _ = 0" in src

    def test_raw_cpp_expr_emits_default(self):
        fn = make_fn_with([], ret=IRRawCppExpr(cpp_source="expr", default_type="u64"))
        src = emit([fn])
        assert "Default>::default()" in src

    def test_raw_cpp_stmt_includes_comment(self):
        fn = make_fn_with([IRRawCppStmt(cpp_source="foo->bar(x, y);")])
        src = emit([fn])
        assert "UNTRANSLATED" in src
        assert "bar" in src  # -> may be post-processed to .

    def test_raw_cpp_expr_includes_comment(self):
        fn = make_fn_with([], ret=IRRawCppExpr(cpp_source="foo()", comment="unhandled call"))
        src = emit([fn])
        assert "UNTRANSLATED" in src

    def test_comment_only_stmt_passthrough(self):
        fn = make_fn_with([IRRawCppStmt(cpp_source="// this is a comment")])
        src = emit([fn])
        assert "// this is a comment" in src
        assert "UNTRANSLATED" not in src

    def test_no_todo_in_full_file(self):
        stmts = [
            IRRawCppStmt(cpp_source="int a = 1;"),
            IRRawCppStmt(cpp_source="a += 2;"),
        ]
        ret = IRRawCppExpr(cpp_source="a", default_type="i32")
        fn = make_fn_with(stmts, ret=ret)
        src = emit([fn])
        assert "todo!" not in src


class TestTypeMapper:
    def test_std_string(self):
        from cpp2rust.core.mapper.type_mapper import TypeMapper
        tm = TypeMapper()
        t = tm.map_type("std::string")
        assert t.name == "String"

    def test_uint64_t(self):
        from cpp2rust.core.mapper.type_mapper import TypeMapper
        tm = TypeMapper()
        t = tm.map_type("uint64_t")
        assert t.name == "u64"

    def test_plugin_override(self):
        from cpp2rust.core.mapper.type_mapper import TypeMapper
        from cpp2rust.plugins.v8.plugin import V8Plugin
        tm = TypeMapper(plugin=V8Plugin())
        t = tm.map_type("base::Mutex")
        assert "Mutex" in t.name

    def test_pointer_type(self):
        from cpp2rust.core.mapper.type_mapper import TypeMapper
        tm = TypeMapper()
        t = tm.map_type("int*")
        assert t.is_pointer or t.is_mut_pointer

    def test_vector_template(self):
        from cpp2rust.core.mapper.type_mapper import TypeMapper
        tm = TypeMapper()
        t = tm.map_type("std::vector<int32_t>")
        assert "Vec" in t.name


class TestPatternLibrary:
    def test_load_builtin(self):
        from cpp2rust.core.pattern_library import PatternLibrary
        lib = PatternLibrary.load_builtin()
        assert lib is not None

    def test_add_and_lookup(self):
        from cpp2rust.core.pattern_library import PatternLibrary, PatternEntry
        lib = PatternLibrary()
        lib.add(PatternEntry(cpp="DCHECK($x)", rust="debug_assert!($x)", kind="macro"))
        entry = lib.lookup("DCHECK($x)")
        assert entry is not None
        assert entry.rust == "debug_assert!($x)"

    def test_lookup_missing_returns_none(self):
        from cpp2rust.core.pattern_library import PatternLibrary
        lib = PatternLibrary()
        assert lib.lookup("nonexistent") is None
