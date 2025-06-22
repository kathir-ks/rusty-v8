// This header should only be included if WebAssembly is enabled.
// #![cfg(feature = "webassembly")]

// use std::convert::TryInto;
// use std::mem;
// use std::ptr;
// use std::rc::Rc;
// use std::sync::Arc;

// use crate::base::macros::UNREACHABLE;
// use crate::handles::{Handle, HandleScope};
// use crate::wasm::wasm_module::WasmModule;

mod wasm_interpreter_runtime;
mod wasm_interpreter;

use wasm_interpreter_runtime::*;
use wasm_interpreter::*;

pub mod v8 {
    pub mod internal {
        pub mod wasm {
            use std::ops::Deref;
            use std::sync::{Arc, Mutex};

            use crate::wasm_interpreter::BodyLocalDecls;
            use crate::wasm_interpreter::WasmInterpreter;
            use crate::wasm_interpreter::WasmInterpreterThread;
            use crate::wasm_interpreter_runtime::WasmInterpreterRuntime;
            // use crate::handles::Isolate;
            // use crate::wasm::wasm_module::WasmFunction;

            pub type Address = usize;
            pub type ValueType = i32;
            pub type ValueKind = i32; // Placeholder
            pub type WasmValue = i32; // Placeholder
            pub type Simd128 = i128; // Placeholder
            pub type WasmRef = i64; // Placeholder
            pub const kSlotSize: usize = 8;

            pub fn is_reference(kind: ValueKind) -> bool {
                kind == kRef || kind == kRefNull
            }

            pub const kI32: ValueKind = 0;
            pub const kI64: ValueKind = 1;
            pub const kF32: ValueKind = 2;
            pub const kF64: ValueKind = 3;
            pub const kS128: ValueKind = 4;
            pub const kRef: ValueKind = 5;
            pub const kRefNull: ValueKind = 6;
            pub const kWasmBottom: ValueType = 7;
            pub const kWasmVoid: ValueType = 8;

            pub struct InterpreterCode {
                function: *const WasmFunction, // Raw pointer to WasmFunction
                locals: BodyLocalDecls,
                start: *mut u8,  // Raw pointer to start of code
                end: *mut u8,  // Raw pointer to end of code
                bytecode: Option<Box<WasmBytecode>>,
            }

            impl InterpreterCode {
                pub fn new(
                    function: *const WasmFunction,
                    locals: BodyLocalDecls,
                    start: *mut u8,
                    end: *mut u8,
                ) -> Self {
                    InterpreterCode {
                        function,
                        locals,
                        start,
                        end,
                        bytecode: None,
                    }
                }
            }

            impl WasmInterpreter {
                impl CodeMap {
                    pub fn get_code(&mut self, function_index: u32) -> &mut InterpreterCode {
                        assert!(function_index < self.interpreter_code_.len() as u32);
                        let code = &mut self.interpreter_code_[function_index as usize];
                        if code.bytecode.is_none() && !code.start.is_null() {
                            self.preprocess(function_index);
                        }
                        code
                    }

                    pub fn get_function_bytecode(&mut self, func_index: u32) -> Option<&WasmBytecode> {
                        assert!(func_index < self.interpreter_code_.len() as u32);

                        let code = self.get_code(func_index);
                        code.bytecode.as_ref().map(|x| x.deref())
                    }

                    pub fn add_function(
                        &mut self,
                        function: *const WasmFunction, // Raw pointer to WasmFunction
                        code_start: *const u8,
                        code_end: *const u8,
                    ) {
                        assert_eq!(self.interpreter_code_.len() as u32, unsafe { (*function).func_index });
                        self.interpreter_code_.push(InterpreterCode::new(
                            function,
                            BodyLocalDecls::default(),
                            code_start as *mut u8,
                            code_end as *mut u8,
                        ));
                    }
                }
            }

            impl WasmInterpreterThread {
                impl Activation {
                    pub fn get_isolate(&self) -> *mut std::ffi::c_void {
                        self.wasm_runtime_.get_isolate()
                    }
                }

                pub fn start_activation(
                    &mut self,
                    wasm_runtime: &mut WasmInterpreterRuntime,
                    frame_pointer: Address,
                    interpreter_fp: *mut u8,
                    frame_state: FrameState,
                ) -> *mut Activation {
                    self.run();
                    self.activations_.push(Box::new(Activation::new(
                        self,
                        wasm_runtime,
                        frame_pointer,
                        interpreter_fp,
                        frame_state,
                    )));
                    self.activations_.last_mut().map(|x| x.as_mut()).unwrap()
                }

                pub fn finish_activation(&mut self) {
                    assert!(!self.activations_.is_empty());
                    self.activations_.pop();
                    if self.activations_.is_empty() {
                        if self.state_ != State::TRAPPED && self.state_ != State::STOPPED {
                            self.finish();
                        }
                    }
                }

                pub fn get_current_activation_for(
                    &self,
                    wasm_runtime: &WasmInterpreterRuntime,
                ) -> Option<&FrameState> {
                    for activation in self.activations_.iter().rev() {
                        if activation.wasm_runtime_.as_ref() as *const _ == wasm_runtime as *const _ {
                            return Some(&activation.current_frame_);
                        }
                    }
                    None
                }
            }

            impl WasmInterpreter {
                pub fn begin_execution(
                    &mut self,
                    thread: &mut WasmInterpreterThread,
                    function_index: u32,
                    frame_pointer: Address,
                    interpreter_fp: *mut u8,
                    ref_stack_offset: u32,
                    args: &Vec<WasmValue>,
                ) {
                    self.codemap_.get_code(function_index);
                    self.wasm_runtime_.begin_execution(
                        thread,
                        function_index,
                        frame_pointer,
                        interpreter_fp,
                        ref_stack_offset,
                        args,
                    );
                }

                pub fn begin_execution2(
                    &mut self,
                    thread: &mut WasmInterpreterThread,
                    function_index: u32,
                    frame_pointer: Address,
                    interpreter_fp: *mut u8,
                ) {
                    self.codemap_.get_code(function_index);
                    self.wasm_runtime_.begin_execution(
                        thread,
                        function_index,
                        frame_pointer,
                        interpreter_fp,
                        thread.next_ref_stack_offset(),
                        &vec![],
                    );
                }

                pub fn get_return_value(&self, index: i32) -> WasmValue {
                    self.wasm_runtime_.get_return_value(index)
                }

                pub fn get_interpreted_stack(&self, frame_pointer: Address) -> Vec<WasmInterpreterStackEntry> {
                    self.wasm_runtime_.get_interpreted_stack(frame_pointer)
                }

                pub fn get_function_index(&self, frame_pointer: Address, index: i32) -> i32 {
                    self.wasm_runtime_.get_function_index(frame_pointer, index)
                }

                pub fn set_trap_function_index(&mut self, func_index: i32) {
                    self.wasm_runtime_.set_trap_function_index(func_index);
                }
            }

            impl WasmBytecode {
                pub fn return_type(&self, index: usize) -> ValueType {
                    assert!(index < self.return_count() as usize);
                    self.signature_.returns[index]
                }

                pub fn arg_type(&self, index: usize) -> ValueType {
                    assert!(index < self.args_count() as usize);
                    self.signature_.params[index]
                }

                pub fn local_type(&self, index: usize) -> ValueType {
                    assert!(index < self.locals_count() as usize);
                    assert!(index < self.interpreter_code_.locals.num_locals as usize);
                    self.interpreter_code_.locals.local_types[index]
                }

                pub fn args_size_in_slots(sig: &FunctionSig) -> u32 {
                    let mut args_slots_size = 0;
                    let args_count = sig.params.len();
                    for i in 0..args_count {
                        args_slots_size += get_value_size_in_slots(sig.params[i]).unwrap() as u32;
                    }
                    args_slots_size
                }

                pub fn rets_size_in_slots(sig: &FunctionSig) -> u32 {
                    let mut rets_slots_size = 0;
                    let return_count = sig.returns.len();
                    for i in 0..return_count {
                        rets_slots_size += get_value_size_in_slots(sig.returns[i]).unwrap() as u32;
                    }
                    rets_slots_size
                }

                pub fn ref_args_count(sig: &FunctionSig) -> u32 {
                    let mut refs_args_count = 0;
                    let args_count = sig.params.len();
                    for i in 0..args_count {
                        let kind = sig.params[i];
                        if is_reference(kind) {
                            refs_args_count += 1;
                        }
                    }
                    refs_args_count
                }

                pub fn ref_rets_count(sig: &FunctionSig) -> u32 {
                    let mut refs_rets_count = 0;
                    let return_count = sig.returns.len();
                    for i in 0..return_count {
                        let kind = sig.returns[i];
                        if is_reference(kind) {
                            refs_rets_count += 1;
                        }
                    }
                    refs_rets_count
                }

                pub fn contains_simd(sig: &FunctionSig) -> bool {
                    let args_count = sig.params.len();
                    for i in 0..args_count {
                        if sig.params[i] == kS128 {
                            return true;
                        }
                    }

                    let return_count = sig.returns.len();
                    for i in 0..return_count {
                        if sig.returns[i] == kS128 {
                            return true;
                        }
                    }

                    false
                }

                pub fn has_ref_or_simd_args(sig: &FunctionSig) -> bool {
                    let args_count = sig.params.len();
                    for i in 0..args_count {
                        let kind = sig.params[i];
                        if is_reference(kind) || kind == kS128 {
                            return true;
                        }
                    }
                    false
                }

                pub fn js_to_wasm_wrapper_packed_array_size(sig: &FunctionSig) -> u32 {
                    // static_assert(kSystemPointerSize == 8);

                    let mut args_size = 0;
                    let args_count = sig.params.len();
                    for i in 0..args_count {
                        match sig.params[i] {
                            kI32 | kF32 => {
                                args_size += std::mem::size_of::<i32>() as u32;
                            }
                            kI64 | kF64 => {
                                args_size += std::mem::size_of::<i64>() as u32;
                            }
                            kS128 => {
                                args_size += std::mem::size_of::<Simd128>() as u32;
                            }
                            kRef | kRefNull => {
                                // Make sure Ref slots are 64-bit aligned.
                                args_size += args_size & 0x04;
                                args_size += std::mem::size_of::<WasmRef>() as u32;
                            }
                            _ => unreachable!(),
                        }
                    }

                    let mut rets_size = 0;
                    let rets_count = sig.returns.len();
                    for i in 0..rets_count {
                        match sig.returns[i] {
                            kI32 | kF32 => {
                                rets_size += std::mem::size_of::<i32>() as u32;
                            }
                            kI64 | kF64 => {
                                rets_size += std::mem::size_of::<i64>() as u32;
                            }
                            kS128 => {
                                rets_size += std::mem::size_of::<Simd128>() as u32;
                            }
                            kRef | kRefNull => {
                                // Make sure Ref slots are 64-bit aligned.
                                rets_size += rets_size & 0x04;
                                rets_size += std::mem::size_of::<WasmRef>() as u32;
                            }
                            _ => unreachable!(),
                        }
                    }

                    let mut size = std::cmp::max(args_size, rets_size);
                    // Make sure final size is 64-bit aligned.
                    size += size & 0x04;
                    size
                }

                pub fn ref_locals_count(wasm_code: &InterpreterCode) -> u32 {
                    let mut refs_locals_count = 0;
                    let locals_count = wasm_code.locals.num_locals;
                    for i in 0..locals_count {
                        let kind = wasm_code.locals.local_types[i];
                        if is_reference(kind) {
                            refs_locals_count += 1;
                        }
                    }
                    refs_locals_count
                }

                pub fn locals_size_in_slots(wasm_code: &InterpreterCode) -> u32 {
                    let mut locals_slots_size = 0;
                    let locals_count = wasm_code.locals.num_locals;
                    for i in 0..locals_count {
                        locals_slots_size += get_value_size_in_slots(wasm_code.locals.local_types[i]).unwrap() as u32;
                    }
                    locals_slots_size
                }

                pub fn initialize_slots(&self, sp: *mut u8, stack_space: usize) -> bool {
                    // Check for overflow
                    if self.total_frame_size_in_bytes_ > stack_space {
                        return false;
                    }

                    let args_slots_size_in_bytes = self.args_slots_size() * kSlotSize as u32;
                    let rets_slots_size_in_bytes = self.rets_slots_size() * kSlotSize as u32;
                    let const_slots_size_in_bytes = self.const_slots_size_in_bytes();

                    let start_const_area = unsafe {
                        sp.add(args_slots_size_in_bytes as usize + rets_slots_size_in_bytes as usize)
                    };

                    // Initialize const slots
                    if const_slots_size_in_bytes > 0 {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                self.const_slots_values_.as_ptr(),
                                start_const_area,
                                const_slots_size_in_bytes as usize,
                            );
                        }
                    }

                    // Initialize local slots
                    unsafe {
                        std::ptr::write_bytes(
                            start_const_area.add(const_slots_size_in_bytes as usize),
                            0,
                            (self.locals_slots_size() * kSlotSize as u32) as usize,
                        );
                    }

                    true
                }
            }

            pub fn get_value_size_in_slots(kind: ValueKind) -> Option<u32> {
                match kind {
                    kI32 => Some((std::mem::size_of::<i32>() / kSlotSize) as u32),
                    kI64 => Some((std::mem::size_of::<i64>() / kSlotSize) as u32),
                    kF32 => Some((std::mem::size_of::<f32>() / kSlotSize) as u32),
                    kF64 => Some((std::mem::size_of::<f64>() / kSlotSize) as u32),
                    kS128 => Some((std::mem::size_of::<Simd128>() / kSlotSize) as u32),
                    kRef | kRefNull => Some((std::mem::size_of::<WasmRef>() / kSlotSize) as u32),
                    _ => None, //UNREACHABLE(),
                }
            }

            impl FrameState {
                pub fn reset_handle_scope(&mut self, _isolate: *mut std::ffi::c_void) {
                    //DCHECK_NOT_NULL(handle_scope_);
                    //{
                    //  HandleScope old(std::move(*handle_scope_));
                    // The HandleScope destructor cleans up the old HandleScope.
                    //}
                    // Now that the old HandleScope has been destroyed, make a new one.
                    //*handle_scope_ = HandleScope(isolate);
                    println!("FrameState::reset_handle_scope not implemented");
                }
            }

            // Placeholder definitions for types used in WasmBytecodeGenerator
            pub type WasmInstruction = i32; // Placeholder
            pub type RegMode = i32; // Placeholder

            pub const kExprGlobalGet: WasmInstruction = 0;
            pub const kExprSelect: WasmInstruction = 1;
            pub const kExprSelectWithType: WasmInstruction = 2;

            pub const kNoReg: RegMode = 0;
            pub const kI32Reg: RegMode = 1;
            pub const kI64Reg: RegMode = 2;
            pub const kF32Reg: RegMode = 3;
            pub const kF64Reg: RegMode = 4;
            pub const kAnyReg: RegMode = 5;

            pub struct Slot {
                pub kind: ValueKind,
                pub slot_offset: u32,
                pub ref_stack_index: u32
            }

            pub struct WasmModule {
                pub globals: Vec<Global>,
                pub functions: Vec<Function>,
                pub memories: Vec<Memory>,
                pub signatures: Vec<FunctionSig>,
            }

            pub struct Global {
                pub type_: ValueType_,
            }

            pub struct ValueType_ {
                pub kind: ValueKind,
            }

            pub struct Function {
                pub sig: *const FunctionSig,
            }

            pub struct Memory {
                pub is_memory64: bool,
            }

            pub struct FunctionSig {
                pub params: Vec<ValueKind>,
                pub returns: Vec<ValueKind>,
            }

            impl WasmBytecodeGenerator {
                pub fn to_register_is_allowed(&self, instr: &WasmInstructionData) -> bool {
                    if !instr.supports_to_register {
                        return false;
                    }

                    // Even if the instruction is marked as supporting ToRegister, reference
                    // values should not be stored in the register.
                    match instr.opcode {
                        kExprGlobalGet => {
                            let kind = self.get_global_type(instr.optional_index);
                            !is_reference(kind) && kind != kS128
                        }
                        kExprSelect | kExprSelectWithType => {
                            if self.stack_.len() < 2 {
                                return false;
                            }
                            let kind = self.slots_[self.stack_[self.stack_.len() - 2]].kind;
                            !is_reference(kind) && kind != kS128
                        }
                        _ => true,
                    }
                }

                pub fn i32_push(&mut self, emit: bool) {
                    let slot_index = self._push_slot(kI32);
                    let slot_offset = self.slots_[slot_index].slot_offset;
                    if emit {
                        self.emit_slot_offset(slot_offset);
                    }
                }

                pub fn i64_push(&mut self, emit: bool) {
                    let slot_index = self._push_slot(kI64);
                    let slot_offset = self.slots_[slot_index].slot_offset;
                    if emit {
                        self.emit_slot_offset(slot_offset);
                    }
                }

                pub fn f32_push(&mut self, emit: bool) {
                    let slot_index = self._push_slot(kF32);
                    let slot_offset = self.slots_[slot_index].slot_offset;
                    if emit {
                        self.emit_slot_offset(slot_offset);
                    }
                }

                pub fn f64_push(&mut self, emit: bool) {
                    let slot_index = self._push_slot(kF64);
                    let slot_offset = self.slots_[slot_index].slot_offset;
                    if emit {
                        self.emit_slot_offset(slot_offset);
                    }
                }

                pub fn s128_push(&mut self, emit: bool) {
                    let slot_index = self._push_slot(kS128);
                    let slot_offset = self.slots_[slot_index].slot_offset;
                    if emit {
                        self.emit_slot_offset(slot_offset);
                    }
                }

                pub fn ref_push(&mut self, type_: ValueType, emit: bool) {
                    let slot_index = self._push_slot(type_);
                    let slot_offset = self.slots_[slot_index].slot_offset;
                    if emit {
                        self.emit_slot_offset(slot_offset);
                        self.emit_ref_stack_index(self.slots_[slot_index].ref_stack_index);
                    }
                }

                pub fn push(&mut self, type_: ValueType) {
                    match type_ {
                        kI32 => self.i32_push(true),
                        kI64 => self.i64_push(true),
                        kF32 => self.f32_push(true),
                        kF64 => self.f64_push(true),
                        kS128 => self.s128_push(true),
                        kRef | kRefNull => self.ref_push(type_, true),
                        _ => panic!("UNREACHABLE"),
                    }
                }

                pub fn push_copy_slot(&mut self, from_stack_index: usize) {
                    assert!(from_stack_index < self.stack_.len());
                    self.push_slot(self.stack_[from_stack_index]);
                }

                pub fn push_const_slot(&mut self, slot_index: u32) {
                    self.push_slot(slot_index);
                }

                pub fn has_void_signature(&self, block_data: &BlockData) -> bool {
                    if block_data.signature.value_type == kWasmBottom {
                        let sig = unsafe { &*self.module_.signatures[block_data.signature.sig_index as usize].as_ptr() };
                        0 == (sig.params.len() + sig.returns.len())
                    } else if block_data.signature.value_type == kWasmVoid {
                        true
                    } else {
                        false
                    }
                }

                pub fn params_count(&self, block_data: &BlockData) -> u32 {
                    if block_data.signature.value_type == kWasmBottom {
                        let sig = unsafe { &*self.module_.signatures[block_data.signature.sig_index as usize].as_ptr() };
                        sig.params.len() as u32
                    } else {
                        0
                    }
                }

                pub fn get_param_type(&self, block_data: &BlockData, index: usize) -> ValueType {
                    assert_eq!(block_data.signature.value_type, kWasmBottom);
                    let sig = unsafe { &*self.module_.signatures[block_data.signature.sig_index as usize].as_ptr() };
                    sig.params[index]
                }

                pub fn returns_count(&self, block_data: &BlockData) -> u32 {
                    if block_data.signature.value_type == kWasmBottom {
                        let sig = unsafe { &*self.module_.signatures[block_data.signature.sig_index as usize].as_ptr() };
                        sig.returns.len() as u32
                    } else if block_data.signature.value_type == kWasmVoid {
                        0
                    } else {
                        1
                    }
                }

                pub fn get_return_type(&self, block_data: &BlockData, index: usize) -> ValueType {
                    assert_ne!(block_data.signature.value_type, kWasmVoid);
                    if block_data.signature.value_type == kWasmBottom {
                        let sig = unsafe { &*self.module_.signatures[block_data.signature.sig_index as usize].as_ptr() };
                        return sig.returns[index];
                    }
                    assert_eq!(index, 0);
                    block_data.signature.value_type
                }

                pub fn get_global_type(&self, index: u32) -> ValueKind {
                     self.module_.globals[index as usize].type_.kind
                }

                pub fn is_memory64(&self) -> bool {
                    !self.module_.memories.is_empty() && self.module_.memories[0].is_memory64
                }

                pub fn is_multi_memory(&self) -> bool {
                    self.module_.memories.len() > 1
                }

                pub fn emit_global_index(&mut self, index: u32) {
                    self.emit(&index.to_ne_bytes());
                }

                pub fn get_current_branch_depth(&self) -> u32 {
                    assert!(self.current_block_index_ >= 0);
                    let mut index = self.blocks_[self.current_block_index_ as usize].parent_block_index_;
                    let mut depth = 0;
                    while index >= 0 {
                        depth += 1;
                        index = self.blocks_[index as usize].parent_block_index_;
                    }
                    depth
                }

                pub fn get_target_branch(&self, delta: u32) -> i32 {
                    let mut index = self.current_block_index_;
                    let mut delta_left = delta;
                    while delta_left > 0 {
                        assert!(index >= 0);
                        index = self.blocks_[index as usize].parent_block_index_;
                        delta_left -= 1;
                    }
                    index
                }

                pub fn emit_branch_offset(&mut self, delta: u32) {
                    let target_branch_index = self.get_target_branch(delta);
                    assert!(target_branch_index >= 0);
                    self.blocks_[target_branch_index as usize].branch_code_offsets_.push(self.current_code_pos());

                    let current_code_offset = self.current_code_pos();
                    self.emit(&current_code_offset.to_ne_bytes());
                }

                pub fn emit_branch_table_offset(&mut self, delta: u32, code_pos: u32) {
                    let target_branch_index = self.get_target_branch(delta);
                    assert!(target_branch_index >= 0);
                    self.blocks_[target_branch_index as usize].branch_code_offsets_.push(code_pos);

                    self.emit(&code_pos.to_ne_bytes());
                }

                pub fn emit_if_else_branch_offset(&mut self) {
                    // Initially emits offset to jump the end of the 'if' block. If we meet an
                    // 'else' instruction later, this offset needs to be updated with the offset
                    // to the beginning of that 'else' block.
                    self.blocks_[self.current_block_index_ as usize].branch_code_offsets_.push(self.current_code_pos());

                    let current_code_offset = self.current_code_pos();
                    self.emit(&current_code_offset.to_ne_bytes());
                }

                pub fn emit_try_catch_branch_offset(&mut self) {
                    // Initially emits offset to jump the end of the 'try/catch' blocks. When we
                    // meet the corresponding 'end' instruction later, this offset needs to be
                    // updated with the offset to the 'end' instruction.
                    self.blocks_[self.current_block_index_ as usize].branch_code_offsets_.push(self.current_code_pos());

                    let current_code_offset = self.current_code_pos();
                    self.emit(&current_code_offset.to_ne_bytes());
                }

                pub fn begin_else_block(&mut self, if_block_index: u32, dummy: bool) {
                    self.end_block(kExprElse); // End matching if block.
                    self.restore_if_else_params(if_block_index);

                    let else_block_index =
                        self.begin_block(kExprElse, self.blocks_[if_block_index as usize].signature.clone());
                    self.blocks_[if_block_index as usize].if_else_block_index_ = else_block_index;
                    self.blocks_[else_block_index as usize].if_else_block_index_ = if_block_index as i32;
                    self.blocks_[else_block_index as usize].first_block_index_ =
                        self.blocks_[if_block_index as usize].first_block_index_;
                }

                pub fn get_function_signature(&self, function_index: u32) -> &FunctionSig {
                    unsafe { &*self.module_.functions[function_index as usize].sig.as_ptr() }
                }

                pub fn get_top_stack_type(&self, reg_mode: RegMode) -> ValueKind {
                    match reg_mode {
                        kNoReg => {
                            if self.stack_.is_empty() {
                                return kI32; // not used
                            }
                            self.slots_[self.stack_[self.stack_.len() - 1]].kind
                        }
                        kI32Reg => kI32,
                        kI64Reg => kI64,
                        kF32Reg => kF32,
                        kF64Reg => kF64,
                        kAnyReg => panic!("UNREACHABLE"),
                        _ => panic!("UNREACHABLE"),
                    }
                }
            }

            // Placeholder structs, enums and impls to allow compilation
            #[derive(Debug, Default, Clone)]
            pub struct BlockSignature {
                pub value_type: ValueType,
                pub sig_index: u32,
            }

            pub struct BlockData {
                pub parent_block_index_: i32,
                pub signature_: BlockSignature,
                pub branch_code_offsets_: Vec<u32>,
                pub if_else_block_index_: i32,
                pub first_block_index_: i32
            }

            impl Default for BlockData {
                fn default() -> Self {
                    BlockData {
                        parent_block_index_: 0,
                        signature_: BlockSignature::default(),
                        branch_code_offsets_: Vec::new(),
                        if_else_block_index_: 0,
                        first_block_index_: 0
                    }
                }
            }

            pub struct WasmBytecodeGenerator {
                pub module_: Box<WasmModule>,
                pub code_: Vec<u8>,
                pub current_block_index_: i32,
                pub blocks_: Vec<BlockData>,
                pub stack_: Vec<u32>,
                pub slots_: Vec<Slot>,
                pub locals_: Vec<ValueKind>,
            }

            pub struct WasmInstructionData {
                pub opcode: WasmInstruction,
                pub optional_index: u32,
                pub supports_to_register: bool,
            }

            impl WasmBytecodeGenerator {
                fn emit(&mut self, data: &[u8]) {
                    self.code_.extend_from_slice(data);
                }

                fn emit_slot_offset(&mut self, offset: u32) {
                    self.emit(&offset.to_ne_bytes());
                }

                fn emit_ref_stack_index(&mut self, offset: u32) {
                    self.emit(&offset.to_ne_bytes());
                }

                fn current_code_pos(&self) -> u32 {
                    self.code_.len() as u32
                }

                fn _push_slot(&mut self, kind: ValueKind) -> u32 {
                    let slot = Slot {
                        kind,
                        slot_offset: 0, // Dummy value
                        ref_stack_index: 0
                    };

                    self.slots_.push(slot);

                    self.slots_.len() as u32 - 1
                }

                fn push_slot(&mut self, slot_index: u32) {
                    self.stack_.push(slot_index);
                }

                fn end_block(&mut self, _expr: WasmInstruction) {
                    // Dummy implementation
                    println!("WasmBytecodeGenerator::end_block");
                }

                fn restore_if_else_params(&mut self, _if