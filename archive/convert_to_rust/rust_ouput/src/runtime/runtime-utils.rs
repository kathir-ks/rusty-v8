// Converted from V8 C++ source files:
// Header: runtime-utils.h
// Implementation: runtime-utils.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::sync::Mutex;

pub struct Isolate {
    has_exception: Mutex<bool>,
}

impl Isolate {
    pub fn new() -> Self {
        Isolate {
            has_exception: Mutex::new(false),
        }
    }
    pub fn has_exception(&self) -> bool {
        *self.has_exception.lock().unwrap()
    }

    pub fn set_exception(&self, value: bool) {
        *self.has_exception.lock().unwrap() = value;
    }
}

pub mod trap_handler {
    pub fn IsTrapHandlerEnabled() -> bool {
        false
    }

    pub fn IsThreadInWasm() -> bool {
        false
    }

    pub fn ClearThreadInWasm() {}

    pub fn SetThreadInWasm() {}
}

pub mod internal {
    use super::*;
    #[derive(Debug, Copy, Clone)]
    pub struct ObjectPair {
        pub x: usize,
        pub y: usize,
    }

    pub fn MakePair(x: usize, y: usize) -> ObjectPair {
        ObjectPair { x, y }
    }

    pub struct SaveAndClearThreadInWasmFlag<'a> {
        thread_was_in_wasm_: bool,
        isolate_: &'a Isolate,
    }

    impl<'a> SaveAndClearThreadInWasmFlag<'a> {
        pub fn new(isolate: &'a Isolate) -> Self {
            SaveAndClearThreadInWasmFlag {
                thread_was_in_wasm_: false,
                isolate_: isolate,
            }
        }
    }
}
