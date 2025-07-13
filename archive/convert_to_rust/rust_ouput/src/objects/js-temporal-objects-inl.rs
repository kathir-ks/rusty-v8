// Converted from V8 C++ source files:
// Header: js-temporal-objects-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod js_temporal_objects_inl {
    use crate::api::api_inl::*;
    use crate::objects::js_temporal_objects::*;
    use crate::objects::objects_inl::*;
    use crate::objects::object_macros::*;

    pub struct JSTemporalPlainDate {
        year_month_day: i32,
    }

    impl JSTemporalPlainDate {
        pub fn set_iso_year(&mut self, field: i32) {
            assert!(field >= -271821);
            assert!(field <= 275760);
            let hints = self.year_month_day;
            // Mask out unrelated bits
            let mut field_masked = field & !(((-1i32) as u32 ^ ((-1i32) as u32) << IsoYearBits::kSize) as i32);
            let hints_updated = IsoYearBits::update(hints, field_masked);
            self.year_month_day = hints_updated;
        }
        pub fn iso_year(&self) -> i32 {
            let mut v = IsoYearBits::decode(self.year_month_day);
            /* Restore bits for negative values based on the MSB in that field */
            v |= if (1 << (IsoYearBits::kSize - 1) & v) != 0 {
                ((-1i32) as u32 << IsoYearBits::kSize) as i32
            } else {
                0
            };
            assert!(v >= -271821);
            assert!(v <= 275760);
            v
        }

        pub fn set_iso_month(&mut self, field: i32) {
            assert!(field >= 1);
            assert!(field <= 12);
            let hints = self.year_month_day;
            let hints_updated = IsoMonthBits::update(hints, field);
            self.year_month_day = hints_updated;
        }
        pub fn iso_month(&self) -> i32 {
            let v = IsoMonthBits::decode(self.year_month_day);
            assert!(v >= 1);
            assert!(v <= 12);
            v
        }

        pub fn set_iso_day(&mut self, field: i32) {
            assert!(field >= 1);
            assert!(field <= 31);
            let hints = self.year_month_day;
            let hints_updated = IsoDayBits::update(hints, field);
            self.year_month_day = hints_updated;
        }
        pub fn iso_day(&self) -> i32 {
            let v = IsoDayBits::decode(self.year_month_day);
            assert!(v >= 1);
            assert!(v <= 31);
            v
        }
    }

    pub struct JSTemporalPlainDateTime {
        year_month_day: i32,
        hour_minute_second: i32,
        second_parts: i32,
    }

    impl JSTemporalPlainDateTime {
        pub fn set_iso_year(&mut self, field: i32) {
            assert!(field >= -271821);
            assert!(field <= 275760);
            let hints = self.year_month_day;
            // Mask out unrelated bits
            let mut field_masked = field & !(((-1i32) as u32 ^ ((-1i32) as u32) << IsoYearBits::kSize) as i32);
            let hints_updated = IsoYearBits::update(hints, field_masked);
            self.year_month_day = hints_updated;
        }
        pub fn iso_year(&self) -> i32 {
            let mut v = IsoYearBits::decode(self.year_month_day);
            /* Restore bits for negative values based on the MSB in that field */
            v |= if (1 << (IsoYearBits::kSize - 1) & v) != 0 {
                ((-1i32) as u32 << IsoYearBits::kSize) as i32
            } else {
                0
            };
            assert!(v >= -271821);
            assert!(v <= 275760);
            v
        }

        pub fn set_iso_month(&mut self, field: i32) {
            assert!(field >= 1);
            assert!(field <= 12);
            let hints = self.year_month_day;
            let hints_updated = IsoMonthBits::update(hints, field);
            self.year_month_day = hints_updated;
        }
        pub fn iso_month(&self) -> i32 {
            let v = IsoMonthBits::decode(self.year_month_day);
            assert!(v >= 1);
            assert!(v <= 12);
            v
        }

        pub fn set_iso_day(&mut self, field: i32) {
            assert!(field >= 1);
            assert!(field <= 31);
            let hints = self.year_month_day;
            let hints_updated = IsoDayBits::update(hints, field);
            self.year_month_day = hints_updated;
        }
        pub fn iso_day(&self) -> i32 {
            let v = IsoDayBits::decode(self.year_month_day);
            assert!(v >= 1);
            assert!(v <= 31);
            v
        }

        pub fn set_iso_hour(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 23);
            let hints = self.hour_minute_second;
            let hints_updated = IsoHourBits::update(hints, field);
            self.hour_minute_second = hints_updated;
        }
        pub fn iso_hour(&self) -> i32 {
            let v = IsoHourBits::decode(self.hour_minute_second);
            assert!(v >= 0);
            assert!(v <= 23);
            v
        }

        pub fn set_iso_minute(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 59);
            let hints = self.hour_minute_second;
            let hints_updated = IsoMinuteBits::update(hints, field);
            self.hour_minute_second = hints_updated;
        }
        pub fn iso_minute(&self) -> i32 {
            let v = IsoMinuteBits::decode(self.hour_minute_second);
            assert!(v >= 0);
            assert!(v <= 59);
            v
        }

        pub fn set_iso_second(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 59);
            let hints = self.hour_minute_second;
            let hints_updated = IsoSecondBits::update(hints, field);
            self.hour_minute_second = hints_updated;
        }
        pub fn iso_second(&self) -> i32 {
            let v = IsoSecondBits::decode(self.hour_minute_second);
            assert!(v >= 0);
            assert!(v <= 59);
            v
        }

        pub fn set_iso_millisecond(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 999);
            let hints = self.second_parts;
            let hints_updated = IsoMillisecondBits::update(hints, field);
            self.second_parts = hints_updated;
        }
        pub fn iso_millisecond(&self) -> i32 {
            let v = IsoMillisecondBits::decode(self.second_parts);
            assert!(v >= 0);
            assert!(v <= 999);
            v
        }

        pub fn set_iso_microsecond(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 999);
            let hints = self.second_parts;
            let hints_updated = IsoMicrosecondBits::update(hints, field);
            self.second_parts = hints_updated;
        }
        pub fn iso_microsecond(&self) -> i32 {
            let v = IsoMicrosecondBits::decode(self.second_parts);
            assert!(v >= 0);
            assert!(v <= 999);
            v
        }

        pub fn set_iso_nanosecond(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 999);
            let hints = self.second_parts;
            let hints_updated = IsoNanosecondBits::update(hints, field);
            self.second_parts = hints_updated;
        }
        pub fn iso_nanosecond(&self) -> i32 {
            let v = IsoNanosecondBits::decode(self.second_parts);
            assert!(v >= 0);
            assert!(v <= 999);
            v
        }
    }

    pub struct JSTemporalPlainMonthDay {
        year_month_day: i32,
    }

    impl JSTemporalPlainMonthDay {
        pub fn set_iso_year(&mut self, field: i32) {
            assert!(field >= -271821);
            assert!(field <= 275760);
            let hints = self.year_month_day;
            // Mask out unrelated bits
            let mut field_masked = field & !(((-1i32) as u32 ^ ((-1i32) as u32) << IsoYearBits::kSize) as i32);
            let hints_updated = IsoYearBits::update(hints, field_masked);
            self.year_month_day = hints_updated;
        }
        pub fn iso_year(&self) -> i32 {
            let mut v = IsoYearBits::decode(self.year_month_day);
            /* Restore bits for negative values based on the MSB in that field */
            v |= if (1 << (IsoYearBits::kSize - 1) & v) != 0 {
                ((-1i32) as u32 << IsoYearBits::kSize) as i32
            } else {
                0
            };
            assert!(v >= -271821);
            assert!(v <= 275760);
            v
        }

        pub fn set_iso_month(&mut self, field: i32) {
            assert!(field >= 1);
            assert!(field <= 12);
            let hints = self.year_month_day;
            let hints_updated = IsoMonthBits::update(hints, field);
            self.year_month_day = hints_updated;
        }
        pub fn iso_month(&self) -> i32 {
            let v = IsoMonthBits::decode(self.year_month_day);
            assert!(v >= 1);
            assert!(v <= 12);
            v
        }

        pub fn set_iso_day(&mut self, field: i32) {
            assert!(field >= 1);
            assert!(field <= 31);
            let hints = self.year_month_day;
            let hints_updated = IsoDayBits::update(hints, field);
            self.year_month_day = hints_updated;
        }
        pub fn iso_day(&self) -> i32 {
            let v = IsoDayBits::decode(self.year_month_day);
            assert!(v >= 1);
            assert!(v <= 31);
            v
        }
    }

    pub struct JSTemporalPlainTime {
        hour_minute_second: i32,
        second_parts: i32,
    }

    impl JSTemporalPlainTime {
        pub fn set_iso_hour(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 23);
            let hints = self.hour_minute_second;
            let hints_updated = IsoHourBits::update(hints, field);
            self.hour_minute_second = hints_updated;
        }
        pub fn iso_hour(&self) -> i32 {
            let v = IsoHourBits::decode(self.hour_minute_second);
            assert!(v >= 0);
            assert!(v <= 23);
            v
        }

        pub fn set_iso_minute(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 59);
            let hints = self.hour_minute_second;
            let hints_updated = IsoMinuteBits::update(hints, field);
            self.hour_minute_second = hints_updated;
        }
        pub fn iso_minute(&self) -> i32 {
            let v = IsoMinuteBits::decode(self.hour_minute_second);
            assert!(v >= 0);
            assert!(v <= 59);
            v
        }

        pub fn set_iso_second(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 59);
            let hints = self.hour_minute_second;
            let hints_updated = IsoSecondBits::update(hints, field);
            self.hour_minute_second = hints_updated;
        }
        pub fn iso_second(&self) -> i32 {
            let v = IsoSecondBits::decode(self.hour_minute_second);
            assert!(v >= 0);
            assert!(v <= 59);
            v
        }

        pub fn set_iso_millisecond(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 999);
            let hints = self.second_parts;
            let hints_updated = IsoMillisecondBits::update(hints, field);
            self.second_parts = hints_updated;
        }
        pub fn iso_millisecond(&self) -> i32 {
            let v = IsoMillisecondBits::decode(self.second_parts);
            assert!(v >= 0);
            assert!(v <= 999);
            v
        }

        pub fn set_iso_microsecond(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 999);
            let hints = self.second_parts;
            let hints_updated = IsoMicrosecondBits::update(hints, field);
            self.second_parts = hints_updated;
        }
        pub fn iso_microsecond(&self) -> i32 {
            let v = IsoMicrosecondBits::decode(self.second_parts);
            assert!(v >= 0);
            assert!(v <= 999);
            v
        }

        pub fn set_iso_nanosecond(&mut self, field: i32) {
            assert!(field >= 0);
            assert!(field <= 999);
            let hints = self.second_parts;
            let hints_updated = IsoNanosecondBits::update(hints, field);
            self.second_parts = hints_updated;
        }
        pub fn iso_nanosecond(&self) -> i32 {
            let v = IsoNanosecondBits::decode(self.second_parts);
            assert!(v >= 0);
            assert!(v <= 999);
            v
        }
    }

    pub struct JSTemporalPlainYearMonth {
        year_month_day: i32,
    }

    impl JSTemporalPlainYearMonth {
        pub fn set_iso_year(&mut self, field: i32) {
            assert!(field >= -271821);
            assert!(field <= 275760);
            let hints = self.year_month_day;
            // Mask out unrelated bits
            let mut field_masked = field & !(((-1i32) as u32 ^ ((-1i32) as u32) << IsoYearBits::kSize) as i32);
            let hints_updated = IsoYearBits::update(hints, field_masked);
            self.year_month_day = hints_updated;
        }
        pub fn iso_year(&self) -> i32 {
            let mut v = IsoYearBits::decode(self.year_month_day);
            /* Restore bits for negative values based on the MSB in that field */
            v |= if (1 << (IsoYearBits::kSize - 1) & v) != 0 {
                ((-1i32) as u32 << IsoYearBits::kSize) as i32
            } else {
                0
            };
            assert!(v >= -271821);
            assert!(v <= 275760);
            v
        }

        pub fn set_iso_month(&mut self, field: i32) {
            assert!(field >= 1);
            assert!(field <= 12);
            let hints = self.year_month_day;
            let hints_updated = IsoMonthBits::update(hints, field);
            self.year_month_day = hints_updated;
        }
        pub fn iso_month(&self) -> i32 {
            let v = IsoMonthBits::decode(self.year_month_day);
            assert!(v >= 1);
            assert!(v <= 12);
            v
        }
    }

    pub struct JSTemporalCalendar {
        flags: i32,
    }

    impl JSTemporalCalendar {
        pub fn set_calendar_index(&mut self, field: i32) {
            let hints = self.flags;
            let hints_updated = CalendarIndexBits::update(hints, field);
            self.flags = hints_updated;
        }
        pub fn calendar_index(&self) -> i32 {
            let v = CalendarIndexBits::decode(self.flags);
            v
        }
    }

    pub struct JSTemporalTimeZone {
        flags: i32,
        details: i32,
    }

    impl JSTemporalTimeZone {
        pub fn set_is_offset(&mut self, field: bool) {
             let current_flags = self.flags;
            self.flags = if field {
                current_flags | (1 << IsOffsetBit::kShift)
            } else {
                current_flags & !(1 << IsOffsetBit::kShift)
            };
        }
        pub fn is_offset(&self) -> bool {
            (self.flags >> IsOffsetBit::kShift) & 1 != 0
        }

        pub fn set_offset_milliseconds(&mut self, field: i32) {
            assert!(field >= -24 * 60 * 60 * 1000);
            assert!(field <= 24 * 60 * 60 * 1000);
            let hints = self.flags;
            // Mask out unrelated bits
            let mut field_masked = field & !(((-1i32) as u32 ^ ((-1i32) as u32) << OffsetMillisecondsOrTimeZoneIndexBits::kSize) as i32);
            let hints_updated = OffsetMillisecondsOrTimeZoneIndexBits::update(hints, field_masked);
            self.flags = hints_updated;
        }
        pub fn offset_milliseconds(&self) -> i32 {
            let mut v = OffsetMillisecondsOrTimeZoneIndexBits::decode(self.flags);
            /* Restore bits for negative values based on the MSB in that field */
            v |= if (1 << (OffsetMillisecondsOrTimeZoneIndexBits::kSize - 1) & v) != 0 {
                ((-1i32) as u32 << OffsetMillisecondsOrTimeZoneIndexBits::kSize) as i32
            } else {
                0
            };
            assert!(v >= -24 * 60 * 60 * 1000);
            assert!(v <= 24 * 60 * 60 * 1000);
            v
        }
         pub fn set_offset_sub_milliseconds(&mut self, field: i32) {
            assert!(field >= -1000000);
            assert!(field <= 1000000);
            let hints = self.details;
            // Mask out unrelated bits
            let mut field_masked = field & !(((-1i32) as u32 ^ ((-1i32) as u32) << OffsetSubMillisecondsBits::kSize) as i32);
            let hints_updated = OffsetSubMillisecondsBits::update(hints, field_masked);
            self.details = hints_updated;
        }
        pub fn offset_sub_milliseconds(&self) -> i32 {
            let mut v = OffsetSubMillisecondsBits::decode(self.details);
            /* Restore bits for negative values based on the MSB in that field */
            v |= if (1 << (OffsetSubMillisecondsBits::kSize - 1) & v) != 0 {
                ((-1i32) as u32 << OffsetSubMillisecondsBits::kSize) as i32
            } else {
                0
            };
            assert!(v >= -1000000);
            assert!(v <= 1000000);
            v
        }

        pub fn set_offset_milliseconds_or_time_zone_index(&mut self, field: i32) {
            let hints = self.flags;
            let hints_updated = OffsetMillisecondsOrTimeZoneIndexBits::update(hints, field);
            self.flags = hints_updated;
        }

        pub fn offset_milliseconds_or_time_zone_index(&self) -> i32 {
            let v = OffsetMillisecondsOrTimeZoneIndexBits::decode(self.flags);
            v
        }
    }
}
