#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(clippy::missing_safety_doc)]

//use std::arch::asm;
//use std::convert::TryInto;
//use std::mem::size_of;
//use std::ptr::null_mut;
//use std::sync::atomic::{AtomicBool, Ordering};
//use std::sync::{Arc, Mutex};

//use crate::base::Flags;
//use crate::codegen::{Assembler, Register, CPURegList};
//use crate::codegen::register::RegList;
//use crate::codegen::macro_assembler::MacroAssembler;
//use crate::execution::isolate::Isolate;
//use crate::execution::frame_constants::StackFrame;
//use crate::execution::frame_constants::WasmToJSInterpreterFrameConstants;
//use crate::execution::frame_constants::BuiltinWasmInterpreterWrapperConstants;
//use crate::objects::js_array::JSArray;
//use crate::objects::fixed_array::FixedArray;
//use crate::objects::shared_function_info::SharedFunctionInfo;
//use crate::objects::js_function::JSFunction;
//use crate::runtime::wasm::WasmInterpreterRuntime;
//use crate::wasm::wasm_objects::WasmInstanceObject;
//use crate::wasm::wasm_objects::WasmExportedFunctionData;
//use crate::wasm::wasm_objects::WasmTrustedInstanceData;
//use crate::wasm::value_type::ValueType;
//use crate::wasm::value_type::ValueKind;
//use crate::wasm::function_sig::FunctionSig;

macro_rules! offset_of {
    ($struct:path, $field:tt) => {{
        //use std::mem::MaybeUninit;
        //use std::ptr;

        //let uninit = MaybeUninit::<$struct>::uninit();
        //let ptr = uninit.as_ptr();
        //let field_ptr = unsafe { ptr::addr_of!((*ptr).$field) };

        //unsafe { field_ptr.offset_from(ptr as *const _) as usize }
        0 // placeholder to allow compilation
    }};
}

//mod codegen {
//    pub mod assembler;
//    pub mod register;
//    pub mod macro_assembler;
//}

//mod execution {
//    pub mod isolate;
//    pub mod frame_constants;
//}

//mod objects {
//    pub mod js_array;
//    pub mod fixed_array;
//    pub mod shared_function_info;
//    pub mod js_function;
//}

//mod runtime {
//    pub mod wasm {
//        pub mod wasm_interpreter_runtime;
//    }
//}

//mod wasm {
//    pub mod wasm_objects;
//    pub mod value_type;
//    pub mod function_sig;
//}

const V8_ENABLE_WEBASSEMBLY: bool = true;
const kSystemPointerSize: usize = 8;
const kTaggedSizeLog2: usize = 3;
const kHeapObjectTag: usize = 1;
const kSystemPointerSizeLog2: usize = 3;
const kFPOnStackSize: usize = 8;
const kPCOnStackSize: usize = 8;
const SmiValuesAre32Bits: bool = false;
const kXRegSizeInBits: usize = 64;
const kWasmTrustedInstanceDataIndirectPointerTag: usize = 1;
const kWasmValueKindBitsMask: usize = 0xFFFF;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Register {
    code: usize,
}

impl Register {
    const NoReg: Self = Self { code: 0 };

    fn X(self) -> Self {
        self
    }

    fn W(self) -> Self {
        self
    }
}

const xzr: Register = Register { code: 31 };
const lr: Register = Register { code: 30 };
const fp: Register = Register { code: 29 };
const sp: Register = Register { code: 28 };
const kRootRegister: Register = Register { code: 26 };
const kJSFunctionRegister: Register = Register { code: 1 };
const kWasmImplicitArgRegister: Register = Register { code: 7 };
const kJavaScriptCallArgCountRegister: Register = Register { code: 2 };
const kPtrComprCageBaseRegister: Register = Register { code: 13 };
const kReturnRegister0: Register = Register { code: 0 };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct DoubleRegister {
    code: usize,
}

const kFPReturnRegister0: DoubleRegister = DoubleRegister { code: 0 };

#[macro_export]
macro_rules! ACCESS_MASM {
    ($masm:expr) => {
        $masm
    };
}

macro_rules! RelocInfo {
    ($x:ident) => {
        0
    };
    (CODE_TARGET) => {
        0
    };
}

macro_rules! BUILTIN_CODE {
    ($isolate:expr, $name:ident) => {
        0
    };
}

macro_rules! Immediate {
    ($x:expr) => {
        $x
    };
}

macro_rules! Operand {
    ($reg:expr, LSL, $shift:expr) => {
        $reg
    };
}

macro_rules! MemOperand {
    ($reg:expr, $offset:expr) => {
        $reg
    };
    ($reg:expr, $index:expr, LSL, $shift:expr) => {
        $reg
    };
}

macro_rules! FieldMemOperand {
    ($reg:expr, $offset:expr) => {
        $reg
    };
}

macro_rules! ExternalReference {
    (Create($id:ident, $isolate:expr)) => {
        0
    };
}

macro_rules! IsolateAddressId {
    (kHandlerAddress) => {
        0
    };
    (kCEntryFPAddress) => {
        0
    };
}

macro_rules! Runtime {
    (kWasmRunInterpreter) => {
        0
    };
    (kWasmThrowJSTypeError) => {
        0
    };
}

macro_rules! RootIndex {
    (kUndefinedValue) => {
        0
    };
}

macro_rules! OFFSET_OF_DATA_START {
    ($struct:ident) => {
        0
    };
}

mod internal {
    use super::*;

    // Placeholder structs and enums
    struct MacroAssembler {}
    impl MacroAssembler {
        fn isolate(&self) -> &MacroAssembler {
            self
        }
        fn BindExceptionHandler(&mut self, label: &Label) {}
    }

    struct Assembler {}
    impl Assembler {
        struct BlockPoolsScope<'a>(&'a mut Assembler);
        impl<'a> BlockPoolsScope<'a> {
            fn new(assembler: &'a mut Assembler) -> Self {
                BlockPoolsScope(assembler)
            }
        }

        fn Emit(&mut self, instruction: u32) {}

        struct BlockPoolsScope2<'a>(&'a mut MacroAssembler);
        impl<'a> BlockPoolsScope2<'a> {
            fn new(assembler: &'a mut MacroAssembler) -> Self {
                BlockPoolsScope2(assembler)
            }
        }
    }
    
    #[derive(Debug, Default, Copy, Clone)]
    struct Label {
        pos: usize,
    }

    #[derive(Debug, Default, Copy, Clone)]
    struct CPURegList {
        bits: u64,
        reg_size_in_bits: usize,
    }

    impl CPURegList {
        fn new(reg_size_in_bits: usize, reg_list: RegList) -> Self {
            CPURegList {
                bits: 0,
                reg_size_in_bits,
            }
        }

        fn IsEmpty(&self) -> bool {
            self.bits == 0
        }

        fn PopLowestIndex(&mut self) -> Register {
            Register { code: 0 }
        }

        fn IncludesAliasOf(&self, reg: Register) -> bool {
            true
        }

        fn Remove(&mut self, reg: Register) {}
        fn Remove2(&mut self, reg1: Register, reg2: Register, reg3: Register, reg4: Register) {}
        fn Remove3(&mut self, reg5: Register, reg6: Register, reg7: Register) {}

        fn Combine(&mut self, reg: Register) {}

        fn set_bits(&mut self, bits: u64) {}
    }

    #[derive(Debug, Default, Copy, Clone)]
    struct RegList {}

    //Helper class for managing scratch registers
    struct UseScratchRegisterScope<'a> {
        masm: &'a mut MacroAssembler,
    }

    impl<'a> UseScratchRegisterScope<'a> {
        fn new(masm: &'a mut MacroAssembler) -> Self {
            UseScratchRegisterScope { masm }
        }

        fn AcquireX(&mut self) -> Register {
            Register { code: 0 }
        }
    }

    struct NoRootArrayScope<'a>(&'a mut MacroAssembler);

    impl<'a> NoRootArrayScope<'a> {
        fn new(masm: &'a mut MacroAssembler) -> Self {
            NoRootArrayScope(masm)
        }
    }

    impl<'a> Drop for NoRootArrayScope<'a> {
        fn drop(&mut self) {}
    }
    
    impl<'a> Drop for UseScratchRegisterScope<'a> {
        fn drop(&mut self) {}
    }

    pub struct Builtins {}

    impl Builtins {
        pub fn Generate_WasmInterpreterEntry(masm: &mut MacroAssembler) {}
        pub fn Generate_GenericJSToWasmInterpreterWrapper(masm: &mut MacroAssembler) {}
        pub fn Generate_WasmInterpreterCWasmEntry(masm: &mut MacroAssembler) {}
        pub fn Generate_GenericWasmToJSInterpreterWrapper(masm: &mut MacroAssembler) {}
    }

    fn log2(x: usize) -> usize {
        if x == 0 {
            0
        } else {
            usize::BITS as usize - x.leading_zeros() as usize - 1
        }
    }

    fn LoadFunctionDataAndWasmInstance(masm: &mut MacroAssembler, function_data: Register, wasm_instance: Register) {}
    fn LoadFromSignature(masm: &mut MacroAssembler, valuetypes_array_ptr: Register, return_count: Register, param_count: Register) {}
    fn LoadValueTypesArray(masm: &mut MacroAssembler, function_data: Register, valuetypes_array_ptr: Register, return_count: Register, param_count: Register, signature_data: Register) {}

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
            if *reg != Register::NoReg {
                panic!("Register already assigned");
            }
            if self.available_.IsEmpty() {
                panic!("No registers available");
            }
            *reg = self.available_.PopLowestIndex().X();
            self.allocated_registers_.push(*reg);
        }

        fn Pinned(&mut self, requested: Register, reg: &mut Register) {
            if !self.available_.IncludesAliasOf(requested) {
                panic!("Register not available");
            }
            *reg = requested;
            self.Reserve(requested);
            self.allocated_registers_.push(*reg);
        }

        fn Free(&mut self, reg: &mut Register) {
            if *reg == Register::NoReg {
                panic!("Register already freed");
            }
            self.available_.Combine(*reg);
            self.allocated_registers_.retain(|&x| x != *reg);
            *reg = Register::NoReg;
        }

        fn Reserve(&mut self, reg: Register) {
            if reg == Register::NoReg {
                return;
            }
            if !self.available_.IncludesAliasOf(reg) {
                panic!("Register not available");
            }
            self.available_.Remove(reg);
        }

        fn Reserve2(&mut self, reg1: Register, reg2: Register, reg3: Register, reg4: Register, reg5: Register, reg6: Register) {
            self.Reserve(reg1);
            self.Reserve(reg2);
            self.Reserve(reg3);
            self.Reserve(reg4);
            self.Reserve(reg5);
            self.Reserve(reg6);
        }

        fn IsUsed(&self, reg: Register) -> bool {
            self.initial_.IncludesAliasOf(reg) && !self.available_.IncludesAliasOf(reg)
        }

        fn ResetExcept(&mut self, reg1: Register, reg2: Register, reg3: Register, reg4: Register, reg5: Register, reg6: Register, reg7: Register) {
            self.available_ = self.initial_;
            if reg1 != Register::NoReg {
                self.available_.Remove2(reg1, reg2, reg3, reg4);
            }
            if reg5 != Register::NoReg {
                self.available_.Remove3(reg5, reg6, reg7);
            }
            self.allocated_registers_.retain(|&x| self.available_.IncludesAliasOf(x));
        }

        fn WithAllocatableGeneralRegisters() -> Self {
            let mut list = CPURegList::new(kXRegSizeInBits, RegList {});
            // Only use registers x0-x15, which are volatile (caller-saved).
            list.set_bits(0xffff);
            RegisterAllocator::new(list)
        }
    }

    impl<'a> RegisterAllocator {
        struct Scoped<'b> {
            allocator_: &'a mut RegisterAllocator,
            reg_: &'b mut Register,
        }

        impl<'b> Scoped<'b> {
            fn new(allocator: &'a mut RegisterAllocator, reg: &'b mut Register) -> Self {
                Scoped {
                    allocator_: allocator,
                    reg_: reg,
                }
            }
        }

        impl<'b> Drop for Scoped<'b> {
            fn drop(&mut self) {
                self.allocator_.Free(self.reg_);
            }
        }
    }

    impl Builtins {
        fn SetCWasmInterpreterEntryHandlerOffset(&mut self, pos: usize) {}
    }
}