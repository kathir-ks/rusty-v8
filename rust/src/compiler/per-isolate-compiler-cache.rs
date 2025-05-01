// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod compiler {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct ObjectData {}

    pub struct RefsMap {} // Placeholder for now

    impl RefsMap {
        pub fn is_empty(&self) -> bool {
            true // Placeholder.  Needs actual implementation.
        }
    }

    /// This class serves as a container of data that should persist across all
    /// (optimizing) compiler runs in an isolate. For now it stores serialized data
    /// for various common objects such as builtins, so that these objects don't have
    /// to be serialized in each compilation job. See JSHeapBroker::InitializeRefsMap
    /// for details.
    pub struct PerIsolateCompilerCache {
        zone: Rc<Zone>,
        refs_snapshot: RefCell<Option<Rc<RefsMap>>>,
    }

    impl PerIsolateCompilerCache {
        pub fn new(zone: Rc<Zone>) -> Self {
            PerIsolateCompilerCache {
                zone,
                refs_snapshot: RefCell::new(None),
            }
        }

        pub fn has_snapshot(&self) -> bool {
            self.refs_snapshot.borrow().is_some()
        }

        pub fn get_snapshot(&self) -> Option<Rc<RefsMap>> {
            // TODO: Add Debug Assertion.
            self.refs_snapshot.borrow().clone()
        }

        pub fn set_snapshot(&self, refs: Rc<RefsMap>) {
            // TODO: Add Debug Assertion.
            // TODO: Add Debug Assertion that refs is not empty
            *self.refs_snapshot.borrow_mut() = Some(refs);
            // TODO: Add Debug Assertion
        }

        pub fn zone(&self) -> &Rc<Zone> {
            &self.zone
        }
    }

    pub struct Isolate {
        compiler_cache: RefCell<Option<Rc<PerIsolateCompilerCache>>>,
        compiler_zone: RefCell<Option<Rc<Zone>>>,
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                compiler_cache: RefCell::new(None),
                compiler_zone: RefCell::new(None),
            }
        }

        pub fn compiler_cache(&self) -> Option<Rc<PerIsolateCompilerCache>> {
            self.compiler_cache.borrow().clone()
        }

        pub fn set_compiler_utils(&self, cache: Rc<PerIsolateCompilerCache>, zone: Rc<Zone>) {
            *self.compiler_cache.borrow_mut() = Some(cache);
            *self.compiler_zone.borrow_mut() = Some(zone);
        }
    }

    pub struct Zone {
        // allocator: Allocator, //Placeholder
        name: String,
    }

    impl Zone {
        pub fn new(name: String) -> Self {
            Zone {
                // allocator,
                name,
            }
        }

        pub fn new_per_isolate_compiler_cache(&self) -> Rc<PerIsolateCompilerCache> {
            Rc::new(PerIsolateCompilerCache::new(Rc::new(Zone::new("Compiler zone".to_string()))))
        }

    }

    pub fn setup(isolate: &Isolate) {
        if isolate.compiler_cache().is_none() {
            let zone = Rc::new(Zone::new("Compiler zone".to_string()));
            let cache = Rc::new(PerIsolateCompilerCache::new(zone.clone()));
            isolate.set_compiler_utils(cache, zone);
        }
        // TODO: Add Debug assertion
    }

}