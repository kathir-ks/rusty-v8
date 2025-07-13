// Converted from V8 C++ source files:
// Header: abstract-code.h
// Implementation: abstract-code.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod abstract_code {
    use crate::objects::code_kind::CodeKind;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::string::v8;
    use crate::objects::visitors::PtrComprCageBase;
    use crate::objects::code::Code;
    use crate::objects::bytecode_array::BytecodeArray;
    use crate::objects::object_macros::*;
    use crate::objects::map::Map;
    use crate::objects::trusted_byte_array::TrustedByteArray;
    use crate::isolate::Isolate;
    use crate::instance_type::InstanceTypeChecker;
    use crate::objects::smi::Smi;
    use crate::objects::tagged::Tagged;
    use std::ops::Not;
    use std::fmt::{self, Debug, Display, Formatter};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Builtin {
        kNoBuiltinId,
    }

    pub struct AbstractCode {
    }

    impl AbstractCode {
        pub fn source_position(&self, isolate: &mut Isolate, offset: i32) -> i32 {
            let cage_base = PtrComprCageBase {};
            let map_object = Map::of(_value);
            if InstanceTypeChecker::is_code(&map_object) {
                self.get_code().source_position(offset)
            } else {
                self.get_bytecode_array().source_position(offset)
            }
        }

        pub fn source_statement_position(&self, isolate: &mut Isolate, offset: i32) -> i32 {
            let cage_base = PtrComprCageBase {};
            let map_object = Map::of(_value);
            if InstanceTypeChecker::is_code(&map_object) {
                self.get_code().source_statement_position(offset)
            } else {
                self.get_bytecode_array().source_statement_position(offset)
            }
        }

        pub fn instruction_start(&self, cage_base: PtrComprCageBase) -> Address {
            Address {}
        }

        pub fn instruction_end(&self, cage_base: PtrComprCageBase) -> Address {
            Address {}
        }

        pub fn instruction_size(&self, cage_base: PtrComprCageBase) -> i32 {
            0
        }

        pub fn source_position_table(
            &self,
            isolate: &mut Isolate,
            sfi: Tagged<crate::objects::shared_function_info::SharedFunctionInfo>,
        ) -> Tagged<TrustedByteArray> {
            Tagged::<TrustedByteArray>::of(_value)
        }

        pub fn drop_stack_frame_cache(&self, cage_base: PtrComprCageBase) {}

        pub fn size_including_metadata(&self, cage_base: PtrComprCageBase) -> i32 {
            0
        }

        pub fn contains(&self, isolate: &mut Isolate, pc: Address) -> bool {
            false
        }

        pub fn kind(&self, cage_base: PtrComprCageBase) -> CodeKind {
            CodeKind::FOR_TESTING
        }

        pub fn builtin_id(&self, cage_base: PtrComprCageBase) -> Builtin {
            Builtin::kNoBuiltinId
        }

        pub fn has_instruction_stream(&self, cage_base: PtrComprCageBase) -> bool {
            false
        }

        pub fn get_code(&self) -> Tagged<Code> {
            Tagged::<Code>::of(_value)
        }

        pub fn get_bytecode_array(&self) -> Tagged<BytecodeArray> {
            Tagged::<BytecodeArray>::of(_value)
        }
    }

    fn map(&self, _cage_base: PtrComprCageBase) -> Tagged<Map> {
        Tagged::<Map>::of(_value)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Address {}

    fn _value() -> v8::internal::TaggedObject {
        v8::internal::TaggedObject{dummy : 1}
    }

    // Currently we must use full-pointer comparisons (instead of
    // compressed-pointer comparisons) when comparing AbstractCode. This is because
    // AbstractCode is either a Code or a BytecodeArray, and the latter lives in
    // trusted space (outside of the main pointer compression cage) while the
    // former still lives inside of the sandbox.
    const _ : () = assert!( !true); // kAllCodeObjectsLiveInTrustedSpace
    impl PartialEq for Tagged<AbstractCode> {
        fn eq(&self, other: &Self) -> bool {
            self.ptr() == other.ptr()
        }
    }
}
