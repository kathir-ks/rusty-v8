// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod v8_forward {
    use v8_local_handle::Local;

    pub struct AccessorSignature;
    pub struct Array;
    pub struct ArrayBuffer;
    pub struct ArrayBufferView;
    pub struct BigInt;
    pub struct BigInt64Array;
    pub struct BigIntObject;
    pub struct BigUint64Array;
    pub struct Boolean;
    pub struct BooleanObject;
    pub struct Context;
    pub struct DataView;
    pub struct Data;
    pub struct Date;
    pub struct DictionaryTemplate;
    pub struct Extension;
    pub struct External;
    pub struct FixedArray;
    pub struct Float32Array;
    pub struct Float64Array;
    pub struct Function;
    pub struct FunctionCallbackInfo<F> {
        _marker: std::marker::PhantomData<F>,
    }
    pub struct FunctionTemplate;
    pub struct Int16Array;
    pub struct Int32;
    pub struct Int32Array;
    pub struct Int8Array;
    pub struct Integer;
    pub struct Isolate;
    pub struct Map;
    pub struct Module;
    pub struct Name;
    pub struct Number;
    pub struct NumberObject;
    pub struct Object;
    pub struct ObjectTemplate;
    pub struct Platform;
    pub struct Primitive;
    pub struct Private;
    pub struct Promise;
    pub struct Proxy;
    pub struct RegExp;
    pub struct Script;
    pub struct Set;
    pub struct SharedArrayBuffer;
    pub struct Signature;
    pub struct String;
    pub struct StringObject;
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
    pub struct Value;
    pub struct WasmMemoryObject;
    pub struct WasmModuleObject;
}

mod v8_local_handle {
    pub struct Local<T> {
        _marker: std::marker::PhantomData<T>,
    }
}