// src/wasm/wasm_debug.rs

//use std::collections::HashMap;
use std::fmt;
//use std::io::Write;
use std::mem;
//use std::ops::Deref;
//use std::ptr;
//use std::sync::{Arc, Mutex, MutexGuard};
//use std::sync::atomic::{AtomicUsize, Ordering};

//use crate::base::{OwnedVector, Vector};
//use crate::common::assert_scope::AssertScope;
//use crate::common::simd128::Simd128;
//use crate::compiler::wasm_compiler::ExecuteLiftoffCompilation;
//use crate::debug::debug_evaluate::DebugEvaluate;
//use crate::debug::debug::Debug;
//use crate::execution::frames::{Frame, WasmFrame};
//use crate::heap::factory::Factory;
//use crate::isolate::{Isolate, ReadOnlyRoots};
//use crate::objects::{FixedArray, Object, Script, String};
//use crate::wasm::baseline::liftoff_compiler::LiftoffOptions;
//use crate::wasm::baseline::liftoff_register::LiftoffRegister;
//use crate::wasm::compilation_environment::{CompilationEnv, FunctionBody};
//use crate::wasm::module_decoder::{ValidateFunctionBody, WasmDetectedFeatures, DecodeResult};
//use crate::wasm::std_object_sizes::WasmDebugBreakFrameConstants;
//use crate::wasm::value_type::ValueType;
//use crate::wasm::wasm_code_manager::GetWasmCodeManager;
//use crate::wasm::wasm_engine::GetWasmEngine;
//use crate::wasm::wasm_limits::kMaxInt;
//use crate::wasm::wasm_module::WasmModule;
//use crate::wasm::wasm_objects::{WasmInstanceObject};
//use crate::wasm::wasm_opcodes::{kExprReturn, kExprEnd};
//use crate::wasm::wasm_subtyping::CanonicalValueType;
//use crate::wasm::wasm_value::WasmValue;
//use crate::zone::accounting_allocator::Zone;
//use crate::strings::String;

// TODO: Implement the following missing functionalities:
// 1. Mutex and MutexGuard
// 2. base::Vector, base::OwnedVector, base::ArrayVector
// 3. Isolate, Factory, Object, Script, FixedArray, String, etc. from the V8 context
// 4. NativeModule, WasmModule, WasmFunction, WasmCode, etc. from the V8 wasm context
// 5. WasmValue, ValueType, Simd128
// 6. DebugEvaluate, Debug
// 7. WasmFrame, Frame, StackFrameId
// 8. LiftoffRegister, LiftoffOptions
// 9. CompilationEnv, FunctionBody
// 10. ValidateFunctionBody, WasmDetectedFeatures, DecodeResult
// 11. CanonicalValueType
// 12. WasmCodeRefScope

/// Represents the kind of import or export.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ImportExportKindCode {
    Function,
    Table,
    Memory,
    Global,
}

/// A pair representing the kind of import/export and its index.
type ImportExportKey = (ImportExportKindCode, u32);

/// Represents the location to return to after a breakpoint or Wasm call.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ReturnLocation {
    AfterBreakpoint,
    AfterWasmCall,
}

// Address type to represent memory addresses (replace with your actual type)
type Address = usize;
// Register type to represent CPU registers (replace with your actual type)
type Register = u32;
// StackFrameId type to represent a stack frame ID (replace with your actual type)
type StackFrameId = u32;

const NO_ID: StackFrameId = 0;
const kSystemPointerSize: usize = mem::size_of::<usize>();
const kOSRTargetOffset: isize = -8; // Example value
const kOnEntryBreakpointPosition: i32 = -1;

/// Finds the new program counter (PC) after a breakpoint or Wasm call.
///
/// # Arguments
///
/// * `frame` - A mutable reference to the Wasm frame.
/// * `wasm_code` - A pointer to the Wasm code object.
/// * `byte_offset` - The byte offset within the Wasm code.
/// * `return_location` - The location to return to (after breakpoint or Wasm call).
///
/// # Returns
///
/// The new program counter address.
#[allow(dead_code)]
fn find_new_pc(
    _frame: &mut WasmFrame,
    _wasm_code: *mut WasmCode,
    _byte_offset: i32,
    _return_location: ReturnLocation,
) -> Address {
    // This function needs access to the V8-specific WasmCode and related
    // structures, so it cannot be fully implemented without those definitions.
    0 // Placeholder return value
}

/// Represents an entry in the debug side table.
#[derive(Debug, Clone)]
struct DebugSideTableEntryValue {
    type_: ValueType,
    storage: ValueStorage,
    i32_const: i32,
    reg_code: Register,
    stack_offset: i32,
    module: *const WasmModule // Add wasmModule.
}

impl DebugSideTableEntryValue {
    fn is_constant(&self) -> bool {
        self.storage == ValueStorage::Constant
    }

    fn is_register(&self) -> bool {
        self.storage == ValueStorage::Register
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValueStorage {
    Constant,
    Register,
    Stack,
}

/// Represents an entry in the debug side table.
#[derive(Debug, Clone)]
struct DebugSideTableEntry {
    pc_offset_: u32,
    stack_height_: u32,
    changed_values_: Vec<DebugSideTableEntryValue>,
}

impl DebugSideTableEntry {
    /// Prints the debug side table entry to the given output stream.
    ///
    /// # Arguments
    ///
    /// * `os` - A mutable reference to the output stream.
    #[allow(dead_code)]
    fn print(&self, os: &mut dyn fmt::Write) -> fmt::Result {
        write!(os, "{:06x} stack height {} [", self.pc_offset_, self.stack_height_)?;
        for value in &self.changed_values_ {
            write!(os, " {}:", value.type_.name())?;
            match value.storage {
                ValueStorage::Constant => write!(os, "const#{}", value.i32_const)?,
                ValueStorage::Register => write!(os, "reg#{}", value.reg_code)?,
                ValueStorage::Stack => write!(os, "stack#{}", value.stack_offset)?,
            }
        }
        write!(os, " ]\n")
    }

    /// Estimates the current memory consumption of this entry.
    ///
    /// # Returns
    ///
    /// The estimated memory consumption in bytes.
    #[allow(dead_code)]
    fn estimate_current_memory_consumption(&self) -> usize {
        let mut size = mem::size_of::<DebugSideTableEntry>();
        size += self.changed_values_.len() * mem::size_of::<DebugSideTableEntryValue>();
        size
    }
    
}

/// Represents a debug side table for a Wasm function.
#[derive(Debug, Clone)]
struct DebugSideTable {
    num_locals_: u32,
    entries_: Vec<DebugSideTableEntry>,
}

impl DebugSideTable {
    /// Prints the debug side table to the given output stream.
    ///
    /// # Arguments
    ///
    /// * `os` - A mutable reference to the output stream.
    #[allow(dead_code)]
    fn print(&self, os: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(
            os,
            "Debug side table ({} locals, {} entries):",
            self.num_locals_,
            self.entries_.len()
        )?;
        for entry in &self.entries_ {
            entry.print(os)?;
        }
        writeln!(os)
    }

    /// Finds the value in the debug side table entry by index.
    fn find_value(&self, entry: &DebugSideTableEntry, index: i32) -> &DebugSideTableEntryValue {
        // Search the changed values for the given index.
        for value in &entry.changed_values_ {
            // TODO: Actually search by index and return the corresponding value.
            // This is a placeholder.
            return value;
        }
        panic!("Value not found in debug side table for index: {}", index);
    }
    /// Estimates the current memory consumption of this debug side table.
    ///
    /// # Returns
    ///
    /// The estimated memory consumption in bytes.
    #[allow(dead_code)]
    fn estimate_current_memory_consumption(&self) -> usize {
        let mut size = mem::size_of::<DebugSideTable>();
        size += self.entries_.len() * mem::size_of::<DebugSideTableEntry>();
        for entry in &self.entries_ {
            size += entry.estimate_current_memory_consumption();
        }
        size
    }

    /// Returns the number of locals in this debug side table.
    fn num_locals(&self) -> u32 {
        self.num_locals_
    }

    /// Returns the debug side table entry for the given program counter offset.
    fn get_entry(&self, _pc_offset: i32) -> &DebugSideTableEntry {
        // TODO: Implement the logic to find the correct entry by PC offset.
        // This is a placeholder to avoid compilation errors.
        &self.entries_[0]
    }
}

/// Represents a Wasm code object.
struct WasmCode {
    _instruction_start: Address,
    _source_positions: Vec<u8>,
    _is_inspectable: bool,
    _index: i32,
    _native_module: *mut NativeModule,
    _for_debugging: ForDebugging
}

impl WasmCode {
    fn instruction_start(&self) -> Address {
        self._instruction_start
    }

    fn source_positions(&self) -> Vec<u8> {
        self._source_positions.clone()
    }

    fn is_inspectable(&self) -> bool {
        self._is_inspectable
    }

    fn index(&self) -> i32 {
        self._index
    }

    fn native_module(&self) -> *mut NativeModule {
        self._native_module
    }

    fn for_debugging(&self) -> ForDebugging {
        self._for_debugging
    }
    
    // Placeholder methods. Implement the actual functionality as needed.
    #[allow(dead_code)]
    fn maybe_print(&self) {}
    #[allow(dead_code)]
    fn inc_ref(&mut self) {}
    #[allow(dead_code)]
    fn dec_ref_on_live_code(&mut self) {}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ForDebugging {
    kNone,
    kForStepping,
    kWithBreakpoints,
}

/// Represents the native module for a Wasm instance.
struct NativeModule {
    _wire_bytes: Vec<u8>,
    _module: *mut WasmModule, // Assuming WasmModule is defined elsewhere
}

impl NativeModule {
    fn wire_bytes(&self) -> Vec<u8> {
        self._wire_bytes.clone()
    }

    fn module(&self) -> &WasmModule {
        unsafe { &*self._module }
    }
    // Placeholder methods. Implement the actual functionality as needed.
    #[allow(dead_code)]
    fn get_code(&self, _func_index: i32) -> *mut WasmCode {
        std::ptr::null_mut()
    }
    #[allow(dead_code)]
    fn get_debug_info(&self) -> &DebugInfo {
        todo!()
    }
    #[allow(dead_code)]
    fn reinstall_debug_code(&self, _code: *mut WasmCode) {}
    #[allow(dead_code)]
    fn add_compiled_code(&self, _result: WasmCompilationResult) -> *mut WasmCode {
        std::ptr::null_mut()
    }
    #[allow(dead_code)]
    fn publish_code(&self, _code: *mut WasmCode) -> *mut WasmCode {
        std::ptr::null_mut()
    }
}

/// Represents the WASM compilation result
struct WasmCompilationResult {
    _assumptions: i32 // placeholder
}

impl WasmCompilationResult {
    fn succeeded(&self) -> bool {
        true // placeholder
    }
}

/// A struct to hold the implementation details of `DebugInfo`.
struct DebugInfoImpl {
    native_module_: *mut NativeModule,
    //mutex_: Mutex<()>,
    debug_side_tables_: HashMap<*const WasmCode, Box<DebugSideTable>>,
    cached_debugging_code_: Vec<CachedDebuggingCode>,
    per_isolate_data_: HashMap<*mut Isolate, PerIsolateDebugData>,
}

impl DebugInfoImpl {
    const K_MAX_CACHED_DEBUGGING_CODE: usize = 3;

    /// Creates a new `DebugInfoImpl` instance.
    ///
    /// # Arguments
    ///
    /// * `native_module` - A raw pointer to the `NativeModule`.
    fn new(native_module: *mut NativeModule) -> Self {
        DebugInfoImpl {
            native_module_: native_module,
            //mutex_: Mutex::new(()),
            debug_side_tables_: HashMap::new(),
            cached_debugging_code_: Vec::new(),
            per_isolate_data_: HashMap::new(),
        }
    }

    /// Gets the number of locals for the function at the given program counter (PC).
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// The number of locals in the function.
    fn get_num_locals(&self, _pc: Address, _isolate: *mut Isolate) -> i32 {
        // This function requires the FrameInspectionScope which in turn needs access to V8 specific structures like WasmCode.
        // Placeholder return value
        0
    }

    /// Gets the value of a local variable.
    ///
    /// # Arguments
    ///
    /// * `local` - The index of the local variable.
    /// * `pc` - The program counter address.
    /// * `fp` - The frame pointer address.
    /// * `debug_break_fp` - The debug break frame pointer address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// The value of the local variable.
    fn get_local_value(
        &self,
        _local: i32,
        _pc: Address,
        _fp: Address,
        _debug_break_fp: Address,
        _isolate: *mut Isolate,
    ) -> WasmValue {
        // This function requires the FrameInspectionScope which in turn needs access to V8 specific structures like WasmCode.
        // Placeholder return value
        WasmValue::I32(0)
    }

    /// Gets the stack depth at the given program counter (PC).
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// The stack depth at the given PC.
    fn get_stack_depth(&self, _pc: Address, _isolate: *mut Isolate) -> i32 {
        // This function requires the FrameInspectionScope which in turn needs access to V8 specific structures like WasmCode.
        // Placeholder return value
        0
    }

    /// Gets the value at the given index on the stack.
    ///
    /// # Arguments
    ///
    /// * `index` - The index on the stack.
    /// * `pc` - The program counter address.
    /// * `fp` - The frame pointer address.
    /// * `debug_break_fp` - The debug break frame pointer address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// The value at the given stack index.
    fn get_stack_value(
        &self,
        _index: i32,
        _pc: Address,
        _fp: Address,
        _debug_break_fp: Address,
        _isolate: *mut Isolate,
    ) -> WasmValue {
        // This function requires the FrameInspectionScope which in turn needs access to V8 specific structures like WasmCode.
        // Placeholder return value
        WasmValue::I32(0)
    }

    /// Gets the function at the given program counter (PC).
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// A reference to the `WasmFunction` at the given PC.
    fn get_function_at_address(&self, _pc: Address, _isolate: *mut Isolate) -> &WasmFunction {
        // This function requires the FrameInspectionScope which in turn needs access to V8 specific structures like WasmCode.
        unsafe { &*((*self.native_module_)._module).functions.as_ptr() } // Placeholder return value
    }

    /// Finds the "dead breakpoint" for the given Wasm frame and breakpoints.
    ///
    /// # Arguments
    ///
    /// * `frame` - A raw pointer to the `WasmFrame`.
    /// * `breakpoints` - A slice of breakpoint offsets.
    ///
    /// # Returns
    ///
    /// The dead breakpoint offset, or 0 if not found.
    fn dead_breakpoint(&self, _frame: &mut WasmFrame, _breakpoints: &[i32]) -> i32 {
        // TODO: Implement the logic for finding a dead breakpoint.
        // This is a placeholder.
        0
    }

    /// Finds the dead breakpoint (see above) for the top wasm frame, if that frame
    /// is in the function of the given index.
    fn dead_breakpoint_by_func_index(
        &self,
        _func_index: i32,
        _breakpoints: &[i32],
        _isolate: *mut Isolate,
    ) -> i32 {
        // TODO: Implement the logic for finding a dead breakpoint.
        // This is a placeholder.
        0
    }

    /// Recompiles Liftoff code with the given breakpoints.
    fn recompile_liftoff_with_breakpoints(
        &mut self,
        _func_index: i32,
        _offsets: &[i32],
        _dead_breakpoint: i32,
    ) -> *mut WasmCode {
        // TODO: Implement the logic for recompiling with breakpoints.
        std::ptr::null_mut()
    }

    /// Sets a breakpoint at the given function index and offset.
    fn set_breakpoint(&mut self, _func_index: i32, _offset: i32, _isolate: *mut Isolate) {
        // TODO: Implement the logic for setting a breakpoint.
    }

    /// Finds all breakpoints for the given function index.
    fn find_all_breakpoints(&self, _func_index: i32) -> Vec<i32> {
        // TODO: Implement the logic for finding all breakpoints.
        Vec::new()
    }

    /// Updates breakpoints for the given function index, breakpoints, isolate,
    /// stepping frame, and dead breakpoint.
    fn update_breakpoints(
        &mut self,
        _func_index: i32,
        _breakpoints: &[i32],
        _isolate: *mut Isolate,
        _stepping_frame: StackFrameId,
        _dead_breakpoint: i32,
    ) {
        // TODO: Implement the logic for updating breakpoints.
    }

    /// Floods the code with breakpoints for the given Wasm frame and return location.
    fn flood_with_breakpoints(&mut self, _frame: &mut WasmFrame, _return_location: ReturnLocation) {
        // TODO: Implement the logic for flooding with breakpoints.
    }

    /// Checks if the given Wasm frame is blackboxed.
    fn is_frame_blackboxed(&self, _frame: &mut WasmFrame) -> bool {
        // TODO: Implement the logic for checking if a frame is blackboxed.
        false
    }

    /// Prepares for stepping in the given Wasm frame.
    fn prepare_step(&mut self, _frame: &mut WasmFrame) -> bool {
        // TODO: Implement the logic for preparing a step.
        false
    }

    /// Prepares to step out to the given Wasm frame.
    fn prepare_step_out_to(&mut self, _frame: &mut WasmFrame) {
        // TODO: Implement the logic for preparing to step out.
    }

    /// Clears stepping for the given Wasm frame.
    fn clear_stepping(&mut self, _frame: &mut WasmFrame) {
        // TODO: Implement the logic for clearing stepping.
    }

    /// Clears stepping for the given isolate.
    fn clear_stepping_isolate(&mut self, _isolate: *mut Isolate) {
        // TODO: Implement the logic for clearing stepping.
    }

    /// Checks if the given Wasm frame is stepping.
    fn is_stepping(&self, _frame: &mut WasmFrame) -> bool {
        // TODO: Implement the logic for checking if stepping.
        false
    }

    /// Removes a breakpoint at the given function index and position.
    fn remove_breakpoint(&mut self, _func_index: i32, _position: i32, _isolate: *mut Isolate) {
        // TODO: Implement the logic for removing a breakpoint.
    }

    /// Removes debug side tables for the given Wasm code objects.
    fn remove_debug_side_tables(&mut self, _codes: &[*const WasmCode]) {
        // TODO: Implement the logic for removing debug side tables.
    }

    /// Gets the debug side table if it exists for the given Wasm code.
    fn get_debug_side_table_if_exists(&self, _code: *const WasmCode) -> Option<&DebugSideTable> {
        //TODO: Implement the logic for getting the debug side table
        None
    }

    /// Checks if any breakpoints have been removed between two sets of offsets.
    fn has_removed_breakpoints(_removed: &[i32], _remaining: &[i32]) -> bool {
        // TODO: Implement the logic for checking if breakpoints have been removed.
        false
    }

    /// Removes all isolate-specific data.
    fn remove_isolate(&mut self, _isolate: *mut Isolate) {
        // TODO: Implement the logic for removing isolate data.
    }

    /// Estimates the current memory consumption.
    fn estimate_current_memory_consumption(&self) -> usize {
        // TODO: Implement the logic for estimating memory consumption.
        0
    }

    fn get_debug_side_table(&mut self, code: *mut WasmCode) -> &DebugSideTable {
        // Ensure the code is inspectable.
        //assert!(code.is_inspectable());

        // Check if the debug side table already exists.
        if let Some(table) = self.get_debug_side_table_if_exists(code) {
            return table;
        }

        // Generate the debug side table and insert it.
        let debug_side_table = generate_liftoff_debug_side_table(code);
        self.debug_side_tables_.insert(code as *const WasmCode, debug_side_table);

        self.get_debug_side_table_if_exists(code).unwrap()
    }

    fn get_value(
        &self,
        _debug_side_table: &DebugSideTable,
        _debug_side_table_entry: &DebugSideTableEntry,
        _index: i32,
        _stack_frame_base: Address,
        _debug_break_fp: Address,
        _isolate: *mut Isolate,
    ) -> WasmValue {
        // TODO: Implement the logic for getting the value.
        WasmValue::I32(0) // Placeholder
    }

    /// Updates the return addresses on the stack after installing a new code object.
    fn update_return_addresses(
        &mut self,
        _isolate: *mut Isolate,
        _new_code: *mut WasmCode,
        _stepping_frame: StackFrameId,
    ) {
        // TODO: Implement the logic for updating return addresses.
    }

    /// Updates a single return address for a given frame and code object.
    fn update_return_address(
        &mut self,
        _frame: &mut WasmFrame,
        _new_code: *mut WasmCode,
        _return_location: ReturnLocation,
    ) {
        // TODO: Implement the logic for updating a single return address.
    }

    /// Checks if the Wasm frame is at a return instruction.
    fn is_at_return(&self, _frame: &mut WasmFrame) -> bool {
        // TODO: Implement the logic for checking if at a return instruction.
        false
    }
}

use std::collections::HashMap;

/// A struct representing the cached debugging code.
#[derive(Debug, Clone)]
struct CachedDebuggingCode {
    func_index: i32,
    breakpoint_offsets: Vec<i32>,
    dead_breakpoint: i32,
    code: *mut WasmCode,
}

/// A struct representing the isolate-specific debug data.
#[derive(Debug, Clone)]
struct PerIsolateDebugData {
    breakpoints_per_function: HashMap<i32, Vec<i32>>,
    stepping_frame: StackFrameId,
}

impl PerIsolateDebugData {
    /// Creates a new `PerIsolateDebugData` instance.
    fn new() -> Self {
        PerIsolateDebugData {
            breakpoints_per_function: HashMap::new(),
            stepping_frame: NO_ID,
        }
    }
}

/// A struct to manage debug information for Wasm.
pub struct DebugInfo {
    impl_: Box<DebugInfoImpl>,
}

impl DebugInfo {
    /// Creates a new `DebugInfo` instance.
    ///
    /// # Arguments
    ///
    /// * `native_module` - A raw pointer to the `NativeModule`.
    pub fn new(native_module: *mut NativeModule) -> Self {
        DebugInfo {
            impl_: Box::new(DebugInfoImpl::new(native_module)),
        }
    }

    /// Gets the number of locals for the function at the given program counter (PC).
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// The number of locals in the function.
    pub fn get_num_locals(&self, pc: Address, isolate: *mut Isolate) -> i32 {
        self.impl_.get_num_locals(pc, isolate)
    }

    /// Gets the value of a local variable.
    ///
    /// # Arguments
    ///
    /// * `local` - The index of the local variable.
    /// * `pc` - The program counter address.
    /// * `fp` - The frame pointer address.
    /// * `debug_break_fp` - The debug break frame pointer address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// The value of the local variable.
    pub fn get_local_value(
        &self,
        local: i32,
        pc: Address,
        fp: Address,
        debug_break_fp: Address,
        isolate: *mut Isolate,
    ) -> WasmValue {
        self.impl_.get_local_value(local, pc, fp, debug_break_fp, isolate)
    }

    /// Gets the stack depth at the given program counter (PC).
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// The stack depth at the given PC.
    pub fn get_stack_depth(&self, pc: Address, isolate: *mut Isolate) -> i32 {
        self.impl_.get_stack_depth(pc, isolate)
    }

    /// Gets the value at the given index on the stack.
    ///
    /// # Arguments
    ///
    /// * `index` - The index on the stack.
    /// * `pc` - The program counter address.
    /// * `fp` - The frame pointer address.
    /// * `debug_break_fp` - The debug break frame pointer address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// The value at the given stack index.
    pub fn get_stack_value(
        &self,
        index: i32,
        pc: Address,
        fp: Address,
        debug_break_fp: Address,
        isolate: *mut Isolate,
    ) -> WasmValue {
        self.impl_.get_stack_value(index, pc, fp, debug_break_fp, isolate)
    }

    /// Gets the function at the given program counter (PC).
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter address.
    /// * `isolate` - A raw pointer to the `Isolate`.
    ///
    /// # Returns
    ///
    /// A reference to the `WasmFunction` at the given PC.
    pub fn get_function_at_address(&self, pc: Address, isolate: *mut Isolate) -> &WasmFunction {
        self.impl_.get_function_at_address(pc, isolate)
    }

    /// Sets a breakpoint at the given function index and offset.
    pub fn set_breakpoint(&self, func_index: i32, offset: i32, isolate: *mut Isolate) {
        self.impl_.set_breakpoint(func_index, offset, isolate);
    }

    /// Checks if the given Wasm frame is blackboxed.
    pub fn is_frame_blackboxed(&self, frame: &mut WasmFrame) -> bool {
        self.impl_.is_frame_blackboxed(frame)
    }

    /// Prepares for stepping in the given Wasm frame.
    pub fn prepare_step(&self, frame: &mut WasmFrame) -> bool {
        self.impl_.prepare_step(frame)
    }

    /// Prepares to step out to the given Wasm frame.
    pub fn prepare_step_out_to(&self, frame: &mut WasmFrame) {
        self.impl_.prepare_step_out_to(frame);
    }

    /// Clears stepping for the given isolate.
    pub fn clear_stepping(&self, isolate: *mut Isolate) {
        self.impl_.clear_stepping_isolate(isolate);
    }

    /// Clears stepping for the given Wasm frame.
    pub fn clear_stepping_frame(&self, frame: &mut WasmFrame) {
        self.impl_.clear_stepping(frame);
    }

    /// Checks if the given Wasm frame is stepping.
    pub fn is_stepping(&self, frame: &mut WasmFrame) -> bool {
        self.impl_.is_stepping(frame)
    }

    /// Removes a breakpoint at the given function index and offset.
    pub fn remove_breakpoint(&self, func_index: i32, offset: i32, isolate: *mut Isolate) {
        self.impl_.remove_breakpoint(func_index, offset, isolate);
    }

    /// Removes debug side tables for the given Wasm code objects.
    pub fn remove_debug_side_tables(&self, code: &[*const WasmCode]) {
        self.impl_.remove_debug_side_tables(code);
    }

    /// Gets the debug side table if it exists for the given Wasm code.
    pub fn get_debug_side_table_if_exists(&self, code: *const WasmCode) -> Option<&DebugSideTable> {
        self.impl_.get_debug_side_table_if_exists(code)
    }

    /// Removes all isolate-specific data.
    pub fn remove_isolate(&self, isolate: *mut Isolate) {
        self.impl_.remove_isolate(isolate);
    }

    /// Estimates the current memory consumption.
    pub fn estimate_current_memory_consumption(&self) -> usize {
        self.impl_.estimate_current_memory_consumption()
    }
}

//TODO: Implement this function
fn generate_liftoff_debug_side_table(_code: *mut WasmCode) -> Box<DebugSideTable> {
    Box::new(DebugSideTable{num_locals_:0, entries_:Vec::new()})
}

/// Finds the containing Wasm function for a given position in the module.
fn get_containing_wasm_function(module: &WasmModule, position: i32) -> i32 {
    for (i, func) in module.functions.iter().enumerate() {
        if position >= func.code.offset() as i32 && position < func.code.end_offset() as i32 {
            return i as i32;
        }
    }
    -1
}

/// Finds the nearest Wasm function for a given position in the module.
fn get_nearest_wasm_function(module: &WasmModule, position: u32) -> i32 {
    let mut nearest_func_index: i32 = -1;
    let mut min_distance: u32 = u32::MAX;

