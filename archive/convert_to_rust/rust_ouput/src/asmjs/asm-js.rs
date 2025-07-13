// Converted from V8 C++ source files:
// Header: asm-js.h
// Implementation: asm-js.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod asm_js {
    // Copyright 2016 the V8 project authors. All rights reserved.
    // Use of this source code is governed by a BSD-style license that can be
    // found in the LICENSE file.

    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::f64::NAN;
    use std::fmt::{format, Debug};
    use std::hash::{Hash, Hasher};
    use std::ops::Deref;
    use std::ptr;
    use std::rc::Rc;
    use std::sync::atomic::AtomicU16;
    use std::sync::Mutex;
    use crate::execution::isolate::MessageTemplate;

    use crate::deoptimizer::deoptimizer::Module;

    pub struct AccountingAllocator {}

    pub struct AsmWasmData {}

    pub struct FunctionLiteral {}

    pub struct JSArrayBuffer {}

    pub struct ParseInfo {}

    pub struct SharedFunctionInfo {}

    pub struct UnoptimizedCompilationJob {}

    pub struct Isolate {}

    pub struct JSReceiver {}

    pub struct Object {}

    pub struct Name {}

    pub struct HeapNumber {}

    pub struct JSFunction {}

    pub struct Script {}

    pub struct WasmModuleObject {}

    pub struct WasmInstanceObject {}

    pub struct ScopeInfo {}

    pub struct String_ExternalOneByteStringResource {}

    pub struct Local<'a, T> {
        _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Local<'a, T> {
        pub fn new() -> Self {
            Local {
                _marker: std::marker::PhantomData,
            }
        }
    }

    pub trait Value {}
    impl Value for i32 {}

    pub struct Builtins {}

    impl Builtins {
        pub fn code(&self, _builtin: Builtin) -> i32 {
            0
        }
    }

    pub enum Builtin {
        kMathAcos,
        kMathAsin,
        kMathAtan,
        kMathCos,
        kMathSin,
        kMathTan,
        kMathExp,
        kMathLog,
        kMathPow,
        kMathCeil,
        kMathFloor,
        kMathFround,
    }

    pub struct Factory {}

    impl Factory {
        pub fn InternalizeString(&self, _vector: base::StaticCharVector) -> DirectHandle<Name> {
            DirectHandle::new()
        }
        pub fn Infinity_string(&self) -> DirectHandle<Name> {
            DirectHandle::new()
        }
        pub fn NaN_string(&self) -> DirectHandle<Name> {
            DirectHandle::new()
        }
        pub fn NewHeapNumberFromBits(&self, _bits: u64) -> DirectHandle<HeapNumber> {
            DirectHandle::new()
        }

        pub fn InternalizeUtf8String(&self, _name: &str) -> DirectHandle<Name> {
            DirectHandle::new()
        }
    }

    pub struct DirectHandle<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> DirectHandle<T> {
        pub fn new() -> Self {
            DirectHandle {
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl DirectHandle<JSReceiver> {
        pub fn GetDataProperty(
            _isolate: *mut Isolate,
            _obj: DirectHandle<JSReceiver>,
            _name: DirectHandle<Name>,
        ) -> DirectHandle<Object> {
            DirectHandle::new()
        }
    }

    impl DirectHandle<JSFunction> {
        pub fn is_identical_to(&self, _other: *mut i32) -> bool {
            false
        }
    }

    pub struct Heap {}

    impl Heap {
        pub fn asm_module_size_bytes(&mut self) -> i32 {
            0
        }
    }

    pub struct AsmJs {}

    impl AsmJs {
        pub const kSingleFunctionName: &'static str = "__single_function__";

        pub fn NewCompilationJob(
            parse_info: *mut ParseInfo,
            literal: *mut FunctionLiteral,
            allocator: *mut AccountingAllocator,
        ) -> Result<Box<UnoptimizedCompilationJob>, String> {
            Ok(Box::new(UnoptimizedCompilationJob {}))
        }

        pub fn InstantiateAsmWasm(
            isolate: *mut Isolate,
            shared: DirectHandle<SharedFunctionInfo>,
            wasm_data: DirectHandle<AsmWasmData>,
            stdlib: DirectHandle<JSReceiver>,
            foreign: DirectHandle<JSReceiver>,
            memory: DirectHandle<JSArrayBuffer>,
        ) -> Result<DirectHandle<Object>, String> {
            Ok(DirectHandle::new())
        }
    }

    pub mod wasm {
        use std::collections::HashSet;
        use crate::asm_js::*;

        pub struct AsmJsParser {
            stdlib_uses: StdlibSet,
        }

        impl AsmJsParser {
            pub fn new(_zone: *mut i32, _stack_limit: i32, _stream: *mut i32) -> Self {
                AsmJsParser {
                    stdlib_uses: StdlibSet::new(),
                }
            }

            pub fn Run(&mut self) -> bool {
                true
            }

            pub fn failure_location(&self) -> i32 {
                0
            }

            pub fn failure_message(&self) -> &'static str {
                ""
            }

            pub fn module_builder(&self) -> ModuleBuilder {
                ModuleBuilder {}
            }

            pub fn stdlib_uses(&self) -> &StdlibSet {
                &self.stdlib_uses
            }
        }

        pub struct ZoneBuffer {}

        pub struct ModuleBuilder {}

        impl ModuleBuilder {
            pub fn WriteTo(&self, _buffer: *mut ZoneBuffer) {}
            pub fn WriteAsmJsOffsetTable(&self, _buffer: *mut ZoneBuffer) {}
        }

        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum StandardMember {
            kInfinity,
            kNaN,
            kMathAcos,
            kMathAsin,
            kMathAtan,
            kMathCos,
            kMathSin,
            kMathTan,
            kMathExp,
            kMathLog,
            kMathPow,
            kMathCeil,
            kMathFloor,
            kMathFround,
            kMathE,
            kMathLN10,
            kMathLN2,
            kMathLOG10E,
            kMathLOG2E,
            kMathPI,
            kMathSQRT1_2,
            kMathSQRT2,
            kInt8Array,
            kUint8Array,
            kInt16Array,
            kUint16Array,
            kInt32Array,
            kUint32Array,
            kFloat32Array,
            kFloat64Array,
        }

        #[derive(Debug, Default, Clone)]
        pub struct StdlibSet {
            members: HashSet<StandardMember>,
        }

        impl StdlibSet {
            pub fn new() -> Self {
                StdlibSet {
                    members: HashSet::new(),
                }
            }

            pub fn contains(&self, member: StandardMember) -> bool {
                self.members.contains(&member)
            }

            pub fn Remove(&mut self, member: StandardMember) {
                self.members.remove(&member);
            }

            pub fn empty(&self) -> bool {
                self.members.is_empty()
            }

            pub fn FromIntegral(value: u64) -> Self {
                let mut set = StdlibSet::new();
                if value & 1 != 0 {
                    set.members.insert(StandardMember::kInfinity);
                }
                if value & 2 != 0 {
                    set.members.insert(StandardMember::kNaN);
                }
                if value & 4 != 0 {
                    set.members.insert(StandardMember::kMathAcos);
                }
                if value & 8 != 0 {
                    set.members.insert(StandardMember::kMathAsin);
                }
                if value & 16 != 0 {
                    set.members.insert(StandardMember::kMathAtan);
                }
                if value & 32 != 0 {
                    set.members.insert(StandardMember::kMathCos);
                }
                if value & 64 != 0 {
                    set.members.insert(StandardMember::kMathSin);
                }
                if value & 128 != 0 {
                    set.members.insert(StandardMember::kMathTan);
                }
                if value & 256 != 0 {
                    set.members.insert(StandardMember::kMathExp);
                }
                if value & 512 != 0 {
                    set.members.insert(StandardMember::kMathLog);
                }
                if value & 1024 != 0 {
                    set.members.insert(StandardMember::kMathPow);
                }
                if value & 2048 != 0 {
                    set.members.insert(StandardMember::kMathCeil);
                }
                if value & 4096 != 0 {
                    set.members.insert(StandardMember::kMathFloor);
                }
                if value & 8192 != 0 {
                    set.members.insert(StandardMember::kMathFround);
                }
                if value & 16384 != 0 {
                    set.members.insert(StandardMember::kMathE);
                }
                if value & 32768 != 0 {
                    set.members.insert(StandardMember::kMathLN10);
                }
                if value & 65536 != 0 {
                    set.members.insert(StandardMember::kMathLN2);
                }
                if value & 131072 != 0 {
                    set.members.insert(StandardMember::kMathLOG10E);
                }
                if value & 262144 != 0 {
                    set.members.insert(StandardMember::kMathLOG2E);
                }
                if value & 524288 != 0 {
                    set.members.insert(StandardMember::kMathPI);
                }
                if value & 1048576 != 0 {
                    set.members.insert(StandardMember::kMathSQRT1_2);
                }
                if value & 2097152 != 0 {
                    set.members.insert(StandardMember::kMathSQRT2);
                }
                if value & 4194304 != 0 {
                    set.members.insert(StandardMember::kInt8Array);
                }
                if value & 8388608 != 0 {
                    set.members.insert(StandardMember::kUint8Array);
                }
                if value & 16777216 != 0 {
                    set.members.insert(StandardMember::kInt16Array);
                }
                if value & 33554432 != 0 {
                    set.members.insert(StandardMember::kUint16Array);
                }
                if value & 67108864 != 0 {
                    set.members.insert(StandardMember::kInt32Array);
                }
                if value & 134217728 != 0 {
                    set.members.insert(StandardMember::kUint32Array);
                }
                if value & 268435456 != 0 {
                    set.members.insert(StandardMember::kFloat32Array);
                }
                if value & 536870912 != 0 {
                    set.members.insert(StandardMember::kFloat64Array);
                }
                set
            }

            pub fn ToIntegral(&self) -> u64 {
                let mut value: u64 = 0;
                if self.members.contains(&StandardMember::kInfinity) {
                    value |= 1;
                }
                if self.members.contains(&StandardMember::kNaN) {
                    value |= 2;
                }
                if self.members.contains(&StandardMember::kMathAcos) {
                    value |= 4;
                }
                if self.members.contains(&StandardMember::kMathAsin) {
                    value |= 8;
                }
                if self.members.contains(&StandardMember::kMathAtan) {
                    value |= 16;
                }
                if self.members.contains(&StandardMember::kMathCos) {
                    value |= 32;
                }
                if self.members.contains(&StandardMember::kMathSin) {
                    value |= 64;
                }
                if self.members.contains(&StandardMember::kMathTan) {
                    value |= 128;
                }
                if self.members.contains(&StandardMember::kMathExp) {
                    value |= 256;
                }
                if self.members.contains(&StandardMember::kMathLog) {
                    value |= 512;
                }
                if self.members.contains(&StandardMember::kMathPow) {
                    value |= 1024;
                }
                if self.members.contains(&StandardMember::kMathCeil) {
                    value |= 2048;
                }
                if self.members.contains(&StandardMember::kMathFloor) {
                    value |= 4096;
                }
                if self.members.contains(&StandardMember::kMathFround) {
                    value |= 8192;
                }
                if self.members.contains(&StandardMember::kMathE) {
                    value |= 16384;
                }
                if self.members.contains(&StandardMember::kMathLN10) {
                    value |= 32768;
                }
                if self.members.contains(&StandardMember::kMathLN2) {
                    value |= 65536;
                }
                if self.members.contains(&StandardMember::kMathLOG10E) {
                    value |= 131072;
                }
                if self.members.contains(&StandardMember::kMathLOG2E) {
                    value |= 262144;
                }
                if self.members.contains(&StandardMember::kMathPI) {
                    value |= 524288;
                }
                if self.members.contains(&StandardMember::kMathSQRT1_2) {
                    value |= 1048576;
                }
                if self.members.contains(&StandardMember::kMathSQRT2) {
                    value |= 2097152;
                }
                if self.members.contains(&StandardMember::kInt8Array) {
                    value |= 4194304;
                }
                if self.members.contains(&StandardMember::kUint8Array) {
                    value |= 8388608;
                }
                if self.members.contains(&StandardMember::kInt16Array) {
                    value |= 16777216;
                }
                if self.members.contains(&StandardMember::kUint16Array) {
                    value |= 33554432;
                }
                if self.members.contains(&StandardMember::kInt32Array) {
                    value |= 67108864;
                }
                if self.members.contains(&StandardMember::kUint32Array) {
                    value |= 134217728;
                }
                if self.members.contains(&StandardMember::kFloat32Array) {
                    value |= 268435456;
                }
                if self.members.contains(&StandardMember::kFloat64Array) {
                    value |= 536870912;
                }
                value
            }
        }

        pub fn max_mem32_bytes() -> usize {
            4294967295
        }
        pub struct ErrorThrower<'a> {
            isolate: *mut Isolate,
            context: &'a str,
            error_msg: String,
        }

        impl<'a> ErrorThrower<'a> {
            pub fn new(isolate: *mut Isolate, context: &'a str) -> Self {
                ErrorThrower {
                    isolate,
                    context,
                    error_msg: String::new(),
                }
            }

            pub fn error(&self) -> bool {
                !self.error_msg.is_empty()
            }

            pub fn error_msg(&self) -> &str {
                &self.error_msg
            }

            pub fn Reset(&mut self) {}
        }
        pub struct WasmEngine {}

        impl WasmEngine {
            pub fn SyncCompileTranslatedAsmJs(
                &self,
                _isolate: *mut Isolate,
                _thrower: *mut ErrorThrower,
                _module: base::OwnedCopyOf<ZoneBuffer>,
                _script: *mut Script,
                _asm_offsets: base::VectorOf<ZoneBuffer>,
                _uses_bitset: DirectHandle<HeapNumber>,
                _language_mode: i32,
            ) -> Result<Handle<AsmWasmData>, String> {
                Ok(Handle::new())
            }

            pub fn FinalizeTranslatedAsmJs(
                &self,
                _isolate: *mut Isolate,
                _wasm_data: DirectHandle<AsmWasmData>,
                _script: *mut Script,
            ) -> DirectHandle<WasmModuleObject> {
                DirectHandle::new()
            }

            pub fn SyncInstantiate(
                &self,
                _isolate: *mut Isolate,
                _thrower: *mut ErrorThrower,
                _module: DirectHandle<WasmModuleObject>,
                _foreign: DirectHandle<JSReceiver>,
                _memory: DirectHandle<JSArrayBuffer>,
            ) -> Result<DirectHandle<WasmInstanceObject>, String> {
                Ok(DirectHandle::new())
            }
        }

        pub fn GetWasmEngine() -> &'static WasmEngine {
            static ENGINE: WasmEngine = WasmEngine {};
            &ENGINE
        }

        pub struct base {}
        impl base {
            pub struct OwnedCopyOf<T> {
                _marker: std::marker::PhantomData<T>,
            }

            impl<T> OwnedCopyOf<T> {
                pub fn new() -> Self {
                    OwnedCopyOf {
                        _marker: std::marker::PhantomData,
                    }
                }
            }

            pub struct VectorOf<T> {
                _marker: std::marker::PhantomData<T>,
            }
            impl<T> VectorOf<T> {
                pub fn new() -> Self {
                    VectorOf {
                        _marker: std::marker::PhantomData,
                    }
                }

                pub fn Of(_buffer: *mut ZoneBuffer) -> Self {
                    VectorOf {
                        _marker: std::marker::PhantomData,
                    }
                }
            }

            pub mod bits {
                pub fn IsPowerOfTwo(_size32: u32) -> bool {
                    false
                }
            }
        }
    }

    pub struct UnoptimizedCompilationInfo {}

    impl UnoptimizedCompilationInfo {
        pub fn literal(&self) -> FunctionLiteral {
            FunctionLiteral {}
        }

        pub fn SetAsmWasmData(&self, _result: Handle<AsmWasmData>) {}
    }

    pub struct Zone {}

    impl Zone {
        pub fn New<T>(&self, _zone: &Zone) -> Box<T> {
            Box::new(std::mem::zeroed())
        }
    }

    pub struct Utf16CharacterStream {}

    impl Utf16CharacterStream {
        pub fn can_access_heap(&self) -> bool {
            false
        }
        pub fn Seek(&self, _position: i32) {}
    }

    pub struct AllowHandleDereference {}

    pub struct Handle<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Handle<T> {
        pub fn new() -> Self {
            Handle {
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl Handle<AsmWasmData> {
        pub fn uses_bitset(&self) -> HeapNumber {
            HeapNumber {}
        }
    }

    pub struct Counters {}

    impl Counters {
        pub fn asm_module_size_bytes(&mut self) -> &mut i32 {
            &mut 0
        }
    }

    impl Isolate {
        pub fn counters(&mut self) -> &mut Counters {
            &mut Counters {}
        }
        pub fn factory(&mut self) -> &mut Factory {
            &mut Factory {}
        }
        pub fn builtins(&self) -> &Builtins {
            &Builtins {}
        }

        pub fn int8_array_fun(&self) -> *mut i32 {
            ptr::null_mut()
        }
        pub fn uint8_array_fun(&self) -> *mut i32 {
            ptr::null_mut()
        }
        pub fn int16_array_fun(&self) -> *mut i32 {
            ptr::null_mut()
        }
        pub fn uint16_array_fun(&self) -> *mut i32 {
            ptr::null_mut()
        }
        pub fn int32_array_fun(&self) -> *mut i32 {
            ptr::null_mut()
        }
        pub fn uint32_array_fun(&self) -> *mut i32 {
            ptr::null_mut()
        }
        pub fn float32_array_fun(&self) -> *mut i32 {
            ptr::null_mut()
        }
        pub fn float64_array_fun(&self) -> *mut i32 {
            ptr::null_mut()
        }

        pub fn is_execution_terminating(&self) -> bool {
            false
        }

        pub fn has_exception(&self) -> bool {
            false
        }

        pub fn clear_exception(&self) {}
    }

    impl JSReceiver {
        pub fn GetDataProperty(
            isolate: *mut Isolate,
            stdlib: DirectHandle<JSReceiver>,
            name: DirectHandle<Name>,
        ) -> DirectHandle<Object> {
            DirectHandle::new()
        }
    }

    impl Object {
        pub fn NumberValue(_value: DirectHandle<Object>) -> f64 {
            0.0
        }

        pub fn GetProperty(
            _isolate: *mut Isolate,
            _instance: DirectHandle<WasmInstanceObject>,
            _single_function_name: DirectHandle<Name>,
        ) -> Result<DirectHandle<Object>, String> {
            Ok(DirectHandle::new())
        }
    }

    pub struct base {}

    impl base {
        pub struct StaticCharVector {
            _marker: std::marker::PhantomData<u8>,
        }
        impl StaticCharVector {
            pub fn new() -> Self {
                StaticCharVector {
                    _marker: std::marker::PhantomData,
                }
            }
        }

        pub struct CStrVector {
            _marker: std::marker::PhantomData<u8>,
        }
        impl CStrVector {
            pub fn new() -> Self {
                CStrVector {
                    _marker: std::marker::PhantomData,
                }
            }
        }
    }
    pub struct MessageLocation {
        _marker: std::marker::PhantomData<u8>,
    }
    impl MessageLocation {
        pub fn new() -> Self {
            MessageLocation {
                _marker: std::marker::PhantomData,
            }
        }
    }

    pub struct JSMessageObject {
        _marker: std::marker::PhantomData<u8>,
        error_level: v8::Isolate::MessageErrorLevel,
    }
    impl JSMessageObject {
        pub fn new() -> Self {
            JSMessageObject {
                _marker: std::marker::PhantomData,
                error_level: v8::Isolate::MessageErrorLevel::kMessageError,
            }
        }

        pub fn set_error_level(&mut self, level: v8::Isolate::MessageErrorLevel) {
            self.error_level = level;
        }
    }

    pub struct MessageHandler {}
    impl MessageHandler {
        pub fn MakeMessageObject(
            _isolate: *mut Isolate,
            _message_template: MessageTemplate,
            _location: *mut MessageLocation,
            _text_object: DirectHandle<String>,
        ) -> DirectHandle<JSMessageObject> {
            DirectHandle::new()
        }
        pub fn ReportMessage(
            _isolate: *mut Isolate,
            _location: *mut MessageLocation,
            _message: DirectHandle<JSMessageObject>,
        ) {
        }
    }

    pub struct Flags {}
    impl Flags {
        pub fn suppress_asm_messages(&self) -> bool {
            false
        }
        pub fn trace_asm_time(&self) -> bool {
            false
        }
        pub fn wasm_max_module_size(&self) -> i32 {
            0
        }
    }

    pub mod v8_flags {
        use crate::asm_js::*;
        pub fn suppress_asm_messages() -> bool {
            false
        }
        pub fn trace_asm_time() -> bool {
            false
        }
        pub fn wasm_max_module_size() -> i32 {
            0
        }
    }

    impl Script {
        pub fn GetIsolate(&self) -> *mut Isolate {
            ptr::null_mut()
        }
    }

    impl SharedFunctionInfo {
        pub fn script(&self) -> *mut i32 {
            ptr::null_mut()
        }

        pub fn StartPosition(&self) -> i32 {
            0
        }
        pub fn GetCode(&self, _isolate: *mut Isolate) -> i32 {
            0
        }
        pub fn HasBuiltinId(&self) -> bool {
            false
        }
        pub fn builtin_id(&self) -> Builtin {
            Builtin::kMathAcos
        }
        pub fn language_mode(&self) -> i32 {
            0
        }
        pub fn scope_info(&self) -> ScopeInfo {
            ScopeInfo {}
        }
    }

    impl ScopeInfo {
        pub fn function_kind(&self) -> i32 {
            0
        }
    }

    pub struct WasmInstanceObject {}

    impl WasmInstanceObject {
        pub fn exports_object(&self) -> *mut i32 {
            ptr::null_mut()
        }
    }
}
