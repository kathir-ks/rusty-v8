// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/wasm/local-decl-encoder.rs

use std::io::Write;

mod leb_helper {
    pub fn write_u32v<W: Write>(writer: &mut W, mut value: u32) {
        loop {
            let mut byte = (value as u8) & 0x7f;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            writer.write_all(&[byte]).unwrap();
            if value == 0 {
                break;
            }
        }
    }

    pub fn write_i32v<W: Write>(writer: &mut W, mut value: i32) {
        loop {
            let mut byte = (value as u8) & 0x7f;
            value >>= 7;
            if (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0) {
            } else {
                byte |= 0x80;
            }
            writer.write_all(&[byte as u8]).unwrap();
            if (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0) {
                break;
            }
        }
    }

    pub fn sizeof_u32v(mut value: usize) -> usize {
        let mut size = 0;
        loop {
            size += 1;
            value >>= 7;
            if value == 0 {
                break;
            }
        }
        size
    }

    pub fn sizeof_i32v(mut value: i32) -> usize {
        let mut size = 0;
        loop {
            size += 1;
            value >>= 7;
            if (value == 0 && (size & 0x40) == 0) || (value == -1 && (size & 0x40) != 0) {
                break;
            }
        }
        size
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ValueType {
    code: u8,
    shared: bool,
    heap_type_code: Option<i32>,
}

impl ValueType {
    pub fn new(code: u8) -> Self {
        ValueType {
            code,
            shared: false,
            heap_type_code: None,
        }
    }

    pub fn value_type_code(&self) -> u8 {
        self.code
    }

    pub fn encoding_needs_shared(&self) -> bool {
        self.shared
    }

    pub fn encoding_needs_heap_type(&self) -> bool {
        self.heap_type_code.is_some()
    }

    pub fn heap_type(&self) -> HeapType {
        HeapType { code: self.heap_type_code.unwrap_or(0)}
    }

    pub fn set_shared(&mut self, shared: bool) {
        self.shared = shared;
    }

    pub fn set_heap_type(&mut self, heap_type_code: i32) {
        self.heap_type_code = Some(heap_type_code);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HeapType {
    code: i32,
}

impl HeapType {
    pub fn code(&self) -> i32 {
        self.code
    }
}

const kSharedFlagCode: u8 = 0x80; // Example value, replace with actual

#[derive(Debug)]
pub struct LocalDeclEncoder {
    local_decls: Vec<(u32, ValueType)>,
    total: u32,
    sig: Option<Signature>,
}

#[derive(Debug)]
pub struct Signature {
    parameter_count: u32,
}

impl Signature {
    pub fn new(parameter_count: u32) -> Self {
        Signature { parameter_count }
    }

    pub fn parameter_count(&self) -> u32 {
        self.parameter_count
    }
}

impl LocalDeclEncoder {
    pub fn new(sig: Option<Signature>) -> Self {
        LocalDeclEncoder {
            local_decls: Vec::new(),
            total: 0,
            sig,
        }
    }

    // The zone allocator is not used in the translated rust code
    pub fn prepend(&self, buffer: &mut Vec<u8>, start: &mut usize, end: &mut usize) {
        let size = *end - *start;
        let mut new_buffer = vec![0u8; self.size() + size];
        let pos = self.emit(&mut new_buffer);
        if size > 0 {
            new_buffer[pos..(pos + size)].copy_from_slice(&buffer[*start..*end]);
        }
        let pos = pos + size;
        *buffer = new_buffer;
        *start = 0;
        *end = pos;
    }

    pub fn emit(&self, buffer: &mut Vec<u8>) -> usize {
        let mut pos = 0;
        leb_helper::write_u32v(&mut &mut buffer[pos..], self.local_decls.len() as u32);
        pos += leb_helper::sizeof_u32v(self.local_decls.len());

        for local_decl in &self.local_decls {
            let locals_count = local_decl.0;
            let locals_type = local_decl.1;
            leb_helper::write_u32v(&mut &mut buffer[pos..], locals_count);
            pos += leb_helper::sizeof_u32v(locals_count as usize);

            buffer[pos] = locals_type.value_type_code();
            pos += 1;

            if locals_type.encoding_needs_shared() {
                buffer[pos] = kSharedFlagCode;
                pos += 1;
            }

            if locals_type.encoding_needs_heap_type() {
                leb_helper::write_i32v(&mut &mut buffer[pos..], locals_type.heap_type().code());
                pos += leb_helper::sizeof_i32v(locals_type.heap_type().code());
            }
        }
        assert_eq!(self.size(), pos);
        pos
    }

    pub fn add_locals(&mut self, count: u32, type_: ValueType) -> u32 {
        let result = self.total + self.sig.as_ref().map_or(0, |s| s.parameter_count());
        self.total += count;
        if let Some(last) = self.local_decls.last_mut() {
            if last.1 == type_ {
                let count = count + last.0;
                self.local_decls.pop();
                self.local_decls.push((count, type_));
                return result;
            }
        }
        self.local_decls.push((count, type_));
        result
    }

    pub fn size(&self) -> usize {
        let mut size = leb_helper::sizeof_u32v(self.local_decls.len());
        for p in &self.local_decls {
            size += leb_helper::sizeof_u32v(p.0 as usize) + 1; // number of locals + opcode

            if p.1.encoding_needs_shared() {
                size += 1;
            }

            if p.1.encoding_needs_heap_type() {
                size += leb_helper::sizeof_i32v(p.1.heap_type().code());
            }
        }
        size
    }
}