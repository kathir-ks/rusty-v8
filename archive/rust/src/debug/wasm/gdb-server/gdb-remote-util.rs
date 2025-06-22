// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod gdb_remote_util {
    use std::fmt::Write;
    use std::str;

    // TODO: Implement v8_flags.trace_wasm_gdb_remote.  For now, just use a const.
    const TRACE_WASM_GDB_REMOTE: bool = false;

    macro_rules! trace_gdb_remote {
        ($($arg:tt)*) => {
            if TRACE_WASM_GDB_REMOTE {
                println!("[gdb-remote] {}", format_args!($($arg)*));
            }
        };
    }

    /// Converts a byte (0-255) to a pair of hexadecimal ASCII characters (0-9, a-f).
    pub fn uint8_to_hex(byte: u8, chars: &mut [char; 2]) {
        let hex_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        chars[0] = hex_chars[(byte >> 4) as usize];
        chars[1] = hex_chars[(byte & 0x0F) as usize];
    }

    /// Converts a pair of hexadecimal characters into a byte (0-255).
    /// Returns `None` if either input character is not a valid hexadecimal digit.
    pub fn hex_to_uint8(chars: &[char; 2]) -> Option<u8> {
        let mut byte1: u8 = 0;
        let mut byte2: u8 = 0;

        if !nibble_to_uint8(chars[0], &mut byte1) {
            return None;
        }
        if !nibble_to_uint8(chars[1], &mut byte2) {
            return None;
        }

        Some((byte1 << 4) | byte2)
    }

    /// Converts an ASCII character (0-9, a-f, A-F) to a 4-bit unsigned integer.
    /// Returns `false` if the input character is not a valid hexadecimal digit.
    pub fn nibble_to_uint8(ch: char, byte: &mut u8) -> bool {
        if ch >= '0' && ch <= '9' {
            *byte = (ch as u8) - ('0' as u8);
            true
        } else if ch >= 'a' && ch <= 'f' {
            *byte = (ch as u8) - ('a' as u8) + 10;
            true
        } else if ch >= 'A' && ch <= 'F' {
            *byte = (ch as u8) - ('A' as u8) + 10;
            true
        } else {
            false
        }
    }

    /// Splits a string into a vector of strings, using the given delimiter.
    pub fn string_split(instr: &str, delim: &str) -> Vec<String> {
        instr.split(delim).map(|s| s.to_string()).collect()
    }

    /// Converts the memory pointed to by {mem} into a hex string in GDB-remote format.
    pub fn mem2hex(mem: &[u8]) -> String {
        let mut hex_string = String::with_capacity(mem.len() * 2);
        for &byte in mem {
            let mut chars = ['\0'; 2];
            uint8_to_hex(byte, &mut chars);
            write!(&mut hex_string, "{}{}", chars[0], chars[1]).unwrap();
        }
        hex_string
    }

    /// Converts the string into a hex string in GDB-remote format.
    pub fn mem2hex_string(str_data: &str) -> String {
        mem2hex(str_data.as_bytes())
    }

    /// Represents an address in a Wasm module code space for LLDB debugging.
    /// The address is 64 bits, where the first 32 bits identify the module ID,
    /// and the last 32 bits represent the offset within the module.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct wasm_addr_t {
        module_id_: u32,
        offset_: u32,
    }

    impl wasm_addr_t {
        /// Creates a new `wasm_addr_t` from a module ID and an offset.
        pub fn new(module_id: u32, offset: u32) -> Self {
            wasm_addr_t {
                module_id_: module_id,
                offset_: offset,
            }
        }

        /// Creates a new `wasm_addr_t` from a 64-bit address.
        pub fn from_address(address: u64) -> Self {
            wasm_addr_t {
                module_id_: (address >> 32) as u32,
                offset_: (address & 0xffffffff) as u32,
            }
        }

        /// Returns the module ID.
        pub fn module_id(&self) -> u32 {
            self.module_id_
        }

        /// Returns the offset within the module.
        pub fn offset(&self) -> u32 {
            self.offset_
        }
    }

    impl From<wasm_addr_t> for u64 {
        fn from(addr: wasm_addr_t) -> Self {
            (addr.module_id_ as u64) << 32 | addr.offset_ as u64
        }
    }
}