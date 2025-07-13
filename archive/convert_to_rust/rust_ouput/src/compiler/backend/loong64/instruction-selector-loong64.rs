// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-selector-loong64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::Arc;

use crate::base::bits::{is_int12, is_int16, is_int32, is_uint12, is_uint5, is_uint6};
use crate::codegen::assembler::MacroAssemblerBase;
use crate::codegen::machine_type::{
    MachineRepresentation, RegisterRepresentation, WordRepresentation,
};
use crate::compiler::backend::instruction_selector::{FlagsContinuationT, InstructionSelectorT};
use crate::compiler::backend::loong64::instruction_codes_loong64::AddressingMode;
use crate::compiler::backend::mips64::code_generator_mips64::WriteBarrierKind;
use crate::compiler::backend::mips64::instruction_selector_mips64::StackCheckKind;
use crate::compiler::backend::s390::instruction_selector_s390::{
    AtomicStoreParameters, AtomicWidth,
};
use crate::compiler::machine_operator::{AtomicMemoryOrder, MemoryAccessKind};
use crate::compiler::turboshaft::{
    OpIndex, Operator, ShiftOp, StoreView, TurboshaftGraph, WriteBarrierKindToRecordWriteMode,
};
use crate::execution::isolate::RootsTable;
use crate::objects::heap_object::HeapObject;
use crate::roots::root_list::RootIndex;
use crate::wasm::wasm_memory::MemoryAccessKind::kMemoryAccessProtectedMemOutOfBounds;
use crate::wasm::wasm_memory::MemoryAccessKind::kMemoryAccessProtectedNullDereference;
use v8::Local;
use InstructionCode::*;

pub mod turboshaft {
    use crate::wasm::wasm_memory::MemoryAccessKind::kMemoryAccessProtectedMemOutOfBounds;
    use crate::wasm::wasm_memory::MemoryAccessKind::kMemoryAccessProtectedNullDereference;
    use crate::objects::heap_object::HeapObject;
    use crate::roots::root_list::RootIndex;
    use crate::codegen::assembler::MacroAssemblerBase;
    use crate::compiler::backend::loong64::instruction_codes_loong64::AddressingMode;
    use crate::compiler::backend::mips64::code_generator_mips64::WriteBarrierKind;
    use crate::compiler::backend::mips64::instruction_selector_mips64::StackCheckKind;
    use crate::compiler::backend::s390::instruction_selector_s390::{AtomicWidth, AtomicStoreParameters};
    use crate::compiler::machine_operator::{AtomicMemoryOrder, MemoryAccessKind};
    use crate::compiler::turboshaft::{OpIndex, Operator, TurboshaftGraph};
    use crate::compiler::turboshaft;
    use crate::compiler::machine_operator;
    use std::any::Any;
    use std::marker::PhantomData;

    #[derive(Debug)]
    pub struct Opmask {}
    impl Opmask {
        pub struct kTruncateFloat64ToInt64OverflowToMin {}
        pub struct kTruncateFloat32ToInt32OverflowToMin {}
        pub struct kTruncateFloat32ToUint32OverflowToMin {}

        pub struct kExternalConstant {}
        pub struct kWord64BitwiseAnd {}
        pub struct kWord32BitwiseAnd {}

        pub struct kWord32Equal {}

        pub struct kWord64ShiftRightArithmetic {}

        pub struct kChangeInt32ToInt64 {}
        pub struct kChangeUint32ToUint64 {}

        pub struct kWord32Constant {}

        pub struct kLoadRootRegister {}

        pub struct kTruncateInt64ToInt32 {}

        pub struct kFloat64Ieee754Binop {}
    }

    #[derive(Debug)]
    pub struct Operation {
        pub opcode: Opcode,
        pub inputs: Vec<OpIndex>,
        pub any: Option<Box<dyn Any>>,
    }

    impl Operation {
        pub fn input(&self, index: usize) -> OpIndex {
            self.inputs[index]
        }

        pub fn Is<T>(&self) -> bool where T: Sized {
            match &self.any {
                Some(data) => data.is::<T>(),
                None => false,
            }
        }

        pub fn TryCast<T: 'static>(&self) -> Option<&T> {
            self.any.as_ref().and_then(|a| a.downcast_ref::<T>())
        }

        pub fn Cast<T: 'static>(&self) -> &T {
             self.TryCast::<T>().unwrap()
        }
    }

    pub struct TurboshaftAdapter {}
    impl TurboshaftAdapter {
        pub struct StoreView {
            displacement: i32,
            base: OpIndex,
            index: OpIndex,
            value: OpIndex,
            rep: StoredRepresentation,
            tag: IndirectPointerTag,
            is_store_trap_on_null: bool,
            access_kind: MemoryAccessKind,
        }

        impl StoreView {
            pub fn displacement(&self) -> i32 {
                self.displacement
            }
            pub fn base(&self) -> OpIndex {
                self.base
            }
            pub fn index(&self) -> OpIndex {
                self.index
            }
            pub fn value(&self) -> OpIndex {
                self.value
            }

            pub fn indirect_pointer_tag(&self) -> IndirectPointerTag {
                self.tag
            }

            pub fn ts_stored_rep(&self) -> StoredRepresentation {
                self.rep
            }

            pub fn is_store_trap_on_null(&self) -> bool {
                self.is_store_trap_on_null
            }

             pub fn access_kind(&self) -> MemoryAccessKind {
                self.access_kind
            }
        }

        pub struct LoadView {
            loaded_rep: LoadRepresentation,
            result_rep: RegisterRepresentation,
        }

          impl LoadView {
            pub fn loaded_rep(&self) -> LoadRepresentation {
                self.loaded_rep
            }
            pub fn ts_loaded_rep(&self) -> LoadRepresentation {
                self.loaded_rep
            }
            pub fn result_rep(&self) -> RegisterRepresentation {
                self.result_rep
            }
            pub fn ts_result_rep(&self) -> RegisterRepresentation {
                self.result_rep
            }
        }
    }

    #[derive(Debug)]
    pub struct StackPointerGreaterThanOp {
        pub kind: StackCheckKind,
        pub stack_limit: OpIndex
    }

    #[derive(Debug)]
    pub struct ComparisonOp {
        pub rep: RegisterRepresentation,
        pub left: OpIndex,
        pub right: OpIndex,
        pub kind: ComparisonOpKind
    }

    impl ComparisonOp {
        pub fn left(&self) -> OpIndex {
            self.left
        }
        pub fn right(&self) -> OpIndex {
            self.right
        }
    }

    #[derive(Debug)]
    pub struct ProjectionOp {
       pub index: usize,
        pub input: OpIndex
    }
    impl ProjectionOp {
        pub fn input(&self) -> OpIndex {
            self.input
        }
    }

    #[derive(Debug)]
    pub struct OverflowCheckedBinopOp {
        pub rep: WordRepresentation,
        pub kind: OverflowCheckedBinopOpKind
    }

    #[derive(Debug)]
    pub struct ShiftOp {
        pub kind: ShiftOpKind,
        pub left: OpIndex,
        pub right: OpIndex,
    }

    impl ShiftOp {
        pub fn left(&self) -> OpIndex {
            self.left
        }
        pub fn right(&self) -> OpIndex {
            self.right
        }
    }

    #[derive(Debug)]
    pub struct LoadOp {
        pub offset: i32,
        pub base: OpIndex,
        pub index: Option<OpIndex>,
        pub element_size_log2: i32,
    }

    impl LoadOp {
        pub fn base(&self) -> OpIndex {
            self.base
        }
        pub fn index(&self) -> Option<OpIndex> {
            self.index
        }
    }
    #[derive(Debug)]
    pub struct ConstantOp {
        pub kind: ConstantOpKind,
        pub handle: Option<HeapObject>,
        pub integral: i64,
        pub float32: f32,
        pub float64: f64,
    }

    impl ConstantOp {
        pub fn signed_integral(&self) -> i64 {
            self.integral
        }

        pub fn handle(&self) -> &HeapObject {
            self.handle.as_ref().unwrap()
        }

        pub fn IsIntegral(&self) -> bool {
            match self.kind {
                ConstantOpKind::kInteger => true,
                _ => false,
            }
        }

         pub fn float32(&self) -> f32 {
            self.float32
        }
         pub fn float64(&self) -> f64 {
            self.float64
        }
    }

    #[derive(Debug)]
    pub struct AtomicRMWOp {
        pub memory_access_kind: MemoryAccessKind,
        pub memory_rep: MemoryRepresentation,
        pub memory_order: AtomicMemoryOrder,
        pub base: OpIndex,
        pub index: OpIndex,
        pub value: OpIndex,
        pub expected: OpIndex,
    }

    #[derive(Debug)]
    pub enum Opcode {
        kDe
