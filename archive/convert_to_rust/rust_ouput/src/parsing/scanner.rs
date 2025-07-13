// Converted from V8 C++ source files:
// Header: scanner.h
// Implementation: scanner.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod strings {
        }
}
pub mod common {
pub mod message_template {
        }
}
pub mod parsing {
pub mod literal_buffer {
        }
pub mod parse_info {
        }
pub mod token {
        }
}
pub mod regexp {
pub mod regexp_flags {
        }
}
pub mod strings {
pub mod char_predicates {
        }
pub mod unicode {
        }
}
pub mod utils {
pub mod allocation {
        }
}
pub mod ast {
pub mod ast_value_factory {
        }
pub mod ast_raw_string {
        }
}
use std::cmp;
use std::fmt;
use std::i32;
use std::marker::PhantomData;
use std::mem;
use std::ops::{BitAnd, BitOr, Not};
use std::ptr;
use std::slice;
use std::string::String as StdString;
use std::sync::{Arc, Mutex, RwLock};
pub struct AstRawString {}
pub struct AstValueFactory {}
pub struct ExternalOneByteString {}
pub struct ExternalTwoByteString {}
pub struct ParserRecorder {}
pub struct RuntimeCallStats {}
pub struct Zone {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenValue {
    KUninitialized,
    KIllegal,
    KEscapedKeyword,
    KNumber,
    KString,
    KIdentifier,
    KYield,
    KPrivateName,
    KRegExpLiteral,
    KTemplateSpan,
    KTemplateTail,
    KFutureStrictReservedWord,
    KLet,
    KStatic,
    KDiv,
    KAssignDiv,
    KWhiteSpace,
    KSmi,
    KLessThan,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageTemplate {
    KNone,
    KInvalidUnicodeEscapeSequence,
    KInvalidHexEscapeSequence,
    KUndefinedUnicodeCodePoint,
    KBigIntTooBig,
    KStrictOctalEscape,
    KStrictDecimalWithLeadingZero,
    KZeroDigitNumericSeparator,
    KContinuousNumericSeparator,
    KTrailingNumericSeparator,
    KInvalidOrUnexpectedToken,
    KHtmlCommentInModule,
    KTemplate8Or9Escape,
    KTemplateOctalLiteral,
    KStrict8Or9Escape,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegExpFlags {}
pub mod unibrow {
pub mod Utf16 {
        }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnoptimizedCompileFlags {}
pub struct Utf16CharacterStream {
    buffer_start_: *const u16,
    buffer_cursor_: *const u16,
    buffer_end_: *const u16,
    buffer_pos_: usize,
    runtime_call_stats_: *mut RuntimeCallStats,
    has_parser_error_: bool,
}
impl Utf16CharacterStream {
    pub const K_END_OF_INPUT: i32 = -1;
    pub fn new(
        buffer_start: *const u16,
        buffer_cursor: *const u16,
        buffer_end: *const u16,
        buffer_pos: usize,
    ) -> Self {
        Utf16CharacterStream {
            buffer_start_: buffer_start,
            buffer_cursor_: buffer_cursor,
            buffer_end_: buffer_end,
            buffer_pos_: buffer_pos,
            runtime_call_stats_: ptr::null_mut(),
            has_parser_error_: false,
        }
    }
    pub fn set_parser_error(&mut self) {
        self.buffer_cursor_ = unsafe { self.buffer_end_.add(1) };
        self.has_parser_error_ = true;
    }
    pub fn reset_parser_error_flag(&mut self) {
        self.has_parser_error_ = false;
    }
    pub fn has_parser_error(&self) -> bool {
        self.has_parser_error_
    }
    pub fn peek(&mut self) -> i32 {
        if self.buffer_cursor_ < self.buffer_end_ {
            unsafe { *self.buffer_cursor_ as i32 }
        } else if self.read_block_checked(self.pos()) {
            unsafe { *self.buffer_cursor_ as i32 }
        } else {
            Utf16CharacterStream::K_END_OF_INPUT
        }
    }
    pub fn advance(&mut self) -> i32 {
        let result = self.peek();
        self.buffer_cursor_ = unsafe { self.buffer_cursor_.add(1) };
        result
    }
    pub fn advance_until<F>(&mut self, check: F) -> i32
    where
        F: Fn(i32) -> bool,
    {
        loop {
            let next_cursor_pos = unsafe {
                slice::from_raw_parts(self.buffer_cursor_, self.buffer_end_ as usize - self.buffer_cursor_ as usize)
                    .iter()
                    .position(|&raw_c0_| {
                        let c0 = raw_c0_ as i32;
                        check(c0)
                    })
            };

            match next_cursor_pos {
                Some(offset) => {
                    self.buffer_cursor_ = unsafe { self.buffer_cursor_.add(offset + 1) };
                    return unsafe { *self.buffer_cursor_.offset(-(1 as isize)) as i32 };
                }
                None => {
                    self.buffer_cursor_ = self.buffer_end_;
                    if !self.read_block_checked(self.pos()) {
                        self.buffer_cursor_ = unsafe { self.buffer_cursor_.add(1) };
                        return Utf16CharacterStream::K_END_OF_INPUT;
                    }
                }
            }
        }
    }
    pub fn back(&mut self) {
        if self.buffer_cursor_ > self.buffer_start_ {
            self.buffer_cursor_ = unsafe { self.buffer_cursor_.offset(-1) };
        } else {
            self.read_block_checked(self.pos() - 1);
        }
    }
    pub fn pos(&self) -> usize {
        unsafe {
            self.buffer_pos_ + (self.buffer_cursor_.offset_from(self.buffer_start_) as usize)
        }
    }
    pub fn seek(&mut self, pos: usize) {
        if pos >= self.buffer_pos_
            && pos < (self.buffer_pos_ + (unsafe { self.buffer_end_.offset_from(self.buffer_start_) } as usize))
        {
            unsafe {
                self.buffer_cursor_ = self.buffer_start_.add(pos - self.buffer_pos_);
            }
        } else {
            self.read_block_checked(pos);
        }
    }
    pub fn can_be_cloned_for_parallel_access(&self) -> bool {
        self.can_be_cloned() && !self.can_access_heap()
    }
    pub fn runtime_call_stats(&self) -> *mut RuntimeCallStats {
        self.runtime_call_stats_
    }
    pub fn set_runtime_call_stats(&mut self, runtime_call_stats: *mut RuntimeCallStats) {
        self.runtime_call_stats_ = runtime_call_stats;
    }
    fn read_block_checked(&mut self, position: usize) -> bool {
        let success = !self.has_parser_error() && self.read_block(position);
        assert_eq!(self.pos(), position);
        assert!(self.buffer_cursor_ <= self.buffer_end_);
        assert!(self.buffer_start_ <= self.buffer_cursor_);
        assert_eq!(success, self.buffer_cursor_ < self.buffer_end_);
        success
    }
    fn read_block(&mut self, position: usize) -> bool {
        false
    }
    fn can_be_cloned(&self) -> bool {
      false
    }
    fn can_access_heap(&self) -> bool {
      false
    }
}
pub struct Location {
    beg_pos: i32,
    end_pos: i32,
}
impl Location {
    pub fn new(b: i32, e: i32) -> Self {
        Location { beg_pos: b, end_pos: e }
    }
    pub fn default() -> Self {
        Location {
            beg_pos: 0,
            end_pos: 0,
        }
    }
    pub fn length(&self) -> i32 {
        self.end_pos - self.beg_pos
    }
    pub fn is_valid(&self) -> bool {
        self.beg_pos >= 0 && self.beg_pos <= self.end_pos
    }
    pub fn invalid() -> Self {
        Location {
            beg_pos: -1,
            end_pos: 0,
        }
    }
}
pub struct Scanner {
    flags_: UnoptimizedCompileFlags,
    source_: *mut Utf16CharacterStream,
    c0_: i32,
    token_storage_: [TokenDesc; 4],
    current_: *mut TokenDesc,
    next_: *mut TokenDesc,
    next_next_: *mut TokenDesc,
    next_next_next_: *mut TokenDesc,
    found_html_comment_: bool,
    source_url_: LiteralBuffer,
    source_mapping_url_: LiteralBuffer,
    saw_source_mapping_url_magic_comment_at_sign_: bool,
    saw_magic_comment_compile_hints_all_: bool,
    saw_non_comment_: bool,
    per_function_compile_hint_positions_: Vec<i32>,
    per_function_compile_hint_positions_idx_: usize,
    octal_pos_: Location,
    octal_message_: MessageTemplate,
    scanner_error_: MessageTemplate,
    scanner_error_location_: Location,
}
pub struct TokenDesc {
    location: Location,
    literal_chars: LiteralBuffer,
    raw_literal_chars: LiteralBuffer,
    token: TokenValue,
    invalid_template_escape_message: MessageTemplate,
    invalid_template_escape_location: Location,
    number_kind: NumberKind,
    smi_value: u32,
    after_line_terminator: bool,
}
impl TokenDesc {
    fn new() -> Self {
        TokenDesc {
            location: Location::default(),
            literal_chars: LiteralBuffer::new(),
            raw_literal_chars: LiteralBuffer::new(),
            token: TokenValue::KUninitialized,
            invalid_template_escape_message: MessageTemplate::KNone,
            invalid_template_escape_location: Location::invalid(),
            number_kind: NumberKind::DECIMAL,
            smi_value: 0,
            after_line_terminator: false,
        }
    }
    fn can_access_literal(&self) -> bool {
        self.token == TokenValue::KPrivateName
            || self.token == TokenValue::KIllegal
            || self.token == TokenValue::KEscapedKeyword
            || self.token == TokenValue::KUninitialized
            || self.token == TokenValue::KRegExpLiteral
            || self.token == TokenValue::KNumber
            || self.token == TokenValue::KString
            || self.token == TokenValue::KIdentifier
            || matches!(
                self.token,
                TokenValue::KIdentifier | TokenValue::KYield | TokenValue::KTemplateSpan | TokenValue::KTemplateTail
            )
    }
    fn can_access_raw_literal(&self) -> bool {
        self.token == TokenValue::KIllegal
            || self.token == TokenValue::KUninitialized
            || self.token == TokenValue::KTemplateSpan
            || self.token == TokenValue::KTemplateTail
    }
}
impl Scanner {
    const K_END_OF_INPUT: i32 = Utf16CharacterStream::K_END_OF_INPUT;
    const K_INVALID_SEQUENCE: i32 = -1;
    const K_CHARACTER_LOOKAHEAD_BUFFER_SIZE: usize = 1;
    const K_MAX_ASCII: i32 = 127;
    fn is_invalid(c: i32) -> bool {
        c == Scanner::K_INVALID_SEQUENCE
    }
    pub fn new(source: *mut Utf16CharacterStream, flags: UnoptimizedCompileFlags) -> Scanner {
        Scanner {
            flags_: flags,
            source_: source,
            c0_: 0,
            token_storage_: [
                TokenDesc::new(),
                TokenDesc::new(),
                TokenDesc::new(),
                TokenDesc::new(),
            ],
            current_: ptr::null_mut(),
            next_: ptr::null_mut(),
            next_next_: ptr::null_mut(),
            next_next_next_: ptr::null_mut(),
            found_html_comment_: false,
            source_url_: LiteralBuffer::new(),
            source_mapping_url_: LiteralBuffer::new(),
            saw_source_mapping_url_magic_comment_at_sign_: false,
            saw_magic_comment_compile_hints_all_: false,
            saw_non_comment_: false,
            per_function_compile_hint_positions_: Vec::new(),
            per_function_compile_hint_positions_idx_: 0,
            octal_pos_: Location::invalid(),
            octal_message_: MessageTemplate::KNone,
            scanner_error_: MessageTemplate::KNone,
            scanner_error_location_: Location::invalid(),
        }
    }
    pub fn initialize(&mut self) {
        self.init();
        unsafe {
            (*self.next_).after_line_terminator = true;
        }
        self.scan();
    }
    fn init(&mut self) {
        unsafe {
            let source = &mut *self.source_;
            self.c0_ = source.advance();
        }
        self.current_ = &mut self.token_storage_[0];
        self.next_ = &mut self.token_storage_[1];
        self.next_next_ = &mut self.token_storage_[2];
        self.next_next_next_ = &mut self.token_storage_[3];
        self.found_html_comment_ = false;
        self.scanner_error_ = MessageTemplate::KNone;
    }
    fn next(&mut self) -> &mut TokenDesc {
        unsafe { &mut *self.next_ }
    }
    fn scan(&mut self) {}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberKind {
    IMPLICIT_OCTAL,
    BINARY,
    OCTAL,
    HEX,
    DECIMAL,
    DECIMAL_WITH_LEADING_ZERO,
}
pub struct LiteralBuffer {}
impl LiteralBuffer {
    pub fn new() -> Self {
        LiteralBuffer {}
    }
}
