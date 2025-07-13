// Converted from V8 C++ source files:
// Header: code-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use crate::objects::abstract_code::Builtin;
use crate::objects::code_kind::CodeKind;
use crate::objects::deoptimization_data_inl::DeoptimizationData;
use crate::objects::fixed_array_inl::Tagged;
use crate::objects::heap_object::HeapObject;
use crate::objects::instruction_stream_inl::InstructionStream;
use crate::objects::object_macros::*;
use crate::objects::trusted_object_inl::TrustedObject;
use crate::objects::visitors::JSDispatchHandle;
use crate::strings::string::String;
use std::mem::size_of;
pub struct V8 {}
pub struct IsolateForSandbox {}
pub struct Object {}
pub struct SharedFunctionInfo {}
pub struct Address {}
pub struct CodeWrapper {}
pub struct PtrComprCageBase {}
pub struct Map {}
pub struct RelaxedLoadTag {}
pub struct DisallowGarbageCollection {}
pub struct HeapLayout {}
pub struct Builtins {}
pub struct Label {}
pub struct Operand {}
pub struct Condition {}
pub struct InstructionOperand {}
pub struct OpIndex {}
pub struct ValueType {}
pub struct Label {}
pub struct InstructionBase {}
pub struct Bytecode {}
pub struct InstructionOperand {}
pub struct Register {}
pub struct DirectHandle<T> {}
pub struct Isolate {}
pub struct FullObjectSlot {}
pub struct RootVisitor {}
pub struct CodeEntrypointTag {}
pub enum LazyDeoptimizeReason {}
pub struct WriteBarrierMode {}
pub struct Heap {}
pub struct BytecodeOffset {}
pub struct TaggedPrimitiveHeapObject {}
pub struct JSHeapBroker {}
pub struct MapRef {}
pub struct ArrayBufferExtension {}
pub struct SafepointTableStackSlotsField_t {}
pub struct TieringBuiltin {}
pub struct IsolateGroup {}
pub struct SafepointTableStackSlotsField_t {}
pub struct CodePointerHandle {}
const kNullAddress: Address = Address {};
const kNullCodePointerHandle: CodePointerHandle = CodePointerHandle {};
const kBytecodeHandlerEntrypointTag: CodeEntrypointTag = CodeEntrypointTag {};
const kRegExpEntrypointTag: CodeEntrypointTag = CodeEntrypointTag {};
const kWasmEntrypointTag: CodeEntrypointTag = CodeEntrypointTag {};
const kJSEntrypointTag: CodeEntrypointTag = CodeEntrypointTag {};
const kDefaultCodeEntrypointTag: CodeEntrypointTag = CodeEntrypointTag {};
const kInt16Size: i32 = 2;
const kInt32Size: i32 = 4;
const kFunctionExitBytecodeOffset: i32 = -1;
pub enum BytecodeToPCPosition {
    kPcAtStartOfBytecode,
    kPcAtEndOfBytecode,
}
#[derive(Debug)]
pub enum Error {
    CastingError,
    DeoptimizationDataError,
    SourcePositionTableError,
    BytecodeOffsetTableError,
    HeapObjectError,
    UnwindingInfoError,
    Other(String),
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Other(error.to_string())
    }
}
macro_rules! OBJECT_CONSTRUCTORS_IMPL {
    ($name:ident, $parent:ident) => {
        impl $name {
            pub fn cast<'a>(obj: &'a $parent) -> Result<&'a Self, Error> {
                if obj.is_kind::<$name>() {
                    unsafe { Ok(&*(obj as *const $parent as *const Self)) }
                } else {
                    Err(Error::CastingError)
                }
            }
            pub fn unchecked_cast<'a>(obj: &'a $parent) -> &'a Self {
                unsafe { &*(obj as *const $parent as *const Self) }
            }
            fn is_kind<T>(&self) -> bool {
                std::any::TypeId::of::<Self>() == std::any::TypeId::of::<T>()
            }
        }
    };
}
OBJECT_CONSTRUCTORS_IMPL!(Code, TrustedObject);
OBJECT_CONSTRUCTORS_IMPL!(GcSafeCode, HeapObject);
impl GcSafeCode {
    pub fn UnsafeCastToCode(&self) -> Tagged<Code> {
        unsafe { std::mem::transmute_copy(self) }
    }
}
macro_rules! GCSAFE_CODE_FWD_ACCESSOR {
    ($ReturnType:ty, $Name:ident) => {
        impl GcSafeCode {
            pub fn $Name(&self) -> $ReturnType {
                self.UnsafeCastToCode().$Name()
            }
        }
    };
}
GCSAFE_CODE_FWD_ACCESSOR!(Address, instruction_start);
GCSAFE_CODE_FWD_ACCESSOR!(Address, instruction_end);
GCSAFE_CODE_FWD_ACCESSOR!(bool, is_builtin);
GCSAFE_CODE_FWD_ACCESSOR!(Builtin, builtin_id);
GCSAFE_CODE_FWD_ACCESSOR!(CodeKind, kind);
GCSAFE_CODE_FWD_ACCESSOR!(bool, is_interpreter_trampoline_builtin);
GCSAFE_CODE_FWD_ACCESSOR!(bool, is_baseline_trampoline_builtin);
GCSAFE_CODE_FWD_ACCESSOR!(bool, is_baseline_leave_frame_builtin);
GCSAFE_CODE_FWD_ACCESSOR!(bool, has_instruction_stream);
GCSAFE_CODE_FWD_ACCESSOR!(bool, is_maglevved);
GCSAFE_CODE_FWD_ACCESSOR!(bool, is_turbofanned);
GCSAFE_CODE_FWD_ACCESSOR!(bool, has_tagged_outgoing_params);
GCSAFE_CODE_FWD_ACCESSOR!(bool, marked_for_deoptimization);
GCSAFE_CODE_FWD_ACCESSOR!(Tagged<Object>, raw_instruction_stream);
GCSAFE_CODE_FWD_ACCESSOR!(u32, stack_slots);
GCSAFE_CODE_FWD_ACCESSOR!(u16, wasm_js_tagged_parameter_count);
GCSAFE_CODE_FWD_ACCESSOR!(u16, wasm_js_first_tagged_parameter);
GCSAFE_CODE_FWD_ACCESSOR!(Address, constant_pool);
GCSAFE_CODE_FWD_ACCESSOR!(Address, safepoint_table_address);
impl GcSafeCode {
    pub fn GetOffsetFromInstructionStart(&self, isolate: *mut Isolate, pc: Address) -> i32 {
        self.UnsafeCastToCode().GetOffsetFromInstructionStart(isolate, pc)
    }
    pub fn InstructionStart(&self, isolate: *mut Isolate, pc: Address) -> Address {
        self.UnsafeCastToCode().InstructionStart(isolate, pc)
    }
    pub fn InstructionEnd(&self, isolate: *mut Isolate, pc: Address) -> Address {
        self.UnsafeCastToCode().InstructionEnd(isolate, pc)
    }
    pub fn CanDeoptAt(&self, isolate: *mut Isolate, pc: Address) -> bool {
        if !self.UnsafeCastToCode().uses_deoptimization_data() {
            return false;
        }
        let deopt_data: Tagged<DeoptimizationData> =
            unsafe { std::mem::transmute_copy(&self.UnsafeCastToCode().unchecked_deoptimization_data()) };
        let code_start_address = self.instruction_start();
        for i in 0..deopt_data.DeoptCount() {
            if deopt_data.Pc(i).value() == -1 {
                continue;
            }
            let address = code_start_address + deopt_data.Pc(i).value();
            if address == pc
                && deopt_data.GetBytecodeOffsetOrBuiltinContinuationId(i) != BytecodeOffset::None()
            {
                return true;
            }
        }
        false
    }
    pub fn raw_instruction_stream(&self, code_cage_base: PtrComprCageBase) -> Tagged<Object> {
        self.UnsafeCastToCode().raw_instruction_stream(code_cage_base)
    }
}
macro_rules! INT_ACCESSORS {
    ($struct_name:ident, $field_name:ident, $offset:ident) => {
        impl $struct_name {
            pub fn $field_name(&self) -> i32 {
                unsafe {
                    let ptr = self as *const Self as *const i32;
                    ptr.add($offset as usize / std::mem::size_of::<i32>()).read()
                }
            }
            pub fn set_$field_name(&mut self, value: i32) {
                unsafe {
                    let ptr = self as *mut Self as *mut i32;
                    ptr.add($offset as usize / std::mem::size_of::<i32>())
                        .write(value);
                }
            }
        }
    };
}
INT_ACCESSORS!(Code, instruction_size, kInstructionSizeOffset);
INT_ACCESSORS!(Code, metadata_size, kMetadataSizeOffset);
INT_ACCESSORS!(Code, handler_table_offset, kHandlerTableOffsetOffset);
INT_ACCESSORS!(Code, code_comments_offset, kCodeCommentsOffsetOffset);
macro_rules! INT32_ACCESSORS {
    ($struct_name:ident, $field_name:ident, $offset:ident) => {
        impl $struct_name {
            pub fn $field_name(&self) -> i32 {
                unsafe {
                    let ptr = self as *const Self as *const i32;
                    ptr.add($offset as usize / std::mem::size_of::<i32>()).read()
                }
            }
            pub fn set_$field_name(&mut self, value: i32) {
                unsafe {
                    let ptr = self as *mut Self as *mut i32;
                    ptr.add($offset as usize / std::mem::size_of::<i32>())
                        .write(value);
                }
            }
        }
    };
}
INT32_ACCESSORS!(Code, unwinding_info_offset, kUnwindingInfoOffsetOffset);
macro_rules! UINT16_ACCESSORS {
    ($struct_name:ident, $field_name:ident, $offset:ident) => {
        impl $struct_name {
            pub fn $field_name(&self) -> u16 {
                unsafe {
                    let ptr = self as *const Self as *const u16;
                    ptr.add($offset as usize / std::mem::size_of::<u16>()).read()
                }
            }
            pub fn set_$field_name(&mut self, value: u16) {
                unsafe {
                    let ptr = self as *mut Self as *mut u16;
                    ptr.add($offset as usize / std::mem::size_of::<u16>())
                        .write(value);
                }
            }
        }
    };
}
UINT16_ACCESSORS!(Code, parameter_count, kParameterCountOffset);
impl Code {
    #[inline]
    pub fn parameter_count_without_receiver(&self) -> u16 {
        self.parameter_count() - 1
    }
    #[inline]
    pub fn deoptimization_data(&self) -> Tagged<ProtectedFixedArray> {
        assert!(self.uses_deoptimization_data());
        unsafe {
            std::mem::transmute_copy(&self.ReadProtectedPointerField(
                kDeoptimizationDataOrInterpreterDataOffset,
            ))
        }
    }
    #[inline]
    pub fn set_deoptimization_data(&mut self, value: Tagged<ProtectedFixedArray>, mode: WriteBarrierMode) {
        assert!(self.uses_deoptimization_data());
        assert!(!HeapLayout::InYoungGeneration(value));
        self.WriteProtectedPointerField(kDeoptimizationDataOrInterpreterDataOffset, value);
        self.CONDITIONAL_PROTECTED_POINTER_WRITE_BARRIER(
            kDeoptimizationDataOrInterpreterDataOffset,
            value,
            mode,
        );
    }
    #[inline]
    pub fn uses_deoptimization_data(&self) -> bool {
        CodeKindUsesDeoptimizationData(self.kind())
    }
    #[inline]
    pub fn clear_deoptimization_data_and_interpreter_data(&mut self) {
        self.ClearProtectedPointerField(kDeoptimizationDataOrInterpreterDataOffset);
    }
    #[inline]
    pub fn has_deoptimization_data_or_interpreter_data(&self) -> bool {
        !self.IsProtectedPointerFieldEmpty(kDeoptimizationDataOrInterpreterDataOffset)
    }
    pub fn bytecode_or_interpreter_data(&self) -> Tagged<TrustedObject> {
        assert_eq!(self.kind(), CodeKind::BASELINE);
        self.ReadProtectedPointerField(kDeoptimizationDataOrInterpreterDataOffset)
    }
    pub fn set_bytecode_or_interpreter_data(&mut self, value: Tagged<TrustedObject>, mode: WriteBarrierMode) {
        assert_eq!(self.kind(), CodeKind::BASELINE);
        assert!(Self::IsBytecodeArray(value) || Self::IsInterpreterData(value));
        self.WriteProtectedPointerField(kDeoptimizationDataOrInterpreterDataOffset, value);
        self.CONDITIONAL_PROTECTED_POINTER_WRITE_BARRIER(
            kDeoptimizationDataOrInterpreterDataOffset,
            value,
            mode,
        );
    }
    #[inline]
    pub fn source_position_table(&self) -> Tagged<TrustedByteArray> {
        assert!(self.has_source_position_table());
        unsafe {
            std::mem::transmute_copy(&self
                .ReadProtectedPointerField(kPositionTableOffset))
        }
    }
    #[inline]
    pub fn set_source_position_table(&mut self, value: Tagged<TrustedByteArray>, mode: WriteBarrierMode) {
        assert!(!CodeKindUsesBytecodeOffsetTable(self.kind()));
        self.WriteProtectedPointerField(kPositionTableOffset, value);
        self.CONDITIONAL_PROTECTED_POINTER_WRITE_BARRIER(kPositionTableOffset, value, mode);
    }
    #[inline]
    pub fn bytecode_offset_table(&self) -> Tagged<TrustedByteArray> {
        assert!(self.has_bytecode_offset_table());
        unsafe {
            std::mem::transmute_copy(&self
                .ReadProtectedPointerField(kPositionTableOffset))
        }
    }
    #[inline]
    pub fn set_bytecode_offset_table(&mut self, value: Tagged<TrustedByteArray>, mode: WriteBarrierMode) {
        assert!(CodeKindUsesBytecodeOffsetTable(self.kind()));
        self.WriteProtectedPointerField(kPositionTableOffset, value);
        self.CONDITIONAL_PROTECTED_POINTER_WRITE_BARRIER(kPositionTableOffset, value, mode);
    }
    pub fn has_source_position_table_or_bytecode_offset_table(&self) -> bool {
        TaggedField::<Object, kPositionTableOffset>::load(*self) != Self::SmiZero()
    }
    pub fn has_source_position_table(&self) -> bool {
        let has_table = self.has_source_position_table_or_bytecode_offset_table()
            && !CodeKindUsesBytecodeOffsetTable(self.kind());
        assert!(!CodeKindMayLackSourcePositionTable(self.kind()) || has_table);
        has_table
    }
    pub fn has_bytecode_offset_table(&self) -> bool {
        self.has_source_position_table_or_bytecode_offset_table()
            && CodeKindUsesBytecodeOffsetTable(self.kind())
    }
    pub fn clear_source_position_table_and_bytecode_offset_table(&mut self) {
        TaggedField::<Object, kPositionTableOffset>::store(*self, Self::SmiZero());
    }
    pub fn wrapper(&self) -> Tagged<CodeWrapper> {
        todo!()
    }
    pub fn set_wrapper(&mut self, _value: Tagged<CodeWrapper>, _mode: WriteBarrierMode) {
        todo!()
    }
    pub fn SourcePositionTable(
        isolate: *mut Isolate,
        sfi: Tagged<SharedFunctionInfo>,
    ) -> Tagged<TrustedByteArray> {
        todo!()
    }
    pub fn body_start(&self) -> Address {
        self.instruction_start()
    }
    pub fn body_end(&self) -> Address {
        self.body_start() + self.body_size() as usize
    }
    pub fn body_size(&self) -> i32 {
        self.instruction_size() + self.metadata_size()
    }
    pub fn instruction_end(&self) -> Address {
        self.instruction_start() + self.instruction_size() as usize
    }
    pub fn metadata_start(&self) -> Address {
        if self.has_instruction_stream() {
            assert!(InstructionStream::kOnHeapBodyIsContiguous);
            return self.instruction_start() + self.instruction_size() as usize;
        }
        assert!(!InstructionStream::kOffHeapBodyIsContiguous);
        EmbeddedData::FromBlob().MetadataStartOf(self.builtin_id())
    }
    pub fn InstructionStart(isolate: *mut Isolate, pc: Address) -> Address {
        if self.has_instruction_stream() {
            return self.instruction_start();
        }
        EmbeddedData::FromBlobForPc(isolate, pc).InstructionStartOf(self.builtin_id())
    }
    pub fn InstructionEnd(isolate: *mut Isolate, pc: Address) -> Address {
        Self::InstructionStart(isolate, pc) + self.instruction_size() as usize
    }
    pub fn GetOffsetFromInstructionStart(isolate: *mut Isolate, pc: Address) -> i32 {
        let offset = pc - Self::InstructionStart(isolate, pc);
        assert!(offset <= self.instruction_size() as usize);
        offset as i32
    }
    pub fn metadata_end(&self) -> Address {
        self.metadata_start() + self.metadata_size() as usize
    }
    pub fn safepoint_table_address(&self) -> Address {
        self.metadata_start() + self.safepoint_table_offset() as usize
    }
    pub fn safepoint_table_size(&self) -> i32 {
        self.handler_table_offset() - self.safepoint_table_offset()
    }
    pub fn has_safepoint_table(&self) -> bool {
        self.safepoint_table_size() > 0
    }
    pub fn handler_table_address(&self) -> Address {
        self.metadata_start() + self.handler_table_offset() as usize
    }
    pub fn handler_table_size(&self) -> i32 {
        self.constant_pool_offset() - self.handler_table_offset()
    }
    pub fn has_handler_table(&self) -> bool {
        self.handler_table_size() > 0
    }
    pub fn constant_pool_size(&self) -> i32 {
        let size = self.code_comments_offset() - self.constant_pool_offset();
        if !V8_EMBEDDED_CONSTANT_POOL_BOOL {
            assert_eq!(size, 0);
            return 0;
        }
        assert!(size >= 0);
        size
    }
    pub fn has_constant_pool(&self) -> bool {
        self.constant_pool_size() > 0
    }
    pub fn unchecked_deoptimization_data(&self) -> Tagged<ProtectedFixedArray> {
        unsafe { std::mem::transmute_copy(&self.ReadProtectedPointerField(kDeoptimizationDataOrInterpreterDataOffset)) }
    }
    pub fn relocation_start(&self) -> *mut u8 {
        if self.has_instruction_stream() {
            return self.instruction_stream().relocation_start();
        }
        std::ptr::null_mut()
    }
    pub fn relocation_end(&self) -> *mut u8 {
        if self.has_instruction_stream() {
            return self.instruction_stream().relocation_end();
        }
        std::ptr::null_mut()
    }
    pub fn relocation_size(&self) -> i32 {
        if self.has_instruction_stream() {
            return self.instruction_stream().relocation_size();
        }
        0
    }
    pub fn contains(isolate: *mut Isolate, inner_pointer: Address) -> bool {
        let start = Self::InstructionStart(isolate, inner_pointer);
        if inner_pointer < start {
            return false;
        }
        inner_pointer < start + self.instruction_size() as usize
    }
    pub fn InstructionStreamObjectSize(&self) -> i32 {
        InstructionStream::SizeFor(self.body_size())
    }
    pub fn SizeIncludingMetadata(&self) -> i32 {
        let mut size = self.InstructionStreamObjectSize();
        size += self.relocation_size();
        if self.uses_deoptimization_data() {
            size += self.deoptimization_data().Size();
        }
        size
    }
    pub fn kind(&self) -> CodeKind {
        Self::KindField::decode(self.flags(RelaxedLoadTag {}))
    }
    pub fn GetBytecodeOffsetForBaselinePC(
        baseline_pc: Address,
        bytecodes: Tagged<BytecodeArray>,
    ) -> i32 {
        todo!()
    }
    pub fn GetBaselinePCForBytecodeOffset(
        bytecode_offset: i32,
        position: BytecodeToPCPosition,
        bytecodes: Tagged<BytecodeArray>,
    ) -> usize {
        todo!()
    }
    pub fn GetBaselineStartPCForBytecodeOffset(
        bytecode_offset: i32,
        bytecodes: Tagged<BytecodeArray>,
    ) -> usize {
        todo!()
    }
    pub fn GetBaselineEndPCForBytecodeOffset(
        bytecode_offset: i32,
        bytecodes: Tagged<BytecodeArray>,
    ) -> usize {
        todo!()
    }
    pub fn GetBaselinePCForNextExecutedBytecode(
        bytecode_offset: i32,
        bytecodes: Tagged<BytecodeArray>,
    ) -> usize {
        todo!()
    }
    #[inline]
    pub fn checks_tiering_state(&self) -> bool {
        (self.builtin_id() == Builtin::kCompileLazy
            || self.builtin_id() == Builtin::kInterpreterEntryTrampoline
            || CodeKindCanTierUp(self.kind()))
            || (CodeKindCanDeoptimize(self.kind()) && self.marked_for_deoptimization())
    }
    #[inline]
    pub fn has_tagged_outgoing_params(&self) -> bool {
        CodeKindHasTaggedOutgoingParams(self.kind())
            && self.builtin_id() != Builtin::kWasmCompileLazy
    }
    #[inline]
    pub fn is_context_specialized(&self) -> bool {
        Self::IsContextSpecializedField::decode(self.flags(RelaxedLoadTag {}))
    }
    #[inline]
    pub fn is_turbofanned(&self) -> bool {
        Self::IsTurbofannedField::decode(self.flags(RelaxedLoadTag {}))
    }
    #[inline]
    pub fn is_maglevved(&self) -> bool {
        self.kind() == CodeKind::MAGLEV
    }
    pub fn inlined_bytecode_size(&self) -> u32 {
        self.RELAXED_READ_UINT_FIELD(kInlinedBytecodeSizeOffset)
    }
    pub fn set_inlined_bytecode_size(&mut self, size: u32) {
        self.RELAXED_WRITE_UINT_FIELD(kInlinedBytecodeSizeOffset, size);
    }
    pub fn set_wasm_js_tagged_parameter_count(&mut self, count: u16) {
        assert_eq!(self.kind(), CodeKind::WASM_TO_JS_FUNCTION);
        self.RELAXED_WRITE_UINT16_FIELD(kInlinedBytecodeSizeOffset, count);
    }
    pub fn wasm_js_tagged_parameter_count(&self) -> u16 {
        assert_eq!(self.kind(), CodeKind::WASM_TO_JS_FUNCTION);
        self.RELAXED_READ_UINT16_FIELD(kInlinedBytecodeSizeOffset)
    }
    pub fn set_wasm_js_first_tagged_parameter(&mut self, count: u16) {
        assert_eq!(self.kind(), CodeKind::WASM_TO_JS_FUNCTION);
        self.RELAXED_WRITE_UINT16_FIELD(kInlinedBytecodeSizeOffset + 2, count);
    }
    pub fn wasm_js_first_tagged_parameter(&self) -> u16 {
        assert_eq!(self.kind(), CodeKind::WASM_TO_JS_FUNCTION);
        self.RELAXED_READ_UINT16_FIELD(kInlinedBytecodeSizeOffset + 2)
    }
    pub fn osr_offset(&self) -> BytecodeOffset {
        BytecodeOffset(self.RELAXED_READ_INT32_FIELD(kOsrOffsetOffset))
    }
    pub fn set_osr_offset(&mut self, offset: BytecodeOffset) {
        self.RELAXED_WRITE_INT32_FIELD(kOsrOffsetOffset, offset.ToInt());
    }
    pub fn uses_safepoint_table(&self) -> bool {
        self.is_turbofanned() || self.is_maglevved() || self.is_wasm_code()
    }
    pub fn stack_slots(&self) -> u32 {
        assert!(!(self.safepoint_table_size() > 0) || self.uses_safepoint_table());
        if self.safepoint_table_size() == 0 {
            return 0;
        }
        assert!(self.safepoint_table_size() >= size_of::<SafepointTableStackSlotsField_t>() as i32);
        assert_eq!(kSafepointTableStackSlotsOffset, 0);
        unsafe {
            (self.safepoint_table_address()
                as *const SafepointTableStackSlotsField_t)
                .read() as u32
        }
    }
    pub fn marked_for_deoptimization(&self) -> bool {
        Self::MarkedForDeoptimizationField::decode(self.flags(RelaxedLoadTag {}))
    }
    pub fn set_marked_for_deoptimization(&mut self, flag: bool) {
        assert!(!flag || AllowDeoptimization::IsAllowed(Self::GetIsolateFromWritableObject(*self)));
        let previous = self.flags(RelaxedLoadTag {});
        let updated = Self::MarkedForDeoptimizationField::update(previous, flag);
        self.set_flags(updated, RelaxedLoadTag {});
    }
    pub fn SetMarkedForDeoptimization(isolate: *mut Isolate, reason: LazyDeoptimizeReason) {
        todo!()
    }
    pub fn embedded_objects_cleared(&self) -> bool {
        Self::EmbeddedObjectsClearedField::decode(self.flags(RelaxedLoadTag {}))
    }
    pub fn set_embedded_objects_cleared(&mut self, flag: bool) {
        assert!(!flag || self.marked_for_deoptimization());
        let previous = self.flags(RelaxedLoadTag {});
        let updated = Self::EmbeddedObjectsClearedField::update(previous, flag);
        self.set_flags(updated, RelaxedLoadTag {});
    }
    #[inline]
    pub fn can_have_weak_objects(&self) -> bool {
        Self::CanHaveWeakObjectsField::decode(self.flags(RelaxedLoadTag {}))
    }
    #[inline]
    pub fn set_can_have_weak_objects(&mut self, value: bool) {
        let previous = self.flags(RelaxedLoadTag {});
        let updated = Self::CanHaveWeakObjectsField::update(previous, value);
        self.set_flags(updated, RelaxedLoadTag {});
    }
    pub fn is_wasm_code(&self) -> bool {
        self.kind() == CodeKind::WASM_FUNCTION
    }
    pub fn constant_pool_offset(&self) -> i32 {
        if !V8_EMBEDDED_CONSTANT_POOL_BOOL {
            return self.code_comments_offset();
        }
        self.ReadField::<i32>(kConstantPoolOffsetOffset)
    }
    pub fn set_constant_pool_offset(&mut self, value: i32) {
        if !V8_EMBEDDED_CONSTANT_POOL_BOOL {
            return;
        }
        assert!(value <= self.metadata_size());
        self.WriteField::<i32>(kConstantPoolOffsetOffset, value);
    }
    pub fn constant_pool(&self) -> Address {
        if !self.has_constant_pool() {
            return kNullAddress;
        }
        self.metadata_start() + self.constant_pool_offset() as usize
    }
    pub fn code_comments(&self) -> Address {
        self.metadata_start() + self.code_comments_offset() as usize
    }
    pub fn code_comments_size(&self) -> i32 {
        self.builtin_jump_table_info_offset() - self.code_comments_offset()
    }
    pub fn has_code_comments(&self) -> bool {
        self.code_comments_size() > 0
    }
    pub fn builtin_jump_table_info_offset(&self) -> i32 {
        if !V8_BUILTIN_JUMP_TABLE_INFO_BOOL {
            return self.unwinding_info_offset();
        }
        self.ReadField::<i32>(kBuiltinJumpTableInfoOffsetOffset)
    }
    pub fn set_builtin_jump_table_info_offset(&mut self, value: i32) {
        if !V8_BUILTIN_JUMP_TABLE_INFO_BOOL {
            return;
        }
        assert!(value <= self.metadata_size());
        self.WriteField::<i32>(kBuiltinJumpTableInfoOffsetOffset, value);
    }
    pub fn builtin_jump_table_info(&self) -> Address {
        self.metadata_start() + self.builtin_jump_table_info_offset() as usize
    }
    pub fn builtin_jump_table_info_size(&self) -> i32 {
        self.unwinding_info_offset() - self.builtin_jump_table_info_offset()
    }
    pub fn has_builtin_jump_table_info(&self) -> bool {
        self.builtin_jump_table_info_size() > 0
    }
    pub fn unwinding_info_start(&self) -> Address {
        self.metadata_start() + self.unwinding_info_offset() as usize
    }
    pub fn unwinding_info_end(&self) -> Address {
        self.metadata_end()
    }
    pub fn unwinding_info_size(&self) -> i32 {
        (self.unwinding_info_end() as usize - self.unwinding_info_start() as usize) as i32
    }
    pub fn has_unwinding_info(&self) -> bool {
        self.unwinding_info_size() > 0
    }
    pub fn FromTargetAddress(address: Address) -> Tagged<Code> {
        InstructionStream::FromTargetAddress(address).code(RelaxedLoadTag {})
    }
    pub fn CanContainWeakObjects(&self) -> bool {
        self.is_optimized_code() && self.can_have_weak_objects()
    }
    pub fn IsWeakObject(object: Tagged<HeapObject>) -> bool {
        self.CanContainWeakObjects() && Self::IsWeakObjectInOptimizedCode(object)
    }
    pub fn IsWeakObjectInOptimizedCode(object: Tagged<HeapObject>) -> bool {
        let map_object = object.map(RelaxedLoadTag {});
        if InstanceTypeChecker::IsMap(map_object) {
            return unsafe { std::mem::transmute_copy(&object) }.CanTransition();
        }
        InstanceTypeChecker::IsPropertyCell(map_object)
            || InstanceTypeChecker::IsJSReceiver(map_object)
            || InstanceTypeChecker::IsContext(map_object)
    }
    pub fn IsWeakObjectInDeoptimizationLiteralArray(object: Tagged<Object>) -> bool {
        if Self::IsHeapObject(object) && !Self::IsMap(object) {
            Self::IsWeakObjectInOptimizedCode(unsafe { std::mem::transmute_copy(&object) })
        } else {
            false
        }
    }
    pub fn IterateDeoptimizationLiterals(_v: *mut RootVisitor) {
        todo!()
    }
    pub fn raw_instruction_stream(&self) -> Tagged<Object> {
        let cage_base = self.code_cage_base();
        self.raw_instruction_stream(cage_base)
    }
    pub fn raw_instruction_stream(cage_base: PtrComprCageBase) -> Tagged<Object> {
        ExternalCodeField::<Object>::load(cage_base, *self)
    }
    pub fn set_raw_instruction_stream(&mut self, value: Tagged<Object>, mode: WriteBarrierMode) {
        ExternalCodeField::<Object>::Release_Store(*self, value);
        self.CONDITIONAL_WRITE_BARRIER(kInstructionStreamOffset, value, mode);
    }
    pub fn has_instruction_stream(&self) -> bool {
        let value: usize = self.ReadField::<usize>(kInstructionStreamOffset);
        assert!(value == 0 || !HeapLayout::InReadOnly
