// Converted from V8 C++ source files:
// Header: asm-names.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod asm_names {
    macro_rules! stdlib_math_value_list {
        ($V:ident) => {
            $V!(E, 2.718281828459045);
            $V!(LN10, 2.302585092994046);
            $V!(LN2, 0.6931471805599453);
            $V!(LOG2E, 1.4426950408889634);
            $V!(LOG10E, 0.4342944819032518);
            $V!(PI, 3.141592653589793);
            $V!(SQRT1_2, 0.7071067811865476);
            $V!(SQRT2, 1.4142135623730951);
        };
    }

    macro_rules! stdlib_math_function_monomorphic_list {
        ($V:ident) => {
            $V!(acos, Acos, kExprF64Acos, dq2d);
            $V!(asin, Asin, kExprF64Asin, dq2d);
            $V!(atan, Atan, kExprF64Atan, dq2d);
            $V!(cos, Cos, kExprF64Cos, dq2d);
            $V!(sin, Sin, kExprF64Sin, dq2d);
            $V!(tan, Tan, kExprF64Tan, dq2d);
            $V!(exp, Exp, kExprF64Exp, dq2d);
            $V!(log, Log, kExprF64Log, dq2d);
            $V!(atan2, Atan2, kExprF64Atan2, dqdq2d);
            $V!(pow, Pow, kExprF64Pow, dqdq2d);
            $V!(imul, Imul, kExprI32Mul, ii2s);
            $V!(clz32, Clz32, kExprI32Clz, i2s);
        };
    }

    macro_rules! stdlib_math_function_ceil_like_list {
        ($V:ident) => {
            $V!(ceil, Ceil, x, ceil_like);
            $V!(floor, Floor, x, ceil_like);
            $V!(sqrt, Sqrt, x, ceil_like);
        };
    }

    macro_rules! stdlib_math_function_list {
        ($V:ident) => {
            $V!(min, Min, x, minmax);
            $V!(max, Max, x, minmax);
            $V!(abs, Abs, x, abs);
            $V!(fround, Fround, x, fround);
            stdlib_math_function_monomorphic_list!($V);
            stdlib_math_function_ceil_like_list!($V);
        };
    }

    macro_rules! stdlib_array_type_list {
        ($V:ident) => {
            $V!(Int8Array, Mem8S, Mem8, I32);
            $V!(Uint8Array, Mem8U, Mem8, I32);
            $V!(Int16Array, Mem16S, Mem16, I32);
            $V!(Uint16Array, Mem16U, Mem16, I32);
            $V!(Int32Array, Mem, Mem, I32);
            $V!(Uint32Array, Mem, Mem, I32);
            $V!(Float32Array, Mem, Mem, F32);
            $V!(Float64Array, Mem, Mem, F64);
        };
    }

    macro_rules! stdlib_other_list {
        ($V:ident) => {
            $V!(Infinity);
            $V!(NaN);
            $V!(Math);
        };
    }

    macro_rules! keyword_name_list {
        ($V:ident) => {
            $V!(arguments);
            $V!(break);
            $V!(case);
            $V!(const);
            $V!(continue);
            $V!(default);
            $V!(do);
            $V!(else);
            $V!(eval);
            $V!(for);
            $V!(function);
            $V!(if);
            $V!(new);
            $V!(return);
            $V!(switch);
            $V!(var);
            $V!(while);
        };
    }

    macro_rules! long_symbol_name_list {
        ($V:ident) => {
            $V!("<=", LE);
            $V!(">=", GE);
            $V!("==", EQ);
            $V!("!=", NE);
            $V!("<<", SHL);
            $V!(">>", SAR);
            $V!(">>>", SHR);
            $V!("'use asm'", UseAsm);
        };
    }

    macro_rules! simple_single_token_list {
        ($V:ident) => {
            $V!('+');
            $V!('-');
            $V!('*');
            $V!('%');
            $V!('~');
            $V!('^');
            $V!('&');
            $V!('|');
            $V!('(');
            $V!(')');
            $V!('[');
            $V!(']');
            $V!('{');
            $V!('}');
            $V!(':');
            $V!(';');
            $V!(',');
            $V!('?');
        };
    }

    macro_rules! special_token_list {
        ($V:ident) => {
            $V!(kUninitialized, 0, "{uninitialized}");
            $V!(kEndOfInput, -1, "{end of input}");
            $V!(kParseError, -2, "{parse error}");
            $V!(kUnsigned, -3, "{unsigned value}");
            $V!(kDouble, -4, "{double value}");
        };
    }

    // Example Usage (This code won't compile without dummy definitions for the tokens and wasm types)

    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    enum WasmType {
        I32,
        F32,
        F64,
    }

    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    enum Token {
        E,
        LN10,
        LN2,
        LOG2E,
        LOG10E,
        PI,
        SQRT1_2,
        SQRT2,
        Acos,
        Asin,
        Atan,
        Cos,
        Sin,
        Tan,
        Exp,
        Log,
        Atan2,
        Pow,
        Imul,
        Clz32,
        Ceil,
        Floor,
        Sqrt,
        Min,
        Max,
        Abs,
        Fround,
        Int8Array,
        Uint8Array,
        Int16Array,
        Uint16Array,
        Int32Array,
        Uint32Array,
        Float32Array,
        Float64Array,
        Infinity,
        NaN,
        Math,
        arguments,
        break_token,
        case_token,
        const_token,
        continue_token,
        default_token,
        do_token,
        else_token,
        eval_token,
        for_token,
        function_token,
        if_token,
        new_token,
        return_token,
        switch_token,
        var_token,
        while_token,
        LE,
        GE,
        EQ,
        NE,
        SHL,
        SAR,
        SHR,
        UseAsm,
        Plus,
        Minus,
        Multiply,
        Modulo,
        BitwiseNot,
        BitwiseXor,
        BitwiseAnd,
        BitwiseOr,
        OpenParen,
        CloseParen,
        OpenBracket,
        CloseBracket,
        OpenBrace,
        CloseBrace,
        Colon,
        Semicolon,
        Comma,
        QuestionMark,
        kUninitialized,
        kEndOfInput,
        kParseError,
        kUnsigned,
        kDouble,
    }

    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    enum Expr {
        kExprF64Acos,
        kExprF64Asin,
        kExprF64Atan,
        kExprF64Cos,
        kExprF64Sin,
        kExprF64Tan,
        kExprF64Exp,
        kExprF64Log,
        kExprF64Atan2,
        kExprF64Pow,
        kExprI32Mul,
        kExprI32Clz,
    }

    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    enum AsmJsType {
        dq2d,
        dqdq2d,
        ii2s,
        i2s,
        ceil_like,
        minmax,
        abs,
        fround,
    }

    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    enum MemType {
        Mem8S,
        Mem8U,
        Mem16S,
        Mem16U,
        Mem,
    }

    #[allow(dead_code)]
    fn example_usage() {
        macro_rules! print_value {
            ($name:ident, $value:expr) => {
                println!("Value: {:?} = {}", stringify!($name), $value);
            };
        }

        macro_rules! print_function_monomorphic {
            ($name:ident, $token:ident, $expr:ident, $asm_type:ident) => {
                println!(
                    "Function (Monomorphic): {} Token: {:?} Expr: {:?} AsmType: {:?}",
                    stringify!($name),
                    Token::$token,
                    Expr::$expr,
                    AsmJsType::$asm_type
                );
            };
        }

        macro_rules! print_function_ceil_like {
            ($name:ident, $token:ident, $unused:ident, $asm_type:ident) => {
                println!(
                    "Function (Ceil Like): {} Token: {:?} AsmType: {:?}",
                    stringify!($name),
                    Token::$token,
                    AsmJsType::$asm_type
                );
            };
        }

        macro_rules! print_function {
            ($name:ident, $token:ident, $unused:ident, $asm_type:ident) => {
                println!(
                    "Function: {} Token: {:?} AsmType: {:?}",
                    stringify!($name),
                    Token::$token,
                    AsmJsType::$asm_type
                );
            };
        }

        macro_rules! print_array_type {
            ($name:ident, $load_type:ident, $store_type:ident, $wasm_type:ident) => {
                println!(
                    "Array Type: {} Load: {:?} Store: {:?} WasmType: {:?}",
                    stringify!($name),
                    MemType::$load_type,
                    MemType::$store_type,
                    WasmType::$wasm_type
                );
            };
        }

        macro_rules! print_other {
            ($name:ident) => {
                println!("Other: {:?}", Token::$name);
            };
        }

        macro_rules! print_keyword {
            ($name:ident) => {
                println!("Keyword: {:?}", Token::$name);
            };
        }

        macro_rules! print_long_symbol {
            ($string:expr, $token:ident) => {
                println!("Long Symbol: {} Token: {:?}", $string, Token::$token);
            };
        }

        macro_rules! print_simple_token {
            ($char:expr) => {
                println!("Simple Token: {}", $char);
            };
        }

        macro_rules! print_special_token {
            ($name:ident, $value:expr, $string:expr) => {
                println!("Special Token: {:?} Value: {} String: {}", Token::$name, $value, $string);
            };
        }

        println!("--- Values ---");
        stdlib_math_value_list!(print_value);

        println!("--- Functions (Monomorphic) ---");
        stdlib_math_function_monomorphic_list!(print_function_monomorphic);

        println!("--- Functions (Ceil Like) ---");
        stdlib_math_function_ceil_like_list!(print_function_ceil_like);

        println!("--- Functions ---");
        stdlib_math_function_list!(print_function);

        println!("--- Array Types ---");
        stdlib_array_type_list!(print_array_type);

        println!("--- Others ---");
        stdlib_other_list!(print_other);

        println!("--- Keywords ---");
        keyword_name_list!(print_keyword);

        println!("--- Long Symbols ---");
        long_symbol_name_list!(print_long_symbol);

        println!("--- Simple Tokens ---");
        simple_single_token_list!(print_simple_token);

        println!("--- Special Tokens ---");
        special_token_list!(print_special_token);
    }
}
