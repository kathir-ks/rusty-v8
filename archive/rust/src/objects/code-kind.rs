// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/objects/code-kind.h

use std::fmt;
use std::ops::{BitAnd, BitOr, BitOrAssign};
use std::usize;

// Placeholder for base::Bounds as it does not have a direct equivalent in Rust's standard library
// You might need to use a custom implementation or a crate like 'num-traits' or similar
// to represent numeric bounds if the functionality is needed.
mod base {
    pub fn is_in_range<T: PartialOrd>(value: T, lower: T, upper: T) -> bool {
        value >= lower && value <= upper
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Flags<T> {
        bits: u32,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Flags<T> {
        pub const fn empty() -> Self {
            Flags { bits: 0, _phantom: std::marker::PhantomData }
        }

        pub fn contains(&self, other: Flags<T>) -> bool {
            (self.bits & other.bits) == other.bits
        }

        pub fn insert(&mut self, other: Flags<T>) {
            self.bits |= other.bits;
        }
    }

    impl<T> BitOr for Flags<T> {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            Flags { bits: self.bits | other.bits, _phantom: std::marker::PhantomData }
        }
    }

    impl<T> BitAnd for Flags<T> {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            Flags { bits: self.bits & other.bits, _phantom: std::marker::PhantomData }
        }
    }

    impl<T> BitOrAssign for Flags<T> {
        fn bitor_assign(&mut self, other: Self) {
            self.bits |= other.bits;
        }
    }

    macro_rules! define_operators_for_flags {
        ($flags_type:ident) => {
            impl std::ops::BitOr for $flags_type {
                type Output = Self;

                fn bitor(self, other: Self) -> Self {
                    $flags_type {
                        bits: self.bits | other.bits,
                        _phantom: std::marker::PhantomData,
                    }
                }
            }

            impl std::ops::BitAnd for $flags_type {
                type Output = Self;

                fn bitand(self, other: Self) -> Self {
                    $flags_type {
                        bits: self.bits & other.bits,
                        _phantom: std::marker::PhantomData,
                    }
                }
            }

            impl std::ops::BitOrAssign for $flags_type {
                fn bitor_assign(&mut self, other: Self) {
                    self.bits |= other.bits;
                }
            }
        };
    }
    pub(crate) use define_operators_for_flags;
}

// Placeholder for flags as this is a separate module in V8
// and requires more context to translate fully.
mod flags {
    pub struct Flags {
        pub wasm_deopt: bool,
    }

    impl Flags {
        pub fn new() -> Self {
            Flags { wasm_deopt: false }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
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
    assert!(
        CodeKind::INTERPRETED_FUNCTION < CodeKind::BASELINE,
        "CodeKind::INTERPRETED_FUNCTION should be less than CodeKind::BASELINE"
    );
    assert!(
        CodeKind::BASELINE < CodeKind::TURBOFAN_JS,
        "CodeKind::BASELINE should be less than CodeKind::TURBOFAN_JS"
    );
};

const K_CODE_KIND_COUNT: usize = 13;
const _: () = {
    assert!(
        K_CODE_KIND_COUNT <= std::u8::MAX as usize,
        "kCodeKindCount should be less than or equal to the maximum value of u8"
    );
};

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
        CodeKind::BYTECODE_HANDLER => "bytecode_handler",
        CodeKind::FOR_TESTING => "for_testing",
        CodeKind::BUILTIN => "builtin",
        CodeKind::REGEXP => "regexp",
        CodeKind::WASM_FUNCTION => "wasm_function",
        CodeKind::WASM_TO_CAPI_FUNCTION => "wasm_to_capi_function",
        CodeKind::WASM_TO_JS_FUNCTION => "wasm_to_js_function",
        CodeKind::JS_TO_WASM_FUNCTION => "js_to_wasm_function",
        CodeKind::C_WASM_ENTRY => "c_wasm_entry",
        CodeKind::INTERPRETED_FUNCTION => "interpreted_function",
        CodeKind::BASELINE => "baseline",
        CodeKind::MAGLEV => "maglev",
        CodeKind::TURBOFAN_JS => {
            if context_specialized {
                "turbofan_js_context_specialized"
            } else {
                "turbofan_js"
            }
        }
    }
}

pub const fn code_kind_is_interpreted_js_function(kind: CodeKind) -> bool {
    kind == CodeKind::INTERPRETED_FUNCTION
}

pub const fn code_kind_is_baselined_js_function(kind: CodeKind) -> bool {
    kind == CodeKind::BASELINE
}

pub const fn code_kind_is_unoptimized_js_function(kind: CodeKind) -> bool {
    const _: () = {
        assert!(
            CodeKind::INTERPRETED_FUNCTION as i32 + 1 == CodeKind::BASELINE as i32,
            "CodeKind::INTERPRETED_FUNCTION + 1 must equal CodeKind::BASELINE"
        );
    };
    base::is_in_range(kind, CodeKind::INTERPRETED_FUNCTION, CodeKind::BASELINE)
}

pub const fn code_kind_is_optimized_js_function(kind: CodeKind) -> bool {
    const _: () = {
        assert!(
            CodeKind::MAGLEV as i32 + 1 == CodeKind::TURBOFAN_JS as i32,
            "CodeKind::MAGLEV + 1 must equal CodeKind::TURBOFAN_JS"
        );
    };
    base::is_in_range(kind, CodeKind::MAGLEV, CodeKind::TURBOFAN_JS)
}

pub const fn code_kind_is_js_function(kind: CodeKind) -> bool {
    const _: () = {
        assert!(
            CodeKind::BASELINE as i32 + 1 == CodeKind::MAGLEV as i32,
            "CodeKind::BASELINE + 1 must equal CodeKind::MAGLEV"
        );
    };
    base::is_in_range(kind, CodeKind::INTERPRETED_FUNCTION, CodeKind::TURBOFAN_JS)
}

pub const fn code_kind_is_builtin_or_js_function(kind: CodeKind) -> bool {
    kind == CodeKind::BUILTIN || code_kind_is_js_function(kind)
}

pub fn code_kind_can_deoptimize(kind: CodeKind, v8_flags: &flags::Flags) -> bool {
    code_kind_is_optimized_js_function(kind)
        || (kind == CodeKind::WASM_FUNCTION && v8_flags.wasm_deopt)
}

pub const fn code_kind_can_osr(kind: CodeKind) -> bool {
    kind == CodeKind::TURBOFAN_JS || kind == CodeKind::MAGLEV
}

pub const fn code_kind_can_tier_up(kind: CodeKind) -> bool {
    code_kind_is_unoptimized_js_function(kind) || kind == CodeKind::MAGLEV
}

// TODO(jgruber): Rename or remove this predicate. Currently it means 'is this
// kind stored either in the FeedbackVector cache, or in the OSR cache?'.
pub const fn code_kind_is_stored_in_optimized_code_cache(kind: CodeKind) -> bool {
    kind == CodeKind::MAGLEV || kind == CodeKind::TURBOFAN_JS
}

pub const fn code_kind_uses_bytecode_or_interpreter_data(kind: CodeKind) -> bool {
    code_kind_is_baselined_js_function(kind)
}

pub const fn code_kind_uses_deoptimization_data(kind: CodeKind) -> bool {
    // Need a flag to pass for wasm deopt feature, it is not included in this header.
    false
}

pub const fn code_kind_uses_bytecode_offset_table(kind: CodeKind) -> bool {
    kind == CodeKind::BASELINE
}

pub const fn code_kind_may_lack_source_position_table(kind: CodeKind) -> bool {
    // Either code that uses a bytecode offset table or code that may be embedded
    // in the snapshot, in which case the source position table is cleared.
    code_kind_uses_bytecode_offset_table(kind)
        || kind == CodeKind::BUILTIN
        || kind == CodeKind::BYTECODE_HANDLER
        || kind == CodeKind::FOR_TESTING
}

pub const fn code_kind_for_top_tier() -> CodeKind {
    CodeKind::TURBOFAN_JS
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CodeKindFlag {
    BYTECODE_HANDLER = 1 << CodeKind::BYTECODE_HANDLER as u32,
    FOR_TESTING = 1 << CodeKind::FOR_TESTING as u32,
    BUILTIN = 1 << CodeKind::BUILTIN as u32,
    REGEXP = 1 << CodeKind::REGEXP as u32,
    WASM_FUNCTION = 1 << CodeKind::WASM_FUNCTION as u32,
    WASM_TO_CAPI_FUNCTION = 1 << CodeKind::WASM_TO_CAPI_FUNCTION as u32,
    WASM_TO_JS_FUNCTION = 1 << CodeKind::WASM_TO_JS_FUNCTION as u32,
    JS_TO_WASM_FUNCTION = 1 << CodeKind::JS_TO_WASM_FUNCTION as u32,
    C_WASM_ENTRY = 1 << CodeKind::C_WASM_ENTRY as u32,
    INTERPRETED_FUNCTION = 1 << CodeKind::INTERPRETED_FUNCTION as u32,
    BASELINE = 1 << CodeKind::BASELINE as u32,
    MAGLEV = 1 << CodeKind::MAGLEV as u32,
    TURBOFAN_JS = 1 << CodeKind::TURBOFAN_JS as u32,
}

const _: () = {
    assert!(
        K_CODE_KIND_COUNT <= (32 /* kInt32Size */ * 8 /* kBitsPerByte */),
        "kCodeKindCount should be less than or equal to kInt32Size * kBitsPerByte"
    );
};

pub const fn code_kind_to_code_kind_flag(kind: CodeKind) -> CodeKindFlag {
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

pub type CodeKinds = base::Flags<CodeKindFlag>;

pub const K_JS_FUNCTION_CODE_KINDS_MASK: CodeKinds = CodeKinds {
    bits: CodeKindFlag::INTERPRETED_FUNCTION as u32
        | CodeKindFlag::BASELINE as u32
        | CodeKindFlag::MAGLEV as u32
        | CodeKindFlag::TURBOFAN_JS as u32,
    _phantom: std::marker::PhantomData,
};

pub const K_OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK: CodeKinds = CodeKinds {
    bits: CodeKindFlag::MAGLEV as u32 | CodeKindFlag::TURBOFAN_JS as u32,
    _phantom: std::marker::PhantomData,
};