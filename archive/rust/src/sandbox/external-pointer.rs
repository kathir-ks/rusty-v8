// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod external_pointer {
    use std::mem;
    use std::ptr;

    // Assuming these are defined elsewhere and are relevant to the sandbox.
    pub type Address = usize;
    pub type ExternalPointer_t = usize;
    pub type Tagged_t = usize;

    pub trait IsolateForSandbox {
        fn is_sandbox_enabled(&self) -> bool;
        fn external_pointer_table_mut(&mut self) -> &mut ExternalPointerTable;
        fn external_pointer_table(&self) -> &ExternalPointerTable;
    }

    pub trait ExternalPointerTag {}

    pub trait ExternalPointerTagRange {}

    pub struct ExternalPointerTable {
        // Dummy implementation for now
    }

    impl ExternalPointerTable {
        pub fn new() -> Self {
            ExternalPointerTable {}
        }

        pub fn insert(&mut self, _value: Address) -> usize {
            // Dummy implementation: returns a dummy handle.
            1
        }

        pub fn get(&self, _handle: usize) -> Address {
            // Dummy implementation: returns a dummy address
            0
        }

        pub fn set(&mut self, _handle: usize, _value: Address) {
            // Dummy Implementation
        }
    }

    pub struct NullExternalPointerTag {}
    impl ExternalPointerTag for NullExternalPointerTag {}

    pub struct SomeExternalPointerTag {}
    impl ExternalPointerTag for SomeExternalPointerTag {}

    pub struct SomeExternalPointerTagRange {}
    impl ExternalPointerTagRange for SomeExternalPointerTagRange {}

    pub struct AnotherExternalPointerTagRange {}
    impl ExternalPointerTagRange for AnotherExternalPointerTagRange {}

    pub struct ExternalPointerMember<T: ExternalPointerTag> {
        storage: [u8; mem::size_of::<ExternalPointer_t>()],
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T: ExternalPointerTag> ExternalPointerMember<T> {
        pub fn new() -> Self {
            ExternalPointerMember {
                storage: [0u8; mem::size_of::<ExternalPointer_t>()],
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn init<I: IsolateForSandbox>(
            &mut self,
            _host_address: Address,
            _isolate: &mut I,
            value: Address,
        ) {
            self.store(_isolate, value);
        }

        #[inline]
        pub fn load<I: IsolateForSandbox>(&self, isolate: &I) -> Address {
            if isolate.is_sandbox_enabled() {
                let handle: ExternalPointer_t = self.load_encoded();
                isolate.external_pointer_table().get(handle)
            } else {
                unsafe { ptr::read_unaligned(self.storage.as_ptr() as *const Address) }
            }
        }

        #[inline]
        pub fn store<I: IsolateForSandbox>(&mut self, isolate: &mut I, value: Address) {
            if isolate.is_sandbox_enabled() {
                let handle = isolate.external_pointer_table_mut().insert(value);
                self.store_encoded(handle);
            } else {
                unsafe {
                    ptr::write_unaligned(self.storage.as_mut_ptr() as *mut Address, value);
                }
            }
        }

        #[inline]
        pub fn load_encoded(&self) -> ExternalPointer_t {
            unsafe { ptr::read_unaligned(self.storage.as_ptr() as *const ExternalPointer_t) }
        }

        #[inline]
        pub fn store_encoded(&mut self, value: ExternalPointer_t) {
            unsafe {
                ptr::write_unaligned(self.storage.as_mut_ptr() as *mut ExternalPointer_t, value);
            }
        }

        pub fn storage_address(&self) -> Address {
            self.storage.as_ptr() as Address
        }
    }

    #[inline]
    pub fn init_external_pointer_field<T: ExternalPointerTag, I: IsolateForSandbox>(
        _host_address: Address,
        field_address: Address,
        isolate: &mut I,
        value: Address,
    ) {
        let mut member = ExternalPointerMember::<T>::new();
        member.init(_host_address, isolate, value);

        let field_ptr = field_address as *mut ExternalPointerMember<T>;
        unsafe {
            ptr::write_unaligned(field_ptr, member);
        }
    }

    #[inline]
    pub fn read_external_pointer_field<TR: ExternalPointerTagRange, I: IsolateForSandbox>(
        field_address: Address,
        isolate: &I,
    ) -> Address {
        //  TR can't be used here because its type parameter is not
        //  used in the return type. To use TR, you'd need to use it in the return type,
        //  or to use it to create an object that's used locally (and potentially returned).

        //  The original C++ code reads from memory pointed to by field_address, so this
        //  Rust translation should do the same.  However, we don't know the type that is stored
        //  at that address without more context from the broader V8 codebase.  So we're going to
        //  assume it's an ExternalPointerMember<NullExternalPointerTag> for now.  That might be
        //  incorrect and needs adjustment when the complete calling context is available.

        let field_ptr = field_address as *const ExternalPointerMember<NullExternalPointerTag>;
        unsafe {
            let member = ptr::read_unaligned(field_ptr);
            member.load(isolate)
        }
    }

    #[inline]
    pub fn write_external_pointer_field<T: ExternalPointerTag, I: IsolateForSandbox>(
        field_address: Address,
        isolate: &mut I,
        value: Address,
    ) {
        let field_ptr = field_address as *mut ExternalPointerMember<T>;
        unsafe {
            let mut member = ExternalPointerMember::<T>::new();
            member.store(isolate, value);
            ptr::write_unaligned(field_ptr, member);
        }
    }
}