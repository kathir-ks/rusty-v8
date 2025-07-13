// Converted from V8 C++ source files:
// Header: handler-table.h
// Implementation: handler-table.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod handler_table {
use std::cmp::Ordering;
use std::io;
use std::mem::size_of;
use std::ops::Deref;
use std::rc::Rc;

use crate::codegen::assembler::Assembler;
use crate::codegen::code_desc::CodeDesc;
use crate::codegen::pending_optimization_table::Isolate;
use crate::interpreter::bytecode_array_writer::TrustedByteArray;
use crate::objects::code::Code;
use crate::objects::objects::Heap;
use crate::strings::uri::V8;
use crate::wasm;
use crate::{BytecodeArray, InstructionStream};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CatchPrediction {
    UNCAUGHT,
    CAUGHT,
    PROMISE,
    ASYNC_AWAIT,
    UNCAUGHT_ASYNC_AWAIT,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EncodingMode {
    kRangeBasedEncoding,
    kReturnAddressBasedEncoding,
}

pub struct HandlerTable {
    number_of_entries_: i32,
    mode_: EncodingMode,
    raw_encoded_data_: Address,
    no_gc_: DisallowGarbageCollection,
}

impl HandlerTable {
    const kRangeStartIndex: i32 = 0;
    const kRangeEndIndex: i32 = 1;
    const kRangeHandlerIndex: i32 = 2;
    const kRangeDataIndex: i32 = 3;
    const kRangeEntrySize: i32 = 4;

    const kReturnOffsetIndex: i32 = 0;
    const kReturnHandlerIndex: i32 = 1;
    const kReturnEntrySize: i32 = 2;
    pub const kNoHandlerFound: i32 = -1;
    pub const kLazyDeopt: i32 = i32::MAX; // HandlerOffsetField::kMax;

    pub fn new_from_code(code: Tagged<Code>) -> Self {
        HandlerTable::new(
            code.handler_table_address(),
            code.handler_table_size(),
            EncodingMode::kReturnAddressBasedEncoding,
        )
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn new_from_wasm_code(code: &wasm::WasmCode) -> Self {
        HandlerTable::new(
            code.handler_table(),
            code.handler_table_size(),
            EncodingMode::kReturnAddressBasedEncoding,
        )
    }

    pub fn new_from_bytecode_array(bytecode_array: Tagged<BytecodeArray>) -> Self {
        HandlerTable::new(
            bytecode_array.handler_table_address(),
            bytecode_array.handler_table_size(),
            EncodingMode::kRangeBasedEncoding, // Assuming range-based for bytecode arrays
        )
    }

    pub fn new_from_trusted_byte_array(byte_array: Tagged<TrustedByteArray>) -> Self {
        HandlerTable::new(
            byte_array.begin(),
            byte_array.length() as i32,
            EncodingMode::kRangeBasedEncoding,
        )
    }

    pub fn new(handler_table: Address, handler_table_size: i32, encoding_mode: EncodingMode) -> Self {
        let entry_size = HandlerTable::entry_size_from_mode(encoding_mode);
        let number_of_entries = handler_table_size / entry_size / size_of::<i32>() as i32;

        HandlerTable {
            number_of_entries_: number_of_entries,
            mode_: encoding_mode,
            raw_encoded_data_: handler_table,
            no_gc_: DisallowGarbageCollection::new(),
        }
    }

    pub fn entry_size_from_mode(mode: EncodingMode) -> i32 {
        match mode {
            EncodingMode::kReturnAddressBasedEncoding => HandlerTable::kReturnEntrySize,
            EncodingMode::kRangeBasedEncoding => HandlerTable::kRangeEntrySize,
        }
    }

    pub fn get_range_start(&self, index: i32) -> i32 {
        assert_eq!(self.mode_, EncodingMode::kRangeBasedEncoding);
        assert!(index < self.number_of_entries());
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeStartIndex;
        unsafe { *(self.raw_encoded_data_.0 as *const i32).add(offset as usize) }
    }

    pub fn get_range_end(&self, index: i32) -> i32 {
        assert_eq!(self.mode_, EncodingMode::kRangeBasedEncoding);
        assert!(index < self.number_of_entries());
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeEndIndex;
        unsafe { *(self.raw_encoded_data_.0 as *const i32).add(offset as usize) }
    }

    fn get_range_handler_bitfield(&self, index: i32) -> i32 {
        assert_eq!(self.mode_, EncodingMode::kRangeBasedEncoding);
        assert!(index < self.number_of_entries());
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeHandlerIndex;
        unsafe {
            *(&*(self.raw_encoded_data_.0 as *const i32).add(offset as usize) as *const i32)
        }
    }

    pub fn get_range_handler(&self, index: i32) -> i32 {
        HandlerOffsetField::decode(self.get_range_handler_bitfield(index))
    }

    pub fn get_range_data(&self, index: i32) -> i32 {
        assert_eq!(self.mode_, EncodingMode::kRangeBasedEncoding);
        assert!(index < self.number_of_entries());
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeDataIndex;
        unsafe { *(self.raw_encoded_data_.0 as *const i32).add(offset as usize) }
    }
    pub fn get_range_prediction(&self, index: i32) -> CatchPrediction {
        HandlerPredictionField::decode(self.get_range_handler_bitfield(index))
    }

    pub fn handler_was_used(&self, index: i32) -> bool {
        HandlerWasUsedField::decode(self.get_range_handler_bitfield(index))
    }

    pub fn mark_handler_used(&self, index: i32) {
        assert_eq!(self.mode_, EncodingMode::kRangeBasedEncoding);
        assert!(index < self.number_of_entries());
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeHandlerIndex;
        let mem_ptr = (self.raw_encoded_data_.0 as *mut i32).add(offset as usize);
        let current_value = unsafe { *mem_ptr };
        let new_value = HandlerWasUsedField::update(current_value, true);
        unsafe { *mem_ptr = new_value };
    }

    pub fn get_return_offset(&self, index: i32) -> i32 {
        assert_eq!(self.mode_, EncodingMode::kReturnAddressBasedEncoding);
        assert!(index < self.number_of_entries());
        let offset = index * HandlerTable::kReturnEntrySize + HandlerTable::kReturnOffsetIndex;
        unsafe { *(self.raw_encoded_data_.0 as *const i32).add(offset as usize) }
    }

    pub fn get_return_handler(&self, index: i32) -> i32 {
        assert_eq!(self.mode_, EncodingMode::kReturnAddressBasedEncoding);
        assert!(index < self.number_of_entries());
        let offset = index * HandlerTable::kReturnEntrySize + HandlerTable::kReturnHandlerIndex;
        HandlerOffsetField::decode(unsafe { *(self.raw_encoded_data_.0 as *const i32).add(offset as usize) })
    }

    pub fn set_range_start(&self, index: i32, value: i32) {
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeStartIndex;
        unsafe { *(self.raw_encoded_data_.0 as *mut i32).add(offset as usize) = value };
    }

    pub fn set_range_end(&self, index: i32, value: i32) {
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeEndIndex;
        unsafe { *(self.raw_encoded_data_.0 as *mut i32).add(offset as usize) = value };
    }

    pub fn set_range_handler(&self, index: i32, handler_offset: i32, prediction: CatchPrediction) {
        let value = HandlerOffsetField::encode(handler_offset)
            | HandlerWasUsedField::encode(false)
            | HandlerPredictionField::encode(prediction);
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeHandlerIndex;
        unsafe { *(self.raw_encoded_data_.0 as *mut i32).add(offset as usize) = value };
    }

    pub fn set_range_data(&self, index: i32, value: i32) {
        let offset = index * HandlerTable::kRangeEntrySize + HandlerTable::kRangeDataIndex;
        unsafe { *(self.raw_encoded_data_.0 as *mut i32).add(offset as usize) = value };
    }

    pub fn length_for_range(entries: i32) -> i32 {
        entries * HandlerTable::kRangeEntrySize * size_of::<i32>() as i32
    }

    pub fn emit_return_table_start(masm: &mut Assembler) -> i32 {
        masm.data_align(InstructionStream::kMetadataAlignment);
        masm.record_comment(";;; Exception handler table.");
        let table_start = masm.pc_offset();
        table_start
    }

    pub fn emit_return_entry(masm: &mut Assembler, offset: i32, handler: i32) {
        masm.dd(offset);
        masm.dd(HandlerOffsetField::encode(handler));
    }

    pub fn number_of_range_entries(&self) -> i32 {
        assert_eq!(self.mode_, EncodingMode::kRangeBasedEncoding);
        self.number_of_entries_
    }

    pub fn number_of_return_entries(&self) -> i32 {
        assert_eq!(self.mode_, EncodingMode::kReturnAddressBasedEncoding);
        self.number_of_entries_
    }

    pub fn lookup_handler_index_for_range(&self, pc_offset: i32) -> i32 {
        let mut innermost_handler = HandlerTable::kNoHandlerFound;
        #[cfg(debug_assertions)]
        {
            let mut innermost_start = i32::MIN;
            let mut innermost_end = i32::MAX;
        }

        for i in 0..self.number_of_range_entries() {
            let start_offset = self.get_range_start(i);
            let end_offset = self.get_range_end(i);

            if end_offset <= pc_offset {
                continue;
            }
            if start_offset > pc_offset {
                break;
            }

            #[cfg(debug_assertions)]
            {
                assert!(start_offset >= innermost_start);
                assert!(end_offset < innermost_end);
                innermost_start = start_offset;
                innermost_end = end_offset;
            }
            innermost_handler = i;
        }

        innermost_handler
    }

    pub fn lookup_return(&self, pc_offset: i32) -> i32 {
        struct Iterator<'a> {
            table: &'a HandlerTable,
            index: i32,
        }

        impl<'a> Iterator<'a> {
            fn new(table: &'a HandlerTable, index: i32) -> Self {
                Iterator { table, index }
            }

            fn value(&self) -> i32 {
                self.table.get_return_offset(self.index)
            }
        }

        impl<'a> PartialEq for Iterator<'a> {
            fn eq(&self, other: &Self) -> bool {
                self.index == other.index
            }
        }

        impl<'a> PartialOrd for Iterator<'a> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.index.cmp(&other.index))
            }
        }

        impl<'a> Eq for Iterator<'a> {}

        impl<'a> Ord for Iterator<'a> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.index.cmp(&other.index)
            }
        }

        let begin = Iterator::new(self, 0);
        let end = Iterator::new(self, self.number_of_return_entries());
        let mut result_index = -1;

        for i in 0..self.number_of_return_entries() {
            let offset = self.get_return_offset(i);
            if offset == pc_offset {
                result_index = i;
                break;
            }
        }

        if result_index != -1 {
            self.get_return_handler(result_index)
        } else {
            -1
        }
    }
    pub fn handler_table_range_print(&self, os: &mut dyn io::Write) -> io::Result<()> {
        writeln!(os, "   from   to       hdlr (prediction,   data)")?;
        for i in 0..self.number_of_range_entries() {
            let pc_start = self.get_range_start(i);
            let pc_end = self.get_range_end(i);
            let handler_offset = self.get_range_handler(i);
            let handler_data = self.get_range_data(i);
            let prediction = self.get_range_prediction(i);
            writeln!(
                os,
                "  ({:4},{:4})  ->  {:4} (prediction={:?}, data={})",
                pc_start, pc_end, handler_offset, prediction, handler_data
            )?;
        }
        Ok(())
    }

    pub fn handler_table_return_print(&self, os: &mut dyn io::Write) -> io::Result<()> {
        writeln!(os, "  offset   handler")?;
        for i in 0..self.number_of_return_entries() {
            let pc_offset = self.get_return_offset(i);
            let handler_offset = self.get_return_handler(i);
            writeln!(os, "    {:4X}  ->  {:4}", pc_offset, handler_offset)?;
        }
        Ok(())
    }

    fn number_of_entries(&self) -> i32 {
        self.number_of_entries_
    }
}

pub struct HandlerPredictionField {}
impl HandlerPredictionField {
    const OFFSET: usize = 0;
    const SIZE: usize = 3;

    fn decode(value: i32) -> CatchPrediction {
        match (value >> Self::OFFSET) & ((1 << Self::SIZE) - 1) {
            0 => CatchPrediction::UNCAUGHT,
            1 => CatchPrediction::CAUGHT,
            2 => CatchPrediction::PROMISE,
            3 => CatchPrediction::ASYNC_AWAIT,
            4 => CatchPrediction::UNCAUGHT_ASYNC_AWAIT,
            _ => CatchPrediction::UNCAUGHT, // Provide a default value
        }
    }
    fn encode(value: CatchPrediction) -> i32 {
        let enum_value = match value {
            CatchPrediction::UNCAUGHT => 0,
            CatchPrediction::CAUGHT => 1,
            CatchPrediction::PROMISE => 2,
            CatchPrediction::ASYNC_AWAIT => 3,
            CatchPrediction::UNCAUGHT_ASYNC_AWAIT => 4,
        };
        (enum_value << Self::OFFSET) as i32
    }
}

pub struct HandlerWasUsedField {}

impl HandlerWasUsedField {
    const OFFSET: usize = HandlerPredictionField::OFFSET + HandlerPredictionField::SIZE;
    const SIZE: usize = 1;
    fn decode(value: i32) -> bool {
        ((value >> Self::OFFSET) & ((1 << Self::SIZE) - 1)) != 0
    }

    fn encode(value: bool) -> i32 {
        ((if value { 1 } else { 0 }) << Self::OFFSET) as i32
    }

    fn update(value: i32, new_value: bool) -> i32 {
        let mask: i32 = !(((1 << Self::SIZE) - 1) << Self::OFFSET);
        (value & mask) | (Self::encode(new_value))
    }
}

pub struct HandlerOffsetField {}
impl HandlerOffsetField {
    const OFFSET: usize = HandlerWasUsedField::OFFSET + HandlerWasUsedField::SIZE;
    const SIZE: usize = 28;
    const kMax: i32 = (1 << Self::SIZE) - 1;
    fn decode(value: i32) -> i32 {
        ((value >> Self::OFFSET) & ((1 << Self::SIZE) - 1)) as i32
    }

    fn encode(value: i32) -> i32 {
        (value << Self::OFFSET) as i32
    }
}

pub struct DisallowGarbageCollection {}
impl DisallowGarbageCollection {
    fn new() -> Self {
        DisallowGarbageCollection {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Address(pub usize);

impl Address {
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Address(ptr as usize)
    }
}

impl From<usize> for Address {
    fn from(address: usize) -> Self {
        Address(address)
    }
}

pub struct Tagged<T> {
    address: Address,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn new(address: Address) -> Self {
        Tagged {
            address,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn address(&self) -> Address {
        self.address
    }
}

impl Tagged<Code> {
    pub fn handler_table_address(&self) -> Address {
        Address(0)
    }
    pub fn handler_table_size(&self) -> i32 {
        0
    }
}

impl Tagged<BytecodeArray> {
    pub fn handler_table_address(&self) -> Address {
        Address(0)
    }
    pub fn handler_table_size(&self) -> i32 {
        0
    }
}

impl Tagged<TrustedByteArray> {
    pub fn begin(&self) -> Address {
        Address(0)
    }
    pub fn length(&self) -> usize {
        0
    }
}
}
