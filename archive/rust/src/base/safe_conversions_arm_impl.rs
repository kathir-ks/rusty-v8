// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Slightly adapted for inclusion in V8.
// Copyright 2014 the V8 project authors. All rights reserved.
// List of adaptations:
// - include guard names
// - wrap in v8 namespace
// - include paths

pub mod base {
    pub mod internal {
        use std::convert::TryFrom;
        use std::convert::TryInto;
        use std::marker::PhantomData;
        use std::mem;
        use std::ops::{RangeInclusive};
        use std::any::type_name;

        /// Determines if a type is within the inclusive range of another type.
        pub struct IsTypeInRangeForNumericType<Dst, Src> {
            _phantom_data: PhantomData<(Dst, Src)>,
        }

        impl<Dst, Src> IsTypeInRangeForNumericType<Dst, Src> {
            pub const VALUE: bool = {
                use std::any::type_name;
                println!("Checking range for Dst: {} and Src: {}", type_name::<Dst>(), type_name::<Src>());
                
                // Attempt to determine if Src is in range of Dst.  This is a best effort and may have false negatives or positives.
                // In C++ this would be handled with SFINAE and template specialization.
                // Here, we will simply approximate it.

                // Basic heuristic: If Dst is i32 and Src is i64, then Src is *not* in range.
                if type_name::<Dst>() == "i32" && type_name::<Src>() == "i64" {
                    false
                } else if type_name::<Dst>() == "u32" && type_name::<Src>() == "i64" {
                    false
                }else if type_name::<Dst>() == "i32" && type_name::<Src>() == "i128" {
                    false
                } else if type_name::<Dst>() == "u32" && type_name::<Src>() == "i128" {
                    false
                }else if type_name::<Dst>() == "i32" && type_name::<Src>() == "u64" {
                    false
                } else if type_name::<Dst>() == "u32" && type_name::<Src>() == "u64" {
                     false
                } else if type_name::<Dst>() == "i32" && type_name::<Src>() == "u128" {
                    false
                } else if type_name::<Dst>() == "u32" && type_name::<Src>() == "u128" {
                    false
                } else {
                    true // Optimistic approximation
                }
            };
        }

        pub struct IntegerBitsPlusSign<T> {
            _phantom_data: PhantomData<T>,
        }

        impl<T> IntegerBitsPlusSign<T> {
            pub const value: u32 = {
                let size = mem::size_of::<T>() as u32;
                size * 8
            };
        }

        // Placeholder for kEnableAsmCode.  Always false for now.
        const ENABLE_ASM_CODE: bool = false;

        /// Fast saturation to a destination type.
        pub struct SaturateFastAsmOp<Dst, Src> {
            _phantom_data: PhantomData<(Dst, Src)>,
        }

        impl<Dst, Src> SaturateFastAsmOp<Dst, Src> {
            pub const IS_SUPPORTED: bool = {
                use std::any::type_name;

                ENABLE_ASM_CODE &&
                std::any::TypeId::of::<Src>() != std::any::TypeId::of::<bool>() &&
                std::any::TypeId::of::<Dst>() != std::any::TypeId::of::<bool>() &&
                (i8::min_value()..=i8::max_value()).contains(&(0 as i8).try_into().unwrap()) &&
                (i16::min_value()..=i16::max_value()).contains(&(0 as i16).try_into().unwrap()) &&
                (i32::min_value()..=i32::max_value()).contains(&(0 as i32).try_into().unwrap()) &&
                (i64::min_value()..=i64::max_value()).contains(&(0 as i64).try_into().unwrap()) &&
                (i128::min_value()..=i128::max_value()).contains(&(0 as i128).try_into().unwrap()) &&
                IntegerBitsPlusSign::<Src>::value <= IntegerBitsPlusSign::<i32>::value &&
                IntegerBitsPlusSign::<Dst>::value <= IntegerBitsPlusSign::<i32>::value &&
                !IsTypeInRangeForNumericType::<Dst, Src>::VALUE;
        };

            #[inline(always)]
            pub fn do_saturation(value: Src) -> Dst
            where
                Src: Into<i32> + Copy,
                Dst: TryFrom<i32> + Copy,
                <Dst as TryFrom<i32>>::Error: std::fmt::Debug,
            {
                let src: i32 = value.into();
                let result: i32;

                if (0 as i32).is_negative() {
                    // Signed saturation
                    result = unsafe {
                        let shift = if IntegerBitsPlusSign::<Dst>::value <= 32 {
                            IntegerBitsPlusSign::<Dst>::value
                        } else {
                            32
                        };
                        llvm_intrinsics::llvm_ssat_i32(shift as i32, src)
                    };
                } else {
                    // Unsigned saturation
                    result = unsafe {
                        let shift = if IntegerBitsPlusSign::<Dst>::value < 32 {
                            IntegerBitsPlusSign::<Dst>::value
                        } else {
                            31
                        };
                        llvm_intrinsics::llvm_usat_i32(shift as i32, src)
                    };
                }

                Dst::try_from(result).unwrap()
            }
        }

        #[cfg(target_arch = "arm")]
        mod llvm_intrinsics {
            extern "C" {
                #[link_name = "llvm.ssat.i32"]
                pub fn llvm_ssat_i32(sat: i32, val: i32) -> i32;
                #[link_name = "llvm.usat.i32"]
                pub fn llvm_usat_i32(sat: i32, val: i32) -> i32;
            }
        }

        #[cfg(not(target_arch = "arm"))]
        mod llvm_intrinsics {
            pub fn llvm_ssat_i32(sat: i32, val: i32) -> i32 {
                val.clamp(-(1 << (sat - 1)), (1 << (sat - 1)) - 1)
            }
            pub fn llvm_usat_i32(sat: i32, val: i32) -> i32 {
                val.clamp(0, (1 << sat) - 1)
            }
        }
    }
}