// Converted from V8 C++ source files:
// Header: object-deserializer.h
// Implementation: object-deserializer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/snapshot/object-deserializer.h
pub mod object_deserializer {
    use crate::execution::isolate::Isolate;
    use crate::snapshot::deserializer::Deserializer;
    use crate::objects::objects::HeapObject;
    use crate::snapshot::snapshot::SharedFunctionInfo;
    use crate::objects::objects::String;
    use crate::snapshot::snapshot::V8;

    pub struct SerializedCodeData {
        payload: Vec<u8>,
        magic_number: i32,
    }

    impl SerializedCodeData {
        pub fn new(payload: Vec<u8>, magic_number: i32) -> Self {
            SerializedCodeData { payload, magic_number }
        }

        pub fn Payload(&self) -> &[u8] {
            &self.payload
        }

        pub fn GetMagicNumber(&self) -> i32 {
            self.magic_number
        }
    }

    pub struct DirectHandle<T>(*mut T);

    impl<T> DirectHandle<T> {
        pub fn new(ptr: *mut T) -> Self {
            DirectHandle(ptr)
        }
        pub fn ToHandle(&self, result: &mut Self) -> bool {
            if self.0.is_null() {
                return false;
            }
            result.0 = self.0;
            true
        }
    }

    pub struct MaybeDirectHandle<T>(*mut T);

    impl<T> MaybeDirectHandle<T> {
        pub fn new(ptr: *mut T) -> Self {
            MaybeDirectHandle(ptr)
        }

        pub fn ToHandle(&self, result: &mut DirectHandle<T>) -> bool {
            if self.0.is_null() {
                return false;
            }
            result.0 = self.0;
            true
        }
    }

    // Deserializes the object graph rooted at a given object.
    pub struct ObjectDeserializer<'a> {
        deserializer: Deserializer<'a, Isolate>,
    }

    impl<'a> ObjectDeserializer<'a> {
        pub fn DeserializeSharedFunctionInfo(
            isolate: &'a mut Isolate,
            data: &SerializedCodeData,
            source: DirectHandle<String>,
        ) -> MaybeDirectHandle<SharedFunctionInfo> {
            let mut d = ObjectDeserializer::new(isolate, data);

            d.deserializer.AddAttachedObject(source);

            let mut result = DirectHandle::<HeapObject>::new(std::ptr::null_mut());
            if d.Deserialize().ToHandle(&mut result) {
                MaybeDirectHandle(result.0 as *mut SharedFunctionInfo)
            } else {
                MaybeDirectHandle(std::ptr::null_mut())
            }
        }

        fn new(isolate: &'a mut Isolate, data: &SerializedCodeData) -> Self {
            let magic_number = data.GetMagicNumber();
            let payload = data.Payload();
            ObjectDeserializer {
                deserializer: Deserializer::new(isolate, payload, magic_number, true, false),
            }
        }

        // Deserialize an object graph. Fail gracefully.
        fn Deserialize(&mut self) -> MaybeDirectHandle<HeapObject> {
            if !self.deserializer.deserializing_user_code() {
                return MaybeDirectHandle(std::ptr::null_mut());
            }

            let mut result = DirectHandle::<HeapObject>::new(std::ptr::null_mut());
            {
                result.0 = self.deserializer.ReadObject().0;
                self.deserializer.DeserializeDeferredObjects();
                assert!(self.deserializer.new_code_objects().is_empty());
                self.LinkAllocationSites();
                assert!(self.deserializer.new_maps().is_empty());
                self.deserializer.WeakenDescriptorArrays();
            }

            self.deserializer.Rehash();
            self.CommitPostProcessedObjects();
            MaybeDirectHandle(result.0)
        }

        fn CommitPostProcessedObjects(&mut self) {
            for script in self.deserializer.new_scripts().iter() {
                // Assign a new script id to avoid collision.
                script.set_id(self.deserializer.isolate().GetNextScriptId());
                self.deserializer.LogScriptEvents(*script);

                // Add script to list.
                let list = self.deserializer.isolate().factory().script_list();
                let list = self.deserializer.isolate().WeakArrayList_AddToEnd(list,
                                                       self.deserializer.MaybeObjectDirectHandle_Weak(*script));
                self.deserializer.isolate().heap().SetRootScriptList(*list);
            }
        }

        fn LinkAllocationSites(&mut self) {
            // Allocation sites are present in the snapshot, and must be linked into
            // a list at deserialization time.
            for site in self.deserializer.new_allocation_sites().iter() {
                if !site.HasWeakNext() {
                    continue;
                }

                let heap = self.deserializer.isolate().heap();
                if heap.allocation_sites_list().is_zero() {
                    site.set_weak_next(self.deserializer.ReadOnlyRoots_undefined_value(heap));
                } else {
                    site.set_weak_next(heap.allocation_sites_list());
                }
                heap.set_allocation_sites_list(*site);
            }
        }
    }

    pub struct LocalIsolate {}

    impl LocalIsolate {
        pub fn factory(&self) -> LocalFactory {
            LocalFactory {}
        }
        pub fn GetNextScriptId(&mut self) -> i32 {
            1
        }
        pub fn heap(&self) -> LocalHeap {
            LocalHeap{}
        }
    }

    pub struct LocalFactory {}
    impl LocalFactory{
        pub fn empty_string(&self) -> DirectHandle<String> {
            DirectHandle(std::ptr::null_mut())
        }
    }

    pub struct IndirectHandle<T>(*mut T);

    pub struct LocalHeap {}
    impl LocalHeap {
        pub fn NewPersistentHandle<T>(&self, script: &DirectHandle<T>) -> IndirectHandle<T>{
            IndirectHandle(script.0)
        }
    }

    // Deserializes the object graph rooted at a given object.
    pub struct OffThreadObjectDeserializer<'a> {
        deserializer: Deserializer<'a, LocalIsolate>,
    }

    impl<'a> OffThreadObjectDeserializer<'a> {
        pub fn DeserializeSharedFunctionInfo(
            isolate: &'a mut LocalIsolate,
            data: &SerializedCodeData,
            deserialized_scripts: &mut Vec<IndirectHandle<String>>,
        ) -> MaybeDirectHandle<SharedFunctionInfo> {
            let mut d = OffThreadObjectDeserializer::new(isolate, data);

            // Attach the empty string as the source.
            d.deserializer.AddAttachedObject(isolate.factory().empty_string());

            let mut result = DirectHandle::<HeapObject>::new(std::ptr::null_mut());
            if !d.Deserialize(deserialized_scripts).ToHandle(&mut result) {
                return MaybeDirectHandle(std::ptr::null_mut());
            }
            MaybeDirectHandle(result.0 as *mut SharedFunctionInfo)
        }

        fn new(isolate: &'a mut LocalIsolate, data: &SerializedCodeData) -> Self {
            let magic_number = data.GetMagicNumber();
            let payload = data.Payload();
            OffThreadObjectDeserializer {
                deserializer: Deserializer::new(isolate, payload, magic_number, true, false),
            }
        }

        fn Deserialize(&mut self, deserialized_scripts: &mut Vec<IndirectHandle<String>>)
                       -> MaybeDirectHandle<HeapObject> {
            if !self.deserializer.deserializing_user_code() {
                return MaybeDirectHandle(std::ptr::null_mut());
            }

            let mut result = DirectHandle::<HeapObject>::new(std::ptr::null_mut());
            {
                result.0 = self.deserializer.ReadObject().0;
                self.deserializer.DeserializeDeferredObjects();
                assert!(self.deserializer.new_code_objects().is_empty());
                assert!(self.deserializer.new_allocation_sites().is_empty());
                assert!(self.deserializer.new_maps().is_empty());
                self.deserializer.WeakenDescriptorArrays();
            }

            self.deserializer.Rehash();

            assert_eq!(self.deserializer.new_scripts().len(), 1);
            for script in self.deserializer.new_scripts().iter() {
                script.set_id(self.deserializer.isolate().GetNextScriptId());
                self.deserializer.LogScriptEvents(*script);
                deserialized_scripts.push(self.deserializer.isolate().heap().NewPersistentHandle(script));
            }

            MaybeDirectHandle(result.0)
        }
    }

    pub trait AllocationSiteTrait {
        fn HasWeakNext(&self) -> bool;
        fn set_weak_next(&self, value: *mut i32);
    }
    pub struct AllocationSite {}

    impl AllocationSiteTrait for AllocationSite{
        fn HasWeakNext(&self) -> bool {
            true
        }
        fn set_weak_next(&self, value: *mut i32){

        }
    }
    pub trait ScriptTrait {
        fn set_id(&self, id: i32);
    }
    pub struct Script {}

    impl ScriptTrait for Script{
        fn set_id(&self, id: i32){

        }
    }
}

// src/snapshot/object-deserializer.cc
pub mod object_deserializer_impl {
    use crate::execution::isolate::Isolate;
    use crate::heap::heap::Heap;
    use crate::heap::heap::ReadOnlyRoots;
    use crate::objects::objects::Smi;
    use crate::snapshot::object_deserializer::AllocationSiteTrait;
    use crate::snapshot::object_deserializer::DirectHandle;
    use crate::snapshot::object_deserializer::ObjectDeserializer;
    use crate::snapshot::object_deserializer::ScriptTrait;

    pub trait HeapObjectTrait{
        fn Size(&self) -> usize;
    }
    pub struct HeapObject{}
    impl HeapObjectTrait for HeapObject{
        fn Size(&self) -> usize{
            1
        }
    }

    pub trait DisallowGarbageCollectionTrait{}
    pub struct DisallowGarbageCollection{}
    impl DisallowGarbageCollectionTrait for DisallowGarbageCollection{}

    impl<'a> ObjectDeserializer<'a> {
        // fn CommitPostProcessedObjects(&mut self) {
        //     for script in self.deserializer.new_scripts().iter() {
        //         // Assign a new script id to avoid collision.
        //         script.set_id(self.deserializer.isolate().GetNextScriptId());
        //         self.deserializer.LogScriptEvents(*script);

        //         // Add script to list.
        //         let list = self.deserializer.isolate().factory().script_list();
        //         let list = WeakArrayList::AddToEnd(self.deserializer.isolate(), list,
        //                                            MaybeObjectDirectHandle::Weak(script));
        //         self.deserializer.isolate().heap().SetRootScriptList(*list);
        //     }
        // }

        // fn LinkAllocationSites(&mut self) {
        //     let _no_gc = DisallowGarbageCollection{};
        //     let heap = self.deserializer.isolate().heap();
        //     // Allocation sites are present in the snapshot, and must be linked into
        //     // a list at deserialization time.
        //     for site in self.deserializer.new_allocation_sites().iter() {
        //         if !site.HasWeakNext() {
        //             continue;
        //         }

        //         if heap.allocation_sites_list() == Smi::zero() {
        //             site.set_weak_next(ReadOnlyRoots::undefined_value(heap));
        //         } else {
        //             site.set_weak_next(heap.allocation_sites_list());
        //         }
        //         heap.set_allocation_sites_list(*site);
        //     }
        // }
    }

    pub struct WeakArrayList {}
    impl WeakArrayList{
        pub fn AddToEnd(arg0:&Isolate, arg1: &WeakArrayList, arg2: DirectHandle<HeapObject>) -> *mut i32{
            std::ptr::null_mut()
        }
    }

    pub trait FactoryTrait {
        fn script_list(&self) -> *mut i32;
    }
    pub struct Factory {}
    impl FactoryTrait for Factory{
        fn script_list(&self) -> *mut i32{
            std::ptr::null_mut()
        }
    }
}
