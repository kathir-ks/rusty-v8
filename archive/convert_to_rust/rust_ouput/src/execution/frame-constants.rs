// Converted from V8 C++ source files:
// Header: frame-constants.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub const kSystemPointerSize: i32 = 8;
pub const kPCOnStackSize: i32 = kSystemPointerSize;
pub const kFPOnStackSize: i32 = kSystemPointerSize;

pub struct CommonFrameConstants {}

impl CommonFrameConstants {
    pub const kCallerFPOffset: i32 = 0 * kSystemPointerSize;
    pub const kCallerPCOffset: i32 = Self::kCallerFPOffset + 1 * kFPOnStackSize;
    pub const kCallerSPOffset: i32 = Self::kCallerPCOffset + 1 * kPCOnStackSize;

    pub const kFixedFrameSizeAboveFp: i32 = kPCOnStackSize + kFPOnStackSize;
    pub const kFixedSlotCountAboveFp: i32 = Self::kFixedFrameSizeAboveFp / kSystemPointerSize;
    pub const kCPSlotSize: i32 = if true { kSystemPointerSize } else { 0 };
    pub const kCPSlotCount: i32 = Self::kCPSlotSize / kSystemPointerSize;
    pub const kConstantPoolOffset: i32 = if Self::kCPSlotSize != 0 { -1 * kSystemPointerSize } else { 0 };
    pub const kContextOrFrameTypeSize: i32 = kSystemPointerSize;
    pub const kContextOrFrameTypeOffset: i32 = -(Self::kCPSlotSize + Self::kContextOrFrameTypeSize);
}

pub struct StandardFrameConstants {}

impl StandardFrameConstants {
    pub const kFixedFrameSizeFromFp: i32 = 3 * kSystemPointerSize + CommonFrameConstants::kCPSlotSize;
    pub const kFixedFrameSize: i32 = CommonFrameConstants::kFixedFrameSizeAboveFp + Self::kFixedFrameSizeFromFp;
    pub const kFixedSlotCountFromFp: i32 = Self::kFixedFrameSizeFromFp / kSystemPointerSize;
    pub const kFixedSlotCount: i32 = Self::kFixedFrameSize / kSystemPointerSize;
    pub const kContextOffset: i32 = CommonFrameConstants::kContextOrFrameTypeOffset;
    pub const kFunctionOffset: i32 = -2 * kSystemPointerSize - CommonFrameConstants::kCPSlotSize;
    pub const kArgCOffset: i32 = -3 * kSystemPointerSize - CommonFrameConstants::kCPSlotSize;
    pub const kExpressionsOffset: i32 = -4 * kSystemPointerSize - CommonFrameConstants::kCPSlotSize;
    pub const kFirstPushedFrameValueOffset: i32 = Self::kExpressionsOffset;
    pub const kLastObjectOffset: i32 = Self::kContextOffset;
}

pub struct TypedFrameConstants {}

impl TypedFrameConstants {
    pub const kFrameTypeSize: i32 = CommonFrameConstants::kContextOrFrameTypeSize;
    pub const kFrameTypeOffset: i32 = CommonFrameConstants::kContextOrFrameTypeOffset;
    pub const kFixedFrameSizeFromFp: i32 = CommonFrameConstants::kCPSlotSize + Self::kFrameTypeSize;
    pub const kFixedSlotCountFromFp: i32 = Self::kFixedFrameSizeFromFp / kSystemPointerSize;
    pub const kFixedFrameSize: i32 = StandardFrameConstants::kFixedFrameSizeAboveFp + Self::kFixedFrameSizeFromFp;
    pub const kFixedSlotCount: i32 = Self::kFixedFrameSize / kSystemPointerSize;
    pub const kFirstPushedFrameValueOffset: i32 = -Self::kFixedFrameSizeFromFp - kSystemPointerSize;
}

pub fn frame_pushed_value_offset(parent_first_pushed_frame_value_offset: i32, x: i32) -> i32 {
    parent_first_pushed_frame_value_offset - (x) * kSystemPointerSize
}

pub fn frame_size(parent_fixed_frame_size: i32, count: i32) -> i32 {
    parent_fixed_frame_size + (count) * kSystemPointerSize
}

pub fn frame_size_from_fp(parent_fixed_frame_size_from_fp: i32, count: i32) -> i32 {
    parent_fixed_frame_size_from_fp + (count) * kSystemPointerSize
}

macro_rules! define_frame_sizes {
    ($parent:ident, $count:expr) => {
        const kFixedFrameSize: i32 = frame_size($parent::kFixedFrameSize, $count);
        const kFixedSlotCount: i32 = Self::kFixedFrameSize / kSystemPointerSize;
        const kFixedFrameSizeFromFp: i32 = frame_size_from_fp($parent::kFixedFrameSizeFromFp, $count);
        const kFixedSlotCountFromFp: i32 = Self::kFixedFrameSizeFromFp / kSystemPointerSize;
        const kFirstPushedFrameValueOffset: i32 =
            $parent::kFirstPushedFrameValueOffset - ($count) * kSystemPointerSize;

        impl Self {
            const EXTRA_SLOTS_COUNT: i32 = Self::kFixedSlotCount - $parent::kFixedSlotCount;
            const K_EXTRA_SLOT_COUNT: i32 = Self::kFixedSlotCount - $parent::kFixedSlotCount;

            fn get_extra_slots_count_from<TParentFrameConstants>() -> i32 {
                Self::kFixedSlotCount - TParentFrameConstants::kFixedSlotCount
            }

        }
    };
}

macro_rules! standard_frame_extra_pushed_value_offset {
    ($x:expr) => {
        frame_pushed_value_offset(StandardFrameConstants::kFirstPushedFrameValueOffset, $x)
    };
}

macro_rules! define_standard_frame_sizes {
    ($count:expr) => {
        define_frame_sizes!(StandardFrameConstants, $count);
    };
}

macro_rules! typed_frame_pushed_value_offset {
    ($x:expr) => {
        frame_pushed_value_offset(TypedFrameConstants::kFirstPushedFrameValueOffset, $x)
    };
}

macro_rules! define_typed_frame_sizes {
    ($count:expr) => {
        define_frame_sizes!(TypedFrameConstants, $count);
    };
}

pub struct BuiltinFrameConstants {}

impl BuiltinFrameConstants {
    pub const kFunctionOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kLengthOffset: i32 = typed_frame_pushed_value_offset!(1);
}

define_typed_frame_sizes!(BuiltinFrameConstants, 2);

pub struct ConstructFrameConstants {}

impl ConstructFrameConstants {
    pub const kContextOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kLengthOffset: i32 = typed_frame_pushed_value_offset!(1);
    pub const kConstructorOffset: i32 = typed_frame_pushed_value_offset!(2);
    pub const kPaddingOffset: i32 = typed_frame_pushed_value_offset!(3);
    pub const kNewTargetOrImplicitReceiverOffset: i32 = typed_frame_pushed_value_offset!(4);
}

define_typed_frame_sizes!(ConstructFrameConstants, 5);

impl ConstructFrameConstants {
    pub const kLastObjectOffset: i32 = Self::kContextOffset;
}

pub struct FastConstructFrameConstants {}

impl FastConstructFrameConstants {
    pub const kContextOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kImplicitReceiverOffset: i32 = typed_frame_pushed_value_offset!(1);
}

define_typed_frame_sizes!(FastConstructFrameConstants, 2);

pub struct CWasmEntryFrameConstants {}

impl CWasmEntryFrameConstants {
    pub const kCEntryFPOffset: i32 = typed_frame_pushed_value_offset!(0);
}

define_typed_frame_sizes!(CWasmEntryFrameConstants, 1);

pub struct WasmFrameConstants {}

impl WasmFrameConstants {
    pub const kWasmInstanceDataOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kProtectedInstructionReturnAddressOffset: i32 = 1;
}

define_typed_frame_sizes!(WasmFrameConstants, 1);

pub struct WasmInterpreterFrameConstants {}

impl WasmInterpreterFrameConstants {
    pub const kWasmInstanceObjectOffset: i32 = typed_frame_pushed_value_offset!(0);
}

define_typed_frame_sizes!(WasmInterpreterFrameConstants, 1);

pub struct WasmToJSInterpreterFrameConstants {}

impl WasmToJSInterpreterFrameConstants {
    pub const kGCScanSlotLimitOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kGCSPOffset: i32 = typed_frame_pushed_value_offset!(1);
}

define_typed_frame_sizes!(WasmToJSInterpreterFrameConstants, 2);

pub struct WasmInterpreterCWasmEntryConstants {}

impl WasmInterpreterCWasmEntryConstants {
    pub const kCEntryFPOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kSPFPOffset: i32 = typed_frame_pushed_value_offset!(1);
}

define_typed_frame_sizes!(WasmInterpreterCWasmEntryConstants, 2);

pub struct WasmExitFrameConstants {}

impl WasmExitFrameConstants {
    pub const kCallingPCOffset: i32 = typed_frame_pushed_value_offset!(1);
}

define_typed_frame_sizes!(WasmExitFrameConstants, 2);

impl WasmExitFrameConstants {}

pub struct JSToWasmWrapperFrameConstants {}

impl JSToWasmWrapperFrameConstants {
    pub const kResultArrayParamOffset: i32 = 2 * kSystemPointerSize;
    pub const kImplicitArgOffset: i32 = 3 * kSystemPointerSize;

    pub const kWrapperBufferOffset: i32 = typed_frame_pushed_value_offset!(0);

    pub const kWrapperBufferReturnCount: usize = 0;
    pub const kWrapperBufferRefReturnCount: usize = 4;
    pub const kWrapperBufferSigRepresentationArray: usize = 8;
    pub const kWrapperBufferStackReturnBufferSize: usize = 16;
    pub const kWrapperBufferCallTarget: usize = 24;
    pub const kWrapperBufferParamStart: usize = 32;
    pub const kWrapperBufferParamEnd: usize = 40;

    pub const kWrapperBufferStackReturnBufferStart: usize = 16;
    pub const kWrapperBufferFPReturnRegister1: usize = 24;
    pub const kWrapperBufferFPReturnRegister2: usize = 32;
    pub const kWrapperBufferGPReturnRegister1: usize = 40;
    pub const kWrapperBufferGPReturnRegister2: usize = Self::kWrapperBufferGPReturnRegister1 + kSystemPointerSize as usize;

    pub const kWrapperBufferSize: i32 = Self::kWrapperBufferGPReturnRegister2 as i32 + kSystemPointerSize;
}

define_typed_frame_sizes!(JSToWasmWrapperFrameConstants, 1);

pub struct StackSwitchFrameConstants {}

impl StackSwitchFrameConstants {
    pub const kGCScanSlotCountOffset: i32 = typed_frame_pushed_value_offset!(1);
    pub const kImplicitArgOffset: i32 = typed_frame_pushed_value_offset!(2);
    pub const kResultArrayOffset: i32 = typed_frame_pushed_value_offset!(3);

    pub const kLastSpillOffset: i32 = Self::kResultArrayOffset;
    pub const kNumSpillSlots: i32 = 4;
}

define_typed_frame_sizes!(StackSwitchFrameConstants, 1);

pub struct WasmToJSWrapperConstants {}

impl WasmToJSWrapperConstants {
    pub const kSignatureOffset: i32 = 2 * kSystemPointerSize;
}

pub struct BuiltinWasmInterpreterWrapperConstants {}

impl BuiltinWasmInterpreterWrapperConstants {
    pub const kGCScanSlotCountOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kInParamCountOffset: i32 = typed_frame_pushed_value_offset!(1);
    pub const kParamCountOffset: i32 = typed_frame_pushed_value_offset!(2);
    pub const kReturnCountOffset: i32 = typed_frame_pushed_value_offset!(3);
    pub const kSigRepsOffset: i32 = typed_frame_pushed_value_offset!(4);
    pub const kValueTypesArrayStartOffset: i32 = typed_frame_pushed_value_offset!(5);
    pub const kArgRetsAddressOffset: i32 = typed_frame_pushed_value_offset!(6);
    pub const kArgRetsIsArgsOffset: i32 = typed_frame_pushed_value_offset!(7);
    pub const kCurrentIndexOffset: i32 = typed_frame_pushed_value_offset!(8);
    pub const kSignatureDataOffset: i32 = typed_frame_pushed_value_offset!(9);
}

define_typed_frame_sizes!(BuiltinWasmInterpreterWrapperConstants, 10);

pub struct BuiltinContinuationFrameConstants {}

impl BuiltinContinuationFrameConstants {
    pub const kFunctionOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kFrameSPtoFPDeltaAtDeoptimize: i32 = typed_frame_pushed_value_offset!(1);
    pub const kBuiltinContextOffset: i32 = typed_frame_pushed_value_offset!(2);
    pub const kBuiltinIndexOffset: i32 = typed_frame_pushed_value_offset!(3);

    pub const kArgCOffset: i32 = typed_frame_pushed_value_offset!(4);
}

define_typed_frame_sizes!(BuiltinContinuationFrameConstants, 4);

impl BuiltinContinuationFrameConstants {
    pub fn padding_slot_count(register_count: i32) -> i32 {
        0
    }
}

pub struct ExitFrameConstants {}

impl ExitFrameConstants {
    pub const kSPOffset: i32 = typed_frame_pushed_value_offset!(0);
    pub const kLastExitFrameField: i32 = Self::kSPOffset;

    pub const kCallerSPDisplacement: i32 = CommonFrameConstants::kCallerSPOffset;
}

define_typed_frame_sizes!(ExitFrameConstants, 1);

macro_rules! exit_frame_pushed_value_offset {
    ($x:expr) => {
        frame_pushed_value_offset(ExitFrameConstants::kFirstPushedFrameValueOffset, $x)
    };
}

macro_rules! define_exit_frame_sizes {
    ($x:expr) => {
        define_frame_sizes!(ExitFrameConstants, $x);
    };
}

pub struct BuiltinExitFrameConstants {}

impl BuiltinExitFrameConstants {
    pub const kNewTargetIndex: i32 = 0;
    pub const kTargetIndex: i32 = 1;
    pub const kArgcIndex: i32 = 2;
    pub const kPaddingIndex: i32 = 3;
    pub const kNumExtraArgs: i32 = 4;
    pub const kNumExtraArgsWithReceiver: i32 = Self::kNumExtraArgs + 1;

    pub const kArgumentsArrayOffset: i32 = CommonFrameConstants::kFixedFrameSizeAboveFp;
    pub const kTargetOffset: i32 = Self::kArgumentsArrayOffset + Self::kTargetIndex * kSystemPointerSize;
    pub const kNewTargetOffset: i32 = Self::kArgumentsArrayOffset + Self::kNewTargetIndex * kSystemPointerSize;
    pub const kArgcOffset: i32 = Self::kArgumentsArrayOffset + Self::kArgcIndex * kSystemPointerSize;

    pub const kReceiverOffset: i32 = Self::kArgumentsArrayOffset + Self::kNumExtraArgs * kSystemPointerSize;

    pub const kFirstArgumentOffset: i32 = Self::kReceiverOffset + kSystemPointerSize;
}

pub struct ApiCallbackExitFrameConstants {}

impl ApiCallbackExitFrameConstants {
    pub const kFunctionCallbackInfoContextIndex: i32 = 2;
    pub const kFunctionCallbackInfoReturnValueIndex: i32 = 3;
    pub const kFunctionCallbackInfoTargetIndex: i32 = 4;
    pub const kFunctionCallbackInfoNewTargetIndex: i32 = 5;
    pub const kFunctionCallbackInfoArgsLength: i32 = 6;

    pub const kFCIArgcOffset: i32 = exit_frame_pushed_value_offset!(0);
    pub const kFCIValuesOffset: i32 = exit_frame_pushed_value_offset!(1);
    pub const kFCIImplicitArgsOffset: i32 = exit_frame_pushed_value_offset!(2);

    pub const kFunctionCallbackInfoOffset: i32 = Self::kFCIImplicitArgsOffset;

    pub const kImplicitArgsArrayOffset: i32 = CommonFrameConstants::kFixedFrameSizeAboveFp;
    pub const kTargetOffset: i32 = Self::kImplicitArgsArrayOffset + Self::kFunctionCallbackInfoTargetIndex * kSystemPointerSize;
    pub const kNewTargetOffset: i32 = Self::kImplicitArgsArrayOffset + Self::kFunctionCallbackInfoNewTargetIndex * kSystemPointerSize;
    pub const kContextOffset: i32 = Self::kImplicitArgsArrayOffset + Self::kFunctionCallbackInfoContextIndex * kSystemPointerSize;
    pub const kReturnValueOffset: i32 = Self::kImplicitArgsArrayOffset + Self::kFunctionCallbackInfoReturnValueIndex * kSystemPointerSize;

    pub const kReceiverOffset: i32 = Self::kImplicitArgsArrayOffset + Self::kFunctionCallbackInfoArgsLength * kSystemPointerSize;

    pub const kFirstArgumentOffset: i32 = Self::kReceiverOffset + kSystemPointerSize;
}

define_exit_frame_sizes!(ApiCallbackExitFrameConstants, 3);

pub struct ApiAccessorExitFrameConstants {}

impl ApiAccessorExitFrameConstants {
    pub const kPropertyCallbackInfoPropertyKeyIndex: i32 = 0;
    pub const kPropertyCallbackInfoHolderIndex: i32 = 2;
    pub const kPropertyCallbackInfoReturnValueIndex: i32 = 5;
    pub const kPropertyCallbackInfoReceiverIndex: i32 = 7;
    pub const kPropertyCallbackInfoArgsLength: i32 = 8;

    pub const kArgsArrayOffset: i32 = CommonFrameConstants::kFixedFrameSizeAboveFp;
    pub const kPropertyNameOffset: i32 = Self::kArgsArrayOffset + Self::kPropertyCallbackInfoPropertyKeyIndex * kSystemPointerSize;
    pub const kReturnValueOffset: i32 = Self::kArgsArrayOffset + Self::kPropertyCallbackInfoReturnValueIndex * kSystemPointerSize;
    pub const kReceiverOffset: i32 = Self::kArgsArrayOffset + Self::kPropertyCallbackInfoReceiverIndex * kSystemPointerSize;
    pub const kHolderOffset: i32 = Self::kArgsArrayOffset + Self::kPropertyCallbackInfoHolderIndex * kSystemPointerSize;

    pub const kPropertyCallbackInfoOffset: i32 = Self::kArgsArrayOffset;
}

pub struct UnoptimizedFrameConstants {}

impl UnoptimizedFrameConstants {
    pub const kBytecodeArrayFromFp: i32 = standard_frame_extra_pushed_value_offset!(0);
    pub const kBytecodeOffsetOrFeedbackCellFromFp: i32 = standard_frame_extra_pushed_value_offset!(1);
    pub const kFeedbackVectorFromFp: i32 = standard_frame_extra_pushed_value_offset!(2);
}

define_standard_frame_sizes!(UnoptimizedFrameConstants, 3);

impl UnoptimizedFrameConstants {
    pub const kFirstParamFromFp: i32 = StandardFrameConstants::kCallerSPOffset;
    pub const kRegisterFileFromFp: i32 = -Self::kFixedFrameSizeFromFp - kSystemPointerSize;
    pub const kExpressionsOffset: i32 = Self::kRegisterFileFromFp;

    pub const kBytecodeArrayExpressionIndex: i32 = -3;
    pub const kBytecodeOffsetOrFeedbackCellExpressionIndex: i32 = -2;
    pub const kFeedbackVectorExpressionIndex: i32 = -1;
    pub const kRegisterFileExpressionIndex: i32 = 0;

    pub fn register_stack_slot_count(register_count: i32) -> i32 {
        0
    }
}

pub struct InterpreterFrameConstants {}

impl InterpreterFrameConstants {
    pub const kBytecodeOffsetExpressionIndex: i32 = Self::kBytecodeOffsetOrFeedbackCellExpressionIndex;
    pub const kBytecodeOffsetFromFp: i32 = Self::kBytecodeOffsetOrFeedbackCellFromFp;
}

impl InterpreterFrameConstants {}

pub struct BaselineFrameConstants {}

impl BaselineFrameConstants {
    pub const kFeedbackCellExpressionIndex: i32 = Self::kBytecodeOffsetOrFeedbackCellExpressionIndex;
    pub const kFeedbackCellFromFp: i32 = Self::kBytecodeOffsetOrFeedbackCellFromFp;
}

impl BaselineFrameConstants {}

pub fn fp_offset_to_frame_slot(frame_offset: i32) -> i32 {
    StandardFrameConstants::kFixedSlotCountAboveFp - 1 - frame_offset / kSystemPointerSize
}

pub fn frame_slot_to_fp_offset(slot: i32) -> i32 {
    (StandardFrameConstants::kFixedSlotCountAboveFp - 1 - slot) * kSystemPointerSize
}
