// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This is a partial translation. Some parts rely on V8 internal
// structures and functionalities which can't be directly represented
// in Rust.  The placeholder types and functions are used to maintain
// the structure and demonstrate the conversion approach.

//use std::convert::TryInto;
//use std::mem::size_of;
//use std::ops::{BitOr, BitOrAssign};

mod builtins;
mod codegen;
mod compiler;
mod interpreter;
mod objects;

use self::builtins::*;
use self::codegen::*;
use self::compiler::*;
use self::interpreter::*;
use self::objects::*;

//use crate::base::TryAsUsize;

//use crate::compiler::{
//    CodeAssemblerState, Descriptor, ExternalReference, MachineRepresentation, MachineType,
//};
//use crate::debug::DebugDelegate;
//use crate::interpreter::{Bytecode, Bytecodes, InterpreterDispatchDescriptor, OperandScale};
//use crate::objects::{AllocationSite, Context, FeedbackCell, FeedbackVector, JSFunction, Object};
//use crate::roots::RootIndex;
//use crate::wasm::WasmCodeManager;
//use crate::{Address, Isolate};

//use compiler::Node;
//use std::rc::Rc;

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: {:?} != {:?}", $left, $right);
        }
    };
}

macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {:?}", stringify!($condition));
        }
    };
}

macro_rules! CSA_DCHECK {
    ($self:expr, $condition:expr) => {
        if !$condition {
            panic!("CSA_DCHECK failed: {:?}", stringify!($condition));
        }
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ImplicitRegisterUse {
    KNone,
    KReadAccumulator,
    KWriteAccumulator,
    KClobberAccumulator,
    KWriteShortStar,
}

impl ImplicitRegisterUse {
    fn is_none(&self) -> bool {
        *self == ImplicitRegisterUse::KNone
    }
}

impl std::ops::BitOr for ImplicitRegisterUse {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        use ImplicitRegisterUse::*;
        match (self, other) {
            (KNone, x) | (x, KNone) => x,
            _ => panic!("BitOr not fully implemented"),
        }
    }
}

impl std::ops::BitOrAssign for ImplicitRegisterUse {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

#[derive(Debug, Clone, Copy)]
struct RegListNodePair {
    base_reg_location: IntPtrT,
    reg_count: Uint32T,
}

impl RegListNodePair {
    fn new(base_reg_location: IntPtrT, reg_count: Uint32T) -> Self {
        RegListNodePair {
            base_reg_location,
            reg_count,
        }
    }

    fn reg_count(&self) -> Uint32T {
        self.reg_count
    }
    // Add other methods as needed
}

struct InterpreterAssembler<'a> {
    state: &'a mut CodeAssemblerState,
    bytecode_: Bytecode,
    operand_scale_: OperandScale,
    interpreted_frame_pointer_: TVariable<RawPtrT>,
    bytecode_array_: TVariable<BytecodeArray>,
    bytecode_offset_: TVariable<IntPtrT>,
    dispatch_table_: TVariable<ExternalReference>,
    accumulator_: TVariable<Object>,
    implicit_register_use_: ImplicitRegisterUse,
    made_call_: bool,
    reloaded_frame_ptr_: bool,
    bytecode_array_valid_: bool,
    call_prologue_callback: Option<Box<dyn FnMut()>>,
    call_epilogue_callback: Option<Box<dyn FnMut()>>,
}

impl<'a> InterpreterAssembler<'a> {
    fn new(state: &'a mut CodeAssemblerState, bytecode: Bytecode, operand_scale: OperandScale) -> Self {
        let bytecode_array_param = Parameter::<BytecodeArray>::new(InterpreterDispatchDescriptor::kBytecodeArray);
        let bytecode_offset_param = UncheckedParameter::<IntPtrT>::new(InterpreterDispatchDescriptor::kBytecodeOffset);
        let dispatch_table_param = UncheckedParameter::<ExternalReference>::new(InterpreterDispatchDescriptor::kDispatchTable);
        let accumulator_param = Parameter::<Object>::new(InterpreterDispatchDescriptor::kAccumulator);
        let mut assembler = InterpreterAssembler {
            state,
            bytecode_: bytecode,
            operand_scale_: operand_scale,
            interpreted_frame_pointer_: TVariable::new(),
            bytecode_array_: TVariable::new_with_value(bytecode_array_param),
            bytecode_offset_: TVariable::new_with_value(bytecode_offset_param),
            dispatch_table_: TVariable::new_with_value(dispatch_table_param),
            accumulator_: TVariable::new_with_value(accumulator_param),
            implicit_register_use_: ImplicitRegisterUse::KNone,
            made_call_: false,
            reloaded_frame_ptr_: false,
            bytecode_array_valid_: true,
            call_prologue_callback: None,
            call_epilogue_callback: None,
        };
        //assembler.trace_bytecode(Runtime::kTraceUnoptimizedBytecodeEntry);
        assembler.register_call_generation_callbacks(
            Box::new(|| assembler.call_prologue()),
            Box::new(|| assembler.call_epilogue()),
        );

        if Bytecodes::makes_call_along_critical_path(bytecode) || Bytecodes::returns(bytecode) {
            assembler.save_bytecode_offset();
        }

        assembler
    }

    fn register_call_generation_callbacks(
        &mut self,
        prologue_callback: Box<dyn FnMut()>,
        epilogue_callback: Box<dyn FnMut()>,
    ) {
        self.call_prologue_callback = Some(prologue_callback);
        self.call_epilogue_callback = Some(epilogue_callback);
    }

    fn unregister_call_generation_callbacks(&mut self) {
        self.call_prologue_callback = None;
        self.call_epilogue_callback = None;
    }

    fn get_interpreted_frame_pointer(&mut self) -> RawPtrT {
        if !self.interpreted_frame_pointer_.is_bound() {
            self.interpreted_frame_pointer_ = TVariable::new_with_value(self.load_parent_frame_pointer());
        } else if Bytecodes::makes_call_along_critical_path(self.bytecode_) && self.made_call_ && !self.reloaded_frame_ptr_ {
            self.interpreted_frame_pointer_ = TVariable::new_with_value(self.load_parent_frame_pointer());
            self.reloaded_frame_ptr_ = true;
        }
        self.interpreted_frame_pointer_.value()
    }

    fn bytecode_offset(&mut self) -> IntPtrT {
        if Bytecodes::makes_call_along_critical_path(self.bytecode_) && self.made_call_ && (self.bytecode_offset_.value() == UncheckedParameter::<IntPtrT>::new(InterpreterDispatchDescriptor::kBytecodeOffset).value()) {
            self.bytecode_offset_ = TVariable::new_with_value(self.reload_bytecode_offset());
        }
        self.bytecode_offset_.value()
    }

    fn reload_bytecode_offset(&self) -> IntPtrT {
        let offset = self.load_and_untag_register(Register::bytecode_offset());
        if self.operand_scale() != OperandScale::kSingle {
            // Add one to the offset such that it points to the actual bytecode rather
            // than the Wide / ExtraWide prefix bytecode.
            return IntPtrAdd(offset, IntPtrConstant(1));
        }
        offset
    }

    fn save_bytecode_offset(&mut self) {
        let mut bytecode_offset = self.bytecode_offset();
        if self.operand_scale() != OperandScale::kSingle {
            // Subtract one from the bytecode_offset such that it points to the Wide /
            // ExtraWide prefix bytecode.
            bytecode_offset = IntPtrSub(self.bytecode_offset(), IntPtrConstant(1));
        }
        let store_offset = Register::bytecode_offset().to_operand() * kSystemPointerSize;
        let base = self.get_interpreted_frame_pointer();

        if smi_values_are_32_bits() {
            let zero_offset = store_offset + 4;
            let payload_offset = store_offset;
            // #[cfg(target_endian = "little")]
            // std::mem::swap(zero_offset, payload_offset);

            self.store_no_write_barrier(MachineRepresentation::kWord32, base, IntPtrConstant(zero_offset), Int32Constant(0));
            self.store_no_write_barrier(MachineRepresentation::kWord32, base, IntPtrConstant(payload_offset), self.truncate_intptr_to_int32(bytecode_offset));
        } else {
            self.store_full_tagged_no_write_barrier(base, IntPtrConstant(store_offset), self.smi_tag(bytecode_offset));
        }
    }

    fn bytecode_array_tagged_pointer(&mut self) -> BytecodeArray {
        // Force a re-load of the bytecode array after every call in case the debugger
        // has been activated.
        if !self.bytecode_array_valid_ {
            self.bytecode_array_ = TVariable::new_with_value(self.cast(self.load_register(Register::bytecode_array())));
            self.bytecode_array_valid_ = true;
        }
        self.bytecode_array_.value()
    }

    fn dispatch_table_pointer(&mut self) -> ExternalReference {
        if Bytecodes::makes_call_along_critical_path(self.bytecode_) && self.made_call_ && (self.dispatch_table_.value() == UncheckedParameter::<ExternalReference>::new(InterpreterDispatchDescriptor::kDispatchTable).value()) {
            self.dispatch_table_ = TVariable::new_with_value(self.external_constant(ExternalReference::interpreter_dispatch_table_address(/*self.isolate()*/)));
        }
        self.dispatch_table_.value()
    }

    fn get_accumulator_unchecked(&self) -> Object {
        self.accumulator_.value()
    }

    fn get_accumulator(&mut self) -> Object {
        DCHECK!(Bytecodes::reads_accumulator(self.bytecode_));
        self.implicit_register_use_ |= ImplicitRegisterUse::KReadAccumulator;
        self.get_accumulator_unchecked()
    }

    fn set_accumulator(&mut self, value: Object) {
        DCHECK!(Bytecodes::writes_accumulator(self.bytecode_));
        self.implicit_register_use_ |= ImplicitRegisterUse::KWriteAccumulator;
        self.accumulator_ = TVariable::new_with_value(value);
    }

    fn clobber_accumulator(&mut self, clobber_value: Object) {
        DCHECK!(Bytecodes::clobbers_accumulator(self.bytecode_));
        self.implicit_register_use_ |= ImplicitRegisterUse::KClobberAccumulator;
        self.accumulator_ = TVariable::new_with_value(clobber_value);
    }

    fn get_context(&self) -> Context {
        self.cast(self.load_register(Register::current_context()))
    }

    fn set_context(&self, value: Context) {
        self.store_register(value, Register::current_context());
    }

    fn get_context_at_depth(&self, context: Context, depth: Uint32T) -> Context {
        let mut cur_context = TVariable::new_with_value(context);
        let mut cur_depth = TVariable::new_with_value(depth);

        let context_found = Label::new();
        let context_search = Label::new_with_vars(vec![&mut cur_depth, &mut cur_context]);

        // Fast path if the depth is 0.
        self.branch(self.word32_equal(depth, Int32Constant(0)), &context_found, &context_search);

        // Loop until the depth is 0.
        self.bind(&context_search);
        {
            cur_depth.set(self.unsigned(self.int32_sub(cur_depth.value(), Int32Constant(1))));
            cur_context.set(self.cast(self.load_context_element(cur_context.value(), Context::PREVIOUS_INDEX)));

            self.branch(self.word32_equal(cur_depth.value(), Int32Constant(0)), &context_found, &context_search);
        }

        self.bind(&context_found);
        cur_context.value()
    }

    fn register_location(&self, reg_index: IntPtrT) -> IntPtrT {
        self.signed(IntPtrAdd(self.get_interpreted_frame_pointer(), self.register_frame_offset(reg_index)))
    }

    fn register_location_reg(&self, reg: Register) -> IntPtrT {
        self.register_location(IntPtrConstant(reg.to_operand()))
    }

    fn register_frame_offset(&self, index: IntPtrT) -> IntPtrT {
        self.times_system_pointer_size(index)
    }

    fn load_register(&self, reg_index: IntPtrT) -> Object {
        self.load_full_tagged(self.get_interpreted_frame_pointer(), self.register_frame_offset(reg_index))
    }

    fn load_register_reg(&self, reg: Register) -> Object {
        self.load_full_tagged(self.get_interpreted_frame_pointer(), IntPtrConstant(reg.to_operand() * kSystemPointerSize))
    }

    fn load_and_untag_register(&self, reg: Register) -> IntPtrT {
        let base = self.get_interpreted_frame_pointer();
        let index = reg.to_operand() * kSystemPointerSize;
        if smi_values_are_32_bits() {
            // #[cfg(target_endian = "little")]
            // index += 4;

            self.change_int32_to_intptr(self.load::<Int32T>(base, IntPtrConstant(index)))
        } else {
            self.smi_to_intptr(self.cast(self.load_full_tagged(base, IntPtrConstant(index))))
        }
    }

    fn load_register_at_operand_index(&self, operand_index: i32) -> Object {
        self.load_register(self.bytecode_operand_reg(operand_index))
    }

    fn load_register_pair_at_operand_index(&self, operand_index: i32) -> (Object, Object) {
        DCHECK_EQ!(Bytecodes::get_operand_type(self.bytecode_, operand_index), OperandType::kRegPair);
        let first_reg_index = self.bytecode_operand_reg(operand_index);
        let second_reg_index = self.next_register(first_reg_index);
        (self.load_register(first_reg_index), self.load_register(second_reg_index))
    }

    fn get_register_list_at_operand_index(&self, operand_index: i32) -> RegListNodePair {
        DCHECK!(Bytecodes::is_register_list_operand_type(Bytecodes::get_operand_type(
            self.bytecode_,
            operand_index
        )));
        DCHECK_EQ!(
            Bytecodes::get_operand_type(self.bytecode_, operand_index + 1),
            OperandType::kRegCount
        );
        let base_reg = self.register_location(self.bytecode_operand_reg(operand_index));
        let reg_count = self.bytecode_operand_count(operand_index + 1);
        RegListNodePair::new(base_reg, reg_count)
    }

    fn load_register_from_register_list(&self, reg_list: &RegListNodePair, index: i32) -> Object {
        let location = self.register_location_in_register_list(reg_list, index);
        self.load_full_tagged(location)
    }

    fn register_location_in_register_list(&self, reg_list: &RegListNodePair, index: i32) -> IntPtrT {
        CSA_DCHECK!(self, self.uint32_greater_than(reg_list.reg_count(), Int32Constant(index)));
        let offset = self.register_frame_offset(IntPtrConstant(index));
        // Register indexes are negative, so subtract index from base location to get
        // location.
        self.signed(IntPtrSub(reg_list.base_reg_location(), offset))
    }

    fn store_register(&self, value: Object, reg: Register) {
        self.store_full_tagged_no_write_barrier(
            self.get_interpreted_frame_pointer(),
            IntPtrConstant(reg.to_operand() * kSystemPointerSize),
            value,
        );
    }

    fn store_register_reg_index(&self, value: Object, reg_index: IntPtrT) {
        self.store_full_tagged_no_write_barrier(
            self.get_interpreted_frame_pointer(),
            self.register_frame_offset(reg_index),
            value,
        );
    }

    fn store_register_for_short_star(&mut self, value: Object, opcode: WordT) {
        DCHECK!(Bytecodes::is_short_star(self.bytecode_));
        self.implicit_register_use_ |= ImplicitRegisterUse::KWriteShortStar;

        CSA_DCHECK!(
            self,
            self.uintptr_greater_than_or_equal(
                opcode,
                UintPtrConstant(Bytecode::kFirstShortStar as usize as u64)
            )
        );
        CSA_DCHECK!(
            self,
            self.uintptr_less_than_or_equal(
                opcode,
                UintPtrConstant(Bytecode::kLastShortStar as usize as u64)
            )
        );

        // Compute the constant that we can add to a Bytecode value to map the range
        // [Bytecode::kStar15, Bytecode::kStar0] to the range
        // [Register(15).ToOperand(), Register(0).ToOperand()].
        const SHORT_STAR_TO_OPERAND: i32 = Register(0).to_operand() - Bytecode::kStar0 as i32;

        // Make sure the values count in the right direction.
        assert_eq!(
            SHORT_STAR_TO_OPERAND,
            Register(1).to_operand() - Bytecode::kStar1 as i32
        );

        let offset = IntPtrAdd(
            self.register_frame_offset(self.signed(opcode)),
            IntPtrConstant((SHORT_STAR_TO_OPERAND * kSystemPointerSize) as i32),
        );
        self.store_full_tagged_no_write_barrier(self.get_interpreted_frame_pointer(), offset, value);
    }

    fn store_register_at_operand_index(&self, value: Object, operand_index: i32) {
        self.store_register_reg_index(value, self.bytecode_operand_reg(operand_index));
    }

    fn store_register_pair_at_operand_index(&self, value1: Object, value2: Object, operand_index: i32) {
        DCHECK_EQ!(
            Bytecodes::get_operand_type(self.bytecode_, operand_index),
            OperandType::kRegOutPair
        );
        let first_reg_index = self.bytecode_operand_reg(operand_index);
        self.store_register_reg_index(value1, first_reg_index);
        let second_reg_index = self.next_register(first_reg_index);
        self.store_register_reg_index(value2, second_reg_index);
    }

    fn store_register_triple_at_operand_index(&self, value1: Object, value2: Object, value3: Object, operand_index: i32) {
        DCHECK_EQ!(
            Bytecodes::get_operand_type(self.bytecode_, operand_index),
            OperandType::kRegOutTriple
        );
        let first_reg_index = self.bytecode_operand_reg(operand_index);
        self.store_register_reg_index(value1, first_reg_index);
        let second_reg_index = self.next_register(first_reg_index);
        self.store_register_reg_index(value2, second_reg_index);
        let third_reg_index = self.next_register(second_reg_index);
        self.store_register_reg_index(value3, third_reg_index);
    }

    fn next_register(&self, reg_index: IntPtrT) -> IntPtrT {
        // Register indexes are negative, so the next index is minus one.
        self.signed(IntPtrAdd(reg_index, IntPtrConstant(-1)))
    }

    fn operand_offset(&self, operand_index: i32) -> IntPtrT {
        IntPtrConstant(Bytecodes::get_operand_offset(
            self.bytecode_,
            operand_index,
            self.operand_scale(),
        ) as i32)
    }

    fn bytecode_operand_unsigned_byte(&self, operand_index: i32) -> Uint8T {
        DCHECK!(operand_index < Bytecodes::number_of_operands(self.bytecode_) as i32);
        DCHECK_EQ!(
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale()),
            OperandSize::kByte
        );
        let operand_offset = self.operand_offset(operand_index);
        self.load::<Uint8T>(
            self.bytecode_array_tagged_pointer(),
            IntPtrAdd(self.bytecode_offset(), operand_offset),
        )
    }

    fn bytecode_operand_signed_byte(&self, operand_index: i32) -> Int8T {
        DCHECK!(operand_index < Bytecodes::number_of_operands(self.bytecode_) as i32);
        DCHECK_EQ!(
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale()),
            OperandSize::kByte
        );
        let operand_offset = self.operand_offset(operand_index);
        self.load::<Int8T>(
            self.bytecode_array_tagged_pointer(),
            IntPtrAdd(self.bytecode_offset(), operand_offset),
        )
    }

    fn bytecode_operand_read_unaligned(&self, relative_offset: i32, result_type: MachineType) -> Word32T {
        const K_MAX_COUNT: i32 = 4;
        DCHECK!(!self.target_supports_unaligned_access());

        let count: i32;
        match result_type.representation() {
            MachineRepresentation::kWord16 => {
                count = 2;
            }
            MachineRepresentation::kWord32 => {
                count = 4;
            }
            _ => panic!("UNREACHABLE"),
        }
        let msb_type = if result_type.is_signed() {
            MachineType::Int8()
        } else {
            MachineType::Uint8()
        };

        // #[cfg(target_endian = "little")]
        let k_step = -1;
        let mut msb_offset = count - 1;

        // #[cfg(target_endian = "big")]
        // let k_step = 1;
        // let msb_offset = 0;

        // Read the most signicant bytecode into bytes[0] and then in order
        // down to least significant in bytes[count - 1].
        DCHECK!(count <= K_MAX_COUNT);
        let mut bytes = [Int32Constant(0); K_MAX_COUNT as usize];
        for i in 0..count {
            let machine_type = if i == 0 { msb_type } else { MachineType::Uint8() };
            let offset = IntPtrConstant(relative_offset + msb_offset + i * k_step);
            let array_offset = IntPtrAdd(self.bytecode_offset(), offset);
            bytes[i as usize] = self.unchecked_cast::<Word32T>(self.load(
                machine_type,
                self.bytecode_array_tagged_pointer(),
                array_offset,
            ));
        }

        // Pack LSB to MSB.
        let mut count = count;
        count -= 1;
        let mut result = bytes[count as usize];
        let mut i = 1;
        while count >= 0 {
            count -= 1;
            let shift = Int32Constant(i * kBitsPerByte);
            let value = self.word32_shl(bytes[count as usize], shift);
            result = self.word32_or(value, result);
            i += 1;
        }
        result
    }

    fn bytecode_operand_unsigned_short(&self, operand_index: i32) -> Uint16T {
        DCHECK!(operand_index < Bytecodes::number_of_operands(self.bytecode_) as i32);
        DCHECK_EQ!(
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale()),
            OperandSize::kShort
        );
        let operand_offset = Bytecodes::get_operand_offset(
            self.bytecode_,
            operand_index,
            self.operand_scale(),
        ) as i32;
        if self.target_supports_unaligned_access() {
            self.load::<Uint16T>(
                self.bytecode_array_tagged_pointer(),
                IntPtrAdd(self.bytecode_offset(), IntPtrConstant(operand_offset)),
            )
        } else {
            self.unchecked_cast::<Uint16T>(self.bytecode_operand_read_unaligned(
                operand_offset,
                MachineType::Uint16(),
            ))
        }
    }

    fn bytecode_operand_signed_short(&self, operand_index: i32) -> Int16T {
        DCHECK!(operand_index < Bytecodes::number_of_operands(self.bytecode_) as i32);
        DCHECK_EQ!(
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale()),
            OperandSize::kShort
        );
        let operand_offset = Bytecodes::get_operand_offset(
            self.bytecode_,
            operand_index,
            self.operand_scale(),
        ) as i32;
        if self.target_supports_unaligned_access() {
            self.load::<Int16T>(
                self.bytecode_array_tagged_pointer(),
                IntPtrAdd(self.bytecode_offset(), IntPtrConstant(operand_offset)),
            )
        } else {
            self.unchecked_cast::<Int16T>(self.bytecode_operand_read_unaligned(
                operand_offset,
                MachineType::Int16(),
            ))
        }
    }

    fn bytecode_operand_unsigned_quad(&self, operand_index: i32) -> Uint32T {
        DCHECK!(operand_index < Bytecodes::number_of_operands(self.bytecode_) as i32);
        DCHECK_EQ!(
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale()),
            OperandSize::kQuad
        );
        let operand_offset = Bytecodes::get_operand_offset(
            self.bytecode_,
            operand_index,
            self.operand_scale(),
        ) as i32;
        if self.target_supports_unaligned_access() {
            self.load::<Uint32T>(
                self.bytecode_array_tagged_pointer(),
                IntPtrAdd(self.bytecode_offset(), IntPtrConstant(operand_offset)),
            )
        } else {
            self.unchecked_cast::<Uint32T>(self.bytecode_operand_read_unaligned(
                operand_offset,
                MachineType::Uint32(),
            ))
        }
    }

    fn bytecode_operand_signed_quad(&self, operand_index: i32) -> Int32T {
        DCHECK!(operand_index < Bytecodes::number_of_operands(self.bytecode_) as i32);
        DCHECK_EQ!(
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale()),
            OperandSize::kQuad
        );
        let operand_offset = Bytecodes::get_operand_offset(
            self.bytecode_,
            operand_index,
            self.operand_scale(),
        ) as i32;
        if self.target_supports_unaligned_access() {
            self.load::<Int32T>(
                self.bytecode_array_tagged_pointer(),
                IntPtrAdd(self.bytecode_offset(), IntPtrConstant(operand_offset)),
            )
        } else {
            self.unchecked_cast::<Int32T>(self.bytecode_operand_read_unaligned(
                operand_offset,
                MachineType::Int32(),
            ))
        }
    }

    fn bytecode_signed_operand(&self, operand_index: i32, operand_size: OperandSize) -> Int32T {
        DCHECK!(!Bytecodes::is_unsigned_operand_type(Bytecodes::get_operand_type(
            self.bytecode_,
            operand_index
        )));
        match operand_size {
            OperandSize::kByte => self.bytecode_operand_signed_byte(operand_index),
            OperandSize::kShort => self.bytecode_operand_signed_short(operand_index),
            OperandSize::kQuad => self.bytecode_operand_signed_quad(operand_index),
            OperandSize::kNone => panic!("UNREACHABLE"),
        }
    }

    fn bytecode_unsigned_operand(&self, operand_index: i32, operand_size: OperandSize) -> Uint32T {
        DCHECK!(Bytecodes::is_unsigned_operand_type(Bytecodes::get_operand_type(
            self.bytecode_,
            operand_index
        )));
        match operand_size {
            OperandSize::kByte => self.bytecode_operand_unsigned_byte(operand_index),
            OperandSize::kShort => self.bytecode_operand_unsigned_short(operand_index),
            OperandSize::kQuad => self.bytecode_operand_unsigned_quad(operand_index),
            OperandSize::kNone => panic!("UNREACHABLE"),
        }
    }

    fn bytecode_operand_count(&self, operand_index: i32) -> Uint32T {
        DCHECK_EQ!(
            Bytecodes::get_operand_type(self.bytecode_, operand_index),
            OperandType::kRegCount
        );
        let operand_size =
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale());
        self.bytecode_unsigned_operand(operand_index, operand_size)
    }

    fn bytecode_operand_flag8(&self, operand_index: i32) -> Uint32T {
        DCHECK_EQ!(
            Bytecodes::get_operand_type(self.bytecode_, operand_index),
            OperandType::kFlag8
        );
        let operand_size =
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale());
        DCHECK_EQ!(operand_size, OperandSize::kByte);
        self.bytecode_unsigned_operand(operand_index, operand_size)
    }

    fn bytecode_operand_flag16(&self, operand_index: i32) -> Uint32T {
        DCHECK_EQ!(
            Bytecodes::get_operand_type(self.bytecode_, operand_index),
            OperandType::kFlag16
        );
        let operand_size =
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale());
        DCHECK_EQ!(operand_size, OperandSize::kShort);
        self.bytecode_unsigned_operand(operand_index, operand_size)
    }

    fn bytecode_operand_uimm(&self, operand_index: i32) -> Uint32T {
        DCHECK_EQ!(
            Bytecodes::get_operand_type(self.bytecode_, operand_index),
            OperandType::kUImm
        );
        let operand_size =
            Bytecodes::get_operand_size(self.bytecode_, operand_index, self.operand_scale());
        self.bytecode_unsigned_operand(operand_index, operand_size)
    }

    fn bytecode_operand_uimm_word(&self, operand_index: i32) -> UintPtrT {
        self.change_uint32_to_word(self.bytecode_operand_uimm(operand_index))
    }

    fn bytecode_operand_uimm_smi(&self, operand_index: i32) -> Smi {
        self.smi_from_uint32(self.bytecode_operand_uimm(operand_index))
    }

    fn bytecode_operand_imm(&self, operand_index: i32) -> Int32T {
        DCHECK_EQ