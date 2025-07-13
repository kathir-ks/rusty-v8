// Converted from V8 C++ source files:
// Header: garbage-collected.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    pub fn Fatal(message: &str) {
        panic!("{}", message);
    }
}
pub mod platform {}
pub mod trace_trait {}
pub mod type_traits {}

pub struct Visitor {}

pub struct GarbageCollected<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> GarbageCollected<T> {
    pub type IsGarbageCollectedTypeMarker = void;
    pub type ParentMostGarbageCollectedType = T;

    // Must use MakeGarbageCollected.
    // In Rust, we don't have direct equivalent of `operator new` and `delete`.
    // The allocation is handled by Box::new.
    // The deallocation is handled automatically by Rust's RAII.

    protected_fn!();

    pub fn new() -> Self {
        GarbageCollected {
            _phantom: std::marker::PhantomData,
        }
    }
}

macro_rules! protected_fn {
    () => {
        // Simulate protected access by making the constructor available only within the same module
        // or in child modules.
        #[allow(dead_code)]
        pub(crate) fn protected_constructor() -> Self {
            GarbageCollected {
                _phantom: std::marker::PhantomData,
            }
        }
    };
}

pub struct GarbageCollectedMixin {}

impl GarbageCollectedMixin {
    pub type IsGarbageCollectedMixinTypeMarker = void;

    // Must use MakeGarbageCollected.
    // Same as above, allocation is handled implicitly, and deallocation by RAII.

    /**
     * This Trace method must be overriden by objects inheriting from
     * GarbageCollectedMixin.
     */
    pub fn trace(&self, _visitor: &mut Visitor) {}
}

pub enum void {}
