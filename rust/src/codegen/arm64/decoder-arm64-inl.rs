// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod decoder_arm64 {
    use crate::codegen::arm64::decoder_arm64::Instruction;

    pub trait Visitor {
        fn visit_unallocated(&self, instr: &Instruction);
        fn visit_pc_rel_addressing(&self, instr: &Instruction);
        fn visit_unconditional_branch(&self, instr: &Instruction);
        fn visit_compare_branch(&self, instr: &Instruction);
        fn visit_test_branch(&self, instr: &Instruction);
        fn visit_conditional_branch(&self, instr: &Instruction);
        fn visit_exception(&self, instr: &Instruction);
        fn visit_system(&self, instr: &Instruction);
        fn visit_unconditional_branch_to_register(&self, instr: &Instruction);
        fn visit_load_store_acquire_release(&self, instr: &Instruction);
        fn visit_unimplemented(&self, instr: &Instruction);
        fn visit_load_store_pair_post_index(&self, instr: &Instruction);
        fn visit_load_literal(&self, instr: &Instruction);
        fn visit_load_store_unscaled_offset(&self, instr: &Instruction);
        fn visit_load_store_post_index(&self, instr: &Instruction);
        fn visit_load_store_pre_index(&self, instr: &Instruction);
        fn visit_load_store_register_offset(&self, instr: &Instruction);
        fn visit_atomic_memory(&self, instr: &Instruction);
        fn visit_load_store_pair_offset(&self, instr: &Instruction);
        fn visit_load_store_pair_pre_index(&self, instr: &Instruction);
        fn visit_load_store_unsigned_offset(&self, instr: &Instruction);
        fn visit_logical_immediate(&self, instr: &Instruction);
        fn visit_move_wide_immediate(&self, instr: &Instruction);
        fn visit_bitfield(&self, instr: &Instruction);
        fn visit_extract(&self, instr: &Instruction);
        fn visit_add_sub_immediate(&self, instr: &Instruction);
        fn visit_logical_shifted(&self, instr: &Instruction);
        fn visit_add_sub_with_carry(&self, instr: &Instruction);
        fn visit_conditional_compare_register(&self, instr: &Instruction);
        fn visit_conditional_compare_immediate(&self, instr: &Instruction);
        fn visit_conditional_select(&self, instr: &Instruction);
        fn visit_data_processing_2_source(&self, instr: &Instruction);
        fn visit_data_processing_1_source(&self, instr: &Instruction);
        fn visit_add_sub_shifted(&self, instr: &Instruction);
        fn visit_add_sub_extended(&self, instr: &Instruction);
        fn visit_data_processing_3_source(&self, instr: &Instruction);
        fn visit_fp_fixed_point_convert(&self, instr: &Instruction);
        fn visit_fp_integer_convert(&self, instr: &Instruction);
        fn visit_fp_data_processing_1_source(&self, instr: &Instruction);
        fn visit_fp_compare(&self, instr: &Instruction);
        fn visit_fp_immediate(&self, instr: &Instruction);
        fn visit_fp_data_processing_2_source(&self, instr: &Instruction);
        fn visit_fp_conditional_compare(&self, instr: &Instruction);
        fn visit_fp_conditional_select(&self, instr: &Instruction);
        fn visit_fp_data_processing_3_source(&self, instr: &Instruction);
        fn visit_neon_load_store_multi_struct(&self, instr: &Instruction);
        fn visit_neon_load_store_single_struct(&self, instr: &Instruction);
        fn visit_neon_load_store_multi_struct_post_index(&self, instr: &Instruction);
        fn visit_neon_load_store_single_struct_post_index(&self, instr: &Instruction);
        fn visit_neon_table(&self, instr: &Instruction);
        fn visit_neon_perm(&self, instr: &Instruction);
        fn visit_neon_extract(&self, instr: &Instruction);
        fn visit_neon_copy(&self, instr: &Instruction);
        fn visit_neon_3_same_hp(&self, instr: &Instruction);
        fn visit_neon_3_extension(&self, instr: &Instruction);
        fn visit_neon_3_different(&self, instr: &Instruction);
        fn visit_neon_2_reg_misc(&self, instr: &Instruction);
        fn visit_neon_across_lanes(&self, instr: &Instruction);
        fn visit_neon_3_same(&self, instr: &Instruction);
        fn visit_neon_by_indexed_element(&self, instr: &Instruction);
        fn visit_neon_modified_immediate(&self, instr: &Instruction);
        fn visit_neon_shift_immediate(&self, instr: &Instruction);
        fn visit_neon_scalar_copy(&self, instr: &Instruction);
        fn visit_neon_scalar_3_diff(&self, instr: &Instruction);
        fn visit_neon_scalar_2_reg_misc(&self, instr: &Instruction);
        fn visit_neon_scalar_pairwise(&self, instr: &Instruction);
        fn visit_neon_scalar_3_same(&self, instr: &Instruction);
        fn visit_neon_scalar_by_indexed_element(&self, instr: &Instruction);
        fn visit_neon_scalar_shift_immediate(&self, instr: &Instruction);
    }

    pub struct Decoder<V: Visitor> {
        _phantom: std::marker::PhantomData<V>,
    }

    impl<V: Visitor> Decoder<V> {
        pub fn new() -> Self {
            Decoder {
                _phantom: std::marker::PhantomData,
            }
        }

        /// Top-level instruction decode function.
        pub fn decode(&self, instr: &mut Instruction, visitor: &V) {
            if instr.bits(28, 27) == 0 {
                visitor.visit_unallocated(instr);
            } else {
                match instr.bits(27, 24) {
                    // 0:   PC relative addressing.
                    0x0 => self.decode_pc_rel_addressing(instr, visitor),

                    // 1:   Add/sub immediate.
                    0x1 => self.decode_add_sub_immediate(instr, visitor),

                    // A:   Logical shifted register.
                    //      Add/sub with carry.
                    //      Conditional compare register.
                    //      Conditional compare immediate.
                    //      Conditional select.
                    //      Data processing 1 source.
                    //      Data processing 2 source.
                    // B:   Add/sub shifted register.
                    //      Add/sub extended register.
                    //      Data processing 3 source.
                    0xA | 0xB => self.decode_data_processing(instr, visitor),

                    // 2:   Logical immediate.
                    //      Move wide immediate.
                    0x2 => self.decode_logical(instr, visitor),

                    // 3:   Bitfield.
                    //      Extract.
                    0x3 => self.decode_bitfield_extract(instr, visitor),

                    // 4:   Unconditional branch immediate.
                    //      Exception generation.
                    //      Compare and branch immediate.
                    // 5:   Compare and branch immediate.
                    //      Conditional branch.
                    //      System.
                    // 6,7: Unconditional branch.
                    //      Test and branch immediate.
                    0x4 | 0x5 | 0x6 | 0x7 => self.decode_branch_system_exception(instr, visitor),

                    // 8,9: Load/store register pair post-index.
                    //      Load register literal.
                    //      Load/store register unscaled immediate.
                    //      Load/store register immediate post-index.
                    //      Load/store register immediate pre-index.
                    //      Load/store register offset.
                    //      Load/store exclusive.
                    //      Load/store ordered.
                    //      Compare and swap [Armv8.1].
                    //      Compare and swap pair [Armv8.1].
                    //      Atomic memory operations [Armv8.1].
                    // C,D: Load/store register pair offset.
                    //      Load/store register pair pre-index.
                    //      Load/store register unsigned immediate.
                    //      Advanced SIMD.
                    0x8 | 0x9 | 0xC | 0xD => self.decode_load_store(instr, visitor),

                    // E:   FP fixed point conversion.
                    //      FP integer conversion.
                    //      FP data processing 1 source.
                    //      FP compare.
                    //      FP immediate.
                    //      FP data processing 2 source.
                    //      FP conditional compare.
                    //      FP conditional select.
                    //      Advanced SIMD.
                    // F:   FP data processing 3 source.
                    //      Advanced SIMD.
                    0xE | 0xF => self.decode_fp(instr, visitor),

                    _ => unreachable!(),
                }
            }
        }

        fn decode_pc_rel_addressing(&self, instr: &mut Instruction, visitor: &V) {
            debug_assert_eq!(0x0, instr.bits(27, 24));
            // We know bit 28 is set, as <b28:b27> = 0 is filtered out at the top level
            // decode.
            debug_assert_eq!(0x1, instr.bit(28));
            visitor.visit_pc_rel_addressing(instr);
        }

        fn decode_branch_system_exception(&self, instr: &mut Instruction, visitor: &V) {
            debug_assert_eq!(0x4, instr.bits(27, 24) & 0xC); // 0x4, 0x5, 0x6, 0x7

            match instr.bits(31, 29) {
                0 | 4 => {
                    visitor.visit_unconditional_branch(instr);
                }
                1 | 5 => {
                    if instr.bit(25) == 0 {
                        visitor.visit_compare_branch(instr);
                    } else {
                        visitor.visit_test_branch(instr);
                    }
                }
                2 => {
                    if instr.bit(25) == 0 {
                        if (instr.bit(24) == 0x1)
                            || (instr.mask(0x0100_0010) == 0x0000_0010)
                        {
                            visitor.visit_unallocated(instr);
                        } else {
                            visitor.visit_conditional_branch(instr);
                        }
                    } else {
                        visitor.visit_unallocated(instr);
                    }
                }
                6 => {
                    if instr.bit(25) == 0 {
                        if instr.bit(24) == 0 {
                            if (instr.bits(4, 2) != 0)
                                || (instr.mask(0x00E0_001D) == 0x0020_0001)
                                || (instr.mask(0x00E0_001D) == 0x0040_0001)
                                || (instr.mask(0x00E0_001E) == 0x0020_0002)
                                || (instr.mask(0x00E0_001E) == 0x0040_0002)
                                || (instr.mask(0x00E0_001C) == 0x0060_0000)
                                || (instr.mask(0x00E0_001C) == 0x0080_0000)
                                || (instr.mask(0x00E0_001F) == 0x00A0_0000)
                                || (instr.mask(0x00C0_001C) == 0x00C0_0000)
                            {
                                visitor.visit_unallocated(instr);
                            } else {
                                visitor.visit_exception(instr);
                            }
                        } else {
                            if instr.bits(23, 22) == 0 {
                                let masked_003FF0E0 = instr.mask(0x003F_F0E0);
                                if (instr.bits(21, 19) == 0x4)
                                    || (masked_003FF0E0 == 0x0003_3000)
                                    || (masked_003FF0E0 == 0x003F_F020)
                                    || (masked_003FF0E0 == 0x003F_F060)
                                    || (masked_003FF0E0 == 0x003F_F0E0)
                                    || (instr.mask(0x0038_8000) == 0x0000_8000)
                                    || (instr.mask(0x0038_E000) == 0x0000_0000)
                                    || (instr.mask(0x0039_E000) == 0x0000_2000)
                                    || (instr.mask(0x003A_E000) == 0x0000_2000)
                                    || (instr.mask(0x003C_E000) == 0x0004_2000)
                                    || (instr.mask(0x0038_F000) == 0x0000_5000)
                                    || (instr.mask(0x0038_E000) == 0x0000_6000)
                                {
                                    visitor.visit_unallocated(instr);
                                } else {
                                    visitor.visit_system(instr);
                                }
                            } else {
                                visitor.visit_unallocated(instr);
                            }
                        }
                    } else {
                        if (instr.bit(24) == 0x1)
                            || (instr.bits(20, 16) != 0x1F)
                            || (instr.bits(15, 10) != 0)
                            || (instr.bits(4, 0) != 0)
                            || (instr.bits(24, 21) == 0x3)
                            || (instr.bits(24, 22) == 0x3)
                        {
                            visitor.visit_unallocated(instr);
                        } else {
                            visitor.visit_unconditional_branch_to_register(instr);
                        }
                    }
                }
                3 | 7 => {
                    visitor.visit_unallocated(instr);
                }
                _ => {}
            }
        }

        fn decode_load_store(&self, instr: &mut Instruction, visitor: &V) {
            debug_assert_eq!(0x8, instr.bits(27, 24) & 0xA); // 0x8, 0x9, 0xC, 0xD

            if (instr.bit(28) == 0) && (instr.bit(29) == 0) && (instr.bit(26) == 1) {
                self.decode_neon_load_store(instr, visitor);
                return;
            }

            if instr.bit(24) == 0 {
                if instr.bit(28) == 0 {
                    if instr.bit(29) == 0 {
                        if instr.bit(26) == 0 {
                            if instr.mask(0xA080_00) == 0x8000_00 {
                                visitor.visit_unallocated(instr);
                            } else if instr.mask(0xA080_00) == 0 {
                                // Load/Store exclusive without acquire/release are unimplemented.
                                visitor.visit_unimplemented(instr);
                            } else {
                                visitor.visit_load_store_acquire_release(instr);
                            }
                        } else {
                            // This is handled by DecodeNEONLoadStore().
                            unreachable!();
                        }
                    } else {
                        if (instr.bits(31, 30) == 0x3)
                            || (instr.mask(0xC440_0000) == 0x4000_0000)
                        {
                            visitor.visit_unallocated(instr);
                        } else {
                            if instr.bit(23) == 0 {
                                if instr.mask(0xC440_0000) == 0xC040_0000 {
                                    visitor.visit_unallocated(instr);
                                } else {
                                    // Nontemporals are unimplemented.
                                    visitor.visit_unimplemented(instr);
                                }
                            } else {
                                visitor.visit_load_store_pair_post_index(instr);
                            }
                        }
                    }
                } else {
                    if instr.bit(29) == 0 {
                        if instr.mask(0xC400_0000) == 0xC400_0000 {
                            visitor.visit_unallocated(instr);
                        } else {
                            visitor.visit_load_literal(instr);
                        }
                    } else {
                        if (instr.mask(0x4480_0000) == 0x4480_0000)
                            || (instr.mask(0x8480_0000) == 0x8480_0000)
                        {
                            visitor.visit_unallocated(instr);
                        } else {
                            if instr.bit(21) == 0 {
                                match instr.bits(11, 10) {
                                    0 => {
                                        visitor.visit_load_store_unscaled_offset(instr);
                                    }
                                    1 => {
                                        if instr.mask(0xC4C0_0000) == 0xC080_0000 {
                                            visitor.visit_unallocated(instr);
                                        } else {
                                            visitor.visit_load_store_post_index(instr);
                                        }
                                    }
                                    2 => {
                                        // TODO(all): VisitLoadStoreRegisterOffsetUnpriv.
                                        visitor.visit_unimplemented(instr);
                                    }
                                    3 => {
                                        if instr.mask(0xC4C0_0000) == 0xC080_0000 {
                                            visitor.visit_unallocated(instr);
                                        } else {
                                            visitor.visit_load_store_pre_index(instr);
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                if instr.bits(11, 10) == 0x2 {
                                    if instr.bit(14) == 0 {
                                        visitor.visit_unallocated(instr);
                                    } else {
                                        visitor.visit_load_store_register_offset(instr);
                                    }
                                } else {
                                    if (instr.bits(11, 10) == 0x0) && (instr.bits(26, 25) == 0x0)
                                    {
                                        if (instr.bit(15) == 1)
                                            && ((instr.bits(14, 12) == 0x1)
                                                || (instr.bit(13) == 1)
                                                || (instr.bits(14, 12) == 0x5)
                                                || ((instr.bits(14, 12) == 0x4)
                                                    && ((instr.bit(23) == 0)
                                                        || (instr.bits(23, 22) == 0x3))))
                                        {
                                            visitor.visit_unallocated(instr);
                                        } else {
                                            visitor.visit_atomic_memory(instr);
                                        }
                                    } else {
                                        visitor.visit_unallocated(instr);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if instr.bit(28) == 0 {
                    if instr.bit(29) == 0 {
                        visitor.visit_unallocated(instr);
                    } else {
                        if (instr.bits(31, 30) == 0x3)
                            || (instr.mask(0xC440_0000) == 0x4000_0000)
                        {
                            visitor.visit_unallocated(instr);
                        } else {
                            if instr.bit(23) == 0 {
                                visitor.visit_load_store_pair_offset(instr);
                            } else {
                                visitor.visit_load_store_pair_pre_index(instr);
                            }
                        }
                    }
                } else {
                    if instr.bit(29) == 0 {
                        visitor.visit_unallocated(instr);
                    } else {
                        if (instr.mask(0x84C0_0000) == 0x80C0_0000)
                            || (instr.mask(0x4480_0000) == 0x4480_0000)
                            || (instr.mask(0x8480_0000) == 0x8480_0000)
                        {
                            visitor.visit_unallocated(instr);
                        } else {
                            visitor.visit_load_store_unsigned_offset(instr);
                        }
                    }
                }
            }
        }

        fn decode_logical(&self, instr: &mut Instruction, visitor: &V) {
            debug_assert_eq!(0x2, instr.bits(27, 24));

            if instr.mask(0x8040_0000) == 0x0040_0000 {
                visitor.visit_unallocated(instr);
            } else {
                if instr.bit(23) == 0 {
                    visitor.visit_logical_immediate(instr);
                } else {
                    if instr.bits(30, 29) == 0x1 {
                        visitor.visit_unallocated(instr);
                    } else {
                        visitor.visit_move_wide_immediate(instr);
                    }
                }
            }
        }

        fn decode_bitfield_extract(&self, instr: &mut Instruction, visitor: &V) {
            debug_assert_eq!(0x3, instr.bits(27, 24));

            if (instr.mask(0x8040_0000) == 0x8000_0000)
                || (instr.mask(0x8040_0000) == 0x0040_0000)
                || (instr.mask(0x8000_8000) == 0x0000_8000)
            {
                visitor.visit_unallocated(instr);
            } else if instr.bit(23) == 0 {
                if (instr.mask(0x8020_0000) == 0x0020_0000)
                    || (instr.mask(0x6000_0000) == 0x6000_0000)
                {
                    visitor.visit_unallocated(instr);
                } else {
                    visitor.visit_bitfield(instr);
                }
            } else {
                if (instr.mask(0x6020_0000) == 0x0020_0000)
                    || (instr.mask(0x6000_0000) != 0x0000_0000)
                {
                    visitor.visit_unallocated(instr);
                } else {
                    visitor.visit_extract(instr);
                }
            }
        }

        fn decode_add_sub_immediate(&self, instr: &mut Instruction, visitor: &V) {
            debug_assert_eq!(0x1, instr.bits(27, 24));
            if instr.bit(23) == 1 {
                visitor.visit_unallocated(instr);
            } else {
                visitor.visit_add_sub_immediate(instr);
            }
        }

        fn decode_data_processing(&self, instr: &mut Instruction, visitor: &V) {
            debug_assert!((instr.bits(27, 24) == 0xA) || (instr.bits(27, 24) == 0xB));

            if instr.bit(24) == 0 {
                if instr.bit(28) == 0 {
                    if instr.mask(0x8000_8000) == 0x0000_8000 {
                        visitor.visit_unallocated(instr);
                    } else {
                        visitor.visit_logical_shifted(instr);
                    }
                } else {
                    match instr.bits(23, 21) {
                        0 => {
                            if instr.mask(0x0000_FC00) != 0 {
                                visitor.visit_unallocated(instr);
                            } else {
                                visitor.visit_add_sub_with_carry(instr);
                            }
                        }
                        2 => {
                            if (instr.bit(29) == 0) || (instr.mask(0x0000_0410) != 0) {
                                visitor.visit_unallocated(instr);
                            } else {
                                if instr.bit(11) == 0 {
                                    visitor.visit_conditional_compare_register(instr);
                                } else {
                                    visitor.visit_conditional_compare_immediate(instr);
                                }
                            }
                        }
                        4 => {
                            if instr.mask(0x2000_0800) != 0x0000_0000 {
                                visitor.visit_unallocated(instr);
                            } else {
                                visitor.visit_conditional_select(instr);
                            }
                        }
                        6 => {
                            if instr.bit(29) == 0x1 {
                                visitor.visit_unallocated(instr);
                            } else {
                                if instr.bit(30) == 0 {
                                    if (instr.bit(15) == 0x1)
                                        || (instr.bits(15, 11) == 0)
                                        || (instr.bits(15, 12) == 0x1)
                                        || (instr.bits(15, 12) == 0x3)
                                        || (instr.bits(15, 13) == 0x3)
                                        || (instr.mask(0x8000_EC00) == 0x0000_4C00)
                                        || (instr.mask(0x8000_E800) == 0x8000_4000)
                                        || (instr.mask(0x8000_E400) == 0x8000_4000)
                                    {
                                        visitor.visit_unallocated(instr);
                                    } else {
                                        visitor.visit_data_processing_2_source(instr);
                                    }
                                } else {
                                    if (instr.bit(13) == 1)
                                        || (instr.bits(20, 16) != 0)
                                        || (instr.bits(15, 14) != 0)
                                        || (instr.mask(0xA01F_FC00) == 0x0000_0C00)
                                        || (instr.mask(0x201F_F800) == 0x0000_1800)
                                    {
                                        visitor.visit_unallocated(instr);
                                    } else {
                                        visitor.visit_data_processing_1_source(instr);
                                    }
                                }
                            }
                        }
                        1 | 3 | 5 | 7 => visitor.visit_unallocated(instr),
                        _ => {}
                    }
                }
            } else {
                if instr.bit(28) == 0 {
                    if instr.bit(21) == 0 {
                        if (instr.bits(23, 22) == 0x3)
                            || (instr.mask(0x8000_8000) == 0x0000_8000)
                        {
                            visitor.visit_unallocated(instr);
                        } else {
                            visitor.visit_add_sub_shifted(instr);
                        }
                    } else {
                        if (instr.mask(0x00C0_0000) != 0x0000_0000)
                            || (instr.mask(0x0000_1400) == 0x0000_1400)
                            || (instr.mask(0x0000_1800) == 0x0000_1800)
                        {
                            visitor.visit_unallocated(instr);
                        } else {
                            visitor.visit_add_sub_extended(instr);
                        }
                    }
                } else {
                    if (instr.bit(30) == 0x1)
                        || (instr.bits(30, 29) == 0x1)
                        || (instr.mask(0xE060_0000) == 0x0020_0000)
                        || (instr.mask(0xE060_8000) == 0x0040_0000)
                        || (instr.mask(0x6060_8000) == 0x0040_8000)
                        || (instr.mask(0x60E0_0000) == 0x00E0_0000)
                        || (instr.mask(0x60E0_0000) == 0x0080_0000)
                        || (instr.mask(0x60E0_0000) == 0x0060_0000)
                    {
                        visitor.visit_unallocated(instr);
                    } else {
                        visitor.visit_data_processing_3_source(instr);
                    }
                }
            }
        }

        fn decode_fp(&self, instr: &mut Instruction, visitor: &V) {
            debug_assert!((instr.bits(27, 24) == 0xE) || (instr.bits(27, 24) == 0xF));

            if instr.bit(28) == 0 {
                self.decode_neon_vector_data_processing(instr, visitor);
            } else {
                if instr.bits(31, 30) == 0x3 {
                    visitor.visit_unallocated(instr);
                } else if instr.bits(31, 30) == 0x1 {
                    self.decode_neon_scalar_data_processing(instr, visitor);
                } else {
                    if instr.bit(29) == 0 {
                        if instr.bit(24) == 0 {
                            if instr.bit(21) == 0 {
                                if (instr.bit(23) == 1)
                                    || (instr.bit(18) == 1)
                                    || (instr.mask(0x8000_8000) == 0x0000_0000)
                                    || (instr.mask(0x000E_0000) == 0x0000_0000)
                                    || (instr.mask(0x000E_0000) == 0x000A_0000)
                                    || (instr.mask(0x0016_0000) == 0x0000_0000)
                                    || (instr.mask(0x0016_0000) == 0x0012_0000)
                                {
                                    visitor.visit_unallocated(instr);
                                } else {
                                    visitor.visit_fp_fixed_point_convert(instr);
                                }
                            } else {
                                if instr.bits(15, 10) == 32 {
                                    visitor.visit_unallocated(instr);
                                } else if instr.bits(15, 10) == 0 {
                                    if (instr.bits(23, 22) == 0x3)
                                        || (instr.mask(0x000E_0000) == 0x000A_0000)
                                        || (instr.mask(0x000E_0000) == 0x000C_0000)
                                        || (instr.mask(0x0016_0000) == 0x0012_0000)
                                        || (instr