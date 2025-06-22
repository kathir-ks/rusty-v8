// src/maglev/maglev-assembler-inl.rs

// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};

//use crate::base::iterator::Iterator; // Assuming Iterator is defined elsewhere
//use crate::base::template_utils::*; // Assuming template_utils are defined elsewhere
use crate::codegen::machine_type::MachineType;
use crate::maglev::maglev_assembler::*;
use crate::compiler::object_ref::*;

// Conditional compilation based on target architecture
cfg_if::cfg_if! {
    if #[cfg(target_arch = "arm")] {
        mod arm;
        use arm::*;
    } else if #[cfg(target_arch = "aarch64")] {
        mod arm64;
        use arm64::*;
    } else if #[cfg(target_arch = "riscv64")] {
        mod riscv;
        use riscv::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x64;
        use x64::*;
    } else if #[cfg(target_arch = "s390x")] {
        mod s390;
        use s390::*;
    } else {
        compile_error!("Maglev does not support this architecture.");
    }
}

pub mod detail {

    use super::*;
    use std::{
        marker::PhantomData,
        mem,
        ops::{Deref, DerefMut},
    };

    // Base case provides an error.
    pub trait CopyForDeferred {
        fn copy_for_deferred(compilation_info: *mut MaglevCompilationInfo, value: Self) -> Self;
    }

    pub struct NoCopyHelperImplementedForType<T>(PhantomData<T>);

    // Helper for copies by value.
    pub trait CopyForDeferredByValue: CopyForDeferred {}

    impl<T: Copy> CopyForDeferred for T {
        fn copy_for_deferred(compilation_info: *mut MaglevCompilationInfo, value: Self) -> Self {
            value
        }
    }

    // Node pointers are copied by value.
    impl<T: NodeBase> CopyForDeferredByValue for *mut T {}
    impl<T: NodeBase> CopyForDeferred for *mut T {
        fn copy_for_deferred(compilation_info: *mut MaglevCompilationInfo, value: Self) -> Self {
            value
        }
    }
    // Arithmetic values and enums are copied by value.
    impl<T: std::fmt::Debug + Copy + std::ops::Add<Output = T>> CopyForDeferredByValue for T where T: num::Num {}

    // MaglevCompilationInfos are copied by value.
    impl CopyForDeferredByValue for *mut MaglevCompilationInfo {}

    // Machine registers are copied by value.
    impl CopyForDeferredByValue for Register {}
    impl CopyForDeferredByValue for Option<Register> {}
    impl CopyForDeferredByValue for DoubleRegister {}

    // Bytecode offsets are copied by value.
    impl CopyForDeferredByValue for BytecodeOffset {}

    // EagerDeoptInfo pointers are copied by value.
    impl CopyForDeferredByValue for *mut EagerDeoptInfo {}

    // LazyDeoptInfo pointers are copied by value.
    impl CopyForDeferredByValue for *mut LazyDeoptInfo {}

    // ZoneLabelRef is copied by value.
    impl CopyForDeferredByValue for ZoneLabelRef {}

    // MapCompare is copied by value.
    impl CopyForDeferredByValue for MapCompare {}

    // RegList are copied by value.
    impl CopyForDeferredByValue for RegList {}

    // Register snapshots are copied by value.
    impl CopyForDeferredByValue for RegisterSnapshot {}

    // Feedback slots are copied by value.
    impl CopyForDeferredByValue for FeedbackSlot {}

    // Heap Refs are copied by value.
    impl<T: ObjectRef> CopyForDeferredByValue for T {}

    pub fn copy_for_deferred<T: CopyForDeferred>(
        compilation_info: *mut MaglevCompilationInfo,
        value: T,
    ) -> T {
        T::copy_for_deferred(compilation_info, value)
    }

    // FunctionArgumentsTupleHelper and its specializations
    pub trait FunctionArgumentsTupleHelper {
        type FunctionPointer;
        type Tuple;
        const kSize: usize;
    }

    impl<F, R, A> FunctionArgumentsTupleHelper for fn(A...) -> R {
        type FunctionPointer = fn(A...) -> R;
        type Tuple = (A,);
        const kSize: usize = mem::size_of::<A>();
    }

    pub struct StripFirstTupleArg<T>(PhantomData<T>);

    impl<T1, T> StripFirstTupleArg<(T1, T)> {
        pub type Stripped = (T,);
    }

    pub struct DeferredCodeInfoImpl<Function> {
        function: Function,
        args: Vec<u8>, // Changed to bytes, need to manage properly
        deferred_scratch_: MaglevAssemblerTemporaryRegisterScopeSavedData,
        // allow_call_: bool, // Removed DEBUG flags
        // allow_allocate_: bool, // Removed DEBUG flags
        _phantom: PhantomData<Function>,
    }

    impl<Function> DeferredCodeInfoImpl<Function>
    where
        Function: FnMut(*mut MaglevAssembler, &mut [u8]) -> (), // Need to figure out actual call sig
    {
        pub fn new<Args>(
            compilation_info: *mut MaglevCompilationInfo,
            deferred_scratch: MaglevAssemblerTemporaryRegisterScopeSavedData,
            function: Function,
            args: Args,
        ) -> Self
        where Args: Copy {
            let args_bytes = unsafe {
                let mut args_bytes = Vec::new();
                let arg_ptr = &args as *const Args as *const u8;
                args_bytes.extend_from_slice(std::slice::from_raw_parts(arg_ptr, std::mem::size_of::<Args>()));
                args_bytes
            };

            Self {
                function,
                args: args_bytes,
                deferred_scratch_: deferred_scratch,
                _phantom: PhantomData,
            }
        }

        pub fn generate(&mut self, masm: *mut MaglevAssembler) {
            let mut scratch_scope =
                MaglevAssemblerTemporaryRegisterScope::restore(masm, self.deferred_scratch_);

            (self.function)(masm, &mut self.args);
            
        }
    }

    pub trait DeferredCodeInfo {
        fn generate(&mut self, masm: *mut MaglevAssembler);
    }

    impl<Function> DeferredCodeInfo for DeferredCodeInfoImpl<Function>
    where
        Function: FnMut(*mut MaglevAssembler, &mut [u8]) -> ()
    {
        fn generate(&mut self, masm: *mut MaglevAssembler) {
            self.generate(masm)
        }
    }
}

impl MaglevAssembler {
    pub fn make_deferred_code<F, Args>(
        &mut self,
        deferred_code_gen: F,
        args: Args,
    ) -> *mut Label
    where
        F: FnMut(*mut MaglevAssembler, &mut [u8]) -> (),
        Args: Copy,
    {
        let mut scratch_scope = MaglevAssemblerTemporaryRegisterScope::new(self);

        //let deferred_code: *mut detail::DeferredCodeInfoImpl<F> = self.compilation_info().zone().New<detail::DeferredCodeInfoImpl<F>>(

        unsafe {
            let deferred_code = (*self.compilation_info()).zone.allocate(
                detail::DeferredCodeInfoImpl::new(
                    self.compilation_info(),
                    scratch_scope.copy_for_defer(),
                    deferred_code_gen,
                    args,
                ),
            );

            // Enable call/allocate temporarily.
            // if DEBUG {
            //     deferred_code.set_allow_call(self.allow_deferred_call());
            //     deferred_code.set_allow_allocate(self.allow_allocate());
            // }

            self.code_gen_state().push_deferred_code(deferred_code as *mut dyn detail::DeferredCodeInfo); //TODO dyn DeferredCodeInfo

            &mut (*deferred_code).deferred_code_label
        }
    }

    pub fn jump_to_deferred_if<F, Args>(
        &mut self,
        cond: Condition,
        deferred_code_gen: F,
        args: Args,
    ) where
        F: FnMut(*mut MaglevAssembler, &mut [u8]) -> (),
        Args: Copy,
    {
        if v8_flags.code_comments {
            self.record_comment("-- Jump to deferred code");
        }
        let deferred_label = self.make_deferred_code(deferred_code_gen, args);
        self.jump_if(cond, unsafe { &*deferred_label }, LabelDistance::Far);
    }
    
    pub fn allocate_slow<T: num::Num + Copy>(
        &mut self,
        register_snapshot: RegisterSnapshot,
        object: Register,
        builtin: Builtin,
        size_in_bytes: T,
        done: *mut ZoneLabelRef,
    ) {
        let mut register_snapshot_copy = register_snapshot.clone();
    
        register_snapshot_copy.live_registers.clear(object);
        register_snapshot_copy.live_tagged_registers.clear(object);
    
        {
            let mut save_register_state = SaveRegisterStateForCall::new(self, register_snapshot_copy.clone());
            
            use crate::maglev::maglev_assembler::AllocateDescriptor as D;
            self.move_(D::get_register_parameter(D::kRequestedSize), size_in_bytes);
            self.call_builtin(builtin);
            save_register_state.define_safepoint();
            self.move_(object, kReturnRegister0);
        }
        self.jump(unsafe { &(**done).label }, LabelDistance::Far);
    }

    pub fn smi_to_double(&mut self, result: DoubleRegister, smi: Register) {
        self.assert_smi(smi);
        self.smi_untag(smi);
        self.int32_to_double(result, smi);
    }
}

// !defined(V8_TARGET_ARCH_RISCV64)
cfg_if::cfg_if! {
    if !#[cfg(target_arch = "riscv64")] {
        impl MaglevAssembler {
            pub fn compare_instance_type_and_jump_if(
                &mut self,
                map: Register,
                instance_type: InstanceType,
                cond: Condition,
                target: *mut Label,
                distance: LabelDistance,
            ) {
                self.compare_instance_type(map, instance_type);
                self.jump_if(cond, unsafe { &*target }, distance);
            }
    
            pub fn compare_instance_type_range_and_eager_deopt_if<NodeT: NodeBase>(
                &mut self,
                map: Register,
                instance_type_out: Register,
                lower_limit: InstanceType,
                higher_limit: InstanceType,
                cond: Condition,
                reason: DeoptimizeReason,
                node: *mut NodeT,
            ) {
                self.compare_instance_type_range(
                    map,
                    instance_type_out,
                    lower_limit,
                    higher_limit,
                );
                self.emit_eager_deopt_if(cond, reason, unsafe { &*node });
            }
    
            pub fn compare_root_and_emit_eager_deopt_if<NodeT: NodeBase>(
                &mut self,
                reg: Register,
                index: RootIndex,
                cond: Condition,
                reason: DeoptimizeReason,
                node: *mut NodeT,
            ) {
                self.compare_root(reg, index);
                self.emit_eager_deopt_if(cond, reason, unsafe { &*node });
            }
    
            pub fn compare_map_with_root_and_emit_eager_deopt_if<NodeT: NodeBase>(
                &mut self,
                reg: Register,
                index: RootIndex,
                scratch: Register,
                cond: Condition,
                reason: DeoptimizeReason,
                node: *mut NodeT,
            ) {
                self.compare_map_with_root(reg, index, scratch);
                self.emit_eager_deopt_if(cond, reason, unsafe { &*node });
            }
    
            pub fn compare_tagged_root_and_emit_eager_deopt_if<NodeT: NodeBase>(
                &mut self,
                reg: Register,
                index: RootIndex,
                cond: Condition,
                reason: DeoptimizeReason,
                node: *mut NodeT,
            ) {
                self.compare_tagged_root(reg, index);
                self.emit_eager_deopt_if(cond, reason, unsafe { &*node });
            }
    
            pub fn compare_uint32_and_emit_eager_deopt_if<NodeT: NodeBase>(
                &mut self,
                reg: Register,
                imm: i32,
                cond: Condition,
                reason: DeoptimizeReason,
                node: *mut NodeT,
            ) {
                self.cmp(reg, imm);
                self.emit_eager_deopt_if(cond, reason, unsafe { &*node });
            }
        }
    }
}

impl MaglevAssembler {
    pub fn compare_int32_and_branch(
        &mut self,
        r1: Register,
        value: i32,
        cond: Condition,
        if_true: *mut BasicBlock,
        if_false: *mut BasicBlock,
        next_block: *mut BasicBlock,
    ) {
        self.compare_int32_and_branch_(
            r1,
            value,
            cond,
            unsafe {&(**if_true).label},
            LabelDistance::Far,
            unsafe { if &(**if_true) == &(**next_block) },
            unsafe {&(**if_false).label},
            LabelDistance::Far,
            unsafe { if &(**if_false) == &(**next_block) },
        );
    }

    pub fn compare_int32_and_branch_reg(
        &mut self,
        r1: Register,
        r2: Register,
        cond: Condition,
        if_true: *mut BasicBlock,
        if_false: *mut BasicBlock,
        next_block: *mut BasicBlock,
    ) {
        self.compare_int32_and_branch_reg_(
            r1,
            r2,
            cond,
            unsafe {&(**if_true).label},
            LabelDistance::Far,
            unsafe { if &(**if_true) == &(**next_block) },
            unsafe {&(**if_false).label},
            LabelDistance::Far,
            unsafe { if &(**if_false) == &(**next_block) },
        );
    }

    pub fn compare_intptr_and_branch(
        &mut self,
        r1: Register,
        value: i32,
        cond: Condition,
        if_true: *mut BasicBlock,
        if_false: *mut BasicBlock,
        next_block: *mut BasicBlock,
    ) {
        self.compare_intptr_and_branch_(
            r1,
            value,
            cond,
            unsafe {&(**if_true).label},
            LabelDistance::Far,
            unsafe { if &(**if_true) == &(**next_block) },
            unsafe {&(**if_false).label},
            LabelDistance::Far,
            unsafe { if &(**if_false) == &(**next_block) },
        );
    }

    pub fn branch(
        &mut self,
        condition: Condition,
        if_true: *mut BasicBlock,
        if_false: *mut BasicBlock,
        next_block: *mut BasicBlock,
    ) {
        self.branch_(
            condition,
            unsafe {&(**if_true).label},
            LabelDistance::Far,
            unsafe { if &(**if_true) == &(**next_block) },
            unsafe {&(**if_false).label},
            LabelDistance::Far,
            unsafe { if &(**if_false) == &(**next_block) },
        );
    }

    pub fn branch_(
        &mut self,
        condition: Condition,
        if_true: *mut Label,
        true_distance: LabelDistance,
        fallthrough_when_true: bool,
        if_false: *mut Label,
        false_distance: LabelDistance,
        fallthrough_when_false: bool,
    ) {
        if fallthrough_when_false {
            if fallthrough_when_true {
                // If both paths are a fallthrough, do nothing.
                if unsafe { if_true == if_false } {
                    return;
                }
            }
            // Jump over the false block if true, otherwise fall through into it.
            self.jump_if(condition, unsafe { &*if_true }, true_distance);
        } else {
            // Jump to the false block if true.
            self.jump_if(self.negate_condition(condition), unsafe { &*if_false }, false_distance);
            // Jump to the true block if it's not the next block.
            if !fallthrough_when_true {
                self.jump(unsafe { &*if_true }, true_distance);
            }
        }
    }

    pub fn load_tagged_field(&mut self, result: Register, operand: MemOperand) {
        self.macro_assembler.load_tagged_field(result, operand);
    }

    pub fn load_tagged_field_reg(&mut self, result: Register, object: Register, offset: i32) {
        self.macro_assembler.load_tagged_field(result, FieldMemOperand(object, offset));
    }

    pub fn load_tagged_signed_field(&mut self, result: Register, operand: MemOperand) {
        self.macro_assembler.load_tagged_field(result, operand);
    }

    pub fn load_tagged_signed_field_reg(&mut self, result: Register, object: Register, offset: i32) {
        self.macro_assembler.load_tagged_field(result, FieldMemOperand(object, offset));
    }

    pub fn load_and_untag_tagged_signed_field(
        &mut self,
        result: Register,
        object: Register,
        offset: i32,
    ) {
        self.macro_assembler
            .smi_untag_field(result, FieldMemOperand(object, offset));
    }

    pub fn load_heap_number_or_oddball_value(
        &mut self,
        result: DoubleRegister,
        object: Register,
    ) {
        // static_assert(offsetof(HeapNumber, value_) == offsetof(Oddball, to_number_raw_));
        self.load_heap_number_value(result, object);
    }

    pub fn store_heap_number_value(&mut self, value: DoubleRegister, heap_number: Register) {
        self.macro_assembler.store_float64(
            FieldMemOperand(
                heap_number,
                mem::size_of::<HeapNumber>() as i32, //offsetof(HeapNumber, value_)
            ),
            value,
        );
    }
}

pub mod reg_detail {
    use super::*;
    #[cfg(debug_assertions)]
    pub fn clobbered_by(written_registers: RegList, reg: Register) -> bool {
        written_registers.has(reg)
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_double(written_registers: DoubleRegList, reg: DoubleRegister) -> bool {
        written_registers.has(reg)
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_handle(_written_registers: RegList, _handle: DirectHandle<Object>) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_smi(_written_registers: RegList, _smi: Tagged<Smi>) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_index(_written_registers: RegList, _index: Tagged<TaggedIndex>) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_imm(_written_registers: RegList, _imm: i32) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_root_index(_written_registers: RegList, _index: RootIndex) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_input(written_registers: RegList, input: &Input) -> bool {
        if !input.is_general_register() {
            return false;
        }
        clobbered_by(written_registers, input.assigned_general_register())
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_double_reg(written_registers: DoubleRegList, reg: Register) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_double_handle(_written_registers: DoubleRegList, _handle: DirectHandle<Object>) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_double_smi(_written_registers: DoubleRegList, _smi: Tagged<Smi>) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_double_index(_written_registers: DoubleRegList, _index: Tagged<TaggedIndex>) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_double_imm(_written_registers: DoubleRegList, _imm: i32) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_double_root_index(_written_registers: DoubleRegList, _index: RootIndex) -> bool {
        false
    }

    #[cfg(debug_assertions)]
    pub fn clobbered_by_double_input(written_registers: DoubleRegList, input: &Input) -> bool {
        if !input.is_double_register() {
            return false;
        }
        clobbered_by_double(written_registers, input.assigned_double_register())
    }
    //DEBUG end

    #[cfg(debug_assertions)]
    pub fn machine_type_matches(type_: MachineType, reg: Register) -> bool {
        !type_.is_floating_point()
    }

    #[cfg(debug_assertions)]
    pub fn machine_type_matches_double(type_: MachineType, reg: DoubleRegister) -> bool {
        type_.is_floating_point()
    }

    #[cfg(debug_assertions)]
    pub fn machine_type_matches_mem_operand(_type_: MachineType, _reg: MemOperand) -> bool {
        true
    }

    #[cfg(debug_assertions)]
    pub fn machine_type_matches_handle(type_: MachineType, _handle: DirectHandle<HeapObject>) -> bool {
        type_.is_tagged() && !type_.is_tagged_signed()
    }

    #[cfg(debug_assertions)]
    pub fn machine_type_matches_smi(type_: MachineType, _smi: Tagged<Smi>) -> bool {
        type_.is_tagged() && !type_.is_tagged_pointer()
    }

    #[cfg(debug_assertions)]
    pub fn machine_type_matches_index(type_: MachineType, _index: Tagged<TaggedIndex>) -> bool {
        // TaggedIndex doesn't have a separate type, so check for the same type as for
        // Smis.
        type_.is_tagged() && !type_.is_tagged_pointer()
    }

    #[cfg(debug_assertions)]
    pub fn machine_type_matches_imm(type_: MachineType, _imm: i32) -> bool {
        // 32-bit immediates can be used for 64-bit params -- they'll be
        // zero-extended.
        type_.representation() == MachineRepresentation::Word32
            || type_.representation() == MachineRepresentation::Word64
    }

    #[cfg(debug_assertions)]
    pub fn machine_type_matches_root_index(type_: MachineType, _index: RootIndex) -> bool {
        type_.is_tagged() && !type_.is_tagged_signed()
    }

    #[cfg(debug_assertions)]
    pub fn machine_type_matches_input(type_: MachineType, input: &Input) -> bool {
        if type_.representation() == input.node().get_machine_representation() {
            return true;
        }
        if type_.is_tagged() {
            return input.node().is_tagged();
        }
        false
    }

    #[cfg(debug_assertions)]
    pub fn check_arg<Descriptor: CallInterfaceDescriptorTrait, Arg>(
        masm: &MaglevAssembler,
        arg: &Arg,
        i: &mut i32,
    ) {
        if *i >= Descriptor::get_parameter_count() {
            assert!(Descriptor::allow_var_args());
        }
        //assert!(machine_type_matches(Descriptor::get_parameter_type(*i), arg));

        *i += 1;
    }
}

impl MaglevAssembler {
    pub fn call_builtin(&mut self, builtin: Builtin) {
        if self.allow_call() || builtin == Builtin::kDoubleToI {
            let mut reset_temps = MaglevAssemblerTemporaryRegisterScope::new(self);
            reset_temps.reset_to_default();
        
            self.macro_assembler.call_builtin(builtin);
        }
        
    }

    // TODO(leszeks): Use the parallel move helper to do register moves, instead
    // of detecting clobbering.
    #[cfg(debug_assertions)]
    fn check_args<Descriptor: CallInterfaceDescriptorTrait>(args: &mut Vec<&dyn std::any::Any>, i: &mut i32) {
        
        if *i >= Descriptor::get_parameter_count() {
            assert!(Descriptor::allow_var_args());
        }
        todo!()
    }
}

impl MaglevAssembler {
    pub fn call_runtime(&mut self, fid: RuntimeFunctionId) {
        assert!(self.allow_call());
        let mut reset_temps = MaglevAssemblerTemporaryRegisterScope::new(self);
        reset_temps.reset_to_default();

        self.macro_assembler.call_runtime(fid, 0); //TODO args
    }

    pub fn set_map_as_root(&mut self, object: Register, map: RootIndex) {
        let mut temps = MaglevAssemblerTemporaryRegisterScope::new(self);
        let scratch = temps.acquire_scratch();
        self.load_tagged_root(scratch, map);
        self.store_tagged_field_no_write_barrier(object, HeapObject::k_map_offset(), scratch);
    }

    pub fn smi_tag_int32_and_jump_if_fail(
        &mut self,
        dst: Register,
        src: Register,
        fail: *mut Label,
        distance: LabelDistance,
    ) {
        self.smi_tag_int32_and_set_flags(dst, src);
        if !self.smi_values_are_32_bits() {
            self.jump_if(Condition::kOverflow, unsafe { &*fail }, distance);
        }
    }

    pub fn smi_tag_int32_and_jump_if_fail_reg(
        &mut self,
        reg: Register,
        fail: *mut Label,
        distance: LabelDistance,
    ) {
        self.smi_tag_int32_and_jump_if_fail(reg, reg, fail, distance);
    }

    pub fn smi_tag_int32_and_jump_if_success(
        &mut self,
        dst: Register,
        src: Register,
        success: *mut Label,
        distance: LabelDistance,
    ) {
        self.smi_tag_int32_and_set_flags(dst, src);
        if !self.smi_values_are_32_bits() {
            self.jump_if(Condition::kNoOverflow, unsafe { &*success }, distance);
        } else {
            self.jmp(unsafe { &*success });
        }
    }

    pub fn smi_tag_int32_and_jump_if_success_reg(
        &mut self,
        reg: Register,
        success: *mut Label,
        distance: LabelDistance,
    ) {
        self.smi_tag_int32_and_jump_if_success(reg, reg, success, distance);
    }

    pub fn unchecked_smi_tag_int32(&mut self, dst: Register, src: Register) {
        self.smi_tag_int32_and_set_flags(dst, src);
        if !self.smi_values_are_32_bits() {
            self.assert(Condition::kNoOverflow, AbortReason::kInputDoesNotFitSmi);
        }
    }

    pub fn unchecked_smi_tag_int32_reg(&mut self, reg: Register) {
        self.unchecked_smi_tag_int32(reg, reg);
    }

    pub fn smi_tag_uint32_and_jump_if_fail(
        &mut self,
        dst: Register,
        src: Register,
        fail: *mut Label,
        distance: LabelDistance,
    ) {
        // Perform an unsigned comparison against Smi::kMaxValue.
        self.compare_int32_and_jump_if(
            src,
            Smi::k_max_value(),
            Condition::kUnsignedGreaterThan,
            unsafe { &*fail },
            distance,
        );
        self.smi_tag_int32_and_set_flags(dst, src);
        if !self.smi_values_are_32_bits() {
            self.assert(Condition::kNoOverflow, AbortReason::kInputDoesNotFitSmi);
        }
    }

    pub fn smi_tag_uint32_and_jump_if_fail_reg(
        &mut self,
        reg: Register,
        fail: *mut Label,
        distance: LabelDistance,
    ) {
        self.smi_tag_uint32_and_jump_if_fail(reg, reg, fail, distance);
    }

    pub fn smi_tag_intptr_and_jump_if_fail(
        &mut self,
        dst: Register,
        src: Register,
        fail: *mut Label,
        distance: LabelDistance,
    ) {
        self.check_intptr_is_smi(src, unsafe { &*fail }, distance);
        // If the IntPtr is in the Smi range, we can treat it as Int32.
        self.smi_tag_int32_and_set_flags(dst, src);
        if !self.smi_values_are_32_bits() {
            self.assert(Condition::kNoOverflow, AbortReason::kInputDoesNotFitSmi);
        }
    }

    pub fn smi_tag_intptr_and_jump_if_success(
        &mut self,
        dst: Register,
        src: Register,
        success: *mut Label,
        distance: LabelDistance,
    ) {
        let mut done = Label::new();
        self.smi_tag_intptr_and_jump_if_fail(dst, src, &mut done, distance);
        self.jump(unsafe { &*success }, distance);
        self.bind(&mut done);
    }

    pub fn smi_tag_uint32_and_jump_if_success(
        &mut self,
        dst: Register,
        src: Register,
        success: *mut Label,
        distance: LabelDistance,
    ) {
        let mut fail = Label::new();
        self.smi_tag_uint32_and_jump_if_fail(dst, src, &mut fail, LabelDistance::Near);
        self.jump(unsafe { &*success }, distance);
        self.bind(&mut fail);
    }

    pub fn smi_tag_uint32_and_jump_if_success_reg(
        &mut self,
        reg: Register,
        success: *mut Label,
        distance: LabelDistance,
    ) {
        self.smi_tag_uint32_and_jump_if_success(reg, reg, success, distance);
    }

    pub fn unchecked_smi_tag_uint32(&mut self, dst: Register, src: Register) {
        if v8_flags.debug_code {
            // Perform an unsigned comparison against Smi::kMaxValue.
            self.compare_int32_and_assert(
                src,
                Smi::k_max_