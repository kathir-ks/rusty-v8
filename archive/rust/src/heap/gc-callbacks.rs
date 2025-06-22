// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod gc_callbacks {
    use std::vec::Vec;
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicBool, Ordering};

    // Dummy types and constants to satisfy the original C++ code's dependencies.
    // Replace with actual implementations if available.
    pub type Isolate = usize;
    pub type GCType = u32;
    pub type GCCallbackFlags = u32;

    #[macro_export]
    macro_rules! DCHECK_NOT_NULL {
        ($x:expr) => {
            assert!(!$x.is_null());
        };
    }

    #[macro_export]
    macro_rules! DCHECK_EQ {
        ($x:expr, $y:expr) => {
            assert_eq!($x, $y);
        };
    }

    #[macro_export]
    macro_rules! DCHECK_NE {
        ($x:expr, $y:expr) => {
            assert_ne!($x, $y);
        };
    }

    struct AllowGarbageCollectionScope {
        previous_state: bool,
    }

    static GC_ALLOWED: AtomicBool = AtomicBool::new(true);

    impl AllowGarbageCollectionScope {
        fn new() -> Self {
            let previous_state = GC_ALLOWED.load(Ordering::Relaxed);
            GC_ALLOWED.store(true, Ordering::Relaxed);
            AllowGarbageCollectionScope { previous_state }
        }
    }

    impl Drop for AllowGarbageCollectionScope {
        fn drop(&mut self) {
            GC_ALLOWED.store(self.previous_state, Ordering::Relaxed);
        }
    }
    
    struct DisallowGarbageCollectionScope {
      previous_state: bool,
    }
    
    impl DisallowGarbageCollectionScope {
        fn new() -> Self {
            let previous_state = GC_ALLOWED.load(Ordering::Relaxed);
            GC_ALLOWED.store(false, Ordering::Relaxed);
            DisallowGarbageCollectionScope { previous_state }
        }
    }
    
    impl Drop for DisallowGarbageCollectionScope {
        fn drop(&mut self) {
            GC_ALLOWED.store(self.previous_state, Ordering::Relaxed);
        }
    }

    pub struct GCCallbacks {
        callbacks_: Vec<CallbackData>,
    }

    impl GCCallbacks {
        pub type CallbackType = fn(isolate: *mut Isolate, gc_type: GCType, gc_callback_flags: GCCallbackFlags, data: *mut std::ffi::c_void);

        pub fn new() -> Self {
            GCCallbacks { callbacks_: Vec::new() }
        }

        pub fn add(&mut self, callback: Self::CallbackType, isolate: *mut Isolate, gc_type: GCType, data: *mut std::ffi::c_void) {
            DCHECK_NOT_NULL!(callback as *mut _);
            DCHECK_EQ!(self.find_callback(callback, data), None);
            self.callbacks_.push(CallbackData::new(callback, isolate, gc_type, data));
        }

        pub fn remove(&mut self, callback: Self::CallbackType, data: *mut std::ffi::c_void) {
            let it = self.find_callback(callback, data).expect("Callback not found");
            let last = self.callbacks_.pop().expect("Callbacks is empty");
            if it < self.callbacks_.len() {
                self.callbacks_[it] = last;
            }
        }

        pub fn invoke(&self, gc_type: GCType, gc_callback_flags: GCCallbackFlags) {
            let _scope = AllowGarbageCollectionScope::new();
            for callback_data in &self.callbacks_ {
                if gc_type & callback_data.gc_type != 0 {
                    (callback_data.callback)(
                        callback_data.isolate,
                        gc_type,
                        gc_callback_flags,
                        callback_data.user_data,
                    );
                }
            }
        }

        pub fn is_empty(&self) -> bool {
            self.callbacks_.is_empty()
        }

        fn find_callback(&self, callback: Self::CallbackType, data: *mut std::ffi::c_void) -> Option<usize> {
            self.callbacks_
                .iter()
                .enumerate()
                .find(|(_, callback_data)| {
                    callback_data.callback as usize == callback as usize && callback_data.user_data == data
                })
                .map(|(index, _)| index)
        }
    }

    struct CallbackData {
        callback: GCCallbacks::CallbackType,
        isolate: *mut Isolate,
        gc_type: GCType,
        user_data: *mut std::ffi::c_void,
    }

    impl CallbackData {
        fn new(callback: GCCallbacks::CallbackType, isolate: *mut Isolate, gc_type: GCType, user_data: *mut std::ffi::c_void) -> Self {
            CallbackData {
                callback,
                isolate,
                gc_type,
                user_data,
            }
        }
    }

    pub struct GCCallbacksInSafepoint {
        callbacks_: Vec<CallbackDataInSafepoint>,
    }

    impl GCCallbacksInSafepoint {
        pub type CallbackType = fn(data: *mut std::ffi::c_void);

        pub const K_LOCAL: u32 = 1 << 0;
        pub const K_SHARED: u32 = 1 << 1;
        pub const K_ALL: u32 = Self::K_LOCAL | Self::K_SHARED;

        pub fn new() -> Self {
            GCCallbacksInSafepoint { callbacks_: Vec::new() }
        }

        pub fn add(&mut self, callback: Self::CallbackType, data: *mut std::ffi::c_void, gc_type: u32) {
            DCHECK_NOT_NULL!(callback as *mut _);
            DCHECK_EQ!(self.find_callback(callback, data), None);
            self.callbacks_.push(CallbackDataInSafepoint::new(callback, data, gc_type));
        }

        pub fn remove(&mut self, callback: Self::CallbackType, data: *mut std::ffi::c_void) {
            let it = self.find_callback(callback, data).expect("Callback not found");
            let last = self.callbacks_.pop().expect("Callbacks is empty");
            if it < self.callbacks_.len() {
                self.callbacks_[it] = last;
            }
        }

        pub fn invoke(&self, gc_type: u32) {
          let _scope = DisallowGarbageCollectionScope::new();
            for callback_data in &self.callbacks_ {
                if callback_data.gc_type_ & gc_type != 0 {
                    (callback_data.callback)(callback_data.user_data);
                }
            }
        }

        pub fn is_empty(&self) -> bool {
            self.callbacks_.is_empty()
        }

        fn find_callback(&self, callback: Self::CallbackType, data: *mut std::ffi::c_void) -> Option<usize> {
            self.callbacks_
                .iter()
                .enumerate()
                .find(|(_, callback_data)| {
                    callback_data.callback as usize == callback as usize && callback_data.user_data == data
                })
                .map(|(index, _)| index)
        }
    }

    struct CallbackDataInSafepoint {
        callback: GCCallbacksInSafepoint::CallbackType,
        user_data: *mut std::ffi::c_void,
        gc_type_: u32,
    }

    impl CallbackDataInSafepoint {
        fn new(callback: GCCallbacksInSafepoint::CallbackType, user_data: *mut std::ffi::c_void, gc_type_: u32) -> Self {
            CallbackDataInSafepoint {
                callback,
                user_data,
                gc_type_,
            }
        }
    }
}