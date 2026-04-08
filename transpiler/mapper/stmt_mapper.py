"""C++ statement/expression → Rust statement/expression mapper.

Maps IR statement and expression nodes from C++ semantics to Rust semantics.
Works on the IR level (not raw Clang cursors — that's stmt_converter's job).
"""

from __future__ import annotations

import re
from typing import List, Optional

from ir.nodes import (
    IRStmt, IRExpr, IRBlock, IRVarDecl, IRExprStmt, IRReturnStmt,
    IRIfStmt, IRMatchStmt, IRMatchArm, IRForLoop, IRForRangeLoop,
    IRWhileLoop, IRBreakStmt, IRContinueStmt, IRThrowStmt, IRTryCatchStmt,
    IRUnsafeBlock, IRRawStmt,
    IRLiteral, IRNameRef, IRBinaryExpr, IRUnaryExpr, IRCallExpr,
    IRMemberExpr, IRIndexExpr, IRCastExpr, IRTernaryExpr,
    IRNewExpr, IRDeleteExpr, IRSizeofExpr, IRLambdaExpr,
    IRThisExpr, IRInitListExpr, IRRawExpr,
    IRType, IRParam,
    BinaryOp, UnaryOp, CastKind, LiteralKind, TypeKind,
)


class StmtMapper:
    """Maps C++ IR statements/expressions to Rust-idiomatic IR."""

    def __init__(self, type_mapper=None):
        self.type_mapper = type_mapper

    # ─── Statement Mapping ───────────────────────────────────────────────

    def map_stmt(self, stmt: IRStmt) -> IRStmt:
        """Map a C++ IR statement to a Rust-idiomatic IR statement."""
        if stmt is None:
            return IRRawStmt(cpp_source="", comment="null statement")

        if isinstance(stmt, IRBlock):
            return self.map_block(stmt)
        elif isinstance(stmt, IRVarDecl):
            return self.map_var_decl(stmt)
        elif isinstance(stmt, IRReturnStmt):
            return self.map_return(stmt)
        elif isinstance(stmt, IRIfStmt):
            return self.map_if(stmt)
        elif isinstance(stmt, IRMatchStmt):
            return self.map_match(stmt)
        elif isinstance(stmt, IRForLoop):
            return self.map_for_loop(stmt)
        elif isinstance(stmt, IRForRangeLoop):
            return self.map_for_range(stmt)
        elif isinstance(stmt, IRWhileLoop):
            return self.map_while(stmt)
        elif isinstance(stmt, IRExprStmt):
            return IRExprStmt(expr=self.map_expr(stmt.expr))
        elif isinstance(stmt, IRThrowStmt):
            return self.map_throw(stmt)
        elif isinstance(stmt, IRTryCatchStmt):
            return self.map_try_catch(stmt)
        elif isinstance(stmt, (IRBreakStmt, IRContinueStmt)):
            return stmt
        else:
            return stmt

    def map_block(self, block: IRBlock) -> IRBlock:
        """Map all statements in a block."""
        return IRBlock(
            statements=[self.map_stmt(s) for s in block.statements]
        )

    def map_var_decl(self, decl: IRVarDecl) -> IRVarDecl:
        """Map a variable declaration.

        C++ const → Rust let (immutable by default).
        C++ non-const → Rust let mut.
        """
        mapped_init = self.map_expr(decl.initializer) if decl.initializer else None

        # In Rust, variables are immutable by default
        # C++ const → not mutable, C++ non-const → mutable
        is_mutable = decl.is_mutable

        # Map the type
        mapped_type = decl.type
        if self.type_mapper and decl.type:
            mapped_type = self.type_mapper.map_type(decl.type.cpp_name or decl.type.name)

        return IRVarDecl(
            name=decl.name,
            type=mapped_type,
            initializer=mapped_init,
            is_mutable=is_mutable,
            is_static=decl.is_static,
            cpp_name=decl.cpp_name,
        )

    def map_return(self, ret: IRReturnStmt) -> IRReturnStmt:
        """Map a return statement."""
        return IRReturnStmt(
            value=self.map_expr(ret.value) if ret.value else None,
        )

    def map_if(self, if_stmt: IRIfStmt) -> IRIfStmt:
        """Map an if statement. Remove unnecessary parentheses around condition."""
        return IRIfStmt(
            condition=self.map_expr(if_stmt.condition),
            then_block=self.map_block(if_stmt.then_block) if if_stmt.then_block else None,
            else_block=self.map_stmt(if_stmt.else_block) if if_stmt.else_block else None,
        )

    def map_match(self, match: IRMatchStmt) -> IRMatchStmt:
        """Map a C++ switch → Rust match."""
        mapped_arms = []
        for arm in match.arms:
            mapped_arms.append(IRMatchArm(
                patterns=arm.patterns,
                body=self.map_block(arm.body) if arm.body else None,
                is_default=arm.is_default,
            ))

        # Ensure there's a default arm
        has_default = any(a.is_default for a in mapped_arms)
        if not has_default:
            mapped_arms.append(IRMatchArm(
                patterns=["_"],
                body=IRBlock(statements=[]),
                is_default=True,
            ))

        return IRMatchStmt(
            expr=self.map_expr(match.expr),
            arms=mapped_arms,
        )

    def map_for_loop(self, for_loop: IRForLoop) -> IRStmt:
        """Map a C-style for loop to Rust.

        Simple range patterns: for (int i = 0; i < n; i++) → for i in 0..n
        Everything else: while loop with manual init/increment
        """
        # Try to detect simple range pattern
        range_loop = self._try_simplify_for_to_range(for_loop)
        if range_loop:
            return range_loop

        # Fall back to while loop pattern
        stmts = []
        if for_loop.init:
            stmts.append(self.map_stmt(for_loop.init))

        body_stmts = []
        if for_loop.body:
            mapped_body = self.map_block(for_loop.body)
            body_stmts.extend(mapped_body.statements)

        if for_loop.increment:
            body_stmts.append(IRExprStmt(expr=self.map_expr(for_loop.increment)))

        while_loop = IRWhileLoop(
            condition=self.map_expr(for_loop.condition) if for_loop.condition else IRLiteral(value="true", kind=LiteralKind.BOOL),
            body=IRBlock(statements=body_stmts),
        )
        stmts.append(while_loop)

        return IRBlock(statements=stmts)

    def _try_simplify_for_to_range(self, for_loop: IRForLoop) -> Optional[IRForRangeLoop]:
        """Try to simplify `for (int i = 0; i < n; i++)` to `for i in 0..n`."""
        # Check init: must be `let i = start`
        init = for_loop.init
        if not isinstance(init, IRVarDecl) or init.initializer is None:
            return None

        var_name = init.name

        # Check condition: must be `i < end` or `i <= end`
        cond = for_loop.condition
        if not isinstance(cond, IRBinaryExpr):
            return None
        if cond.op not in (BinaryOp.LT, BinaryOp.LE):
            return None
        if not isinstance(cond.left, IRNameRef) or cond.left.name != var_name:
            return None

        # Check increment: must be `i++` or `i += 1`
        incr = for_loop.increment
        is_simple_incr = False
        if isinstance(incr, IRUnaryExpr) and incr.op in (UnaryOp.POST_INC, UnaryOp.PRE_INC):
            if isinstance(incr.operand, IRNameRef) and incr.operand.name == var_name:
                is_simple_incr = True
        elif isinstance(incr, IRBinaryExpr) and incr.op == BinaryOp.ADD_ASSIGN:
            if isinstance(incr.left, IRNameRef) and incr.left.name == var_name:
                if isinstance(incr.right, IRLiteral) and incr.right.value == "1":
                    is_simple_incr = True

        if not is_simple_incr:
            return None

        # Build range expression: start..end or start..=end
        start = self.map_expr(init.initializer)
        end = self.map_expr(cond.right)

        if cond.op == BinaryOp.LE:
            # i <= n → i in start..=end
            range_expr = IRBinaryExpr(
                op=BinaryOp.ADD,  # We'll emit this as ..= in the emitter
                left=start,
                right=end,
            )
            # Actually, represent as raw for now
            range_expr = IRRawExpr(cpp_source=f"(range_inclusive)")
        else:
            range_expr = IRRawExpr(cpp_source=f"(range)")

        return IRForRangeLoop(
            variable=var_name,
            var_type=init.type,
            iterable=end,  # Will be emitted as `0..end` by the emitter
            body=self.map_block(for_loop.body) if for_loop.body else IRBlock(),
        )

    def map_for_range(self, for_range: IRForRangeLoop) -> IRForRangeLoop:
        """Map a C++ range-for to Rust for-in."""
        return IRForRangeLoop(
            variable=for_range.variable,
            var_type=for_range.var_type,
            iterable=self.map_expr(for_range.iterable),
            body=self.map_block(for_range.body) if for_range.body else IRBlock(),
            is_ref=for_range.is_ref,
            is_mut_ref=for_range.is_mut_ref,
        )

    def map_while(self, while_loop: IRWhileLoop) -> IRStmt:
        """Map while/do-while to Rust."""
        if while_loop.is_do_while:
            # do { body } while (cond) → loop { body; if !cond { break; } }
            body_stmts = []
            if while_loop.body:
                mapped = self.map_block(while_loop.body)
                body_stmts.extend(mapped.statements)

            # Add break condition
            break_check = IRIfStmt(
                condition=IRUnaryExpr(
                    op=UnaryOp.NOT,
                    operand=self.map_expr(while_loop.condition),
                ),
                then_block=IRBlock(statements=[IRBreakStmt()]),
            )
            body_stmts.append(break_check)

            return IRWhileLoop(
                condition=IRLiteral(value="true", kind=LiteralKind.BOOL),
                body=IRBlock(statements=body_stmts),
            )

        return IRWhileLoop(
            condition=self.map_expr(while_loop.condition),
            body=self.map_block(while_loop.body) if while_loop.body else IRBlock(),
        )

    def map_throw(self, throw: IRThrowStmt) -> IRReturnStmt:
        """Map C++ throw to Rust return Err(...)."""
        if throw.expr:
            return IRReturnStmt(
                value=IRCallExpr(
                    function=IRNameRef(name="Err"),
                    args=[self.map_expr(throw.expr)],
                ),
            )
        # Re-throw → return Err(current_error)
        return IRReturnStmt(
            value=IRCallExpr(
                function=IRNameRef(name="Err"),
                args=[IRRawExpr(cpp_source="/* re-throw */")],
            ),
        )

    def map_try_catch(self, try_catch: IRTryCatchStmt) -> IRStmt:
        """Map C++ try/catch to Rust. This is a rough mapping since Rust
        uses Result<T, E> instead of exceptions."""
        # For now, emit the try block as-is with a comment
        stmts = []
        stmts.append(IRRawStmt(
            cpp_source="// TODO: Convert try/catch to Result-based error handling",
        ))
        if try_catch.try_block:
            mapped = self.map_block(try_catch.try_block)
            stmts.extend(mapped.statements)
        return IRBlock(statements=stmts)

    # ─── Expression Mapping ──────────────────────────────────────────────

    def map_expr(self, expr: IRExpr) -> IRExpr:
        """Map a C++ IR expression to a Rust-idiomatic IR expression."""
        if expr is None:
            return IRRawExpr(cpp_source="/* null expr */")

        if isinstance(expr, IRLiteral):
            return self.map_literal(expr)
        elif isinstance(expr, IRNameRef):
            return self.map_name_ref(expr)
        elif isinstance(expr, IRBinaryExpr):
            return self.map_binary(expr)
        elif isinstance(expr, IRUnaryExpr):
            return self.map_unary(expr)
        elif isinstance(expr, IRCallExpr):
            return self.map_call(expr)
        elif isinstance(expr, IRMemberExpr):
            return self.map_member(expr)
        elif isinstance(expr, IRIndexExpr):
            return self.map_index(expr)
        elif isinstance(expr, IRCastExpr):
            return self.map_cast(expr)
        elif isinstance(expr, IRTernaryExpr):
            return self.map_ternary(expr)
        elif isinstance(expr, IRNewExpr):
            return self.map_new(expr)
        elif isinstance(expr, IRDeleteExpr):
            return self.map_delete(expr)
        elif isinstance(expr, IRSizeofExpr):
            return self.map_sizeof(expr)
        elif isinstance(expr, IRLambdaExpr):
            return self.map_lambda(expr)
        elif isinstance(expr, IRThisExpr):
            return IRNameRef(name="self")
        elif isinstance(expr, IRInitListExpr):
            return self.map_init_list(expr)
        else:
            return expr

    def map_literal(self, lit: IRLiteral) -> IRLiteral:
        """Map C++ literals to Rust literals."""
        if lit.kind == LiteralKind.NULL:
            return IRNameRef(name="None")  # Will need Option context
        if lit.kind == LiteralKind.STRING:
            # C++ "str" → Rust "str" (same syntax)
            return lit
        if lit.kind == LiteralKind.CHAR:
            # C++ 'c' → Rust 'c' (same syntax)
            return lit
        if lit.kind == LiteralKind.BOOL:
            return lit
        if lit.kind == LiteralKind.INT:
            # Remove C++ suffixes: 42u, 42L, 42ULL, etc.
            value = lit.value.rstrip("uUlL")
            return IRLiteral(value=value, kind=LiteralKind.INT)
        if lit.kind == LiteralKind.FLOAT:
            # Remove C++ f suffix: 3.14f → 3.14
            value = lit.value.rstrip("fF")
            return IRLiteral(value=value, kind=LiteralKind.FLOAT)
        return lit

    def map_name_ref(self, ref: IRNameRef) -> IRExpr:
        """Map name references. Handle nullptr, this, etc."""
        name = ref.name
        # nullptr → None
        if name == "nullptr":
            return IRNameRef(name="None")
        # this → self
        if name == "this":
            return IRNameRef(name="self")
        # true/false are the same
        if name in ("true", "false"):
            return IRLiteral(value=name, kind=LiteralKind.BOOL)
        # Convert to snake_case if it looks like a variable
        if name and name[0].islower() and not "::" in name:
            name = self._to_snake_case(name)
        return IRNameRef(name=name, qualified_name=ref.qualified_name)

    def map_binary(self, expr: IRBinaryExpr) -> IRExpr:
        """Map binary expressions. Most operators are the same in Rust."""
        left = self.map_expr(expr.left)
        right = self.map_expr(expr.right)

        # C++ comma operator → block with both expressions
        if expr.op == BinaryOp.COMMA:
            return IRRawExpr(
                cpp_source="/* comma operator */",
                comment="TODO: convert comma operator to block",
            )

        return IRBinaryExpr(op=expr.op, left=left, right=right)

    def map_unary(self, expr: IRUnaryExpr) -> IRExpr:
        """Map unary expressions."""
        operand = self.map_expr(expr.operand)

        # Post-increment/decrement need special handling in Rust
        if expr.op == UnaryOp.POST_INC:
            # x++ → { let tmp = x; x += 1; tmp } or just x += 1 as stmt
            return IRBinaryExpr(
                op=BinaryOp.ADD_ASSIGN,
                left=operand,
                right=IRLiteral(value="1", kind=LiteralKind.INT),
            )
        elif expr.op == UnaryOp.POST_DEC:
            return IRBinaryExpr(
                op=BinaryOp.SUB_ASSIGN,
                left=operand,
                right=IRLiteral(value="1", kind=LiteralKind.INT),
            )
        elif expr.op == UnaryOp.PRE_INC:
            return IRBinaryExpr(
                op=BinaryOp.ADD_ASSIGN,
                left=operand,
                right=IRLiteral(value="1", kind=LiteralKind.INT),
            )
        elif expr.op == UnaryOp.PRE_DEC:
            return IRBinaryExpr(
                op=BinaryOp.SUB_ASSIGN,
                left=operand,
                right=IRLiteral(value="1", kind=LiteralKind.INT),
            )
        elif expr.op == UnaryOp.BITNOT:
            # C++ ~ → Rust ! (for integer types)
            return IRUnaryExpr(op=UnaryOp.NOT, operand=operand)

        return IRUnaryExpr(op=expr.op, operand=operand)

    def map_call(self, call: IRCallExpr) -> IRExpr:
        """Map function/method calls."""
        mapped_func = self.map_expr(call.function)
        mapped_args = [self.map_expr(a) for a in call.args]

        # Check for special function names
        if isinstance(mapped_func, IRNameRef):
            name = mapped_func.name
            # std::move(x) → just x
            if name in ("std::move", "std::forward"):
                if mapped_args:
                    return mapped_args[0]
            # std::make_unique<T>(args) → Box::new(T::new(args))
            if name == "std::make_unique":
                return IRCallExpr(
                    function=IRNameRef(name="Box::new"),
                    args=mapped_args,
                )
            if name == "std::make_shared":
                return IRCallExpr(
                    function=IRNameRef(name="Arc::new"),
                    args=mapped_args,
                )

        return IRCallExpr(
            function=mapped_func,
            args=mapped_args,
            is_method_call=call.is_method_call,
            generic_args=call.generic_args,
        )

    def map_member(self, expr: IRMemberExpr) -> IRMemberExpr:
        """Map member access. C++ -> becomes . in Rust."""
        return IRMemberExpr(
            object=self.map_expr(expr.object),
            member=self._to_snake_case(expr.member),
            is_arrow=False,  # Always . in Rust (auto-deref)
        )

    def map_index(self, expr: IRIndexExpr) -> IRIndexExpr:
        """Map array subscript."""
        return IRIndexExpr(
            object=self.map_expr(expr.object),
            index=self.map_expr(expr.index),
        )

    def map_cast(self, cast: IRCastExpr) -> IRExpr:
        """Map C++ casts to Rust casts."""
        mapped_expr = self.map_expr(cast.expr)
        mapped_type = cast.target_type

        if cast.kind == CastKind.AS:
            # static_cast → `as` for primitives
            return IRCastExpr(expr=mapped_expr, target_type=mapped_type, kind=CastKind.AS)
        elif cast.kind == CastKind.INTO:
            # dynamic_cast → downcast pattern
            return IRCastExpr(expr=mapped_expr, target_type=mapped_type, kind=CastKind.INTO)
        elif cast.kind == CastKind.UNSAFE_TRANSMUTE:
            # reinterpret_cast → unsafe transmute
            return IRCastExpr(expr=mapped_expr, target_type=mapped_type, kind=CastKind.UNSAFE_TRANSMUTE)
        elif cast.kind == CastKind.UNSAFE_PTR_CAST:
            # const_cast → unsafe ptr cast
            return IRCastExpr(expr=mapped_expr, target_type=mapped_type, kind=CastKind.UNSAFE_PTR_CAST)

        return IRCastExpr(expr=mapped_expr, target_type=mapped_type, kind=cast.kind)

    def map_ternary(self, expr: IRTernaryExpr) -> IRExpr:
        """Map C++ ternary to Rust if expression."""
        return IRTernaryExpr(
            condition=self.map_expr(expr.condition),
            then_expr=self.map_expr(expr.then_expr),
            else_expr=self.map_expr(expr.else_expr),
        )

    def map_new(self, new: IRNewExpr) -> IRExpr:
        """Map C++ new T(args) to Rust Box::new(T::new(args))."""
        if new.is_array:
            # new T[n] → vec![T::default(); n]
            return IRCallExpr(
                function=IRNameRef(name="vec!"),
                args=[
                    IRCallExpr(
                        function=IRMemberExpr(
                            object=IRNameRef(name=new.type.name if new.type else "T"),
                            member="default",
                        ),
                        args=[],
                    ),
                    self.map_expr(new.array_size) if new.array_size else IRLiteral(value="0", kind=LiteralKind.INT),
                ],
            )

        # new T(args) → Box::new(T::new(args))
        inner_args = [self.map_expr(a) for a in new.args]
        type_name = new.type.name if new.type else "T"
        return IRCallExpr(
            function=IRNameRef(name="Box::new"),
            args=[
                IRCallExpr(
                    function=IRNameRef(name=f"{type_name}::new"),
                    args=inner_args,
                ),
            ],
        )

    def map_delete(self, delete: IRDeleteExpr) -> IRExpr:
        """Map C++ delete → Rust drop (usually just let it go out of scope)."""
        return IRCallExpr(
            function=IRNameRef(name="drop"),
            args=[self.map_expr(delete.expr)],
        )

    def map_sizeof(self, sizeof: IRSizeofExpr) -> IRExpr:
        """Map sizeof/alignof to Rust std::mem functions."""
        if sizeof.is_alignof:
            fn_name = "std::mem::align_of"
        else:
            fn_name = "std::mem::size_of"

        if sizeof.target_type:
            return IRCallExpr(
                function=IRNameRef(name=f"{fn_name}::<{sizeof.target_type.name}>"),
                args=[],
            )
        elif sizeof.target_expr:
            return IRCallExpr(
                function=IRNameRef(name=f"std::mem::size_of_val"),
                args=[IRUnaryExpr(op=UnaryOp.ADDR_OF, operand=self.map_expr(sizeof.target_expr))],
            )
        return IRRawExpr(cpp_source="sizeof(?)")

    def map_lambda(self, lam: IRLambdaExpr) -> IRLambdaExpr:
        """Map C++ lambda to Rust closure."""
        mapped_body = self.map_block(lam.body) if lam.body else IRBlock()

        # C++ [=] capture → Rust move
        # C++ [&] capture → Rust borrow (default)
        capture = ""
        if lam.capture_mode == "value" or lam.capture_mode == "=":
            capture = "move"

        return IRLambdaExpr(
            params=lam.params,
            body=mapped_body,
            capture_mode=capture,
            return_type=lam.return_type,
        )

    def map_init_list(self, init: IRInitListExpr) -> IRExpr:
        """Map aggregate initialization {a, b, c}."""
        mapped = [self.map_expr(e) for e in init.elements]
        return IRInitListExpr(elements=mapped)

    # ─── Helpers ─────────────────────────────────────────────────────────

    @staticmethod
    def _to_snake_case(name: str) -> str:
        if not name:
            return name
        s1 = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
        s2 = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s1)
        return s2.lower().strip("_")
