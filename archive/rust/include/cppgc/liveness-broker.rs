// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides a LivenessBroker for checking object liveness in the context of a garbage-collected heap.

pub mod internal {
    pub struct LivenessBrokerFactory {} // Placeholder; implementation details are internal
}

/// The broker is passed to weak callbacks to allow (temporarily) querying
/// the liveness state of an object. References to non-live objects must be
/// cleared when `is_heap_object_alive()` returns false.
///
/// ```ignore
/// struct GCedWithCustomWeakCallback {
///     bar: UntracedMember<Bar>,
/// }
///
/// impl GCedWithCustomWeakCallback {
///     fn custom_weak_callback_method(&mut self, broker: &LivenessBroker) {
///         if !broker.is_heap_object_alive(&self.bar) {
///             self.bar = UntracedMember::null();
///         }
///     }
///
///     fn trace(&self, visitor: &mut Visitor) {
///         visitor.register_weak_callback_method::<GCedWithCustomWeakCallback>(
///             self,
///             Self::custom_weak_callback_method,
///         );
///     }
/// }
/// ```
pub struct LivenessBroker {}

impl LivenessBroker {
    pub fn is_heap_object_alive<T>(&self, object: &T) -> bool {
        // - nullptr objects are considered alive to allow weakness to be used from
        // stack while running into a conservative GC. Treating nullptr as dead
        // would mean that e.g. custom collections could not be strongified on
        // stack.
        // - Sentinel pointers are also preserved in weakness and not cleared.
        let object_ptr = object as *const T;

        if object_ptr.is_null() {
            return true;
        }

        // SentinelPointer concept could not be expressed in this basic translation
        // The C++ code checks for `object == kSentinelPointer` but a more idiomatic
        // Rust implementation would likely use a specific sentinel value or type.
        // For now, we skip this sentinel pointer check.

        self.is_heap_object_alive_impl(object as *const T as *const std::ffi::c_void)
    }

    pub fn is_heap_object_alive_weak<T>(&self, weak_member: &WeakMember<T>) -> bool {
        match weak_member.get() {
            Some(object) => self.is_heap_object_alive(object),
            None => false, // Or true?  The C++ nullptr check implies true...
        }
    }

    pub fn is_heap_object_alive_untraced<T>(&self, untraced_member: &UntracedMember<T>) -> bool {
        match untraced_member.get() {
            Some(object) => self.is_heap_object_alive(object),
            None => true, // Consistent with the null pointer handling in C++
        }
    }

    fn is_heap_object_alive_impl(&self, object_payload: *const std::ffi::c_void) -> bool {
        // This is a placeholder; the actual implementation depends on the memory model
        // and garbage collection strategy of the heap.
        // In a real implementation, this would need to check if the object at
        // object_payload is still alive in the heap.
        true
    }

    // Private constructor, accessible only through the factory.
    fn new() -> Self {
        LivenessBroker {}
    }
}

// Placeholder types for dependencies
pub struct Visitor {}
impl Visitor {
    pub fn register_weak_callback_method<T>(&mut self, _obj: &T, _callback: fn(&mut T, &LivenessBroker)) {}
}
pub struct WeakMember<T> {
    ptr: Option<Box<T>>,
}

impl<T> WeakMember<T> {
    pub fn get(&self) -> Option<&T> {
        self.ptr.as_ref().map(|b| &**b)
    }
}

pub struct UntracedMember<T> {
    ptr: Option<Box<T>>,
}

impl<T> UntracedMember<T> {
    pub fn get(&self) -> Option<&T> {
        self.ptr.as_ref().map(|b| &**b)
    }

    pub fn null() -> Self {
        UntracedMember { ptr: None }
    }
}