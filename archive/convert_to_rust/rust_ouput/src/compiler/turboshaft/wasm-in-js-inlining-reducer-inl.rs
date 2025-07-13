// Converted from V8 C++ source files:
// Header: wasm-in-js-inlining-reducer-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::rc::Rc;

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/js-inlining.h
struct JSInliner {}

impl JSInliner {
    fn WasmFunctionNameForTrace(_native_module: *mut NativeModule, _func_idx: u32) -> String {
        "WasmFunctionNameForTrace".to_string()
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/assembler.h
struct Assembler<T> {
    phase_zone_: Zone,
    reducer_list: T,
}

impl<T> Assembler<T> {
    fn phase_zone(&self) -> &Zone {
        &self.phase_zone_
    }
    fn new_block(&mut self) -> Block {
        Block {}
    }
    fn goto(&mut self, _block: &Block) {}
    fn bind(&mut self, _block: &Block) {}
    fn data(&self) -> AssemblerData {
        AssemblerData {}
    }
    fn new<U>(&self) -> *mut U {
        0 as *mut U
    }
    fn load_root_register(&mut self) -> OpIndex {
        OpIndex {}
    }
    fn load(&mut self, _root: OpIndex, _kind: LoadOpKind, _mem_rep: MemoryRepresentation, _offset: usize) -> V<WordPtr> {
        V {}
    }
    fn store(&mut self, _address: V<WordPtr>, _value: V<Word32>, _kind: LoadOpKind, _mem_rep: MemoryRepresentation, _write_barrier: compiler::kNoWriteBarrier) {}
    fn word32_constant(&mut self, _value: i32) -> V<Word32> {
        V {}
    }
    fn heap_constant(&mut self, _value: *mut Value) -> V<Any> {
        V {}
    }
    fn null(&mut self, _type: ValueType) -> V<Any> {
        V {}
    }
    fn simd128_constant(&mut self, _value: [u8; 16]) -> V<Any> {
        V {}
    }
}

struct AssemblerData {}

impl AssemblerData {
    fn isolate(&self) -> *mut Isolate {
        0 as *mut Isolate
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
struct LoadOpKind {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/index.h
#[derive(Debug, Clone, Copy)]
struct Index {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct Operation {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/factory-inl.h
struct Factory {}

impl Factory {
    fn undefined_value(&self) -> *mut Value {
        0 as *mut Value
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/instance-type-inl.h
struct InstanceType {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/compilation-environment-inl.h
struct CompilationEnv {}

impl CompilationEnv {
    fn ForModule(_native_module: *mut NativeModule) -> CompilationEnv {
        CompilationEnv {}
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/decoder.h
struct Decoder {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/function-body-decoder-impl.h
struct FunctionBodyDecoderImpl {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-engine.h
struct WasmEngine {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-module.h
struct WasmModule {
    functions: Vec<WasmFunction>,
    num_imported_functions: u32,
    num_declared_functions: u32,
    types: Vec<WasmType>,
}

impl WasmModule {
    fn type_(&self, sig_index: u32) -> &WasmType {
        &self.types[sig_index as usize]
    }
    fn function_was_validated(&self, _func_idx: u32) -> bool {
        true
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-objects.h
struct WasmObjects {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-opcodes-inl.h
struct WasmOpcodesInl {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-subtyping.h
struct WasmSubtyping {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/js-inlining.h
struct OpEffects {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/index.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BlockIndex {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/execution/isolate.h
struct Isolate {
    factory_: Factory,
}

impl Isolate {
    fn factory(&self) -> &Factory {
        &self.factory_
    }
    fn thread_in_wasm_flag_address_offset() -> usize {
        0
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-module.h
struct WasmFunction {
    sig: *const wasm::FunctionSig,
    code: WasmCode,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-module.h
struct WasmCode {
    offset_: u32,
    end_offset_: u32,
}

impl WasmCode {
    fn offset(&self) -> u32 {
        self.offset_
    }
    fn end_offset(&self) -> u32 {
        self.end_offset_
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-module.h
struct WasmType {
    is_shared: bool,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/index.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OpIndex {}

impl OpIndex {
    fn invalid() -> OpIndex {
        OpIndex {}
    }
    fn valid(&self) -> bool {
        true
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct CallTarget {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct FrameState {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/wasm-assembler-helpers.h
struct WasmAssemblerHelpers {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/execution/isolate.h
struct IsolateRoot {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/base/vector.h
struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    fn begin(&self) -> *const T {
        self.data.as_ptr()
    }
    fn sub_vector_from(&self, start: usize) -> Vector<T> where T: Copy {
        Vector {
            data: self.data[start..].to_vec(),
        }
    }

    fn sub_vector_from_to(&self, start: usize, end: usize) -> Vector<T> where T: Copy {
        Vector {
            data: self.data[start..end].to_vec(),
        }
    }
}

impl<T: Copy> Vector<&T> {
    fn sub_vector_from(&self, start: usize) -> Vector<&T> {
        Vector {
            data: self.data[start..].to_vec(),
        }
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct TSCallDescriptor {
    js_wasm_call_parameters: Option<JSWasmCallParameters>,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct V<T> {}

impl<T> V<T> {
    fn valid(&self) -> bool {
        true
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct Any {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct Word32 {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct WordPtr {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct WasmTrustedInstanceData {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct Word64 {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct Float32 {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
struct Float64 {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-module.h
mod wasm {
    pub struct FunctionSig {
        pub return_count_: usize,
    }
    impl FunctionSig {
        pub fn return_count(&self) -> usize {
            self.return_count_
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum ValueType {
        I32,
        I64,
        F32,
        F64,
        RefNull,
        S128,
        Void,
        Ref,
        Bottom,
        Top,
        I8,
        I16,
    }

    impl ValueType {
        pub fn kind(&self) -> Self {
            *self
        }
        pub fn is_defaultable(&self) -> bool {
            match self {
                ValueType::RefNull => true,
                ValueType::I32 | ValueType::I64 | ValueType::F32 | ValueType::F64 | ValueType::S128 => true,
                ValueType::Ref => true,
                _ => false,
            }
        }
        pub fn is_reference(&self) -> bool {
            match self {
                ValueType::RefNull | ValueType::Ref => true,
                _ => false,
            }
        }
    }
    pub struct ArrayIndexImmediate {}
    pub struct BranchTableImmediate {}
    pub struct CallFunctionImmediate {}
    pub struct CallIndirectImmediate {}
    pub struct FieldImmediate {}
    pub struct GlobalIndexImmediate {
        pub index: u32,
        pub global: OpIndex,
    }
    pub struct IndexImmediate {
        pub index: u32,
    }
    pub struct MemoryAccessImmediate {}
    pub struct MemoryCopyImmediate {}
    pub struct MemoryIndexImmediate {}
    pub struct MemoryInitImmediate {}
    pub struct Simd128Immediate {}
    pub struct SimdLaneImmediate {}
    pub struct StringConstImmediate {}
    pub struct StructIndexImmediate {}
    pub struct TableCopyImmediate {}
    pub struct TableIndexImmediate {}
    pub struct TableInitImmediate {}
    pub struct TagIndexImmediate {}
    pub type WasmOpcode = u32;
    pub const kExprI32Eqz: WasmOpcode = 0x45;
    pub const kExprF32Abs: WasmOpcode = 0xA7;
    pub const kExprF32Neg: WasmOpcode = 0xA8;
    pub const kExprF32Sqrt: WasmOpcode = 0xA9;
    pub const kExprF64Abs: WasmOpcode = 0xB4;
    pub const kExprF64Neg: WasmOpcode = 0xB5;
    pub const kExprF64Sqrt: WasmOpcode = 0xB6;
    pub const kExprF64SConvertI32: WasmOpcode = 0xBF;
    pub const kExprF64UConvertI32: WasmOpcode = 0xC0;
    pub const kExprF32SConvertI32: WasmOpcode = 0xBD;
    pub const kExprF32UConvertI32: WasmOpcode = 0xBE;
    pub const kExprF32ConvertF64: WasmOpcode = 0xB9;
    pub const kExprF64ConvertF32: WasmOpcode = 0xB8;
    pub const kExprF32ReinterpretI32: WasmOpcode = 0xC4;
    pub const kExprI32ReinterpretF32: WasmOpcode = 0xC3;
    pub const kExprI32Clz: WasmOpcode = 0x67;
    pub const kExprI32SExtendI8: WasmOpcode = 0x91;
    pub const kExprI32SExtendI16: WasmOpcode = 0x92;
    pub const kExprRefIsNull: WasmOpcode = 0xD0;
    pub const kExprRefAsNonNull: WasmOpcode = 0xD1;
    pub const kExprI32ConvertI64: WasmOpcode = 0x6F;
    pub const kExprI64SConvertI32: WasmOpcode = 0x70;
    pub const kExprI64UConvertI32: WasmOpcode = 0x71;
    pub const kExprF64ReinterpretI64: WasmOpcode = 0xC6;
    pub const kExprI64ReinterpretF64: WasmOpcode = 0xC5;
    pub const kExprI64Clz: WasmOpcode = 0x79;
    pub const kExprI64Eqz: WasmOpcode = 0x46;
    pub const kExprI64SExtendI8: WasmOpcode = 0x93;
    pub const kExprI64SExtendI16: WasmOpcode = 0x94;
    pub const kExprI64SExtendI32: WasmOpcode = 0x95;
    pub const kExprI32SConvertF32: WasmOpcode = 0xC1;
    pub const kExprI32UConvertF32: WasmOpcode = 0xC2;
    pub const kExprI32SConvertF64: WasmOpcode = 0xBD;
    pub const kExprI32UConvertF64: WasmOpcode = 0xBE;
    pub const kExprI64SConvertF32: WasmOpcode = 0xC7;
    pub const kExprI64UConvertF32: WasmOpcode = 0xC8;
    pub const kExprI64SConvertF64: WasmOpcode = 0xC9;
    pub const kExprI64UConvertF64: WasmOpcode = 0xCA;
    pub const kExprI32SConvertSatF32: WasmOpcode = 0xBF;
    pub const kExprI32UConvertSatF32: WasmOpcode = 0xC0;
    pub const kExprI32SConvertSatF64: WasmOpcode = 0xBD;
    pub const kExprI32UConvertSatF64: WasmOpcode = 0xBE;
    pub const kExprI64SConvertSatF32: WasmOpcode = 0xC1;
    pub const kExprI64UConvertSatF32: WasmOpcode = 0xC2;
    pub const kExprI64SConvertSatF64: WasmOpcode = 0xC3;
    pub const kExprI64UConvertSatF64: WasmOpcode = 0xC4;
    pub const kExprI32Ctz: WasmOpcode = 0x68;
    pub const kExprI32Popcnt: WasmOpcode = 0x69;
    pub const kExprF32Floor: WasmOpcode = 0xAA;
    pub const kExprF32Ceil: WasmOpcode = 0xAB;
    pub const kExprF32Trunc: WasmOpcode = 0xAC;
    pub const kExprF32NearestInt: WasmOpcode = 0xAD;
    pub const kExprF64Floor: WasmOpcode = 0xB7;
    pub const kExprF64Ceil: WasmOpcode = 0xB8;
    pub const kExprF64Trunc: WasmOpcode = 0xB9;
    pub const kExprF64NearestInt: WasmOpcode = 0xBA;
    pub const kExprI64Ctz: WasmOpcode = 0x7A;
    pub const kExprI64Popcnt: WasmOpcode = 0x7B;
    pub const kExprF32SConvertI64: WasmOpcode = 0xBE;
    pub const kExprF32UConvertI64: WasmOpcode = 0xBF;
    pub const kExprF64SConvertI64: WasmOpcode = 0xC0;
    pub const kExprF64UConvertI64: WasmOpcode = 0xC1;
    pub const kExprAnyConvertExtern: WasmOpcode = 0xFC;
    pub const kExprExternConvertAny: WasmOpcode = 0xFD;
    pub const kExprF64Acos: WasmOpcode = 0x99;
    pub const kExprF64Asin: WasmOpcode = 0x9A;
    pub const kExprF64Atan: WasmOpcode = 0x9B;
    pub const kExprF64Cos: WasmOpcode = 0x9C;
    pub const kExprF64Sin: WasmOpcode = 0x9D;
    pub const kExprF64Tan: WasmOpcode = 0x9E;
    pub const kExprF64Exp: WasmOpcode = 0x9F;
    pub const kExprF64Log: WasmOpcode = 0xA0;
    pub const kExprI32AsmjsLoadMem8S: WasmOpcode = 0x00;
    pub const kExprI32AsmjsLoadMem8U: WasmOpcode = 0x00;
    pub const kExprI32AsmjsLoadMem16S: WasmOpcode = 0x00;
    pub const kExprI32AsmjsLoadMem16U: WasmOpcode = 0x00;
    pub const kExprI32AsmjsLoadMem: WasmOpcode = 0x00;
    pub const kExprF32AsmjsLoadMem: WasmOpcode = 0x00;
    pub const kExprF64AsmjsLoadMem: WasmOpcode = 0x00;
    pub const kExprI32AsmjsSConvertF32: WasmOpcode = 0x00;
    pub const kExprI32AsmjsUConvertF32: WasmOpcode = 0x00;
    pub const kExprI32AsmjsSConvertF64: WasmOpcode = 0x00;
    pub const kExprI32AsmjsUConvertF64: WasmOpcode = 0x00;

    pub const kExprI32Add: WasmOpcode = 0x6A;
    pub const kExprI32Sub: WasmOpcode = 0x6B;
    pub const kExprI32Mul: WasmOpcode = 0x6C;
    pub const kExprI32And: WasmOpcode = 0x71;
    pub const kExprI32Ior: WasmOpcode = 0x72;
    pub const kExprI32Xor: WasmOpcode = 0x73;
    pub const kExprI32Shl: WasmOpcode = 0x74;
    pub const kExprI32ShrS: WasmOpcode = 0x75;
    pub const kExprI32ShrU: WasmOpcode = 0x76;
    pub const kExprI32Ror: WasmOpcode = 0x78;
    pub const kExprI32Rol: WasmOpcode = 0x77;
    pub const kExprI32Eq: WasmOpcode = 0x46;
    pub const kExprI32Ne: WasmOpcode = 0x47;
    pub const kExprI32LtS: WasmOpcode = 0x48;
    pub const kExprI32LeS: WasmOpcode = 0x49;
    pub const kExprI32LtU: WasmOpcode = 0x4A;
    pub const kExprI32LeU: WasmOpcode = 0x4B;
    pub const kExprI32GtS: WasmOpcode = 0x4C;
    pub const kExprI32GeS: WasmOpcode = 0x4D;
    pub const kExprI32GtU: WasmOpcode = 0x4E;
    pub const kExprI32GeU: WasmOpcode = 0x4F;
    pub const kExprF32CopySign: WasmOpcode = 0xBC;
    pub const kExprF32Add: WasmOpcode = 0x92;
    pub const kExprF32Sub: WasmOpcode = 0x93;
    pub const kExprF32Mul: WasmOpcode = 0x94;
    pub const kExprF32Div: WasmOpcode = 0x95;
    pub const kExprF32Eq: WasmOpcode = 0x5B;
    pub const kExprF32Ne: WasmOpcode = 0x5C;
    pub const kExprF32Lt: WasmOpcode = 0x5D;
    pub const kExprF32Le: WasmOpcode = 0x5E;
    pub const kExprF32Gt: WasmOpcode = 0x5F;
    pub const kExprF32Ge: WasmOpcode = 0x60;
    pub const kExprF32Min: WasmOpcode = 0x96;
    pub const kExprF32Max: WasmOpcode = 0x97;
    pub const kExprF64Add: WasmOpcode = 0xA0;
    pub const kExprF64Sub: WasmOpcode = 0xA1;
    pub const kExprF64Mul: WasmOpcode = 0xA2;
    pub const kExprF64Div: WasmOpcode = 0xA3;
    pub const kExprF64Eq: WasmOpcode = 0x61;
    pub const kExprF64Ne: WasmOpcode = 0x62;
    pub const kExprF64Lt: WasmOpcode = 0x63;
    pub const kExprF64Le: WasmOpcode = 0x64;
    pub const kExprF64Gt: WasmOpcode = 0x65;
    pub const kExprF64Ge: WasmOpcode = 0x66;
    pub const kExprF64Min: WasmOpcode = 0xA4;
    pub const kExprF64Max: WasmOpcode = 0xA5;
    pub const kExprRefEq: WasmOpcode = 0xD2;
    pub const kExprI64Add: WasmOpcode = 0x7C;
    pub const kExprI64Sub: WasmOpcode = 0x7D;
    pub const kExprI64Mul: WasmOpcode = 0x7E;
    pub const kExprI64And: WasmOpcode = 0x83;
    pub const kExprI64Ior: WasmOpcode = 0x84;
    pub const kExprI64Xor: WasmOpcode = 0x85;
    pub const kExprI64Shl: WasmOpcode = 0x86;
    pub const kExprI64ShrS: WasmOpcode = 0x87;
    pub const kExprI64ShrU: WasmOpcode = 0x88;
    pub const kExprI64Ror: WasmOpcode = 0x8A;
    pub const kExprI64Rol: WasmOpcode = 0x89;
    pub const kExprI64Eq: WasmOpcode = 0x51;
    pub const kExprI64Ne: WasmOpcode = 0x52;
    pub const kExprI64LtS: WasmOpcode = 0x53;
    pub const kExprI64LeS: WasmOpcode = 0x54;
    pub const kExprI64LtU: WasmOpcode = 0x55;
    pub const kExprI64LeU: WasmOpcode = 0x56;
    pub const kExprI64GtS: WasmOpcode = 0x57;
    pub const kExprI64GeS: WasmOpcode = 0x58;
    pub const kExprI64GtU: WasmOpcode = 0x59;
    pub const kExprI64GeU: WasmOpcode = 0x5A;
    pub const kExprF64CopySign: WasmOpcode = 0xB3;
    pub const kExprI32DivS: WasmOpcode = 0x6D;
    pub const kExprI32DivU: WasmOpcode = 0x6E;
    pub const kExprI32RemS: WasmOpcode = 0x6F;
    pub const kExprI32RemU: WasmOpcode = 0x70;
    pub const kExprI64DivS: WasmOpcode = 0x7F;
    pub const kExprI64DivU: WasmOpcode = 0x80;
    pub const kExprI64RemS: WasmOpcode = 0x81;
    pub const kExprI64RemU: WasmOpcode = 0x82;
    pub const kExprF64Atan2: WasmOpcode = 0xA6;
    pub const kExprF64Pow: WasmOpcode = 0xA7;
    pub const kExprF64Mod: WasmOpcode = 0xA8;
    pub const kExprI32AsmjsDivS: WasmOpcode = 0x00;
    pub const kExprI32AsmjsDivU: WasmOpcode = 0x00;
    pub const kExprI32AsmjsRemS: WasmOpcode = 0x00;
    pub const kExprI32AsmjsRemU: WasmOpcode = 0x00;
    pub const kExprI32AsmjsStoreMem8: WasmOpcode = 0x00;
    pub const kExprI32AsmjsStoreMem16: WasmOpcode = 0x00;
    pub const kExprI32AsmjsStoreMem: WasmOpcode = 0x00;
    pub const kExprF32AsmjsStoreMem: WasmOpcode = 0x00;
    pub const kExprF64AsmjsStoreMem: WasmOpcode = 0x00;
    pub const kSimd128Size: usize = 16;

    pub enum TrapReason {}
    pub type ModuleTypeIndex = u32;
    #[derive(Clone, Copy)]
    pub enum HeapType {}
    pub enum LoadType {}
    pub enum LoadTransformationKind {}
    pub enum StoreType {}
    pub struct CatchCase {}
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
#[derive(Clone, Copy)]
struct OptionalV<T> {
    value: Option<T>,
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/zone/zone.h
struct Zone {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/loop-finder.h
struct Block {}

// From /home/kathirks_gc/v8_go/archive/codebase/src/wasm/wasm-module.h
fn is_asmjs_module(_module: *const WasmModule) -> bool {
    false
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/define-assembler-macros.inc
macro_rules! trace {
    ($($arg:tt)*) => {
        if v8_flags.trace_turbo_inlining {
            println!($($arg)*);
        }
    }
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
mod compiler {
    pub struct kNoWriteBarrier;
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/compiler/turboshaft/operations.h
mod base {
    pub struct Vector<T> {
        pub data: Vec<T>
    }
}

struct NativeModule {

}

impl NativeModule {
    fn wire_bytes(&mut self) -> Vector<u8> {
        Vector {
            data: Vec::new()
        }
    }
}

struct JSWasmCallParameters {
    module_: *const WasmModule,
    native_module_: *mut NativeModule,
    function_index_: u32,
}

impl JSWasmCallParameters {
    fn module(&self) -> *const WasmModule {
        self.module_
    }
    fn native_module(&self) -> *mut NativeModule {
        self.native_module_
    }
    fn function_index(&self) -> u32 {
        self.function_index_
    }
}

const KWASM_INSTANCE_DATA_PARAMETER_INDEX: usize = 0;

struct V8Flags {
    trace_turbo_inlining: bool,
    turboshaft_wasm_in_js_inlining: bool,
}

static v8_flags: V8Flags = V8Flags {
    trace_turbo_inlining: false,
    turboshaft_wasm_in_js_inlining: true,
};

trait ReducerList {}

struct WasmInJSInliningReducer<Next: ReducerList> {
    next: Next,
}

impl<Next: ReducerList> WasmInJSInliningReducer<Next> {
    fn new(next: Next) -> Self {
        WasmInJSInliningReducer { next }
    }

    fn reduce_call(&mut self, callee: V<CallTarget>, frame_state: OptionalV<FrameState>, arguments: Vector<OpIndex>, descriptor: &TSCallDescriptor, effects: OpEffects) -> V<Any> {
        if !descriptor.js_wasm_call_parameters.is_some() {
            return self.next.reduce_call(callee, frame_state, arguments, descriptor, effects);
        }

        if !v8_flags.turboshaft_wasm_in_js_inlining {
            panic!("We shouldn't have attached `JSWasmCallParameters` at this call, unless we have TS Wasm-in-JS inlining enabled.");
        }

        let module = unsafe { descriptor.js_wasm_call_parameters.as_ref().unwrap().module() };
        let native_module = unsafe { descriptor.js_wasm_call_parameters.as_ref().unwrap().native_module() };
        let func_idx = unsafe { descriptor.js_wasm_call_
