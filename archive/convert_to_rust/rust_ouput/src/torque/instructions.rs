// Converted from V8 C++ source files:
// Header: instructions.h
// Implementation: instructions.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod instructions {
    use std::borrow::Cow;
    use std::fmt;
    use std::io::Write;
    use std::rc::Rc;

    use crate::compiler::backend::instruction::InstructionOperand;
    use crate::execution::isolate::Isolate;
    use crate::snapshot::references::SnapshotSpace;
    use crate::torque::ast;
    use crate::torque::cfg::{Block, ControlFlowGraph, Stack, StackRange};
    use crate::torque::constants::FieldSynchronization;
    use crate::torque::runtime_macro_shims::Error;
    use crate::torque::source_positions::{CurrentSourcePosition, SourcePosition};
    use crate::torque::type_oracle::TypeOracle;
    use crate::torque::types::{
        BuiltinPointerType, LowerParameterTypes, LowerType, NameAndType, TopType, Type, TypeVector,
    };
    use crate::torque::utils::{PrintCommaSeparatedList, ReportError, StringLiteralQuote, Worklist};

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct BitField {
        pub name_and_type: NameAndType,
    }

    pub enum InstructionKind {
        // Instructions where all backends generate code the same way.
        kPeekInstruction,
        kPokeInstruction,
        kDeleteRangeInstruction,
        // Instructions where different backends may generate different code.
        kPushUninitializedInstruction,
        kPushBuiltinPointerInstruction,
        kLoadReferenceInstruction,
        kStoreReferenceInstruction,
        kLoadBitFieldInstruction,
        kStoreBitFieldInstruction,
        kCallCsaMacroInstruction,
        kCallIntrinsicInstruction,
        kNamespaceConstantInstruction,
        kCallCsaMacroAndBranchInstruction,
        kCallBuiltinInstruction,
        kCallRuntimeInstruction,
        kCallBuiltinPointerInstruction,
        kBranchInstruction,
        kConstexprBranchInstruction,
        kGotoInstruction,
        kGotoExternalInstruction,
        kMakeLazyNodeInstruction,
        kReturnInstruction,
        kPrintErrorInstruction,
        kAbortInstruction,
        kUnsafeCastInstruction,
    }

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DefinitionLocation {
        kind: Kind,
        location: usize,
        index: usize,
    }

    impl DefinitionLocation {
        pub fn new() -> Self {
            DefinitionLocation {
                kind: Kind::kInvalid,
                location: 0,
                index: 0,
            }
        }
        pub fn parameter(index: usize) -> Self {
            DefinitionLocation {
                kind: Kind::kParameter,
                location: 0,
                index,
            }
        }

        pub fn phi(block: *const Block, index: usize) -> Self {
            DefinitionLocation {
                kind: Kind::kPhi,
                location: block as usize,
                index,
            }
        }

        pub fn instruction(instruction: *const InstructionBase, index: usize) -> Self {
            DefinitionLocation {
                kind: Kind::kInstruction,
                location: instruction as usize,
                index,
            }
        }

        pub fn get_kind(&self) -> Kind {
            self.kind.clone()
        }
        pub fn is_valid(&self) -> bool {
            self.kind != Kind::kInvalid
        }
        pub fn is_parameter(&self) -> bool {
            self.kind == Kind::kParameter
        }
        pub fn is_phi(&self) -> bool {
            self.kind == Kind::kPhi
        }
        pub fn is_instruction(&self) -> bool {
            self.kind == Kind::kInstruction
        }

        pub fn get_parameter_index(&self) -> usize {
            assert!(self.is_parameter());
            self.index
        }

        pub fn get_phi_block(&self) -> *const Block {
            assert!(self.is_phi());
            self.location as *const Block
        }

        pub fn is_phi_from_block(&self, block: *const Block) -> bool {
            self.is_phi() && self.get_phi_block() == block
        }

        pub fn get_phi_index(&self) -> usize {
            assert!(self.is_phi());
            self.index
        }

        pub fn get_instruction(&self) -> *const InstructionBase {
            assert!(self.is_instruction());
            self.location as *const InstructionBase
        }

        pub fn get_instruction_index(&self) -> usize {
            assert!(self.is_instruction());
            self.index
        }
    }

    impl fmt::Display for DefinitionLocation {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                Kind::kInvalid => write!(f, "DefinitionLocation::Invalid()"),
                Kind::kParameter => write!(f, "DefinitionLocation::Parameter({})", self.index),
                Kind::kPhi => write!(f, "DefinitionLocation::Phi({:x}, {})", self.location, self.index),
                Kind::kInstruction => write!(f, "DefinitionLocation::Instruction({:x}, {})", self.location, self.index),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Kind {
        kInvalid,
        kParameter,
        kPhi,
        kInstruction,
    }

    pub struct InstructionBase {
        pub pos: SourcePosition,
    }

    impl InstructionBase {
        pub fn new() -> Self {
            InstructionBase {
                pos: CurrentSourcePosition::get(),
            }
        }
        pub fn invalidate_transient_types(&self, stack: &mut Stack<&Type>) {
            for current in stack.iter_mut() {
                if (*current).is_transient() {
                    let mut stream = String::new();
                    stream.push_str("type ");
                    stream.push_str(&format!("{}", *current));
                    stream.push_str(" is made invalid by transitioning callable invocation at ");
                    stream.push_str(&PositionAsString(self.pos.clone()));
                    *current = TypeOracle::get_top_type(stream, *current);
                }
            }
        }

        pub fn is_block_terminator(&self) -> bool {
            false
        }

        pub fn append_successor_blocks(&self, _block_list: &mut Vec<*mut Block>) {}
    }

    impl fmt::Display for InstructionBase {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "InstructionBase")
        }
    }

    pub struct Instruction {
        kind: InstructionKind,
        instruction_: Box<dyn ConcreteInstruction>,
    }

    trait ConcreteInstruction: fmt::Display {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction>;
        fn assign(&mut self, other: &dyn ConcreteInstruction);
        fn type_instruction(&self, stack: &mut Stack<&Type>, cfg: *mut ControlFlowGraph);
        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            worklist: &mut Worklist<*mut Block>,
        );
        fn is_block_terminator(&self) -> bool {
            false
        }
        fn append_successor_blocks(&self, _block_list: &mut Vec<*mut Block>) {}
        fn get_value_definition_count(&self) -> usize {
            0
        }
        fn get_value_definition(&self, _index: usize) -> DefinitionLocation {
            DefinitionLocation::new()
        }
        fn get_exception_object_definition(&self) -> Option<DefinitionLocation> {
            None
        }
        fn get_label_count(&self) -> usize {
            0
        }
        fn get_label_value_definition_count(&self, _label: usize) -> usize {
            0
        }
        fn get_label_value_definition(&self, _label: usize, _index: usize) -> DefinitionLocation {
            DefinitionLocation::new()
        }
    }

    impl Instruction {
        pub fn new<T: ConcreteInstruction + 'static>(instr: T) -> Self {
            Instruction {
                kind: get_instruction_kind(&instr),
                instruction_: Box::new(instr),
            }
        }
        pub fn kind(&self) -> &InstructionKind {
            &self.kind
        }

        pub fn mnemonic(&self) -> &'static str {
            match self.kind() {
                InstructionKind::kPeekInstruction => "PeekInstruction",
                InstructionKind::kPokeInstruction => "PokeInstruction",
                InstructionKind::kDeleteRangeInstruction => "DeleteRangeInstruction",
                InstructionKind::kPushUninitializedInstruction => "PushUninitializedInstruction",
                InstructionKind::kPushBuiltinPointerInstruction => "PushBuiltinPointerInstruction",
                InstructionKind::kLoadReferenceInstruction => "LoadReferenceInstruction",
                InstructionKind::kStoreReferenceInstruction => "StoreReferenceInstruction",
                InstructionKind::kLoadBitFieldInstruction => "LoadBitFieldInstruction",
                InstructionKind::kStoreBitFieldInstruction => "StoreBitFieldInstruction",
                InstructionKind::kCallCsaMacroInstruction => "CallCsaMacroInstruction",
                InstructionKind::kCallIntrinsicInstruction => "CallIntrinsicInstruction",
                InstructionKind::kNamespaceConstantInstruction => "NamespaceConstantInstruction",
                InstructionKind::kCallCsaMacroAndBranchInstruction => {
                    "CallCsaMacroAndBranchInstruction"
                }
                InstructionKind::kCallBuiltinInstruction => "CallBuiltinInstruction",
                InstructionKind::kCallRuntimeInstruction => "CallRuntimeInstruction",
                InstructionKind::kCallBuiltinPointerInstruction => {
                    "CallBuiltinPointerInstruction"
                }
                InstructionKind::kBranchInstruction => "BranchInstruction",
                InstructionKind::kConstexprBranchInstruction => "ConstexprBranchInstruction",
                InstructionKind::kGotoInstruction => "GotoInstruction",
                InstructionKind::kGotoExternalInstruction => "GotoExternalInstruction",
                InstructionKind::kMakeLazyNodeInstruction => "MakeLazyNodeInstruction",
                InstructionKind::kReturnInstruction => "ReturnInstruction",
                InstructionKind::kPrintErrorInstruction => "PrintErrorInstruction",
                InstructionKind::kAbortInstruction => "AbortInstruction",
                InstructionKind::kUnsafeCastInstruction => "UnsafeCastInstruction",
            }
        }

        pub fn type_instruction(&self, stack: &mut Stack<&Type>, cfg: *mut ControlFlowGraph) {
            self.instruction_.type_instruction(stack, cfg);
        }

        pub fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            worklist: &mut Worklist<*mut Block>,
        ) {
            self.instruction_
                .recompute_definition_locations(locations, worklist);
        }
    }

    impl Clone for Instruction {
        fn clone(&self) -> Self {
            Instruction {
                kind: self.kind.clone(),
                instruction_: self.instruction_.clone_instruction(),
            }
        }
    }

    impl fmt::Display for Instruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Instruction {{ kind: {:?}, instruction_: {} }}", self.kind, self.instruction_)
        }
    }

    impl Instruction {
        pub fn as_any(&self) -> &dyn ConcreteInstruction {
            self.instruction_.as_ref()
        }

        pub fn as_any_mut(&mut self) -> &mut dyn ConcreteInstruction {
            self.instruction_.as_mut()
        }

        pub fn downcast_ref<T: ConcreteInstruction>(&self) -> Option<&T> {
            self.as_any().downcast_ref()
        }

        pub fn downcast_mut<T: ConcreteInstruction>(&mut self) -> Option<&mut T> {
            self.as_any_mut().downcast_mut()
        }

        pub fn is<T: ConcreteInstruction>(&self) -> bool {
            get_instruction_kind(self.instruction_.as_ref()) == get_instruction_kind(&(std::any::Any::type_id(&self.instruction_)))
        }

        pub fn dynamic_cast<T: ConcreteInstruction + 'static>(&self) -> Option<&T> {
            if self.is::<T>() {
                Some(self.instruction_.as_ref().downcast_ref::<T>().unwrap())
            } else {
                None
            }
        }

        pub fn dynamic_cast_mut<T: ConcreteInstruction + 'static>(&mut self) -> Option<&mut T> {
            if self.is::<T>() {
                Some(self.instruction_.as_mut().downcast_mut::<T>().unwrap())
            } else {
                None
            }
        }
    }

    pub fn get_instruction_kind<T: ConcreteInstruction>(_instr: &T) -> InstructionKind {
        match std::any::type_name::<T>() {
            "torque::instructions::PeekInstruction" => InstructionKind::kPeekInstruction,
            "torque::instructions::PokeInstruction" => InstructionKind::kPokeInstruction,
            "torque::instructions::DeleteRangeInstruction" => {
                InstructionKind::kDeleteRangeInstruction
            }
            "torque::instructions::PushUninitializedInstruction" => {
                InstructionKind::kPushUninitializedInstruction
            }
            "torque::instructions::PushBuiltinPointerInstruction" => {
                InstructionKind::kPushBuiltinPointerInstruction
            }
            "torque::instructions::LoadReferenceInstruction" => {
                InstructionKind::kLoadReferenceInstruction
            }
            "torque::instructions::StoreReferenceInstruction" => {
                InstructionKind::kStoreReferenceInstruction
            }
            "torque::instructions::LoadBitFieldInstruction" => {
                InstructionKind::kLoadBitFieldInstruction
            }
            "torque::instructions::StoreBitFieldInstruction" => {
                InstructionKind::kStoreBitFieldInstruction
            }
            "torque::instructions::CallCsaMacroInstruction" => {
                InstructionKind::kCallCsaMacroInstruction
            }
            "torque::instructions::CallIntrinsicInstruction" => {
                InstructionKind::kCallIntrinsicInstruction
            }
            "torque::instructions::NamespaceConstantInstruction" => {
                InstructionKind::kNamespaceConstantInstruction
            }
            "torque::instructions::CallCsaMacroAndBranchInstruction" => {
                InstructionKind::kCallCsaMacroAndBranchInstruction
            }
            "torque::instructions::CallBuiltinInstruction" => {
                InstructionKind::kCallBuiltinInstruction
            }
            "torque::instructions::CallRuntimeInstruction" => {
                InstructionKind::kCallRuntimeInstruction
            }
            "torque::instructions::CallBuiltinPointerInstruction" => {
                InstructionKind::kCallBuiltinPointerInstruction
            }
            "torque::instructions::BranchInstruction" => InstructionKind::kBranchInstruction,
            "torque::instructions::ConstexprBranchInstruction" => {
                InstructionKind::kConstexprBranchInstruction
            }
            "torque::instructions::GotoInstruction" => InstructionKind::kGotoInstruction,
            "torque::instructions::GotoExternalInstruction" => {
                InstructionKind::kGotoExternalInstruction
            }
            "torque::instructions::MakeLazyNodeInstruction" => {
                InstructionKind::kMakeLazyNodeInstruction
            }
            "torque::instructions::ReturnInstruction" => InstructionKind::kReturnInstruction,
            "torque::instructions::PrintErrorInstruction" => {
                InstructionKind::kPrintErrorInstruction
            }
            "torque::instructions::AbortInstruction" => InstructionKind::kAbortInstruction,
            "torque::instructions::UnsafeCastInstruction" => {
                InstructionKind::kUnsafeCastInstruction
            }
            _ => panic!("Unknown instruction type: {}", std::any::type_name::<T>()),
        }
    }

    pub fn get_instruction_kind_from_instruction_kind(kind: &InstructionKind) -> InstructionKind {
        match kind {
            InstructionKind::kPeekInstruction => InstructionKind::kPeekInstruction,
            InstructionKind::kPokeInstruction => InstructionKind::kPokeInstruction,
            InstructionKind::kDeleteRangeInstruction => InstructionKind::kDeleteRangeInstruction,
            InstructionKind::kPushUninitializedInstruction => {
                InstructionKind::kPushUninitializedInstruction
            }
            InstructionKind::kPushBuiltinPointerInstruction => {
                InstructionKind::kPushBuiltinPointerInstruction
            }
            InstructionKind::kLoadReferenceInstruction => InstructionKind::kLoadReferenceInstruction,
            InstructionKind::kStoreReferenceInstruction => {
                InstructionKind::kStoreReferenceInstruction
            }
            InstructionKind::kLoadBitFieldInstruction => InstructionKind::kLoadBitFieldInstruction,
            InstructionKind::kStoreBitFieldInstruction => {
                InstructionKind::kStoreBitFieldInstruction
            }
            InstructionKind::kCallCsaMacroInstruction => InstructionKind::kCallCsaMacroInstruction,
            InstructionKind::kCallIntrinsicInstruction => {
                InstructionKind::kCallIntrinsicInstruction
            }
            InstructionKind::kNamespaceConstantInstruction => {
                InstructionKind::kNamespaceConstantInstruction
            }
            InstructionKind::kCallCsaMacroAndBranchInstruction => {
                InstructionKind::kCallCsaMacroAndBranchInstruction
            }
            InstructionKind::kCallBuiltinInstruction => InstructionKind::kCallBuiltinInstruction,
            InstructionKind::kCallRuntimeInstruction => InstructionKind::kCallRuntimeInstruction,
            InstructionKind::kCallBuiltinPointerInstruction => {
                InstructionKind::kCallBuiltinPointerInstruction
            }
            InstructionKind::kBranchInstruction => InstructionKind::kBranchInstruction,
            InstructionKind::kConstexprBranchInstruction => {
                InstructionKind::kConstexprBranchInstruction
            }
            InstructionKind::kGotoInstruction => InstructionKind::kGotoInstruction,
            InstructionKind::kGotoExternalInstruction => InstructionKind::kGotoExternalInstruction,
            InstructionKind::kMakeLazyNodeInstruction => {
                InstructionKind::kMakeLazyNodeInstruction
            }
            InstructionKind::kReturnInstruction => InstructionKind::kReturnInstruction,
            InstructionKind::kPrintErrorInstruction => InstructionKind::kPrintErrorInstruction,
            InstructionKind::kAbortInstruction => InstructionKind::kAbortInstruction,
            InstructionKind::kUnsafeCastInstruction => InstructionKind::kUnsafeCastInstruction,
        }
    }

    #[derive(Debug, Clone)]
    pub struct PeekInstruction {
        pub slot: usize,
        pub widened_type: Option<&'static Type>,
    }

    impl PeekInstruction {
        pub fn new(slot: usize, widened_type: Option<&'static Type>) -> Self {
            PeekInstruction {
                slot,
                widened_type,
            }
        }
    }

    impl fmt::Display for PeekInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Peek {}", self.slot)?;
            if let Some(widened_type) = self.widened_type {
                write!(f, ", {}", widened_type)?;
            }
            Ok(())
        }
    }

    impl ConcreteInstruction for PeekInstruction {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn ConcreteInstruction) {
            let other = other.downcast_ref::<Self>().unwrap();
            self.slot = other.slot;
            self.widened_type = other.widened_type;
        }

        fn type_instruction(&self, stack: &mut Stack<&Type>, _cfg: *mut ControlFlowGraph) {
            let mut r#type = stack.peek(self.slot).clone();
            if let Some(widened_type) = self.widened_type {
                if r#type.is_top_type() {
                    let top_type = TopType::cast(r#type);
                    ReportError("use of ".to_string() + &top_type.reason());
                }
                ExpectSubtype(r#type, widened_type);
                r#type = widened_type;
            }
            stack.push(r#type);
        }

        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            let location = locations.peek(self.slot).clone();
            locations.push(location);
        }
    }

    impl std::any::Any for PeekInstruction {
        fn type_id(&self) -> std::any::TypeId {
            std::any::TypeId::of::<Self>()
        }
    }

    #[derive(Debug, Clone)]
    pub struct PokeInstruction {
        pub slot: usize,
        pub widened_type: Option<&'static Type>,
    }

    impl PokeInstruction {
        pub fn new(slot: usize, widened_type: Option<&'static Type>) -> Self {
            PokeInstruction {
                slot,
                widened_type,
            }
        }
    }

    impl fmt::Display for PokeInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Poke {}", self.slot)?;
            if let Some(widened_type) = self.widened_type {
                write!(f, ", {}", widened_type)?;
            }
            Ok(())
        }
    }

    impl ConcreteInstruction for PokeInstruction {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn ConcreteInstruction) {
            let other = other.downcast_ref::<Self>().unwrap();
            self.slot = other.slot;
            self.widened_type = other.widened_type;
        }

        fn type_instruction(&self, stack: &mut Stack<&Type>, _cfg: *mut ControlFlowGraph) {
            let mut r#type = stack.top().clone();
            if let Some(widened_type) = self.widened_type {
                ExpectSubtype(r#type, widened_type);
                r#type = widened_type;
            }
            stack.poke(self.slot, r#type);
            stack.pop();
        }

        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            let location = locations.pop();
            locations.poke(self.slot, location);
        }
    }

    impl std::any::Any for PokeInstruction {
        fn type_id(&self) -> std::any::TypeId {
            std::any::TypeId::of::<Self>()
        }
    }

    // Preserve the top {preserved_slots} number of slots, and delete
    // {deleted_slots} number or slots below.
    #[derive(Debug, Clone)]
    pub struct DeleteRangeInstruction {
        pub range: StackRange,
    }

    impl DeleteRangeInstruction {
        pub fn new(range: StackRange) -> Self {
            DeleteRangeInstruction { range }
        }
    }

    impl fmt::Display for DeleteRangeInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "DeleteRange {}", self.range)
        }
    }

    impl ConcreteInstruction for DeleteRangeInstruction {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn ConcreteInstruction) {
            let other = other.downcast_ref::<Self>().unwrap();
            self.range = other.range;
        }

        fn type_instruction(&self, stack: &mut Stack<&Type>, _cfg: *mut ControlFlowGraph) {
            stack.delete_range(self.range);
        }

        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            locations.delete_range(self.range);
        }
    }

    impl std::any::Any for DeleteRangeInstruction {
        fn type_id(&self) -> std::any::TypeId {
            std::any::TypeId::of::<Self>()
        }
    }

    #[derive(Debug, Clone)]
    pub struct PushUninitializedInstruction {
        pub r#type: &'static Type,
    }

    impl PushUninitializedInstruction {
        pub fn new(r#type: &'static Type) -> Self {
            PushUninitializedInstruction { r#type }
        }

        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::instruction(self as *const PushUninitializedInstruction as *const InstructionBase, 0)
        }
    }

    impl fmt::Display for PushUninitializedInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "PushUninitialized {}", self.r#type)
        }
    }

    impl ConcreteInstruction for PushUninitializedInstruction {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn ConcreteInstruction) {
            let other = other.downcast_ref::<Self>().unwrap();
            self.r#type = other.r#type;
        }

        fn type_instruction(&self, stack: &mut Stack<&Type>, _cfg: *mut ControlFlowGraph) {
            stack.push(self.r#type);
        }

        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            locations.push(self.get_value_definition());
        }

        fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::instruction(self as *const PushUninitializedInstruction as *const InstructionBase, 0)
        }
        fn get_value_definition_count(&self) -> usize {
            1
        }
    }

    impl std::any::Any for PushUninitializedInstruction {
        fn type_id(&self) -> std::any::TypeId {
            std::any::TypeId::of::<Self>()
        }
    }

    #[derive(Debug, Clone)]
    pub struct PushBuiltinPointerInstruction {
        pub external_name: String,
        pub r#type: &'static Type,
    }

    impl PushBuiltinPointerInstruction {
        pub fn new(external_name: String, r#type: &'static Type) -> Self {
            assert!(r#type.is_builtin_pointer_type());
            PushBuiltinPointerInstruction {
                external_name,
                r#type,
            }
        }

        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::instruction(self as *const PushBuiltinPointerInstruction as *const InstructionBase, 0)
        }
    }

    impl fmt::Display for PushBuiltinPointerInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "PushBuiltinPointer {}, {}",
                StringLiteralQuote(self.external_name.clone()),
                self.r#type
            )
        }
    }

    impl ConcreteInstruction for PushBuiltinPointerInstruction {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn ConcreteInstruction) {
            let other = other.downcast_ref::<Self>().unwrap();
            self.external_name = other.external_name.clone();
            self.r#type = other.r#type;
        }

        fn type_instruction(&self, stack: &mut Stack<&Type>, _cfg: *mut ControlFlowGraph) {
            stack.push(self.r#type);
        }

        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            locations.push(self.get_value_definition());
        }

        fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::instruction(self as *const PushBuiltinPointerInstruction as *const InstructionBase, 0)
        }

        fn get_value_definition_count(&self) -> usize {
            1
        }
    }

    impl std::any::Any for PushBuiltinPointerInstruction {
        fn type_id(&self) -> std::any::TypeId {
            std::any::TypeId::of::<Self>()
        }
    }

    #[derive(Clone, Debug)]
    pub struct NamespaceConstantInstruction {
        pub constant: *mut ast::NamespaceConstant,
    }

    impl NamespaceConstantInstruction {
        pub fn new(constant: *mut ast::NamespaceConstant) -> Self {
            NamespaceConstantInstruction { constant }
        }

        pub fn get_value_definition_count(&self) -> usize {
            let constant = unsafe { &*self.constant };
            LowerType(constant.r#type()).len()
        }

        pub fn get_value_definition(&self, index: usize) -> DefinitionLocation {
            assert!(index < self.get_value_definition_count());
            DefinitionLocation::instruction(self as *const NamespaceConstantInstruction as *const InstructionBase, index)
        }
    }

    impl fmt::Display for NamespaceConstantInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let constant = unsafe { &*self.constant };
            write!(f, "NamespaceConstant {}", constant.external_name())
        }
    }

    impl ConcreteInstruction for NamespaceConstantInstruction {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn ConcreteInstruction) {
            let other = other.downcast_ref::<Self>().unwrap();
            self.constant = other.constant;
        }

        fn type_instruction(&self, stack: &mut Stack<&Type>, _cfg: *mut ControlFlowGraph) {
            let constant = unsafe { &*self.constant };
            stack.push_many(LowerType(constant.r#type()));
        }

        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            for i in 0..self.get_value_definition_count() {
                locations.push(self.get_value_definition(i));
            }
        }

        fn get_value_definition_count(&self) -> usize {
            let constant = unsafe { &*self.constant };
            LowerType(constant.r#type()).len()
        }

        fn get_value_definition(&self, index: usize) -> DefinitionLocation {
            assert!(index < self.get_value_definition_count());
            DefinitionLocation::instruction(self as *const NamespaceConstantInstruction as *const InstructionBase, index)
        }
    }

    impl std::any::Any for NamespaceConstantInstruction {
        fn type_id(&self) -> std::any::TypeId {
            std::any::TypeId::of::<Self>()
        }
    }

    #[derive(Debug, Clone)]
    pub struct LoadReferenceInstruction {
        pub r#type: &'static Type,
        pub synchronization: FieldSynchronization,
    }

    impl LoadReferenceInstruction {
        pub fn new(r#type: &'static Type, synchronization: FieldSynchronization) -> Self {
            LoadReferenceInstruction {
                r#type,
                synchronization,
            }
        }

        pub fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::instruction(self as *const LoadReferenceInstruction as *const InstructionBase, 0)
        }
    }

    impl fmt::Display for LoadReferenceInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "LoadReference {}", self.r#type)
        }
    }

    impl ConcreteInstruction for LoadReferenceInstruction {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn ConcreteInstruction) {
            let other = other.downcast_ref::<Self>().unwrap();
            self.r#type = other.r#type;
            self.synchronization = other.synchronization;
        }

        fn type_instruction(&self, stack: &mut Stack<&Type>, _cfg: *mut ControlFlowGraph) {
            ExpectType(TypeOracle::get_intptr_type(), stack.pop());
            ExpectSubtype(
                stack.pop(),
                TypeOracle::get_union_type(
                    TypeOracle::get_heap_object_type(),
                    TypeOracle::get_tagged_zero_pattern_type(),
                ),
            );
            assert_eq!(vec![self.r#type], LowerType(self.r#type));
            stack.push(self.r#type);
        }

        fn recompute_definition_locations(
            &self,
            locations: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            locations.pop();
            locations.pop();
            locations.push(self.get_value_definition());
        }

        fn get_value_definition(&self) -> DefinitionLocation {
            DefinitionLocation::instruction(self as *const LoadReferenceInstruction as *const InstructionBase, 0)
        }

        fn get_value_definition_count(&self) -> usize {
            1
        }
    }

    impl std::any::Any for LoadReferenceInstruction {
        fn type_id(&self) -> std::any::TypeId {
            std::any::TypeId::of::<Self>()
        }
    }

    #[derive(Debug, Clone)]
    pub struct StoreReferenceInstruction {
        pub r#type: &'static Type,
    }

    impl StoreReferenceInstruction {
        pub fn new(r#type: &'static Type) -> Self {
            StoreReferenceInstruction { r#type }
        }
    }

    impl fmt::Display for StoreReferenceInstruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "StoreReference {}", self.r#type)
        }
    }

    impl ConcreteInstruction for StoreReferenceInstruction {
        fn clone_instruction(&self) -> Box<dyn ConcreteInstruction> {
            Box::new(self.clone())
        }

        fn assign(&mut self, other: &dyn
