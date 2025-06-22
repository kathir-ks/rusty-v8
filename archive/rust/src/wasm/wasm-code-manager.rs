// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)] // Suppress warnings for unimplemented parts.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::sync::{atomic, Arc, Mutex, MutexGuard, RwLock};
use std::{
    collections::{HashMap, HashSet},
    fmt,
    mem,
    ops::{Deref, DerefMut},
    ptr,
    slice,
    sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicPtr, AtomicU32, Ordering},
};

use absl::flat_hash_map::FlatHashMap;

macro_rules! V8_EXPORT_PRIVATE {
    ($vis:vis struct $name:ident) => {
        $vis struct $name
        ;
    };
    ($vis:vis class $name:ident) => {
        $vis struct $name
        ;
    };
    ($vis:vis enum $name:ident) => {
        $vis enum $name
        ;
    };
    ($vis:vis trait $name:ident) => {
        $vis trait $name
        ;
    };
}

macro_rules! MOVE_ONLY_WITH_DEFAULT_CONSTRUCTORS {
    ($struct_name:ident) => {
        impl Default for $struct_name {
            fn default() -> Self {
                // Provide a default implementation. You might need to
                // implement Debug, Clone, etc., depending on what
                // `Default` is used for.
                Self::new()  // Replace with the correct constructor
            }
        }
    };
}

macro_rules! V8_WARN_UNUSED_RESULT {
    ($expression:expr) => {
        #[must_use]
        $expression
    };
}

macro_rules! CHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("Check failed: {}", stringify!($condition));
        }
    };
}

macro_rules! CHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("Check failed: {} == {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_LE {
    ($left:expr, $right:expr) => {
        if $left > $right {
            panic!("Check failed: {} <= {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("Check failed: {} == {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_LE {
    ($left:expr, $right:expr) => {
        if $left > $right {
            panic!("Check failed: {} <= {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_GT {
    ($left:expr, $right:expr) => {
        if $left <= $right {
            panic!("Check failed: {} > {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! V8_LIKELY {
    ($e:expr) => {
        $e
    };
}

macro_rules! V8_UNLIKELY {
    ($e:expr) => {
        $e
    };
}

pub mod absl {
    pub mod flat_hash_map {
        use std::collections::HashMap;

        pub type FlatHashMap<K, V> = HashMap<K, V>; // Replace with an actual implementation if necessary
    }
}

pub mod src {
    pub mod base {
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Range, RangeInclusive},
            ptr::NonNull,
        };
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct AddressRegion {
            start: usize,
            size: usize,
        }

        impl AddressRegion {
            pub fn new(start: usize, size: usize) -> Self {
                AddressRegion { start, size }
            }

            pub fn start(&self) -> usize {
                self.start
            }

            pub fn size(&self) -> usize {
                self.size
            }

            pub fn end(&self) -> usize {
                self.start + self.size
            }

            pub fn contains(&self, addr: usize) -> bool {
                addr >= self.start && addr < self.end()
            }

            pub fn is_empty(&self) -> bool {
                self.size == 0
            }

            pub fn intersects(&self, other: &AddressRegion) -> bool {
                !(self.end() <= other.start() || other.end() <= self.start())
            }

            pub fn merge(&self, other: &AddressRegion) -> Option<AddressRegion> {
                let start = usize::min(self.start(), other.start());
                let end = usize::max(self.end(), other.end());
                Some(AddressRegion {
                    start,
                    size: end - start,
                })
            }

            // Define a custom comparison function for AddressRegion
            pub struct StartAddressLess;

            impl StartAddressLess {
                pub fn compare(a: &AddressRegion, b: &AddressRegion) -> Ordering {
                    a.start.cmp(&b.start)
                }
            }
        }

        pub mod bit_field {
            pub struct BitField8<T, const START: u8, const LENGTH: u8>(std::marker::PhantomData<T>);

            impl<T, const START: u8, const LENGTH: u8> BitField8<T, START, LENGTH> {
                pub fn encode<U: Into<u8>>(value: U) -> u8 {
                    (value.into() & ((1 << LENGTH) - 1)) << START
                }

                pub fn decode(flags: u8) -> T {
                    unimplemented!() // Needs to return T based on the bits
                }

                pub fn Next<NewType, const NEW_LENGTH: u8>(self) -> BitField8<NewType, {START + LENGTH}, NEW_LENGTH> {
                    BitField8(std::marker::PhantomData)
                }
            }
        }

        pub mod macros {
            //empty
        }

        pub mod vector {
            use std::ops::{Deref, DerefMut};

            #[derive(Debug)]
            pub struct Vector<T> {
                data: Vec<T>,
            }

            impl<T> Vector<T> {
                pub fn new(data: Vec<T>) -> Self {
                    Vector { data }
                }

                pub fn empty() -> Self {
                    Vector { data: Vec::new() }
                }

                pub fn from_raw_parts(ptr: *mut T, length: usize) -> Self {
                    let data = unsafe { Vec::from_raw_parts(ptr, length, length) };
                    Vector { data }
                }

                pub fn as_slice(&self) -> &[T] {
                    &self.data
                }

                pub fn as_mut_slice(&mut self) -> &mut [T] {
                    &mut self.data
                }

                pub fn len(&self) -> usize {
                    self.data.len()
                }

                pub fn is_empty(&self) -> bool {
                    self.data.is_empty()
                }

                pub fn push(&mut self, value: T) {
                    self.data.push(value);
                }
            }

            impl<T> Deref for Vector<T> {
                type Target = Vec<T>;

                fn deref(&self) -> &Self::Target {
                    &self.data
                }
            }

            impl<T> DerefMut for Vector<T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.data
                }
            }

            impl<T> From<Vec<T>> for Vector<T> {
                fn from(vec: Vec<T>) -> Self {
                    Vector { data: vec }
                }
            }

            pub struct OwnedVector<T> {
                data: Vec<T>,
            }

            impl<T> OwnedVector<T> {
                pub fn new(data: Vec<T>) -> Self {
                    OwnedVector { data }
                }

                pub fn as_vector(&self) -> &Vector<T> {
                    // Convert &OwnedVector<T> to &Vector<T>
                    unsafe { std::mem::transmute(&self.data) } //This is very unsafe
                }

                pub fn empty() -> Self {
                    OwnedVector { data: Vec::new() }
                }

                pub fn is_empty(&self) -> bool {
                    self.data.is_empty()
                }
            }
        }
    }

    pub mod builtins {
        pub enum Builtin {
            kRecordWriteIgnoreFP,
            kRecordWriteSaveFP,
            kTSANRelaxedStore8IgnoreFP,
            kTSANRelaxedStore8SaveFP,
            kTSANRelaxedStore16IgnoreFP,
            kTSANRelaxedStore16SaveFP,
            kTSANRelaxedStore32IgnoreFP,
            kTSANRelaxedStore32SaveFP,
            kTSANRelaxedStore64IgnoreFP,
            kTSANRelaxedStore64SaveFP,
            kTSANSeqCstStore8IgnoreFP,
            kTSANSeqCstStore8SaveFP,
            kTSANSeqCstStore16IgnoreFP,
            kTSANSeqCstStore16SaveFP,
            kTSANSeqCstStore32IgnoreFP,
            kTSANSeqCstStore32SaveFP,
            kTSANSeqCstStore64IgnoreFP,
            kTSANSeqCstStore64SaveFP,
            kTSANRelaxedLoad32IgnoreFP,
            kTSANRelaxedLoad32SaveFP,
            kTSANRelaxedLoad64IgnoreFP,
            kTSANRelaxedLoad64SaveFP,
            kNoBuiltinId,
        }
    }

    pub mod codegen {
        pub struct SafepointTable {}
        pub struct SourcePosition {}
    }

    pub mod handles {
        pub struct Handles {}
        pub struct DirectHandle<T> {
            data: *mut T,
        }

        impl<T> DirectHandle<T> {
            pub fn new(data: *mut T) -> Self {
                DirectHandle { data }
            }
        }
    }

    pub mod tasks {
        pub struct OperationsBarrier {}
    }

    pub mod trap_handler {
        pub struct ProtectedInstructionData {}
    }

    pub mod wasm {
        use super::{
            super::base::{
                vector::{OwnedVector, Vector},
                AddressRegion,
            },
            builtins::Builtin,
            codegen::SourcePosition,
            handles::DirectHandle,
        };
        use std::collections::HashSet;
        use std::sync::{atomic, Arc, Mutex, MutexGuard, RwLock};
        use std::{
            collections::{HashMap, HashSet},
            fmt,
            mem,
            ops::{Deref, DerefMut},
            ptr,
            slice,
            sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicPtr, AtomicU32, Ordering},
        };

        pub struct AssumptionsJournal {}
        pub struct DebugInfo {}
        pub struct NamesProvider {}
        pub struct NativeModule {}
        pub struct WasmCompilationResult {}
        pub struct WasmEngine {}
        pub struct WasmImportWrapperCache {}
        pub struct WasmModule {}
        pub enum WellKnownImport {
            kUnknown,
        }

        pub type Address = usize;
        pub const kNullAddress: Address = 0;

        // Sorted, disjoint and non-overlapping memory regions. A region is of the
        // form [start, end). So there's no [start, end), [end, other_end),
        // because that should have been reduced to [start, other_end).
        #[derive(Default)]
        pub struct DisjointAllocationPool {
            regions_: Mutex<std::collections::BTreeSet<AddressRegion>>,
        }

        impl DisjointAllocationPool {
            pub fn new(region: AddressRegion) -> Self {
                let mut regions_ = std::collections::BTreeSet::new();
                regions_.insert(region);
                DisjointAllocationPool {
                    regions_: Mutex::new(regions_),
                }
            }

            // Merge the parameter region into this object. The assumption is that the
            // passed parameter is not intersecting this object - for example, it was
            // obtained from a previous Allocate. Returns the merged region.
            pub fn merge(&self, region: AddressRegion) -> AddressRegion {
                let mut regions = self.regions_.lock().unwrap();
                regions.insert(region);
                //Todo: Merge logic
                region
            }

            // Allocate a contiguous region of size {size}. Return an empty region on
            // failure.
            pub fn allocate(&self, size: usize) -> AddressRegion {
                let mut regions = self.regions_.lock().unwrap();
                if let Some(region) = regions.iter().next() {
                    let allocated_region = AddressRegion::new(region.start(), size);
                    if allocated_region.size() > 0 {
                        return allocated_region;
                    }
                }
                AddressRegion::new(0, 0) // Empty region on failure
            }

            // Allocate a contiguous region of size {size} within {region}. Return an
            // empty region on failure.
            pub fn allocate_in_region(&self, size: usize, region: AddressRegion) -> AddressRegion {
                if size <= region.size() {
                    AddressRegion::new(region.start(), size)
                } else {
                    AddressRegion::new(0, 0) // Empty region on failure
                }
            }

            pub fn is_empty(&self) -> bool {
                let regions = self.regions_.lock().unwrap();
                regions.is_empty()
            }

            pub fn regions(&self) -> Vec<AddressRegion> {
                let regions = self.regions_.lock().unwrap();
                regions.iter().cloned().collect()
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct WasmCodePointer {
            handle: usize,
        }

        pub mod wasm_code_pointer_table {
            pub const kInvalidHandle: usize = 0;
        }

        pub const kInvalidWasmCodePointer: WasmCodePointer =
            WasmCodePointer {
                handle: wasm_code_pointer_table::kInvalidHandle,
            };

        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        pub enum Kind {
            kWasmFunction,
            kWasmToCapiWrapper,
            kWasmToJsWrapper,
            kJumpTable,
        }

        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        pub enum ExecutionTier {
            kNone,
            kLiftoff,
            kTurbofan,
        }

        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        pub enum SaveFPRegsMode {
            kIgnore,
            kSave,
        }

        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        pub enum ForDebugging {
            kDisabled,
            kEnabled,
        }

        pub const kInt8Size: i32 = 1;
        pub const kInt16Size: i32 = 2;
        pub const kInt32Size: i32 = 4;
        pub const kInt64Size: i32 = 8;

        #[derive(Debug)]
        pub struct WasmCode {
            native_module_: *const NativeModule, //Raw pointer
            instructions_: *mut u8,               //Raw pointer
            signature_hash_: u64,
            meta_data_: Arc<Vec<u8>>,
            instructions_size_: i32,
            reloc_info_size_: i32,
            source_positions_size_: i32,
            inlining_positions_size_: i32,
            deopt_data_size_: i32,
            protected_instructions_size_: i32,
            index_: i32,
            constant_pool_offset_: i32,
            stack_slots_: i32,
            ool_spills_: i32,
            tagged_parameter_slots_: u32,
            safepoint_table_offset_: i32,
            handler_table_offset_: i32,
            code_comments_offset_: i32,
            unpadded_binary_size_: i32,
            trap_handler_index_: i32,
            flags_: u8,
            dying_: AtomicBool,
            ref_count_: AtomicI32,
        }

        impl WasmCode {
            pub fn get_record_write_builtin(fp_mode: SaveFPRegsMode) -> Builtin {
                match fp_mode {
                    SaveFPRegsMode::kIgnore => Builtin::kRecordWriteIgnoreFP,
                    SaveFPRegsMode::kSave => Builtin::kRecordWriteSaveFP,
                }
            }

            //Todo: Implement TSAN builtins
            /*#[cfg(V8_IS_TSAN)]
            pub fn get_tsan_store_builtin(fp_mode: SaveFPRegsMode, size: i32, order: std::memory_order) -> Builtin {
                if order == std::memory_order::Relaxed {
                    if size == kInt8Size {
                        return if fp_mode == SaveFPRegsMode::kIgnore {
                            Builtin::kTSANRelaxedStore8IgnoreFP
                        } else {
                            Builtin::kTSANRelaxedStore8SaveFP
                        };
                    } else if size == kInt16Size {
                        return if fp_mode == SaveFPRegsMode::kIgnore {
                            Builtin::kTSANRelaxedStore16IgnoreFP
                        } else {
                            Builtin::kTSANRelaxedStore16SaveFP
                        };
                    } else if size == kInt32Size {
                        return if fp_mode == SaveFPRegsMode::kIgnore {
                            Builtin::kTSANRelaxedStore32IgnoreFP
                        } else {
                            Builtin::kTSANRelaxedStore32SaveFP
                        };
                    } else {
                        CHECK_EQ(size, kInt64Size);
                        return if fp_mode == SaveFPRegsMode::kIgnore {
                            Builtin::kTSANRelaxedStore64IgnoreFP
                        } else {
                            Builtin::kTSANRelaxedStore64SaveFP
                        };
                    }
                } else {
                    DCHECK_EQ(order, std::memory_order::SeqCst);
                    if size == kInt8Size {
                        return if fp_mode == SaveFPRegsMode::kIgnore {
                            Builtin::kTSANSeqCstStore8IgnoreFP
                        } else {
                            Builtin::kTSANSeqCstStore8SaveFP
                        };
                    } else if size == kInt16Size {
                        return if fp_mode == SaveFPRegsMode::kIgnore {
                            Builtin::kTSANSeqCstStore16IgnoreFP
                        } else {
                            Builtin::kTSANSeqCstStore16SaveFP
                        };
                    } else if size == kInt32Size {
                        return if fp_mode == SaveFPRegsMode::kIgnore {
                            Builtin::kTSANSeqCstStore32IgnoreFP
                        } else {
                            Builtin::kTSANSeqCstStore32SaveFP
                        };
                    } else {
                        CHECK_EQ(size, kInt64Size);
                        return if fp_mode == SaveFPRegsMode::kIgnore {
                            Builtin::kTSANSeqCstStore64IgnoreFP
                        } else {
                            Builtin::kTSANSeqCstStore64SaveFP
                        };
                    }
                }
            }

            pub fn get_tsan_relaxed_load_builtin(fp_mode: SaveFPRegsMode, size: i32) -> Builtin {
                if size == kInt32Size {
                    return if fp_mode == SaveFPRegsMode::kIgnore {
                        Builtin::kTSANRelaxedLoad32IgnoreFP
                    } else {
                        Builtin::kTSANRelaxedLoad32SaveFP
                    };
                } else {
                    CHECK_EQ(size, kInt64Size);
                    return if fp_mode == SaveFPRegsMode::kIgnore {
                        Builtin::kTSANRelaxedLoad64IgnoreFP
                    } else {
                        Builtin::kTSANRelaxedLoad64SaveFP
                    };
                }
            }*/

            pub fn instructions(&self) -> Vector<u8> {
                unsafe {
                    Vector::from_raw_parts(
                        self.instructions_,
                        self.instructions_size_ as usize,
                    )
                }
            }
            pub fn instruction_start(&self) -> Address {
                self.instructions_ as Address
            }
            pub fn instructions_size(&self) -> usize {
                self.instructions_size_ as usize
            }
            pub fn reloc_info(&self) -> Vector<u8> {
                let start = self.protected_instructions_data().len();
                let end = start + self.reloc_info_size_ as usize;
                Vector::new(self.meta_data_.as_slice()[start..end].to_vec())
            }
            pub fn source_positions(&self) -> Vector<u8> {
                let start = self.reloc_info().len();
                let end = start + self.source_positions_size_ as usize;
                Vector::new(self.meta_data_.as_slice()[start..end].to_vec())
            }
            pub fn inlining_positions(&self) -> Vector<u8> {
                let start = self.source_positions().len();
                let end = start + self.inlining_positions_size_ as usize;
                Vector::new(self.meta_data_.as_slice()[start..end].to_vec())
            }
            pub fn deopt_data(&self) -> Vector<u8> {
                let start = self.inlining_positions().len();
                let end = start + self.deopt_data_size_ as usize;
                Vector::new(self.meta_data_.as_slice()[start..end].to_vec())
            }

            pub fn index(&self) -> i32 {
                self.index_
            }
            // Anonymous functions are functions that don't carry an index.
            pub fn is_anonymous(&self) -> bool {
                self.index_ == kAnonymousFuncIndex
            }
            pub fn kind(&self) -> Kind {
                KindField::decode(self.flags_)
            }
            pub fn native_module(&self) -> *const NativeModule {
                self.native_module_
            }
            pub fn tier(&self) -> ExecutionTier {
                ExecutionTierField::decode(self.flags_)
            }
            pub fn constant_pool(&self) -> Address {
                unimplemented!()
            }
            pub fn handler_table(&self) -> Address {
                unimplemented!()
            }
            pub fn handler_table_size(&self) -> i32 {
                unimplemented!()
            }
            pub fn code_comments(&self) -> Address {
                unimplemented!()
            }
            pub fn code_comments_size(&self) -> i32 {
                unimplemented!()
            }
            pub fn constant_pool_offset(&self) -> i32 {
                self.constant_pool_offset_
            }
            pub fn safepoint_table_offset(&self) -> i32 {
                self.safepoint_table_offset_
            }
            pub fn handler_table_offset(&self) -> i32 {
                self.handler_table_offset_
            }
            pub fn code_comments_offset(&self) -> i32 {
                self.code_comments_offset_
            }
            pub fn unpadded_binary_size(&self) -> i32 {
                self.unpadded_binary_size_
            }
            pub fn stack_slots(&self) -> i32 {
                self.stack_slots_
            }
            pub fn ool_spills(&self) -> i32 {
                self.ool_spills_
            }
            pub fn signature_hash(&self) -> u64 {
                self.signature_hash_
            }
            pub fn first_tagged_parameter_slot(&self) -> u16 {
                (self.tagged_parameter_slots_ >> 16) as u16
            }
            pub fn num_tagged_parameter_slots(&self) -> u16 {
                (self.tagged_parameter_slots_ & 0xFFFF) as u16
            }
            pub fn raw_tagged_parameter_slots_for_serialization(&self) -> u32 {
                self.tagged_parameter_slots_
            }

            pub fn is_liftoff(&self) -> bool {
                self.tier() == ExecutionTier::kLiftoff
            }

            pub fn is_turbofan(&self) -> bool {
                self.tier() == ExecutionTier::kTurbofan
            }

            pub fn contains(&self, pc: Address) -> bool {
                self.instruction_start() <= pc
                    && pc < self.instruction_start() + self.instructions_size()
            }

            // Only Liftoff code that was generated for debugging can be inspected
            // (otherwise debug side table positions would not match up).
            pub fn is_inspectable(&self) -> bool {
                self.is_liftoff() && self.for_debugging() == ForDebugging::kEnabled
            }

            pub fn protected_instructions_data(&self) -> Vector<u8> {
                Vector::new(self.meta_data_.as_slice()[0..self.protected_instructions_size_ as usize].to_vec())
            }

            pub fn protected_instructions(&self) -> Vector<trap_handler::ProtectedInstructionData> {
                //This is unsafe, but necessary
                unsafe {
                    let slice = slice::from_raw_parts(
                        self.protected_instructions_data().as_ptr()
                            as *const trap_handler::ProtectedInstructionData,
                        self.protected_instructions_size_ as usize,
                    );
                    Vector::new(slice.to_vec())
                }
            }

            pub fn is_protected_instruction(&self, pc: Address) -> bool {
                unimplemented!()
            }

            pub fn validate(&self) {
                unimplemented!()
            }
            pub fn print(&self, name: Option<&str>) {
                unimplemented!()
            }
            pub fn maybe_print(&self) {
                unimplemented!()
            }
            pub fn disassemble(&self, name: &str, os: &mut dyn std::io::Write, current_pc: Address) {
                unimplemented!()
            }

            pub fn should_be_logged(isolate: *const ()) -> bool {
                unimplemented!()
            }
            pub fn log_code(&self, isolate: *const (), source_url: &str, script_id: i32) {
                unimplemented!()
            }

            //WasmCode(const WasmCode&) = delete;
            //WasmCode& operator=(const WasmCode&) = delete;
            //~WasmCode();

            pub fn inc_ref(&self) {
                let old_val = self.ref_count_.fetch_add(1, Ordering::AcqRel);
                DCHECK_LE(1, old_val);
                DCHECK_GT(i32::MAX, old_val);
            }

            // Decrement the ref count. Returns whether this code becomes dead and needs
            // to be freed.
            pub fn dec_ref(&self) -> bool {
                let mut old_count = self.ref_count_.load(Ordering::Acquire);
                loop {
                    DCHECK_LE(1, old_count);
                    if V8_UNLIKELY(old_count == 1) {
                        if self.is_dying() {
                            // The code was already on the path to deletion, only temporary
                            // C++ references to it are left. Decrement the refcount, and
                            // return true if it drops to zero.
                            return self.dec_ref_on_dead_code();
                        }
                        // Otherwise, the code enters the path to destruction now.
                        self.mark_as_dying();
                        old_count = self.ref_count_.load(Ordering::Acquire);
                        if V8_LIKELY(old_count == 1) {
                            // No other thread got in the way. Commit to the decision.
                            self.dec_ref_on_potentially_dead_code();
                            return false;
                        }
                        // Another thread managed to increment the refcount again, just
                        // before we set the "dying" bit. So undo that, and resume the
                        // loop to evaluate again what needs to be done.
                        self.undo_mark_as_dying();
                    }
                    match self.ref_count_.compare_exchange_weak(
                        old_count,
                        old_count - 1,
                        Ordering::AcqRel,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => return false,
                        Err(current) => old_count = current,
                    }
                }
            }

            // Decrement the ref count on code that is known to be in use (i.e. the ref
            // count cannot drop to zero here).
            pub fn dec_ref_on_live_code(&self) {
                let old_count = self.ref_count_.fetch_sub(1, Ordering::AcqRel);
                DCHECK_LE(2, old_count);
            }

            // Decrement the ref count on code that is known to be dead, even though there
            // might still be C++ references. Returns whether this drops the last
            // reference and the code needs to be freed.
            pub fn dec_ref_on_dead_code(&self) -> bool {
                self.ref_count_.fetch_sub(1, Ordering::AcqRel) == 1
            }

            // Decrement the ref count on a set of {WasmCode} objects, potentially
            // belonging to different {NativeModule}s. Dead code will be deleted.
            pub fn decrement_ref_count(code_objects: Vector<&WasmCode>) {
                unimplemented!()
            }

            // Called by the WasmEngine when it shuts down for code it thinks is
            // probably dead (i.e. is in the "potentially_dead_code_" set). Wrapped
            // in a method only because {ref_count_} is private.
            pub fn dcheck_ref_count_is_one(&self) {
                DCHECK_EQ(1, self.ref_count_.load(Ordering::Acquire));
            }

            // Returns the last source position before {offset}.
            pub fn get_source_position_before(&self, code_offset: i32) -> SourcePosition {
                unimplemented!()
            }
            pub fn get_source_offset_before(&self, code_offset: i32) -> i32 {
                unimplemented!()
            }

            pub fn get_inlining_position(&self, inlining_id: i32) -> (i32, bool, SourcePosition) {
                unimplemented!()
            }

            // Returns whether this code was generated for debugging. If this returns
            // {kForDebugging}, but {tier()} is not {kLiftoff}, then Liftoff compilation
            // bailed out.
            pub fn for_debugging(&self) -> ForDebugging {
                ForDebuggingField::decode(self.flags_)
            }

            pub fn is_dying(&self) -> bool {
                self.dying_.load(Ordering::Acquire)
            }

            // Returns {true} for Liftoff code that sets up a feedback vector slot in its
            // stack frame.
            // TODO(jkummerow): This can be dropped when we ship Wasm inlining.
            pub fn frame_has_feedback_slot(&self) -> bool {
                FrameHasFeedbackSlotField::decode(self.flags_)
            }

            pub enum FlushICache {
                kFlushICache,
                kNoFlushICache,
            }

            pub fn estimate_current_memory_consumption(&self) -> usize {
                unimplemented!()
            }

            // Tries to get a reasonable name. Lazily looks up the name section, and falls
            // back to the function index. Return value is guaranteed to not be empty.
            pub fn debug_name(&self) -> String {
                unimplemented!()
            }

            // Private methods

            fn new(
                native_module: *const NativeModule,
                index: i32,
                instructions: Vector<u8>,
                stack_slots: i32,
                ool_spills: i32,
                tagged_parameter_slots: u32,
                safepoint_table_offset: i32,
                handler_table_offset: i32,
                constant_pool_offset: i32,
                code_comments_offset: i32,
                unpadded_binary_size: i32,
                protected_instructions_data: Vector<u8>,
                reloc_info: Vector<u8>,
                source_position_table: Vector<u8>,
                inlining_positions: Vector<u8>,
                deopt_data: Vector<u8>,
                kind: Kind,
                tier: ExecutionTier,
                for_debugging: ForDebugging,
                signature_hash: u64,
                frame_has_feedback_slot: bool,
            ) -> Self {
                DCHECK_LE(safepoint_table_offset, unpadded_binary_size);
                DCHECK_LE(handler_table_offset,