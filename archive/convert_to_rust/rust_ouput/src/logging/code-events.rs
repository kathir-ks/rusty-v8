// Converted from V8 C++ source files:
// Header: code-events.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod platform {
        use std::sync::Mutex;

        pub type RecursiveMutex = Mutex<()>;
        pub struct RecursiveMutexGuard<'a>(&'a RecursiveMutex);

        impl<'a> RecursiveMutexGuard<'a> {
            pub fn new(mutex: &'a RecursiveMutex) -> Self {
                RecursiveMutexGuard(mutex)
            }
        }
        impl<'a> Drop for RecursiveMutexGuard<'a> {
            fn drop(&mut self) {}
        }
    }
    pub struct Vector<T>(Vec<T>);

    impl<T> Vector<T> {
        pub fn begin(&self) -> std::slice::Iter<T> {
            self.0.iter()
        }
        pub fn end(&self) -> std::slice::Iter<T> {
            self.0.iter()
        }
    }
}

pub mod common {
    pub mod globals {
        pub type Address = usize;
    }
}

pub mod objects {
    pub struct BytecodeArray {}
    pub struct Code {}
    pub struct InstructionStream {}
    pub struct Name {}
    pub struct SharedFunctionInfo {}
    pub struct String {}
}

pub mod internal {
    use std::sync::Mutex;

    use super::base;
    use super::common::globals::Address;
    use super::objects::{
        BytecodeArray, Code, InstructionStream, Name, SharedFunctionInfo, String,
    };
    use crate::CodeTag;
    use std::fmt;

    pub struct DirectHandle<T>(*mut T);

    impl<T> DirectHandle<T> {
        pub fn new(ptr: *mut T) -> Self {
            DirectHandle(ptr)
        }
    }

    impl<T> Copy for DirectHandle<T> {}

    impl<T> Clone for DirectHandle<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T> fmt::Debug for DirectHandle<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DirectHandle")
                .field("ptr", &self.0)
                .finish()
        }
    }

    pub struct Tagged<T>(T);

    impl<T> Tagged<T> {
        pub fn new(value: T) -> Self {
            Tagged(value)
        }
    }

    pub mod wasm {
        use super::base::Vector;
        pub struct WasmCode {}
        pub type WasmName = Vector<char>;
    }

    pub struct AbstractCode {}

    #[derive(Debug, Copy, Clone)]
    pub enum DeoptimizeKind {}

    pub struct LogEventListener {
        is_listening: bool,
        allows_compaction: bool,
    }

    impl LogEventListener {
        pub fn new() -> Self {
            LogEventListener {
                is_listening: false,
                allows_compaction: true,
            }
        }

        pub fn set_is_listening(&mut self, value: bool) {
            self.is_listening = value;
        }
        pub fn set_allows_compaction(&mut self, value: bool) {
            self.allows_compaction = value;
        }
    }

    impl LogEventListener {
        pub fn default() -> Self {
            LogEventListener {
                is_listening: false,
                allows_compaction: true,
            }
        }
        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: DirectHandle<AbstractCode>,
            name: *const i8,
        ) {
        }
        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: DirectHandle<AbstractCode>,
            name: DirectHandle<Name>,
        ) {
        }
        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: DirectHandle<AbstractCode>,
            shared: DirectHandle<SharedFunctionInfo>,
            script_name: DirectHandle<Name>,
        ) {
        }
        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: DirectHandle<AbstractCode>,
            shared: DirectHandle<SharedFunctionInfo>,
            script_name: DirectHandle<Name>,
            line: i32,
            column: i32,
        ) {
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: *const wasm::WasmCode,
            name: wasm::WasmName,
            source_url: *const i8,
            code_offset: i32,
            script_id: i32,
        ) {
        }

        pub fn CallbackEvent(&mut self, name: DirectHandle<Name>, entry_point: Address) {}
        pub fn GetterCallbackEvent(&mut self, name: DirectHandle<Name>, entry_point: Address) {}
        pub fn SetterCallbackEvent(&mut self, name: DirectHandle<Name>, entry_point: Address) {}
        pub fn RegExpCodeCreateEvent(
            &mut self,
            code: DirectHandle<AbstractCode>,
            source: DirectHandle<String>,
            flags: RegExpFlags,
        ) {
        }
        pub fn CodeMoveEvent(&mut self, from: Tagged<InstructionStream>, to: Tagged<InstructionStream>) {}
        pub fn BytecodeMoveEvent(&mut self, from: Tagged<BytecodeArray>, to: Tagged<BytecodeArray>) {}
        pub fn SharedFunctionInfoMoveEvent(&mut self, from: Address, to: Address) {}
        pub fn NativeContextMoveEvent(&mut self, from: Address, to: Address) {}
        pub fn CodeMovingGCEvent(&mut self) {}
        pub fn CodeDisableOptEvent(
            &mut self,
            code: DirectHandle<AbstractCode>,
            shared: DirectHandle<SharedFunctionInfo>,
        ) {
        }
        pub fn CodeDeoptEvent(
            &mut self,
            code: DirectHandle<Code>,
            kind: DeoptimizeKind,
            pc: Address,
            fp_to_sp_delta: i32,
        ) {
        }
        pub fn CodeDependencyChangeEvent(
            &mut self,
            code: DirectHandle<Code>,
            shared: DirectHandle<SharedFunctionInfo>,
            reason: *const i8,
        ) {
        }
        pub fn WeakCodeClearEvent(&mut self) {}

        pub fn is_listening_to_code_events(&self) -> bool {
            self.is_listening
        }

        pub fn allows_code_compaction(&self) -> bool {
            self.allows_compaction
        }
    }

    // Dispatches events to a set of registered listeners.
    pub struct Logger {
        listeners_: Vec<Box<LogEventListener>>,
        mutex_: base::platform::RecursiveMutex,
    }

    impl Logger {
        pub fn new() -> Self {
            Logger {
                listeners_: Vec::new(),
                mutex_: Mutex::new(()),
            }
        }

        pub fn AddListener(&mut self, listener: Box<LogEventListener>) -> bool {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            if self.listeners_.iter().any(|l| {
                (l as *const LogEventListener) == (listener.as_ref() as *const LogEventListener)
            }) {
                return false;
            }
            self.listeners_.push(listener);
            true
        }

        pub fn RemoveListener(&mut self, listener: *const LogEventListener) -> bool {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            let position = self
                .listeners_
                .iter()
                .position(|l| (l.as_ref() as *const LogEventListener) == listener);
            match position {
                Some(index) => {
                    self.listeners_.remove(index);
                    true
                }
                None => false,
            }
        }

        pub fn is_listening_to_code_events(&self) -> bool {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &self.listeners_ {
                if listener.is_listening_to_code_events() {
                    return true;
                }
            }
            false
        }

        pub fn allows_code_compaction(&self) -> bool {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &self.listeners_ {
                if !listener.allows_code_compaction() {
                    return false;
                }
            }
            true
        }

        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: DirectHandle<AbstractCode>,
            comment: *const i8,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeCreateEvent(tag, code, comment);
            }
        }

        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: DirectHandle<AbstractCode>,
            name: DirectHandle<Name>,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeCreateEvent(tag, code, name);
            }
        }

        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: DirectHandle<AbstractCode>,
            shared: DirectHandle<SharedFunctionInfo>,
            name: DirectHandle<Name>,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeCreateEvent(tag, code, shared, name);
            }
        }

        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: DirectHandle<AbstractCode>,
            shared: DirectHandle<SharedFunctionInfo>,
            source: DirectHandle<Name>,
            line: i32,
            column: i32,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeCreateEvent(tag, code, shared, source, line, column);
            }
        }

        #[cfg(V8_ENABLE_WEBASSEMBLY)]
        pub fn CodeCreateEvent(
            &mut self,
            tag: CodeTag,
            code: *const wasm::WasmCode,
            name: wasm::WasmName,
            source_url: *const i8,
            code_offset: i32,
            script_id: i32,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeCreateEvent(tag, code, name, source_url, code_offset, script_id);
            }
        }

        pub fn CallbackEvent(&mut self, name: DirectHandle<Name>, entry_point: Address) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CallbackEvent(name, entry_point);
            }
        }

        pub fn GetterCallbackEvent(&mut self, name: DirectHandle<Name>, entry_point: Address) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.GetterCallbackEvent(name, entry_point);
            }
        }

        pub fn SetterCallbackEvent(&mut self, name: DirectHandle<Name>, entry_point: Address) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.SetterCallbackEvent(name, entry_point);
            }
        }

        pub fn RegExpCodeCreateEvent(
            &mut self,
            code: DirectHandle<AbstractCode>,
            source: DirectHandle<String>,
            flags: RegExpFlags,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.RegExpCodeCreateEvent(code, source, flags);
            }
        }

        pub fn CodeMoveEvent(&mut self, from: Tagged<InstructionStream>, to: Tagged<InstructionStream>) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeMoveEvent(from, to);
            }
        }

        pub fn BytecodeMoveEvent(&mut self, from: Tagged<BytecodeArray>, to: Tagged<BytecodeArray>) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.BytecodeMoveEvent(from, to);
            }
        }

        pub fn SharedFunctionInfoMoveEvent(&mut self, from: Address, to: Address) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.SharedFunctionInfoMoveEvent(from, to);
            }
        }

        pub fn NativeContextMoveEvent(&mut self, from: Address, to: Address) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.NativeContextMoveEvent(from, to);
            }
        }

        pub fn CodeMovingGCEvent(&mut self) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeMovingGCEvent();
            }
        }

        pub fn CodeDisableOptEvent(
            &mut self,
            code: DirectHandle<AbstractCode>,
            shared: DirectHandle<SharedFunctionInfo>,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeDisableOptEvent(code, shared);
            }
        }

        pub fn CodeDeoptEvent(
            &mut self,
            code: DirectHandle<Code>,
            kind: DeoptimizeKind,
            pc: Address,
            fp_to_sp_delta: i32,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeDeoptEvent(code, kind, pc, fp_to_sp_delta);
            }
        }

        pub fn CodeDependencyChangeEvent(
            &mut self,
            code: DirectHandle<Code>,
            sfi: DirectHandle<SharedFunctionInfo>,
            reason: *const i8,
        ) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.CodeDependencyChangeEvent(code, sfi, reason);
            }
        }

        pub fn WeakCodeClearEvent(&mut self) {
            let guard = base::platform::RecursiveMutexGuard::new(&self.mutex_);
            for listener in &mut self.listeners_ {
                listener.WeakCodeClearEvent();
            }
        }
    }
    #[derive(Debug, Copy, Clone)]
    pub enum RegExpFlags {}
}

pub use internal::DeoptimizeKind;
pub use internal::Logger;

#[derive(Debug, Copy, Clone)]
pub enum LogEvent {
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

#[derive(Debug, Copy, Clone)]
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
