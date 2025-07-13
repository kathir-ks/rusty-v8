// Converted from V8 C++ source files:
// Header: simulator-mips64.h
// Implementation: simulator-mips64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::sync::Mutex;

pub enum Register {
    no_reg = -1,
    zero_reg = 0,
    at,
    v0,
    v1,
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    t0,
    t1,
    t2,
    t3,
    s0,
    s1,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    t8,
    t9,
    k0,
    k1,
    gp,
    sp,
    s8,
    ra,
    LO,
    HI,
    pc,
    kNumSimuRegisters,
    fp = s8,
}

pub enum FPURegister {
    f0,
    f1,
    f2,
    f3,
    f4,
    f5,
    f6,
    f7,
    f8,
    f9,
    f10,
    f11,
    f12,
    f13,
    f14,
    f15,
    f16,
    f17,
    f18,
    f19,
    f20,
    f21,
    f22,
    f23,
    f24,
    f25,
    f26,
    f27,
    f28,
    f29,
    f30,
    f31,
    kNumFPURegisters
}

pub enum MSARegister {
    w0,
    w1,
    w2,
    w3,
    w4,
    w5,
    w6,
    w7,
    w8,
    w9,
    w10,
    w11,
    w12,
    w13,
    w14,
    w15,
    w16,
    w17,
    w18,
    w19,
    w20,
    w21,
    w22,
    w23,
    w24,
    w25,
    w26,
    w27,
    w28,
    w29,
    w30,
    w31,
    kNumMSARegisters
}

struct Instruction {}
impl Instruction {
    fn IsForbiddenAfterBranch(&self) -> bool {
        false
    }
    fn OpcodeValue(&self) -> i32 {
        0
    }
    fn IsTrap(&self) -> bool {
        false
    }
    fn Bits(&self, arg1: i32, arg2: i32) -> u32 {0}
    fn InstructionBits(&self) -> i32 {0}
    fn FunctionFieldRaw(&self) -> i32 {0}
    fn InstructionType(&self) -> i32 {0}
    fn RtValue(&self) -> i32 {0}
    fn RsValue(&self) -> i32 {0}
    fn SaValue(&self) -> i32 {0}
    fn IsLinkingInstruction(&self) -> bool { false}
    fn Imm16Value(&self) -> i16 { 0 }
    fn Imm18Value(&self) -> i32 { 0 }
    fn ImmValue(&self, bits: i32) -> i32 {0}
    fn FunctionFieldRaw(&self) -> u32 {0}
    fn FbccValue(&self) -> u32 { 0 }
    fn IsMSABranchInstr(&self) -> bool { false }
    fn WsValue(&self) -> i32 { 0 }
    fn WdValue(&self) -> i32 { 0 }
    fn MsaImm8Value(&self) -> i8 { 0 }
    fn MsaBitDf(&self) -> i32 { 0 }
    fn MsaElmDf(&self) -> i32 { 0 }
    fn FTrueValue(&self) -> bool { false}
    fn FrValue(&self) -> i32 { 0 }
    fn FsValue(&self) -> i32 { 0 }
    fn FtValue(&self) -> i32 { 0 }
    fn FdValue(&self) -> i32 { 0 }
    fn MsaImm5Value(&self) -> i8 { 0 }
    fn MsaImm10Value(&self) -> i16 { 0 }
    fn LsaSaValue(&self) -> i32 { 0 }
    fn Imm26Value(&self) -> i32 { 0 }
    fn MsaImmMI10Value(&self) -> i16 { 0 }
    fn MsaBitMValue(&self) -> i32 { 0 }
    fn Msa3RFMinorOpcodeField(&self) -> u32 { 0 }
    fn BaseValue(&self) -> i32 { 0 }
    fn Imm9Value(&self) -> i32 { 0 }
    fn IsMSAType(&self) -> bool { false }
    fn FBtrueValue(&self) -> bool {false}
    fn IsForbiddenAfterBranch(&self) -> bool {false}
}
trait InstructionBase {
    fn InstructionType() -> i32;
}
struct SimInstruction {}
impl SimInstruction {
    fn RsValue(&self) -> i32 {0}
    fn RtValue(&self) -> i32 {0}
    fn RdValue(&self) -> i32 {0}
    fn WsValue(&self) -> i32 {0}
}

trait InstructionGetters<T> {
}

struct CachePage {}
impl CachePage {
    const LINE_VALID: i32 = 0;
    const LINE_INVALID: i32 = 1;

    const kPageShift: i32 = 12;
    const kPageSize: i32 = 1 << Self::kPageShift;
    const kPageMask: i32 = Self::kPageSize - 1;
    const kLineShift: i32 = 2;
    const kLineLength: i32 = 1 << Self::kLineShift;
    const kLineMask: i32 = Self::kLineLength - 1;

    fn new() -> CachePage {
        CachePage {}
    }
    fn ValidityByte(&self, _offset: i32) -> i32 {
        0
    }
    fn CachedData(&self, _offset: i32) -> i32 {
        0
    }

}

struct SimInstructionBase {}

impl SimInstructionBase {
    fn new() -> SimInstructionBase {
        SimInstructionBase {}
    }
    fn instr(&self) -> i32 {
        0
    }
    fn operand(&self) -> i32 {
        0
    }
    fn InstructionType(&self) -> i32 {
        0
    }
}

struct Simulator {
    registers_: [i64; 35],
    FPUregisters_: [i64; 64],
    FCSR_: u32,
    MSACSR_: u32,
    stack_: usize,
    stack_limit_: usize,
    pc_modified_: bool,
    icount_: i64,
    break_count_: i32,
    trace_buf_: String,
    last_debugger_input_: String,
    isolate_: *mut V8,
    break_pc_: *mut Instruction,
}
impl Simulator {
    const bad_ra: i32 = -1;
    const end_sim_pc: i32 = -2;
    const Unpredictable: i32 = 0xbadbeaf;
    const kStackProtectionSize: usize = 1024;
    const kMaxStopCode: i32 = 100;
    fn new(isolate: *mut V8) -> Simulator {
        Simulator {
            registers_: [0; 35],
            FPUregisters_: [0; 64],
            FCSR_: 0,
            MSACSR_: 0,
            stack_: 0,
            stack_limit_: 0,
            pc_modified_: false,
            icount_: 0,
            break_count_: 0,
            trace_buf_: String::new(),
            last_debugger_input_: String::new(),
            isolate_: isolate,
            break_pc_: std::ptr::null_mut(),
        }
    }
    fn set_register(&mut self, reg: i32, value: i64) {}
    fn set_register_word(&mut self, reg: i32, value: i32) {}
    fn set_dw_register(&mut self, dreg: i32, dbl: *const i32) {}
    fn get_register(&self, reg: i32) -> i64 {
        0
    }
    fn get_double_from_register_pair(&self, reg: i32) -> f64 {
        0.0
    }
    fn set_fpu_register(&mut self, fpureg: i32, value: i64) {}
    fn set_fpu_register_word(&mut self, fpureg: i32, value: i32) {}
    fn set_fpu_register_hi_word(&mut self, fpureg: i32, value: i32) {}
    fn set_fpu_register_float(&mut self, fpureg: i32, value: f32) {}
    fn set_fpu_register_double(&mut self, fpureg: i32, value: f64) {}
    fn set_fpu_register_invalid_result64(&mut self, original: f32, rounded: f32) {}
    fn set_fpu_register_invalid_result(&mut self, original: f32, rounded: f32) {}
    fn set_fpu_register_word_invalid_result(&mut self, original: f32, rounded: f32) {}
    fn set_fpu_register_invalid_result64(&mut self, original: f64, rounded: f64) {}
    fn set_fpu_register_invalid_result(&mut self, original: f64, rounded: f64) {}
    fn set_fpu_register_word_invalid_result(&mut self, original: f64, rounded: f64) {}
    fn get_fpu_register(&self, fpureg: i32) -> i64 {
        0
    }
    fn get_fpu_register_word(&self, fpureg: i32) -> i32 {
        0
    }
    fn get_fpu_register_signed_word(&self, fpureg: i32) -> i32 {
        0
    }
    fn get_fpu_register_hi_word(&self, fpureg: i32) -> i32 {
        0
    }
    fn get_fpu_register_float(&self, fpureg: i32) -> f32 {
        0.0
    }
    fn get_fpu_register_double(&self, fpureg: i32) -> f64 {
        0.0
    }
    fn set_fcsr_bit(&mut self, cc: u32, value: bool) {}
    fn test_fcsr_bit(&self, cc: u32) -> bool {
        false
    }
    fn set_fcsr_round_error(&mut self, original: f64, rounded: f64) -> bool {
        false
    }
    fn set_fcsr_round64_error(&mut self, original: f64, rounded: f64) -> bool {
        false
    }
    fn set_fcsr_round_error(&mut self, original: f32, rounded: f32) -> bool {
        false
    }
    fn set_fcsr_round64_error(&mut self, original: f32, rounded: f32) -> bool {
        false
    }
    fn clear_fcsr_cause(&mut self) {}
    fn set_fcsr_rounding_mode(&mut self, mode: i32) {}
    fn set_msacsr_rounding_mode(&mut self, mode: i32) {}
    fn get_fcsr_rounding_mode(&self) -> u32 {0}
    fn get_msacsr_rounding_mode(&self) -> u32 {0}
    fn set_pc(&mut self, value: i64) {}
    fn get_pc(&self) -> i64 {
        0
    }
    fn get_sp(&self) -> usize {0}
    fn has_bad_pc(&self) -> bool {false}
    fn StackLimit(&self, c_limit: usize) -> usize {0}
    fn StackLimit(c_limit: usize) -> usize {0}
    fn set_last_debugger_input(&mut self, input: String) {}
    fn last_debugger_input(&self) -> &String {&self.last_debugger_input_}
    fn ICacheMatch(one: *mut i32, two: *mut i32) -> bool {false}
    fn SetRedirectInstruction(instruction: *mut i32) {}
    fn FlushICache(i_cache: *mut i32, start: *mut i32, size: usize) {}
    fn Execute(&mut self) {}
    fn now(&self) -> f64{0.0}
}
unsafe impl Send for Simulator {}
unsafe impl Sync for Simulator {}

lazy_static::lazy_static! {
    static ref SIMULATOR_MUTEX: Mutex<Option<*mut Simulator>> = Mutex::new(None);
}
impl Simulator {
  pub fn current(isolate: *mut V8) -> *mut Simulator {
    let mut guard = SIMULATOR_MUTEX.lock().unwrap();
    match *guard {
      Some(sim) => sim,
      None => {
        let sim = Box::into_raw(Box::new(Simulator::new(isolate)));
        *guard = Some(sim);
        sim
      }
    }
  }
}
pub enum V8 {}
pub enum GCInfoIndex {}
pub enum GCInfo {
    None,
}
pub struct IsolateData {}
pub struct Mutex {}
pub struct HeapHandle {}
pub enum CpuProfilingMode {}
pub struct CpuProfile {}
pub struct ProfilerId {}
pub struct String_ExternalOneByteStringResource {}
pub struct Tagged {}
pub struct HeapObject {}
pub struct NativeContext {}
pub struct Handle<T> {}
pub struct Tagged<T> {}
pub struct MaybeObject {}
pub struct DirectHandle<T> {}
pub struct ArrayList {}
pub struct FeedbackVector {}
pub struct Context {}
pub struct Value {}
pub struct Local<'a, T> {}
pub struct MaybeLocal<'a, T> {}
pub enum CpuProfileNode {}
pub struct Zone {}
pub enum ZoneMemoryBuffer {}
pub struct ZoneSegment {}
type AnyCType = i64;
pub struct EncodedCSignature {}
impl EncodedCSignature {
  pub fn ParameterCount(&self) -> i32 { 0 }
  pub fn IsReturnFloat(&self) -> bool { false }
  pub fn IsFloat(&self, _i: i32) -> bool { false }
  pub fn IsValid(&self) -> bool { false }
}
pub struct SimulatorData {}
impl SimulatorData {
  pub fn GetSignatureForTarget(&self, _target_address: usize) -> EncodedCSignature { EncodedCSignature {} }
}
mod base {
    pub mod bits {
        pub fn CountLeadingZeros64(_value: i64) -> i32 { 0 }
        pub fn SignedMulHigh64(_a: i64, _b: i64) -> i64 {0}
        pub fn UnsignedMulHigh64(_a: u64, _b: u64) -> u64 {0}
        pub fn RotateRight32(_value: u32, _shift: u32) -> u32 { 0 }
        pub fn RotateRight64(_value: u64, _shift: u32) -> u64 { 0 }
        
    }
    pub mod OS {
      pub fn ActivationFrameAlignment() -> i32 {0}
      pub fn Abort() {}
      pub fn DebugBreak() {}
    }
    pub mod platform {
        pub mod memory {
            pub fn AllocatePages(arg1: i32, arg2: i32, arg3: i32) -> i32 {0}
        }
        pub struct Platform {}
        impl Platform{
            pub fn GetSharedMemoryIdForSymbol(arg1: i32) -> i32 {0}
        }
    }
    pub mod strings {
        pub fn SNPrintF(_arg1: *mut i8, _arg2: &str) {}
        pub fn SNPrintF(trace_buf_: &String, _arg1: &str, _arg2: u64){}
    }
    pub struct EmbeddedVector<T, const N: usize> {}
    impl <T, const N: usize> EmbeddedVector<T, N>{
        pub fn begin(&self) -> *const T {std::ptr::null()}
    }
}
mod diagnostics {
    pub mod disasm {
        pub struct Disassembler {}
        impl Disassembler{
            pub fn InstructionDecode(_buffer: &String, _instr: *mut i32){}
        }
        pub struct NameConverter {}
    }
}
mod runtime {
    pub mod RuntimeUtils {
    }
}
mod codegen {
    pub mod assembler_inl {
        
    }
    pub mod mips64 {
        pub mod constants_mips64 {
            pub const kFCSRRegister: i32 = 0;
            pub const rtCallRedirInstr: i32 = 0;
        }
    }
}

pub enum ExternalReference {
  BUILTIN_FP_FP_CALL,
  BUILTIN_COMPARE_CALL,
  BUILTIN_FP_CALL,
  BUILTIN_FP_INT_CALL,
  BUILTIN_INT_FP_CALL,
  BUILTIN_FP_POINTER_CALL,
  DIRECT_API_CALL,
  BUILTIN_CALL,
  BUILTIN_CALL_PAIR,
  DIRECT_GETTER_CALL,
  FAST_C_CALL,
}

pub struct Redirection {}

impl Redirection {
  pub fn FromInstruction(_instr: *mut i32) -> *mut Redirection {
    std::ptr::null_mut()
  }
  pub fn external_function(&self) -> usize {0}
  pub fn type_(&self) -> ExternalReference { ExternalReference::BUILTIN_CALL }
}

pub struct MutexGuard {}
impl MutexGuard {
    fn NotifyStore_Locked(&self) {}
}
trait Isolate {
    fn stack_guard(&mut self) -> &mut StackGuard;
    fn simulator_data(&self) -> *mut SimulatorData;
}
pub struct StackGuard {}
impl StackGuard {
    fn AdjustStackLimitForSimulator(&mut self){}
}
pub struct SafeForInterruptsScope {}
pub struct Scope {}
pub struct Vector {}
pub enum Object {}
pub struct Any {
  x:i64,
  y:i64,
}
pub struct ExternalReferenceTable {}
pub enum Type {}
pub struct Address {}
pub struct InstructionStream {}
pub struct Instr {}
pub struct ScopeDescription {}
pub struct InnerPointerToCodeCacheEntry {}
pub struct BuiltinCode {}
pub struct Flags {}
pub struct PageMemoryAccessState {}
pub struct ExternalStringResource {}
pub struct String {}
impl String {
  pub fn Of(t: &str) -> String {
    String {}
  }
}
pub mod source_map {
    pub struct SourcePositionTable {}
}
pub struct SourcePosition {}
pub enum wasm {
    WasmMemoryMapDescriptor
}

pub mod std {
  pub mod ffi {
      pub type c_void = u8;
  }
}

pub mod cppgc {
  pub mod internal {
      pub mod gc_info {
        pub struct GCInfoIndex {}
        pub struct AtomicU16 {}
      }
  }
}

