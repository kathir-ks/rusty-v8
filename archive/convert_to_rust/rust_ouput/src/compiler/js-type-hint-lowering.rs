// Converted from V8 C++ source files:
// Header: js-type-hint-lowering.h
// Implementation: js-type-hint-lowering.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod js_type_hint_lowering {
use crate::base::flags::Flags;
use crate::compiler::graph_reducer::GraphReducer;
use crate::deoptimizer::deoptimize_reason::DeoptimizeReason;

pub struct FeedbackSlot {}

pub struct JSGraph {}
pub struct Node {}
pub struct Operator {}
pub struct JSHeapBroker {}
pub struct FeedbackVectorRef {}
pub struct Isolate {}
pub struct TFGraph {}
pub struct JSOperatorBuilder {}
pub struct SimplifiedOperatorBuilder {}
pub struct CommonOperatorBuilder {}
pub struct FeedbackSource {}
pub enum BranchHint {}
pub enum MemoryAccessKind {}
pub struct MapRef {}
pub struct HeapObjectRef {}
pub enum Mode {}
pub enum Type {}
pub struct Builtin {}
pub enum AtomicMemoryOrder {}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BinaryOperationHint {
    kSignedSmall,
    kSignedSmallInputs,
    kAdditiveSafeInteger,
    kNumber,
    kNumberOrOddball,
    kAny,
    kNone,
    kString,
    kStringOrStringWrapper,
    kBigInt,
    kBigInt64,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CompareOperationHint {
    kSignedSmall,
    kNumber,
    kNumberOrBoolean,
    kNumberOrOddball,
    kAny,
    kNone,
    kString,
    kSymbol,
    kBigInt,
    kBigInt64,
    kReceiver,
    kReceiverOrNullOrUndefined,
    kInternalizedString,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum NumberOperationHint {
    kSignedSmall,
    kSignedSmallInputs,
    kAdditiveSafeInteger,
    kNumber,
    kNumberOrOddball,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BigIntOperationHint {
    kBigInt64,
    kBigInt,
}

pub enum TypeOfFeedbackResult {
    kNumber,
    kString,
    kFunction,
    kOther,
}

pub struct JSTypeHintLowering {
    broker_: *mut JSHeapBroker,
    jsgraph_: *mut JSGraph,
    flags_: Flags<Flag>,
    feedback_vector_: FeedbackVectorRef,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum LoweringResultKind {
    kNoChange,
    kSideEffectFree,
    kExit,
}

pub struct LoweringResult {
    kind_: LoweringResultKind,
    value_: *mut Node,
    effect_: *mut Node,
    control_: *mut Node,
}

impl LoweringResult {
    pub fn value(&self) -> *mut Node {
        self.value_
    }
    pub fn effect(&self) -> *mut Node {
        self.effect_
    }
    pub fn control(&self) -> *mut Node {
        self.control_
    }

    pub fn changed(&self) -> bool {
        self.kind_ != LoweringResultKind::kNoChange
    }
    pub fn is_exit(&self) -> bool {
        self.kind_ == LoweringResultKind::kExit
    }
    pub fn is_side_effect_free(&self) -> bool {
        self.kind_ == LoweringResultKind::kSideEffectFree
    }

    pub fn side_effect_free(value: *mut Node, effect: *mut Node, control: *mut Node) -> LoweringResult {
        assert!(!effect.is_null());
        assert!(!control.is_null());
        unsafe {
            assert!((&*value).op().HasProperty());
        }
        LoweringResult {
            kind_: LoweringResultKind::kSideEffectFree,
            value_: value,
            effect_: effect,
            control_: control,
        }
    }

    pub fn no_change() -> LoweringResult {
        LoweringResult {
            kind_: LoweringResultKind::kNoChange,
            value_: std::ptr::null_mut(),
            effect_: std::ptr::null_mut(),
            control_: std::ptr::null_mut(),
        }
    }

    pub fn exit(control: *mut Node) -> LoweringResult {
        LoweringResult {
            kind_: LoweringResultKind::kExit,
            value_: std::ptr::null_mut(),
            effect_: std::ptr::null_mut(),
            control_: control,
        }
    }
}

impl JSTypeHintLowering {
    pub const kNoFlags: u32 = 0u;
    pub const kBailoutOnUninitialized: u32 = 1u << 1;

    pub type Flag = u32;
    pub type Flags = Flags<Self::Flag>;

    pub fn new(
        broker: *mut JSHeapBroker,
        jsgraph: *mut JSGraph,
        feedback_vector: FeedbackVectorRef,
        flags: Flags<Self::Flag>,
    ) -> JSTypeHintLowering {
        JSTypeHintLowering {
            broker_: broker,
            jsgraph_: jsgraph,
            flags_: flags,
            feedback_vector_: feedback_vector,
        }
    }

    pub fn reduce_unary_operation(
        &self,
        op: *const Operator,
        operand: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForUnaryOperation,
            ) {
                return LoweringResult::exit(node);
            }

            let feedback = FeedbackSource::new(self.feedback_vector(), slot);

            let mut node: *mut Node = std::ptr::null_mut();
            let mut check: *mut Node = std::ptr::null_mut();

            match (&*op).opcode() {
                0 => { //IrOpcode::kJSBitwiseNot
                    let b = JSSpeculativeBinopBuilder::new(
                        self,
                        (&*self.jsgraph_).javascript().BitwiseXor(feedback),
                        operand,
                        (&*self.jsgraph_).SmiConstant(-1),
                        effect,
                        control,
                        slot,
                    );
                    node = b.try_build_number_binop();
                }
                1 => { //IrOpcode::kJSDecrement
                    let b = JSSpeculativeBinopBuilder::new(
                        self,
                        (&*self.jsgraph_).javascript().Subtract(feedback),
                        operand,
                        (&*self.jsgraph_).SmiConstant(1),
                        effect,
                        control,
                        slot,
                    );
                    node = b.try_build_number_binop();
                }
                2 => { //IrOpcode::kJSIncrement
                    let b = JSSpeculativeBinopBuilder::new(
                        self,
                        (&*self.jsgraph_).javascript().Add(feedback),
                        operand,
                        (&*self.jsgraph_).SmiConstant(1),
                        effect,
                        control,
                        slot,
                    );
                    node = b.try_build_number_binop();
                }
                3 => { //IrOpcode::kJSNegate
                    let b = JSSpeculativeBinopBuilder::new(
                        self,
                        (&*self.jsgraph_).javascript().Multiply(feedback),
                        operand,
                        (&*self.jsgraph_).SmiConstant(-1),
                        effect,
                        control,
                        slot,
                    );
                    node = b.try_build_number_binop();
                    if node.is_null() {
                        if (&*self.jsgraph_).machine().Is64() {
                            if self.get_binary_operation_hint(slot) == BinaryOperationHint::kBigInt {
                                let op = (&*self.jsgraph_).simplified().SpeculativeBigIntNegate(BigIntOperationHint::kBigInt);
                                node = (&*(&*self.jsgraph_).graph()).NewNode(op, operand, effect, control);
                            }
                        }
                    }
                }
                4 => { //IrOpcode::kTypeOf
                    let hint = (&*self.broker_).GetFeedbackForTypeOf(feedback);
                    match hint {
                        TypeOfFeedbackResult::kNumber => {
                            check = (&*(&*self.jsgraph_).graph()).NewNode(
                                (&*self.jsgraph_).simplified().CheckNumber(FeedbackSource::new(FeedbackVectorRef {}, FeedbackSlot {})),
                                operand,
                                effect,
                                control,
                            );
                            node = (&*self.jsgraph_).ConstantNoHole((&*self.broker_).number_string(), &*self.broker_);
                        }
                        TypeOfFeedbackResult::kString => {
                            check = (&*(&*self.jsgraph_).graph()).NewNode(
                                (&*self.jsgraph_).simplified().CheckString(FeedbackSource::new(FeedbackVectorRef {}, FeedbackSlot {})),
                                operand,
                                effect,
                                control,
                            );
                            node = (&*self.jsgraph_).ConstantNoHole((&*self.broker_).string_string(), &*self.broker_);
                        }
                        TypeOfFeedbackResult::kFunction => {
                            let condition = (&*(&*self.jsgraph_).graph()).NewNode(
                                (&*self.jsgraph_).simplified().ObjectIsDetectableCallable(),
                                operand,
                            );
                            check = (&*(&*self.jsgraph_).graph()).NewNode(
                                (&*self.jsgraph_).simplified().CheckIf(
                                    DeoptimizeReason::kNotDetectableReceiver,
                                    FeedbackSource::new(FeedbackVectorRef {}, FeedbackSlot {}),
                                ),
                                condition,
                                effect,
                                control,
                            );
                            node = (&*self.jsgraph_).ConstantNoHole((&*self.broker_).function_string(), &*self.broker_);
                        }
                        _ => {
                            node = std::ptr::null_mut();
                        }
                    }
                }
                _ => {
                    panic!("UNREACHABLE");
                }
            }

            if !node.is_null() {
                return LoweringResult::side_effect_free(node, if !check.is_null() { check } else { node }, control);
            } else {
                return LoweringResult::no_change();
            }
        }
    }

    pub fn reduce_binary_operation(
        &self,
        op: *const Operator,
        left: *mut Node,
        right: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            match (&*op).opcode() {
                5 => { //IrOpcode::kJSStrictEqual
                    if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                        slot,
                        effect,
                        control,
                        DeoptimizeReason::kInsufficientTypeFeedbackForCompareOperation,
                    ) {
                        return LoweringResult::exit(node);
                    }
                }
                6 | 7 | 8 | 9 | 10 => { //IrOpcode::kJSEqual, IrOpcode::kJSLessThan, IrOpcode::kJSGreaterThan, IrOpcode::kJSLessThanOrEqual, IrOpcode::kJSGreaterThanOrEqual
                    if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                        slot,
                        effect,
                        control,
                        DeoptimizeReason::kInsufficientTypeFeedbackForCompareOperation,
                    ) {
                        return LoweringResult::exit(node);
                    }
                    let b = JSSpeculativeBinopBuilder::new(self, op, left, right, effect, control, slot);
                    if let Some(node) = b.try_build_number_compare() {
                        return LoweringResult::side_effect_free(node, node, control);
                    }
                    if let Some(node) = b.try_build_big_int_compare() {
                        return LoweringResult::side_effect_free(node, node, control);
                    }
                }
                11 => { //IrOpcode::kJSInstanceOf
                    if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                        slot,
                        effect,
                        control,
                        DeoptimizeReason::kInsufficientTypeFeedbackForCompareOperation,
                    ) {
                        return LoweringResult::exit(node);
                    }
                }
                12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 => { //IrOpcode::kJSBitwiseOr, IrOpcode::kJSBitwiseXor, IrOpcode::kJSBitwiseAnd, IrOpcode::kJSShiftLeft, IrOpcode::kJSShiftRight, IrOpcode::kJSShiftRightLogical, IrOpcode::kJSAdd, IrOpcode::kJSSubtract, IrOpcode::kJSMultiply, IrOpcode::kJSDivide, IrOpcode::kJSModulus, IrOpcode::kJSExponentiate
                    if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                        slot,
                        effect,
                        control,
                        DeoptimizeReason::kInsufficientTypeFeedbackForBinaryOperation,
                    ) {
                        return LoweringResult::exit(node);
                    }
                    let b = JSSpeculativeBinopBuilder::new(self, op, left, right, effect, control, slot);
                    if let Some(node) = b.try_build_number_binop() {
                        return LoweringResult::side_effect_free(node, node, control);
                    }
                    if (&*op).opcode() != 16 && (&*op).opcode() != 21 { //IrOpcode::kJSShiftRightLogical && IrOpcode::kJSExponentiate
                        if let Some(node) = b.try_build_big_int_binop() {
                            return LoweringResult::side_effect_free(node, node, control);
                        }
                    }
                }
                _ => {
                    panic!("UNREACHABLE");
                }
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_for_in_next_operation(
        &self,
        receiver: *mut Node,
        cache_array: *mut Node,
        cache_type: *mut Node,
        index: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForForIn,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_for_in_prepare_operation(
        &self,
        enumerator: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForForIn,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_to_number_operation(
        &self,
        input: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        assert!(!slot.IsInvalid());
        let mut hint: NumberOperationHint = NumberOperationHint::kNumber;
        if self.binary_operation_hint_to_number_operation_hint(self.get_binary_operation_hint(slot), &mut hint) {
            unsafe {
                let node = (&*(&*self.jsgraph_).graph()).NewNode(
                    (&*self.jsgraph_).simplified().SpeculativeToNumber(hint, FeedbackSource::new(FeedbackVectorRef {}, FeedbackSlot {})),
                    input,
                    effect,
                    control,
                );
                return LoweringResult::side_effect_free(node, node, control);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_call_operation(
        &self,
        op: *const Operator,
        args: *const *mut Node,
        arg_count: i32,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            assert!((&*op).opcode() == 22 || (&*op).opcode() == 23); //IrOpcode::kJSCall, IrOpcode::kJSCallWithSpread
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForCall,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_construct_operation(
        &self,
        op: *const Operator,
        args: *const *mut Node,
        arg_count: i32,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            assert!((&*op).opcode() == 24 || (&*op).opcode() == 25 || (&*op).opcode() == 26); //IrOpcode::kJSConstruct, IrOpcode::kJSConstructWithSpread, IrOpcode::kJSConstructForwardAllArgs
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForConstruct,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_get_iterator_operation(
        &self,
        op: *const Operator,
        receiver: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        load_slot: FeedbackSlot,
        call_slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            assert_eq!((&*op).opcode(), 27); //IrOpcode::kJSGetIterator
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                load_slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericNamedAccess,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_load_named_operation(
        &self,
        op: *const Operator,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            assert!((&*op).opcode() == 28 || (&*op).opcode() == 29); //IrOpcode::kJSLoadNamed, IrOpcode::kJSLoadNamedFromSuper
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericNamedAccess,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_load_keyed_operation(
        &self,
        op: *const Operator,
        obj: *mut Node,
        key: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            assert_eq!((&*op).opcode(), 30); //IrOpcode::kJSLoadProperty
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericKeyedAccess,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_store_named_operation(
        &self,
        op: *const Operator,
        obj: *mut Node,
        val: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            assert!((&*op).opcode() == 31 || (&*op).opcode() == 32); //IrOpcode::kJSSetNamedProperty, IrOpcode::kJSDefineNamedOwnProperty
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericNamedAccess,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    pub fn reduce_store_keyed_operation(
        &self,
        op: *const Operator,
        obj: *mut Node,
        key: *mut Node,
        val: *mut Node,
        effect: *mut Node,
        control: *mut Node,
        slot: FeedbackSlot,
    ) -> LoweringResult {
        unsafe {
            assert!((&*op).opcode() == 33 || (&*op).opcode() == 34 || (&*op).opcode() == 35 || (&*op).opcode() == 36); //IrOpcode::kJSSetKeyedProperty, IrOpcode::kJSStoreInArrayLiteral, IrOpcode::kJSDefineKeyedOwnPropertyInLiteral, IrOpcode::kJSDefineKeyedOwnProperty
            if let Some(node) = self.build_deopt_if_feedback_is_insufficient(
                slot,
                effect,
                control,
                DeoptimizeReason::kInsufficientTypeFeedbackForGenericKeyedAccess,
            ) {
                return LoweringResult::exit(node);
            }
        }
        return LoweringResult::no_change();
    }

    fn get_binary_operation_hint(&self, slot: FeedbackSlot) -> BinaryOperationHint {
        unsafe {
            let source = FeedbackSource::new(self.feedback_vector(), slot);
            (&*self.broker_).GetFeedbackForBinaryOperation(source)
        }
    }

    fn get_compare_operation_hint(&self, slot: FeedbackSlot) -> CompareOperationHint {
        unsafe {
            let source = FeedbackSource::new(self.feedback_vector(), slot);
            (&*self.broker_).GetFeedbackForCompareOperation(source)
        }
    }

    fn build_deopt_if_feedback_is_insufficient(
        &self,
        slot: FeedbackSlot,
        effect: *mut Node,
        control: *mut Node,
        reason: DeoptimizeReason,
    ) -> Option<*mut Node> {
        if !(self.flags_ & Flags::from(Self::kBailoutOnUninitialized)).is_empty() {
            return None;
        }

        unsafe {
            let source = FeedbackSource::new(self.feedback_vector(), slot);
            if !(&*self.broker_).FeedbackIsInsufficient(source) {
                return None;
            }

            let deoptimize = (&*(&*self.jsgraph_).graph()).NewNode(
                (&*self.jsgraph_).common().Deoptimize(reason, FeedbackSource::new(FeedbackVectorRef {}, FeedbackSlot {})),
                (&*self.jsgraph_).Dead(),
                effect,
                control,
            );
            let frame_state =
                NodeProperties::find_frame_state_before(deoptimize, (&*self.jsgraph_).Dead());
            (&mut *deoptimize).ReplaceInput(0, frame_state);
            Some(deoptimize)
        }
    }

    fn broker(&self) -> *mut JSHeapBroker {
        self.broker_
    }
    fn jsgraph(&self) -> *mut JSGraph {
        self.jsgraph_
    }
    fn isolate(&self) -> *mut Isolate {
        unsafe { (&*self.jsgraph_).isolate() }
    }
    fn flags(&self) -> Flags<Self::Flag> {
        self.flags_
    }
    fn feedback_vector(&self) -> FeedbackVectorRef {
        self.feedback_vector_
    }

    fn binary_operation_hint_to_number_operation_hint(&self, binop_hint: BinaryOperationHint, number_hint: &mut NumberOperationHint) -> bool {
        match binop_hint {
            BinaryOperationHint::kSignedSmall => {
                *number_hint = NumberOperationHint::kSignedSmall;
                return true;
            }
            BinaryOperationHint::kSignedSmallInputs => {
                *number_hint = NumberOperationHint::kSignedSmallInputs;
                return true;
            }
            BinaryOperationHint::kAdditiveSafeInteger => {
                *number_hint = NumberOperationHint::kAdditiveSafeInteger;
                return true;
            }
            BinaryOperationHint::kNumber => {
                *number_hint = NumberOperationHint::kNumber;
                return true;
            }
            BinaryOperationHint::kNumberOrOddball => {
                *number_hint = NumberOperationHint::kNumberOrOddball;
                return true;
            }
            BinaryOperationHint::kAny
            | BinaryOperationHint::kNone
            | BinaryOperationHint::kString
            | BinaryOperationHint::kStringOrStringWrapper
            | BinaryOperationHint::kBigInt
            | BinaryOperationHint::kBigInt64 => {
                return false;
            }
        }
    }
}

impl FeedbackSlot {
    pub fn IsInvalid(&self) -> bool {
        true
    }
}

struct JSSpeculativeBinopBuilder<'a> {
    lowering_: &'a JSTypeHintLowering,
    op_: *const Operator,
    left_: *mut Node,
    right_: *mut Node,
    effect_: *mut Node,
    control_: *mut Node,
    slot_: FeedbackSlot,
}

impl<'a> JSSpeculativeBinopBuilder<'a> {
    fn new(
        lowering_: &'a JSTypeHintLowering,
        op_: *const Operator,
        left_: *mut Node,
        right_: *mut Node,
        effect_: *mut Node,
        control_: *mut Node,
        slot_: FeedbackSlot,
    ) -> Self {
        JSSpeculativeBinopBuilder {
            lowering_: lowering_,
            op_: op_,
            left_: left_,
            right_: right_,
            effect_: effect_,
            control_: control_,
            slot_: slot_,
        }
    }

    fn get_binary_number_operation_hint(&self, hint: &mut NumberOperationHint) -> bool {
        self.lowering_.binary_operation_hint_to_number_operation_hint(
            self.get_binary_operation_hint(),
            hint,
        )
    }

    fn get_binary_big_int_operation_hint(&self, hint: &mut BigIntOperationHint) -> bool {
        match self.get_binary_operation_hint() {
            BinaryOperationHint::kBigInt64 => {
                *hint = BigIntOperationHint::kBigInt64;
                return true;
            }
            BinaryOperationHint::kBigInt => {
                *hint = BigIntOperationHint::kBigInt;
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn get_compare_number_operation_hint(&self, hint: &mut NumberOperationHint) -> bool {
        match self.get_compare_operation_hint() {
            CompareOperationHint::kSignedSmall => {
                *hint = NumberOperationHint::kSignedSmall;
                return true;
            }
            CompareOperationHint::kNumber => {
                *hint = NumberOperationHint::kNumber;
                return true;
            }
            CompareOperationHint::kNumberOrBoolean => {
                *hint = NumberOperationHint::kNumberOrBoolean;
                return true;
            }
            CompareOperationHint::kNumberOrOddball => {
                *hint = NumberOperationHint::kNumberOrOddball;
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn get_compare_big_int_operation_hint(&self, hint: &mut BigIntOperationHint) -> bool {
        match self.get_compare_operation_hint() {
            CompareOperationHint::kBigInt => {
                *hint = BigIntOperationHint::kBigInt;
                return true;
            }
            CompareOperationHint::kBigInt64 => {
                *hint = BigIntOperationHint::kBigInt64;
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn speculative_number_op(&self, hint: NumberOperationHint) -> *const Operator {
        unsafe {
            match (&*self.op_).opcode() {
                17 => { //IrOpcode::kJSAdd
                    if hint == NumberOperationHint::kSignedSmall {
                        return (&*self.lowering_.jsgraph()).simplified().SpeculativeSmallIntegerAdd(hint);
                    } else if hint == NumberOperationHint::kAdditiveSafeInteger {
                        return (&*self.lowering_.jsgraph()).simplified().SpeculativeAdditiveSafeIntegerAdd(hint);
                    } else {
                        return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberAdd(hint);
                    }
                }
                18 => { //IrOpcode::kJSSubtract
                    if hint == NumberOperationHint::kSignedSmall {
                        return (&*self.lowering_.jsgraph()).simplified().SpeculativeSmallIntegerSubtract(hint);
                    } else if hint == NumberOperationHint::kAdditiveSafeInteger {
                        return (&*self.lowering_.jsgraph()).simplified().SpeculativeAdditiveSafeIntegerSubtract(hint);
                    } else {
                        return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberSubtract(hint);
                    }
                }
                19 => { //IrOpcode::kJSMultiply
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberMultiply(hint);
                }
                21 => { //IrOpcode::kJSExponentiate
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberPow(hint);
                }
                20 => { //IrOpcode::kJSDivide
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberDivide(hint);
                }
                22 => { //IrOpcode::kJSModulus
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberModulus(hint);
                }
                14 => { //IrOpcode::kJSBitwiseAnd
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberBitwiseAnd(hint);
                }
                12 => { //IrOpcode::kJSBitwiseOr
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberBitwiseOr(hint);
                }
                13 => { //IrOpcode::kJSBitwiseXor
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberBitwiseXor(hint);
                }
                15 => { //IrOpcode::kJSShiftLeft
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberShiftLeft(hint);
                }
                16 => { //IrOpcode::kJSShiftRight
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberShiftRight(hint);
                }
                23 => { //IrOpcode::kJSShiftRightLogical
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeNumberShiftRightLogical(hint);
                }
                _ => {
                    panic!("UNREACHABLE");
                }
            }
        }
    }

    fn speculative_big_int_op(&self, hint: BigIntOperationHint) -> *const Operator {
        unsafe {
            match (&*self.op_).opcode() {
                17 => { //IrOpcode::kJSAdd
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeBigIntAdd(hint);
                }
                18 => { //IrOpcode::kJSSubtract
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeBigIntSubtract(hint);
                }
                19 => { //IrOpcode::kJSMultiply
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeBigIntMultiply(hint);
                }
                20 => { //IrOpcode::kJSDivide
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeBigIntDivide(hint);
                }
                22 => { //IrOpcode::kJSModulus
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeBigIntModulus(hint);
                }
                14 => { //IrOpcode::kJSBitwiseAnd
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeBigIntBitwiseAnd(hint);
                }
                12 => { //IrOpcode::kJSBitwiseOr
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeBigIntBitwiseOr(hint);
                }
                13 => { //IrOpcode::kJSBitwiseXor
                    return (&*self.lowering_.jsgraph()).simplified().SpeculativeBigIntBitwiseXor(hint);
                
