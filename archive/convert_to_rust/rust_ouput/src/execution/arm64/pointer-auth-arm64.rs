// Converted from V8 C++ source files:
// Header: N/A
// Implementation: pointer-auth-arm64.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]

mod util {
    pub fn unsigned_bitextract_64(high: i32, low: i32, value: u64) -> u64 {
        let mask = ((1u64 << (high - low + 1)) - 1) << low;
        (value & mask) >> low
    }
}

#[cfg(feature = "USE_SIMULATOR")]
mod arm64 {
    use std::convert::TryInto;

    use crate::util::unsigned_bitextract_64;

    pub struct Simulator {
    }

    impl Simulator {
        // Randomly generated example key for simulating only.
        pub const kPACKeyIB: Simulator::PACKey = Simulator::PACKey {
            high: 0xeebb163b474e04c8,
            low: 0x5267ac6fc280fb7c,
            number: 1,
        };

        pub struct PACKey {
            pub high: u64,
            pub low: u64,
            pub number: u8,
        }

        fn get_nibble(in_data: u64, position: i32) -> u64 {
            (in_data >> position) & 0xf
        }

        fn pac_cell_shuffle(in_data: u64) -> u64 {
            static IN_POSITIONS: [i32; 16] = [52, 24, 44, 0, 28, 48, 4, 40, 32, 12, 56, 20, 8, 36, 16, 60];
            let mut out_data: u64 = 0;
            for i in 0..16 {
                out_data |= Self::get_nibble(in_data, IN_POSITIONS[i]) << (4 * i);
            }
            out_data
        }

        fn pac_cell_inv_shuffle(in_data: u64) -> u64 {
            static IN_POSITIONS: [i32; 16] = [12, 24, 48, 36, 56, 44, 4, 16, 32, 52, 28, 8, 20, 0, 40, 60];
            let mut out_data: u64 = 0;
            for i in 0..16 {
                out_data |= Self::get_nibble(in_data, IN_POSITIONS[i]) << (4 * i);
            }
            out_data
        }

        fn rot_cell(in_cell: u64, amount: i32) -> u64 {
            assert!((amount >= 1) && (amount <= 3));

            let mut in_cell = in_cell & 0xf;
            let temp = (in_cell << 4 | in_cell) as u8;
            ((temp >> (4 - amount)) & 0xf) as u64
        }

        fn pac_mult(s_input: u64) -> u64 {
            let mut s_output: u64 = 0;

            for i in 0..4 {
                let s12 = (s_input >> (4 * (i + 12))) & 0xf;
                let s8 = (s_input >> (4 * (i + 8))) & 0xf;
                let s4 = (s_input >> (4 * (i + 4))) & 0xf;
                let s0 = (s_input >> (4 * (i + 0))) & 0xf;

                let t0 = Self::rot_cell(s8, 1) ^ Self::rot_cell(s4, 2) ^ Self::rot_cell(s0, 1);
                let t1 = Self::rot_cell(s12, 1) ^ Self::rot_cell(s4, 1) ^ Self::rot_cell(s0, 2);
                let t2 = Self::rot_cell(s12, 2) ^ Self::rot_cell(s8, 1) ^ Self::rot_cell(s0, 1);
                let t3 = Self::rot_cell(s12, 1) ^ Self::rot_cell(s8, 2) ^ Self::rot_cell(s4, 1);

                s_output |= t3 << (4 * (i + 0));
                s_output |= t2 << (4 * (i + 4));
                s_output |= t1 << (4 * (i + 8));
                s_output |= t0 << (4 * (i + 12));
            }
            s_output
        }

        fn pac_sub(t_input: u64) -> u64 {
            let mut t_output: u64 = 0;
            static SUBSTITUTIONS: [u8; 16] = [0xb, 0x6, 0x8, 0xf, 0xc, 0x0, 0x9, 0xe, 0x3, 0x7, 0x4, 0x5, 0xd, 0x2, 0x1, 0xa];
            for i in 0..16 {
                let index = ((t_input >> (4 * i)) & 0xf) as usize;
                t_output |= (SUBSTITUTIONS[index] as u64) << (4 * i);
            }
            t_output
        }

        fn pac_inv_sub(t_input: u64) -> u64 {
            let mut t_output: u64 = 0;
            static SUBSTITUTIONS: [u8; 16] = [0x5, 0xe, 0xd, 0x8, 0xa, 0xb, 0x1, 0x9, 0x2, 0x6, 0xf, 0x0, 0x4, 0xc, 0x7, 0x3];
            for i in 0..16 {
                let index = ((t_input >> (4 * i)) & 0xf) as usize;
                t_output |= (SUBSTITUTIONS[index] as u64) << (4 * i);
            }
            t_output
        }

        fn tweak_cell_inv_rot(in_cell: u64) -> u64 {
            let mut out_cell: u64 = 0;
            out_cell |= (in_cell & 0x7) << 1;
            out_cell |= (in_cell & 0x1) ^ ((in_cell >> 3) & 0x1);
            out_cell
        }

        fn tweak_inv_shuffle(in_data: u64) -> u64 {
            let mut out_data: u64 = 0;
            out_data |= Self::tweak_cell_inv_rot(in_data >> 48) << 0;
            out_data |= ((in_data >> 52) & 0xf) << 4;
            out_data |= ((in_data >> 20) & 0xff) << 8;
            out_data |= ((in_data >> 0) & 0xff) << 16;
            out_data |= Self::tweak_cell_inv_rot(in_data >> 8) << 24;
            out_data |= ((in_data >> 12) & 0xf) << 28;
            out_data |= Self::tweak_cell_inv_rot(in_data >> 28) << 32;
            out_data |= Self::tweak_cell_inv_rot(in_data >> 60) << 36;
            out_data |= Self::tweak_cell_inv_rot(in_data >> 56) << 40;
            out_data |= Self::tweak_cell_inv_rot(in_data >> 16) << 44;
            out_data |= ((in_data >> 32) & 0xfff) << 48;
            out_data |= Self::tweak_cell_inv_rot(in_data >> 44) << 60;
            out_data
        }

        fn tweak_cell_rot(in_cell: u64) -> u64 {
            let mut out_cell: u64 = 0;
            out_cell |= ((in_cell & 0x1) ^ ((in_cell >> 1) & 0x1)) << 3;
            out_cell |= (in_cell >> 0x1) & 0x7;
            out_cell
        }

        fn tweak_shuffle(in_data: u64) -> u64 {
            let mut out_data: u64 = 0;
            out_data |= ((in_data >> 16) & 0xff) << 0;
            out_data |= Self::tweak_cell_rot(in_data >> 24) << 8;
            out_data |= ((in_data >> 28) & 0xf) << 12;
            out_data |= Self::tweak_cell_rot(in_data >> 44) << 16;
            out_data |= ((in_data >> 8) & 0xff) << 20;
            out_data |= Self::tweak_cell_rot(in_data >> 32) << 28;
            out_data |= ((in_data >> 48) & 0xfff) << 32;
            out_data |= Self::tweak_cell_rot(in_data >> 60) << 44;
            out_data |= Self::tweak_cell_rot(in_data >> 0) << 48;
            out_data |= ((in_data >> 4) & 0xf) << 52;
            out_data |= Self::tweak_cell_rot(in_data >> 40) << 56;
            out_data |= Self::tweak_cell_rot(in_data >> 36) << 60;
            out_data
        }

        // For a description of QARMA see:
        // The QARMA Block Cipher Family, Roberto Avanzi, Qualcomm Product Security
        // Initiative.
        // The pseudocode is available in ARM DDI 0487D.b, J1-6946.
        pub fn compute_pac(data: u64, context: u64, key: Simulator::PACKey) -> u64 {
            let key0 = key.high;
            let key1 = key.low;
            const RC: [u64; 5] = [0x0000000000000000, 0x13198a2e03707344,
                0xa4093822299f31d0, 0x082efa98ec4e6c89,
                0x452821e638d01377];
            const ALPHA: u64 = 0xc0ac29B7c97c50dd;

            let modk0 = ((key0 & 0x1) << 63) | ((key0 >> 2) << 1) |
                ((key0 >> 63) ^ ((key0 >> 1) & 0x1));
            let mut running_mod = context;
            let mut working_val = data ^ key0;
            let mut round_key;
            for i in 0..5 {
                round_key = key1 ^ running_mod;
                working_val ^= round_key;
                working_val ^= RC[i];
                if i > 0 {
                    working_val = Self::pac_cell_shuffle(working_val);
                    working_val = Self::pac_mult(working_val);
                }
                working_val = Self::pac_sub(working_val);
                running_mod = Self::tweak_shuffle(running_mod);
            }

            round_key = modk0 ^ running_mod;
            working_val ^= round_key;
            working_val = Self::pac_cell_shuffle(working_val);
            working_val = Self::pac_mult(working_val);
            working_val = Self::pac_sub(working_val);
            working_val = Self::pac_cell_shuffle(working_val);
            working_val = Self::pac_mult(working_val);
            working_val ^= key1;
            working_val = Self::pac_cell_inv_shuffle(working_val);
            working_val = Self::pac_inv_sub(working_val);
            working_val = Self::pac_mult(working_val);
            working_val = Self::pac_cell_inv_shuffle(working_val);
            working_val ^= key0;
            working_val ^= running_mod;

            for i in 0..5 {
                working_val = Self::pac_inv_sub(working_val);
                if i < 4 {
                    working_val = Self::pac_mult(working_val);
                    working_val = Self::pac_cell_inv_shuffle(working_val);
                }
                running_mod = Self::tweak_inv_shuffle(running_mod);
                round_key = key1 ^ running_mod;
                working_val ^= RC[4 - i];
                working_val ^= round_key;
                working_val ^= ALPHA;
            }

            working_val ^ modk0
        }

        const K_TTBR_MASK: u64 = 0x8000000000000000;

        fn has_tbi(ptr: u64, _type: PointerType) -> bool {
            (ptr >> 55) & 1 == 1
        }

        fn get_bottom_pac_bit(_ptr: u64, ttbr: i32) -> i32 {
            if ttbr == 0 {
                56
            } else {
                0
            }
        }

        fn get_top_pac_bit(_ptr: u64, _type: PointerType) -> i32 {
            62
        }

        // The TTBR is selected by bit 63 or 55 depending on TBI for pointers without
        // codes, but is always 55 once a PAC code is added to a pointer. For this
        // reason, it must be calculated at the call site.
        pub fn calculate_pac_mask(ptr: u64, type_: PointerType, ttbr: i32) -> u64 {
            let bottom_pac_bit = Self::get_bottom_pac_bit(ptr, ttbr);
            let top_pac_bit = Self::get_top_pac_bit(ptr, type_);
            unsigned_bitextract_64(top_pac_bit, bottom_pac_bit,
                                    0xffffffffffffffff & !Self::K_TTBR_MASK)
                << bottom_pac_bit
        }

        pub fn auth_pac(ptr: u64, context: u64, key: Simulator::PACKey,
                        type_: PointerType) -> u64 {
            assert!((key.number == 0) || (key.number == 1));

            let pac_mask = Self::calculate_pac_mask(ptr, type_, ((ptr >> 55) & 1) as i32);
            let original_ptr =
                if ((ptr & Self::K_TTBR_MASK) == 0) { (ptr & !pac_mask) } else { (ptr | pac_mask) };

            let pac = Self::compute_pac(original_ptr, context, key);

            let error_code = 1u64 << key.number;
            if (pac & pac_mask) == (ptr & pac_mask) {
                original_ptr
            } else {
                let error_lsb = Self::get_top_pac_bit(ptr, type_) - 2;
                let error_mask = 0x3u64 << error_lsb;
                if true {
                    panic!("Pointer authentication failure.");
                }
                (original_ptr & !error_mask) | (error_code << error_lsb)
            }
        }

        pub fn add_pac(ptr: u64, context: u64, key: Simulator::PACKey,
                       type_: PointerType) -> u64 {
            let top_pac_bit = Self::get_top_pac_bit(ptr, type_);

            assert!(Self::has_tbi(ptr, type_));
            let ttbr = ((ptr >> 55) & 1) as i32;
            let pac_mask = Self::calculate_pac_mask(ptr, type_, ttbr);
            let ext_ptr = if ttbr == 0 { (ptr & !pac_mask) } else { (ptr | pac_mask) };

            let pac = Self::compute_pac(ext_ptr, context, key);

            // If the pointer isn't all zeroes or all ones in the PAC bitfield, corrupt
            // the resulting code.
            if (((ptr & (pac_mask | Self::K_TTBR_MASK)) != 0x0) &&
                ((!ptr & (pac_mask | Self::K_TTBR_MASK)) != 0x0)) {
                let pac = pac ^ (1 << (top_pac_bit - 1));
            }

            let ttbr_shifted = (ttbr as u64) << 55;
            (pac & pac_mask) | ttbr_shifted | (ptr & !pac_mask)
        }

        pub fn strip_pac(ptr: u64, type_: PointerType) -> u64 {
            let pac_mask = Self::calculate_pac_mask(ptr, type_, ((ptr >> 55) & 1) as i32);
            if ((ptr & Self::K_TTBR_MASK) == 0) { (ptr & !pac_mask) } else { (ptr | pac_mask) }
        }
    }

    #[derive(Clone, Copy)]
    pub enum PointerType {
        Normal,
        Tagged,
    }
}
