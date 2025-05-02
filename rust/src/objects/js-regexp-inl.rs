// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod js_regexp {
    use crate::objects::{js_array::JSArray, object::Object, smi::Smi, string::String, trusted_byte_array::TrustedByteArray};
    use std::convert::TryFrom;
    use std::ops::{BitAnd, BitOr};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Flags(i32);

    impl Flags {
        pub const kNone: Flags = Flags(0);
        pub const kGlobal: Flags = Flags(1);
        pub const kIgnoreCase: Flags = Flags(2);
        pub const kMultiline: Flags = Flags(4);
        pub const kSticky: Flags = Flags(8);
        pub const kUnicode: Flags = Flags(16);
        pub const kDotAll: Flags = Flags(32);
        pub const kHasIndices: Flags = Flags(64);

        pub fn contains(&self, other: Flags) -> bool {
            (self.0 & other.0) == other.0
        }
    }

    impl BitOr for Flags {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            Flags(self.0 | other.0)
        }
    }

    impl BitAnd for Flags {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            Flags(self.0 & other.0)
        }
    }

    impl From<i32> for Flags {
        fn from(value: i32) -> Self {
            Flags(value)
        }
    }

    impl TryFrom<Flags> for i32 {
        type Error = std::num::TryFromIntError;

        fn try_from(flags: Flags) -> Result<Self, Self::Error> {
            i32::try_from(flags.0)
        }
    }

    pub struct JSRegExp {
        //TODO: Add TorqueGeneratedClass fields, which are not currently defined
    }

    impl JSRegExp {
        pub fn source(&self) -> String {
            //TODO: Access TorqueGeneratedClass::source() and cast to String.
            String {} // Placeholder
        }

        pub fn flags(&self) -> Flags {
            //TODO: Access TorqueGeneratedClass::flags() and cast to Smi, then to Flags.
            Flags::kNone // Placeholder
        }

        pub fn data(&self) -> RegExpData {
            RegExpData{} // Placeholder
        }

        pub fn last_index(&self) -> Object{
            Object{}
        }

        pub fn flags_to_string(flags: Flags, out_buffer: &mut FlagsBuffer) -> String {
            let mut cursor = 0;
            let buffer = out_buffer;

            if flags.contains(Flags::kGlobal) {
                buffer.0[cursor] = 'g';
                cursor += 1;
            }
            if flags.contains(Flags::kIgnoreCase) {
                buffer.0[cursor] = 'i';
                cursor += 1;
            }
            if flags.contains(Flags::kMultiline) {
                buffer.0[cursor] = 'm';
                cursor += 1;
            }
            if flags.contains(Flags::kSticky) {
                buffer.0[cursor] = 'y';
                cursor += 1;
            }
            if flags.contains(Flags::kUnicode) {
                buffer.0[cursor] = 'u';
                cursor += 1;
            }
            if flags.contains(Flags::kDotAll) {
                buffer.0[cursor] = 's';
                cursor += 1;
            }
            if flags.contains(Flags::kHasIndices) {
                buffer.0[cursor] = 'd';
                cursor += 1;
            }

            buffer.0[cursor] = '\0';
            String {} //Placeholder
        }

        pub fn escaped_pattern(&self) -> String {
            String {} // Placeholder
        }
    }

    pub struct JSRegExpResult {}
    pub struct JSRegExpResultIndices {}
    pub struct JSRegExpResultWithIndices {}

    pub struct RegExpData {}

    impl RegExpData {
        pub fn type_tag(&self) -> Type {
            Type::ATOM
        }
        pub fn set_type_tag(&self, _type: Type){

        }

        pub fn source(&self) -> String {
            String{}
        }
        pub fn flags(&self) -> Flags {
            Flags::kNone
        }

        pub fn set_flags(&self, _flags: Flags) {}

        pub fn wrapper(&self) -> RegExpDataWrapper{
            RegExpDataWrapper{}
        }
        
        pub fn capture_count(&self) -> i32 {
             match self.type_tag() {
                 Type::ATOM => 0,
                 Type::EXPERIMENTAL | Type::IRREGEXP => {
                     let ir_regexp_data = IrRegExpData{};
                     ir_regexp_data.capture_count()
                 }
             }
        }
    }

    pub struct RegExpDataWrapper {}
    impl RegExpDataWrapper {
        pub fn data(&self) -> RegExpData {
            RegExpData{}
        }
    }

    pub struct AtomRegExpData {}
    impl AtomRegExpData {
        pub fn pattern(&self) -> String{
            String{}
        }
    }

    pub struct IrRegExpData {}

    impl IrRegExpData {
        pub fn latin1_code(&self) -> Code{
            Code{}
        }
        pub fn uc16_code(&self) -> Code{
            Code{}
        }
        pub fn has_code(&self, _is_one_byte: bool) -> bool {
            false
        }

        pub fn set_code(&self, _is_one_byte: bool, _code: Code){

        }

        pub fn code(&self, _isolate: IsolateForSandbox, _is_one_byte: bool) -> Code {
            Code{}
        }

        pub fn latin1_bytecode(&self) -> TrustedByteArray{
            TrustedByteArray{}
        }

        pub fn uc16_bytecode(&self) -> TrustedByteArray{
            TrustedByteArray{}
        }
        pub fn has_bytecode(&self, _is_one_byte: bool) -> bool {
            false
        }

        pub fn clear_bytecode(&self, _is_one_byte: bool) {}

        pub fn set_bytecode(&self, _is_one_byte: bool, _bytecode: TrustedByteArray) {}

        pub fn bytecode(&self, _is_one_byte: bool) -> TrustedByteArray {
            TrustedByteArray{}
        }
        pub fn capture_name_map(&self) -> Object{
            Object{}
        }

        pub fn set_capture_name_map(&self, _capture_name_map: DirectHandleFixedArray) {}

        pub fn max_register_count(&self) -> i32 {
            0
        }
        pub fn capture_count(&self) -> i32 {
            0
        }
        pub fn ticks_until_tier_up(&self) -> i32 {
            0
        }
        pub fn backtrack_limit(&self) -> i32 {
            0
        }
    }

    pub struct DirectHandleFixedArray {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Type {
        ATOM,
        EXPERIMENTAL,
        IRREGEXP,
    }

    pub struct FlagsBuffer([char; 8]);

    impl FlagsBuffer {
        pub fn new() -> Self {
            FlagsBuffer(['\0'; 8])
        }

        pub fn begin(&self) -> *const char {
            self.0.as_ptr() as *const char
        }
    }

    pub struct IsolateForSandbox{}
    pub struct Code {}

    pub struct ExposedTrustedObject{}
}

pub mod objects {
    pub mod js_array {
        pub struct JSArray {}
    }
    pub mod object {
        pub struct Object {}
    }
    pub mod smi {
        pub struct Smi {}
    }
    pub mod string {
        pub struct String {}
    }
    pub mod trusted_byte_array {
        pub struct TrustedByteArray {}
    }
}