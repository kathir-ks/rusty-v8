// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod instructions {
    use std::any::Any;
    use std::fmt;
    use std::ops::Deref;
    use std::rc::Rc;
    use std::string::String;
    use std::vec::Vec;

    use crate::ast;
    use crate::source_positions::SourcePosition;
    use crate::types::Type;
    use crate::utils::Worklist;

    pub type Stack<T> = Vec<T>;
    pub type TypeVector = Vec<Rc<Type>>; // Assuming Type is behind a pointer
    pub type StackRange = std::ops::Range<usize>;

    pub trait InstructionTrait: Any {
        fn clone_box(&self) -> Box<dyn InstructionTrait>;
        fn assign(&mut self, other: &dyn InstructionTrait);
        fn type_instruction(&self, stack: &mut Stack<Rc<Type>>, cfg: &mut ControlFlowGraph);
        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            worklist: &mut Worklist<Block>,
        );
        fn invalidate_transient_types(&self, stack: &mut Stack<Rc<Type>>) {
            // Default implementation, can be overridden
        }
        fn is_block_terminator(&self) -> bool {
            false
        }
        fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {}
        fn get_source_position(&self) -> SourcePosition;
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    impl<T: 'static + InstructionBase + Clone> InstructionTrait for T {
        fn clone_box(&self) -> Box<dyn InstructionTrait> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn InstructionTrait) {
            if let Some(other_typed) = other.as_any().downcast_ref::<Self>() {
                *self = other_typed.clone();
            } else {
                panic!("Attempted to assign incompatible Instruction types");
            }
        }

        fn type_instruction(&self, stack: &mut Stack<Rc<Type>>, cfg: &mut ControlFlowGraph) {
            InstructionBase::type_instruction(self, stack, cfg);
        }

        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            worklist: &mut Worklist<Block>,
        ) {
            InstructionBase::recompute_definition_locations(self, locations, worklist);
        }

        fn is_block_terminator(&self) -> bool {
            InstructionBase::is_block_terminator(self)
        }

        fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {
            InstructionBase::append_successor_blocks(self, block_list);
        }

        fn get_source_position(&self) -> SourcePosition {
            self.pos.clone()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    macro_rules! define_instructions {
        ($($name:ident,)*) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum InstructionKind {
                $(
                    $name,
                )*
            }

            impl InstructionKind {
                pub fn mnemonic(&self) -> &'static str {
                    match self {
                        $(
                            InstructionKind::$name => stringify!($name),
                        )*
                    }
                }
            }
        }
    }

    define_instructions! {
        PeekInstruction,
        PokeInstruction,
        DeleteRangeInstruction,
        PushUninitializedInstruction,
        PushBuiltinPointerInstruction,
        LoadReferenceInstruction,
        StoreReferenceInstruction,
        LoadBitFieldInstruction,
        StoreBitFieldInstruction,
        CallCsaMacroInstruction,
        CallIntrinsicInstruction,
        NamespaceConstantInstruction,
        CallCsaMacroAndBranchInstruction,
        CallBuiltinInstruction,
        CallRuntimeInstruction,
        CallBuiltinPointerInstruction,
        BranchInstruction,
        ConstexprBranchInstruction,
        GotoInstruction,
        GotoExternalInstruction,
        MakeLazyNodeInstruction,
        ReturnInstruction,
        PrintErrorInstruction,
        AbortInstruction,
        UnsafeCastInstruction,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum DefinitionLocation {
        Invalid,
        Parameter { index: usize },
        Phi { block: *const Block, index: usize },
        Instruction { instruction: *const dyn InstructionTrait, index: usize },
    }

    impl DefinitionLocation {
        pub fn parameter(index: usize) -> Self {
            DefinitionLocation::Parameter { index }
        }

        pub fn phi(block: *const Block, index: usize) -> Self {
            DefinitionLocation::Phi { block, index }
        }

        pub fn instruction(instruction: *const dyn InstructionTrait, index: usize) -> Self {
            DefinitionLocation::Instruction { instruction, index }
        }

        pub fn get_kind(&self) -> &Self {
            &self
        }

        pub fn is_valid(&self) -> bool {
            match self {
                DefinitionLocation::Invalid => false,
                _ => true,
            }
        }

        pub fn is_parameter(&self) -> bool {
            matches!(self, DefinitionLocation::Parameter { .. })
        }

        pub fn is_phi(&self) -> bool {
            matches!(self, DefinitionLocation::Phi { .. })
        }

        pub fn is_instruction(&self) -> bool {
            matches!(self, DefinitionLocation::Instruction { .. })
        }

        pub fn get_parameter_index(&self) -> usize {
            match self {
                DefinitionLocation::Parameter { index } => *index,
                _ => panic!("Not a parameter location"),
            }
        }

        pub fn get_phi_block(&self) -> *const Block {
            match self {
                DefinitionLocation::Phi { block, .. } => *block,
                _ => panic!("Not a phi location"),
            }
        }

        pub fn is_phi_from_block(&self, block: *const Block) -> bool {
            match self {
                DefinitionLocation::Phi { block: b, .. } => *b == block,
                _ => false,
            }
        }

        pub fn get_phi_index(&self) -> usize {
            match self {
                DefinitionLocation::Phi { index, .. } => *index,
                _ => panic!("Not a phi location"),
            }
        }

        pub fn get_instruction(&self) -> *const dyn InstructionTrait {
            match self {
                DefinitionLocation::Instruction { instruction, .. } => *instruction,
                _ => panic!("Not an instruction location"),
            }
        }

        pub fn get_instruction_index(&self) -> usize {
            match self {
                DefinitionLocation::Instruction { index, .. } => *index,
                _ => panic!("Not an instruction location"),
            }
        }
    }

    impl fmt::Display for DefinitionLocation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DefinitionLocation::Invalid => write!(f, "DefinitionLocation::Invalid()"),
                DefinitionLocation::Parameter { index } => {
                    write!(f, "DefinitionLocation::Parameter({})", index)
                }
                DefinitionLocation::Phi { block, index } => {
                    write!(f, "DefinitionLocation::Phi({:p}, {})", block, index)
                }
                DefinitionLocation::Instruction { instruction, index } => {
                    write!(f, "DefinitionLocation::Instruction({:p}, {})", instruction, index)
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Instruction {
        pub kind: InstructionKind,
        pub instruction: Box<dyn InstructionTrait>,
    }

    impl Instruction {
        pub fn new<T: 'static + InstructionBase + Clone>(instr: T) -> Self {
            Instruction {
                kind: T::KIND,
                instruction: Box::new(instr),
            }
        }

        pub fn cast<T: 'static + InstructionBase>(&self) -> &T {
            if let Some(t) = self.instruction.as_any().downcast_ref::<T>() {
                t
            } else {
                panic!("Failed to cast to {}", std::any::type_name::<T>());
            }
        }

        pub fn cast_mut<T: 'static + InstructionBase>(&mut self) -> &mut T {
            if let Some(t) = self.instruction.as_any_mut().downcast_mut::<T>() {
                t
            } else {
                panic!("Failed to cast to {}", std::any::type_name::<T>());
            }
        }

        pub fn is<T: 'static + InstructionBase>(&self) -> bool {
            self.kind == T::KIND
        }

        pub fn dynamic_cast<T: 'static + InstructionBase>(&self) -> Option<&T> {
            self.instruction.as_any().downcast_ref::<T>()
        }

        pub fn dynamic_cast_mut<T: 'static + InstructionBase>(&mut self) -> Option<&mut T> {
            self.instruction.as_any_mut().downcast_mut::<T>()
        }

        pub fn type_instruction(&self, stack: &mut Stack<Rc<Type>>, cfg: &mut ControlFlowGraph) {
            self.instruction.type_instruction(stack, cfg);
        }

        pub fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            worklist: &mut Worklist<Block>,
        ) {
            self.instruction
                .recompute_definition_locations(locations, worklist);
        }

        pub fn mnemonic(&self) -> &'static str {
            self.kind.mnemonic()
        }
    }

    #[derive(Clone, Debug)]
    pub struct InstructionBaseData {
        pub pos: SourcePosition,
    }

    pub trait InstructionBase: InstructionTrait + fmt::Debug {
        const KIND: InstructionKind;
        fn type_instruction(&self, stack: &mut Stack<Rc<Type>>, cfg: &mut ControlFlowGraph);
        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            worklist: &mut Worklist<Block>,
        );
        fn is_block_terminator(&self) -> bool {
            false
        }
        fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {}
        fn get_pos(&self) -> &SourcePosition;
    }

    macro_rules! implement_instruction_base {
        ($struct_name:ident) => {
            impl InstructionBase for $struct_name {
                const KIND: InstructionKind = InstructionKind::$struct_name;

                fn type_instruction(&self, _stack: &mut Stack<Rc<Type>>, _cfg: &mut ControlFlowGraph) {}
                fn recompute_definition_locations(
                    &self,
                    _locations: &mut Stack<DefinitionLocation>,
                    _worklist: &mut Worklist<Block>,
                ) {
                }
                fn get_pos(&self) -> &SourcePosition {
                    &self.pos
                }
            }
        };
    }

    pub type BottomOffset = usize; // Replace with actual type if needed.

    #[derive(Clone, Debug)]
    pub struct PeekInstruction {
        pub pos: SourcePosition,
        pub slot: BottomOffset,
        pub widened_type: Option<Rc<Type>>,
    }

    impl PeekInstruction {
        pub const KIND: InstructionKind = InstructionKind::PeekInstruction;
    }
    implement_instruction_base!(PeekInstruction);

    impl fmt::Display for PeekInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "Peek {}", self.slot)?;
            if let Some(widened_type) = &self.widened_type {
                write!(os, ", {}", widened_type)?;
            }
            Ok(())
        }
    }

    #[derive(Clone, Debug)]
    pub struct PokeInstruction {
        pub pos: SourcePosition,
        pub slot: BottomOffset,
        pub widened_type: Option<Rc<Type>>,
    }

    impl PokeInstruction {
        pub const KIND: InstructionKind = InstructionKind::PokeInstruction;
    }
    implement_instruction_base!(PokeInstruction);

    impl fmt::Display for PokeInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "Poke {}", self.slot)?;
            if let Some(widened_type) = &self.widened_type {
                write!(os, ", {}", widened_type)?;
            }
            Ok(())
        }
    }

    #[derive(Clone, Debug)]
    pub struct DeleteRangeInstruction {
        pub pos: SourcePosition,
        pub range: StackRange,
    }
    impl DeleteRangeInstruction {
        pub const KIND: InstructionKind = InstructionKind::DeleteRangeInstruction;
    }
    implement_instruction_base!(DeleteRangeInstruction);

    impl fmt::Display for DeleteRangeInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "DeleteRange {:?}", self.range)
        }
    }

    #[derive(Clone, Debug)]
    pub struct PushUninitializedInstruction {
        pub pos: SourcePosition,
        pub type_: Rc<Type>,
    }

    impl PushUninitializedInstruction {
        pub const KIND: InstructionKind = InstructionKind::PushUninitializedInstruction;
        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(PushUninitializedInstruction);

    impl fmt::Display for PushUninitializedInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "PushUninitialized {}", self.type_)
        }
    }

    #[derive(Clone, Debug)]
    pub struct PushBuiltinPointerInstruction {
        pub pos: SourcePosition,
        pub external_name: String,
        pub type_: Rc<Type>,
    }

    impl PushBuiltinPointerInstruction {
        pub const KIND: InstructionKind = InstructionKind::PushBuiltinPointerInstruction;
        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(PushBuiltinPointerInstruction);

    impl fmt::Display for PushBuiltinPointerInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                os,
                "PushBuiltinPointer {:?}, {}",
                self.external_name, self.type_
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct NamespaceConstantInstruction {
        pub pos: SourcePosition,
        pub constant: *mut NamespaceConstant,
    }

    impl NamespaceConstantInstruction {
        pub const KIND: InstructionKind = InstructionKind::NamespaceConstantInstruction;
        pub fn get_value_definition_count(&self) -> usize {
            0 //TODO: Implement
        }
        pub fn get_value_definition(&self, _index: usize) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(NamespaceConstantInstruction);

    impl fmt::Display for NamespaceConstantInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "NamespaceConstant {:?}", self.constant)
        }
    }

    #[derive(Clone, Debug)]
    pub struct LoadReferenceInstruction {
        pub pos: SourcePosition,
        pub type_: Rc<Type>,
        pub synchronization: FieldSynchronization,
    }

    impl LoadReferenceInstruction {
        pub const KIND: InstructionKind = InstructionKind::LoadReferenceInstruction;
        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(LoadReferenceInstruction);

    impl fmt::Display for LoadReferenceInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "LoadReference {}", self.type_)
        }
    }

    #[derive(Clone, Debug)]
    pub struct StoreReferenceInstruction {
        pub pos: SourcePosition,
        pub type_: Rc<Type>,
    }

    impl StoreReferenceInstruction {
        pub const KIND: InstructionKind = InstructionKind::StoreReferenceInstruction;
    }
    implement_instruction_base!(StoreReferenceInstruction);

    impl fmt::Display for StoreReferenceInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "StoreReference {}", self.type_)
        }
    }

    #[derive(Clone, Debug)]
    pub struct LoadBitFieldInstruction {
        pub pos: SourcePosition,
        pub bit_field_struct_type: Rc<Type>,
        pub bit_field: BitField,
    }

    impl LoadBitFieldInstruction {
        pub const KIND: InstructionKind = InstructionKind::LoadBitFieldInstruction;
        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(LoadBitFieldInstruction);

    impl fmt::Display for LoadBitFieldInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                os,
                "LoadBitField {}, {}",
                self.bit_field_struct_type, self.bit_field.name_and_type.name
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct StoreBitFieldInstruction {
        pub pos: SourcePosition,
        pub bit_field_struct_type: Rc<Type>,
        pub bit_field: BitField,
        pub starts_as_zero: bool,
    }

    impl StoreBitFieldInstruction {
        pub const KIND: InstructionKind = InstructionKind::StoreBitFieldInstruction;
        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(StoreBitFieldInstruction);

    impl fmt::Display for StoreBitFieldInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                os,
                "StoreBitField {}, {}",
                self.bit_field_struct_type, self.bit_field.name_and_type.name
            )?;
            if self.starts_as_zero {
                write!(os, ", starts_as_zero")?;
            }
            Ok(())
        }
    }

    #[derive(Clone, Debug)]
    pub struct CallIntrinsicInstruction {
        pub pos: SourcePosition,
        pub intrinsic: *mut Intrinsic,
        pub specialization_types: TypeVector,
        pub constexpr_arguments: Vec<String>,
    }

    impl CallIntrinsicInstruction {
        pub const KIND: InstructionKind = InstructionKind::CallIntrinsicInstruction;
        pub fn get_value_definition_count(&self) -> usize {
            0 //TODO: Implement
        }
        pub fn get_value_definition(&self, _index: usize) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(CallIntrinsicInstruction);

    impl fmt::Display for CallIntrinsicInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "CallIntrinsic {:?}", self.intrinsic)
        }
    }

    #[derive(Clone, Debug)]
    pub struct CallCsaMacroInstruction {
        pub pos: SourcePosition,
        pub macro_: *mut Macro,
        pub constexpr_arguments: Vec<String>,
        pub catch_block: Option<*mut Block>,
    }

    impl CallCsaMacroInstruction {
        pub const KIND: InstructionKind = InstructionKind::CallCsaMacroInstruction;
        pub fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {
            if let Some(catch_block) = self.catch_block {
                // Safety: Assumed that the block pointer is valid.
                unsafe { block_list.push(&mut *catch_block) };
            }
        }

        pub fn get_exception_object_definition(&self) -> Option<DefinitionLocation> {
            None //TODO: Implement
        }
        pub fn get_value_definition_count(&self) -> usize {
            0 //TODO: Implement
        }
        pub fn get_value_definition(&self, _index: usize) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(CallCsaMacroInstruction);

    impl fmt::Display for CallCsaMacroInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "CallCsaMacro {:?}", self.macro_)
        }
    }

    #[derive(Clone, Debug)]
    pub struct CallCsaMacroAndBranchInstruction {
        pub pos: SourcePosition,
        pub macro_: *mut Macro,
        pub constexpr_arguments: Vec<String>,
        pub return_continuation: Option<*mut Block>,
        pub label_blocks: Vec<*mut Block>,
        pub catch_block: Option<*mut Block>,
    }

    impl CallCsaMacroAndBranchInstruction {
        pub const KIND: InstructionKind = InstructionKind::CallCsaMacroAndBranchInstruction;
        pub fn is_block_terminator(&self) -> bool {
            true
        }
        pub fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {
            if let Some(catch_block) = self.catch_block {
                // Safety: Assumed that the block pointer is valid.
                unsafe { block_list.push(&mut *catch_block) };
            }
            if let Some(return_continuation) = self.return_continuation {
                // Safety: Assumed that the block pointer is valid.
                unsafe { block_list.push(&mut *return_continuation) };
            }
            for block in &self.label_blocks {
                // Safety: Assumed that the block pointer is valid.
                unsafe { block_list.push(&mut **block) };
            }
        }

        pub fn get_label_count(&self) -> usize {
            0 //TODO: Implement
        }
        pub fn get_label_value_definition_count(&self, _label: usize) -> usize {
            0 //TODO: Implement
        }
        pub fn get_label_value_definition(&self, _label: usize, _index: usize) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
        pub fn get_value_definition_count(&self) -> usize {
            0 //TODO: Implement
        }
        pub fn get_value_definition(&self, _index: usize) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
        pub fn get_exception_object_definition(&self) -> Option<DefinitionLocation> {
            None //TODO: Implement
        }
    }
    implement_instruction_base!(CallCsaMacroAndBranchInstruction);

    impl fmt::Display for CallCsaMacroAndBranchInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "CallCsaMacroAndBranch {:?}", self.macro_)
        }
    }

    #[derive(Clone, Debug)]
    pub struct MakeLazyNodeInstruction {
        pub pos: SourcePosition,
        pub macro_: *mut Macro,
        pub result_type: Rc<Type>,
        pub constexpr_arguments: Vec<String>,
    }

    impl MakeLazyNodeInstruction {
        pub const KIND: InstructionKind = InstructionKind::MakeLazyNodeInstruction;
        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(MakeLazyNodeInstruction);

    impl fmt::Display for MakeLazyNodeInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "MakeLazyNode {:?} {}", self.macro_, self.result_type)
        }
    }

    #[derive(Clone, Debug)]
    pub struct CallBuiltinInstruction {
        pub pos: SourcePosition,
        pub is_tailcall: bool,
        pub builtin: *mut Builtin,
        pub argc: usize,
        pub catch_block: Option<*mut Block>,
    }

    impl CallBuiltinInstruction {
        pub const KIND: InstructionKind = InstructionKind::CallBuiltinInstruction;
        pub fn is_block_terminator(&self) -> bool {
            self.is_tailcall
        }
        pub fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {
            if let Some(catch_block) = self.catch_block {
                // Safety: Assumed that the block pointer is valid.
                unsafe { block_list.push(&mut *catch_block) };
            }
        }

        pub fn get_value_definition_count(&self) -> usize {
            0 //TODO: Implement
        }
        pub fn get_value_definition(&self, _index: usize) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
        pub fn get_exception_object_definition(&self) -> Option<DefinitionLocation> {
            None //TODO: Implement
        }
    }
    implement_instruction_base!(CallBuiltinInstruction);

    impl fmt::Display for CallBuiltinInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "CallBuiltin {:?} argc: {}", self.builtin, self.argc)
        }
    }

    #[derive(Clone, Debug)]
    pub struct CallBuiltinPointerInstruction {
        pub pos: SourcePosition,
        pub is_tailcall: bool,
        pub type_: Rc<BuiltinPointerType>,
        pub argc: usize,
    }

    impl CallBuiltinPointerInstruction {
        pub const KIND: InstructionKind = InstructionKind::CallBuiltinPointerInstruction;
        pub fn is_block_terminator(&self) -> bool {
            self.is_tailcall
        }

        pub fn get_value_definition_count(&self) -> usize {
            0 //TODO: Implement
        }
        pub fn get_value_definition(&self, _index: usize) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
    }
    implement_instruction_base!(CallBuiltinPointerInstruction);

    impl fmt::Display for CallBuiltinPointerInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "CallBuiltinPointer {} argc: {}", self.type_, self.argc)?;
            if self.is_tailcall {
                write!(os, ", is_tailcall")?;
            }
            Ok(())
        }
    }

    #[derive(Clone, Debug)]
    pub struct CallRuntimeInstruction {
        pub pos: SourcePosition,
        pub is_tailcall: bool,
        pub runtime_function: *mut RuntimeFunction,
        pub argc: usize,
        pub catch_block: Option<*mut Block>,
    }

    impl CallRuntimeInstruction {
        pub const KIND: InstructionKind = InstructionKind::CallRuntimeInstruction;
        pub fn is_block_terminator(&self) -> bool {
            todo!() //TODO: Implement
        }
        pub fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {
            if let Some(catch_block) = self.catch_block {
                // Safety: Assumed that the block pointer is valid.
                unsafe { block_list.push(&mut *catch_block) };
            }
        }

        pub fn get_value_definition_count(&self) -> usize {
            0 //TODO: Implement
        }
        pub fn get_value_definition(&self, _index: usize) -> DefinitionLocation {
            DefinitionLocation::Invalid //TODO: Implement
        }
        pub fn get_exception_object_definition(&self) -> Option<DefinitionLocation> {
            None //TODO: Implement
        }
    }
    implement_instruction_base!(CallRuntimeInstruction);

    impl fmt::Display for CallRuntimeInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                os,
                "CallRuntime {:?} argc: {}",
                self.runtime_function, self.argc
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct BranchInstruction {
        pub pos: SourcePosition,
        pub if_true: *mut Block,
        pub if_false: *mut Block,
    }

    impl BranchInstruction {
        pub const KIND: InstructionKind = InstructionKind::BranchInstruction;
        pub fn is_block_terminator(&self) -> bool {
            true
        }
        pub fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {
            // Safety: Assumed that the block pointer is valid.
            unsafe { block_list.push(&mut *self.if_true) };
            // Safety: Assumed that the block pointer is valid.
            unsafe { block_list.push(&mut *self.if_false) };
        }
    }
    implement_instruction_base!(BranchInstruction);

    impl fmt::Display for BranchInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "Branch if_true: {:?}, if_false: {:?}", self.if_true, self.if_false)
        }
    }

    #[derive(Clone, Debug)]
    pub struct ConstexprBranchInstruction {
        pub pos: SourcePosition,
        pub condition: String,
        pub if_true: *mut Block,
        pub if_false: *mut Block,
    }

    impl ConstexprBranchInstruction {
        pub const KIND: InstructionKind = InstructionKind::ConstexprBranchInstruction;
        pub fn is_block_terminator(&self) -> bool {
            true
        }
        pub fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {
            // Safety: Assumed that the block pointer is valid.
            unsafe { block_list.push(&mut *self.if_true) };
            // Safety: Assumed that the block pointer is valid.
            unsafe { block_list.push(&mut *self.if_false) };
        }
    }
    implement_instruction_base!(ConstexprBranchInstruction);

    impl fmt::Display for ConstexprBranchInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                os,
                "ConstexprBranch condition: {}, if_true: {:?}, if_false: {:?}",
                self.condition, self.if_true, self.if_false
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct GotoInstruction {
        pub pos: SourcePosition,
        pub destination: *mut Block,
    }

    impl GotoInstruction {
        pub const KIND: InstructionKind = InstructionKind::GotoInstruction;
        pub fn is_block_terminator(&self) -> bool {
            true
        }
        pub fn append_successor_blocks(&self, block_list: &mut Vec<&mut Block>) {
            // Safety: Assumed that the block pointer is valid.
            unsafe { block_list.push(&mut *self.destination) };
        }
    }
    implement_instruction_base!(GotoInstruction);

    impl fmt::Display for GotoInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "Goto destination: {:?}", self.destination)
        }
    }

    #[derive(Clone, Debug)]
    pub struct GotoExternalInstruction {
        pub pos: SourcePosition,
        pub destination: String,
        pub variable_names: Vec<String>,
    }

    impl GotoExternalInstruction {
        pub const KIND: InstructionKind = InstructionKind::GotoExternalInstruction;
        pub fn is_block_terminator(&self) -> bool {
            true
        }
    }
    implement_instruction_base!(GotoExternalInstruction);

    impl fmt::Display for GotoExternalInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "GotoExternal {}", self.destination)?;
            for name in &self.variable_names {
                write!(os, ", {}", name)?;
            }
            Ok(())
        }
    }

    #[derive(Clone, Debug)]
    pub struct ReturnInstruction {
        pub pos: SourcePosition,
        pub count: usize,
    }

    impl ReturnInstruction {
        pub const KIND: InstructionKind = InstructionKind::ReturnInstruction;
        pub fn is_block_terminator(&self) -> bool {
            true
        }
    }
    implement_instruction_base!(ReturnInstruction);

    impl fmt::Display for ReturnInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "Return count: {}", self.count)
        }
    }

    #[derive(Clone, Debug)]
    pub struct PrintErrorInstruction {
        pub pos: SourcePosition,
        pub message: String,
    }

    impl PrintErrorInstruction {
        pub const KIND: InstructionKind = InstructionKind::PrintErrorInstruction;
    }
    implement_instruction_base!(PrintErrorInstruction);

    impl fmt::Display for PrintErrorInstruction {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(os, "PrintConstantString {:?}", self.message)
        }
    }

    #[derive(Clone, Debug)]
    pub struct AbortInstruction {
        pub pos: SourcePosition,
        pub kind: AbortKind,
        pub message: String,
    }

    impl AbortInstruction {