// Converted from V8 C++ source files:
// Header: macro-assembler-mips64.h
// Implementation: macro-assembler-mips64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod mips64 {
use std::rc::Rc;
use std::sync::Mutex;
pub struct ExternalReference {}
pub struct Operand {}
pub struct Label {}
pub struct Register {}
pub struct Condition {}
pub struct FPUCondition {}
pub struct FPURegister {}
pub enum BranchDelaySlot {
    USE_DELAY_SLOT,
    PROTECT,
}
pub enum LiFlags {
    OPTIMIZE_SIZE = 0,
    CONSTANT_SIZE = 1,
    ADDRESS_LOAD = 2,
}
pub enum RAStatus {
    kRAHasNotBeenSaved,
    kRAHasBeenSaved,
}
pub struct MacroAssembler {}
pub struct Isolate {}
pub struct AbortReason {}
pub struct SavedRegisters {}
pub struct Smi {}
pub struct RootIndex {}
pub struct HeapObject {}
pub struct RelocInfo {}
pub struct FeedbackVector {}
pub struct Context {}
pub struct Instruction {}
pub struct Builtins {}
pub struct CodeEntrypointTag {}
pub enum JumpMode {}
pub struct MSARegister {}
pub enum MSASize {}
pub enum MSABranchDF {}
pub enum MSABranchCondition {}
pub struct MemoryChunk {}
pub struct SaveFPRegsMode {}
pub struct Tagged {}
pub struct Base {}
pub struct Address {}
pub struct MachineOperatorBuilder {}
pub struct Frame {}
pub struct DirectHandle {}
pub struct FixedArray {}
pub enum StubCallMode {}
pub enum ConvertReceiverMode {}
pub struct DoubleRegList {}
pub struct RegList {}
pub struct Immediate {}
pub struct AstRawString {}
pub struct Code {}
pub struct Space {}
pub struct CodePointerHandle {}
pub struct OperationType {}
pub struct Cancelable {}
pub mod internal {
pub struct SharedObjectConveyorHandles {}
}
pub struct Zone {}
pub enum VariableMode {}
pub struct CodeKind {}
pub struct VMState {}
pub enum StackFrame {}
pub struct ZonePtrList {}
pub struct CaseClause {}
pub struct AstNode {}
pub struct AstNodeSourceRanges {}
pub struct SharedFunctionInfo {}
pub struct BytecodeArrayWrapper {}
pub struct ObjectHashTable {}
pub struct InstructionBase {}
pub struct MemOperand {}
pub struct Tagged_t {}
pub mod base {
pub struct Double {}
}
pub struct StoreRepresentation {}
pub struct UnoptimizedCompileFlags {}
pub struct MSARegister {}
pub struct MSAControlRegister {}
pub struct Shift {}
pub enum class MSADataType {}
pub struct CPURegister {}
pub mod CPURegister {
pub enum class RegisterType {}
}
pub struct RelocInfo {}
pub enum class ArgvMode {}
pub mod wasm {
pub struct WasmCode {}
}
pub mod base {
pub mod bits {
fn IsPowerOfTwo(value:i64) -> bool { todo!() }
fn CountTrailingZeros64(value:i64) -> i32 { todo!() }
}
}
pub struct WasmCodePointerTableEntry {}
pub struct InstructionSequence {}
pub mod js_typification {
pub mod Types {}
}
pub mod feedback_vector {
fn CompileOptimizedCode(result: Self) -> Self {
}
fn feedback(&self) -> Option<i32> {
    None
}
fn has_feedback(&self) -> bool { true }
}
pub fn from(_: Handle<Code>) -> Self {
}
pub struct VRegister {}
pub struct Shift {}
pub enum class Condition {}
pub enum Register {
    no_reg,
    t1,
    at,
	kRootRegister,
	a0,
	a1,
	t8,
	fp,
	s6,
	t9,
	kJavaScriptCallArgCountRegister,
	v0,
	v1,
	a3,
    zero_reg,
	kScratchReg,
    kDoubleRegZero,
    
}

pub struct RegisterConfiguration {}
pub enum MemoryRepresentation {}
pub struct V<T> {}
pub enum class FeedbackSource {}
pub struct BuiltinEntry {}
pub enum isize {}
pub struct Local<'a, T> {}
pub struct DoubleRegister {}
pub mod RelocInfo {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
   NO_FLAGS,
	CODE_TARGET,
    OFF_HEAP_TARGET,
    WASM_STUB_CALL,
    WASM_CALL,
    WASM_SHARED_FUNCTION_CALL,
    CODE_TARGET_WITH_INSIDE_START_BYTE,
    RELATIVE_CODE_TARGET,
    JS_FUNCTION_LINK,
    EMBEDDED_OBJECT,
    EMBEDDED_IMMOVABLE_OBJECT,
    EXTERNAL_REFERENCE,
	wasm_canonical_sig_id,
    WASM_CODE_POINTER_TABLE_ENTRY,
    kArm64LdrFpRelSmallImm,
}
}
pub mod StackFrame {
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Type {
        NO_FRAME_TYPE,
        INTERNAL,
        JAVA_SCRIPT,
        WASM,
        CONSTRUCT,
		EXIT,
        WASM_LIFTOFF_SETUP,
        BUILTIN_EXIT,
        API_ACCESSOR_EXIT,
        API_CALLBACK_EXIT,
        TURBOSHAFT,
    }
    pub fn TypeToMarker(type_:Type) -> i32 { todo!() }
    pub fn IsJavaScript(t:Type) -> bool {
		t == Type::JAVA_SCRIPT
    }
}
pub struct Range {}
pub struct OffsetAccessType {}
pub struct TaggedIndex {}
pub struct Builtins {}
pub fn NegateCondition(_cc: Condition) -> Condition { todo!() }
pub struct InstructionOperand {}
pub struct FeedbackCell {}
pub struct Location {}
pub mod JSFunction {
const kCodeOffset:i32 = 0;
const kContextOffset:i32 = 0;
const kFeedbackCellOffset:i32 = 0;
const kDispatchHandleOffset:i32 = 0;
}
pub mod FeedbackVector {
const kFlagsOffset:i32 = 0;
const kMaybeOptimizedCodeOffset:i32 = 0;
}
pub struct CPURegList {}
pub fn kUnsignedLessThanEqual() -> Condition { todo!() }
pub fn CodeKindCanTierUp(_code_kind: CodeKind) -> bool { todo!() }
pub fn Feedbacks(object: Self) -> Self { todo!() }
pub struct Debug {}
pub fn ToRegister(code: i32) -> Register { todo!() }
pub fn SmiValuesAre32Bits() -> bool {todo!()}
pub fn SmiValuesAre31Bits() -> bool {todo!()}
pub fn Address() -> Address {todo!()}
fn InstructionsGeneratedSince(arg: Self) -> i32 {todo!()}
pub struct V8_EXPORT_PRIVATE {}
pub fn root_array_available() -> bool {todo!()}
pub fn RootRegisterOffsetForExternalReferenceTableEntry(isolate:Self, reference: ExternalReference) -> i32 {todo!()}
pub struct AssemblerBase {}
impl MacroAssembler {
fn Branch(arg0: Self, eq: Condition, arg1: Self, scratch: Operand){ todo!()}
fn SetIsolateDataSlots() -> Self {todo!()}
fn LoadRootRelative(arg0: Register, arg1: i32){todo!()}
fn BranchAndLinkShortCheck(arg0: i32, l: Self, al: Condition, arg1: Register, scratch: Self, kProtect: BranchDelaySlot)-> bool{ todo!()}
fn Daddu(arg0: Register, arg1: Register, scratch: Operand){todo!()}
fn is_int16(immediate: i32) -> bool {todo!()}
fn li(t8: Register, a1: i32){todo!()}
fn BranchLong(l: &Label, kProtect: BranchDelaySlot){todo!()}
fn Bind(done: &Label){todo!()}
fn Dsubu(sp: Register, t3: Register, imm: i32){todo!()}
fn Sw(arg0: Register, arg1: MemOperand){todo!()}
fn Pop(d: Register){todo!()}
fn Mov(v: Register, sp: Register){todo!()}
fn LoadConstantPool(cp: Register, t3: Code) {}
fn CallRuntime(code: &Code){}
fn LoadMap(arg0: Register, object: Register){}
fn CheckDebugHook(fun: Register, new_target: Register, param_count: i32, arg_count: Register){}
fn LoadEntryFromBuiltinIndex(destination: Register, builtin_index: Register){}
fn LoadEntryFromBuiltin(builtin: Builtin, entry: Register){todo!()}
fn Call(target: Register) {}
fn RegisterConfiguration() -> RegisterConfiguration { todo!()}
fn Call(arg0: This, target_address: Address) -> Self {todo!()}
fn Jump(ra: Register) {todo!()}
fn Nal(this: &mut Self) {}
fn Ld(reg : Register,memop: MemOperand){}
fn Push(t2: Register, sp: Register){todo!()}
fn Pop(to: Register, src: Self){todo!()}
fn Mips64ArchVariant() -> i32 {todo!()}
fn is_int32(immediate: i64) -> bool {todo!()}
fn RecordRelocInfo(offset_heap_object: i32){}
fn TestCFlags() -> i32 {todo!()}
fn get_current_code() -> Code { todo!()}
fn CallRecordWriteStub(arg: Register, a7: Register, sav: SaveFPRegsMode){todo!()}
fn CallRecordWriteStubSaveRegisters(a: Register, aa: Register, sav: SaveFPRegsMode){}
fn SmiTst(object:Register, tmp:Register){}
fn AddPoisonByte(a: Register){}
fn is_near(arg: *mut i32, scratch: DoubleRegister) -> DoubleRegister {todo!()}
fn Dinsm(arg0: Register, t3: Register, u32: i32){}
fn Jic(t1: Self) {}
fn blt(rd: Self, ra: Register) {todo!()}
fn LoadRootRelative(rd: Register, ra: Self){}
fn Sd(arg0: Register, object: MemOperand){}
fn Movn(arg0: Register, r0: Register, src: Register){todo!()}
fn dsll(arg0: Register, scratch: Self, y : i32){}
fn dmfc1(t2: Register, src: Register) {todo!()}
fn ext_(tmp:Register, reg:Register, shift:i32, mask:i32){}
fn addiu(arg: Register, dst: Register, kNumInitialRegisterArguments: isize){}
fn Bnezal(scratch: Register, scratch2 : i32){}
fn And(r0: Register, scratch: Register, x: u32){}
fn Balc(v: i32){}
fn ExtMulLow(d: MSADataType, src1: MSARegister, v1: Register, tmp: Register) {}
fn sll(tmp: Register, scratch2: Register, code: i32) {}
fn li(arg1: Register, zeroReg: i32) {
}
}
}
