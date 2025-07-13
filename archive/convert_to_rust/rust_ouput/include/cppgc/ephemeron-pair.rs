// Converted from V8 C++ source files:
// Header: ephemeron-pair.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
    use crate::cppgc::liveness_broker::LivenessBroker;
    use crate::cppgc::member::Member;
    use crate::cppgc::weak_member::WeakMember;

    pub struct EphemeronPair<K, V> {
        key: WeakMember<K>,
        value: Member<V>,
    }

    impl<K, V> EphemeronPair<K, V> {
        pub fn new(k: *mut K, v: *mut V) -> Self {
            EphemeronPair {
                key: WeakMember::new(k),
                value: Member::new(v),
            }
        }

        pub fn clear_value_if_key_is_dead(&mut self, broker: &LivenessBroker) {
            if !broker.is_heap_object_alive(self.key.get()) {
                self.value = Member::empty();
            }
        }
    }

    pub mod liveness_broker {
        pub struct LivenessBroker {}

        impl LivenessBroker {
            pub fn is_heap_object_alive<T>(&self, _ptr: *mut T) -> bool {
                true
            }
        }
    }

    pub mod member {
        pub struct Member<T> {
            ptr: *mut T,
        }

        impl<T> Member<T> {
            pub fn new(ptr: *mut T) -> Self {
                Member { ptr }
            }
            pub fn empty() -> Self {
                Member { ptr: std::ptr::null_mut() }
            }
            pub fn get_ptr(&self) -> *mut T {
                self.ptr
            }
        }
    }

    pub mod weak_member {
        pub struct WeakMember<T> {
            ptr: *mut T,
        }

        impl<T> WeakMember<T> {
            pub fn new(ptr: *mut T) -> Self {
                WeakMember { ptr }
            }

            pub fn get(&self) -> *mut T {
                self.ptr
            }
        }
    }
}
