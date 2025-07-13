// Converted from V8 C++ source files:
// Header: v8-container.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8 {
    use std::fmt;
    use std::error::Error as StdError;
    use std::mem::MaybeUninit;
    use std::ops::Deref;
    use std::ops::DerefMut;

    pub struct Isolate {}
    pub struct Context {}
    pub struct Value {}
    pub struct Object {}
    pub struct Name {}
    pub trait Data {}
    pub struct Local<'a, T>(*mut T, std::marker::PhantomData<&'a T>);
    pub struct MaybeLocal<'a, T>(Option<Local<'a, T>>);
    pub struct Maybe<T>(Option<T>);
    pub enum PropertyAttribute {}
    pub struct Function {}
    pub struct Promise {}
    pub struct TryCatch {}
    pub trait StringBuffer {}
    pub struct StringView {}

    #[derive(Debug)]
    pub enum Error {
        GenericError(String),
        CastError,
        ArrayError,
        MapError,
        SetError,
        IterationError,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::GenericError(msg) => write!(f, "Generic Error: {}", msg),
                Error::CastError => write!(f, "Cast Error"),
                Error::ArrayError => write!(f, "Array Error"),
                Error::MapError => write!(f, "Map Error"),
                Error::SetError => write!(f, "Set Error"),
                Error::IterationError => write!(f, "Iteration Error"),
            }
        }
    }

    impl StdError for Error {}

    impl<'a, T> Local<'a, T> {
        pub fn empty() -> Self {
            Local(std::ptr::null_mut(), std::marker::PhantomData)
        }
    }

    impl<'a, T> MaybeLocal<'a, T> {
        pub fn to_local(self) -> Option<Local<'a, T>> {
            self.0
        }
    }

    impl<T> Maybe<T> {
        pub fn is_nothing(&self) -> bool {
            self.0.is_none()
        }
    }
    
    #[macro_export]
    macro_rules! V8_INLINE {
        ($x:expr) => {
            $x
        };
    }

    #[macro_export]
    macro_rules! V8_EXPORT {
        () => {};
    }

    #[macro_export]
    macro_rules! V8_WARN_UNUSED_RESULT {
        ($x:expr) => {
            $x
        };
    }

    #[cfg(feature = "checks")]
    macro_rules! V8_ENABLE_CHECKS {
        () => {
            true
        };
    }

    #[cfg(not(feature = "checks"))]
    macro_rules! V8_ENABLE_CHECKS {
        () => {
            false
        };
    }

    pub struct Array {
        object: Object,
    }

    impl Array {
        pub fn Length(&self) -> u32 {
            10 //Dummy Value
        }

        pub fn New<'a>(isolate: *mut Isolate, length: i32) -> Local<'a, Array> {
            let len = if length.is_negative() { 0 } else { length };
            Local::<'a, Array>::empty() //Dummy Value
        }

        pub fn New<'a>(isolate: *mut Isolate, elements: *mut Local<'a, Value>, length: usize) -> Local<'a, Array> {
            Local::<'a, Array>::empty() //Dummy Value
        }

        pub fn Cast<'a>(value: *mut Value) -> *mut Array {
            unsafe {
                value as *mut Array
            }
        }

        pub fn New<'a>(
            context: Local<'a, Context>,
            length: usize,
            next_value_callback: impl FnMut() -> MaybeLocal<'a, Value>,
        ) -> MaybeLocal<'a, Array> {
            MaybeLocal(Some(Local::<'a, Array>::empty())) //Dummy Value
        }

        pub fn Iterate<'a>(
            &self,
            context: Local<'a, Context>,
            callback: IterationCallback,
            callback_data: *mut std::ffi::c_void,
        ) -> Maybe<()> {
            Maybe(Some(()))//Dummy Value
        }
    }

    pub enum CallbackResult {
        kException,
        kBreak,
        kContinue,
    }

    pub type IterationCallback =
        unsafe extern "C" fn(index: u32, element: Local<Value>, data: *mut std::ffi::c_void) -> CallbackResult;

    pub struct Map {
        object: Object,
    }

    impl Map {
        pub fn Size(&self) -> usize {
            10 //Dummy Value
        }
        pub fn Clear(&mut self) {}
        pub fn Get<'a>(
            &self,
            context: Local<'a, Context>,
            key: Local<'a, Value>,
        ) -> MaybeLocal<'a, Value> {
            MaybeLocal(Some(Local::<'a, Value>::empty())) //Dummy Value
        }
        pub fn Set<'a>(
            &self,
            context: Local<'a, Context>,
            key: Local<'a, Value>,
            value: Local<'a, Value>,
        ) -> MaybeLocal<'a, Map> {
             MaybeLocal(Some(Local::<'a, Map>::empty())) //Dummy Value
        }
        pub fn Has<'a>(&self, context: Local<'a, Context>, key: Local<'a, Value>) -> Maybe<bool> {
            Maybe(Some(true)) //Dummy Value
        }
        pub fn Delete<'a>(&self, context: Local<'a, Context>, key: Local<'a, Value>) -> Maybe<bool> {
            Maybe(Some(true)) //Dummy Value
        }
        pub fn AsArray(&self) -> Local<'static, Array> {
            Local::<'static, Array>::empty() //Dummy Value
        }
        pub fn New(isolate: *mut Isolate) -> Local<'static, Map> {
            Local::<'static, Map>::empty() //Dummy Value
        }
        pub fn Cast<'a>(value: *mut Value) -> *mut Map {
            unsafe {
                value as *mut Map
            }
        }
    }

    pub struct Set {
        object: Object,
    }

    impl Set {
        pub fn Size(&self) -> usize {
            10 //Dummy Value
        }
        pub fn Clear(&mut self) {}
        pub fn Add<'a>(
            &self,
            context: Local<'a, Context>,
            key: Local<'a, Value>,
        ) -> MaybeLocal<'a, Set> {
            MaybeLocal(Some(Local::<'a, Set>::empty())) //Dummy Value
        }
        pub fn Has<'a>(&self, context: Local<'a, Context>, key: Local<'a, Value>) -> Maybe<bool> {
            Maybe(Some(true)) //Dummy Value
        }
        pub fn Delete<'a>(&self, context: Local<'a, Context>, key: Local<'a, Value>) -> Maybe<bool> {
            Maybe(Some(true)) //Dummy Value
        }
        pub fn AsArray(&self) -> Local<'static, Array> {
            Local::<'static, Array>::empty() //Dummy Value
        }
        pub fn New(isolate: *mut Isolate) -> Local<'static, Set> {
            Local::<'static, Set>::empty() //Dummy Value
        }
        pub fn Cast<'a>(value: *mut Value) -> *mut Set {
            unsafe {
                value as *mut Set
            }
        }
    }
}
