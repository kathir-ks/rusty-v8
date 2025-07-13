// Converted from V8 C++ source files:
// Header: temporal-parser.h
// Implementation: temporal-parser.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod temporal_parser {
    use std::optional::Optional;
    pub use crate::execution::isolate::Isolate;
    pub use crate::objects::string::String;
    use crate::deoptimizer::deoptimizer::DisallowGarbageCollection;
    use crate::strings::char_predicates_inl::{IsDecimalDigit, IsAlphaNumeric, AsciiAlphaToLower};
    use crate::base::bounds::IsInRange;

    pub const K_MIN_INT31: i32 = i32::MIN;

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
        pub calendar_name_start: i32,
        pub calendar_name_length: i32,
        pub offset_string_start: i32,
        pub offset_string_length: i32,
    }

    impl ParsedISO8601Result {
        pub fn new() -> Self {
            ParsedISO8601Result {
                date_year: K_MIN_INT31,
                date_month: K_MIN_INT31,
                date_day: K_MIN_INT31,
                time_hour: K_MIN_INT31,
                time_minute: K_MIN_INT31,
                time_second: K_MIN_INT31,
                time_nanosecond: K_MIN_INT31,
                tzuo_sign: K_MIN_INT31,
                tzuo_hour: K_MIN_INT31,
                tzuo_minute: K_MIN_INT31,
                tzuo_second: K_MIN_INT31,
                tzuo_nanosecond: K_MIN_INT31,
                utc_designator: false,
                tzi_name_start: 0,
                tzi_name_length: 0,
                calendar_name_start: 0,
                calendar_name_length: 0,
                offset_string_start: 0,
                offset_string_length: 0,
            }
        }

        pub fn date_year_is_undefined(&self) -> bool {
            self.date_year == K_MIN_INT31
        }
        pub fn date_month_is_undefined(&self) -> bool {
            self.date_month == K_MIN_INT31
        }
        pub fn date_day_is_undefined(&self) -> bool {
            self.date_day == K_MIN_INT31
        }
        pub fn time_hour_is_undefined(&self) -> bool {
            self.time_hour == K_MIN_INT31
        }
        pub fn time_minute_is_undefined(&self) -> bool {
            self.time_minute == K_MIN_INT31
        }
        pub fn time_second_is_undefined(&self) -> bool {
            self.time_second == K_MIN_INT31
        }
        pub fn time_nanosecond_is_undefined(&self) -> bool {
            self.time_nanosecond == K_MIN_INT31
        }
        pub fn tzuo_hour_is_undefined(&self) -> bool {
            self.tzuo_hour == K_MIN_INT31
        }
        pub fn tzuo_minute_is_undefined(&self) -> bool {
            self.tzuo_minute == K_MIN_INT31
        }
        pub fn tzuo_second_is_undefined(&self) -> bool {
            self.tzuo_second == K_MIN_INT31
        }
        pub fn tzuo_sign_is_undefined(&self) -> bool {
            self.tzuo_sign == K_MIN_INT31
        }
        pub fn tzuo_nanosecond_is_undefined(&self) -> bool {
            self.tzuo_nanosecond == K_MIN_INT31
        }
    }

    pub struct ParsedISO8601Duration {
        pub sign: f64,
        pub years: f64,
        pub months: f64,
        pub weeks: f64,
        pub days: f64,
        pub whole_hours: f64,
        pub whole_minutes: f64,
        pub whole_seconds: f64,
        pub hours_fraction: i32,
        pub minutes_fraction: i32,
        pub seconds_fraction: i32,
    }

    impl ParsedISO8601Duration {
        pub const K_EMPTY: i32 = -1;
        pub fn new() -> Self {
            ParsedISO8601Duration {
                sign: 1.0,
                years: ParsedISO8601Duration::K_EMPTY as f64,
                months: ParsedISO8601Duration::K_EMPTY as f64,
                weeks: ParsedISO8601Duration::K_EMPTY as f64,
                days: ParsedISO8601Duration::K_EMPTY as f64,
                whole_hours: ParsedISO8601Duration::K_EMPTY as f64,
                whole_minutes: ParsedISO8601Duration::K_EMPTY as f64,
                whole_seconds: ParsedISO8601Duration::K_EMPTY as f64,
                hours_fraction: ParsedISO8601Duration::K_EMPTY,
                minutes_fraction: ParsedISO8601Duration::K_EMPTY,
                seconds_fraction: ParsedISO8601Duration::K_EMPTY,
            }
        }
    }
    pub struct DirectHandle<T> {
        pub value: T
    }
    impl <T> DirectHandle<T>{
        pub fn new(value: T) -> Self{
            DirectHandle{value}
        }
    }
    pub struct TemporalParser {}

    impl TemporalParser {
        pub fn parse_temporal_date_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_temporal_date_string_impl(isolate, iso_string)
        }
        fn parse_temporal_date_string_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_temporal_date_time_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_temporal_date_time_string_impl(isolate, iso_string)
        }
        fn parse_temporal_date_time_string_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_temporal_time_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_temporal_time_string_impl(isolate, iso_string)
        }
        fn parse_temporal_time_string_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_temporal_year_month_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_temporal_year_month_string_impl(isolate, iso_string)
        }
        fn parse_temporal_year_month_string_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_temporal_month_day_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_temporal_month_day_string_impl(isolate, iso_string)
        }
        fn parse_temporal_month_day_string_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_temporal_instant_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_temporal_instant_string_impl(isolate, iso_string)
        }
        fn parse_temporal_instant_string_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_temporal_zoned_date_time_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_temporal_zoned_date_time_string_impl(isolate, iso_string)
        }
        fn parse_temporal_zoned_date_time_string_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_time_zone_identifier(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_time_zone_identifier_impl(isolate, iso_string)
        }
        fn parse_time_zone_identifier_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_temporal_relative_to_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_temporal_relative_to_string_impl(isolate, iso_string)
        }
        fn parse_temporal_relative_to_string_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_calendar_name(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_calendar_name_impl(isolate, iso_string)
        }
        fn parse_calendar_name_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }

        pub fn parse_temporal_duration_string(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Duration> {
            TemporalParser::parse_temporal_duration_string_impl(isolate, iso_string)
        }
        fn parse_temporal_duration_string_impl(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Duration> {
            let mut parsed = ParsedISO8601Duration::new();
            let iso_string = String::Flatten(isolate, iso_string);
            {
                let _no_gc = DisallowGarbageCollection{};
                if let Some(str_content) = iso_string.GetFlatContent(){
                    if str_content.IsOneByte(){
                        let str = str_content.ToOneByteVector();
                        if SatisfyTemporalDurationString(str, &mut parsed){
                            return Optional::Some(parsed)
                        } else {
                            return Optional::None
                        }
                    } else{
                        let str = str_content.ToUC16Vector();
                        if SatisfyTemporalDurationString(str, &mut parsed){
                            return Optional::Some(parsed)
                        } else {
                            return Optional::None
                        }
                    }
                } else {
                    return Optional::None
                }
            }
        }

        pub fn parse_time_zone_numeric_utc_offset(
            isolate: *mut Isolate,
            iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            TemporalParser::parse_time_zone_numeric_utc_offset_impl(isolate, iso_string)
        }
        fn parse_time_zone_numeric_utc_offset_impl(
            _isolate: *mut Isolate,
            _iso_string: DirectHandle<String>,
        ) -> Optional<ParsedISO8601Result> {
            Optional::None
        }
    }

    // Temporal #prod-TZLeadingChar
    #[inline]
    const fn is_tz_leading_char(c: u32) -> bool {
        IsInRange(AsciiAlphaToLower(c), 'a' as u32, 'z' as u32) || c == '.' as u32 ||
            c == '_' as u32
    }

    // Temporal #prod-TZChar
    #[inline]
    const fn is_tz_char(c: u32) -> bool {
        is_tz_leading_char(c) || c == '-' as u32
    }

    // Temporal #prod-DecimalSeparator
    #[inline]
    const fn is_decimal_separator(c: u32) -> bool {
        c == '.' as u32 || c == ',' as u32
    }

    // Temporal #prod-DateTimeSeparator
    #[inline]
    const fn is_date_time_separator(c: u32) -> bool {
        c == ' ' as u32 || AsciiAlphaToLower(c) == 't' as u32
    }

    // Temporal #prod-ASCIISign
    #[inline]
    const fn is_ascii_sign(c: u32) -> bool {
        c == '-' as u32 || c == '+' as u32
    }

    // Temporal #prod-Sign
    #[inline]
    const fn is_sign(c: u32) -> bool {
        c == 0x2212 || is_ascii_sign(c)
    }

    // Temporal #prod-TimeZoneUTCOffsetSign
    #[inline]
    const fn is_time_zone_utc_offset_sign(c: u32) -> bool {
        is_sign(c)
    }

    #[inline]
    const fn canonical_sign(c: u32) -> u32 {
        if c == 0x2212 { '-' as u32 } else { c }
    }

    #[inline]
    const fn to_int(c: u32) -> i32 {
        (c - '0' as u32) as i32
    }

    // A helper template to make the scanning of production w/ two digits simpler.
    fn has_two_digits<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> bool {
        let str_slice = str.as_ref();
        if str_slice.len() >= (s + 2) as usize && IsDecimalDigit(str_slice[s as usize]) &&
            IsDecimalDigit(str_slice[s as usize + 1]) {
            *out = to_int(str_slice[s as usize]) * 10 + to_int(str_slice[s as usize + 1]);
            true
        } else {
            false
        }
    }

    // A helper template to make the scanning of production w/ a single two digits
    // value simpler.
    fn scan_two_digits_expect_value<Char: AsRef<[u32]>>(str: Char, s: i32,
                                                         expected: i32, out: &mut i32) -> i32 {
        if has_two_digits(str.as_ref(), s, out) && (*out == expected) {
            2
        } else {
            0
        }
    }

    // A helper template to make the scanning of production w/ two digits value in a
    // range simpler.
    fn scan_two_digits_expect_range<Char: AsRef<[u32]>>(str: Char, s: i32, min: i32,
                                                         max: i32, out: &mut i32) -> i32 {
        if has_two_digits(str.as_ref(), s, out) && IsInRange(*out as u32, min as u32, max as u32) {
            2
        } else {
            0
        }
    }

    // A helper template to make the scanning of production w/ two digits value as 0
    // or in a range simpler.
    fn scan_two_digits_expect_zero_or_range<Char: AsRef<[u32]>>(str: Char, s: i32,
                                                                 min: i32, max: i32, out: &mut i32) -> i32 {
        if has_two_digits(str.as_ref(), s, out) &&
            (*out == 0 || IsInRange(*out as u32, min as u32, max as u32)) {
            2
        } else {
            0
        }
    }

    // For Hour Production
    // Hour:
    //   [0 1] Digit
    //   2 [0 1 2 3]
    fn scan_hour<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(str, s, 0, 23, out)
    }

    // UnpaddedHour :
    //   DecimalDigit
    //   1 DecimalDigit
    //   20
    //   21
    //   22
    //   23
    fn scan_unpadded_hour<Char: AsRef<[u32]>>(str: Char, s: i32) -> i32 {
        let mut dummy = 0;
        let len = scan_two_digits_expect_range(str.as_ref(), s, 10, 23, &mut dummy);
        if len > 0 { return len; }
        if str.as_ref().len() >= (s + 1) as usize && IsDecimalDigit(str.as_ref()[s as usize]) {
            1
        } else {
            0
        }
    }

    // MinuteSecond:
    //   [0 1 2 3 4 5] Digit
    fn scan_minute_second<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(str, s, 0, 59, out)
    }

    // For the forward production in the grammar such as
    // ProductionB:
    //   ProductionT
    macro_rules! scan_forward {
        ($B:ident, $T:ident, $R:ty) => {
            fn $B<Char: AsRef<[u32]>>(str: Char, s: i32, r: &mut $R) -> i32 {
                $T(str, s, r)
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
            fn $B<Char: AsRef<[u32]>>(str: Char, s: i32, r: &mut $R) -> i32 {
                let mut len;
                if { len = $T1(str.as_ref(), s, r); len > 0 } { return len; }
                $T2(str, s, r)
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
    fn scan_time_second<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(str, s, 0, 60, out)
    }

    const K_POWER_OF_TEN: [i32; 9] = [1, 10, 100, 1000, 10000,
        100000, 1000000, 10000000, 100000000];

    // FractionalPart : Digit{1,9}
    fn scan_fractional_part<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        let mut cur = s;
        let str_slice = str.as_ref();
        if str_slice.len() < (cur + 1) as usize || !IsDecimalDigit(str_slice[cur as usize]) { return 0; }
        *out = to_int(str_slice[cur as usize]);
        cur += 1;
        while (cur < str_slice.len() as i32 && ((cur - s) < 9) && IsDecimalDigit(str_slice[cur as usize])) {
            *out = 10 * (*out) + to_int(str_slice[cur as usize]);
            cur += 1;
        }
        *out *= K_POWER_OF_TEN[9 - ((cur - s) as usize)];
        cur - s
    }

    // TimeFraction: FractionalPart
    scan_forward!(scan_time_fractional_part, scan_fractional_part, i32);

    // Fraction: DecimalSeparator FractionalPart
    // DecimalSeparator: one of , .
    fn scan_fraction<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        let str_slice = str.as_ref();
        if str_slice.len() < (s + 2) as usize || (!is_decimal_separator(str_slice[s as usize])) { return 0; }
        let len = scan_fractional_part(str.as_ref(), s + 1, out);
        if len == 0 { return 0; }
        len + 1
    }

    // TimeFraction: DecimalSeparator TimeFractionalPart
    // DecimalSeparator: one of , .
    fn scan_time_fraction<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        let str_slice = str.as_ref();
        if str_slice.len() < (s + 2) as usize || (!is_decimal_separator(str_slice[s as usize])) { return 0; }
        let len = scan_time_fractional_part(str.as_ref(), s + 1, out);
        if len == 0 { return 0; }
        len + 1
    }

    fn scan_time_fraction_result<Char: AsRef<[u32]>>(str: Char, s: i32,
                                 r: &mut ParsedISO8601Result) -> i32 {
        scan_time_fraction(str, s, &mut r.time_nanosecond)
    }

    // TimeSpec:
    //  TimeHour
    //  TimeHour : TimeMinute
    //  TimeHour : TimeMinute : TimeSecond [TimeFraction]
    //  TimeHour TimeMinute
    //  TimeHour TimeMinute TimeSecond [TimeFraction]
    fn scan_time_spec<Char: AsRef<[u32]>>(str: Char, s: i32,
                         r: &mut ParsedISO8601Result) -> i32 {
        let mut time_hour = 0;
        let mut time_minute = 0;
        let mut time_second = 0;
        let mut len = 0;
        let mut cur = s;
        if { len = scan_time_hour(str.as_ref(), cur, &mut time_hour); len == 0 } { return 0; }
        cur += len;
        let str_slice = str.as_ref();
        if (cur + 1) > str_slice.len() as i32 {
            // TimeHour
            r.time_hour = time_hour;
            return cur - s;
        }
        if str_slice[cur as usize] == ':' as u32 {
            cur += 1;
            if { len = scan_time_minute(str.as_ref(), cur, &mut time_minute); len == 0 } { return 0; }
            cur += len;
            if (cur + 1) > str_slice.len() as i32 || (str_slice[cur as usize] != ':' as u32) {
                // TimeHour : TimeMinute
                r.time_hour = time_hour;
                r.time_minute = time_minute;
                return cur - s;
            }
            cur += 1;
            if { len = scan_time_second(str.as_ref(), cur, &mut time_second); len == 0 } { return 0; }
        } else {
            if { len = scan_time_minute(str.as_ref(), cur, &mut time_minute); len == 0 } {
                // TimeHour
                r.time_hour = time_hour;
                return cur - s;
            }
            cur += len;
            if { len = scan_time_second(str.as_ref(), cur, &mut time_second); len == 0 } {
                // TimeHour TimeMinute
                r.time_hour = time_hour;
                r.time_minute = time_minute;
                return cur - s;
            }
        }
        cur += len;
        len = scan_time_fraction_result(str.as_ref(), cur, r);
        r.time_hour = time_hour;
        r.time_minute = time_minute;
        r.time_second = time_second;
        cur += len;
        return cur - s;
    }

    // TimeSpecSeparator: DateTimeSeparator TimeSpec
    // DateTimeSeparator: SPACE, 't', or 'T'
    fn scan_time_spec_separator<Char: AsRef<[u32]>>(str: Char, s: i32,
                                  r: &mut ParsedISO8601Result) -> i32 {
        let str_slice = str.as_ref();
        if (!(((s + 1) < str_slice.len() as i32) && is_date_time_separator(str_slice[s as usize]))) { return 0; }
        let len = scan_time_spec(str.as_ref(), s + 1, r);
        if len == 0 {
            0
        } else {
            len + 1
        }
    }

    // DateExtendedYear: Sign Digit Digit Digit Digit Digit Digit
    fn scan_date_extended_year<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        let str_slice = str.as_ref();
        if str_slice.len() < (s + 7) as usize { return 0; }
        if is_sign(str_slice[s as usize]) && IsDecimalDigit(str_slice[s as usize + 1]) &&
            IsDecimalDigit(str_slice[s as usize + 2]) && IsDecimalDigit(str_slice[s as usize + 3]) &&
            IsDecimalDigit(str_slice[s as usize + 4]) && IsDecimalDigit(str_slice[s as usize + 5]) &&
            IsDecimalDigit(str_slice[s as usize + 6]) {
            let sign = if canonical_sign(str_slice[s as usize]) == '-' as u32 { -1 } else { 1 };
            *out = sign * (to_int(str_slice[s as usize + 1]) * 100000 + to_int(str_slice[s as usize + 2]) * 10000 +
                           to_int(str_slice[s as usize + 3]) * 1000 + to_int(str_slice[s as usize + 4]) * 100 +
                           to_int(str_slice[s as usize + 5]) * 10 + to_int(str_slice[s as usize + 6]));
            // In the end of #sec-temporal-iso8601grammar
            // It is a Syntax Error if DateExtendedYear is "-000000" or "−000000"
            // (U+2212 MINUS SIGN followed by 000000).
            if sign == -1 && *out == 0 { return 0; }
            7
        } else {
            0
        }
    }

    // DateFourDigitYear: Digit Digit Digit Digit
    fn scan_date_four_digit_year<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        let str_slice = str.as_ref();
        if str_slice.len() < (s + 4) as usize { return 0; }
        if IsDecimalDigit(str_slice[s as usize]) && IsDecimalDigit(str_slice[s as usize + 1]) &&
            IsDecimalDigit(str_slice[s as usize + 2]) && IsDecimalDigit(str_slice[s as usize + 3]) {
            *out = to_int(str_slice[s as usize]) * 1000 + to_int(str_slice[s as usize + 1]) * 100 +
                   to_int(str_slice[s as usize + 2]) * 10 + to_int(str_slice[s as usize + 3]);
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
    fn scan_date_month<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(str, s, 1, 12, out)
    }

    // DateDay:
    //   0 NonzeroDigit
    //   1 Digit
    //   2 Digit
    //   30
    //   31
    fn scan_date_day<Char: AsRef<[u32]>>(str: Char, s: i32, out: &mut i32) -> i32 {
        scan_two_digits_expect_range(str, s, 1, 31, out)
    }

    // Date:
    //   DateYear - DateMonth - DateDay
    //   DateYear DateMonth DateDay
    fn scan_date<Char: AsRef<[u32]>>(str: Char, s: i32, r: &mut ParsedISO8601Result) -> i32 {
        let mut date_year = 0;
        let mut date_month = 0;
        let mut date_day = 0;
        let mut cur = s;
        let mut len = 0;
        if { len = scan_date_year(str.as_ref(), cur, &mut date_year); len == 0 } { return 0; }
        let str_slice = str.as_ref();
        if { cur += len; (cur + 1) > str_slice.len() as i32 } { return 0; }
        if str_slice[cur
