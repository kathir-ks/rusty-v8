// Converted from V8 C++ source files:
// Header: cfg.h
// Implementation: cfg.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ast {
    // Placeholder for ast module
}
pub mod instructions {
    // Placeholder for instructions module

    use std::any::Any;
    use std::fmt::Debug;

    use super::types::Type;

    #[derive(Debug, Clone)]
    pub struct InstructionBase {}
    pub trait InstructionTrait: Debug {
        fn as_any(&self) -> &dyn Any;
        fn eq(&self, other: &dyn InstructionTrait) -> bool;
        fn type_instruction(&self, current_stack: &mut Stack<*const Type>, cfg: &ControlFlowGraph);
        fn is_block_terminator(&self) -> bool;
        fn append_successor_blocks(&self, successors: &mut Vec<*mut Block>);
        fn recompute_definition_locations(
            &self,
            definitions: &mut Stack<DefinitionLocation>,
            worklist: &mut Worklist<*mut Block>,
        );
    }

    #[derive(Debug, Clone)]
    pub struct GotoInstruction {
        pub destination: *mut Block,
    }

    impl InstructionTrait for GotoInstruction {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn eq(&self, other: &dyn InstructionTrait) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<Self>() {
                self.destination as *const _ == other.destination as *const _
            } else {
                false
            }
        }
        fn type_instruction(&self, _current_stack: &mut Stack<*const Type>, _cfg: &ControlFlowGraph) {}
        fn is_block_terminator(&self) -> bool {
            true
        }
        fn append_successor_blocks(&self, successors: &mut Vec<*mut Block>) {
            successors.push(self.destination);
        }
        fn recompute_definition_locations(
            &self,
            _definitions: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
        }
    }

    #[derive(Debug, Clone)]
    pub struct BranchInstruction {
        pub if_true: *mut Block,
        pub if_false: *mut Block,
    }

    impl InstructionTrait for BranchInstruction {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn eq(&self, other: &dyn InstructionTrait) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<Self>() {
                self.if_true as *const _ == other.if_true as *const _
                    && self.if_false as *const _ == other.if_false as *const _
            } else {
                false
            }
        }
        fn type_instruction(&self, current_stack: &mut Stack<*const Type>, _cfg: &ControlFlowGraph) {
            current_stack.Pop();
        }
        fn is_block_terminator(&self) -> bool {
            true
        }
        fn append_successor_blocks(&self, successors: &mut Vec<*mut Block>) {
            successors.push(self.if_true);
            successors.push(self.if_false);
        }
        fn recompute_definition_locations(
            &self,
            definitions: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            definitions.Pop();
        }
    }

    #[derive(Debug, Clone)]
    pub struct DeleteRangeInstruction {
        pub range: StackRange,
    }

    impl InstructionTrait for DeleteRangeInstruction {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn eq(&self, other: &dyn InstructionTrait) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<Self>() {
                self.range == other.range
            } else {
                false
            }
        }
        fn type_instruction(&self, current_stack: &mut Stack<*const Type>, _cfg: &ControlFlowGraph) {
            for _ in 0..self.range.Size() {
                current_stack.Pop();
            }
        }
        fn is_block_terminator(&self) -> bool {
            false
        }
        fn append_successor_blocks(&self, _successors: &mut Vec<*mut Block>) {}
        fn recompute_definition_locations(
            &self,
            definitions: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            for _ in 0..self.range.Size() {
                definitions.Pop();
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct PeekInstruction {
        pub offset: usize,
        pub type_: Option<*const Type>,
    }

    impl InstructionTrait for PeekInstruction {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn eq(&self, other: &dyn InstructionTrait) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<Self>() {
                self.offset == other.offset && self.type_ == other.type_
            } else {
                false
            }
        }
        fn type_instruction(&self, current_stack: &mut Stack<*const Type>, _cfg: &ControlFlowGraph) {
            current_stack.Push(current_stack.PeekAt(self.offset));
        }
        fn is_block_terminator(&self) -> bool {
            false
        }
        fn append_successor_blocks(&self, _successors: &mut Vec<*mut Block>) {}
        fn recompute_definition_locations(
            &self,
            definitions: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            definitions.Push(definitions.PeekAt(self.offset));
        }
    }

    #[derive(Debug, Clone)]
    pub struct PokeInstruction {
        pub offset: usize,
        pub type_: Option<*const Type>,
    }

    impl InstructionTrait for PokeInstruction {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn eq(&self, other: &dyn InstructionTrait) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<Self>() {
                self.offset == other.offset && self.type_ == other.type_
            } else {
                false
            }
        }
        fn type_instruction(&self, current_stack: &mut Stack<*const Type>, _cfg: &ControlFlowGraph) {
            current_stack.Pop();
        }
        fn is_block_terminator(&self) -> bool {
            false
        }
        fn append_successor_blocks(&self, _successors: &mut Vec<*mut Block>) {}
        fn recompute_definition_locations(
            &self,
            definitions: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
            definitions.Pop();
        }
    }

    #[derive(Debug, Clone)]
    pub struct PrintErrorInstruction {
        pub message: String,
    }

    impl InstructionTrait for PrintErrorInstruction {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn eq(&self, other: &dyn InstructionTrait) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<Self>() {
                self.message == other.message
            } else {
                false
            }
        }
        fn type_instruction(&self, _current_stack: &mut Stack<*const Type>, _cfg: &ControlFlowGraph) {}
        fn is_block_terminator(&self) -> bool {
            false
        }
        fn append_successor_blocks(&self, _successors: &mut Vec<*mut Block>) {}
        fn recompute_definition_locations(
            &self,
            _definitions: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
        }
    }

    #[derive(Debug, Clone)]
    pub struct AbortInstruction {
        pub kind: AbortInstructionKind,
        pub message: String,
    }

    #[derive(Debug, Clone)]
    pub enum AbortInstructionKind {
        kAssertionFailure,
        kUnreachable,
        kDebugBreak,
    }

    impl InstructionTrait for AbortInstruction {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn eq(&self, other: &dyn InstructionTrait) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<Self>() {
                self.kind == other.kind && self.message == other.message
            } else {
                false
            }
        }
        fn type_instruction(&self, _current_stack: &mut Stack<*const Type>, _cfg: &ControlFlowGraph) {}
        fn is_block_terminator(&self) -> bool {
            true
        }
        fn append_successor_blocks(&self, _successors: &mut Vec<*mut Block>) {}
        fn recompute_definition_locations(
            &self,
            _definitions: &mut Stack<DefinitionLocation>,
            _worklist: &mut Worklist<*mut Block>,
        ) {
        }
    }

    #[derive(Clone)]
    pub struct Instruction(Box<dyn InstructionTrait>);

    impl Instruction {
        pub fn new<T: InstructionTrait + 'static>(instruction: T) -> Self {
            Instruction(Box::new(instruction))
        }
        pub fn as_any(&self) -> &dyn Any {
            self.0.as_any()
        }
        pub fn eq(&self, other: &Instruction) -> bool {
            self.0.eq(other.0.as_ref())
        }
        pub fn type_instruction(&self, current_stack: &mut Stack<*const Type>, cfg: &ControlFlowGraph) {
            self.0.type_instruction(current_stack, cfg)
        }
        pub fn is_block_terminator(&self) -> bool {
            self.0.is_block_terminator()
        }
        pub fn append_successor_blocks(&self, successors: &mut Vec<*mut Block>) {
            self.0.append_successor_blocks(successors);
        }
        pub fn recompute_definition_locations(
            &self,
            definitions: &mut Stack<DefinitionLocation>,
            worklist: &mut Worklist<*mut Block>,
        ) {
            self.0.recompute_definition_locations(definitions, worklist);
        }
        pub fn is<T: InstructionTrait + 'static>(&self) -> bool {
            self.0.as_any().is::<T>()
        }
        pub fn cast<T: InstructionTrait + 'static>(&self) -> &T {
            self.0.as_any().downcast_ref::<T>().unwrap()
        }
    }

    impl Debug for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&self.0, f)
        }
    }
}
pub mod source_positions {
    // Placeholder for source_positions module
}
pub mod types {
    use std::fmt::{Debug, Display, Formatter};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Type {}

    impl Display for Type {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Type")
        }
    }

    impl Type {
        pub fn is_subtype_of(&self, other: &Type) -> bool {
            self == other
        }
    }

    // Placeholder for types module
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Block {}
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Debug;
use std::mem;
use std::optional::Option;
use std::ptr;
use std::rc::Rc;
use std::sstream::stringstream;
use std::vec;

use instructions::{
    AbortInstruction, AbortInstructionKind, BranchInstruction, DeleteRangeInstruction, GotoInstruction,
    Instruction, InstructionTrait, PeekInstruction, PokeInstruction, PrintErrorInstruction,
};
use types::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BottomOffset {
    pub offset: usize,
}

impl From<usize> for BottomOffset {
    fn from(offset: usize) -> Self {
        BottomOffset { offset }
    }
}

impl std::ops::Add for BottomOffset {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BottomOffset {
            offset: self.offset + other.offset,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackRange {
    begin: usize,
    end: usize,
}

impl StackRange {
    pub fn new(begin: usize, end: usize) -> Self {
        StackRange { begin, end }
    }
    pub fn Size(&self) -> usize {
        self.end - self.begin
    }
    pub fn begin(&self) -> usize {
        self.begin
    }
    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Clone, Debug)]
pub struct Stack<T: Clone + Debug> {
    data: Vec<T>,
}

impl<T: Clone + Debug> Stack<T> {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn Push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn Pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn PeekAt(&self, offset: usize) -> &T {
        &self.data[offset]
    }

    pub fn Peek(&self, i: BottomOffset) -> &T {
        &self.data[i.offset]
    }

    pub fn Poke(&mut self, i: BottomOffset, value: T) {
        self.data[i.offset] = value;
    }

    pub fn Size(&self) -> usize {
        self.data.len()
    }

    pub fn AboveTop(&self) -> usize {
        self.data.len()
    }

    pub fn TopRange(&self, slot_count: usize) -> StackRange {
        let end = self.AboveTop();
        let begin = end - slot_count;
        StackRange { begin, end }
    }

    pub fn AboveTopOffset(&self) -> BottomOffset {
        BottomOffset {
            offset: self.data.len(),
        }
    }

    pub fn begin(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

impl<T: Clone + Debug + PartialEq> Stack<T> {
    pub fn equals(&self, other: &Stack<T>) -> bool {
        self.data == other.data
    }
}

impl<T: Clone + Debug> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: Clone + Debug + fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            write!(f, "{}", item)?;
            if i < self.data.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionLocation {
    Parameter(usize),
    Phi(*mut Block, usize),
}

impl DefinitionLocation {
    pub fn Parameter(i: usize) -> Self {
        DefinitionLocation::Parameter(i)
    }
    pub fn Phi(block: *mut Block, offset: usize) -> Self {
        DefinitionLocation::Phi(block, offset)
    }
}

#[derive(Debug)]
pub struct Block {
    cfg_: *mut ControlFlowGraph,
    input_types_: Option<Stack<*const Type>>,
    input_definitions_: Option<Stack<DefinitionLocation>>,
    id_: usize,
    is_deferred_: bool,
    instructions_: Vec<Instruction>,
}

impl Block {
    pub fn new(
        cfg: *mut ControlFlowGraph,
        id: usize,
        input_types: Option<Stack<*const Type>>,
        is_deferred: bool,
    ) -> Self {
        Block {
            cfg_: cfg,
            input_types_: input_types,
            input_definitions_: None,
            id_: id,
            is_deferred_: is_deferred,
            instructions_: Vec::new(),
        }
    }

    pub fn Add(&mut self, instruction: Instruction) {
        assert!(!self.IsComplete());
        self.instructions_.push(instruction);
    }

    pub fn HasInputTypes(&self) -> bool {
        self.input_types_.is_some()
    }
    pub fn InputTypes(&self) -> &Stack<*const Type> {
        self.input_types_.as_ref().unwrap()
    }

    pub fn SetInputTypes(&mut self, input_types: &Stack<*const Type>) {
        if self.input_types_.is_none() {
            self.input_types_ = Some(input_types.clone());
            return;
        } else if self.input_types_.as_ref().unwrap().equals(input_types) {
            return;
        }

        assert_eq!(input_types.Size(), self.input_types_.as_ref().unwrap().Size());
        let mut merged_types = Stack::new();
        let mut widened = false;
        let mut c2_iterator = input_types.begin();
        for c1 in self.input_types_.as_ref().unwrap().data.iter() {
            let merged_type = TypeOracle::GetUnionType(*c1, *c2_iterator.next().unwrap());
            if !merged_type.is_subtype_of(*c1) {
                widened = true;
            }
            merged_types.Push(merged_type);
        }
        if merged_types.Size() == self.input_types_.as_ref().unwrap().Size() {
            if widened {
                self.input_types_ = Some(merged_types);
                self.Retype();
            }
            return;
        }

        let mut error = stringstream::new();
        error << "incompatible types at branch:\n";
        for i in (0..std::cmp::max(
            self.input_types_.as_ref().unwrap().Size(),
            input_types.Size(),
        ))
        .rev()
        {
            let left = if i < input_types.Size() {
                Some(input_types.Peek(BottomOffset::from(i)))
            } else {
                None
            };
            let right = if i < self.input_types_.as_ref().unwrap().Size() {
                Some(self.input_types_.as_ref().unwrap().Peek(BottomOffset::from(i)))
            } else {
                None
            };
            if left.is_some() && right.is_some() && *left.unwrap() == *right.unwrap() {
                error << **left.unwrap() << "\n";
            } else {
                if left.is_some() {
                    error << **left.unwrap();
                } else {
                    error << "/*missing*/";
                }
                error << "   =>   ";
                if right.is_some() {
                    error << **right.unwrap();
                } else {
                    error << "/*missing*/";
                }
                error << "\n";
            }
        }
        ReportError(error.str());
    }

    pub fn Retype(&mut self) {
        let mut current_stack = self.InputTypes().clone();
        let cfg = unsafe { &*self.cfg_ };
        for instruction in self.instructions().iter() {
            instruction.type_instruction(&mut current_stack, cfg);
        }
    }

    pub fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions_
    }

    pub fn instructions_const(&self) -> &Vec<Instruction> {
        &self.instructions_
    }

    pub fn IsComplete(&self) -> bool {
        !self.instructions_.is_empty() && self.instructions_.last().unwrap().is_block_terminator()
    }

    pub fn id(&self) -> usize {
        self.id_
    }
    pub fn IsDeferred(&self) -> bool {
        self.is_deferred_
    }

    pub fn MergeInputDefinitions(
        &mut self,
        input_definitions: &Stack<DefinitionLocation>,
        worklist: &mut Worklist<*mut Block>,
    ) {
        if self.input_definitions_.is_none() {
            self.input_definitions_ = Some(input_definitions.clone());
            if worklist.non_empty() {
                worklist.Enqueue(self as *mut Block);
            }
            return;
        }

        assert_eq!(
            self.input_definitions_.as_ref().unwrap().Size(),
            input_definitions.Size()
        );
        let mut changed = false;
        for i in 0..input_definitions.AboveTop() {
            let current = self
                .input_definitions_
                .as_mut()
                .unwrap()
                .Peek(BottomOffset::from(i))
                .clone();
            let input = input_definitions.Peek(BottomOffset::from(i)).clone();
            if current == input {
                continue;
            }
            if current == DefinitionLocation::Phi(self as *mut Block, i) {
                continue;
            }
            self.input_definitions_
                .as_mut()
                .unwrap()
                .Poke(BottomOffset::from(i), DefinitionLocation::Phi(self as *mut Block, i));
            changed = true;
        }

        if changed && worklist.non_empty() {
            worklist.Enqueue(self as *mut Block);
        }
    }

    pub fn HasInputDefinitions(&self) -> bool {
        self.input_definitions_.is_some()
    }
    pub fn InputDefinitions(&self) -> &Stack<DefinitionLocation> {
        assert!(self.HasInputDefinitions());
        self.input_definitions_.as_ref().unwrap()
    }

    pub fn IsDead(&self) -> bool {
        !self.HasInputDefinitions()
    }
}

#[derive(Debug)]
pub struct ControlFlowGraph {
    blocks_: Vec<Block>,
    start_: *mut Block,
    placed_blocks_: Vec<*mut Block>,
    end_: Option<*mut Block>,
    return_type_: Option<Vec<*const Type>>,
    next_block_id_: usize,
}

impl ControlFlowGraph {
    pub fn new(input_types: Stack<*const Type>) -> Self {
        let mut cfg = ControlFlowGraph {
            blocks_: Vec::new(),
            start_: ptr::null_mut(),
            placed_blocks_: Vec::new(),
            end_: None,
            return_type_: None,
            next_block_id_: 0,
        };
        let start_block = cfg.NewBlock(Some(input_types), false);
        cfg.start_ = start_block;
        cfg.PlaceBlock(start_block);
        cfg
    }

    pub fn NewBlock(&mut self, input_types: Option<Stack<*const Type>>, is_deferred: bool) -> *mut Block {
        self.blocks_.push(Block::new(
            self as *mut ControlFlowGraph,
            self.next_block_id_,
            input_types,
            is_deferred,
        ));
        self.next_block_id_ += 1;
        &mut self.blocks_.last_mut().unwrap() as *mut Block
    }
    pub fn PlaceBlock(&mut self, block: *mut Block) {
        self.placed_blocks_.push(block);
    }
    pub fn UnplaceBlockIf<F>(&mut self, predicate: F)
    where
        F: FnMut(&*mut Block) -> bool,
    {
        self.placed_blocks_.retain(|block| !predicate(block));
    }
    pub fn start(&self) -> *mut Block {
        self.start_
    }
    pub fn end(&self) -> Option<*mut Block> {
        self.end_
    }
    pub fn set_end(&mut self, end: *mut Block) {
        self.end_ = Some(end);
    }
    pub fn SetReturnType(&mut self, t: Vec<*const Type>) {
        if self.return_type_.is_none() {
            self.return_type_ = Some(t);
            return;
        }
        if t != *self.return_type_.as_ref().unwrap() {
            let mut message = stringstream::new();
            message << "expected return type ";
            PrintCommaSeparatedList(message, self.return_type_.as_ref().unwrap());
            message << " instead of ";
            PrintCommaSeparatedList(message, &t);
            ReportError(message.str());
        }
    }
    pub fn blocks(&self) -> &Vec<*mut Block> {
        &self.placed_blocks_
    }
    pub fn NumberOfBlockIds(&self) -> usize {
        self.next_block_id_
    }
    pub fn ParameterCount(&self) -> usize {
        if self.start_.is_null() {
            0
        } else {
            unsafe { (*self.start_).InputTypes().Size() }
        }
    }
}

#[derive(Debug)]
pub struct CfgAssembler {
    current_stack_: Stack<*const Type>,
    cfg_: ControlFlowGraph,
    current_block_: *mut Block,
}

impl CfgAssembler {
    pub fn new(input_types: Stack<*const Type>) -> Self {
        let cfg = ControlFlowGraph::new(input_types.clone());
        CfgAssembler {
            current_stack_: input_types,
            cfg_: cfg,
            current_block_: unsafe { &mut *cfg.start() },
        }
    }

    pub fn Result(&mut self) -> &ControlFlowGraph {
        if !self.CurrentBlockIsComplete() {
            self.cfg_.set_end(self.current_block_);
        }
        self.OptimizeCfg();
        assert!(self.CfgIsComplete());
        self.ComputeInputDefinitions();
        &self.cfg_
    }

    pub fn NewBlock(&mut self, input_types: Option<Stack<*const Type>>, is_deferred: bool) -> *mut Block {
        self.cfg_.NewBlock(input_types, is_deferred)
    }

    pub fn CurrentBlockIsComplete(&self) -> bool {
        unsafe { (*self.current_block_).IsComplete() }
    }
    pub fn CfgIsComplete(&self) -> bool {
        self.cfg_.blocks().iter().all(|block| {
            (self.cfg_.end().is_some() && self.cfg_.end().unwrap() == *block)
                || unsafe { (**block).IsComplete() }
        })
    }

    pub fn Emit(&mut self, instruction: Instruction) {
        instruction.type_instruction(&mut self.current_stack_, &self.cfg_);
        unsafe {
            (&mut *self.current_block_).Add(instruction);
        }
    }

    pub fn CurrentStack(&self) -> &Stack<*const Type> {
        &self.current_stack_
    }

    pub fn TopRange(&self, slot_count: usize) -> StackRange {
        self.CurrentStack().TopRange(slot_count)
    }

    pub fn Bind(&mut self, block: *mut Block) {
        assert!(unsafe { (*self.current_block_).IsComplete() });
        assert!(unsafe { (*block).instructions().is_empty() });
        assert!(unsafe { (*block).HasInputTypes() });
        self.current_block_ = block;
        self.current_stack_ = unsafe { (*block).InputTypes().clone() };
        self.cfg_.PlaceBlock(block);
    }

    pub fn Goto(&mut self, block: *mut Block) {
        if unsafe { (*block).HasInputTypes() } {
            self.DropTo(self.cfg_.blocks().len().into());
        }
        self.Emit(Instruction::new(GotoInstruction { destination: block }));
    }

    pub fn Goto_with_preserved_slots(&mut self, block: *mut Block, preserved_slots: usize) -> StackRange {
        assert!(unsafe { (*block).HasInputTypes() });
        assert!(self.CurrentStack().Size() >= unsafe { (*block).InputTypes().Size() });
        self.Emit(Instruction::new(DeleteRangeInstruction {
            range: StackRange {
                begin: unsafe { (*block).InputTypes().AboveTop() } - preserved_slots,
                end: self.CurrentStack().AboveTop() - preserved_slots,
            },
        }));
        let preserved_slot_range = self.TopRange(preserved_slots);
        self.Emit(Instruction::new(GotoInstruction { destination: block }));
        preserved_slot_range
    }

    pub fn Branch(&mut self, if_true: *mut Block, if_false: *mut Block) {
        self.Emit(Instruction::new(BranchInstruction { if_true, if_false }));
    }

    // Delete the specified range of slots, moving upper slots to fill the gap.
    pub fn DeleteRange(&mut self, range: StackRange) {
        assert!(range.end() <= self.current_stack_.AboveTop());
        if range.Size() == 0 {
            return;
        }
        self.Emit(Instruction::new(DeleteRangeInstruction { range }));
    }

    pub fn DropTo(&mut self, new_level: BottomOffset) {
        self.DeleteRange(StackRange {
            begin: new_level.offset,
            end: self.CurrentStack().AboveTop(),
        });
    }

    pub fn Peek(&mut self, range: StackRange, type_: Option<*const Type>) -> StackRange {
        let lowered_types: Vec<*const Type>;
        if type_.is_some() {
            lowered_types = LowerType(type_.unwrap());
            assert_eq!(lowered_types.len(), range.Size());
        } else {
            lowered_types = Vec::new();
        }
        for i in 0..range.Size() {
            self.Emit(Instruction::new(PeekInstruction {
                offset: range.begin() + i,
                type_: if type_.is_some() {
                    Some(lowered_types[i])
                } else {
                    None
                },
            }));
        }
        self.TopRange(range.Size())
    }

    pub fn Poke(&mut self, destination: StackRange, origin: StackRange, type_: Option<*const Type>) {
        assert_eq!(destination.Size(), origin.Size());
        assert!(destination.end() <= origin.begin());
        assert_eq!(origin.end(), self.CurrentStack().AboveTop());
        let lowered_types: Vec<*const Type>;
        if type_.is_some() {
            lowered_types = LowerType(type_.unwrap());
            assert_eq!(lowered_types.len(), origin.Size());
        } else {
            lowered_types = Vec::new();
        }
        for i in (0..origin.Size()).rev() {
            self.Emit(Instruction::new(PokeInstruction {
                offset: destination.begin() + i,
                type_: if type_.is_some() {
                    Some(lowered_types[i])
                } else {
                    None
                },
            }));
        }
    }

    pub fn Print(&mut self, s: String) {
        self.Emit(Instruction::new(PrintErrorInstruction { message: s }));
    }

    pub fn AssertionFailure(&mut self, message: String) {
        self.Emit(Instruction::new(AbortInstruction {
            kind: AbortInstructionKind::kAssertionFailure,
            message,
        }));
    }

    pub fn Unreachable(&mut self) {
        self.Emit(Instruction::new(AbortInstruction {
            kind: AbortInstructionKind::kUnreachable,
            message: String::new(),
        }));
    }

    pub fn DebugBreak(&mut self) {
        self.Emit(Instruction::new(AbortInstruction {
            kind: AbortInstructionKind::kDebugBreak,
            message: String::new(),
        }));
    }

    pub fn PrintCurrentStack(&self, s: &mut dyn std::fmt::Write) -> fmt::Result {
        write!(s, "stack: {}", self.current_stack_)?;
        Ok(())
    }

    pub fn OptimizeCfg(&mut self) {
        let predecessor_count = CountBlockPredecessors(&self.cfg_);

        for block in self.cfg_.blocks().iter() {
            if self.cfg_.end().is_some() && self.cfg_.end().unwrap() == *block {
                continue;
            }
            if predecessor_count[unsafe { (**block).id() }] == 0 {
                continue;
            }

            while !unsafe { (**block).instructions_const() }.is_empty() {
                let instruction = unsafe { (**block).instructions_const() }.last().unwrap();
                if !instruction.is::<GotoInstruction>() {
                    break;
                }
                let destination = instruction.cast::<GotoInstruction>().destination;
                if destination as
