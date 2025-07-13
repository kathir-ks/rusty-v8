// Converted from V8 C++ source files:
// Header: abstract-code-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/abstract-code.h
use crate::objects::objects::HeapObject;
use crate::objects::objects::Map;
use crate::objects::objects::Tagged;
use crate::objects::objects::Object;
use crate::objects::string::v8;
use crate::objects::objects::Address;
use crate::objects::objects::CodeKind;
use crate::objects::code::Builtin;
use crate::objects::shared_function_info::SharedFunctionInfo;
use crate::objects::trusted_space::TrustedByteArray;
use crate::objects::bytecode_array::BytecodeArray;
use crate::objects::code::Code;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Isolate {}
pub struct AbstractCode {
    dummy: i32,
}

pub struct PtrComprCageBase {}

impl AbstractCode {
    pub fn InstructionSize(&self, cage_base: PtrComprCageBase) -> i32 {
        let map_object = self.map(cage_base);
        if self.IsCode(map_object) {
            self.GetCode().instruction_size()
        } else {
            self.GetBytecodeArray().length()
        }
    }

    pub fn SourcePositionTable(&self, isolate: &Isolate, sfi: Tagged<SharedFunctionInfo>) -> Tagged<TrustedByteArray> {
        let map_object = self.map(isolate);
        if self.IsCode(map_object) {
            self.GetCode().SourcePositionTable(isolate, sfi)
        } else {
            self.GetBytecodeArray().SourcePositionTable(isolate)
        }
    }

    pub fn SizeIncludingMetadata(&self, cage_base: PtrComprCageBase) -> i32 {
        let map_object = self.map(cage_base);
        if self.IsCode(map_object) {
            self.GetCode().SizeIncludingMetadata()
        } else {
            self.GetBytecodeArray().SizeIncludingMetadata()
        }
    }

    pub fn InstructionStart(&self, cage_base: PtrComprCageBase) -> Address {
        let map_object = self.map(cage_base);
        if self.IsCode(map_object) {
            self.GetCode().instruction_start()
        } else {
            self.GetBytecodeArray().GetFirstBytecodeAddress()
        }
    }

    pub fn InstructionEnd(&self, cage_base: PtrComprCageBase) -> Address {
        let map_object = self.map(cage_base);
        if self.IsCode(map_object) {
            self.GetCode().instruction_end()
        } else {
            let bytecode_array = self.GetBytecodeArray();
            bytecode_array.GetFirstBytecodeAddress() + bytecode_array.length()
        }
    }

    pub fn contains(&self, isolate: &Isolate, inner_pointer: Address) -> bool {
        let cage_base = PtrComprCageBase {};
        let map_object = self.map(cage_base);
        if self.IsCode(map_object) {
            self.GetCode().contains(isolate, inner_pointer)
        } else {
            (self.address() <= inner_pointer) && (inner_pointer <= self.address() + self.Size(cage_base))
        }
    }

    pub fn kind(&self, cage_base: PtrComprCageBase) -> CodeKind {
        let map_object = self.map(cage_base);
        if self.IsCode(map_object) {
            self.GetCode().kind()
        } else {
            CodeKind::INTERPRETED_FUNCTION
        }
    }

    pub fn builtin_id(&self, cage_base: PtrComprCageBase) -> Builtin {
        let map_object = self.map(cage_base);
        if self.IsCode(map_object) {
            self.GetCode().builtin_id()
        } else {
            Builtin::kNoBuiltinId
        }
    }

    pub fn has_instruction_stream(&self, cage_base: PtrComprCageBase) -> bool {
        self.IsCode(self.map(cage_base)) && self.GetCode().has_instruction_stream()
    }

    pub fn GetCode(&self) -> Tagged<Code> {
        Tagged::<Code> { }
    }

    pub fn GetBytecodeArray(&self) -> Tagged<BytecodeArray> {
        Tagged::<BytecodeArray> { }
    }

    fn map(&self, _cage_base: PtrComprCageBase) -> Tagged<Map> {
        Tagged::<Map> {}
    }
    fn map(&self, _isolate: &Isolate) -> Tagged<Map> {
        Tagged::<Map> {}
    }

    fn IsCode(&self, _map_object: Tagged<Map>) -> bool {
        true
    }

    fn address(&self) -> Address {
        Address {}
    }

    fn Size(&self, _cage_base: PtrComprCageBase) -> Address {
        Address {}
    }

    fn Size(&self, _cage_base: PtrComprCageBase) -> i32 {
        1
    }
}
//impl OBJECT_CONSTRUCTORS_IMPL for AbstractCode{
//    fn new() -> Self {
//        AbstractCode{dummy: 1}
//    }
//}
pub trait OBJECT_CONSTRUCTORS_IMPL {
    fn new() -> Self;
}

impl OBJECT_CONSTRUCTORS_IMPL for AbstractCode {
    fn new() -> Self {
        AbstractCode { dummy: 1 }
    }
}
