// Converted from V8 C++ source files:
// Header: macro-assembler-arm.h
// Implementation: macro-assembler-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/codegen/arm/macro-assembler-arm.rs
#![allow(non_snake_case)]
use std::rc::Rc;
use std::sync::Arc;

use crate::codegen::arm::assembler_arm::*;
use crate::codegen::bailout_reason::AbortReason;
use crate::codegen::macro_assembler_base::*;
use crate::common::globals::*;
use crate::execution::frame_constants::*;
use crate::interpreter::interpreter_assembler::*;
use crate::objects::tagged_index::*;

pub enum StackLimitKind {
    kInterruptStackLimit,
    kRealStackLimit,
}

pub fn FieldMemOperand(object: Register, offset: i32) -> MemOperand {
    MemOperand(object, offset - kHeapObjectTag)
}

pub enum LinkRegisterStatus {
    kLRHasNotBeenSaved,
    kLRHasBeenSaved,
}

pub fn GetRegisterThatIsNotOneOf(
    reg1: Register,
    reg2: Option<Register>,
    reg3: Option<Register>,
    reg4: Option<Register>,
    reg5: Option<Register>,
    reg6: Option<Register>,
) -> Register {
    let regs = vec![
        Some(reg1),
        reg2,
        reg3,
        reg4,
        reg5,
        reg6,
    ];

    let config = RegisterConfiguration::default();
    for i in 0..config.num_allocatable_general_registers() {
        let code = config.get_allocatable_general_code(i);
        let candidate = Register::from_code(code);
        if regs.iter().any(|&r| r == Some(candidate)) {
            continue;
        }
        return candidate;
    }
    unreachable!()
}

pub enum TargetAddressStorageMode {
    CAN_INLINE_TARGET_ADDRESS,
    NEVER_INLINE_TARGET_ADDRESS,
}

pub struct MacroAssembler {
    base: MacroAssemblerBase,
}

impl MacroAssembler {
    pub fn new(base: MacroAssemblerBase) -> Self {
        Self { base }
    }

    pub fn base(&self) -> &MacroAssemblerBase {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut MacroAssemblerBase {
        &mut self.base
    }

    pub fn enter_frame(&mut self, type_: StackFrame::Type, load_constant_pool_pointer_reg: bool) {
        todo!()
    }

    pub fn leave_frame(&mut self, type_: StackFrame::Type) -> i32 {
        todo!()
    }

    #[cfg(target_os = "windows")]
    pub fn allocate_stack_space(&mut self, bytes_scratch: Register) {
        todo!()
    }

    #[cfg(not(target_os = "windows"))]
    pub fn allocate_stack_space(&mut self, bytes: Register) {
        self.base_mut().sub(sp, sp, bytes);
    }

    #[cfg(target_os = "windows")]
    pub fn allocate_stack_space(&mut self, bytes: i32) {
        todo!()
    }

    #[cfg(not(target_os = "windows"))]
    pub fn allocate_stack_space(&mut self, bytes: i32) {
        todo!()
    }

    pub fn push_common_frame(&mut self, marker_reg: Register) {
        todo!()
    }

    pub fn stub_prologue(&mut self, type_: StackFrame::Type) {
        todo!()
    }

    pub fn prologue(&mut self) {
        todo!()
    }

    pub fn drop_arguments(&mut self, count: Register) {
        todo!()
    }

    pub fn drop_arguments_and_push_new_receiver(&mut self, argc: Register, receiver: Register) {
        todo!()
    }

    pub fn push_standard_frame(&mut self, function_reg: Register) {
        todo!()
    }

    pub fn initialize_root_register(&mut self) {
        todo!()
    }

    pub fn push(&mut self, src: Register) {
        self.base_mut().push(src);
    }

    pub fn push_handle(&mut self, handle: Handle<HeapObject>) {
        todo!()
    }

    pub fn push_smi(&mut self, smi: Tagged<Smi>) {
        todo!()
    }

    pub fn push_tagged_index(&mut self, index: Tagged<TaggedIndex>) {
        todo!()
    }

    pub fn push_2(&mut self, src1: Register, src2: Register, cond: Condition) {
        todo!()
    }

    pub fn push_3(&mut self, src1: Register, src2: Register, src3: Register, cond: Condition) {
        todo!()
    }

    pub fn push_4(&mut self, src1: Register, src2: Register, src3: Register, src4: Register, cond: Condition) {
        todo!()
    }

    pub fn push_5(&mut self, src1: Register, src2: Register, src3: Register, src4: Register, src5: Register, cond: Condition) {
        todo!()
    }

    pub enum PushArrayOrder {
        kNormal,
        kReverse,
    }

    pub fn push_array(&mut self, array: Register, size: Register, scratch: Register, order: PushArrayOrder) {
        todo!()
    }

    pub fn pop(&mut self, dst: Register) {
        self.base_mut().pop(dst);
    }

    pub fn pop_2(&mut self, src1: Register, src2: Register, cond: Condition) {
        todo!()
    }

    pub fn pop_3(&mut self, src1: Register, src2: Register, src3: Register, cond: Condition) {
        todo!()
    }

    pub fn pop_4(&mut self, src1: Register, src2: Register, src3: Register, src4: Register, cond: Condition) {
        todo!()
    }

    pub fn prepare_call_c_function(&mut self, num_reg_arguments: i32, num_double_registers: i32, scratch: Register) {
        todo!()
    }

    pub fn mov_to_float_parameter(&mut self, src: DwVfpRegister) {
        todo!()
    }

    pub fn mov_to_float_parameters(&mut self, src1: DwVfpRegister, src2: DwVfpRegister) {
        todo!()
    }

    pub fn mov_to_float_result(&mut self, src: DwVfpRegister) {
        todo!()
    }

    pub fn call_c_function_external_reference(
        &mut self,
        function: ExternalReference,
        num_arguments: i32,
        set_isolate_data_slots: SetIsolateDataSlots,
        return_label: Option<&mut Label>,
    ) -> i32 {
        todo!()
    }

    pub fn call_c_function_register(
        &mut self,
        function: Register,
        num_arguments: i32,
        set_isolate_data_slots: SetIsolateDataSlots,
        return_label: Option<&mut Label>,
    ) -> i32 {
        todo!()
    }

    pub fn call_c_function_external_reference2(
        &mut self,
        function: ExternalReference,
        num_reg_arguments: i32,
        num_double_arguments: i32,
        set_isolate_data_slots: SetIsolateDataSlots,
        return_label: Option<&mut Label>,
    ) -> i32 {
        todo!()
    }

    pub fn call_c_function_register2(
        &mut self,
        function: Register,
        num_reg_arguments: i32,
        num_double_arguments: i32,
        set_isolate_data_slots: SetIsolateDataSlots,
        return_label: Option<&mut Label>,
    ) -> i32 {
        todo!()
    }

    pub fn mov_from_float_parameter(&mut self, dst: DwVfpRegister) {
        todo!()
    }

    pub fn mov_from_float_result(&mut self, dst: DwVfpRegister) {
        todo!()
    }

    pub fn trap(&mut self) {
        todo!()
    }

    pub fn debug_break(&mut self) {
        todo!()
    }

    pub fn assert_cond(&mut self, cond: Condition, reason: AbortReason) {
        todo!()
    }

    pub fn assert_unreachable(&mut self, reason: AbortReason) {
        todo!()
    }

    pub fn check(&mut self, cond: Condition, reason: AbortReason) {
        todo!()
    }

    pub fn abort(&mut self, msg: AbortReason) {
        todo!()
    }

    pub fn lsl_pair_register(&mut self, dst_low: Register, dst_high: Register, src_low: Register, src_high: Register, shift: Register) {
        todo!()
    }

    pub fn lsl_pair_immediate(&mut self, dst_low: Register, dst_high: Register, src_low: Register, src_high: Register, shift: u32) {
        todo!()
    }

    pub fn lsr_pair_register(&mut self, dst_low: Register, dst_high: Register, src_low: Register, src_high: Register, shift: Register) {
        todo!()
    }

    pub fn lsr_pair_immediate(&mut self, dst_low: Register, dst_high: Register, src_low: Register, src_high: Register, shift: u32) {
        todo!()
    }

    pub fn asr_pair_register(&mut self, dst_low: Register, dst_high: Register, src_low: Register, src_high: Register, shift: Register) {
        todo!()
    }

    pub fn asr_pair_immediate(&mut self, dst_low: Register, dst_high: Register, src_low: Register, src_high: Register, shift: u32) {
        todo!()
    }

    pub fn load_from_constants_table(&mut self, destination: Register, constant_index: i32) {
        todo!()
    }

    pub fn load_root_register_offset(&mut self, destination: Register, offset: i64) {
        todo!()
    }

    pub fn load_root_relative(&mut self, destination: Register, offset: i32) {
        todo!()
    }

    pub fn store_root_relative(&mut self, offset: i32, value: Register) {
        todo!()
    }

    pub fn external_reference_as_operand(&mut self, reference: ExternalReference, scratch: Register) -> MemOperand {
        todo!()
    }

    pub fn external_reference_as_operand_id(&mut self, id: IsolateFieldId) -> MemOperand {
        todo!()
    }

    pub fn call(&mut self, target: Register, cond: Condition) {
        todo!()
    }

    pub fn call_address(
        &mut self,
        target: Address,
        rmode: RelocInfo::Mode,
        cond: Condition,
        mode: TargetAddressStorageMode,
        check_constant_pool: bool,
    ) {
        todo!()
    }

    pub fn call_code_handle(
        &mut self,
        code: Handle<Code>,
        rmode: RelocInfo::Mode,
        cond: Condition,
        mode: TargetAddressStorageMode,
        check_constant_pool: bool,
    ) {
        todo!()
    }

    pub fn call_label(&mut self, target: &mut Label) {
        todo!()
    }

    pub fn entry_from_builtin_as_operand(&mut self, builtin: Builtin) -> MemOperand {
        todo!()
    }

    pub fn load_entry_from_builtin(&mut self, builtin: Builtin, destination: Register) {
        todo!()
    }

    pub fn load_entry_from_builtin_index(&mut self, builtin_index: Register, target: Register) {
        todo!()
    }

    pub fn call_builtin_by_index(&mut self, builtin_index: Register, target: Register) {
        todo!()
    }

    pub fn call_builtin(&mut self, builtin: Builtin, cond: Condition) {
        todo!()
    }

    pub fn tail_call_builtin(&mut self, builtin: Builtin, cond: Condition) {
        todo!()
    }

    pub fn jump_if_code_is_marked_for_deoptimization(&mut self, code: Register, scratch: Register, if_marked_for_deoptimization: &mut Label) {
        todo!()
    }

    pub fn jump_if_code_is_turbofanned(&mut self, code: Register, scratch: Register, if_turbofanned: &mut Label) {
        todo!()
    }

    pub fn try_load_optimized_osr_code(&mut self, scratch_and_result: Register, min_opt_level: CodeKind, feedback_vector: Register, slot: FeedbackSlot, on_result: &mut Label, distance: Label::Distance) {
        todo!()
    }

    pub fn jump(&mut self, target: Register, cond: Condition) {
        todo!()
    }

    pub fn jump_address(&mut self, target: Address, rmode: RelocInfo::Mode, cond: Condition) {
        todo!()
    }

    pub fn jump_code_handle(&mut self, code: Handle<Code>, rmode: RelocInfo::Mode, cond: Condition) {
        todo!()
    }

    pub fn jump_label(&mut self, reference: &ExternalReference) {
        todo!()
    }

    pub fn get_label_address(&mut self, dst: Register, target: &mut Label) {
        todo!()
    }

    pub fn compute_code_start_address(&mut self, dst: Register) {
        todo!()
    }

    pub fn code_entry(&mut self) {
        todo!()
    }

    pub fn exception_handler(&mut self) {
        todo!()
    }

    pub fn bind_exception_handler(&mut self, label: &mut Label) {
        todo!()
    }

    pub fn il64x2_bit_mask(&mut self, dst: Register, src: QwNeonRegister) {
        todo!()
    }

    pub fn il64x2_eq(&mut self, dst: QwNeonRegister, src1: QwNeonRegister, src2: QwNeonRegister) {
        todo!()
    }

    pub fn il64x2_ne(&mut self, dst: QwNeonRegister, src1: QwNeonRegister, src2: QwNeonRegister) {
        todo!()
    }

    pub fn il64x2_gt_s(&mut self, dst: QwNeonRegister, src1: QwNeonRegister, src2: QwNeonRegister) {
        todo!()
    }

    pub fn il64x2_ge_s(&mut self, dst: QwNeonRegister, src1: QwNeonRegister, src2: QwNeonRegister) {
        todo!()
    }

    pub fn il64x2_all_true(&mut self, dst: Register, src: QwNeonRegister) {
        todo!()
    }

    pub fn il64x2_abs(&mut self, dst: QwNeonRegister, src: QwNeonRegister) {
        todo!()
    }

    pub fn f64x2_convert_low_i32x4_s(&mut self, dst: QwNeonRegister, src: QwNeonRegister) {
        todo!()
    }

    pub fn f64x2_convert_low_i32x4_u(&mut self, dst: QwNeonRegister, src: QwNeonRegister) {
        todo!()
    }

    pub fn f64x2_promote_low_f32x4(&mut self, dst: QwNeonRegister, src: QwNeonRegister) {
        todo!()
    }

    pub fn mls(&mut self, dst: Register, src1: Register, src2: Register, src_a: Register, cond: Condition) {
        todo!()
    }

    pub fn and(&mut self, dst: Register, src1: Register, src2: Operand, cond: Condition) {
        todo!()
    }

    pub fn ubfx(&mut self, dst: Register, src: Register, lsb: i32, width: i32, cond: Condition) {
        todo!()
    }

    pub fn sbfx(&mut self, dst: Register, src: Register, lsb: i32, width: i32, cond: Condition) {
        todo!()
    }

    pub fn bfc(&mut self, dst: Register, src: Register, lsb: i32, width: i32, cond: Condition) {
        todo!()
    }

    pub fn record_write_field(
        &mut self,
        object: Register,
        offset: i32,
        value: Register,
        lr_status: LinkRegisterStatus,
        save_fp: SaveFPRegsMode,
        smi_check: SmiCheck,
    ) {
        todo!()
    }

    pub fn record_write(
        &mut self,
        object: Register,
        offset: Operand,
        value: Register,
        lr_status: LinkRegisterStatus,
        save_fp: SaveFPRegsMode,
        smi_check: SmiCheck,
    ) {
        todo!()
    }

    pub fn enter_exit_frame(&mut self, scratch: Register, stack_space: i32, frame_type: StackFrame::Type) {
        todo!()
    }

    pub fn leave_exit_frame(&mut self, scratch: Register) {
        todo!()
    }

    pub fn load_global_proxy(&mut self, dst: Register) {
        todo!()
    }

    pub fn load_native_context_slot(&mut self, dst: Register, index: i32) {
        todo!()
    }

    pub fn invoke_function_code(
        &mut self,
        function: Register,
        new_target: Register,
        expected_parameter_count: Register,
        actual_parameter_count: Register,
        type_: InvokeType,
    ) {
        todo!()
    }

    pub fn call_debug_on_function_call(
        &mut self,
        fun: Register,
        new_target: Register,
        expected_parameter_count: Register,
        actual_parameter_count: Register,
    ) {
        todo!()
    }

    pub fn invoke_function_with_new_target(
        &mut self,
        fun: Register,
        new_target: Register,
        actual_parameter_count: Register,
        type_: InvokeType,
    ) {
        todo!()
    }

    pub fn invoke_function(
        &mut self,
        function: Register,
        expected_parameter_count: Register,
        actual_parameter_count: Register,
        type_: InvokeType,
    ) {
        todo!()
    }

    pub fn push_stack_handler(&mut self) {
        todo!()
    }

    pub fn pop_stack_handler(&mut self) {
        todo!()
    }

    pub fn compare_object_type(&mut self, object: Register, map: Register, type_reg: Register, type_: InstanceType) {
        todo!()
    }

    pub fn compare_object_type_range(
        &mut self,
        object: Register,
        map: Register,
        type_reg: Register,
        scratch: Register,
        lower_limit: InstanceType,
        higher_limit: InstanceType,
    ) {
        todo!()
    }

    pub fn compare_instance_type(&mut self, map: Register, type_reg: Register, type_: InstanceType) {
        todo!()
    }

    pub fn compare_range(
        &mut self,
        value: Register,
        scratch: Register,
        lower_limit: u32,
        higher_limit: u32,
    ) {
        todo!()
    }

    pub fn compare_instance_type_range(
        &mut self,
        map: Register,
        type_reg: Register,
        scratch: Register,
        lower_limit: InstanceType,
        higher_limit: InstanceType,
    ) {
        todo!()
    }

    pub fn compare_tagged_root(&mut self, obj: Register, index: RootIndex) {
        todo!()
    }

    pub fn compare_root(&mut self, obj: Register, index: RootIndex) {
        todo!()
    }

    pub fn jump_if_is_in_range(
        &mut self,
        value: Register,
        scratch: Register,
        lower_limit: u32,
        higher_limit: u32,
        on_in_range: &mut Label,
    ) {
        todo!()
    }

    pub fn receiver_operand(&mut self) -> MemOperand {
        todo!()
    }

    pub fn assert_feedback_cell(&mut self, object: Register, scratch: Register) {
        todo!()
    }

    pub fn assert_feedback_vector(&mut self, object: Register, scratch: Register) {
        todo!()
    }

    pub fn replace_closure_code_with_optimized_code(&mut self, optimized_code: Register, closure: Register) {
        todo!()
    }

    pub fn generate_tail_call_to_returned_code(&mut self, function_id: Runtime::FunctionId) {
        todo!()
    }

    pub fn load_feedback_vector_flags_and_check_if_needs_processing(
        &mut self,
        flags: Register,
        feedback_vector: Register,
        current_code_kind: CodeKind,
    ) -> Condition {
        todo!()
    }

    pub fn load_feedback_vector_flags_and_jump_if_needs_processing(
        &mut self,
        flags: Register,
        feedback_vector: Register,
        current_code_kind: CodeKind,
        flags_need_processing: &mut Label,
    ) {
        todo!()
    }

    pub fn optimize_code_or_tail_call_optimized_code_slot(
        &mut self,
        flags: Register,
        feedback_vector: Register,
    ) {
        todo!()
    }

    pub fn call_runtime(&mut self, f: &Runtime::Function, num_arguments: i32) {
        todo!()
    }

    pub fn call_runtime_function_id(&mut self, fid: Runtime::FunctionId) {
        todo!()
    }

    pub fn call_runtime_function_id_2(&mut self, fid: Runtime::FunctionId, num_arguments: i32) {
        todo!()
    }

    pub fn tail_call_runtime(&mut self, fid: Runtime::FunctionId) {
        todo!()
    }

    pub fn jump_to_external_reference(&mut self, builtin: &ExternalReference, builtin_exit_frame: bool) {
        todo!()
    }

    pub fn load_weak_value(&mut self, out: Register, in_: Register, target_if_cleared: &mut Label) {
        todo!()
    }

    pub fn increment_counter(&mut self, counter: &mut StatsCounter, value: i32, scratch1: Register, scratch2: Register) {
        todo!()
    }

    pub fn emit_increment_counter(&mut self, counter: &mut StatsCounter, value: i32, scratch1: Register, scratch2: Register) {
        todo!()
    }

    pub fn decrement_counter(&mut self, counter: &mut StatsCounter, value: i32, scratch1: Register, scratch2: Register) {
        todo!()
    }

    pub fn emit_decrement_counter(&mut self, counter: &mut StatsCounter, value: i32, scratch1: Register, scratch2: Register) {
        todo!()
    }

    pub fn load_stack_limit(&mut self, destination: Register, kind: StackLimitKind) {
        todo!()
    }

    pub fn stack_overflow_check(&mut self, num_args: Register, scratch: Register, stack_overflow: &mut Label) {
        todo!()
    }

    pub fn smi_tag(&mut self, reg: Register, s: SBit) {
        todo!()
    }

    pub fn smi_tag_2(&mut self, dst: Register, src: Register, s: SBit) {
        todo!()
    }

    pub fn smi_tst(&mut self, value: Register) {
        todo!()
    }

    pub fn jump_if_smi(&mut self, value: Register, smi_label: &mut Label) {
        todo!()
    }

    pub fn jump_if_equal(&mut self, x: Register, y: i32, dest: &mut Label) {
        todo!()
    }

    pub fn jump_if_less_than(&mut self, x: Register, y: i32, dest: &mut Label) {
        todo!()
    }

    pub fn jump_if_not_smi(&mut self, value: Register, not_smi_label: &mut Label) {
        todo!()
    }

    pub fn zero(&mut self, dest: MemOperand) {
        todo!()
    }

    pub fn zero_2(&mut self, dest1: MemOperand, dest2: MemOperand) {
        todo!()
    }

    pub fn decompress_tagged(&mut self, destination: Register, field_operand: MemOperand) {
        todo!()
    }

    pub fn decompress_tagged2(&mut self, destination: Register, source: Register) {
        todo!()
    }

    pub fn smi_untag_field(&mut self, dst: Register, src: MemOperand) {
        todo!()
    }

    pub fn load_tagged_field(&mut self, destination: Register, field_operand: MemOperand) {
        todo!()
    }

    pub fn load_tagged_field_without_decompressing(&mut self, destination: Register, field_operand: MemOperand) {
        todo!()
    }

    pub fn store_tagged_field(&mut self, value: Register, dst_field_operand: MemOperand) {
        todo!()
    }

    pub fn store_tagged_field2(&mut self, dst_field_operand: MemOperand, value: Register) {
        todo!()
    }

    pub fn switch(&mut self, scratch: Register, value: Register, case_value_base: i32, labels: &mut [*mut Label], num_labels: i32) {
        todo!()
    }

    pub fn bfc(&mut self, dst: Register, src: Register, lsb: i32, width: i32, cond: Condition) {
        todo!()
    }

    pub fn assert_zero_extended(&mut self, int32_register: Register) {
        todo!()
    }

    pub fn test_code_is_marked_for_deoptimization(&mut self, code: Register, scratch: Register) {
        todo!()
    }

    pub fn assert_map(&mut self, object: Register) {
        todo!()
    }

    pub fn assert_not_smi(&mut self, object: Register, reason: AbortReason) {
        todo!()
    }

    pub fn assert_smi(&mut self, object: Register, reason: AbortReason) {
        todo!()
    }

    pub fn assert_constructor(&mut self, object: Register) {
        todo!()
    }

    pub fn assert_function(&mut self, object: Register) {
        todo!()
    }

    pub fn assert_callable_function(&mut self, object: Register) {
        todo!()
    }

    pub fn assert_bound_function(&mut self, object: Register) {
        todo!()
    }

    pub fn assert_generator_object(&mut self, object: Register) {
        todo!()
    }

    pub fn assert_undefined_or_allocation_site(&mut self, object: Register, scratch: Register) {
        todo!()
    }

    pub fn assert_js_any(&mut self, object: Register, map_tmp: Register, tmp: Register, abort_reason: AbortReason) {
        todo!()
    }

    pub fn decode_field_bytefield_64<const OFFSET: usize, const SIZE: usize>(&mut self, dst: Register, src: Register) {
        todo!()
    }

    pub fn decode_field_bytefield_64_2<const OFFSET: usize, const SIZE: usize>(&mut self, reg: Register) {
        todo!()
    }

    pub fn has_pending_constants(&self) -> bool {
        todo!()
    }
}

pub struct MoveCycleState {
    pub scratch_v_reglist: VfpRegList,
    pub temps: Option<UseScratchRegisterScope>,
    pub scratch_reg_code: i32,
}

impl MoveCycleState {
	fn dummy_method(&self) {}
}

pub fn ExitFrameStackSlotOperand(offset: i32) -> MemOperand {
    let kSPOffset: i32 = 1 * kPointerSize;
    MemOperand(sp, kSPOffset + offset)
}

pub fn ExitFrameCallerStackSlotOperand(index: i32) -> MemOperand {
    MemOperand(fp, (BuiltinExitFrameConstants::kFixedSlotCountAboveFp + index) * kSystemPointerSize)
}

// Calls an API function. Allocates HandleScope, extracts returned value
// from handle and propagates exceptions. Clobbers C argument registers
// and C caller-saved registers. Restores context. On return removes
//   (*argc_operand + slots_to_drop_on_return) * kSystemPointerSize
// (GCed, includes the call JS arguments space and the additional space
// allocated for the fast call).
fn CallApiFunctionAndReturn(
    masm: &mut MacroAssembler,
    with_profiling: bool,
    function_address: Register,
    thunk_ref: ExternalReference,
    thunk_arg: Register,
    slots_to_drop_on_return: i32,
    argc_operand: &mut MemOperand,
    return_value_operand: &mut MemOperand,
) {
    todo!()
}

struct RegisterArray {}

impl RegisterArray {
	fn dummy_method(&self) {}
}

impl CodeEntrypointTag {
	fn dummy_method(&self) {}
}

pub enum Call {
    kCall,
    kJump,
}

pub struct Javascript {
	instruction : i32
}

impl Javascript {
	fn dummy_method(&self) {}
}

pub struct List {}

impl List {
	fn dummy_method(&self) {}
}

pub struct platform {
	instruction : i32
}

impl platform {
	fn dummy_method(&self) {}
}

pub struct SaveFPRegsMode {}

impl SaveFPRegsMode {
	fn dummy_method(&self) {}
}

pub struct CodeKind {}

impl CodeKind {
	fn dummy_method(&self) {}
}

pub struct ExternalReference {}

impl ExternalReference {
	fn dummy_method(&self) {}
}

pub struct Operand {}

impl Operand {
	fn dummy_method(&self) {}
}

pub struct RootIndex {}

impl RootIndex {
	fn dummy_method(&self) {}
}

pub struct FeedbackSlot {}

impl FeedbackSlot {
	fn dummy_method(&self) {}
}

pub struct Immediate {}

impl Immediate {
	fn dummy_method(&self) {}
}

pub struct External {}

impl External {
	fn dummy_method(&self) {}
}

pub enum JumpMode {
	kValue,
}

pub enum CallJumpMode {
    kCall,
    kTailCall,
}

pub struct HeapObject {}

impl HeapObject {
	fn dummy_method(&self) {}
}

pub struct CodeEntrypointTag {}

impl CodeEntrypointTag {
	fn dummy_method(&self) {}
}

pub enum SmiCheck {
    kInline,
    kOmit,
}

pub struct TVARIABLE<'a, T> {
	variable_use : i32,
	variable : i32,
	is_used : bool,
	v:i32,
	dummy : i32,
	f : i32,
	t: i32,
	reference : i32,
	list: Vec<String>,
	v8: Vec<i32>,
	string: String,
	element : i32,
    _phantom: std::marker::PhantomData<&'a T>,

}
