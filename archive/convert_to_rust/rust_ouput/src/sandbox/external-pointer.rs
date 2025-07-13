// Converted from V8 C++ source files:
// Header: external-pointer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod external_pointer {
    use crate::sandbox::isolate::IsolateForSandbox;
    use crate::sandbox::code_pointer_table::Address;
    use std::marker::PhantomData;
    use std::mem;
    use std::sync::atomic::{AtomicU64, Ordering};

    pub trait ExternalPointerTag {}
    pub trait ExternalPointerTagRange {}

    pub type ExternalPointer_t = u64;
    pub const kNullExternalPointerHandle: ExternalPointer_t = 0; // Define a reasonable default value

    pub struct ExternalPointerMember<T: ExternalPointerTag> {
        storage_: [u8; 8], // Assuming ExternalPointer_t is 64-bit
        _phantom: PhantomData<T>,
    }

    impl<T: ExternalPointerTag> Default for ExternalPointerMember<T> {
        fn default() -> Self {
            ExternalPointerMember {
                storage_: [0u8; 8],
                _phantom: PhantomData,
            }
        }
    }

    impl<T: ExternalPointerTag> ExternalPointerMember<T> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn init(&mut self, host_address: Address, isolate: IsolateForSandbox, value: Address) {
            self.store(isolate, value);
        }

        #[inline]
        pub fn load(&self, _isolate: IsolateForSandbox) -> Address {
            let encoded_value = self.load_encoded();
            Address {address: encoded_value}
        }

        #[inline]
        pub fn store(&mut self, _isolate: IsolateForSandbox, value: Address) {
            self.store_encoded(value.address);
        }

        #[inline]
        pub fn load_encoded(&self) -> ExternalPointer_t {
            unsafe { u64::from_le_bytes(self.storage_.clone()) }
        }

        #[inline]
        pub fn store_encoded(&mut self, value: ExternalPointer_t) {
            self.storage_ = value.to_le_bytes();
        }

        pub fn storage_address(&mut self) -> Address {
            Address {address: self.storage_.as_mut_ptr() as u64}
        }
    }

    #[inline]
    pub fn init_external_pointer_field<T: ExternalPointerTag>(
        _host_address: Address,
        field_address: Address,
        isolate: IsolateForSandbox,
        value: Address,
    ) {
        // Assuming field_address points to memory where we can store the Address value
        unsafe {
            let ptr = field_address.address as *mut u64;
            *ptr = value.address;
        }
    }

    #[inline]
    pub fn read_external_pointer_field<TR: ExternalPointerTagRange>(
        field_address: Address,
        _isolate: IsolateForSandbox,
    ) -> Address {
        // Assuming field_address points to a memory location containing an ExternalPointer_t
        unsafe {
            let ptr = field_address.address as *const u64;
            let value = *ptr;
            Address {address: value}
        }
    }

    #[inline]
    pub fn write_external_pointer_field<T: ExternalPointerTag>(
        field_address: Address,
        _isolate: IsolateForSandbox,
        value: Address,
    ) {
        // Assuming field_address points to a memory location where we can write an ExternalPointer_t
        unsafe {
            let ptr = field_address.address as *mut u64;
            *ptr = value.address;
        }
    }
}
