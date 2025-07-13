// Converted from V8 C++ source files:
// Header: instruction-selector-impl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod instruction_selector_impl {
    use std::cmp::Ordering;
    use std::vec;

    use crate::codegen::macro_assembler::RelocatablePtrConstantInfo;
    use crate::codegen::macro_assembler::RelocInfo;
    use crate::compiler::backend::instruction::InstructionCode;
    use crate::compiler::backend::instruction_selector::InstructionSelectorT;
    use crate::compiler::backend::move_optimizer::Is64;
    use crate::compiler::backend::register_allocator::Register;
    use crate::compiler::c_linkage::LinkageLocation;
    use crate::compiler::turboshaft::build_graph_phase::ZoneWithNamePointer;
    use crate::compiler::turboshaft::ConstantOp;
    use crate::compiler::turboshaft::TurboshaftGraph;
    use crate::objects::tagged_index::TaggedIndex;
    use crate::Address;
    use crate::IsFloatingPoint;
    use crate::MachineRepresentation;
    use crate::RelocatablePtrConstantType;
    use crate::UNREACHABLE;
    use crate::Zone;
    use crate::ZoneVector;
    use crate::kInt32Size;
    use crate::kSystemPointerSize;
    use crate::v8::internal::compiler::turboshaft;
    use crate::v8::internal::compiler::TurboshaftAdapter;

    #[derive(Clone, Debug)]
    pub struct CaseInfoT {
        pub value: i32,
        pub order: i32,
        pub branch: *mut turboshaft::Block,
    }

    impl CaseInfoT {
        pub fn new(value: i32, order: i32, branch: *mut turboshaft::Block) -> Self {
            CaseInfoT { value, order, branch }
        }
    }

    impl PartialEq for CaseInfoT {
        fn eq(&self, other: &Self) -> bool {
            self.order == other.order
        }
    }

    impl Eq for CaseInfoT {}

    impl PartialOrd for CaseInfoT {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for CaseInfoT {
        fn cmp(&self, other: &Self) -> Ordering {
            self.order.cmp(&other.order)
        }
    }

    // Helper struct containing data about a table or lookup switch.
    pub struct SwitchInfoT {
        cases_: ZoneVector<CaseInfoT>,
        min_value_: i32,
        max_value_: i32,
        value_range_: usize,
        default_branch_: *mut turboshaft::Block,
    }

    impl SwitchInfoT {
        pub fn new(
            cases: ZoneVector<CaseInfoT>,
            min_value: i32,
            max_value: i32,
            default_branch: *mut turboshaft::Block,
        ) -> Self {
            let mut value_range_ = 0;

            if cases.len() != 0 {
                assert!(min_value <= max_value);
                // Note that {value_range} can be 0 if {min_value} is -2^31 and
                // {max_value} is 2^31-1, so don't assume that it's non-zero below.
                value_range_ =
                    1 + (max_value as u32).wrapping_sub(min_value as u32) as usize;
            }

            SwitchInfoT {
                cases_: cases,
                min_value_: min_value,
                max_value_: max_value,
                value_range_: value_range_,
                default_branch_: default_branch,
            }
        }

        pub fn cases_sorted_by_value(&self) -> Vec<CaseInfoT> {
            let mut result = self.cases_.clone().into_vec();
            result.sort_by(|a, b| a.value.cmp(&b.value));
            result
        }

        pub fn cases_unsorted(&self) -> &ZoneVector<CaseInfoT> {
            &self.cases_
        }

        pub fn min_value(&self) -> i32 {
            self.min_value_
        }

        pub fn max_value(&self) -> i32 {
            self.max_value_
        }

        pub fn value_range(&self) -> usize {
            self.value_range_
        }

        pub fn case_count(&self) -> usize {
            self.cases_.len()
        }

        pub fn default_branch(&self) -> *mut turboshaft::Block {
            self.default_branch_
        }
    }

    // A helper class for the instruction selector that simplifies construction of
    // Operands. This class implements a base for architecture-specific helpers.
    pub struct OperandGeneratorT<'a> {
        selector_: &'a mut InstructionSelectorT<'a>,
        turboshaft_adapter: TurboshaftAdapter<'a>,
    }

    impl<'a> OperandGeneratorT<'a> {
        pub fn new(selector: &'a mut InstructionSelectorT<'a>) -> Self {
            OperandGeneratorT {
                selector_: selector,
                turboshaft_adapter: TurboshaftAdapter::new(selector.schedule()),
            }
        }

        pub fn no_output(&self) -> crate::compiler::backend::instruction::InstructionOperand {
            crate::compiler::backend::instruction::InstructionOperand::default()
        }

        pub fn define_as_register(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.define(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                    self.get_vreg(node),
                ),
            )
        }

        pub fn define_same_as_input(
            &mut self,
            node: turboshaft::OpIndex,
            input_index: i32,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.define(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_same_as_input(
                    self.get_vreg(node),
                    input_index,
                ),
            )
        }

        pub fn define_same_as_first(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.define_same_as_input(node, 0)
        }

        pub fn define_as_fixed(
            &mut self,
            node: turboshaft::OpIndex,
            reg: Register,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.define(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_register(
                    reg.code(),
                    self.get_vreg(node),
                ),
            )
        }

        pub fn define_as_fixed_fp<FPRegType: Copy + Clone>(
            &mut self,
            node: turboshaft::OpIndex,
            reg: FPRegType,
        ) -> crate::compiler::backend::instruction::InstructionOperand
        where
            i32: From<FPRegType>,
        {
            self.define(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_fp_register(
                    i32::from(reg),
                    self.get_vreg(node),
                ),
            )
        }

        pub fn define_as_constant(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.selector_.mark_as_defined(node);
            let virtual_register = self.get_vreg(node);
            self.sequence()
                .add_constant(virtual_register, self.to_constant(node));
            crate::compiler::backend::instruction::InstructionOperand::Constant(virtual_register)
        }

        pub fn define_as_location(
            &mut self,
            node: turboshaft::OpIndex,
            location: LinkageLocation,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.define(node, self.to_unallocated_operand(location, self.get_vreg(node)))
        }

        pub fn define_as_dual_location(
            &mut self,
            node: turboshaft::OpIndex,
            primary_location: LinkageLocation,
            secondary_location: LinkageLocation,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.define(
                node,
                self.to_dual_location_unallocated_operand(
                    primary_location,
                    secondary_location,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_node(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_none(
                    crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_any_at_end(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_register_or_slot(
                    crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_END,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_any(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_register_or_slot(
                    crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_register_or_slot_or_constant(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_register_or_slot_or_constant(
                    crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_unique_register_or_slot_or_constant(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_register_or_slot_or_constant(
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_register(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                    crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_register_at_end(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                    crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_END,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_unique_slot(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_slot(
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_unique(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_none(
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_unique_register(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_register_with_mode(
            &mut self,
            node: turboshaft::OpIndex,
            register_mode: RegisterMode,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            match register_mode {
                RegisterMode::kRegister => self.use_register(node),
                RegisterMode::kUniqueRegister => self.use_unique_register(node),
            }
        }

        pub fn use_fixed(
            &mut self,
            node: turboshaft::OpIndex,
            reg: Register,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_register(
                    reg.code(),
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_fixed_fp<FPRegType: Copy + Clone>(
            &mut self,
            node: turboshaft::OpIndex,
            reg: FPRegType,
        ) -> crate::compiler::backend::instruction::InstructionOperand
        where
            i32: From<FPRegType>,
        {
            self.use_op(
                node,
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_fp_register(
                    i32::from(reg),
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_immediate(&mut self, immediate: i32) -> crate::compiler::backend::instruction::InstructionOperand {
            self.sequence()
                .add_immediate(crate::compiler::backend::code_generator_impl::Constant::new_int32(immediate))
        }

        pub fn use_immediate64(&mut self, immediate: i64) -> crate::compiler::backend::instruction::InstructionOperand {
            self.sequence()
                .add_immediate(crate::compiler::backend::code_generator_impl::Constant::new_int64(immediate))
        }

        pub fn use_immediate_op(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.sequence().add_immediate(self.to_constant(node))
        }

        pub fn use_negated_immediate(
            &mut self,
            node: turboshaft::OpIndex,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.sequence()
                .add_immediate(self.to_negated_constant(node))
        }

        pub fn use_location(
            &mut self,
            node: turboshaft::OpIndex,
            location: LinkageLocation,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            self.use_op(node, self.to_unallocated_operand(location, self.get_vreg(node)))
        }

        pub fn use_pointer_location(
            &mut self,
            to_location: LinkageLocation,
            from_location: LinkageLocation,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            let casted_from_operand =
                crate::compiler::backend::register_allocator::UnallocatedOperand::from_temp_location(
                    self.temp_location(from_location),
                );
            self.selector_.emit(
                InstructionCode::ArchNop,
                casted_from_operand.clone(),
                &[],
                0,
            );
            self.to_unallocated_operand(
                to_location,
                casted_from_operand.virtual_register(),
            )
        }

        pub fn temp_register(&mut self) -> crate::compiler::backend::instruction::InstructionOperand {
            crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                self.sequence().next_virtual_register(),
            )
        }

        pub fn allocate_virtual_register(&mut self) -> i32 {
            self.sequence().next_virtual_register()
        }

        pub fn define_same_as_first_for_vreg(&mut self, vreg: i32) -> crate::compiler::backend::instruction::InstructionOperand {
            crate::compiler::backend::register_allocator::UnallocatedOperand::new_same_as_input(vreg, 0)
        }

        pub fn define_as_register_for_vreg(&mut self, vreg: i32) -> crate::compiler::backend::instruction::InstructionOperand {
            crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(vreg)
        }

        pub fn use_register_for_vreg(&mut self, vreg: i32) -> crate::compiler::backend::instruction::InstructionOperand {
            crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                vreg,
            )
        }

        pub fn use_register_enum(
            &mut self,
            node: turboshaft::OpIndex,
            unique_reg: RegisterUseKind,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            match unique_reg {
                RegisterUseKind::kUseRegister => self.use_register(node),
                RegisterUseKind::kUseUniqueRegister => self.use_unique_register(node),
            }
        }

        pub fn temp_double_register(&mut self) -> crate::compiler::backend::instruction::InstructionOperand {
            let op = crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                self.sequence().next_virtual_register(),
            );
            self.sequence().mark_as_representation(
                MachineRepresentation::kFloat64,
                op.virtual_register(),
            );
            op
        }

        pub fn temp_simd128_register(&mut self) -> crate::compiler::backend::instruction::InstructionOperand {
            let op = crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                self.sequence().next_virtual_register(),
            );
            self.sequence().mark_as_representation(
                MachineRepresentation::kSimd128,
                op.virtual_register(),
            );
            op
        }

        pub fn temp_simd256_register(&mut self) -> crate::compiler::backend::instruction::InstructionOperand {
            let op = crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(
                crate::compiler::backend::register_allocator::UnallocatedOperand::USED_AT_START,
                self.sequence().next_virtual_register(),
            );
            self.sequence().mark_as_representation(
                MachineRepresentation::kSimd256,
                op.virtual_register(),
            );
            op
        }

        pub fn temp_register_reg(&mut self, reg: Register) -> crate::compiler::backend::instruction::InstructionOperand {
            crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_register(
                reg.code(),
                crate::compiler::backend::instruction::InstructionOperand::kInvalidVirtualRegister,
            )
        }

        pub fn temp_register_code(&mut self, code: i32) -> crate::compiler::backend::instruction::InstructionOperand {
            crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_register(
                code,
                self.sequence().next_virtual_register(),
            )
        }

        pub fn temp_fp_register<FPRegType: Copy + Clone>(
            &mut self,
            reg: FPRegType,
        ) -> crate::compiler::backend::instruction::InstructionOperand
        where
            i32: From<FPRegType>,
        {
            let op = crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_fp_register(
                i32::from(reg),
                self.sequence().next_virtual_register(),
            );
            self.sequence().mark_as_representation(
                MachineRepresentation::kSimd128,
                op.virtual_register(),
            );
            op
        }

        pub fn temp_immediate(&mut self, imm: i32) -> crate::compiler::backend::instruction::InstructionOperand {
            self.sequence()
                .add_immediate(crate::compiler::backend::code_generator_impl::Constant::new_int32(imm))
        }

        pub fn temp_location(&mut self, location: LinkageLocation) -> crate::compiler::backend::instruction::InstructionOperand {
            self.to_unallocated_operand(location, self.sequence().next_virtual_register())
        }

        pub fn label(&mut self, block: *mut turboshaft::Block) -> crate::compiler::backend::instruction::InstructionOperand {
            self.sequence()
                .add_immediate(crate::compiler::backend::code_generator_impl::Constant::new_int32(self.rpo_number(block) as i32))
        }

        fn selector(&self) -> &mut InstructionSelectorT<'a> {
            self.selector_
        }

        fn sequence(&mut self) -> &mut crate::compiler::backend::instruction::InstructionSequence {
            self.selector().sequence()
        }

        fn zone(&self) -> &Zone {
            self.selector().instruction_zone()
        }

        fn get_vreg(&self, node: turboshaft::OpIndex) -> i32 {
            self.selector_.get_virtual_register(node)
        }

        fn turboshaft_graph(&self) -> &TurboshaftGraph<'a> {
            self.turboshaft_adapter.turboshaft_graph()
        }

        fn to_constant(&self, node: turboshaft::OpIndex) -> crate::compiler::backend::code_generator_impl::Constant {
            if let Some(constant) = self
                .turboshaft_graph()
                .get(node)
                .try_cast::<turboshaft::ConstantOp>()
            {
                match constant.kind {
                    ConstantOp::Kind::kWord32 => {
                        crate::compiler::backend::code_generator_impl::Constant::new_int32(constant.word32() as i32)
                    }
                    ConstantOp::Kind::kWord64 => {
                        crate::compiler::backend::code_generator_impl::Constant::new_int64(constant.word64() as i64)
                    }
                    ConstantOp::Kind::kSmi => {
                        if crate::compiler::backend::move_optimizer::Is64() {
                            crate::compiler::backend::code_generator_impl::Constant::new_int64(constant.smi().ptr() as i64)
                        } else {
                            crate::compiler::backend::code_generator_impl::Constant::new_int32(constant.smi().ptr() as i32)
                        }
                    }
                    ConstantOp::Kind::kHeapObject
                    | ConstantOp::Kind::kCompressedHeapObject
                    | ConstantOp::Kind::kTrustedHeapObject => crate::compiler::backend::code_generator_impl::Constant::new_handle(
                        constant.handle(),
                        constant.kind == ConstantOp::Kind::kCompressedHeapObject,
                    ),
                    ConstantOp::Kind::kExternal => crate::compiler::backend::code_generator_impl::Constant::new_external_reference(constant.external_reference()),
                    ConstantOp::Kind::kNumber => crate::compiler::backend::code_generator_impl::Constant::new_float64(constant.number()),
                    ConstantOp::Kind::kFloat32 => crate::compiler::backend::code_generator_impl::Constant::new_float32(constant.float32()),
                    ConstantOp::Kind::kFloat64 => crate::compiler::backend::code_generator_impl::Constant::new_float64(constant.float64()),
                    ConstantOp::Kind::kTaggedIndex => {
                        let value = constant.tagged_index() as isize;
                        assert!(TaggedIndex::is_valid(value));
                        let tagged_index = TaggedIndex::from_intptr(value).ptr();
                        if kSystemPointerSize == kInt32Size {
                            crate::compiler::backend::code_generator_impl::Constant::new_int32(tagged_index as i32)
                        } else {
                            crate::compiler::backend::code_generator_impl::Constant::new_int64(tagged_index as i64)
                        }
                    }
                    ConstantOp::Kind::kRelocatableWasmCall | ConstantOp::Kind::kRelocatableWasmStubCall => {
                        let value = constant.integral();
                        let mode = if constant.kind == ConstantOp::Kind::kRelocatableWasmCall {
                            RelocInfo::WASM_CALL
                        } else {
                            RelocInfo::WASM_STUB_CALL
                        };
                        if crate::compiler::backend::move_optimizer::Is64() {
                            crate::compiler::backend::code_generator_impl::Constant::new_relocatable_ptr(
                                RelocatablePtrConstantInfo::new(value as i64, mode),
                            )
                        } else {
                            crate::compiler::backend::code_generator_impl::Constant::new_relocatable_ptr(
                                RelocatablePtrConstantInfo::new(value as i32, mode),
                            )
                        }
                    }
                    ConstantOp::Kind::kRelocatableWasmCanonicalSignatureId => crate::compiler::backend::code_generator_impl::Constant::new_relocatable_ptr(
                        RelocatablePtrConstantInfo::new(
                            constant.integral() as i32,
                            RelocInfo::WASM_CANONICAL_SIG_ID,
                        ),
                    ),
                    ConstantOp::Kind::kRelocatableWasmIndirectCallTarget => {
                        let value = constant.integral();
                        crate::compiler::backend::code_generator_impl::Constant::new_relocatable_ptr(
                            RelocatablePtrConstantInfo::new(
                                value as i32,
                                RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY,
                            ),
                        )
                    }
                }
            } else {
                UNREACHABLE()
            }
        }

        fn to_negated_constant(&self, node: turboshaft::OpIndex) -> crate::compiler::backend::code_generator_impl::Constant {
            let constant = self
                .turboshaft_graph()
                .get(node)
                .cast::<turboshaft::ConstantOp>();
            match constant.kind {
                ConstantOp::Kind::kWord32 => {
                    crate::compiler::backend::code_generator_impl::Constant::new_int32(-(constant.word32() as i32))
                }
                ConstantOp::Kind::kWord64 => {
                    crate::compiler::backend::code_generator_impl::Constant::new_int64(-(constant.word64() as i64))
                }
                ConstantOp::Kind::kSmi => {
                    if crate::compiler::backend::move_optimizer::Is64() {
                        crate::compiler::backend::code_generator_impl::Constant::new_int64(-(constant.smi().ptr() as i64))
                    } else {
                        crate::compiler::backend::code_generator_impl::Constant::new_int32(-(constant.smi().ptr() as i32))
                    }
                }
                _ => UNREACHABLE(),
            }
        }

        fn define(
            &mut self,
            node: turboshaft::OpIndex,
            operand: crate::compiler::backend::register_allocator::UnallocatedOperand,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            assert!(node.is_valid());
            assert_eq!(operand.virtual_register(), self.get_vreg(node));
            self.selector_.mark_as_defined(node);
            crate::compiler::backend::instruction::InstructionOperand::Unallocated(operand)
        }

        fn use_op(
            &mut self,
            node: turboshaft::OpIndex,
            operand: crate::compiler::backend::register_allocator::UnallocatedOperand,
        ) -> crate::compiler::backend::instruction::InstructionOperand {
            assert!(node.is_valid());
            assert_eq!(operand.virtual_register(), self.get_vreg(node));
            self.selector_.mark_as_used(node);
            crate::compiler::backend::instruction::InstructionOperand::Unallocated(operand)
        }

        fn to_dual_location_unallocated_operand(
            &self,
            primary_location: LinkageLocation,
            secondary_location: LinkageLocation,
            virtual_register: i32,
        ) -> crate::compiler::backend::register_allocator::UnallocatedOperand {
            assert!(primary_location.is_register() && secondary_location.is_callee_frame_slot());
            let reg_id = primary_location.as_register();
            let slot_id = secondary_location.as_callee_frame_slot();
            crate::compiler::backend::register_allocator::UnallocatedOperand::new_dual_location(
                reg_id,
                slot_id,
                virtual_register,
            )
        }

        fn to_unallocated_operand(
            &self,
            location: LinkageLocation,
            virtual_register: i32,
        ) -> crate::compiler::backend::register_allocator::UnallocatedOperand {
            if location.is_any_register() || location.is_null_register() {
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_must_have_register(virtual_register)
            } else if location.is_caller_frame_slot() {
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_slot(
                    location.as_caller_frame_slot(),
                    virtual_register,
                )
            } else if location.is_callee_frame_slot() {
                crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_slot(
                    location.as_callee_frame_slot(),
                    virtual_register,
                )
            } else {
                if crate::IsFloatingPoint(location.get_type().representation()) {
                    crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_fp_register(
                        location.as_register(),
                        virtual_register,
                    )
                } else {
                    crate::compiler::backend::register_allocator::UnallocatedOperand::new_fixed_register(
                        location.as_register(),
                        virtual_register,
                    )
                }
            }
        }

        fn rpo_number(&self, block: *mut turboshaft::Block) -> usize {
            self.selector_.rpo_number(block)
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum RegisterUseKind {
        kUseRegister,
        kUseUniqueRegister,
    }

    pub enum RegisterMode {
        kRegister,
        kUniqueRegister,
    }
}
