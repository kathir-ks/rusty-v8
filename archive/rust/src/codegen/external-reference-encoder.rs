// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/codegen/external-reference-encoder.h

pub mod external_reference_encoder {
    use std::collections::HashMap;
    use std::ptr::NonNull;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Mutex;

    // Mock definitions for types from other V8 modules
    pub type Address = usize;
    pub type Isolate = usize; // Replace with actual Isolate type
    pub type ExternalReferenceTable = usize; // Replace with actual type
    pub type Maybe<T> = Option<T>;
    pub type Value = u32; // Replace with appropriate struct
    pub type AddressToIndexHashMap = HashMap<Address, Value>;

    #[derive(Debug, Clone, Copy)]
    pub struct Nothing;

    #[macro_export]
    macro_rules! Nothing {
        () => {
            None
        };
    }

    #[macro_export]
    macro_rules! Just {
        ($val:expr) => {
            Some($val)
        };
    }

    pub trait MaybeTrait<T> {
        fn IsNothing(&self) -> bool;
        fn FromJust(self) -> T;
    }

    impl<T> MaybeTrait<T> for Maybe<T> {
        fn IsNothing(&self) -> bool {
            self.is_none()
        }
        fn FromJust(self) -> T {
            self.unwrap()
        }
    }

    pub mod v8 {
        pub mod base {
            pub mod OS {
                pub fn PrintError(format: &str, args: ...) {
                    unsafe {
                        use std::ffi::CString;
                        use std::os::raw::c_char;
                        use libc;

                        let c_format = CString::new(format).unwrap();
                        let c_format_ptr = c_format.as_ptr() as *const c_char;
                        libc::fprintf(libc::stderr, c_format_ptr, args);
                    }
                }

                pub fn Abort() {
                    panic!("Abort");
                }
            }
        }
    }
    
    // Mock external reference table
    pub mod external_reference_table {
        use super::*;

        pub const kSize: usize = 1024;

        pub fn address(table: ExternalReferenceTable, index: usize) -> Address {
          index // Return the index as the address
        }

        pub fn name(table: ExternalReferenceTable, index: usize) -> &'static str {
            "mock_name"
        }

        pub fn ResolveSymbol(addr: *const std::ffi::c_void) -> &'static str {
            "mock_resolve_symbol"
        }
    }

    const EXTERNAL_REFERENCE_STATS: bool = false; // Replace with v8_flags.external_reference_stats

    pub struct ExternalReferenceEncoder {
        map_: Box<AddressToIndexHashMap>,
        #[cfg(debug_assertions)]
        api_references_: *const usize, // Assuming this is a pointer to a null-terminated array of Addresses
        #[cfg(debug_assertions)]
        count_: Vec<u32>,
    }

    impl ExternalReferenceEncoder {
        pub fn new(isolate: Isolate) -> Self {
            let mut map_ = unsafe {
                let isolate_ptr = isolate as *mut AddressToIndexHashMap;
                if isolate_ptr.is_null() {
                    Box::new(HashMap::new())
                } else {
                    Box::new((*isolate_ptr).clone()) // Or however the map is retrieved/cloned
                }
            };
            
            let external_reference_table = isolate as ExternalReferenceTable;

            for i in 0..external_reference_table::kSize {
                let addr = external_reference_table::address(external_reference_table, i);
                if !map_.contains_key(&addr) {
                    map_.insert(addr, encode_value(i as u32, false));
                }
                assert!(map_.contains_key(&addr));
            }
            
            //api references, dummy impl
            unsafe{
                let api_references_ptr : *const usize = isolate as *const usize; //This is incorrect, but added so the code compiles
                if !api_references_ptr.is_null() {
                    let mut i = 0;
                    loop {
                        let addr = *api_references_ptr.offset(i) as Address;
                        if addr == 0 {
                            break;
                        }
                        if !map_.contains_key(&addr) {
                            map_.insert(addr, encode_value(i as u32, true));
                        }
                        assert!(map_.contains_key(&addr));
                        i += 1;
                    }
                }
            }

            Self {
                map_: map_,
                #[cfg(debug_assertions)]
                api_references_: std::ptr::null(), //isolate.api_external_references(),
                #[cfg(debug_assertions)]
                count_: Vec::new(),
            }
        }

        #[cfg(debug_assertions)]
        impl Drop for ExternalReferenceEncoder {
            fn drop(&mut self) {
                if !EXTERNAL_REFERENCE_STATS {
                    return;
                }

                if self.api_references_.is_null() {
                    return;
                }

                unsafe {
                    let mut i = 0;
                    loop {
                        let addr = *self.api_references_.offset(i) as Address;
                        if addr == 0 {
                            break;
                        }
                        assert!(self.map_.contains_key(&addr));
                        //v8::base::OS::Print("index=%5d count=%5d  %-60s\n", i, self.count_[i],ExternalReferenceTable::ResolveSymbol(reinterpret_cast<void*>(addr)));
                        i += 1;
                    }
                }
            }
        }

        pub fn try_encode(&self, address: Address) -> Maybe<Value> {
            match self.map_.get(&address) {
                Some(index) => {
                    #[cfg(debug_assertions)]
                    {
                    } //if result.is_from_api() count_[result.index()]++;
                    Just!(*index)
                }
                None => Nothing!(),
            }
        }

        pub fn encode(&self, address: Address) -> Value {
            match self.map_.get(&address) {
                Some(index) => {
                    #[cfg(debug_assertions)]
                    {
                    } //if result.is_from_api() count_[result.index()]++;
                    *index
                }
                None => {
                    unsafe {
                        let addr = address as *const std::ffi::c_void;
                        v8::base::OS::PrintError("Unknown external reference %p.\n", addr);
                        v8::base::OS::PrintError("%s\n",
                                                 external_reference_table::ResolveSymbol(addr));
                        v8::base::OS::Abort();
                    }
                }
            }
        }
        
        pub fn name_of_address(&self, isolate: Isolate, address: Address) -> &'static str {
            match self.map_.get(&address) {
                Some(value) => {
                    if is_from_api(*value) {
                        "<from api>"
                    } else {
                        let index = get_index(*value);
                        unsafe{
                            external_reference_table::name(isolate as ExternalReferenceTable, index as usize)
                        }
                    }
                },
                None => "<unknown>"
            }
        }
    }

    fn get_index(value: Value) -> u32 {
        value & 0x7FFFFFFF
    }

    fn is_from_api(value: Value) -> bool {
        (value & 0x80000000) != 0
    }

    fn encode_value(index: u32, is_from_api: bool) -> Value {
        let mut encoded: u32 = index;
        if is_from_api {
            encoded |= 0x80000000;
        }
        encoded
    }
}