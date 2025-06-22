use std::collections::HashMap;
use std::convert::TryInto;
use std::f64;
use std::num::ParseFloatError;
use std::str;

mod base {
    pub type uc32 = u32;

    pub fn is_in_range<T: PartialOrd>(value: T, low: T, high: T) -> bool {
        value >= low && value <= high
    }

    pub mod iterator {
        pub trait Iterator<T> {
            fn advance(&mut self) -> Option<T>;
            fn back(&mut self, value: T);
        }
    }

    pub mod vector {
        #[derive(Debug, Clone)]
        pub struct Vector<T> {
            data: Vec<T>,
        }

        impl<T: Copy> Vector<T> {
            pub fn new(data: Vec<T>) -> Self {
                Vector { data }
            }

            pub fn as_slice(&self) -> &[T] {
                &self.data
            }

            pub fn cast(v: Vector<u8>) -> Vector<u8> {
                v
            }

            pub fn of(s: &str) -> Vector<u8> {
                Vector { data: s.bytes().collect() }
            }
        }
    }
}

mod flags {
    pub struct Flags {
        pub trace_asm_scanner: bool,
    }

    impl Flags {
        pub fn new() -> Self {
            Flags {
                trace_asm_scanner: false,
            }
        }
    }

    pub static mut FLAGS: Flags = Flags { trace_asm_scanner: false };
}

mod numbers {
    pub mod conversions {
        pub fn string_to_double(s: &str) -> Result<f64, ParseFloatError> {
            s.parse::<f64>()
        }

        pub fn binary_string_to_double(s: &str) -> Result<f64, ParseFloatError> {
            let mut result = 0.0;
            for (i, c) in s.chars().rev().enumerate() {
                if c == '1' {
                    result += 2.0_f64.powi(i as i32);
                } else if c != '0' && c != 'b'{
                    return Err("Invalid character in binary string".parse::<f64>().unwrap_err());
                }
            }
            Ok(result)
        }

        pub fn octal_string_to_double(s: &str) -> Result<f64, ParseFloatError> {
            let mut result = 0.0;
            for (i, c) in s.chars().rev().enumerate() {
                if c.is_digit(8) {
                    result += (c.to_digit(10).unwrap() as f64) * 8.0_f64.powi(i as i32);
                } else if c != 'o' {
                     return Err("Invalid character in octal string".parse::<f64>().unwrap_err());
                }

            }
             Ok(result)
        }
        pub fn implicit_octal_string_to_double(s: &str) -> Result<f64, ParseFloatError> {
              let mut result = 0.0;
            for (i, c) in s.chars().rev().enumerate() {
                if c.is_digit(8) {
                    result += (c.to_digit(10).unwrap() as f64) * 8.0_f64.powi(i as i32);
                } else{
                     return Err("Invalid character in octal string".parse::<f64>().unwrap_err());
                }

            }
             Ok(result)
        }

        pub fn hex_string_to_double(s: &str) -> Result<f64, ParseFloatError> {
            let mut result = 0.0;
            for (i, c) in s.chars().rev().enumerate() {
                if c.is_digit(16) {
                    result += (c.to_digit(16).unwrap() as f64) * 16.0_f64.powi(i as i32);
                } else if c != 'x'{
                     return Err("Invalid character in hexadecimal string".parse::<f64>().unwrap_err());
                }

            }
             Ok(result)
        }
    }
}

mod parsing {
    pub mod scanner {
        pub trait Utf16CharacterStream {
            fn advance(&mut self) -> base::uc32;
            fn back(&mut self);
            fn pos(&self) -> usize;
            fn seek(&mut self, pos: usize);
        }
    }
}

mod strings {
    pub mod char_predicates {
        pub fn is_ascii_identifier(ch: base::uc32) -> bool {
            is_ascii_alpha(ch) || is_decimal_digit(ch) || ch == 0x24 || ch == 0x5F
        }

        pub fn is_ascii_alpha(ch: base::uc32) -> bool {
            (ch >= 'a' as base::uc32 && ch <= 'z' as base::uc32)
                || (ch >= 'A' as base::uc32 && ch <= 'Z' as base::uc32)
        }

        pub fn is_decimal_digit(ch: base::uc32) -> bool {
            ch >= '0' as base::uc32 && ch <= '9' as base::uc32
        }
        pub fn is_octal_digit(ch: u8) -> bool {
             ch >= b'0' && ch <= b'7'
        }
        pub fn ascii_alpha_to_lower(ch: base::uc32) -> base::uc32 {
            if ch >= 'A' as base::uc32 && ch <= 'Z' as base::uc32 {
                ch + 32
            } else {
                ch
            }
        }
    }
}

pub mod asmjs {
    use super::*;
    use base::{uc32, vector::Vector};
    use parsing::scanner::Utf16CharacterStream;
    use strings::char_predicates::*;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Token {
        Uninitialized,
        EndOfInput,
        ParseError,
        Slash,
        Double,
        Unsigned,
        Dot,
        Token_Imul,
        Token_Fround,
        Token_Ems,
        Token_Sqrt,
        Token_Abs,
        Token_Cos,
        Token_Sin,
        Token_Tan,
        Token_Acos,
        Token_Asin,
        Token_Atan,
        Token_Ceil,
        Token_Floor,
        Token_Exp,
        Token_Log,
        Token_Atan2,
        Token_Pow,
        Token_Min,
        Token_Max,
        Token_Idiv,
        Token_Udiv,
        Token_Shl,
        Token_Shr,
        Token_Sar,
        Token_Or,
        Token_And,
        Token_Xor,
        Token_Not,
        Token_Neg,
        Token_Add,
        Token_Sub,
        Token_Mul,
        Token_Div,
        Token_Mod,
        Token_Eq,
        Token_NE,
        Token_LE,
        Token_GE,
        Token_ToInt32,
        Token_ToUint32,
        Token_ToFloat32,
        Token_UseAsm,
        Infinity,
        NaN,
        GlobalsStart,
        LocalsStart,
        // Single character tokens
        LParen,
        RParen,
        LBrace,
        RBrace,
        LBracket,
        RBracket,
        Semicolon,
        Comma,
        Assign,
        Plus,
        Minus,
        Multiply,
        Divide,
        Modulo,
        BitwiseAnd,
        BitwiseOr,
        BitwiseXor,
        BitwiseNot,
        LessThan,
        GreaterThan,
        Equals,
        NotEquals,
    }

    use Token::*;

    // Adapt any preprocessor macros to Rust macro_rules! or const values
    macro_rules! define_tokens {
        ($($(#[$attr:meta])* $name:ident = $value:expr,)*) => {
            $(
                $(#[$attr])*
                pub const $name: Token = $value;
            )*
        }
    }

    define_tokens! {
        kMaxIdentifierCount = 0xF000000 as isize,
        kGlobalsStart = Token::GlobalsStart,
        kLocalsStart = Token::LocalsStart,
    }

    pub struct AsmJsScanner<'a> {
        stream_: &'a mut dyn Utf16CharacterStream,
        token_: Token,
        preceding_token_: Token,
        next_token_: Token,
        position_: usize,
        preceding_position_: usize,
        next_position_: usize,
        rewind_: bool,
        in_local_scope_: bool,
        global_count_: usize,
        double_value_: f64,
        unsigned_value_: u32,
        preceded_by_newline_: bool,
        identifier_string_: String,
        property_names_: HashMap<String, Token>,
        global_names_: HashMap<String, Token>,
        local_names_: HashMap<String, Token>,
    }

    impl<'a> AsmJsScanner<'a> {
        pub fn new(stream: &'a mut dyn Utf16CharacterStream) -> Self {
            let mut scanner = AsmJsScanner {
                stream_: stream,
                token_: Uninitialized,
                preceding_token_: Uninitialized,
                next_token_: Uninitialized,
                position_: 0,
                preceding_position_: 0,
                next_position_: 0,
                rewind_: false,
                in_local_scope_: false,
                global_count_: 0,
                double_value_: 0.0,
                unsigned_value_: 0,
                preceded_by_newline_: false,
                identifier_string_: String::new(),
                property_names_: HashMap::new(),
                global_names_: HashMap::new(),
                local_names_: HashMap::new(),
            };

            // Pre-populate property names.
            macro_rules! insert_property_name {
                ($name:ident) => {
                    scanner
                        .property_names_
                        .insert(stringify!($name).to_string(), Token::$name);
                };
            }

            insert_property_name!(Token_Imul);
            insert_property_name!(Token_Fround);
            insert_property_name!(Token_Ems);
            insert_property_name!(Token_Sqrt);
            insert_property_name!(Token_Abs);
            insert_property_name!(Token_Cos);
            insert_property_name!(Token_Sin);
            insert_property_name!(Token_Tan);
            insert_property_name!(Token_Acos);
            insert_property_name!(Token_Asin);
            insert_property_name!(Token_Atan);
            insert_property_name!(Token_Ceil);
            insert_property_name!(Token_Floor);
            insert_property_name!(Token_Exp);
            insert_property_name!(Token_Log);
            insert_property_name!(Token_Atan2);
            insert_property_name!(Token_Pow);
            insert_property_name!(Token_Min);
            insert_property_name!(Token_Max);
            insert_property_name!(Token_Idiv);
            insert_property_name!(Token_Udiv);
            insert_property_name!(Token_Shl);
            insert_property_name!(Token_Shr);
            insert_property_name!(Token_Sar);
            insert_property_name!(Token_Or);
            insert_property_name!(Token_And);
            insert_property_name!(Token_Xor);
            insert_property_name!(Token_Not);
            insert_property_name!(Token_Neg);
            insert_property_name!(Token_Add);
            insert_property_name!(Token_Sub);
            insert_property_name!(Token_Mul);
            insert_property_name!(Token_Div);
            insert_property_name!(Token_Mod);
            insert_property_name!(Token_Eq);
            insert_property_name!(Token_NE);
            insert_property_name!(Token_LE);
            insert_property_name!(Token_GE);
            insert_property_name!(Token_ToInt32);
            insert_property_name!(Token_ToUint32);
            insert_property_name!(Token_ToFloat32);
            insert_property_name!(Infinity);
            insert_property_name!(NaN);
            macro_rules! insert_global_name {
                ($name:ident) => {
                    scanner
                        .global_names_
                        .insert(stringify!($name).to_string(), Token::$name);
                };
            }
            insert_global_name!(LParen);
            insert_global_name!(RParen);
            insert_global_name!(LBrace);
            insert_global_name!(RBrace);
            insert_global_name!(LBracket);
            insert_global_name!(RBracket);
            insert_global_name!(Semicolon);
            insert_global_name!(Comma);
            insert_global_name!(Assign);
            insert_global_name!(Plus);
            insert_global_name!(Minus);
            insert_global_name!(Multiply);
            insert_global_name!(Divide);
            insert_global_name!(Modulo);
            insert_global_name!(BitwiseAnd);
            insert_global_name!(BitwiseOr);
            insert_global_name!(BitwiseXor);
            insert_global_name!(BitwiseNot);
            insert_global_name!(LessThan);
            insert_global_name!(GreaterThan);
            insert_global_name!(Equals);
            insert_global_name!(NotEquals);
            scanner.Next();
            scanner
        }

        pub fn next(&mut self) {
            if self.rewind_ {
                self.preceding_token_ = self.token_;
                self.preceding_position_ = self.position_;
                self.token_ = self.next_token_;
                self.position_ = self.next_position_;
                self.next_token_ = Uninitialized;
                self.next_position_ = 0;
                self.rewind_ = false;
                return;
            }

            if self.token_ == EndOfInput || self.token_ == ParseError {
                return;
            }

            #[cfg(debug_assertions)]
            unsafe {
                if flags::FLAGS.trace_asm_scanner {
                    if self.Token() == Double {
                        print!("{} ", self.AsDouble());
                    } else if self.Token() == Unsigned {
                        print!("{} ", self.AsUnsigned());
                    } else {
                        let name = self.Name(self.Token());
                        print!("{} ", name);
                    }
                }
            }

            self.preceded_by_newline_ = false;
            self.preceding_token_ = self.token_;
            self.preceding_position_ = self.position_;

            loop {
                self.position_ = self.stream_.pos();
                let ch = self.stream_.advance();
                match ch {
                    Some(' ' as uc32) | Some('\t' as uc32) | Some('\r' as uc32) => {
                        // Ignore whitespace.
                    }

                    Some('\n' as uc32) => {
                        // Track when we've passed a newline for optional semicolon support,
                        // but keep scanning.
                        self.preceded_by_newline_ = true;
                    }

                    None => {
                        self.token_ = EndOfInput;
                        return;
                    }

                    Some('\'' as uc32) | Some('"' as uc32) => {
                        self.ConsumeString(ch.unwrap());
                        return;
                    }

                    Some('/' as uc32) => {
                        let next_ch = self.stream_.advance();
                        match next_ch {
                            Some('/' as uc32) => {
                                self.ConsumeCPPComment();
                            }
                            Some('*' as uc32) => {
                                if !self.ConsumeCComment() {
                                    self.token_ = ParseError;
                                    return;
                                }
                            }
                            _ => {
                                if let Some(c) = next_ch {
                                    self.stream_.back();
                                }
                                self.token_ = Slash;
                                return;
                            }
                        }
                        // Breaks out of switch, but loops again (i.e. the case when we parsed
                        // a comment, but need to continue to look for the next token).
                    }

                    Some('<' as uc32) | Some('>' as uc32) | Some('=' as uc32) | Some('!' as uc32) =>
                    {
                        self.ConsumeCompareOrShift(ch.unwrap());
                        return;
                    }

                    Some('(' as uc32) => {
                        self.token_ = LParen;
                        return;
                    }
                    Some(')' as uc32) => {
                        self.token_ = RParen;
                        return;
                    }
                    Some('{' as uc32) => {
                        self.token_ = LBrace;
                        return;
                    }
                    Some('}' as uc32) => {
                        self.token_ = RBrace;
                        return;
                    }
                    Some('[' as uc32) => {
                        self.token_ = LBracket;
                        return;
                    }
                    Some(']' as uc32) => {
                        self.token_ = RBracket;
                        return;
                    }
                    Some(';' as uc32) => {
                        self.token_ = Semicolon;
                        return;
                    }
                    Some(',' as uc32) => {
                        self.token_ = Comma;
                        return;
                    }
                    Some('+' as uc32) => {
                        self.token_ = Plus;
                        return;
                    }
                    Some('-' as uc32) => {
                        self.token_ = Minus;
                        return;
                    }
                    Some('*' as uc32) => {
                        self.token_ = Multiply;
                        return;
                    }
                    Some('%' as uc32) => {
                        self.token_ = Modulo;
                        return;
                    }
                    Some('&' as uc32) => {
                        self.token_ = BitwiseAnd;
                        return;
                    }
                    Some('|' as uc32) => {
                        self.token_ = BitwiseOr;
                        return;
                    }
                    Some('^' as uc32) => {
                        self.token_ = BitwiseXor;
                        return;
                    }
                    Some('~' as uc32) => {
                        self.token_ = BitwiseNot;
                        return;
                    }
                    Some('=' as uc32) => {
                        self.token_ = Assign;
                        return;
                    }
                    Some('<' as uc32) => {
                        self.token_ = LessThan;
                        return;
                    }
                    Some('>' as uc32) => {
                        self.token_ = GreaterThan;
                        return;
                    }
                    Some(ch) => {
                        if Self::IsIdentifierStart(ch) {
                            self.ConsumeIdentifier(ch);
                        } else if Self::IsNumberStart(ch) {
                            self.ConsumeNumber(ch);
                        } else {
                            // TODO(bradnelson): Support unicode (probably via UnicodeCache).
                            self.token_ = ParseError;
                        }
                        return;
                    }
                }
            }
        }

        pub fn rewind(&mut self) {
            debug_assert_ne!(Uninitialized, self.preceding_token_);
            debug_assert!(!self.rewind_);
            self.next_token_ = self.token_;
            self.next_position_ = self.position_;
            self.token_ = self.preceding_token_;
            self.position_ = self.preceding_position_;
            self.preceding_token_ = Uninitialized;
            self.preceding_position_ = 0;
            self.rewind_ = true;
            self.identifier_string_.clear();
        }

        pub fn reset_locals(&mut self) {
            self.local_names_.clear();
        }

        #[cfg(debug_assertions)]
        pub fn Name(&self, token: Token) -> String {
            if let Ok(token_value) = TryInto::<u32>::try_into(token as isize){
                if token_value >= 32 && token_value < 127 {
                return String::from_utf8(vec![token_value as u8]).unwrap();
                }
            }
            for (name, tok) in &self.local_names_ {
                if *tok == token {
                    return name.clone();
                }
            }
            for (name, tok) in &self.global_names_ {
                if *tok == token {
                    return name.clone();
                }
            }
            for (name, tok) in &self.property_names_ {
                if *tok == token {
                    return name.clone();
                }
            }

            match token {
                Token_UseAsm => "use asm".to_string(),
                Infinity => "Infinity".to_string(),
                NaN => "NaN".to_string(),
                EndOfInput => "EndOfInput".to_string(),
                ParseError => "ParseError".to_string(),
                Uninitialized => "Uninitialized".to_string(),
                _ => panic!("Unhandled token in Name function: {:?}", token),
            }
        }

        pub fn seek(&mut self, pos: usize) {
            self.stream_.seek(pos);
            self.preceding_token_ = Uninitialized;
            self.token_ = Uninitialized;
            self.next_token_ = Uninitialized;
            self.preceding_position_ = 0;
            self.position_ = 0;
            self.next_position_ = 0;
            self.rewind_ = false;
            self.Next();
        }

        fn ConsumeIdentifier(&mut self, ch: uc32) {
            // Consume characters while still part of the identifier.
            self.identifier_string_.clear();
            let mut current_char = Some(ch);
            while let Some(c) = current_char {
                if Self::IsIdentifierPart(c) {
                    self.identifier_string_.push(c as u8 as char);
                    current_char = self.stream_.advance();
                } else {
                    break;
                }
            }

            if let Some(c) = current_char {
                 self.stream_.back();
            }
           
            // Decode what the identifier means.
            if self.preceding_token_ == Dot {
                if let Some(token) = self.property_names_.get(&self.identifier_string_) {
                    self.token_ = *token;
                    return;
                }
            } else {
                if let Some(token) = self.local_names_.get(&self.identifier_string_) {
                    self.token_ = *token;
                    return;
                }

                if !self.in_local_scope_ {
                    if let Some(token) = self.global_names_.get(&self.identifier_string_) {
                        self.token_ = *token;
                        return;
                    }
                }
            }

            if self.preceding_token_ == Dot {
                assert!((self.global_count_ as isize) < kMaxIdentifierCount);
                self.token_ = GlobalsStart;
                self.token_ = match self.token_ {
                    GlobalsStart => {
                        let new_token = unsafe {
                            std::mem::transmute::<isize, Token>((GlobalsStart as isize) + self.global_count_ as isize)
                        };
                         self.property_names_.insert(self.identifier_string_.clone(),new_token);
                         self.global_count_ +=1;
                         new_token
                    }
                    _ => unreachable!(),
                };
            } else if self.in_local_scope_ {
                assert!(self.local_names_.len() as isize < kMaxIdentifierCount);
               self.token_ = LocalsStart;
                self.token_ = match self.token_ {
                    LocalsStart => {
                         let new_token = unsafe {
                            std::mem::transmute::<isize, Token>((LocalsStart as isize) - self.local_names_.len() as isize)
                        };
                        self.local_names_.insert(self.identifier_string_.clone(), new_token);
                        new_token
                    }
                    _ => unreachable!(),
                };
            } else {
                assert!((self.global_count_ as isize) < kMaxIdentifierCount);
               self.token_ = GlobalsStart;
                self.token_ = match self.token_ {
                    GlobalsStart => {
                         let new_token = unsafe {
                            std::mem::transmute::<isize, Token>((GlobalsStart as isize) + self.global_count_ as isize)
                        };
                         self.global_names_.insert(self.identifier_string_.clone(),new_token);
                         self.global_count_ +=1;
                         new_token
                    }
                    _ => unreachable!(),
                };
            }
        }

        fn is_valid_implicit_octal(number: &str) -> bool {
            debug_assert_eq!(number.chars().next(), Some('0'));
            number.chars().skip(1).all(|c| is_octal_digit(c as u8))
        }

        fn ConsumeNumber(&mut self, ch: uc32) {
            let mut number = String::new();
            number.push(ch as u8 as char);
            let mut has_dot = ch == '.' as u32;
            let mut has_prefix = false;

            loop {
                let ch = self.stream_.advance();
                match ch {
                    Some(c)
                        if (c >= '0' as uc32 && c <= '9' as uc32)
                            || (c >= 'a' as uc32 && c <= 'f' as uc32)
                            || (c >= 'A' as uc32 && c <= 'F' as uc32)
                            || c == '.' as uc32
                            || c == 'b' as uc32
                            || c == 'o' as uc32
                            || c == 'x' as uc32
                            || ((c == '-' as uc32 || c == '+' as uc32)
                                && !has_prefix
                                && (number.chars().last() == Some('e') || number.chars().last() == Some('E'))) =>
                    {
                        if c == '.' as uc32 {
                            has_dot = true;
                        }
                        if c == 'b' as uc32 || c == 'o' as uc32 || c == 'x' as uc32 {
                            has_prefix = true;
                        }
                        number.push(c as u8 as char);
                    }
                    _ => {
                        if let Some(c) = ch {
                            self.stream_.back();
                        }
                        break;
                    }
                }
            }

            if number.len() == 1 && number.chars().next() == Some('0') {
                self.unsigned_value_ = 0;
                self.token_ = Unsigned;
                return;
            }

            if number.len() == 1 && number.chars().next() == Some('.') {
                self.token_ = Dot;
                return;
            }

            let double_value_result: Result<f64, ParseFloatError> = if has_prefix && number.chars().next() == Some('0') {
                if number.len() <= 2 {
                    self.token_ = ParseError;
                    return;
                }
                match number.chars().nth(1) {
                    Some('b') => numbers::conversions::binary_string_to_double(&number),
                    Some('o') => numbers::conversions::octal_string_to_double(&number),
                    Some('x') => numbers::conversions::hex_string_to_double(&number),
                    _ => {
                        self.token_ = ParseError;
                         return;
                    }
                }
            } else if number.chars().next() == Some('0') && !has_prefix && Self::is_valid_implicit_octal(&number) {
                 numbers::conversions::implicit_octal_string_to_double(&number)
            } else {
                numbers::conversions::string_to_double(&number)
            };

            match double_value_result {
                Ok(double_value) => {
                    if double_value.is_nan() {
                        if number.chars().next() == Some('.') {
                            for _ in 1..number.len() {
                                self.stream_.back();
                            }
                            self.token_ = Dot;
                            return;
                        }
                        self.token_ = ParseError;
                        return;
                    }

                    if has_dot || double_value.trunc() != double_value {
                        self.token_ = Double;
                        self.double_value_ = double_value;
                    } else {
                        if double_value > (u32::MAX as f64) {
                            self.token_ = ParseError;
                            return;
                        }
                        self.unsigned_value_ = double_value as u32;
                        self.token_ = Unsigned;
                    }
                }
                Err(_) => {
                    self.token_ = ParseError;
                }
            }
        }

        fn ConsumeCComment(&mut self) -> bool {
            loop {
                let ch = self.stream_.advance();
                match ch {
                    Some('*' as uc32) => {
                        let next_ch = self.stream_.advance();
                        if next_ch == Some('/' as uc32) {
                            return true;
                        }
                    }
                    Some('\n' as uc32) => {
                        self.preceded_by_newline_ = true;
                    }
                    None => {
                        return false;
                    }
                    _ => {}
                }
            }
        }

        fn ConsumeCPPComment(&mut self) {
            loop {
                let ch = self.stream_.advance();
                match ch {
                    Some('\n' as uc32) => {
                        self.preceded_by_newline_ = true;
                        return;
                    }
                    None => {
                        return;
                    }
                    _ => {}
                }
            }
        }

        fn ConsumeString(&mut self, quote: uc32) {
            let expected = "use asm";
            for expected_char in expected.chars() {
                if self.stream_.advance() != Some(expected_char as uc32) {
                    self.token_ = ParseError;
                    return;
                }
            }
            if self.stream_.advance() != Some(quote) {
                self.token_ = ParseError;
                return;
            }
            self.token_ = Token_UseAsm;
        }

        fn ConsumeCompareOrShift(&mut self, ch: uc32) {
            let next_ch = self.stream_.advance();
            match next_ch {
                Some('=' as uc32) => match ch {
                    '<' as uc32 => self.token_ = Token_LE,
                    '>' as uc32 => self.token_ = Token_GE,
                    '=' as uc32 => self.token_ = Token_EQ,
                    '!' as uc32 => self.token_ = Token_NE,
                    _ => unreachable!(),
                },
                Some('<' as uc32) if ch == '<' as uc32 => self.token_ = Token_SHL,
                Some('>' as uc32) if ch == '>' as uc32 => {
                    if self.stream_.advance() == Some('>' as uc32) {
                        self.token_ = Token_SHR;
                    } else {
                        self.token_ = Token_SAR;
                        self.stream_.back();
                    }
                }
                _ => {
                    if let Some(c) = next_ch {
                        self.stream_.back();
                    }
                    self.token_ = unsafe { std::mem::transmute::<u32, Token>(ch) };
                }
            }
        }

        fn IsIdentifierStart(ch: uc32) -> bool {
            base::is_in_range(ascii_alpha_to_lower(ch), 'a' as uc32, 'z' as uc32)
                || ch == '_' as uc32
                || ch == '$' as uc32
        }

        fn IsIdentifierPart(ch: uc32) -> bool {
            is_ascii_identifier(ch)
        }

        fn IsNumberStart(ch: uc32) -> bool {
            ch == '.' as uc32 || is_decimal_digit(ch)
        }

        pub fn Token(&self) -> Token {
            self.token_
        }
        pub fn AsDouble(&self) -> f64 {
            self.double_value_
        }
        pub fn AsUnsigned(&self) -> u32 {
            self.unsigned_value_
        }

        pub fn PrecededByNewline(&self) -> bool {
            self.preceded_by_newline_
        }
    }
}