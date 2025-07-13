// Converted from V8 C++ source files:
// Header: v8-forward.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_local_handle {
    pub struct Local<T> {
        pub value: *mut T,
    }

    impl<T> Local<T> {
        pub fn new(value: *mut T) -> Self {
            Local { value }
        }

        pub fn empty() -> Self {
            Local { value: std::ptr::null_mut() }
        }

        pub fn is_empty(&self) -> bool {
            self.value.is_null()
        }
    }

    impl<T> Copy for Local<T> {}

    impl<T> Clone for Local<T> {
        fn clone(&self) -> Self {
            *self
        }
    }
}

pub use v8_local_handle::Local;

pub struct AccessorSignature;
pub struct ArrayBuffer;
pub struct ArrayBufferView;
pub struct BigInt;
pub struct BigInt64Array;
pub struct BigIntObject;
pub struct BigUint64Array;
pub struct Boolean;
pub struct BooleanObject;
pub struct DataView;
pub struct Date;
pub struct DictionaryTemplate;
pub struct Extension;
pub struct External;
pub struct FixedArray;
pub struct Float32Array;
pub struct Float64Array;
template_struct!(FunctionCallbackInfo, F); // mimic the C++ template syntax
pub struct FunctionTemplate;
pub struct Int16Array;
pub struct Int32;
pub struct Int32Array;
pub struct Int8Array;
pub struct Integer;
pub struct Map;
pub struct Module;
pub struct Number;
pub struct NumberObject;
pub struct ObjectTemplate;
pub struct Platform;
pub struct Primitive;
pub struct Private;
pub struct Proxy;
pub struct RegExp;
pub struct Script;
pub struct Set;
pub struct SharedArrayBuffer;
pub struct Signature;
pub struct Symbol;
pub struct SymbolObject;
pub struct Template;
pub struct TryCatch;
pub struct TypedArray;
pub struct Uint16Array;
pub struct Uint32;
pub struct Uint32Array;
pub struct Uint8Array;
pub struct Uint8ClampedArray;
pub struct UnboundModuleScript;
pub struct WasmMemoryObject;
pub struct WasmModuleObject;

macro_rules! template_struct {
    ($name:ident, $generic:ident) => {
        pub struct $name<$generic> {
            _phantom: std::marker::PhantomData<$generic>,
        }

        impl<F> $name<F> {
            pub fn new() -> Self {
                $name {
                    _phantom: std::marker::PhantomData,
                }
            }
        }
    };
}
