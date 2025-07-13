// Converted from V8 C++ source files:
// Header: keywords-gen.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

mod token;

pub mod base {
    pub fn is_in_range<T: PartialOrd>(value: T, lower: T, upper: T) -> bool {
        value >= lower && value <= upper
    }
}

pub mod internal {
    use crate::token::Token;

    #[derive(Debug, Copy, Clone)]
    pub struct PerfectKeywordHashTableEntry {
        pub name: &'static str,
        pub value: Token,
    }

    #[allow(dead_code)]
    pub enum TotalKeywords {
        Total = 52,
    }

    const MIN_WORD_LENGTH: usize = 2;
    const MAX_WORD_LENGTH: usize = 10;
    const MIN_HASH_VALUE: usize = 3;
    const MAX_HASH_VALUE: usize = 64;

    pub struct PerfectKeywordHash {}

    impl PerfectKeywordHash {
        #[inline]
        fn hash(str: &str, len: usize) -> u32 {
            let str_bytes = str.as_bytes();
            assert!(str_bytes.len() >= 2);
            assert!(str_bytes[1].wrapping_add(1) < 129);
            assert!(str_bytes[0] < 129);

            const ASSO_VALUES: [u8; 129] = [
                65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
                65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
                65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
                65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
                65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
                65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 33, 0, 24, 18, 17,
                0, 31, 65, 15, 33, 65, 0, 25, 24, 14, 1, 65, 0, 10, 3, 36, 4,
                23, 26, 13, 1, 65, 65, 65, 65, 65, 65,
            ];

            (len as u32).wrapping_add(ASSO_VALUES[str_bytes[1].wrapping_add(1) as usize] as u32)
                .wrapping_add(ASSO_VALUES[str_bytes[0] as usize] as u32)
        }

        #[inline]
        pub fn get_token(str: &str, len: usize) -> Token {
            if base::is_in_range(len, MIN_WORD_LENGTH, MAX_WORD_LENGTH) {
                let key = (Self::hash(str, len) & 0x7f) as usize;

                const PERFECT_KEYWORD_LENGTH_TABLE: [u8; 128] = [
                    0, 0, 0, 3, 3, 5, 6, 3, 7, 4, 6, 6, 8, 3, 0, 5, 3, 4, 7, 5, 9, 2,
                    4, 5, 6, 7, 8, 3, 4, 5, 5, 2, 4, 8, 3, 4, 6, 7, 9, 10, 7, 5, 6, 5,
                    5, 6, 4, 2, 2, 10, 0, 5, 6, 0, 5, 0, 0, 0, 0, 8, 4, 0, 0, 0, 5, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];

                const PERFECT_KEYWORD_HASH_TABLE: [PerfectKeywordHashTableEntry; 128] = [
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "let",
                        value: Token::kLet,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "for",
                        value: Token::kFor,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "false",
                        value: Token::kFalseLiteral,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "return",
                        value: Token::kReturn,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "var",
                        value: Token::kVar,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "package",
                        value: Token::kFutureStrictReservedWord,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "void",
                        value: Token::kVoid,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "typeof",
                        value: Token::kTypeOf,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "public",
                        value: Token::kFutureStrictReservedWord,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "function",
                        value: Token::kFunction,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "set",
                        value: Token::kSet,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "break",
                        value: Token::kBreak,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "try",
                        value: Token::kTry,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "true",
                        value: Token::kTrueLiteral,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "private",
                        value: Token::kFutureStrictReservedWord,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "super",
                        value: Token::kSuper,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "protected",
                        value: Token::kFutureStrictReservedWord,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "do",
                        value: Token::kDo,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "this",
                        value: Token::kThis,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "throw",
                        value: Token::kThrow,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "delete",
                        value: Token::kDelete,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "default",
                        value: Token::kDefault,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "debugger",
                        value: Token::kDebugger,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "new",
                        value: Token::kNew,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "case",
                        value: Token::kCase,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "catch",
                        value: Token::kCatch,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "const",
                        value: Token::kConst,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "in",
                        value: Token::kIn,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "null",
                        value: Token::kNullLiteral,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "continue",
                        value: Token::kContinue,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "get",
                        value: Token::kGet,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "enum",
                        value: Token::kEnum,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "export",
                        value: Token::kExport,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "extends",
                        value: Token::kExtends,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "interface",
                        value: Token::kFutureStrictReservedWord,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "instanceof",
                        value: Token::kInstanceOf,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "finally",
                        value: Token::kFinally,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "async",
                        value: Token::kAsync,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "switch",
                        value: Token::kSwitch,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "while",
                        value: Token::kWhile,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "using",
                        value: Token::kUsing,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "import",
                        value: Token::kImport,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "else",
                        value: Token::kElse,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "of",
                        value: Token::kOf,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "if",
                        value: Token::kIf,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "implements",
                        value: Token::kFutureStrictReservedWord,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "yield",
                        value: Token::kYield,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "static",
                        value: Token::kStatic,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "class",
                        value: Token::kClass,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "accessor",
                        value: Token::kAccessor,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "with",
                        value: Token::kWith,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "await",
                        value: Token::kAwait,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                    PerfectKeywordHashTableEntry {
                        name: "",
                        value: Token::kIdentifier,
                    },
                ];

                if len == PERFECT_KEYWORD_LENGTH_TABLE[key] as usize {
                    let s = PERFECT_KEYWORD_HASH_TABLE[key].name;
                    let mut s_chars = s.chars();
                    let mut str_chars = str.chars();

                    loop {
                        match (s_chars.next(), str_chars.next()) {
                            (Some(s_char), Some(str_char)) => {
                                if s_char != str_char {
                                    return Token::kIdentifier;
                                }
                            }
                            (None, _) => {
                                return PERFECT_KEYWORD_HASH_TABLE[key].value;
                            }
                            _ => {
                                return Token::kIdentifier;
                            }
                        }
                    }
                }
            }
            Token::kIdentifier
        }
    }
}
