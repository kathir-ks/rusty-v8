// Converted from V8 C++ source files:
// Header: debug-objects.h
// Implementation: debug-objects.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::cell::RefCell;
use std::rc::Rc;

//use crate::v8::internal::Address;
//use crate::v8::internal::Isolate;
//use crate::v8::internal::Object;
//use crate::v8::internal::SharedFunctionInfo;

pub struct BreakPoint {}
pub struct BytecodeArray {}
pub struct StructBodyDescriptor {}
pub struct Script {}
pub struct FixedArray {}
pub struct HeapObject {}
pub struct Module {}
pub struct SharedFunctionInfo {}
pub struct GCType {}
pub struct Isolate {}
pub struct Object {}
pub struct Address {}
pub struct DirectHandle<T> {
    _dummy: i32,
    phantom: std::marker::PhantomData<T>,
}
pub struct Handle<T> {
    _dummy: i32,
    phantom: std::marker::PhantomData<T>,
}
pub struct WasmFrame {}
pub struct WasmInternalFunction {}
pub struct Tagged<T> {
    _dummy: i32,
    phantom: std::marker::PhantomData<T>,
}
pub struct RootIndex {}
pub struct Tagged_t {}
pub struct CpuFeatures {}
pub struct ZoneObject {}
pub struct AstNodeSourceRanges {}
pub struct SourceTextModuleInfo {}
pub struct Code {}
pub struct Range<T> {
    _dummy: i32,
    phantom: std::marker::PhantomData<T>,
}
pub struct AbortReason {}
pub struct turboshaft {
    Graph: i32,
    Block: i32,
}
pub struct OpIndex {}
pub struct Node {}
pub struct BranchHint {}
pub struct Register {}
pub struct Operand {}
pub struct Condition {}
pub struct InstructionOperand {}
pub struct JsonObject {}
pub struct Position {}
pub struct Extend {}
pub struct VariableMode {}
pub struct GCType {}
pub struct RegisterT {}
pub struct MachineType {}
pub struct Bytecode {}
pub struct FPUControlRegister {}
pub struct V8_EXPORT_PRIVATE {}
pub struct flags {}
pub struct std {
    ostream: i32,
}
pub struct DirectHandle_DebugInfo {}
pub struct DirectHandle_BreakPoint {}
pub struct DirectHandle_BreakPointInfo {}
pub struct DirectHandle_SharedFunctionInfo {}
pub struct MaybeDirectHandle<T> {
    _dummy: i32,
    phantom: std::marker::PhantomData<T>,
}

//pub use base::bits::RoundUp;
//pub use base::bits::Align;

#[derive(Debug)]
pub struct DebugInfo {
    flags: u32,
    debugger_hints: i32,
    original_bytecode_array: *mut BytecodeArray,
    debug_bytecode_array: *mut BytecodeArray,
    break_points: *mut FixedArray,
    coverage_info: *mut Object,
    side_effect_state: i32,
    debugging_id: i32,
}

impl DebugInfo {
    const kBreakpoints: u8 = 0;
    const kSideEffects: u8 = 1;

    pub fn is_empty(&self) -> bool {
        self.flags == 0 && self.debugger_hints == 0
    }

    pub fn debug_execution_mode(&self) -> u8 {
        if (self.flags & 0x01) != 0 {
            DebugInfo::kSideEffects
        } else {
            DebugInfo::kBreakpoints
        }
    }

    pub fn set_debug_execution_mode(&mut self, value: u8) {
        if value == DebugInfo::kSideEffects {
            self.flags |= 0x01;
        } else {
            self.flags &= !0x01;
        }
    }

    pub fn has_instrumented_bytecode_array(&self) -> bool {
        self.original_bytecode_array as *const _ != std::ptr::null()
    }

    pub fn original_bytecode_array(&self, _isolate: *mut Isolate) -> Tagged<BytecodeArray> {
        unsafe { Tagged::<BytecodeArray> { _dummy: 0, phantom: std::marker::PhantomData } }
    }

    pub fn debug_bytecode_array(&self, _isolate: *mut Isolate) -> Tagged<BytecodeArray> {
        unsafe { Tagged::<BytecodeArray> { _dummy: 0, phantom: std::marker::PhantomData } }
    }

    pub fn has_break_info(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    pub fn clear_break_info(&mut self, isolate: *mut Isolate) {
        self.set_break_points(unsafe { &mut *(&mut FixedArray { _dummy: 0 }) });
        self.flags &= !(0x02 | 0x04 | 0x08 | 0x10 | 0x01);
        self.set_original_bytecode_array(std::ptr::null_mut());
        self.set_debug_bytecode_array(std::ptr::null_mut());

    }

    pub fn set_break_at_entry(&mut self) {
        if self.can_break_at_entry() {
            self.flags |= 0x04;
        }
    }

    pub fn clear_break_at_entry(&mut self) {
        if self.can_break_at_entry() {
            self.flags &= !0x04;
        }
    }

    pub fn break_at_entry(&self) -> bool {
        (self.flags & 0x04) != 0
    }

    pub fn has_break_point(&self, isolate: *mut Isolate, source_position: i32) -> bool {
        if !self.has_break_info() {
            return false;
        }
        let break_point_info = self.get_break_point_info(isolate, source_position);
        break_point_info as *const _ != std::ptr::null()
    }

    pub fn clear_break_point(
        isolate: *mut Isolate,
        debug_info: DirectHandle<DebugInfo>,
        break_point: DirectHandle<BreakPoint>,
    ) -> bool {
        unsafe {
            if !(*debug_info._dummy as *mut DebugInfo).has_break_info() {
                return false;
            }
            for i in 0..(*(*debug_info._dummy as *mut DebugInfo).break_points)._dummy {
                //if ((*(*debug_info).break_points).get(i) as *const _ != std::ptr::null()) {
                    //let break_point_info = Cast::<BreakPointInfo>((*(*debug_info).break_points).get(i));
                    //if BreakPointInfo::has_break_point(isolate, break_point_info, break_point) {
                    //    BreakPointInfo::clear_break_point(isolate, break_point_info, break_point);
                    //    return true;
                    //}
                //}
            }
            return false;
        }
    }

    pub fn set_break_point(
        isolate: *mut Isolate,
        debug_info: DirectHandle<DebugInfo>,
        source_position: i32,
        break_point: DirectHandle<BreakPoint>,
    ) {
        unsafe {
            if !(*debug_info._dummy as *mut DebugInfo).has_break_info() {
                return;
            }
            let break_point_info =
                (*debug_info._dummy as *mut DebugInfo).get_break_point_info(isolate, source_position);
            if break_point_info as *const _ != std::ptr::null() {
                //BreakPointInfo::set_break_point(isolate, Cast::<BreakPointInfo>(break_point_info), break_point);
                return;
            }
            //let k_no_break_point_info: i32 = -1;
            //let mut index: i32 = k_no_break_point_info;
            //for i in 0..(*debug_info).break_points.length() {
            //    if (*debug_info).break_points.get(i) as *const _ == std::ptr::null() {
            //        index = i;
            //        break;
            //    }
            //}
            //if index == k_no_break_point_info {
            //    let old_break_points = (*debug_info).break_points;
            //    let new_break_points = FixedArray::new(old_break_points.length() + 4);
            //    (*debug_info).break_points = new_break_points;
            //    for i in 0..old_break_points.length() {
            //        new_break_points.set(i, old_break_points.get(i));
            //    }
            //    index = old_break_points.length();
            //}
            //let new_break_point_info = BreakPointInfo::new(source_position);
            //BreakPointInfo::set_break_point(isolate, new_break_point_info, break_point);
            //*(*debug_info).break_points.get(index) = new_break_point_info;
        }
    }

    pub fn get_break_points(&self, isolate: *mut Isolate, source_position: i32) -> DirectHandle<Object> {
        if !self.has_break_info() {
            return DirectHandle::<Object> { _dummy: 0, phantom: std::marker::PhantomData };
        }
        let break_point_info = self.get_break_point_info(isolate, source_position);
        if break_point_info as *const _ == std::ptr::null() {
            return DirectHandle::<Object> { _dummy: 0, phantom: std::marker::PhantomData };
        }
        DirectHandle::<Object> { _dummy: 0, phantom: std::marker::PhantomData }
    }

    pub fn find_break_point_info(
        isolate: *mut Isolate,
        debug_info: DirectHandle<DebugInfo>,
        break_point: DirectHandle<BreakPoint>,
    ) -> DirectHandle<Object> {
        unsafe {
            if !(*debug_info._dummy as *mut DebugInfo).has_break_info() {
                return DirectHandle::<Object> { _dummy: 0, phantom: std::marker::PhantomData };
            }
            for i in 0..(*(*debug_info._dummy as *mut DebugInfo).break_points)._dummy {
                //if ((*(*debug_info).break_points).get(i) as *const _ != std::ptr::null()) {
                //    let break_point_info = Cast::<BreakPointInfo>((*(*debug_info).break_points).get(i));
                //    if BreakPointInfo::has_break_point(isolate, break_point_info, break_point) {
                //        return break_point_info;
                //    }
                //}
            }
        }
        DirectHandle::<Object> { _dummy: 0, phantom: std::marker::PhantomData }
    }

    pub fn get_break_point_count(&self, isolate: *mut Isolate) -> i32 {
        if !self.has_break_info() {
            return 0;
        }
        let mut count: i32 = 0;
        unsafe {
            for i in 0..(*self.break_points)._dummy {
                //if (*self.break_points).get(i) as *const _ != std::ptr::null() {
                //    let break_point_info = Cast::<BreakPointInfo>((*self.break_points).get(i));
                //    count += break_point_info.get_break_point_count(isolate);
                //}
            }
        }
        count
    }

    pub fn can_break_at_entry(&self) -> bool {
        (self.flags & 0x08) != 0
    }

    pub fn has_coverage_info(&self) -> bool {
        (self.flags & 0x10) != 0
    }

    pub fn clear_coverage_info(&mut self, isolate: *mut Isolate) {
        if self.has_coverage_info() {
            self.set_coverage_info(std::ptr::null_mut());
            self.flags &= !0x10;
        }
    }

    pub fn get_side_effect_state(&mut self, isolate: *mut Isolate) -> i32 {
        if self.side_effect_state == 0 {
            //let has_no_side_effect = DebugEvaluate::FunctionGetSideEffectState(isolate, self.shared);
            //self.side_effect_state = has_no_side_effect;
        }
        self.side_effect_state
    }

    fn get_break_point_info(&self, isolate: *mut Isolate, source_position: i32) -> *mut Object {
        if !self.has_break_info() {
            return std::ptr::null_mut();
        }
        unsafe {
            for i in 0..(*self.break_points)._dummy {
                //if (*self.break_points).get(i) as *const _ != std::ptr::null() {
                //    let break_point_info = Cast::<BreakPointInfo>((*self.break_points).get(i));
                //    if break_point_info.source_position() == source_position {
                //        return break_point_info;
                //    }
                //}
            }
        }
        std::ptr::null_mut()
    }

    pub fn set_flags(&mut self, flags: i32, relaxed_store: i32) {
         self.flags = flags as u32;
    }

    pub fn flags(&self, relaxed_load: i32) -> i32 {
        self.flags as i32
    }
    pub fn set_original_bytecode_array(&mut self, original_bytecode_array: *mut BytecodeArray) {
         self.original_bytecode_array = original_bytecode_array;
    }
    pub fn set_debug_bytecode_array(&mut self, debug_bytecode_array: *mut BytecodeArray) {
         self.debug_bytecode_array = debug_bytecode_array;
    }
    pub fn set_break_points(&mut self, break_points: *mut FixedArray) {
         self.break_points = break_points;
    }
    pub fn break_points(&self) -> *mut FixedArray {
         self.break_points
    }
    pub fn set_coverage_info(&mut self, coverage_info: *mut Object) {
         self.coverage_info = coverage_info;
    }

}

#[derive(Debug)]
pub struct BreakPointInfo {
    break_points: *mut Object,
    source_position: i32,
}

impl BreakPointInfo {
    pub fn clear_break_point(
        isolate: *mut Isolate,
        break_point_info: DirectHandle<BreakPointInfo>,
        break_point: DirectHandle<BreakPoint>,
    ) {
        unsafe {
            if (*break_point_info._dummy as *mut BreakPointInfo).break_points as *const _ == std::ptr::null() {
                return;
            }
            //if (!IsFixedArray((*break_point_info).break_points)) {
            //    if (IsEqual(Cast::<BreakPoint>((*break_point_info).break_points), *break_point)) {
            //        (*break_point_info).break_points = std::ptr::null_mut();
            //    }
            //    return;
            //}
            //let old_array = Cast::<FixedArray>((*break_point_info).break_points);
            //let new_array = FixedArray::new(old_array.length() - 1);
            //let mut found_count: i32 = 0;
            //for i in 0..old_array.length() {
            //    if (IsEqual(Cast::<BreakPoint>(old_array.get(i)), *break_point)) {
            //        assert_eq!(found_count, 0);
            //        found_count += 1;
            //    } else {
            //        new_array.set(i - found_count, old_array.get(i));
            //    }
            //}
            //if found_count > 0 {
            //    (*break_point_info).break_points = new_array;
            //}
        }
    }

    pub fn set_break_point(
        isolate: *mut Isolate,
        break_point_info: DirectHandle<BreakPointInfo>,
        break_point: DirectHandle<BreakPoint>,
    ) {
        unsafe {
            if (*break_point_info._dummy as *mut BreakPointInfo).break_points as *const _ == std::ptr::null() {
                (*break_point_info._dummy as *mut BreakPointInfo).break_points = (*break_point._dummy as *mut Object);
                return;
            }
            //if (!IsFixedArray((*break_point_info).break_points)) {
            //    if (IsEqual(Cast::<BreakPoint>((*break_point_info).break_points), *break_point)) {
            //        return;
            //    }
            //    let array = FixedArray::new(2);
            //    array.set(0, (*break_point_info).break_points);
            //    array.set(1, *break_point);
            //    (*break_point_info).break_points = array;
            //    return;
            //}
            //let old_array = Cast::<FixedArray>((*break_point_info).break_points);
            //let new_array = FixedArray::new(old_array.length() + 1);
            //for i in 0..old_array.length() {
            //    if (IsEqual(Cast::<BreakPoint>(old_array.get(i)), *break_point)) {
            //        return;
            //    }
            //    new_array.set(i, old_array.get(i));
            //}
            //new_array.set(old_array.length(), *break_point);
            //(*break_point_info).break_points = new_array;
        }
    }

    pub fn has_break_point(
        isolate: *mut Isolate,
        break_point_info: DirectHandle<BreakPointInfo>,
        break_point: DirectHandle<BreakPoint>,
    ) -> bool {
        unsafe {
            if (*break_point_info._dummy as *mut BreakPointInfo).break_points as *const _ == std::ptr::null() {
                return false;
            }
            //if (!IsFixedArray((*break_point_info).break_points)) {
            //    return IsEqual(Cast::<BreakPoint>((*break_point_info).break_points), *break_point);
            //}
            //let array = Cast::<FixedArray>((*break_point_info).break_points);
            //for i in 0..array.length() {
            //    if (IsEqual(Cast::<BreakPoint>(array.get(i)), *break_point)) {
            //        return true;
            //    }
            //}
            return false;
        }
    }

    pub fn get_break_point_by_id(
        isolate: *mut Isolate,
        break_point_info: DirectHandle<BreakPointInfo>,
        breakpoint_id: i32,
    ) -> MaybeDirectHandle<BreakPoint> {
        unsafe {
            if (*break_point_info._dummy as *mut BreakPointInfo).break_points as *const _ == std::ptr::null() {
                return MaybeDirectHandle::<BreakPoint> { _dummy: 0, phantom: std::marker::PhantomData };
            }
            //if (!IsFixedArray((*break_point_info).break_points)) {
            //    let breakpoint = Cast::<BreakPoint>((*break_point_info).break_points);
            //    if (breakpoint.id() == breakpoint_id) {
            //        return breakpoint;
            //    }
            //} else {
            //    let array = Cast::<FixedArray>((*break_point_info).break_points);
            //    for i in 0..array.length() {
            //        let breakpoint = Cast::<BreakPoint>(array.get(i));
            //        if (breakpoint.id() == breakpoint_id) {
            //            return breakpoint;
            //        }
            //    }
            //}
            MaybeDirectHandle::<BreakPoint> { _dummy: 0, phantom: std::marker::PhantomData }
        }
    }

    pub fn get_break_point_count(&self, isolate: *mut Isolate) -> i32 {
        if self.break_points as *const _ == std::ptr::null() {
            return 0;
        }
        //if (!IsFixedArray(self.break_points)) {
        //    return 1;
        //}
        //return Cast::<FixedArray>(self.break_points).length();
        0
    }

    pub fn get_statement_position(&self, debug_info: Handle<DebugInfo>) -> i32 {
        0
    }
    pub fn source_position(&self) -> i32 {
        self.source_position
    }
}

#[derive(Debug)]
pub struct CoverageInfo {
    slots: *mut i32,
    slot_count: i32,
}

impl CoverageInfo {
    pub fn initialize_slot(&mut self, slot_index: i32, start_pos: i32, end_pos: i32) {
        self.set_slots_start_source_position(slot_index, start_pos);
        self.set_slots_end_source_position(slot_index, end_pos);
        self.reset_block_count(slot_index);
        self.set_slots_padding(slot_index, 0);
    }

    pub fn reset_block_count(&mut self, slot_index: i32) {
        unsafe {
            *self.slots.offset(slot_index as isize * 4) = 0;
        }
    }

    pub fn size_for(slot_count: i32) -> i32 {
        0
    }

    pub fn coverage_info_print(&self, os: &mut std::ostream, function_name: std::unique_ptr<[char]>) {
        //os << "Coverage info (";
        //if (function_name == nullptr) {
        //    os << "{unknown}";
        //} else if (strlen(function_name.get()) > 0) {
        //    os << function_name.get();
        //} else {
        //    os << "{anonymous}";
        //}
        //os << "):" << std::endl;
        //for (int i = 0; i < slot_count(); i++) {
        //    os << "{" << slots_start_source_position(i) << ","
        //       << slots_end_source_position(i) << "}" << std::endl;
        //}
    }

    fn set_slots_start_source_position(&mut self, slot_index: i32, from_pos: i32) {
        unsafe {
            *self.slots.offset(slot_index as isize * 4 + 0) = from_pos;
        }
    }

    fn set_slots_end_source_position(&mut self, slot_index: i32, to_pos: i32) {
        unsafe {
            *self.slots.offset(slot_index as isize * 4 + 1) = to_pos;
        }
    }

    fn set_slots_padding(&mut self, slot_index: i32, padding: i32) {
        unsafe {
            *self.slots.offset(slot_index as isize * 4 + 2) = padding;
        }
    }

    fn set_slots_block_count(&mut self, slot_index: i32, block_count: i32) {
        unsafe {
            *self.slots.offset(slot_index as isize * 4 + 3) = block_count;
        }
    }

    fn slot_count(&self) -> i32 {
        self.slot_count
    }

}

#[derive(Debug)]
pub struct StackFrameInfo {
    shared_or_script: *mut Object,
    bytecode_offset_or_source_position: i32,
    flags: i32,
}

impl StackFrameInfo {
    pub fn get_source_position(info: DirectHandle<StackFrameInfo>) -> i32 {
        0
    }

    pub fn script(&self) -> Tagged<Script> {
        unsafe { Tagged::<Script> { _dummy: 0, phantom: std::marker::PhantomData } }
    }
    pub fn set_shared_or_script(&mut self, _script: Script) {}
}

#[derive(Debug)]
pub struct StackTraceInfo {
    frames: *mut FixedArray,
}

impl StackTraceInfo {
    pub fn length(&self) -> i32 {
        0
    }

    pub fn get(&self, index: i32) -> Tagged<StackFrameInfo> {
        unsafe { Tagged::<StackFrameInfo> { _dummy: 0, phantom: std::marker::PhantomData } }
    }
}

#[derive(Debug)]
pub struct ErrorStackData {
    formatted_stack: *mut Object,
    call_site_infos: *mut FixedArray,
}

impl ErrorStackData {
    pub fn has_formatted_stack(&self) -> bool {
        self.formatted_stack as *const _ != std::ptr::null()
    }

    pub fn has_call_site_infos(&self) -> bool {
        self.call_site_infos as *const _ != std::ptr::null()
    }
}
