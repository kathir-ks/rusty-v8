// Converted from V8 C++ source files:
// Header: vlq-base64.h
// Implementation: vlq-base64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub use self::vlq_base64::*;

    mod vlq_base64 {
        use std::i32;

        #[allow(dead_code)]
        extern "C" {
            static mut stdout: *mut libc::FILE;
        }

        const K_CHAR_TO_DIGIT: [i8; 128] = {
            let mut arr = [-1i8; 128];
            arr[0x2D as usize] = -1;
            arr[0x2E as usize] = -1;
            arr[0x30 as usize] = 0x34;
            arr[0x31 as usize] = 0x35;
            arr[0x32 as usize] = 0x36;
            arr[0x33 as usize] = 0x37;
            arr[0x34 as usize] = 0x38;
            arr[0x35 as usize] = 0x39;
            arr[0x36 as usize] = 0x3a;
            arr[0x37 as usize] = 0x3b;
            arr[0x38 as usize] = 0x3c;
            arr[0x39 as usize] = 0x3d;
            arr[0x41 as usize] = 0x00;
            arr[0x42 as usize] = 0x01;
            arr[0x43 as usize] = 0x02;
            arr[0x44 as usize] = 0x03;
            arr[0x45 as usize] = 0x04;
            arr[0x46 as usize] = 0x05;
            arr[0x47 as usize] = 0x06;
            arr[0x48 as usize] = 0x07;
            arr[0x49 as usize] = 0x08;
            arr[0x4A as usize] = 0x09;
            arr[0x4B as usize] = 0x0a;
            arr[0x4C as usize] = 0x0b;
            arr[0x4D as usize] = 0x0c;
            arr[0x4E as usize] = 0x0d;
            arr[0x4F as usize] = 0x0e;
            arr[0x50 as usize] = 0x0f;
            arr[0x51 as usize] = 0x10;
            arr[0x52 as usize] = 0x11;
            arr[0x53 as usize] = 0x12;
            arr[0x54 as usize] = 0x13;
            arr[0x55 as usize] = 0x14;
            arr[0x56 as usize] = 0x15;
            arr[0x57 as usize] = 0x16;
            arr[0x58 as usize] = 0x17;
            arr[0x59 as usize] = 0x18;
            arr[0x5A as usize] = 0x19;
            arr[0x61 as usize] = 0x1a;
            arr[0x62 as usize] = 0x1b;
            arr[0x63 as usize] = 0x1c;
            arr[0x64 as usize] = 0x1d;
            arr[0x65 as usize] = 0x1e;
            arr[0x66 as usize] = 0x1f;
            arr[0x67 as usize] = 0x20;
            arr[0x68 as usize] = 0x21;
            arr[0x69 as usize] = 0x22;
            arr[0x6A as usize] = 0x23;
            arr[0x6B as usize] = 0x24;
            arr[0x6C as usize] = 0x25;
            arr[0x6D as usize] = 0x26;
            arr[0x6E as usize] = 0x27;
            arr[0x6F as usize] = 0x28;
            arr[0x70 as usize] = 0x29;
            arr[0x71 as usize] = 0x2a;
            arr[0x72 as usize] = 0x2b;
            arr[0x73 as usize] = 0x2c;
            arr[0x74 as usize] = 0x2d;
            arr[0x75 as usize] = 0x2e;
            arr[0x76 as usize] = 0x2f;
            arr[0x77 as usize] = 0x30;
            arr[0x78 as usize] = 0x31;
            arr[0x79 as usize] = 0x32;
            arr[0x7A as usize] = 0x33;
            arr[0x2B as usize] = 0x3e;
            arr[0x2F as usize] = 0x3f;
            arr
        };

        const K_CONTINUE_SHIFT: u32 = 5;
        const K_CONTINUE_MASK: u32 = 1 << K_CONTINUE_SHIFT;
        const K_DATA_MASK: u32 = K_CONTINUE_MASK - 1;

        fn char_to_digit_decode(c: u8) -> i8 {
            if c < 128 {
                K_CHAR_TO_DIGIT[c as usize]
            } else {
                -1
            }
        }

        pub fn char_to_digit_decode_for_testing(c: u8) -> i8 {
            char_to_digit_decode(c)
        }

        pub fn vlq_base64_decode(start: &str, sz: usize, pos: &mut usize) -> i32 {
            let mut res: u32 = 0;
            let mut shift: u64 = 0;
            let mut digit: i32;

            loop {
                if *pos >= sz {
                    return i32::min_value();
                }
                let char_code = start.as_bytes()[*pos];
                digit = char_to_digit_decode(char_code) as i32;

                let is_last_byte = (shift + K_CONTINUE_SHIFT as u64) >= 32;

                if digit == -1 || (is_last_byte && (digit >> 2) != 0) {
                    return i32::min_value();
                }

                res += ((digit as u32) & K_DATA_MASK) << shift;
                shift += K_CONTINUE_SHIFT as u64;
                *pos += 1;

                if (digit as u32) & K_CONTINUE_MASK == 0 {
                    break;
                }
            }

            if (res & 1) != 0 {
                return -((res >> 1) as i32);
            } else {
                return (res >> 1) as i32;
            }
        }
    }
}
