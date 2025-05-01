// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod instruction_selector_impl {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;

    // Placeholder for turboshaft::Block, replace with actual definition if available.
    #[derive(Debug, Clone, Copy)]
    pub struct Block {
        id: usize,
    }

    impl Block {
        pub fn new(id: usize) -> Self {
            Block { id }
        }
    }

    impl Eq for Block {}

    impl PartialEq for Block {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Hash for Block {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    /// Represents information about a single case in a switch statement.
    #[derive(Debug, Clone)]
    pub struct CaseInfoT {
        /// The case value.
        pub value: i32,
        /// The order for lowering to comparisons (less means earlier).
        pub order: i32,
        /// The basic blocks corresponding to the case value.
        pub branch: Block,
    }

    impl Ord for CaseInfoT {
        fn cmp(&self, other: &Self) -> Ordering {
            self.order.cmp(&other.order)
        }
    }

    impl PartialOrd for CaseInfoT {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for CaseInfoT {
        fn eq(&self, other: &Self) -> bool {
            self.order == other.order
        }
    }

    impl Eq for CaseInfoT {}

    /// Helper struct containing data about a table or lookup switch.
    #[derive(Debug, Clone)]
    pub struct SwitchInfoT {
        cases_: Vec<CaseInfoT>,
        min_value_: i32,
        max_value_: i32,
        value_range_: usize,
        default_branch_: Block,
    }

    impl SwitchInfoT {
        pub fn new(cases: &Vec<CaseInfoT>, min_value: i32, max_value: i32, default_branch: Block) -> Self {
            let value_range = if !cases.is_empty() {
                assert!(min_value <= max_value);
                // Mimic the C++ bit_cast using transmute.  This is crucial for
                // reproducing the exact calculation and behavior.
                let min_value_u32 = min_value as u32;
                let max_value_u32 = max_value as u32;
                1 + max_value_u32.wrapping_sub(min_value_u32) as usize
            } else {
                0
            };

            SwitchInfoT {
                cases_: cases.clone(),
                min_value_: min_value,
                max_value_: max_value,
                value_range_: value_range,
                default_branch_: default_branch,
            }
        }

        pub fn cases_sorted_by_value(&self) -> Vec<CaseInfoT> {
            let mut result = self.cases_.clone();
            result.sort_by(|a, b| a.value.cmp(&b.value));
            result
        }

        pub fn cases_unsorted(&self) -> &Vec<CaseInfoT> {
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

        pub fn default_branch(&self) -> Block {
            self.default_branch_
        }
    }

    // Placeholder types.  Replace with actual definitions.
    #[derive(Debug, Clone, Copy)]
    pub struct OpIndex {
        id: usize,
    }

    impl OpIndex {
        pub fn new(id: usize) -> Self {
            OpIndex { id }
        }

        pub fn valid(&self) -> bool {
            self.id != 0
        }
    }

    impl Default for OpIndex {
        fn default() -> Self {
            OpIndex{ id: 0 }
        }
    }

    impl Eq for OpIndex {}

    impl PartialEq for OpIndex {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Hash for OpIndex {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Register {
        code: i32,
    }

    impl Register {
        pub fn new(code: i32) -> Self {
            Register { code }
        }

        pub fn code(&self) -> i32 {
            self.code
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct FPRegType {
        code: i32,
    }

    impl FPRegType {
        pub fn new(code: i32) -> Self {
            FPRegType { code }
        }

        pub fn code(&self) -> i32 {
            self.code
        }
    }

    #[derive(Debug, Clone)]
    pub struct Constant(i64);

    impl Constant {
        pub fn new(value: i64) -> Self {
            Constant(value)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum MachineRepresentation {
        kFloat64,
        kSimd128,
        kSimd256,
        kWord32,
        kWord64,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct LinkageLocation {
        reg: i32,
        slot: i32,
        location_type: LocationType,
        representation: MachineRepresentation,
    }

    impl LinkageLocation {
        pub fn new(reg: i32, slot: i32, location_type: LocationType, representation: MachineRepresentation) -> Self {
            LinkageLocation { reg, slot, location_type, representation }
        }

        pub fn IsAnyRegister(&self) -> bool {
            self.location_type == LocationType::AnyRegister
        }

        pub fn IsNullRegister(&self) -> bool {
            self.location_type == LocationType::NullRegister
        }

        pub fn IsCallerFrameSlot(&self) -> bool {
            self.location_type == LocationType::CallerFrameSlot
        }

        pub fn IsCalleeFrameSlot(&self) -> bool {
            self.location_type == LocationType::CalleeFrameSlot
        }

        pub fn AsRegister(&self) -> i32 {
            self.reg
        }

        pub fn AsCalleeFrameSlot(&self) -> i32 {
            self.slot
        }

        pub fn GetType(&self) -> LocationValueType {
            LocationValueType { representation: self.representation }
        }

        pub fn IsRegister(&self) -> bool {
            self.location_type == LocationType::FixedRegister
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum LocationType {
        AnyRegister,
        NullRegister,
        CallerFrameSlot,
        CalleeFrameSlot,
        FixedRegister
    }

    #[derive(Debug, Clone, Copy)]
    pub struct LocationValueType {
        representation: MachineRepresentation
    }

    impl LocationValueType {
        pub fn representation(&self) -> MachineRepresentation {
            self.representation
        }
    }

    fn IsFloatingPoint(representation: MachineRepresentation) -> bool {
        match representation {
            MachineRepresentation::kFloat64 => true,
            _ => false
        }
    }

    // Replace with actual InstructionOperand definition
    #[derive(Debug, Clone)]
    pub struct InstructionOperand {}

    impl InstructionOperand {
        pub fn new() -> Self {
            InstructionOperand {}
        }
    }

    #[derive(Debug, Clone)]
    pub struct UnallocatedOperand {
        kind: UnallocatedOperandKind,
        flags: UnallocatedOperandFlag,
        virtual_register: i32,
        reg_code: i32,
        slot_id: i32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum UnallocatedOperandKind {
        NONE,
        MUST_HAVE_REGISTER,
        FIXED_REGISTER,
        FIXED_FP_REGISTER,
        MUST_HAVE_SLOT,
        REGISTER_OR_SLOT,
        REGISTER_OR_SLOT_OR_CONSTANT,
        SAME_AS_INPUT,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum UnallocatedOperandFlag {
        NONE,
        USED_AT_START,
        USED_AT_END,
    }

    impl UnallocatedOperand {
        pub fn new(kind: UnallocatedOperandKind, flags: UnallocatedOperandFlag, virtual_register: i32) -> Self {
            UnallocatedOperand { kind, flags, virtual_register, reg_code: 0, slot_id: 0 }
        }

        pub fn new_fixed(kind: UnallocatedOperandKind, reg_code: i32, virtual_register: i32) -> Self {
            UnallocatedOperand { kind, flags: UnallocatedOperandFlag::NONE, virtual_register, reg_code, slot_id: 0 }
        }

        pub fn new_slot(kind: UnallocatedOperandKind, slot_id: i32, virtual_register: i32) -> Self {
            UnallocatedOperand { kind, flags: UnallocatedOperandFlag::NONE, virtual_register, reg_code: 0, slot_id }
        }

        pub fn new_same_as_input(virtual_register: i32, input_index: i32) -> Self {
            UnallocatedOperand { kind: UnallocatedOperandKind::SAME_AS_INPUT, flags: UnallocatedOperandFlag::NONE, virtual_register, reg_code: input_index, slot_id: 0 }
        }

        pub fn new_kind(kind: UnallocatedOperandKind, virtual_register: i32) -> Self {
            UnallocatedOperand { kind, flags: UnallocatedOperandFlag::NONE, virtual_register, reg_code: 0, slot_id: 0 }
        }

        pub fn virtual_register(&self) -> i32 {
            self.virtual_register
        }

        // TODO: Properly implement cast
        pub fn cast(temp_location: Self) -> Self {
            temp_location
        }
    }

    #[derive(Debug, Clone)]
    pub struct ConstantOperand(i32);

    impl ConstantOperand {
        pub fn new(virtual_register: i32) -> Self {
            ConstantOperand(virtual_register)
        }
    }

    // Placeholder
    pub mod reloc_info {
        #[derive(Debug, Clone, Copy)]
        pub enum Mode {
            WASM_CALL,
            WASM_STUB_CALL,
            WASM_CANONICAL_SIG_ID,
            WASM_CODE_POINTER_TABLE_ENTRY
        }
    }

    #[derive(Debug, Clone)]
    pub struct RelocatablePtrConstantInfo {
        value: i32,
        mode: reloc_info::Mode,
    }

    impl RelocatablePtrConstantInfo {
        pub fn new(value: i32, mode: reloc_info::Mode) -> Self {
            RelocatablePtrConstantInfo { value, mode }
        }
    }

    /// A helper class for the instruction selector that simplifies construction of
    /// Operands. This class implements a base for architecture-specific helpers.
    pub struct OperandGeneratorT<'a, T: InstructionSelectorT> {
        turboshaft_adapter: TurboshaftAdapter<'a, T>,
        selector_: &'a T,
    }

    impl<'a, T: InstructionSelectorT> OperandGeneratorT<'a, T> {
        pub fn new(selector: &'a T) -> Self {
            OperandGeneratorT {
                turboshaft_adapter: TurboshaftAdapter::new(selector.schedule()),
                selector_: selector,
            }
        }

        pub fn no_output(&self) -> InstructionOperand {
            InstructionOperand::new() // Generates an invalid operand.
        }

        pub fn define_as_register(&self, node: OpIndex) -> InstructionOperand {
            self.define(
                node,
                UnallocatedOperand::new(
                    UnallocatedOperandKind::MUST_HAVE_REGISTER,
                    UnallocatedOperandFlag::NONE,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn define_same_as_input(&self, node: OpIndex, input_index: i32) -> InstructionOperand {
            self.define(
                node,
                UnallocatedOperand::new_same_as_input(self.get_vreg(node), input_index),
            )
        }

        pub fn define_same_as_first(&self, node: OpIndex) -> InstructionOperand {
            self.define_same_as_input(node, 0)
        }

        pub fn define_as_fixed(&self, node: OpIndex, reg: Register) -> InstructionOperand {
            self.define(
                node,
                UnallocatedOperand::new_fixed(
                    UnallocatedOperandKind::FIXED_REGISTER,
                    reg.code(),
                    self.get_vreg(node),
                ),
            )
        }

        pub fn define_as_fixed_fp(&self, node: OpIndex, reg: FPRegType) -> InstructionOperand {
            self.define(
                node,
                UnallocatedOperand::new_fixed(
                    UnallocatedOperandKind::FIXED_FP_REGISTER,
                    reg.code(),
                    self.get_vreg(node),
                ),
            )
        }

        pub fn define_as_constant(&self, node: OpIndex) -> InstructionOperand {
            self.selector().mark_as_defined(node);
            let virtual_register = self.get_vreg(node);
            self.sequence().add_constant(virtual_register, self.to_constant(node));
            ConstantOperand::new(virtual_register).into()
        }

        // TODO: Add impl InstructionOperand
        pub fn define_as_location(&self, node: OpIndex, location: LinkageLocation) -> InstructionOperand {
            self.define(
                node,
                self.to_unallocated_operand(location, self.get_vreg(node)),
            )
            .into()
        }

        pub fn define_as_dual_location(
            &self,
            node: OpIndex,
            primary_location: LinkageLocation,
            secondary_location: LinkageLocation,
        ) -> InstructionOperand {
            self.define(
                node,
                self.to_dual_location_unallocated_operand(
                    primary_location,
                    secondary_location,
                    self.get_vreg(node),
                ),
            )
            .into()
        }

        pub fn use_(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new(
                    UnallocatedOperandKind::NONE,
                    UnallocatedOperandFlag::USED_AT_START,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_any_at_end(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new(
                    UnallocatedOperandKind::REGISTER_OR_SLOT,
                    UnallocatedOperandFlag::USED_AT_END,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_any(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new(
                    UnallocatedOperandKind::REGISTER_OR_SLOT,
                    UnallocatedOperandFlag::USED_AT_START,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_register_or_slot_or_constant(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new(
                    UnallocatedOperandKind::REGISTER_OR_SLOT_OR_CONSTANT,
                    UnallocatedOperandFlag::USED_AT_START,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_unique_register_or_slot_or_constant(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new_kind(
                    UnallocatedOperandKind::REGISTER_OR_SLOT_OR_CONSTANT,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_register(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new(
                    UnallocatedOperandKind::MUST_HAVE_REGISTER,
                    UnallocatedOperandFlag::USED_AT_START,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_register_at_end(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new(
                    UnallocatedOperandKind::MUST_HAVE_REGISTER,
                    UnallocatedOperandFlag::USED_AT_END,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_unique_slot(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new_kind(
                    UnallocatedOperandKind::MUST_HAVE_SLOT,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_unique(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new_kind(UnallocatedOperandKind::NONE, self.get_vreg(node)),
            )
        }

        pub fn use_unique_register(&self, node: OpIndex) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new(
                    UnallocatedOperandKind::MUST_HAVE_REGISTER,
                    UnallocatedOperandFlag::NONE,
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_register_with_mode(
            &self,
            node: OpIndex,
            unique_reg: RegisterUseKind,
        ) -> InstructionOperand {
            match unique_reg {
                RegisterUseKind::kUseRegister => self.use_register(node),
                RegisterUseKind::kUseUniqueRegister => self.use_unique_register(node),
            }
        }

        pub fn use_fixed(&self, node: OpIndex, reg: Register) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new_fixed(
                    UnallocatedOperandKind::FIXED_REGISTER,
                    reg.code(),
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_fixed_fp(&self, node: OpIndex, reg: FPRegType) -> InstructionOperand {
            self.use_(
                node,
                UnallocatedOperand::new_fixed(
                    UnallocatedOperandKind::FIXED_FP_REGISTER,
                    reg.code(),
                    self.get_vreg(node),
                ),
            )
        }

        pub fn use_immediate(&self, immediate: i32) -> InstructionOperand {
            self.sequence().add_immediate(Constant::new(immediate as i64)).into()
        }

        pub fn use_immediate64(&self, immediate: i64) -> InstructionOperand {
            self.sequence().add_immediate(Constant::new(immediate)).into()
        }

        pub fn use_immediate_node(&self, node: OpIndex) -> InstructionOperand {
            self.sequence().add_immediate(self.to_constant(node)).into()
        }

        pub fn use_negated_immediate(&self, node: OpIndex) -> InstructionOperand {
            self.sequence().add_immediate(self.to_negated_constant(node)).into()
        }

        pub fn use_location(&self, node: OpIndex, location: LinkageLocation) -> InstructionOperand {
            self.use_(
                node,
                self.to_unallocated_operand(location, self.get_vreg(node)),
            )
        }

        // TODO: Add Emit method to instruction selector
        pub fn use_pointer_location(&self, to_location: LinkageLocation, from_location: LinkageLocation) -> InstructionOperand {
            let casted_from_operand = UnallocatedOperand::cast(self.temp_location(from_location));
            self.selector().emit(ArchNop::kNop, casted_from_operand);
            self.to_unallocated_operand(to_location, casted_from_operand.virtual_register())
        }

        pub fn temp_register(&self) -> InstructionOperand {
            UnallocatedOperand::new(
                UnallocatedOperandKind::MUST_HAVE_REGISTER,
                UnallocatedOperandFlag::USED_AT_START,
                self.sequence().next_virtual_register(),
            )
            .into()
        }

        pub fn allocate_virtual_register(&self) -> i32 {
            self.sequence().next_virtual_register()
        }

        pub fn define_same_as_first_for_vreg(&self, vreg: i32) -> InstructionOperand {
            UnallocatedOperand::new_same_as_input(vreg, 0).into()
        }

        pub fn define_as_registert_for_vreg(&self, vreg: i32) -> InstructionOperand {
            UnallocatedOperand::new(UnallocatedOperandKind::MUST_HAVE_REGISTER, UnallocatedOperandFlag::NONE, vreg).into()
        }

        pub fn use_register_for_vreg(&self, vreg: i32) -> InstructionOperand {
            UnallocatedOperand::new(
                UnallocatedOperandKind::MUST_HAVE_REGISTER,
                UnallocatedOperandFlag::USED_AT_START,
                vreg,
            )
            .into()
        }

        pub fn use_register_with_mode2(
            &self,
            node: OpIndex,
            register_mode: RegisterMode,
        ) -> InstructionOperand {
            match register_mode {
                RegisterMode::kRegister => self.use_register(node),
                RegisterMode::kUniqueRegister => self.use_unique_register(node),
            }
        }

        pub fn temp_double_register(&self) -> InstructionOperand {
            let mut op = UnallocatedOperand::new(
                UnallocatedOperandKind::MUST_HAVE_REGISTER,
                UnallocatedOperandFlag::USED_AT_START,
                self.sequence().next_virtual_register(),
            );
            self.sequence()
                .mark_as_representation(MachineRepresentation::kFloat64, op.virtual_register());
            op.into()
        }

        pub fn temp_simd128_register(&self) -> InstructionOperand {
            let mut op = UnallocatedOperand::new(
                UnallocatedOperandKind::MUST_HAVE_REGISTER,
                UnallocatedOperandFlag::USED_AT_START,
                self.sequence().next_virtual_register(),
            );
            self.sequence()
                .mark_as_representation(MachineRepresentation::kSimd128, op.virtual_register());
            op.into()
        }

        pub fn temp_simd256_register(&self) -> InstructionOperand {
            let mut op = UnallocatedOperand::new(
                UnallocatedOperandKind::MUST_HAVE_REGISTER,
                UnallocatedOperandFlag::USED_AT_START,
                self.sequence().next_virtual_register(),
            );
            self.sequence()
                .mark_as_representation(MachineRepresentation::kSimd256, op.virtual_register());
            op.into()
        }

        pub fn temp_register_reg(&self, reg: Register) -> InstructionOperand {
            UnallocatedOperand::new_fixed(
                UnallocatedOperandKind::FIXED_REGISTER,
                reg.code(),
                InstructionOperand::kInvalidVirtualRegister,
            )
            .into()
        }

        pub fn temp_register_code(&self, code: i32) -> InstructionOperand {
            UnallocatedOperand::new_fixed(
                UnallocatedOperandKind::FIXED_REGISTER,
                code,
                self.sequence().next_virtual_register(),
            )
            .into()
        }

        pub fn temp_fp_register(&self, reg: FPRegType) -> InstructionOperand {
            let mut op = UnallocatedOperand::new_fixed(
                UnallocatedOperandKind::FIXED_FP_REGISTER,
                reg.code(),
                self.sequence().next_virtual_register(),
            );
            self.sequence()
                .mark_as_representation(MachineRepresentation::kSimd128, op.virtual_register());
            op.into()
        }

        pub fn temp_immediate(&self, imm: i32) -> InstructionOperand {
            self.sequence().add_immediate(Constant::new(imm as i64)).into()
        }

        pub fn temp_location(&self, location: LinkageLocation) -> InstructionOperand {
            self.to_unallocated_operand(location, self.sequence().next_virtual_register()).into()
        }

        pub fn label(&self, block: Block) -> InstructionOperand {
            self.sequence()
                .add_immediate(Constant::new(self.turboshaft_adapter.rpo_number(&block) as i64))
                .into()
        }

        fn selector(&self) -> &T {
            self.selector_
        }

        fn sequence(&self) -> &InstructionSequence {
            self.selector().sequence()
        }

        fn get_vreg(&self, node: OpIndex) -> i32 {
            self.selector().get_virtual_register(node)
        }

        fn to_constant(&self, node: OpIndex) -> Constant {
            use constant_op::Kind;
            if let Some(constant) = self
                .turboshaft_adapter
                .turboshaft_graph()
                .get(node)
                .try_cast::<constant_op::ConstantOp>()
            {
                match constant.kind {
                    Kind::kWord32 => Constant::new(constant.word32() as i64),
                    Kind::kWord64 => Constant::new(constant.word64() as i64),
                    Kind::kSmi => {
                        if cfg!(target_pointer_width = "64") {
                            Constant::new(constant.smi().ptr() as i64)
                        } else {
                            Constant::new(constant.smi().ptr() as i32 as i64)
                        }
                    }
                    Kind::kHeapObject | Kind::kCompressedHeapObject | Kind::kTrustedHeapObject => {
                        todo!()
                        //Constant(constant->handle(), constant->kind == Kind::kCompressedHeapObject);
                    }
                    Kind::kExternal => {
                        todo!()
                        //Constant(constant->external_reference());
                    }
                    Kind::kNumber => {
                        todo!()
                        //Constant(constant->number());
                    }
                    Kind::kFloat32 => {
                        todo!()
                        //Constant(constant->float32());
                    }
                    Kind::kFloat64 => {
                        todo!()
                        //Constant(constant->float64());
                    }
                    Kind::kTaggedIndex => {
                        todo!()
                        //Unencoded index value.
                        //intptr_t value = static_cast<intptr_t>(constant->tagged_index());
                        //DCHECK(TaggedIndex::IsValid(value));
                        //Generate it as 32/64-bit constant in a tagged form.
                        //Address tagged_index = TaggedIndex::FromIntptr(value).ptr();
                        //if (kSystemPointerSize == kInt32Size) {
                        //  return Constant(static_cast<int32_t>(tagged_index));
                        //} else {
                        //  return Constant(static_cast<int64_t>(tagged_index));
                        //}
                    }
                    Kind::kRelocatableWasmCall | Kind::kRelocatableWasmStubCall => {
                        let value = constant.integral();
                        let mode = if constant.kind == Kind::kRelocatableWasmCall {
                            reloc_info::Mode::WASM_CALL
                        } else {
                            reloc_info::Mode::WASM_STUB_CALL
                        };
                        //using constant_type = std::conditional_t<Is64(), int64_t, int32_t>;
                        let value_i32 = value as i32;
                        Constant::new(RelocatablePtrConstantInfo::new(value_i32, mode).value as i64)
                    }
                    Kind::kRelocatableWasmCanonicalSignatureId => {
                        todo!()
                        //Constant(RelocatablePtrConstantInfo(
                        //  base::checked_cast<int32_t>(constant->integral()),
                        //  RelocInfo::WASM_CANONICAL_SIG_ID));
                    }
                    Kind::kRelocatableWasmIndirectCallTarget => {
                        todo!()
                        //uint64_t value = constant->integral();
                        //return Constant(RelocatablePtrConstantInfo(
                        //  base::checked_cast<int32_t>(value),
                        //  RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY));
                    }
                }
            } else {
                unreachable!()
            }
        }

        fn to_negated_constant(&self, node: OpIndex) -> Constant {
            let constant = self
                .turboshaft_adapter
                .turboshaft_graph()
                .get(node)
                .cast::<constant_op::ConstantOp>();
            match constant.kind {
                constant_op::Kind::kWord32 => Constant::new(-(constant.word32() as i64)),
                constant_op::Kind::kWord64 => Constant::new(-(constant.word64() as i64)),
                constant_op::Kind::kSmi => {
                    if cfg!(target_pointer_width = "64") {
                        Constant::new(-(constant.smi().ptr() as i64))
                    } else {
                        Constant::new(-(constant.smi().ptr() as i32 as i64))
                    }
                }
                _ => unreachable!(),
            }
        }

        fn define(&self, node: OpIndex, operand: UnallocatedOperand) -> UnallocatedOperand {
            assert!(node.valid());
            assert_eq!(operand.virtual_register(), self.get_vreg(node));
            self.selector().mark_as_defined(node);
            operand
        }

        fn use_(&self, node: OpIndex, operand: UnallocatedOperand) -> InstructionOperand {
            assert!(node.valid());
            assert_eq!(operand.virtual_register(), self.get_vreg(node));
            self.selector().mark_as_used(node);
            operand.into()
        }

        fn to_dual_location_unallocated_operand(
            &self,
            primary_location: LinkageLocation,
            secondary_location: LinkageLocation,
            virtual_register: i32,
        ) -> UnallocatedOperand {
            assert!(primary_location.IsRegister() && secondary_location.IsCalleeFrameSlot());
            let reg_id = primary_location.AsRegister();
            let slot_id = secondary_location.AsCalleeFrameSlot();
            UnallocatedOperand { kind: UnallocatedOperandKind::NONE, flags: UnallocatedOperandFlag::NONE, virtual_register, reg_code: reg_id, slot_id }
        }

        fn to_unallocated_operand(&self, location: LinkageLocation, virtual_register: i32) -> UnallocatedOperand {
            if location.IsAnyRegister() || location.IsNullRegister() {
                UnallocatedOperand::new(
                    UnallocatedOperandKind::MUST_HAVE_REGISTER,
                    UnallocatedOperandFlag::NONE,
                    virtual_register,
                )
            } else if location.IsCallerFrameSlot() {
                UnallocatedOperand::new_slot(
                    UnallocatedOperandKind::FIXED_REGISTER,
                    location.AsCallerFrameSlot(),
                    virtual_register,
                )
            } else if location.IsCalleeFrameSlot() {
                UnallocatedOperand::new_slot(
                    UnallocatedOperandKind::FIXED_REGISTER,
                    location.AsCalleeFrameSlot(),
                    virtual_register,
                )
            } else {
                let reg = location.AsRegister();
                if IsFloatingPoint(location.GetType().representation()) {
                    UnallocatedOperand::new_fixed(
                        UnallocatedOperandKind::FIXED_FP_REGISTER,
                        reg,
                        virtual_register,
                    )
                } else {
                    UnallocatedOperand::new_fixed(
                        UnallocatedOperandKind::FIXED_REGISTER,
                        reg,
                        virtual_register,
                    )
                }
            }
        }
    }

    impl From<UnallocatedOperand> for InstructionOperand {
        fn from(_: UnallocatedOperand) -> Self {
            InstructionOperand::new()
        }
    }

    impl From<ConstantOperand> for InstructionOperand {
        fn from(_: ConstantOperand) -> Self {
            InstructionOperand::new()
        }
    }

    impl InstructionOperand {
        const kInvalidVirtual