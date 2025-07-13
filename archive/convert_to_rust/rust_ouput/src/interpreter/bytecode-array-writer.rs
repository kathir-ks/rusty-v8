// Converted from V8 C++ source files:
// Header: bytecode-array-writer.h
// Implementation: bytecode-array-writer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
use crate::v8::*;
use crate::interpreter::bytecodes::Bytecode;
use crate::interpreter::interpreter::OperandScale;
use crate::interpreter::bytecode_generator::If;
use crate::ast::ast::instruction;
use crate::interpreter::bytecode_decoder::OperandSize;
use crate::ast::ast::CallType;

pub struct BytecodeArray;
pub struct TrustedByteArray;
pub struct SourcePositionTableBuilder;

pub struct BytecodeLabel;
pub struct BytecodeLoopHeader;
pub struct BytecodeNode;
pub struct BytecodeJumpTable;
pub struct ConstantArrayBuilder;
pub struct HandlerTableBuilder;

mod bytecode_array_writer_unittest {
    pub struct BytecodeArrayWriterUnittest;
}

#[derive(Debug)]
pub enum BytecodeArrayWriterError {
    JumpTargetNotFound,
    InvalidOperandScale,
    GenericError(String),
}

pub struct BytecodeArrayWriter {
    bytecodes_: RefCell<Vec<u8>>,
    unbound_jumps_: RefCell<usize>,
    source_position_table_builder_: SourcePositionTableBuilder,
    constant_array_builder_: Rc<RefCell<ConstantArrayBuilder>>,
    last_bytecode_: RefCell<Bytecode>,
    last_bytecode_offset_: RefCell<usize>,
    last_bytecode_had_source_info_: RefCell<bool>,
    elide_noneffectful_bytecodes_: bool,
    exit_seen_in_block_: RefCell<bool>,
}

impl BytecodeArrayWriter {
    const K_MAX_SIZE_OF_PACKED_BYTECODE: usize = 2 * 8 + 4 * 8; // Example size

    const K8_BIT_JUMP_PLACEHOLDER: u32 = 0x7f;
    const K16_BIT_JUMP_PLACEHOLDER: u32 = 0x7f | (0x7f << 8);
    const K32_BIT_JUMP_PLACEHOLDER: u32 = 0x7f | (0x7f << 8) | (0x7f << 16) | (0x7f << 24);

    pub fn new(
        constant_array_builder: Rc<RefCell<ConstantArrayBuilder>>,
        source_position_mode: SourcePositionTableBuilderRecordingMode,
    ) -> Self {
        BytecodeArrayWriter {
            bytecodes_: RefCell::new(Vec::with_capacity(512)),
            unbound_jumps_: RefCell::new(0),
            source_position_table_builder_: SourcePositionTableBuilder::new(source_position_mode),
            constant_array_builder_: constant_array_builder,
            last_bytecode_: RefCell::new(Bytecode::kIllegal),
            last_bytecode_offset_: RefCell::new(0),
            last_bytecode_had_source_info_: RefCell::new(false),
            elide_noneffectful_bytecodes_: true,
            exit_seen_in_block_: RefCell::new(false),
        }
    }

    pub fn write(&self, node: &BytecodeNode) {
        if *self.exit_seen_in_block_.borrow() {
            return;
        }

        self.update_exit_seen_in_block(node.bytecode());
        self.maybe_elide_last_bytecode(node.bytecode(), node.source_info().is_valid());
        self.update_source_position_table(node);
        self.emit_bytecode(node);
    }

    pub fn write_jump(&self, node: &BytecodeNode, label: &BytecodeLabel) {
        if *self.exit_seen_in_block_.borrow() {
            return;
        }
        self.update_exit_seen_in_block(node.bytecode());
        self.maybe_elide_last_bytecode(node.bytecode(), node.source_info().is_valid());
        self.update_source_position_table(node);
        self.emit_jump(node, label).expect("Failed to emit jump");
    }

    pub fn write_jump_loop(&self, node: &BytecodeNode, loop_header: &BytecodeLoopHeader) {
        if *self.exit_seen_in_block_.borrow() {
            return;
        }
        self.update_exit_seen_in_block(node.bytecode());
        self.maybe_elide_last_bytecode(node.bytecode(), node.source_info().is_valid());
        self.update_source_position_table(node);
        self.emit_jump_loop(node, loop_header);
    }

    pub fn write_switch(&self, node: &BytecodeNode, jump_table: &BytecodeJumpTable) {
        if *self.exit_seen_in_block_.borrow() {
            return;
        }
        self.update_exit_seen_in_block(node.bytecode());
        self.maybe_elide_last_bytecode(node.bytecode(), node.source_info().is_valid());
        self.update_source_position_table(node);
        self.emit_switch(node, jump_table);
    }

    pub fn bind_label(&self, label: &mut BytecodeLabel) {
        if !label.has_referrer_jump() {
            panic!("Label has no referrer jump");
        }
        let current_offset = self.bytecodes_().borrow().len();
        self.patch_jump(current_offset, label.jump_offset()).expect("Failed to patch jump");
        label.bind();
        self.start_basic_block();
    }

    pub fn bind_loop_header(&self, loop_header: &mut BytecodeLoopHeader) {
        let current_offset = self.bytecodes_().borrow().len();
        loop_header.bind_to(current_offset);
        if *self.exit_seen_in_block_.borrow() {
            return;
        }
        self.start_basic_block();
    }

    pub fn bind_jump_table_entry(&self, jump_table: &mut BytecodeJumpTable, case_value: i32) {
        if jump_table.is_bound(case_value) {
            panic!("Jump table entry already bound");
        }

        let current_offset = self.bytecodes_().borrow().len();
        let relative_jump = current_offset - jump_table.switch_bytecode_offset();

        self.constant_array_builder_().borrow_mut().set_jump_table_smi(
            jump_table.constant_pool_entry_for(case_value),
            relative_jump as i32,
        );
        jump_table.mark_bound(case_value);

        self.start_basic_block();
    }

    pub fn bind_handler_target(
        &self,
        handler_table_builder: &mut HandlerTableBuilder,
        handler_id: i32,
    ) {
        let current_offset = self.bytecodes_().borrow().len();
        self.start_basic_block();
        handler_table_builder.set_handler_target(handler_id, current_offset);
    }

    pub fn bind_try_region_start(
        &self,
        handler_table_builder: &mut HandlerTableBuilder,
        handler_id: i32,
    ) {
        let current_offset = self.bytecodes_().borrow().len();
        self.invalidate_last_bytecode();
        handler_table_builder.set_try_region_start(handler_id, current_offset);
    }

    pub fn bind_try_region_end(
        &self,
        handler_table_builder: &mut HandlerTableBuilder,
        handler_id: i32,
    ) {
        self.invalidate_last_bytecode();
        let current_offset = self.bytecodes_().borrow().len();
        handler_table_builder.set_try_region_end(handler_id, current_offset);
    }

    pub fn set_function_entry_source_position(&self, position: i32) {
        let is_statement = false;
        self.source_position_table_builder_.add_position(
            0,
            position,
            is_statement,
        );
    }

    pub fn to_bytecode_array<T>(
        &self,
        isolate: &T,
        register_count: i32,
        parameter_count: u16,
        max_arguments: u16,
        handler_table: DirectHandle<TrustedByteArray>,
    ) -> Handle<BytecodeArray> {
        if *self.unbound_jumps_().borrow() != 0 {
            panic!("Unbound jumps detected");
        }

        let bytecode_size = self.bytecodes_().borrow().len() as i32;
        let frame_size = register_count * 4; // Assuming kSystemPointerSize is 4
        let constant_pool = self.constant_array_builder_().borrow_mut().to_fixed_array(isolate);
        Handle::new() // Replace with actual bytecode array creation logic
    }

    pub fn to_source_position_table<T>(&self, isolate: &T) -> DirectHandle<TrustedByteArray> {
        if self.source_position_table_builder_.lazy() {
            panic!("Source position table builder is lazy");
        }

        if self.source_position_table_builder_.omit() {
            return DirectHandle::new(); // Replace with actual empty trusted byte array
        }

        self.source_position_table_builder_.to_source_position_table(isolate)
    }

    #[cfg(debug_assertions)]
    pub fn check_bytecode_matches(&self, _bytecode: &BytecodeArray) -> i32 {
        -1 // Replace with actual implementation
    }

    pub fn remainder_of_block_is_dead(&self) -> bool {
        *self.exit_seen_in_block_.borrow()
    }

    fn patch_jump(&self, jump_target: usize, jump_location: usize) -> Result<(), BytecodeArrayWriterError> {
        let mut bytecodes = self.bytecodes_().borrow_mut();
        let jump_bytecode = bytecodes[jump_location];
        let delta = jump_target as i32 - jump_location as i32;

        let prefix_offset: usize;
        let operand_scale: OperandScale;
        let mut jump_bytecode_unscaled = jump_bytecode;

        if Bytecodes::is_prefix_scaling_bytecode(Bytecode::from_byte(jump_bytecode)) {
            prefix_offset = 1;
            operand_scale = Bytecodes::prefix_bytecode_to_operand_scale(Bytecode::from_byte(jump_bytecode));
             jump_bytecode_unscaled = bytecodes[jump_location + prefix_offset];
        } else {
            prefix_offset = 0;
            operand_scale = OperandScale::kSingle;
        }

        if !Bytecodes::is_jump(Bytecode::from_byte(jump_bytecode_unscaled)){
            return Err(BytecodeArrayWriterError::GenericError(String::from("Jump bytecode not valid")))
        }

        match operand_scale {
            OperandScale::kSingle => {
                self.patch_jump_with_8_bit_operand(jump_location, delta)?;
            }
            OperandScale::kDouble => {
                self.patch_jump_with_16_bit_operand(jump_location + prefix_offset, delta)?;
            }
            OperandScale::kQuadruple => {
                self.patch_jump_with_32_bit_operand(jump_location + prefix_offset, delta)?;
            }
            _ => {
                return Err(BytecodeArrayWriterError::InvalidOperandScale);
            }
        }

        *self.unbound_jumps_().borrow_mut() -= 1;
        Ok(())
    }

    fn patch_jump_with_8_bit_operand(&self, jump_location: usize, delta: i32) -> Result<(), BytecodeArrayWriterError> {
        let mut bytecodes = self.bytecodes_().borrow_mut();
        let jump_bytecode = Bytecode::from_byte(bytecodes[jump_location]);

        if !Bytecodes::is_forward_jump(jump_bytecode) {
            return Err(BytecodeArrayWriterError::GenericError(String::from("Not forward jump")));
        }

        let operand_location = jump_location + 1;
        if bytecodes[operand_location] != BytecodeArrayWriter::K8_BIT_JUMP_PLACEHOLDER as u8 {
            return Err(BytecodeArrayWriterError::GenericError(String::from("Invalid jump placeholder")));
        }

        if Bytecodes::scale_for_unsigned_operand(delta as u32) == OperandScale::kSingle {
            self.constant_array_builder_().borrow_mut().discard_reserved_entry(OperandSize::kByte);
            bytecodes[operand_location] = delta as u8;
        } else {
            let entry = self.constant_array_builder_().borrow_mut().commit_reserved_entry(
                OperandSize::kByte,
                delta,
            );
            let jump_bytecode = BytecodeArrayWriter::get_jump_with_constant_operand(jump_bytecode);
            bytecodes[jump_location] = jump_bytecode.into();
            bytecodes[operand_location] = entry as u8;
        }

        Ok(())
    }

    fn patch_jump_with_16_bit_operand(&self, jump_location: usize, delta: i32) -> Result<(), BytecodeArrayWriterError> {
        let mut bytecodes = self.bytecodes_().borrow_mut();
        let jump_bytecode = Bytecode::from_byte(bytecodes[jump_location]);

        if !Bytecodes::is_forward_jump(jump_bytecode) {
            return Err(BytecodeArrayWriterError::GenericError(String::from("Not forward jump")));
        }

        let operand_location = jump_location + 1;
        let mut operand_bytes = [0u8; 2];
        if Bytecodes::scale_for_unsigned_operand(delta as u32) <= OperandScale::kDouble {
            self.constant_array_builder_().borrow_mut().discard_reserved_entry(OperandSize::kShort);
            operand_bytes[0] = (delta & 0xFF) as u8;
            operand_bytes[1] = ((delta >> 8) & 0xFF) as u8;
        } else {
            let entry = self.constant_array_builder_().borrow_mut().commit_reserved_entry(
                OperandSize::kShort,
                delta,
            );
            let jump_bytecode = BytecodeArrayWriter::get_jump_with_constant_operand(jump_bytecode);
            bytecodes[jump_location] = jump_bytecode.into();
            operand_bytes[0] = (entry & 0xFF) as u8;
            operand_bytes[1] = ((entry >> 8) & 0xFF) as u8;
        }

        bytecodes[operand_location -1] = operand_bytes[0];
        bytecodes[operand_location] = operand_bytes[1];

        Ok(())
    }

    fn patch_jump_with_32_bit_operand(&self, jump_location: usize, delta: i32) -> Result<(), BytecodeArrayWriterError> {
        let mut bytecodes = self.bytecodes_().borrow_mut();
        self.constant_array_builder_().borrow_mut().discard_reserved_entry(OperandSize::kQuad);

        let operand_location = jump_location + 1;
        let mut operand_bytes = [0u8; 4];
        operand_bytes[0] = (delta & 0xFF) as u8;
        operand_bytes[1] = ((delta >> 8) & 0xFF) as u8;
        operand_bytes[2] = ((delta >> 16) & 0xFF) as u8;
        operand_bytes[3] = ((delta >> 24) & 0xFF) as u8;

        bytecodes[operand_location -3] = operand_bytes[0];
        bytecodes[operand_location -2] = operand_bytes[1];
        bytecodes[operand_location -1] = operand_bytes[2];
        bytecodes[operand_location] = operand_bytes[3];

        Ok(())
    }

    fn emit_jump_loop(&self, node: &BytecodeNode, loop_header: &BytecodeLoopHeader) {
        let current_offset = self.bytecodes_().borrow().len();

        if current_offset < loop_header.offset() {
            panic!("current_offset < loop_header.offset()")
        }
        self.emit_bytecode(node);
    }

    fn emit_jump(&self, node: &BytecodeNode, label: &BytecodeLabel) -> Result<(), BytecodeArrayWriterError> {
        let current_offset = self.bytecodes_().borrow().len();

        *self.unbound_jumps_().borrow_mut() += 1;
        label.set_referrer(current_offset);
        let reserved_operand_size = self
            .constant_array_builder_()
            .borrow_mut()
            .create_reserved_entry(OperandSize::from(node.operand_scale()));
        match reserved_operand_size {
            OperandSize::kNone => Err(BytecodeArrayWriterError::GenericError(String::from("Operand size is none")))?,
            OperandSize::kByte => {
                node.update_operand0(BytecodeArrayWriter::K8_BIT_JUMP_PLACEHOLDER);
            }
            OperandSize::kShort => {
                node.update_operand0(BytecodeArrayWriter::K16_BIT_JUMP_PLACEHOLDER);
            }
            OperandSize::kQuad => {
                node.update_operand0(BytecodeArrayWriter::K32_BIT_JUMP_PLACEHOLDER);
            }
        }
        self.emit_bytecode(node);
        Ok(())
    }

    fn emit_switch(&self, node: &BytecodeNode, jump_table: &BytecodeJumpTable) {
        let current_offset = self.bytecodes_().borrow().len();
        jump_table.set_switch_bytecode_offset(current_offset);
        self.emit_bytecode(node);
    }

    fn update_source_position_table(&self, node: &BytecodeNode) {
        let bytecode_offset = self.bytecodes_().borrow().len() as i32;
        let source_info = node.source_info();
        if source_info.is_valid() {
            self.source_position_table_builder_.add_position(
                bytecode_offset,
                source_info.source_position(),
                source_info.is_statement(),
            );
        }
    }

    fn update_exit_seen_in_block(&self, bytecode: Bytecode) {
        let mut exit_seen = self.exit_seen_in_block_.borrow_mut();
        match bytecode {
            Bytecode::kReturn
            | Bytecode::kThrow
            | Bytecode::kReThrow
            | Bytecode::kAbort
            | Bytecode::kJump
            | Bytecode::kJumpLoop
            | Bytecode::kJumpConstant
            | Bytecode::kSuspendGenerator => {
                *exit_seen = true;
            }
            _ => {}
        }
    }

    fn maybe_elide_last_bytecode(&self, next_bytecode: Bytecode, has_source_info: bool) {
        if !self.elide_noneffectful_bytecodes_ {
            return;
        }

        if Bytecodes::is_accumulator_load_without_effects(*self.last_bytecode_().borrow())
            && Bytecodes::get_implicit_register_use(next_bytecode)
                == ImplicitRegisterUse::kWriteAccumulator
            && (!*self.last_bytecode_had_source_info_().borrow() || !has_source_info)
        {
            let mut bytecodes = self.bytecodes_().borrow_mut();
            if bytecodes.len() > *self.last_bytecode_offset_().borrow() {
                bytecodes.resize(*self.last_bytecode_offset_().borrow());
                let mut last_had_source = self.last_bytecode_had_source_info_().borrow_mut();
                has_source_info || *last_had_source;
            }
        }
        *self.last_bytecode_().borrow_mut() = next_bytecode;
        *self.last_bytecode_had_source_info_().borrow_mut() = has_source_info;
        *self.last_bytecode_offset_().borrow_mut() = self.bytecodes_().borrow().len();
    }

    fn invalidate_last_bytecode(&self) {
        *self.last_bytecode_().borrow_mut() = Bytecode::kIllegal;
    }

    fn emit_bytecode(&self, node: &BytecodeNode) {
        let bytecode = node.bytecode();
        let operand_scale = node.operand_scale();

        if operand_scale != OperandScale::kSingle {
            let prefix = Bytecodes::operand_scale_to_prefix_bytecode(operand_scale);
            self.bytecodes_().borrow_mut().push(Bytecodes::to_byte(prefix));
        }
        self.bytecodes_().borrow_mut().push(Bytecodes::to_byte(bytecode));

        let operands = node.operands();
        let operand_count = node.operand_count();
        let operand_sizes = Bytecodes::get_operand_sizes(bytecode, operand_scale);
        for i in 0..operand_count {
            match operand_sizes[i as usize] {
                OperandSize::kNone => panic!("Operand size is none"),
                OperandSize::kByte => {
                    self.bytecodes_().borrow_mut().push(operands[i as usize] as u8);
                }
                OperandSize::kShort => {
                    let operand = operands[i as usize] as u16;
                    let raw_operand = operand.to_ne_bytes();
                    self.bytecodes_().borrow_mut().push(raw_operand[0]);
                    self.bytecodes_().borrow_mut().push(raw_operand[1]);
                }
                OperandSize::kQuad => {
                    let raw_operand = (operands[i as usize] as u32).to_ne_bytes();
                    self.bytecodes_().borrow_mut().push(raw_operand[0]);
                    self.bytecodes_().borrow_mut().push(raw_operand[1]);
                    self.bytecodes_().borrow_mut().push(raw_operand[2]);
                    self.bytecodes_().borrow_mut().push(raw_operand[3]);
                }
            }
        }
    }

    fn get_jump_with_constant_operand(jump_bytecode: Bytecode) -> Bytecode {
        match jump_bytecode {
            Bytecode::kJump => Bytecode::kJumpConstant,
            Bytecode::kJumpIfTrue => Bytecode::kJumpIfTrueConstant,
            Bytecode::kJumpIfFalse => Bytecode::kJumpIfFalseConstant,
            Bytecode::kJumpIfToBooleanTrue => Bytecode::kJumpIfToBooleanTrueConstant,
            Bytecode::kJumpIfToBooleanFalse => Bytecode::kJumpIfToBooleanFalseConstant,
            Bytecode::kJumpIfNull => Bytecode::kJumpIfNullConstant,
            Bytecode::kJumpIfNotNull => Bytecode::kJumpIfNotNullConstant,
            Bytecode::kJumpIfUndefined => Bytecode::kJumpIfUndefinedConstant,
            Bytecode::kJumpIfNotUndefined => Bytecode::kJumpIfNotUndefinedConstant,
            Bytecode::kJumpIfUndefinedOrNull => Bytecode::kJumpIfUndefinedOrNullConstant,
            Bytecode::kJumpIfJSReceiver => Bytecode::kJumpIfJSReceiverConstant,
            Bytecode::kJumpIfForInDone => Bytecode::kJumpIfForInDoneConstant,
            _ => panic!("Invalid jump bytecode"),
        }
    }

    fn bytecodes_(&self) -> &RefCell<Vec<u8>> {
        &self.bytecodes_
    }
    fn unbound_jumps_(&self) -> &RefCell<usize> {
        &self.unbound_jumps_
    }
    fn source_position_table_builder_(&self) -> &SourcePositionTableBuilder {
        &self.source_position_table_builder_
    }
    fn constant_array_builder_(&self) -> &Rc<RefCell<ConstantArrayBuilder>> {
        &self.constant_array_builder_
    }
    fn last_bytecode_(&self) -> &RefCell<Bytecode> {
        &self.last_bytecode_
    }
    fn last_bytecode_offset_(&self) -> &RefCell<usize> {
        &self.last_bytecode_offset_
    }
    fn last_bytecode_had_source_info_(&self) -> &RefCell<bool> {
        &self.last_bytecode_had_source_info_
    }
    fn start_basic_block(&self) {
        self.invalidate_last_bytecode();
        *self.exit_seen_in_block_.borrow_mut() = false;
    }
}

pub enum SourcePositionTableBuilderRecordingMode {
    Precise,
    Lazy,
    Omit,
}

impl SourcePositionTableBuilder {
    pub fn new(mode: SourcePositionTableBuilderRecordingMode) -> Self {
        SourcePositionTableBuilder {}
    }

    pub fn add_position(&mut self, bytecode_offset: i32, source_position: i32, is_statement: bool) {}
    pub fn lazy(&self) -> bool { false }
    pub fn omit(&self) -> bool { false }
    pub fn to_source_position_table<T>(&self, isolate: &T) -> DirectHandle<TrustedByteArray> { DirectHandle::new() }
}

impl BytecodeLabel {
    pub fn has_referrer_jump(&self) -> bool { false }
    pub fn jump_offset(&self) -> usize { 0 }
    pub fn bind(&mut self) {}
    pub fn set_referrer(&self, current_offset: usize) {}
}

impl BytecodeLoopHeader {
    pub fn bind_to(&mut self, current_offset: usize) {}
    pub fn offset(&self) -> usize { 0 }
}

impl BytecodeNode {
    pub fn bytecode(&self) -> Bytecode { Bytecode::kIllegal }
    pub fn operand_scale(&self) -> OperandScale { OperandScale::kSingle }
    pub fn operands(&self) -> &[u32] { &[] }
    pub fn operand_count(&self) -> i32 { 0 }
    pub fn source_info(&self) -> &BytecodeSourceInfo { &BytecodeSourceInfo {} }
    pub fn update_operand0(&self, value: u32) {}
}

impl BytecodeJumpTable {
    pub fn is_bound(&self, case_value: i32) -> bool { false }
    pub fn switch_bytecode_offset(&self) -> usize { 0 }
    pub fn constant_pool_entry_for(&self, case_value: i32) -> usize { 0 }
    pub fn mark_bound(&mut self, case_value: i32) {}
    pub fn set_switch_bytecode_offset(&self, current_offset: usize) {}
}

impl ConstantArrayBuilder {
    pub fn set_jump_table_smi(&mut self, constant_pool_entry_for: usize, relative_jump: i32) {}
    pub fn create_reserved_entry(&mut self, scale_from: OperandSize) -> OperandSize {scale_from}
    pub fn discard_reserved_entry(&mut self, entry: OperandSize) {}
    pub fn commit_reserved_entry(&mut self, entry: OperandSize, value: i32) -> usize {0}
    pub fn to_fixed_array<T>(&mut self, isolate: &T) -> DirectHandle<TrustedFixedArray>{ DirectHandle::new() }
}

impl HandlerTableBuilder {
    pub fn set_handler_target(&mut self, handler_id: i32, current_offset: usize) {}
    pub fn set_try_region_start(&mut self, handler_id: i32, current_offset: usize) {}
    pub fn set_try_region_end(&mut self, handler_id: i32, current_offset: usize) {}
}

pub struct BytecodeSourceInfo {}

impl BytecodeSourceInfo {
    pub fn is_valid(&self) -> bool {
        false
    }
    pub fn source_position(&self) -> i32 {
        0
    }
    pub fn is_statement(&self) -> bool {
        false
    }
}

impl From<OperandScale> for OperandSize{
    fn from(scale: OperandScale) -> Self {
        match scale{
            OperandScale::kSingle => OperandSize::kByte,
            OperandScale::kDouble => OperandSize::kShort,
            OperandScale::kQuadruple => OperandSize::kQuad,
            _ => OperandSize::kNone
        }
    }
}

pub struct TrustedFixedArray;

pub struct Handle<T>{}

impl <T> Handle<T>{
    pub fn new() -> Self{
        Handle{}
    }
}

pub struct DirectHandle<T>{}

impl <T> DirectHandle<T>{
    pub fn new() -> Self{
        DirectHandle{}
    }
}

pub enum ImplicitRegisterUse {
    kNone,
    kReadAccumulator,
    kWriteAccumulator,
    kReadWriteAccumulator,
}
