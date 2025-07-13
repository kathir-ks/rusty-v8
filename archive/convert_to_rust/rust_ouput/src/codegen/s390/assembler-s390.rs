// Converted from V8 C++ source files:
// Header: assembler-s390.h
// Implementation: assembler-s390.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

pub mod constants_s390;
pub mod register_s390;

use std::mem;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex, RwLock};
use std::vec::Vec;

use crate::codegen::s390::constants_s390::Condition;
use crate::codegen::s390::register_s390::Register;
use crate::codegen::x64::assembler_x64::code;
use crate::codegen::x64::assembler_x64::V8;

pub struct SafepointTableBuilder;
pub struct CodeDesc;
pub struct Label;
pub struct AssemblerBuffer;
pub struct AssemblerOptions;
pub struct CodeDescBuilder;

pub struct HeapNumberRequest {
//
}
//use crate::codegen::arm64::register_arm64::Register;
//use crate::codegen::arm64::macro_assembler_arm64::Register;

//#[derive(Debug, Copy, Clone, PartialEq, Eq)]
//#[repr(u8)]
//pub enum Condition {
//// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
//
//}
pub struct Code {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Instruction {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct ZoneObject {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Builtins {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Cancelable {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Isolate {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct InstructionBase {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct PhiOp {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Builtin {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct DirectHandle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct WritableJitAllocation {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct MemoryRepresentation {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Block {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Operation {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Operand {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct LocalHeap {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CallInterfaceDescriptor {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CallInterfaceDescriptorData {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct BytecodeArrayWrapper {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Handle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Tagged<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Object {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CPURegister {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct VRegister {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct RegisterArray {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct SCTableReference {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct StdoutStream {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct SSAControlRegister {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct UnoptimizedCompileFlags {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CfgAssembler {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct ArrayBuffer {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Local<'a, T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CFunction {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Args {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct R {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Int64Representation {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct VectorFormat {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct WordPtr {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct StoreRepresentation {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Shift {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Call {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Emission {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Jump {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CRegister {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct IrregexpImplementation {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct AstNodeSourceRangesMethods {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct LocalValue {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Binding<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct VisitResult {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct UnoptimizedCompileFlags {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Position {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct JumpTable {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct WasmJsFunction {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct IndirectHandle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct RpoNumber {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Type {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct LocalValue {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct SourcePosition {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Scope {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Block {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct JsonObject {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct SaveOptions {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct InstructionOperand {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct OpIndex {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct RegExpNodeInfo {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CallStub {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct BuiltinPtr {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct FixedArray {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Operator {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct KnownMap {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct FrameOffset {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct MaybeHandle<T> {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct StackFrameIterator {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct RelocInfoWriter {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct InstructionStream {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct InstructionSequence {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CodePointerHandle {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Map {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Space {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct String {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct BytecodeArray {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CodeStubAssembler {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct Name {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct TypeFeedbackVector {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}
pub struct CodeObject {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/constants-s390.h
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AbortReason {
    kNoReason,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CompactBranchType {
    kNoCompactBranch,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ICacheFlushMode {
    FLUSH_ICACHE,
    FLUSH_ICACHE_IF_NEEDED,
    SKIP_ICACHE_FLUSH,
}

impl RelocInfo {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub fn IsRelativeCodeTarget(rmode: Self::Mode) -> bool {todo!()}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub fn IsCodeTarget(rmode: Self::Mode) -> bool {todo!()}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub fn IsInternalReference(rmode: Self::Mode) -> bool {todo!()}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub fn IsInternalReferenceEncoded(rmode: Self::Mode) -> bool {todo!()}
}
//impl Operand{
//// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
//pub fn setBits(&mut self,arg0:i32) {todo!()}
//}
impl MemOperand{
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub fn getBaseRegister(&self) -> Register{todo!()}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub fn getIndexRegister(&self) -> Register{todo!()}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub fn getDisplacement(&self) -> i32{todo!()}
}
impl Assembler {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub fn ShouldRecordRelocInfo(&self, rmode: RelocInfo::Mode) -> bool {todo!()}
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub enum RelocInfoMode {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
NO_INFO,
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
FULL_EMBEDDED_OBJECT,
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
EXTERNAL_REFERENCE,
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
CODE_TARGET,
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
INTERNAL_REFERENCE
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pub struct RelocInfo{
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
constant_pool_:Address,
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
pc_:Address,
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.h
rmode_:RelocInfoMode
}

pub struct AssemblerBase {}

extern "C" {
    fn memmove(dest: *mut std::os::raw::c_void, src: *const std::os::raw::c_void, n: usize) -> *mut std::os::raw::c_void;
}

impl Operand {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.cc
pub fn setBits(&mut self, n:i32){todo!()}
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.cc
fn supportsSTFLE() -> bool{todo!()}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.cc
impl Assembler{
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.cc
pub fn RecordDeoptReason(&mut self,reason: DeoptimizeReason,node_id:u32,position: SourcePosition,id:i32){todo!()}
}
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.cc
pub enum DeoptimizeReason {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.cc
kNoReason,
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.cc
}

pub type Address = *mut u8;
pub static kNullAddress: Address = 0 as Address;
const kDefaultStopCode: i32 = 0;
const kNumRegisters: usize = 16;
const B8: u64 = 256;
const B12: u64 = 4096;
const B16: u64 = 65536;
const B20: u64 = 1048576;
const B24: u64 = 16777216;
const B28: u64 = 268435456;
const B32: u64 = 4294967296;
const B36: u64 = 68719476736;
const B40: u64 = 1099511627776;

const FPU: i32 = 1;
const GENERAL_INSTR_EXT: i32 = 2;
const FLOATING_POINT_EXT: i32 = 3;
const DISTINCT_OPS: i32 = 4;
const VECTOR_FACILITY: i32 = 5;
const VECTOR_ENHANCE_FACILITY_1: i32 = 6;
const VECTOR_ENHANCE_FACILITY_2: i32 = 7;
const MISC_INSTR_EXT2: i32 = 8;
const LDI: constants_s390::Opcode = constants_s390::Opcode::LDI;
const A: constants_s390::Opcode = constants_s390::Opcode::A;
const BRC: constants_s390::Opcode = constants_s390::Opcode::BRC;
const BRCT: constants_s390::Opcode = constants_s390::Opcode::BRCT;
const BRCTG: constants_s390::Opcode = constants_s390::Opcode::BRCTG;
const BRCL: constants_s390::Opcode = constants_s390::Opcode::BRCL;
const BRASL: constants_s390::Opcode = constants_s390::Opcode::BRASL;
const LARL: constants_s390::Opcode = constants_s390::Opcode::LARL;
const LGRL: constants_s390::Opcode = constants_s390::Opcode::LGRL;
const LLILF: constants_s390::Opcode = constants_s390::Opcode::LLILF;
const B: constants_s390::Opcode = constants_s390::Opcode::B;
const C: constants_s390::Opcode = constants_s390::Opcode::C;
const lzdr: constants_s390::Opcode = constants_s390::Opcode::LZDR;
const lzer: constants_s390::Opcode = constants_s390::Opcode::LZER;
const LEDBR: constants_s390::Opcode = constants_s390::Opcode::LEDBR;
const CDFBR: constants_s390::Opcode = constants_s390::Opcode::CDFBR;
const CDGBR: constants_s390::Opcode = constants_s390::Opcode::CDGBR;
const CEGBR: constants_s390::Opcode = constants_s390::Opcode::CEGBR;
const PFD: constants_s390::Opcode = constants_s390::Opcode::PFD;
const SLL: constants_s390::Opcode = constants_s390::Opcode::SLL;
const SRL: constants_s390::Opcode = constants_s390::Opcode::SRL;
const SLA: constants_s390::Opcode = constants_s390::Opcode::SLA;
const SRA: constants_s390::Opcode = constants_s390::Opcode::SRA;
const SLDL: constants_s390::Opcode = constants_s390::Opcode::SLDL;
const SRDA: constants_s390::Opcode = constants_s390::Opcode::SRDA;
const SRDL: constants_s390::Opcode = constants_s390::Opcode::SRDL;
const OI: constants_s390::Opcode = constants_s390::Opcode::OI;
const STM: constants_s390::Opcode = constants_s390::Opcode::STM;
const LM: constants_s390::Opcode = constants_s390::Opcode::LM;
const STE: constants_s390::Opcode = constants_s390::Opcode::STE;
const CEBRA: constants_s390::Opcode = constants_s390::Opcode::CEBRA;
const CGDBRA: constants_s390::Opcode = constants_s390::Opcode::CGDBRA;
const CFDBRA: constants_s390::Opcode = constants_s390::Opcode::CFDBRA;
const CFEBRA: constants_s390::Opcode = constants_s390::Opcode::CFEBRA;
const IIHF: constants_s390::Opcode = constants_s390::Opcode::IIHF;
const IILF: constants_s390::Opcode = constants_s390::Opcode::IILF;
const BRXH: constants_s390::Opcode = constants_s390::Opcode::BRXH;
const BRXHG: constants_s390::Opcode = constants_s390::Opcode::BRXHG;
const LHI: constants_s390::Opcode = constants_s390::Opcode::LHI;
const VFA: constants_s390::Opcode = constants_s390::Opcode::VFA;
const VFS: constants_s390::Opcode = constants_s390::Opcode::VFS;
const VFM: constants_s390::Opcode = constants_s390::Opcode::VFM;
const VFD: constants_s390::Opcode = constants_s390::Opcode::VFD;
const BASR: constants_s390::Opcode = constants_s390::Opcode::BASR;
const BCR: constants_s390::Opcode = constants_s390::Opcode::BCR;
const LIC: constants_s390::Opcode = constants_s390::Opcode::LIC;
const OILL: constants_s390::Opcode = constants_s390::Opcode::OILL;

const eq: constants_s390::Condition = constants_s390::Condition::eq;
const ne: constants_s390::Condition = constants_s390::Condition::ne;
const lt: constants_s390::Condition = constants_s390::Condition::lt;
const le: constants_s390::Condition = constants_s390::Condition::le;
const gt: constants_s390::Condition = constants_s390::Condition::gt;
const ge: constants_s390::Condition = constants_s390::Condition::ge;
const al: constants_s390::Condition = constants_s390::Condition::al;
const ordered: constants_s390::Condition = constants_s390::Condition::ordered;
const unordered: constants_s390::Condition = constants_s390::Condition::unordered;

const r0: Register = Register::r0;
const r1: Register = Register::r1;
const r2: Register = Register::r2;
const r3: Register = Register::r3;
const r4: Register = Register::r4;
const r5: Register = Register::r5;
const r6: Register = Register::r6;
const r7: Register = Register::r7;
const r8: Register = Register::r8;
const r9: Register = Register::r9;
const r10: Register = Register::r10;
const fp: Register = Register::fp;
const ip: Register = Register::ip;
const r13: Register = Register::r13;
const r14: Register = Register::r14;
const sp: Register = Register::sp;
const kScratchDoubleReg: DoubleRegister = DoubleRegister{code_: 0};
const kDoubleRegZero: DoubleRegister = DoubleRegister{code_: 0};

impl CpuFeatures {
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/s390/assembler-s390.cc
pub fn SupportsWasmSimd128() -> bool{todo!()}
// From /home/kathirks_gc/v8_go/archive/
