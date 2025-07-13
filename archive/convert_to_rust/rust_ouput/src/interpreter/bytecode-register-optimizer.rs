// Converted from V8 C++ source files:
// Header: bytecode-register-optimizer.h
// Implementation: bytecode-register-optimizer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

pub struct Register {
    index_: i32,
}

impl Register {
    pub fn virtual_accumulator() -> Self {
        Register { index_: -1 }
    }
    pub fn FromParameterIndex(index: i32) -> Self {
        Register { index_: index }
    }
    pub fn index(&self) -> i32 {
        self.index_
    }
    pub fn FromOperand(operand: i32) -> Self {
        Register{ index_: operand }
    }
}

pub struct RegisterList {
    first_register_: Register,
    register_count_: i32,
}

impl RegisterList {
    pub fn first_register(&self) -> &Register {
        &self.first_register_
    }
    pub fn register_count(&self) -> i32 {
        self.register_count_
    }
    pub fn new(first_register: Register, register_count: i32) -> Self {
        RegisterList{
            first_register_: first_register,
            register_count_: register_count,
        }
    }
    pub fn from_register(reg: Register) -> Self {
        RegisterList {
            first_register_: reg,
            register_count_: 1,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TypeHint {
    kAny,
}

pub enum ImplicitRegisterUse {}

pub struct Variable {}

pub enum Bytecode {}

pub mod Bytecodes {
    use super::Bytecode;
    pub fn IsJump(_bytecode: Bytecode) -> bool {
        false
    }
    pub fn IsSwitch(_bytecode: Bytecode) -> bool {
        false
    }
}

pub mod BytecodeOperands {
    use super::ImplicitRegisterUse;
    pub fn ReadsAccumulator(_implicit_register_use: ImplicitRegisterUse) -> bool {
        false
    }
    pub fn WritesOrClobbersAccumulator(_implicit_register_use: ImplicitRegisterUse) -> bool {
        false
    }
}

pub struct BytecodeGenerator {}

impl BytecodeGenerator {
    pub fn IsSameOrSubTypeHint(_a: TypeHint, _b: TypeHint) -> bool {
        false
    }
}

pub struct Zone {}

impl Zone {
    pub fn new() -> Self {
        Zone {}
    }
    pub fn allocate<T>(&self, value: T) -> Box<T> {
        Box::new(value)
    }
}

pub struct ZoneObject {}

pub struct BytecodeRegisterAllocator {}

impl BytecodeRegisterAllocator {
    pub fn set_observer(&mut self, _observer: *const ()) {}
    pub fn new() -> Self {
        BytecodeRegisterAllocator {}
    }
}

pub trait Observer {
    fn RegisterAllocateEvent(&mut self, _reg: Register);
    fn RegisterListAllocateEvent(&mut self, _reg_list: RegisterList);
    fn RegisterListFreeEvent(&mut self, _reg: RegisterList);
    fn RegisterFreeEvent(&mut self, _reg: Register);
}

const kMaxUInt32: u32 = u32::MAX;

pub struct BytecodeRegisterOptimizer<'a> {
    accumulator_: Register,
    accumulator_info_: Box<RegisterInfo>,
    temporary_base_: Register,
    max_register_index_: i32,
    register_info_table_: Vec<Box<RegisterInfo>>,
    register_info_table_offset_: i32,
    registers_needing_flushed_: VecDeque<*mut RegisterInfo>,
    equivalence_id_: u32,
    bytecode_writer_: &'a mut dyn BytecodeWriter,
    flush_required_: bool,
    zone_: &'a Zone,
}

impl<'a> BytecodeRegisterOptimizer<'a> {
    const kInvalidEquivalenceId: u32 = kMaxUInt32;

    pub fn new(
        zone: &'a Zone,
        register_allocator: &mut BytecodeRegisterAllocator,
        fixed_registers_count: i32,
        parameter_count: i32,
        bytecode_writer: &'a mut dyn BytecodeWriter,
    ) -> Self {
        register_allocator.set_observer(std::ptr::null());

        let accumulator_ = Register::virtual_accumulator();
        let temporary_base_ = Register { index_: fixed_registers_count };
        let max_register_index_ = fixed_registers_count - 1;
        let mut register_info_table_: Vec<Box<RegisterInfo>> = Vec::new();
        let registers_needing_flushed_: VecDeque<*mut RegisterInfo> = VecDeque::new();
        let equivalence_id_: u32 = 0;
        let flush_required_: bool = false;

        // Calculate offset so register index values can be mapped into
        // a vector of register metadata.
        // There is at least one parameter, which is the JS receiver.
        assert_ne!(parameter_count, 0);
        let first_slot_index = parameter_count - 1;
        let register_info_table_offset_ = -Register::FromParameterIndex(first_slot_index).index();

        // Initialize register map for parameters, locals, and the
        // accumulator.
        register_info_table_.resize((register_info_table_offset_ + temporary_base_.index()) as usize);
        for i in 0..register_info_table_.len() {
            let reg = Self::RegisterFromRegisterInfoTableIndex(i as usize, register_info_table_offset_);
            register_info_table_[i] = Box::new(RegisterInfo::new(
                reg,
                Self::NextEquivalenceId_static(i as u32),
                true,
                true,
            ));
            assert_eq!(register_info_table_[i].register_value().index(), Self::RegisterFromRegisterInfoTableIndex(i as usize, register_info_table_offset_).index());
        }

        let accumulator_info_ = register_info_table_[Self::GetRegisterInfoTableIndex_static(&accumulator_, register_info_table_offset_) as usize].as_mut() as *mut RegisterInfo;

        BytecodeRegisterOptimizer {
            accumulator_: accumulator_,
            accumulator_info_: unsafe { Box::from_raw(accumulator_info_) },
            temporary_base_: temporary_base_,
            max_register_index_: max_register_index_,
            register_info_table_: register_info_table_,
            register_info_table_offset_: register_info_table_offset_,
            registers_needing_flushed_: registers_needing_flushed_,
            equivalence_id_: equivalence_id_,
            bytecode_writer_: bytecode_writer,
            flush_required_: flush_required_,
            zone_: zone,
        }
    }

    fn PushToRegistersNeedingFlush(&mut self, reg: *mut RegisterInfo) {
        // Flushing is required in two cases:
        // 1) Two or more registers in the same equivalence set.
        // 2) Binding a variable to a register.
        self.flush_required_ = true;
        let reg_info = unsafe { &mut *reg };
        if !reg_info.needs_flush() {
            reg_info.set_needs_flush(true);
            self.registers_needing_flushed_.push_back(reg);
        }
    }

    fn EnsureAllRegistersAreFlushed(&self) -> bool {
        for reg_info in &self.register_info_table_ {
            if reg_info.needs_flush() {
                return false;
            } else if !reg_info.IsOnlyMemberOfEquivalenceSet() {
                return false;
            } else if reg_info.allocated() && !reg_info.materialized() {
                return false;
            }
        }
        true
    }

    fn Flush(&mut self) {
        if !self.flush_required_ {
            return;
        }

        // Materialize all live registers and break equivalences.
        for reg_ptr in &self.registers_needing_flushed_.clone() {
            let reg_info = unsafe { &mut **reg_ptr };
            if !reg_info.needs_flush() {
                continue;
            }
            reg_info.set_needs_flush(false);
            reg_info.flush_variable_hint(false);
            reg_info.set_type_hint(TypeHint::kAny);

            let materialized = if reg_info.materialized() {
                Some(reg_info as *mut RegisterInfo)
            } else {
                reg_info.GetMaterializedEquivalent()
            };

            if let Some(materialized_ptr) = materialized {
                let materialized = unsafe { &mut *materialized_ptr };
                // Walk equivalents of materialized registers, materializing
                // each equivalent register as necessary and placing in their
                // own equivalence set.
                let mut equivalent = materialized.GetEquivalent();
                while equivalent != Some(materialized as *mut RegisterInfo) {
                    let equivalent_reg_info = unsafe { &mut *equivalent.unwrap() };
                    if equivalent_reg_info.allocated() && !equivalent_reg_info.materialized() {
                        self.OutputRegisterTransfer(materialized as *mut RegisterInfo, equivalent.unwrap());
                    }
                    equivalent_reg_info.MoveToNewEquivalenceSet(self.NextEquivalenceId(), MaterializedInfo::kMaterialized, ResetVariableHint::kDontReset);
                    equivalent_reg_info.set_needs_flush(false);
                    equivalent = materialized.GetEquivalent();
                }
            } else {
                // Equivalence class containing only unallocated registers.
                assert!(reg_info.GetAllocatedEquivalent().is_none());
                reg_info.MoveToNewEquivalenceSet(self.NextEquivalenceId(), MaterializedInfo::kNotMaterialized, ResetVariableHint::kDontReset);
            }
        }

        self.registers_needing_flushed_.clear();
        assert!(self.EnsureAllRegistersAreFlushed());

        self.flush_required_ = false;
    }

    fn OutputRegisterTransfer(&mut self, input_info_ptr: *mut RegisterInfo, output_info_ptr: *mut RegisterInfo) {
        let input_info = unsafe { &*input_info_ptr };
        let output_info = unsafe { &mut *output_info_ptr };
        let input = input_info.register_value();
        let output = output_info.register_value();
        assert_ne!(input.index(), output.index());

        if input.index() == self.accumulator_.index() {
            self.bytecode_writer_.EmitStar(output);
        } else if output.index() == self.accumulator_.index() {
            self.bytecode_writer_.EmitLdar(input);
        } else {
            self.bytecode_writer_.EmitMov(input, output);
        }
        if output.index() != self.accumulator_.index() {
            self.max_register_index_ = std::cmp::max(self.max_register_index_, output.index());
        }
        output_info.set_materialized(true);
    }

    fn CreateMaterializedEquivalent(&mut self, info_ptr: *mut RegisterInfo) {
        let info = unsafe { &*info_ptr };
        assert!(info.materialized());
        if let Some(unmaterialized_ptr) = info.GetEquivalentToMaterialize() {
            self.OutputRegisterTransfer(info_ptr, unmaterialized_ptr);
        }
    }

    fn GetMaterializedEquivalentNotAccumulator(&mut self, info_ptr: *mut RegisterInfo) -> *mut RegisterInfo {
        let info = unsafe { &mut *info_ptr };
        if info.materialized() {
            return info;
        }

        let result = info.GetMaterializedEquivalentOtherThan(self.accumulator_);
        let result_ptr = match result {
            Some(ptr) => ptr,
            None => {
                self.Materialize(info_ptr);
                info
            }
        };
        let result_info = unsafe { &*result_ptr };
        assert_ne!(result_info.register_value().index(), self.accumulator_.index());
        result_ptr
    }

    fn Materialize(&mut self, info_ptr: *mut RegisterInfo) {
        let info = unsafe { &mut *info_ptr };
        if !info.materialized() {
            if let Some(materialized_ptr) = info.GetMaterializedEquivalent() {
                self.OutputRegisterTransfer(materialized_ptr, info_ptr);
            }
        }
    }

    fn AddToEquivalenceSet(&mut self, set_member_ptr: *mut RegisterInfo, non_set_member_ptr: *mut RegisterInfo) {
        // Equivalence class is now of size >= 2, so we make sure it will be flushed.
        self.PushToRegistersNeedingFlush(non_set_member_ptr);
        let non_set_member = unsafe {&mut *non_set_member_ptr};
        let set_member = unsafe {&mut *set_member_ptr};
        non_set_member.AddToEquivalenceSetOf(set_member);
    }

    fn RegisterTransfer(&mut self, input_info_ptr: *mut RegisterInfo, output_info_ptr: *mut RegisterInfo) {
        let input_info = unsafe { &mut *input_info_ptr };
        let output_info = unsafe { &mut *output_info_ptr };
        let output_is_observable = self.RegisterIsObservable(output_info.register_value());
        let in_same_equivalence_set = output_info.IsInSameEquivalenceSet(input_info);
        if in_same_equivalence_set
            && (!output_is_observable || output_info.materialized())
        {
            return; // Nothing more to do.
        }

        // Materialize an alternate in the equivalence set that
        // |output_info| is leaving.
        if output_info.materialized() {
            self.CreateMaterializedEquivalent(output_info_ptr);
        }

        // Add |output_info| to new equivalence set.
        if !in_same_equivalence_set {
            self.AddToEquivalenceSet(input_info_ptr, output_info_ptr);
        }

        if output_is_observable {
            // Force store to be emitted when register is observable.
            output_info.set_materialized(false);
            let materialized_info = input_info.GetMaterializedEquivalent();
            if let Some(materialized_info) = materialized_info {
                self.OutputRegisterTransfer(materialized_info, output_info_ptr);
            }
        }

        let input_is_observable = self.RegisterIsObservable(input_info.register_value());
        if input_is_observable {
            // If input is observable by the debugger, mark all other temporaries
            // registers as unmaterialized so that this register is used in preference.
            input_info.MarkTemporariesAsUnmaterialized(self.temporary_base_);
        }
    }

    fn PrepareOutputRegister(&mut self, reg: Register) {
        let reg_info_ptr = self.GetRegisterInfo(reg);
        let reg_info = unsafe { &mut *reg_info_ptr };
        if reg_info.materialized() {
            self.CreateMaterializedEquivalent(reg_info_ptr);
        }
        reg_info.MoveToNewEquivalenceSet(
            self.NextEquivalenceId(),
            MaterializedInfo::kMaterialized,
        );
        self.max_register_index_ =
            std::cmp::max(self.max_register_index_, reg_info.register_value().index());
    }

    fn PrepareOutputRegisterList(&mut self, reg_list: RegisterList) {
        let start_index = reg_list.first_register().index();
        for i in 0..reg_list.register_count() {
            let current = Register { index_: start_index + i };
            self.PrepareOutputRegister(current);
        }
    }

    fn GetInputRegister(&mut self, reg: Register) -> Register {
        let reg_info = self.GetRegisterInfo(reg);
        let reg_info_val = unsafe { &*reg_info };
        if reg_info_val.materialized() {
            return reg;
        } else {
            let equivalent_info_ptr = self.GetMaterializedEquivalentNotAccumulator(reg_info);
            let equivalent_info = unsafe { &*equivalent_info_ptr };
            return equivalent_info.register_value();
        }
    }

    fn GetInputRegisterList(&mut self, reg_list: RegisterList) -> RegisterList {
        if reg_list.register_count() == 1 {
            // If there is only a single register, treat it as a normal input register.
            let reg = self.GetInputRegister(reg_list.first_register().clone());
            return RegisterList {
                first_register_: reg,
                register_count_: 1,
            };
        } else {
            let start_index = reg_list.first_register().index();
            for i in 0..reg_list.register_count() {
                let current = Register { index_: start_index + i };
                let input_info = self.GetRegisterInfo(current);
                self.Materialize(input_info);
            }
            return reg_list;
        }
    }

    fn GrowRegisterMap(&mut self, reg: Register) {
        assert!(self.RegisterIsTemporary(reg));
        let index = Self::GetRegisterInfoTableIndex_static(&reg, self.register_info_table_offset_) as usize;
        if index >= self.register_info_table_.len() {
            let new_size = index + 1;
            let old_size = self.register_info_table_.len();
            self.register_info_table_.resize(new_size);
            for i in old_size..new_size {
                let new_reg = Self::RegisterFromRegisterInfoTableIndex(i, self.register_info_table_offset_);
                self.register_info_table_[i] = Box::new(RegisterInfo::new(
                    new_reg,
                    Self::NextEquivalenceId_static(i as u32),
                    true,
                    false,
                ));
            }
        }
    }

    fn AllocateRegister(&mut self, info_ptr: *mut RegisterInfo) {
        let info = unsafe { &mut *info_ptr };
        info.set_allocated(true);
        if !info.materialized() {
            info.MoveToNewEquivalenceSet(
                self.NextEquivalenceId(),
                MaterializedInfo::kMaterialized,
            );
        }
    }

    pub fn RegisterAllocateEvent(&mut self, reg: Register) {
        let reg_info = self.GetOrCreateRegisterInfo(reg);
        self.AllocateRegister(reg_info);
    }

    pub fn RegisterListAllocateEvent(&mut self, reg_list: RegisterList) {
        if reg_list.register_count() != 0 {
            let first_index = reg_list.first_register().index();
            self.GrowRegisterMap(Register {
                index_: first_index + reg_list.register_count() - 1,
            });
            for i in 0..reg_list.register_count() {
                let reg = Register { index_: first_index + i };
                let reg_info = self.GetRegisterInfo(reg);
                self.AllocateRegister(reg_info);
            }
        }
    }

    pub fn RegisterListFreeEvent(&mut self, reg_list: RegisterList) {
        let first_index = reg_list.first_register().index();
        for i in 0..reg_list.register_count() {
            let reg = Register { index_: first_index + i };
            self.GetRegisterInfo(reg).set_allocated(false);
        }
    }

    pub fn RegisterFreeEvent(&mut self, reg: Register) {
        self.GetRegisterInfo(reg).set_allocated(false);
    }

    fn RegisterIsTemporary(&self, reg: Register) -> bool {
        reg.index() >= self.temporary_base_.index()
    }

    fn RegisterIsObservable(&self, reg: Register) -> bool {
        reg.index() != self.accumulator_.index() && !self.RegisterIsTemporary(reg)
    }

    fn OperandToRegister(operand: u32) -> Register {
        Register::FromOperand(operand as i32)
    }

    fn GetRegisterInfoTableIndex_static(reg: &Register, register_info_table_offset_: i32) -> i32 {
        reg.index() + register_info_table_offset_
    }

    fn RegisterFromRegisterInfoTableIndex(index: usize, register_info_table_offset_: i32) -> Register {
        Register {
            index: index as i32 - register_info_table_offset_,
        }
    }

    fn NextEquivalenceId_static(equivalence_id_: u32) -> u32 {
        let mut equivalence_id = equivalence_id_;
        equivalence_id += 1;
        assert_ne!(equivalence_id, Self::kInvalidEquivalenceId);
        equivalence_id
    }

    fn NextEquivalenceId(&mut self) -> u32 {
        self.equivalence_id_ += 1;
        assert_ne!(self.equivalence_id_, Self::kInvalidEquivalenceId);
        self.equivalence_id_
    }

    fn zone(&self) -> &Zone {
        self.zone_
    }

    fn GetRegisterInfoTableIndex(&self, reg: Register) -> i32 {
        Self::GetRegisterInfoTableIndex_static(&reg, self.register_info_table_offset_)
    }

    fn GetRegisterInfo(&self, reg: Register) -> *mut RegisterInfo {
        let index = self.GetRegisterInfoTableIndex(reg) as usize;
        assert!(index < self.register_info_table_.len());
        self.register_info_table_[index].as_mut() as *mut RegisterInfo
    }

    fn GetOrCreateRegisterInfo(&mut self, reg: Register) -> *mut RegisterInfo {
        let index = self.GetRegisterInfoTableIndex(reg) as usize;
        if index < self.register_info_table_.len() {
            self.register_info_table_[index].as_mut() as *mut RegisterInfo
        } else {
            self.NewRegisterInfo(reg)
        }
    }

    fn NewRegisterInfo(&mut self, reg: Register) -> *mut RegisterInfo {
        let index = self.GetRegisterInfoTableIndex(reg) as usize;
        assert!(index >= self.register_info_table_.len());
        self.GrowRegisterMap(reg);
        self.register_info_table_[index].as_mut() as *mut RegisterInfo
    }

    fn DoLdar(&mut self, input: Register) {
        let input_info = self.GetRegisterInfo(input);
        self.RegisterTransfer(input_info, self.accumulator_info_.as_mut());
    }
    fn DoStar(&mut self, output: Register) {
        let output_info = self.GetRegisterInfo(output);
        self.RegisterTransfer(
            self.accumulator_info_.as_mut(),
            output_info,
        );
    }
    fn DoMov(&mut self, input: Register, output: Register) {
        let input_info = self.GetRegisterInfo(input);
        let output_info = self.GetRegisterInfo(output);
        self.RegisterTransfer(input_info, output_info);
    }
    fn PrepareForBytecode<const JUMP: bool>(&mut self, implicit_register_use: ImplicitRegisterUse) {
        self.Flush();
    }

    fn SetVariableInRegister(&mut self, var: *mut Variable, reg: Register) {
        let info = self.GetRegisterInfo(reg);
        let info_val = unsafe { &mut *info };

        let mut current = info_val;
        loop {
            self.PushToRegistersNeedingFlush(current);
            current.set_variable_hint(VariableHint {
                variable: var,
                mode: VariableHintMode::kDefinitelyHasVariable,
            });

            let next_reg = current.next;
            let info_next = unsafe { &mut *next_reg };
            if std::ptr::eq(info_next, info_val) {
                break;
            }

            current = info_next;
        }
    }

    fn GetPotentialVariableInRegister(&self, reg: Register) -> *mut Variable {
        let info = self.GetRegisterInfo(reg);
        let info_val = unsafe { &*info };
        info_val.variable_hint().variable
    }

    fn IsVariableInRegister(&self, var: *mut Variable, reg: Register) -> bool {
        let info = self.GetRegisterInfo(reg);
        let info_val = unsafe { &*info };

        let hint = info_val.variable_hint();
        hint.mode == VariableHintMode::kDefinitelyHasVariable && hint.variable == var
    }

    fn GetTypeHint(&self, reg: Register) -> TypeHint {
        let info = self.GetRegisterInfo(reg);
        let info_val = unsafe { &*info };

        info_val.type_hint()
    }

    fn SetTypeHintForAccumulator(&mut self, hint: TypeHint) {
        if self.accumulator_info_.type_hint() != hint {
            self.accumulator_info_.set_type_hint(hint);
        }
    }
    fn ResetTypeHintForAccumulator(&mut self) {
        self.accumulator_info_.set_type_hint(TypeHint::kAny);
    }
    fn IsAccumulatorReset(&self) -> bool {
        self.accumulator_info_.type_hint() == TypeHint::kAny
    }

    fn maxiumum_register_index(&self) -> i32 {
        self.max_register_index_
    }

}

impl<'a> Observer for BytecodeRegisterOptimizer<'a> {
    fn RegisterAllocateEvent(&mut self, reg: Register) {
        self.RegisterAllocateEvent(reg);
    }
    fn RegisterListAllocateEvent(&mut self, reg_list: RegisterList) {
        self.RegisterListAllocateEvent(reg_list);
    }
    fn RegisterListFreeEvent(&mut self, reg: RegisterList) {
        self.RegisterListFreeEvent(reg);
    }
    fn RegisterFreeEvent(&mut self, reg: Register) {
        self.RegisterFreeEvent(reg);
    }
}

#[derive(Debug, PartialEq)]
enum VariableHintMode {
    kDefinitelyHasVariable,
    kMightHaveVariable,
}

#[derive(Debug, PartialEq)]
struct VariableHint {
    variable: *mut Variable,
    mode: VariableHintMode,
}

#[derive(Debug, PartialEq)]
enum MaterializedInfo {
    kNotMaterialized,
    kMaterialized,
}

#[derive(Debug, PartialEq)]
enum ResetVariableHint {
    kDontReset,
    kReset,
}

struct RegisterInfo {
    register_: Register,
    equivalence_id_: u32,
    materialized_: bool,
    allocated_: bool,
    needs_flush_: bool,
    type_hint_: TypeHint,
    variable_hint_: VariableHint,
    next: *mut RegisterInfo,
    prev: *mut RegisterInfo,
}

impl RegisterInfo {
    fn new(reg: Register, equivalence_id: u32, materialized: bool, allocated: bool) -> Self {
        RegisterInfo {
            register_: reg,
            equivalence_id_: equivalence_id,
            materialized_: materialized,
            allocated_: allocated,
            needs_flush_: false,
            type_hint_: TypeHint::kAny,
            variable_hint_: VariableHint {
                variable: std::ptr::null_mut(),
                mode: VariableHintMode::kDefinitelyHasVariable,
            },
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        }
    }

    fn AddToEquivalenceSetOf(&mut self, info: &mut RegisterInfo) {
        assert_ne!(kMaxUInt32, info.equivalence_id());
        let info_next = unsafe {&mut *info.next};
        let info_prev = unsafe {&mut *info.prev};
        
        // Fix old list
        let self_next = unsafe{&mut *self.next};
        let self_prev = unsafe{&mut *self.prev};
        
        self_next.prev = self.prev;
        self_prev.next = self.next;
        // Add to new list.
        
        self.next = info.next;
        self.prev = Some(info);
        unsafe{(&mut *info_next).prev = Some(self)};
        unsafe{(&mut *info.next).prev = Some(self)};
        
        
        self.set_equivalence_id(info.equivalence_id());
        self.set_materialized(false);
        self.set_variable_hint(info.variable_hint().clone());
        self.type_hint_ = info.type_hint();
    }

    fn MoveToNewEquivalenceSet(
        &mut self,
        equivalence_id: u32,
        materialized: MaterializedInfo,
        reset: ResetVariableHint,
    ) {
        let self_next = unsafe {&mut *self.next};
        let self_prev = unsafe {&mut *self.prev};
        self_next.prev = self.prev;
        self_prev.next = self.next;

        self.next = Some(self);
        self.prev = Some(self);
        self.equivalence_id_ = equivalence_id;
        self.materialized_ = materialized == MaterializedInfo::kMaterialized;
        self.flush_variable_hint(reset == ResetVariableHint::kReset);
        self.type_hint_ = TypeHint::kAny;
    }

    fn IsOnlyMemberOfEquivalenceSet(&self) -> bool {
        std::ptr::eq(self.next, self as *const Self as *mut Self)
    }

    fn IsInSameEquivalenceSet(&mut self, info: &mut RegisterInfo) -> bool {
        self.equivalence_id() == info.equivalence_id()
    }

    fn GetAllocatedEquivalent(&mut self) -> Option<&mut RegisterInfo> {
        let mut visitor = self;
        loop {
            if visitor.allocated() {
                return Some(visitor);
            }

            let next_reg = visitor.next;
            let next_reg_info = unsafe { &mut *next_reg };
            if std::ptr::eq(next_reg_info, self) {
                break;
            }
            visitor = next_reg_info;
        }
        None
    }

    fn GetMaterializedEquivalent(&mut self) -> Option<*mut RegisterInfo> {
        let mut visitor = self;
        loop {
            if visitor.materialized() {
                return Some(visitor);
            }
            let next_reg = visitor.next;
            let next_reg_info = unsafe { &mut *next_reg };
            if std::ptr::eq(next_reg_info, self) {
                break;
            }

            visitor = next_reg_info;
        }

        None
    }

    fn GetMaterializedEquivalentOtherThan(&mut self, reg: Register) -> Option<*mut RegisterInfo> {
        let mut visitor = self;
        loop {
            if visitor.materialized() && visitor.register_value() != reg {
                return Some(visitor);
            }

            let next_reg = visitor.next;
            let next_reg_info = unsafe { &mut *next_reg };
            if std::ptr::eq(next_reg_info, self) {
                break;
            }

            visitor = next_reg_info;
        }

        None
    }

    fn GetEquivalentToMaterialize(&mut self) -> Option<*mut RegisterInfo> {
        assert!(self.materialized());
        let mut visitor = self.next;
        let mut best_info: Option<*mut RegisterInfo> = None;

        let self_info = self as *mut Self;
        unsafe {
            while !std::ptr::eq(visitor, self_info) {
                let visitor_reg_info = &mut *visitor;
                if visitor_reg_info.materialized() {
                    return None;
                }
                if visitor_reg_info.allocated() {
                    match best_info {
                        None => {
                            best_info = Some(visitor);
                        }
                        Some(best_info_reg) => {
                            let best_reg_info = &mut *best_info_reg;
                            if visitor_reg_info.register_value()
                                < best_reg_info.register_value()
                            {
                                best_info = Some(visitor);
                            }
                        }
                    }
                }
                visitor = (&mut *visitor).next;
            }
        }

        best_info
    }

    fn MarkTemporariesAsUnmaterialized(&mut self, temporary_base: Register) {
        assert!(self.register_value() < temporary_base);
        assert!(self.materialized());
        let mut visitor = self.next;

        let self_info = self as *mut Self;
        unsafe {
            while !std::ptr::eq(visitor, self_info) {
                let visitor_reg_info = &mut *visitor;
                if visitor_reg_info.register_value() >= temporary_base {
                    visitor_reg_info.set_materialized(false);
                }
                visitor = (&mut *visitor).next;
            }
        }
    }

    fn GetEquivalent(&mut self) -> Option<*mut RegisterInfo> {
        Some(self.next)
    }

    fn register_value(&self) -> Register {
        self.register_.clone()
    }
    fn materialized(&self) -> bool {
        self.materialized_
    }
    fn set_materialized(&mut self, materialized: bool) {
        self.materialized_ = materialized;
    }
    fn allocated(&self) -> bool {
        self.allocated_
    }
    fn set_allocated(&mut self, allocated: bool) {
