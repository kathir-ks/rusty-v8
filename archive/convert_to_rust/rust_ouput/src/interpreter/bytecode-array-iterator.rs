// Converted from V8 C++ source files:
// Header: bytecode-array-iterator.h
// Implementation: bytecode-array-iterator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/interpreter/bytecode-array-iterator.h

use crate::interpreter::bytecode_register::Register;
use crate::interpreter::bytecodes::Bytecode;
use crate::objects::objects::Smi;
use crate::objects::objects::Tagged;
use crate::runtime::runtime::Runtime;
use std::cell::RefCell;
use std::rc::Rc;

pub struct JumpTableTargetOffset {
    pub case_value: i32,
    pub target_offset: i32,
}

pub struct JumpTableTargetOffsets<'a> {
    iterator_: &'a BytecodeArrayIterator,
    table_start_: i32,
    table_size_: i32,
    case_value_base_: i32,
}

impl<'a> JumpTableTargetOffsets<'a> {
    pub fn new(
        iterator_: &'a BytecodeArrayIterator,
        table_start_: i32,
        table_size_: i32,
        case_value_base_: i32,
    ) -> Self {
        Self {
            iterator_,
            table_start_,
            table_size_,
            case_value_base_,
        }
    }
    pub fn begin(&self) -> JumpTableTargetOffsetsIterator {
        JumpTableTargetOffsetsIterator::new(
            self.case_value_base_,
            self.table_start_,
            self.table_start_ + self.table_size_,
            self.iterator_,
        )
    }
    pub fn end(&self) -> JumpTableTargetOffsetsIterator {
        JumpTableTargetOffsetsIterator::new(
            self.case_value_base_ + self.table_size_,
            self.table_start_ + self.table_size_,
            self.table_start_ + self.table_size_,
            self.iterator_,
        )
    }
    pub fn size(&self) -> i32 {
        let mut ret = 0;
        for _entry in self.begin() {
            ret += 1;
        }
        ret
    }
}

pub struct JumpTableTargetOffsetsIterator<'a> {
    iterator_: &'a BytecodeArrayIterator,
    current_: Tagged<Smi>,
    index_: i32,
    table_offset_: i32,
    table_end_: i32,
}

impl<'a> JumpTableTargetOffsetsIterator<'a> {
    pub fn new(
        case_value: i32,
        table_offset: i32,
        table_end: i32,
        iterator: &'a BytecodeArrayIterator,
    ) -> Self {
        let mut new_iter = Self {
            iterator_: iterator,
            current_: Tagged::<Smi>::default(),
            index_: case_value,
            table_offset_: table_offset,
            table_end_: table_end,
        };
        new_iter.update_and_advance_to_valid();
        new_iter
    }

    fn update_and_advance_to_valid(&mut self) {
        while self.table_offset_ < self.table_end_
        /*&& !self.iterator_.is_constant_at_index_smi(self.table_offset_)*/
        {
            self.table_offset_ += 1;
            self.index_ += 1;
        }

        if self.table_offset_ < self.table_end_ {
            /*DCHECK!(self
            .iterator_
            .is_constant_at_index_smi(self.table_offset_));*/
            self.current_ = Tagged::<Smi>::default(); //self.iterator_.get_constant_at_index_as_smi(self.table_offset_);
        }
    }
}

impl<'a> Iterator for JumpTableTargetOffsetsIterator<'a> {
    type Item = JumpTableTargetOffset;

    fn next(&mut self) -> Option<Self::Item> {
        if self.table_offset_ < self.table_end_ {
            let result = JumpTableTargetOffset {
                case_value: self.index_,
                target_offset: 0, //self.iterator_.get_absolute_offset(self.current_.value()),
            };
            self.table_offset_ += 1;
            self.index_ += 1;
            self.update_and_advance_to_valid();
            Some(result)
        } else {
            None
        }
    }
}

// src/interpreter/bytecode-array-iterator.cc
use crate::interpreter::bytecode_decoder::BytecodeDecoder;
use crate::interpreter::bytecodes;
use crate::interpreter::bytecodes::OperandScale;
use crate::objects::feedback_vector::FeedbackSlot;
use crate::interpreter::interpreter_intrinsics::IntrinsicsHelper;
use crate::interpreter::bytecode_register::RegisterList;

use std::fmt;

pub struct BytecodeArray {
    length_: usize,
    first_bytecode_address_: usize,
    constant_pool_: ConstantPool,
}

impl BytecodeArray {
    pub fn new(length: usize, first_bytecode_address: usize, constant_pool_: ConstantPool) -> Self {
        BytecodeArray {
            length_: length,
            first_bytecode_address_: first_bytecode_address,
            constant_pool_: constant_pool_
        }
    }
    pub fn length(&self) -> usize {
        self.length_
    }
    pub fn get_first_bytecode_address(&self) -> usize {
        self.first_bytecode_address_
    }
    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constant_pool_
    }
}

#[derive(Debug)]
pub enum BytecodeArrayIteratorError {
    InvalidOffset,
    Other(String),
}

pub struct DisallowGarbageCollection {}

pub struct BytecodeArrayIterator {
    bytecode_array_: Rc<BytecodeArray>,
    start_: usize,
    end_: usize,
    cursor_: usize,
    operand_scale_: OperandScale,
    prefix_size_: i32,
    local_heap_: Option<LocalHeap>,
}

impl BytecodeArrayIterator {
    pub fn new(bytecode_array: Rc<BytecodeArray>, initial_offset: usize) -> Self {
        let start_ = bytecode_array.get_first_bytecode_address();
        let end_ = start_ + bytecode_array.length();
        let mut iterator = BytecodeArrayIterator {
            bytecode_array_: bytecode_array.clone(),
            start_: start_,
            end_: end_,
            cursor_: start_,
            operand_scale_: OperandScale::kSingle,
            prefix_size_: 0,
            local_heap_: LocalHeap::current(),
        };

        if let Some(local_heap) = &iterator.local_heap_ {
            //local_heap.add_gc_epilogue_callback(Self::update_pointers_callback, &iterator);
        }

        iterator.update_operand_scale();

        if initial_offset != 0 {
            iterator.advance_to(initial_offset).expect("Failed to advance");
        }
        iterator
    }

    pub fn new_no_gc(
        bytecode_array: Rc<BytecodeArray>,
        initial_offset: usize,
        _no_gc: &DisallowGarbageCollection,
    ) -> Self {
        let start_ = bytecode_array.get_first_bytecode_address();
        let end_ = start_ + bytecode_array.length();

        let mut iterator = BytecodeArrayIterator {
            bytecode_array_: bytecode_array.clone(),
            start_: start_,
            end_: end_,
            cursor_: start_,
            operand_scale_: OperandScale::kSingle,
            prefix_size_: 0,
            local_heap_: None,
        };

        iterator.update_operand_scale();
        if initial_offset != 0 {
            iterator.advance_to(initial_offset).expect("Failed to advance");
        }
        iterator
    }

    pub fn advance(&mut self) {
        self.cursor_ += self.current_bytecode_size_without_prefix() as usize;
        self.update_operand_scale();
    }

    pub fn advance_to(&mut self, offset: usize) -> Result<(), BytecodeArrayIteratorError> {
        if offset < self.current_offset() {
            return Err(BytecodeArrayIteratorError::InvalidOffset);
        }
        while self.current_offset() != offset && self.cursor_ < self.end_ {
            self.advance();
        }

        if self.current_offset() != offset {
            return Err(BytecodeArrayIteratorError::InvalidOffset);
        }

        Ok(())
    }

    pub fn set_offset(&mut self, offset: usize) -> Result<(), BytecodeArrayIteratorError> {
        if offset < 0 {
            return Err(BytecodeArrayIteratorError::InvalidOffset);
        }
        if offset < self.current_offset() {
            self.reset();
        }
        self.advance_to(offset)?;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.cursor_ = self.start_;
        self.update_operand_scale();
    }

    pub fn is_valid_offset(bytecode_array: Rc<BytecodeArray>, offset: i32) -> bool {
        let mut it = BytecodeArrayIterator::new(bytecode_array, 0);
        while !it.done() {
            if it.current_offset() == offset as usize {
                return true;
            }
            if it.current_offset() > offset as usize {
                break;
            }
            it.advance();
        }
        false
    }

    pub fn is_valid_osr_entry_offset(bytecode_array: Rc<BytecodeArray>, offset: i32) -> bool {
        let it = BytecodeArrayIterator::new(bytecode_array, offset as usize);
        it.current_bytecode_is_valid_osr_entry()
    }

    pub fn current_bytecode_is_valid_osr_entry(&self) -> bool {
        self.current_bytecode() == bytecodes::Bytecode::kJumpLoop
    }

    pub fn current_bytecode(&self) -> bytecodes::Bytecode {
        assert!(!self.done());
        let current_byte = unsafe { *(self.cursor_ as *const u8) };
        let current_bytecode = bytecodes::Bytecode::from_byte(current_byte);
        assert!(!bytecodes::Bytecodes::is_prefix_scaling_bytecode(
            current_bytecode
        ));
        current_bytecode
    }

    pub fn current_bytecode_size(&self) -> i32 {
        self.prefix_size_ + self.current_bytecode_size_without_prefix()
    }

    pub fn current_bytecode_size_without_prefix(&self) -> i32 {
        bytecodes::Bytecodes::size(self.current_bytecode(), self.current_operand_scale())
    }

    pub fn current_offset(&self) -> usize {
        (self.cursor_ as usize) - self.start_
    }

    pub fn current_operand_scale(&self) -> OperandScale {
        self.operand_scale_
    }

    fn update_operand_scale(&mut self) {
        if self.done() {
            return;
        }
        let current_byte = unsafe { *(self.cursor_ as *const u8) };
        let current_bytecode = bytecodes::Bytecode::from_byte(current_byte);

        if bytecodes::Bytecodes::is_prefix_scaling_bytecode(current_bytecode) {
            self.operand_scale_ =
                bytecodes::Bytecodes::prefix_bytecode_to_operand_scale(current_bytecode);
            self.cursor_ += 1;
            self.prefix_size_ = 1;
        } else {
            self.operand_scale_ = OperandScale::kSingle;
            self.prefix_size_ = 0;
        }
    }

    pub fn get_unsigned_operand(&self, operand_index: usize, operand_type: OperandType) -> u32 {
        assert!(operand_index < bytecodes::Bytecodes::number_of_operands(self.current_bytecode()) as usize);
        assert_eq!(operand_type, bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32));
        assert!(bytecodes::Bytecodes::is_unsigned_operand_type(operand_type));
        let operand_start = self.cursor_ + bytecodes::Bytecodes::get_operand_offset(self.current_bytecode(), operand_index as i32, self.current_operand_scale()) as usize;
        BytecodeDecoder::decode_unsigned_operand(operand_start, operand_type, self.current_operand_scale()) as u32
    }

    pub fn get_signed_operand(&self, operand_index: usize, operand_type: OperandType) -> i32 {
        assert!(operand_index < bytecodes::Bytecodes::number_of_operands(self.current_bytecode()) as usize);
        assert_eq!(operand_type, bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32));
        assert!(!bytecodes::Bytecodes::is_unsigned_operand_type(operand_type));
        let operand_start = self.cursor_ + bytecodes::Bytecodes::get_operand_offset(self.current_bytecode(), operand_index as i32, self.current_operand_scale()) as usize;
        BytecodeDecoder::decode_signed_operand(operand_start, operand_type, self.current_operand_scale())
    }

    pub fn get_flag8_operand(&self, operand_index: usize) -> u32 {
        assert_eq!(bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32), OperandType::kFlag8);
        self.get_unsigned_operand(operand_index, OperandType::kFlag8)
    }

    pub fn get_flag16_operand(&self, operand_index: usize) -> u32 {
        assert_eq!(bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32), OperandType::kFlag16);
        self.get_unsigned_operand(operand_index, OperandType::kFlag16)
    }

    pub fn get_unsigned_immediate_operand(&self, operand_index: usize) -> u32 {
        assert_eq!(bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32), OperandType::kUImm);
        self.get_unsigned_operand(operand_index, OperandType::kUImm)
    }

    pub fn get_immediate_operand(&self, operand_index: usize) -> i32 {
        assert_eq!(bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32), OperandType::kImm);
        self.get_signed_operand(operand_index, OperandType::kImm)
    }

    pub fn get_register_count_operand(&self, operand_index: usize) -> u32 {
        assert_eq!(bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32), OperandType::kRegCount);
        self.get_unsigned_operand(operand_index, OperandType::kRegCount)
    }

    pub fn get_index_operand(&self, operand_index: usize) -> u32 {
        let operand_type = bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32);
        assert_eq!(operand_type, OperandType::kIdx);
        self.get_unsigned_operand(operand_index, operand_type)
    }

    pub fn get_slot_operand(&self, operand_index: usize) -> FeedbackSlot {
        let index = self.get_index_operand(operand_index);
        FeedbackSlot::from_u32(index)
    }

    pub fn get_parameter(&self, parameter_index: usize) -> Register {
        assert!(parameter_index >= 0);
        Register::from_parameter_index(parameter_index as i32 + 1)
    }

    pub fn get_register_operand(&self, operand_index: usize) -> Register {
        let operand_type = bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32);
        let operand_start = self.cursor_ + bytecodes::Bytecodes::get_operand_offset(self.current_bytecode(), operand_index as i32, self.current_operand_scale()) as usize;
        BytecodeDecoder::decode_register_operand(operand_start, operand_type, self.current_operand_scale())
    }

    pub fn get_star_target_register(&self) -> Register {
        let bytecode = self.current_bytecode();
        assert!(bytecodes::Bytecodes::is_any_star(bytecode));
        if bytecodes::Bytecodes::is_short_star(bytecode) {
            Register::from_short_star(bytecode)
        } else {
            assert_eq!(bytecode, bytecodes::Bytecode::kStar);
            assert_eq!(bytecodes::Bytecodes::number_of_operands(bytecode), 1);
            assert_eq!(bytecodes::Bytecodes::get_operand_types(bytecode)[0], OperandType::kRegOut);
            self.get_register_operand(0)
        }
    }

    pub fn get_register_pair_operand(&self, operand_index: usize) -> (Register, Register) {
        let first = self.get_register_operand(operand_index);
        let second = Register::new(first.index() + 1);
        (first, second)
    }

    pub fn get_register_list_operand(&self, operand_index: usize) -> RegisterList {
        let first = self.get_register_operand(operand_index);
        let count = self.get_register_count_operand(operand_index + 1);
        RegisterList::new(first.index(), count)
    }

    pub fn get_register_operand_range(&self, operand_index: usize) -> i32 {
        assert!(operand_index <= bytecodes::Bytecodes::number_of_operands(self.current_bytecode()) as usize);
        let operand_types = bytecodes::Bytecodes::get_operand_types(self.current_bytecode());
        let operand_type = operand_types[operand_index];
        assert!(bytecodes::Bytecodes::is_register_operand_type(operand_type));
        if operand_type == OperandType::kRegList || operand_type == OperandType::kRegOutList {
            self.get_register_count_operand(operand_index + 1) as i32
        } else {
            bytecodes::Bytecodes::get_number_of_registers_represented_by(operand_type)
        }
    }

    pub fn get_runtime_id_operand(&self, operand_index: usize) -> Runtime::FunctionId {
        let operand_type = bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32);
        assert_eq!(operand_type, OperandType::kRuntimeId);
        let raw_id = self.get_unsigned_operand(operand_index, operand_type);
        raw_id as Runtime::FunctionId
    }

    pub fn get_native_context_index_operand(&self, operand_index: usize) -> u32 {
        let operand_type = bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32);
        assert_eq!(operand_type, OperandType::kNativeContextIndex);
        self.get_unsigned_operand(operand_index, operand_type)
    }

    pub fn get_intrinsic_id_operand(&self, operand_index: usize) -> Runtime::FunctionId {
        let operand_type = bytecodes::Bytecodes::get_operand_type(self.current_bytecode(), operand_index as i32);
        assert_eq!(operand_type, OperandType::kIntrinsicId);
        let raw_id = self.get_unsigned_operand(operand_index, operand_type);
        IntrinsicsHelper::to_runtime_id(raw_id as IntrinsicsHelper::IntrinsicId)
    }

    pub fn get_constant_at_index(&self, index: usize) -> &Object {
        self.bytecode_array_.constant_pool().get(index)
    }
    
    pub fn is_constant_at_index_smi(&self, index: usize) -> bool {
       self.bytecode_array_.constant_pool().is_smi(index)
    }

    pub fn get_constant_at_index_as_smi(&self, index: usize) -> Tagged<Smi> {
       self.bytecode_array_.constant_pool().get_smi(index)
    }

    pub fn get_constant_for_index_operand(&self, operand_index: usize) -> &Object {
        self.get_constant_at_index(self.get_index_operand(operand_index) as usize)
    }

    pub fn get_relative_jump_target_offset(&self) -> i32 {
        let bytecode = self.current_bytecode();
        if bytecodes::Bytecodes::is_jump_immediate(bytecode) {
            let relative_offset = self.get_unsigned_immediate_operand(0) as i32;
            if bytecode == bytecodes::Bytecode::kJumpLoop {
                -relative_offset
            } else {
                relative_offset
            }
        } else if bytecodes::Bytecodes::is_jump_constant(bytecode) {
            let smi = self.get_constant_at_index_as_smi(self.get_index_operand(0) as usize);
            smi.value()
        } else {
            panic!("not a jump bytecode")
        }
    }

    pub fn get_jump_target_offset(&self) -> i32 {
        self.get_absolute_offset(self.get_relative_jump_target_offset())
    }

    pub fn get_jump_table_target_offsets(&self) -> JumpTableTargetOffsets {
        let table_start: u32;
        let table_size: u32;
        let case_value_base: i32;

        if self.current_bytecode() == bytecodes::Bytecode::kSwitchOnGeneratorState {
            table_start = self.get_index_operand(1);
            table_size = self.get_unsigned_immediate_operand(2);
            case_value_base = 0;
        } else {
            assert_eq!(
                self.current_bytecode(),
                bytecodes::Bytecode::kSwitchOnSmiNoFeedback
            );
            table_start = self.get_index_operand(0);
            table_size = self.get_unsigned_immediate_operand(1);
            case_value_base = self.get_immediate_operand(2);
        }

        JumpTableTargetOffsets::new(
            self,
            table_start as i32,
            table_size as i32,
            case_value_base,
        )
    }

    pub fn get_absolute_offset(&self, relative_offset: i32) -> i32 {
        self.current_offset() as i32 + relative_offset + self.prefix_size_
    }

    pub fn done(&self) -> bool {
        self.cursor_ >= self.end_
    }
}

impl PartialEq for BytecodeArrayIterator {
    fn eq(&self, other: &Self) -> bool {
        self.cursor_ == other.cursor_
    }
}

impl fmt::Display for BytecodeArrayIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        BytecodeDecoder::decode_to_fmt(f, self.cursor_ - self.prefix_size_ as usize)
    }
}

pub enum OperandType {
    kFlag8,
    kFlag16,
    kUImm,
    kImm,
    kRegCount,
    kIdx,
    kRuntimeId,
    kNativeContextIndex,
    kIntrinsicId,
    kRegOut,
    kReg,
    kRegList,
    kRegOutList
}

pub struct Object {}

pub struct ConstantPool {}

impl ConstantPool {
    pub fn get(&self, _index: usize) -> &Object {
        &Object{}
    }

    pub fn is_smi(&self, _index: usize) -> bool {
        true
    }

    pub fn get_smi(&self, _index: usize) -> Tagged<Smi> {
        Tagged::<Smi>::default()
    }
}

pub struct LocalHeap {}

impl LocalHeap {
    pub fn current() -> Option<LocalHeap> {
        Some(LocalHeap{})
    }

}
