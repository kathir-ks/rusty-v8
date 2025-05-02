#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// TODO: Add necessary Rust crates

// use crate::base::bits;
// use crate::builtins;
// use crate::codegen::{assembler, callable, code_factory, external_reference_table, interface_descriptors, macro_assembler, register_configuration};
// use crate::debug::debug;
// use crate::deoptimizer::deoptimizer;
// use crate::execution::frames;
// use crate::heap::mutable_page_metadata;
// use crate::init::bootstrapper;
// use crate::logging::counters;
// use crate::objects::heap_number;
// use crate::runtime::runtime;
// use crate::snapshot::snapshot;

// use std::optional::Optional;

// MacroAssembler equivalent in Rust
pub struct MacroAssembler {
    // TODO: Add fields as necessary, representing the assembler state.
    root_array_available_: bool,
    has_double_zero_reg_set_: bool,
    options_: MacroAssemblerOptions,
    isolate_: Option<Isolate>, // Assuming Isolate is a struct
}

#[derive(Clone, Copy)]
pub struct MacroAssemblerOptions {
    isolate_independent_code: bool,
    enable_root_relative_access: bool,
    compress_pointers: bool,
    // ... other options
}

impl MacroAssemblerOptions {
    fn new() -> Self {
        Self {
            isolate_independent_code: false,
            enable_root_relative_access: false,
            compress_pointers: false,
            // ... other options
        }
    }
}

impl MacroAssembler {
    pub fn new(root_array_available: bool) -> Self {
        Self {
            root_array_available_: root_array_available,
            has_double_zero_reg_set_: false,
            options_: MacroAssemblerOptions::new(),
            isolate_: None,
        }
    }

    pub fn set_options(&mut self, options: MacroAssemblerOptions) {
        self.options_ = options;
    }

    pub fn set_isolate(&mut self, isolate: Isolate) {
        self.isolate_ = Some(isolate);
    }

    fn options(&self) -> MacroAssemblerOptions {
        self.options_
    }

    fn isolate(&self) -> Option<&Isolate> {
        self.isolate_.as_ref()
    }
}

// Constants
const kSystemPointerSize: i32 = 8;
const kDoubleSize: i32 = 8;
const kTaggedSize: i32 = 4;
const kHeapObjectTag: i32 = 1;
const kImm12Mask: i64 = 0xFFF;
const kSandboxedPointerShift: i32 = 1;
const kExternalPointerIndexShift: i32 = 1;
const kExternalPointerShiftedTagMask: i32 = 0x7f;
const kExternalPointerPayloadMask: i32 = 0x7f;
const kExternalPointerTagShift: i32 = 1;
const kCodePointerHandleMarker: i32 = 1;
const kCodePointerHandleShift: i32 = 1;
const kTrustedPointerHandleShift: i32 = 1;
const kTrustedPointerTableMarkBit: i32 = 1;
const kCodePointerTableEntrySizeLog2: i32 = 1;
const kTrustedPointerTableEntrySizeLog2: i32 = 1;
const kJSDispatchHandleShift: i32 = 1;
const kJSDispatchTableEntrySizeLog2: i32 = 1;
const kPtrComprCageBaseRegister: Register = Register::gp(0); // Example register
const kRootRegister: Register = Register::gp(1); // Example register

macro_rules! V8_STATIC_ROOTS_BOOL {
    () => {
        true // Replace with actual logic if needed
    };
}

macro_rules! COMPRESS_POINTERS_BOOL {
    () => {
        true // Replace with actual logic if needed
    };
}

macro_rules! V8_ENABLE_SANDBOX {
    () => {
        true // Replace with actual logic if needed
    };
}

macro_rules! V8_ENABLE_WEBASSEMBLY {
    () => {
        true // Replace with actual logic if needed
    };
}

macro_rules! V8_ENABLE_LEAPTIERING {
    () => {
        true // Replace with actual logic if needed
    };
}

// Enums
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SaveFPRegsMode {
    kIgnore,
    kSave,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SmiCheck {
    kInline,
    kOmit,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RAStatus {
    kRAHasBeenSaved,
    kRAHasNotBeenSaved,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum StubCallMode {
    kCallBuiltinPointer,
    kCallWasmRuntimeStub,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ComparisonMode {
    kFullPointer,
    kZeroExtendSmi,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum IndirectPointerTag {
    kUnknownIndirectPointerTag,
    kCodeIndirectPointerTag,
    kTrustedIndirectPointerTag,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CodeEntrypointTag {
    kInvalidEntrypointTag,
    kDefault,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Condition {
    eq,
    ne,
    lt,
    gt,
    le,
    ge,
    lo,
    hs,
    hi,
    ls,
    Ugreater,
    Ugreater_equal,
    Uless,
    Uless_equal,
    greater,
    greater_equal,
    less,
    less_equal,
    cc_always
}

fn NegateCondition(condition: Condition) -> Condition {
    match condition {
        Condition::eq => Condition::ne,
        Condition::ne => Condition::eq,
        Condition::lt => Condition::ge,
        Condition::gt => Condition::le,
        Condition::le => Condition::gt,
        Condition::ge => Condition::lt,
        Condition::lo => Condition::hs,
        Condition::hs => Condition::lo,
        Condition::hi => Condition::ls,
        Condition::ls => Condition::hi,
        Condition::Ugreater => Condition::Uless_equal,
        Condition::Ugreater_equal => Condition::Uless,
        Condition::Uless => Condition::Ugreater_equal,
        Condition::Uless_equal => Condition::Ugreater,
        Condition::greater => Condition::less_equal,
        Condition::greater_equal => Condition::less,
        Condition::less => Condition::greater_equal,
        Condition::less_equal => Condition::greater,
        Condition::cc_always => Condition::cc_always,
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FPURoundingMode {
    mode_floor,
    mode_ceil,
    mode_trunc,
    mode_round,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FPUCondition {
    CUN,
    CULT,
    CULE,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CFRegister {}

impl MacroAssembler {
    fn is_int12(value: i64) -> bool {
        value >= -2048 && value <= 2047
    }

    fn is_uint12(value: i64) -> bool {
        value >= 0 && value <= 4095
    }

    fn is_int14(value: i64) -> bool {
        value >= -8192 && value <= 8191
    }

    fn is_int16(value: i64) -> bool {
        value >= -32768 && value <= 32767
    }

    fn is_int22(value: i64) -> bool {
        value >= -(1 << 21) && value < (1 << 21)
    }

    fn is_int28(value: i64) -> bool {
        value >= -(1 << 27) && value < (1 << 27)
    }

    fn is_int32(value: i64) -> bool {
        value >= i32::MIN && value <= i32::MAX
    }

    fn is_int52(value: i64) -> bool {
        value >= -(1 << 51) && value < (1 << 51)
    }

    fn is_uint32(value: i64) -> bool {
        value >= 0 && value <= u32::MAX as i64
    }
}

// Dummy structs and enums for now.  Replace with actual definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register {
    code: i32,
    is_fpu: bool,
}

impl Register {
    const fn gp(code: i32) -> Self {
        Register { code, is_fpu: false }
    }

    const fn fpu(code: i32) -> Self {
        Register { code, is_fpu: true }
    }

    fn code(&self) -> i32 {
        self.code
    }

    fn is_valid(&self) -> bool {
        self.code >= 0
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.code)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FPURegister {
    code: i32,
}

impl FPURegister {
    const fn from_code(code: i32) -> Self {
        FPURegister { code }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Operand {
    imm: i64,
    reg: Register,
    is_reg: bool,
    rmode: RelocInfoMode,
    heap_number_request: i32,
    is_heap_number_request: bool,
}

impl Operand {
    pub fn immediate(imm: i64) -> Self {
        Operand {
            imm: imm,
            reg: Register::gp(-1), // Invalid register
            is_reg: false,
            rmode: RelocInfoMode::NoRelocation,
            heap_number_request: 0,
            is_heap_number_request: false,
        }
    }

    pub fn register(reg: Register) -> Self {
        Operand {
            imm: 0,
            reg: reg,
            is_reg: true,
            rmode: RelocInfoMode::NoRelocation,
            heap_number_request: 0,
            is_heap_number_request: false,
        }
    }

    fn is_reg(&self) -> bool {
        self.is_reg
    }

    fn immediate(&self) -> i64 {
        self.imm
    }

    fn rm(&self) -> Register {
        self.reg
    }

    fn rmode(&self) -> RelocInfoMode {
        self.rmode
    }

    fn IsHeapNumberRequest(&self) -> bool {
        self.is_heap_number_request
    }

    fn heap_number_request(&self) -> i32 {
        self.heap_number_request
    }
}

// Implement traits for conversion
impl From<i64> for Operand {
    fn from(imm: i64) -> Self {
        Operand::immediate(imm)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemOperand {
    base: Register,
    offset: i32,
    index: Register,
    has_index: bool,
}

impl MemOperand {
    pub fn new(base: Register, offset: i32) -> Self {
        MemOperand {
            base: base,
            offset: offset,
            index: Register::gp(-1), // Invalid register
            has_index: false,
        }
    }

    pub fn with_index(base: Register, index: Register) -> Self {
        MemOperand {
            base: base,
            offset: 0,
            index: index,
            has_index: true,
        }
    }

    fn base(&self) -> Register {
        self.base
    }

    fn offset(&self) -> i32 {
        self.offset
    }

    fn index(&self) -> Register {
        self.index
    }

    fn hasIndexReg(&self) -> bool {
        self.has_index
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FieldMemOperand {
    base: Register,
    offset: i32,
}

impl FieldMemOperand {
    pub fn new(base: Register, offset: i32) -> Self {
        FieldMemOperand {
            base: base,
            offset: offset,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RelocInfoMode {
    NoRelocation,
    WASM_CALL,
    WASM_STUB_CALL,
    // ... other relocation modes
    EmbeddedObject, // For Handle<HeapObject>
    CompressedEmbeddedObject,
    FullEmbeddedObject,
    WasmCanonicalSigId,
    WasmCodePointerTableEntry,
    JSDispatchHandle
}

// Example definition.  Add others as needed.
const zero_reg: Register = Register::gp(0);
const ra: Register = Register::gp(1);
const fp: Register = Register::gp(2);
const sp: Register = Register::gp(3);
const cp: Register = Register::gp(4);
const s6: Register = Register::gp(6);
const t7: Register = Register::gp(7);

const kJavaScriptCallArgCountRegister: Register = Register::gp(5);

const kCallerSavedFPU: DoubleRegList = DoubleRegList {
    bits_: 0, // Replace with the actual bitmask
};
const kDoubleRegZero: FPURegister = FPURegister { code: 0 };

#[derive(Debug, Clone, Copy)]
pub struct RegList {
    bits_: u64,
}

impl RegList {
    fn is_empty(&self) -> bool {
        self.bits_ == 0
    }

    fn Count(&self) -> i32 {
        self.bits_.count_ones() as i32
    }

    fn bits(&self) -> u64 {
        self.bits_
    }

    fn is_valid(&self) -> bool {
        true
    }
}

// Implement bitwise operators for RegList
impl std::ops::BitAnd for RegList {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self { bits_: self.bits_ & other.bits_ }
    }
}

impl std::ops::BitXor for RegList {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self { bits_: self.bits_ ^ other.bits_ }
    }
}

impl std::ops::Sub for RegList {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { bits_: self.bits_ & !other.bits_ }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DoubleRegList {
    bits_: u64,
}

impl DoubleRegList {
    fn Count(&self) -> i32 {
        self.bits_.count_ones() as i32
    }

    fn bits(&self) -> u64 {
        self.bits_
    }
}

trait Assembler {
    fn code_size(&self) -> usize;
    fn pc_offset(&self) -> usize;
    fn emitb(&mut self, byte: u8);
}

// Dummy macro
macro_rules! ASM_CODE_COMMENT {
    ($masm:ident) => {
        // No-op in Rust version
    };
}

macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! CHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("CHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

macro_rules! USE {
    ($x:expr) => {
        let _ = $x;
    }
}

impl MacroAssembler {

    fn is_near(&self, target: &Label, offset_size: OffsetSize) -> bool {
        // TODO: Implement the near calculation logic.
        true // Placeholder
    }

    fn branch_offset_helper(&self, l: &Label, bits: OffsetSize) -> i32 {
        // Placeholder for branch offset calculation
        0 // Placeholder
    }

    fn set_pc_for_safepoint(&mut self) {
        // Placeholder for set pc
    }

    fn AddEmbeddedObject(&mut self, handle: &Handle<HeapObject>) -> u32 {
        0 // Placeholder
    }

    fn AddCodeTarget(&mut self, handle: &Handle<Code>) -> i32 {
        0 // Placeholder
    }

    fn RequestHeapNumber(&mut self, request: i32) -> () {
        () // Placeholder
    }

    fn RecordRelocInfo(&mut self, rmode: RelocInfoMode, immediate: i64) -> () {
        () // Placeholder
    }
}

impl MacroAssembler {
    fn RequiredStackSizeForCallerSaved(
        &self,
        fp_mode: SaveFPRegsMode,
        exclusion1: Register,
        exclusion2: Register,
        exclusion3: Register,
    ) -> i32 {
        let mut bytes = 0;

        let exclusions = RegList {
            bits_: (1 << exclusion1.code()) | (1 << exclusion2.code()) | (1 << exclusion3.code()),
        };
        let list = RegList {
            bits_: (0xFFFF_FFFF_FFFF_FFFFu64) & !(exclusions.bits()), // Assuming all registers are caller-saved
        }; //kJSCallerSaved - exclusions;
        bytes += list.Count() * kSystemPointerSize;

        if fp_mode == SaveFPRegsMode::kSave {
            bytes += kCallerSavedFPU.Count() * kDoubleSize;
        }

        bytes
    }

    fn PushCallerSaved(
        &mut self,
        fp_mode: SaveFPRegsMode,
        exclusion1: Register,
        exclusion2: Register,
        exclusion3: Register,
    ) -> i32 {
        ASM_CODE_COMMENT!(self);
        let mut bytes = 0;

        let exclusions = RegList {
            bits_: (1 << exclusion1.code()) | (1 << exclusion2.code()) | (1 << exclusion3.code()),
        };
        let list = RegList {
            bits_: (0xFFFF_FFFF_FFFF_FFFFu64) & !(exclusions.bits()), // Assuming all registers are caller-saved
        };
        self.MultiPush(list);
        bytes += list.Count() * kSystemPointerSize;

        if fp_mode == SaveFPRegsMode::kSave {
            self.MultiPushFPU(kCallerSavedFPU);
            bytes += kCallerSavedFPU.Count() * kDoubleSize;
        }

        bytes
    }

    fn PopCallerSaved(
        &mut self,
        fp_mode: SaveFPRegsMode,
        exclusion1: Register,
        exclusion2: Register,
        exclusion3: Register,
    ) -> i32 {
        ASM_CODE_COMMENT!(self);
        let mut bytes = 0;
        if fp_mode == SaveFPRegsMode::kSave {
            self.MultiPopFPU(kCallerSavedFPU);
            bytes += kCallerSavedFPU.Count() * kDoubleSize;
        }

        let exclusions = RegList {
            bits_: (1 << exclusion1.code()) | (1 << exclusion2.code()) | (1 << exclusion3.code()),
        };
        let list = RegList {
            bits_: (0xFFFF_FFFF_FFFF_FFFFu64) & !(exclusions.bits()), // Assuming all registers are caller-saved
        };
        self.MultiPop(list);
        bytes += list.Count() * kSystemPointerSize;

        bytes
    }

    fn LoadRoot(&mut self, destination: Register, index: RootIndex) {
        if V8_STATIC_ROOTS_BOOL!() && RootsTable::IsReadOnly(index) && Self::is_int12(ReadOnlyRootPtr(index) as i64) {
            self.DecompressTagged(destination, ReadOnlyRootPtr(index) as i64);
            return;
        }
        // Many roots have addresses that are too large to fit into addition immediate
        // operands. Evidence suggests that the extra instruction for decompression
        // costs us more than the load.
        self.Ld_d(destination, MemOperand::new(s6, RootRegisterOffsetForRootIndex(index)));
    }

    fn LoadTaggedRoot(&mut self, destination: Register, index: RootIndex) {
        if V8_STATIC_ROOTS_BOOL!() && RootsTable::IsReadOnly(index) && Self::is_int12(ReadOnlyRootPtr(index) as i64) {
            self.li(destination, ReadOnlyRootPtr(index) as i32);
            return;
        }
        self.Ld_w(destination, MemOperand::new(s6, RootRegisterOffsetForRootIndex(index)));
    }

    fn PushCommonFrame(&mut self, marker_reg: Register) {
        if marker_reg.is_valid() {
            self.Push(ra, fp, marker_reg);
            self.Add_d(fp, sp, Operand::from(kSystemPointerSize as i64));
        } else {
            self.Push(ra, fp);
            self.mov(fp, sp);
        }
    }

    fn PushStandardFrame(&mut self, function_reg: Register) {
        let offset = -StandardFrameConstants::kContextOffset;
        if function_reg.is_valid() {
            self.Push(ra, fp, cp, function_reg, kJavaScriptCallArgCountRegister);
            //offset += 2 * kSystemPointerSize;  // This assignment is unused
        } else {
            self.Push(ra, fp, cp, kJavaScriptCallArgCountRegister);
            //offset += kSystemPointerSize; // This assignment is unused
        }
        self.Add_d(fp, sp, Operand::from(offset as i64));
    }

    // Clobbers object, dst, value, and ra, if (ra_status == kRAHasBeenSaved)
    // The register 'object' contains a heap object pointer.  The heap object
    // tag is shifted away.
    fn RecordWriteField(
        &mut self,
        object: Register,
        offset: i32,
        value: Register,
        ra_status: RAStatus,
        save_fp: SaveFPRegsMode,
        smi_check: SmiCheck,
        slot: SlotDescriptor,
    ) {
        ASM_CODE_COMMENT!(self);
        // First, check if a write barrier is even needed. The tests below
        // catch stores of Smis.
        let mut done = Label::new();

        // Skip barrier if writing a smi.
        if smi_check == SmiCheck::kInline {
            self.JumpIfSmi(value, &mut done);
        }

        // Although the object register is tagged, the offset is relative to the start
        // of the object, so offset must be a multiple of kPointerSize.
        DCHECK!(Self::IsAligned(offset, kTaggedSize));

        if true {
            // if v8_flags.slow_debug_code {
            let mut ok = Label::new();
            let mut temps = UseScratchRegisterScope::new(self);
            let scratch = temps.Acquire();
            self.Add_d(scratch, object, Operand::from((offset - kHeapObjectTag) as i64));
            self.And(scratch, scratch, Operand::from((kTaggedSize - 1) as i64));
            self.Branch(&mut ok, Condition::eq, scratch, Operand::register(zero_reg));
            self.Abort(AbortReason::kUnalignedCellInWriteBarrier);
            self.bind(&mut ok);
        }

        self.RecordWrite(
            object,
            Operand::from((offset - kHeapObjectTag) as i64),
            value,
            ra_status,
            save_fp,
            SmiCheck::kOmit,
            slot,
        );

        self.bind(&mut done);
    }

    fn DecodeSandboxedPointer(&mut self, value: Register) {
        ASM_CODE_COMMENT!(self);
        if V8_ENABLE_SANDBOX!() {
            self.srli_d(value, value, kSandboxedPointerShift as i64);
            self.Add_d(value, value, Operand::from(kPtrComprCageBaseRegister.code() as i64));
        } else {
            UNREACHABLE!();
        }
    }

    fn LoadSandboxedPointerField(&mut self, destination: Register, field_operand: MemOperand) {
        if V8_ENABLE_SANDBOX!() {
            ASM_CODE_COMMENT!(self);
            self.Ld_d(destination, field_operand);
            self.DecodeSandboxedPointer(destination);
        } else {
            UNREACHABLE!();
        }
    }

    fn StoreSandboxedPointerField(&mut self, value: Register, dst_field_operand: MemOperand) {
        if V8_ENABLE_SANDBOX!() {
            ASM_CODE_COMMENT!(self);
            let mut temps = UseScratchRegisterScope::new(self);
            let scratch = temps.Acquire();
            self.Sub_d(scratch, value, Operand::from(kPtrComprCageBaseRegister.code() as i64));
            self.slli_d(scratch, scratch, kSandboxedPointerShift as i64);
            self.St_d(scratch, dst_field_operand);
        } else {
            UNREACHABLE!();
        }
    }

    fn LoadExternalPointerField(
        &mut self,
        destination: Register,
        field_operand: MemOperand,
        tag_range: ExternalPointerTagRange,
        isolate_root: Register,
    ) {
        DCHECK!(!Self::AreAliased(destination, isolate_root));
        ASM_CODE_COMMENT!(self);
        if V8_ENABLE_SANDBOX!() {
            DCHECK!(!tag_range.IsEmpty());
            DCHECK!(!Self::IsSharedExternalPointerType(tag_range));
            let mut temps = UseScratchRegisterScope::new(self);
            let external_table = temps.Acquire();
            let mut isolate_root_local = isolate_root;

            if isolate_root == no_reg {
                DCHECK!(self.root_array_available_);
                isolate_root_local = kRootRegister;
            }
            self.Ld_d(
                external_table,
                MemOperand::new(
                    isolate_root_local,
                    IsolateData::external_pointer_table_offset() + Internals::kExternalPointerTableBasePointerOffset,
                ),
            );
            self.Ld_wu(destination, field_operand);
            self.srli_d(destination, destination, kExternalPointerIndexShift as i64);
            self.slli_d(destination, destination, kExternalPointerTableEntrySizeLog2 as i64);
            self.Ld_d(destination, MemOperand::new(external_table, 0));

            // We don't expect to see empty fields here. If this is ever needed, consider
            // using an dedicated empty value entry for those tags instead (i.e. an entry
            // with the right tag and nullptr payload).
            DCHECK!(!Self::ExternalPointerCanBeEmpty(tag_range));

            // We need another scratch register for the 64-bit tag constant. Instead of
            // forcing the `And` to allocate a new temp register (which we may not have),
            // reuse the temp register that we used for the external pointer table base.
            let scratch = external_table;
            if tag_range.Size() == 1 {
                // The common and simple case: we expect exactly one tag.
                //static_assert!(kExternalPointerShiftedTagMask == 0x7f); // TODO: Translate static_assert!
                self.bstrpick_d(scratch, destination, (kExternalPointerTagShift + 7) as u16, kExternalPointerTagShift as u16);
                self.SbxCheck(Condition::eq, AbortReason::kExternalPointerTagMismatch, scratch, Operand::from(tag_range.first as i64));
                self.And(destination, destination, Operand::from(kExternalPointerPayloadMask as i64));
            } else {
                // Not currently supported. Implement once needed.
                DCHECK!(tag_range != kAnyExternalPointerTagRange);
                UNREACHABLE!();
            }
        } else {
            self.Ld_d(destination, field_operand);
        } // V8_ENABLE_SANDBOX
    }

    fn LoadTrustedPointerField(
        &mut self,
        destination: Register,
        field_operand: MemOperand,
        tag: IndirectPointerTag,
    ) {
        if V8_ENABLE_SANDBOX!() {
            self.LoadIndirectPointerField(destination, field_operand, tag);
        } else {
            self.LoadTaggedField(destination, field_operand);
        }
    }

    fn StoreTrustedPointerField(&mut self, value: Register, dst_field_operand: MemOperand) {
        if V8_ENABLE_SANDBOX!() {
            self.StoreIndirectPointerField(value, dst_field_operand);
        } else {
            self.StoreTaggedField(value, dst_field_operand);
        }
    }

    fn LoadIndirectPointerField(
        &mut self,
        destination: Register,
        field_operand: MemOperand,
        tag: IndirectPointerTag,
    ) {
        if V8_ENABLE_SANDBOX!() {
            ASM_CODE_COMMENT!(self);
            let mut temps = UseScratchRegisterScope::new(self);
            let handle = temps.Acquire();
            self.Ld_wu(handle, field_operand);

            self.ResolveIndirectPointerHandle(destination, handle, tag);
        } else {
            UNREACHABLE!();
        } // V8_ENABLE_SANDBOX
    }

    fn StoreIndirectPointerField(&mut self, value: Register, dst_field_operand: MemOperand) {
        if V8_ENABLE_SANDBOX!() {
            let mut temps = UseScratchRegisterScope::new(self);
            let scratch = temps.Acquire();
            self.Ld_w(scratch, FieldMemOperand::new(value, ExposedTrustedObject::kSelfIndirectPointerOffset));
            self.St_w(scratch, dst_field_operand);
        } else {
            UNREACHABLE!();
        }
    }

    fn ResolveIndirectPointerHandle(&mut self, destination: Register, handle: Register, tag: IndirectPointerTag) {
        // The tag implies which pointer table to use.
        if tag == IndirectPointerTag::kUnknownIndirectPointerTag {
            // In this case we have to rely on the handle marking to determine which
            // pointer table to use.
            let mut is_trusted_pointer_handle = Label::new();
            let mut done = Label::new();

            DCHECK!(!Self::AreAliased(destination, handle));
            self.And(destination, handle, Operand::from(kCodePointerHandleMarker as i64));
            self.Branch(&mut is_trusted_pointer_handle, Condition::eq, destination, Operand::register(zero_reg));
            self.ResolveCodePointerHandle(destination, handle);
            self.Branch(&mut done);
            self.bind(&mut is_trusted_pointer_handle);
            self.ResolveTrustedPointerHandle(destination, handle, IndirectPointerTag::kUnknownIndirectPointerTag);
            self.bind(&mut done);
        } else if tag == IndirectPointerTag::kCodeIndirectPointerTag {
            self.ResolveCodePointerHandle(destination, handle);
        } else {
            self.ResolveTrustedPointerHandle(destination, handle, tag);
        }
    }

    fn ResolveTrustedPointerHandle(&mut self, destination: Register, handle: Register, tag: IndirectPointerTag) {
        DCHECK!(tag != IndirectPointerTag::kCodeIndirectPointerTag);
        DCHECK!(!Self::AreAliased(handle, destination));

        DCHECK!(self.root_array_available_);
        let table = destination;
        self.Ld_d(table, MemOperand::new(kRootRegister, IsolateData::trusted_pointer_table_offset()));
        self.srli_d(handle, handle, kTrustedPointerHandleShift as i64);
        self.Alsl_d(destination, handle, table, kTrustedPointerTableEntrySizeLog2 as u8);
        self.Ld_d(destination, MemOperand::new(destination, 0));
        // Untag the pointer and remove the marking bit in one operation.
        let tag_reg = handle;
        self.li(tag_reg, Operand::from(!(tag as i32 | kTrustedPointerTableMarkBit) as i64));
        self.and_(destination, destination, tag_reg);
    }

    fn ResolveCodePointerHandle(&mut self, destination: Register, handle: Register) {
        