// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This is a partial translation and likely incomplete.
// Many V8-specific types and functionalities are not available in Rust and
// would require significant reimplementation or mocking.

// src/maglev/maglev-graph-builder.h (module definition and public interfaces)
mod maglev_graph_builder {
    use std::{
        borrow::Borrow,
        cell::{Cell, RefCell},
        collections::HashMap,
        fmt::{self, Debug},
        mem,
        ops::{Deref, DerefMut},
        rc::Rc,
        sync::{Arc, Mutex},
    };

    // Mock declarations and implementations:

    pub type LocalIsolate = u32; // Placeholder

    pub struct MaglevCompilationUnit {
        //Placeholder
    }

    impl MaglevCompilationUnit {
        pub fn new_dummy(
            zone: &Zone,
            parent: &MaglevCompilationUnit,
            variable_count: i32,
            _i: i32,
            _i1: i32,
        ) -> MaglevCompilationUnit {
            MaglevCompilationUnit {}
        }

        pub fn info(&self) -> CompilationInfo {
            CompilationInfo {}
        }
        pub fn parameter_count(&self) -> i32 {
            0
        }
        pub fn register_count(&self) -> i32 {
            0
        }
    }

    #[derive(Clone)]
    pub struct Graph {
        is_osr_: bool,
    }
    impl Graph {
        pub fn is_osr(&self) -> bool {
            self.is_osr_
        }

        pub fn osr_values(&self) -> &Vec<ValueNode> {
            &self.osr_values_vec
        }
        pub fn osr_values_mut(&mut self) -> &mut Vec<ValueNode> {
            &mut self.osr_values_vec
        }

        osr_values_vec: Vec<ValueNode>,

        pub fn new(is_osr_: bool) -> Graph {
            Graph {
                is_osr_: is_osr_,
                osr_values_vec: Vec::new(),
            }
        }
    }

    pub struct MaglevCallerDetails {
        pub known_node_aspects: KnownNodeAspects,
        pub deopt_frame: DeoptFrame,
        pub arguments: Vec<ValueNode>,
        pub loop_effects: Option<LoopEffects>,
        pub unobserved_context_slot_stores: Vec<i32>,
    }

    pub struct InterpreterFrameState {} // Placeholder

    pub struct FeedbackVector {} // Placeholder

    pub struct FeedbackSlot {} // Placeholder

    pub struct SharedFunctionInfo {} // Placeholder

    pub struct Builtin {} // Placeholder

    pub struct Zone {
        // Placeholder.  Consider using a memory arena crate.
    }

    impl Zone {
        pub fn allocate_array<T>(&self, size: usize) -> *mut T {
            let mut vec: Vec<T> = Vec::with_capacity(size);
            let ptr = vec.as_mut_ptr();
            mem::forget(vec); // Prevent deallocation
            ptr
        }

        pub fn new<T>(&self, value: T) -> Box<T> {
            Box::new(value)
        }

        pub fn new_uninit_slice<T>(&self, size: usize) -> Box<[T]> {
            let mut v = Vec::with_capacity(size);
            unsafe {
                v.set_len(size);
            }
            v.into_boxed_slice()
        }

        pub fn clone_vector<T: Clone>(&self, vector: &Vec<T>) -> Vec<T> {
            vector.clone()
        }
    }

    pub struct CompilerBytecodeLivenessState {} // Placeholder

    pub struct CompilerLoopInfo {} // Placeholder

    pub struct BasicBlock {} // Placeholder

    pub struct MergePointInterpreterFrameState {} // Placeholder

    pub struct ValueNode {
        properties_: ValueProperties,
        opcode_: Opcode,
    } // Placeholder

    impl ValueNode {
        pub fn try_cast<T: NodeTrait>(&self) -> Option<&T> {
            if T::opcode() == self.opcode_ {
                Some(unsafe { &*(self as *const ValueNode as *const T) })
            } else {
                None
            }
        }

        pub fn properties(&self) -> &ValueProperties {
            &self.properties_
        }

        pub fn properties_mut(&mut self) -> &mut ValueProperties {
            &mut self.properties_
        }

        pub fn opcode(&self) -> Opcode {
            self.opcode_
        }

        pub fn add_use(&self) {}

        pub fn is<T: NodeTrait>(&self) -> bool {
            T::opcode() == self.opcode_
        }
    }

    #[derive(Clone, Copy)]
    pub enum Opcode {
        kInvalid,
        kJump,
        kJumpLoop,
        kCheckpointedJump,
        kConstant,
        kSmiConstant,
        kRootConstant,
        kFloat64Constant,
        kInt32Constant,
        kUint32Constant,
        kUnsafeSmiTagInt32,
        kInt32ToNumber,
        kUnsafeSmiTagUint32,
        kUint32ToNumber,
        kFloat64ToTagged,
        kHoleyFloat64ToTagged,
        kIntPtrToNumber,
        kUnsafeSmiTagIntPtr,
        kCheckedSmiTagInt32,
        kCheckedSmiTagUint32,
        kCheckedSmiTagFloat64,
        kCheckedSmiTagIntPtr,
        kCheckedSmiUntag,
        kCheckedNumberToInt32,
        kCheckedUint32ToInt32,
        kTruncateFloat64ToInt32,
        kCheckedTruncateFloat64ToInt32,
        kCheckedIntPtrToInt32,
        kTruncateUint32ToInt32,
        kChangeInt32ToFloat64,
        kChangeUint32ToFloat64,
        kChangeIntPtrToFloat64,
        kCheckedHoleyFloat64ToFloat64,
        kFloat64Negate,
        kCheckedNumberToUint8Clamped,
        kInt32ToUint8Clamped,
        kUint32ToUint8Clamped,
        kFloat64ToUint8Clamped,
        kInt32BitwiseNot,
        kInt32IncrementWithOverflow,
        kInt32DecrementWithOverflow,
        kInt32NegateWithOverflow,
        kInt32BitwiseAnd,
        kInt32BitwiseOr,
        kInt32BitwiseXor,
        kInt32ShiftLeft,
        kInt32ShiftRight,
        kInt32ShiftRightLogical,
        kInt32AddWithOverflow,
        kInt32SubtractWithOverflow,
        kInt32MultiplyWithOverflow,
        kInt32DivideWithOverflow,
        kInt32ModulusWithOverflow,
        kFloat64Add,
        kFloat64Subtract,
        kFloat64Multiply,
        kFloat64Divide,
        kFloat64Modulus,
        kFloat64Exponentiate,
        kTruncateNumberOrOddballToInt32,
        kCheckedTruncateNumberOrOddballToInt32,
        kHoleyFloat64ToMaybeNanFloat64,
        kCheckedInternalizedString,
        kInitialValue,
        kCreateFunctionContext,
        kCallRuntime,
        kPhi,
        kLoadRegister,
        kBuildConsStringMap,
    }

    pub trait NodeTrait {
        fn opcode() -> Opcode;
    }

    macro_rules! define_node_trait {
        ($name:ident, $opcode:expr) => {
            pub struct $name {}
            impl NodeTrait for $name {
                fn opcode() -> Opcode {
                    $opcode
                }
            }
        };
    }

    define_node_trait!(Jump, Opcode::kJump);
    define_node_trait!(JumpLoop, Opcode::kJumpLoop);
    define_node_trait!(CheckpointedJump, Opcode::kCheckpointedJump);
    define_node_trait!(Constant, Opcode::kConstant);
    define_node_trait!(SmiConstant, Opcode::kSmiConstant);
    define_node_trait!(RootConstant, Opcode::kRootConstant);
    define_node_trait!(Float64Constant, Opcode::kFloat64Constant);
    define_node_trait!(Int32Constant, Opcode::kInt32Constant);
    define_node_trait!(Uint32Constant, Opcode::kUint32Constant);

    pub struct UnsafeSmiTagInt32 {}
    impl NodeTrait for UnsafeSmiTagInt32 {
        fn opcode() -> Opcode {
            Opcode::kUnsafeSmiTagInt32
        }
    }

    pub struct Int32ToNumber {}
    impl NodeTrait for Int32ToNumber {
        fn opcode() -> Opcode {
            Opcode::kInt32ToNumber
        }
    }

    pub struct UnsafeSmiTagUint32 {}
    impl NodeTrait for UnsafeSmiTagUint32 {
        fn opcode() -> Opcode {
            Opcode::kUnsafeSmiTagUint32
        }
    }

    pub struct Uint32ToNumber {}
    impl NodeTrait for Uint32ToNumber {
        fn opcode() -> Opcode {
            Opcode::kUint32ToNumber
        }
    }

    pub struct Float64ToTagged {
        conversion_mode_: Float64ToTaggedConversionMode,
    }
    impl Float64ToTagged {
        pub fn new(mode: Float64ToTaggedConversionMode) -> Float64ToTagged {
            Float64ToTagged {
                conversion_mode_: mode,
            }
        }
    }

    impl NodeTrait for Float64ToTagged {
        fn opcode() -> Opcode {
            Opcode::kFloat64ToTagged
        }
    }

    #[derive(Clone, Copy)]
    pub enum Float64ToTaggedConversionMode {
        kCanonicalizeSmi,
    }

    pub struct HoleyFloat64ToTagged {
        conversion_mode_: HoleyFloat64ToTaggedConversionMode,
    }
    impl HoleyFloat64ToTagged {
        pub fn new(mode: HoleyFloat64ToTaggedConversionMode) -> HoleyFloat64ToTagged {
            HoleyFloat64ToTagged {
                conversion_mode_: mode,
            }
        }
        pub fn set_mode(&mut self, mode: HoleyFloat64ToTaggedConversionMode) {
            self.conversion_mode_ = mode;
        }
    }

    impl NodeTrait for HoleyFloat64ToTagged {
        fn opcode() -> Opcode {
            Opcode::kHoleyFloat64ToTagged
        }
    }

    #[derive(Clone, Copy)]
    pub enum HoleyFloat64ToTaggedConversionMode {
        kForceHeapNumber,
        kCanonicalizeSmi,
    }

    pub struct IntPtrToNumber {}
    impl NodeTrait for IntPtrToNumber {
        fn opcode() -> Opcode {
            Opcode::kIntPtrToNumber
        }
    }

    pub struct UnsafeSmiTagIntPtr {}
    impl NodeTrait for UnsafeSmiTagIntPtr {
        fn opcode() -> Opcode {
            Opcode::kUnsafeSmiTagIntPtr
        }
    }

    pub struct CheckedSmiTagInt32 {}
    impl NodeTrait for CheckedSmiTagInt32 {
        fn opcode() -> Opcode {
            Opcode::kCheckedSmiTagInt32
        }
    }

    pub struct CheckedSmiTagUint32 {}
    impl NodeTrait for CheckedSmiTagUint32 {
        fn opcode() -> Opcode {
            Opcode::kCheckedSmiTagUint32
        }
    }

    pub struct CheckedSmiTagFloat64 {}
    impl NodeTrait for CheckedSmiTagFloat64 {
        fn opcode() -> Opcode {
            Opcode::kCheckedSmiTagFloat64
        }
    }

    pub struct CheckedSmiTagIntPtr {}
    impl NodeTrait for CheckedSmiTagIntPtr {
        fn opcode() -> Opcode {
            Opcode::kCheckedSmiTagIntPtr
        }
    }

    pub struct CheckedSmiUntag {}
    impl NodeTrait for CheckedSmiUntag {
        fn opcode() -> Opcode {
            Opcode::kCheckedSmiUntag
        }
    }

    pub struct CheckedNumberToInt32 {}
    impl NodeTrait for CheckedNumberToInt32 {
        fn opcode() -> Opcode {
            Opcode::kCheckedNumberToInt32
        }
    }

    pub struct CheckedUint32ToInt32 {}
    impl NodeTrait for CheckedUint32ToInt32 {
        fn opcode() -> Opcode {
            Opcode::kCheckedUint32ToInt32
        }
    }

    pub struct TruncateFloat64ToInt32 {}
    impl NodeTrait for TruncateFloat64ToInt32 {
        fn opcode() -> Opcode {
            Opcode::kTruncateFloat64ToInt32
        }
    }

    pub struct CheckedTruncateFloat64ToInt32 {}
    impl NodeTrait for CheckedTruncateFloat64ToInt32 {
        fn opcode() -> Opcode {
            Opcode::kCheckedTruncateFloat64ToInt32
        }
    }

    pub struct CheckedIntPtrToInt32 {}
    impl NodeTrait for CheckedIntPtrToInt32 {
        fn opcode() -> Opcode {
            Opcode::kCheckedIntPtrToInt32
        }
    }

    pub struct TruncateUint32ToInt32 {}
    impl NodeTrait for TruncateUint32ToInt32 {
        fn opcode() -> Opcode {
            Opcode::kTruncateUint32ToInt32
        }
    }

    pub struct ChangeInt32ToFloat64 {}
    impl NodeTrait for ChangeInt32ToFloat64 {
        fn opcode() -> Opcode {
            Opcode::kChangeInt32ToFloat64
        }
    }

    pub struct ChangeUint32ToFloat64 {}
    impl NodeTrait for ChangeUint32ToFloat64 {
        fn opcode() -> Opcode {
            Opcode::kChangeUint32ToFloat64
        }
    }

    pub struct ChangeIntPtrToFloat64 {}
    impl NodeTrait for ChangeIntPtrToFloat64 {
        fn opcode() -> Opcode {
            Opcode::kChangeIntPtrToFloat64
        }
    }

    pub struct CheckedHoleyFloat64ToFloat64 {}
    impl NodeTrait for CheckedHoleyFloat64ToFloat64 {
        fn opcode() -> Opcode {
            Opcode::kCheckedHoleyFloat64ToFloat64
        }
    }

    pub struct Float64Negate {}
    impl NodeTrait for Float64Negate {
        fn opcode() -> Opcode {
            Opcode::kFloat64Negate
        }
    }

    pub struct CheckedNumberToUint8Clamped {}
    impl NodeTrait for CheckedNumberToUint8Clamped {
        fn opcode() -> Opcode {
            Opcode::kCheckedNumberToUint8Clamped
        }
    }

    pub struct Int32ToUint8Clamped {}
    impl NodeTrait for Int32ToUint8Clamped {
        fn opcode() -> Opcode {
            Opcode::kInt32ToUint8Clamped
        }
    }

    pub struct Uint32ToUint8Clamped {}
    impl NodeTrait for Uint32ToUint8Clamped {
        fn opcode() -> Opcode {
            Opcode::kUint32ToUint8Clamped
        }
    }

    pub struct Float64ToUint8Clamped {}
    impl NodeTrait for Float64ToUint8Clamped {
        fn opcode() -> Opcode {
            Opcode::kFloat64ToUint8Clamped
        }
    }

    pub struct Int32BitwiseNot {}
    impl NodeTrait for Int32BitwiseNot {
        fn opcode() -> Opcode {
            Opcode::kInt32BitwiseNot
        }
    }

    pub struct Int32IncrementWithOverflow {}
    impl NodeTrait for Int32IncrementWithOverflow {
        fn opcode() -> Opcode {
            Opcode::kInt32IncrementWithOverflow
        }
    }

    pub struct Int32DecrementWithOverflow {}
    impl NodeTrait for Int32DecrementWithOverflow {
        fn opcode() -> Opcode {
            Opcode::kInt32DecrementWithOverflow
        }
    }

    pub struct Int32NegateWithOverflow {}
    impl NodeTrait for Int32NegateWithOverflow {
        fn opcode() -> Opcode {
            Opcode::kInt32NegateWithOverflow
        }
    }

    pub struct Int32BitwiseAnd {}
    impl NodeTrait for Int32BitwiseAnd {
        fn opcode() -> Opcode {
            Opcode::kInt32BitwiseAnd
        }
    }

    pub struct Int32BitwiseOr {}
    impl NodeTrait for Int32BitwiseOr {
        fn opcode() -> Opcode {
            Opcode::kInt32BitwiseOr
        }
    }

    pub struct Int32BitwiseXor {}
    impl NodeTrait for Int32BitwiseXor {
        fn opcode() -> Opcode {
            Opcode::kInt32BitwiseXor
        }
    }

    pub struct Int32ShiftLeft {}
    impl NodeTrait for Int32ShiftLeft {
        fn opcode() -> Opcode {
            Opcode::kInt32ShiftLeft
        }
    }

    pub struct Int32ShiftRight {}
    impl NodeTrait for Int32ShiftRight {
        fn opcode() -> Opcode {
            Opcode::kInt32ShiftRight
        }
    }

    pub struct Int32ShiftRightLogical {}
    impl NodeTrait for Int32ShiftRightLogical {
        fn opcode() -> Opcode {
            Opcode::kInt32ShiftRightLogical
        }
    }

    pub struct Int32AddWithOverflow {}
    impl NodeTrait for Int32AddWithOverflow {
        fn opcode() -> Opcode {
            Opcode::kInt32AddWithOverflow
        }
    }

    pub struct Int32SubtractWithOverflow {}
    impl NodeTrait for Int32SubtractWithOverflow {
        fn opcode() -> Opcode {
            Opcode::kInt32SubtractWithOverflow
        }
    }

    pub struct Int32MultiplyWithOverflow {}
    impl NodeTrait for Int32MultiplyWithOverflow {
        fn opcode() -> Opcode {
            Opcode::kInt32MultiplyWithOverflow
        }
    }

    pub struct Int32DivideWithOverflow {}
    impl NodeTrait for Int32DivideWithOverflow {
        fn opcode() -> Opcode {
            Opcode::kInt32DivideWithOverflow
        }
    }

    pub struct Int32ModulusWithOverflow {}
    impl NodeTrait for Int32ModulusWithOverflow {
        fn opcode() -> Opcode {
            Opcode::kInt32ModulusWithOverflow
        }
    }

    pub struct Float64Add {}
    impl NodeTrait for Float64Add {
        fn opcode() -> Opcode {
            Opcode::kFloat64Add
        }
    }

    pub struct Float64Subtract {}
    impl NodeTrait for Float64Subtract {
        fn opcode() -> Opcode {
            Opcode::kFloat64Subtract
        }
    }

    pub struct Float64Multiply {}
    impl NodeTrait for Float64Multiply {
        fn opcode() -> Opcode {
            Opcode::kFloat64Multiply
        }
    }

    pub struct Float64Divide {}
    impl NodeTrait for Float64Divide {
        fn opcode() -> Opcode {
            Opcode::kFloat64Divide
        }
    }

    pub struct Float64Modulus {}
    impl NodeTrait for Float64Modulus {
        fn opcode() -> Opcode {
            Opcode::kFloat64Modulus
        }
    }

    pub struct Float64Exponentiate {}
    impl NodeTrait for Float64Exponentiate {
        fn opcode() -> Opcode {
            Opcode::kFloat64Exponentiate
        }
    }

    pub struct TruncateNumberOrOddballToInt32 {}
    impl NodeTrait for TruncateNumberOrOddballToInt32 {
        fn opcode() -> Opcode {
            Opcode::kTruncateNumberOrOddballToInt32
        }
    }

    pub struct CheckedTruncateNumberOrOddballToInt32 {}
    impl NodeTrait for CheckedTruncateNumberOrOddballToInt32 {
        fn opcode() -> Opcode {
            Opcode::kCheckedTruncateNumberOrOddballToInt32
        }
    }

    pub struct HoleyFloat64ToMaybeNanFloat64 {}
    impl NodeTrait for HoleyFloat64ToMaybeNanFloat64 {
        fn opcode() -> Opcode {
            Opcode::kHoleyFloat64ToMaybeNanFloat64
        }
    }

    pub struct CheckedInternalizedString {}
    impl NodeTrait for CheckedInternalizedString {
        fn opcode() -> Opcode {
            Opcode::kCheckedInternalizedString
        }
    }

    pub struct InitialValue {}
    impl NodeTrait for InitialValue {
        fn opcode() -> Opcode {
            Opcode::kInitialValue
        }
    }

    pub struct CreateFunctionContext {}
    impl NodeTrait for CreateFunctionContext {
        fn opcode() -> Opcode {
            Opcode::kCreateFunctionContext
        }
    }

    pub struct CallRuntime {}
    impl NodeTrait for CallRuntime {
        fn opcode() -> Opcode {
            Opcode::kCallRuntime
        }
    }

    pub struct Phi {}
    impl NodeTrait for Phi {
        fn opcode() -> Opcode {
            Opcode::kPhi
        }
    }

    pub struct LoadRegister {}
    impl NodeTrait for LoadRegister {
        fn opcode() -> Opcode {
            Opcode::kLoadRegister
        }
    }

    pub struct BuildConsStringMap {}
    impl NodeTrait for BuildConsStringMap {
        fn opcode() -> Opcode {
            Opcode::kBuildConsStringMap
        }
    }

    pub struct InterpreterFrame {
        known_node_aspects_: KnownNodeAspects,
        virtual_objects_: VirtualObjectList,
    } // Placeholder

    impl InterpreterFrame {
        pub fn new(
            compilation_unit: &MaglevCompilationUnit,
            known_node_aspects: KnownNodeAspects,
            virtual_objects: VirtualObjectList,
        ) -> InterpreterFrame {
            InterpreterFrame {
                known_node_aspects_: known_node_aspects,
                virtual_objects_: virtual_objects,
            }
        }

        pub fn virtual_objects(&self) -> &VirtualObjectList {
            &self.virtual_objects_
        }

        pub fn set_virtual_objects(&mut self, virtual_objects: VirtualObjectList) {
            self.virtual_objects_ = virtual_objects;
        }

        pub fn known_node_aspects(&self) -> &KnownNodeAspects {
            &self.known_node_aspects_
        }

        pub fn set_known_node_aspects(&mut self, known_node_aspects: KnownNodeAspects) {
            self.known_node_aspects_ = known_node_aspects;
        }
    }

    pub struct KnownNodeAspects {} // Placeholder

    pub struct VirtualObjectList {} // Placeholder

    impl VirtualObjectList {
        pub fn snapshot(&self) {}
        pub fn is_empty(&self) -> bool {
            true
        }
    }

    pub struct CallInterfaceDescriptor {} // Placeholder

    pub struct DeoptFrame {} // Placeholder

    pub struct InlinedArgumentsDeoptFrame {} // Placeholder

    pub struct CompactInterpreterFrameState {} // Placeholder

    pub struct InterpretedDeoptFrame {} // Placeholder

    pub struct BytecodeOffset {} // Placeholder

    pub struct SourcePosition {} // Placeholder

    pub struct Oddball {} // Placeholder

    pub struct HeapNumber {} // Placeholder

    pub struct Tagged<T> {
        _phantom: std::marker::PhantomData<T>,
    } // Placeholder

    pub struct Type {} // Placeholder

    pub struct NodeInfo {} // Placeholder

    impl NodeInfo {
        pub fn alternative(&mut self) -> &mut AlternativeRepresentation {
            &mut self.alternative_
        }

        pub fn is_smi(&self) -> bool {
            false
        }
        pub fn combine_type(&mut self, _node_type: NodeType) {}
        pub fn type_(&self) -> NodeType {
            NodeType::kAny
        }

        pub fn alternative_mut(&mut self) -> &mut AlternativeRepresentation {
            &mut self.alternative_
        }

        alternative_: AlternativeRepresentation,
    }

    pub struct AlternativeRepresentation {
        tagged_: Option<ValueNode>,
        int32_: Option<ValueNode>,
        float64_: Option<ValueNode>,
        truncated_int32_to_number_: Option<ValueNode>,
        checked_value_: Option<ValueNode>,
    } // Placeholder

    impl AlternativeRepresentation {
        pub fn set_tagged(&mut self, node: ValueNode) -> &mut ValueNode {
            self.tagged_ = Some(node);
            self.tagged_.as_mut().unwrap()
        }
        pub fn set_int32(&mut self, node: ValueNode) -> &mut ValueNode {
            self.int32_ = Some(node);
            self.int32_.as_mut().unwrap()
        }
        pub fn set_float64(&mut self, node: ValueNode) -> &mut ValueNode {
            self.float64_ = Some(node);
            self.float64_.as_mut().unwrap()
        }
        pub fn tagged(&self) -> Option<&ValueNode> {
            self.tagged_.as_ref()
        }
        pub fn int32(&self) -> Option<&ValueNode> {
            self.int32_.as_ref()
        }
        pub fn float64(&self) -> Option<&ValueNode> {
            self.float64_.as_ref()
        }

        pub fn truncated_int32_to_number(&self) -> Option<&ValueNode> {
            self.truncated_int32_to_number_.as_ref()
        }

        pub fn set_truncated_int32_to_number(
            &mut self,
            node: ValueNode,
        ) -> &mut ValueNode {
            self.truncated_int32_to_number_ = Some(node);
            self.truncated_int32_to_number_.as_mut().unwrap()
        }
        pub fn set_checked_value(&mut self, node: ValueNode) -> &mut ValueNode {
            self.checked_value_ = Some(node);
            self.checked_value_.as_mut().unwrap()
        }

        pub fn checked_value(&self) -> Option<&ValueNode> {
            self.checked_value_.as_ref()
        }
    }

    pub struct Code {} // Placeholder

    pub struct FeedbackNexus {} // Placeholder

    pub enum ConvertReceiverMode {
        kNullOrUndefined,
        kAny,
    }

    pub struct ObjectRef {} // Placeholder

    impl ObjectRef {
        pub fn is_heap_number(&self) -> bool {
            false
        }

        pub fn as_heap_number(&self) -> HeapNumberRef {
            HeapNumberRef {}
        }
        pub fn object(&self) -> &Tagged<Object> {
            todo!()
        }
    }
    pub struct HeapNumberRef {}
    impl HeapNumberRef {
        pub fn value(&self) -> f64 {
            0.0
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum NodeType {
        kAny,
        kNumber,
        kSmi,
        kNumberOrOddball,
        kString,
        kInternalizedString,
        kAnyHeapObject,
        kNumberOrBoolean,
    }

    pub struct ValueProperties {
        value_representation_: ValueRepresentation,
    } // Placeholder

    impl ValueProperties {
        pub fn value_representation(&self) -> ValueRepresentation {
            self.value_representation_
        }

        pub fn set_value_representation(&mut self, repr: ValueRepresentation) {
            self.value_representation_ = repr;
        }

        pub fn is_conversion(&self) -> bool {
            false
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum ValueRepresentation {
        kTagged,
        kInt32,
        kUint32,
        kFloat64,
        kHoleyFloat64,
        kIntPtr,
    }

    pub enum Operation {
        kAdd,
        kSubtract,
        kMultiply,
        kDivide,
        kModulus,
        kBitwiseAnd,
        kBitwiseOr,
        kBitwiseXor,
        kShiftLeft,
        kShiftRight,
        kShiftRightLogical,
        kExponentiate,
        kBitwiseNot,
        kIncrement,
        kDecrement,
        kNegate,
    }

    pub enum UseRepresentation {
        kTagged,
        kInt32,
        kFloat64,
        kTruncatedInt32,
        kHoleyFloat64,
    }

    pub enum RootIndex {
        kUndefinedValue,
    }

    pub enum DeoptimizeReason {
        kInsufficientTypeFeedbackForBinaryOperation,
    }

    pub enum BranchType {
        kBranchIfTrue,
        kBranchIfFalse,
    }

    pub enum BranchResult {
        kAlwaysTrue,
        kAlwaysFalse,
        kDefault,
    }

    pub enum BranchSpecializationMode {
        kAlwaysBoolean,
    }

    pub struct MaybeHandle<T> {
        _phantom: std::marker::PhantomData<T>,
    } // Placeholder

    impl<T> MaybeHandle<T> {
        pub fn to_handle(&self) -> Handle<T> {
            Handle {}
        }
    }

    pub struct Handle<T> {
        _phantom: std::marker::PhantomData<T>,
    } // Placeholder

    pub struct CallRuntimeArguments {} // Placeholder

    pub struct SourcePositionTableIterator {} // Placeholder

    pub struct BytecodeArrayIterator {} // Placeholder

    pub struct Builtins {} // Placeholder

    pub struct JavaScriptCallNewTargetRegister {} // Placeholder

    pub struct CompilationDependencies {} // Placeholder
    impl CompilationDependencies {
        pub fn depend_on_stable_property_chain(
            &self,
            _handle: Handle<SharedFunctionInfo>,
            _details: u32,
            _name: Handle<Type>,
        ) {
        }
    }

    pub struct Script {} // Placeholder

    pub struct FeedbackSource {} // Placeholder

    pub struct ProcessedFeedback {} // Placeholder
    impl ProcessedFeedback {
        pub fn is_insufficient(&self) -> bool {
            true
        }

        pub fn as_call(&self) -> Call {
            Call {}
        }
    }
    pub struct Call {}
    impl Call {
        pub fn speculation_mode(&self) -> SpeculationMode {
            SpeculationMode::kAllowSpeculation
        }
    }
    pub enum SpeculationMode {
        kAllowSpeculation,
    }

    pub struct Arguments {} // Placeholder

    pub struct ElementsKind {} // Placeholder

    pub struct FixedArray {} // Placeholder

    pub struct JSArray {} // Placeholder

    pub struct JSFunction {} // Placeholder

    pub struct JSObject {} // Placeholder

    pub struct Literal {} // Placeholder

    pub struct Name {} // Placeholder

    pub struct PropertyCell {} // Placeholder

    pub struct PropertyDetails {} // Placeholder

    pub struct SharedFunctionInfo {} // Placeholder

    pub struct Slots {} // Placeholder

    pub struct TypeHints {} // Placeholder

    pub struct Utils {} // Placeholder

    pub struct Intl {} // Placeholder

    pub struct Double {} // Placeholder

    impl Double {
        pub fn get_scalar(&self) -> f64 {
            0.0
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum TaggedToFloat64ConversionType {
        kOnlyNumber,
        kNumberOrOddball,
        kNumberOrBoolean,
    }

    pub struct InlinedAllocation {} // Placeholder

    impl InlinedAllocation {
        pub fn object(&self) -> &ObjectPlaceHolder {
            &self.object_
        }
        pub fn force_escaping(&self) {}

        object_: ObjectPlaceHolder,
    }
    pub struct ObjectPlaceHolder {}
    impl ObjectPlaceHolder {
        pub fn get(&self, _offset: i32) -> &ValueNode {
            todo!()
        }
    }

    pub struct ConstructInvokeStubFrameData {} // Placeholder

    pub struct BuiltinContinuationFrameData {} // Placeholder

    pub struct LoopEffects {} // Placeholder

    pub enum CheckType {
        kOmitHeapObjectCheck,
        kCheckHeapObject,
    }

    pub enum BinaryOperationHint {
        kNone,
        kSignedSmall,
        kSignedSmallInputs,
        kAdditiveSafeInteger,
        kNumber,
        kNumberOrOddball,
        kString,
        kStringOrStringWrapper,
        kBigInt,
        kBigInt64,
        kAny,
    }

    // Impls for enums:
    impl From<bool> for BranchResult {
        fn from(value: bool) -> Self {
            if value {
                BranchResult::kAlwaysTrue
            } else {
                BranchResult::kAlwaysFalse
            }
        }
    }

    impl From<u32> for LocalIsolate {
        fn from(value: u32) -> Self {
            value
        }
    }

    impl From<i32> for FeedbackSlot {
        fn from(value: i32) -> Self {
