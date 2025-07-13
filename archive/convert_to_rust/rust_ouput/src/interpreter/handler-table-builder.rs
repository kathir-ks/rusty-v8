// Converted from V8 C++ source files:
// Header: handler-table-builder.h
// Implementation: handler-table-builder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/interpreter/handler-table-builder.h
use std::fmt;

//use crate::codegen::handler_table::HandlerTable;
use crate::interpreter::bytecode_register::Register;
//use crate::objects::fixed_array::FixedArray;
//use crate::zone::zone_containers::ZoneVector;

#[derive(Debug, Clone, Copy)]
pub enum CatchPrediction {
    UNCAUGHT,
}

#[derive(Debug)]
pub struct HandlerTable {
    data: Vec<u8>, // Simulate the byte array
}

impl HandlerTable {
    pub fn LengthForRange(handler_table_size: i32) -> i32 {
        handler_table_size * 16  // Placeholder calculation
    }

    pub fn new(size: usize) -> Self {
        HandlerTable { data: vec![0; size] }
    }

    pub fn SetRangeStart(&mut self, index: usize, value: i32) {
        let offset = index * 16;
        self.set_i32(offset, value);
    }

    pub fn SetRangeEnd(&mut self, index: usize, value: i32) {
        let offset = index * 16 + 4;
        self.set_i32(offset, value);
    }

    pub fn SetRangeHandler(&mut self, index: usize, value: i32, prediction: CatchPrediction) {
        let offset = index * 16 + 8;
        self.set_i32(offset, value);
    }

    pub fn SetRangeData(&mut self, index: usize, value: i32) {
        let offset = index * 16 + 12;
        self.set_i32(offset, value);
    }

    fn set_i32(&mut self, offset: usize, value: i32) {
        let bytes = value.to_le_bytes();
        self.data[offset..offset + 4].copy_from_slice(&bytes);
    }
}

pub struct TrustedByteArray {
    data: Vec<u8>,
}

impl TrustedByteArray {
    pub fn new(size: usize) -> Self {
        TrustedByteArray { data: vec![0; size] }
    }
}

pub struct Factory {}

impl Factory {
    pub fn NewTrustedByteArray(&self, size: i32) -> DirectHandle<TrustedByteArray> {
        DirectHandle::new(TrustedByteArray::new(size as usize))
    }
}

pub struct Isolate {
    factory: Factory,
}

impl Isolate {
    pub fn factory(&mut self) -> &mut Factory {
        &mut self.factory
    }
}

pub struct LocalIsolate {
    factory: Factory,
}

impl LocalIsolate {
    pub fn factory(&mut self) -> &mut Factory {
        &mut self.factory
    }
}

#[derive(Debug)]
pub struct DirectHandle<T> {
    value: T,
}

impl<T> DirectHandle<T> {
    pub fn new(value: T) -> Self {
        DirectHandle { value }
    }
}

pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
}

#[derive(Debug)]
pub enum HandlerTableBuilderError {
    InvalidOffset,
    Other(String),
}

impl fmt::Display for HandlerTableBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandlerTableBuilderError::InvalidOffset => {
                write!(f, "Offset is invalid")
            }
            HandlerTableBuilderError::Other(msg) => {
                write!(f, "Other error: {}", msg)
            }
        }
    }
}

// A helper class for constructing exception handler tables for the interpreter.
#[derive(Debug)]
pub struct HandlerTableBuilder {
    entries_: Vec<Entry>,
    zone: Zone,
}

impl HandlerTableBuilder {
    pub fn new(zone: Zone) -> Self {
        HandlerTableBuilder {
            entries_: Vec::new(),
            zone,
        }
    }

    // Builds the actual handler table by copying the current values into a heap
    // object. Any further mutations to the builder won't be reflected.
    pub fn to_handler_table(
        &mut self,
        isolate: &mut Isolate,
    ) -> Result<DirectHandle<TrustedByteArray>, HandlerTableBuilderError> {
        let handler_table_size = self.entries_.len();
        let table_byte_array = isolate
            .factory()
            .NewTrustedByteArray(HandlerTable::LengthForRange(handler_table_size as i32));
        let mut table = HandlerTable {
            data: vec![0; HandlerTable::LengthForRange(handler_table_size as i32) as usize],
        };

        for i in 0..handler_table_size {
            let entry = &self.entries_[i];
            let pred = entry.catch_prediction_;

            table.SetRangeStart(i, entry.offset_start as i32);
            table.SetRangeEnd(i, entry.offset_end as i32);
            table.SetRangeHandler(i, entry.offset_target as i32, pred);
            table.SetRangeData(i, entry.context.index());
            // table.SetRangeData(i, 0);
        }
        Ok(table_byte_array)
    }

    pub fn to_handler_table_local(
        &mut self,
        isolate: &mut LocalIsolate,
    ) -> Result<DirectHandle<TrustedByteArray>, HandlerTableBuilderError> {
        let handler_table_size = self.entries_.len();
        let table_byte_array = isolate
            .factory()
            .NewTrustedByteArray(HandlerTable::LengthForRange(handler_table_size as i32));
        let mut table = HandlerTable {
            data: vec![0; HandlerTable::LengthForRange(handler_table_size as i32) as usize],
        };

        for i in 0..handler_table_size {
            let entry = &self.entries_[i];
            let pred = entry.catch_prediction_;

            table.SetRangeStart(i, entry.offset_start as i32);
            table.SetRangeEnd(i, entry.offset_end as i32);
            table.SetRangeHandler(i, entry.offset_target as i32, pred);
            table.SetRangeData(i, entry.context.index());
        }
        Ok(table_byte_array)
    }

    // Creates a new handler table entry and returns a {hander_id} identifying the
    // entry, so that it can be referenced by below setter functions.
    pub fn new_handler_entry(&mut self) -> i32 {
        let handler_id = self.entries_.len() as i32;
        let entry = Entry {
            offset_start: 0,
            offset_end: 0,
            offset_target: 0,
            context: Register::invalid_value(),
            catch_prediction_: CatchPrediction::UNCAUGHT,
        };
        self.entries_.push(entry);
        handler_id
    }

    // Setter functions that modify certain values within the handler table entry
    // being referenced by the given {handler_id}. All values will be encoded by
    // the resulting {HandlerTable} class when copied into the heap.
    pub fn set_try_region_start(
        &mut self,
        handler_id: i32,
        offset: usize,
    ) -> Result<(), HandlerTableBuilderError> {
        if offset > i32::MAX as usize {
            return Err(HandlerTableBuilderError::InvalidOffset);
        }
        self.entries_[handler_id as usize].offset_start = offset;
        Ok(())
    }

    pub fn set_try_region_end(
        &mut self,
        handler_id: i32,
        offset: usize,
    ) -> Result<(), HandlerTableBuilderError> {
        if offset > i32::MAX as usize {
            return Err(HandlerTableBuilderError::InvalidOffset);
        }
        self.entries_[handler_id as usize].offset_end = offset;
        Ok(())
    }

    pub fn set_handler_target(
        &mut self,
        handler_id: i32,
        offset: usize,
    ) -> Result<(), HandlerTableBuilderError> {
        if offset > i32::MAX as usize {
            return Err(HandlerTableBuilderError::InvalidOffset);
        }
        self.entries_[handler_id as usize].offset_target = offset;
        Ok(())
    }

    pub fn set_prediction(
        &mut self,
        handler_id: i32,
        prediction: CatchPrediction,
    ) -> Result<(), HandlerTableBuilderError> {
        self.entries_[handler_id as usize].catch_prediction_ = prediction;
        Ok(())
    }

    pub fn set_context_register(
        &mut self,
        handler_id: i32,
        reg: Register,
    ) -> Result<(), HandlerTableBuilderError> {
        self.entries_[handler_id as usize].context = reg;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Entry {
    offset_start: usize,   // Bytecode offset starting try-region.
    offset_end: usize,     // Bytecode offset ending try-region.
    offset_target: usize,  // Bytecode offset of handler target.
    context: Register,      // Register holding context for handler.
                           // Optimistic prediction for handler.
    catch_prediction_: CatchPrediction,
}
