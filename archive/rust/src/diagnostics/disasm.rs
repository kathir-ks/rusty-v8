// Copyright 2007-2008 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod disasm {
    use std::fmt;
    use std::fmt::Write;
    use std::ptr;

    /// Interface and default implementation for converting addresses and
    /// register-numbers to text. The default implementation is machine
    /// specific.
    pub trait NameConverter {
        fn name_of_cpu_register(&self, reg: i32) -> String;
        fn name_of_byte_cpu_register(&self, reg: i32) -> String;
        fn name_of_xmm_register(&self, reg: i32) -> String;
        fn name_of_address(&self, addr: *const u8) -> String;
        fn name_of_constant(&self, addr: *const u8) -> String;
        fn name_in_code(&self, addr: *const u8) -> String;

        /// Given a root-register-relative offset, returns either a name or None if
        /// none is found.
        /// TODO(jgruber,v8:7989): This is a temporary solution until we can preserve
        /// code comments through snapshotting.
        fn root_relative_name(&self, offset: i32) -> Option<String> {
            None //UNREACHABLE
        }
    }

    /// A generic Disassembler interface
    pub struct Disassembler<'a, T: NameConverter> {
        converter: &'a T,
        unimplemented_opcode_action: UnimplementedOpcodeAction,
    }

    #[derive(Clone, Copy)]
    pub enum UnimplementedOpcodeAction {
        ContinueOnUnimplementedOpcode,
        AbortOnUnimplementedOpcode,
    }

    impl<'a, T: NameConverter> Disassembler<'a, T> {
        pub fn new(converter: &'a T, unimplemented_opcode_action: UnimplementedOpcodeAction) -> Self {
            Disassembler {
                converter,
                unimplemented_opcode_action,
            }
        }

        pub fn unimplemented_opcode_action(&self) -> UnimplementedOpcodeAction {
            self.unimplemented_opcode_action
        }

        /// Writes one disassembled instruction into 'buffer' (0-terminated).
        /// Returns the length of the disassembled machine instruction in bytes.
        pub fn instruction_decode(&self, buffer: &mut String, instruction: *const u8) -> usize {
            // Placeholder implementation.  The real implementation would
            // perform the decoding and write the disassembled
            // instruction into the buffer.
            buffer.clear();
            write!(buffer, "/* instruction decoding not implemented */").unwrap();
            1 // Dummy return value
        }

        /// Returns -1 if instruction does not mark the beginning of a constant pool,
        /// or the number of entries in the constant pool beginning here.
        pub fn constant_pool_size_at(&self, instruction: *const u8) -> i32 {
            -1 // Placeholder
        }

        /// Write disassembly into specified writer using specified NameConverter
        /// (see constructor).
        pub fn disassemble<W: Write>(
            f: &mut W,
            begin: *const u8,
            end: *const u8,
            converter: &T,
            unimplemented_action: UnimplementedOpcodeAction,
        ) -> fmt::Result {
            let disasm = Disassembler::new(converter, unimplemented_action);
            let mut current = begin;
            while current < end {
                let mut buffer = String::new();
                let instruction_length = disasm.instruction_decode(&mut buffer, current);
                writeln!(f, "{:p}: {}", current, buffer)?;
                current = unsafe { current.add(instruction_length) };
            }
            Ok(())
        }
    }

    pub struct DefaultNameConverter {}

    impl DefaultNameConverter {
        pub fn new() -> Self {
            DefaultNameConverter {}
        }
    }

    impl NameConverter for DefaultNameConverter {
        fn name_of_cpu_register(&self, reg: i32) -> String {
            format!("r{}", reg)
        }

        fn name_of_byte_cpu_register(&self, reg: i32) -> String {
            format!("r{}b", reg)
        }

        fn name_of_xmm_register(&self, reg: i32) -> String {
            format!("xmm{}", reg)
        }

        fn name_of_address(&self, addr: *const u8) -> String {
            format!("{:p}", addr)
        }

        fn name_of_constant(&self, addr: *const u8) -> String {
            format!("const_{:p}", addr)
        }

        fn name_in_code(&self, addr: *const u8) -> String {
            format!("code_{:p}", addr)
        }
    }
}