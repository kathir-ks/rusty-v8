// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod perfetto_utils {
    use std::mem;

    //use v8config; // Assuming v8config contains platform-specific configurations, might need custom solution
    use std::hash::{Hash, Hasher};
    use std::cmp::PartialEq;

    // Helper class to write String objects into Perfetto protos. Deals with
    // character encoding and String objects composed of multiple slices.
    #[derive(Debug)]
    pub struct PerfettoV8String {
        is_one_byte: bool,
        size: usize,
        buffer: Vec<u8>, // Changed to Vec<u8> as we no longer need unique_ptr and Box is managed by Vec
    }

    impl PerfettoV8String {
        pub fn new(string: String) -> Self {
            let is_one_byte = string.is_ascii();
            let size = string.len();
            let buffer = string.into_bytes();

            PerfettoV8String {
                is_one_byte,
                size,
                buffer,
            }
        }

        pub fn is_one_byte(&self) -> bool {
            self.is_one_byte
        }

        pub fn write_to_proto<Proto: ProtoWriter>(&self, proto: &mut Proto) {
            if self.is_one_byte() {
                proto.set_latin1(&self.buffer);
            } else {
                // Assuming UTF-16 handling.  Conversion might be needed
                // based on original V8 String object structure.
                let utf16_buffer: Vec<u16> = self.buffer.chunks(2).map(|chunk| {
                    u16::from_ne_bytes([chunk[0], chunk[1]])
                }).collect();

                #[cfg(target_endian = "big")]
                {
                    proto.set_utf16_be(&utf16_buffer);
                }
                #[cfg(target_endian = "little")]
                {
                    proto.set_utf16_le(&utf16_buffer);
                }

            }
        }
    }

    impl PartialEq for PerfettoV8String {
        fn eq(&self, other: &Self) -> bool {
            self.is_one_byte == other.is_one_byte && self.size == other.size && self.buffer == other.buffer
        }
    }

    impl Eq for PerfettoV8String {}

    impl Hash for PerfettoV8String {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.buffer.as_slice().hash(state);
            self.is_one_byte.hash(state);
        }
    }

    pub struct PerfettoV8StringHasher {}

    impl PerfettoV8StringHasher {
        pub fn new() -> Self {
            PerfettoV8StringHasher {}
        }
    }

    impl Hasher for PerfettoV8StringHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, bytes: &[u8]) {
        }
    }

    // Trait to simulate the Proto object's `set_latin1`, `set_utf16_le`, `set_utf16_be` methods.
    // Replace with your actual Proto implementation.
    pub trait ProtoWriter {
        fn set_latin1(&mut self, data: &[u8]);
        fn set_utf16_le(&mut self, data: &[u16]);
        fn set_utf16_be(&mut self, data: &[u16]);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // A mock proto struct for testing purposes
        #[derive(Debug, PartialEq)]
        struct MockProto {
            latin1: Vec<u8>,
            utf16_le: Vec<u16>,
            utf16_be: Vec<u16>,
        }

        impl MockProto {
            fn new() -> Self {
                MockProto {
                    latin1: Vec::new(),
                    utf16_le: Vec::new(),
                    utf16_be: Vec::new(),
                }
            }
        }

        impl ProtoWriter for MockProto {
            fn set_latin1(&mut self, data: &[u8]) {
                self.latin1 = data.to_vec();
            }

            fn set_utf16_le(&mut self, data: &[u16]) {
                self.utf16_le = data.to_vec();
            }

            fn set_utf16_be(&mut self, data: &[u16]) {
                self.utf16_be = data.to_vec();
            }
        }

        #[test]
        fn test_perfetto_v8_string_latin1() {
            let string = String::from("hello");
            let v8_string = PerfettoV8String::new(string);
            let mut proto = MockProto::new();

            v8_string.write_to_proto(&mut proto);

            assert_eq!(proto.latin1, vec![104, 101, 108, 108, 111]);
            assert_eq!(proto.utf16_le, Vec::new());
            assert_eq!(proto.utf16_be, Vec::new());
        }

        #[test]
        fn test_perfetto_v8_string_utf16() {
            let string = String::from("你好"); // Non-ascii
            let v8_string = PerfettoV8String::new(string);
            let mut proto = MockProto::new();

            v8_string.write_to_proto(&mut proto);

            // UTF-16 representation of "你好" (depending on endianness)
            let expected_utf16_le: Vec<u16> = vec![0x607d, 0x7d60];
            let expected_utf16_be: Vec<u16> = vec![0x7d60, 0x607d];

            #[cfg(target_endian = "little")]
            {
                assert_eq!(proto.utf16_le, expected_utf16_le);
                assert_eq!(proto.utf16_be, Vec::new());
            }
            #[cfg(target_endian = "big")]
            {
                assert_eq!(proto.utf16_be, expected_utf16_be);
                assert_eq!(proto.utf16_le, Vec::new());
            }
            assert_eq!(proto.latin1, Vec::new());

        }
    }
}