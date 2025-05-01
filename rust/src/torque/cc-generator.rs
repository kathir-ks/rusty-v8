// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;
use std::{fmt, result};

mod common {
    pub mod globals {}
}
mod torque {
    pub mod global_context {}
    pub mod type_oracle {}
    pub mod types {
        use std::fmt;
        use std::rc::Rc;
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct NameAndType {
            pub name: String,
            pub type_: Rc<Type>,
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Field {
            pub name_and_type: NameAndType,
        }

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Type {
            name: String,
        }

        impl Type {
            pub fn new(name: String) -> Self {
                Type { name }
            }
            pub fn is_constexpr(&self) -> bool {
                // Placeholder implementation
                false
            }
            pub fn get_debug_type(&self) -> String {
                format!("Debug<{}>", self.name)
            }
            pub fn get_runtime_type(&self) -> String {
                self.name.clone()
            }
            pub fn struct_supertype(&self) -> Option<Rc<StructType>> {
                None
            }
            pub fn is_subtype_of(&self, _other: &Type) -> bool {
                false
            }
            pub fn get_constexpr_generated_type_name(&self) -> String {
                self.name.clone()
            }
        }
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct StructType {
            name: String,
            fields: Vec<Field>,
        }

        impl StructType {
            pub fn new(name: String, fields: Vec<Field>) -> Self {
                StructType { name, fields }
            }

            pub fn fields(&self) -> &Vec<Field> {
                &self.fields
            }
        }

        impl fmt::Display for Type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.name)
            }
        }

        pub type TypeVector = Vec<Rc<Type>>;
        pub struct TypeOracle {}
        impl TypeOracle {
            pub fn get_smi_type() -> Rc<Type> {
                Rc::new(Type::new("Smi".to_string()))
            }
            pub fn get_heap_object_type() -> Rc<Type> {
                Rc::new(Type::new("HeapObject".to_string()))
            }
            pub fn get_uninitialized_heap_object_type() -> Rc<Type> {
                Rc::new(Type::new("UninitializedHeapObject".to_string()))
            }
            pub fn get_tagged_type() -> Rc<Type> {
                Rc::new(Type::new("Tagged".to_string()))
            }
            pub fn get_smi_tagged_generic() -> Rc<Type> {
                Rc::new(Type::new("SmiTaggedGeneric".to_string()))
            }
        }
        pub fn match_unary_generic(_ty: &Type, _generic: Rc<Type>) -> Option<Rc<Type>> {
            None
        }

    }
    pub mod utils {
        use std::fmt::Write;
        pub fn string_literal_quote(s: &str) -> String {
            format!("\"{}\"", s)
        }

        pub fn print_comma_separated_list<T: std::fmt::Display>(
            out: &mut impl Write,
            list: &Vec<T>,
        ) -> std::fmt::Result {
            for (i, item) in list.iter().enumerate() {
                if i > 0 {
                    write!(out, ", ")?;
                }
                write!(out, "{}", item)?;
            }
            Ok(())
        }
    }
}

use common::globals::*;
use torque::global_context::*;
use torque::type_oracle::*;
use torque::types::*;
use torque::utils::*;

type Result<T> = result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BottomOffset {
    offset: usize,
}

impl BottomOffset {
    fn new(offset: usize) -> Self {
        Self { offset }
    }
}

impl std::ops::Add for BottomOffset {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            offset: self.offset + other.offset,
        }
    }
}

impl std::ops::AddAssign for BottomOffset {
    fn add_assign(&mut self, other: Self) {
        self.offset += other.offset;
    }
}

impl std::ops::Sub for BottomOffset {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            offset: self.offset - other.offset,
        }
    }
}

impl std::ops::SubAssign for BottomOffset {
    fn sub_assign(&mut self, other: Self) {
        self.offset -= other.offset;
    }
}

impl std::fmt::Display for BottomOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.offset)
    }
}

struct Stack<T> {
    data: Vec<T>,
}

impl<T: Clone> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> T {
        self.data.pop().expect("Stack::pop(): stack is empty")
    }

    fn pop_many(&mut self, count: usize) {
        if count > self.data.len() {
            panic!("Stack::pop_many(): count exceeds stack size");
        }
        self.data.truncate(self.data.len() - count);
    }

    fn peek(&self, offset: BottomOffset) -> &T {
        &self.data[offset.offset]
    }

    fn poke(&mut self, index: usize, value: T) {
        self.data[index] = value;
    }

    fn top(&self) -> &T {
        self.data.last().expect("Stack::top(): stack is empty")
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn above_top(&self) -> BottomOffset {
        BottomOffset::new(self.size())
    }

    fn top_range(&self, slot_count: usize) -> StackRange {
        let end = self.data.len();
        let start = end - slot_count;
        StackRange { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StackRange {
    start: usize,
    end: usize,
}

impl StackRange {
    fn size(&self) -> usize {
        self.end - self.start
    }

    fn begin(&self) -> BottomOffset {
        BottomOffset::new(self.start)
    }
}

#[derive(Debug, Clone)]
struct SourcePosition {
    source: String,
    start: Position,
}

impl SourcePosition {
    fn compare_start_ignore_column(&self, other: &SourcePosition) -> bool {
        self.source == other.source && self.start.line == other.start.line
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    line: usize,
}

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    input_types: Vec<Rc<Type>>,
    input_definitions: Vec<DefinitionLocation>,
    instructions: Vec<Instruction>,
}

impl Block {
    fn new(id: usize) -> Self {
        Block {
            id,
            input_types: Vec::new(),
            input_definitions: Vec::new(),
            instructions: Vec::new(),
        }
    }

    fn is_dead(&self) -> bool {
        false
    }

    fn input_types(&self) -> &Vec<Rc<Type>> {
        &self.input_types
    }

    fn input_definitions(&self) -> &Vec<DefinitionLocation> {
        &self.input_definitions
    }

    fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

#[derive(Debug, Clone)]
enum DefinitionLocation {
    Parameter(usize),
    Value(usize),
}

impl DefinitionLocation {
    fn is_phi_from_block(&self, _block: &Block) -> bool {
        // Placeholder implementation
        true
    }
}

#[derive(Debug, Clone)]
struct CFG {
    blocks: Vec<Block>,
    start: usize,
    end: Option<usize>,
}

impl CFG {
    fn new(start: usize) -> Self {
        CFG {
            blocks: Vec::new(),
            start,
            end: None,
        }
    }

    fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    fn end(&self) -> &Option<usize> {
        &self.end
    }
}

#[derive(Debug, Clone)]
struct GotoInstruction {
    destination: usize,
}

#[derive(Debug, Clone)]
struct BranchInstruction {
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
struct ConstexprBranchInstruction {
    condition: String,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
struct GotoInstruction {
    destination: usize,
}

#[derive(Debug, Clone)]
struct AbortInstruction {
    kind: AbortInstructionKind,
    message: String,
    pos: SourcePosition,
}

#[derive(Debug, Clone)]
enum AbortInstructionKind {
    kUnreachable,
    kDebugBreak,
    kAssertionFailure,
}

#[derive(Debug, Clone)]
struct UnsafeCastInstruction {
    destination_type: Rc<Type>,
    value_definition: usize,
}

#[derive(Debug, Clone)]
struct LoadReferenceInstruction {
    type_: Rc<Type>,
    value_definition: usize,
    synchronization: FieldSynchronization,
}

#[derive(Debug, Clone)]
enum FieldSynchronization {
    kNone,
    kRelaxed,
    kAcquireRelease,
}

#[derive(Debug, Clone)]
struct LoadBitFieldInstruction {
    bit_field_struct_type: Rc<Type>,
    bit_field: BitField,
    value_definition: usize,
}

#[derive(Debug, Clone)]
struct BitField {
    name_and_type: NameAndType,
    offset: usize,
    num_bits: usize,
}

#[derive(Debug, Clone)]
struct CallIntrinsicInstruction {
    intrinsic: Rc<Intrinsic>,
    constexpr_arguments: Vec<String>,
    value_definitions: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Intrinsic {
    name: String,
    external_name: String,
    signature: Signature,
}

impl Intrinsic {
    fn external_name(&self) -> &str {
        &self.external_name
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }
}

#[derive(Debug, Clone)]
struct Signature {
    parameter_types: ParameterTypes,
    return_type: Rc<Type>,
}

#[derive(Debug, Clone)]
struct ParameterTypes {
    types: TypeVector,
}

#[derive(Debug, Clone)]
struct CallCsaMacroInstruction {
    macro_: Rc<CsaMacro>,
    constexpr_arguments: Vec<String>,
    value_definitions: Vec<usize>,
    catch_block: Option<usize>,
}

#[derive(Debug, Clone)]
struct CsaMacro {
    name: String,
    cc_name: String,
    cc_debug_name: String,
    signature: Signature,
}

impl CsaMacro {
    fn cc_name(&self) -> &str {
        &self.cc_name
    }

    fn cc_debug_name(&self) -> &str {
        &self.cc_debug_name
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Goto(GotoInstruction),
    Branch(BranchInstruction),
    ConstexprBranch(ConstexprBranchInstruction),
    Abort(AbortInstruction),
    UnsafeCast(UnsafeCastInstruction),
    LoadReference(LoadReferenceInstruction),
    LoadBitField(LoadBitFieldInstruction),
    CallIntrinsic(CallIntrinsicInstruction),
    CallCsaMacro(CallCsaMacroInstruction),
    PrintError(PrintErrorInstruction),
    // Missing instructions:
    // PushUninitializedInstruction,
    // PushBuiltinPointerInstruction,
    // NamespaceConstantInstruction,
    // CallCsaMacroAndBranchInstruction,
    // MakeLazyNodeInstruction,
    // CallBuiltinInstruction,
    // CallBuiltinPointerInstruction,
    // CallRuntimeInstruction,
    // GotoExternalInstruction,
    // ReturnInstruction,
    // StoreReferenceInstruction,
    // StoreBitFieldInstruction,
}

#[derive(Debug, Clone)]
struct PrintErrorInstruction {
    message: String,
}

impl From<GotoInstruction> for Instruction {
    fn from(instruction: GotoInstruction) -> Self {
        Instruction::Goto(instruction)
    }
}

impl From<BranchInstruction> for Instruction {
    fn from(instruction: BranchInstruction) -> Self {
        Instruction::Branch(instruction)
    }
}

impl From<ConstexprBranchInstruction> for Instruction {
    fn from(instruction: ConstexprBranchInstruction) -> Self {
        Instruction::ConstexprBranch(instruction)
    }
}

impl From<AbortInstruction> for Instruction {
    fn from(instruction: AbortInstruction) -> Self {
        Instruction::Abort(instruction)
    }
}

impl From<UnsafeCastInstruction> for Instruction {
    fn from(instruction: UnsafeCastInstruction) -> Self {
        Instruction::UnsafeCast(instruction)
    }
}

impl From<LoadReferenceInstruction> for Instruction {
    fn from(instruction: LoadReferenceInstruction) -> Self {
        Instruction::LoadReference(instruction)
    }
}

impl From<LoadBitFieldInstruction> for Instruction {
    fn from(instruction: LoadBitFieldInstruction) -> Self {
        Instruction::LoadBitField(instruction)
    }
}

impl From<CallIntrinsicInstruction> for Instruction {
    fn from(instruction: CallIntrinsicInstruction) -> Self {
        Instruction::CallIntrinsic(instruction)
    }
}

impl From<CallCsaMacroInstruction> for Instruction {
    fn from(instruction: CallCsaMacroInstruction) -> Self {
        Instruction::CallCsaMacro(instruction)
    }
}

impl From<PrintErrorInstruction> for Instruction {
    fn from(instruction: PrintErrorInstruction) -> Self {
        Instruction::PrintError(instruction)
    }
}

#[derive(Debug, Clone)]
struct VisitResult<'a> {
    type_: Rc<Type>,
    stack_range: StackRange,
    constexpr_value: Option<String>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> VisitResult<'a> {
    fn new(type_: Rc<Type>, stack_range: StackRange) -> Self {
        VisitResult {
            type_,
            stack_range,
            constexpr_value: None,
            _phantom: std::marker::PhantomData,
        }
    }

    fn new_constexpr(type_: Rc<Type>, constexpr_value: String) -> Self {
        VisitResult {
            type_,
            stack_range: StackRange { start: 0, end: 0 }, // Dummy range
            constexpr_value: Some(constexpr_value),
            _phantom: std::marker::PhantomData,
        }
    }

    fn is_on_stack(&self) -> bool {
        self.constexpr_value.is_none()
    }

    fn type_(&self) -> &Rc<Type> {
        &self.type_
    }

    fn stack_range(&self) -> StackRange {
        self.stack_range
    }

    fn constexpr_value(&self) -> &String {
        self.constexpr_value.as_ref().unwrap()
    }
}

mod source_file_map {
    pub fn absolute_path(source: String) -> String {
        source // Placeholder implementation
    }
    pub fn path_from_v8_root(source: String) -> String {
        source
    }
}

/// Generates C++ code from Torque IR.
pub struct CCGenerator<'a, W: Write> {
    out_: &'a mut W,
    decls_: String,
    cfg_: CFG,
    is_cc_debug_: bool,
    variable_definitions: HashMap<DefinitionLocation, String>,
    previous_position_: SourcePosition,
    error_occurred: bool,
}

impl<'a, W: Write> CCGenerator<'a, W> {
    /// Creates a new CCGenerator.
    pub fn new(out: &'a mut W, cfg: CFG, is_cc_debug: bool) -> Self {
        CCGenerator {
            out_: out,
            decls_: String::new(),
            cfg_: cfg,
            is_cc_debug_: is_cc_debug,
            variable_definitions: HashMap::new(),
            previous_position_: SourcePosition {
                source: "".to_string(),
                start: Position { line: 0 },
            },
            error_occurred: false,
        }
    }

    fn out(&mut self) -> &mut W {
        self.out_
    }

    fn decls(&mut self) -> &mut String {
        &mut self.decls_
    }

    fn block_name(&self, block: &Block) -> String {
        format!("block{}", block.id)
    }

    fn definition_to_variable(&self, def: &DefinitionLocation) -> String {
        self.variable_definitions
            .get(def)
            .expect("Variable not found")
            .clone()
    }

    fn set_definition_variable(&mut self, def: DefinitionLocation, name: String) {
        self.variable_definitions.insert(def, name);
    }

    /// Emits the C++ code for the Torque IR graph.
    pub fn emit_graph(&mut self, mut parameters: Stack<String>) -> Option<Stack<String>> {
        for i in 0..parameters.above_top().offset {
            self.set_definition_variable(
                DefinitionLocation::Parameter(i),
                parameters.peek(BottomOffset::new(i)).clone(),
            );
        }

        // Redirect the output of non-declarations into a buffer and only output
        // declarations right away.
        let mut out_buffer = Vec::new();
        let old_out = std::mem::replace(self.out_, &mut out_buffer);

        self.emit_instruction(
            &Instruction::Goto(GotoInstruction {
                destination: self.cfg_.start,
            }),
            &mut parameters,
        );

        for block in self.cfg_.blocks() {
            if self.cfg_.end().map_or(false, |&end| end == block.id) {
                continue;
            }
            if block.is_dead() {
                continue;
            }
            self.emit_block(block);
        }

        let result = if let Some(&end) = self.cfg_.end() {
            Some(self.emit_block(&self.cfg_.blocks[end].clone()))
        } else {
            None
        };

        // All declarations have been printed now, so we can append the buffered
        // output and redirect back to the original output stream.
        *self.out_ = old_out;
        write!(self.out_, "{}", String::from_utf8(out_buffer).unwrap()).unwrap();

        Some(parameters)
    }

    fn emit_block(&mut self, block: &Block) -> Stack<String> {
        writeln!(self.out(), "\n  {}:", self.block_name(block)).unwrap();

        let mut stack = Stack::new();

        for i in 0..block.input_types().len() {
            let def = &block.input_definitions()[i];
            stack.push(self.definition_to_variable(def));
            if def.is_phi_from_block(block) {
                let type_ = &block.input_types()[i];
                writeln!(
                    self.decls(),
                    "  {} {} {{}}; USE({});\n",
                    if self.is_cc_debug_ {
                        type_.get_debug_type()
                    } else {
                        type_.get_runtime_type()
                    },
                    stack.top(),
                    stack.top()
                )
                .unwrap();
            }
        }

        for instruction in block.instructions() {
            self.emit_instruction(instruction, &mut stack);
        }
        stack
    }

    fn emit_source_position(&mut self, pos: SourcePosition, always_emit: bool) {
        let file = source_file_map::absolute_path(pos.source.clone());
        if always_emit || !self.previous_position_.compare_start_ignore_column(&pos) {
            // Lines in Torque SourcePositions are zero-based, while the
            // CodeStubAssembler and downwind systems are one-based.
            writeln!(self.out(), "  // {}:{}", file, pos.start.line + 1).unwrap();
            self.previous_position_ = pos;
        }
    }

    fn emit_instruction(&mut self, instruction: &Instruction, stack: &mut Stack<String>) {
        match instruction {
            Instruction::Goto(instruction) => self.emit_instruction_goto(instruction, stack),
            Instruction::Branch(instruction) => self.emit_instruction_branch(instruction, stack),
            Instruction::ConstexprBranch(instruction) => {
                self.emit_instruction_constexpr_branch(instruction, stack)
            }
            Instruction::Abort(instruction) => self.emit_instruction_abort(instruction, stack),
            Instruction::UnsafeCast(instruction) => {
                self.emit_instruction_unsafe_cast(instruction, stack)
            }
            Instruction::LoadReference(instruction) => {
                self.emit_instruction_load_reference(instruction, stack)
            }
            Instruction::LoadBitField(instruction) => {
                self.emit_instruction_load_bit_field(instruction, stack)
            }
            Instruction::CallIntrinsic(instruction) => {
                self.emit_instruction_call_intrinsic(instruction, stack)
            }
            Instruction::CallCsaMacro(instruction) => {
                self.emit_instruction_call_csa_macro(instruction, stack)
            }
            Instruction::PrintError(instruction) => {
                self.emit_instruction_print_error(instruction, stack)
            }
            // _ => self.report_error("Instruction not supported in C++ output"),
        }
    }

    fn emit_instruction_goto(&mut self, instruction: &GotoInstruction, stack: &mut Stack<String>) {
        self.emit_goto(
            &self.cfg_.blocks[instruction.destination].clone(),
            stack,
            "  ".to_string(),
        );
    }

    fn emit_instruction_branch(
        &mut self,
        instruction: &BranchInstruction,
        stack: &mut Stack<String>,
    ) {
        writeln!(self.out(), "  if ({}) {{", stack.pop()).unwrap();
        self.emit_goto(
            &self.cfg_.blocks[instruction.if_true].clone(),
            stack,
            "    ".to_string(),
        );
        writeln!(self.out(), "  }} else {{").unwrap();
        self.emit_goto(
            &self.cfg_.blocks[instruction.if_false].clone(),
            stack,
            "    ".to_string(),
        );
        writeln!(self.out(), "  }}").unwrap();
    }

    fn emit_instruction_constexpr_branch(
        &mut self,
        instruction: &ConstexprBranchInstruction,
        stack: &mut Stack<String>,
    ) {
        writeln!(self.out(), "  if (({})) {{", instruction.condition).unwrap();
        self.emit_goto(
            &self.cfg_.blocks[instruction.if_true].clone(),
            stack,
            "    ".to_string(),
        );
        writeln!(self.out(), "  }} else {{").unwrap();
        self.emit_goto(
            &self.cfg_.blocks[instruction.if_false].clone(),
            stack,
            "    ".to_string(),
        );
        writeln!(self.out(), "  }}").unwrap();
    }

    fn emit_goto(&mut self, destination: &Block, stack: &mut Stack<String>, indentation: String) {
        let destination_definitions = &destination.input_definitions();
        assert_eq!(stack.size(), destination_definitions.len());
        for i in 0..stack.above_top().offset {
            let def = &destination_definitions[i];
            if def.is_phi_from_block(destination) {
                writeln!(
                    self.out(),
                    "{} {} = {};",
                    indentation,
                    self.definition_to_variable(def),
                    stack.peek(BottomOffset::new(i))
                )
                .unwrap();
            }
        }
        writeln!(self.out(), "{}goto {};", indentation, self.block_name(destination)).unwrap();
    }

    fn emit_instruction_abort(&mut self, instruction: &AbortInstruction, _stack: &mut Stack<String>) {
        match instruction.kind {
            AbortInstructionKind::kUnreachable => {
                assert!(instruction.message.is_empty());
                writeln!(self.out(), "  UNREACHABLE();").unwrap();
            }
            AbortInstructionKind::kDebugBreak => {
                assert!(instruction.message.is_empty());
                writeln!(self.out(), "  base::OS::DebugBreak();").unwrap();
            }
            AbortInstructionKind::kAssertionFailure => {
                let file = string_literal_quote(&source_file_map::path_from_v8_root(instruction.pos.source.clone()));
                writeln!(
                    self.out(),
                    "  CHECK(false, \"Failed Torque assertion: '{}' at {}\":{}\");",
                    instruction.message,
                    file,
                    instruction.pos.start.line + 1
                )
                .unwrap();
            }
        }
    }

    fn emit_instruction_unsafe_cast(
        &mut self,
        instruction: &UnsafeCastInstruction,
        stack: &mut Stack<String>,
    ) {
        let str = format!(
            "static_cast::<{}>({})",
            instruction.destination_type.get_runtime_type(),
            stack.top()
        );
        let top_index = stack.above_top().offset - 1;
        stack.poke(top_index, str.clone());
        self.set_definition_variable(DefinitionLocation::Value(instruction.value_definition), str);
    }

    fn emit_instruction_load_reference(
        &mut self,
        instruction: &LoadReferenceInstruction,
        stack: &mut Stack<String>,
    ) {
        let result_name = self.definition_to_variable(&DefinitionLocation::Value(instruction.value_definition));
        let offset = stack.pop();
        let object = stack.pop();
        stack.push(result_name.clone());

        if !self.is_cc_debug_ {
            let result_type = instruction.type_.get_runtime_type();
            writeln!(
                self.decls(),
                "  {} {} {{}}; USE({});\n",
                result_type, result_name, result_name
            )
            .unwrap();
            write!(self.out(), "  {} = ", result_name).unwrap();
            if instruction.type_.is_subtype_of(&TypeOracle::get_tagged_type()) {
                // Currently, all of the tagged loads we emit are for smi values, so there
                // is no point in providing an PtrComprCageBase. If at some point we start
                // emitting loads for tagged fields which might be HeapObjects, then we
                // should plumb an PtrComprCageBase through the generated functions that
                // need it.
                if !instruction.type_.is_subtype_of(&TypeOracle::get_smi_type()) {
                    self.report_error(
                        "Not supported in C++ output: LoadReference on non-smi tagged value",
                    );
                    return;
                }
                if instruction.synchronization != FieldSynchronization::kNone {
                    // TODO(ishell): generate proper TaggedField<..>::load() call once
                    // there's a real use case.
                    self.report_error(
                        "Torque doesn't support @cppRelaxedLoad/@cppAcquireLoad on tagged data",
                    );
                    return;
                }
                // References and slices can cause some values to have the Torque type
                // HeapObject|TaggedZeroPattern, which is output as "Object". TaggedField
                // requires HeapObject, so we need a cast.
                writeln!(
                    self.out(),
                    "TaggedField::<{}>::load(UncheckedCast::<HeapObject>({}), static_cast::<int>({}));",
                    result_type, object, offset
                )
                .unwrap();
            } else {
                // This code replicates the way we load the field in accessors, see
                // CppClassGenerator::EmitLoadFieldStatement().
                let load = match instruction.synchronization {
                    FieldSynchronization::kNone => "ReadField",
                    FieldSynchronization::kRelaxed => "Relaxed_ReadField",
                    FieldSynchronization::kAcquireRelease => {
                        self.report_error("Torque doesn't support @cppAcquireLoad on untagged data");
                        return;
                    }
                };
                writeln!(
                    self.out(),
                    "({})->{}::<{}>({});",
                    object, load, result_type, offset
                )
                .unwrap();
            }
        } else {
            let result_type = instruction.type_.get_debug_type();
            writeln!(
                self.decls(),
                "  {} {} {{}}; USE({});\n",
                result_type, result_name, result_name
            )
            .unwrap();
            if instruction.type_.is_subtype_of(&TypeOracle::get_tagged_type()) {
                writeln!(
                    self.out(),
                    "  READ_TAGGED_FIELD_OR_FAIL({}, accessor, {}, static_cast::<int>({}));",
                    result_name, object, offset
                )
                .unwrap();
            } else {
                writeln!(
                    self.out(),
                    "  READ_FIELD_OR_FAIL({}, {}, accessor, {}, {});",
                    result_type, result_name, object, offset
                )
                .unwrap();
            }
        }
    }

    fn emit_instruction_load_bit_field(
        &mut self,
        instruction: &LoadBitFieldInstruction,
        stack: &mut Stack<String>,
    ) {
        let result_name = self.definition_to_variable(&DefinitionLocation::Value(instruction.value_definition));

        let bit_field_struct = stack.pop();
        stack.push(result_name.clone());

        let struct_type = instruction.bit_field_struct_type.clone();

        writeln!(
            self.decls(),
            "  {} {} {{}}; USE({});\n",
            instruction.bit_field.name_and_type.type_.get_runtime_type(),
            result_name,
            result_name
        )
        .unwrap();

        let mut bit_field_struct_val = bit_field_struct.clone();
        let mut struct_type_val = struct_type.clone();

        if let Some(smi_tagged_type) =
            Type::match_unary_generic(&struct_type, TypeOracle::get_smi_tagged_generic())
        {
            // Get the untagged value and its type.
            if self.is_cc_debug_ {
                bit_field_struct_val = format!("Internals::SmiValue({})", bit_field_struct);
            } else {
                bit_field_struct_val = format!("{}.value()", bit_field_struct);
            }
            struct_type_val = smi_tagged_type;
        }

        writeln!(
            self.out(),
            "  {} = CastToUnderlyingTypeIfEnum({}::decode({}));\n",
            result_name,
            get_bit_field_specialization(&struct_type_val, &instruction.bit_field),
            bit_field_struct_val
        )
        .unwrap();
    }

    fn emit_instruction_call_intrinsic(
        &mut self,
        instruction: &CallIntrinsicInstruction,
        stack: &mut Stack<String>,
    ) {
        let parameter_types = instruction.intrinsic.signature().parameter_types.types.clone();
        let args = self.process_arguments_common(parameter_types, instruction.constexpr_arguments.clone(), stack);

        let return_type = instruction.intrinsic.signature().return_type.clone();
        let mut results = Vec::new();

        let lowered = self.lower_type(&return_type);
        for i in 0..lowered.len() {
            results.push(self.definition_to_variable(&DefinitionLocation::Value(instruction.value_definitions[i])));