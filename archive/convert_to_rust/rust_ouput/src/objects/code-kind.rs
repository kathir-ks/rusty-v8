// Converted from V8 C++ source files:
// Header: code-kind.h
// Implementation: code-kind.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod code_kind {
    use std::{
        fmt,
        ops::{BitAnd, BitOr, BitXor, Not},
    };

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum CodeKind {
        BYTECODE_HANDLER,
        FOR_TESTING,
        BUILTIN,
        REGEXP,
        WASM_FUNCTION,
        WASM_TO_CAPI_FUNCTION,
        WASM_TO_JS_FUNCTION,
        JS_TO_WASM_FUNCTION,
        C_WASM_ENTRY,
        INTERPRETED_FUNCTION,
        BASELINE,
        MAGLEV,
        TURBOFAN_JS,
    }

    const _: () = {
        assert!(CodeKind::INTERPRETED_FUNCTION as i32 < CodeKind::BASELINE as i32);
        assert!(CodeKind::BASELINE as i32 < CodeKind::TURBOFAN_JS as i32);
    };

    pub const K_CODE_KIND_COUNT: usize = 13;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum CodeKindFlag {
        BYTECODE_HANDLER = 1 << CodeKind::BYTECODE_HANDLER as i32,
        FOR_TESTING = 1 << CodeKind::FOR_TESTING as i32,
        BUILTIN = 1 << CodeKind::BUILTIN as i32,
        REGEXP = 1 << CodeKind::REGEXP as i32,
        WASM_FUNCTION = 1 << CodeKind::WASM_FUNCTION as i32,
        WASM_TO_CAPI_FUNCTION = 1 << CodeKind::WASM_TO_CAPI_FUNCTION as i32,
        WASM_TO_JS_FUNCTION = 1 << CodeKind::WASM_TO_JS_FUNCTION as i32,
        JS_TO_WASM_FUNCTION = 1 << CodeKind::JS_TO_WASM_FUNCTION as i32,
        C_WASM_ENTRY = 1 << CodeKind::C_WASM_ENTRY as i32,
        INTERPRETED_FUNCTION = 1 << CodeKind::INTERPRETED_FUNCTION as i32,
        BASELINE = 1 << CodeKind::BASELINE as i32,
        MAGLEV = 1 << CodeKind::MAGLEV as i32,
        TURBOFAN_JS = 1 << CodeKind::TURBOFAN_JS as i32,
    }

    pub fn code_kind_to_code_kind_flag(kind: CodeKind) -> CodeKindFlag {
        match kind {
            CodeKind::BYTECODE_HANDLER => CodeKindFlag::BYTECODE_HANDLER,
            CodeKind::FOR_TESTING => CodeKindFlag::FOR_TESTING,
            CodeKind::BUILTIN => CodeKindFlag::BUILTIN,
            CodeKind::REGEXP => CodeKindFlag::REGEXP,
            CodeKind::WASM_FUNCTION => CodeKindFlag::WASM_FUNCTION,
            CodeKind::WASM_TO_CAPI_FUNCTION => CodeKindFlag::WASM_TO_CAPI_FUNCTION,
            CodeKind::WASM_TO_JS_FUNCTION => CodeKindFlag::WASM_TO_JS_FUNCTION,
            CodeKind::JS_TO_WASM_FUNCTION => CodeKindFlag::JS_TO_WASM_FUNCTION,
            CodeKind::C_WASM_ENTRY => CodeKindFlag::C_WASM_ENTRY,
            CodeKind::INTERPRETED_FUNCTION => CodeKindFlag::INTERPRETED_FUNCTION,
            CodeKind::BASELINE => CodeKindFlag::BASELINE,
            CodeKind::MAGLEV => CodeKindFlag::MAGLEV,
            CodeKind::TURBOFAN_JS => CodeKindFlag::TURBOFAN_JS,
        }
    }

    bitflags::bitflags! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct CodeKinds: u32 {
            const INTERPRETED_FUNCTION = CodeKindFlag::INTERPRETED_FUNCTION as u32;
            const BASELINE = CodeKindFlag::BASELINE as u32;
            const MAGLEV = CodeKindFlag::MAGLEV as u32;
            const TURBOFAN_JS = CodeKindFlag::TURBOFAN_JS as u32;
        }
    }

    impl fmt::Display for CodeKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", code_kind_to_string(*self))
        }
    }

    pub fn code_kind_to_string(kind: CodeKind) -> &'static str {
        match kind {
            CodeKind::BYTECODE_HANDLER => "BYTECODE_HANDLER",
            CodeKind::FOR_TESTING => "FOR_TESTING",
            CodeKind::BUILTIN => "BUILTIN",
            CodeKind::REGEXP => "REGEXP",
            CodeKind::WASM_FUNCTION => "WASM_FUNCTION",
            CodeKind::WASM_TO_CAPI_FUNCTION => "WASM_TO_CAPI_FUNCTION",
            CodeKind::WASM_TO_JS_FUNCTION => "WASM_TO_JS_FUNCTION",
            CodeKind::JS_TO_WASM_FUNCTION => "JS_TO_WASM_FUNCTION",
            CodeKind::C_WASM_ENTRY => "C_WASM_ENTRY",
            CodeKind::INTERPRETED_FUNCTION => "INTERPRETED_FUNCTION",
            CodeKind::BASELINE => "BASELINE",
            CodeKind::MAGLEV => "MAGLEV",
            CodeKind::TURBOFAN_JS => "TURBOFAN_JS",
        }
    }

    pub fn code_kind_to_marker(kind: CodeKind, context_specialized: bool) -> &'static str {
        match kind {
            CodeKind::INTERPRETED_FUNCTION => "~",
            CodeKind::BASELINE => "^",
            CodeKind::MAGLEV => {
                if context_specialized {
                    "+'"
                } else {
                    "+"
                }
            }
            CodeKind::TURBOFAN_JS => {
                if context_specialized {
                    "*'"
                } else {
                    "*"
                }
            }
            _ => "",
        }
    }

    pub const fn code_kind_is_interpreted_js_function(kind: CodeKind) -> bool {
        kind == CodeKind::INTERPRETED_FUNCTION
    }

    pub const fn code_kind_is_baselined_js_function(kind: CodeKind) -> bool {
        kind == CodeKind::BASELINE
    }

    pub const fn code_kind_is_unoptimized_js_function(kind: CodeKind) -> bool {
        kind as i32 >= CodeKind::INTERPRETED_FUNCTION as i32 && kind as i32 <= CodeKind::BASELINE as i32
    }

    pub const fn code_kind_is_optimized_js_function(kind: CodeKind) -> bool {
        kind as i32 >= CodeKind::MAGLEV as i32 && kind as i32 <= CodeKind::TURBOFAN_JS as i32
    }

    pub const fn code_kind_is_js_function(kind: CodeKind) -> bool {
        kind as i32 >= CodeKind::INTERPRETED_FUNCTION as i32 && kind as i32 <= CodeKind::TURBOFAN_JS as i32
    }

    pub const fn code_kind_is_builtin_or_js_function(kind: CodeKind) -> bool {
        kind == CodeKind::BUILTIN || code_kind_is_js_function(kind)
    }

    pub const fn code_kind_can_deoptimize(kind: CodeKind) -> bool {
        code_kind_is_optimized_js_function(kind)
            //|| (kind == CodeKind::WASM_FUNCTION && v8_flags.wasm_deopt)
    }

    pub const fn code_kind_can_osr(kind: CodeKind) -> bool {
        kind == CodeKind::TURBOFAN_JS || kind == CodeKind::MAGLEV
    }

    pub const fn code_kind_can_tier_up(kind: CodeKind) -> bool {
        code_kind_is_unoptimized_js_function(kind) || kind == CodeKind::MAGLEV
    }

    pub const fn code_kind_is_stored_in_optimized_code_cache(kind: CodeKind) -> bool {
        kind == CodeKind::MAGLEV || kind == CodeKind::TURBOFAN_JS
    }

    pub const fn code_kind_uses_bytecode_or_interpreter_data(kind: CodeKind) -> bool {
        code_kind_is_baselined_js_function(kind)
    }

    pub const fn code_kind_uses_deoptimization_data(kind: CodeKind) -> bool {
        code_kind_can_deoptimize(kind)
    }

    pub const fn code_kind_uses_bytecode_offset_table(kind: CodeKind) -> bool {
        kind == CodeKind::BASELINE
    }

    pub const fn code_kind_may_lack_source_position_table(kind: CodeKind) -> bool {
        code_kind_uses_bytecode_offset_table(kind)
            || kind == CodeKind::BUILTIN
            || kind == CodeKind::BYTECODE_HANDLER
            || kind == CodeKind::FOR_TESTING
    }

    pub fn code_kind_for_top_tier() -> CodeKind {
        CodeKind::TURBOFAN_JS
    }

    pub const K_JS_FUNCTION_CODE_KINDS_MASK: CodeKinds = CodeKinds::INTERPRETED_FUNCTION.union(CodeKinds::BASELINE).union(CodeKinds::MAGLEV).union(CodeKinds::TURBOFAN_JS);
    pub const K_OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK: CodeKinds = CodeKinds::MAGLEV.union(CodeKinds::TURBOFAN_JS);
}
