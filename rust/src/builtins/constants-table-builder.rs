// src/builtins/constants_table_builder.rs

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use crate::execution::isolate::Isolate;
use crate::heap::{Heap, HeapObject};
use crate::objects::{Object, HeapObjectImpl, FixedArray, ByteArray, Code, CodeKind, IsCode};
use crate::roots::{ReadOnlyRoots, RootIndex};
use crate::factory::Factory;
use crate::allocation::AllocationType;

pub struct BuiltinsConstantsTableBuilder<'i> {
    isolate_: &'i Isolate,
    map_: ConstantsMap,
    mutex_: Mutex<()>,
}

impl<'i> BuiltinsConstantsTableBuilder<'i> {
    pub fn new(isolate: &'i Isolate) -> Self {
        // Ensure this is only called once per Isolate.
        debug_assert_eq!(
            ReadOnlyRoots::new(isolate).empty_fixed_array(),
            isolate.heap().builtins_constants_table()
        );

        // And that the initial value of the builtins constants table can be treated
        // as a constant, which means that codegen will load it using the root
        // register.
        debug_assert!(ReadOnlyRoots::is_immortal_immovable(RootIndex::kEmptyFixedArray));

        BuiltinsConstantsTableBuilder {
            isolate_: isolate,
            map_: ConstantsMap::new(isolate.heap()),
            mutex_: Mutex::new(()),
        }
    }

    pub fn add_object(&mut self, object: &HeapObjectImpl) -> u32 {
        #[cfg(debug_assertions)]
        {
            // Roots must not be inserted into the constants table as they are already
            // accessibly from the root list.
            let mut root_list_index = RootIndex::kFirst;
            debug_assert!(!self.isolate_.roots_table().is_root_handle(object, &mut root_list_index));
            //debug_assert!(if object.is_map() {
            //    !HeapLayout::in_read_only_space(object.cast::<HeapObjectImpl>())
            //} else {
            //    true
            //});

            // Not yet finalized.
            debug_assert_eq!(
                ReadOnlyRoots::new(self.isolate_).empty_fixed_array(),
                self.isolate_.heap().builtins_constants_table()
            );

            // Must be generating embedded builtin code.
            debug_assert!(self.isolate_.is_generating_embedded_builtins());

            // All code objects should be loaded through the root register or use
            // pc-relative addressing.
            //debug_assert!(!object.is_instruction_stream());
        }

        // This method is called concurrently from both the main thread and
        // compilation threads. Constant indices need to be reproducible during
        // builtin generation, so the main thread pre-adds all the constants. A lock
        // is still needed since the map data structure is still being concurrently
        // accessed.
        let guard = self.mutex_.lock().unwrap();
        if thread::current().id() != self.isolate_.thread_id() {
            let find_result = self.map_.find(object);
            debug_assert!(find_result.is_some());
            *find_result.unwrap()
        } else {
            let find_result = self.map_.find_or_insert(object);
            if !find_result.already_exists {
                debug_assert!(HeapObjectImpl::is_heap_object(object));
                *find_result.entry = (self.map_.map.len() - 1) as u32;
            }
            *find_result.entry
        }
    }

    pub fn patch_self_reference(
        &mut self,
        self_reference: &mut HeapObjectImpl,
        code_object: &HeapObjectImpl,
    ) {
        Self::check_preconditions_for_patching(self.isolate_, code_object);
        debug_assert_eq!(
            *self_reference,
            ReadOnlyRoots::new(self.isolate_).self_reference_marker()
        );

        let mut key: u32 = 0;
        if self.map_.delete(self_reference, &mut key) {
            //DCHECK(IsInstructionStream(*code_object));
            self.map_.insert(code_object, key);
        }
    }

    pub fn patch_basic_block_counters_reference(
        &mut self,
        counters: &HeapObjectImpl,
    ) {
        Self::check_preconditions_for_patching(self.isolate_, counters);

        let mut key: u32 = 0;
        if self.map_.delete(&ReadOnlyRoots::new(self.isolate_).basic_block_counters_marker(), &mut key) {
            self.map_.insert(counters, key);
        }
    }

    pub fn finalize(&mut self) {
        //HandleScope handle_scope(isolate_);

        debug_assert_eq!(
            ReadOnlyRoots::new(self.isolate_).empty_fixed_array(),
            self.isolate_.heap().builtins_constants_table()
        );
        debug_assert!(self.isolate_.is_generating_embedded_builtins());

        // An empty map means there's nothing to do.
        if self.map_.map.is_empty() {
            return;
        }

        //DirectHandle<FixedArray> table =
        //    isolate_->factory()->NewFixedArray(map_.size(), AllocationType::kOld);

        let mut table = self.isolate_.factory().new_fixed_array(self.map_.map.len(), AllocationType::kOld);

        //Builtins* builtins = isolate_->builtins();
        //ConstantsMap::IteratableScope it_scope(&map_);

        for (i, (&key, &index)) in self.map_.map.iter().enumerate() {
            //uint32_t index = *it.entry();
            //Tagged<Object> value = it.key();
            let mut value = key.clone();
            if HeapObjectImpl::is_code(&value) {
                // Replace placeholder code objects with the real builtin.
                // See also: SetupIsolateDelegate::PopulateWithPlaceholders.
                // TODO(jgruber): Deduplicate placeholders and their corresponding
                // builtin.
                //value = builtins->code(Cast<Code>(value)->builtin_id());
                if let Some(code) = self.isolate_.builtins().code(value.clone()) {
                    value = code;
                }
            }
            //DCHECK(IsHeapObject(value));
            debug_assert!(HeapObjectImpl::is_heap_object(&value));
            table.set(index as usize, value);
        }

        #[cfg(debug_assertions)]
        for i in 0..self.map_.map.len() {
            debug_assert!(HeapObjectImpl::is_heap_object(&table.get(i)));
            debug_assert_ne!(
                ReadOnlyRoots::new(self.isolate_).undefined_value(),
                table.get(i)
            );
            debug_assert_ne!(
                ReadOnlyRoots::new(self.isolate_).self_reference_marker(),
                table.get(i)
            );
            debug_assert_ne!(
                ReadOnlyRoots::new(self.isolate_).basic_block_counters_marker(),
                table.get(i)
            );
        }

        self.isolate_.heap().set_builtins_constants_table(table);
    }

    fn check_preconditions_for_patching(
        isolate: &Isolate,
        replacement_object: &HeapObjectImpl,
    ) {
        // Roots must not be inserted into the constants table as they are already
        // accessible from the root list.
        let mut root_list_index = RootIndex::kFirst;
        debug_assert!(!isolate
            .roots_table()
            .is_root_handle(replacement_object, &mut root_list_index));

        // Not yet finalized.
        debug_assert_eq!(
            ReadOnlyRoots::new(isolate).empty_fixed_array(),
            isolate.heap().builtins_constants_table()
        );

        debug_assert!(isolate.is_generating_embedded_builtins());
    }
}

struct FindOrInsertResult<'a> {
    entry: &'a mut u32,
    already_exists: bool,
}

struct ConstantsMap {
    heap_: *const Heap,
    map: HashMap<HeapObjectImpl, u32>,
}

impl ConstantsMap {
    fn new(heap: &Heap) -> Self {
        ConstantsMap {
            heap_: heap,
            map: HashMap::new(),
        }
    }

    fn find(&self, object: &HeapObjectImpl) -> Option<&u32> {
        self.map.get(object)
    }

    fn find_or_insert(&mut self, object: &HeapObjectImpl) -> FindOrInsertResult {
        use std::collections::hash_map::Entry;
        match self.map.entry(object.clone()) {
            Entry::Occupied(entry) => {
                let value = entry.into_mut();
                FindOrInsertResult {
                    entry: value,
                    already_exists: true,
                }
            }
            Entry::Vacant(entry) => {
                let value = entry.insert(0); // Assign a dummy value here
                FindOrInsertResult {
                    entry: value,
                    already_exists: false,
                }
            }
        }
    }

    fn insert(&mut self, object: &HeapObjectImpl, key: u32) {
        self.map.insert(object.clone(), key);
    }

    fn delete(&mut self, object: &HeapObjectImpl, key: &mut u32) -> bool {
        if let Some(value) = self.map.remove(object) {
            *key = value;
            true
        } else {
            false
        }
    }

    fn empty(&self) -> bool {
        self.map.is_empty()
    }
}

struct IteratableScope<'a> {
    map_: &'a ConstantsMap,
    // Add any fields needed for iteration here
}

impl<'a> IteratableScope<'a> {
    fn new(map: &'a ConstantsMap) -> Self {
        IteratableScope {
            map_: map,
        }
    }

    fn begin(&self) -> ConstantsMapIterator<'a> {
        ConstantsMapIterator {
            map_: self.map_,
            iter_: self.map_.map.iter(),
        }
    }
}

struct ConstantsMapIterator<'a> {
    map_: &'a ConstantsMap,
    iter_: std::collections::hash_map::Iter<'a, HeapObjectImpl, u32>,
}

impl<'a> ConstantsMapIterator<'a> {
    fn entry(&self) -> &u32 {
        self.iter_.next().map(|(_, v)| v).unwrap()
    }

    fn key(&self) -> &HeapObjectImpl {
        self.iter_.next().map(|(k, _)| k).unwrap()
    }
}