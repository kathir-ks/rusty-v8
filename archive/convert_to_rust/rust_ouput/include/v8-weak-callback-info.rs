// Converted from V8 C++ source files:
// Header: v8-weak-callback-info.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {
pub struct ConditionalStackAllocatedBase<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ConditionalStackAllocatedBase<T> {
    pub fn new() -> Self {
        ConditionalStackAllocatedBase {
            _phantom: std::marker::PhantomData,
        }
    }
}
}
}
pub struct Isolate;

pub mod api_internal {
    pub fn InternalFieldOutOfBounds(index: i32) {
        eprintln!("Internal field out of bounds at index: {}", index);
        panic!("Internal field out of bounds");
    }
}

pub const K_INTERNAL_FIELDS_IN_WEAK_CALLBACK: i32 = 2;
pub const K_EMBEDDER_FIELDS_IN_WEAK_CALLBACK: i32 = 2;

pub struct WeakCallbackInfo<'a, T> {
    isolate_: &'a Isolate,
    parameter_: *mut T,
    callback_: *mut WeakCallbackInfoCallback<'a, T>,
    embedder_fields_: [*mut std::ffi::c_void; K_EMBEDDER_FIELDS_IN_WEAK_CALLBACK as usize],
    _phantom: std::marker::PhantomData<&'a T>,
}

type WeakCallbackInfoCallback<'a, T> = dyn Fn(&WeakCallbackInfo<'a, T>);

impl<'a, T> WeakCallbackInfo<'a, T> {
    pub fn new(
        isolate: &'a Isolate,
        parameter: *mut T,
        embedder_fields: [*mut std::ffi::c_void; K_EMBEDDER_FIELDS_IN_WEAK_CALLBACK as usize],
        callback: *mut WeakCallbackInfoCallback<'a, T>,
    ) -> Self {
        WeakCallbackInfo {
            isolate_: isolate,
            parameter_: parameter,
            callback_: callback,
            embedder_fields_: embedder_fields,
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn GetIsolate(&self) -> &Isolate {
        self.isolate_
    }

    #[inline]
    pub fn GetParameter(&self) -> *mut T {
        self.parameter_
    }

    #[inline]
    pub fn GetInternalField(&self, index: i32) -> *mut std::ffi::c_void {
        if index < 0 || index >= K_EMBEDDER_FIELDS_IN_WEAK_CALLBACK {
            api_internal::InternalFieldOutOfBounds(index);
        }
        self.embedder_fields_[index as usize]
    }

    pub fn SetSecondPassCallback(&self, callback: Box<WeakCallbackInfoCallback<'a, T>>)  {
        unsafe {
            *self.callback_ = *Box::leak(callback);
        }
    }
}

#[derive(Clone, Copy)]
pub enum WeakCallbackType {
    KParameter,
    KInternalFields,
}
