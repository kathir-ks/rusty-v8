// Converted from V8 C++ source files:
// Header: gc-callbacks.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
    pub struct Isolate {}
}

mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! DCHECK_NOT_NULL {
            ($arg:expr) => {
                if $arg.is_null() {
                    panic!("DCHECK_NOT_NULL failed: argument is null");
                }
            };
        }
    }
}

mod common {
    pub mod assert_scope {
        pub struct AssertScope {}
    }
}

use std::vec::Vec;
use std::ptr::null_mut;
use std::mem::ManuallyDrop;

pub mod internal {
    use super::v8;
    use super::GCType;
    use super::GCCallbackFlags;
    use super::AllowGarbageCollection;
    use super::DisallowGarbageCollection;
    use super::base::logging::DCHECK_NOT_NULL;

    pub struct GCCallbacks {
        callbacks_: Vec<CallbackData>,
    }

    impl GCCallbacks {
        pub fn new() -> Self {
            GCCallbacks { callbacks_: Vec::new() }
        }

        pub type CallbackType = fn(*mut v8::Isolate, GCType, GCCallbackFlags, *mut std::ffi::c_void);

        pub fn add(&mut self, callback: GCCallbacks::CallbackType, isolate: *mut v8::Isolate, gc_type: GCType, data: *mut std::ffi::c_void) {
            DCHECK_NOT_NULL!(callback as *mut std::ffi::c_void);
            if self.find_callback(callback, data).is_some() {
                panic!("Callback already exists");
            }
            self.callbacks_.push(CallbackData {
                callback,
                isolate,
                gc_type,
                user_data: data,
            });
        }

        pub fn remove(&mut self, callback: GCCallbacks::CallbackType, data: *mut std::ffi::c_void) {
            let it = self.find_callback(callback, data);
            match it {
                Some(index) => {
                    if index < self.callbacks_.len() - 1 {
                        self.callbacks_.swap(index, self.callbacks_.len() - 1);
                    }
                    self.callbacks_.pop();
                }
                None => {
                    panic!("Callback not found");
                }
            }
        }

        pub fn invoke(&self, gc_type: GCType, gc_callback_flags: GCCallbackFlags) {
            let _scope = AllowGarbageCollection {};
            for callback_data in &self.callbacks_ {
                if (gc_type as u32 & callback_data.gc_type as u32) != 0 {
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

        fn find_callback(
            &self,
            callback: GCCallbacks::CallbackType,
            data: *mut std::ffi::c_void,
        ) -> Option<usize> {
            self.callbacks_
                .iter()
                .position(|callback_data| {
                    callback_data.callback as *mut std::ffi::c_void == callback as *mut std::ffi::c_void
                        && callback_data.user_data == data
                })
        }
    }

    struct CallbackData {
        callback: GCCallbacks::CallbackType,
        isolate: *mut v8::Isolate,
        gc_type: GCType,
        user_data: *mut std::ffi::c_void,
    }

    pub struct GCCallbacksInSafepoint {
        callbacks_: Vec<CallbackDataInSafepoint>,
    }

    impl GCCallbacksInSafepoint {
        pub fn new() -> Self {
            GCCallbacksInSafepoint { callbacks_: Vec::new() }
        }

        pub type CallbackType = fn(*mut std::ffi::c_void);

        pub enum GCType {
            kLocal = 1 << 0,
            kShared = 1 << 1,
            kAll = Self::kLocal as isize | Self::kShared as isize,
        }

        pub fn add(&mut self, callback: GCCallbacksInSafepoint::CallbackType, data: *mut std::ffi::c_void, gc_type: GCType) {
            DCHECK_NOT_NULL!(callback as *mut std::ffi::c_void);
            if self.find_callback(callback, data).is_some() {
                panic!("Callback already exists");
            }
            self.callbacks_.push(CallbackDataInSafepoint {
                callback,
                user_data: data,
                gc_type_: gc_type as isize,
            });
        }

        pub fn remove(&mut self, callback: GCCallbacksInSafepoint::CallbackType, data: *mut std::ffi::c_void) {
            let it = self.find_callback(callback, data);
            match it {
                Some(index) => {
                    if index < self.callbacks_.len() - 1 {
                        self.callbacks_.swap(index, self.callbacks_.len() - 1);
                    }
                    self.callbacks_.pop();
                }
                None => {
                    panic!("Callback not found");
                }
            }
        }

        pub fn invoke(&self, gc_type: GCType) const {
            let _scope = DisallowGarbageCollection {};
            for callback_data in &self.callbacks_ {
                if (callback_data.gc_type_ & gc_type as isize) != 0 {
                    (callback_data.callback)(callback_data.user_data);
                }
            }
        }

        pub fn is_empty(&self) -> bool {
            self.callbacks_.is_empty()
        }

        fn find_callback(
            &self,
            callback: GCCallbacksInSafepoint::CallbackType,
            data: *mut std::ffi::c_void,
        ) -> Option<usize> {
            self.callbacks_
                .iter()
                .position(|callback_data| {
                    callback_data.callback as *mut std::ffi::c_void == callback as *mut std::ffi::c_void
                        && callback_data.user_data == data
                })
        }
    }

    struct CallbackDataInSafepoint {
        callback: GCCallbacksInSafepoint::CallbackType,
        user_data: *mut std::ffi::c_void,
        gc_type_: isize,
    }
}
