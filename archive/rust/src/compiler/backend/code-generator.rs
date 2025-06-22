// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod code_generator {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::vec::Vec;

    //use crate::codegen::macro_assembler::MacroAssembler;  // Assuming a Rust equivalent exists
    //use crate::codegen::optimized_compilation_info::OptimizedCompilationInfo; // Assuming a Rust equivalent exists
    //use crate::codegen::safepoint_table::SafepointTableBuilder; // Assuming a Rust equivalent exists
    //use crate::codegen::source_position_table::SourcePositionTableBuilder; // Assuming a Rust equivalent exists
    //use crate::compiler::backend::gap_resolver::GapResolver; // Assuming a Rust equivalent exists
    //use crate::compiler::backend::instruction::Instruction; // Assuming a Rust equivalent exists
    //use crate::compiler::backend::unwinding_info_writer::UnwindingInfoWriter; // Assuming a Rust equivalent exists
    //use crate::compiler::osr::OsrHelper; // Assuming a Rust equivalent exists
    //use crate::deoptimizer::deoptimizer::Deoptimizer; // Assuming a Rust equivalent exists
    //use crate::objects::code_kind::CodeKind; // Assuming a Rust equivalent exists
    //use crate::objects::deoptimization_data::DeoptimizationData; // Assuming a Rust equivalent exists

    //use crate::trap_handler::trap_handler; // Assuming a Rust equivalent exists

    // Forward declarations (using dummy types for now)
    pub struct DeoptimizationExit {}
    pub struct FrameAccessState {}
    pub struct Linkage {}
    pub struct OutOfLineCode {}
    pub struct Frame {}
    pub struct InstructionSequence {}
    pub struct OptimizedCompilationInfo {}
    pub struct Isolate {}
    pub struct JumpOptimizationInfo {}
    pub struct AssemblerOptions {}
    pub struct InstructionBlock {}
    pub struct InstructionOperand {}
    pub struct ReferenceMap {}
    pub struct MacroAssembler {}
    pub struct SafepointTableBuilder {}
    pub struct SourcePosition {}
    pub struct Handle<T> {}
    pub struct HeapObject {}
    pub struct Zone {}
    pub struct RpoNumber {}
    pub struct RootIndex {}
    pub struct MoveOperands {}
    pub struct FrameStateDescriptor {}
    pub struct StateValueDescriptor {}
    pub struct StateValueList {}
    pub struct IndirectHandle<T> {}
    pub struct DeoptimizationLiteral {}
    pub struct FrameTranslationBuilder {}
    pub struct GapResolver {}
    pub struct JumpTable {}
    pub struct MoveCycleState {}
    pub struct Builtin {}
    pub struct ZoneVector<T> {
        data: Vec<T>,
    }
    
    impl <T> ZoneVector<T> {
        pub fn new() -> Self {
            ZoneVector { data: Vec::new() }
        }
    }
    pub struct ZoneDeque<T> {
        data: Vec<T>,
    }
    
    impl <T> ZoneDeque<T> {
        pub fn new() -> Self {
            ZoneDeque { data: Vec::new() }
        }
    }
    pub struct base {
        // need to implement Flags and OwnedVector
    }
    
    #[derive(Clone, Copy)]
    pub enum FlagsCondition {
        // Example variant, add more as needed
        kUnconditional,
    }

    pub struct Label {}

    impl Label {
        pub fn new() -> Self {
            Label {}
        }
    }

    #[derive(Clone, Copy)]
    pub enum MachineRepresentation {
        // Example variant, add more as needed
        Tagged,
    }
    #[derive(Clone, Copy)]
    pub enum MachineType {
        // Example variant, add more as needed
        Any,
    }
    #[derive(Clone, Copy)]
    pub enum OutputFrameStateCombine {
        // Example variant, add more as needed
        PushOutput,
    }
    
    #[derive(Clone, Copy)]
    pub enum SaveFPRegsMode {
        // Example variant, add more as needed
        Save,
    }

    pub struct BranchInfo {
        pub condition: FlagsCondition,
        pub true_label: *mut Label,
        pub false_label: *mut Label,
        pub fallthru: bool,
    }

    pub struct InstructionOperandIterator<'a> {
        instr_: &'a Instruction,
        pos_: usize,
    }

    impl<'a> InstructionOperandIterator<'a> {
        pub fn new(instr: &'a Instruction, pos: usize) -> Self {
            InstructionOperandIterator { instr_: instr, pos_: pos }
        }

        pub fn instruction(&self) -> &Instruction {
            self.instr_
        }

        pub fn advance(&mut self) -> &InstructionOperand {
            self.instr_.input_at(self.pos_).unwrap()
        }
    }

    #[derive(Default)]
    pub struct TurbolizerCodeOffsetsInfo {
        pub code_start_register_check: i32,
        pub deopt_check: i32,
        pub blocks_start: i32,
        pub out_of_line_code: i32,
        pub deoptimization_exits: i32,
        pub pools: i32,
        pub jump_tables: i32,
    }
    #[derive(Default)]
    pub struct TurbolizerInstructionStartInfo {
        pub gap_pc_offset: i32,
        pub arch_instr_pc_offset: i32,
        pub condition_pc_offset: i32,
    }

    pub struct CodeGenerator {
        zone_: *mut Zone,
        isolate_: *mut Isolate,
        frame_access_state_: *mut FrameAccessState,
        linkage_: *mut Linkage,
        instructions_: *mut InstructionSequence,
        unwinding_info_writer_: UnwindingInfoWriter,
        info_: *mut OptimizedCompilationInfo,
        labels_: Vec<Label>, // Changed from Label* const labels_;
        return_label_: Label,
        current_block_: RpoNumber,
        start_source_position_: SourcePosition,
        current_source_position_: SourcePosition,
        masm_: MacroAssembler,
        resolver_: GapResolver,
        safepoints_: SafepointTableBuilder,
        handlers_: Vec<HandlerInfo>,
        next_deoptimization_id_: i32,
        deopt_exit_start_offset_: i32,
        eager_deopt_count_: i32,
        lazy_deopt_count_: i32,
        deoptimization_exits_: ZoneDeque<DeoptimizationExit>,
        protected_deoptimization_literals_: ZoneDeque<IndirectHandle<HeapObject>>,
        deoptimization_literals_: ZoneDeque<DeoptimizationLiteral>,
        inlined_function_count_: usize,
        translations_: FrameTranslationBuilder,
        handler_table_offset_: usize,
        jump_deoptimization_entry_labels_: [Label; 2], // Assuming kDeoptimizeKindCount = 2
        max_unoptimized_frame_height_: usize,
        max_pushed_argument_count_: usize,
        parameter_count_: u16,
        caller_registers_saved_: bool,
        fp_mode_: SaveFPRegsMode,
        jump_tables_: *mut JumpTable,
        ools_: *mut OutOfLineCode,
        osr_helper_: Option<OsrHelper>,
        osr_pc_offset_: i32,
        source_position_table_builder_: SourcePositionTableBuilder,
        protected_instructions_: ZoneVector<i32>, // Assuming trap_handler::ProtectedInstructionData is just i32
        result_: CodeGenResult,
        block_starts_: ZoneVector<i32>,
        offsets_info_: TurbolizerCodeOffsetsInfo,
        instr_starts_: ZoneVector<TurbolizerInstructionStartInfo>,
        move_cycle_: MoveCycleState,
        debug_name_: *const char,
    }
    

    impl CodeGenerator {
        pub fn new(
            codegen_zone: *mut Zone,
            frame: *mut Frame,
            linkage: *mut Linkage,
            instructions: *mut InstructionSequence,
            info: *mut OptimizedCompilationInfo,
            isolate: *mut Isolate,
            osr_helper: Option<OsrHelper>,
            start_source_position: i32,
            jump_opt: *mut JumpOptimizationInfo,
            options: &AssemblerOptions,
            builtin: Builtin,
            max_unoptimized_frame_height: usize,
            max_pushed_argument_count: usize,
            debug_name: *const char,
        ) -> Self {
            let num_blocks = 10; // Placeholder, determine the actual size

            CodeGenerator {
                zone_: codegen_zone,
                isolate_: isolate,
                frame_access_state_: std::ptr::null_mut(), // Initialize properly later
                linkage_: linkage,
                instructions_: instructions,
                unwinding_info_writer_: UnwindingInfoWriter {}, // Initialize properly
                info_: info,
                labels_: (0..num_blocks).map(|_| Label::new()).collect(), // Initialize based on number of blocks
                return_label_: Label::new(),
                current_block_: RpoNumber {},       // Initialize properly
                start_source_position_: SourcePosition {}, // Initialize properly
                current_source_position_: SourcePosition {}, // Initialize properly
                masm_: MacroAssembler {},           // Initialize properly
                resolver_: GapResolver {},           // Initialize properly
                safepoints_: SafepointTableBuilder {}, // Initialize properly
                handlers_: Vec::new(),
                next_deoptimization_id_: 0,
                deopt_exit_start_offset_: 0,
                eager_deopt_count_: 0,
                lazy_deopt_count_: 0,
                deoptimization_exits_: ZoneDeque::new(), // Initialize properly
                protected_deoptimization_literals_: ZoneDeque::new(), // Initialize properly
                deoptimization_literals_: ZoneDeque::new(), // Initialize properly
                inlined_function_count_: 0,
                translations_: FrameTranslationBuilder {}, // Initialize properly
                handler_table_offset_: 0,
                jump_deoptimization_entry_labels_: [Label::new(), Label::new()],
                max_unoptimized_frame_height_: max_unoptimized_frame_height,
                max_pushed_argument_count_: max_pushed_argument_count,
                parameter_count_: 0,
                caller_registers_saved_: false,
                fp_mode_: SaveFPRegsMode::Save,    // Initialize properly
                jump_tables_: std::ptr::null_mut(), // Initialize properly
                ools_: std::ptr::null_mut(),       // Initialize properly
                osr_helper_: osr_helper,
                osr_pc_offset_: 0,
                source_position_table_builder_: SourcePositionTableBuilder {}, // Initialize properly
                protected_instructions_: ZoneVector::new(), // Initialize properly
                result_: CodeGenResult::kSuccess,
                block_starts_: ZoneVector::new(),
                offsets_info_: TurbolizerCodeOffsetsInfo::default(),
                instr_starts_: ZoneVector::new(),
                move_cycle_: MoveCycleState {},
                debug_name_: debug_name,
            }
        }

        pub fn assemble_code(&mut self) {
            // Implementation for AssembleCode
        }

        pub fn finalize_code(&mut self) -> Result<(), String> {
            // Implementation for FinalizeCode (using Result for error handling)
            Ok(())
        }
        
        pub fn get_source_position_table(&mut self) -> base {
            // Implementation for GetSourcePositionTable
            base{}
        }

        pub fn get_protected_instructions_data(&mut self) -> base {
            // Implementation for GetProtectedInstructionsData
            base{}
        }

        pub fn instructions(&self) -> *mut InstructionSequence {
            self.instructions_
        }

        pub fn frame_access_state(&self) -> *mut FrameAccessState {
            self.frame_access_state_
        }

        pub fn frame(&self) -> *mut Frame {
            // Access frame through frame_access_state
            self.frame_access_state_
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn linkage(&self) -> *mut Linkage {
            self.linkage_
        }

        pub fn get_label(&mut self, rpo: RpoNumber) -> *mut Label {
            &mut self.labels_[0] // needs to be replaced with correct indexing based on rpo
        }

        pub fn record_protected_instruction(&mut self, instr_offset: u32) {
            // Implementation for RecordProtectedInstruction
        }

        pub fn start_source_position(&self) -> SourcePosition {
            self.start_source_position_
        }

        pub fn assemble_source_position_instr(&mut self, instr: *mut Instruction) {
             // Implementation for AssembleSourcePosition for instructions
        }

        pub fn assemble_source_position(&mut self, source_position: SourcePosition) {
            // Implementation for AssembleSourcePosition
        }

        pub fn record_safepoint(&mut self, references: *mut ReferenceMap, pc_offset: i32) {
             // Implementation for RecordSafepoint
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone_
        }

        pub fn masm(&mut self) -> &mut MacroAssembler {
            &mut self.masm_
        }

        pub fn safepoint_table_builder(&mut self) -> &mut SafepointTableBuilder {
            &mut self.safepoints_
        }

        pub fn handler_table_offset(&self) -> usize {
            self.handler_table_offset_
        }

        pub fn block_starts(&self) -> &ZoneVector<i32> {
            &self.block_starts_
        }

        pub fn instr_starts(&self) -> &ZoneVector<TurbolizerInstructionStartInfo> {
            &self.instr_starts_
        }

        pub fn offsets_info(&self) -> &TurbolizerCodeOffsetsInfo {
            &self.offsets_info_
        }

        pub const K_BINARY_SEARCH_SWITCH_MINIMAL_CASES: i32 = 4;

        pub fn should_apply_offset_to_stack_check(&self, instr: *mut Instruction, offset: *mut u32) -> bool {
            // Implementation for ShouldApplyOffsetToStackCheck
            false
        }

        pub fn get_stack_check_offset(&self) -> u32 {
            // Implementation for GetStackCheckOffset
            0
        }

        pub fn code_kind(&self) -> OptimizedCompilationInfo {
            // Assuming CodeKind can be obtained from OptimizedCompilationInfo
            unsafe {
                *self.info_
            }
        }

        fn resolver(&mut self) -> &mut GapResolver {
            &mut self.resolver_
        }

        fn safepoints(&mut self) -> &mut SafepointTableBuilder {
            &mut self.safepoints_
        }

        fn info(&self) -> &OptimizedCompilationInfo {
            unsafe { &*self.info_ }
        }

        fn osr_helper(&mut self) -> Option<&mut OsrHelper> {
            self.osr_helper_.as_mut()
        }

        fn create_frame_access_state(&mut self, frame: *mut Frame) {
            // Implementation for CreateFrameAccessState
        }

        fn finish_frame(&mut self, frame: *mut Frame) {
            // Implementation for FinishFrame
        }

        fn is_next_in_assembly_order(&self, block: RpoNumber) -> bool {
            // Implementation for IsNextInAssemblyOrder
            false
        }

        fn is_materializable_from_root(&self, object: Handle<HeapObject>, index_return: *mut RootIndex) -> bool {
            // Implementation for IsMaterializableFromRoot
            false
        }

        fn assemble_block(&mut self, block: *const InstructionBlock) -> CodeGenResult {
            // Implementation for AssembleBlock
            CodeGenResult::kSuccess
        }

        fn assemble_instruction(&mut self, instruction_index: i32, block: *const InstructionBlock) -> CodeGenResult {
            // Implementation for AssembleInstruction
            CodeGenResult::kSuccess
        }

        fn assemble_gaps(&mut self, instr: *mut Instruction) {
            // Implementation for AssembleGaps
        }

        fn compute_branch_info(&mut self, branch: *mut BranchInfo, condition: FlagsCondition, instr: *mut Instruction) -> RpoNumber {
            // Implementation for ComputeBranchInfo
            RpoNumber {}
        }

        fn get_slot_above_sp_before_tail_call(&mut self, instr: *mut Instruction, slot: *mut i32) -> bool {
            // Implementation for GetSlotAboveSPBeforeTailCall
            false
        }

        fn determine_stub_call_mode(&self) -> i32 {
            // Implementation for DetermineStubCallMode
            0
        }

        fn assemble_deoptimizer_call(&mut self, exit: *mut DeoptimizationExit) -> CodeGenResult {
            // Implementation for AssembleDeoptimizerCall
            CodeGenResult::kSuccess
        }

        fn build_translation(
            &mut self,
            instr: *mut Instruction,
            pc_offset: i32,
            frame_state_offset: usize,
            immediate_args_count: usize,
            state_combine: OutputFrameStateCombine,
        ) -> *mut DeoptimizationExit {
            // Implementation for BuildTranslation
            std::ptr::null_mut()
        }

        fn assemble_arch_instruction(&mut self, instr: *mut Instruction) -> CodeGenResult {
            // Implementation for AssembleArchInstruction
            CodeGenResult::kSuccess
        }

        fn assemble_arch_jump(&mut self, target: RpoNumber) {
            // Implementation for AssembleArchJump
        }

        fn assemble_arch_jump_regardless_of_assembly_order(&mut self, target: RpoNumber) {
            // Implementation for AssembleArchJumpRegardlessOfAssemblyOrder
        }

        fn assemble_arch_branch(&mut self, instr: *mut Instruction, branch: *mut BranchInfo) {
            // Implementation for AssembleArchBranch
        }

        fn assemble_arch_conditional_branch(&mut self, instr: *mut Instruction, branch: *mut BranchInfo) {
            // Implementation for AssembleArchConditionalBranch
        }

        fn assemble_arch_deopt_branch(&mut self, instr: *mut Instruction, branch: *mut BranchInfo) {
            // Implementation for AssembleArchDeoptBranch
        }

        fn assemble_arch_boolean(&mut self, instr: *mut Instruction, condition: FlagsCondition) {
            // Implementation for AssembleArchBoolean
        }

        fn assemble_arch_conditional_boolean(&mut self, instr: *mut Instruction) {
            // Implementation for AssembleArchConditionalBoolean
        }

        fn assemble_arch_select(&mut self, instr: *mut Instruction, condition: FlagsCondition) {
            // Implementation for AssembleArchSelect
        }

        fn assemble_arch_binary_search_switch(&mut self, instr: *mut Instruction) {
            // Implementation for AssembleArchBinarySearchSwitch
        }

        fn assemble_arch_table_switch(&mut self, instr: *mut Instruction) {
            // Implementation for AssembleArchTableSwitch
        }

        fn assemble_code_start_register_check(&mut self) {
            // Implementation for AssembleCodeStartRegisterCheck
        }

        fn bailout_if_deoptimized(&mut self) {
            // Implementation for BailoutIfDeoptimized
        }

        fn assemble_place_holder_for_lazy_deopt(&mut self, instr: *mut Instruction) {
            // Implementation for AssemblePlaceHolderForLazyDeopt
        }

        fn assemble_construct_frame(&mut self) {
            // Implementation for AssembleConstructFrame
        }

        fn assemble_return(&mut self, pop: *mut InstructionOperand) {
            // Implementation for AssembleReturn
        }

        fn assemble_deconstruct_frame(&mut self) {
            // Implementation for AssembleDeconstructFrame
        }

        fn assemble_prepare_tail_call(&mut self) {
            // Implementation for AssemblePrepareTailCall
        }

        fn get_push_compatible_moves(instr: *mut Instruction, push_type: PushTypeFlags, pushes: *mut ZoneVector<MoveOperands>) {
            // Implementation for GetPushCompatibleMoves
        }

        fn assemble_tail_call_before_gap(&mut self, instr: *mut Instruction, first_unused_stack_slot: i32) {
            // Implementation for AssembleTailCallBeforeGap
        }

        fn assemble_tail_call_after_gap(&mut self, instr: *mut Instruction, first_unused_stack_slot: i32) {
            // Implementation for AssembleTailCallAfterGap
        }

        fn finish_code(&mut self) {
            // Implementation for FinishCode
        }

        fn maybe_emit_out_of_line_constant_pool(&mut self) {
            // Implementation for MaybeEmitOutOfLineConstantPool
        }

        fn increment_stack_access_counter(&mut self, source: *mut InstructionOperand, destination: *mut InstructionOperand) {
            // Implementation for IncrementStackAccessCounter
        }

        fn add_jump_table(&mut self, targets: base) -> *mut Label {
            // Implementation for AddJumpTable
            std::ptr::null_mut()
        }

        fn assemble_jump_table(&mut self, targets: base) {
            // Implementation for AssembleJumpTable
        }

        fn record_call_position(&mut self, instr: *mut Instruction) {
            // Implementation for RecordCallPosition
        }

        fn record_deopt_info(&mut self, instr: *mut Instruction, pc_offset: i32) {
            // Implementation for RecordDeoptInfo
        }

        fn generate_deoptimization_data(&mut self) -> Handle<DeoptimizationData> {
            // Implementation for GenerateDeoptimizationData
            Handle {}
        }

        fn define_protected_deoptimization_literal(&mut self, object: IndirectHandle<HeapObject>) -> i32 {
            // Implementation for DefineProtectedDeoptimizationLiteral
            0
        }

        fn define_deoptimization_literal(&mut self, literal: DeoptimizationLiteral) -> i32 {
            // Implementation for DefineDeoptimizationLiteral
            0
        }

        fn has_protected_deoptimization_literal(&self, object: IndirectHandle<HeapObject>) -> bool {
            // Implementation for HasProtectedDeoptimizationLiteral
            false
        }

        fn get_deoptimization_entry(&self, instr: *mut Instruction, frame_state_offset: usize) -> DeoptimizationExit {
            // Implementation for GetDeoptimizationEntry
            DeoptimizationExit {}
        }

        fn build_translation_for_frame_state_descriptor(
            &mut self,
            descriptor: *mut FrameStateDescriptor,
            iter: *mut InstructionOperandIterator,
            state_combine: OutputFrameStateCombine,
        ) {
            // Implementation for BuildTranslationForFrameStateDescriptor
        }

        fn translate_state_value_descriptor(
            &mut self,
            desc: *mut StateValueDescriptor,
            nested: *mut StateValueList,
            iter: *mut InstructionOperandIterator,
        ) {
            // Implementation for TranslateStateValueDescriptor
        }

        fn translate_frame_state_descriptor_operands(
            &mut self,
            desc: *mut FrameStateDescriptor,
            iter: *mut InstructionOperandIterator,
        ) {
            // Implementation for TranslateFrameStateDescriptorOperands
        }

        fn add_translation_for_operand(&mut self, instr: *mut Instruction, op: *mut InstructionOperand, type_: MachineType) {
            // Implementation for AddTranslationForOperand
        }

        fn prepare_for_deoptimization_exits(&mut self, exits: *mut ZoneDeque<DeoptimizationExit>) {
            // Implementation for PrepareForDeoptimizationExits
        }

        fn add_deoptimization_exit(&mut self, instr: *mut Instruction, frame_state_offset: usize, immediate_args_count: usize) -> *mut DeoptimizationExit {
            // Implementation for AddDeoptimizationExit
            std::ptr::null_mut()
        }
    }

    impl GapResolverTrait for CodeGenerator {
        fn assemble_move(&mut self, source: *mut InstructionOperand, destination: *mut InstructionOperand) {
            // Implementation for AssembleMove
        }

        fn assemble_swap(&mut self, source: *mut InstructionOperand, destination: *mut InstructionOperand) {
            // Implementation for AssembleSwap
        }
        fn push(&mut self, src: *mut InstructionOperand) -> AllocatedOperand {
            // Implementation for Push
            AllocatedOperand{}
        }
        fn pop(&mut self, src: *mut InstructionOperand, rep: MachineRepresentation){
            // Implementation for Pop
        }
        fn pop_temp_stack_slots(&mut self){
             // Implementation for PopTempStackSlots
        }
        fn move_to_temp_location(&mut self, src: *mut InstructionOperand, rep: MachineRepresentation){
             // Implementation for MoveToTempLocation
        }
        fn move_temp_location_to(&mut self, dst: *mut InstructionOperand, rep: MachineRepresentation){
             // Implementation for MoveTempLocationTo
        }
        fn set_pending_move(&mut self, move_: *mut MoveOperands){
             // Implementation for SetPendingMove
        }
    }

    pub trait GapResolverTrait {
        fn assemble_move(&mut self, source: *mut InstructionOperand, destination: *mut InstructionOperand);
        fn assemble_swap(&mut self, source: *mut InstructionOperand, destination: *mut InstructionOperand);
        fn push(&mut self, src: *mut InstructionOperand) -> AllocatedOperand;
        fn pop(&mut self, src: *mut InstructionOperand, rep: MachineRepresentation);
        fn pop_temp_stack_slots(&mut self);
        fn move_to_temp_location(&mut self, src: *mut InstructionOperand, rep: MachineRepresentation);
        fn move_temp_location_to(&mut self, dst: *mut InstructionOperand, rep: MachineRepresentation);
        fn set_pending_move(&mut self, move_: *mut MoveOperands);
    }
    
    pub struct AllocatedOperand {}
    
    pub struct PushTypeFlag {}

    pub struct PushTypeFlags {}

    pub enum CodeGenResult {
        kSuccess,
        kTooManyDeoptimizationBailouts,
    }

    pub struct HandlerInfo {
        pub handler: *mut Label,
        pub pc_offset: i32,
    }

    // Implementations for helper structs/enums
    impl RpoNumber {
        pub fn to_size(&self) -> usize {
            0 // Placeholder
        }
    }

    impl Default for RpoNumber {
        fn default() -> Self {
            RpoNumber {}
        }
    }

    impl SourcePosition {}
    
    pub struct UnwindingInfoWriter {}
}