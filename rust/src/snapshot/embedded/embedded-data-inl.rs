// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/snapshot/embedded/embedded-data-inl.h

use crate::snapshot::embedded::embedded_data::EmbeddedData;
use crate::builtins::Builtin;
use crate::builtins::kFirstBytecodeHandler;
use crate::builtins::kBytecodeHandlersAreSortedLast;
use crate::builtins::Builtins;

// Assuming LayoutDescription and PadAndAlignCode are defined elsewhere,
// and accessible from here.  If they are template functions or classes,
// appropriate Rust generics or traits will be needed.  For now, I'm
// assuming they exist.

impl EmbeddedData {
    /// Returns the start address of the instruction for the given builtin.
    pub fn instruction_start_of(&self, builtin: Builtin) -> usize {
        debug_assert!(Builtins::is_builtin_id(builtin));
        let desc = self.layout_description(builtin);
        let result = self.raw_code() as usize + desc.instruction_offset as usize;
        debug_assert!(result < self.code as usize + self.code_size as usize);
        result
    }

    /// Returns the end address of the instruction for the given builtin.
    pub fn instruction_end_of(&self, builtin: Builtin) -> usize {
        debug_assert!(Builtins::is_builtin_id(builtin));
        let desc = self.layout_description(builtin);
        let result = self.raw_code() as usize + desc.instruction_offset as usize + desc.instruction_length as usize;
        debug_assert!(result < self.code as usize + self.code_size as usize);
        result
    }

    /// Returns the size of the instruction for the given builtin.
    pub fn instruction_size_of(&self, builtin: Builtin) -> u32 {
        debug_assert!(Builtins::is_builtin_id(builtin));
        let desc = self.layout_description(builtin);
        desc.instruction_length
    }

    /// Returns the start address of the metadata for the given builtin.
    pub fn metadata_start_of(&self, builtin: Builtin) -> usize {
        debug_assert!(Builtins::is_builtin_id(builtin));
        let desc = self.layout_description(builtin);
        let result = self.raw_metadata() as usize + desc.metadata_offset as usize;
        debug_assert!(desc.metadata_offset <= self.data_size);
        result
    }

    /// Returns the start address of the bytecode handlers instruction.
    pub fn instruction_start_of_bytecode_handlers(&self) -> usize {
        self.instruction_start_of(kFirstBytecodeHandler)
    }

    /// Returns the end address of the bytecode handlers instruction.
    pub fn instruction_end_of_bytecode_handlers(&self) -> usize {
        //static_assert!(Builtins::kBytecodeHandlersAreSortedLast);
        // Note this also includes trailing padding, but that's fine for our purposes.
        debug_assert!(kBytecodeHandlersAreSortedLast);
        self.code as usize + self.code_size as usize
    }

    /// Returns the padded instruction size of the given builtin.
    pub fn padded_instruction_size_of(&self, builtin: Builtin) -> u32 {
        let size = self.instruction_size_of(builtin);
        assert_ne!(size, 0);
        self.pad_and_align_code(size)
    }

    // Placeholders for LayoutDescription and PadAndAlignCode
    fn layout_description(&self, builtin: Builtin) -> LayoutDescription {
        // Replace with actual implementation
        LayoutDescription {
            instruction_offset: 0,
            instruction_length: 0,
            metadata_offset: 0,
        }
    }

    fn pad_and_align_code(&self, size: u32) -> u32 {
        // Replace with actual implementation
        size
    }

    // Placeholders for raw_code and raw_metadata.  Assuming they return *const u8
    fn raw_code(&self) -> *const u8 {
        self.code
    }

    fn raw_metadata(&self) -> *const u8 {
        self.data
    }
}

// Placeholder for LayoutDescription struct.  Replace with actual definition.
#[derive(Debug, Copy, Clone)]
struct LayoutDescription {
    instruction_offset: u32,
    instruction_length: u32,
    metadata_offset: u32,
}