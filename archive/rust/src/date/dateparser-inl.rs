// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod dateparser {
    use crate::execution::isolate::Isolate;
    use crate::strings::char_predicates::*;
    use std::fmt;

    pub struct DateParser {}

    impl DateParser {
        pub fn parse<CharType: CharLike>(
            isolate: &mut Isolate,
            str_: Vec<CharType>,
            out: &mut f64,
        ) -> bool {
            let mut in_ = InputReader::new(str_);
            let mut scanner = DateStringTokenizer::new(&mut in_);
            let mut tz = TimeZoneComposer::new();
            let mut time = TimeComposer::new();
            let mut day = DayComposer::new();

            let mut next_unhandled_token =
                DateParser::parse_es5_date_time(&mut scanner, &mut day, &mut time, &mut tz);
            if next_unhandled_token.is_invalid() {
                return false;
            }
            let mut has_read_number = !day.is_empty();
            let mut legacy_parser = false;

            while !next_unhandled_token.is_end_of_input() {
                let token = next_unhandled_token;
                if token.is_number() {
                    legacy_parser = true;
                    has_read_number = true;
                    let n = token.number();
                    if scanner.skip_symbol(':') {
                        if scanner.skip_symbol(':') {
                            if !time.is_empty() {
                                return false;
                            }
                            time.add(n);
                            time.add(0);
                        } else {
                            if !time.add(n) {
                                return false;
                            }
                            if scanner.peek().is_symbol('.') {
                                scanner.next();
                            }
                        }
                    } else if scanner.skip_symbol('.') && time.is_expecting(n) {
                        time.add(n);
                        if !scanner.peek().is_number() {
                            return false;
                        }
                        let ms = DateParser::read_milliseconds(scanner.next());
                        if ms < 0 {
                            return false;
                        }
                        time.add_final(ms);
                    } else if tz.is_expecting(n) {
                        tz.set_absolute_minute(n);
                    } else if time.is_expecting(n) {
                        time.add_final(n);
                        let peek = scanner.peek();
                        if !peek.is_end_of_input()
                            && !peek.is_white_space()
                            && !peek.is_keyword_z()
                            && !peek.is_ascii_sign()
                        {
                            return false;
                        }
                    } else {
                        if !day.add(n) {
                            return false;
                        }
                        scanner.skip_symbol('-');
                    }
                } else if token.is_keyword() {
                    legacy_parser = true;
                    let type_ = token.keyword_type();
                    let value = token.keyword_value();
                    if type_ == KeywordType::AM_PM && !time.is_empty() {
                        time.set_hour_offset(value);
                    } else if type_ == KeywordType::MONTH_NAME {
                        day.set_named_month(value);
                        scanner.skip_symbol('-');
                    } else if type_ == KeywordType::TIME_ZONE_NAME && has_read_number {
                        tz.set(value);
                    } else {
                        if has_read_number {
                            return false;
                        }
                        if scanner.peek().is_number() {
                            return false;
                        }
                    }
                } else if token.is_ascii_sign() && (tz.is_utc() || !time.is_empty()) {
                    legacy_parser = true;
                    tz.set_sign(token.ascii_sign());
                    let mut n = 0;
                    let mut length = 0;
                    if scanner.peek().is_number() {
                        let next_token = scanner.next();
                        length = next_token.length();
                        n = next_token.number();
                    }
                    has_read_number = true;

                    if scanner.peek().is_symbol(':') {
                        tz.set_absolute_hour(n);
                        tz.set_absolute_minute(kNone);
                    } else if length == 2 || length == 1 {
                        tz.set_absolute_hour(n);
                        tz.set_absolute_minute(0);
                    } else if length == 4 || length == 3 {
                        tz.set_absolute_hour(n / 100);
                        tz.set_absolute_minute(n % 100);
                    } else {
                        return false;
                    }
                } else if (token.is_ascii_sign() || token.is_symbol(')')) && has_read_number {
                    return false;
                }

                next_unhandled_token = scanner.next();
            }

            let success = day.write(out) && time.write(out) && tz.write(out);

            if legacy_parser && success {
                isolate.count_usage(v8::IsolateUsage::kLegacyDateParser);
            }

            success
        }

        fn parse_es5_date_time<CharType: CharLike>(
            scanner: &mut DateStringTokenizer<'_, CharType>,
            day: &mut DayComposer,
            time: &mut TimeComposer,
            tz: &mut TimeZoneComposer,
        ) -> DateToken {
            debug_assert!(day.is_empty());
            debug_assert!(time.is_empty());
            debug_assert!(tz.is_empty());

            if scanner.peek().is_ascii_sign() {
                let sign_token = scanner.next();
                if !scanner.peek().is_fixed_length_number(6) {
                    return sign_token;
                }
                let sign = sign_token.ascii_sign();
                let year = scanner.next().number();
                if sign < 0 && year == 0 {
                    return sign_token;
                }
                day.add(sign * year);
            } else if scanner.peek().is_fixed_length_number(4) {
                day.add(scanner.next().number());
            } else {
                return scanner.next();
            }
            if scanner.skip_symbol('-') {
                if !scanner.peek().is_fixed_length_number(2)
                    || !DayComposer::is_month(scanner.peek().number())
                {
                    return scanner.next();
                }
                day.add(scanner.next().number());
                if scanner.skip_symbol('-') {
                    if !scanner.peek().is_fixed_length_number(2)
                        || !DayComposer::is_day(scanner.peek().number())
                    {
                        return scanner.next();
                    }
                    day.add(scanner.next().number());
                }
            }
            if !scanner.peek().is_keyword_type(KeywordType::TIME_SEPARATOR) {
                if !scanner.peek().is_end_of_input() {
                    return scanner.next();
                }
            } else {
                scanner.next();
                if !scanner.peek().is_fixed_length_number(2)
                    || !between(scanner.peek().number(), 0, 24)
                {
                    return DateToken::Invalid();
                }
                let hour_is_24 = (scanner.peek().number() == 24);
                time.add(scanner.next().number());
                if !scanner.skip_symbol(':') {
                    return DateToken::Invalid();
                }
                if !scanner.peek().is_fixed_length_number(2)
                    || !TimeComposer::is_minute(scanner.peek().number())
                    || (hour_is_24 && scanner.peek().number() > 0)
                {
                    return DateToken::Invalid();
                }
                time.add(scanner.next().number());
                if scanner.skip_symbol(':') {
                    if !scanner.peek().is_fixed_length_number(2)
                        || !TimeComposer::is_second(scanner.peek().number())
                        || (hour_is_24 && scanner.peek().number() > 0)
                    {
                        return DateToken::Invalid();
                    }
                    time.add(scanner.next().number());
                    if scanner.skip_symbol('.') {
                        if !scanner.peek().is_number() || (hour_is_24 && scanner.peek().number() > 0)
                        {
                            return DateToken::Invalid();
                        }
                        time.add(DateParser::read_milliseconds(scanner.next()));
                    }
                }
                if scanner.peek().is_keyword_z() {
                    scanner.next();
                    tz.set(0);
                } else if scanner.peek().is_symbol('+') || scanner.peek().is_symbol('-') {
                    tz.set_sign(if scanner.next().symbol() == '+' { 1 } else { -1 });
                    if scanner.peek().is_fixed_length_number(4) {
                        let hourmin = scanner.next().number();
                        let hour = hourmin / 100;
                        let min = hourmin % 100;
                        if !TimeComposer::is_hour(hour) || !TimeComposer::is_minute(min) {
                            return DateToken::Invalid();
                        }
                        tz.set_absolute_hour(hour);
                        tz.set_absolute_minute(min);
                    } else {
                        if !scanner.peek().is_fixed_length_number(2)
                            || !TimeComposer::is_hour(scanner.peek().number())
                        {
                            return DateToken::Invalid();
                        }
                        tz.set_absolute_hour(scanner.next().number());
                        if !scanner.skip_symbol(':') {
                            return DateToken::Invalid();
                        }
                        if !scanner.peek().is_fixed_length_number(2)
                            || !TimeComposer::is_minute(scanner.peek().number())
                        {
                            return DateToken::Invalid();
                        }
                        tz.set_absolute_minute(scanner.next().number());
                    }
                }
                if !scanner.peek().is_end_of_input() {
                    return DateToken::Invalid();
                }
            }
            if tz.is_empty() && time.is_empty() {
                tz.set(0);
            }
            day.set_iso_date();
            DateToken::EndOfInput()
        }

        fn read_milliseconds(token: DateToken) -> i32 {
            token.number()
        }
    }

    trait CharLike: Copy {
        fn as_u32(&self) -> u32;
    }

    impl CharLike for char {
        fn as_u32(&self) -> u32 {
            *self as u32
        }
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum DateToken {
        Number(i32, i32),
        Symbol(char),
        Keyword(KeywordType, i32, i32),
        WhiteSpace(i32),
        Unknown,
        EndOfInput,
        Invalid,
    }

    impl DateToken {
        pub fn is_number(&self) -> bool {
            matches!(self, DateToken::Number(_, _))
        }

        pub fn is_symbol(&self, c: char) -> bool {
            matches!(self, DateToken::Symbol(s) if *s == c)
        }

        pub fn is_keyword(&self) -> bool {
            matches!(self, DateToken::Keyword(_, _, _))
        }

        pub fn is_white_space(&self) -> bool {
            matches!(self, DateToken::WhiteSpace(_))
        }

        pub fn is_end_of_input(&self) -> bool {
            matches!(self, DateToken::EndOfInput)
        }

        pub fn is_invalid(&self) -> bool {
            matches!(self, DateToken::Invalid)
        }

        pub fn is_ascii_sign(&self) -> bool {
            matches!(self, DateToken::Symbol('+') | DateToken::Symbol('-'))
        }

        pub fn is_keyword_z(&self) -> bool {
            matches!(
                self,
                DateToken::Keyword(KeywordType::TIME_ZONE_NAME, 0, _)
            )
        }

        pub fn is_keyword_type(&self, kind: KeywordType) -> bool {
            matches!(self, DateToken::Keyword(t, _, _) if *t == kind)
        }

        pub fn number(&self) -> i32 {
            match self {
                DateToken::Number(n, _) => *n,
                _ => panic!("Not a number token"),
            }
        }

        pub fn symbol(&self) -> char {
            match self {
                DateToken::Symbol(c) => *c,
                _ => panic!("Not a symbol token"),
            }
        }

        pub fn keyword_type(&self) -> KeywordType {
            match self {
                DateToken::Keyword(t, _, _) => *t,
                _ => panic!("Not a keyword token"),
            }
        }

        pub fn keyword_value(&self) -> i32 {
            match self {
                DateToken::Keyword(_, v, _) => *v,
                _ => panic!("Not a keyword token"),
            }
        }

        pub fn length(&self) -> i32 {
            match self {
                DateToken::Number(_, l) => *l,
                DateToken::Keyword(_, _, l) => *l,
                _ => 0,
            }
        }

        pub fn is_fixed_length_number(&self, length: i32) -> bool {
            matches!(self, DateToken::Number(_, l) if *l == length)
        }

        pub fn number_length(&self) -> i32 {
            match self {
                DateToken::Number(_, len) => *len,
                _ => 0,
            }
        }
        pub fn Symbol(c: char) -> DateToken {
            DateToken::Symbol(c)
        }
        pub fn Number(n: i32, length: i32) -> DateToken {
            DateToken::Number(n, length)
        }

        pub fn Keyword(kind: KeywordType, value: i32, length: i32) -> DateToken {
            DateToken::Keyword(kind, value, length)
        }
        pub fn WhiteSpace(length: i32) -> DateToken {
            DateToken::WhiteSpace(length)
        }
        pub fn Unknown() -> DateToken {
            DateToken::Unknown
        }
        pub fn EndOfInput() -> DateToken {
            DateToken::EndOfInput
        }
        pub fn Invalid() -> DateToken {
            DateToken::Invalid
        }
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum KeywordType {
        AM_PM,
        MONTH_NAME,
        TIME_ZONE_NAME,
        TIME_SEPARATOR,
    }

    struct KeywordTable {}

    impl KeywordTable {
        const kPrefixLength: usize = 3;

        fn lookup(buffer: &[u32], length: usize) -> i32 {
            // This is a placeholder; implement the actual lookup logic
            // based on the V8 KeywordTable.
            0
        }

        fn get_type(index: i32) -> KeywordType {
            // Placeholder; implement the logic based on the V8 KeywordTable.
            KeywordType::TIME_ZONE_NAME
        }

        fn get_value(index: i32) -> i32 {
            // Placeholder; implement the logic based on the V8 KeywordTable.
            0
        }
    }

    struct InputReader<'a, CharType: CharLike> {
        str_: Vec<CharType>,
        position_: usize,
        ch_: Option<CharType>,
        _phantom: std::marker::PhantomData<&'a CharType>,
    }

    impl<'a, CharType: CharLike> InputReader<'a, CharType> {
        fn new(str_: Vec<CharType>) -> Self {
            let mut reader = InputReader {
                str_: str_,
                position_: 0,
                ch_: None,
                _phantom: std::marker::PhantomData,
            };
            reader.next();
            reader
        }

        fn position(&self) -> usize {
            self.position_
        }

        fn is_end(&self) -> bool {
            self.ch_.is_none()
        }

        fn is_ascii_digit(&self) -> bool {
            match self.ch_ {
                Some(c) => is_ascii_digit(c.as_u32() as u8),
                None => false,
            }
        }

        fn is_ascii_alpha_or_above(&self) -> bool {
            match self.ch_ {
                Some(c) => c.as_u32() >= 'A' as u32,
                None => false,
            }
        }

        fn is_white_space_char(&self) -> bool {
            match self.ch_ {
                Some(c) => is_white_space_or_line_terminator(c.as_u32()),
                None => false,
            }
        }
        fn next(&mut self) {
            if self.position_ < self.str_.len() {
                self.ch_ = Some(self.str_[self.position_]);
                self.position_ += 1;
            } else {
                self.ch_ = None;
            }
        }

        fn read_unsigned_numeral(&mut self) -> i32 {
            let mut value: i32 = 0;
            while let Some(c) = self.ch_ {
                if !is_ascii_digit(c.as_u32() as u8) {
                    break;
                }
                value = value * 10 + (c.as_u32() as u8 - b'0') as i32;
                self.next();
            }
            value
        }
        fn skip(&mut self, c: char) -> bool {
            match self.ch_ {
                Some(ch) if ch.as_u32() as u8 == c as u8 => {
                    self.next();
                    true
                }
                _ => false,
            }
        }

        fn skip_white_space(&mut self) -> bool {
            if let Some(ch) = self.ch_ {
                if is_white_space_or_line_terminator(ch.as_u32()) {
                    self.next();
                    return true;
                }
            }
            false
        }

        fn skip_parentheses(&mut self) -> bool {
            if self.ch_.map_or(false, |ch| ch.as_u32() == '(' as u32) {
                let mut balance = 0;
                while let Some(ch) = self.ch_ {
                    if ch.as_u32() == ')' as u32 {
                        balance -= 1;
                    } else if ch.as_u32() == '(' as u32 {
                        balance += 1;
                    }
                    self.next();
                    if balance <= 0 || self.ch_.is_none() {
                        break;
                    }
                }
                true
            } else {
                false
            }
        }

        fn read_word(&mut self, buffer: &mut [u32], max_length: usize) -> usize {
            let mut length = 0;
            while length < max_length {
                if let Some(ch) = self.ch_ {
                    if self.is_ascii_alpha_or_above() && !self.is_white_space_char() {
                        buffer[length] = ch.as_u32();
                        length += 1;
                        self.next();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            length
        }
    }

    struct DateStringTokenizer<'a, CharType: CharLike> {
        in_: &'a mut InputReader<'a, CharType>,
        next_token: Option<DateToken>,
    }

    impl<'a, CharType: CharLike> DateStringTokenizer<'a, CharType> {
        fn new(in_: &'a mut InputReader<'a, CharType>) -> Self {
            let mut tokenizer = DateStringTokenizer {
                in_: in_,
                next_token: None,
            };
            tokenizer.next_token = Some(tokenizer.scan());
            tokenizer
        }

        fn next(&mut self) -> DateToken {
            let token = self.next_token.take().unwrap_or(DateToken::EndOfInput());
            self.next_token = Some(self.scan());
            token
        }

        fn peek(&self) -> DateToken {
            self.next_token.unwrap_or(DateToken::EndOfInput())
        }

        fn scan(&mut self) -> DateToken {
            let pre_pos = self.in_.position();
            if self.in_.is_end() {
                return DateToken::EndOfInput();
            }
            if self.in_.is_ascii_digit() {
                let n = self.in_.read_unsigned_numeral();
                let length = self.in_.position() - pre_pos;
                return DateToken::Number(n, length as i32);
            }
            if self.in_.skip(':') {
                return DateToken::Symbol(':');
            }
            if self.in_.skip('-') {
                return DateToken::Symbol('-');
            }
            if self.in_.skip('+') {
                return DateToken::Symbol('+');
            }
            if self.in_.skip('.') {
                return DateToken::Symbol('.');
            }
            if self.in_.skip(')') {
                return DateToken::Symbol(')');
            }
            if self.in_.is_ascii_alpha_or_above() && !self.in_.is_white_space_char() {
                let mut buffer: [u32; 3] = [0, 0, 0];
                let length = self.in_.read_word(&mut buffer, 3);
                let index = KeywordTable::lookup(&buffer[..length], length);
                return DateToken::Keyword(
                    KeywordTable::get_type(index),
                    KeywordTable::get_value(index),
                    length as i32,
                );
            }
            if self.in_.skip_white_space() {
                return DateToken::WhiteSpace((self.in_.position() - pre_pos) as i32);
            }
            if self.in_.skip_parentheses() {
                return DateToken::Unknown();
            }
            self.in_.next();
            DateToken::Unknown()
        }

        fn skip_symbol(&mut self, symbol: char) -> bool {
            if self.peek().is_symbol(symbol) {
                self.next();
                true
            } else {
                false
            }
        }
    }

    struct TimeComposer {
        hours: i32,
        minutes: i32,
        seconds: i32,
        milliseconds: i32,
        hour_offset: i32,
        state: TimeState,
    }

    #[derive(PartialEq)]
    enum TimeState {
        Initial,
        Hour,
        Minute,
        Second,
        Millisecond,
        Finalized,
    }

    impl TimeComposer {
        fn new() -> Self {
            TimeComposer {
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
                hour_offset: 0,
                state: TimeState::Initial,
            }
        }
        fn is_empty(&self) -> bool {
            self.state == TimeState::Initial
        }
        fn add(&mut self, value: i32) -> bool {
            match self.state {
                TimeState::Initial => {
                    if !TimeComposer::is_hour(value) {
                        return false;
                    }
                    self.hours = value;
                    self.state = TimeState::Hour;
                    true
                }
                TimeState::Hour => {
                    if !TimeComposer::is_minute(value) {
                        return false;
                    }
                    self.minutes = value;
                    self.state = TimeState::Minute;
                    true
                }
                _ => false,
            }
        }

        fn add_final(&mut self, value: i32) {
            match self.state {
                TimeState::Minute => {
                    self.seconds = value;
                    self.state = TimeState::Second;
                }
                TimeState::Second => {
                    self.milliseconds = value;
                    self.state = TimeState::Millisecond;
                }
                _ => {}
            }
            self.state = TimeState::Finalized;
        }

        fn is_expecting(&self, value: i32) -> bool {
            match self.state {
                TimeState::Initial => TimeComposer::is_hour(value),
                TimeState::Hour => TimeComposer::is_minute(value),
                TimeState::Minute => TimeComposer::is_second(value),
                TimeState::Second => true,
                _ => false,
            }
        }

        fn set_hour_offset(&mut self, value: i32) {
            self.hour_offset = value;
        }

        fn write(&self, out: &mut f64) -> bool {
            *out = self.hours as f64 * 3600000.0 + self.minutes as f64 * 60000.0
                + self.seconds as f64 * 1000.0
                + self.milliseconds as f64;
            true
        }

        fn is_hour(hour: i32) -> bool {
            hour >= 0 && hour <= 24
        }
        fn is_minute(minute: i32) -> bool {
            minute >= 0 && minute <= 59
        }
        fn is_second(second: i32) -> bool {
            second >= 0 && second <= 59
        }
    }

    struct DayComposer {
        year: i32,
        month: i32,
        day: i32,
        named_month: i32,
        state: DayState,
        iso_date: bool,
    }
    #[derive(PartialEq)]
    enum DayState {
        Initial,
        Year,
        Month,
        Day,
        Finalized,
    }

    impl DayComposer {
        fn new() -> Self {
            DayComposer {
                year: 0,
                month: 0,
                day: 0,
                named_month: 0,
                state: DayState::Initial,
                iso_date: false,
            }
        }
        fn is_empty(&self) -> bool {
            self.state == DayState::Initial
        }
        fn add(&mut self, value: i32) -> bool {
            match self.state {
                DayState::Initial => {
                    self.year = value;
                    self.state = DayState::Year;
                    true
                }
                DayState::Year => {
                    if !DayComposer::is_month(value) {
                        return false;
                    }
                    self.month = value;
                    self.state = DayState::Month;
                    true
                }
                DayState::Month => {
                    if !DayComposer::is_day(value) {
                        return false;
                    }
                    self.day = value;
                    self.state = DayState::Day;
                    true
                }
                _ => false,
            }
        }
        fn set_named_month(&mut self, value: i32) {
            self.named_month = value;
        }
        fn write(&self, out: &mut f64) -> bool {
            *out = self.year as f64 * 3600000.0;
            true
        }

        fn is_month(month: i32) -> bool {
            month >= 1 && month <= 12
        }
        fn is_day(day: i32) -> bool {
            day >= 1 && day <= 31
        }
        fn set_iso_date(&mut self) {
            self.iso_date = true;
        }
    }

    const kNone: i32 = -1;

    struct TimeZoneComposer {
        utc: bool,
        sign: i32,
        absolute_hour: i32,
        absolute_minute: i32,
    }

    impl TimeZoneComposer {
        fn new() -> Self {
            TimeZoneComposer {
                utc: true,
                sign: 1,
                absolute_hour: 0,
                absolute_minute: 0,
            }
        }
        fn is_empty(&self) -> bool {
            self.utc
        }
        fn set(&mut self, value: i32) {
            self.utc = value == 0;
        }
        fn set_sign(&mut self, value: i32) {
            self.sign = value;
            self.utc = false;
        }
        fn set_absolute_hour(&mut self, value: i32) {
            self.absolute_hour = value;
            self.utc = false;
        }
        fn set_absolute_minute(&mut self, value: i32) {
            self.absolute_minute = value;
            self.utc = false;
        }
        fn is_utc(&self) -> bool {
            self.utc
        }
        fn is_expecting(&self, _value: i32) -> bool {
            self.utc
        }
        fn write(&self, out: &mut f64) -> bool {
            *out = self.sign as f64 * self.absolute_hour as f64 * 3600000.0
                + self.sign as f64 * self.absolute_minute as f64 * 60000.0;
            true
        }
    }

    fn between(value: i32, low: i32, high: i32) -> bool {
        value >= low && value <= high
    }
}

mod execution {
    pub mod isolate {
        #[derive(Debug, Copy, Clone)]
        pub enum IsolateUsage {
            kLegacyDateParser,
        }

        #[derive(Debug)]
        pub struct Isolate {}
        impl Isolate {
            pub fn count_usage(&mut self, usage: IsolateUsage) {
                println!("Usage: {:?}", usage);
            }
        }
    }
}

mod strings {
    pub mod char_predicates {
        pub fn is_ascii_digit(c: u8) -> bool {
            c.is_ascii_digit()
        }

        pub fn is_white_space_or_line_terminator(c: u32) -> bool {
            match c {
                0x0009 | 0x000B | 0x000C | 0x0020 | 0x00A0 | 0x202F | 0x205F | 0x3000 | 0xFEFF
                | 0x1680 | 0x2000..=0x200A | 0x000A | 0x000D | 0x2028 | 0x2029 => true,
                _ => false,
            }
        }
    }
}

mod v8 {
    pub enum IsolateUsage {
        kLegacyDateParser,
    }
}