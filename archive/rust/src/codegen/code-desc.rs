// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod code_desc {
    //use crate::common::globals::*; // Assuming globals are defined elsewhere
    //use crate::assembler::Assembler; // Assuming Assembler is defined elsewhere
    //use std::ptr::NonNull;

    /// A CodeDesc describes a buffer holding instructions and relocation
    /// information. The instructions start at the beginning of the buffer
    /// and grow forward, the relocation information starts at the end of
    /// the buffer and grows backward. Inlined metadata sections may exist
    /// at the end of the instructions.
    ///
    ///  |<--------------- buffer_size ----------------------------------->|
    ///  |<---------------- instr_size ------------->|      |<-reloc_size->|
    ///  |--------------+----------------------------+------+--------------|
    ///  | instructions |         data               | free |  reloc info  |
    ///  +--------------+----------------------------+------+--------------+
    ///
    /// TODO(jgruber): Add a single chokepoint for specifying the instruction area
    /// layout (i.e. the order of inlined metadata fields).
    /// TODO(jgruber): Systematically maintain inlined metadata offsets and sizes
    /// to simplify CodeDesc initialization.
    #[derive(Debug)]
    pub struct CodeDesc {
        pub buffer: *mut u8, // TODO: Use Box<[u8]> or Vec<u8> if possible and manage lifetime
        pub buffer_size: usize,

        /// The instruction area contains executable code plus inlined metadata.
        pub instr_size: usize,

        /// Metadata packed into the instructions area.
        pub safepoint_table_offset: usize,
        pub safepoint_table_size: usize,

        pub handler_table_offset: usize,
        pub handler_table_size: usize,

        pub constant_pool_offset: usize,
        pub constant_pool_size: usize,

        pub code_comments_offset: usize,
        pub code_comments_size: usize,

        pub builtin_jump_table_info_offset: usize,
        pub builtin_jump_table_info_size: usize,

        /// Relocation info is located at the end of the buffer and not part of the
        /// instructions area.
        pub reloc_offset: usize,
        pub reloc_size: usize,

        /// Unwinding information.
        pub unwinding_info: *mut u8, // TODO: Use Box<[u8]> or Vec<u8> if possible and manage lifetime
        pub unwinding_info_size: usize,

        pub origin: *mut std::ffi::c_void, //Assembler, // Assuming Assembler is defined elsewhere, using c_void for now.
    }

    impl CodeDesc {
        pub fn initialize(
            desc: &mut CodeDesc,
            //assembler: &mut Assembler,
            assembler: *mut std::ffi::c_void, //Using c_void for now
            safepoint_table_offset: usize,
            handler_table_offset: usize,
            constant_pool_offset: usize,
            code_comments_offset: usize,
            builtin_jump_table_info_offset: usize,
            reloc_info_offset: usize,
        ) {
            desc.origin = assembler;
            desc.safepoint_table_offset = safepoint_table_offset;
            desc.handler_table_offset = handler_table_offset;
            desc.constant_pool_offset = constant_pool_offset;
            desc.code_comments_offset = code_comments_offset;
            desc.builtin_jump_table_info_offset = builtin_jump_table_info_offset;
            desc.reloc_offset = reloc_info_offset;
        }

        #[cfg(debug_assertions)]
        pub fn verify(desc: &CodeDesc) {
            // Implementation for verification in debug mode
            // Add your assertions or checks here.
            // Example:
            // assert!(desc.buffer_size > 0, "Buffer size should be greater than 0");
            let _ = desc; // Avoid unused variable warning.
        }

        #[cfg(not(debug_assertions))]
        #[inline]
        pub fn verify(desc: &CodeDesc) {
            let _ = desc; // Avoid unused variable warning.
        }


        // TODO(jgruber,v8:11036): Remove these functions once CodeDesc fields have
        // been made consistent with InstructionStream layout.
        pub fn body_size(&self) -> usize {
            self.instr_size + self.unwinding_info_size
        }
        pub fn instruction_size(&self) -> usize {
            self.safepoint_table_offset
        }
        pub fn metadata_size(&self) -> usize {
            self.body_size() - self.instruction_size()
        }
        pub fn safepoint_table_offset_relative(&self) -> usize {
            self.safepoint_table_offset - self.instruction_size()
        }
        pub fn handler_table_offset_relative(&self) -> usize {
            self.handler_table_offset - self.instruction_size()
        }
        pub fn constant_pool_offset_relative(&self) -> usize {
            self.constant_pool_offset - self.instruction_size()
        }
        pub fn code_comments_offset_relative(&self) -> usize {
            self.code_comments_offset - self.instruction_size()
        }
        pub fn builtin_jump_table_info_offset_relative(&self) -> usize {
            self.builtin_jump_table_info_offset - self.instruction_size()
        }
        pub fn unwinding_info_offset_relative(&self) -> usize {
            // TODO(jgruber,v8:11036): Remove this function once unwinding_info setup
            // is more consistent with other metadata tables.
            self.builtin_jump_table_info_offset_relative() + self.builtin_jump_table_info_size
        }
    }
}