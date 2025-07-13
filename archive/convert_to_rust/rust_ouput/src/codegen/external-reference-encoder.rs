// Converted from V8 C++ source files:
// Header: external-reference-encoder.h
// Implementation: external-reference-encoder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct BitField<T, const OFFSET: usize, const SIZE: usize> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const OFFSET: usize, const SIZE: usize> BitField<T, const OFFSET, const SIZE> {
        pub fn encode(value: T) -> u32 {
            let value_as_u32 = unsafe { std::mem::transmute::<T, u32>(value) };
            (value_as_u32 as u32) << OFFSET
        }

        pub fn decode(raw: u32) -> T {
            let shifted = (raw >> OFFSET) & ((1 << SIZE) - 1);
            unsafe { std::mem::transmute::<u32, T>(shifted as u32) }
        }
    }

}

pub mod common {
    pub type Address = usize;
}

pub mod utils {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub enum AddressMapError {
        NotFound,
        InsertError,
    }

    pub struct AddressToIndexHashMap {
        map: HashMap<usize, u32>,
    }

    impl AddressToIndexHashMap {
        pub fn new() -> Self {
            AddressToIndexHashMap { map: HashMap::new() }
        }

        pub fn Get(&self, address: usize) -> Result<u32, AddressMapError> {
            match self.map.get(&address) {
                Some(&index) => Ok(index),
                None => Err(AddressMapError::NotFound),
            }
        }

        pub fn Set(&mut self, address: usize, index: u32) -> Result<(), AddressMapError> {
            self.map.insert(address, index);
            Ok(())
        }
    }
}

pub mod codegen {
    use crate::{
        base::BitField,
        common::Address,
        utils::AddressToIndexHashMap,
    };
    use std::{
        fmt,
        sync::{Mutex, MutexGuard},
    };

    pub struct Isolate {
        external_reference_map: Mutex<Option<Box<AddressToIndexHashMap>>>,
        api_external_references: *const i32,
        external_reference_table: Box<ExternalReferenceTable>,
    }

    impl Isolate {
        pub fn new(api_external_references: *const i32, external_reference_table: ExternalReferenceTable) -> Self {
            Isolate {
                external_reference_map: Mutex::new(None),
                api_external_references,
                external_reference_table: Box::new(external_reference_table),
            }
        }

        pub fn external_reference_map(&self) -> Option<MutexGuard<Option<Box<AddressToIndexHashMap>>>> {
            match self.external_reference_map.lock() {
                Ok(guard) => Some(guard),
                Err(_) => None,
            }
        }

        pub fn set_external_reference_map(&self, map: AddressToIndexHashMap) {
            if let Ok(mut guard) = self.external_reference_map.lock() {
                *guard = Some(Box::new(map));
            }
        }

        pub fn api_external_references(&self) -> *const i32 {
            self.api_external_references
        }

        pub fn external_reference_table(&self) -> &ExternalReferenceTable {
            &self.external_reference_table
        }
    }

    pub struct ExternalReferenceTable {
        names: Vec<String>,
        addresses: Vec<usize>,
    }

    impl ExternalReferenceTable {
        pub const kSize: usize = 10;

        pub fn new() -> Self {
            let mut names = Vec::new();
            let mut addresses = Vec::new();

            for i in 0..ExternalReferenceTable::kSize {
                names.push(format!("ExternalReference{}", i));
                addresses.push(i * 8);
            }
            ExternalReferenceTable { names, addresses }
        }

        pub fn address(&self, i: u32) -> usize {
            self.addresses[i as usize]
        }

        pub fn name(&self, i: u32) -> &str {
            &self.names[i as usize]
        }

        pub fn ResolveSymbol(addr: *const std::ffi::c_void) -> String {
            format!("Symbol at address {:?}", addr)
        }
    }

    #[derive(Debug, Clone)]
    pub enum ExternalReferenceEncoderError {
        AddressMapError(crate::utils::AddressMapError),
        NotFound,
    }

    pub struct ExternalReferenceEncoder {
        map_: Box<AddressToIndexHashMap>,
        api_references_: *const i32,

        #[cfg(debug_assertions)]
        count_: Vec<i32>,
    }

    impl ExternalReferenceEncoder {
        pub fn new(isolate: &Isolate) -> Self {
            let mut map_ = match isolate.external_reference_map() {
                Some(guard) => {
                    if guard.is_some() {
                        guard.take().unwrap()
                    } else {
                        Box::new(AddressToIndexHashMap::new())
                    }
                }
                None => Box::new(AddressToIndexHashMap::new()),
            };

            let api_references_ = isolate.api_external_references();

            #[cfg(debug_assertions)]
            let mut count_ = Vec::new();

            #[cfg(debug_assertions)]
            if api_references_ != std::ptr::null() {
                let mut i = 0;
                unsafe {
                    while *api_references_.add(i) != 0 {
                        count_.push(0);
                        i += 1;
                    }
                }
            }

            if let Ok(mut guard) = isolate.external_reference_map.lock() {
                if guard.is_none() {
                    *guard = Some(map_);
                }
            }

            // Add V8's external references.
            let table = isolate.external_reference_table();
            for i in 0..ExternalReferenceTable::kSize as u32 {
                let addr = table.address(i);
                // Ignore duplicate references.
                // This can happen due to ICF. See http://crbug.com/726896.
                if map_.Get(addr).is_err() {
                    map_.Set(addr, Value::Encode(i, false)).unwrap();
                }
                assert!(map_.Get(addr).is_ok());
            }

            // Add external references provided by the embedder.
            if api_references_ != std::ptr::null() {
                let mut i = 0;
                unsafe {
                    while *api_references_.add(i) != 0 {
                        let addr = *api_references_.add(i) as usize;
                        // Ignore duplicate references.
                        // This can happen due to ICF. See http://crbug.com/726896.
                        if map_.Get(addr).is_err() {
                            map_.Set(i as usize, Value::Encode(i as u32, true)).unwrap();
                        }
                        assert!(map_.Get(addr).is_ok());
                        i += 1;
                    }
                }
            }

            ExternalReferenceEncoder {
                map_: Box::new(AddressToIndexHashMap::new()),
                api_references_: std::ptr::null(),
                #[cfg(debug_assertions)]
                count_: Vec::new(),
            }
        }

        #[cfg(debug_assertions)]
        fn dump_stats(&self) {
            if std::env::var("EXTERNAL_REFERENCE_STATS").is_err() {
                return;
            }
            if self.api_references_.is_null() {
                return;
            }

            let mut i = 0;
            unsafe {
                while *self.api_references_.add(i) != 0 {
                    let addr = *self.api_references_.add(i) as usize;
                    assert!(self.map_.Get(addr).is_ok());

                    println!(
                        "index={:5} count={:5}  {}",
                        i,
                        self.count_[i as usize],
                        ExternalReferenceTable::ResolveSymbol(addr as *const std::ffi::c_void)
                    );

                    i += 1;
                }
            }
        }

        pub fn TryEncode(&self, address: Address) -> Result<Value, ExternalReferenceEncoderError> {
            match self.map_.Get(address) {
                Ok(index) => {
                    let result = Value(index);
                    #[cfg(debug_assertions)]
                    {
                        if result.is_from_api() {
                            //self.count_[result.index() as usize] += 1;
                        }
                    }
                    Ok(result)
                }
                Err(e) => Err(ExternalReferenceEncoderError::AddressMapError(e)),
            }
        }

        pub fn Encode(&self, address: Address) -> Value {
            match self.map_.Get(address) {
                Ok(index) => {
                    let result = Value(index);
                    #[cfg(debug_assertions)]
                    {
                        if result.is_from_api() {
                            //self.count_[result.index() as usize] += 1;
                        }
                    }
                    result
                }
                Err(_e) => {
                    let addr = address as *const std::ffi::c_void;
                    eprintln!("Unknown external reference {:p}.", addr);
                    eprintln!("{}", ExternalReferenceTable::ResolveSymbol(addr));
                    std::process::abort();
                }
            }
        }

        pub fn NameOfAddress(&self, isolate: &Isolate, address: Address) -> &str {
            match self.map_.Get(address) {
                Ok(index) => {
                    let value = Value(index);
                    if value.is_from_api() {
                        "<from api>"
                    } else {
                        isolate.external_reference_table().name(value.index())
                    }
                }
                Err(_e) => "<unknown>",
            }
        }
    }

    impl Drop for ExternalReferenceEncoder {
        fn drop(&mut self) {
            #[cfg(debug_assertions)]
            self.dump_stats();
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Value(u32);

    impl Value {
        pub fn new(raw: u32) -> Self {
            Value(raw)
        }

        pub fn Encode(index: u32, is_from_api: bool) -> u32 {
            BitField::<u32, 0, 31>::encode(index) | BitField::<bool, 31, 1>::encode(is_from_api)
        }

        pub fn is_from_api(&self) -> bool {
            BitField::<bool, 31, 1>::decode(self.0)
        }

        pub fn index(&self) -> u32 {
            BitField::<u32, 0, 31>::decode(self.0)
        }
    }
}
