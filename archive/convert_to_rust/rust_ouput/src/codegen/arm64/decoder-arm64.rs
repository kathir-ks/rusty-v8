// Converted from V8 C++ source files:
// Header: decoder-arm64.h
// Implementation: decoder-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/codegen/arm64/decoder-arm64.h

pub mod decoder_arm64 {
    use std::collections::LinkedList;
    use std::error::Error;
    use std::fmt;

    // Assuming Instruction and other necessary types are defined elsewhere.
    pub struct Instruction {
        // Add fields as necessary to represent the ARM64 instruction.
    }

    // Placeholder for instruction field masks
    pub type InstructionMask = u32;

    const PCRelAddressingFMask: InstructionMask = 0;
    const PCRelAddressingFixed: InstructionMask = 0;
    const AddSubImmediateFMask: InstructionMask = 0;
    const AddSubImmediateFixed: InstructionMask = 0;
    const LogicalImmediateFMask: InstructionMask = 0;
    const LogicalImmediateFixed: InstructionMask = 0;
    const MoveWideImmediateFMask: InstructionMask = 0;
    const MoveWideImmediateFixed: InstructionMask = 0;
    const BitfieldFMask: InstructionMask = 0;
    const BitfieldFixed: InstructionMask = 0;
    const ExtractFMask: InstructionMask = 0;
    const ExtractFixed: InstructionMask = 0;
    const UnconditionalBranchFMask: InstructionMask = 0;
    const UnconditionalBranchFixed: InstructionMask = 0;
    const UnconditionalBranchToRegisterFMask: InstructionMask = 0;
    const UnconditionalBranchToRegisterFixed: InstructionMask = 0;
    const CompareBranchFMask: InstructionMask = 0;
    const CompareBranchFixed: InstructionMask = 0;
    const TestBranchFMask: InstructionMask = 0;
    const TestBranchFixed: InstructionMask = 0;
    const ConditionalBranchFMask: InstructionMask = 0;
    const ConditionalBranchFixed: InstructionMask = 0;
    const SystemFMask: InstructionMask = 0;
    const SystemFixed: InstructionMask = 0;
    const ExceptionFMask: InstructionMask = 0;
    const ExceptionFixed: InstructionMask = 0;
    const LoadStorePairPostIndexFMask: InstructionMask = 0;
    const LoadStorePairPostIndexFixed: InstructionMask = 0;
    const LoadStorePairOffsetFMask: InstructionMask = 0;
    const LoadStorePairOffsetFixed: InstructionMask = 0;
    const LoadStorePairPreIndexFMask: InstructionMask = 0;
    const LoadStorePairPreIndexFixed: InstructionMask = 0;
    const LoadLiteralFMask: InstructionMask = 0;
    const LoadLiteralFixed: InstructionMask = 0;
    const LoadStoreUnscaledOffsetFMask: InstructionMask = 0;
    const LoadStoreUnscaledOffsetFixed: InstructionMask = 0;
    const LoadStorePostIndexFMask: InstructionMask = 0;
    const LoadStorePostIndexFixed: InstructionMask = 0;
    const LoadStorePreIndexFMask: InstructionMask = 0;
    const LoadStorePreIndexFixed: InstructionMask = 0;
    const LoadStoreRegisterOffsetFMask: InstructionMask = 0;
    const LoadStoreRegisterOffsetFixed: InstructionMask = 0;
    const LoadStoreUnsignedOffsetFMask: InstructionMask = 0;
    const LoadStoreUnsignedOffsetFixed: InstructionMask = 0;
    const LoadStoreAcquireReleaseFMask: InstructionMask = 0;
    const LoadStoreAcquireReleaseFixed: InstructionMask = 0;
    const AtomicMemoryFMask: InstructionMask = 0;
    const AtomicMemoryFixed: InstructionMask = 0;
    const LogicalShiftedFMask: InstructionMask = 0;
    const LogicalShiftedFixed: InstructionMask = 0;
    const AddSubShiftedFMask: InstructionMask = 0;
    const AddSubShiftedFixed: InstructionMask = 0;
    const AddSubExtendedFMask: InstructionMask = 0;
    const AddSubExtendedFixed: InstructionMask = 0;
    const AddSubWithCarryFMask: InstructionMask = 0;
    const AddSubWithCarryFixed: InstructionMask = 0;
    const ConditionalCompareRegisterFMask: InstructionMask = 0;
    const ConditionalCompareRegisterFixed: InstructionMask = 0;
    const ConditionalCompareImmediateFMask: InstructionMask = 0;
    const ConditionalCompareImmediateFixed: InstructionMask = 0;
    const ConditionalSelectFMask: InstructionMask = 0;
    const ConditionalSelectFixed: InstructionMask = 0;
    const DataProcessing1SourceFMask: InstructionMask = 0;
    const DataProcessing1SourceFixed: InstructionMask = 0;
    const DataProcessing2SourceFMask: InstructionMask = 0;
    const DataProcessing2SourceFixed: InstructionMask = 0;
    const DataProcessing3SourceFMask: InstructionMask = 0;
    const DataProcessing3SourceFixed: InstructionMask = 0;
    const FPCompareFMask: InstructionMask = 0;
    const FPCompareFixed: InstructionMask = 0;
    const FPConditionalCompareFMask: InstructionMask = 0;
    const FPConditionalCompareFixed: InstructionMask = 0;
    const FPConditionalSelectFMask: InstructionMask = 0;
    const FPConditionalSelectFixed: InstructionMask = 0;
    const FPImmediateFMask: InstructionMask = 0;
    const FPImmediateFixed: InstructionMask = 0;
    const FPDataProcessing1SourceFMask: InstructionMask = 0;
    const FPDataProcessing1SourceFixed: InstructionMask = 0;
    const FPDataProcessing2SourceFMask: InstructionMask = 0;
    const FPDataProcessing2SourceFixed: InstructionMask = 0;
    const FPDataProcessing3SourceFMask: InstructionMask = 0;
    const FPDataProcessing3SourceFixed: InstructionMask = 0;
    const FPIntegerConvertFMask: InstructionMask = 0;
    const FPIntegerConvertFixed: InstructionMask = 0;
    const FPFixedPointConvertFMask: InstructionMask = 0;
    const FPFixedPointConvertFixed: InstructionMask = 0;
    const NEON2RegMiscFMask: InstructionMask = 0;
    const NEON2RegMiscFixed: InstructionMask = 0;
    const NEON3DifferentFMask: InstructionMask = 0;
    const NEON3DifferentFixed: InstructionMask = 0;
    const NEON3ExtensionFMask: InstructionMask = 0;
    const NEON3ExtensionFixed: InstructionMask = 0;
    const NEON3SameFMask: InstructionMask = 0;
    const NEON3SameFixed: InstructionMask = 0;
    const NEON3SameHPFMask: InstructionMask = 0;
    const NEON3SameHPFixed: InstructionMask = 0;
    const NEONAcrossLanesFMask: InstructionMask = 0;
    const NEONAcrossLanesFixed: InstructionMask = 0;
    const NEONByIndexedElementFMask: InstructionMask = 0;
    const NEONByIndexedElementFixed: InstructionMask = 0;
    const NEONCopyFMask: InstructionMask = 0;
    const NEONCopyFixed: InstructionMask = 0;
    const NEONExtractFMask: InstructionMask = 0;
    const NEONExtractFixed: InstructionMask = 0;
    const NEONLoadStoreMultiStructFMask: InstructionMask = 0;
    const NEONLoadStoreMultiStructFixed: InstructionMask = 0;
    const NEONLoadStoreMultiStructPostIndexFMask: InstructionMask = 0;
    const NEONLoadStoreMultiStructPostIndexFixed: InstructionMask = 0;
    const NEONLoadStoreSingleStructFMask: InstructionMask = 0;
    const NEONLoadStoreSingleStructFixed: InstructionMask = 0;
    const NEONLoadStoreSingleStructPostIndexFMask: InstructionMask = 0;
    const NEONLoadStoreSingleStructPostIndexFixed: InstructionMask = 0;
    const NEONModifiedImmediateFMask: InstructionMask = 0;
    const NEONModifiedImmediateFixed: InstructionMask = 0;
    const NEONScalar2RegMiscFMask: InstructionMask = 0;
    const NEONScalar2RegMiscFixed: InstructionMask = 0;
    const NEONScalar3DiffFMask: InstructionMask = 0;
    const NEONScalar3DiffFixed: InstructionMask = 0;
    const NEONScalar3SameFMask: InstructionMask = 0;
    const NEONScalar3SameFixed: InstructionMask = 0;
    const NEONScalarByIndexedElementFMask: InstructionMask = 0;
    const NEONScalarByIndexedElementFixed: InstructionMask = 0;
    const NEONScalarCopyFMask: InstructionMask = 0;
    const NEONScalarCopyFixed: InstructionMask = 0;
    const NEONScalarPairwiseFMask: InstructionMask = 0;
    const NEONScalarPairwiseFixed: InstructionMask = 0;
    const NEONScalarShiftImmediateFMask: InstructionMask = 0;
    const NEONScalarShiftImmediateFixed: InstructionMask = 0;
    const NEONShiftImmediateFMask: InstructionMask = 0;
    const NEONShiftImmediateFixed: InstructionMask = 0;
    const NEONTableFMask: InstructionMask = 0;
    const NEONTableFixed: InstructionMask = 0;
    const NEONPermFMask: InstructionMask = 0;
    const NEONPermFixed: InstructionMask = 0;
    const UnallocatedFMask: InstructionMask = 0;
    const UnallocatedFixed: InstructionMask = 0;
    const UnimplementedFMask: InstructionMask = 0;
    const UnimplementedFixed: InstructionMask = 0;

    // Define the Visitor interface.
    pub trait DecoderVisitor {
        fn visit_pc_rel_addressing(&mut self, instr: &Instruction);
        fn visit_add_sub_immediate(&mut self, instr: &Instruction);
        fn visit_logical_immediate(&mut self, instr: &Instruction);
        fn visit_move_wide_immediate(&mut self, instr: &Instruction);
        fn visit_bitfield(&mut self, instr: &Instruction);
        fn visit_extract(&mut self, instr: &Instruction);
        fn visit_unconditional_branch(&mut self, instr: &Instruction);
        fn visit_unconditional_branch_to_register(&mut self, instr: &Instruction);
        fn visit_compare_branch(&mut self, instr: &Instruction);
        fn visit_test_branch(&mut self, instr: &Instruction);
        fn visit_conditional_branch(&mut self, instr: &Instruction);
        fn visit_system(&mut self, instr: &Instruction);
        fn visit_exception(&mut self, instr: &Instruction);
        fn visit_load_store_pair_post_index(&mut self, instr: &Instruction);
        fn visit_load_store_pair_offset(&mut self, instr: &Instruction);
        fn visit_load_store_pair_pre_index(&mut self, instr: &Instruction);
        fn visit_load_literal(&mut self, instr: &Instruction);
        fn visit_load_store_unscaled_offset(&mut self, instr: &Instruction);
        fn visit_load_store_post_index(&mut self, instr: &Instruction);
        fn visit_load_store_pre_index(&mut self, instr: &Instruction);
        fn visit_load_store_register_offset(&mut self, instr: &Instruction);
        fn visit_load_store_unsigned_offset(&mut self, instr: &Instruction);
        fn visit_load_store_acquire_release(&mut self, instr: &Instruction);
        fn visit_atomic_memory(&mut self, instr: &Instruction);
        fn visit_logical_shifted(&mut self, instr: &Instruction);
        fn visit_add_sub_shifted(&mut self, instr: &Instruction);
        fn visit_add_sub_extended(&mut self, instr: &Instruction);
        fn visit_add_sub_with_carry(&mut self, instr: &Instruction);
        fn visit_conditional_compare_register(&mut self, instr: &Instruction);
        fn visit_conditional_compare_immediate(&mut self, instr: &Instruction);
        fn visit_conditional_select(&mut self, instr: &Instruction);
        fn visit_data_processing1_source(&mut self, instr: &Instruction);
        fn visit_data_processing2_source(&mut self, instr: &Instruction);
        fn visit_data_processing3_source(&mut self, instr: &Instruction);
        fn visit_fp_compare(&mut self, instr: &Instruction);
        fn visit_fp_conditional_compare(&mut self, instr: &Instruction);
        fn visit_fp_conditional_select(&mut self, instr: &Instruction);
        fn visit_fp_immediate(&mut self, instr: &Instruction);
        fn visit_fp_data_processing1_source(&mut self, instr: &Instruction);
        fn visit_fp_data_processing2_source(&mut self, instr: &Instruction);
        fn visit_fp_data_processing3_source(&mut self, instr: &Instruction);
        fn visit_fp_integer_convert(&mut self, instr: &Instruction);
        fn visit_fp_fixed_point_convert(&mut self, instr: &Instruction);
        fn visit_neon2_reg_misc(&mut self, instr: &Instruction);
        fn visit_neon3_different(&mut self, instr: &Instruction);
        fn visit_neon3_extension(&mut self, instr: &Instruction);
        fn visit_neon3_same(&mut self, instr: &Instruction);
        fn visit_neon3_same_hp(&mut self, instr: &Instruction);
        fn visit_neon_across_lanes(&mut self, instr: &Instruction);
        fn visit_neon_by_indexed_element(&mut self, instr: &Instruction);
        fn visit_neon_copy(&mut self, instr: &Instruction);
        fn visit_neon_extract(&mut self, instr: &Instruction);
        fn visit_neon_load_store_multi_struct(&mut self, instr: &Instruction);
        fn visit_neon_load_store_multi_struct_post_index(&mut self, instr: &Instruction);
        fn visit_neon_load_store_single_struct(&mut self, instr: &Instruction);
        fn visit_neon_load_store_single_struct_post_index(&mut self, instr: &Instruction);
        fn visit_neon_modified_immediate(&mut self, instr: &Instruction);
        fn visit_neon_scalar2_reg_misc(&mut self, instr: &Instruction);
        fn visit_neon_scalar3_diff(&mut self, instr: &Instruction);
        fn visit_neon_scalar3_same(&mut self, instr: &Instruction);
        fn visit_neon_scalar_by_indexed_element(&mut self, instr: &Instruction);
        fn visit_neon_scalar_copy(&mut self, instr: &Instruction);
        fn visit_neon_scalar_pairwise(&mut self, instr: &Instruction);
        fn visit_neon_scalar_shift_immediate(&mut self, instr: &Instruction);
        fn visit_neon_shift_immediate(&mut self, instr: &Instruction);
        fn visit_neon_table(&mut self, instr: &Instruction);
        fn visit_neon_perm(&mut self, instr: &Instruction);
        fn visit_unallocated(&mut self, instr: &Instruction);
        fn visit_unimplemented(&mut self, instr: &Instruction);
    }

    // A visitor that dispatches to a list of visitors.
    pub struct DispatchingDecoderVisitor {
        visitors: LinkedList<Box<dyn DecoderVisitor>>,
    }

    impl DispatchingDecoderVisitor {
        pub fn new() -> Self {
            DispatchingDecoderVisitor {
                visitors: LinkedList::new(),
            }
        }

        // Register a new visitor class with the decoder.
        pub fn append_visitor(&mut self, visitor: Box<dyn DecoderVisitor>) {
            self.remove_visitor_trait_object(visitor.as_ref());
            self.visitors.push_back(visitor);
        }

        pub fn prepend_visitor(&mut self, visitor: Box<dyn DecoderVisitor>) {
            self.remove_visitor_trait_object(visitor.as_ref());
            self.visitors.push_front(visitor);
        }

        pub fn insert_visitor_before(
            &mut self,
            new_visitor: Box<dyn DecoderVisitor>,
            registered_visitor: &dyn DecoderVisitor,
        ) {
            self.remove_visitor_trait_object(new_visitor.as_ref());
            let mut insert_point = None;
            for (i, existing_visitor) in self.visitors.iter().enumerate() {
                if existing_visitor.as_ref() as *const dyn DecoderVisitor == registered_visitor as *const dyn DecoderVisitor {
                    insert_point = Some(i);
                    break;
                }
            }

            if let Some(index) = insert_point {
                let mut temp_list: LinkedList<Box<dyn DecoderVisitor>> = LinkedList::new();
                while self.visitors.len() > index {
                    if let Some(node) = self.visitors.pop_front() {
                        temp_list.push_back(node);
                    }
                }
                self.visitors.push_back(new_visitor);
                while let Some(node) = temp_list.pop_front() {
                    self.visitors.push_back(node);
                }
            } else {
                // registered_visitor not found. Append.
                self.visitors.push_back(new_visitor);
            }
        }

        pub fn insert_visitor_after(
            &mut self,
            new_visitor: Box<dyn DecoderVisitor>,
            registered_visitor: &dyn DecoderVisitor,
        ) {
            self.remove_visitor_trait_object(new_visitor.as_ref());
            let mut insert_point = None;
            for (i, existing_visitor) in self.visitors.iter().enumerate() {
                if existing_visitor.as_ref() as *const dyn DecoderVisitor == registered_visitor as *const dyn DecoderVisitor {
                    insert_point = Some(i);
                    break;
                }
            }

            if let Some(index) = insert_point {
                let mut temp_list: LinkedList<Box<dyn DecoderVisitor>> = LinkedList::new();
                while self.visitors.len() > index + 1 {
                    if let Some(node) = self.visitors.pop_front() {
                        temp_list.push_back(node);
                    }
                }

                if let Some(node) = self.visitors.pop_front() {
                    temp_list.push_back(node);
                }
                self.visitors.push_back(new_visitor);

                while let Some(node) = temp_list.pop_front() {
                    self.visitors.push_back(node);
                }
            } else {
                // registered_visitor not found. Append.
                self.visitors.push_back(new_visitor);
            }
        }

        // Remove a previously registered visitor class from the list of visitors
        // stored by the decoder.
        pub fn remove_visitor(&mut self, visitor: &dyn DecoderVisitor) {
            self.remove_visitor_trait_object(visitor);
        }

        fn remove_visitor_trait_object(&mut self, visitor: &dyn DecoderVisitor) {
            let mut to_remove: Option<*const dyn DecoderVisitor> = None;
            for existing_visitor in self.visitors.iter() {
                if existing_visitor.as_ref() as *const dyn DecoderVisitor == visitor as *const dyn DecoderVisitor {
                    to_remove = Some(visitor as *const dyn DecoderVisitor);
                    break;
                }
            }

            if let Some(visitor_ptr) = to_remove {
                self.visitors.retain(|existing_visitor| {
                    existing_visitor.as_ref() as *const dyn DecoderVisitor != visitor_ptr
                });
            }
        }
    }

    impl DecoderVisitor for DispatchingDecoderVisitor {
        fn visit_neon_shift_immediate(&mut self, instr: &Instruction) {
            self.visit(instr, NEONShiftImmediateFixed, NEONShiftImmediateFMask);
        }

        fn visit_pc_rel_addressing(&mut self, instr: &Instruction) {
            self.visit(instr, PCRelAddressingFixed, PCRelAddressingFMask);
        }

        fn visit_add_sub_immediate(&mut self, instr: &Instruction) {
            self.visit(instr, AddSubImmediateFixed, AddSubImmediateFMask);
        }

        fn visit_logical_immediate(&mut self, instr: &Instruction) {
            self.visit(instr, LogicalImmediateFixed, LogicalImmediateFMask);
        }

        fn visit_move_wide_immediate(&mut self, instr: &Instruction) {
            self.visit(instr, MoveWideImmediateFixed, MoveWideImmediateFMask);
        }

        fn visit_bitfield(&mut self, instr: &Instruction) {
            self.visit(instr, BitfieldFixed, BitfieldFMask);
        }

        fn visit_extract(&mut self, instr: &Instruction) {
            self.visit(instr, ExtractFixed, ExtractFMask);
        }

        fn visit_unconditional_branch(&mut self, instr: &Instruction) {
            self.visit(instr, UnconditionalBranchFixed, UnconditionalBranchFMask);
        }

        fn visit_unconditional_branch_to_register(&mut self, instr: &Instruction) {
            self.visit(instr, UnconditionalBranchToRegisterFixed, UnconditionalBranchToRegisterFMask);
        }

        fn visit_compare_branch(&mut self, instr: &Instruction) {
            self.visit(instr, CompareBranchFixed, CompareBranchFMask);
        }

        fn visit_test_branch(&mut self, instr: &Instruction) {
            self.visit(instr, TestBranchFixed, TestBranchFMask);
        }

        fn visit_conditional_branch(&mut self, instr: &Instruction) {
            self.visit(instr, ConditionalBranchFixed, ConditionalBranchFMask);
        }

        fn visit_system(&mut self, instr: &Instruction) {
            self.visit(instr, SystemFixed, SystemFMask);
        }

        fn visit_exception(&mut self, instr: &Instruction) {
            self.visit(instr, ExceptionFixed, ExceptionFMask);
        }

        fn visit_load_store_pair_post_index(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStorePairPostIndexFixed, LoadStorePairPostIndexFMask);
        }

        fn visit_load_store_pair_offset(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStorePairOffsetFixed, LoadStorePairOffsetFMask);
        }

        fn visit_load_store_pair_pre_index(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStorePairPreIndexFixed, LoadStorePairPreIndexFMask);
        }

        fn visit_load_literal(&mut self, instr: &Instruction) {
            self.visit(instr, LoadLiteralFixed, LoadLiteralFMask);
        }

        fn visit_load_store_unscaled_offset(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStoreUnscaledOffsetFixed, LoadStoreUnscaledOffsetFMask);
        }

        fn visit_load_store_post_index(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStorePostIndexFixed, LoadStorePostIndexFMask);
        }

        fn visit_load_store_pre_index(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStorePreIndexFixed, LoadStorePreIndexFMask);
        }

        fn visit_load_store_register_offset(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStoreRegisterOffsetFixed, LoadStoreRegisterOffsetFMask);
        }

        fn visit_load_store_unsigned_offset(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStoreUnsignedOffsetFixed, LoadStoreUnsignedOffsetFMask);
        }

        fn visit_load_store_acquire_release(&mut self, instr: &Instruction) {
            self.visit(instr, LoadStoreAcquireReleaseFixed, LoadStoreAcquireReleaseFMask);
        }

        fn visit_atomic_memory(&mut self, instr: &Instruction) {
            self.visit(instr, AtomicMemoryFixed, AtomicMemoryFMask);
        }

        fn visit_logical_shifted(&mut self, instr: &Instruction) {
            self.visit(instr, LogicalShiftedFixed, LogicalShiftedFMask);
        }

        fn visit_add_sub_shifted(&mut self, instr: &Instruction) {
            self.visit(instr, AddSubShiftedFixed, AddSubShiftedFMask);
        }

        fn visit_add_sub_extended(&mut self, instr: &Instruction) {
            self.visit(instr, AddSubExtendedFixed, AddSubExtendedFMask);
        }

        fn visit_add_sub_with_carry(&mut self, instr: &Instruction) {
            self.visit(instr, AddSubWithCarryFixed, AddSubWithCarryFMask);
        }

        fn visit_conditional_compare_register(&mut self, instr: &Instruction) {
            self.visit(instr, ConditionalCompareRegisterFixed, ConditionalCompareRegisterFMask);
        }

        fn visit_conditional_compare_immediate(&mut self, instr: &Instruction) {
            self.visit(instr, ConditionalCompareImmediateFixed, ConditionalCompareImmediateFMask);
        }

        fn visit_conditional_select(&mut self, instr: &Instruction) {
            self.visit(instr, ConditionalSelectFixed, ConditionalSelectFMask);
        }

        fn visit_data_processing1_source(&mut self, instr: &Instruction) {
            self.visit(instr, DataProcessing1SourceFixed, DataProcessing1SourceFMask);
        }

        fn visit_data_processing2_source(&mut self, instr: &Instruction) {
            self.visit(instr, DataProcessing2SourceFixed, DataProcessing2SourceFMask);
        }

        fn visit_data_processing3_source(&mut self, instr: &Instruction) {
            self.visit(instr, DataProcessing3SourceFixed, DataProcessing3SourceFMask);
        }

        fn visit_fp_compare(&mut self, instr: &Instruction) {
            self.visit(instr, FPCompareFixed, FPCompareFMask);
        }

        fn visit_fp_conditional_compare(&mut self, instr: &Instruction) {
            self.visit(instr, FPConditionalCompareFixed, FPConditionalCompareFMask);
        }

        fn visit_fp_conditional_select(&mut self, instr: &Instruction) {
            self.visit(instr, FPConditionalSelectFixed, FPConditionalSelectFMask);
        }

        fn visit_fp_immediate(&mut self, instr: &Instruction) {
            self.visit(instr, FPImmediateFixed, FPImmediateFMask);
        }

        fn visit_fp_data_processing1_source(&mut self, instr: &Instruction) {
            self.visit(instr, FPDataProcessing1SourceFixed, FPDataProcessing1SourceFMask);
        }

        fn visit_fp_data_processing2_source(&mut self, instr: &Instruction) {
            self.visit(instr, FPDataProcessing2SourceFixed, FPDataProcessing2SourceFMask);
        }

        fn visit_fp_data_processing3_source(&mut self, instr: &Instruction) {
            self.visit(instr, FPDataProcessing3SourceFixed, FPDataProcessing3SourceFMask);
        }

        fn visit_fp_integer_convert(&mut self, instr: &Instruction) {
            self.visit(instr, FPIntegerConvertFixed, FPIntegerConvertFMask);
        }

        fn visit_fp_fixed_point_convert(&mut self, instr: &Instruction) {
            self.visit(instr, FPFixedPointConvertFixed, FPFixedPointConvertFMask);
        }

        fn visit_neon2_reg_misc(&mut self, instr: &Instruction) {
            self.visit(instr, NEON2RegMiscFixed, NEON2RegMiscFMask);
        }

        fn visit_neon3_different(&mut self, instr: &Instruction) {
            self.visit(instr, NEON3DifferentFixed, NEON3DifferentFMask);
        }

        fn visit_neon3_extension(&mut self, instr: &Instruction) {
            self.visit(instr, NEON3ExtensionFixed, NEON3ExtensionFMask);
        }

        fn visit_neon3_same(&mut self, instr: &Instruction) {
            self.visit(instr, NEON3SameFixed, NEON3SameFMask);
        }

        fn visit_neon3_same_hp(&mut self, instr: &Instruction) {
            self.visit(instr, NEON3SameHPFixed, NEON3SameHPFMask);
        }

        fn visit_neon_across_lanes(&mut self, instr: &Instruction) {
            self.visit(instr, NEONAcrossLanesFixed, NEONAcrossLanesFMask);
        }

        fn visit_neon_by_indexed_element(&mut self, instr: &Instruction) {
            self.visit(instr, NEONByIndexedElementFixed, NEONByIndexedElementFMask);
        }

        fn visit_neon_copy(&mut self, instr: &Instruction) {
            self.visit(instr, NEONCopyFixed, NEONCopyFMask);
        }

        fn visit_neon_extract(&mut self, instr: &Instruction) {
            self.visit(instr, NEONExtractFixed, NEONExtractFMask);
        }

        fn visit_neon_load_store_multi_struct(&mut self, instr: &Instruction) {
            self.visit(instr, NEONLoadStoreMultiStructFixed, NEONLoadStoreMultiStructFMask);
        }

        fn visit_neon_load_store_multi_struct_post_index(&mut self, instr: &Instruction) {
            self.visit(instr, NEONLoadStoreMultiStructPostIndexFixed, NEONLoadStoreMultiStructPostIndexFMask);
        }

        fn visit_neon_load_store_single_struct(&mut self, instr: &Instruction) {
            self.visit(instr, NEONLoadStoreSingleStructFixed, NEONLoadStoreSingleStructFMask);
        }

        fn visit_neon_load_store_single_struct_post_index(&mut self, instr: &Instruction) {
            self.visit(instr, NEONLoadStoreSingleStructPostIndexFixed, NEONLoadStoreSingleStructPostIndexFMask);
        }

        fn visit_neon_modified_immediate(&mut self, instr: &Instruction) {
            self.visit(instr, NEONModifiedImmediateFixed, NEONModifiedImmediateFMask);
        }

        fn visit_neon_scalar2_reg_misc(&mut self, instr: &Instruction) {
            self.visit(instr, NEONScalar2RegMiscFixed, NEONScalar2RegMiscFMask);
        }

        fn visit_neon_scalar3_diff(&mut self, instr: &Instruction) {
            self.visit(instr, NEONScalar3DiffFixed, NEONScalar3DiffFMask);
        }

        fn visit_neon_scalar3_same(&mut self, instr: &Instruction) {
            self.visit(instr, NEONScalar3SameFixed, NEONScalar3SameFMask);
        }

        fn visit_neon_scalar_by_indexed_element(&mut self, instr: &Instruction) {
            self.visit(instr, NEONScalarByIndexedElementFixed, NEONScalarByIndexedElementFMask);
        }

        fn visit_neon_scalar_copy(&mut self, instr: &Instruction) {
            self.visit(instr, NEONScalarCopyFixed, NEONScalarCopyFMask);
        }

        fn visit_neon_scalar_pairwise(&mut self, instr: &Instruction) {
            self.visit(instr, NEONScalarPairwiseFixed, NEONScalarPairwiseFMask);
        }

        fn visit_neon_scalar_shift_immediate(&mut self, instr: &Instruction) {
            self.visit(instr, NEONScalarShiftImmediateFixed, NEONScalarShiftImmediateFMask);
        }

        fn visit_neon_table(&mut self, instr: &Instruction) {
            self.visit(instr, NEONTableFixed, NEONTableFMask);
        }

        fn visit_neon_perm(&mut self, instr: &Instruction) {
            self.visit(instr, NEONPermFixed, NEONPermFMask);
        }

        fn visit_unallocated(&mut self, instr: &Instruction) {
            self.visit(instr, UnallocatedFixed, UnallocatedFMask);
        }

        fn visit_unimplemented(&mut self, instr: &Instruction) {
            self.visit(instr, UnimplementedFixed, UnimplementedFMask);
        }
    }

    impl DispatchingDecoderVisitor {
        fn visit(&mut self, instr: &Instruction, fixed: InstructionMask, mask: InstructionMask) {
            if !(instr.mask(mask) == fixed) {
                assert_eq!(instr.mask(mask), fixed);
            }
            for visitor in &mut self.visitors {
                visitor.visit_pc_rel_addressing(instr);
                visitor.visit_add_sub_immediate(instr);
                visitor.visit_logical_immediate(instr);
                visitor.visit_move_wide_immediate(instr);
                visitor.visit_bitfield(instr);
                visitor.visit_extract(instr);
                visitor.visit_unconditional_branch(instr);
                visitor.visit_unconditional_branch_to_register(instr);
                visitor.visit_compare_branch(instr);
                visitor.visit_test_branch(instr);
                visitor.visit_conditional_branch(instr);
                visitor.visit_system(instr);
                visitor.visit_exception(instr);
                visitor.visit_load_store_pair_post_index(instr);
                visitor.visit_load_store_pair_offset(instr);
                visitor.visit_load_store_pair_pre_index(instr);
                visitor.visit_load_literal(instr);
                visitor.visit_load_store_unscaled_offset(instr);
                visitor.visit_load_store_post_index(instr);
                visitor.visit_load_store_pre_index(instr);
                visitor.visit_load_store_register_offset(instr);
                visitor.visit_load_store_unsigned_offset(instr);
                visitor.visit_load_store_acquire_release(instr);
                visitor.visit_atomic_memory(instr);
                visitor.visit_logical_shifted(instr);
                visitor.visit_add_sub_shifted(instr);
                visitor.visit_add_sub_extended(instr);
                visitor.visit_add_sub_with_carry(instr);
                visitor.visit_conditional_compare_register(instr);
                visitor
