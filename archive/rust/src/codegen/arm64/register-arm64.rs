// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: V8_TARGET_ARCH_ARM64 is assumed to be true for this Rust conversion.
//       Conditional compilation based on target architecture is handled via Cargo features in Rust.

pub mod register_arm64 {
    /// Represents the format of a vector register.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum VectorFormat {
        /// Undefined format.
        kFormatUndefined,
        /// 8-bit.
        kFormatB,
        /// 16-bit.
        kFormatH,
        /// 32-bit.
        kFormatS,
        /// 64-bit.
        kFormatD,
        /// 8 x 8-bit.
        kFormat8B,
        /// 16 x 8-bit.
        kFormat16B,
        /// 4 x 16-bit.
        kFormat4H,
        /// 8 x 16-bit.
        kFormat8H,
        /// 2 x 32-bit.
        kFormat2S,
        /// 4 x 32-bit.
        kFormat4S,
        /// 1 x 64-bit.
        kFormat1D,
        /// 2 x 64-bit.
        kFormat2D,
        /// 1 x 128-bit
        kFormat1Q,
    }

    const KB_REG_SIZE_IN_BITS: u32 = 8;
    const KH_REG_SIZE_IN_BITS: u32 = 16;
    const KS_REG_SIZE_IN_BITS: u32 = 32;
    const KD_REG_SIZE_IN_BITS: u32 = 64;
    const KQ_REG_SIZE_IN_BITS: u32 = 128;

    /// Returns the half-width vector format.
    pub fn vector_format_half_width(vform: VectorFormat) -> VectorFormat {
        assert!(
            vform == VectorFormat::kFormat8H
                || vform == VectorFormat::kFormat4S
                || vform == VectorFormat::kFormat2D
                || vform == VectorFormat::kFormat1Q
                || vform == VectorFormat::kFormatH
                || vform == VectorFormat::kFormatS
                || vform == VectorFormat::kFormatD
        );
        match vform {
            VectorFormat::kFormat8H => VectorFormat::kFormat8B,
            VectorFormat::kFormat4S => VectorFormat::kFormat4H,
            VectorFormat::kFormat2D => VectorFormat::kFormat2S,
            VectorFormat::kFormat1Q => VectorFormat::kFormat1D,
            VectorFormat::kFormatH => VectorFormat::kFormatB,
            VectorFormat::kFormatS => VectorFormat::kFormatH,
            VectorFormat::kFormatD => VectorFormat::kFormatS,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the double-width vector format.
    pub fn vector_format_double_width(vform: VectorFormat) -> VectorFormat {
        assert!(
            vform == VectorFormat::kFormat8B
                || vform == VectorFormat::kFormat4H
                || vform == VectorFormat::kFormat2S
                || vform == VectorFormat::kFormatB
                || vform == VectorFormat::kFormatH
                || vform == VectorFormat::kFormatS
        );
        match vform {
            VectorFormat::kFormat8B => VectorFormat::kFormat8H,
            VectorFormat::kFormat4H => VectorFormat::kFormat4S,
            VectorFormat::kFormat2S => VectorFormat::kFormat2D,
            VectorFormat::kFormatB => VectorFormat::kFormatH,
            VectorFormat::kFormatH => VectorFormat::kFormatS,
            VectorFormat::kFormatS => VectorFormat::kFormatD,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the vector format filled to Q register size.
    pub fn vector_format_fill_q(vform: VectorFormat) -> VectorFormat {
        match vform {
            VectorFormat::kFormatB | VectorFormat::kFormat8B | VectorFormat::kFormat16B => {
                VectorFormat::kFormat16B
            }
            VectorFormat::kFormatH | VectorFormat::kFormat4H | VectorFormat::kFormat8H => {
                VectorFormat::kFormat8H
            }
            VectorFormat::kFormatS | VectorFormat::kFormat2S | VectorFormat::kFormat4S => {
                VectorFormat::kFormat4S
            }
            VectorFormat::kFormatD | VectorFormat::kFormat1D | VectorFormat::kFormat2D => {
                VectorFormat::kFormat2D
            }
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the half-width vector format with double lanes.
    pub fn vector_format_half_width_double_lanes(vform: VectorFormat) -> VectorFormat {
        match vform {
            VectorFormat::kFormat4H => VectorFormat::kFormat8B,
            VectorFormat::kFormat8H => VectorFormat::kFormat16B,
            VectorFormat::kFormat2S => VectorFormat::kFormat4H,
            VectorFormat::kFormat4S => VectorFormat::kFormat8H,
            VectorFormat::kFormat1D => VectorFormat::kFormat2S,
            VectorFormat::kFormat2D => VectorFormat::kFormat4S,
            VectorFormat::kFormat1Q => VectorFormat::kFormat2D,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the vector format with double lanes.
    pub fn vector_format_double_lanes(vform: VectorFormat) -> VectorFormat {
        assert!(
            vform == VectorFormat::kFormat8B || vform == VectorFormat::kFormat4H || vform == VectorFormat::kFormat2S
        );
        match vform {
            VectorFormat::kFormat8B => VectorFormat::kFormat16B,
            VectorFormat::kFormat4H => VectorFormat::kFormat8H,
            VectorFormat::kFormat2S => VectorFormat::kFormat4S,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the vector format with half lanes.
    pub fn vector_format_half_lanes(vform: VectorFormat) -> VectorFormat {
        assert!(
            vform == VectorFormat::kFormat16B || vform == VectorFormat::kFormat8H || vform == VectorFormat::kFormat4S
        );
        match vform {
            VectorFormat::kFormat16B => VectorFormat::kFormat8B,
            VectorFormat::kFormat8H => VectorFormat::kFormat4H,
            VectorFormat::kFormat4S => VectorFormat::kFormat2S,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the scalar format from the given lane size.
    pub fn scalar_format_from_lane_size(lane_size: i32) -> VectorFormat {
        match lane_size {
            8 => VectorFormat::kFormatB,
            16 => VectorFormat::kFormatH,
            32 => VectorFormat::kFormatS,
            64 => VectorFormat::kFormatD,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the vector format filled to Q register size, based on lane size.
    pub fn vector_format_fill_q_from_lane_size(lane_size: i32) -> VectorFormat {
        vector_format_fill_q(scalar_format_from_lane_size(lane_size))
    }

    /// Returns the scalar format from a vector format.
    pub fn scalar_format_from_format(vform: VectorFormat) -> VectorFormat {
        scalar_format_from_lane_size(lane_size_in_bits_from_format(vform) as i32)
    }

    /// Returns the register size in bytes for a given vector format.
    pub fn register_size_in_bytes_from_format(vform: VectorFormat) -> u32 {
        register_size_in_bits_from_format(vform) / 8
    }

    /// Returns the register size in bits for a given vector format.
    pub fn register_size_in_bits_from_format(vform: VectorFormat) -> u32 {
        assert_ne!(vform, VectorFormat::kFormatUndefined);
        match vform {
            VectorFormat::kFormatB => KB_REG_SIZE_IN_BITS,
            VectorFormat::kFormatH => KH_REG_SIZE_IN_BITS,
            VectorFormat::kFormatS => KS_REG_SIZE_IN_BITS,
            VectorFormat::kFormatD => KD_REG_SIZE_IN_BITS,
            VectorFormat::kFormat8B => KD_REG_SIZE_IN_BITS,
            VectorFormat::kFormat4H => KD_REG_SIZE_IN_BITS,
            VectorFormat::kFormat2S => KD_REG_SIZE_IN_BITS,
            VectorFormat::kFormat1D => KD_REG_SIZE_IN_BITS,
            _ => KQ_REG_SIZE_IN_BITS,
        }
    }

    /// Returns the lane size in bits for a given vector format.
    pub fn lane_size_in_bits_from_format(vform: VectorFormat) -> u32 {
        assert_ne!(vform, VectorFormat::kFormatUndefined);
        match vform {
            VectorFormat::kFormatB | VectorFormat::kFormat8B | VectorFormat::kFormat16B => 8,
            VectorFormat::kFormatH | VectorFormat::kFormat4H | VectorFormat::kFormat8H => 16,
            VectorFormat::kFormatS | VectorFormat::kFormat2S | VectorFormat::kFormat4S => 32,
            VectorFormat::kFormatD | VectorFormat::kFormat1D | VectorFormat::kFormat2D => 64,
            VectorFormat::kFormat1Q => 128,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the lane size in bytes for a given vector format.
    pub fn lane_size_in_bytes_from_format(vform: VectorFormat) -> i32 {
        (lane_size_in_bits_from_format(vform) / 8) as i32
    }

    /// Returns the base-2 logarithm of the lane size in bytes for a given vector format.
    pub fn lane_size_in_bytes_log2_from_format(vform: VectorFormat) -> i32 {
        assert_ne!(vform, VectorFormat::kFormatUndefined);
        match vform {
            VectorFormat::kFormatB | VectorFormat::kFormat8B | VectorFormat::kFormat16B => 0,
            VectorFormat::kFormatH | VectorFormat::kFormat4H | VectorFormat::kFormat8H => 1,
            VectorFormat::kFormatS | VectorFormat::kFormat2S | VectorFormat::kFormat4S => 2,
            VectorFormat::kFormatD | VectorFormat::kFormat1D | VectorFormat::kFormat2D => 3,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the number of lanes in a given vector format.
    pub fn lane_count_from_format(vform: VectorFormat) -> i32 {
        assert_ne!(vform, VectorFormat::kFormatUndefined);
        match vform {
            VectorFormat::kFormat16B => 16,
            VectorFormat::kFormat8B | VectorFormat::kFormat8H => 8,
            VectorFormat::kFormat4H | VectorFormat::kFormat4S => 4,
            VectorFormat::kFormat2S | VectorFormat::kFormat2D => 2,
            VectorFormat::kFormat1D | VectorFormat::kFormat1Q | VectorFormat::kFormatB
            | VectorFormat::kFormatH | VectorFormat::kFormatS | VectorFormat::kFormatD => 1,
            _ => panic!("Unreachable"),
        }
    }

    /// Returns the maximum number of lanes for the given vector format family.
    pub fn max_lane_count_from_format(vform: VectorFormat) -> i32 {
        assert_ne!(vform, VectorFormat::kFormatUndefined);
        match vform {
            VectorFormat::kFormatB | VectorFormat::kFormat8B | VectorFormat::kFormat16B => 16,
            VectorFormat::kFormatH | VectorFormat::kFormat4H | VectorFormat::kFormat8H => 8,
            VectorFormat::kFormatS | VectorFormat::kFormat2S | VectorFormat::kFormat4S => 4,
            VectorFormat::kFormatD | VectorFormat::kFormat1D | VectorFormat::kFormat2D => 2,
            _ => panic!("Unreachable"),
        }
    }

    /// Checks if the vector format is indeed a vector.
    pub fn is_vector_format(vform: VectorFormat) -> bool {
        assert_ne!(vform, VectorFormat::kFormatUndefined);
        match vform {
            VectorFormat::kFormatB | VectorFormat::kFormatH | VectorFormat::kFormatS
            | VectorFormat::kFormatD => false,
            _ => true,
        }
    }

    /// Returns the maximum integer value for the given format.
    pub fn max_int_from_format(vform: VectorFormat) -> i64 {
        i64::MAX >> (64 - lane_size_in_bits_from_format(vform))
    }

    /// Returns the minimum integer value for the given format.
    pub fn min_int_from_format(vform: VectorFormat) -> i64 {
        i64::MIN >> (64 - lane_size_in_bits_from_format(vform))
    }

    /// Returns the maximum unsigned integer value for the given format.
    pub fn max_uint_from_format(vform: VectorFormat) -> u64 {
        u64::MAX >> (64 - lane_size_in_bits_from_format(vform))
    }
}