// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add Rust equivalent for base::tmp::StringLiteral
// For now, using &'static str as a placeholder

mod zone_stats {
    pub struct Scope {
        zone: *mut Zone,
    }

    impl Scope {
        pub fn new(zone: *mut Zone) -> Self {
            Scope { zone }
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone
        }

        pub fn destroy(&mut self) {
            unsafe {
                drop(Box::from_raw(self.zone));
                self.zone = std::ptr::null_mut();
            }
        }
    }
}

use std::marker::PhantomData;

#[cfg(debug_assertions)]
mod zone_with_name_impl {
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    pub struct ZoneWithNamePointerImpl<T, const NAME: &'static str> {
        ptr_: *mut T,
        _phantom: PhantomData<T>,
    }

    impl<T, const NAME: &'static str> ZoneWithNamePointerImpl<T, const NAME> {
        pub fn new(ptr: *mut T) -> Self {
            ZoneWithNamePointerImpl {
                ptr_: ptr,
                _phantom: PhantomData,
            }
        }

        pub fn get(&self) -> *mut T {
            self.ptr_
        }
    }

    impl<T, const NAME: &'static str> From<std::ptr::null::type_> for ZoneWithNamePointerImpl<T, const NAME> {
        fn from(_: std::ptr::null::type_) -> Self {
            ZoneWithNamePointerImpl {
                ptr_: std::ptr::null_mut(),
                _phantom: PhantomData,
            }
        }
    }

    impl<T, const NAME: &'static str> Copy for ZoneWithNamePointerImpl<T, const NAME> {}
    impl<T, const NAME: &'static str> Clone for ZoneWithNamePointerImpl<T, const NAME> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T, U, const NAME: &'static str> From<ZoneWithNamePointerImpl<U, NAME>> for ZoneWithNamePointerImpl<T, NAME>
    where
        *mut U: Into<*mut T>,
    {
        fn from(other: ZoneWithNamePointerImpl<U, NAME>) -> Self {
            ZoneWithNamePointerImpl {
                ptr_: other.ptr_ as *mut T,
                _phantom: PhantomData,
            }
        }
    }

    impl<T, const NAME: &'static str> Deref for ZoneWithNamePointerImpl<T, const NAME> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.ptr_ }
        }
    }

    impl<T, const NAME: &'static str> DerefMut for ZoneWithNamePointerImpl<T, const NAME> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.ptr_ }
        }
    }

    impl<T, const NAME: &'static str> From<ZoneWithNamePointerImpl<T, NAME>> for *mut T {
        fn from(wrapper: ZoneWithNamePointerImpl<T, NAME>) -> Self {
            wrapper.ptr_
        }
    }

    impl<T, const NAME: &'static str> ZoneWithNamePointerImpl<T, NAME> {
        pub fn as_ptr(&self) -> *mut T {
            self.ptr_
        }
    }
}

#[cfg(debug_assertions)]
pub type ZoneWithNamePointer<T, const NAME: &'static str> = zone_with_name_impl::ZoneWithNamePointerImpl<T, NAME>;

#[cfg(not(debug_assertions))]
pub type ZoneWithNamePointer<T, const NAME: &'static str> = *mut T;

pub struct Zone { }

impl Zone {
    pub fn new<T>(&self) -> *mut T {
        Box::into_raw(Box::new(unsafe { std::mem::zeroed() }))
    }
    pub fn allocate_array<T>(&self, length: usize) -> *mut T {
        let vec = vec![unsafe { std::mem::zeroed() }; length];
        Box::into_raw(vec.into_boxed_slice()) as *mut T
    }
}

pub struct ZoneWithName<const NAME: &'static str> {
    scope_: zone_stats::Scope,
    _phantom: PhantomData<[(); NAME.len()]>,
}

impl<const NAME: &'static str> ZoneWithName<NAME> {
    pub fn new(pool: *mut ZoneStats, name: &str, support_zone_compression: bool) -> Self {
        assert_eq!(name, NAME);
        let zone = Box::into_raw(Box::new(Zone {}));
        let scope_ = zone_stats::Scope::new(zone);
        ZoneWithName {
            scope_: scope_,
            _phantom: PhantomData,
        }
    }

    pub fn new_t<T>(&self) -> ZoneWithNamePointer<T, NAME> {
        let ptr = unsafe { (*self.scope_.zone()).new::<T>() };
        ZoneWithNamePointer::new(ptr)
    }

    pub fn allocate_array<T>(&self, length: usize) -> ZoneWithNamePointer<T, NAME> {
        let ptr = unsafe { (*self.scope_.zone()).allocate_array::<T>(length) };
        ZoneWithNamePointer::new(ptr)
    }
    
    pub fn get(&self) -> *mut Zone {
        self.scope_.zone()
    }

    pub fn destroy(&mut self) {
        self.scope_.destroy();
    }
}

impl<const NAME: &'static str> Drop for ZoneWithName<NAME> {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl<const NAME: &'static str> From<&ZoneWithName<NAME>> for *mut Zone {
    fn from(zone_with_name: &ZoneWithName<NAME>) -> Self {
        zone_with_name.get()
    }
}

struct ZoneStats {}