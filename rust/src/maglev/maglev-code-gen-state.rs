// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod maglev_code_gen_state {
    use std::cmp;
    use std::mem;

    pub struct ValueLocation {
        operand_: compiler::InstructionOperand,
    }

    impl ValueLocation {
        pub fn operand(&self) -> &compiler::InstructionOperand {
            &self.operand_
        }
    }

    pub mod compiler {
        pub struct InstructionOperand {}
        pub struct AllocatedOperand {}

        impl AllocatedOperand {
            pub fn cast(operand: &InstructionOperand) -> &AllocatedOperand {
                // Dummy implementation as we cannot instantiate AllocatedOperand directly.
                unsafe { std::mem::transmute(operand) }
            }

            pub fn get_register(&self) -> Register {
                // Placeholder register value
                Register {}
            }

            pub fn get_double_register(&self) -> DoubleRegister {
                // Placeholder double register value
                DoubleRegister {}
            }
        }

        pub struct NativeContextRef {}
        pub struct JSHeapBroker {}
    }

    pub mod codegen {
        pub struct Label {}
        pub struct Assembler {}
        pub enum MachineType {}
        pub struct MaglevSafepointTableBuilder {}
    }

    pub mod common {
        pub const kMaxUInt32: u32 = u32::MAX;
        pub const kSystemPointerSize: i32 = 8;
    }

    pub mod execution {
        pub mod frame_constants {
            pub const kFixedFrameSize: i32 = 96;
            pub const kFixedSlotCount: i32 = 12;
        }
    }

    pub mod maglev {
        use super::*;
        use std::borrow::Borrow;
        use std::cell::RefCell;

        pub struct MaglevCompilationInfo<'a> {
            broker_: &'a compiler::JSHeapBroker,
            graph_labeller_: &'a MaglevGraphLabeller,
            toplevel_compilation_unit_: &'a ToplevelCompilationUnit,
        }

        impl<'a> MaglevCompilationInfo<'a> {
            pub fn broker(&self) -> &compiler::JSHeapBroker {
                self.broker_
            }
            pub fn graph_labeller(&self) -> &MaglevGraphLabeller {
                self.graph_labeller_
            }
            pub fn toplevel_compilation_unit(&self) -> &ToplevelCompilationUnit {
                self.toplevel_compilation_unit_
            }
        }

        pub struct MaglevGraphLabeller {}

        pub struct ToplevelCompilationUnit {
            parameter_count_: u16,
        }
        impl ToplevelCompilationUnit {
            pub fn parameter_count(&self) -> u16 {
                self.parameter_count_
            }
        }

        pub struct EagerDeoptInfo {}
        pub struct LazyDeoptInfo {}
        pub struct MaglevAssembler {}

        pub trait NodeBase {}

        pub trait DeferredCodeInfo {
            fn generate(&self, masm: &mut MaglevAssembler);
            fn deferred_code_label(&self) -> &codegen::Label;
        }

        pub struct DeferredCodeInfoImpl {
            deferred_code_label_: codegen::Label,
        }

        impl DeferredCodeInfoImpl {
            pub fn new() -> Self {
                DeferredCodeInfoImpl {
                    deferred_code_label_: codegen::Label {},
                }
            }
        }

        impl DeferredCodeInfo for DeferredCodeInfoImpl {
            fn generate(&self, _masm: &mut MaglevAssembler) {
                // Implementation for generate
            }
            fn deferred_code_label(&self) -> &codegen::Label {
                &self.deferred_code_label_
            }
        }

        pub struct MaglevCodeGenState<'a> {
            compilation_info_: &'a MaglevCompilationInfo<'a>,
            safepoint_table_builder_: &'a codegen::MaglevSafepointTableBuilder,
            deferred_code_: RefCell<Vec<Box<dyn DeferredCodeInfo>>>,
            eager_deopts_: Vec<*mut EagerDeoptInfo>,
            lazy_deopts_: Vec<*mut LazyDeoptInfo>,
            handlers_: Vec<*mut dyn NodeBase>,
            untagged_slots_: i32,
            tagged_slots_: i32,
            max_deopted_stack_size_: u32,
            max_call_stack_args_: u32,
            entry_label_: codegen::Label,
            osr_entry_: codegen::Label,
        }

        impl<'a> MaglevCodeGenState<'a> {
            pub fn new(
                compilation_info: &'a MaglevCompilationInfo<'a>,
                safepoint_table_builder: &'a codegen::MaglevSafepointTableBuilder,
            ) -> Self {
                MaglevCodeGenState {
                    compilation_info_: compilation_info,
                    safepoint_table_builder_: safepoint_table_builder,
                    deferred_code_: RefCell::new(Vec::new()),
                    eager_deopts_: Vec::new(),
                    lazy_deopts_: Vec::new(),
                    handlers_: Vec::new(),
                    untagged_slots_: 0,
                    tagged_slots_: 0,
                    max_deopted_stack_size_: common::kMaxUInt32,
                    max_call_stack_args_: common::kMaxUInt32,
                    entry_label_: codegen::Label {},
                    osr_entry_: codegen::Label {},
                }
            }

            pub fn set_tagged_slots(&mut self, slots: i32) {
                self.tagged_slots_ = slots;
            }
            pub fn set_untagged_slots(&mut self, slots: i32) {
                self.untagged_slots_ = slots;
            }

            pub fn push_deferred_code(&self, deferred_code: Box<dyn DeferredCodeInfo>) {
                self.deferred_code_.borrow_mut().push(deferred_code);
            }

            pub fn deferred_code(&self) -> std::cell::Ref<Vec<Box<dyn DeferredCodeInfo>>> {
                self.deferred_code_.borrow()
            }

            pub fn take_deferred_code(&mut self) -> Vec<Box<dyn DeferredCodeInfo>> {
                mem::take(&mut *self.deferred_code_.borrow_mut())
            }
            pub fn push_eager_deopt(&mut self, info: *mut EagerDeoptInfo) {
                self.eager_deopts_.push(info);
            }
            pub fn push_lazy_deopt(&mut self, info: *mut LazyDeoptInfo) {
                self.lazy_deopts_.push(info);
            }
            pub fn eager_deopts(&self) -> &Vec<*mut EagerDeoptInfo> {
                &self.eager_deopts_
            }
            pub fn lazy_deopts(&self) -> &Vec<*mut LazyDeoptInfo> {
                &self.lazy_deopts_
            }

            pub fn push_handler_info(&mut self, node: *mut dyn NodeBase) {
                self.handlers_.push(node);
            }
            pub fn handlers(&self) -> &Vec<*mut dyn NodeBase> {
                &self.handlers_
            }

            pub fn native_context(&self) -> &compiler::NativeContextRef {
                self.broker().target_native_context()
            }
            pub fn broker(&self) -> &compiler::JSHeapBroker {
                self.compilation_info_.broker()
            }
            pub fn graph_labeller(&self) -> &MaglevGraphLabeller {
                self.compilation_info_.graph_labeller()
            }
            pub fn stack_slots(&self) -> i32 {
                self.untagged_slots_ + self.tagged_slots_
            }
            pub fn tagged_slots(&self) -> i32 {
                self.tagged_slots_
            }

            pub fn parameter_count(&self) -> u16 {
                self.compilation_info_
                    .toplevel_compilation_unit()
                    .parameter_count()
            }

            pub fn safepoint_table_builder(&self) -> &codegen::MaglevSafepointTableBuilder {
                self.safepoint_table_builder_
            }
            pub fn compilation_info(&self) -> &MaglevCompilationInfo<'a> {
                self.compilation_info_
            }

            pub fn entry_label(&mut self) -> &mut codegen::Label {
                &mut self.entry_label_
            }

            pub fn set_max_deopted_stack_size(&mut self, max_deopted_stack_size: u32) {
                self.max_deopted_stack_size_ = max_deopted_stack_size;
            }

            pub fn set_max_call_stack_args_(&mut self, max_call_stack_args: u32) {
                self.max_call_stack_args_ = max_call_stack_args;
            }

            pub fn stack_check_offset(&self) -> u32 {
                let parameter_slots = self
                    .compilation_info_
                    .toplevel_compilation_unit()
                    .parameter_count() as i32;
                let stack_slots = self.tagged_slots_ + self.untagged_slots_;

                let optimized_frame_height = parameter_slots * common::kSystemPointerSize
                    + execution::frame_constants::kFixedFrameSize
                    + stack_slots * common::kSystemPointerSize;

                let signed_max_unoptimized_frame_height =
                    self.max_deopted_stack_size_ as i32;

                let frame_height_delta = cmp::max(
                    signed_max_unoptimized_frame_height - optimized_frame_height,
                    0,
                ) as u32;
                let max_pushed_argument_bytes =
                    (self.max_call_stack_args_ as i32 * common::kSystemPointerSize) as u32;
                cmp::max(frame_height_delta, max_pushed_argument_bytes)
            }

            pub fn osr_entry(&mut self) -> &mut codegen::Label {
                &mut self.osr_entry_
            }
        }

        pub fn get_safepoint_index_for_stack_slot(i: i32) -> i32 {
            execution::frame_constants::kFixedSlotCount + i
        }

        #[derive(Debug)]
        pub struct Register {}

        #[derive(Debug)]
        pub struct DoubleRegister {}

        pub fn to_register(operand: &compiler::InstructionOperand) -> Register {
            compiler::AllocatedOperand::cast(operand).get_register()
        }

        pub fn to_double_register(operand: &compiler::InstructionOperand) -> DoubleRegister {
            compiler::AllocatedOperand::cast(operand).get_double_register()
        }

        pub fn to_register_t<T>(operand: &compiler::InstructionOperand) -> T
        where
            T: From<Register> + From<DoubleRegister>,
        {
            let register = compiler::AllocatedOperand::cast(operand).get_register();
            T::from(register)
        }

        impl From<Register> for Register {
            fn from(reg: Register) -> Self {
                reg
            }
        }

        impl From<DoubleRegister> for Register {
            fn from(_reg: DoubleRegister) -> Self {
                // This conversion might lose information, handle carefully
                Register {}
            }
        }

        impl From<Register> for DoubleRegister {
            fn from(_reg: Register) -> Self {
                //This conversion might lose information, handle carefully
                DoubleRegister {}
            }
        }

        impl From<DoubleRegister> for DoubleRegister {
            fn from(reg: DoubleRegister) -> Self {
                reg
            }
        }

        pub fn to_register_value_location(location: &ValueLocation) -> Register {
            to_register(location.operand())
        }

        pub fn to_double_register_value_location(location: &ValueLocation) -> DoubleRegister {
            to_double_register(location.operand())
        }
    }
}