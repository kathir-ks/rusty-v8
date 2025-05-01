// src/logging/code_events.rs

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::any::Any;
// use base::platform::mutex::Mutex; // Assuming a cross-platform mutex is desired
// use base::vector::Vector; // Using std::vec::Vec instead
// use common::globals::Address; // Using usize for address representation
// use objects::bytecode_array::BytecodeArray; // Assuming a suitable replacement exists
// use objects::code::Code; // Assuming a suitable replacement exists
// use objects::instruction_stream::InstructionStream; // Assuming a suitable replacement exists
// use objects::name::Name; // Assuming a suitable replacement exists
// use objects::shared_function_info::SharedFunctionInfo; // Assuming a suitable replacement exists
// use objects::string::String; // Assuming a suitable replacement exists

// Dummy types for V8 specific classes as replacements
#[derive(Debug, Clone, Copy)]
pub struct Address(usize);

pub type Tagged<T> = T;

#[derive(Debug, Clone, Copy)]
pub struct AbstractCode;

#[derive(Debug, Clone, Copy)]
pub struct Name;

#[derive(Debug, Clone, Copy)]
pub struct SharedFunctionInfo;

#[derive(Debug, Clone, Copy)]
pub struct String;

#[derive(Debug, Clone, Copy)]
pub struct BytecodeArray;

#[derive(Debug, Clone, Copy)]
pub struct Code;

#[derive(Debug, Clone, Copy)]
pub struct InstructionStream;

#[derive(Debug, Clone, Copy)]
pub struct RegExpFlags;

#[derive(Debug, Clone, Copy)]
pub struct DeoptimizeKind;

pub mod wasm {
    // pub type WasmName = base::Vector<const char>; // Using Vec<u8> for simplicity
    pub type WasmName = Vec<u8>;

    #[derive(Debug, Clone, Copy)]
    pub struct WasmCode;
}

macro_rules! log_event_list {
    ($V:ident) => {
        $V!(kCodeCreation, "code-creation");
        $V!(kCodeDisableOpt, "code-disable-optimization");
        $V!(kCodeMove, "code-move");
        $V!(kCodeDeopt, "code-deopt");
        $V!(kCodeDelete, "code-delete");
        $V!(kCodeMovingGC, "code-moving-gc");
        $V!(kSharedFuncMove, "sfi-move");
        $V!(kSnapshotCodeName, "snapshot-code-name");
        $V!(kTick, "tick");
    };
}

macro_rules! code_type_list {
    ($V:ident) => {
        $V!(kBuiltin, Builtin);
        $V!(kCallback, Callback);
        $V!(kEval, Eval);
        $V!(kFunction, JS);
        $V!(kHandler, Handler);
        $V!(kBytecodeHandler, BytecodeHandler);
        $V!(kRegExp, RegExp);
        $V!(kScript, Script);
        $V!(kStub, Stub);
        $V!(kNativeFunction, JS);
        $V!(kNativeScript, Script);
    };
}

// #[macro_export]
// macro_rules! profile {
//     ($the_isolate:expr, $Call:expr) => {
//         $the_isolate.logger().$Call
//     };
// }
//
// //This macro requires access to Isolate and Logger which cannot be provided without refactoring
// //The macro is kept commented for reference

#[allow(unused_variables)]
pub trait LogEventListener: Send + Sync {
    type Error;

    fn code_create_event_str(&self, tag: CodeTag, code: AbstractCode, name: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn code_create_event_name(&self, tag: CodeTag, code: AbstractCode, name: Name) -> Result<(), Self::Error> {
        Ok(())
    }

    fn code_create_event_shared(&self, tag: CodeTag, code: AbstractCode, shared: SharedFunctionInfo, script_name: Name) -> Result<(), Self::Error> {
        Ok(())
    }

    fn code_create_event_line(&self, tag: CodeTag, code: AbstractCode, shared: SharedFunctionInfo, script_name: Name, line: i32, column: i32) -> Result<(), Self::Error> {
        Ok(())
    }

    #[cfg(feature = "v8_enable_webassembly")]
    fn code_create_event_wasm(&self, tag: CodeTag, code: &wasm::WasmCode, name: wasm::WasmName, source_url: &str, code_offset: i32, script_id: i32) -> Result<(), Self::Error> {
        Ok(())
    }

    fn callback_event(&self, name: Name, entry_point: Address) -> Result<(), Self::Error> {
        Ok(())
    }

    fn getter_callback_event(&self, name: Name, entry_point: Address) -> Result<(), Self::Error> {
        Ok(())
    }

    fn setter_callback_event(&self, name: Name, entry_point: Address) -> Result<(), Self::Error> {
        Ok(())
    }

    fn regexp_code_create_event(&self, code: AbstractCode, source: String, flags: RegExpFlags) -> Result<(), Self::Error> {
        Ok(())
    }

    fn code_move_event(&self, from: Tagged<InstructionStream>, to: Tagged<InstructionStream>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn bytecode_move_event(&self, from: Tagged<BytecodeArray>, to: Tagged<BytecodeArray>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn shared_function_info_move_event(&self, from: Address, to: Address) -> Result<(), Self::Error> {
        Ok(())
    }

    fn native_context_move_event(&self, from: Address, to: Address) -> Result<(), Self::Error> {
        Ok(())
    }

    fn code_moving_gc_event(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn code_disable_opt_event(&self, code: AbstractCode, shared: SharedFunctionInfo) -> Result<(), Self::Error> {
        Ok(())
    }

    fn code_deopt_event(&self, code: Code, kind: DeoptimizeKind, pc: Address, fp_to_sp_delta: i32) -> Result<(), Self::Error> {
        Ok(())
    }

    fn code_dependency_change_event(&self, code: Code, shared: SharedFunctionInfo, reason: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn weak_code_clear_event(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn is_listening_to_code_events(&self) -> bool {
        false
    }

    fn allows_code_compaction(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Event {
    kCodeCreation,
    kCodeDisableOpt,
    kCodeMove,
    kCodeDeopt,
    kCodeDelete,
    kCodeMovingGC,
    kSharedFuncMove,
    kSnapshotCodeName,
    kTick,
    kLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CodeTag {
    kBuiltin,
    kCallback,
    kEval,
    kFunction,
    kHandler,
    kBytecodeHandler,
    kRegExp,
    kScript,
    kStub,
    kNativeFunction,
    kNativeScript,
    kLength,
}

impl From<u8> for CodeTag {
    fn from(value: u8) -> Self {
        match value {
            0 => CodeTag::kBuiltin,
            1 => CodeTag::kCallback,
            2 => CodeTag::kEval,
            3 => CodeTag::kFunction,
            4 => CodeTag::kHandler,
            5 => CodeTag::kBytecodeHandler,
            6 => CodeTag::kRegExp,
            7 => CodeTag::kScript,
            8 => CodeTag::kStub,
            9 => CodeTag::kNativeFunction,
            10 => CodeTag::kNativeScript,
            _ => CodeTag::kBuiltin
        }
    }
}

impl From<CodeTag> for u8 {
    fn from(code_tag: CodeTag) -> Self {
        match code_tag {
            CodeTag::kBuiltin => 0,
            CodeTag::kCallback => 1,
            CodeTag::kEval => 2,
            CodeTag::kFunction => 3,
            CodeTag::kHandler => 4,
            CodeTag::kBytecodeHandler => 5,
            CodeTag::kRegExp => 6,
            CodeTag::kScript => 7,
            CodeTag::kStub => 8,
            CodeTag::kNativeFunction => 9,
            CodeTag::kNativeScript => 10
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeType {
    Builtin,
    Callback,
    Eval,
    JS,
    Handler,
    BytecodeHandler,
    RegExp,
    Script,
    Stub,
}

#[derive(Default)]
pub struct Logger {
    listeners: Arc<Mutex<Vec<Box<dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>>>>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_listener(&self, listener: Box<dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>>) -> Result<bool, PoisonError<MutexGuard<'_, Vec<Box<dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>>>>>> {
        let mut guard = self.listeners.lock()?;
        if guard.iter().any(|l| {
            let listener_ptr = &*listener as *const dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>;
            let l_ptr = &**l as *const dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>;
            listener_ptr == l_ptr
        }) {
            return Ok(false);
        }
        guard.push(listener);
        Ok(true)
    }

    pub fn remove_listener(&self, listener: &dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>> -> Result<bool, PoisonError<MutexGuard<'_, Vec<Box<dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>>>>>> {
        let mut guard = self.listeners.lock()?;
        let listener_ptr = listener as *const dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>;
        if let Some(position) = guard.iter().position(|l| {
            let l_ptr = &**l as *const dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>;
            listener_ptr == l_ptr
        }) {
            guard.remove(position);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn is_listening_to_code_events(&self) -> Result<bool, PoisonError<MutexGuard<'_, Vec<Box<dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>>>>>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            if listener.is_listening_to_code_events() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn allows_code_compaction(&self) -> Result<bool, PoisonError<MutexGuard<'_, Vec<Box<dyn LogEventListener<Error = Box<dyn Any + Send + Sync>>>>>>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            if !listener.allows_code_compaction() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn code_create_event_str(&self, tag: CodeTag, code: AbstractCode, comment: &str) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_create_event_str(tag, code, comment)?;
        }
        Ok(())
    }

    pub fn code_create_event_name(&self, tag: CodeTag, code: AbstractCode, name: Name) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_create_event_name(tag, code, name)?;
        }
        Ok(())
    }

    pub fn code_create_event_shared(&self, tag: CodeTag, code: AbstractCode, shared: SharedFunctionInfo, name: Name) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_create_event_shared(tag, code, shared, name)?;
        }
        Ok(())
    }

    pub fn code_create_event_line(&self, tag: CodeTag, code: AbstractCode, shared: SharedFunctionInfo, source: Name, line: i32, column: i32) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_create_event_line(tag, code, shared, source, line, column)?;
        }
        Ok(())
    }

    #[cfg(feature = "v8_enable_webassembly")]
    pub fn code_create_event_wasm(&self, tag: CodeTag, code: &wasm::WasmCode, name: wasm::WasmName, source_url: &str, code_offset: i32, script_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_create_event_wasm(tag, code, name.clone(), source_url, code_offset, script_id)?;
        }
        Ok(())
    }

    pub fn callback_event(&self, name: Name, entry_point: Address) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.callback_event(name, entry_point)?;
        }
        Ok(())
    }

    pub fn getter_callback_event(&self, name: Name, entry_point: Address) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.getter_callback_event(name, entry_point)?;
        }
        Ok(())
    }

    pub fn setter_callback_event(&self, name: Name, entry_point: Address) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.setter_callback_event(name, entry_point)?;
        }
        Ok(())
    }

    pub fn regexp_code_create_event(&self, code: AbstractCode, source: String, flags: RegExpFlags) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.regexp_code_create_event(code, source, flags)?;
        }
        Ok(())
    }

    pub fn code_move_event(&self, from: Tagged<InstructionStream>, to: Tagged<InstructionStream>) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_move_event(from, to)?;
        }
        Ok(())
    }

    pub fn bytecode_move_event(&self, from: Tagged<BytecodeArray>, to: Tagged<BytecodeArray>) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.bytecode_move_event(from, to)?;
        }
        Ok(())
    }

    pub fn shared_function_info_move_event(&self, from: Address, to: Address) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.shared_function_info_move_event(from, to)?;
        }
        Ok(())
    }

    pub fn native_context_move_event(&self, from: Address, to: Address) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.native_context_move_event(from, to)?;
        }
        Ok(())
    }

    pub fn code_moving_gc_event(&self) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_moving_gc_event()?;
        }
        Ok(())
    }

    pub fn code_disable_opt_event(&self, code: AbstractCode, shared: SharedFunctionInfo) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_disable_opt_event(code, shared)?;
        }
        Ok(())
    }

    pub fn code_deopt_event(&self, code: Code, kind: DeoptimizeKind, pc: Address, fp_to_sp_delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_deopt_event(code, kind, pc, fp_to_sp_delta)?;
        }
        Ok(())
    }

    pub fn code_dependency_change_event(&self, code: Code, sfi: SharedFunctionInfo, reason: &str) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.code_dependency_change_event(code, sfi, reason)?;
        }
        Ok(())
    }

    pub fn weak_code_clear_event(&self) -> Result<(), Box<dyn std::error::Error>> {
        let guard = self.listeners.lock()?;
        for listener in guard.iter() {
            listener.weak_code_clear_event()?;
        }
        Ok(())
    }
}