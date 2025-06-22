// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod dateparser {
    use std::i32::MAX as kMaxInt;

    /// Represents the different components of a date.
    #[allow(dead_code)]
    pub enum DateComponent {
        YEAR,
        MONTH,
        DAY,
        HOUR,
        MINUTE,
        SECOND,
        MILLISECOND,
        UTC_OFFSET,
        OUTPUT_SIZE,
    }

    /// Parses a date string into its components.
    pub struct DateParser {}

    impl DateParser {
        /// Parses a date string into its components.
        ///
        /// * `isolate`: An isolate. Not used in the rust version
        /// * `str`: The date string to parse.
        /// * `output`: An array to store the parsed date components.
        ///
        /// Returns `Ok(true)` if parsing succeeds, `Ok(false)` otherwise.
        /// If parsing fails, the content of the output array is not defined.
        pub fn parse<Char: AsRef<[u8]>>(
            _isolate: &(), //Isolate is not used in the Rust implementation
            str: Char,
            output: &mut [f64],
        ) -> bool {
            let str_as_bytes = str.as_ref();
            let input_reader = InputReader::new(str_as_bytes.to_vec());
            let mut tokenizer = DateStringTokenizer::new(&input_reader);
            let mut day = DayComposer::new();
            let mut time = TimeComposer::new();
            let mut tz = TimeZoneComposer::new();
            
            let result = Self::parse_es5_date_time(&mut tokenizer, &mut day, &mut time, &mut tz);

            match result {
                DateTokenResult::Valid => {
                    if day.write(output) && time.write(output) && tz.write(output) {
                        true
                    } else {
                        false
                    }
                }
                DateTokenResult::Invalid => false,
                DateTokenResult::Continue => false // legacy parser is not implemented
            }
        }

        #[inline]
        fn between(x: i32, lo: i32, hi: i32) -> bool {
            (x - lo) as u32 <= (hi - lo) as u32
        }

        const K_NONE: i32 = kMaxInt;
        const K_MAX_SIGNIFICANT_DIGITS: usize = 9;

        /// Provides basic string parsing and character classification.
        pub struct InputReader {
            index_: usize,
            buffer_: Vec<u8>,
            ch_: u32,
        }

        impl InputReader {
            pub fn new(s: Vec<u8>) -> Self {
                let mut reader = InputReader {
                    index_: 0,
                    buffer_: s,
                    ch_: 0,
                };
                reader.next();
                reader
            }

            pub fn position(&self) -> usize {
                self.index_ - 1
            }

            pub fn next(&mut self) {
                self.ch_ = if self.index_ < self.buffer_.len() {
                    self.buffer_[self.index_] as u32
                } else {
                    0
                };
                self.index_ += 1;
            }

            pub fn read_unsigned_numeral(&mut self) -> i32 {
                let mut n: i32 = 0;
                let mut i: usize = 0;

                while self.ch_ == '0' as u32 {
                    self.next();
                }

                while self.is_ascii_digit() {
                    if i < Self::K_MAX_SIGNIFICANT_DIGITS {
                        n = n * 10 + (self.ch_ - '0' as u32) as i32;
                    }
                    i += 1;
                    self.next();
                }
                n
            }

            pub fn read_word(&mut self, prefix: &mut [u32], prefix_size: usize) -> usize {
                let mut len: usize = 0;
                while self.is_ascii_alpha_or_above() && !self.is_white_space_char() {
                    if len < prefix_size {
                        prefix[len] = Self::ascii_alpha_to_lower(self.ch_);
                    }
                    self.next();
                    len += 1;
                }
                for i in len..prefix_size {
                    prefix[i] = 0;
                }
                len
            }

            pub fn skip(&mut self, c: u32) -> bool {
                if self.ch_ == c {
                    self.next();
                    true
                } else {
                    false
                }
            }

            #[inline]
            pub fn skip_white_space(&mut self) -> bool {
                if self.is_white_space_char() {
                    while self.is_white_space_char() {
                        self.next();
                    }
                    true
                } else {
                    false
                }
            }

            #[inline]
            pub fn skip_parentheses(&mut self) -> bool {
                if self.ch_ == '(' as u32 {
                    self.next();
                    let mut count = 1;
                    while !self.is_end() {
                        if self.ch_ == '(' as u32 {
                            count += 1;
                        } else if self.ch_ == ')' as u32 {
                            count -= 1;
                            if count == 0 {
                                self.next();
                                return true;
                            }
                        }
                        self.next();
                    }
                    false
                } else {
                    false
                }
            }

            pub fn is(&self, c: u32) -> bool {
                self.ch_ == c
            }

            pub fn is_end(&self) -> bool {
                self.ch_ == 0
            }

            pub fn is_ascii_digit(&self) -> bool {
                Self::is_decimal_digit(self.ch_)
            }

            pub fn is_ascii_alpha_or_above(&self) -> bool {
                self.ch_ >= 'A' as u32
            }

            pub fn is_white_space_char(&self) -> bool {
                Self::is_white_space(self.ch_)
            }

            pub fn is_ascii_sign(&self) -> bool {
                self.ch_ == '+' as u32 || self.ch_ == '-' as u32
            }

            pub fn get_ascii_sign_value(&self) -> i32 {
                44 - self.ch_ as i32
            }

            fn is_decimal_digit(c: u32) -> bool {
                c >= '0' as u32 && c <= '9' as u32
            }

            fn is_white_space(c: u32) -> bool {
                c == ' ' as u32 || c == '\t' as u32 || c == '\n' as u32 || c == '\r' as u32
                || c == 0x0b || c == 0x0c
            }

            fn ascii_alpha_to_lower(c: u32) -> u32 {
                if c >= 'A' as u32 && c <= 'Z' as u32 {
                    c + 32
                } else {
                    c
                }
            }
        }

        #[derive(Debug, PartialEq, Copy, Clone)]
        enum KeywordType {
            INVALID,
            MONTH_NAME,
            TIME_ZONE_NAME,
            TIME_SEPARATOR,
            AM_PM,
        }

        #[derive(Debug, Copy, Clone)]
        struct DateToken {
            tag_: i32,
            length_: i32,
            value_: i32,
        }

        impl DateToken {
            pub fn is_invalid(&self) -> bool {
                self.tag_ == Self::K_INVALID_TOKEN_TAG
            }

            pub fn is_unknown(&self) -> bool {
                self.tag_ == Self::K_UNKNOWN_TOKEN_TAG
            }

            pub fn is_number(&self) -> bool {
                self.tag_ == Self::K_NUMBER_TAG
            }

            pub fn is_symbol(&self) -> bool {
                self.tag_ == Self::K_SYMBOL_TAG
            }

            pub fn is_white_space(&self) -> bool {
                self.tag_ == Self::K_WHITE_SPACE_TAG
            }

            pub fn is_end_of_input(&self) -> bool {
                self.tag_ == Self::K_END_OF_INPUT_TAG
            }

            pub fn is_keyword(&self) -> bool {
                self.tag_ >= Self::K_KEYWORD_TAG_START
            }

            pub fn length(&self) -> i32 {
                self.length_
            }

            pub fn number(&self) -> i32 {
                assert!(self.is_number());
                self.value_
            }

            pub fn keyword_type(&self) -> KeywordType {
                assert!(self.is_keyword());
                unsafe { std::mem::transmute(self.tag_) }
            }

            pub fn keyword_value(&self) -> i32 {
                assert!(self.is_keyword());
                self.value_
            }

            pub fn symbol(&self) -> char {
                assert!(self.is_symbol());
                self.value_ as u8 as char
            }

            pub fn is_symbol_char(&self, symbol: char) -> bool {
                self.is_symbol() && self.symbol() == symbol
            }

            pub fn is_keyword_type(&self, tag: KeywordType) -> bool {
                self.tag_ == tag as i32
            }

            pub fn is_fixed_length_number(&self, length: i32) -> bool {
                self.is_number() && self.length_ == length
            }

            pub fn is_ascii_sign(&self) -> bool {
                self.is_symbol() && (self.value_ == '-' as i32 || self.value_ == '+' as i32)
            }

            pub fn ascii_sign(&self) -> i32 {
                assert!(self.is_ascii_sign());
                44 - self.value_
            }

            pub fn is_keyword_z(&self) -> bool {
                self.is_keyword_type(KeywordType::TIME_ZONE_NAME)
                    && self.length_ == 1
                    && self.value_ == 0
            }

            pub fn is_unknown_char(&self, character: i32) -> bool {
                self.is_unknown() && self.value_ == character
            }

            pub fn keyword(tag: KeywordType, value: i32, length: i32) -> DateToken {
                DateToken {
                    tag_: tag as i32,
                    length_: length,
                    value_: value,
                }
            }

            pub fn number_token(value: i32, length: i32) -> DateToken {
                DateToken {
                    tag_: Self::K_NUMBER_TAG,
                    length_: length,
                    value_: value,
                }
            }

            pub fn symbol_token(symbol: char) -> DateToken {
                DateToken {
                    tag_: Self::K_SYMBOL_TAG,
                    length_: 1,
                    value_: symbol as i32,
                }
            }

            pub fn end_of_input() -> DateToken {
                DateToken {
                    tag_: Self::K_END_OF_INPUT_TAG,
                    length_: 0,
                    value_: -1,
                }
            }

            pub fn white_space(length: i32) -> DateToken {
                DateToken {
                    tag_: Self::K_WHITE_SPACE_TAG,
                    length_: length,
                    value_: -1,
                }
            }

            pub fn unknown() -> DateToken {
                DateToken {
                    tag_: Self::K_UNKNOWN_TOKEN_TAG,
                    length_: 1,
                    value_: -1,
                }
            }

            pub fn invalid() -> DateToken {
                DateToken {
                    tag_: Self::K_INVALID_TOKEN_TAG,
                    length_: 0,
                    value_: -1,
                }
            }

            const K_INVALID_TOKEN_TAG: i32 = -6;
            const K_UNKNOWN_TOKEN_TAG: i32 = -5;
            const K_WHITE_SPACE_TAG: i32 = -4;
            const K_NUMBER_TAG: i32 = -3;
            const K_SYMBOL_TAG: i32 = -2;
            const K_END_OF_INPUT_TAG: i32 = -1;
            const K_KEYWORD_TAG_START: i32 = 0;
        }

        /// Tokenizes a date string.
        struct DateStringTokenizer<'a> {
            in_: &'a InputReader,
            next_: DateToken,
        }

        impl<'a> DateStringTokenizer<'a> {
            pub fn new(in_: &'a InputReader) -> Self {
                let mut tokenizer = DateStringTokenizer {
                    in_: in_,
                    next_: DateToken::invalid(),
                };
                tokenizer.next_ = tokenizer.scan();
                tokenizer
            }

            pub fn next(&mut self) -> DateToken {
                let result = self.next_;
                self.next_ = self.scan();
                result
            }

            pub fn peek(&self) -> DateToken {
                self.next_
            }

            pub fn skip_symbol(&mut self, symbol: char) -> bool {
                if self.next_.is_symbol_char(symbol) {
                    self.next_ = self.scan();
                    true
                } else {
                    false
                }
            }

            fn scan(&self) -> DateToken {
                let mut in_copy = InputReader {
                    index_: self.in_.index_ - 1,
                    buffer_: self.in_.buffer_.clone(),
                    ch_: self.in_.ch_,
                };
                if in_copy.is_end() {
                    return DateToken::end_of_input();
                }
                if in_copy.is_ascii_digit() {
                    let start = in_copy.position();
                    let value = in_copy.read_unsigned_numeral();
                    let length = (in_copy.position() - start) as i32;
                    return DateToken::number_token(value, length);
                }
                if in_copy.is_white_space_char() {
                    let start = in_copy.position();
                    while in_copy.is_white_space_char() {
                        in_copy.next();
                    }
                    let length = (in_copy.position() - start) as i32;
                    return DateToken::white_space(length);
                }
                if in_copy.is('(' as u32) {
                    in_copy.skip_parentheses();
                    return DateToken::unknown();
                }
                if in_copy.is_ascii_alpha_or_above() {
                    let mut prefix: [u32; KeywordTable::K_PREFIX_LENGTH] =
                        [0; KeywordTable::K_PREFIX_LENGTH];
                    let len = in_copy.read_word(&mut prefix, KeywordTable::K_PREFIX_LENGTH);
                    let index = KeywordTable::lookup(&prefix, len);
                    if index >= 0 {
                        let tag = KeywordTable::get_type(index);
                        let value = KeywordTable::get_value(index);
                        return DateToken::keyword(tag, value, len as i32);
                    } else {
                        return DateToken::unknown();
                    }
                }
                if in_copy.is('+' as u32)
                    || in_copy.is('-' as u32)
                    || in_copy.is('.' as u32)
                    || in_copy.is(',' as u32)
                    || in_copy.is(':' as u32)
                    || in_copy.is('/' as u32)
                {
                    let c = in_copy.ch_ as u8 as char;
                    in_copy.next();
                    return DateToken::symbol_token(c);
                }
                in_copy.next();
                DateToken::unknown()
            }
        }

        fn read_milliseconds(number: DateToken) -> i32 {
            let value = number.number();
            let length = number.length();
            match length {
                1 => value * 100,
                2 => value * 10,
                3 => value,
                _ => value,
            }
        }

        /// Maps names of months, time zones, am/pm to numbers.
        struct KeywordTable {}

        impl KeywordTable {
            /// Look up a word in the keyword table and return an index.
            /// 'pre' contains a prefix of the word, zero-padded to size kPrefixLength
            /// and 'len' is the word length.
            fn lookup(pre: &[u32], len: usize) -> i32 {
                for (i, entry) in Self::ARRAY.iter().enumerate() {
                    let mut match_ = true;
                    for j in 0..Self::K_PREFIX_LENGTH {
                        if entry[j] as u32 != pre[j] {
                            match_ = false;
                            break;
                        }
                    }
                    if match_ && len == entry[Self::K_LENGTH_OFFSET] as usize{
                        return i as i32;
                    }
                }
                -1
            }

            /// Get the type of the keyword at index i.
            fn get_type(i: usize) -> KeywordType {
                unsafe { std::mem::transmute(Self::ARRAY[i][Self::K_TYPE_OFFSET]) }
            }

            /// Get the value of the keyword at index i.
            fn get_value(i: usize) -> i32 {
                Self::ARRAY[i][Self::K_VALUE_OFFSET]
            }

            const K_PREFIX_LENGTH: usize = 3;
            const K_LENGTH_OFFSET: usize = Self::K_PREFIX_LENGTH;
            const K_TYPE_OFFSET: usize = Self::K_LENGTH_OFFSET + 1;
            const K_VALUE_OFFSET: usize = Self::K_TYPE_OFFSET + 1;
            const K_ENTRY_SIZE: usize = Self::K_VALUE_OFFSET + 1;
            const ARRAY: &'static [[i8; Self::K_ENTRY_SIZE]] = &[
                [('j' as u8) as i8, ('a' as u8) as i8, ('n' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 1],
                [('f' as u8) as i8, ('e' as u8) as i8, ('b' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 2],
                [('m' as u8) as i8, ('a' as u8) as i8, ('r' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 3],
                [('a' as u8) as i8, ('p' as u8) as i8, ('r' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 4],
                [('m' as u8) as i8, ('a' as u8) as i8, ('y' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 5],
                [('j' as u8) as i8, ('u' as u8) as i8, ('n' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 6],
                [('j' as u8) as i8, ('u' as u8) as i8, ('l' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 7],
                [('a' as u8) as i8, ('u' as u8) as i8, ('g' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 8],
                [('s' as u8) as i8, ('e' as u8) as i8, ('p' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 9],
                [('o' as u8) as i8, ('c' as u8) as i8, ('t' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 10],
                [('n' as u8) as i8, ('o' as u8) as i8, ('v' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 11],
                [('d' as u8) as i8, ('e' as u8) as i8, ('c' as u8) as i8, 3, KeywordType::MONTH_NAME as i8, 12],
                [('g' as u8) as i8, ('m' as u8) as i8, ('t' as u8) as i8, 3, KeywordType::TIME_ZONE_NAME as i8, 0],
                [('u' as u8) as i8, ('t' as u8) as i8, ('c' as u8) as i8, 3, KeywordType::TIME_ZONE_NAME as i8, 0],
                [('e' as u8) as i8, ('s' as u8) as i8, ('t' as u8) as i8, 3, KeywordType::TIME_ZONE_NAME as i8, 300],
                [('c' as u8) as i8, ('d' as u8) as i8, ('t' as u8) as i8, 3, KeywordType::TIME_ZONE_NAME as i8, 240],
                [('m' as u8) as i8, ('s' as u8) as i8, ('t' as u8) as i8, 3, KeywordType::TIME_ZONE_NAME as i8, 420],
                [('p' as u8) as i8, ('s' as u8) as i8, ('t' as u8) as i8, 3, KeywordType::TIME_ZONE_NAME as i8, 480],
                [('a' as u8) as i8, ('m' as u8) as i8, (0 as u8) as i8, 2, KeywordType::AM_PM as i8, 0],
                [('p' as u8) as i8, ('m' as u8) as i8, (0 as u8) as i8, 2, KeywordType::AM_PM as i8, 12],
                [('a' as u8) as i8, (0 as u8) as i8, (0 as u8) as i8, 1, KeywordType::AM_PM as i8, 0],
                [('p' as u8) as i8, (0 as u8) as i8, (0 as u8) as i8, 1, KeywordType::AM_PM as i8, 12],
                [('t' as u8) as i8, (0 as u8) as i8, (0 as u8) as i8, 1, KeywordType::TIME_SEPARATOR as i8, 0],
                [('z' as u8) as i8, (0 as u8) as i8, (0 as u8) as i8, 1, KeywordType::TIME_ZONE_NAME as i8, 0],
            ];
        }

        /// Composes the time zone offset.
        struct TimeZoneComposer {
            sign_: i32,
            hour_: i32,
            minute_: i32,
        }

        impl TimeZoneComposer {
            pub fn new() -> Self {
                TimeZoneComposer {
                    sign_: DateParser::K_NONE,
                    hour_: DateParser::K_NONE,
                    minute_: DateParser::K_NONE,
                }
            }

            pub fn set(&mut self, offset_in_hours: i32) {
                self.sign_ = if offset_in_hours < 0 { -1 } else { 1 };
                self.hour_ = offset_in_hours * self.sign_;
                self.minute_ = 0;
            }

            pub fn set_sign(&mut self, sign: i32) {
                self.sign_ = if sign < 0 { -1 } else { 1 };
            }

            pub fn set_absolute_hour(&mut self, hour: i32) {
                self.hour_ = hour;
            }

            pub fn set_absolute_minute(&mut self, minute: i32) {
                self.minute_ = minute;
            }

            pub fn is_expecting(&self, n: i32) -> bool {
                self.hour_ != DateParser::K_NONE
                    && self.minute_ == DateParser::K_NONE
                    && TimeComposer::is_minute(n)
            }

            pub fn is_utc(&self) -> bool {
                self.hour_ == 0 && self.minute_ == 0
            }

            pub fn write(&self, output: &mut [f64]) -> bool {
                if self.hour_ == DateParser::K_NONE {
                    return true;
                }

                let mut offset = self.hour_ * 60 + self.minute_;
                offset *= self.sign_;
                output[DateComponent::UTC_OFFSET as usize] = offset as f64 * 60.0;
                true
            }

            pub fn is_empty(&self) -> bool {
                self.hour_ == DateParser::K_NONE
            }
        }

        /// Composes the time components.
        struct TimeComposer {
            comp_: [i32; Self::K_SIZE],
            index_: usize,
            hour_offset_: i32,
        }

        impl TimeComposer {
            pub fn new() -> Self {
                TimeComposer {
                    comp_: [0; Self::K_SIZE],
                    index_: 0,
                    hour_offset_: DateParser::K_NONE,
                }
            }

            pub fn is_empty(&self) -> bool {
                self.index_ == 0
            }

            pub fn is_expecting(&self, n: i32) -> bool {
                (self.index_ == 1 && Self::is_minute(n))
                    || (self.index_ == 2 && Self::is_second(n))
                    || (self.index_ == 3 && Self::is_millisecond(n))
            }

            pub fn add(&mut self, n: i32) -> bool {
                if self.index_ < Self::K_SIZE {
                    self.comp_[self.index_] = n;
                    self.index_ += 1;
                    true
                } else {
                    false
                }
            }

            pub fn add_final(&mut self, n: i32) -> bool {
                if !self.add(n) {
                    return false;
                }
                while self.index_ < Self::K_SIZE {
                    self.comp_[self.index_] = 0;
                    self.index_ += 1;
                }
                true
            }

            pub fn set_hour_offset(&mut self, n: i32) {
                self.hour_offset_ = n;
            }

            pub fn write(&self, output: &mut [f64]) -> bool {
                if self.is_empty() {
                    return true;
                }

                let hour = self.comp_[0];
                let minute = self.comp_[1];
                let second = self.comp_[2];
                let millisecond = self.comp_[3];

                if !Self::is_hour(hour) || !Self::is_minute(minute) || !Self::is_second(second) {
                    return false;
                }

                output[DateComponent::HOUR as usize] =
                    (hour + if self.hour_offset_ != DateParser::K_NONE {
                        self.hour_offset_
                    } else {
                        0
                    }) as f64;
                output[DateComponent::MINUTE as usize] = minute as f64;
                output[DateComponent::SECOND as usize] = second as f64;
                output[DateComponent::MILLISECOND as usize] = millisecond as f64;

                true
            }

            fn is_minute(x: i32) -> bool {
                DateParser::between(x, 0, 59)
            }
            fn is_hour(x: i32) -> bool {
                DateParser::between(x, 0, 23)
            }
            fn is_second(x: i32) -> bool {
                DateParser::between(x, 0, 59)
            }

            #[allow(dead_code)]
            fn is_hour12(x: i32) -> bool {
                DateParser::between(x, 0, 12)
            }
            fn is_millisecond(x: i32) -> bool {
                DateParser::between(x, 0, 999)
            }

            const K_SIZE: usize = 4;
        }

        /// Composes the day components.
        struct DayComposer {
            comp_: [i32; Self::K_SIZE],
            index_: usize,
            named_month_: i32,
            is_iso_date_: bool,
        }

        impl DayComposer {
            pub fn new() -> Self {
                DayComposer {
                    comp_: [0; Self::K_SIZE],
                    index_: 0,
                    named_month_: DateParser::K_NONE,
                    is_iso_date_: false,
                }
            }

            pub fn is_empty(&self) -> bool {
                self.index_ == 0
            }

            pub fn add(&mut self, n: i32) -> bool {
                if self.index_ < Self::K_SIZE {
                    self.comp_[self.index_] = n;
                    self.index_ += 1;
                    true
                } else {
                    false
                }
            }

            pub fn set_named_month(&mut self, n: i32) {
                self.named_month_ = n;
            }

            pub fn write(&self, output: &mut [f64]) -> bool {
                if self.is_empty() {
                    return true;
                }
                let year = self.comp_[0];
                let month = if self.named_month_ != DateParser::K_NONE {
                    self.named_month_
                } else {
                    self.comp_[1]
                };
                let day = self.comp_[2];

                let mut y = year;
                let mut m = month;
                let mut d = day;

                if !self.is_iso_date_ {
                    if !Self::is_month(m) {
                        if Self::is_month(d) {
                            std::mem::swap(&mut m, &mut d);
                        }
                    }
                    if !Self::is_month(m) {
                        return false;
                    }
                } else {
                    if !Self::is_month(m) {
                        return false;
                    }
                }

                if !Self::is_day(d) {
                    return false;
                }

                if y < 100 {
                    y += 2000;
                    if y > 2100 {
                        y -= 100;
                    }
                }

                output[DateComponent::YEAR as usize] = y as f64;
                output[DateComponent::MONTH as usize] = (m - 1) as f64;
                output[DateComponent::DAY as usize] = d as f64;

                true
            }

            pub fn set_iso_date(&mut self) {
                self.is_iso_date_ = true;
            }

            fn is_month(x: i32) -> bool {
                DateParser::between(x, 1, 12)
            }
            fn is_day(x: i32) -> bool {
                DateParser::between(x, 1, 31)
            }

            const K_SIZE: usize = 3;
        }

        enum DateTokenResult{
            Valid,
            Invalid,
            Continue
        }

        /// Parses an ES5 Date Time String.
        fn parse_es5_date_time<Char: AsRef<[u8]>>(
            scanner: &mut DateStringTokenizer<'_>,
            day: &mut DayComposer,
            time: &mut TimeComposer,
            tz: &mut TimeZoneComposer,
        ) -> DateTokenResult {
            let mut token = scanner.next();
            if token.is_end_of_input() {
                return DateTokenResult::Valid;
            }

            // Date section.
            if token.is_number() {
                // Year.
                let year = token.number();
                let length = token.length();
                if length == 4 {
                    if !day.add(year) {
                        return DateTokenResult::Invalid;
                    }
                    token = scanner.next();
                    if token.is_symbol_char('-') {
                        day.set_iso_date();
                        token = scanner.next();
                        if token.is_number() {
                            // Month.
                            let month = token.number();
                            if !day.add(month) {
                                return DateTokenResult::Invalid;
                            }
                            token = scanner.next();
                            if token.is_symbol_char('-') {
                                token =