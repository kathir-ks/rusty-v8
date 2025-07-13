// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-atomics.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::sync::Mutex;

pub struct Isolate {}

pub struct JSTypedArray {}

impl JSTypedArray {
    pub fn GetBuffer(&self) -> &JSArrayBuffer {
        &JSArrayBuffer {}
    }
    pub fn byte_offset(&self) -> usize {
        0
    }
    pub fn type_(&self) -> i32 {
        0
    }
    pub fn GetLength(&self) -> usize {
        0
    }
    pub fn GetLengthOrOutOfBounds(&self, _out_of_bounds: bool) -> usize {
        0
    }
    pub fn WasDetached(&self) -> bool {
        false
    }
    pub fn IsDetachedOrOutOfBounds(&self) -> bool {
        false
    }
}

pub struct JSArrayBuffer {}

impl JSArrayBuffer {
    pub fn backing_store(&self) -> *mut std::ffi::c_void {
        std::ptr::null_mut()
    }
}

pub struct Object {}

impl Object {
    pub fn ToInteger(_isolate: *mut Isolate, _obj: Handle<Object>) -> Result<Handle<Object>, String> {
        Ok(Handle::new(Object {}))
    }
    pub fn ToName(_isolate: *mut Isolate, _obj: Handle<Object>) -> Result<Handle<Name>, String> {
        Ok(Handle::new(Name {}))
    }
    pub fn Share(_isolate: *mut Isolate, _obj: Handle<Object>, _throw_on_error: i32) -> Result<Handle<Object>, String> {
        Ok(Handle::new(Object {}))
    }
    pub fn WriteToReadOnlyProperty(_it: &LookupIterator, _value: Handle<Object>, _throw_on_error: i32) -> Maybe<bool> {
        Maybe::Nothing
    }
    pub fn AddDataProperty(_it: &LookupIterator, _value: Handle<Object>, _attributes: i32, _throw_on_error: Just<i32>, _store_origin: i32) -> Maybe<bool> {
        Maybe::Nothing
    }
}

pub struct Name {}

pub struct BigInt {}

impl BigInt {
    pub fn FromObject(_isolate: *mut Isolate, _obj: Handle<Object>) -> Result<Handle<BigInt>, String> {
        Ok(Handle::new(BigInt {}))
    }
    pub fn FromInt64(_isolate: *mut Isolate, _t: i64) -> *mut Object {
        &mut Object {}
    }
    pub fn FromUint64(_isolate: *mut Isolate, _t: u64) -> *mut Object {
        &mut Object {}
    }
    pub fn AsInt64(&self) -> i64 {
        0
    }
    pub fn AsUint64(&self) -> u64 {
        0
    }
}

pub struct Smi {}

impl Smi {
    pub fn FromInt(_t: i32) -> Tagged<Object> {
        Tagged {dummy: 0}
    }
}

pub struct Factory {}

impl Factory {
    pub fn NewNumber(&self, _t: i32) -> *mut Object {
        &mut Object {}
    }
    pub fn NewStringFromAsciiChecked(&self, _str: &str) -> *mut Object {
        &mut Object {}
    }
}

pub struct ReadOnlyRoots {}

impl ReadOnlyRoots {
    pub fn undefined_value(&self) -> Tagged<Object> {
        Tagged{dummy: 0}
    }
    pub fn exception(&self) -> Tagged<Object> {
        Tagged{dummy: 0}
    }
}

pub struct Isolate_ {
    factory: Factory,
    read_only_roots: ReadOnlyRoots,
}

impl Isolate_ {
    pub fn factory(&mut self) -> &mut Factory {
        &mut self.factory
    }
    pub fn read_only_roots(&self) -> &ReadOnlyRoots {
        &self.read_only_roots
    }
}

pub struct HandleScope {}

impl HandleScope {
    pub fn new(_isolate: *mut Isolate) -> Self {
        HandleScope {}
    }
}

pub struct Handle<T> {
    _value: T,
}

impl<T> Handle<T> {
    pub fn new(_value: T) -> Self {
        Handle { _value }
    }
}

pub struct DirectHandle<T> {
   _value: T,
}

impl<T> DirectHandle<T> {
    pub fn new(_value: T) -> Self {
        DirectHandle { _value }
    }
}

pub struct RuntimeArguments {}

impl RuntimeArguments {
    pub fn length(&self) -> i32 {
        0
    }
    pub fn at<T>(&self, _index: usize) -> Handle<T> {
        Handle::new(unsafe { std::mem::zeroed() })
    }
    pub fn at_<T>(&self, _index: usize) -> DirectHandle<T> {
        DirectHandle::new(unsafe { std::mem::zeroed() })
    }
    pub fn get(&self, _index: usize) -> Handle<Object> {
        Handle::new(Object {})
    }
}

pub struct Tagged<T> {
    dummy: i32
}

pub struct LookupIterator {}
impl LookupIterator {
    pub fn new(_isolate: *mut Isolate, _object: DirectHandle<JSObject>, _property_key: PropertyKey, _lookup_owner: i32) -> Self {
        LookupIterator{}
    }
    pub fn IsFound(&self) -> bool {
        false
    }
    pub fn IsReadOnly(&self) -> bool {
        false
    }
    pub fn GetDataValue(&self, _kseqcst_access: i32) -> *mut Object {
        &mut Object{}
    }
    pub fn WriteDataValue(&self, _shared_value: Handle<Object>, _kseqcst_access: i32) {}
    pub fn SwapDataValue(&self, _shared_value: Handle<Object>, _kseqcst_access: i32) -> *mut Object {
        &mut Object{}
    }
    pub fn CompareAndSwapDataValue(&self, _shared_expected: Handle<Object>, _shared_value: Handle<Object>, _kseqcst_access: i32) -> *mut Object {
        &mut Object{}
    }
}

pub struct PropertyKey {}
impl PropertyKey {
    pub fn new(_isolate: *mut Isolate, _field_name: Handle<Name>) -> Self {
        PropertyKey{}
    }
}

#[derive(Debug)]
pub enum Error {
    GenericError,
}

pub enum Maybe<T> {
    Just(T),
    Nothing,
}

pub struct Just<T>(T);

macro_rules! ASSIGN_RETURN_FAILURE_ON_EXCEPTION {
    ($isolate:expr, $var:ident, $call:expr) => {
        match $call {
            Ok(val) => {
                $var = val;
            }
            Err(_err) => {
                return ReadOnlyRoots{}.exception();
            }
        }
    };
}

macro_rules! CHECK_LT {
    ($left:expr, $right:expr) => {
        if $left >= $right {
            panic!("CHECK_LT failed: {} >= {}", $left, $right);
        }
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

macro_rules! THROW_ERROR_RETURN_FAILURE_ON_DETACHED_OR_OUT_OF_BOUNDS {
    ($isolate:expr, $sta:expr, $index:expr, $method_name:expr) => {
        let mut out_of_bounds = false;
        let length = $sta.GetLengthOrOutOfBounds(out_of_bounds);
        if $sta.WasDetached() || out_of_bounds || $index >= length {
            return ReadOnlyRoots{}.exception();
        }
    };
}

const NONE: i32 = 0;
const kThrowOnError: i32 = 1;
const kSeqCstAccess: i32 = 0;
const kMaybeKeyed: i32 = 0;

#[no_mangle]
pub extern "C" fn Runtime_AtomicsLoad64(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let scope = HandleScope::new(_isolate);
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    assert_eq!(2, args.length());
    let sta: Handle<JSTypedArray> = args.at(0);
    let index = 0;

    let source = sta.GetBuffer().backing_store() as *mut u8;

    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsStore64(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let scope = HandleScope::new(_isolate);
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    assert_eq!(3, args.length());
    let sta: Handle<JSTypedArray> = args.at(0);
    let index = 0;
    let value_obj: Handle<Object> = args.at(2);

    let source = sta.GetBuffer().backing_store() as *mut u8;

    let bigint: Handle<BigInt> = Handle::new(BigInt{});

    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsExchange(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    let result = GetModifySetValueInBuffer::<Exchange>(args, _isolate, "Atomics.exchange");
    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsCompareExchange(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let scope = HandleScope::new(_isolate);
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    assert_eq!(4, args.length());
    let sta: Handle<JSTypedArray> = args.at(0);
    let index = 0;
    let old_value_obj: Handle<Object> = args.at(2);
    let new_value_obj: Handle<Object> = args.at(3);

    let source = sta.GetBuffer().backing_store() as *mut u8;

    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsAdd(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    let result = GetModifySetValueInBuffer::<Add>(args, _isolate, "Atomics.add");
    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsSub(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
     let result = GetModifySetValueInBuffer::<Sub>(args, _isolate, "Atomics.sub");
    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsAnd(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    let result = GetModifySetValueInBuffer::<And>(args, _isolate, "Atomics.and");
    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsOr(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
     let result = GetModifySetValueInBuffer::<Or>(args, _isolate, "Atomics.or");
    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsXor(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    let result = GetModifySetValueInBuffer::<Xor>(args, _isolate, "Atomics.xor");
    return &mut Object {};
}

fn GetModifySetValueInBuffer<Op>(args: &RuntimeArguments, isolate: *mut Isolate_, method_name: &str) -> Tagged<Object> {
    let isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let scope = HandleScope::new(isolate);
    assert_eq!(3, args.length());
    let sta: Handle<JSTypedArray> = args.at(0);
    let index = 0;
    let value_obj: Handle<Object> = args.at(2);

    let source = sta.GetBuffer().backing_store() as *mut u8;

    if sta.type_() >= 0 {
        let bigint: Handle<BigInt> = Handle::new(BigInt{});

        THROW_ERROR_RETURN_FAILURE_ON_DETACHED_OR_OUT_OF_BOUNDS!(isolate, sta, index, method_name);

        CHECK_LT!(index, sta.GetLength());
        return Tagged{dummy: 0};
    }

    let value: Handle<Object> = Handle::new(Object{});

    THROW_ERROR_RETURN_FAILURE_ON_DETACHED_OR_OUT_OF_BOUNDS!(isolate, sta, index, method_name);

    CHECK_LT!(index, sta.GetLength());

    UNREACHABLE!();
}

struct Exchange {}
struct Add {}
struct Sub {}
struct And {}
struct Or {}
struct Xor {}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsLoadSharedStructOrArray(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let scope = HandleScope::new(_isolate);
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    assert_eq!(2, args.length());
    let shared_struct_or_shared_array: DirectHandle<JSObject> = args.at_(0);
    let field_name: Handle<Name> = Handle::new(Name{});
    // Shared structs are prototypeless.
    let it = LookupIterator::new(_isolate, shared_struct_or_shared_array, PropertyKey{}, LookupIterator::OWN);
    if it.IsFound() { return it.GetDataValue(kSeqCstAccess); }
    return ReadOnlyRoots{}.undefined_value();
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsStoreSharedStructOrArray(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let scope = HandleScope::new(_isolate);
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    assert_eq!(3, args.length());
    let shared_struct_or_shared_array: DirectHandle<JSObject> = args.at_(0);
    let field_name: Handle<Name> = Handle::new(Name{});
    let shared_value: Handle<Object> = Handle::new(Object{});

    let result = AtomicFieldWrite(_isolate, shared_struct_or_shared_array, field_name,
        shared_value, |it| {
            it.WriteDataValue(shared_value, kSeqCstAccess);
            return *shared_value;
        });

    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsExchangeSharedStructOrArray(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let scope = HandleScope::new(_isolate);
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    assert_eq!(3, args.length());
    let shared_struct_or_shared_array: DirectHandle<JSObject> = args.at_(0);
    let field_name: Handle<Name> = Handle::new(Name{});
    let shared_value: Handle<Object> = Handle::new(Object{});

    let result = AtomicFieldWrite(_isolate, shared_struct_or_shared_array, field_name,
        shared_value, |it| {
            return *it.SwapDataValue(shared_value, kSeqCstAccess);
        });

    return &mut Object {};
}

#[no_mangle]
pub extern "C" fn Runtime_AtomicsCompareExchangeSharedStructOrArray(_args_length: i32, _args_object: *mut std::ffi::c_void, _isolate: *mut Isolate) -> *mut Object {
    let _isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let scope = HandleScope::new(_isolate);
    let args = unsafe { &*( _args_object as *mut RuntimeArguments) };
    assert_eq!(4, args.length());
    let shared_struct_or_shared_array: DirectHandle<JSObject> = args.at_(0);
    let field_name: Handle<Name> = Handle::new(Name{});
    let shared_expected: Handle<Object> = Handle::new(Object{});
    let shared_value: Handle<Object> = Handle::new(Object{});

    let result = AtomicFieldWrite(_isolate, shared_struct_or_shared_array, field_name,
        shared_value, |it| {
            return *it.CompareAndSwapDataValue(
                shared_expected, shared_value, kSeqCstAccess);
        });

    return &mut Object {};
}

fn AtomicFieldWrite<WriteOperation>(isolate: *mut Isolate_, object: DirectHandle<JSObject>,
                                    field_name: Handle<Name>,
                                    value: DirectHandle<Object>,
                                    write_operation: WriteOperation) -> Tagged<Object>
    where WriteOperation: Fn(LookupIterator) -> *mut Object {
    let isolate = unsafe { &mut *(_isolate as *mut Isolate_) };
    let it = LookupIterator::new(isolate, object, PropertyKey{}, LookupIterator::OWN);
    let mut result: Maybe<bool> = Maybe::Nothing;
    if it.IsFound() {
        if !it.IsReadOnly() {
            return Tagged{dummy: 0};
        }
        // Shared structs and arrays are non-extensible and have non-configurable,
        // writable, enumerable properties. The only exception is SharedArrays'
        // "length" property, which is non-writable.
        result = Object::WriteToReadOnlyProperty(&it, Handle::new(value._value), Just(kThrowOnError));
    } else {
        // Shared structs are non-extensible. Instead of duplicating logic, call
        // Object::AddDataProperty to handle the error case.
        result = Object::AddDataProperty(&it, Handle::new(value._value), NONE, Just(kThrowOnError),
                                            kMaybeKeyed);
    }
    // Treat as strict code and always throw an error.
    USE(result);
    return ReadOnlyRoots{}.exception();
}

macro_rules! USE {
    ($x:expr) => {
        let _ = $x;
    };
}

pub struct JSObject {}
