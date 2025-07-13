// Converted from V8 C++ source files:
// Header: string-builder-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod string_builder_inl {
    use crate::execution::isolate::Isolate;
    use crate::handles::handles_inl::Handle;
    use crate::objects::string_inl::{SeqOneByteString, SeqString, SeqTwoByteString, String};
    use crate::strings::string_builder::{
        FixedArrayBuilder, IncrementalStringBuilder, ReplacementStringBuilder,
    };
    use std::string::String as StdString;
    use crate::bigint::div_helpers::kIntToStringViewBufferSize;
    use crate::bigint::div_helpers::IntToStringView;
    use crate::objects::fixed_array::FixedArray;
    use crate::objects::smi::Smi;
    use crate::strings::uri::base;
    use crate::factory::factory::Factory;
    use std::string::String;
    use std::borrow::Borrow;

    pub const K_STRING_BUILDER_CONCAT_HELPER_LENGTH_BITS: i32 = 11;
    pub const K_STRING_BUILDER_CONCAT_HELPER_POSITION_BITS: i32 = 19;

    pub struct StringBuilderSubstringLength {}

    impl StringBuilderSubstringLength {
        pub fn is_valid(length: i32) -> bool {
            length >= 0 && length < (1 << K_STRING_BUILDER_CONCAT_HELPER_LENGTH_BITS)
        }

        pub fn encode(length: i32) -> i32 {
            length
        }
    }

    pub struct StringBuilderSubstringPosition {}

    impl StringBuilderSubstringPosition {
        pub fn is_valid(position: i32) -> bool {
            position >= 0 && position < (1 << K_STRING_BUILDER_CONCAT_HELPER_POSITION_BITS)
        }

        pub fn encode(position: i32) -> i32 {
            position << K_STRING_BUILDER_CONCAT_HELPER_LENGTH_BITS
        }
    }

    pub fn string_builder_concat_helper<SinkChar>(
        special: &String,
        sink: &mut [SinkChar],
        fixed_array: &FixedArray,
        array_length: i32,
    ) {
        // Placeholder implementation. Replace with actual logic.
        println!("StringBuilderConcatHelper called");
    }

    pub fn string_builder_concat_length(
        special_length: i32,
        fixed_array: &FixedArray,
        array_length: i32,
        one_byte: &mut bool,
    ) -> i32 {
        // Placeholder implementation. Replace with actual logic.
        println!("StringBuilderConcatLength called");
        0
    }

    impl ReplacementStringBuilder {
        pub fn add_subject_slice(builder: &mut FixedArrayBuilder, from: i32, to: i32) {
            debug_assert!(from >= 0);
            let length = to - from;
            debug_assert!(length > 0);

            if StringBuilderSubstringLength::is_valid(length)
                && StringBuilderSubstringPosition::is_valid(from)
            {
                let encoded_slice =
                    StringBuilderSubstringLength::encode(length) | StringBuilderSubstringPosition::encode(from);
                builder.add(Smi::from_int(encoded_slice));
            } else {
                // Otherwise encode as two smis.
                builder.add(Smi::from_int(-length));
                builder.add(Smi::from_int(from));
            }
        }

        pub fn add_subject_slice_self(&mut self, from: i32, to: i32) {
            self.ensure_capacity(2); // Subject slices are encoded with up to two smis.
            Self::add_subject_slice(&mut self.array_builder_, from, to);
            self.increment_character_count(to - from);
        }
    }

    impl<SrcChar, DestChar> IncrementalStringBuilder {
        pub fn append(&mut self, c: SrcChar)
            where SrcChar: Into<u32>, DestChar: From<u32> + Copy,
        {
            debug_assert_eq!(
                self.encoding_ == String::ONE_BYTE_ENCODING,
                std::mem::size_of::<DestChar>() == 1
            );

            if std::mem::size_of::<DestChar>() == 1 {
                debug_assert_eq!(String::ONE_BYTE_ENCODING, self.encoding_);
                let part = unsafe { self.current_part_.as_mut().unwrap() };
                unsafe {
                    let seq_one_byte_string = &mut *(part as *mut String as *mut SeqOneByteString);
                    seq_one_byte_string.seq_one_byte_string_set(self.current_index_ as usize, c.into() as u8);
                }
                self.current_index_ += 1;
            } else {
                debug_assert_eq!(String::TWO_BYTE_ENCODING, self.encoding_);
                let part = unsafe { self.current_part_.as_mut().unwrap() };
                unsafe {
                    let seq_two_byte_string = &mut *(part as *mut String as *mut SeqTwoByteString);
                    seq_two_byte_string.seq_two_byte_string_set(self.current_index_ as usize, c.into() as u16);
                }
                self.current_index_ += 1;
            }

            if self.current_index_ == self.part_length_ {
                self.extend();
            }

            debug_assert!(self.has_valid_current_index());
        }

        pub fn append_character(&mut self, c: u8) {
            if self.encoding_ == String::ONE_BYTE_ENCODING {
                self.append::<u8, u8>(c);
            } else {
                self.append::<u8, base::uc16>(c);
            }
        }

        pub fn append_cstring_literal<const N: usize>(&mut self, literal: &[u8; N]) {
            // Note that the literal contains the zero char.
            let length = N - 1;
            assert!(length > 0);

            if length == 1 {
                self.append_character(literal[0]);
                return;
            }

            if self.encoding_ == String::ONE_BYTE_ENCODING && self.current_part_can_fit(N as i32) {
                unsafe {
                    let seq_one_byte_string = &mut *(self.current_part_.as_mut().unwrap() as *mut String as *mut SeqOneByteString);
                    seq_one_byte_string.seq_one_byte_string_set_chars(self.current_index_ as usize, literal.as_ptr() as *const u8, length);
                }
                self.current_index_ += length as i32;

                if self.current_index_ == self.part_length_ {
                    self.extend();
                }

                debug_assert!(self.has_valid_current_index());
                return;
            }

            self.append_cstring(literal.as_ptr());
        }

        pub fn append_cstring<SrcChar>(&mut self, s: *const SrcChar)
            where SrcChar: Into<u32> + Copy
        {
            if self.encoding_ == String::ONE_BYTE_ENCODING {
                let mut current = s;
                unsafe {
                    while *current as u32 != 0 {
                        self.append::<SrcChar, u8>(*current);
                        current = current.offset(1);
                    }
                }
            } else {
                let mut current = s;
                unsafe {
                    while *current as u32 != 0 {
                        self.append::<SrcChar, base::uc16>(*current);
                        current = current.offset(1);
                    }
                }
            }
        }

        pub fn append_string(&mut self, str: std::string_view::StringView) {
            let length = str.len() as u32;

            if self.encoding_ == String::ONE_BYTE_ENCODING && self.current_part_can_fit(length as i32) {
                unsafe {
                    let seq_one_byte_string = &mut *(self.current_part_.as_mut().unwrap() as *mut String as *mut SeqOneByteString);
                    seq_one_byte_string.seq_one_byte_string_set_chars(self.current_index_ as usize, str.data() as *const u8, length as usize);
                }
                self.current_index_ += str.len() as i32;

                if self.current_index_ == self.part_length_ {
                    self.extend();
                }

                debug_assert!(self.has_valid_current_index());
            } else {
                for i in 0..str.len() {
                    self.append_character(str[i] as u8);
                }
            }
        }

        pub fn append_int(&mut self, i: i32) {
            let mut buffer: [char; kIntToStringViewBufferSize as usize] = ['\0'; kIntToStringViewBufferSize as usize];
            let str = IntToStringView(i, &mut buffer);
            self.append_string(str);
        }

        pub fn escaped_length_if_current_part_fits(&self, length: i32) -> i32 {
            if length > IncrementalStringBuilder::K_MAX_PART_LENGTH {
                return 0;
            }

            // The worst case length of an escaped character is 6. Shifting the remaining
            // string length right by 3 is a more pessimistic estimate, but faster to
            // calculate.
            assert!((IncrementalStringBuilder::K_MAX_PART_LENGTH << 3) <= String::K_MAX_LENGTH);

            // This shift will not overflow because length is already less than the
            // maximum part length.
            let worst_case_length = length << 3;

            if self.current_part_can_fit(worst_case_length) {
                worst_case_length
            } else {
                0
            }
        }

        // Change encoding to two-byte.
        pub fn change_encoding(&mut self) {
            debug_assert_eq!(String::ONE_BYTE_ENCODING, self.encoding_);
            self.shrink_current_part();
            self.encoding_ = String::TWO_BYTE_ENCODING;
            self.extend();
        }

         pub fn factory(&mut self) -> *mut Factory {
            self.isolate_.factory() as *mut Factory
        }

        pub fn shrink_current_part(&mut self) {
            debug_assert!(self.current_index_ < self.part_length_);
            let isolate = self.isolate_;
            let current_part = unsafe { self.current_part_.as_mut().unwrap() };
            unsafe {
                let seq_string = &mut *(current_part as *mut String as *mut SeqString);
                self.set_current_part(SeqString::truncate(
                   isolate,
                    seq_string,
                    self.current_index_ as usize
                ));
            }
        }
    }
}

