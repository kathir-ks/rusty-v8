// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod context_serializer {
    use std::any::Any;
    use std::marker::PhantomData;

    use crate::objects::contexts::Context;
    use crate::snapshot::serializer::Serializer;
    use crate::snapshot::snapshot_source_sink::SnapshotByteSink;

    pub type SerializeEmbedderFieldsCallback =
        Option<Box<dyn Fn(&mut SnapshotByteSink, &dyn Any)>>;

    pub struct ContextSerializer<'a> {
        serializer: Serializer,
        startup_serializer: &'a mut StartupSerializer<'a>,
        serialize_embedder_fields: SerializeEmbedderFieldsCallback,
        can_be_rehashed: bool,
        context: Option<Context>,
        embedder_fields_sink: SnapshotByteSink,
        api_wrapper_sink: SnapshotByteSink,
        phantom: PhantomData<&'a ()>,
    }

    impl<'a> ContextSerializer<'a> {
        pub fn new(
            isolate: *mut Isolate,
            flags: SerializerFlags,
            startup_serializer: &'a mut StartupSerializer<'a>,
            callback: SerializeEmbedderFieldsCallback,
        ) -> Self {
            ContextSerializer {
                serializer: Serializer::new(isolate, flags),
                startup_serializer,
                serialize_embedder_fields: callback,
                can_be_rehashed: true,
                context: None,
                embedder_fields_sink: SnapshotByteSink::new(),
                api_wrapper_sink: SnapshotByteSink::new(),
                phantom: PhantomData,
            }
        }

        pub fn serialize(&mut self, o: &mut Context, _no_gc: &DisallowGarbageCollection) {
            self.context = Some(o.clone()); // Cloning to avoid lifetime issues, consider alternatives
            self.serializer.serialize_root(o);
        }

        pub fn can_be_rehashed(&self) -> bool {
            self.can_be_rehashed
        }
    }

    impl<'a> Drop for ContextSerializer<'a> {
        fn drop(&mut self) {}
    }

    impl<'a> ContextSerializer<'a> {
        fn serialize_object_impl(&mut self, o: &mut HeapObject, slot_type: SlotType) {
            self.serializer.serialize_object_impl(o, slot_type);
        }

        fn should_be_in_the_startup_object_cache(&self, _o: &HeapObject) -> bool {
            false // Placeholder
        }

        fn should_be_in_the_shared_object_cache(&self, _o: &HeapObject) -> bool {
            false // Placeholder
        }

        fn check_rehashability(&mut self, _obj: &HeapObject) {
            // Placeholder
        }

        // TODO: Implement SerializeObjectWithEmbedderFields once the involved types
        // are fully translated.
        // fn serialize_object_with_embedder_fields<V8Type, UserSerializerWrapper, UserCallback, ApiObjectType>(
        //     &mut self,
        //     data_holder: Handle<V8Type>,
        //     embedder_fields_count: i32,
        //     wrapper: UserSerializerWrapper,
        //     user_callback: UserCallback,
        //     api_obj: ApiObjectType,
        // ) {
        //     // Placeholder implementation
        // }

        fn serialize_api_wrapper_fields(&mut self, _js_object: &mut JSObject) {
            // Placeholder
        }
    }

    // Dummy types and functions to satisfy the compiler.  These should be
    // replaced with actual implementations when the corresponding C++ code is
    // translated.
    #[derive(Clone, Copy)]
    pub struct SerializerFlags {}
    pub struct Isolate {}
    pub struct HeapObject {}
    pub struct SlotType {}
    pub struct DisallowGarbageCollection {}
    pub struct JSObject {}

    pub struct StartupSerializer<'a> {
        phantom: PhantomData<&'a ()>
    }

    impl<'a> StartupSerializer<'a> {
        pub fn new() -> Self {
            StartupSerializer{
                phantom: PhantomData
            }
        }
    }

    impl Serializer {
        fn serialize_root<T>(&mut self, _root: &mut T) {
            // Placeholder implementation
        }
        fn serialize_object_impl(&mut self, _o: &mut HeapObject, _slot_type: SlotType) {}
    }
}