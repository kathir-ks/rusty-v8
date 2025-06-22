// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "webassembly"))]
// compile_error!("This module should only be included if WebAssembly is enabled.");

use std::cmp;
use std::io::Write;
use std::mem;
use std::ptr;
use std::slice;

mod string_builder;
use string_builder::StringBuilder;
use string_builder::StringBuilderMode;

pub mod debug {
    pub struct DisassemblyCollector {} // Placeholder
}

pub mod wasm {
    use super::*;

    /// Computes the number of decimal digits required to print {value}.
    #[inline]
    pub fn get_num_digits(value: u32) -> usize {
        let mut digits = 1;
        let mut compare = 10;
        while value >= compare {
            digits += 1;
            compare *= 10;
        }
        digits
    }

    #[derive(Debug)]
    pub struct LabelInfo<'a> {
        pub name_section_index: u32,
        pub line_number: usize,
        pub offset: usize,
        pub start: *mut u8,
        pub length: usize,
        _marker: std::marker::PhantomData<&'a ()>, // added lifetime parameter
    }

    impl<'a> LabelInfo<'a> {
        pub fn new(
            line_number: usize,
            offset: usize,
            index_by_occurrence_order: u32,
        ) -> LabelInfo<'a> {
            LabelInfo {
                name_section_index: index_by_occurrence_order,
                line_number,
                offset,
                start: ptr::null_mut(),
                length: 0,
                _marker: std::marker::PhantomData,
            }
        }
    }

    pub struct MultiLineStringBuilder {
        string_builder: StringBuilder,
        lines: Vec<Line>,
        pending_bytecode_offset: u32,
    }

    impl MultiLineStringBuilder {
        pub fn new() -> Self {
            MultiLineStringBuilder {
                string_builder: StringBuilder::new(StringBuilderMode::KeepOldChunks),
                lines: Vec::new(),
                pending_bytecode_offset: 0,
            }
        }

        pub fn next_line(&mut self, byte_offset: u32) {
            let alloc = self.string_builder.allocate(1);
            unsafe {
                *alloc = b'\n';
            }

            let len = self.string_builder.length();
            self.lines.push(Line::new(
                self.string_builder.start() as *const u8,
                len,
                self.pending_bytecode_offset,
            ));
            self.string_builder.start_here();
            self.pending_bytecode_offset = byte_offset;
        }

        pub fn line_number(&self) -> usize {
            self.lines.len()
        }

        pub fn set_current_line_bytecode_offset(&mut self, offset: u32) {
            self.pending_bytecode_offset = offset;
        }

        pub fn current_line_bytecode_offset(&self) -> u32 {
            self.pending_bytecode_offset
        }

        pub fn patch_label(&mut self, label: &mut LabelInfo, label_source: *const u8, label_length: usize) {
            debug_assert!(label.length > 0);
            debug_assert!(label.line_number < self.lines.len());

            // Step 1: Patching a line makes it longer, and we can't grow it in-place
            // because it's boxed in, so allocate space for its patched copy.
            let l = &mut self.lines[label.line_number];
            // +1 because we add a space before the label: "block" -> "block $label0",
            // "block i32" -> "block $label0 i32".
            let patched_length = l.len + label.length + 1;
            let patched_line;
            if self.string_builder.length() == 0 {
                // No current unfinished line. Allocate the patched line as if it was
                // the next line.
                patched_line = self.string_builder.allocate(patched_length);
                self.string_builder.start_here();
            } else {
                // Shift the current unfinished line out of the way.
                // TODO(jkummerow): This approach ends up being O(n²) for a `br_table`
                // with `n` labels. If that ever becomes a problem, we could allocate a
                // separate new chunk for patched copies of old lines, then we wouldn't
                // need to shift the unfinished line around.
                let unfinished_start = self.string_builder.start() as *const u8; // Remember the unfinished
                let unfinished_length = self.string_builder.length(); // line, and...
                self.string_builder.rewind_to_start(); // ...free up its space.
                patched_line = self.string_builder.allocate(patched_length);
                // Write the unfinished line into its new location.
                self.string_builder.start_here();

                let new_location = self.string_builder.allocate(unfinished_length);

                unsafe {
                    ptr::copy_nonoverlapping(unfinished_start, new_location, unfinished_length);
                }
                // Update label_source if it was pointing inside the old chunk.
                // This is unsafe, but it's mimicking what the original code does.
                if label_source >= unfinished_start && label_source < unsafe { unfinished_start.add(unfinished_length) } {
                    let offset = (label_source as usize) - (unfinished_start as usize);
                    label_source = unsafe { new_location.add(offset) };
                }
            }

            // Step 2: Write the patched copy of the line to be patched.
            let mut cursor = patched_line;
            unsafe {
                ptr::copy_nonoverlapping(l.data, cursor, label.offset);
                cursor = cursor.add(label.offset);
                *cursor = b' ';
                cursor = cursor.add(1);
                label.start = cursor;
                ptr::copy_nonoverlapping(label_source, cursor, label.length);
                cursor = cursor.add(label.length);
                ptr::copy_nonoverlapping(l.data.add(label.offset), cursor, l.len - label.offset);
            }

            l.data = patched_line;
            l.len = patched_length;
        }

        // Note: implemented in wasm-disassembler.cc (which is also the only user).
        // fn to_disassembly_collector(&self, collector: &mut v8::debug::DisassemblyCollector) {}

        pub fn write_to(&mut self, out: &mut dyn Write, print_offsets: bool, collect_offsets: &mut Vec<u32>) {
            if self.string_builder.length() != 0 {
                self.next_line(0);
            }
            if self.lines.is_empty() {
                return;
            }

            if print_offsets {
                // The last offset is expected to be the largest.
                let width = get_num_digits(self.lines.last().unwrap().bytecode_offset as u32);
                // We could have used std::setw(width), but this is faster.
                const K_BUF_SIZE: usize = 12; // Enough for any uint32 plus '|'.
                let mut buffer: [u8; K_BUF_SIZE] =
                    [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, b'|'];
                let buffer_end = buffer.as_mut_ptr().add(K_BUF_SIZE - 1);
                let buffer_start = unsafe { buffer_end.sub(width) };

                for l in &self.lines {
                    let mut offset = l.bytecode_offset;
                    let mut ptr = buffer_end;
                    while offset > 0 {
                        unsafe {
                            *ptr = b'0' + (offset % 10) as u8;
                            ptr = ptr.sub(1);
                        }
                        offset /= 10;
                        // We pre-filled the buffer with spaces, and the offsets are expected
                        // to be increasing, so we can just stop the loop here and don't need
                        // to write spaces until {ptr == buffer_start}.
                    }
                    let slice = unsafe { slice::from_raw_parts(buffer_start, width + 1) };
                    out.write_all(slice).unwrap();
                    let line_slice = unsafe { slice::from_raw_parts(l.data, l.len) };
                    out.write_all(line_slice).unwrap();
                }
                return;
            }
            // In the name of speed, batch up lines that happen to be stored
            // consecutively.
            let first = &self.lines[0];
            let mut last_start = first.data;
            let mut len = first.len;
            for i in 1..self.lines.len() {
                let l = &self.lines[i];
                if last_start as usize + len == l.data as usize {
                    len += l.len;
                } else {
                    let slice = unsafe { slice::from_raw_parts(last_start, len) };
                    out.write_all(slice).unwrap();
                    last_start = l.data;
                    len = l.len;
                }
            }
            let slice = unsafe { slice::from_raw_parts(last_start, len) };
            out.write_all(slice).unwrap();

            collect_offsets.reserve(self.lines.len());
            for l in &self.lines {
                collect_offsets.push(l.bytecode_offset);
            }
        }

        pub fn approximate_size_mb(&self) -> usize {
            self.string_builder.approximate_size_mb()
        }
    }

    #[derive(Debug)]
    struct Line {
        data: *mut u8,
        len: usize,
        bytecode_offset: u32,
    }

    impl Line {
        fn new(data: *const u8, len: usize, bytecode_offset: u32) -> Self {
            Line {
                data: data as *mut u8,
                len,
                bytecode_offset,
            }
        }
    }
}