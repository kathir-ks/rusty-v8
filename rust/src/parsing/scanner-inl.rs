// src/parsing/scanner.rs

mod unibrow {
    pub fn is_line_terminator(c: char) -> bool {
        c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}'
    }
}

mod base {
    pub type Uc32 = char;
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new(data: Vec<T>) -> Self {
            Vector { data }
        }

        pub fn begin(&self) -> &[T] {
            &self.data
        }

        pub fn length(&self) -> usize {
            self.data.len()
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenValue {
    kAsync,
    kAwait,
    kBreak,
    kCase,
    kCatch,
    kClass,
    kConst,
    kContinue,
    kDebugger,
    kDefault,
    kDelete,
    kDo,
    kElse,
    kEnum,
    kExport,
    kExtends,
    kFalseLiteral,
    kFinally,
    kFor,
    kFunction,
    kGet,
    kIf,
    kImplements, // kFutureStrictReservedWord
    kImport,
    kIn,
    kInstanceOf,
    kInterface, // kFutureStrictReservedWord
    kLet,
    kNew,
    kNullLiteral,
    kOf,
    kPackage,    // kFutureStrictReservedWord
    kPrivate,    // kFutureStrictReservedWord
    kProtected,  // kFutureStrictReservedWord
    kPublic,     // kFutureStrictReservedWord
    kReturn,
    kSet,
    kStatic,
    kSuper,
    kSwitch,
    kThis,
    kThrow,
    kTrueLiteral,
    kTry,
    kTypeOf,
    kUsing,
    kVar,
    kVoid,
    kWhile,
    kWith,
    kYield,
    kLeftParen,
    kRightParen,
    kLeftBrace,
    kRightBrace,
    kLeftBracket,
    kRightBracket,
    kConditional,
    kColon,
    kSemicolon,
    kComma,
    kPeriod,
    kBitOr,
    kBitAnd,
    kBitXor,
    kBitNot,
    kNot,
    kLessThan,
    kGreaterThan,
    kMod,
    kAssign,
    kAdd,
    kSub,
    kMul,
    kDiv,
    kPrivateName,
    kString,
    kTemplateSpan,
    kWhitespace,
    kNumber,
    kIdentifier,
    kIllegal,
    kQuestionPeriod,
    kAssignNullish,
    kNullish,
    kLessThanEq,
    kAssignShl,
    kShl,
    kGreaterThanEq,
    kAssignSar,
    kSar,
    kAssignShr,
    kShr,
    kEqStrict,
    kEq,
    kArrow,
    kNotEqStrict,
    kNotEq,
    kInc,
    kAssignAdd,
    kDec,
    kAssignSub,
    kAssignExp,
    kExp,
    kAssignMul,
    kAssignMod,
    kAssignDiv,
    kAssignAnd,
    kAnd,
    kAssignBitAnd,
    kAssignOr,
    kOr,
    kAssignBitOr,
    kAssignBitXor,
    kEllipsis,
    kEos,
}

impl TokenValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenValue::kAsync => "async",
            TokenValue::kAwait => "await",
            TokenValue::kBreak => "break",
            TokenValue::kCase => "case",
            TokenValue::kCatch => "catch",
            TokenValue::kClass => "class",
            TokenValue::kConst => "const",
            TokenValue::kContinue => "continue",
            TokenValue::kDebugger => "debugger",
            TokenValue::kDefault => "default",
            TokenValue::kDelete => "delete",
            TokenValue::kDo => "do",
            TokenValue::kElse => "else",
            TokenValue::kEnum => "enum",
            TokenValue::kExport => "export",
            TokenValue::kExtends => "extends",
            TokenValue::kFalseLiteral => "false",
            TokenValue::kFinally => "finally",
            TokenValue::kFor => "for",
            TokenValue::kFunction => "function",
            TokenValue::kGet => "get",
            TokenValue::kIf => "if",
            TokenValue::kImplements => "implements",
            TokenValue::kImport => "import",
            TokenValue::kIn => "in",
            TokenValue::kInstanceOf => "instanceof",
            TokenValue::kInterface => "interface",
            TokenValue::kLet => "let",
            TokenValue::kNew => "new",
            TokenValue::kNullLiteral => "null",
            TokenValue::kOf => "of",
            TokenValue::kPackage => "package",
            TokenValue::kPrivate => "private",
            TokenValue::kProtected => "protected",
            TokenValue::kPublic => "public",
            TokenValue::kReturn => "return",
            TokenValue::kSet => "set",
            TokenValue::kStatic => "static",
            TokenValue::kSuper => "super",
            TokenValue::kSwitch => "switch",
            TokenValue::kThis => "this",
            TokenValue::kThrow => "throw",
            TokenValue::kTrueLiteral => "true",
            TokenValue::kTry => "try",
            TokenValue::kTypeOf => "typeof",
            TokenValue::kUsing => "using",
            TokenValue::kVar => "var",
            TokenValue::kVoid => "void",
            TokenValue::kWhile => "while",
            TokenValue::kWith => "with",
            TokenValue::kYield => "yield",
            _ => unreachable!(),
        }
    }
}

// static_assert(sizeof(Token::Value) == 1);
const _ASSERT_TOKEN_VALUE_SIZE: () = assert!(std::mem::size_of::<TokenValue>() == 1);

mod keywords_gen {
    use super::TokenValue;

    pub struct PerfectKeywordHash;

    impl PerfectKeywordHash {
        pub fn get_token(input: &str, input_length: usize) -> TokenValue {
             match input {
                "async" => TokenValue::kAsync,
                "await" => TokenValue::kAwait,
                "break" => TokenValue::kBreak,
                "case" => TokenValue::kCase,
                "catch" => TokenValue::kCatch,
                "class" => TokenValue::kClass,
                "const" => TokenValue::kConst,
                "continue" => TokenValue::kContinue,
                "debugger" => TokenValue::kDebugger,
                "default" => TokenValue::kDefault,
                "delete" => TokenValue::kDelete,
                "do" => TokenValue::kDo,
                "else" => TokenValue::kElse,
                "enum" => TokenValue::kEnum,
                "export" => TokenValue::kExport,
                "extends" => TokenValue::kExtends,
                "false" => TokenValue::kFalseLiteral,
                "finally" => TokenValue::kFinally,
                "for" => TokenValue::kFor,
                "function" => TokenValue::kFunction,
                "get" => TokenValue::kGet,
                "if" => TokenValue::kIf,
                "implements" => TokenValue::kImplements,
                "import" => TokenValue::kImport,
                "in" => TokenValue::kIn,
                "instanceof" => TokenValue::kInstanceOf,
                "interface" => TokenValue::kInterface,
                "let" => TokenValue::kLet,
                "new" => TokenValue::kNew,
                "null" => TokenValue::kNullLiteral,
                "of" => TokenValue::kOf,
                "package" => TokenValue::kPackage,
                "private" => TokenValue::kPrivate,
                "protected" => TokenValue::kProtected,
                "public" => TokenValue::kPublic,
                "return" => TokenValue::kReturn,
                "set" => TokenValue::kSet,
                "static" => TokenValue::kStatic,
                "super" => TokenValue::kSuper,
                "switch" => TokenValue::kSwitch,
                "this" => TokenValue::kThis,
                "throw" => TokenValue::kThrow,
                "true" => TokenValue::kTrueLiteral,
                "try" => TokenValue::kTry,
                "typeof" => TokenValue::kTypeOf,
                "using" => TokenValue::kUsing,
                "var" => TokenValue::kVar,
                "void" => TokenValue::kVoid,
                "while" => TokenValue::kWhile,
                "with" => TokenValue::kWith,
                "yield" => TokenValue::kYield,
                _ => TokenValue::kIdentifier,
            }
        }
    }
}

mod char_predicates {
    pub fn is_decimal_digit(c: char) -> bool {
        c.is_digit(10)
    }

    pub fn is_ascii_identifier(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_' || c == '$'
    }
}

pub mod scanner {
    use super::*;
    use char_predicates::*;
    use keywords_gen::PerfectKeywordHash;

    const K_MAX_ASCII: usize = 127;
    const INT_0_TO_127_LIST: [char; 128] = [
    '\x00', '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07', '\x08', '\x09', '\x0a', '\x0b', '\x0c', '\x0d', '\x0e', '\x0f',
    '\x10', '\x11', '\x12', '\x13', '\x14', '\x15', '\x16', '\x17', '\x18', '\x19', '\x1a', '\x1b', '\x1c', '\x1d', '\x1e', '\x1f',
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_',
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '\x7f',
];

    macro_rules! keywords {
        ($keyword_group:ident, $keyword:ident) => {
            $keyword_group!('a');
            $keyword!("async", TokenValue::kAsync);
            $keyword!("await", TokenValue::kAwait);
            $keyword_group!('b');
            $keyword!("break", TokenValue::kBreak);
            $keyword_group!('c');
            $keyword!("case", TokenValue::kCase);
            $keyword!("catch", TokenValue::kCatch);
            $keyword!("class", TokenValue::kClass);
            $keyword!("const", TokenValue::kConst);
            $keyword!("continue", TokenValue::kContinue);
            $keyword_group!('d');
            $keyword!("debugger", TokenValue::kDebugger);
            $keyword!("default", TokenValue::kDefault);
            $keyword!("delete", TokenValue::kDelete);
            $keyword!("do", TokenValue::kDo);
            $keyword_group!('e');
            $keyword!("else", TokenValue::kElse);
            $keyword!("enum", TokenValue::kEnum);
            $keyword!("export", TokenValue::kExport);
            $keyword!("extends", TokenValue::kExtends);
            $keyword_group!('f');
            $keyword!("false", TokenValue::kFalseLiteral);
            $keyword!("finally", TokenValue::kFinally);
            $keyword!("for", TokenValue::kFor);
            $keyword!("function", TokenValue::kFunction);
            $keyword_group!('g');
            $keyword!("get", TokenValue::kGet);
            $keyword_group!('i');
            $keyword!("if", TokenValue::kIf);
            $keyword!("implements", TokenValue::kImplements);
            $keyword!("import", TokenValue::kImport);
            $keyword!("in", TokenValue::kIn);
            $keyword!("instanceof", TokenValue::kInstanceOf);
            $keyword!("interface", TokenValue::kInterface);
            $keyword_group!('l');
            $keyword!("let", TokenValue::kLet);
            $keyword_group!('n');
            $keyword!("new", TokenValue::kNew);
            $keyword!("null", TokenValue::kNullLiteral);
            $keyword_group!('o');
            $keyword!("of", TokenValue::kOf);
            $keyword_group!('p');
            $keyword!("package", TokenValue::kPackage);
            $keyword!("private", TokenValue::kPrivate);
            $keyword!("protected", TokenValue::kProtected);
            $keyword!("public", TokenValue::kPublic);
            $keyword_group!('r');
            $keyword!("return", TokenValue::kReturn);
            $keyword_group!('s');
            $keyword!("set", TokenValue::kSet);
            $keyword!("static", TokenValue::kStatic);
            $keyword!("super", TokenValue::kSuper);
            $keyword!("switch", TokenValue::kSwitch);
            $keyword_group!('t');
            $keyword!("this", TokenValue::kThis);
            $keyword!("throw", TokenValue::kThrow);
            $keyword!("true", TokenValue::kTrueLiteral);
            $keyword!("try", TokenValue::kTry);
            $keyword!("typeof", TokenValue::kTypeOf);
            $keyword_group!('u');
            $keyword!("using", TokenValue::kUsing);
            $keyword_group!('v');
            $keyword!("var", TokenValue::kVar);
            $keyword!("void", TokenValue::kVoid);
            $keyword_group!('w');
            $keyword!("while", TokenValue::kWhile);
            $keyword!("with", TokenValue::kWith);
            $keyword_group!('y');
            $keyword!("yield", TokenValue::kYield);
        };
    }

    const fn is_keyword_start(c: char) -> bool {
        macro_rules! keyword_group_check {
            ($ch:expr) => {
                $ch == c ||
            };
        }
        macro_rules! keyword_check {
            ($keyword:expr, $token:expr) => {};
        }

        keywords!(keyword_group_check, keyword_check);

        false
    }

    const fn can_be_keyword_character(c: char) -> bool {
        let keyword_string = {
            macro_rules! keyword_group_case {
                ($ch:expr) => {};
            }

            macro_rules! keyword {
                ($keyword:expr, $token:expr) => {
                    $keyword
                };
            }

            let mut result = "";

            macro_rules! keywords_inner {
                ($keyword_group:ident, $keyword:ident) => {
                    $keyword_group!('a');
                    result = concat!(result, $keyword!("async", TokenValue::kAsync));
                    result = concat!(result, $keyword!("await", TokenValue::kAwait));
                    $keyword_group!('b');
                    result = concat!(result, $keyword!("break", TokenValue::kBreak));
                    $keyword_group!('c');
                    result = concat!(result, $keyword!("case", TokenValue::kCase));
                    result = concat!(result, $keyword!("catch", TokenValue::kCatch));
                    result = concat!(result, $keyword!("class", TokenValue::kClass));
                    result = concat!(result, $keyword!("const", TokenValue::kConst));
                    result = concat!(result, $keyword!("continue", TokenValue::kContinue));
                    $keyword_group!('d');
                    result = concat!(result, $keyword!("debugger", TokenValue::kDebugger));
                    result = concat!(result, $keyword!("default", TokenValue::kDefault));
                    result = concat!(result, $keyword!("delete", TokenValue::kDelete));
                    result = concat!(result, $keyword!("do", TokenValue::kDo));
                    $keyword_group!('e');
                    result = concat!(result, $keyword!("else", TokenValue::kElse));
                    result = concat!(result, $keyword!("enum", TokenValue::kEnum));
                    result = concat!(result, $keyword!("export", TokenValue::kExport));
                    result = concat!(result, $keyword!("extends", TokenValue::kExtends));
                    $keyword_group!('f');
                    result = concat!(result, $keyword!("false", TokenValue::kFalseLiteral));
                    result = concat!(result, $keyword!("finally", TokenValue::kFinally));
                    result = concat!(result, $keyword!("for", TokenValue::kFor));
                    result = concat!(result, $keyword!("function", TokenValue::kFunction));
                    $keyword_group!('g');
                    result = concat!(result, $keyword!("get", TokenValue::kGet));
                    $keyword_group!('i');
                    result = concat!(result, $keyword!("if", TokenValue::kIf));
                    result = concat!(result, $keyword!("implements", TokenValue::kImplements));
                    result = concat!(result, $keyword!("import", TokenValue::kImport));
                    result = concat!(result, $keyword!("in", TokenValue::kIn));
                    result = concat!(result, $keyword!("instanceof", TokenValue::kInstanceOf));
                    result = concat!(result, $keyword!("interface", TokenValue::kInterface));
                    $keyword_group!('l');
                    result = concat!(result, $keyword!("let", TokenValue::kLet));
                    $keyword_group!('n');
                    result = concat!(result, $keyword!("new", TokenValue::kNew));
                    result = concat!(result, $keyword!("null", TokenValue::kNullLiteral));
                    $keyword_group!('o');
                    result = concat!(result, $keyword!("of", TokenValue::kOf));
                    $keyword_group!('p');
                    result = concat!(result, $keyword!("package", TokenValue::kPackage));
                    result = concat!(result, $keyword!("private", TokenValue::kPrivate));
                    result = concat!(result, $keyword!("protected", TokenValue::kProtected));
                    result = concat!(result, $keyword!("public", TokenValue::kPublic));
                    $keyword_group!('r');
                    result = concat!(result, $keyword!("return", TokenValue::kReturn));
                    $keyword_group!('s');
                    result = concat!(result, $keyword!("set", TokenValue::kSet));
                    result = concat!(result, $keyword!("static", TokenValue::kStatic));
                    result = concat!(result, $keyword!("super", TokenValue::kSuper));
                    result = concat!(result, $keyword!("switch", TokenValue::kSwitch));
                    $keyword_group!('t');
                    result = concat!(result, $keyword!("this", TokenValue::kThis));
                    result = concat!(result, $keyword!("throw", TokenValue::kThrow));
                    result = concat!(result, $keyword!("true", TokenValue::kTrueLiteral));
                    result = concat!(result, $keyword!("try", TokenValue::kTry));
                    result = concat!(result, $keyword!("typeof", TokenValue::kTypeOf));
                    $keyword_group!('u');
                    result = concat!(result, $keyword!("using", TokenValue::kUsing));
                    $keyword_group!('v');
                    result = concat!(result, $keyword!("var", TokenValue::kVar));
                    result = concat!(result, $keyword!("void", TokenValue::kVoid));
                    $keyword_group!('w');
                    result = concat!(result, $keyword!("while", TokenValue::kWhile));
                    result = concat!(result, $keyword!("with", TokenValue::kWith));
                    $keyword_group!('y');
                    result = concat!(result, $keyword!("yield", TokenValue::kYield));
                };
            }
            keywords_inner!(keyword_group_case, keyword);

            result
        };
        keyword_string.contains(c)
    }

    const fn get_one_char_token(c: char) -> TokenValue {
        match c {
            '(' => TokenValue::kLeftParen,
            ')' => TokenValue::kRightParen,
            '{' => TokenValue::kLeftBrace,
            '}' => TokenValue::kRightBrace,
            '[' => TokenValue::kLeftBracket,
            ']' => TokenValue::kRightBracket,
            '?' => TokenValue::kConditional,
            ':' => TokenValue::kColon,
            ';' => TokenValue::kSemicolon,
            ',' => TokenValue::kComma,
            '.' => TokenValue::kPeriod,
            '|' => TokenValue::kBitOr,
            '&' => TokenValue::kBitAnd,
            '^' => TokenValue::kBitXor,
            '~' => TokenValue::kBitNot,
            '!' => TokenValue::kNot,
            '<' => TokenValue::kLessThan,
            '>' => TokenValue::kGreaterThan,
            '%' => TokenValue::kMod,
            '=' => TokenValue::kAssign,
            '+' => TokenValue::kAdd,
            '-' => TokenValue::kSub,
            '*' => TokenValue::kMul,
            '/' => TokenValue::kDiv,
            '#' => TokenValue::kPrivateName,
            '"' => TokenValue::kString,
            '\'' => TokenValue::kString,
            '`' => TokenValue::kTemplateSpan,
            '\\' => TokenValue::kIdentifier,
            ' ' | '\t' | '\x0b' | '\x0c' | '\r' | '\n' => TokenValue::kWhitespace,
            c if is_decimal_digit(c) => TokenValue::kNumber,
            c if is_ascii_identifier(c) => TokenValue::kIdentifier,
            _ => TokenValue::kIllegal,
        }
    }

    const ONE_CHAR_TOKENS: [TokenValue; 128] = {
        let mut tokens = [TokenValue::kIllegal; 128];
        let mut i = 0;
        while i < 128 {
            tokens[i] = get_one_char_token(INT_0_TO_127_LIST[i]);
            i += 1;
        }
        tokens
    };

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    enum ScanFlags {
        TerminatesLiteral = 1 << 0,
        CannotBeKeyword = 1 << 1,
        CannotBeKeywordStart = 1 << 2,
        StringTerminator = 1 << 3,
        IdentifierNeedsSlowPath = 1 << 4,
        MultilineCommentCharacterNeedsSlowPath = 1 << 5,
    }

    const fn get_scan_flags(c: char) -> u8 {
        let mut flags: u8 = 0;

        if is_ascii_identifier(c) && !can_be_keyword_character(c) {
            flags |= ScanFlags::CannotBeKeyword as u8;
        }

        if !is_keyword_start(c) {
            flags |= ScanFlags::CannotBeKeywordStart as u8;
        }

        if !is_ascii_identifier(c) {
            flags |= ScanFlags::TerminatesLiteral as u8;
        }

        if c == '\'' || c == '"' || c == '\n' || c == '\r' || c == '\\' {
            flags |= ScanFlags::StringTerminator as u8;
        }

        if c == '\\' {
            flags |= ScanFlags::IdentifierNeedsSlowPath as u8;
        }

        if c == '\n' || c == '\r' || c == '*' {
            flags |= ScanFlags::MultilineCommentCharacterNeedsSlowPath as u8;
        }

        flags
    }

    const fn terminates_literal(scan_flags: u8) -> bool {
        (scan_flags & (ScanFlags::TerminatesLiteral as u8)) != 0
    }

    const fn can_be_keyword(scan_flags: u8) -> bool {
        (scan_flags & (ScanFlags::CannotBeKeyword as u8)) == 0
    }

    const fn identifier_needs_slow_path(scan_flags: u8) -> bool {
        (scan_flags & (ScanFlags::IdentifierNeedsSlowPath as u8)) != 0
    }

    const fn multiline_comment_character_needs_slow_path(scan_flags: u8) -> bool {
        (scan_flags & (ScanFlags::MultilineCommentCharacterNeedsSlowPath as u8)) != 0
    }

    const fn may_terminate_string(scan_flags: u8) -> bool {
        (scan_flags & (ScanFlags::StringTerminator as u8)) != 0
    }

    const CHARACTER_SCAN_FLAGS: [u8; 128] = {
        let mut flags = [0u8; 128];
        let mut i = 0;
        while i < 128 {
            flags[i] = get_scan_flags(INT_0_TO_127_LIST[i]);
            i += 1;
        }
        flags
    };

    fn char_can_be_keyword(c: char) -> bool {
        (c as usize) < CHARACTER_SCAN_FLAGS.len() && can_be_keyword(CHARACTER_SCAN_FLAGS[c as usize])
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Location {
        beg_pos: usize,
        end_pos: usize,
    }

    impl Location {
        pub fn new(beg_pos: usize, end_pos: usize) -> Self {
            Location { beg_pos, end_pos }
        }
    }

    pub struct LiteralChars {
        buffer: String,
    }

    impl LiteralChars {
        pub fn new() -> Self {
            LiteralChars {
                buffer: String::new(),
            }
        }

        pub fn start(&mut self) {
            self.buffer.clear();
        }

        pub fn add_char(&mut self, c: char) {
            self.buffer.push(c);
        }

        pub fn one_byte_literal(&self) -> base::Vector<u8> {
            let bytes = self.buffer.as_bytes().to_vec();
            base::Vector::new(bytes)
        }

        pub fn as_str(&self) -> &str {
            &self.buffer
        }

        pub fn len(&self) -> usize {
            self.buffer.len()
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct TokenDesc {
        pub token: TokenValue,
        pub location: Location,
    }

    impl TokenDesc {
        pub fn new() -> Self {
            TokenDesc {
                token: TokenValue::kIllegal,
                location: Location::new(0, 0),
            }
        }
    }

    pub struct Scanner<'source> {
        source_: &'source str,
        pos_: usize,
        c0_: char,
        next_: TokenDesc,
        current_: TokenDesc,
        next_next_: TokenDesc,
        next_next_next_: TokenDesc,
        source_pos_: usize,
        saw_non_comment_: bool,
        has_parser_error_: bool,
        literal_chars: LiteralChars,
        after_line_terminator_: bool,
    }

    impl<'source> Scanner<'source> {
        pub fn new(source: &'source str) -> Self {
            let mut scanner = Scanner {
                source_: source,
                pos_: 0,
                c0_: '\0',
                next_: TokenDesc::new(),
                current_: TokenDesc::new(),
                next_next_: TokenDesc::new(),
                next_next_next_: TokenDesc::new(),
                source_pos_: 0,
                saw_non_comment_: false,
                has_parser_error_: false,
                literal_chars: LiteralChars::new(),
                after_line_terminator_: false,
            };
            scanner.advance();
            scanner
        }

        fn advance(&mut self) {
            if self.pos_ < self.source_.len() {
                let ch = self.source_[self.pos_..].chars().next().unwrap();
                self.c0_ = ch;
                self.source_pos_ += ch.len_utf8();
                self.pos_ += ch.len_utf8();
            } else {
                self.c0_ = '\0'; // End of input
            }
        }

        fn peek(&self) -> char {
            if self.pos_ < self.source_.len() {
                self.source_[self.pos_..].chars().next().unwrap()
            } else {
                '\0'
            }
        }

        fn push_back(&mut self, c: char) {
            // This is a simplified version, assuming we only push back one character at a time
            self.pos_ -= c.len_utf8();
            self.source_pos_ -= c.len_utf8();
            self.c0_ = c;
        }

        fn select(&mut self, token: TokenValue) -> TokenValue {
            self.advance();
            token
        }

        fn select_char(&mut self, expected: char, token1: TokenValue, token2: TokenValue) -> TokenValue {
            if self.c0_ == expected {
                self.advance();
                token1
            } else {
                token2
            }
        }

        fn select(&mut self, expected: char, token1: TokenValue, token2: TokenValue) -> TokenValue {
            if self.c0_ == expected {
                self.advance();
                token1
            } else {
                token2
            }
        }

        fn source_pos(&self) -> usize {
            self.source_pos_
        }

        fn has_parser_error(&self) -> bool {
            self.has_parser_error_
        }

        fn is_identifier_start(&self, c: char) -> bool {
            c.is_alphabetic() || c == '_' || c == '$'
        }

        fn combine_surrogate_pair(&self) -> bool {
            false // Placeholder, needs proper implementation for surrogate pairs
        }

        fn add_literal_char(&mut self, c: char) {
            self.literal_chars.add_char(c);
        }

        fn advance_until<F>(&mut self, mut predicate: F)
        where
            F: FnMut(char) -> bool,
        {
            while self.pos_ < self.source_.len() {
                let c0 = self.source_[self.pos_..].chars().next().unwrap();
                if predicate(c0) {
                    break;
                }
                self.advance();
            }
        }

        fn is_white_space_or_line_terminator(c: char) -> bool {
            c.is_whitespace() || unibrow::is_line_terminator(c)
        }

        fn scan_number(&mut self, _is_period: bool) -> TokenValue {
             TokenValue::kNumber // Placeholder, needs proper implementation for number scanning
        }

        fn scan_template_span(&mut self) -> TokenValue {
             TokenValue::kTemplateSpan // Placeholder, needs proper implementation for template span scanning
        }

        fn scan_private_name(&mut self) -> TokenValue {
             TokenValue::kPrivateName // Placeholder, needs proper implementation for private name scanning
        }

        fn skip_single_line_comment(&mut self) -> TokenValue {
            self.advance_until(|c| c == '\n' || c == '\r' || self.pos_ >= self.source_.len());
            TokenValue::kWhitespace // Treat comment as whitespace
        }

        fn skip_multi_line_comment(&mut self) -> TokenValue {
            let mut prev_char = '\0';
            self.advance_until(|c| {
                let end = prev_char == '*' && c == '/';
                prev_char = c;
                end || self.pos_ >= self.source_.len()
            });
            if self.pos_ < self.source_.len() {
                self.advance(); // Consume the '/'
            }
            TokenValue::kWhitespace // Treat comment as whitespace
        }

        fn scan_string(&mut self) -> TokenValue {
            TokenValue::kString // Placeholder
        }

        fn scan_html_comment(&mut self) -> TokenValue {
            TokenValue::kWhitespace // Placeholder
        }

        fn skip_single_html_comment(&mut self) -> TokenValue {
             TokenValue::kWhitespace // Placeholder
        }

        fn skip_magic_comment(&mut self, _c: char) -> TokenValue {
            TokenValue::kWhitespace // Placeholder