// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add appropriate Rust crates for any C++ libraries used

// pub mod read_only_heap;
// pub mod objects;
// pub mod read_only_serializer;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Dummy definitions to allow compilation. Replace with actual V8 types.
type HeapObject = u64;
type Tagged<T> = T;
type Isolate = u64;
type StringTable = u64;
type Object = u64;
type Handle<T> = u64;
type SnapshotByteSink = u64;
type FullObjectSlot = u64;
type OffHeapObjectSlot = u64;
type Root = u64;
type SlotType = u64;

const kSharedHeapObjectCache: u8 = 1; // Dummy value
const RootIndex_kFirstStrongRoot: u64 = 0; // Dummy value

fn IsString(_obj: Tagged<HeapObject>) -> bool {
    true // Dummy implementation
}

fn IsInternalizedString(_obj: Tagged<HeapObject>) -> bool {
    true // Dummy implementation
}

fn String_IsInPlaceInternalizable(_obj: Tagged<HeapObject>) -> bool {
    true // Dummy implementation
}

fn Cast<T>(_obj: Tagged<HeapObject>) -> Tagged<HeapObject> {
    _obj // Dummy implementation
}

fn ReadOnlyHeap_Contains(_obj: Tagged<HeapObject>) -> bool {
    false // Dummy implementation
}

fn IsUndefined(_obj: Tagged<Object>, _roots: ReadOnlyRoots) -> bool {
    false // Dummy implementation
}

struct ReadOnlyRoots {}
impl ReadOnlyRoots {
  fn undefined_value(&self) -> Tagged<Object>{
    0 // Dummy implementation
  }
}

struct Snapshot {
  struct SerializerFlags {}
}

struct RootsSerializer {
    isolate: *mut Isolate,
    flags: u32,
    first_strong_root: u64,
}

impl RootsSerializer {
    fn new(isolate: *mut Isolate, flags: u32, first_strong_root: u64) -> Self {
        RootsSerializer {
            isolate,
            flags,
            first_strong_root,
        }
    }
}

struct DisallowGarbageCollection {}
impl DisallowGarbageCollection {
    fn new() -> Self {
        DisallowGarbageCollection {}
    }
}

struct ObjectSerializer {}
impl ObjectSerializer {
    fn new() -> Self {
        ObjectSerializer {}
    }
}

struct SharedSpaceIsolate {}
impl SharedSpaceIsolate {
    fn shared_heap_object_cache(&self) -> &mut Vec<Tagged<Object>> {
        todo!()
    }
}

// Dummy implementations to allow compilation.
fn ShouldReconstructSharedHeapObjectCacheForTesting() -> bool {
  false
}

impl Drop for DisallowGarbageCollection {
    fn drop(&mut self) {}
}

struct SharedHeapSerializer {
    roots_serializer: RootsSerializer,
    // TODO: serialized_objects_: IdentityMap<int, base::DefaultAllocationPolicy>,
    serialized_objects_: RefCell<HashMap<HeapObject, i32>>,
    isolate: *mut Isolate,
    sink_: u64, //SnapshotByteSink, // Dummy for now.
    flags_: u32,
    reconstruct_read_only_and_shared_object_caches_for_testing_: bool,
}

impl SharedHeapSerializer {
    fn new(isolate: *mut Isolate, flags: u32) -> Self {
        SharedHeapSerializer {
            roots_serializer: RootsSerializer::new(isolate, flags, RootIndex_kFirstStrongRoot),
            serialized_objects_: RefCell::new(HashMap::new()),
            isolate,
            sink_: 0,
            flags_: flags,
            reconstruct_read_only_and_shared_object_caches_for_testing_: false,
        }
    }

    fn can_be_in_shared_old_space(obj: Tagged<HeapObject>) -> bool {
        if ReadOnlyHeap_Contains(obj) {
            return false;
        }
        if IsString(obj) {
            return IsInternalizedString(obj) || String_IsInPlaceInternalizable(Cast(obj));
        }
        false
    }

    fn should_be_in_shared_heap_object_cache(obj: Tagged<HeapObject>) -> bool {
        if Self::can_be_in_shared_old_space(obj) {
            if IsInternalizedString(obj) {
                return true;
            }
        }
        false
    }

    fn isolate(&self) -> *mut Isolate {
        self.isolate
    }

    fn finalize_serialization(&mut self) {
        // This is called after serialization of the startup and context snapshots
        // which entries are added to the shared heap object cache. Terminate the
        // cache with an undefined.
        let undefined = ReadOnlyRoots {}.undefined_value();
        self.visit_root_pointer(Root::kSharedHeapObjectCache, 0, undefined);

        // When v8_flags.shared_string_table is true, all internalized and
        // internalizable-in-place strings are in the shared heap.
        self.serialize_string_table(0); //isolate().string_table());
        self.serialize_deferred_objects();
        self.pad();

        // #ifdef DEBUG
        // Check that all serialized object are in shared heap and not RO. RO objects
        // should be in the RO snapshot.
        for (obj, _value) in self.serialized_objects_.borrow().iter() {
            let obj = *obj;
            assert!(Self::can_be_in_shared_old_space(obj));
            assert!(!ReadOnlyHeap_Contains(obj));
        }
        // #endif
    }

    fn serialize_using_shared_heap_object_cache(
        &mut self,
        sink: *mut SnapshotByteSink,
        obj: Handle<HeapObject>,
    ) -> bool {
        if !Self::should_be_in_shared_heap_object_cache(obj) {
            return false;
        }
        let cache_index = self.serialize_in_object_cache(obj);

        // When testing deserialization of a snapshot from a live Isolate where there
        // is also a shared Isolate, the shared object cache needs to be extended
        // because the live isolate may have had new internalized strings that were
        // not present in the startup snapshot to be serialized.
        if self.should_reconstruct_shared_heap_object_cache_for_testing() {
            let existing_cache: &mut Vec<Tagged<Object>> = unsafe {
                (*(self.isolate as *mut Isolate as *mut Isolate))
                    .shared_space_isolate()
                    .shared_heap_object_cache()
            };
            let existing_cache_size = existing_cache.len();
            // This is strictly < because the existing cache contains the terminating
            // undefined value, which the reconstructed cache does not.
            assert!(cache_index < existing_cache_size);
            if cache_index == existing_cache_size - 1 {
                let roots = ReadOnlyRoots {};
                assert!(!IsUndefined(existing_cache.last().copied().unwrap(), roots)); //DCHECK(IsUndefined(existing_cache->back(), roots));
                existing_cache[existing_cache.len()-1] = obj; //existing_cache->back() = *obj;
                existing_cache.push(ReadOnlyRoots{}.undefined_value()); //existing_cache->push_back(roots.undefined_value());
            }
        }

        unsafe {
          (*(sink as *mut SnapshotByteSink)).put(kSharedHeapObjectCache, "SharedHeapObjectCache");
          (*(sink as *mut SnapshotByteSink)).put_uint30(cache_index as u32, "shared_heap_object_cache_index");
        }
        
        true
    }

    fn serialize_string_table(&mut self, string_table: StringTable) {
        // A StringTable is serialized as:
        //
        //   N : int
        //   string 1
        //   string 2
        //   ...
        //   string N
        //
        // Notably, the hashmap structure, including empty and deleted elements, is
        // not serialized.

        self.sink_.put_uint30(0, "String table number of elements"); //string_table.NumberOfElements(),

        // Custom RootVisitor which walks the string table, but only serializes the
        // string entries. This is an inline class to be able to access the non-public
        // SerializeObject method.
        // TODO: Implement SharedHeapSerializerStringTableVisitor
        struct SharedHeapSerializerStringTableVisitor<'a> {
            serializer_: &'a mut SharedHeapSerializer,
        }

        impl<'a> SharedHeapSerializerStringTableVisitor<'a> {
            fn new(serializer: &'a mut SharedHeapSerializer) -> Self {
                SharedHeapSerializerStringTableVisitor { serializer_: serializer }
            }

            fn visit_root_pointers(
                &mut self,
                _root: Root,
                _description: &str,
                _start: FullObjectSlot,
                _end: FullObjectSlot,
            ) {
                panic!("UNREACHABLE()");
            }

            fn visit_root_pointers(
                &mut self,
                root: Root,
                _description: &str,
                start: OffHeapObjectSlot,
                end: OffHeapObjectSlot,
            ) {
                assert_eq!(root, Root::kStringTable);
                let isolate = self.serializer_.isolate();
                for current in start..end {
                    let obj = current; //current.load(isolate);
                    if true { //IsHeapObject(obj)
                        assert!(true); //IsInternalizedString(obj));
                        //self.serializer_.serialize_object(handle(Cast<HeapObject>(obj), isolate), SlotType::kAnySlot);
                    }
                }
            }
        }

        let mut string_table_visitor = SharedHeapSerializerStringTableVisitor::new(self);
        //isolate().string_table().IterateElements(&string_table_visitor);
    }

    fn serialize_object_impl(&mut self, obj: Handle<HeapObject>, slot_type: SlotType) {
        // Objects in the shared heap cannot depend on per-Isolate roots but can
        // depend on RO roots since sharing objects requires sharing the RO space.
        assert!(Self::can_be_in_shared_old_space(0) || ReadOnlyHeap_Contains(0)); //*obj));
        {
            let _no_gc = DisallowGarbageCollection::new();
            let raw = 0; //*obj;
            if self.serialize_hot_object(raw) {
                return;
            }
            if self.is_root_and_has_been_serialized(raw) && self.serialize_root(raw) {
                return;
            }
        }
        if self.serialize_read_only_object_reference(0, &self.sink_) { //*obj
            return;
        }
        {
            let _no_gc = DisallowGarbageCollection::new();
            let raw = 0; //*obj;
            if self.serialize_back_reference(raw) {
                return;
            }
            self.check_rehashability(raw);

            assert!(!ReadOnlyHeap_Contains(raw));
        }

        let mut object_serializer = ObjectSerializer::new(); //ObjectSerializer(self, obj, &sink_);
        //object_serializer.Serialize(slot_type);

        // #ifdef DEBUG
        // There's no "IdentitySet", so use an IdentityMap with a value that is
        // later ignored.
        // #endif
        self.serialized_objects_.borrow_mut().insert(0, 0);
    }

    fn should_reconstruct_shared_heap_object_cache_for_testing(&self) -> bool {
        // When the live Isolate being serialized is not a client Isolate, there's no
        // need to reconstruct the shared heap object cache because it is not actually
        // shared.
        self.reconstruct_read_only_and_shared_object_caches_for_testing_ && unsafe {
            (*(self.isolate as *mut Isolate as *mut Isolate)).has_shared_space()
        }
    }

    fn reconstruct_shared_heap_object_cache_for_testing(&mut self) {
        let cache = unsafe {
            (*(self.isolate as *mut Isolate as *mut Isolate))
                .shared_space_isolate()
                .shared_heap_object_cache()
        };
        // Don't reconstruct the final element, which is always undefined and marks
        // the end of the cache, since serializing the live Isolate may extend the
        // shared object cache.
        for i in 0..cache.len() - 1 {
            let obj = cache[i]; //Cast<HeapObject>(cache->at(i)), isolate());
            assert!(Self::should_be_in_shared_heap_object_cache(obj));
            let cache_index = self.serialize_in_object_cache(obj);
            let _ = cache_index;
            assert_eq!(cache_index, i);
        }
        assert!(!IsUndefined(cache.last().copied().unwrap(), ReadOnlyRoots {})); //DCHECK(IsUndefined(cache->back(), isolate()));
    }

    // Dummy implementations
    fn output_statistics(&self, _name: &str) {}
    fn visit_root_pointer(&mut self, _root: Root, _description: u64, _slot: Tagged<Object>) {}
    fn serialize_deferred_objects(&mut self) {}
    fn pad(&mut self) {}
    fn serialize_hot_object(&mut self, _raw: Tagged<HeapObject>) -> bool {
        false
    }
    fn is_root_and_has_been_serialized(&self, _raw: Tagged<HeapObject>) -> bool {
        false
    }
    fn serialize_root(&mut self, _raw: Tagged<HeapObject>) -> bool {
        false
    }
    fn serialize_read_only_object_reference(&self, _obj: Tagged<HeapObject>, _sink: &u64) -> bool {
        false
    }
    fn serialize_back_reference(&mut self, _raw: Tagged<HeapObject>) -> bool {
        false
    }
    fn check_rehashability(&self, _raw: Tagged<HeapObject>) {}
    fn serialize_in_object_cache(&mut self, _obj: Handle<HeapObject>) -> usize {
        0
    }

    fn reconstruct_read_only_and_shared_object_caches_for_testing(&self) -> bool {
        self.reconstruct_read_only_and_shared_object_caches_for_testing_
    }

    fn has_shared_space(&self) -> bool {
        false
    }
}

trait PutU32 {
    fn put_uint30(&mut self, value: u32, description: &str);
    fn put(&mut self, byte: u8, description: &str);
}

impl PutU32 for u64 { //SnapshotByteSink {
    fn put_uint30(&mut self, _value: u32, _description: &str) {}
    fn put(&mut self, _byte: u8, _description: &str) {}
}

impl Drop for SharedHeapSerializer {
    fn drop(&mut self) {
        self.output_statistics("SharedHeapSerializer");
    }
}

trait SharedSpace {
    fn shared_space_isolate(&self) -> &SharedSpaceIsolate;
    fn has_shared_space(&self) -> bool;
}

impl SharedSpace for Isolate {
    fn shared_space_isolate(&self) -> &SharedSpaceIsolate {
        todo!()
    }
    fn has_shared_space(&self) -> bool {
        false
    }
}