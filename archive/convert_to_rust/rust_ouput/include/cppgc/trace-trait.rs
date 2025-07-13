// Converted from V8 C++ source files:
// Header: trace-trait.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct Visitor {}

pub mod internal {
    use super::Visitor;

    pub struct RootVisitor {}

    pub type TraceRootCallback = fn(_: &mut RootVisitor, object: *const std::ffi::c_void);

    pub struct TraceTraitImpl<T, const B: bool> {
        _phantom: std::marker::PhantomData<T>,
    }
}

pub type TraceCallback = fn(_: *mut Visitor, object: *const std::ffi::c_void);

#[derive(Clone, Copy)]
pub struct TraceDescriptor {
    pub base_object_payload: *const std::ffi::c_void,
    pub callback: TraceCallback,
}

pub type TraceDescriptorCallback = fn(address: *const std::ffi::c_void) -> TraceDescriptor;

pub mod internal {
    use super::{TraceDescriptor, TraceCallback, Visitor};

    pub struct V8_EXPORT {}

    pub struct TraceTraitFromInnerAddressImpl {}

    impl TraceTraitFromInnerAddressImpl {
        pub fn get_trace_descriptor(address: *const std::ffi::c_void) -> TraceDescriptor {
            TraceDescriptor {
                base_object_payload: address,
                callback: Self::dummy_trace_callback,
            }
        }

        fn dummy_trace_callback(_: *mut Visitor, object: *const std::ffi::c_void) {}
    }

    pub trait IsTraceable {
        fn trace(&self, visitor: *mut Visitor);
    }

    pub struct TraceTraitBase<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> TraceTraitBase<T>
    where
        T: IsTraceable,
    {
        pub fn get_trace_descriptor(self_: *const T) -> TraceDescriptor {
            TraceTraitImpl::<T, false>::get_trace_descriptor(self_)
        }

        pub fn trace(visitor: *mut Visitor, self_: *const std::ffi::c_void) {
            let obj: &T = unsafe { &*(self_ as *const T) };
            obj.trace(visitor);
        }
    }
}

pub struct TraceTrait<T> {
    _phantom: std::marker::PhantomData<T>,
}

pub mod internal {
    use super::{
        internal::{TraceTraitFromInnerAddressImpl, TraceTraitBase},
        TraceCallback,
        TraceDescriptor,
    };

    impl<T> TraceTraitImpl<T, false> {
        pub fn get_trace_descriptor(self_: *const T) -> TraceDescriptor {
            TraceDescriptor {
                base_object_payload: self_ as *const std::ffi::c_void,
                callback: TraceTraitBase::<T>::trace,
            }
        }
    }

    impl<T> TraceTraitImpl<T, true> {
        pub fn get_trace_descriptor(self_: *const T) -> TraceDescriptor {
            TraceTraitFromInnerAddressImpl::get_trace_descriptor(self_ as *const std::ffi::c_void)
        }
    }
}
