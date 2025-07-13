// Converted from V8 C++ source files:
// Header: flags.h
// Implementation: flags.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2006-2008 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
use std::any::Any;
use std::fmt;
use std::fmt::Debug;
use std::io::Stdout;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::{
    cmp,
    ffi::{CStr, CString},
    mem,
    num::ParseIntError,
    ptr,
    str,
};

use crate::V8;

#[cfg(V8_ENABLE_WEBASSEMBLY)]
mod wasm {
    pub const kMaxNumInitialMemoryPages: usize = 256;
}

pub mod base {
    pub mod hashing {
        pub fn hash_range(start: *const u8, end: *const u8) -> u32 {
            let mut hash: u32 = 5381;
            let mut ptr = start;
            while ptr < end {
                hash = ((hash << 5).wrapping_add(hash)).wrapping_add(unsafe { *ptr } as u32);
                ptr = unsafe { ptr.add(1) };
            }
            hash
        }
    }
    pub mod platform {
        pub mod OS {
            pub fn ExitProcess(_code: i32) -> ! {
                std::process::exit(_code);
            }
        }
    }
    pub struct LazyInstance<T> {
        value: Mutex<Option<T>>,
    }

    impl<T> LazyInstance<T> {
        pub const fn new() -> Self {
            LazyInstance {
                value: Mutex::new(None),
            }
        }

        pub fn get<F>(&self, init: F) -> std::sync::MutexGuard<'_, Option<T>>
        where
            F: FnOnce() -> T,
        {
            let mut value = self.value.lock().unwrap();
            if value.is_none() {
                *value = Some(init());
            }
            value
        }
    }

    pub struct ScopedVector<T>(Vec<T>);

    impl<T> ScopedVector<T> {
        pub fn new(vec: Vec<T>) -> Self {
            ScopedVector(vec)
        }

        pub fn begin(&self) -> *const T {
            self.0.as_ptr()
        }
    }
}

pub mod codegen {
    pub mod cpu_features {
        pub fn Probe(_arg: bool) {}
        pub fn PrintTarget() {}
        pub fn PrintFeatures() {}
    }
}

pub mod logging {
    pub mod tracing_flags {}
}

pub mod tracing {
    pub mod tracing_category_observer {}
}

pub mod utils {
    pub mod allocation {
        pub fn NewArray<T>(len: usize) -> Vec<T>
        where
            T: Default + Copy,
        {
            vec![T::default(); len]
        }
    }
    pub mod memcopy {
        pub fn MemCopy<T: Copy>(dest: *mut T, src: *const T, count: usize) {
            unsafe {
                ptr::copy_nonoverlapping(src, dest, count);
            }
        }
    }
    pub mod ostreams {
        use std::io::{self, Write};

        pub struct StdoutStream;

        impl StdoutStream {
            pub fn new() -> Self {
                StdoutStream
            }
        }

        impl Write for StdoutStream {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                io::stdout().write(buf)
            }

            fn flush(&mut self) -> io::Result<()> {
                io::stdout().flush()
            }
        }
    }
    pub mod utils {}
}

const KB: usize = 1024;

#[derive(Debug, PartialEq)]
pub enum FlagTypeError {
    InvalidBoolValue(String),
    InvalidIntValue(String, ParseIntError),
    InvalidUintValue(String, ParseIntError),
    InvalidUint64Value(String, ParseIntError),
    InvalidFloatValue(String, std::num::ParseFloatError),
    OutOfRange {
        flag: String,
        flag_type: String,
        min: i64,
        max: u64,
    },
}

impl fmt::Display for FlagTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlagTypeError::InvalidBoolValue(flag) => {
                write!(f, "Error: invalid value for flag {} of type bool", flag)
            }
            FlagTypeError::InvalidIntValue(flag, err) => {
                write!(f, "Error: invalid value for flag {} of type int: {}", flag, err)
            }
            FlagTypeError::InvalidUintValue(flag, err) => {
                write!(f, "Error: invalid value for flag {} of type uint: {}", flag, err)
            }
            FlagTypeError::InvalidUint64Value(flag, err) => {
                write!(f, "Error: invalid value for flag {} of type uint64: {}", flag, err)
            }
            FlagTypeError::InvalidFloatValue(flag, err) => {
                write!(f, "Error: invalid value for flag {} of type float: {}", flag, err)
            }
            FlagTypeError::OutOfRange {
                flag,
                flag_type,
                min,
                max,
            } => {
                write!(
                    f,
                    "Error: Value for flag {} of type {} is out of bounds [{} - {}]",
                    flag, flag_type, min, max
                )
            }
        }
    }
}

pub trait FlagValueType: Copy {
    const TYPE: Flag::FlagType;
    fn as_any(&self) -> &dyn Any;
}

macro_rules! impl_flag_value_type {
    ($type:ty, $flag_type:expr) => {
        impl FlagValueType for $type {
            const TYPE: Flag::FlagType = $flag_type;
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

impl_flag_value_type!(bool, Flag::FlagType::TYPE_BOOL);
impl_flag_value_type!(Option<bool>, Flag::FlagType::TYPE_MAYBE_BOOL);
impl_flag_value_type!(i32, Flag::FlagType::TYPE_INT);
impl_flag_value_type!(u32, Flag::FlagType::TYPE_UINT);
impl_flag_value_type!(u64, Flag::FlagType::TYPE_UINT64);
impl_flag_value_type!(f64, Flag::FlagType::TYPE_FLOAT);
impl_flag_value_type!(usize, Flag::FlagType::TYPE_SIZE_T);

#[repr(C, align(4096))]
pub struct FlagValues {
    pub allow_overwriting_for_next_flag: bool,
    pub abort_on_contradictory_flags: bool,
    pub exit_on_contradictory_flags: bool,
    pub fuzzing: bool,
    pub help: bool,
    pub print_feature_flags_json: bool,
    pub profile_deserialization: bool,
    pub random_seed: i32,
    pub predictable: bool,
    pub concurrent_sparkplug: bool,
    pub concurrent_recompilation: bool,
    pub lazy_feedback_allocation: bool,
    #[cfg(V8_ENABLE_MAGLEV)]
    pub maglev_deopt_data_on_background: bool,
    #[cfg(V8_ENABLE_MAGLEV)]
    pub maglev_build_code_on_background: bool,
    pub parallel_scavenge: bool,
    pub concurrent_marking: bool,
    pub concurrent_minor_ms_marking: bool,
    pub concurrent_array_buffer_sweeping: bool,
    pub parallel_marking: bool,
    pub concurrent_sweeping: bool,
    pub parallel_compaction: bool,
    pub parallel_pointer_update: bool,
    pub parallel_weak_ref_clearing: bool,
    pub memory_reducer: bool,
    pub cppheap_concurrent_marking: bool,
    pub cppheap_incremental_marking: bool,
    pub single_threaded_gc: bool,
    pub fuzzing_and_concurrent_recompilation: bool,
    pub predictable_and_random_seed_is_0: bool,
    pub always_osr_from_maglev: bool,
    pub disable_optimizing_compilers: bool,
    pub jitless: bool,
    pub lite_mode: bool,
    pub turbofan: bool,
    pub turboshaft: bool,
    pub correctness_fuzzer_suppressions: bool,
    pub stress_lazy_compilation: bool,
    pub turbo_stats: bool,
    pub turbo_stats_nvp: bool,
    pub turbo_stats_wasm: bool,
    pub expose_async_hooks: bool,
    pub parallel_compile_tasks_for_lazy: bool,
    pub stress_snapshot: bool,
    pub always_turbofan: bool,
    pub assert_types: bool,
    pub stress_concurrent_inlining: bool,
    pub stress_concurrent_inlining_attach_code: bool,
    pub maglev_future: bool,
    pub stress_maglev: bool,
    pub turboshaft_wasm_in_js_inlining: bool,
    pub predictable_gc_schedule: bool,
    pub optimize_for_size: bool,
    pub single_threaded: bool,
    pub turboshaft_assert_types: bool,
    pub stress_compaction: bool,
    pub trace_turbo: bool,
    pub trace_turbo_graph: bool,
}

impl FlagValues {
    pub fn new() -> Self {
        FlagValues {
            allow_overwriting_for_next_flag: false,
            abort_on_contradictory_flags: true,
            exit_on_contradictory_flags: false,
            fuzzing: false,
            help: false,
            print_feature_flags_json: false,
            profile_deserialization: false,
            random_seed: 0,
            predictable: false,
            concurrent_sparkplug: true,
            concurrent_recompilation: true,
            lazy_feedback_allocation: true,
            #[cfg(V8_ENABLE_MAGLEV)]
            maglev_deopt_data_on_background: true,
            #[cfg(V8_ENABLE_MAGLEV)]
            maglev_build_code_on_background: true,
            parallel_scavenge: true,
            concurrent_marking: true,
            concurrent_minor_ms_marking: true,
            concurrent_array_buffer_sweeping: true,
            parallel_marking: true,
            concurrent_sweeping: true,
            parallel_compaction: true,
            parallel_pointer_update: true,
            parallel_weak_ref_clearing: true,
            memory_reducer: true,
            cppheap_concurrent_marking: true,
            cppheap_incremental_marking: true,
            single_threaded_gc: false,
            fuzzing_and_concurrent_recompilation: true,
            predictable_and_random_seed_is_0: false,
            always_osr_from_maglev: false,
            disable_optimizing_compilers: false,
            jitless: false,
            lite_mode: false,
            turbofan: true,
            turboshaft: true,
            correctness_fuzzer_suppressions: false,
            stress_lazy_compilation: false,
            turbo_stats: false,
            turbo_stats_nvp: false,
            turbo_stats_wasm: false,
            expose_async_hooks: false,
            parallel_compile_tasks_for_lazy: true,
            stress_snapshot: false,
            always_turbofan: false,
            assert_types: false,
            stress_concurrent_inlining: false,
            stress_concurrent_inlining_attach_code: false,
            maglev_future: true,
            stress_maglev: false,
            turboshaft_wasm_in_js_inlining: true,
            predictable_gc_schedule: false,
            optimize_for_size: false,
            single_threaded: false,
            turboshaft_assert_types: false,
            stress_compaction: false,
            trace_turbo: false,
            trace_turbo_graph: false,
        }
    }
}

#[no_mangle]
pub static mut v8_flags: FlagValues = FlagValues {
    allow_overwriting_for_next_flag: false,
    abort_on_contradictory_flags: true,
    exit_on_contradictory_flags: false,
    fuzzing: false,
    help: false,
    print_feature_flags_json: false,
    profile_deserialization: false,
    random_seed: 0,
    predictable: false,
    concurrent_sparkplug: true,
    concurrent_recompilation: true,
    lazy_feedback_allocation: true,
    #[cfg(V8_ENABLE_MAGLEV)]
    maglev_deopt_data_on_background: true,
    #[cfg(V8_ENABLE_MAGLEV)]
    maglev_build_code_on_background: true,
    parallel_scavenge: true,
    concurrent_marking: true,
    concurrent_minor_ms_marking: true,
    concurrent_array_buffer_sweeping: true,
    parallel_marking: true,
    concurrent_sweeping: true,
    parallel_compaction: true,
    parallel_pointer_update: true,
    parallel_weak_ref_clearing: true,
    memory_reducer: true,
    cppheap_concurrent_marking: true,
    cppheap_incremental_marking: true,
    single_threaded_gc: false,
    fuzzing_and_concurrent_recompilation: true,
    predictable_and_random_seed_is_0: false,
    always_osr_from_maglev: false,
    disable_optimizing_compilers: false,
    jitless: false,
    lite_mode: false,
    turbofan: true,
    turboshaft: true,
    correctness_fuzzer_suppressions: false,
    stress_lazy_compilation: false,
    turbo_stats: false,
    turbo_stats_nvp: false,
    turbo_stats_wasm: false,
    expose_async_hooks: false,
    parallel_compile_tasks_for_lazy: true,
    stress_snapshot: false,
    always_turbofan: false,
    assert_types: false,
    stress_concurrent_inlining: false,
    stress_concurrent_inlining_attach_code: false,
    maglev_future: true,
    stress_maglev: false,
    turboshaft_wasm_in_js_inlining: true,
    predictable_gc_schedule: false,
    optimize_for_size: false,
    single_threaded: false,
    turboshaft_assert_types: false,
    stress_compaction: false,
    trace_turbo: false,
    trace_turbo_graph: false,
};

// The value of a single flag (this is the type of all v8_flags.* fields).
pub struct FlagValue<T: FlagValueType> {
    value: T,
}

impl<T: FlagValueType> FlagValue<T> {
    pub const fn new(value: T) -> Self {
        FlagValue { value }
    }

    pub fn value(&self) -> T {
        self.value
    }
}

impl<T: FlagValueType> std::ops::Deref for FlagValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: FlagValueType> std::ops::DerefMut for FlagValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: FlagValueType> FlagValue<T> {
    pub fn set(&mut self, new_value: T) -> &mut FlagValue<T> {
        self.value = new_value;
        self
    }
}

#[derive(Debug)]
pub struct FlagName {
    negated: bool,
    name: String,
}

impl FlagName {
    pub fn new(name: String, negated: bool) -> Self {
        FlagName { negated, name }
    }
}

impl From<&str> for FlagName {
    fn from(name: &str) -> Self {
        FlagName {
            negated: false,
            name: name.to_string(),
        }
    }
}

pub struct FlagHelpers;

impl FlagHelpers {
    pub fn NormalizeChar(ch: char) -> char {
        if ch == '_' {
            '-'
        } else {
            ch
        }
    }

    pub fn FlagNamesCmp(a: &str, b: &str) -> i32 {
        let mut i = 0;
        loop {
            let ac = a.chars().nth(i).unwrap_or('\0');
            let bc = b.chars().nth(i).unwrap_or('\0');

            let nac = FlagHelpers::NormalizeChar(ac);
            let nbc = FlagHelpers::NormalizeChar(bc);

            if nac < nbc {
                return -1;
            }
            if nac > nbc {
                return 1;
            }
            if nac == '\0' {
                if nbc == '\0' {
                    return 0;
                } else {
                    return 0;
                }
            }
            i += 1;
        }
    }

    pub fn EqualNames(a: &str, b: &str) -> bool {
        FlagHelpers::FlagNamesCmp(a, b) == 0
    }

    pub fn EqualNameWithSuffix(a: &str, b: &str) -> bool {
        let mut i = 0;
        loop {
            let ac = a.chars().nth(i).unwrap_or('\0');
            let bc = b.chars().nth(i).unwrap_or('\0');
            let nac = FlagHelpers::NormalizeChar(ac);
            let nbc = FlagHelpers::NormalizeChar(bc);

            if nac == '\0' {
                break;
            }
            if nac != nbc {
                return false;
            }
            i += 1;
        }
        let bc = b.chars().nth(i).unwrap_or('\0');
        bc == '\0' || bc.is_whitespace()
    }
}

impl fmt::Display for FlagName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negated {
            write!(f, "--no-")?;
        } else {
            write!(f, "--")?;
        }
        for c in self.name.chars() {
            write!(f, "{}", FlagHelpers::NormalizeChar(c))?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SetBy {
    kDefault,
    kWeakImplication,
    kImplication,
    kCommandLine,
}

impl Default for SetBy {
    fn default() -> Self {
        SetBy::kDefault
    }
}

#[derive(Debug)]
pub struct Flag {
    name: String,
    comment: String,
    type_: Flag::FlagType,
    valptr_: *mut u8,
    default_value_: Box<dyn Any>,
    bool_default_: bool,
    int_default_: i32,
    uint_default_: u32,
    uint64_default_: u64,
    float_default_: f64,
    size_t_default_: usize,
    string_default_: *const i8,
    owns_ptr_: bool,
    read_only_: bool,
    set_by_: SetBy,
    implied_by_: *const i8,
    implied_by_ptr_: *mut Flag,
}

impl Flag {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum FlagType {
        TYPE_BOOL,
        TYPE_MAYBE_BOOL,
        TYPE_INT,
        TYPE_UINT,
        TYPE_UINT64,
        TYPE_FLOAT,
        TYPE_SIZE_T,
        TYPE_STRING,
    }

    pub fn new<T: FlagValueType + 'static>(
        name: String,
        comment: String,
        type_: Flag::FlagType,
        valptr_: *mut u8,
        default_value: T,
        read_only_: bool,
    ) -> Self {
        let mut result = Self {
            name,
            comment,
            type_,
            valptr_,
            default_value_: Box::new(default_value),
            bool_default_: false,
            int_default_: 0,
            uint_default_: 0,
            uint64_default_: 0,
            float_default_: 0.0,
            size_t_default_: 0,
            string_default_: ptr::null(),
            owns_ptr_: false,
            read_only_: read_only_,
            set_by_: SetBy::kDefault,
            implied_by_: ptr::null(),
            implied_by_ptr_: ptr::null_mut(),
        };
        match type_ {
            Flag::FlagType::TYPE_BOOL => {
                result.bool_default_ = *result
                    .default_value_
                    .downcast_ref::<bool>()
                    .expect("Downcast failed");
            }
            Flag::FlagType::TYPE_MAYBE_BOOL => {
                // Handle maybe_bool default
            }
            Flag::FlagType::TYPE_INT => {
                result.int_default_ = *result
                    .default_value_
                    .downcast_ref::<i32>()
                    .expect("Downcast failed");
            }
            Flag::FlagType::TYPE_UINT => {
                result.uint_default_ = *result
                    .default_value_
                    .downcast_ref::<u32>()
                    .expect("Downcast failed");
            }
            Flag::FlagType::TYPE_UINT64 => {
                result.uint64_default_ = *result
                    .default_value_
                    .downcast_ref::<u64>()
                    .expect("Downcast failed");
            }
            Flag::FlagType::TYPE_FLOAT => {
                result.float_default_ = *result
                    .default_value_
                    .downcast_ref::<f64>()
                    .expect("Downcast failed");
            }
            Flag::FlagType::TYPE_SIZE_T => {
                result.size_t_default_ = *result
                    .default_value_
                    .downcast_ref::<usize>()
                    .expect("Downcast failed");
            }
            Flag::FlagType::TYPE_STRING => {
                // Handle string default
            }
        }
        result
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn type_(&self) -> Flag::FlagType {
        self.type_
    }

    pub fn bool_variable(&self) -> bool {
        unsafe { *(self.valptr_ as *const bool) }
    }

    pub fn maybe_bool_variable(&self) -> Option<bool> {
        unsafe { *(self.valptr_ as *const Option<bool>) }
    }

    pub fn int_variable(&self) -> i32 {
        unsafe { *(self.valptr_ as *const i32) }
    }

    pub fn uint_variable(&self) -> u32 {
        unsafe { *(self.valptr_ as *const u32) }
    }

    pub fn uint64_variable(&self) -> u64 {
        unsafe { *(self.valptr_ as *const u64) }
    }

    pub fn float_variable(&self) -> f64 {
        unsafe { *(self.valptr_ as *const f64) }
    }

    pub fn size_t_variable(&self) -> usize {
        unsafe { *(self.valptr_ as *const usize) }
    }

    pub fn string_value(&self) -> &str {
        unsafe {
            let ptr = *(self.valptr_ as *const *const i8);
            if ptr.is_null() {
                ""
            } else {
                CStr::from_ptr(ptr).to_str().unwrap()
            }
        }
    }

    pub fn bool_default(&self) -> bool {
        self.bool_default_
    }

    pub fn int_default(&self) -> i32 {
        self.int_default_
    }

    pub fn uint_default(&self) -> u32 {
        self.uint_default_
    }

    pub fn uint64_default(&self) -> u64 {
        self.uint64_default_
    }

    pub fn float_default(&self) -> f64 {
        self.float_default_
    }

    pub fn size_t_default(&self) -> usize {
        self.size_t_default_
    }

    pub fn string_default(&self) -> *const i8 {
        self.string_default_
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only_
    }

    pub fn set_bool_variable(&mut self, new_value: bool, set_by: SetBy) -> bool {
        let change_flag = self.CheckFlagChange(set_by, self.bool_variable() != new_value, ptr::null());
        if change_flag {
            unsafe { *(self.valptr_ as *mut bool) = new_value };
        }
        change_flag
    }

    pub fn set_maybe_bool_variable(&mut self, new_value: Option<bool>, set_by: SetBy) -> bool {
        let change_flag = self.CheckFlagChange(set_by, self.maybe_bool_variable() != new_value, ptr::null());
        if change_flag {
            unsafe { *(self.valptr_ as *mut Option<bool>) = new_value };
        }
        change_flag
    }

    pub fn set_int_variable(&mut self, new_value: i32, set_by: SetBy) -> bool {
         let change_flag = self.CheckFlagChange(set_by, self.int_variable() != new_value, ptr::null());
        if change_flag {
            unsafe { *(self.valptr_ as *mut i32) = new_value };
        }
        change_flag
    }

    pub fn set_uint_variable(&mut self, new_value: u32, set_by: SetBy) -> bool {
        let change_flag = self.CheckFlagChange(set_by, self.uint_variable() != new_value, ptr::null());
        if change_flag {
            unsafe { *(self.valptr_ as *mut u32) = new_value };
        }
        change_flag
    }

    pub fn set_uint64_variable(&mut self, new_value: u64, set_by: SetBy) -> bool {
        let change_flag = self.CheckFlagChange(set_by, self.uint64_variable() != new_value, ptr::null());
        if change_flag {
            unsafe { *(self.valptr_ as *mut u64) = new_value };
        }
        change_flag
    }

    pub fn set_float_variable(&mut self, new_value: f64, set_by: SetBy) -> bool {
        let change_flag = self.CheckFlagChange(set_by, self.float_variable() != new_value, ptr::null());
        if change_flag {
            unsafe { *(self.valptr_ as *mut f64) = new_value };
        }
        change_flag
    }

    pub fn set_size_t_variable(&mut self, new_value: usize, set_by: SetBy) -> bool {
        let change_flag = self.CheckFlagChange(set_by, self.size_t_variable() != new_value, ptr::null());
        if change_flag {
            unsafe { *(self.valptr_ as *mut usize) = new_value };
        }
        change_flag
    }

    pub fn set_string_value(&mut self, new_value: *const i8, owns_new_value: bool, set_by: SetBy) {
        if self.type_ != Flag::FlagType::TYPE_STRING {
            return;
        }
        let old_value = unsafe { *(self.valptr_ as *const *const i8) };
        let change_flag = unsafe {
            if old_value.is_null() {
                !new_value.is_null()
            } else if new_value.is_null() {
                true
            } else {
                CStr::from_ptr(old_value) != CStr::from_ptr(new_value)
            }
        };
        let change_flag = self.CheckFlagChange(set_by, change_flag, ptr::null());
        if change_flag {
            if self.owns_ptr_ {
                unsafe {
                    if !old_value.is_null(){
                      drop(CString::from_raw(old_value as *mut i8));
                    }
                }
            }
            unsafe { *(self.valptr_ as *mut *const i8) = new_value };
            self.owns_ptr_ = owns_new_value;
        } else {
             if owns_new_value {
                unsafe{
                    if !new_value.is_null(){
                       drop(CString::from_raw(new_value as *mut i8));
                    }
                }
            }
        }
    }

     fn CheckFlagChange(
        &mut self,
        new_set_by: SetBy,
        change_flag: bool,
        implied_by: *const i8,
    ) -> bool {
        if new_set_by == SetBy::kWeakImplication
            && (self.set_by_ == SetBy::kImplication || self.set_by_ == SetBy::kCommandLine)
        {
            return false;
        }
        if Self::ShouldCheckFlagContradictions() {
            static HINT: &str = "If a test variant caused this, it might be necessary to specify additional contradictory flags in tools/testrunner/local/variants.py.";
            // Readonly flags cannot change value.
            if change_flag && self.IsReadOnly() {
                // Exit instead of abort for certain testing situations.
                if unsafe { v8_flags.exit_on_contradictory_flags } {
                     std::process::exit(0);
                }
                if implied_by.is_null() {
                    panic!("Contradictory value for readonly flag {}.\n{}",FlagName { negated: false, name:self.name().to_string() }, HINT);
                } else {
                    if Self::IsAnyImplication(new_set_by) {
                         unsafe {
                             let implied_by_str = CStr::from_ptr(implied_by).to_str().unwrap();
                             panic!("Contradictory value for readonly flag {} implied by {}.\n{}",FlagName { negated: false, name:self.name().to_string() }, implied_by_str, HINT);
                         }
                    } else {
                         panic!("Is not implication");
                    }
                }
            }
            // For bool flags, we only check for a conflict if the value actually
            // changes. So specifying the same flag with the same value multiple times
            // is allowed.
            // For other flags, we disallow specifying them explicitly or in the
            // presence of an implication if the value is not the same.
            // This is to simplify the rules describing conflicts in variants.py: A
            // repeated non-boolean flag is considered an error.
            let is_bool_flag = self.type_ == Flag::FlagType::TYPE_MAYBE_BOOL
                || self.type_ == Flag::FlagType::TYPE_BOOL;
            let check_implications = change_flag;
            match self.set_by_ {
                SetBy::kDefault => {}
                SetBy::kWeakImplication => {
                    if new_set_by == SetBy::kWeakImplication && check_implications {
                        unsafe {
                            let implied_by_str = CStr::from_ptr(implied_by).to_str().unwrap();
                            if self.implied_by_.is_null() {
                                panic!("Self implied by is null");
                            }
                             let implied_by_self_str = CStr::from_ptr(self.implied_by_).to_str().unwrap();

                            panic!(
                                "Contradictory weak flag implications from {} and {} for flag {}.\n{}",
                                implied_by_self_str, implied_by_str,FlagName { negated: false, name:self.name().to_string() },
                                HINT
                            );
                        }
                    }
                }
                SetBy::kImplication => {
                    if new_set_by == SetBy::kImplication && check_implications {
                        unsafe {
                            let implied_by_str = CStr::from_ptr(implied_by).to_str().unwrap();
                            if self.implied_by_.is_null() {
                                panic!("Self implied by is null");
                            }
                             let implied_by_self_str = CStr::from_ptr(self.impl
