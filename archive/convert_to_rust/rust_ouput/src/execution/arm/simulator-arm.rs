// Converted from V8 C++ source files:
// Header: simulator-arm.h
// Implementation: simulator-arm.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Declares a Simulator for ARM instructions if we are not generating a native
// ARM binary. This Simulator allows us to run and debug ARM code generation on
// regular desktop machines.
// V8 calls into generated code by using the GeneratedCode class,
// which will start execution in the Simulator or forwards to the real entry
// on an ARM HW platform.

// globals.h defines USE_SIMULATOR.

// Running with a simulator.

use std::sync::{Arc, Mutex};

use crate::execution::riscv::simulator_riscv::Simulator;
use crate::execution::isolate::Isolate;

pub struct CachePage {
    validity_map_: [char; CachePage::K_VALIDITY_MAP_SIZE],
    data_: [char; CachePage::K_PAGE_SIZE],
}

impl CachePage {
    pub const LINE_VALID: i32 = 0;
    pub const LINE_INVALID: i32 = 1;

    pub const K_PAGE_SHIFT: i32 = 12;
    pub const K_PAGE_SIZE: usize = 1 << Self::K_PAGE_SHIFT;
    pub const K_PAGE_MASK: i32 = Self::K_PAGE_SIZE as i32 - 1;
    pub const K_LINE_SHIFT: i32 = 2;
    pub const K_LINE_LENGTH: i32 = 1 << Self::K_LINE_SHIFT;
    pub const K_LINE_MASK: i32 = Self::K_LINE_LENGTH - 1;
    pub const K_VALIDITY_MAP_SIZE: usize = Self::K_PAGE_SIZE >> Self::K_LINE_SHIFT;

    pub fn new() -> Self {
        CachePage {
            validity_map_: [Self::LINE_INVALID as char; Self::K_VALIDITY_MAP_SIZE],
            data_: [0; Self::K_PAGE_SIZE],
        }
    }
    pub fn validity_byte(&mut self, offset: i32) -> &mut char {
        &mut self.validity_map_[(offset >> Self::K_LINE_SHIFT) as usize]
    }

    pub fn cached_data(&mut self, offset: i32) -> &mut char {
        &mut self.data_[offset as usize]
    }
}

pub struct Address {}

pub struct IsolateData {}

impl IsolateData {
    pub fn simulator(&mut self) -> *mut Simulator {
        std::ptr::null_mut()
    }
}

pub struct InnerPointerToCodeCacheEntry {}

pub struct MutexGuard {}

pub struct SimulatorBase {}

pub struct Instruction {}

impl Instruction {
    pub fn instruction_bits(&self) -> i32 {
        0
    }
    pub fn set_instruction_bits(&mut self, _bits: i32) {}
}

pub struct Redirection {}
impl Redirection {
    pub fn from_instruction(_instruction: &Instruction) -> Self {
        Redirection{}
    }
    pub fn type_(&self) -> i32{0}
    pub fn external_function(&self) -> *mut i8{std::ptr::null_mut()}
}

pub struct String_ExternalOneByteStringResource {}

pub struct v8 {
    data: i32,
}

pub struct Object {}
pub type BuiltinCode = i32;
pub type HeapObject = i32;
pub type Tagged<T> = i32;
pub type MaybeObject = i32;
pub type Smi = i32;
pub struct StackFrame {}
pub struct CpuProfile {}
pub type ProfilerId = i32;
pub enum CpuProfilingMode {}
pub struct Local<'a, T> {
    _phantom: std::marker::PhantomData<&'a T>,
}
pub struct Context {}
pub type MaybeLocal<'a, T> = Local<'a, T>;
pub struct Value {}
pub struct String {}
pub struct ArrayBuffer {}
pub struct Function {}
pub enum ModuleStatus {}
pub struct Data {}
pub struct Module {}
pub struct HeapHandle {}
pub struct Heap {}
pub struct HeapObjectBase {}
pub struct Frame {}
pub struct StackHandler {}
pub struct ObjectPair {}
pub type Digits = i32;
pub type RWDigits = i32;
pub enum Status {}
pub struct StringView {}
pub struct Maybe<T> {
    _phantom: std::marker::PhantomData<T>,
}

mod cppgc {
    pub mod internal {
        pub struct GCInfoIndex {}
        pub struct AtomicU16 {}
    }
    pub struct Heap {}
    pub trait GarbageCollected {}
}

impl Default for CachePage {
    fn default() -> Self {
        Self::new()
    }
}

