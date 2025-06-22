// src/maglev/arm/maglev_assembler_arm_inl.rs

pub mod maglev {
    pub mod arm {
        use crate::base::numbers::double::Double;
        use crate::codegen::interface_descriptors_inl::*;
        use crate::codegen::macro_assembler_inl::*;
        use crate::common::globals::*;
        use crate::compiler::compilation_dependencies::*;
        use crate::maglev::maglev_assembler::*;
        use crate::maglev::maglev_basic_block::*;
        use crate::maglev::maglev_code_gen_state::*;

        pub const fn condition_for_float64(operation: Operation) -> Condition {
            condition_for(operation)
        }

        pub const fn condition_for_nan() -> Condition {
            Condition::Vs
        }

        pub fn shift_from_scale(n: i32) -> i32 {
            match n {
                1 => 0,
                2 => 1,
                4 => 2,
                8 => 3,
                _ => unreachable!(),
            }
        }

        pub struct MapCompare<'a> {
            masm_: &'a mut MaglevAssembler,
            object_: Register,
            map_count_: usize,
            map_: Register,
        }

        impl<'a> MapCompare<'a> {
            pub fn new(masm: &'a mut MaglevAssembler, object: Register, map_count: usize) -> Self {
                let mut map_compare = MapCompare {
                    masm_: masm,
                    object_: object,
                    map_count_: map_count,
                    map_: masm_.scratch_register_scope().acquire(),
                };
                masm_.load_map(map_compare.map_, map_compare.object_);
                map_compare
            }

            pub fn generate(
                &mut self,
                map: &Handle<Map>,
                cond: Condition,
                if_true: &mut Label,
                distance: LabelDistance,
            ) {
                let mut temps = TemporaryRegisterScope::new(self.masm_);
                let temp = temps.acquire_scratch();
                self.masm_.move_(temp, map);
                self.masm_.cmp(self.map_, temp);
                self.masm_.jump_if(cond, if_true, distance);
            }

            pub fn get_map(&self) -> Register {
                self.map_
            }

            pub fn temporary_count(map_count: usize) -> i32 {
                1
            }
        }

        // Note: Due to lifetime limitations and the complexity of the C++ code,
        // some parts of the Push functions have not been fully translated,
        // especially regarding iterator ranges and template metaprogramming.
        // The following is a simplified version.
        //
        // The original C++ code uses template metaprogramming and iterator ranges
        // extensively for pushing values onto the stack. This is difficult to
        // replicate exactly in Rust due to Rust's different approach to generics
        // and lack of direct equivalent to C++'s template metaprogramming.

        pub mod detail {

            // Note: PushAllHelper cannot be fully translated as it relies heavily
            // on C++ template metaprogramming features that do not directly
            // translate to Rust.
            pub trait PushAllHelperTrait {
                fn push(masm: &mut MaglevAssembler);
                fn push_reverse(masm: &mut MaglevAssembler);
            }
        }

        impl MaglevAssembler {
            pub fn push<T>(&mut self, _val: T) {
                // Placeholder implementation - adapt according to actual usage
                todo!()
            }

            pub fn push_reverse<T>(&mut self, _val: T) {
                // Placeholder implementation - adapt according to actual usage
                todo!()
            }

            pub fn bind_jump_target(&mut self, label: &mut Label) {
                self.bind(label);
            }

            pub fn bind_block(&mut self, block: &mut BasicBlock) {
                self.bind(block.label());
            }

            pub fn smi_tag_int32_and_set_flags(&mut self, dst: Register, src: Register) {
                self.add(dst, src, src, true);
            }

            pub fn check_int32_is_smi(&mut self, obj: Register, fail: &mut Label, scratch: Option<Register>) {
                assert!(!smi_values_are_32_bits());

                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = match scratch {
                    Some(r) => r,
                    None => temps.acquire_scratch(),
                };
                self.add(scratch, obj, obj, true);
                self.jump_if(Condition::Overflow, fail, LabelDistance::Near);
            }

            pub fn smi_add_constant(
                &mut self,
                dst: Register,
                src: Register,
                value: i32,
                fail: &mut Label,
                distance: LabelDistance,
            ) {
                assert!(!smi_values_are_32_bits());
                self.assert_smi(src);
                if value != 0 {
                    self.add(dst, src, Operand::Smi(Smi::from_int(value)), true);
                    self.jump_if(Condition::Overflow, fail, distance);
                } else {
                    self.move_(dst, src);
                }
            }

            pub fn smi_sub_constant(
                &mut self,
                dst: Register,
                src: Register,
                value: i32,
                fail: &mut Label,
                distance: LabelDistance,
            ) {
                assert!(!smi_values_are_32_bits());
                self.assert_smi(src);
                if value != 0 {
                    self.sub(dst, src, Operand::Smi(Smi::from_int(value)), true);
                    self.jump_if(Condition::Overflow, fail, distance);
                } else {
                    self.move_(dst, src);
                }
            }

            pub fn move_heap_number(&mut self, dst: Register, value: f64) {
                self.mov(dst, Operand::EmbeddedNumber(value));
            }

            pub fn is_root_constant(&mut self, input: Input, root_index: RootIndex) -> Condition {
                if input.operand().is_register() {
                    self.compare_root(to_register(input), root_index);
                } else {
                    debug_assert!(input.operand().is_stack_slot());
                    let mut temps = TemporaryRegisterScope::new(self);
                    let scratch = temps.acquire_scratch();
                    self.ldr(scratch, to_mem_operand(input, self));
                    self.compare_root(scratch, root_index);
                }
                Condition::Eq
            }

            pub fn stack_slot_operand(&self, slot: StackSlot) -> MemOperand {
                MemOperand::new(Register::Fp, slot.index)
            }

            pub fn get_frame_pointer(&self) -> Register {
                Register::Fp
            }

            pub fn get_stack_slot(&self, operand: &compiler::AllocatedOperand) -> MemOperand {
                MemOperand::new(Register::Fp, self.get_frame_pointer_offset_for_stack_slot(operand))
            }

            pub fn to_mem_operand(&self, operand: &compiler::InstructionOperand) -> MemOperand {
                self.get_stack_slot(compiler::AllocatedOperand::cast(operand))
            }

            pub fn to_mem_operand_value_location(&self, location: &ValueLocation) -> MemOperand {
                self.to_mem_operand(&location.operand())
            }

            pub fn build_typed_array_data_pointer(&mut self, data_pointer: Register, object: Register) {
                debug_assert_ne!(data_pointer, object);
                self.ldr(
                    data_pointer,
                    MemOperand::new_field(object, JSTypedArray::k_external_pointer_offset() as i32)
                );
                if JSTypedArray::k_max_size_in_heap() == 0 {
                    return;
                }
                let mut temps = TemporaryRegisterScope::new(self);
                let base = temps.acquire_scratch();
                self.ldr(
                    base,
                    MemOperand::new_field(object, JSTypedArray::k_base_pointer_offset() as i32)
                );
                self.add(data_pointer, data_pointer, Operand::Reg(base));
            }

            pub fn typed_array_element_operand(
                &self,
                data_pointer: Register,
                index: Register,
                element_size: i32,
            ) -> MemOperand {
                let shift = shift_from_scale(element_size);
                self.add(data_pointer, data_pointer, Operand::ShiftedReg(index, ShiftType::LSL, shift));
                MemOperand::new(data_pointer, 0)
            }

            pub fn data_view_element_operand(&self, data_pointer: Register, index: Register) -> MemOperand {
                MemOperand::new(data_pointer, index)
            }

            pub fn load_tagged_field_by_index(
                &mut self,
                result: Register,
                object: Register,
                index: Register,
                scale: i32,
                offset: i32,
            ) {
                let shift = shift_from_scale(scale);
                self.add(result, object, Operand::ShiftedReg(index, ShiftType::LSL, shift));
                MacroAssembler::load_tagged_field(
                    self,
                    result,
                    MemOperand::new_field(result, offset)
                );
            }

            pub fn load_bounded_size_from_object(&mut self, result: Register, object: Register, offset: i32) {
                self.move_(result, MemOperand::new_field(object, offset));
            }

            pub fn load_external_pointer_field(&mut self, result: Register, operand: MemOperand) {
                self.move_(result, operand);
            }

            pub fn load_fixed_array_element(&mut self, result: Register, array: Register, index: Register) {
                if v8_flags.debug_code {
                    self.assert_object_type(array, InstanceType::FixedArray, AbortReason::UnexpectedValue);
                    self.compare_int32_and_assert(index, 0, Condition::UnsignedGreaterThanEqual, AbortReason::UnexpectedNegativeValue);
                }
                self.load_tagged_field_by_index(result, array, index, k_tagged_size() as i32, offset_of_data_start(InstanceType::FixedArray) as i32);
            }

            pub fn load_fixed_array_element_without_decompressing(&mut self, result: Register, array: Register, index: Register) {
                // No compression mode on arm.
                self.load_fixed_array_element(result, array, index);
            }

            pub fn load_fixed_double_array_element(
                &mut self,
                result: DoubleRegister,
                array: Register,
                index: Register,
            ) {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                if v8_flags.debug_code {
                    self.assert_object_type(array, InstanceType::FixedDoubleArray, AbortReason::UnexpectedValue);
                    self.compare_int32_and_assert(index, 0, Condition::UnsignedGreaterThanEqual, AbortReason::UnexpectedNegativeValue);
                }
                self.add(scratch, array, Operand::ShiftedReg(index, ShiftType::LSL, k_double_size_log2() as i32));
                self.vldr(result, MemOperand::new_field(scratch, offset_of_data_start(InstanceType::FixedArray) as i32));
            }

            pub fn store_fixed_double_array_element(
                &mut self,
                array: Register,
                index: Register,
                value: DoubleRegister,
            ) {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                self.add(scratch, array, Operand::ShiftedReg(index, ShiftType::LSL, k_double_size_log2() as i32));
                self.vstr(value, MemOperand::new_field(scratch, offset_of_data_start(InstanceType::FixedArray) as i32));
            }

            pub fn load_signed_field(&mut self, result: Register, operand: MemOperand, size: i32) {
                if size == 1 {
                    self.ldrsb(result, operand);
                } else if size == 2 {
                    self.ldrsh(result, operand);
                } else {
                    debug_assert_eq!(size, 4);
                    self.ldr(result, operand);
                }
            }

            pub fn load_unsigned_field(&mut self, result: Register, operand: MemOperand, size: i32) {
                if size == 1 {
                    self.ldrb(result, operand);
                } else if size == 2 {
                    self.ldrh(result, operand);
                } else {
                    debug_assert_eq!(size, 4);
                    self.ldr(result, operand);
                }
            }

            pub fn set_slot_address_for_tagged_field(&mut self, slot_reg: Register, object: Register, offset: i32) {
                self.add(slot_reg, object, Operand::Imm(offset - k_heap_object_tag()));
            }

            pub fn set_slot_address_for_fixed_array_element(
                &mut self,
                slot_reg: Register,
                object: Register,
                index: Register,
            ) {
                self.add(
                    slot_reg,
                    object,
                    Operand::Imm(offset_of_data_start(InstanceType::FixedArray) as i32 - k_heap_object_tag())
                );
                self.add(slot_reg, slot_reg, Operand::ShiftedReg(index, ShiftType::LSL, k_tagged_size_log2() as i32));
            }

            pub fn store_tagged_field_no_write_barrier(&mut self, object: Register, offset: i32, value: Register) {
                MacroAssembler::store_tagged_field(
                    self,
                    value,
                    MemOperand::new_field(object, offset)
                );
            }

            pub fn store_fixed_array_element_no_write_barrier(
                &mut self,
                array: Register,
                index: Register,
                value: Register,
            ) {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                self.add(scratch, array, Operand::ShiftedReg(index, ShiftType::LSL, k_tagged_size_log2() as i32));
                MacroAssembler::store_tagged_field(
                    self,
                    value,
                    MemOperand::new_field(scratch, offset_of_data_start(InstanceType::FixedArray) as i32)
                );
            }

            pub fn store_tagged_signed_field(&mut self, object: Register, offset: i32, value: Register) {
                self.assert_smi(value);
                MacroAssembler::store_tagged_field(
                    self,
                    value,
                    MemOperand::new_field(object, offset)
                );
            }

            pub fn store_tagged_signed_field_tagged_smi(&mut self, object: Register, offset: i32, value: Tagged<Smi>) {
                let mut scope = TemporaryRegisterScope::new(self);
                let scratch = scope.acquire_scratch();
                self.move_(scratch, value);
                MacroAssembler::store_tagged_field(
                    self,
                    scratch,
                    MemOperand::new_field(object, offset)
                );
            }

            pub fn store_int32_field(&mut self, object: Register, offset: i32, value: i32) {
                let mut scope = TemporaryRegisterScope::new(self);
                let scratch = scope.acquire_scratch();
                self.move_(scratch, value);
                self.str(scratch, MemOperand::new_field(object, offset));
            }

            pub fn store_field(&mut self, operand: MemOperand, value: Register, size: i32) {
                debug_assert!(size == 1 || size == 2 || size == 4);
                if size == 1 {
                    self.strb(value, operand);
                } else if size == 2 {
                    self.strh(value, operand);
                } else {
                    debug_assert_eq!(size, 4);
                    self.str(value, operand);
                }
            }

            pub fn reverse_byte_order(&mut self, value: Register, size: i32) {
                if size == 2 {
                    self.rev(value, value);
                    self.asr(value, value, Operand::Imm(16));
                } else if size == 4 {
                    self.rev(value, value);
                } else {
                    debug_assert_eq!(size, 1);
                }
            }

            pub fn increment_int32(&mut self, reg: Register) {
                self.add(reg, reg, Operand::Imm(1));
            }

            pub fn decrement_int32(&mut self, reg: Register) {
                self.sub(reg, reg, Operand::Imm(1));
            }

            pub fn add_int32(&mut self, reg: Register, amount: i32) {
                self.add(reg, reg, Operand::Imm(amount));
            }

            pub fn and_int32(&mut self, reg: Register, mask: i32) {
                self.and_(reg, reg, Operand::Imm(mask));
            }

            pub fn or_int32(&mut self, reg: Register, mask: i32) {
                self.orr(reg, reg, Operand::Imm(mask));
            }

            pub fn and_int32_register(&mut self, reg: Register, other: Register) {
                self.and_(reg, reg, Operand::Reg(other));
            }

            pub fn or_int32_register(&mut self, reg: Register, other: Register) {
                self.orr(reg, reg, Operand::Reg(other));
            }

            pub fn shift_left(&mut self, reg: Register, amount: i32) {
                self.lsl(reg, reg, Operand::Imm(amount));
            }

            pub fn increment_address(&mut self, reg: Register, delta: i32) {
                self.add(reg, reg, Operand::Imm(delta));
            }

            pub fn load_address(&mut self, dst: Register, location: MemOperand) {
                debug_assert_eq!(location.am(), AddressingMode::Offset);
                self.add(dst, location.rn(), Operand::Imm(location.offset()));
            }

            pub fn call(&mut self, target: &mut Label) {
                self.bl(target);
            }

            pub fn emit_enter_exit_frame(
                &mut self,
                extra_slots: i32,
                frame_type: StackFrameType,
                c_function: Register,
                scratch: Register,
            ) {
                self.enter_exit_frame(scratch, extra_slots, frame_type);
            }

            pub fn move_stack_slot_register(&mut self, dst: StackSlot, src: Register) {
                self.str(src, self.stack_slot_operand(dst));
            }

            pub fn move_stack_slot_double_register(&mut self, dst: StackSlot, src: DoubleRegister) {
                self.vstr(src, self.stack_slot_operand(dst));
            }

            pub fn move_register_stack_slot(&mut self, dst: Register, src: StackSlot) {
                self.ldr(dst, self.stack_slot_operand(src));
            }

            pub fn move_double_register_stack_slot(&mut self, dst: DoubleRegister, src: StackSlot) {
                self.vldr(dst, self.stack_slot_operand(src));
            }

            pub fn move_mem_operand_register(&mut self, dst: MemOperand, src: Register) {
                self.str(src, dst);
            }

            pub fn move_register_mem_operand(&mut self, dst: Register, src: MemOperand) {
                self.ldr(dst, src);
            }

            pub fn move_double_register_double_register(&mut self, dst: DoubleRegister, src: DoubleRegister) {
                if dst != src {
                    self.vmov(dst, src);
                }
            }

            pub fn move_register_tagged_smi(&mut self, dst: Register, src: Tagged<Smi>) {
                MacroAssembler::move_(self, dst, src);
            }

            pub fn move_register_external_reference(&mut self, dst: Register, src: ExternalReference) {
                MacroAssembler::move_(self, dst, src);
            }

            pub fn move_register_register(&mut self, dst: Register, src: Register) {
                if dst != src {
                    self.mov(dst, src);
                }
            }

            pub fn move_register_tagged_tagged_index(&mut self, dst: Register, i: Tagged<TaggedIndex>) {
                self.mov(dst, Operand::Imm(i.ptr() as i32));
            }

            pub fn move_register_int32(&mut self, dst: Register, i: i32) {
                self.mov(dst, Operand::Imm(i));
            }

            pub fn move_register_uint32(&mut self, dst: Register, i: u32) {
                self.mov(dst, Operand::Imm(i as i32));
            }

            pub fn move_double_register_double(&mut self, dst: DoubleRegister, n: f64) {
                self.vmov(dst, Double::new(n));
            }

            pub fn move_double_register_float64(&mut self, dst: DoubleRegister, n: Float64) {
                self.vmov(dst, Double::new(n.get_bits()));
            }

            pub fn move_register_handle_heap_object(&mut self, dst: Register, obj: &Handle<HeapObject>) {
                MacroAssembler::move_(self, dst, obj);
            }

            pub fn move_tagged(&mut self, dst: Register, obj: &Handle<HeapObject>) {
                self.move_(dst, obj);
            }

            pub fn load_int32(&mut self, dst: Register, src: MemOperand) {
                self.ldr(dst, src);
            }

            pub fn store_int32(&mut self, dst: MemOperand, src: Register) {
                self.str(src, dst);
            }

            pub fn load_float32(&mut self, dst: DoubleRegister, src: MemOperand) {
                let mut temps = UseScratchRegisterScope::new(self);
                let mut temp_vfps: SwVfpRegister = SwVfpRegister::NoReg;
                if dst.code() < 16 {
                    temp_vfps = LowDwVfpRegister::from_code(dst.code()).low();
                } else {
                    temp_vfps = temps.acquire_s();
                }
                self.vldr(temp_vfps, src);
                self.vcvt_f64_f32(dst, temp_vfps);
            }

            pub fn store_float32(&mut self, dst: MemOperand, src: DoubleRegister) {
                let mut temps = UseScratchRegisterScope::new(self);
                let temp_vfps: SwVfpRegister = temps.acquire_s();
                self.vcvt_f32_f64(temp_vfps, src);
                self.vstr(temp_vfps, dst);
            }

            pub fn load_float64(&mut self, dst: DoubleRegister, src: MemOperand) {
                self.vldr(dst, src);
            }

            pub fn store_float64(&mut self, dst: MemOperand, src: DoubleRegister) {
                self.vstr(src, dst);
            }

            pub fn load_unaligned_float64(&mut self, dst: DoubleRegister, base: Register, index: Register) {
                // vldr only works on 4 bytes aligned access.
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                self.ldr(scratch, MemOperand::new(base, index));
                self.vmov_low(dst, scratch);
                self.add(scratch, index, Operand::Imm(4));
                self.ldr(scratch, MemOperand::new(base, scratch));
                self.vmov_high(dst, scratch);
            }

            pub fn load_unaligned_float64_and_reverse_byte_order(&mut self, dst: DoubleRegister, base: Register, index: Register) {
                // vldr only works on 4 bytes aligned access.
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                self.ldr(scratch, MemOperand::new(base, index));
                self.rev(scratch, scratch);
                self.vmov_high(dst, scratch);
                self.add(scratch, index, Operand::Imm(4));
                self.ldr(scratch, MemOperand::new(base, scratch));
                self.rev(scratch, scratch);
                self.vmov_low(dst, scratch);
            }

            pub fn store_unaligned_float64(&mut self, base: Register, index: Register, src: DoubleRegister) {
                // vstr only works on 4 bytes aligned access.
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                let index_scratch = temps.acquire_scratch();
                self.vmov_low(scratch, src);
                self.str(scratch, MemOperand::new(base, index));
                self.add(index_scratch, index, Operand::Imm(4));
                self.vmov_high(scratch, src);
                self.str(scratch, MemOperand::new(base, index_scratch));
            }

            pub fn reverse_byte_order_and_store_unaligned_float64(&mut self, base: Register, index: Register, src: DoubleRegister) {
                // vstr only works on 4 bytes aligned access.
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                let index_scratch = temps.acquire_scratch();
                self.vmov_high(scratch, src);
                self.rev(scratch, scratch);
                self.str(scratch, MemOperand::new(base, index));
                self.add(index_scratch, index, Operand::Imm(4));
                self.vmov_low(scratch, src);
                self.rev(scratch, scratch);
                self.str(scratch, MemOperand::new(base, index_scratch));
            }

            pub fn sign_extend32_to_64_bits(&mut self, _dst: Register, _src: Register) {
                // No 64-bit registers.
            }

            pub fn negate_int32(&mut self, val: Register) {
                self.rsb(val, val, Operand::Imm(0));
            }

            pub fn to_uint8_clamped(&mut self, result: Register, value: DoubleRegister, min: &mut Label, max: &mut Label, done: &mut Label) {
                let mut scope = CpuFeatureScope::new(self, CpuFeatures::Armv8);
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch_double();
                self.move_double_register_double(scratch, 0.0);
                self.vfp_compare_and_set_flags(scratch, value);
                // Set to 0 if NaN.
                self.jump_if(Condition::Overflow, min, LabelDistance::Near);
                self.jump_if(Condition::GreaterThanEqual, min, LabelDistance::Near);
                self.move_double_register_double(scratch, 255.0);
                self.vfp_compare_and_set_flags(value, scratch);
                self.jump_if(Condition::GreaterThanEqual, max, LabelDistance::Near);
                // if value in [0, 255], then round up to the nearest.
                self.vrintn(scratch, value);
                self.truncate_double_to_int32(result, scratch);
                self.jump(done);
            }

            pub fn deopt_if_buffer_detached<NodeT>(&mut self, array: Register, scratch: Register, node: &NodeT) {
                // A detached buffer leads to megamorphic feedback, so we won't have a deopt
                // loop if we deopt here.
                self.load_tagged_field(
                    scratch,
                    MemOperand::new_field(array, JSArrayBufferView::k_buffer_offset() as i32)
                );
                self.load_tagged_field(
                    scratch,
                    MemOperand::new_field(scratch, JSArrayBuffer::k_bit_field_offset() as i32)
                );
                self.tst(scratch, Operand::Imm(JSArrayBuffer::WasDetachedBit::k_mask()));
                self.emit_eager_deopt_if(Condition::NotEqual, DeoptimizeReason::ArrayBufferWasDetached, node);
            }

            pub fn load_byte(&mut self, dst: Register, src: MemOperand) {
                self.ldrb(dst, src);
            }

            pub fn is_callable_and_not_undetectable(&mut self, map: Register, scratch: Register) -> Condition {
                self.ldrb(scratch, MemOperand::new_field(map, Map::k_bit_field_offset() as i32));
                self.and_(
                    scratch,
                    scratch,
                    Operand::Imm(Map::Bits1::is_undetectable_bit() | Map::Bits1::is_callable_bit()),
                );
                self.cmp(scratch, Operand::Imm(Map::Bits1::is_callable_bit()));
                Condition::Equal
            }

            pub fn is_not_callable_nor_undetactable(&mut self, map: Register, scratch: Register) -> Condition {
                self.ldrb(scratch, MemOperand::new_field(map, Map::k_bit_field_offset() as i32));
                self.tst(
                    scratch,
                    Operand::Imm(Map::Bits1::is_undetectable_bit() | Map::Bits1::is_callable_bit()),
                );
                Condition::Equal
            }

            pub fn load_instance_type(&mut self, instance_type: Register, heap_object: Register) {
                self.load_map(instance_type, heap_object);
                self.ldrh(instance_type, MemOperand::new_field(instance_type, Map::k_instance_type_offset() as i32));
            }

            pub fn jump_if_object_type(
                &mut self,
                heap_object: Register,
                type_: InstanceType,
                target: &mut Label,
                distance: LabelDistance,
            ) {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                self.compare_object_type(heap_object, scratch, scratch, type_);
                self.jump_if(Condition::Equal, target, distance);
            }

            pub fn jump_if_not_object_type(
                &mut self,
                heap_object: Register,
                type_: InstanceType,
                target: &mut Label,
                distance: LabelDistance,
            ) {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                self.compare_object_type(heap_object, scratch, scratch, type_);
                self.jump_if(Condition::NotEqual, target, distance);
            }

            pub fn assert_object_type(&mut self, heap_object: Register, type_: InstanceType, reason: AbortReason) {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                self.assert_not_smi(heap_object);
                self.compare_object_type(heap_object, scratch, scratch, type_);
                self.assert(Condition::Equal, reason);
            }

            pub fn branch_on_object_type(
                &mut self,
                heap_object: Register,
                type_: InstanceType,
                if_true: &mut Label,
                true_distance: LabelDistance,
                fallthrough_when_true: bool,
                if_false: &mut Label,
                false_distance: LabelDistance,
                fallthrough_when_false: bool,
            ) {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.acquire_scratch();
                self.compare_object_type(heap_object, scratch, scratch, type_);
                self.branch(
                    Condition::Equal,
                    if_true,
                    true_distance,
                    fallthrough_when_true,
                    if_false,
                    false_distance,
                    fallthrough_when_false,
                );
            }

            pub fn jump_if_object_type_in_range(
                &mut self,
                heap_object: Register,
                lower_limit: InstanceType,
                higher_limit: InstanceType,
                target: &mut Label,
                distance: LabelDistance,
            ) {
                let mut temps = TemporaryRegisterScope::new(self);
                let scratch = temps.