// Converted from V8 C++ source files:
// Header: bytecode-array-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::mem;
use std::ptr::null_mut;
use crate::v8::internal::Address;
use crate::v8::internal::Tagged;

// From /home/kathirks_gc/v8_go/archive/codebase/src/heap/factory.h
pub struct Isolate {
    heap_: Heap,
}

impl Isolate {
    pub fn heap(&mut self) -> &mut Heap {
        &mut self.heap_
    }
}

pub struct Heap {
    empty_byte_array: Tagged<ByteArray>,
    empty_trusted_byte_array: Tagged<TrustedByteArray>,
}

impl Heap {
    pub fn empty_trusted_byte_array(&self) -> Tagged<TrustedByteArray> {
        self.empty_trusted_byte_array
    }
}

fn GetIsolateFromWritableObject(object: BytecodeArray) -> *mut Isolate {
    unsafe {
        let isolate = object.isolate;
        return isolate;
    }
}

fn IsTrustedByteArray(maybe_table: Tagged<Object>) -> bool {
    maybe_table.is_trusted_byte_array()
}

fn IsByteArray(maybe_table: Tagged<Object>) -> bool {
    maybe_table.is_byte_array()
}

fn IsTrustedFixedArray(maybe_constant_pool: Tagged<Object>) -> bool {
    maybe_constant_pool.is_trusted_fixed_array()
}

trait TaggedObject {
    fn is_trusted_byte_array(&self) -> bool;
    fn is_byte_array(&self) -> bool;
    fn is_trusted_fixed_array(&self) -> bool;
}

impl TaggedObject for Tagged<Object> {
    fn is_trusted_byte_array(&self) -> bool {
        true
    }
    fn is_byte_array(&self) -> bool {
        true
    }
    fn is_trusted_fixed_array(&self) -> bool {
        true
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Object {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ByteArray {}

impl ByteArray {
    fn AllocatedSize(&self) -> i32 {
        1024
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TrustedByteArray {}

impl TrustedByteArray {
    fn AllocatedSize(&self) -> i32 {
        1024
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TrustedFixedArray {}

impl TrustedFixedArray {
    fn Size(&self) -> i32 {
        1024
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BytecodeWrapper {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Struct {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Smi {}

impl Smi {
    pub fn zero() -> Self {
        Smi {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExposedTrustedObject {}

const kSystemPointerSize: usize = 8;
const kSystemPointerSizeLog2: i32 = 3;
const kHeapObjectTag: usize = 1;
const kCharSize: usize = 1;

#[repr(C)]
pub struct BytecodeArray {
    length_field: i32,
    frame_size_field: i32,
    parameter_size_field: u16,
    max_arguments_field: u16,
    incoming_new_target_or_generator_register_offset_field: i32,
    wrapper_field: Tagged<BytecodeWrapper>,
    handler_table_field: Tagged<TrustedByteArray>,
    constant_pool_field: Tagged<TrustedFixedArray>,
    source_position_table_field: Tagged<TrustedByteArray>,
    isolate: *mut Isolate,
}

impl BytecodeArray {
    fn address(&self) -> usize {
        self as *const Self as usize
    }

    fn ptr(&self) -> usize {
        self as *const Self as usize
    }
}

impl BytecodeArray {
    pub fn length(&self) -> i32 {
        self.length_field
    }

    pub fn set_length(&mut self, length: i32) {
        self.length_field = length;
    }

    pub fn handler_table(&self) -> Tagged<TrustedByteArray> {
        self.handler_table_field
    }

    pub fn set_handler_table(&mut self, handler_table: Tagged<TrustedByteArray>) {
        self.handler_table_field = handler_table;
    }

    pub fn constant_pool(&self) -> Tagged<TrustedFixedArray> {
        self.constant_pool_field
    }

    pub fn set_constant_pool(&mut self, constant_pool: Tagged<TrustedFixedArray>) {
        self.constant_pool_field = constant_pool;
    }

    pub fn wrapper(&self) -> Tagged<BytecodeWrapper> {
        self.wrapper_field
    }

    pub fn set_wrapper(&mut self, wrapper: Tagged<BytecodeWrapper>) {
        self.wrapper_field = wrapper;
    }

    pub fn source_position_table(&self) -> Tagged<TrustedByteArray> {
        self.source_position_table_field
    }

    pub fn set_source_position_table(&mut self, source_position_table: Tagged<TrustedByteArray>) {
        self.source_position_table_field = source_position_table;
    }
}

const kLengthOffset: usize = 0;
const kHandlerTableOffset: usize = 4;
const kConstantPoolOffset: usize = 8;
const kWrapperOffset: usize = 12;
const kSourcePositionTableOffset: usize = 16;
const kFrameSizeOffset: usize = 20;
const kParameterSizeOffset: usize = 24;
const kMaxArgumentsOffset: usize = 26;
const kIncomingNewTargetOrGeneratorRegisterOffset: usize = 28;
const kHeaderSize: usize = 32;

impl BytecodeArray {
    pub fn get(&self, index: i32) -> u8 {
        assert!(index >= 0 && index < self.length());
        let offset = kHeaderSize + index as usize * kCharSize;
        unsafe {
            let ptr = (self as *const Self as *const u8).add(offset);
            *ptr
        }
    }

    pub fn set(&mut self, index: i32, value: u8) {
        assert!(index >= 0 && index < self.length());
        let offset = kHeaderSize + index as usize * kCharSize;
        unsafe {
            let ptr = (self as *mut Self as *mut u8).add(offset);
            *ptr = value;
        }
    }

    pub fn set_frame_size(&mut self, frame_size: i32) {
        assert!(frame_size >= 0);
        assert!(frame_size % kSystemPointerSize as i32 == 0);
        self.frame_size_field = frame_size;
    }

    pub fn frame_size(&self) -> i32 {
        self.frame_size_field
    }

    pub fn register_count(&self) -> i32 {
        (self.frame_size() / kSystemPointerSize as i32) as i32
    }

    pub fn parameter_count(&self) -> u16 {
        self.parameter_size_field
    }

    pub fn parameter_count_without_receiver(&self) -> u16 {
        self.parameter_count() - 1
    }

    pub fn set_parameter_count(&mut self, number_of_parameters: u16) {
        self.parameter_size_field = number_of_parameters;
    }

    pub fn max_arguments(&self) -> u16 {
        self.max_arguments_field
    }

    pub fn set_max_arguments(&mut self, max_arguments: u16) {
        self.max_arguments_field = max_arguments;
    }

    pub fn max_frame_size(&self) -> i32 {
        self.frame_size() + (self.max_arguments() as i32 << kSystemPointerSizeLog2)
    }

    pub fn incoming_new_target_or_generator_register(&self) -> interpreter::Register {
        let register_operand = self.incoming_new_target_or_generator_register_offset_field;
        if register_operand == 0 {
            interpreter::Register::invalid_value()
        } else {
            interpreter::Register::FromOperand(register_operand)
        }
    }

    pub fn set_incoming_new_target_or_generator_register(
        &mut self,
        incoming_new_target_or_generator_register: interpreter::Register,
    ) {
        if !incoming_new_target_or_generator_register.is_valid() {
            self.incoming_new_target_or_generator_register_offset_field = 0;
        } else {
            assert!(
                incoming_new_target_or_generator_register.index() < self.register_count() as usize
            );
            assert_ne!(0, incoming_new_target_or_generator_register.ToOperand());
            self.incoming_new_target_or_generator_register_offset_field =
                incoming_new_target_or_generator_register.ToOperand();
        }
    }

    pub fn clear_padding(&mut self) {
        let data_size = kHeaderSize + self.length() as usize;
        let size_for = Self::SizeFor(self.length());
        let padding_size = size_for - data_size;

        if padding_size > 0 {
            unsafe {
                let start_ptr = (self as *mut Self as *mut u8).add(data_size);
                std::ptr::write_bytes(start_ptr, 0, padding_size);
            }
        }
    }

    pub fn GetFirstBytecodeAddress(&self) -> Address {
        (self as *const Self as usize - kHeapObjectTag + kHeaderSize) as Address
    }

    pub fn HasSourcePositionTable(&self) -> bool {
        self.has_source_position_table(AcquireLoad::AcquireLoad)
    }

    pub fn SourcePositionTable(&self) -> Tagged<TrustedByteArray> {
        let maybe_table = self.raw_source_position_table(AcquireLoad::AcquireLoad);
        if IsTrustedByteArray(maybe_table) {
            unsafe { std::mem::transmute(maybe_table) }
        } else {
            assert_eq!(maybe_table, Smi::zero());
            unsafe {
                GetIsolateFromWritableObject(*self)
                    .as_mut()
                    .unwrap()
                    .heap()
                    .empty_trusted_byte_array()
            }
        }
    }

    pub fn SetSourcePositionsFailedToCollect(&mut self) {
        self.source_position_table_field = unsafe { std::mem::transmute(Smi::zero()) };
    }

    pub fn raw_constant_pool(&self) -> Tagged<Object> {
        let value = self.constant_pool_field as Tagged<Object>;
        assert!(value == unsafe { std::mem::transmute(Smi::zero()) } || IsTrustedFixedArray(value));
        value
    }

    pub fn raw_handler_table(&self) -> Tagged<Object> {
        let value = self.handler_table_field as Tagged<Object>;
        assert!(value == unsafe { std::mem::transmute(Smi::zero()) } || IsTrustedByteArray(value));
        value
    }

    pub fn raw_source_position_table(&self, acquire_load: AcquireLoad) -> Tagged<Object> {
        let value = self.source_position_table_field as Tagged<Object>;
        assert!(value == unsafe { std::mem::transmute(Smi::zero()) } || IsTrustedByteArray(value));
        value
    }

    pub fn BytecodeArraySize(&self) -> i32 {
        Self::SizeFor(self.length()) as i32
    }

    pub fn SizeIncludingMetadata(&self) -> i32 {
        let mut size = self.BytecodeArraySize();
        let maybe_constant_pool = self.raw_constant_pool();
        if IsTrustedFixedArray(maybe_constant_pool) {
            size += unsafe { std::mem::transmute::<_, TrustedFixedArray>(maybe_constant_pool) }.Size();
        } else {
            assert_eq!(maybe_constant_pool, unsafe { std::mem::transmute(Smi::zero()) });
        }
        let maybe_handler_table = self.raw_handler_table();
        if IsTrustedByteArray(maybe_handler_table) {
            size += unsafe { std::mem::transmute::<_, TrustedByteArray>(maybe_handler_table) }.AllocatedSize();
        } else {
            assert_eq!(maybe_handler_table, unsafe { std::mem::transmute(Smi::zero()) });
        }
        let maybe_table = self.raw_source_position_table(AcquireLoad::AcquireLoad);
        if IsByteArray(maybe_table) {
            size += unsafe { std::mem::transmute::<_, ByteArray>(maybe_table) }.AllocatedSize();
        }
        size
    }

    fn has_source_position_table(&self, acquire_load: AcquireLoad) -> bool {
        true
    }

    fn SizeFor(length: i32) -> usize {
        kHeaderSize + length as usize
    }

    fn ReadField<T>(&self, offset: usize) -> T {
        unsafe {
            let ptr = (self as *const Self as *const u8).add(offset) as *const T;
            *ptr
        }
    }

    fn WriteField<T>(&mut self, offset: usize, value: T) {
        unsafe {
            let ptr = (self as *mut Self as *mut u8).add(offset) as *mut T;
            *ptr = value;
        }
    }
}

mod interpreter {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Register {
        index: i32,
    }

    impl Register {
        pub fn invalid_value() -> Self {
            Register { index: -1 }
        }

        pub fn FromOperand(operand: i32) -> Self {
            Register { index: operand }
        }

        pub fn is_valid(&self) -> bool {
            self.index >= 0
        }

        pub fn ToOperand(&self) -> i32 {
            self.index
        }

        pub fn index(&self) -> usize {
            self.index as usize
        }
    }
}

enum AcquireLoad {
    AcquireLoad,
}

struct TaggedField<T> {}

impl TaggedField<Object> {
    fn Release_Store(bytecode_array: BytecodeArray, kSourcePositionTableOffset: usize, arg: Smi) {}
}
