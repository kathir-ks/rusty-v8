// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/temporal/temporal-parser.h (Rust module definition - incomplete)
// This is a placeholder; the full header file would define the public interface.

mod temporal_parser {
    use std::fmt;
    use std::fmt::Formatter;
    use std::num::ParseIntError;

    // This struct represent the parsed result of ISO8601 date/time string
    #[derive(Debug, Clone, Copy, Default)]
    pub struct ParsedISO8601Result {
        pub date_year: i32,
        pub date_month: i32,
        pub date_day: i32,
        pub time_hour: i32,
        pub time_minute: i32,
        pub time_second: i32,
        pub time_nanosecond: i32,
        pub tzuo_sign: i32,
        pub tzuo_hour: i32,
        pub tzuo_minute: i32,
        pub tzuo_second: i32,
        pub tzuo_nanosecond: i32,
        pub utc_designator: bool,
        pub tzi_name_start: i32,
        pub tzi_name_length: i32,
        pub offset_string_start: i32,
        pub offset_string_length: i32,
        pub calendar_name_start: i32,
        pub calendar_name_length: i32,
    }

    // This struct represent the parsed result of ISO8601 Duration string
    #[derive(Debug, Clone, Copy, Default)]
    pub struct ParsedISO8601Duration {
        pub sign: i32,
        pub years: f64,
        pub months: f64,
        pub weeks: f64,
        pub days: f64,
        pub whole_hours: f64,
        pub hours_fraction: i32,
        pub whole_minutes: f64,
        pub minutes_fraction: i32,
        pub whole_seconds: f64,
        pub seconds_fraction: i32,
    }

    impl ParsedISO8601Duration {
        pub const kEmpty: f64 = f64::NAN;
    }

    pub struct TemporalParser {}

    impl TemporalParser {
        pub fn new() -> Self {
            TemporalParser {}
        }

        pub fn parse_temporal_date_time_string(
            &self,
            iso_string: &str,
        ) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_temporal_date_time_string,
            )
        }

        pub fn parse_temporal_year_month_string(
            &self,
            iso_string: &str,
        ) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_temporal_year_month_string,
            )
        }

        pub fn parse_temporal_month_day_string(
            &self,
            iso_string: &str,
        ) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_temporal_month_day_string,
            )
        }

        pub fn parse_temporal_time_string(
            &self,
            iso_string: &str,
        ) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_temporal_time_string,
            )
        }

        pub fn parse_temporal_instant_string(
            &self,
            iso_string: &str,
        ) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_temporal_instant_string,
            )
        }

        pub fn parse_temporal_zoned_date_time_string(
            &self,
            iso_string: &str,
        ) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_temporal_zoned_date_time_string,
            )
        }

        pub fn parse_time_zone_identifier(&self, iso_string: &str) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_time_zone_identifier,
            )
        }

        pub fn parse_calendar_name(&self, iso_string: &str) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_calendar_name,
            )
        }

        pub fn parse_time_zone_numeric_utc_offset(
            &self,
            iso_string: &str,
        ) -> Option<ParsedISO8601Result> {
            parse::<ParsedISO8601Result, fn(&[char], &mut ParsedISO8601Result) -> bool>(
                iso_string,
                satisfy_time_zone_numeric_utc_offset,
            )
        }

        pub fn parse_temporal_duration_string(&self, iso_string: &str) -> Option<ParsedISO8601Duration> {
            parse::<ParsedISO8601Duration, fn(&[char], &mut ParsedISO8601Duration) -> bool>(
                iso_string,
                satisfy_temporal_duration_string,
            )
        }
    }

    fn parse<R, F>(s: &str, satisfy_fn: F) -> Option<R>
    where
        R: Default + Copy,
        F: Fn(&[char], &mut R) -> bool,
    {
        let chars: Vec<char> = s.chars().collect();
        let mut result = R::default();
        if satisfy_fn(&chars, &mut result) {
            Some(result)
        } else {
            None
        }
    }

    // Helper functions and constants
    const K_POWER_OF_TEN: [i32; 9] = [
        1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
    ];

    #[inline]
    const fn is_in_range<T: PartialOrd>(val: T, min: T, max: T) -> bool {
        val >= min && val <= max
    }

    #[inline]
    const fn ascii_alpha_to_lower(c: char) -> char {
        if is_in_range(c, 'A', 'Z') {
            (c as u8 + 32) as char
        } else {
            c
        }
    }

    #[inline]
    const fn is_tz_leading_char(c: char) -> bool {
        is_in_range(ascii_alpha_to_lower(c), 'a', 'z') || c == '.' || c == '_'
    }

    #[inline]
    const fn is_tz_char(c: char) -> bool {
        is_tz_leading_char(c) || c == '-'
    }

    #[inline]
    const fn is_decimal_separator(c: char) -> bool {
        c == '.' || c == ','
    }

    #[inline]
    const fn is_date_time_separator(c: char) -> bool {
        c == ' ' || ascii_alpha_to_lower(c) == 't'
    }

    #[inline]
    const fn is_ascii_sign(c: char) -> bool {
        c == '-' || c == '+'
    }

    #[inline]
    const fn is_sign(c: char) -> bool {
        c == '−' || is_ascii_sign(c)
    }

    #[inline]
    const fn is_time_zone_utc_offset_sign(c: char) -> bool {
        is_sign(c)
    }

    #[inline]
    const fn canonical_sign(c: char) -> char {
        if c == '−' {
            '-'
        } else {
            c
        }
    }

    #[inline]
    fn to_int(c: char) -> i32 {
        c as i32 - '0' as i32
    }

    /// A helper to make the scanning of production w/ two digits simpler.
    fn has_two_digits(s: &[char], pos: i32, out: &mut i32) -> bool {
        let pos = pos as usize;
        if s.len() >= (pos + 2)
            && s.get(pos).map_or(false, |&c| c.is_ascii_digit())
            && s.get(pos + 1).map_or(false, |&c| c.is_ascii_digit())
        {
            *out = to_int(s[pos]) * 10 + to_int(s[pos + 1]);
            true
        } else {
            false
        }
    }

    /// A helper to make the scanning of production w/ a single two digits
    /// value simpler.
    fn scan_two_digits_expect_value(s: &[char], pos: i32, expected: i32, out: &mut i32) -> i32 {
        if has_two_digits(s, pos, out) && (*out == expected) {
            2
        } else {
            0
        }
    }

    /// A helper to make the scanning of production w/ two digits value in a
    /// range simpler.
    fn scan_two_digits_expect_range(s: &[char], pos: i32, min: i32, max: i32, out: &mut i32) -> i32 {
        if has_two_digits(s, pos, out) && is_in_range(*out, min, max) {
            2
        } else {
            0
        }
    }

    /// A helper to make the scanning of production w/ two digits value as 0
    /// or in a range simpler.
    fn scan_two_digits_expect_zero_or_range(
        s: &[char],
        pos: i32,
        min: i32,
        max: i32,
        out: &mut i32,
    ) -> i32 {
        if has_two_digits(s, pos, out) && (*out == 0 || is_in_range(*out, min, max)) {
            2
        } else {
            0
        }
    }

    // Hour:
    //   [0 1] Digit
    //   2 [0 1 2 3]
    fn scan_hour(s: &[char], pos: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(s, pos, 0, 23, out)
    }

    // UnpaddedHour :
    //   DecimalDigit
    //   1 DecimalDigit
    //   20
    //   21
    //   22
    //   23
    fn scan_unpadded_hour(s: &[char], pos: i32) -> i32 {
        let mut dummy = 0;
        let len = scan_two_digits_expect_range(s, pos, 10, 23, &mut dummy);
        if len > 0 {
            return len;
        }
        if s.len() >= (pos as usize + 1) && s[pos as usize].is_ascii_digit() {
            return 1;
        }
        0
    }

    // MinuteSecond:
    //   [0 1 2 3 4 5] Digit
    fn scan_minute_second(s: &[char], pos: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(s, pos, 0, 59, out)
    }

    // For the forward production in the grammar such as
    // ProductionB:
    //   ProductionT
    macro_rules! scan_forward {
        ($B:ident, $T:ident, $R:ty) => {
            fn $B(s: &[char], pos: i32, r: &mut $R) -> i32 {
                $T(s, pos, r)
            }
        };
    }

    // Same as above but store the result into a particular field in R

    // For the forward production in the grammar such as
    // ProductionB:
    //   ProductionT1
    //   ProductionT2
    macro_rules! scan_either_forward {
        ($B:ident, $T1:ident, $T2:ident, $R:ty) => {
            fn $B(s: &[char], pos: i32, r: &mut $R) -> i32 {
                let mut len = $T1(s, pos, r);
                if len > 0 {
                    return len;
                }
                $T2(s, pos, r)
            }
        };
    }

    // TimeHour: Hour
    scan_forward!(scan_time_hour, scan_hour, i32);

    // TimeMinute: MinuteSecond
    scan_forward!(scan_time_minute, scan_minute_second, i32);

    // TimeSecond:
    //   MinuteSecond
    //   60
    fn scan_time_second(s: &[char], pos: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(s, pos, 0, 60, out)
    }

    // FractionalPart : Digit{1,9}
    fn scan_fractional_part(s: &[char], pos: i32, out: &mut i32) -> i32 {
        let mut cur = pos as usize;
        if s.len() < (cur + 1) || !s[cur].is_ascii_digit() {
            return 0;
        }
        *out = to_int(s[cur]);
        cur += 1;
        while (cur < s.len()) && ((cur - (pos as usize)) < 9) && s[cur].is_ascii_digit() {
            *out = 10 * (*out) + to_int(s[cur]);
            cur += 1;
        }
        *out *= K_POWER_OF_TEN[9 - (cur - (pos as usize))] as i32;
        (cur as i32) - pos
    }

    // TimeFraction: FractionalPart
    scan_forward!(scan_time_fractional_part, scan_fractional_part, i32);

    // Fraction: DecimalSeparator FractionalPart
    // DecimalSeparator: one of , .
    fn scan_fraction(s: &[char], pos: i32, out: &mut i32) -> i32 {
        let pos_usize = pos as usize;
        if (s.len() < (pos_usize + 2)) || (!is_decimal_separator(s[pos_usize])) {
            return 0;
        }
        let len = scan_fractional_part(s, pos + 1, out);
        if len == 0 {
            return 0;
        }
        len + 1
    }

    // TimeFraction: DecimalSeparator TimeFractionalPart
    // DecimalSeparator: one of , .
    fn scan_time_fraction(s: &[char], pos: i32, out: &mut i32) -> i32 {
        let pos_usize = pos as usize;
        if (s.len() < (pos_usize + 2)) || (!is_decimal_separator(s[pos_usize])) {
            return 0;
        }
        let len = scan_time_fractional_part(s, pos + 1, out);
        if len == 0 {
            return 0;
        }
        len + 1
    }

    fn scan_time_fraction_parsed_result(s: &[char], pos: i32, r: &mut ParsedISO8601Result) -> i32 {
        scan_time_fraction(s, pos, &mut r.time_nanosecond)
    }

    // TimeSpec:
    //  TimeHour
    //  TimeHour : TimeMinute
    //  TimeHour : TimeMinute : TimeSecond [TimeFraction]
    //  TimeHour TimeMinute
    //  TimeHour TimeMinute TimeSecond [TimeFraction]
    fn scan_time_spec(s: &[char], pos: i32, r: &mut ParsedISO8601Result) -> i32 {
        let mut time_hour = 0;
        let mut time_minute = 0;
        let mut time_second = 0;
        let mut len;
        let mut cur = pos as usize;
        if {
            len = scan_time_hour(s, pos, &mut time_hour);
            len
        } == 0
        {
            return 0;
        }
        cur += len as usize;
        if (cur + 1) > s.len() {
            // TimeHour
            r.time_hour = time_hour;
            return (cur as i32) - pos;
        }
        if s[cur] == ':' {
            cur += 1;
            if {
                len = scan_time_minute(s, cur as i32, &mut time_minute);
                len
            } == 0
            {
                return 0;
            }
            cur += len as usize;
            if (cur + 1) > s.len() || (s[cur] != ':') {
                // TimeHour : TimeMinute
                r.time_hour = time_hour;
                r.time_minute = time_minute;
                return (cur as i32) - pos;
            }
            cur += 1;
            if {
                len = scan_time_second(s, cur as i32, &mut time_second);
                len
            } == 0
            {
                return 0;
            }
        } else {
            if {
                len = scan_time_minute(s, cur as i32, &mut time_minute);
                len
            } == 0
            {
                // TimeHour
                r.time_hour = time_hour;
                return (cur as i32) - pos;
            }
            cur += len as usize;
            if {
                len = scan_time_second(s, cur as i32, &mut time_second);
                len
            } == 0
            {
                // TimeHour TimeMinute
                r.time_hour = time_hour;
                r.time_minute = time_minute;
                return (cur as i32) - pos;
            }
        }
        cur += len as usize;
        len = scan_time_fraction_parsed_result(s, cur as i32, r);
        r.time_hour = time_hour;
        r.time_minute = time_minute;
        r.time_second = time_second;
        cur += len as usize;
        ((cur) as i32) - pos
    }

    // TimeSpecSeparator: DateTimeSeparator TimeSpec
    // DateTimeSeparator: SPACE, 't', or 'T'
    fn scan_time_spec_separator(s: &[char], pos: i32, r: &mut ParsedISO8601Result) -> i32 {
        if !(((pos + 1) as usize) < s.len() && is_date_time_separator(s[pos as usize])) {
            return 0;
        }
        let len = scan_time_spec(s, pos + 1, r);
        if len == 0 {
            0
        } else {
            len + 1
        }
    }

    // DateExtendedYear: Sign Digit Digit Digit Digit Digit Digit
    fn scan_date_extended_year(s: &[char], pos: i32, out: &mut i32) -> i32 {
        let pos_usize = pos as usize;
        if s.len() < (pos_usize + 7) {
            return 0;
        }
        if is_sign(s[pos_usize])
            && s[pos_usize + 1].is_ascii_digit()
            && s[pos_usize + 2].is_ascii_digit()
            && s[pos_usize + 3].is_ascii_digit()
            && s[pos_usize + 4].is_ascii_digit()
            && s[pos_usize + 5].is_ascii_digit()
            && s[pos_usize + 6].is_ascii_digit()
        {
            let sign = if canonical_sign(s[pos_usize]) == '-' {
                -1
            } else {
                1
            };
            *out = sign
                * (to_int(s[pos_usize + 1]) * 100000
                    + to_int(s[pos_usize + 2]) * 10000
                    + to_int(s[pos_usize + 3]) * 1000
                    + to_int(s[pos_usize + 4]) * 100
                    + to_int(s[pos_usize + 5]) * 10
                    + to_int(s[pos_usize + 6]));
            // In the end of #sec-temporal-iso8601grammar
            // It is a Syntax Error if DateExtendedYear is "-000000" or "−000000"
            // (U+2212 MINUS SIGN followed by 000000).
            if sign == -1 && *out == 0 {
                return 0;
            }
            7
        } else {
            0
        }
    }

    // DateFourDigitYear: Digit Digit Digit Digit
    fn scan_date_four_digit_year(s: &[char], pos: i32, out: &mut i32) -> i32 {
        let pos_usize = pos as usize;
        if s.len() < (pos_usize + 4) {
            return 0;
        }
        if s[pos_usize].is_ascii_digit()
            && s[pos_usize + 1].is_ascii_digit()
            && s[pos_usize + 2].is_ascii_digit()
            && s[pos_usize + 3].is_ascii_digit()
        {
            *out = to_int(s[pos_usize]) * 1000
                + to_int(s[pos_usize + 1]) * 100
                + to_int(s[pos_usize + 2]) * 10
                + to_int(s[pos_usize + 3]);
            4
        } else {
            0
        }
    }

    // DateYear:
    //   DateFourDigitYear
    //   DateExtendedYear
    // The lookahead is at most 1 char.
    scan_either_forward!(scan_date_year, scan_date_four_digit_year, scan_date_extended_year, i32);

    // DateMonth:
    //   0 NonzeroDigit
    //   10
    //   11
    //   12
    fn scan_date_month(s: &[char], pos: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(s, pos, 1, 12, out)
    }

    // DateDay:
    //   0 NonzeroDigit
    //   1 Digit
    //   2 Digit
    //   30
    //   31
    fn scan_date_day(s: &[char], pos: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(s, pos, 1, 31, out)
    }

    // Date:
    //   DateYear - DateMonth - DateDay
    //   DateYear DateMonth DateDay
    fn scan_date(s: &[char], pos: i32, r: &mut ParsedISO8601Result) -> i32 {
        let mut date_year = 0;
        let mut date_month = 0;
        let mut date_day = 0;
        let mut cur = pos as usize;
        let mut len = scan_date_year(s, pos, &mut date_year);
        if len == 0 {
            return 0;
        }
        cur += len as usize;
        if (cur + 1) > s.len() {
            return 0;
        }
        if s[cur] == '-' {
            cur += 1;
            len = scan_date_month(s, cur as i32, &mut date_month);
            if len == 0 {
                return 0;
            }
            cur += len as usize;
            if ((cur + 1) > s.len()) || (s[cur] != '-') {
                return 0;
            }
            cur += 1;
        } else {
            len = scan_date_month(s, cur as i32, &mut date_month);
            if len == 0 {
                return 0;
            }
            cur += len as usize;
        }
        len = scan_date_day(s, cur as i32, &mut date_day);
        if len == 0 {
            return 0;
        }
        r.date_year = date_year;
        r.date_month = date_month;
        r.date_day = date_day;
        ((cur + len as usize) as i32) - pos
    }

    // DateMonthWithThirtyOneDays : one of
    //    01 03 05 07 08 10 12
    fn scan_date_month_with_thirty_one_days(s: &[char], pos: i32) -> bool {
        let mut value = 0;
        if !has_two_digits(s, pos, &mut value) {
            return false;
        }
        value == 1 || value == 3 || value == 5 || value == 7 || value == 8 || value == 10 || value == 12
    }

    // TimeZoneUTCOffsetHour: Hour
    scan_forward!(scan_time_zone_utc_offset_hour, scan_hour, i32);

    // TimeZoneUTCOffsetMinute
    scan_forward!(scan_time_zone_utc_offset_minute, scan_minute_second, i32);

    // TimeZoneUTCOffsetSecond
    scan_forward!(scan_time_zone_utc_offset_second, scan_minute_second, i32);

    // TimeZoneUTCOffsetFractionalPart: FractionalPart
    // See PR1796
    scan_forward!(
        scan_time_zone_utc_offset_fractional_part,
        scan_fractional_part,
        i32
    );

    // TimeZoneUTCOffsetFraction: DecimalSeparator TimeZoneUTCOffsetFractionalPart
    // See PR1796
    fn scan_time_zone_utc_offset_fraction(s: &[char], pos: i32, out: &mut i32) -> i32 {
        let pos_usize = pos as usize;
        if (s.len() < (pos_usize + 2)) || (!is_decimal_separator(s[pos_usize])) {
            return 0;
        }
        let len = scan_time_zone_utc_offset_fractional_part(s, pos + 1, out);
        if len > 0 {
            len + 1
        } else {
            0
        }
    }

    // TimeZoneNumericUTCOffset:
    //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour
    //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour : TimeZoneUTCOffsetMinute
    //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour TimeZoneUTCOffsetMinute
    //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour : TimeZoneUTCOffsetMinute :
    //   TimeZoneUTCOffsetSecond [TimeZoneUTCOffsetFraction] TimeZoneUTCOffsetSign
    //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour TimeZoneUTCOffsetMinute TimeZoneUTCOffsetSecond
    //   [TimeZoneUTCOffsetFraction]
    fn scan_time_zone_numeric_utc_offset(
        s: &[char],
        pos: i32,
        r: &mut ParsedISO8601Result,
    ) -> i32 {
        let mut len;
        let mut hour = 0;
        let mut minute = 0;
        let mut second = 0;
        let mut nanosecond = 0;
        let mut cur = pos as usize;
        if (s.len() < (cur + 1)) || (!is_time_zone_utc_offset_sign(s[cur])) {
            return 0;
        }
        let sign = if canonical_sign(s[cur]) == '-' {
            -1
        } else {
            1
        };
        cur += 1;
        if {
            len = scan_time_zone_utc_offset_hour(s, cur as i32, &mut hour);
            len
        } == 0
        {
            return 0;
        }
        cur += len as usize;
        if (cur + 1) > s.len() {
            //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour
            r.tzuo_sign = sign;
            r.tzuo_hour = hour;
            r.offset_string_start = pos;
            r.offset_string_length = (cur as i32) - pos;
            return (cur as i32) - pos;
        }
        if s[cur] == ':' {
            cur += 1;
            if {
                len = scan_time_zone_utc_offset_minute(s, cur as i32, &mut minute);
                len
            } == 0
            {
                return 0;
            }
            cur += len as usize;
            if (cur + 1) > s.len() || s[cur] != ':' {
                //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour : TimeZoneUTCOffsetMinute
                r.tzuo_sign = sign;
                r.tzuo_hour = hour;
                r.tzuo_minute = minute;
                r.offset_string_start = pos;
                r.offset_string_length = (cur as i32) - pos;
                return (cur as i32) - pos;
            }
            cur += 1;
            if {
                len = scan_time_zone_utc_offset_second(s, cur as i32, &mut second);
                len
            } == 0
            {
                return 0;
            }
        } else {
            if {
                len = scan_time_zone_utc_offset_minute(s, cur as i32, &mut minute);
                len
            } == 0
            {
                //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour
                r.tzuo_sign = sign;
                r.tzuo_hour = hour;
                r.offset_string_start = pos;
                r.offset_string_length = (cur as i32) - pos;
                return (cur as i32) - pos;
            }
            cur += len as usize;
            if {
                len = scan_time_zone_utc_offset_second(s, cur as i32, &mut second);
                len
            } == 0
            {
                //   TimeZoneUTCOffsetSign TimeZoneUTCOffsetHour TimeZoneUTCOffsetMinute
                r.tzuo_sign = sign;
                r.tzuo_hour = hour;
                r.tzuo_minute = minute;
                r.offset_string_start = pos;
                r.offset_string_length = (cur as i32) - pos;
                return (cur as i32) - pos;
            }
        }
        cur += len as usize;
        len = scan_time_zone_utc_offset_fraction(s, cur as i32, &mut nanosecond);
        r.tzuo_sign = sign;
        r.tzuo_hour = hour;
        r.tzuo_minute = minute;
        r.tzuo_second = second;
        if