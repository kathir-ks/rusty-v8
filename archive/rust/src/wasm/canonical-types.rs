// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)] // Suppress warnings about unused code
#![allow(clippy::too_many_arguments)] // Suppress warnings about too many arguments
#![allow(clippy::new_without_default)] // Suppress warnings about new without default
#![allow(clippy::result_unit_arg)] // Suppress warnings about returning Result<()>
#![allow(clippy::unused_unit)] // Suppress warnings about unused unit

use std::any::Any;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};

//use crate::base::bounds::IsInRange; // Assuming a similar function exists
//use crate::base::hashing::Hasher; // Assuming a similar struct exists
//use crate::wasm::value_type::ValueType; // Assuming this struct is defined elsewhere
//use crate::wasm::wasm_module::WasmModule; // Assuming this struct is defined elsewhere

// Define necessary constants.  The actual values would be derived from the
// surrounding V8 context, but we'll define them here for completeness.
const KV8_MAX_WASM_TYPES: usize = 1024;
const KINVALID_CANONICAL_INDEX: u32 = KV8_MAX_WASM_TYPES as u32 + 1;

// TODO: Figure out a suitable value based on the original V8 code.
const K_NO_SUPER_TYPE: CanonicalTypeIndex = CanonicalTypeIndex { index: KINVALID_CANONICAL_INDEX };

// Define a trait to represent value types.
trait ValueTypeBase: Any + Send + Sync {}

// A dummy implementation for now
impl ValueTypeBase for i32 {}
impl ValueTypeBase for f32 {}
impl ValueTypeBase for f64 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanonicalTypeIndex {
    pub index: u32,
}

impl CanonicalTypeIndex {
  pub const MAX: CanonicalTypeIndex = CanonicalTypeIndex { index: u32::MAX };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CanonicalValueType {
    kind: u8, // Representing the kind of value
    index: u32,
    has_index_flag: bool,
}

impl CanonicalValueType {
  const K_NUM_INDEX_BITS: u32 = 20;

  fn has_index(&self) -> bool {
    self.has_index_flag
  }

  fn ref_index(&self) -> CanonicalTypeIndex {
    CanonicalTypeIndex { index: self.index }
  }

  fn kind(&self) -> u8 {
    self.kind
  }
}

// Dummy data for now, replace with your actual implementations.
struct FunctionSig {
    params: Vec<CanonicalValueType>,
    returns: Vec<CanonicalValueType>,
}

impl FunctionSig {
    fn parameter_count(&self) -> usize {
        self.params.len()
    }

    fn all(&self) -> Vec<CanonicalValueType> {
        let mut all = self.params.clone();
        all.extend(self.returns.clone());
        all
    }
}

struct CanonicalStructType {
    fields: Vec<Box<dyn ValueTypeBase>>,
    mutabilities: Vec<u8>, // Represents mutability of each field
}

impl CanonicalStructType {
  fn fields(&self) -> &Vec<Box<dyn ValueTypeBase>> {
      &self.fields
  }

  fn mutabilities(&self) -> &Vec<u8> {
      &self.mutabilities
  }
}

struct CanonicalArrayType {
    element_type: CanonicalValueType,
    mutability: u8,
}

impl CanonicalArrayType {
  fn element_type(&self) -> CanonicalValueType {
    self.element_type
  }

  fn mutability(&self) -> u8 {
    self.mutability
  }
}

struct CanonicalContType {
    contfun_typeindex: CanonicalTypeIndex,
}

impl CanonicalContType {
  fn contfun_typeindex(&self) -> CanonicalTypeIndex {
    self.contfun_typeindex
  }
}

struct WasmModule {
    isorecursive_canonical_type_ids: Vec<CanonicalTypeIndex>,
}

impl WasmModule {
  fn new() -> Self {
    WasmModule {
      isorecursive_canonical_type_ids: Vec::new()
    }
  }
}

type ModuleTypeIndex = u32;

// The isolate that's used in v8
struct Isolate {}

// Dummy implementation for Isolate
impl Isolate {
  fn new() -> Self {
    Isolate {}
  }
}

// Forward declaration of AccountingAllocator.
struct AccountingAllocator {}

impl AccountingAllocator {
    fn new() -> Self {
        AccountingAllocator {}
    }
}

struct Zone<'a> {
    allocator: &'a AccountingAllocator,
    name: &'static str,
}

impl<'a> Zone<'a> {
    fn new(allocator: &'a AccountingAllocator, name: &'static str) -> Self {
        Zone {
            allocator,
            name,
        }
    }

    fn allocate_vector<T>(&self, size: usize) -> base::Vector<T> {
        base::Vector::with_capacity(size)
    }

    fn new<T>(&self) -> Box<T> where T: Default {
        Box::<T>::new(T::default())
    }
}

mod base {
  pub mod bounds {
    pub fn is_in_range<T: PartialOrd>(value: T, lower: T, upper: T) -> bool {
      value >= lower && value <= upper
    }
  }

  pub mod hashing {
    pub struct Hasher {
      state: u64,
    }

    impl Hasher {
      pub fn new() -> Self {
        Hasher { state: 0 }
      }

      pub fn add<T: Hash>(&mut self, value: T) {
        let mut s = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut s);
        self.state = self.state.wrapping_add(s.finish());
      }

      pub fn add_range<T: Copy + Hash>(&mut self, values: &[T]) {
        for &value in values {
          self.add(value);
        }
      }

      pub fn hash(&self) -> usize {
        self.state as usize
      }
    }
  }

  #[derive(Debug, Clone)]
  pub struct Vector<T> {
    data: Vec<T>
  }

  impl<T> Vector<T> {
      pub fn with_capacity(capacity: usize) -> Self {
          Vector { data: Vec::with_capacity(capacity) }
      }

      pub fn push(&mut self, value: T) {
          self.data.push(value);
      }

      pub fn get(&self, index: usize) -> Option<&T> {
          self.data.get(index)
      }

      pub fn len(&self) -> usize {
          self.data.len()
      }

      pub fn begin(&self) -> std::slice::Iter<T> {
        self.data.iter()
      }

      pub fn end(&self) -> std::slice::Iter<T> {
        self.data.iter()
      }

      pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
      }

      pub fn size(&self) -> usize {
        self.data.len()
      }
  }

  impl<'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
  }
}

/// A singleton class, responsible for isorecursive canonicalization of wasm
/// types.
/// A recursive group is a subsequence of types explicitly marked in the type
/// section of a wasm module. Identical recursive groups have to be canonicalized
/// to a single canonical group. Respective types in two identical groups are
/// considered identical for all purposes.
/// Two groups are considered identical if they have the same shape, and all
/// type indices referenced in the same position in both groups reference:
/// - identical types, if those do not belong to the rec. group,
/// - types in the same relative position in the group, if those belong to the
///   rec. group.
pub struct TypeCanonicalizer {
    canonical_supertypes: Mutex<Vec<CanonicalTypeIndex>>,
    canonical_groups: Mutex<HashSet<CanonicalGroup>>,
    canonical_singleton_groups: Mutex<HashSet<CanonicalSingletonGroup>>,
    canonical_types: CanonicalTypeVector,
    allocator: AccountingAllocator,
    zone: RefCell<Zone<'static>>,
    mutex: Mutex<()>,
}

impl TypeCanonicalizer {
    pub const K_PREDEFINED_ARRAY_I8_INDEX: CanonicalTypeIndex = CanonicalTypeIndex { index: 0 };
    pub const K_PREDEFINED_ARRAY_I16_INDEX: CanonicalTypeIndex = CanonicalTypeIndex { index: 1 };
    pub const K_NUMBER_OF_PREDEFINED_TYPES: u32 = 2;

    pub fn new() -> Self {
        let allocator = AccountingAllocator::new();
        let zone = Zone::new(&allocator, "canonical type zone");

        let canonicalizer = TypeCanonicalizer {
            canonical_supertypes: Mutex::new(Vec::new()),
            canonical_groups: Mutex::new(HashSet::new()),
            canonical_singleton_groups: Mutex::new(HashSet::new()),
            canonical_types: CanonicalTypeVector::new(),
            allocator,
            zone: RefCell::new(zone),
            mutex: Mutex::new(()),
        };

        canonicalizer.add_predefined_array_types();
        canonicalizer
    }

    /// Register the last {size} types of {module} as a recursive group and
    /// possibly canonicalize it if an identical one has been found.
    /// Modifies {module->isorecursive_canonical_type_ids}.
    pub fn add_recursive_group(&self, module: &mut WasmModule, size: u32) {
        let _lock = self.mutex.lock().unwrap(); // Acquire lock

        let module_types_len = module.isorecursive_canonical_type_ids.len() as u32;
        let recgroup_start = module_types_len - size;

        let group = {
            let mut zone = self.zone.borrow_mut();
            CanonicalGroup::new(&mut zone, size as usize, CanonicalTypeIndex {index: recgroup_start})
        };

        let group_index = self.find_canonical_group(group);
        if group_index.index == KINVALID_CANONICAL_INDEX {
            // Add the group if not found
            self.add_new_canonical_group(group, module, recgroup_start, size);
        } else {
            // If found, use the existing canonical index
            for i in 0..size {
                module.isorecursive_canonical_type_ids[(recgroup_start + i) as usize] = CanonicalTypeIndex { index: group_index.index + i };
            }
        }
    }

    /// Same as above, but for a group of size 1 (using the last type in the
    /// module).
    pub fn add_recursive_singleton_group(&self, module: &mut WasmModule) {
        let _lock = self.mutex.lock().unwrap(); // Acquire lock
        let module_types_len = module.isorecursive_canonical_type_ids.len() as u32;
        let index = module_types_len - 1;

        let singleton_group = {
            let mut zone = self.zone.borrow_mut();
            CanonicalSingletonGroup::new(&mut zone, CanonicalTypeIndex { index })
        };

        let group_index = self.find_canonical_group_singleton(singleton_group);
        if group_index.index == KINVALID_CANONICAL_INDEX {
            // Add the group if not found
            self.add_new_canonical_group_singleton(singleton_group, module, index);
        } else {
            // If found, use the existing canonical index
            module.isorecursive_canonical_type_ids[index as usize] = group_index;
        }
    }

    /// Adds a module-independent signature as a recursive group, and canonicalizes
    /// it if an identical is found. Returns the canonical index of the added
    /// signature.
    pub fn add_recursive_group_sig(&self, sig: &FunctionSig) -> CanonicalTypeIndex {
        let _lock = self.mutex.lock().unwrap(); // Acquire lock

        let singleton_group = {
            let mut zone = self.zone.borrow_mut();
            CanonicalSingletonGroup::new_sig(&mut zone, sig)
        };

        let group_index = self.find_canonical_group_singleton(singleton_group);
        if group_index.index == KINVALID_CANONICAL_INDEX {
            // Add the group if not found
            self.add_new_canonical_group_singleton_sig(singleton_group);
            self.canonical_types.FindIndex_Slow(sig)
        } else {
            // If found, use the existing canonical index
            group_index
        }
    }

    /// Retrieve back a type from a canonical index later.
    pub fn lookup_function_signature(&self, index: CanonicalTypeIndex) -> Option<&FunctionSig> {
        let _lock = self.mutex.lock().unwrap(); // Acquire lock
        let canonical_type = self.canonical_types[index];
        if canonical_type.kind == CanonicalType::Kind::Function {
          Some(canonical_type.function_sig)
        } else {
          None
        }
    }

    pub fn lookup_struct(&self, index: CanonicalTypeIndex) -> Option<&CanonicalStructType> {
        let _lock = self.mutex.lock().unwrap(); // Acquire lock
        let canonical_type = self.canonical_types[index];
        if canonical_type.kind == CanonicalType::Kind::Struct {
          Some(canonical_type.struct_type)
        } else {
          None
        }
    }

    pub fn lookup_array(&self, index: CanonicalTypeIndex) -> Option<&CanonicalArrayType> {
      let _lock = self.mutex.lock().unwrap(); // Acquire lock
      let canonical_type = self.canonical_types[index];
      if canonical_type.kind == CanonicalType::Kind::Array {
        Some(canonical_type.array_type)
      } else {
        None
      }
    }

    /// Returns if {canonical_sub_index} is a canonical subtype of
    /// {canonical_super_index}.
    pub fn is_canonical_subtype(&self, sub_index: CanonicalTypeIndex, super_index: CanonicalTypeIndex) -> bool {
        let _lock = self.mutex.lock().unwrap(); // Acquire lock
        self.is_canonical_subtype_locked(sub_index, super_index)
    }

    /// Returns if the type at {sub_index} in {sub_module} is a subtype of the
    /// type at {super_index} in {super_module} after canonicalization.
    pub fn is_canonical_subtype_modules(
        &self,
        sub_index: ModuleTypeIndex,
        super_index: ModuleTypeIndex,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        let sub_canonical_index = sub_module.isorecursive_canonical_type_ids[sub_index as usize];
        let super_canonical_index = super_module.isorecursive_canonical_type_ids[super_index as usize];
        self.is_canonical_subtype(sub_canonical_index, super_canonical_index)
    }

    /// Deletes recursive groups. Used by fuzzers to avoid accumulating memory, and
    /// used by specific tests e.g. for serialization / deserialization.
    pub fn empty_storage_for_testing(&self) {
        let mut canonical_groups = self.canonical_groups.lock().unwrap();
        canonical_groups.clear();
        let mut canonical_singleton_groups = self.canonical_singleton_groups.lock().unwrap();
        canonical_singleton_groups.clear();
        self.canonical_types.ClearForTesting();
    }

    pub fn estimate_current_memory_consumption(&self) -> usize {
        let canonical_groups = self.canonical_groups.lock().unwrap();
        let canonical_singleton_groups = self.canonical_singleton_groups.lock().unwrap();

        canonical_groups.capacity() * std::mem::size_of::<CanonicalGroup>() +
            canonical_singleton_groups.capacity() * std::mem::size_of::<CanonicalSingletonGroup>()
    }

    pub fn get_current_number_of_types(&self) -> usize {
        // TODO: add lock here or use atomics.
        //self.canonical_types.segments.iter().filter(|&s| s.load(Ordering::Relaxed).is_some()).count()
        0 // Dummy implementation.
    }

    /// Prepares wasm for the provided canonical type index. This reserves enough
    /// space in the canonical rtts and the JSToWasm wrappers on the isolate roots.
    pub fn prepare_for_canonical_type_id(isolate: &mut Isolate, id: CanonicalTypeIndex) {
        // Placeholder - actual implementation would interact with Isolate.
        println!("Preparing isolate {:?} for canonical type id {:?}", isolate, id);
    }

    /// Reset the canonical rtts and JSToWasm wrappers on the isolate roots for
    /// testing purposes (in production cases canonical type ids are never freed).
    pub fn clear_wasm_canonical_types_for_testing(isolate: &mut Isolate) {
        // Placeholder - actual implementation would interact with Isolate.
        println!("Clearing wasm canonical types for isolate {:?}", isolate);
    }

    pub fn is_function_signature(&self, index: CanonicalTypeIndex) -> bool {
        let _lock = self.mutex.lock().unwrap(); // Acquire lock
        let canonical_type = self.canonical_types[index];
        canonical_type.kind == CanonicalType::Kind::Function
    }

    pub fn is_struct(&self, index: CanonicalTypeIndex) -> bool {
        let _lock = self.mutex.lock().unwrap(); // Acquire lock
        let canonical_type = self.canonical_types[index];
        canonical_type.kind == CanonicalType::Kind::Struct
    }

    pub fn is_array(&self, index: CanonicalTypeIndex) -> bool {
      let _lock = self.mutex.lock().unwrap(); // Acquire lock
      let canonical_type = self.canonical_types[index];
      canonical_type.kind == CanonicalType::Kind::Array
    }

    fn add_predefined_array_types(&self) {
      // TODO: fill with the real value.
    }

    fn find_canonical_group(&self, group: CanonicalGroup) -> CanonicalTypeIndex {
        let canonical_groups = self.canonical_groups.lock().unwrap();
        for g in canonical_groups.iter() {
            if *g == group {
                return g.first;
            }
        }
        CanonicalTypeIndex { index: KINVALID_CANONICAL_INDEX }
    }

    fn find_canonical_group_singleton(&self, group: CanonicalSingletonGroup) -> CanonicalTypeIndex {
        let canonical_singleton_groups = self.canonical_singleton_groups.lock().unwrap();
        for g in canonical_singleton_groups.iter() {
            if *g == group {
                return g.index;
            }
        }
        CanonicalTypeIndex { index: KINVALID_CANONICAL_INDEX }
    }

    fn add_new_canonical_group(&self, group: CanonicalGroup, module: &mut WasmModule, recgroup_start: u32, size: u32) {
        let mut canonical_groups = self.canonical_groups.lock().unwrap();
        let mut canonical_supertypes = self.canonical_supertypes.lock().unwrap();
        let group_first_index = {
          let mut zone = self.zone.borrow_mut();
          let new_index = canonical_supertypes.len() as u32;

          let size_usize = size as usize;
          self.canonical_types.reserve(new_index as u32 + size, &mut zone);

          for i in 0..size_usize {
            let module_type_idx = recgroup_start + i as u32;

            let canonical_type = self.canonicalize_type_def(module, module_type_idx, recgroup_start, CanonicalTypeIndex {index: new_index + i as u32});

            let canonical_index = CanonicalTypeIndex { index: new_index + i as u32 };
            let type_ptr: *const CanonicalType = Box::leak(Box::new(canonical_type));
            self.canonical_types.set(canonical_index, type_ptr);
            module.isorecursive_canonical_type_ids[module_type_idx as usize] = canonical_index;
          }

          new_index
        };

        canonical_groups.insert(group);
        drop(canonical_groups);

        for i in 0..size {
            let index = (recgroup_start + i) as usize;
            module.isorecursive_canonical_type_ids[index] = CanonicalTypeIndex { index: group_first_index + i };
        }
    }

    fn add_new_canonical_group_singleton(&self, group: CanonicalSingletonGroup, module: &mut WasmModule, index: u32) {
        let mut canonical_singleton_groups = self.canonical_singleton_groups.lock().unwrap();
        let mut canonical_supertypes = self.canonical_supertypes.lock().unwrap();

        let group_index = {
          let mut zone = self.zone.borrow_mut();
          let new_index = canonical_supertypes.len() as u32;
          self.canonical_types.reserve(new_index + 1, &mut zone);

          let type_ptr: *const CanonicalType = Box::leak(Box::new(group.type_));
          self.canonical_types.set(CanonicalTypeIndex {index: new_index}, type_ptr);

          module.isorecursive_canonical_type_ids[index as usize] = CanonicalTypeIndex { index: new_index };
          new_index
        };

        canonical_singleton_groups.insert(group);

        module.isorecursive_canonical_type_ids[index as usize] = CanonicalTypeIndex { index: group_index };
    }

    fn add_new_canonical_group_singleton_sig(&self, group: CanonicalSingletonGroup) {
        let mut canonical_singleton_groups = self.canonical_singleton_groups.lock().unwrap();
        let mut canonical_supertypes = self.canonical_supertypes.lock().unwrap();

        let _group_index = {
          let mut zone = self.zone.borrow_mut();
          let new_index = canonical_supertypes.len() as u32;
          self.canonical_types.reserve(new_index + 1, &mut zone);

          let type_ptr: *const CanonicalType = Box::leak(Box::new(group.type_));
          self.canonical_types.set(CanonicalTypeIndex {index: new_index}, type_ptr);

          new_index
        };

        canonical_singleton_groups.insert(group);
    }

    fn canonicalize_type_def(
        &self,
        module: &WasmModule,
        module_type_idx: ModuleTypeIndex,
        recgroup_start: ModuleTypeIndex,
        canonical_recgroup_start: CanonicalTypeIndex,
    ) -> CanonicalType {
        // TODO: Add actual logic for determining the canonical type.
        // This is a placeholder.
        println!(
            "Canonicalizing type at index {} in module, recgroup starts at {}, canonical recgroup starts at {}",
            module_type_idx, recgroup_start, canonical_recgroup_start.index
        );
        CanonicalType::default()
    }

    fn is_canonical_subtype_locked(&self, sub_index: CanonicalTypeIndex, super_index: CanonicalTypeIndex) -> bool {
        if sub_index == super_index {
            return true;
        }

        let canonical_supertypes = self.canonical_supertypes.lock().unwrap();

        let mut current_index = sub_index;
        while current_index.index != KINVALID_CANONICAL_INDEX {
            if current_index == super_index {
                return true;
            }

            // TODO: Consider storing the supertype index directly in the CanonicalType, instead of in a separate vector.
            if (current_index.index as usize) < canonical_supertypes.len() {
                current_index = canonical_supertypes[current_index.index as usize];
            } else {
                return false; // Out of bounds.
            }
        }
        false
    }

    pub fn is_heap_subtype(&self, sub: CanonicalTypeIndex, super_: CanonicalTypeIndex) -> bool {
        // Placeholder. In a real implementation, this would likely involve
        // traversing a heap type hierarchy.
        sub == super_
    }

    // This function is needed if the index wasn't already found.
    pub fn FindIndex_Slow(&self, sig: &CanonicalSig) -> CanonicalTypeIndex {
        let _lock = self.mutex.lock().unwrap();
        self.canonical_types.FindIndex_Slow(sig)
    }

    #[cfg(debug_assertions)]
    pub fn contains(&self, sig: &CanonicalSig) -> bool {
      // Placeholder.
      true
    }
}

// Conceptually a vector of CanonicalType. Modification generally requires
// synchronization, read-only access can be done without locking.
struct CanonicalTypeVector {
  segments: [AtomicPtr<Segment>; CanonicalTypeVector::K_NUM_SEGMENTS],
}

impl CanonicalTypeVector {
  const K_SEGMENT_SIZE: u32 = 1024;
  const K_NUM_SEGMENTS: usize = ((KV8_MAX_WASM_TYPES + Self::K_SEGMENT_SIZE as usize - 1) / Self::K_SEGMENT_SIZE as usize);

  fn new() -> Self {
    CanonicalTypeVector {
      segments: {
        let mut arr: [AtomicPtr<Segment>; Self::K_NUM_SEGMENTS] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        for item in &mut arr {
          *item = AtomicPtr::new(std::ptr::null_mut());
        }
        arr
      },
    }
  }

  fn reserve(&self, size: u32, zone: &mut Zone) {
    assert!(KV8_MAX_WASM_TYPES as u32 >= size);
    let segment_idx = size / Self::K_SEGMENT_SIZE;
    let mut segment_idx_mut = segment_idx as i32;

    while segment_idx_mut >= 0 && self.segments[segment_idx_mut as usize].load(Ordering::Relaxed).is_null() {
      let new_segment = Box::into_raw(Box::new(Segment::new()));
      self.segments[segment_idx_mut as usize].store(new_segment, Ordering::Relaxed);
      segment_idx_mut -= 1;
    }
  }

  fn set(&self, index: CanonicalTypeIndex, type_: *const CanonicalType) {
    let segment_idx = index.index / Self::K_SEGMENT_SIZE;
    let segment = unsafe { &mut *self.segments[segment_idx as usize].load(Ordering::Relaxed) };
    segment.set(index.index % Self::K_SEGMENT_SIZE, type_);
  }

  fn ClearForTesting(&self) {
    for i in 0..Self::K_NUM_SEGMENTS {
        let segment_ptr = self.segments[i].load(Ordering::Relaxed);
        if segment_ptr.is_null() {
            break; // Stop at the first null segment.
        }

        // Deallocate the segment if it exists.
        unsafe {
            Box::from_raw(segment_ptr);
        }
        self.segments[i].store(std::ptr::null_mut(), Ordering::Relaxed); // Set the segment to null.
    }
  }

  fn FindIndex_Slow(&self, sig: &FunctionSig) -> CanonicalTypeIndex {
    for i in 0..Self::K_NUM_SEGMENTS {
      let segment_ptr = self.segments[i].load(Ordering::Relaxed);

      // If callers have a CanonicalSig* to pass into this function, the
      // type canonicalizer must know about this sig, hence we must find it
      // before hitting a `null` segment.
      assert!(!segment_ptr.is_null());

      let segment = unsafe { &*segment_ptr };
      for k in 0..Self::K_SEGMENT_SIZE {
        let type_ptr = segment.content[k as usize].load(Ordering::Relaxed);

        assert!(!type_ptr.is_null());

        let type_ = unsafe { &*type_ptr };

        if type_.kind == CanonicalType::Kind::Function && type_.function_sig == sig {
          return CanonicalTypeIndex { index: i as u32 * Self::K_SEGMENT_SIZE + k };
        }
      }
    }
    panic!();
  }

  fn index(&self, index: CanonicalTypeIndex) -> &CanonicalType {
    let segment_idx = index.index / Self::K_SEGMENT_SIZE;

    // Only check against the static constant here; uninitialized segments are
    // {nullptr}, so accessing them crashes.
    assert!(Self::K_NUM_SEGMENTS > segment_idx as usize);

    let segment = unsafe { &*self.segments[segment_idx as usize].load(Ordering::Relaxed) };
    let type_ptr = segment.content[(index.index % Self::K_SEGMENT_SIZE) as usize].load(Ordering::Relaxed);

    unsafe { &*type_ptr }
  }
}

struct Segment {
  content: [AtomicPtr<CanonicalType>; Segment::K_SEGMENT_SIZE as usize],
}

impl Segment {
  const K_SEGMENT_SIZE: u32 = 1024;

  fn new() -> Self {
    let mut content: [AtomicPtr<CanonicalType>; Self::K_SEGMENT_SIZE as usize] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    for item in &mut content {
      *item = AtomicPtr::new(std::ptr::null_mut());
    }
    Segment {
      content,
    }
  }

  fn set(&mut self, index: u32, type_: *const CanonicalType) {
    assert!(Self::K_SEGMENT_SIZE > index);
    let current_value = self.content[index as usize].load(Ordering::Relaxed);
    assert!(current_value.is_null());
    self.content[index as usize].store(type_, Ordering::Relaxed);
  }
}

impl std::ops::Index<CanonicalTypeIndex> for CanonicalTypeVector {
  type Output = CanonicalType;

  fn index(&self, index: CanonicalTypeIndex) -> &Self::Output {
    self.index(index)
  }
}

impl std::ops::Index<CanonicalValueType> for CanonicalTypeVector {
  type Output = CanonicalType;

  fn index(&self, type_: CanonicalValueType) -> &Self::Output {
    assert!(type_.has_index());
    self.index(type_.ref_index())
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct CanonicalGroup {
    types: base::Vector<CanonicalType>,
    first: CanonicalTypeIndex,
}

impl CanonicalGroup {
    fn new(zone: &mut Zone, size: usize, first: CanonicalTypeIndex) -> Self {
        assert!(size >= 2);
        CanonicalGroup {
            types: zone.allocate_vector(size),
            first,
        }
    }

    fn hash_value(&self) -> u64 {
      let last = CanonicalTypeIndex { index: self.first.index + self.types.size() as u32 - 1 };
      let hasher_data = HashingData {
        recgroup: RecursionGroupRange { first: self.first, last },
        function_sig: None,
        canonical_struct_type: None,
        canonical_array_type: None,
        canonical_cont_type: None,
      };

        let mut hasher = CanonicalHashing {
            hasher: base::hashing::Hasher::new(),
            hasher_data,
        };
        for t in &self.types {
            hasher.add(t.clone());
        }
        hasher.hasher.hash() as u64
    }
}

// Implement Hash manually to use the custom hash_value function
impl Hash for TypeCanonicalizer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical_groups.lock().unwrap().iter().for_each(|group| {
            group.hash_value().hash(state);
        });
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct CanonicalSingletonGroup {
    type_: CanonicalType,
    index: CanonicalTypeIndex,
}

impl CanonicalSingletonGroup {
    fn new(zone: &mut Zone, index: CanonicalTypeIndex) -> Self {
        CanonicalSingletonGroup {
            type_: CanonicalType::default(),
            index,
        }
    }

    fn new_sig(zone: &mut Zone, sig: &FunctionSig) -> Self {
        let type_ = CanonicalType::new_function(sig, K_NO_SUPER_TYPE, false, false);
        let index = CanonicalTypeIndex { index: KINVALID_CANONICAL_INDEX };

        CanonicalSingletonGroup {