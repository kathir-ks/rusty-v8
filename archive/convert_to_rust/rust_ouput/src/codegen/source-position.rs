// Converted from V8 C++ source files:
// Header: source-position.h
// Implementation: source-position.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/codegen/source-position.h

use std::fmt;

// use crate::base::bit_field::BitField; // Assuming this is correctly implemented elsewhere
use crate::common::globals::*;
// use crate::flags::flags; // Assuming this is correctly implemented elsewhere
// use crate::handles::handles; // Assuming this is correctly implemented elsewhere

// use crate::codegen::instruction_stream::InstructionStream;
use crate::compiler::js_inlining::OptimizedCompilationInfo;
use crate::snapshot::snapshot::Script;
use crate::snapshot::snapshot::SharedFunctionInfo;
// use crate::codegen::source_position_info::SourcePositionInfo;
use crate::execution::isolate::Isolate;
use crate::objects::objects::*;
use crate::objects::objects_inl::*;
use crate::strings::string::String;
use crate::codegen::external_reference_encoder::index;
use crate::ast::modules::Location;

pub struct SourcePositionInfo {
    pub position: SourcePosition,
    pub shared: IndirectHandle<SharedFunctionInfo>,
    pub script: IndirectHandle<Script>,
    pub line: i32,
    pub column: i32,
}

// A defined inlining_id refers to positions in
// OptimizedCompilationInfo::inlined_functions or
// DeoptimizationData::InliningPositions, depending on the compilation stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourcePosition {
    value_: u64,
}

impl SourcePosition {
    pub fn new(script_offset: i32, inlining_id: i32) -> Self {
        let mut pos = SourcePosition { value_: 0 };
        pos.set_is_external(false);
        pos.set_script_offset(script_offset);
        pos.set_inlining_id(inlining_id);
        pos
    }

    // External SourcePositions should use the following method to construct
    // SourcePositions to avoid confusion.
    pub fn external(line: i32, file_id: i32) -> Self {
        SourcePosition::external_with_inlining(line, file_id, SourcePosition::K_NOT_INLINED)
    }

    fn external_with_inlining(line: i32, file_id: i32, inlining_id: i32) -> Self {
        let mut pos = SourcePosition { value_: 0 };
        pos.set_is_external(true);
        pos.set_external_line(line);
        pos.set_external_file_id(file_id);
        pos.set_inlining_id(inlining_id);
        pos
    }

    pub fn unknown() -> Self {
        SourcePosition { value_: 0 }
    }

    pub fn is_known(&self) -> bool {
        self.raw() != SourcePosition::unknown().raw()
    }

    pub fn is_inlined(&self) -> bool {
        if self.is_external() {
            return false;
        }
        self.inlining_id() != SourcePosition::K_NOT_INLINED
    }

    pub fn is_external(&self) -> bool {
        Self::IsExternalField::decode(self.value_)
    }

    pub fn is_javascript(&self) -> bool {
        !self.is_external()
    }

    pub fn external_line(&self) -> i32 {
        assert!(self.is_external());
        Self::ExternalLineField::decode(self.value_)
    }

    pub fn external_file_id(&self) -> i32 {
        assert!(self.is_external());
        Self::ExternalFileIdField::decode(self.value_)
    }

    // Assumes that the code object is optimized.
    pub fn inlining_stack_from_code(&self, isolate: &mut Isolate, code: Tagged<Code>) -> Vec<SourcePositionInfo> {
        let deopt_data = unsafe { code.deoptimization_data().cast::<DeoptimizationData>() };

        let mut pos = *self;
        let mut stack = Vec::new();
        while pos.is_inlined() {
            let inl = unsafe { deopt_data.inlining_positions().get(pos.inlining_id()) };
            let function = unsafe { DirectHandle::new(deopt_data.get_inlined_function(inl.inlined_function_id()), isolate) };
            stack.push(SourcePositionInfo::new(isolate, pos, function));
            pos = inl.position;
        }

        let function = unsafe { DirectHandle::new(deopt_data.get_shared_function_info(), isolate) };
        stack.push(SourcePositionInfo::new(isolate, pos, function));
        stack
    }

    pub fn inlining_stack_from_cinfo(&self, isolate: &mut Isolate, cinfo: &OptimizedCompilationInfo) -> Vec<SourcePositionInfo> {
        let mut pos = *self;
        let mut stack = Vec::new();
        while pos.is_inlined() {
            let inl = &cinfo.inlined_functions()[pos.inlining_id() as usize]; // potential panic here
            stack.push(SourcePositionInfo::new(isolate, pos, inl.shared_info));
            pos = inl.position.position;
        }
        stack.push(SourcePositionInfo::new(isolate, pos, cinfo.shared_info()));
        stack
    }

    pub fn first_info_from_code(&self, isolate: &mut Isolate, code: Tagged<Code>) -> SourcePositionInfo {
        // DisallowGarbageCollection no_gc; // Assuming DisallowGarbageCollection is not needed in Rust
        let deopt_data = unsafe { code.deoptimization_data().cast::<DeoptimizationData>() };
        let mut pos = *self;
        if pos.is_inlined() {
            let inl = unsafe { deopt_data.inlining_positions().get(pos.inlining_id()) };
            let function = unsafe { DirectHandle::new(deopt_data.get_inlined_function(inl.inlined_function_id()), isolate) };
            return SourcePositionInfo::new(isolate, pos, function);
        }

        let function = unsafe { DirectHandle::new(deopt_data.get_shared_function_info(), isolate) };
        SourcePositionInfo::new(isolate, pos, function)
    }

    pub fn print(&self, out: &mut dyn std::fmt::Write, code: Tagged<Code>) -> fmt::Result {
       let deopt_data = unsafe { code.deoptimization_data().cast::<DeoptimizationData>() };
        if !self.is_inlined() {
            let function = unsafe { deopt_data.get_shared_function_info() };
            self.print_shared_function_info(out, function)
        } else {
            let inl = unsafe { deopt_data.inlining_positions().get(self.inlining_id()) };
            if inl.inlined_function_id == -1 {
                write!(out, "{:?}", self)?;
            } else {
                let function = unsafe { deopt_data.get_inlined_function(inl.inlined_function_id()) };
                self.print_shared_function_info(out, function)?;
            }
            write!(out, " inlined at ")?;
            inl.position.print(out, code)?;
        }
       Ok(())
    }

    fn print_shared_function_info(&self, out: &mut dyn fmt::Write, function: Tagged<SharedFunctionInfo>) -> fmt::Result {
        // Script::PositionInfo pos;
        // Tagged<Object> source_name;
        if unsafe { function.script().is_script() } {
            let script = unsafe { function.script().cast::<Script>() };
            let source_name = unsafe { script.name() };
            let mut pos = ScriptPositionInfo { line: 0, column: 0 };
            script.get_position_info(self.script_offset(), &mut pos);

            write!(out, "<")?;
            if unsafe { source_name.is_string() } {
                let source_name_str = unsafe { source_name.cast::<String>() };
                write!(out, "{}", unsafe { source_name_str.to_string() })?;
            } else {
                write!(out, "unknown")?;
            }
            write!(out, ":{}:{}>", pos.line + 1, pos.column + 1)?;
        } else {
             write!(out, "<unknown>")?;
        }
       Ok(())
    }

    pub fn print_json(&self, out: &mut dyn std::fmt::Write) -> fmt::Result {
        if self.is_external() {
            write!(
                out,
                r#"{{"line" : {}, "fileId" : {}, "inliningId" : {}}}"#,
                self.external_line(),
                self.external_file_id(),
                self.inlining_id()
            )
        } else {
            write!(
                out,
                r#"{{"scriptOffset" : {}, "inliningId" : {}}}"#,
                self.script_offset(),
                self.inlining_id()
            )
        }
    }

    pub fn script_offset(&self) -> i32 {
        assert!(self.is_javascript());
        Self::ScriptOffsetField::decode(self.value_) - 1
    }

    pub fn inlining_id(&self) -> i32 {
        Self::InliningIdField::decode(self.value_) - 1
    }

    pub fn set_is_external(&mut self, external: bool) {
        self.value_ = Self::IsExternalField::update(self.value_, external);
    }

    pub fn set_external_line(&mut self, line: i32) {
        assert!(self.is_external());
        self.value_ = Self::ExternalLineField::update(self.value_, line);
    }

    pub fn set_external_file_id(&mut self, file_id: i32) {
        assert!(self.is_external());
        self.value_ = Self::ExternalFileIdField::update(self.value_, file_id);
    }

    pub fn set_script_offset(&mut self, script_offset: i32) {
        assert!(self.is_javascript());
        assert!(script_offset >= SourcePosition::K_NO_SOURCE_POSITION);
        self.value_ = Self::ScriptOffsetField::update(self.value_, script_offset + 1);
    }

    pub fn set_inlining_id(&mut self, inlining_id: i32) {
        assert!(inlining_id >= SourcePosition::K_NOT_INLINED);
        self.value_ = Self::InliningIdField::update(self.value_, inlining_id + 1);
    }

    pub const K_NOT_INLINED: i32 = -1;
    pub const K_NO_SOURCE_POSITION: i32 = -1;

    pub fn raw(&self) -> i64 {
        self.value_ as i64
    }

    pub fn from_raw(raw: i64) -> Self {
        let mut position = SourcePosition::unknown();
        assert!(raw >= 0);
        position.value_ = raw as u64;
        position
    }

    type IsExternalField = BitField64<bool, 0, 1>;
    type ExternalLineField = BitField64<i32, 1, 20>;
    type ExternalFileIdField = BitField64<i32, 21, 10>;
    type ScriptOffsetField = BitField64<i32, 1, 30>;
    type InliningIdField = BitField64<i32, 31, 16>;

}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_inlined() {
            write!(f, "<inlined({}):", self.inlining_id())?;
        } else {
            write!(f, "<not inlined:")?;
        }

        if self.is_external() {
            write!(f, "{}, {}>", self.external_line(), self.external_file_id())
        } else {
            write!(f, "{}>", self.script_offset())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InliningPosition {
    // position of the inlined call
    pub position: SourcePosition,

    // references position in DeoptimizationData::literals()
    pub inlined_function_id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmInliningPosition {
    // Non-canonicalized (module-specific) index of the inlined function.
    pub inlinee_func_index: i32,
    // Whether the call was a tail call.
    pub was_tail_call: bool,
    // Source location of the caller.
    pub caller_pos: SourcePosition,
}

struct ScriptPositionInfo {
    line: i32,
    column: i32,
}

impl SourcePositionInfo {
    pub fn new(isolate: &mut Isolate, pos: SourcePosition, sfi: DirectHandle<SharedFunctionInfo>) -> Self {
        let shared = indirect_handle(sfi, isolate);
        let mut script: IndirectHandle<Script> = IndirectHandle::null();
        // {
        //     DisallowGarbageCollection no_gc; // Assuming this is handled correctly
        if unsafe { !sfi.is_null() } {
            let maybe_script = unsafe { sfi.script() };
            if unsafe { maybe_script.is_script() } {
                let handle = unsafe { handle(maybe_script.cast::<Script>(), isolate) };
                script = indirect_handle(handle, isolate);
            }
        }
        // }
        let mut info = ScriptPositionInfo { line: -1, column: -1 };
        if unsafe { Script::get_position_info(script, pos.script_offset(), &mut info) } {
        }

        SourcePositionInfo {
            position: pos,
            shared: shared,
            script: script,
            line: info.line,
            column: info.column,
        }
    }
}

impl fmt::Display for SourcePositionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<")?;
        if unsafe { !self.script.is_null() && self.script.name().is_string() } {
            let name = unsafe { self.script.name().cast::<String>() };
            write!(f, "{}", unsafe { name.to_string() })?;
        } else {
            write!(f, "unknown")?;
        }
        write!(f, ":{}:{}>", self.line + 1, self.column + 1)
    }
}

impl fmt::Display for Vec<SourcePositionInfo> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for pos in self {
            if !first {
                write!(f, " inlined at ")?;
            }
            write!(f, "{}", pos)?;
            first = false;
        }
        Ok(())
    }
}

// Mock BitField64 struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T, const OFFSET: usize, const SIZE: usize> BitField64<T, OFFSET, SIZE> {
    const fn decode(value: u64) -> T
    where
        T: From<u64> + std::convert::TryFrom<u64>,
    {
        let mask: u64 = (1 << SIZE) - 1;
        let shifted_value = (value >> OFFSET) & mask;
        T::try_from(shifted_value).unwrap_or_else(|_| panic!("Failed to convert"))
    }

    const fn update(value: u64, new_value: T) -> u64
    where
        T: Into<u64>,
    {
        let new_value_u64: u64 = new_value.into();
        let mask: u64 = ((1u64 << SIZE) - 1) << OFFSET;
        let cleared_value = value & !mask;
        let shifted_new_value = new_value_u64 << OFFSET;
        cleared_value | shifted_new_value
    }
}

// Implement necessary traits for bool
impl From<bool> for u64 {
    fn from(b: bool) -> Self {
        if b {
            1
        } else {
            0
        }
    }
}

impl std::convert::TryFrom<u64> for bool {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        }
    }
}

// Implement necessary traits for i32
impl From<i32> for u64 {
    fn from(i: i32) -> Self {
        i as u64
    }
}

impl std::convert::TryFrom<u64> for i32 {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > i32::MAX as u64 {
            Err(())
        } else {
            Ok(value as i32)
        }
    }
}
