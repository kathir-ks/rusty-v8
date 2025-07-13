// Converted from V8 C++ source files:
// Header: identity-map.h
// Implementation: identity-map.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod zone {
    pub struct Zone {}
}

pub mod base {
    pub mod hashing {
        pub struct Hash<T> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<T> Hash<T> {
            pub fn new() -> Self {
                Hash {
                    _marker: std::marker::PhantomData,
                }
            }

            pub fn call(&self, value: T) -> usize {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&value, &mut hasher);
                hasher.finish() as usize
            }
        }

        impl<T> Default for Hash<T> {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

pub mod handles {
    use std::ops::Deref;
    use std::rc::Rc;

    #[derive(Clone)]
    pub struct Handle<T> {
        pub(crate) value: Rc<T>,
    }

    impl<T> Handle<T> {
        pub fn new(value: T) -> Self {
            Handle {
                value: Rc::new(value),
            }
        }
    }

    impl<T> Deref for Handle<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    
    #[derive(Clone, Copy)]
    pub struct DirectHandle<T> {
        pub(crate) value: *mut T,
    }

    impl<T> DirectHandle<T> {
        pub fn new(value: *mut T) -> Self {
            DirectHandle {
                value: value,
            }
        }
    }

    impl<T> Deref for DirectHandle<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.value }
        }
    }
}

pub mod objects {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Tagged<T> {
        ptr: usize,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Tagged<T> {
        pub fn new(ptr: usize) -> Self {
            Tagged {
                ptr: ptr,
                _marker: std::marker::PhantomData,
            }
        }

        pub fn ptr(&self) -> usize {
            self.ptr
        }
    }
    
    pub trait HeapObject {}

    #[derive(Debug, Clone, PartialEq)]
    pub struct Object {
        pub(crate) address: usize,
    }
    
    impl Object : HeapObject {}
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct String {
        pub(crate) address: usize,
    }
    
    impl String : HeapObject {}
}

pub mod heap {
    use crate::handles::Handle;
    use crate::objects::Object;
    use crate::objects::HeapObject;

    #[derive(Debug)]
    pub enum HeapError {
        AllocationFailed,
        InvalidObject,
    }

    #[derive(Debug)]
    pub struct Heap {
        gc_count: i32,
    }

    impl Heap {
        pub fn new() -> Self {
            Heap { gc_count: 0 }
        }

        pub fn gc_count(&self) -> i32 {
            self.gc_count
        }

        pub fn gc_state(&self) -> GCState {
            GCState::NOT_GC
        }

        pub fn allocate<T: HeapObject>(&mut self) -> Result<Handle<T>, HeapError> {
            Ok(Handle::new(unsafe {
                std::mem::zeroed()
            }))
        }
        
        pub fn register_strong_roots(&mut self, name: &str, start: FullObjectSlot, end: FullObjectSlot) -> *mut StrongRootsEntry {
          Box::into_raw(Box::new(StrongRootsEntry{}))
        }
        
        pub fn unregister_strong_roots(&mut self, entry: *mut StrongRootsEntry) {
          unsafe {
            if !entry.is_null() {
              drop(Box::from_raw(entry));
            }
          }
        }
        
        pub fn update_strong_roots(&mut self, entry: *mut StrongRootsEntry, start: FullObjectSlot, end: FullObjectSlot) {
            
        }
        
        pub fn incr_gc_count(&mut self) {
          self.gc_count += 1;
        }
    }
    
    #[derive(Debug, PartialEq)]
    pub enum GCState {
        MARK_COMPACT,
        NOT_GC
    }

    pub struct ReadOnlyRoots {
        heap: *mut Heap,
    }

    impl ReadOnlyRoots {
        pub fn new(heap: *mut Heap) -> Self {
            ReadOnlyRoots { heap }
        }

        pub fn not_mapped_symbol(&self) -> crate::objects::Tagged<crate::objects::Object> {
            crate::objects::Tagged::new(0)
        }
    }

    pub struct FullObjectSlot(pub *mut usize);
}

pub mod roots {
    pub mod roots_inl {
        use crate::heap::Heap;
        use crate::heap::ReadOnlyRoots;

        pub fn ReadOnlyRoots(heap: *mut Heap) -> ReadOnlyRoots {
            ReadOnlyRoots::new(heap)
        }
    }
}

use crate::handles::DirectHandle;
use crate::heap::Heap;
use crate::objects::Object;
use crate::objects::Tagged;
use std::marker::PhantomData;

pub struct StrongRootsEntry {}

#[derive(Debug)]
pub enum IdentityMapError {
    KeyNotFound,
    AllocationError,
    RehashError,
    IterableError,
}

pub struct IdentityMapFindResult<T> {
    pub entry: *mut T,
    pub already_exists: bool,
}

pub struct IdentityMapBase {
    hasher_: base::hashing::Hash<usize>,
    heap_: *mut Heap,
    gc_counter_: i32,
    size_: i32,
    capacity_: i32,
    mask_: i32,
    keys_: *mut usize,
    strong_roots_entry_: *mut StrongRootsEntry,
    values_: *mut usize,
    is_iterable_: bool,
}

impl IdentityMapBase {
    pub fn new(heap: *mut Heap) -> Self {
        IdentityMapBase {
            hasher_: base::hashing::Hash::new(),
            heap_: heap,
            gc_counter_: -1,
            size_: 0,
            capacity_: 0,
            mask_: 0,
            keys_: std::ptr::null_mut(),
            strong_roots_entry_: std::ptr::null_mut(),
            values_: std::ptr::null_mut(),
            is_iterable_: false,
        }
    }

    pub fn empty(&self) -> bool {
        self.size_ == 0
    }

    pub fn size(&self) -> i32 {
        self.size_
    }

    pub fn capacity(&self) -> i32 {
        self.capacity_
    }

    pub fn is_iterable(&self) -> bool {
        self.is_iterable_
    }

    fn find_or_insert_entry(&mut self, key: usize) -> IdentityMapFindResult<usize> {
        self.check_gc();
        if self.capacity_ == 0 {
            return IdentityMapFindResult {
                entry: self.insert_entry(key),
                already_exists: false,
            };
        }
        let lookup_result = self.lookup_or_insert(key);
        IdentityMapFindResult {
            entry: unsafe { self.values_.add(lookup_result.0) },
            already_exists: lookup_result.1,
        }
    }

    fn find_entry(&self, key: usize) -> *mut usize {
        if self.size_ == 0 {
            return std::ptr::null_mut();
        }
        let index = self.lookup(key);
        if index >= 0 {
            unsafe { self.values_.add(index) }
        } else {
            std::ptr::null_mut()
        }
    }

    fn insert_entry(&mut self, key: usize) -> *mut usize {
        self.check_gc();
        if self.capacity_ == 0 {
            self.capacity_ = 4;
            self.mask_ = 3;
            self.gc_counter_ = unsafe { (*self.heap_).gc_count() };

            let not_mapped = unsafe {
                (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
            };
            
            let keys_size = self.capacity_ as usize;

            self.keys_ = self.new_pointer_array(self.capacity_ as usize, not_mapped);
            unsafe {
                for i in 0..keys_size {
                  self.keys_.add(i).write(not_mapped);
                }
            };
            
            self.values_ = self.new_pointer_array(self.capacity_ as usize, 0);

            let full_object_slot_start = crate::heap::FullObjectSlot(self.keys_ as *mut usize);
            let full_object_slot_end = crate::heap::FullObjectSlot(unsafe {self.keys_.add(self.capacity_ as usize)} as *mut usize);
            self.strong_roots_entry_ = unsafe {
                (*self.heap_).register_strong_roots("IdentityMapBase", full_object_slot_start, full_object_slot_end)
            };
        } else {
            self.rehash();
        }
        let insert_result = self.insert_key(key, self.hash(key) as i32);
        unsafe { self.values_.add(insert_result.0) }
    }

    fn delete_entry(&mut self, key: usize, deleted_value: *mut usize) -> bool {
        if self.size_ == 0 {
            return false;
        }
        let index = self.lookup(key);
        if index < 0 {
            return false;
        }
        self.delete_index(index, deleted_value)
    }

    fn clear(&mut self) {
        if !self.keys_.is_null() {
            assert!(!self.is_iterable());
            assert!(!self.strong_roots_entry_.is_null());
            unsafe {
                (*self.heap_).unregister_strong_roots(self.strong_roots_entry_);
            }
            self.delete_pointer_array(self.keys_, self.capacity_ as usize);
            self.delete_pointer_array(self.values_, self.capacity_ as usize);
            self.keys_ = std::ptr::null_mut();
            self.strong_roots_entry_ = std::ptr::null_mut();
            self.values_ = std::ptr::null_mut();
            self.size_ = 0;
            self.capacity_ = 0;
            self.mask_ = 0;
        }
    }

    fn key_at_index(&self, index: i32) -> usize {
        assert!(index >= 0);
        assert!(index < self.capacity_);
        assert!(!self.is_iterable());

        unsafe { *self.keys_.add(index as usize) }
    }

    fn entry_at_index(&self, index: i32) -> *mut usize {
        assert!(index >= 0);
        assert!(index < self.capacity_);
        assert!(!self.is_iterable());

        unsafe { self.values_.add(index as usize) }
    }

    fn next_index(&self, index: i32) -> i32 {
        assert!(index <= self.capacity_);
        assert!(index >= -1);
        assert!(self.is_iterable());

        let not_mapped = unsafe {
            (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
        };

        let mut index = index + 1;
        while index < self.capacity_ {
            unsafe {
                if *self.keys_.add(index as usize) != not_mapped {
                    return index;
                }
            }
            index += 1;
        }
        return self.capacity_;
    }

    fn enable_iteration(&mut self) {
        assert!(!self.is_iterable());
        self.is_iterable_ = true;
    }

    fn disable_iteration(&mut self) {
        assert!(self.is_iterable());
        self.is_iterable_ = false;
    }

    fn new_pointer_array(&self, length: usize, value: usize) -> *mut usize {
      let mut vec = vec![value; length];
      let ptr = vec.as_mut_ptr();
      std::mem::forget(vec);
      ptr
    }

    fn delete_pointer_array(&self, array: *mut usize, length: usize) {
      unsafe {
        drop(Vec::from_raw_parts(array, length, length));
      }
    }

    fn scan_keys_for(&self, address: usize, hash: u32) -> (i32, bool) {
        let start = (hash & self.mask_ as u32) as i32;
        let not_mapped = unsafe {
            (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
        };
        for index in start..self.capacity_ {
            unsafe {
                if *self.keys_.add(index as usize) == address {
                    return (index, true);
                }
                if *self.keys_.add(index as usize) == not_mapped {
                    return (index, false);
                }
            }
        }
        for index in 0..start {
            unsafe {
                if *self.keys_.add(index as usize) == address {
                    return (index, true);
                }
                if *self.keys_.add(index as usize) == not_mapped {
                    return (index, false);
                }
            }
        }
        (-1, false)
    }

    fn should_grow(&self) -> bool {
        self.size_ + self.size_ / 4 >= self.capacity_
    }

    fn insert_key(&mut self, address: usize, hash: i32) -> (i32, bool) {
        self.check_gc();

        if self.should_grow() {
            self.resize(self.capacity_ * 2);
        }

        let not_mapped = unsafe {
            (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
        };

        let start = (hash & self.mask_) as i32;
        let mut index = start;
        loop {
            unsafe {
                if *self.keys_.add(index as usize) == address {
                    return (index, true);
                }
                if *self.keys_.add(index as usize) == not_mapped {
                    self.size_ += 1;
                    assert!(self.size_ <= self.capacity_);
                    *self.keys_.add(index as usize) = address;
                    return (index, false);
                }
            }
            index = (index + 1) & self.mask_;
            assert!(index != start);
        }
    }

    fn delete_index(&mut self, index: i32, deleted_value: *mut usize) -> bool {
        if !deleted_value.is_null() {
            unsafe { *deleted_value = *self.values_.add(index as usize) };
        }
        let not_mapped = unsafe {
            (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
        };

        unsafe {
            assert!(*self.keys_.add(index as usize) != not_mapped);
            *self.keys_.add(index as usize) = not_mapped;
            *self.values_.add(index as usize) = 0;
        }
        self.size_ -= 1;
        assert!(self.size_ >= 0);

        if self.capacity_ > 4 && self.size_ * 2 < self.capacity_ / 2 {
            self.resize(self.capacity_ / 2);
            return true;
        }

        let mut next_index = index;
        loop {
            next_index = (next_index + 1) & self.mask_;
            let key = unsafe { *self.keys_.add(next_index as usize) };
            if key == not_mapped {
                break;
            }

            let expected_index = (self.hash(key) & self.mask_ as u32) as i32;
            if index < next_index {
                if index < expected_index && expected_index <= next_index {
                    continue;
                }
            } else {
                assert!(index > next_index);
                if index < expected_index || expected_index <= next_index {
                    continue;
                }
            }

            unsafe {
                assert!(*self.keys_.add(index as usize) == not_mapped);
                assert!(*self.values_.add(index as usize) == 0);

                let temp_key = *self.keys_.add(next_index as usize);
                let temp_value = *self.values_.add(next_index as usize);

                *self.keys_.add(index as usize) = temp_key;
                *self.values_.add(index as usize) = temp_value;

                *self.keys_.add(next_index as usize) = not_mapped;
                *self.values_.add(next_index as usize) = 0;
            }

            index = next_index;
        }
        return true;
    }

    fn lookup(&self, key: usize) -> i32 {
        self.check_gc();
        let hash = self.hash(key) as u32;
        let scan_result = self.scan_keys_for(key, hash);
        if !scan_result.1 && self.gc_counter_ != unsafe { (*self.heap_).gc_count() } {
            let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
            mut_self.rehash();
            let scan_result = mut_self.scan_keys_for(key, hash);
            return if scan_result.1 { scan_result.0 } else { -1 };
        }
        if scan_result.1 {
            scan_result.0
        } else {
            -1
        }
    }

    fn lookup_or_insert(&mut self, key: usize) -> (i32, bool) {
        self.check_gc();
        let hash = self.hash(key) as u32;
        let scan_result = self.scan_keys_for(key, hash);
        if !scan_result.1 {
            if self.gc_counter_ != unsafe { (*self.heap_).gc_count() } {
                self.rehash();
            }
            if scan_result.0 < 0 || self.should_grow() {
                return self.insert_key(key, self.hash(key) as i32);
            } else {
                self.size_ += 1;
                assert!(self.size_ <= self.capacity_);
                let not_mapped = unsafe {
                    (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
                };
                unsafe {
                    assert!(*self.keys_.add(scan_result.0 as usize) == not_mapped);
                    *self.keys_.add(scan_result.0 as usize) = key;
                }
                return (scan_result.0, false);
            }
        }
        (scan_result.0, true)
    }

    fn hash(&self, address: usize) -> u32 {
        let not_mapped = unsafe {
            (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
        };
        assert!(address != not_mapped);
        self.hasher_.call(address) as u32
    }

    fn rehash(&mut self) {
        assert!(!self.is_iterable());

        self.gc_counter_ = unsafe { (*self.heap_).gc_count() };

        let mut reinsert = Vec::<(usize, usize)>::new();

        let mut last_empty: i32 = -1;
        let not_mapped = unsafe {
            (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
        };

        for i in 0..self.capacity_ {
            let i_usize = i as usize;
            unsafe {
                if *self.keys_.add(i_usize) == not_mapped {
                    last_empty = i;
                } else {
                    let pos = (self.hash(*self.keys_.add(i_usize)) & self.mask_ as u32) as i32;
                    if pos <= last_empty || pos > i {
                        reinsert.push((*self.keys_.add(i_usize), *self.values_.add(i_usize)));
                        *self.keys_.add(i_usize) = not_mapped;
                        *self.values_.add(i_usize) = 0;
                        last_empty = i;
                        self.size_ -= 1;
                    }
                }
            }
        }

        for pair in reinsert {
            let insert_result = self.insert_key(pair.0, self.hash(pair.0) as i32);
            unsafe {
                *self.values_.add(insert_result.0 as usize) = pair.1;
            }
        }
    }

    fn resize(&mut self, new_capacity: i32) {
        assert!(!self.is_iterable());

        assert!(new_capacity > self.size_);

        let old_capacity = self.capacity_;
        let old_keys = self.keys_;
        let old_values = self.values_;

        self.capacity_ = new_capacity;
        self.mask_ = new_capacity - 1;
        self.gc_counter_ = unsafe { (*self.heap_).gc_count() };
        self.size_ = 0;

        let not_mapped = unsafe {
            (*crate::roots::roots_inl::ReadOnlyRoots(self.heap_).not_mapped_symbol()).ptr()
        };

        self.keys_ = self.new_pointer_array(self.capacity_ as usize, not_mapped);
        unsafe {
          for i in 0..self.capacity_ as usize {
            *self.keys_.add(i) = not_mapped;
          }
        }
        
        self.values_ = self.new_pointer_array(self.capacity_ as usize, 0);

        unsafe {
            for i in 0..old_capacity {
                if *old_keys.add(i as usize) == not_mapped {
                    continue;
                }
                let insert_result = self.insert_key(*old_keys.add(i as usize), self.hash(*old_keys.add(i as usize)) as i32);
                *self.values_.add(insert_result.0 as usize) = *old_values.add(i as usize);
            }
        }

        let full_object_slot_start = crate::heap::FullObjectSlot(self.keys_ as *mut usize);
        let full_object_slot_end = crate::heap::FullObjectSlot(unsafe {self.keys_.add(self.capacity_ as usize)} as *mut usize);
        unsafe {
            (*self.heap_).update_strong_roots(self.strong_roots_entry_, full_object_slot_start, full_object_slot_end);
        }

        self.delete_pointer_array(old_keys, old_capacity as usize);
        self.delete_pointer_array(old_values, old_capacity as usize);
    }
    
    fn check_gc(&self) {
      assert!(unsafe{(*self.heap_).gc_state()} != crate::heap::GCState::MARK_COMPACT);
    }
}

impl Drop for IdentityMapBase {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct AllocationPolicy {}

impl AllocationPolicy {
    pub fn new() -> Self {
        AllocationPolicy {}
    }

    pub fn allocate_array<T, Buffer>(&self, length: usize) -> *mut T {
        let mut vec = Vec::with_capacity(length);
        let ptr = vec.as_mut_ptr();
        std::mem::forget(vec);
        ptr
    }

    pub fn delete_array<T, Buffer>(&self, array: *mut T, length: usize) {
        unsafe {
            drop(Vec::from_raw_parts(array, length, length));
        }
    }
}

impl Default for AllocationPolicy {
  fn default() -> Self {
    Self::new()
  }
}

pub struct IdentityMap<V, AllocationPolicy> {
    base: IdentityMapBase,
    _marker: PhantomData<V>,
    allocator_: AllocationPolicy,
}

impl<V, AllocationPolicy> IdentityMap<V, AllocationPolicy>
where
    AllocationPolicy: Default,
{
    pub fn new(heap: *mut Heap) -> Self {
        IdentityMap {
            base: IdentityMapBase::new(heap),
            _marker: PhantomData,
            allocator_: AllocationPolicy::default(),
        }
    }
}

impl<V, AllocationPolicy> IdentityMap<V, AllocationPolicy> {
    pub fn with_allocator(heap: *mut Heap, allocator: AllocationPolicy) -> Self {
        IdentityMap {
            base: IdentityMapBase::new(heap),
            _marker: PhantomData,
            allocator_: allocator,
        }
    }

    pub fn find_or_insert(&mut self, key: DirectHandle<Object>) -> IdentityMapFindResult<V> {
        self.find_or_insert_tagged(*key)
    }

    pub fn find_or_insert_tagged(&mut self, key: Tagged<Object>) -> IdentityMapFindResult<V> {
        let raw = self.base.find_or_insert_entry(key.ptr());
        IdentityMapFindResult {
            entry: raw.entry as *mut V,
            already_exists: raw.already_exists,
        }
    }

    pub fn find(&self, key: DirectHandle<Object>) -> *mut V {
        self.find_tagged(*key)
    }

    pub fn find_tagged(&self, key: Tagged<Object>) -> *mut V {
        self.base.find_entry(key.ptr()) as *mut V
    }

    pub fn insert(&mut self, key: DirectHandle<Object>, v: V) {
        self.insert_tagged(*key, v);
    }

    pub fn insert_tagged(&mut self, key: Tagged<Object>, v: V) {
        unsafe {
            *(self.base.insert_entry(key.ptr()) as *mut V) = v;
        }
    }

    pub fn delete(&mut self, key: DirectHandle<Object>, deleted_value: *mut V) -> bool {
        self.delete_tagged(*key, deleted_value)
    }

    pub fn delete_tagged(&mut self, key: Tagged<Object>, deleted_value: *mut V) -> bool {
        let mut v: usize = 0;
        let deleted_something = self.base.delete_entry(key.ptr(), &mut v);
        if !deleted_value.is_null() && deleted_something {
            unsafe {
                *deleted_value = *(v as *mut V);
            }
        }
        deleted_something
    }

    pub fn clear(&mut self) {
        self.base.clear();
    }
    
    pub fn empty(&self) -> bool {
      self.base.empty()
    }
    
    pub fn size(&self) -> i32 {
      self.base.size()
    }
    
    pub fn capacity(&self) -> i32 {
      self.base.capacity()
    }
}

impl<V, AllocationPolicy> Drop for IdentityMap<V, AllocationPolicy> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<V, AllocationPolicyT> IdentityMap<V, AllocationPolicyT> {
    
}

impl<V, AllocationPolicy> IdentityMap<V, AllocationPolicy>
where AllocationPolicy: Default,
{
  
}

pub struct Buffer {}

impl<V, AllocationPolicy> IdentityMapBase {
    fn new_pointer_array_templated<AllocationPolicyT>(&self, length: usize, value: usize, allocator_: &AllocationPolicyT) -> *mut usize
    where AllocationPolicyT: Default {
      let mut vec = vec![value; length];
      let ptr = vec.as_mut_ptr();
      std::mem::forget(vec);
      ptr
    }

    fn delete_pointer_array_templated<AllocationPolicyT>(&self, array: *mut usize, length: usize, allocator_: &AllocationPolicyT) {
      unsafe {
        drop(Vec::from_raw_parts(array, length, length));
      }
    }
}

pub struct Iterator<'a, V, AllocationPolicy> {
    map_: *mut IdentityMap<V, AllocationPolicy>,
    index_: i32,
    _marker: PhantomData<&'a V>,
}

impl<'a, V, AllocationPolicy> Iterator<'a, V, AllocationPolicy> {
    fn new(map_: *mut IdentityMap<V, AllocationPolicy>, index_: i32) -> Self {
        Iterator {
            map_: map_,
            index_: index_,
            _marker: PhantomData,
        }
    }

    pub fn key(&self) -> Tagged<Object> {
        unsafe {
            Tagged::new((*self.map_).base.key_at_index(self.index_) )
        }
    }

    pub fn entry(&self) -> *mut V {
        unsafe {
            (*self.map_).base.entry_at_index(self.index_) as *mut V
        }
    }
}

impl<'a, V, AllocationPolicy> std::ops::AddAssign<()> for Iterator<'a, V, AllocationPolicy> {
    fn add_assign(&mut self, _rhs: ()) {
        self.index_ = unsafe { (*self.map_).base.next_index(self.index_) };
    }
}

impl<'a, V, AllocationPolicy> std::ops::Deref for Iterator<'a, V, AllocationPolicy> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.entry() }
    }
}

impl<'a, V, AllocationPolicy> std::ops::DerefMut for Iterator<'a, V, AllocationPolicy> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.entry() }
    }
}

impl<'a, V, AllocationPolicy> std::cmp::PartialEq for Iterator<'a, V, AllocationPolicy> {
    fn eq(&self, other: &Self) -> bool {
        self.index_ == other.index_
    }
}

impl<'a, V, AllocationPolicy> std::cmp::Eq for Iterator<'a, V, AllocationPolicy> {}

impl<'a, V, AllocationPolicy> Iterator<'a, V, AllocationPolicy>
where AllocationPolicy: Default
{
  
}

pub struct IteratableScope<'a, V, AllocationPolicy> {
    map_: *mut IdentityMap<V, AllocationPolicy>,
    _marker: PhantomData<&'a IdentityMap<V, AllocationPolicy>>,
}

impl<'a, V, AllocationPolicy> IteratableScope<'a, V, AllocationPolicy> {
    pub fn new(map_: *mut IdentityMap<V, AllocationPolicy>) -> Self {
        unsafe {
            assert!(!(*map_).base.is_iterable());
            (*map_).base.enable_iteration();
        }
        IteratableScope {
            map_: map_,
            _marker: PhantomData,
        }
    }

    pub fn begin(&self) -> Iterator<'a, V, AllocationPolicy> {
        unsafe {
            Iterator::new(self.map_, (*self.map_).base.next_index(-1))
        }
    }

    pub fn end(&self) -> Iterator<'a, V, AllocationPolicy> {
        unsafe {
            Iterator::new(self.map_, (*self.map_).base.capacity())
        }
    }
}

impl<'a, V, AllocationPolicy> Drop for IteratableScope<'a, V, AllocationPolicy> {
    fn drop(&mut self) {
        unsafe {
            assert!((*self.map_).base.is_iterable());
            (*self.map_).base.disable_iteration();
        }
    }
}
