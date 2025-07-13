// Converted from V8 C++ source files:
// Header: lookup-cache.h
// Implementation: lookup-cache.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct Map {}
pub struct Name {}
pub struct Object {}

pub struct DescriptorLookupCache {
    keys_: [Key; Self::K_LENGTH],
    results_: [i32; Self::K_LENGTH],
}

impl DescriptorLookupCache {
    const K_ABSENT: i32 = -2;
    const K_LENGTH: usize = 64;

    pub fn new() -> Self {
        let mut keys_: [Key; Self::K_LENGTH] = unsafe { std::mem::zeroed() };
        let mut results_: [i32; Self::K_LENGTH] = [0; Self::K_LENGTH];

        for i in 0..Self::K_LENGTH {
            keys_[i].source = Tagged::<Map>::default();
            keys_[i].name = Tagged::<Name>::default();
            results_[i] = Self::K_ABSENT;
        }

        Self {
            keys_: keys_,
            results_: results_,
        }
    }

    pub fn lookup(&self, source: Tagged<Map>, name: Tagged<Name>) -> i32 {
        let index = Self::hash(source, name) % Self::K_LENGTH;
        if self.keys_[index].source == source && self.keys_[index].name == name {
            self.results_[index]
        } else {
            Self::K_ABSENT
        }
    }

    pub fn update(&mut self, source: Tagged<Map>, name: Tagged<Name>, result: i32) {
        let index = Self::hash(source, name) % Self::K_LENGTH;
        self.keys_[index].source = source;
        self.keys_[index].name = name;
        self.results_[index] = result;
    }

    pub fn clear(&mut self) {
        for index in 0..Self::K_LENGTH {
            self.keys_[index].source = Tagged::<Map>::default();
            self.keys_[index].name = Tagged::<Name>::default();
            self.results_[index] = Self::K_ABSENT;
        }
    }

    fn hash(source: Tagged<Map>, name: Tagged<Name>) -> usize {
        let source_ptr = source.ptr as usize;
        let name_ptr = name.ptr as usize;
        (source_ptr.wrapping_add(name_ptr)) % Self::K_LENGTH
    }
}

#[derive(Copy, Clone)]
struct Key {
    source: Tagged<Map>,
    name: Tagged<Name>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Tagged<T> {
    ptr: *mut T,
}

impl<T> Tagged<T> {
    pub fn new(ptr: *mut T) -> Self {
        Tagged { ptr }
    }
}

impl<T> Default for Tagged<T> {
    fn default() -> Self {
        Tagged { ptr: std::ptr::null_mut() }
    }
}

impl<T> std::cmp::PartialEq for Tagged<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> std::cmp::Eq for Tagged<T> {}
