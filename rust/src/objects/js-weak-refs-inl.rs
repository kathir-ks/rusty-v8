// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a Rust translation of the C++ header file:
// /home/kathirks_gc/v8_go/codebase/src/objects/js-weak-refs-inl.h

pub mod js_weak_refs {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::ptr::NonNull;
    use std::sync::atomic::{AtomicBool, Ordering};

    // Placeholder for Torque generated code
    mod torque_generated {
        pub mod js_weak_refs_tq_inl {
            //empty
        }
    }

    macro_rules! tq_object_constructors_impl {
        ($struct_name:ident) => {
            // No constructor implementation needed in Rust, using Default::default()
        };
    }

    macro_rules! bit_field_accessors {
        ($struct_name:ident, $field:ident, $accessor:ident, $bit:expr) => {
            impl $struct_name {
                pub fn $accessor(&self) -> bool {
                    (self.$field.get() & $bit) != 0
                }

                pub fn set_$accessor(&self, value: bool) {
                    if value {
                        self.$field.set(self.$field.get() | $bit);
                    } else {
                        self.$field.set(self.$field.get() & !$bit);
                    }
                }
            }
        };
    }

    #[derive(Default)]
    pub struct WeakCell {
        target: Cell<Option<NonNull<HeapObject>>>, //Tagged<HeapObject>
        unregister_token: Cell<Option<NonNull<HeapObject>>>, //Tagged<HeapObject>
        key_list_prev: Cell<Option<NonNull<WeakCell>>>, //Tagged<UnionOf<Undefined, WeakCell>>
        key_list_next: Cell<Option<NonNull<WeakCell>>>, //Tagged<UnionOf<Undefined, WeakCell>>
        prev: Cell<Option<NonNull<WeakCell>>>, //Tagged<UnionOf<Undefined, WeakCell>>
        next: Cell<Option<NonNull<WeakCell>>>, //Tagged<UnionOf<Undefined, WeakCell>>
        finalization_registry: Cell<Option<NonNull<JSFinalizationRegistry>>>, // Tagged<JSFinalizationRegistry>
    }

    impl WeakCell {
        // Placeholder for offsets, replace with actual values if needed
        const K_TARGET_OFFSET: usize = 0;
        const K_UNREGISTER_TOKEN_OFFSET: usize = 1;
        const K_KEY_LIST_PREV_OFFSET: usize = 2;
        const K_KEY_LIST_NEXT_OFFSET: usize = 3;
        const K_PREV_OFFSET: usize = 4;
        const K_NEXT_OFFSET: usize = 5;

        pub fn target(&self) -> Option<NonNull<HeapObject>> {
            self.target.get()
        }

        pub fn set_target(&self, target: Option<NonNull<HeapObject>>) {
            self.target.set(target);
        }

        pub fn unregister_token(&self) -> Option<NonNull<HeapObject>> {
            self.unregister_token.get()
        }

        pub fn set_unregister_token(&self, token: Option<NonNull<HeapObject>>) {
            self.unregister_token.set(token);
        }
        
        pub fn key_list_prev(&self) -> Option<NonNull<WeakCell>> {
            self.key_list_prev.get()
        }

        pub fn set_key_list_prev(&self, prev: Option<NonNull<WeakCell>>) {
            self.key_list_prev.set(prev);
        }
        
        pub fn key_list_next(&self) -> Option<NonNull<WeakCell>> {
            self.key_list_next.get()
        }

        pub fn set_key_list_next(&self, next: Option<NonNull<WeakCell>>) {
            self.key_list_next.set(next);
        }
                
        pub fn prev(&self) -> Option<NonNull<WeakCell>> {
            self.prev.get()
        }

        pub fn set_prev(&self, prev: Option<NonNull<WeakCell>>) {
            self.prev.set(prev);
        }
        
        pub fn next(&self) -> Option<NonNull<WeakCell>> {
            self.next.get()
        }

        pub fn set_next(&self, next: Option<NonNull<WeakCell>>) {
            self.next.set(next);
        }

        pub fn finalization_registry(&self) -> Option<NonNull<JSFinalizationRegistry>> {
          self.finalization_registry.get()
        }

        pub fn set_finalization_registry(&self, registry: Option<NonNull<JSFinalizationRegistry>>) {
          self.finalization_registry.set(registry);
        }

        pub fn relaxed_target(&self) -> Option<NonNull<HeapObject>> {
            // In Rust, Cell::get provides similar semantics to Relaxed_Load.
            self.target.get()
        }

        pub fn relaxed_unregister_token(&self) -> Option<NonNull<HeapObject>> {
            self.unregister_token.get()
        }

        pub fn nullify<F>(&self, isolate: &Isolate, gc_notify_updated_slot: F)
        where
            F: Fn(&WeakCell, usize, Option<NonNull<HeapObject>>),
        {
            if self.target().is_none() {
                return;
            }

            self.set_target(None);

            unsafe {
                let fr = self.finalization_registry().unwrap().as_ref();

                if let Some(prev) = self.prev().map(|p| p.as_ref()) {
                    if fr.active_cells.get() == Some(NonNull::from(self)) {
                        panic!("DCHECK_NE failed");
                    }
                    prev.set_next(self.next());
                    gc_notify_updated_slot(prev, Self::K_NEXT_OFFSET, self.next().map(|n| n.cast::<HeapObject>()));
                } else {
                    if fr.active_cells.get() != Some(NonNull::from(self)) {
                        panic!("DCHECK_EQ failed");
                    }
                    fr.active_cells.set(self.next());
                    gc_notify_updated_slot(
                        fr.into_heap_object_ref().unwrap(),
                        JSFinalizationRegistry::K_ACTIVE_CELLS_OFFSET,
                        self.next().map(|n| n.cast::<HeapObject>()),
                    );
                }

                if let Some(next) = self.next().map(|n| n.as_ref()) {
                    next.set_prev(self.prev());
                    gc_notify_updated_slot(next, Self::K_PREV_OFFSET, self.prev().map(|p| p.cast::<HeapObject>()));
                }

                self.set_prev(None);

                let cleared_head = fr.cleared_cells.get();

                if let Some(cleared_head_cell) = cleared_head.map(|ch| ch.as_ref()) {
                    cleared_head_cell.set_prev(Some(NonNull::from(self)));
                    gc_notify_updated_slot(
                        cleared_head_cell,
                        Self::K_PREV_OFFSET,
                        Some(NonNull::from(self).cast::<HeapObject>()),
                    );
                }
                self.set_next(fr.cleared_cells.get());
                gc_notify_updated_slot(
                    self,
                    Self::K_NEXT_OFFSET,
                    fr.cleared_cells.get().map(|n| n.cast::<HeapObject>()),
                );
                fr.cleared_cells.set(Some(NonNull::from(self)));
                gc_notify_updated_slot(
                    fr.into_heap_object_ref().unwrap(),
                    JSFinalizationRegistry::K_CLEARED_CELLS_OFFSET,
                    Some(NonNull::from(self).cast::<HeapObject>()),
                );
            }
        }

        pub fn remove_from_finalization_registry_cells(&self, isolate: &Isolate) {
          // This function modifies the WeakCell and potentially other WeakCell instances in the FinalizationRegistry's active_cells or cleared_cells list.
          self.set_target(None); //Mark target as undefined

          unsafe {
            let fr = self.finalization_registry().unwrap().as_ref();

            if fr.active_cells.get() == Some(NonNull::from(self)) {
              if self.prev().is_some() {
                panic!("DCHECK failed: prev() must be None when this cell is the head of active_cells");
              }
              fr.active_cells.set(self.next());
            } else if fr.cleared_cells.get() == Some(NonNull::from(self)) {
              if self.prev().is_some() {
                  panic!("DCHECK failed: prev() must be None when this cell is the head of cleared_cells");
              }
              fr.cleared_cells.set(self.next());
            } else {
              if self.prev().is_none() {
                panic!("DCHECK failed: prev() must be Some if this cell is not the head of either active_cells or cleared_cells");
              }
              let prev_cell = self.prev().unwrap().as_ref();
              prev_cell.set_next(self.next());
            }

            if let Some(next_cell) = self.next().map(|n| n.as_ref()) {
              next_cell.set_prev(self.prev());
            }
            self.set_prev(None);
            self.set_next(None);
          }
        }

        // Helper function to simulate RawField access.  Assumes that memory layout
        // of WeakCell struct is the same as the C++ counterpart (which is not guaranteed).
        #[allow(dead_code)]
        unsafe fn raw_field(&self, offset: usize) -> &Cell<Option<NonNull<HeapObject>>> {
          match offset {
              WeakCell::K_TARGET_OFFSET => &self.target,
              WeakCell::K_UNREGISTER_TOKEN_OFFSET => &self.unregister_token,
              WeakCell::K_KEY_LIST_PREV_OFFSET => &self.key_list_prev,
              WeakCell::K_KEY_LIST_NEXT_OFFSET => &self.key_list_next,
              WeakCell::K_PREV_OFFSET => &self.prev,
              WeakCell::K_NEXT_OFFSET => &self.next,
              _ => panic!("Invalid offset"),
          }
        }
    }

    #[derive(Default)]
    pub struct JSWeakRef {
        // Fields specific to JSWeakRef, if any
    }

    #[derive(Default)]
    pub struct JSFinalizationRegistry {
        flags: Cell<u8>,
        key_map: Cell<Option<NonNull<SimpleNumberDictionary>>>, //Tagged<SimpleNumberDictionary>
        active_cells: Cell<Option<NonNull<WeakCell>>>, //Tagged<UnionOf<Undefined, WeakCell>>
        cleared_cells: Cell<Option<NonNull<WeakCell>>>,//Tagged<UnionOf<Undefined, WeakCell>>
    }

    impl JSFinalizationRegistry {
        pub const SCHEDULED_FOR_CLEANUP_BIT: u8 = 1 << 0;
        const K_ACTIVE_CELLS_OFFSET: usize = 0;
        const K_CLEARED_CELLS_OFFSET: usize = 1;

        pub fn into_heap_object_ref(&self) -> Option<NonNull<JSFinalizationRegistry>> {
            unsafe { NonNull::new(self as *const JSFinalizationRegistry as *mut JSFinalizationRegistry) }
        }

        pub fn key_map(&self) -> Option<NonNull<SimpleNumberDictionary>> {
            self.key_map.get()
        }

        pub fn set_key_map(&self, key_map: Option<NonNull<SimpleNumberDictionary>>) {
            self.key_map.set(key_map);
        }
        
        pub fn active_cells(&self) -> Option<NonNull<WeakCell>> {
            self.active_cells.get()
        }

        pub fn set_active_cells(&self, active_cells: Option<NonNull<WeakCell>>) {
            self.active_cells.set(active_cells);
        }
        
        pub fn cleared_cells(&self) -> Option<NonNull<WeakCell>> {
            self.cleared_cells.get()
        }

        pub fn set_cleared_cells(&self, cleared_cells: Option<NonNull<WeakCell>>) {
            self.cleared_cells.set(cleared_cells);
        }

        bit_field_accessors!(
            JSFinalizationRegistry,
            flags,
            scheduled_for_cleanup,
            JSFinalizationRegistry::SCHEDULED_FOR_CLEANUP_BIT
        );

        pub fn register_weak_cell_with_unregister_token(
            &self,
            finalization_registry_handle: NonNull<JSFinalizationRegistry>,
            weak_cell_handle: NonNull<WeakCell>,
            isolate: &Isolate,
        ) {
            unsafe {
                let mut key_map = if self.key_map().is_none() {
                    SimpleNumberDictionary::new(isolate, 1)
                } else {
                    SimpleNumberDictionary::from_raw(self.key_map().unwrap())
                };

                let weak_cell = weak_cell_handle.as_ref();
                let unregister_token = weak_cell.unregister_token().unwrap();
                let key = Object::get_or_create_hash(unregister_token, isolate).value();

                let entry = key_map.find_entry(isolate, key);
                if entry.is_found() {
                    let value = key_map.value_at(entry).unwrap().cast::<WeakCell>();
                    let existing_weak_cell = value.as_ref();
                    existing_weak_cell.set_key_list_prev(Some(weak_cell_handle));
                    weak_cell.set_key_list_next(Some(NonNull::from(existing_weak_cell)));
                }
                key_map.set(isolate, key, NonNull::from(weak_cell));
                self.set_key_map(Some(key_map.into_raw()));
            }
        }

        pub fn unregister(
            &self,
            finalization_registry_handle: NonNull<JSFinalizationRegistry>,
            unregister_token: NonNull<HeapObject>,
            isolate: &Isolate,
        ) -> bool {
            self.remove_unregister_token(
                unregister_token,
                isolate,
                RemoveUnregisterTokenMode::RemoveMatchedCellsFromRegistry,
                |_, _, _| {},
            )
        }

        fn remove_unregister_token<F>(
            &self,
            unregister_token: NonNull<HeapObject>,
            isolate: &Isolate,
            removal_mode: RemoveUnregisterTokenMode,
            gc_notify_updated_slot: F,
        ) -> bool
        where
            F: Fn(&WeakCell, usize, Option<NonNull<HeapObject>>),
        {
            if self.key_map().is_none() {
                return false;
            }

            let key_map = unsafe { SimpleNumberDictionary::from_raw(self.key_map().unwrap()) };
            
            let hash = Object::get_hash(unregister_token, isolate);
            if hash.is_none() {
                return false;
            }

            let key = Smi::to_int(hash.unwrap());
            let entry = key_map.find_entry(isolate, key);
            if entry.is_not_found() {
                return false;
            }

            let mut value = key_map.value_at(entry);
            let mut was_present = false;
            let undefined: Option<NonNull<HeapObject>> = None; // Assume Undefined is represented by None for now

            unsafe {
                let mut new_key_list_head: Option<NonNull<WeakCell>> = None;
                let mut new_key_list_prev: Option<NonNull<WeakCell>> = None;

                while value.is_some() {
                    let weak_cell = value.unwrap().cast::<WeakCell>().as_ref();
                    value = weak_cell.key_list_next();
                    
                    if weak_cell.unregister_token() == Some(unregister_token) {
                        match removal_mode {
                            RemoveUnregisterTokenMode::RemoveMatchedCellsFromRegistry => {
                                weak_cell.remove_from_finalization_registry_cells(isolate);
                            }
                            RemoveUnregisterTokenMode::KeepMatchedCellsInRegistry => {
                                // Do nothing.
                            }
                        }

                        weak_cell.set_unregister_token(None);
                        weak_cell.set_key_list_prev(None);
                        weak_cell.set_key_list_next(None);
                        was_present = true;
                    } else {
                        weak_cell.set_key_list_prev(new_key_list_prev);
                        gc_notify_updated_slot(weak_cell, WeakCell::K_KEY_LIST_PREV_OFFSET, new_key_list_prev.map(|p| p.cast::<HeapObject>()));
                        weak_cell.set_key_list_next(None);
                        
                        if new_key_list_prev.is_none() {
                            new_key_list_head = Some(NonNull::from(weak_cell));
                        } else {
                            let prev_cell = new_key_list_prev.unwrap().as_ref();
                            prev_cell.set_key_list_next(Some(NonNull::from(weak_cell)));
                            gc_notify_updated_slot(
                                prev_cell,
                                WeakCell::K_KEY_LIST_NEXT_OFFSET,
                                Some(NonNull::from(weak_cell).cast::<HeapObject>()),
                            );
                        }
                        new_key_list_prev = Some(NonNull::from(weak_cell));
                    }
                }

                if new_key_list_head.is_none() {
                    if !was_present {
                        panic!("DCHECK failed: was_present should be true");
                    }
                    key_map.clear_entry(entry);
                    key_map.element_removed();
                } else {
                    key_map.value_at_put(entry, new_key_list_head.map(|h| h.cast::<HeapObject>()));
                    gc_notify_updated_slot(
                        key_map.into_raw().as_ref(),
                        SimpleNumberDictionary::raw_field_of_value_at(entry),
                        new_key_list_head.map(|h| h.cast::<HeapObject>()),
                    );
                }
            }

            was_present
        }

        pub fn needs_cleanup(&self) -> bool {
            self.cleared_cells().is_some()
        }
    }

    enum RemoveUnregisterTokenMode {
        RemoveMatchedCellsFromRegistry,
        KeepMatchedCellsInRegistry,
    }

    // Dummy structs and enums to represent V8 types
    #[derive(Debug, Clone, Copy)]
    pub struct InternalIndex {
        index: usize,
        found: bool,
    }

    impl InternalIndex {
        pub fn is_found(&self) -> bool {
            self.found
        }

        pub fn is_not_found(&self) -> bool {
            !self.found
        }
    }

    pub struct Isolate {}

    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }

    pub struct ReadOnlyRoots {
        undefined_value: Option<NonNull<HeapObject>>,
    }

    impl ReadOnlyRoots {
        pub fn undefined_value(&self) -> Option<NonNull<HeapObject>> {
            self.undefined_value
        }
    }

    impl Isolate {
        pub fn read_only_roots(&self) -> ReadOnlyRoots {
            ReadOnlyRoots {
                undefined_value: None, // Replace with actual undefined value if available
            }
        }
    }

    #[derive(Default)]
    pub struct SimpleNumberDictionary {
        entries: Vec<Option<NonNull<HeapObject>>>,
        elements_removed: usize,
    }

    impl SimpleNumberDictionary {
        pub fn new(isolate: &Isolate, capacity: usize) -> Self {
            SimpleNumberDictionary {
                entries: vec![None; capacity],
                elements_removed: 0,
            }
        }
        
        pub unsafe fn from_raw(ptr: NonNull<SimpleNumberDictionary>) -> Self {
          std::ptr::read(ptr.as_ptr())
        }

        pub fn into_raw(self) -> NonNull<SimpleNumberDictionary> {
            unsafe {
                let boxed = Box::new(self);
                NonNull::new(Box::into_raw(boxed)).unwrap()
            }
        }

        pub fn find_entry(&self, isolate: &Isolate, key: u32) -> InternalIndex {
            // Simple linear search for demonstration purposes
            for (index, entry) in self.entries.iter().enumerate() {
                if let Some(heap_object) = entry {
                    let hash = Object::get_hash(*heap_object, isolate);
                    if let Some(h) = hash {
                        if Smi::to_int(h) as u32 == key {
                            return InternalIndex {
                                index,
                                found: true,
                            };
                        }
                    }
                }
            }
            InternalIndex {
                index: 0,
                found: false,
            }
        }

        pub fn value_at(&self, entry: InternalIndex) -> Option<NonNull<HeapObject>> {
            if entry.is_found() {
                self.entries[entry.index]
            } else {
                None
            }
        }
        
        pub fn raw_field_of_value_at(entry: usize) -> usize {
          entry
        }

        pub fn value_at_put(&mut self, entry: InternalIndex, value: Option<NonNull<HeapObject>>) {
            if entry.is_found() {
                self.entries[entry.index] = value;
            }
        }

        pub fn set(&mut self, isolate: &Isolate, key: u32, value: NonNull<WeakCell>) {
            // Simple linear search for demonstration purposes
            for (index, entry) in self.entries.iter_mut().enumerate() {
                if entry.is_none() {
                    // Store the hash of the value in the dictionary
                    unsafe {
                        let obj_ptr = value.as_ptr() as *mut HeapObject;
                        let obj = NonNull::new(obj_ptr).unwrap();
                        let hash_val = Object::get_or_create_hash(obj, isolate).value();
                        if hash_val == key {
                            *entry = Some(NonNull::new(value.as_ptr() as *mut HeapObject).unwrap());
                            return;
                        }
                    }
                }
            }
            // If no empty slot is found, resize the vector and add the new entry.
            self.entries.push(Some(NonNull::new(value.as_ptr() as *mut HeapObject).unwrap()));
        }

        pub fn clear_entry(&mut self, entry: InternalIndex) {
            if entry.is_found() {
                self.entries[entry.index] = None;
            }
        }

        pub fn element_removed(&mut self) {
            self.elements_removed += 1;
        }
    }

    #[derive(Clone, Copy)]
    pub struct Smi {
        value: usize,
    }

    impl Smi {
        pub fn to_int(heap_object: NonNull<HeapObject>) -> usize {
            unsafe {
                let ptr = heap_object.as_ptr() as usize;
                ptr >> 1
            }
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct HeapObject {}

    impl HeapObject {
        pub fn get_hash(obj: NonNull<HeapObject>, isolate: &Isolate) -> Option<NonNull<HeapObject>> {
            Some(obj)
        }

        pub fn set_hash(obj: NonNull<HeapObject>, hash: NonNull<HeapObject>, isolate: &Isolate) {
            
        }
    }

    impl HeapObject {
      pub fn cast<T>(self) -> T {
        unsafe { std::mem::transmute_copy(&self) }
      }
    }

    pub struct Object {}

    impl Object {
        pub fn get_hash(unregister_token: NonNull<HeapObject>, isolate: &Isolate) -> Option<NonNull<HeapObject>> {
          Some(unregister_token)
        }

        pub fn get_or_create_hash(unregister_token: NonNull<HeapObject>, isolate: &Isolate) -> Smi {
          Smi {
            value: unsafe { unregister_token.as_ptr() as usize } // This is a dummy implementation.
          }
        }

        pub fn can_be_held_weakly(target: Option<NonNull<HeapObject>>) -> bool {
          target.is_some()
        }
    }

    tq_object_constructors_impl!(WeakCell);
    tq_object_constructors_impl!(JSWeakRef);
    tq_object_constructors_impl!(JSFinalizationRegistry);
}