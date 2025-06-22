// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

// #if !V8_ENABLE_WEBASSEMBLY
// #error This header should only be included if WebAssembly is enabled.
// #endif  // !V8_ENABLE_WEBASSEMBLY

use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    convert::TryInto,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    rc::Rc,
};

// use bit_cast::BitCast;
// use byteorder::{LittleEndian, WriteBytesExt};

// mod src {
//     pub mod base {
//         pub mod memory;
//         pub mod platform {
//             pub mod wrappers;
//         }
//         pub mod vector;
//     }
//     pub mod codegen {
//         pub mod signature;
//     }
//     pub mod wasm {
//         pub mod leb_helper;
//         pub mod local_decl_encoder;
//         pub mod value_type;
//         pub mod wasm_module;
//         pub mod wasm_opcodes;
//         pub mod wasm_result;
//         // pub mod zone {
//         //     pub mod zone_containers;
//         // }
//     }
//     pub mod zone {
//         pub mod zone_containers;
//         pub mod zone;
//     }
// }

pub mod base {
    pub mod memory {
        // Placeholder module for base::memory
    }
    pub mod platform {
        pub mod wrappers {
            // Placeholder module for base::platform::wrappers
        }
    }
    pub mod vector {
        // Placeholder module for base::vector
        pub type Vector<'a, T> = &'a [T];
    }
}

pub mod codegen {
    pub mod signature {
        // Placeholder module for codegen::signature
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct FunctionSig {
            pub params: &'static [ValueType],
            pub returns: &'static [ValueType],
        }
    }
}

pub mod wasm {
    pub mod leb_helper {
        // Placeholder module for wasm::leb_helper
        pub const kMaxVarInt32Size: usize = 5;
        pub const kMaxVarInt64Size: usize = 10;
        pub const kPaddedVarInt32Size: usize = 5;

        pub fn write_u32v(ptr: &mut *mut u8, val: u32) {
            unsafe {
                let mut current = val;
                loop {
                    let mut byte = (current & 0x7f) as u8;
                    current >>= 7;
                    if current != 0 {
                        byte |= 0x80;
                    }
                    **ptr = byte;
                    *ptr = (*ptr).add(1);
                    if current == 0 {
                        break;
                    }
                }
            }
        }

        pub fn write_i32v(ptr: &mut *mut u8, val: i32) {
            unsafe {
                let mut current = val as u32;
                loop {
                    let mut byte = (current & 0x7f) as u8;
                    current >>= 7;
                    if current != 0 {
                        byte |= 0x80;
                    }
                    **ptr = byte;
                    *ptr = (*ptr).add(1);
                    if current == 0 {
                        break;
                    }
                }
            }
        }

        pub fn write_u64v(ptr: &mut *mut u8, val: u64) {
            unsafe {
                let mut current = val;
                loop {
                    let mut byte = (current & 0x7f) as u8;
                    current >>= 7;
                    if current != 0 {
                        byte |= 0x80;
                    }
                    **ptr = byte;
                    *ptr = (*ptr).add(1);
                    if current == 0 {
                        break;
                    }
                }
            }
        }

        pub fn write_i64v(ptr: &mut *mut u8, val: i64) {
            unsafe {
                let mut current = val as u64;
                loop {
                    let mut byte = (current & 0x7f) as u8;
                    current >>= 7;
                    if current != 0 {
                        byte |= 0x80;
                    }
                    **ptr = byte;
                    *ptr = (*ptr).add(1);
                    if current == 0 {
                        break;
                    }
                }
            }
        }
    }
    pub mod local_decl_encoder {
        // Placeholder module for wasm::local_decl_encoder
        #[derive(Default)]
        pub struct LocalDeclEncoder {}

        impl LocalDeclEncoder {
            pub fn new() -> Self {
                LocalDeclEncoder {}
            }
        }
    }
    pub mod value_type {
        // Placeholder module for wasm::value_type
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ValueType {
            I32,
            I64,
            F32,
            F64,
            V128,
            Ref(HeapType),
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum HeapType {
            Func,
            Extern,
            Any,
            Eq,
            I31,
            Struct,
            Array,
            None,
            NoExtern,
            NoFunc,
        }

        impl ValueType {
            pub fn code(&self) -> ValueTypeCode {
                match self {
                    ValueType::I32 => ValueTypeCode::I32,
                    ValueType::I64 => ValueTypeCode::I64,
                    ValueType::F32 => ValueTypeCode::F32,
                    ValueType::F64 => ValueTypeCode::F64,
                    ValueType::V128 => ValueTypeCode::V128,
                    ValueType::Ref(_) => ValueTypeCode::Ref,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ValueTypeCode {
            I32 = 0x7f,
            I64 = 0x7e,
            F32 = 0x7d,
            F64 = 0x7c,
            V128 = 0x7b,
            Ref = 0x68,
        }
    }
    pub mod wasm_module {
        // Placeholder module for wasm::wasm_module
    }
    pub mod wasm_opcodes {
        // Placeholder module for wasm::wasm_opcodes
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum WasmOpcode {
            I32Const,
            I64Const,
            F32Const,
            F64Const,
            GetLocal,
            SetLocal,
            TeeLocal,
            Call,
            CallIndirect,
            Nop,
            I32Add,
            I32Sub,
            I32Mul,
            I32DivS,
            I32DivU,
            I32RemS,
            I32RemU,
            I32And,
            I32Or,
            I32Xor,
            I32Shl,
            I32ShrS,
            I32ShrU,
            I32Rotl,
            I32Rotr,
            I32Clz,
            I32Ctz,
            I32Popcnt,
            I32Eqz,
            I32Eq,
            I32Ne,
            I32LtS,
            I32LtU,
            I32GtS,
            I32GtU,
            I32LeS,
            I32LeU,
            I32GeS,
            I32GeU,
            MemorySize,
            MemoryGrow,
            I32Load,
            I64Load,
            F32Load,
            F64Load,
            I32Store,
            I64Store,
            F32Store,
            F64Store,
            RefNull,
            RefIsNull,
            RefFunc,
            Return,
            Block,
            Loop,
            If,
            Else,
            End,
            Br,
            BrIf,
            Unreachable,
            Drop,
        }
    }
    pub mod wasm_result {
        // Placeholder module for wasm::wasm_result
    }
}

pub mod zone {
    pub mod zone_containers {
        // Placeholder module for zone::zone_containers
        use std::marker::PhantomData;

        #[derive(Debug, Default)]
        pub struct ZoneVector<T> {
            data: Vec<T>,
        }

        impl<T> ZoneVector<T> {
            pub fn new() -> Self {
                ZoneVector { data: Vec::new() }
            }

            pub fn push(&mut self, value: T) {
                self.data.push(value);
            }

            pub fn extend_from_slice(&mut self, slice: &[T])
            where
                T: Copy,
            {
                self.data.extend_from_slice(slice);
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }

            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }

            pub fn get(&self, index: usize) -> Option<&T> {
                self.data.get(index)
            }

            pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
                self.data.get_mut(index)
            }

            pub fn iter(&self) -> std::slice::Iter<'_, T> {
                self.data.iter()
            }

            pub fn to_vec(&self) -> Vec<T>
            where
                T: Copy,
            {
                self.data.to_vec()
            }

            pub fn clear(&mut self) {
                self.data.clear();
            }
        }

        impl<T> Deref for ZoneVector<T> {
            type Target = Vec<T>;

            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        impl<T> DerefMut for ZoneVector<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.data
            }
        }

        #[derive(Debug, Default)]
        pub struct ZoneUnorderedMap<K, V> {
            data: HashMap<K, V>,
            _phantom: PhantomData<K>,
        }

        impl<K, V> ZoneUnorderedMap<K, V>
        where
            K: std::hash::Hash + Eq,
        {
            pub fn new() -> Self {
                ZoneUnorderedMap {
                    data: HashMap::new(),
                    _phantom: PhantomData,
                }
            }

            pub fn insert(&mut self, key: K, value: V) {
                self.data.insert(key, value);
            }

            pub fn get(&self, key: &K) -> Option<&V> {
                self.data.get(key)
            }

            pub fn contains_key(&self, key: &K) -> bool {
                self.data.contains_key(key)
            }

            pub fn remove(&mut self, key: &K) -> Option<V> {
                self.data.remove(key)
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }

            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }
        }
    }
    pub mod zone {
        use std::{alloc::{alloc, dealloc, Layout}, ptr::NonNull};
        use std::any::Any;
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::marker::PhantomData;
        use std::mem;
        use std::rc::Rc;

        #[derive(Debug)]
        pub struct Zone {
            blocks: RefCell<Vec<NonNull<u8>>>,
        }

        impl Zone {
            pub fn new() -> Self {
                Zone {
                    blocks: RefCell::new(Vec::new()),
                }
            }

            pub fn allocate_array<T>(&self, count: usize) -> *mut T {
                unsafe {
                    let layout = Layout::array::<T>(count).unwrap();
                    let ptr = alloc(layout) as *mut T;
                    if ptr.is_null() {
                        std::alloc::handle_alloc_error(layout);
                    }
                    self.blocks.borrow_mut().push(NonNull::new(ptr as *mut u8).unwrap());
                    ptr
                }
            }

            pub fn allocate<T>(&self) -> *mut T {
                unsafe {
                    let layout = Layout::new::<T>();
                    let ptr = alloc(layout) as *mut T;
                    if ptr.is_null() {
                        std::alloc::handle_alloc_error(layout);
                    }
                    self.blocks.borrow_mut().push(NonNull::new(ptr as *mut u8).unwrap());
                    ptr
                }
            }
        }
    }
}

pub mod v8_export_private {
    // Placeholder module for v8_export_private
}

pub mod wasm {
    use super::*;
    use base::{vector::Vector};
    use codegen::signature::FunctionSig;
    use std::{
        borrow::Cow,
        convert::TryInto,
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
        rc::Rc,
        sync::Arc,
    };
    use wasm::{
        leb_helper::{kMaxVarInt32Size, kMaxVarInt64Size, kPaddedVarInt32Size},
        local_decl_encoder::LocalDeclEncoder,
        value_type::{ValueType, ValueTypeCode},
        wasm_opcodes::WasmOpcode,
    };
    use zone::{
        zone::Zone,
        zone_containers::{ZoneUnorderedMap, ZoneVector},
    };

    #[derive(Debug)]
    pub struct ZoneBuffer {
        zone: *mut Zone, // Raw pointer to Zone, to be replaced
        buffer: *mut u8,   // Raw pointer to allocated buffer
        pos: *mut u8,
        end: *mut u8,
    }

    impl ZoneBuffer {
        const kInitialSize: usize = 1024;

        pub fn new(zone: *mut Zone, initial: usize) -> Self {
            unsafe {
                let initial_size = if initial == 0 {
                    Self::kInitialSize
                } else {
                    initial
                };
                let buffer = (*zone).allocate_array::<u8>(initial_size);
                let pos = buffer;
                let end = buffer.add(initial_size);
                ZoneBuffer {
                    zone: zone,
                    buffer: buffer,
                    pos: pos,
                    end: end,
                }
            }
        }

        pub fn write_u8(&mut self, x: u8) {
            self.ensure_space(1);
            unsafe {
                *self.pos = x;
                self.pos = self.pos.add(1);
            }
        }

        pub fn write_u16(&mut self, x: u16) {
            self.ensure_space(2);
            unsafe {
                let ptr = self.pos as *mut u16;
                *ptr = x.to_le();
                self.pos = self.pos.add(2);
            }
        }

        pub fn write_u32(&mut self, x: u32) {
            self.ensure_space(4);
            unsafe {
                let ptr = self.pos as *mut u32;
                *ptr = x.to_le();
                self.pos = self.pos.add(4);
            }
        }

        pub fn write_u64(&mut self, x: u64) {
            self.ensure_space(8);
            unsafe {
                let ptr = self.pos as *mut u64;
                *ptr = x.to_le();
                self.pos = self.pos.add(8);
            }
        }

        pub fn write_u32v(&mut self, val: u32) {
            self.ensure_space(kMaxVarInt32Size);
            unsafe {
                let mut pos_ptr = &mut self.pos;
                leb_helper::write_u32v(pos_ptr, val);
            }
        }

        pub fn write_u32v_index(&mut self, index: ModuleTypeIndex) {
            self.write_u32v(index.index);
        }

        pub fn write_i32v(&mut self, val: i32) {
            self.ensure_space(kMaxVarInt32Size);
            unsafe {
                let mut pos_ptr = &mut self.pos;
                leb_helper::write_i32v(pos_ptr, val);
            }
        }

        pub fn write_u64v(&mut self, val: u64) {
            self.ensure_space(kMaxVarInt64Size);
            unsafe {
                let mut pos_ptr = &mut self.pos;
                leb_helper::write_u64v(pos_ptr, val);
            }
        }

        pub fn write_i64v(&mut self, val: i64) {
            self.ensure_space(kMaxVarInt64Size);
            unsafe {
                let mut pos_ptr = &mut self.pos;
                leb_helper::write_i64v(pos_ptr, val);
            }
        }

        pub fn write_size(&mut self, val: usize) {
            self.ensure_space(kMaxVarInt32Size);
            let val_u32 = val.try_into().unwrap();
            unsafe {
                let mut pos_ptr = &mut self.pos;
                leb_helper::write_u32v(pos_ptr, val_u32);
            }
        }

        pub fn write_f32(&mut self, val: f32) {
            self.write_u32(val.to_bits());
        }

        pub fn write_f64(&mut self, val: f64) {
            self.write_u64(val.to_bits());
        }

        pub fn write(&mut self, data: &[u8], size: usize) {
            if size == 0 {
                return;
            }
            self.ensure_space(size);
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), self.pos, size);
                self.pos = self.pos.add(size);
            }
        }

        pub fn write_string(&mut self, name: Vector<char>) {
            let len = name.len();
            self.write_size(len);
            let bytes: Vec<u8> = name.iter().map(|&c| c as u8).collect();
            self.write(&bytes, len);
        }

        pub fn reserve_u32v(&mut self) -> usize {
            let off = self.offset();
            self.ensure_space(kMaxVarInt32Size);
            unsafe {
                self.pos = self.pos.add(kMaxVarInt32Size);
            }
            off
        }

        pub fn patch_u32v(&mut self, offset: usize, val: u32) {
            unsafe {
                let mut ptr = self.buffer.add(offset);
                let mut current_val = val;
                for _ in 0..kPaddedVarInt32Size {
                    let next = current_val >> 7;
                    let mut out = (current_val & 0x7f) as u8;
                    if _ != kPaddedVarInt32Size - 1 {
                        out |= 0x80;
                        *ptr = out;
                        ptr = ptr.add(1);
                        current_val = next;
                    } else {
                        *ptr = out;
                        ptr = ptr.add(1);
                    }
                }
            }
        }

        pub fn patch_u8(&mut self, offset: usize, val: u8) {
            unsafe {
                assert!(self.size() >= offset);
                *self.buffer.add(offset) = val;
            }
        }

        pub fn offset(&self) -> usize {
            unsafe { self.pos.offset_from(self.buffer) as usize }
        }

        pub fn size(&self) -> usize {
            unsafe { self.pos.offset_from(self.buffer) as usize }
        }

        pub fn data(&self) -> *mut u8 {
            self.buffer
        }

        pub fn begin(&self) -> *mut u8 {
            self.buffer
        }

        pub fn end(&self) -> *mut u8 {
            self.pos
        }

        fn ensure_space(&mut self, size: usize) {
            unsafe {
                if self.pos.add(size) > self.end {
                    let new_size = size + self.end.offset_from(self.buffer) as usize * 2;
                    let new_buffer = (*self.zone).allocate_array::<u8>(new_size);
                    std::ptr::copy_nonoverlapping(self.buffer, new_buffer, self.pos.offset_from(self.buffer) as usize);
                    self.pos = new_buffer.add(self.pos.offset_from(self.buffer) as usize);
                    self.buffer = new_buffer;
                    self.end = new_buffer.add(new_size);
                }
                assert!(self.pos.add(size) <= self.end);
            }
        }

        pub fn truncate(&mut self, size: usize) {
            unsafe {
                assert!(self.offset() >= size);
                self.pos = self.buffer.add(size);
            }
        }

        pub fn pos_ptr(&mut self) -> *mut *mut u8 {
            &mut self.pos
        }
    }

    #[derive(Debug)]
    pub struct WasmFunctionBuilder {
        builder_: *mut WasmModuleBuilder,
        locals_: LocalDeclEncoder,
        signature_index_: ModuleTypeIndex,
        func_index_: u32,
        body_: ZoneBuffer,
        name_: Vec<char>, //base::Vector<const char>
        i32_temps_: ZoneVector<u32>,
        i64_temps_: ZoneVector<u32>,
        f32_temps_: ZoneVector<u32>,
        f64_temps_: ZoneVector<u32>,
        direct_calls_: ZoneVector<DirectCallIndex>,

        // Delta-encoded mapping from wasm bytes to asm.js source positions.
        asm_offsets_: ZoneBuffer,
        last_asm_byte_offset_: u32,
        last_asm_source_position_: u32,
        asm_func_start_source_position_: u32,
        hint_: u8,
    }

    impl WasmFunctionBuilder {
        fn new(builder: *mut WasmModuleBuilder) -> Self {
            unsafe {
                let zone = (*builder).zone_;
                WasmFunctionBuilder {
                    builder_: builder,
                    locals_: LocalDeclEncoder::default(),
                    signature_index_: ModuleTypeIndex::new(0),
                    func_index_: 0,
                    body_: ZoneBuffer::new(zone, 0),
                    name_: Vec::new(),
                    i32_temps_: ZoneVector::new(),
                    i64_temps_: ZoneVector::new(),
                    f32_temps_: ZoneVector::new(),
                    f64_temps_: ZoneVector::new(),
                    direct_calls_: ZoneVector::new(),
                    asm_offsets_: ZoneBuffer::new(zone, 0),
                    last_asm_byte_offset_: 0,
                    last_asm_source_position_: 0,
                    asm_func_start_source_position_: 0,
                    hint_: kNoCompilationHint,
                }
            }
        }

        pub fn set_signature(&mut self, sig: &FunctionSig) {
            unsafe {
                let builder = &mut *self.builder_;
                let sig_index = builder.add_signature(sig, false, kNoSuperType);
                self.SetSignature(sig_index);
            }
        }

        pub fn set_signature_index(&mut self, sig_index: ModuleTypeIndex) {
            self.SetSignature(sig_index);
        }

        fn SetSignature(&mut self, sig_index: ModuleTypeIndex) {
            self.signature_index_ = sig_index;
        }

        pub fn add_local(&mut self, type_: ValueType) -> u32 {
            //TODO: implement local encoding
            0
        }

        pub fn emit_byte(&mut self, b: u8) {
            self.EmitByte(b);
        }

        fn EmitByte(&mut self, b: u8) {
            self.body_.write_u8(b);
        }

        pub fn emit_i32v(&mut self, val: i32) {
            self.EmitI32V(val);
        }

        fn EmitI32V(&mut self, val: i32) {
            self.body_.write_i32v(val);
        }

        pub fn emit_u32v(&mut self, val: u32) {
            self.EmitU32V(val);
        }

        fn EmitU32V(&mut self, val: u32) {
            self.body_.write_u32v(val);
        }

        pub fn emit_u64v(&mut self, val: u64) {
            self.EmitU64V(val);
        }

        fn EmitU64V(&mut self, val: u64) {
            self.body_.write_u64v(val);
        }

        pub fn emit_code(&mut self, code: &[u8]) {
            self.EmitCode(code, code.len() as u32);
        }

        fn EmitCode(&mut self, code: &[u8], code_size: u32) {
            self.body_.write(code, code_size as usize);
        }

        pub fn emit(&mut self, opcode: WasmOpcode) {
            self.Emit(opcode);
        }

        fn Emit(&mut self, opcode: WasmOpcode) {
            match opcode {
                WasmOpcode::I32Const => self.EmitByte(0x41),
                WasmOpcode::I64Const => self.EmitByte(0x42),
                WasmOpcode::F32Const => self.EmitByte(0x43),
                WasmOpcode::F64Const => self.EmitByte(0x44),
                WasmOpcode::GetLocal => self.EmitByte(0x20),
                WasmOpcode::SetLocal => self.EmitByte(0x21),
                WasmOpcode::TeeLocal => self.EmitByte(0x22),
                WasmOpcode::Call => self.EmitByte(0x10),
                WasmOpcode::CallIndirect => self.EmitByte(0x11),
                WasmOpcode::Nop => self.EmitByte(0x01),
                WasmOpcode::I32Add => self.EmitByte(0x6a),
                WasmOpcode::I32Sub => self.EmitByte(0x6b),
                WasmOpcode::I32Mul => self.EmitByte(0x6c),
                WasmOpcode::I32DivS => self.EmitByte(0x6d),
                WasmOpcode::I32DivU => self.EmitByte(0x6e),
                WasmOpcode::I32RemS => self.EmitByte(0x6f),
                WasmOpcode::I32RemU => self.EmitByte(0x70),
                WasmOpcode::I32And => self.EmitByte(0x71),
                WasmOpcode::I32Or => self.EmitByte(0x72),
                WasmOpcode::I32Xor => self.EmitByte(0x73),
                WasmOpcode::I32Shl => self.EmitByte(0x74),
                WasmOpcode::I32ShrS => self.EmitByte(0x75),
                WasmOpcode::I32ShrU => self.EmitByte(0x76),
                WasmOpcode::I32Rotl => self.EmitByte(0x77),
                WasmOpcode::I32Rotr => self.EmitByte(0x78),
                WasmOpcode::I32Clz => self.EmitByte(0x67),
                WasmOpcode::I32Ctz => self.EmitByte(0x68),
                WasmOpcode::I32Popcnt => self.EmitByte(0x69),
                WasmOpcode::I32Eqz => self.EmitByte(0x45),
                WasmOpcode::I32Eq => self.EmitByte(0x46),
                WasmOpcode::I32Ne => self.EmitByte(0x47),
                WasmOpcode::I32LtS => self.EmitByte(0x48),
                WasmOpcode::I32LtU => self.EmitByte(0x49),
                WasmOpcode::I32GtS => self.EmitByte(0x4a),
                WasmOpcode::I32GtU => self.EmitByte(0x4b),
                WasmOpcode::I32LeS => self.EmitByte(0x4c),
                WasmOpcode::I32LeU => self.EmitByte(0x4d),
                WasmOpcode::I32GeS => self.EmitByte(0x4e),
                WasmOpcode::I32GeU => self.EmitByte(0x4f),
                WasmOpcode::MemorySize => self.EmitByte(0x3f),
                WasmOpcode::MemoryGrow => self.EmitByte(0x40),
                WasmOpcode::I32Load => self.EmitByte(0x28),
                WasmOpcode::I64Load => self.EmitByte(0x29),
                WasmOpcode::F32Load => self.EmitByte(0x2a),
                WasmOpcode::F64Load => self.EmitByte(0x2b),
                WasmOpcode::I32Store => self.EmitByte(0x36),
                WasmOpcode::I64Store => self.EmitByte(0x37),
                WasmOpcode::F32Store => self.EmitByte(0x38),
                WasmOpcode::F64Store => self.EmitByte(0x39),
                WasmOpcode::RefNull => self.EmitByte(0xd0),
                WasmOpcode::RefIsNull => self.EmitByte(0xd1),
                WasmOpcode::RefFunc => self.EmitByte(0xd2),
                WasmOpcode::Return => self.EmitByte(0x0f),
                WasmOpcode::Block => self.EmitByte(0x02),
                WasmOpcode::Loop => self.EmitByte(0x03),
                WasmOpcode::If => self.EmitByte(0x04),
                WasmOpcode::Else => self.EmitByte(0x05),
                WasmOpcode::End => self.EmitByte(0x0b),
                WasmOpcode::Br => self.EmitByte(0x0c),
                WasmOpcode::BrIf => self.EmitByte(0x0d),
                WasmOpcode::Unreachable => self.EmitByte(0x00),
                WasmOpcode::Drop => self.EmitByte(0x1a),
            };
        }

        pub fn emit_get_local(&mut self, index: u32) {
            self.EmitGetLocal(index);
        }

        fn EmitGetLocal(&mut self, index: u32) {
            self.Emit(WasmOpcode::GetLocal);
            self.EmitU32V(index);
        }

        pub fn emit_set_local(&mut self, index: u32) {
            self.EmitSetLocal(index);
        }

        fn EmitSetLocal(&mut self, index: u32) {
            self.Emit(WasmOpcode::SetLocal);
            self.EmitU32V(index);
        }

        pub fn emit_tee_local(&mut self, index: u32) {
            self.EmitTeeLocal(index);
        }

        fn EmitTeeLocal(&mut self, index: u32) {
            self.Emit(WasmOpcode::TeeLocal);
            self.EmitU32V(index);
        }

        pub fn emit_i32_const(&mut self, val: i32) {
            self.EmitI32Const(val);
        }

        fn EmitI32Const(&mut self, val: i32) {
            self.Emit(WasmOpcode::I32Const);
            self.Emit