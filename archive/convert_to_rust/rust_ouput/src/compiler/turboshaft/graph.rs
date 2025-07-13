// Converted from V8 C++ source files:
// Header: graph.h
// Implementation: graph.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod iterator {
        use std::iter::Iterator;

        pub struct IteratorRange<I> {
            pub begin: I,
            pub end: I,
        }

        impl<I> IteratorRange<I> {
            pub fn new(begin: I, end: I) -> Self {
                IteratorRange { begin, end }
            }
        }

        impl<I: Iterator> Iterator for IteratorRange<I> {
            type Item = I::Item;

            fn next(&mut self) -> Option<Self::Item> {
                todo!()
            }
        }

        pub struct DerefPtrIterator<T> {
            ptr: *const T,
            end: *const T,
        }

        impl<T> DerefPtrIterator<T> {
            pub fn new(ptr: *const T, end: *const T) -> Self {
                DerefPtrIterator { ptr, end }
            }
        }
    }
    pub mod small_vector {
        use std::vec::Vec;

        pub type SmallVector<T, const N: usize> = Vec<T>;
    }
    pub mod vector {
        pub type Vector<T> = Vec<T>;
    }
}

pub mod codegen {
    pub mod source_position {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct SourcePosition {}
    }
}

pub mod compiler {
    pub mod turboshaft {
        use crate::base::{
            iterator::{IteratorRange, DerefPtrIterator},
            small_vector::SmallVector,
            vector::Vector,
        };
        use crate::codegen::source_position::SourcePosition;
        use crate::compiler::turboshaft::opcodes::OpcodeIndex;
        use crate::compiler::turboshaft::sidetable::GrowingBlockSidetable;
        use crate::compiler::turboshaft::types::{Type, Kind};
        use crate::zone::zone_containers::ZoneAbslFlatHashSet;
        use std::cell::RefCell;
        use std::cmp;
        use std::iter::Iterator;
        use std::mem;
        use std::ops::{Deref, DerefMut};
        use std::rc::Rc;
        use std::vec;
        use crate::compiler::turboshaft::type_inference_reducer::OpIndex;

        pub mod operations {
            use crate::compiler::turboshaft::types::Kind;
            use crate::compiler::turboshaft::type_inference_reducer::OpIndex;
            use std::iter::Iterator;
            use std::any::Any;

            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum Opcode {
                kStart,
                kEnd,
                kPhi,
                kGoto,
                kBranch,
                kSwitch,
                kReturn,
                kLoad,
                kStore,
                kAdd,
                kSub,
                kMul,
                kDiv,
                kDead,
                kCheckException,
                kPendingLoopPhi,
                kFrameState,
                kDeoptimize,
                kConstant,
                kTuple,
                kCall,
                kStackCheck,
                kNumberConstant,
                kWord32Constant,
                kWord64Constant,
                kWordPtrConstant,
                kSimd128Constant,
                kFloat32Constant,
                kFloat64Constant,
                kBigIntConstant,
                kExternalConstant,
                kBitcastWordToTagged,
                kBitcastTaggedToWord,
                kInt32Add,
                kInt32Sub,
                kInt32Mul,
                kInt32Div,
                kInt64Add,
                kInt64Sub,
                kInt64Mul,
                kInt64Div,
                kFloat32Add,
                kFloat32Sub,
                kFloat32Mul,
                kFloat32Div,
                kFloat64Add,
                kFloat64Sub,
                kFloat64Mul,
                kFloat64Div,
                kCompareSigned,
                kCompareUnsigned,
                kCompareFloat,
                kProjection,
                kSelect,
                kPoisoned,
                kUnreachable,
                kAssertType,
                kReWriteOnceValue,
                kArgumentsObject,
                kTypeOf,
                kAllocate,
                kReifyStaticContext,
                kLoadContext,
                kStoreContext,
                kCallRuntime,
                kCallJSFunction,
                kCallBuiltin,
                kCallWasmFunction,
                kCallCode,
                kOsrValue,
                kCheckMap,
                kCheckValue,
                kCheckNotTaggedHole,
                kCheckNumber,
                kCheckHeapObject,
                kObjectIsSmi,
                kObjectIsHeapObject,
                kSmiUntag,
                kMaybeGrowElements,
                kIncrementFeedbackVector,
                kUpdateFeedback,
                kCollectCounter,
                kBytecodeSourceInfo,
                kNewConsTrivial,
                kCreateArrayLiteral,
                kCreateObjectLiteral,
                kCreateRegExpLiteral,
                kInvokeIntrinsic,
                kClassFieldsInitializer,
                kDebugBreak,
                kStackArgument,
                kReferenceEqual,
                kDebugAbort,
                kChangeBitToTagged,
                kSameValue,
                kSameValueNumbersOnly,
                kToBoolean,
                kToName,
                kToObject,
                kFromNumber,
                kCheckTagged,
                kAllocateInYoungGeneration,
                kNewBox,
                kLoadBox,
                kStoreBox,
                kTransitionElementsKind,
                kLoadField,
                kStoreField,
                kNumberToString,
                kStringCharCodeAt,
                kStringCodePointAt,
                kStringLength,
                kStringFromCharCode,
                kStringSubstring,
                kStringEquals,
                kStringToString,
                kStringConcat,
                kStringReplaceOneCharWithString,
                kStringReplaceRegExpWithString,
                kRegExpExec,
                kArrayShift,
                kArrayUnshift,
                kArrayPush,
                kArrayPop,
                kArraySlice,
                kArraySplice,
                kArrayIndexOf,
                kArrayIncludes,
                kArrayJoin,
                kArrayIsArray,
                kForInPrepare,
                kForInDone,
                kForInNext,
                kForInStep,
                kMapIterator,
                kSetIterator,
                kWeakMapGet,
                kWeakSetHas,
                kMapHas,
                kSetHas,
                kMapGet,
                kSetGet,
                kMapSet,
                kSetAdd,
                kMapDelete,
                kSetDelete,
                kTypedArrayGetLength,
                kTypedArraySetLength,
                kTypedArrayGetBuffer,
                kDataViewGetInt8,
                kDataViewGetUint8,
                kDataViewSetInt8,
                kDataViewSetUint8,
                kToBigInt,
                kBigIntToWord64,
                kBigIntAdd,
                kBigIntSub,
                kBigIntMul,
                kBigIntDiv,
                kBigIntMod,
                kBigIntBitAnd,
                kBigIntBitOr,
                kBigIntBitXor,
                kBigIntShiftLeft,
                kBigIntShiftRight,
                kBigIntNegate,
                kBigIntAbsolute,
                kBigIntCompare,
                kCallWithArguments,
                kCallWithArrayLikeThis,
                kConstruct,
                kHasInPrototypeChain,
                kLoadModuleVariable,
                kStoreModuleVariable,
                kCallIterator,
                kIsUn WindSafe,
            }

            pub trait OperationTrait {
                fn opcode(&self) -> Opcode;
                fn inputs(&self) -> &[OpIndex];
                fn saturated_use_count(&self) -> &UseCounter;
            }
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
            pub enum Rep {
                kNone,
                kTagged,
                kWord8,
                kWord16,
                kWord32,
                kWord64,
                kWordPtr,
                kSimd128,
                kFloat32,
                kFloat64,
                kBigInt,
                kExternal,
                kHeapObject,
                kSmi,
                kUninitialized,
            }
            #[derive(Debug, Clone)]
            pub struct UseCounter {
                count: std::cell::RefCell<usize>,
            }

            impl UseCounter {
                pub fn new() -> Self {
                    UseCounter {
                        count: std::cell::RefCell::new(0),
                    }
                }

                pub fn Incr(&self) {
                    *self.count.borrow_mut() += 1;
                }

                pub fn Decr(&self) {
                    *self.count.borrow_mut() -= 1;
                }

                pub fn count(&self) -> usize {
                    *self.count.borrow()
                }
            }
            impl Default for UseCounter {
                fn default() -> Self {
                    Self::new()
                }
            }
            #[derive(Debug, Clone)]
            pub struct Operation {
                pub opcode: Opcode,
                pub inputs: Vec<OpIndex>,
                pub saturated_use_count: UseCounter,
            }

            impl Operation {
                pub fn new(opcode: Opcode, inputs: Vec<OpIndex>) -> Self {
                    Operation {
                        opcode,
                        inputs,
                        saturated_use_count: UseCounter::new(),
                    }
                }
                pub fn is<T: OperationTrait + 'static>(&self) -> bool {
                    self.as_any().downcast_ref::<T>().is_some()
                }

                pub fn try_cast<T: OperationTrait + 'static>(&self) -> Option<&T> {
                    self.as_any().downcast_ref::<T>()
                }
                fn as_any(&self) -> &dyn Any {
                    self
                }

            }
            impl OperationTrait for Operation {
                fn opcode(&self) -> Opcode {
                    self.opcode
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.inputs
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.saturated_use_count
                }
            }
            #[derive(Debug, Clone)]
            pub struct StartOp {
                pub base: Operation,
            }
            impl StartOp {
                pub fn new() -> Self {
                    StartOp {
                        base: Operation::new(Opcode::kStart, Vec::new()),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT) -> Self {
                    StartOp::new()
                }
            }
            impl OperationTrait for StartOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct EndOp {
                pub base: Operation,
                pub control: OpIndex,
            }

            impl EndOp {
                pub fn new(control: OpIndex) -> Self {
                    EndOp {
                        base: Operation::new(Opcode::kEnd, vec![control]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, control: OpIndex) -> Self {
                    EndOp::new(control)
                }
            }
            impl OperationTrait for EndOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct PhiOp {
                pub base: Operation,
                pub rep: Rep,
            }

            impl PhiOp {
                pub fn new(inputs: Vec<OpIndex>, rep: Rep) -> Self {
                    PhiOp {
                        base: Operation::new(Opcode::kPhi, inputs),
                        rep,
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, inputs: Vec<OpIndex>, rep: Rep) -> Self {
                    PhiOp::new(inputs, rep)
                }
            }
            impl OperationTrait for PhiOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct GotoOp {
                pub base: Operation,
                pub destination: *mut super::Block,
            }

            impl GotoOp {
                pub fn new(destination: *mut super::Block) -> Self {
                    GotoOp {
                        base: Operation::new(Opcode::kGoto, Vec::new()),
                        destination,
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, destination: *mut super::Block) -> Self {
                    GotoOp::new(destination)
                }
            }
            impl OperationTrait for GotoOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct BranchOp {
                pub base: Operation,
            }
            impl BranchOp {
                pub fn new(condition: OpIndex, true_destination: *mut super::Block, false_destination: *mut super::Block) -> Self {
                    BranchOp {
                        base: Operation::new(Opcode::kBranch, vec![condition]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, condition: OpIndex, true_destination: *mut super::Block, false_destination: *mut super::Block) -> Self {
                    BranchOp::new(condition, true_destination, false_destination)
                }
            }
            impl OperationTrait for BranchOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct SwitchOp {
                pub base: Operation,
            }
            impl SwitchOp {
                pub fn new(value: OpIndex, destinations: Vec<*mut super::Block>, default_destination: *mut super::Block) -> Self {
                    SwitchOp {
                        base: Operation::new(Opcode::kSwitch, vec![value]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, value: OpIndex, destinations: Vec<*mut super::Block>, default_destination: *mut super::Block) -> Self {
                    SwitchOp::new(value, destinations, default_destination)
                }
            }
            impl OperationTrait for SwitchOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct ReturnOp {
                pub base: Operation,
            }
            impl ReturnOp {
                pub fn new(value: OpIndex) -> Self {
                    ReturnOp {
                        base: Operation::new(Opcode::kReturn, vec![value]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, value: OpIndex) -> Self {
                    ReturnOp::new(value)
                }
            }
            impl OperationTrait for ReturnOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct LoadOp {
                pub base: Operation,
            }
            impl LoadOp {
                pub fn new(address: OpIndex) -> Self {
                    LoadOp {
                        base: Operation::new(Opcode::kLoad, vec![address]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, address: OpIndex) -> Self {
                    LoadOp::new(address)
                }
            }
            impl OperationTrait for LoadOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct StoreOp {
                pub base: Operation,
            }
            impl StoreOp {
                pub fn new(address: OpIndex, value: OpIndex) -> Self {
                    StoreOp {
                        base: Operation::new(Opcode::kStore, vec![address, value]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, address: OpIndex, value: OpIndex) -> Self {
                    StoreOp::new(address, value)
                }
            }
            impl OperationTrait for StoreOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct AddOp {
                pub base: Operation,
            }
            impl AddOp {
                pub fn new(left: OpIndex, right: OpIndex) -> Self {
                    AddOp {
                        base: Operation::new(Opcode::kAdd, vec![left, right]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, left: OpIndex, right: OpIndex) -> Self {
                    AddOp::new(left, right)
                }
            }
            impl OperationTrait for AddOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct SubOp {
                pub base: Operation,
            }
            impl SubOp {
                pub fn new(left: OpIndex, right: OpIndex) -> Self {
                    SubOp {
                        base: Operation::new(Opcode::kSub, vec![left, right]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, left: OpIndex, right: OpIndex) -> Self {
                    SubOp::new(left, right)
                }
            }
            impl OperationTrait for SubOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct MulOp {
                pub base: Operation,
            }
            impl MulOp {
                pub fn new(left: OpIndex, right: OpIndex) -> Self {
                    MulOp {
                        base: Operation::new(Opcode::kMul, vec![left, right]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, left: OpIndex, right: OpIndex) -> Self {
                    MulOp::new(left, right)
                }
            }
            impl OperationTrait for MulOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct DivOp {
                pub base: Operation,
            }
            impl DivOp {
                pub fn new(left: OpIndex, right: OpIndex) -> Self {
                    DivOp {
                        base: Operation::new(Opcode::kDiv, vec![left, right]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, left: OpIndex, right: OpIndex) -> Self {
                    DivOp::new(left, right)
                }
            }
            impl OperationTrait for DivOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeadOp {
                pub base: Operation,
            }
            impl DeadOp {
                pub fn new() -> Self {
                    DeadOp {
                        base: Operation::new(Opcode::kDead, Vec::new()),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT) -> Self {
                    DeadOp::new()
                }
            }
            impl OperationTrait for DeadOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct CheckExceptionOp {
                pub base: Operation,
            }
            impl CheckExceptionOp {
                pub fn new(exception: OpIndex) -> Self {
                    CheckExceptionOp {
                        base: Operation::new(Opcode::kCheckException, vec![exception]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, exception: OpIndex) -> Self {
                    CheckExceptionOp::new(exception)
                }
            }
            impl OperationTrait for CheckExceptionOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct PendingLoopPhiOp {
                pub base: Operation,
                pub rep: Rep,
                pub first: OpIndex,
            }

            impl PendingLoopPhiOp {
                pub fn new(rep: Rep, first: OpIndex) -> Self {
                    PendingLoopPhiOp {
                        base: Operation::new(Opcode::kPendingLoopPhi, vec![first]),
                        rep,
                        first,
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, rep: Rep, first: OpIndex) -> Self {
                    PendingLoopPhiOp::new(rep, first)
                }
                pub fn first(&self) -> OpIndex {
                    self.first
                }
            }
            impl OperationTrait for PendingLoopPhiOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct FrameStateOp {
                pub base: Operation,
            }
            impl FrameStateOp {
                pub fn new() -> Self {
                    FrameStateOp {
                        base: Operation::new(Opcode::kFrameState, Vec::new()),
                    }
                }
                 pub fn New<GraphT>(_graph: &mut GraphT) -> Self {
                    FrameStateOp::new()
                }
           }
            impl OperationTrait for FrameStateOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeoptimizeOp {
                pub base: Operation,
            }
            impl DeoptimizeOp {
                pub fn new() -> Self {
                    DeoptimizeOp {
                        base: Operation::new(Opcode::kDeoptimize, Vec::new()),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT) -> Self {
                    DeoptimizeOp::new()
                }
            }
            impl OperationTrait for DeoptimizeOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct ConstantOp {
                pub base: Operation,
                pub value: u64,
            }
            impl ConstantOp {
                pub fn new(value: u64) -> Self {
                    ConstantOp {
                        base: Operation::new(Opcode::kConstant, Vec::new()),
                        value,
                    }
                }
                 pub fn New<GraphT>(_graph: &mut GraphT, value: u64) -> Self {
                    ConstantOp::new(value)
                }
           }
            impl OperationTrait for ConstantOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct TupleOp {
                pub base: Operation,
            }
            impl TupleOp {
                pub fn new(values: Vec<OpIndex>) -> Self {
                    TupleOp {
                        base: Operation::new(Opcode::kTuple, values),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, values: Vec<OpIndex>) -> Self {
                    TupleOp::new(values)
                }
            }
            impl OperationTrait for TupleOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct CallOp {
                pub base: Operation,
            }
            impl CallOp {
                pub fn new(callee: OpIndex, arguments: Vec<OpIndex>) -> Self {
                    CallOp {
                        base: Operation::new(Opcode::kCall, vec![callee]),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT, callee: OpIndex, arguments: Vec<OpIndex>) -> Self {
                    CallOp::new(callee, arguments)
                }
            }
            impl OperationTrait for CallOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
            #[derive(Debug, Clone)]
            pub struct StackCheckOp {
                pub base: Operation,
            }
            impl StackCheckOp {
                pub fn new() -> Self {
                    StackCheckOp {
                        base: Operation::new(Opcode::kStackCheck, Vec::new()),
                    }
                }
                pub fn New<GraphT>(_graph: &mut GraphT) -> Self {
                    StackCheckOp::new()
                }
            }
            impl OperationTrait for StackCheckOp {
                fn opcode(&self) -> Opcode {
                    self.base.opcode()
                }

                fn inputs(&self) -> &[OpIndex] {
                    &self.base.inputs()
                }
                fn saturated_use_count(&self) -> &UseCounter {
                    &self.base.saturated_use_count()
                }
            }
        }

        pub mod sidetable {
            use crate::compiler::turboshaft::type_inference_reducer::OpIndex;

            pub struct GrowingOpIndexSidetable<T> {
                data: Vec<T>,
                phantom: std::marker::PhantomData<T>,
            }

            impl<T: Clone + Default> GrowingOpIndexSidetable<T> {
                pub fn new(zone: *mut Zone, graph: &Graph) -> Self {
                    GrowingOpIndexSidetable {
                        data: vec![T::default(); graph.op_id_count() as usize],
                        phantom: std::marker::PhantomData,
                    }
                }
                pub fn Reset(&mut self) {
                    self.data.clear();
                }
                pub fn SwapData(&mut self, other: &mut Self) {
                    std::mem::swap(&mut self.data, &mut other.data);
                }
                pub fn resize(&mut self, new_size: usize, value: T) {
                    self.data.resize(new_size, value);
                }
                pub fn clear(&mut self) {
                    self.data.clear();
                }
            }
            impl<T: Clone> std::ops::Index<OpIndex> for GrowingOpIndexSidetable<T> {
                type Output = T;

                fn index(&self, index: OpIndex) -> &Self::Output {
                    &self.data[index.id() as usize]
                }
            }

            impl<T: Clone> std::ops::IndexMut<OpIndex> for GrowingOpIndexSidetable<T> {
                fn index_mut(&mut self, index: OpIndex) -> &mut Self::Output {
                    &mut self.data[index.id() as usize]
                }
            }
            pub struct GrowingBlockSidetable<T> {
                data: Vec<T>,
            }
            impl<T: Clone> GrowingBlockSidetable<T> {
                pub fn new(zone: *mut Zone) -> Self {
                    GrowingBlockSidetable {
                        data: Vec::new(),
                    }
                }
                pub fn Reset(&mut self) {
                    self.data.clear();
                }
            }
        }

        pub mod types {
            use std::vec::Vec;
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Kind {
                kNumber,
                kString,
                kBoolean,
                kObject,
                kOther,
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct Type {}
        }

        use self::operations::{
            DeadOp, EndOp, GotoOp, Operation, OperationStorageSlot, OperationTrait, PhiOp, StartOp,
        };

        use self::types::Type;
        use std::fmt;
        use std::ops::Deref;

        pub struct OperationBuffer {
            zone_: *mut Zone,
            begin_: *mut OperationStorageSlot,
            end_: *mut OperationStorageSlot,
            end_cap_: *mut OperationStorageSlot,
            operation_sizes_: *mut u16,
        }

        impl OperationBuffer {
            const K_SLOTS_PER_ID: usize = 8;

            pub struct ReplaceScope<'a> {
                buffer_: &'a mut OperationBuffer,
                replaced_: OpIndex,
                old_end_: *mut OperationStorageSlot,
                old_slot_count_: u16,
            }

            impl<'a> ReplaceScope<'a> {
                pub fn new(buffer: &'a mut OperationBuffer, replaced: OpIndex) -> Self {
                    let old_slot_count_ = buffer.SlotCount(replaced);
                    let old_end_ = buffer.end_;
                    buffer.end_ = buffer.Get(replaced);
                    ReplaceScope {
                        buffer_: buffer,
                        replaced_: replaced,
                        old_end_: old_end_,
                        old_slot_count_:
