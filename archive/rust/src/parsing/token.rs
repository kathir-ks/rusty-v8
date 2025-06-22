// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/parsing/token.rs

/// Defines tokens used in the V8 JavaScript engine's parser.
pub mod token {
    use std::ops::RangeInclusive;

    /// Represents the language mode for JavaScript parsing.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LanguageMode {
        Sloppy,
        Strict,
    }

    /// Determines if the given language mode is sloppy.
    #[inline]
    pub fn is_sloppy(language_mode: LanguageMode) -> bool {
        language_mode == LanguageMode::Sloppy
    }

    // Macro to define tokens and their properties.
    macro_rules! define_tokens {
        ($($(#[$attr:meta])* $name:ident = $string:literal, $precedence:literal;)*) => {
            /// Represents a token in the JavaScript language.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(u8)]
            pub enum Value {
                $($(#[$attr])* $name,)*
                NumTokens,
            }

            impl Value {
                /// Returns a string corresponding to the Rust token name
                pub fn name(&self) -> &'static str {
                    match self {
                        $(Value::$name => stringify!($name),)*
                        Value::NumTokens => "NumTokens", // Dummy value
                    }
                }

                /// Returns a string corresponding to the JS token string
                pub fn string(&self) -> Option<&'static str> {
                    match self {
                        $($(Value::$name => Some($string),)*)?
                        _ => None,
                    }
                }

                /// Returns the string length.
                pub fn string_length(&self) -> u8 {
                    match self {
                        $($(Value::$name => $string.len() as u8,)*)?
                        _ => 0,
                    }
                }

                /// Returns the precedence > 0 for binary and compare operators; returns 0 otherwise.
                pub fn precedence(&self, accept_in: bool) -> i8 {
                    match self {
                        $($(Value::$name => $precedence,)*)?
                        _ => 0,
                    }
                }

                 /// Checks if the token is a keyword.
                pub fn is_keyword(&self) -> bool {
                    KEYWORDS.contains(self)
                }

                /// Checks if the token is a property name.
                pub fn is_property_name(&self) -> bool {
                    PROPERTY_NAMES.contains(self)
                }
            }

            const KEYWORDS: &[Value] = &[
                $(
                    $(#[$attr])*
                    {
                        if $string != core::option::Option::None {
                            if stringify!($attr).contains("keyword") {
                                Value::$name
                            } else {
                                continue
                            }
                        } else {
                            continue
                        }
                    }
                ),*
            ];

            const PROPERTY_NAMES: &[Value] = &[
                $(
                    $(#[$attr])*
                    {
                        if $string != core::option::Option::None {
                            if stringify!($attr).contains("property_name") {
                                Value::$name
                            } else {
                                continue
                            }
                        } else {
                            continue
                        }
                    }
                ),*
            ];
        };
    }

    define_tokens! {
        kTemplateSpan = None, 0;
        kTemplateTail = None, 0;

        //Punctuators
        #[property_name]
        kPeriod = ".", 0;
        #[property_name]
        kLeftBracket = "[", 0;
        kQuestionPeriod = "?.", 0;
        kLeftParen = "(", 0;
        kRightParen = ")", 0;
        kRightBracket = "]", 0;
        kLeftBrace = "{", 0;
        kColon = ":", 0;
        kEllipsis = "...", 0;
        kConditional = "?", 3;
        kSemicolon = ";", 0;
        kRightBrace = "}", 0;
        kEos = "EOS", 0;

        kArrow = "=>", 0;
        kInit = "=init", 2;
        kAssign = "=", 2;
        kAssignNullish = "??=", 2;
        kAssignOr = "||=", 2;
        kAssignAnd = "&&=", 2;
        kAssignBitOr = "|=", 2;
        kAssignBitXor = "^=", 2;
        kAssignBitAnd = "&=", 2;
        kAssignShl = "<<=", 2;
        kAssignSar = ">>=", 2;
        kAssignShr = ">>>=", 2;
        kAssignMul = "*=", 2;
        kAssignDiv = "/=", 2;
        kAssignMod = "%=", 2;
        kAssignExp = "**=", 2;
        kAssignAdd = "+=", 2;
        kAssignSub = "-=", 2;

        kComma = ",", 1;

        kNullish = "??", 3;
        kOr = "||", 4;
        kAnd = "&&", 5;
        kBitOr = "|", 6;
        kBitXor = "^", 7;
        kBitAnd = "&", 8;
        kShl = "<<", 11;
        kSar = ">>", 11;
        kShr = ">>>", 11;
        kMul = "*", 13;
        kDiv = "/", 13;
        kMod = "%", 13;
        kExp = "**", 14;
        kAdd = "+", 12;
        kSub = "-", 12;

        kNot = "!", 0;
        kBitNot = "~", 0;
        #[keyword]
        kDelete = "delete", 0;
        #[keyword]
        kTypeOf = "typeof", 0;
        #[keyword]
        kVoid = "void", 0;

        kInc = "++", 0;
        kDec = "--", 0;

        kEq = "==", 9;
        kEqStrict = "===", 9;
        kNotEq = "!=", 9;
        kNotEqStrict = "!==", 9;
        kLessThan = "<", 10;
        kGreaterThan = ">", 10;
        kLessThanEq = "<=", 10;
        kGreaterThanEq = ">=", 10;
        #[keyword]
        kInstanceOf = "instanceof", 10;
        #[keyword]
        kIn = "in", 10;

        #[keyword]
        kBreak = "break", 0;
        #[keyword]
        kCase = "case", 0;
        #[keyword]
        kCatch = "catch", 0;
        #[keyword]
        kContinue = "continue", 0;
        #[keyword]
        kDebugger = "debugger", 0;
        #[keyword]
        kDefault = "default", 0;
        #[keyword]
        kDo = "do", 0;
        #[keyword]
        kElse = "else", 0;
        #[keyword]
        kFinally = "finally", 0;
        #[keyword]
        kFor = "for", 0;
        #[keyword]
        kFunction = "function", 0;
        #[keyword]
        kIf = "if", 0;
        #[keyword]
        kNew = "new", 0;
        #[keyword]
        kReturn = "return", 0;
        #[keyword]
        kSwitch = "switch", 0;
        #[keyword]
        kThrow = "throw", 0;
        #[keyword]
        kTry = "try", 0;
        #[keyword]
        kVar = "var", 0;
        #[keyword]
        kWhile = "while", 0;
        #[keyword]
        kWith = "with", 0;
        #[keyword]
        kThis = "this", 0;

        #[keyword]
        kNullLiteral = "null", 0;
        #[keyword]
        kTrueLiteral = "true", 0;
        #[keyword]
        kFalseLiteral = "false", 0;
        kNumber = None, 0;
        kSmi = None, 0;
        kBigInt = None, 0;
        kString = None, 0;

        #[keyword]
        kSuper = "super", 0;
        kIdentifier = None, 0;
        #[keyword]
        kGet = "get", 0;
        #[keyword]
        kSet = "set", 0;
        #[keyword]
        kUsing = "using", 0;
        #[keyword]
        kOf = "of", 0;
        #[keyword]
        kAccessor = "accessor", 0;
        #[keyword]
        kAsync = "async", 0;
        #[keyword]
        kAwait = "await", 0;
        #[keyword]
        kYield = "yield", 0;
        #[keyword]
        kLet = "let", 0;
        #[keyword]
        kStatic = "static", 0;
        kFutureStrictReservedWord = None, 0;
        kEscapedStrictReservedWord = None, 0;
        #[keyword]
        kEnum = "enum", 0;
        #[keyword]
        kClass = "class", 0;
        #[keyword]
        kConst = "const", 0;
        #[keyword]
        kExport = "export", 0;
        #[keyword]
        kExtends = "extends", 0;
        #[keyword]
        kImport = "import", 0;
        kPrivateName = None, 0;

        kIllegal = "ILLEGAL", 0;
        kEscapedKeyword = None, 0;

        kWhitespace = None, 0;
        kUninitialized = None, 0;
        kRegExpLiteral = None, 0;
    }

    impl Value {
        /// Checks if the token is a valid identifier.
        #[inline]
        pub fn is_valid_identifier(
            &self,
            language_mode: LanguageMode,
            is_generator: bool,
            disallow_await: bool,
        ) -> bool {
            if Self::is_in_range(*self, Value::kIdentifier, Value::kAsync) {
                return true;
            }
            if *self == Value::kAwait {
                return !disallow_await;
            }
            if *self == Value::kYield {
                return !is_generator && is_sloppy(language_mode);
            }
            Self::is_strict_reserved_word(*self) && is_sloppy(language_mode)
        }

        /// Checks if the token is callable.
        pub fn is_callable(&self) -> bool {
            Self::is_in_range(*self, Value::kSuper, Value::kEscapedStrictReservedWord)
        }

        /// Checks if the token can cause automatic semicolon insertion.
        pub fn is_auto_semicolon(&self) -> bool {
            Self::is_in_range(*self, Value::kSemicolon, Value::kEos)
        }

        /// Checks if the token is any identifier.
        pub fn is_any_identifier(&self) -> bool {
            Self::is_in_range(*self, Value::kIdentifier, Value::kEscapedStrictReservedWord)
        }

        /// Checks if the token is a strict reserved word.
        pub fn is_strict_reserved_word(&self) -> bool {
            Self::is_in_range(*self, Value::kYield, Value::kEscapedStrictReservedWord)
        }

        /// Checks if the token is a literal.
        pub fn is_literal(&self) -> bool {
            Self::is_in_range(*self, Value::kNullLiteral, Value::kString)
        }

        /// Checks if the token is a template.
        pub fn is_template(&self) -> bool {
            Self::is_in_range(*self, Value::kTemplateSpan, Value::kTemplateTail)
        }

        /// Checks if the token is a member.
        pub fn is_member(&self) -> bool {
            Self::is_in_range(*self, Value::kTemplateSpan, Value::kLeftBracket)
        }

        /// Checks if the token is a property.
        pub fn is_property(&self) -> bool {
            Self::is_in_range(*self, Value::kPeriod, Value::kLeftBracket)
        }

        /// Checks if the token is a property or call.
        pub fn is_property_or_call(&self) -> bool {
            Self::is_in_range(*self, Value::kTemplateSpan, Value::kLeftParen)
        }

        /// Checks if the token is an arrow or assignment operator.
        pub fn is_arrow_or_assignment_op(&self) -> bool {
            Self::is_in_range(*self, Value::kArrow, Value::kAssignSub)
        }

        /// Checks if the token is an assignment operator.
        pub fn is_assignment_op(&self) -> bool {
            Self::is_in_range(*self, Value::kInit, Value::kAssignSub)
        }

        /// Checks if the token is a logical assignment operator.
        pub fn is_logical_assignment_op(&self) -> bool {
            Self::is_in_range(*self, Value::kAssignNullish, Value::kAssignAnd)
        }

        /// Checks if the token is a binary operator.
        pub fn is_binary_op(&self) -> bool {
            Self::is_in_range(*self, Value::kComma, Value::kSub)
        }

        /// Checks if the token is a compare operator.
        pub fn is_compare_op(&self) -> bool {
            Self::is_in_range(*self, Value::kEq, Value::kIn)
        }

        pub fn is_ordered_relational_compare_op(&self) -> bool {
            Self::is_in_range(*self, Value::kLessThan, Value::kGreaterThanEq)
        }

        pub fn is_equality_op(&self) -> bool {
            Self::is_in_range(*self, Value::kEq, Value::kEqStrict)
        }

        /// Returns the binary operator for the assignment operator.
        pub fn binary_op_for_assignment(&self) -> Self {
            if !Self::is_in_range(*self, Value::kAssignNullish, Value::kAssignSub) {
                panic!("Token is not an assignment operator");
            }
            let result = unsafe { std::mem::transmute::<u8, Value>(*self as u8 - Value::kAssignNullish as u8 + Value::kNullish as u8) };
            if !Self::is_binary_op(result) {
                panic!("Result is not a binary operator");
            }
            result
        }

        /// Checks if the token is a bitwise operator.
        pub fn is_bit_op(&self) -> bool {
            Self::is_in_range(*self, Value::kBitOr, Value::kShr) || *self == Value::kBitNot
        }

        /// Checks if the token is a unary operator.
        pub fn is_unary_op(&self) -> bool {
            Self::is_in_range(*self, Value::kAdd, Value::kVoid)
        }

        /// Checks if the token is a count operator.
        pub fn is_count_op(&self) -> bool {
            Self::is_in_range(*self, Value::kInc, Value::kDec)
        }

        /// Checks if the token is a unary or count operator.
        pub fn is_unary_or_count_op(&self) -> bool {
            Self::is_in_range(*self, Value::kAdd, Value::kDec)
        }

        /// Checks if the token is a shift operator.
        pub fn is_shift_op(&self) -> bool {
            Self::is_in_range(*self, Value::kShl, Value::kShr)
        }

        fn is_in_range(token: Value, start: Value, end: Value) -> bool {
            (token as u8) >= (start as u8) && (token as u8) <= (end as u8)
        }
    }

    impl From<Value> for u8 {
        fn from(token: Value) -> Self {
            token as u8
        }
    }

    const LAST_ASSIGN: Value = Value::kAssignSub;
    const LAST_BINARY: Value = Value::kSub;
    const LAST_UNARY_OR_COUNT: Value = Value::kDec;
    const LAST_COMPARE: Value = Value::kIn;
}