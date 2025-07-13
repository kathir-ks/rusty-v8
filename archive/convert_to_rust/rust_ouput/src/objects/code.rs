// Converted from V8 C++ source files:
// Header: code.h
// Implementation: code.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod code_generated {
}
pub mod maglev_safepoint_table {
}
pub mod code_kind {
}
pub mod r#struct {
}
pub mod trusted_object {
}
pub mod object_macros {
}
pub mod bytecode_array {
}
pub mod code_desc {
}
pub mod code_wrapper {
}
pub mod factory {
}
pub mod safepoint_entry {
}
pub mod root_visitor {
}
pub mod builtin {
}
pub mod lazy_deoptimize_reason {
}
use std::fmt;
use std::mem::size_of;
//use crate::archive::codebase::src::codegen::safepoint_table::SafepointTable;
//use crate::archive::codebase::src::codegen::reloc_info::RelocInfo;
//use crate::archive::codebase::src::deoptimizer::deoptimizer::Deoptimizer;
//use crate::archive::codebase::src::objects::code_inl::OBJECT_CONSTRUCTORS;
//use crate::archive::codebase::src::objects::code_inl::FLAG_BIT_FIELDS;
use crate::archive::codebase::src::objects::tagged_impl_inl::*;
use crate::archive::codebase::src::objects::instruction_stream::*;
use crate::archive::codebase::src::objects::map::*;

pub enum Builtin { }
pub enum LazyDeoptimizeReason { }

pub struct Code {
    instruction_stream: Tagged<InstructionStream>,
    instruction_start: usize,
    instruction_size: i32,
    metadata_size: i32,
    handler_table_offset: i32,
    builtin_jump_table_info_offset: i32,
    unwinding_info_offset: i32,
    deoptimization_data: Tagged<ProtectedFixedArray>,
    parameter_count: u16,
    wasm_js_tagged_parameter_count: u16,
    wasm_js_first_tagged_parameter: u16,
    bytecode_or_interpreter_data: Tagged<TrustedObject>,
    source_position_table: Tagged<TrustedByteArray>,
    bytecode_offset_table: Tagged<TrustedByteArray>,
    inlined_bytecode_size: u32,
    osr_offset: i32, //BytecodeOffset
    code_comments_offset: i32,
    constant_pool_offset: i32,
    wrapper: Tagged<CodeWrapper>,
    flags: u32,
    can_have_weak_objects: bool,
    marked_for_deoptimization: bool,
    embedded_objects_cleared: bool,
}

impl Code {
    pub fn code_cage_base(&self) -> PtrComprCageBase {
        PtrComprCageBase {}
    }

    pub fn instruction_stream(&self) -> Tagged<InstructionStream> {
        self.instruction_stream
    }

    pub fn instruction_start(&self) -> Address {
        Address {} //self.instruction_start
    }

    pub fn instruction_size(&self) -> i32 {
        self.instruction_size
    }

    pub fn instruction_end(&self) -> Address {
        Address {}
    }

    pub fn entrypoint_tag(&self) -> CodeEntrypointTag {
        CodeEntrypointTag {}
    }

    pub fn FlushICache(&self) const {
        //FlushInstructionCache(instruction_start(), instruction_size());
    }

    pub fn can_have_weak_objects(&self) -> bool {
        self.can_have_weak_objects
    }
    pub fn marked_for_deoptimization(&self) -> bool {
        self.marked_for_deoptimization
    }

    pub fn metadata_size(&self) -> i32 {
        self.metadata_size
    }

    pub fn handler_table_offset(&self) -> i32 {
        self.handler_table_offset
    }
    pub fn builtin_jump_table_info_offset(&self) -> i32 {
        self.builtin_jump_table_info_offset
    }
    pub fn unwinding_info_offset(&self) -> i32 {
        self.unwinding_info_offset
    }
    pub fn deoptimization_data(&self) -> Tagged<ProtectedFixedArray> {
        self.deoptimization_data
    }
    pub fn parameter_count(&self) -> u16 {
        self.parameter_count
    }

    pub fn wasm_js_tagged_parameter_count(&self) -> u16 {
        self.wasm_js_tagged_parameter_count
    }

    pub fn wasm_js_first_tagged_parameter(&self) -> u16 {
        self.wasm_js_first_tagged_parameter
    }

    pub fn bytecode_or_interpreter_data(&self) -> Tagged<TrustedObject> {
        self.bytecode_or_interpreter_data
    }

    pub fn source_position_table(&self) -> Tagged<TrustedByteArray> {
        self.source_position_table
    }
    pub fn bytecode_offset_table(&self) -> Tagged<TrustedByteArray> {
        self.bytecode_offset_table
    }

    pub fn inlined_bytecode_size(&self) -> u32 {
        self.inlined_bytecode_size
    }
    pub fn osr_offset(&self) -> i32 {
        self.osr_offset
    }
    pub fn code_comments_offset(&self) -> i32 {
        self.code_comments_offset
    }
    pub fn constant_pool_offset(&self) -> i32 {
        self.constant_pool_offset
    }
    pub fn wrapper(&self) -> Tagged<CodeWrapper> {
        self.wrapper
    }
    pub fn flags(&self) -> u32 {
        self.flags
    }
    pub fn kind(&self) -> CodeKind {
        CodeKind::UNINITIALIZED
    }

    pub fn has_safepoint_table(&self) -> bool {
        true
    }
    pub fn has_handler_table(&self) -> bool {
        true
    }
    pub fn has_constant_pool(&self) -> bool {
        true
    }
    pub fn has_code_comments(&self) -> bool {
        true
    }
    pub fn has_builtin_jump_table_info(&self) -> bool {
        true
    }
    pub fn has_unwinding_info(&self) -> bool {
        true
    }

    pub fn safepoint_table_address(&self) -> Address {
        Address {}
    }
    pub fn safepoint_table_size(&self) -> i32 {
        0
    }

    pub fn handler_table_address(&self) -> Address {
        Address {}
    }
    pub fn handler_table_size(&self) -> i32 {
        0
    }

    pub fn constant_pool(&self) -> Address {
        Address {}
    }
    pub fn constant_pool_size(&self) -> i32 {
        0
    }

    pub fn code_comments(&self) -> Address {
        Address {}
    }
    pub fn code_comments_size(&self) -> i32 {
        0
    }

    pub fn builtin_jump_table_info(&self) -> Address {
        Address {}
    }
    pub fn builtin_jump_table_info_size(&self) -> i32 {
        0
    }

    pub fn unwinding_info_start(&self) -> Address {
        Address {}
    }
    pub fn unwinding_info_end(&self) -> Address {
        Address {}
    }
    pub fn unwinding_info_size(&self) -> i32 {
        0
    }

    pub fn metadata_start(&self) -> Address {
        Address {}
    }

    pub fn body_size(&self) -> i32{
        0
    }
    pub fn relocation_size(&self) -> i32{
        0
    }
    pub fn instruction_size_value(&self) -> i32{
        0
    }
    pub fn UsesDeoptimizationData(&self) -> bool{
        false
    }
    pub fn marked_for_deoptimization_value(&self) -> bool{
        false
    }

    fn set_instruction_start(&mut self, _isolate:IsolateForSandbox, _value: Address) {}
}

impl Code {
    pub fn raw_instruction_stream(&self) -> Tagged<Object> {
        Tagged {}
    }

    pub fn unchecked_instruction_stream(&self) -> Tagged<InstructionStream> {
        Tagged {}
    }

    pub fn has_instruction_stream(&self) -> bool {
        true
    }

    pub fn safepoint_table_offset(&self) -> i32 {
        0
    }
    pub fn body_start(&self) -> Address {
        Address {}
    }
    pub fn metadata_end(&self) -> Address {
        Address {}
    }
    pub fn relocation_start(&self) -> *mut u8 {
        std::ptr::null_mut()
    }
    pub fn relocation_end(&self) -> *mut u8 {
        std::ptr::null_mut()
    }
}

pub struct GcSafeCode {
    code: Tagged<Code>,
}

impl GcSafeCode {
    pub fn instruction_start(&self) -> Address {
        Address {}
    }
    pub fn instruction_end(&self) -> Address {
        Address {}
    }
    pub fn is_builtin(&self) -> bool {
        false
    }
}
pub struct CodeWrapper {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodeKind {
    UNINITIALIZED
}

impl fmt::Display for CodeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct WritableJitAllocation{}
pub struct Isolate{}
pub struct SharedFunctionInfo{}
pub struct TaggedScope<T>{}
pub struct DeoptimizationData{}
pub struct TrustedByteArray{}
pub struct Address {}

pub struct ExposedTrustedObject {
}
impl ExposedTrustedObject{
}
pub struct SafepointTable{}
impl SafepointTable{
    
}
pub struct MaglevSafepointTable{}
impl MaglevSafepointTable{
    
}
pub struct HandlerTable{}
impl HandlerTable{
    
}
pub struct JSDispatchHandle{}

pub struct WritableRelocIterator{}
impl WritableRelocIterator{
}
pub struct RelocInfo{}
impl RelocInfo{
    
}

pub struct Factory{}
pub struct Heap{}
pub struct ProtectedFixedArray{}
pub struct SourcePositionTableIterator{}
impl SourcePositionTableIterator{
    
}
pub struct SourcePosition{}
impl SourcePosition{
    
}

pub struct TaggedObject{}
pub struct WriteBarrierMode{}

pub struct HeapObject{}

impl Code {
    pub fn initialize_flags(&mut self, _kind: CodeKind, _is_context_specialized: bool, _is_turbofanned: bool) {}
    pub fn clear_padding(&mut self) {}
    pub fn uses_deoptimization_data(&self) -> bool { false }
    pub fn clear_deoptimization_data_and_interpreter_data(&mut self) {}
    pub fn has_deoptimization_data_or_interpreter_data(&self) -> bool { false }
    pub fn has_source_position_table_or_bytecode_offset_table(&self) -> bool { false }
    pub fn has_source_position_table(&self) -> bool { false }
    pub fn has_bytecode_offset_table(&self) -> bool { false }
    pub fn clear_source_position_table_and_bytecode_offset_table(&mut self) {}
    pub fn set_builtin_id(&mut self, _builtin_id: Builtin) {}
    pub fn builtin_id(&self) -> Builtin { Builtin{} }
    pub fn is_builtin(&self) -> bool { false }
    pub fn is_optimized_code(&self) -> bool { false }
    pub fn is_wasm_code(&self) -> bool { false }
    pub fn is_interpreter_trampoline_builtin(&self) -> bool { false }
    pub fn is_baseline_trampoline_builtin(&self) -> bool { false }
    pub fn is_baseline_leave_frame_builtin(&self) -> bool { false }
    pub fn checks_tiering_state(&self) -> bool { false }
    pub fn has_tagged_outgoing_params(&self) -> bool { false }
    pub fn is_maglevved(&self) -> bool { false }
    pub fn is_turbofanned(&self) -> bool { false }
    pub fn is_context_specialized(&self) -> bool { false }
    pub fn uses_safepoint_table(&self) -> bool { false }
    pub fn stack_slots(&self) -> u32 { 0 }
    pub fn SourcePositionTable(&self, _isolate: *mut Isolate, _sfi: Tagged<SharedFunctionInfo>) -> Tagged<TrustedByteArray> { Tagged {} }
    pub fn SourcePosition(&self, _offset: i32) -> i32 { 0 }
    pub fn SourceStatementPosition(&self, _offset: i32) -> i32 { 0 }
    pub fn GetSafepointEntry(&self, _isolate: *mut Isolate, _pc: Address) {}
    pub fn GetMaglevSafepointEntry(&self, _isolate: *mut Isolate, _pc: Address) {}
    pub fn SetMarkedForDeoptimization(&mut self, _isolate: *mut Isolate, _reason: LazyDeoptimizeReason) {}
    pub fn CanContainWeakObjects(&self) -> bool { false }
    pub fn IsWeakObject(&self, _object: Tagged<HeapObject>) -> bool { false }
    pub fn IsWeakObjectInOptimizedCode(_object: Tagged<HeapObject>) -> bool { false }
    pub fn IsWeakObjectInDeoptimizationLiteralArray(_object: Tagged<Object>) -> bool { false }
    pub fn embedded_objects_cleared(&self) -> bool { false }
    pub fn set_embedded_objects_cleared(&mut self, _flag: bool) {}
    pub fn GetBaselineStartPCForBytecodeOffset(&self, _bytecode_offset: i32, _bytecodes: Tagged<BytecodeArray>) -> usize { 0 }
    pub fn GetBaselineEndPCForBytecodeOffset(&self, _bytecode_offset: i32, _bytecodes: Tagged<BytecodeArray>) -> usize { 0 }
    pub fn Inlines(&self, _sfi: Tagged<SharedFunctionInfo>) -> bool { false }
    pub fn GetBaselinePCForNextExecutedBytecode(&self, _bytecode_offset: i32, _bytecodes: Tagged<BytecodeArray>) -> usize { 0 }
    pub fn GetBytecodeOffsetForBaselinePC(&self, _baseline_pc: Address, _bytecodes: Tagged<BytecodeArray>) -> i32 { 0 }
    pub fn IterateDeoptimizationLiterals(&self, _v: *mut RootVisitor) {}
    pub fn FromTargetAddress(_address: Address) -> Tagged<Code> { Tagged {} }

    pub fn set_instruction_stream(&mut self, value: Tagged<InstructionStream>, mode: WriteBarrierMode) {}
    pub fn set_raw_instruction_stream(&mut self, value: Tagged<Object>, mode: WriteBarrierMode) {}
}
pub struct TaggedField<T, const OFFSET: usize> {
    _dummy: i32,
}

pub struct TaggedField<T, const OFFSET: usize, CompressionScheme> {
    _dummy: i32,
}

pub struct IsolateForSandbox {}

impl Code {
    pub fn SetInstructionStreamAndInstructionStart(&mut self, _isolate:IsolateForSandbox, _code: Tagged<InstructionStream>, _mode: WriteBarrierMode) {}
    pub fn SetInstructionStartForOffHeapBuiltin(&mut self, _isolate:IsolateForSandbox, _entry: Address) {}
    pub fn ClearInstructionStartForSerialization(&mut self, _isolate:IsolateForSandbox) {}
    pub fn UpdateInstructionStart(&mut self, _isolate:IsolateForSandbox, _istream: Tagged<InstructionStream>) {}
    pub fn metadata_end(&self) -> Address {
        Address{}
    }
    pub fn UpdateRelocationInfo(){}
}
pub struct ProtectedFixedArray{}
impl ProtectedFixedArray{
    
}
pub struct BytecodeArray{}
impl BytecodeArray{
    
}
impl Code{
    pub fn parameter_count_without_receiver(&self) -> u16{
        0
    }
}
pub struct TrustedObject {}
impl TrustedObject {

}

pub struct RootVisitor {}
pub struct RelocIterator {}

pub enum ThreadIsolation {
}
impl ThreadIsolation {
    pub fn LookupJitAllocation(_address:Address, _size: i32, _type:Self, _arg:bool) -> WritableJitAllocation{
        WritableJitAllocation{}
    }
}
impl ThreadIsolation {
    pub enum JitAllocationType{}
}
pub struct OffHeapInstructionStream{}
impl OffHeapInstructionStream{
    
}

pub struct DeoptimizationLiteralArray{}
impl DeoptimizationLiteralArray{}
pub struct CodeReference{}
impl CodeReference{
    pub fn new(_arg: &mut Tagged<Code>) -> Self{
        CodeReference{}
    }
}
pub struct EhFrameDisassembler{}
impl EhFrameDisassembler{
    pub fn DisassembleToStream(&mut self, arg: impl std::io::Write){}
}
impl CodeKind{
    
}
