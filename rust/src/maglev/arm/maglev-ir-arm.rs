// src/maglev/arm/maglev-ir-arm.rs

//use crate::base::logging::*; // Logging is handled by the log crate in Rust
use crate::codegen::arm::assembler_arm::*;
use crate::codegen::arm::register_arm::*;
use crate::maglev::arm::maglev_assembler_arm::*;
use crate::maglev::maglev_assembler::*;
use crate::maglev::maglev_graph_processor::*;
use crate::maglev::maglev_graph::*;
use crate::maglev::maglev_ir::*;
//use crate::objects::string::String; // String representation

pub mod maglev {
    pub mod internal {
        pub mod maglev {
            use crate::codegen::arm::assembler_arm::Assembler;
            use crate::codegen::arm::register_arm::*;
            use crate::maglev::arm::maglev_assembler_arm::*;
            use crate::maglev::maglev_assembler::*;
            use crate::maglev::maglev_graph_processor::*;
            use crate::maglev::maglev_graph::*;
            use crate::maglev::maglev_ir::*;
            //use crate::objects::string::String;
            use crate::codegen::arm::cpu_features::CpuFeatures;
            use crate::codegen::arm::cpu_features::SupportedFeatures::*;
            use crate::codegen::arm::register_arm::VRegister;
            use crate::codegen::arm::register_arm::SRegister;
            use std::ops::BitAnd;

            macro_rules! check_reglist_empty {
                ($reglist:expr, $eager_deopt_info:expr) => {
                    if !$reglist.is_empty() {
                        // panic!("Register list should be empty");
                        // TODO: Replace with a more appropriate error handling mechanism
                    }
                };
            }

            // Implementations for each node type

            impl Int32NegateWithOverflow {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.value_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let value = self.value_input().to_register();
                    let out = self.result().to_register();

                    // Deopt when result would be -0.
                    masm.cmp(value, 0.into());
                    masm.emit_eager_deopt_if(Condition::Eq, DeoptimizeReason::kOverflow, self);

                    masm.rsb(out, value, 0.into(), true);
                    // Output register must not be a register input into the eager deopt info.
                    // DCHECK_REGLIST_EMPTY(RegList{out} &
                    //                    GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
                    let reglist = RegList::from(out);
                    //let general_registers_used_as_inputs = self.eager_deopt_info().general_registers_used_as_inputs();
                    //check_reglist_empty!(reglist, general_registers_used_as_inputs);

                    masm.emit_eager_deopt_if(Condition::Vs, DeoptimizeReason::kOverflow, self);
                }
            }

            impl Int32AbsWithOverflow {
                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let out = self.result().to_register();
                    let done = Label::new();
                    masm.cmp(out, 0.into());
                    masm.jump_if(Condition::Ge, &done);
                    masm.rsb(out, out, 0.into(), true);
                    // Output register must not be a register input into the eager deopt info.
                    // DCHECK_REGLIST_EMPTY(RegList{out} &
                    //                    GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
                    let reglist = RegList::from(out);
                    //let general_registers_used_as_inputs = self.eager_deopt_info().general_registers_used_as_inputs();
                    //check_reglist_empty!(reglist, general_registers_used_as_inputs);

                    masm.emit_eager_deopt_if(Condition::Vs, DeoptimizeReason::kOverflow, self);
                    masm.bind(&done);
                }
            }

            impl Int32IncrementWithOverflow {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.value_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let value = self.value_input().to_register();
                    let out = self.result().to_register();
                    masm.add(out, value, 1.into(), true);
                    // Output register must not be a register input into the eager deopt info.
                    //DCHECK_REGLIST_EMPTY(RegList{out} & GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
                    let reglist = RegList::from(out);
                    //let general_registers_used_as_inputs = self.eager_deopt_info().general_registers_used_as_inputs();
                    //check_reglist_empty!(reglist, general_registers_used_as_inputs);
                    masm.emit_eager_deopt_if(Condition::Vs, DeoptimizeReason::kOverflow, self);
                }
            }

            impl Int32DecrementWithOverflow {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.value_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let value = self.value_input().to_register();
                    let out = self.result().to_register();
                    masm.sub(out, value, 1.into(), true);
                    // Output register must not be a register input into the eager deopt info.
                    //DCHECK_REGLIST_EMPTY(RegList{out} & GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
                    let reglist = RegList::from(out);
                    //let general_registers_used_as_inputs = self.eager_deopt_info().general_registers_used_as_inputs();
                    //check_reglist_empty!(reglist, general_registers_used_as_inputs);
                    masm.emit_eager_deopt_if(Condition::Vs, DeoptimizeReason::kOverflow, self);
                }
            }

            impl BuiltinStringFromCharCode {
                pub const fn max_call_stack_args() -> i32 {
                    AllocateDescriptor::get_stack_parameter_count()
                }
                pub fn set_value_location_constraints(&mut self) {
                    if self.code_input().node().is::<Int32Constant>() {
                        self.use_any(self.code_input());
                    } else {
                        self.use_and_clobber_register(self.code_input());
                    }
                    self.set_temporaries_needed(1);
                    self.define_as_register(self);
                }
                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let mut temps = MaglevAssembler::TemporaryRegisterScope::new(masm);
                    let scratch = temps.acquire();
                    let result_string = self.result().to_register();
                    if let Some(constant) = self.code_input().node().try_cast::<Int32Constant>() {
                        let char_code = (constant.value() & 0xFFFF) as i32;
                        if (0 <= char_code) && (char_code < StringConstants::kMaxOneByteCharCode as i32) {
                            masm.load_single_character_string(result_string, char_code);
                        } else {
                            assert_ne!(scratch, result_string);
                            masm.allocate_two_byte_string(self.register_snapshot(), result_string, 1);
                            masm.move_(scratch, char_code);
                            //masm.strh(scratch, FieldMemOperand(result_string, OFFSET_OF_DATA_START(SeqTwoByteString)));
                            masm.store_halfword(scratch, MemOperand::new(result_string, TwoByteStringConstants::kDataOffset as i32));

                        }
                    } else {
                        masm.string_from_char_code(self.register_snapshot(), None, result_string, self.code_input().to_register(), scratch, MaglevAssembler::CharCodeMaskMode::kMustApplyMask);
                    }
                }
            }

            impl InlinedAllocation {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.allocation_block_input());
                    if self.offset() == 0 {
                        self.define_same_as_first(self);
                    } else {
                        self.define_as_register(self);
                    }
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    if self.offset() != 0 {
                        masm.add(self.result().to_register(), self.allocation_block_input().to_register(), self.offset().into());
                    }
                }
            }

            impl ArgumentsLength {
                pub fn set_value_location_constraints(&mut self) {
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let argc = self.result().to_register();
                    masm.ldr(argc, MemOperand::new(fp, StandardFrameConstants::kArgCOffset));
                    masm.sub(argc, argc, 1.into()); // Remove receiver.
                }
            }

            impl RestLength {
                pub fn set_value_location_constraints(&mut self) {
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let length = self.result().to_register();
                    let done = Label::new();
                    masm.ldr(length, MemOperand::new(fp, StandardFrameConstants::kArgCOffset));
                    masm.sub(length, length, (self.formal_parameter_count() + 1).into(), true);
                    masm.b(Condition::GreaterThanEqual, &done);
                    masm.move_(length, 0);
                    masm.bind(&done);
                    masm.unchecked_smi_tag_int32(length);
                }
            }

            impl CheckedObjectToIndex {
                pub const fn max_call_stack_args() -> i32 {
                    0
                }
            }

            impl CheckedIntPtrToInt32 {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.input());
                    self.define_same_as_first(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    // On 32-bit platforms, IntPtr is the same as Int32.
                }
            }

            impl Int32AddWithOverflow {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.left_input());
                    self.use_register(self.right_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let left = self.left_input().to_register();
                    let right = self.right_input().to_register();
                    let out = self.result().to_register();
                    masm.add(out, left, right, true);
                    // The output register shouldn't be a register input into the eager deopt
                    // info.
                    //DCHECK_REGLIST_EMPTY(RegList{out} & GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
                    let reglist = RegList::from(out);
                    //let general_registers_used_as_inputs = self.eager_deopt_info().general_registers_used_as_inputs();
                    //check_reglist_empty!(reglist, general_registers_used_as_inputs);
                    masm.emit_eager_deopt_if(Condition::Vs, DeoptimizeReason::kOverflow, self);
                }
            }

            impl Int32SubtractWithOverflow {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.left_input());
                    self.use_register(self.right_input());
                    self.define_as_register(self);
                }
                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let left = self.left_input().to_register();
                    let right = self.right_input().to_register();
                    let out = self.result().to_register();
                    masm.sub(out, left, right, true);
                    // The output register shouldn't be a register input into the eager deopt
                    // info.
                    //DCHECK_REGLIST_EMPTY(RegList{out} & GetGeneralRegistersUsedAsInputs(eager_deopt_info()));
                    let reglist = RegList::from(out);
                    //let general_registers_used_as_inputs = self.eager_deopt_info().general_registers_used_as_inputs();
                    //check_reglist_empty!(reglist, general_registers_used_as_inputs);
                    masm.emit_eager_deopt_if(Condition::Vs, DeoptimizeReason::kOverflow, self);
                }
            }

            impl Int32MultiplyWithOverflow {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.left_input());
                    self.use_register(self.right_input());
                    self.define_as_register(self);
                }
                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let left = self.left_input().to_register();
                    let right = self.right_input().to_register();
                    let out = self.result().to_register();

                    // TODO(leszeks): peephole optimise multiplication by a constant.

                    let mut temps = MaglevAssembler::TemporaryRegisterScope::new(masm);
                    let out_alias_input = out == left || out == right;
                    let mut res_low = out;
                    if out_alias_input {
                        res_low = temps.acquire_scratch();
                    }
                    let res_high = temps.acquire_scratch();
                    masm.smull(res_low, res_high, left, right);

                    // ARM doesn't set the overflow flag for multiplication, so we need to
                    // test on kNotEqual.
                    masm.cmp(res_high, res_low.asr(31));
                    masm.emit_eager_deopt_if(Condition::Ne, DeoptimizeReason::kOverflow, self);

                    // If the result is zero, check if either lhs or rhs is negative.
                    let end = Label::new();
                    masm.tst(res_low, res_low);
                    masm.b(Condition::Ne, &end);
                    let temp = res_high;
                    masm.orr(temp, left, right, true);
                    // If one of them is negative, we must have a -0 result, which is non-int32,
                    // so deopt.
                    masm.emit_eager_deopt_if(Condition::Mi, DeoptimizeReason::kOverflow, self);

                    masm.bind(&end);
                    if out_alias_input {
                        masm.move_(out, res_low);
                    }
                }
            }

            impl Int32DivideWithOverflow {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.left_input());
                    self.use_register(self.right_input());
                    self.define_as_register(self);
                }
                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let left = self.left_input().to_register();
                    let right = self.right_input().to_register();
                    let out = self.result().to_register();

                    // TODO(leszeks): peephole optimise division by a constant.

                    // Pre-check for overflow, since idiv throws a division exception on overflow
                    // rather than setting the overflow flag. Logic copied from
                    // effect-control-linearizer.cc

                    // Check if {right} is positive (and not zero).
                    masm.cmp(right, 0.into());
                    let done = ZoneLabelRef::new(masm);
                    masm.jump_to_deferred_if(
                        Condition::LessThanEqual,
                        |masm: &mut MaglevAssembler, done: ZoneLabelRef, left: Register,
                         right: Register, node: &Int32DivideWithOverflow| {
                            // {right} is negative or zero.

                            // TODO(leszeks): Using kNotInt32 here, but in same places
                            // kDivisionByZerokMinusZero/kMinusZero/kOverflow would be better. Right
                            // now all eager deopts in a node have to be the same -- we should allow
                            // a node to emit multiple eager deopts with different reasons.
                            let deopt = masm.get_deopt_label(node, DeoptimizeReason::kNotInt32);

                            // Check if {right} is zero.
                            // We've already done the compare and flags won't be cleared yet.
                            masm.jump_if(Condition::Eq, deopt);

                            // Check if {left} is zero, as that would produce minus zero.
                            masm.tst(left, left);
                            masm.jump_if(Condition::Eq, deopt);

                            // Check if {left} is kMinInt and {right} is -1, in which case we'd have
                            // to return -kMinInt, which is not representable as Int32.
                            masm.cmp(left, kMinInt.into());
                            masm.jump_if(Condition::Ne, done.borrow());
                            masm.cmp(right, (-1).into());
                            masm.jump_if(Condition::Ne, done.borrow());
                            masm.jump_to_deopt(deopt);
                        },
                        done,
                        left,
                        right,
                        self,
                    );
                    masm.bind(done.borrow());

                    // Perform the actual integer division.
                    let mut temps = MaglevAssembler::TemporaryRegisterScope::new(masm);
                    let out_alias_input = out == left || out == right;
                    let mut res = out;
                    if out_alias_input {
                        res = temps.acquire_scratch();
                    }
                    if CpuFeatures::is_supported(SUDIV) {
                        let mut scope = CpuFeatureScope::new(masm, SUDIV);
                        masm.sdiv(res, left, right);
                    } else {
                        let mut temps2 = UseScratchRegisterScope::new(masm);
                        let double_right = temps2.acquire_low_d();
                        let tmp = double_right.low();
                        let double_left = temps2.acquire_d();
                        let double_res = double_left;
                        masm.vmov(tmp, left);
                        masm.vcvt_f64_s32(double_left, tmp);
                        masm.vmov(tmp, right);
                        masm.vcvt_f64_s32(double_right, tmp);
                        masm.vdiv(double_res, double_left, double_right);
                        masm.vcvt_s32_f64(tmp, double_res);
                        masm.vmov(res, tmp);
                    }

                    // Check that the remainder is zero.
                    let temp = temps.acquire_scratch();
                    masm.mul(temp, res, right);
                    masm.cmp(temp, left);
                    masm.emit_eager_deopt_if(Condition::Ne, DeoptimizeReason::kNotInt32, self);

                    masm.move_(out, res);
                }
            }

            mod uint32_mod {
                use crate::codegen::arm::assembler_arm::Assembler;
                use crate::codegen::arm::cpu_features::CpuFeatures;
                use crate::codegen::arm::cpu_features::SupportedFeatures::*;
                use crate::codegen::arm::register_arm::Register;
                use crate::codegen::arm::register_arm::VRegister;
                use crate::maglev::arm::maglev_assembler_arm::*;
                use crate::maglev::maglev_assembler::*;

                pub fn uint32_mod(masm: &mut MaglevAssembler, out: Register, left: Register, right: Register) {
                    let mut temps = MaglevAssembler::TemporaryRegisterScope::new(masm);
                    let res = temps.acquire_scratch();
                    if CpuFeatures::is_supported(SUDIV) {
                        let mut scope = CpuFeatureScope::new(masm, SUDIV);
                        masm.udiv(res, left, right);
                    } else {
                        let mut temps2 = UseScratchRegisterScope::new(masm);
                        let double_right = temps2.acquire_low_d();
                        let tmp = double_right.low();
                        let double_left = temps2.acquire_d();
                        let double_res = double_left;
                        masm.vmov(tmp, left);
                        masm.vcvt_f64_s32(double_left, tmp);
                        masm.vmov(tmp, right);
                        masm.vcvt_f64_s32(double_right, tmp);
                        masm.vdiv(double_res, double_left, double_right);
                        masm.vcvt_s32_f64(tmp, double_res);
                        masm.vmov(res, tmp);
                    }
                    if CpuFeatures::is_supported(ARMv7) {
                        masm.mls(out, res, right, left);
                    } else {
                        masm.mul(res, res, right);
                        masm.sub(out, left, res);
                    }
                }
            }

            impl Int32ModulusWithOverflow {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_and_clobber_register(self.left_input());
                    self.use_and_clobber_register(self.right_input());
                    self.define_as_register(self);
                }
                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    // If AreAliased(lhs, rhs):
                    //   deopt if lhs < 0  // Minus zero.
                    //   0
                    //
                    // Using same algorithm as in EffectControlLinearizer:
                    //   if rhs <= 0 then
                    //     rhs = -rhs
                    //     deopt if rhs == 0
                    //   if lhs < 0 then
                    //     let lhs_abs = -lsh in
                    //     let res = lhs_abs % rhs in
                    //     deopt if res == 0
                    //     -res
                    //   else
                    //     let msk = rhs - 1 in
                    //     if rhs & msk == 0 then
                    //       lhs & msk
                    //     else
                    //       lhs % rhs

                    let lhs = self.left_input().to_register();
                    let rhs = self.right_input().to_register();
                    let out = self.result().to_register();

                    const DEOPT_REASON: DeoptimizeReason = DeoptimizeReason::kDivisionByZero;

                    if lhs == rhs {
                        // For the modulus algorithm described above, lhs and rhs must not alias
                        // each other.
                        masm.tst(lhs, lhs);
                        // TODO(victorgomes): This ideally should be kMinusZero, but Maglev only
                        // allows one deopt reason per IR.
                        masm.emit_eager_deopt_if(Condition::Mi, DEOPT_REASON, self);
                        masm.move_(self.result().to_register(), 0);
                        return;
                    }

                    assert_ne!(lhs, rhs);

                    let done = ZoneLabelRef::new(masm);
                    let rhs_checked = ZoneLabelRef::new(masm);
                    masm.cmp(rhs, 0.into());
                    masm.jump_to_deferred_if(
                        Condition::LessThanEqual,
                        |masm: &mut MaglevAssembler, rhs_checked: ZoneLabelRef, rhs: Register,
                         node: &Int32ModulusWithOverflow| {
                            masm.rsb(rhs, rhs, 0.into(), true);
                            masm.b(Condition::Ne, rhs_checked.borrow());
                            masm.emit_eager_deopt(node, DEOPT_REASON);
                        },
                        rhs_checked,
                        rhs,
                        self,
                    );
                    masm.bind(rhs_checked.borrow());

                    masm.cmp(lhs, 0.into());
                    masm.jump_to_deferred_if(
                        Condition::LessThan,
                        |masm: &mut MaglevAssembler, done: ZoneLabelRef, lhs: Register,
                         rhs: Register, out: Register, node: &Int32ModulusWithOverflow| {
                            masm.rsb(lhs, lhs, 0.into());
                            uint32_mod::uint32_mod(masm, out, lhs, rhs);
                            masm.rsb(out, out, 0.into(), true);
                            // TODO(victorgomes): This ideally should be kMinusZero, but Maglev
                            // only allows one deopt reason per IR.
                            masm.b(Condition::Ne, done.borrow());
                            masm.emit_eager_deopt(node, DEOPT_REASON);
                        },
                        done,
                        lhs,
                        rhs,
                        out,
                        self,
                    );

                    let rhs_not_power_of_2 = Label::new();
                    {
                        let mut temps = MaglevAssembler::TemporaryRegisterScope::new(masm);
                        let mask = temps.acquire_scratch();
                        masm.add(mask, rhs, (-1).into());
                        masm.tst(mask, rhs);
                        masm.jump_if(Condition::Ne, &rhs_not_power_of_2);

                        // {rhs} is power of 2.
                        masm.and_(out, mask, lhs);
                        masm.jump(done.borrow());
                        // {mask} can be reused from now on.
                    }

                    masm.bind(&rhs_not_power_of_2);
                    uint32_mod::uint32_mod(masm, out, lhs, rhs);
                    masm.bind(done.borrow());
                }
            }

            macro_rules! def_bitwise_binop {
                ($instruction:ident, $opcode:ident) => {
                    impl $instruction {
                        pub fn set_value_location_constraints(&mut self) {
                            self.use_register(self.left_input());
                            self.use_register(self.right_input());
                            self.define_as_register(self);
                        }

                        pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                            let left = self.left_input().to_register();
                            let right = self.right_input().to_register();
                            let out = self.result().to_register();
                            masm.$opcode(out, left, right);
                        }
                    }
                };
            }
            def_bitwise_binop!(Int32BitwiseAnd, and_);
            def_bitwise_binop!(Int32BitwiseOr, orr);
            def_bitwise_binop!(Int32BitwiseXor, eor);

            macro_rules! def_shift_binop {
                ($instruction:ident, $opcode:ident) => {
                    impl $instruction {
                        pub fn set_value_location_constraints(&mut self) {
                            self.use_register(self.left_input());
                            if self.right_input().node().is::<Int32Constant>() {
                                self.use_any(self.right_input());
                            } else {
                                self.use_register(self.right_input());
                            }
                            self.define_as_register(self);
                        }
                        pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                            let left = self.left_input().to_register();
                            let out = self.result().to_register();
                            if let Some(constant) = self.right_input().node().try_cast::<Int32Constant>() {
                                let shift = (constant.value() & 31) as u32;
                                if shift == 0 {
                                    /* TODO(victorgomes): Arm will do a shift of 32 if right == 0. Ideally
                                     * we should not even emit the shift in the first place. We do a move
                                     * here for the moment. */
                                    masm.move_(out, left);
                                } else {
                                    masm.$opcode(out, left, shift.into());
                                }
                            } else {
                                let mut temps = MaglevAssembler::TemporaryRegisterScope::new(masm);
                                let scratch = temps.acquire_scratch();
                                let right = self.right_input().to_register();
                                masm.and_(scratch, right, 31.into());
                                masm.$opcode(out, left, scratch.into());
                            }
                        }
                    }
                };
            }
            def_shift_binop!(Int32ShiftLeft, lsl);
            def_shift_binop!(Int32ShiftRight, asr);
            def_shift_binop!(Int32ShiftRightLogical, lsr);

            impl Int32BitwiseNot {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.value_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let value = self.value_input().to_register();
                    let out = self.result().to_register();
                    masm.mvn(out, value.into());
                }
            }

            impl Float64Add {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.left_input());
                    self.use_register(self.right_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let left = self.left_input().to_double_register();
                    let right = self.right_input().to_double_register();
                    let out = self.result().to_double_register();
                    masm.vadd(out, left, right);
                }
            }

            impl Float64Subtract {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.left_input());
                    self.use_register(self.right_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let left = self.left_input().to_double_register();
                    let right = self.right_input().to_double_register();
                    let out = self.result().to_double_register();
                    masm.vsub(out, left, right);
                }
            }

            impl Float64Multiply {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.left_input());
                    self.use_register(self.right_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let left = self.left_input().to_double_register();
                    let right = self.right_input().to_double_register();
                    let out = self.result().to_double_register();
                    masm.vmul(out, left, right);
                }
            }

            impl Float64Divide {
                pub fn set_value_location_constraints(&mut self) {
                    self.use_register(self.left_input());
                    self.use_register(self.right_input());
                    self.define_as_register(self);
                }

                pub fn generate_code(&self, masm: &mut MaglevAssembler, state: &ProcessingState) {
                    let left = self.left_input().to_double_register();
                    let right = self.right_input().to_double_register();
                    let out = self.result().to_double_register();
                    masm.vdiv(out, left, right);
                }
            }

            impl Float64Modulus {
                pub const fn max_call_stack_args() -> i32 {
                    0
                }
                pub fn set_value_location_constraints(&mut self