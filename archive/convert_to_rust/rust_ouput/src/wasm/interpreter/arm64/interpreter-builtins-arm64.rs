// Converted from V8 C++ source files:
// Header: N/A
// Implementation: interpreter-builtins-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::base::SmallVector;
// Necessary for builtins generation.
use crate::compiler::backend::code_generator_impl::Register;
use crate::compiler::backend::instruction::Instruction;
use crate::compiler::backend::instruction_selector::MachineRepresentation;
use crate::compiler::instruction_scheduler::Scheduler::ReadyList::Node;
use crate::execution::isolate::Isolate;
use crate::execution::isolate::Set;
use crate::objects::JSArray;
use crate::wasm::wasm_features::CompileTimeImport;
use crate::zone::Zone;
use crate::Builtin;

struct Name {}
struct Smi {}
struct HeapNumber {}
struct Integer {}
struct Tagged {}
struct JSFunction {}
struct WasmExportedFunctionData {}
struct SharedFunctionInfo {}
struct WasmTrustedInstanceData {}
struct RelocInfo {}
struct IsolateData {}
struct FixedArray {}
struct WasmValueKindBitsMask {}
struct WasmThread {}
struct StackFrame {}

mod objects_inl {
    pub struct Object {}
}

mod isolate {
    pub type Address = usize;
}

mod internal {
    pub type Address = usize;
}

mod wasm {
    pub mod ObjectAccess {
        pub fn SharedFunctionInfoOffsetInTaggedJSFunction() -> usize {
            0
        }
    }

    pub struct FunctionSig {}

    impl FunctionSig {
        pub const kReturnCountOffset: usize = 0;
        pub const kParameterCountOffset: usize = 8;
        pub const kRepsOffset: usize = 16;
    }
    pub struct ValueType {
        bit_field: u32,
    }
    impl ValueType {
        pub fn bit_field_offset() -> usize {
            0
        }
    }
    pub mod ValueKind {
        pub const kRefNull: u32 = 0;
        pub const kRef: u32 = 1;
    }
}

mod compiler {
    pub mod backend {
        pub mod code_generator_impl {
            pub struct Frame {}
        }
    }
}

const kSystemPointerSize: usize = 8;
const kTaggedSizeLog2: usize = 3;
const kXRegSizeInBits: usize = 64;
const kHeapObjectTag: usize = 1;
const kSystemPointerSizeLog2: usize = 3;
const kWasmTrustedInstanceDataIndirectPointerTag: usize = 1;
const kUnknownIndirectPointerTag: usize = 1;
const kFPOnStackSize: usize = 8;
const kPCOnStackSize: usize = 8;
const kJSFunctionRegister: Register = Register {};
const kWasmImplicitArgRegister: Register = Register {};
const kReturnRegister0: Register = Register {};
const kFPReturnRegister0: Register = Register {};
const kRootRegister: Register = Register {};
const kPtrComprCageBaseRegister: Register = Register {};
const NoReg: Register = Register {};

fn log2(x: usize) -> i32 {
    if x == 0 {
        return 0;
    }
    let mut k = 0;
    let mut n = x;
    while n > 1 {
        n >>= 1;
        k += 1;
    }
    k
}

fn SmiValuesAre32Bits() -> bool {
    true
}

macro_rules! BUILTIN_CODE {
    ($i:expr, $x:ident) => {
        BuiltinCode {}
    };
}

struct BuiltinCode {}
struct RootIndex {}
impl RootIndex {
    const kUndefinedValue: usize = 0;
}
struct WasmToJSInterpreterFrameConstants {}

impl WasmToJSInterpreterFrameConstants {
    const kGCSPOffset: usize = 1;
    const kGCScanSlotLimitOffset: usize = 1;
}

struct StackHandlerConstants {}
impl StackHandlerConstants {
    const kNextOffset: usize = 0;
    const kSlotCount: usize = 1;
}

struct WasmInterpreterCWasmEntryConstants {}

impl WasmInterpreterCWasmEntryConstants {
    const kSPFPOffset: usize = 0;
    const kCEntryFPOffset: usize = 0;
}

struct BuiltinWasmInterpreterWrapperConstants {}

impl BuiltinWasmInterpreterWrapperConstants {
    const kGCScanSlotCountOffset: usize = 0;
    const kParamCountOffset: usize = 0;
    const kReturnCountOffset: usize = 0;
    const kSigRepsOffset: usize = 0;
    const kValueTypesArrayStartOffset: usize = 0;
    const kArgRetsAddressOffset: usize = 0;
    const kArgRetsIsArgsOffset: usize = 0;
    const kCurrentIndexOffset: usize = 0;
    const kSignatureDataOffset: usize = 0;
}

struct CPURegList {
    reg_size_in_bits: usize,
    bits: usize,
}
impl CPURegList {
    fn new(reg_size_in_bits: usize, bits: usize) -> Self {
        CPURegList {
            reg_size_in_bits,
            bits,
        }
    }
    fn set_bits(&mut self, bits: usize) {
        self.bits = bits;
    }
    fn IncludesAliasOf(&self, reg: Register) -> bool {
        true
    }
    fn IsEmpty(&self) -> bool {
        self.bits == 0
    }
    fn PopLowestIndex(&mut self) -> Register {
        self.bits &= self.bits - 1;
        Register {}
    }
    fn Combine(&mut self, reg: Register) {}
    fn Remove(&mut self, reg: Register) {}
    fn Remove1(&mut self, reg1: Register, reg2: Register, reg3: Register, reg4: Register) {}
    fn Remove2(&mut self, reg5: Register, reg6: Register, reg7: Register) {}
}

impl Default for CPURegList {
    fn default() -> Self {
        CPURegList::new(kXRegSizeInBits, 0)
    }
}

struct Assembler {}
impl Assembler {
    fn BlockPoolsScope(_masm: &mut Assembler) -> Assembler {
        Assembler {}
    }
}

#[derive(Debug, Clone)]
struct ExternalReference {}
impl ExternalReference {
    fn Create(_id: IsolateAddressId, _isolate: &mut Isolate) -> Self {
        ExternalReference {}
    }
}

#[derive(Debug, Clone)]
struct IsolateAddressId {}

struct MacroAssembler {}

impl MacroAssembler {
    fn isolate(&mut self) -> &mut Isolate {
        &mut Isolate {}
    }
}

macro_rules! ACCESS_MASM {
    ($masm:expr) => {
        $masm
    };
}

macro_rules! __ {
    ($masm:expr, Mov, $r1:expr, $i:literal) => {};
    ($masm:expr, Str, $r1:expr, $r2:expr) => {};
    ($masm:expr, Push, $($arg:expr),*) => {};
    ($masm:expr, Pop, $($arg:expr),*) => {};
    ($masm:expr, LoadTrustedPointerField, $r1:expr, $r2:expr, $tag:expr) => {};
    ($masm:expr, LoadTaggedField, $r1:expr, $r2:expr) => {};
    ($masm:expr, EnterFrame, $frame:expr) => {};
    ($masm:expr, LeaveFrame, $frame:expr) => {};
    ($masm:expr, Ret) => {};
    ($masm:expr, Sub, $r1:expr, $r2:expr, $i:expr) => {};
    ($masm:expr, CallRuntime, $runtime:expr, $num_args:expr) => {};
    ($masm:expr, Call, $builtin:expr, $reloc_info:expr) => {};
    ($masm:expr, And, $r1:expr, $r2:expr, $imm:expr) => {};
    ($masm:expr, Add, $r1:expr, $r2:expr, $i:expr) => {};
    ($masm:expr, Add, $r1:expr, $r2:expr, $operand:expr) => {};
    ($masm:expr, Cmp, $r1:expr, $i:expr) => {};
    ($masm:expr, B, $label:expr, $cond:expr) => {};
    ($masm:expr, jmp, $label:expr) => {};
    ($masm:expr, Mov, $r1:expr, $r2:expr) => {};
    ($masm:expr, Add, $r1:expr, $r2:expr, $r3:expr) => {};
    ($masm:expr, DropArguments, $count:expr) => {};
    ($masm:expr, Ldr, $r1:expr, $mem:expr) => {};
    ($masm:expr, Cmp, $r1:expr, $r2:expr) => {};
    ($masm:expr, Mov, $r1:expr, $imm:expr) => {};
    ($masm:expr, Tst, $r1:expr, $imm:expr) => {};
    ($masm:expr, Ldr, $r1:expr, $mem:expr) => {};
    ($masm:expr, StoreTaggedField, $r1:expr, $mem:expr) => {};
    ($masm:expr, DebugBreak) => {};
    ($masm:expr, Ldr, $r1:expr, $mem:expr) => {};
    ($masm:expr, cmp, $r1:expr, $i:expr) => {};
    ($masm:expr, cmp, $r1:expr, $r2:expr) => {};
    ($masm:expr, str, $r1:expr, $r2:expr) => {};
    ($masm:expr, SmiTag, $r1:expr) => {};
    ($masm:expr, SmiUntag, $r1:expr) => {};
    ($masm:expr, Ldr, $r1:expr, $mem:expr) => {};
    ($masm:expr, Str, $r1:expr, $mem:expr) => {};
    ($masm:expr, Str, $imm:expr, $mem:expr) => {};
    ($masm:expr, DebugBreak) => {};
    ($masm:expr, LoadRoot, $r1:expr, $root_index:expr) => {};
    ($masm:expr, LoadGlobalProxy, $r1:expr) => {};
    ($masm:expr, cmp, $r1:expr, $r2:expr) => {};
    ($masm:expr, Str, $r1:expr, $r2:expr) => {};
    ($masm:expr, B, $label:expr, $cond:expr) => {};
    ($masm:expr, SmiTag, $r1:expr) => {};
    ($masm:expr, Push, $reg:expr, $value:expr) => {};
    ($masm:expr, LoadRootRelative, $r1:expr, $imm:expr) => {};
    ($masm:expr, Fmov, $r1:expr, $value:expr) => {};
}

macro_rules! DEFINE_REG {
    ($regs:expr, $Name:ident) => {
        let mut $Name: Register = Register {};
        $regs.Ask(&mut $Name);
    };
}

macro_rules! DEFINE_REG_W {
    ($regs:expr, $Name:ident) => {
        DEFINE_REG!($regs, $Name);
    };
}

macro_rules! ASSIGN_REG {
    ($regs:expr, $Name:ident) => {
        let mut $Name: Register = Register {};
        $regs.Ask(&mut $Name);
    };
}

macro_rules! ASSIGN_REG_W {
    ($regs:expr, $Name:ident) => {
        ASSIGN_REG!($regs, $Name);
    };
}

macro_rules! DEFINE_PINNED {
    ($Name:ident, $Reg:expr) => {
        let $Name: Register = $Reg;
    };
}

macro_rules! FREE_REG {
    ($regs:expr, $Name:ident) => {
        $regs.Free(&mut $Name);
    };
}

struct FieldMemOperand {}
impl FieldMemOperand {
    fn new(_base: Register, _offset: usize) -> Self {
        FieldMemOperand {}
    }
}

struct MemOperand {}

impl MemOperand {
    fn new(_base: Register, _offset: usize) -> Self {
        MemOperand {}
    }
    fn new1(_base: Register, _index: Register, _shift: usize, _offset: usize) -> Self {
        MemOperand {}
    }
}

#[derive(PartialEq, Eq)]
enum Condition {
    eq,
    ne,
    lt,
    gt,
    vs
}

struct Builtins {}

impl Builtins {
    fn Generate_WasmInterpreterEntry(masm: &mut MacroAssembler) {}
    fn Generate_GenericJSToWasmInterpreterWrapper(masm: &mut MacroAssembler) {}
    fn Generate_GenericWasmToJSInterpreterWrapper(masm: &mut MacroAssembler) {}
    fn Generate_WasmInterpreterCWasmEntry(masm: &mut MacroAssembler) {}
    fn SetCWasmInterpreterEntryHandlerOffset(&mut self, _pos: usize) {}
}

impl MacroAssembler {
    fn BindExceptionHandler(&mut self, _label: &Label) {}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Label {
    pos: usize,
}

impl Label {
    fn new(pos: usize) -> Self {
        Label { pos }
    }
}

#[allow(non_snake_case)]
impl MacroAssembler {
    fn Bind(&mut self, _label: &Label) {}
}

#[allow(non_camel_case_types)]
struct Runtime {}

impl Runtime {
    const kWasmRunInterpreter: usize = 0;
    const kWasmThrowJSTypeError: usize = 1;
}

fn All() -> Self {
    Self {}
}

fn new() -> Self {
    Self {}
}

struct UseScratchRegisterScope {}
impl UseScratchRegisterScope {
    fn AcquireX(&mut self) -> Register {
        Register {}
    }
}

mod codegen {
    pub struct CodeFactory {}
}

struct RegisterAllocator {
    initial_: CPURegList,
    available_: CPURegList,
    allocated_registers_: Vec<Register>,
}

impl RegisterAllocator {
    fn new(registers: CPURegList) -> Self {
        RegisterAllocator {
            initial_: registers,
            available_: registers,
            allocated_registers_: Vec::new(),
        }
    }
    fn Ask(&mut self, reg: &mut Register) {
    }

    fn Pinned(&mut self, requested: Register, reg: &mut Register) {
        *reg = requested;
        self.Reserve(requested);
        self.allocated_registers_.push(reg.clone());
    }

    fn Free(&mut self, reg: &mut Register) {
        self.available_.Combine(reg.clone());
        *reg = Register {};
    }

    fn Reserve(&mut self, reg: Register) {
        if reg == NoReg {
            return;
        }
        self.available_.Remove(reg);
    }

    fn IsUsed(&self, reg: Register) -> bool {
        self.initial_.IncludesAliasOf(reg) && !self.available_.IncludesAliasOf(reg)
    }

    fn ResetExcept(
        &mut self,
        reg1: Register,
        reg2: Register,
        reg3: Register,
        reg4: Register,
        reg5: Register,
        reg6: Register,
        reg7: Register,
    ) {
    }

    fn WithAllocatableGeneralRegisters() -> Self {
        let mut list = CPURegList::default();
        list.set_bits(0xffff);
        RegisterAllocator::new(list)
    }
}
