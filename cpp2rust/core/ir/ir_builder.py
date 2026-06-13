"""Converts ExtractedFile objects from the AST parser into IRFile objects."""

from __future__ import annotations

import logging
import re
from pathlib import Path

logger = logging.getLogger(__name__)


def extracted_to_ir_file(ef, plugin=None):
    """Convert an ExtractedFile to an IRFile."""
    from cpp2rust.core.ir.nodes import IRFile
    from cpp2rust.core.mapper.type_mapper import TypeMapper

    cpp_path = ef.path
    rust_stem = Path(cpp_path).stem
    rust_path = f"src/{rust_stem}.rs"

    ir_file = IRFile(
        path=rust_path,
        cpp_source_files=[cpp_path],
        uses=[],
        items=[],
    )

    type_mapper = TypeMapper(plugin=plugin)

    for func in ef.functions:
        ir_fn = _map_function(func, type_mapper)
        if ir_fn:
            ir_file.items.append(ir_fn)

    for cls in ef.classes:
        items = _map_class(cls, type_mapper)
        ir_file.items.extend(items)

    for en in ef.enums:
        ir_enum = _map_enum(en)
        if ir_enum:
            ir_file.items.append(ir_enum)

    for td in ef.typedefs:
        ir_ta = _map_typedef(td, type_mapper)
        if ir_ta:
            ir_file.items.append(ir_ta)

    for ns in ef.namespaces:
        _process_namespace(ns, ir_file, type_mapper)

    return ir_file


def _process_namespace(ns, ir_file, type_mapper):
    for func in getattr(ns, "functions", []):
        ir_fn = _map_function(func, type_mapper)
        if ir_fn:
            ir_file.items.append(ir_fn)
    for cls in getattr(ns, "classes", []):
        items = _map_class(cls, type_mapper)
        ir_file.items.extend(items)
    for en in getattr(ns, "enums", []):
        ir_enum = _map_enum(en)
        if ir_enum:
            ir_file.items.append(ir_enum)
    for td in getattr(ns, "typedefs", []):
        ir_ta = _map_typedef(td, type_mapper)
        if ir_ta:
            ir_file.items.append(ir_ta)
    for sub_ns in getattr(ns, "namespaces", []):
        _process_namespace(sub_ns, ir_file, type_mapper)


def _map_function(func, type_mapper):
    from cpp2rust.core.ir.nodes import (
        IRFunction, IRParam, IRType, IRBlock, IRRawCppStmt,
        TypeKind, AccessSpecifier,
    )
    name = getattr(func, "name", "") or ""
    if not name:
        return None

    ret_type_str = getattr(func, "return_type", "void") or "void"
    ret_type = type_mapper.map_type(ret_type_str)

    params = []
    for p in getattr(func, "params", []) or []:
        param_name = getattr(p, "name", "") or ""
        param_type_str = getattr(p, "type", "int") or "int"
        params.append(IRParam(
            name=param_name,
            type=type_mapper.map_type(param_type_str),
        ))

    ir_body = None
    body_ir_dict = getattr(func, "body_ir", None)
    if body_ir_dict is not None:
        try:
            from cpp2rust.core.ir.nodes import ir_from_dict
            ir_body = ir_from_dict(body_ir_dict)
        except Exception:
            body_source = getattr(func, "body_source", "") or ""
            ir_body = IRBlock(statements=[IRRawCppStmt(cpp_source=body_source)])
    elif getattr(func, "has_body", False):
        body_source = getattr(func, "body_source", "") or ""
        if body_source:
            ir_body = IRBlock(statements=[IRRawCppStmt(cpp_source=body_source)])

    return IRFunction(
        name=_to_snake(name),
        params=params,
        return_type=ret_type,
        body=ir_body,
        access=AccessSpecifier.PUBLIC,
    )


def _map_class(cls, type_mapper):
    from cpp2rust.core.ir.nodes import (
        IRStruct, IRField, IRImplBlock, IRFunction, IRParam,
        IRBlock, IRRawCppStmt, AccessSpecifier,
    )
    items = []
    name = getattr(cls, "name", "") or ""
    if not name:
        return items

    fields = []
    for f in getattr(cls, "fields", []) or []:
        fname = getattr(f, "name", "") or ""
        ftype_str = getattr(f, "type", "i32") or "i32"
        fields.append(IRField(
            name=_to_snake(fname) if fname else "_field",
            type=type_mapper.map_type(ftype_str),
            access=AccessSpecifier.PRIVATE,
        ))

    struct = IRStruct(
        name=name,
        fields=fields,
        visibility=AccessSpecifier.PUBLIC,
    )
    items.append(struct)

    methods = []
    for method in getattr(cls, "methods", []) or []:
        ir_fn = _map_function(method, type_mapper)
        if ir_fn:
            methods.append(ir_fn)

    if methods:
        impl = IRImplBlock(
            struct_name=name,
            methods=methods,
        )
        items.append(impl)

    return items


def _map_enum(en):
    from cpp2rust.core.ir.nodes import IREnum, IREnumVariant, AccessSpecifier
    name = getattr(en, "name", "") or ""
    if not name:
        return None

    variants = []
    for v in getattr(en, "variants", []) or []:
        vname = getattr(v, "name", "") or ""
        vval = getattr(v, "value", None)
        if vname:
            variants.append(IREnumVariant(
                name=vname,
                value=str(vval) if vval is not None else None,
            ))

    return IREnum(name=name, variants=variants, visibility=AccessSpecifier.PUBLIC)


def _map_typedef(td, type_mapper):
    from cpp2rust.core.ir.nodes import IRTypeAlias, AccessSpecifier
    name = getattr(td, "name", "") or ""
    underlying = getattr(td, "underlying_type", "") or ""
    if not name:
        return None
    return IRTypeAlias(
        name=name,
        target=type_mapper.map_type(underlying) if underlying else None,
        visibility=AccessSpecifier.PUBLIC,
    )


def _to_snake(name: str) -> str:
    if not name:
        return name
    s1 = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s2 = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1)
    return s2.lower()
