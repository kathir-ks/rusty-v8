pub mod cppgc {
    pub mod internal {
        pub struct BasicCrossThreadPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy> {
            _phantom: std::marker::PhantomData<(T, WeaknessPolicy, LocationPolicy, CheckingPolicy)>,
        }

        pub struct BasicPersistent<T, WeaknessPolicy, LocationPolicy, CheckingPolicy> {
            _phantom: std::marker::PhantomData<(T, WeaknessPolicy, LocationPolicy, CheckingPolicy)>,
        }

        pub struct ConservativeTracingVisitor;

        pub struct VisitorBase;

        pub struct VisitorFactory;

        pub const K_SIZE_OF_UNCOMPRESSED_MEMBER: usize = std::mem::size_of::<*mut std::ffi::c_void>();
        pub const K_SIZEOF_COMPRESSED_MEMBER: usize = std::mem::size_of::<usize>();

        pub struct RawPointer;
        impl RawPointer {
            pub fn LoadAtomic(&self) -> *const std::ffi::c_void {
                std::ptr::null() // Placeholder, implement atomic load here
            }
        }

        pub struct CompressedPointer;
        impl CompressedPointer {
             pub fn LoadAtomic(&self) -> *const std::ffi::c_void {
                std::ptr::null() // Placeholder, implement atomic load here
            }
        }
    }

    pub type WeakCallback = fn(&LivenessBroker, *const std::ffi::c_void);

    pub struct Visitor {
        _private: VisitorKey,
    }

    impl Visitor {
        pub fn new(key: VisitorKey) -> Self {
            Visitor { _private: key }
        }

        pub fn trace<T>(&self, member: &Member<T>) {
            let value = member.get_raw_atomic();
            if value != std::ptr::null() {
                self.trace_impl(value);
            }
        }

        pub fn trace_weak<T>(&self, weak_member: &WeakMember<T>) {
            // TODO: Add static asserts for size, IsGarbageCollectedOrMixinType, IsAllocatedOnCompactableSpace

            let value = weak_member.get_raw_atomic();

            if value.is_null() {
                return;
            }
            self.visit_weak(
                value,
                T::get_trace_descriptor(value),
                handle_weak::<WeakMember<T>>,
                weak_member as *const WeakMember<T> as *const std::ffi::c_void,
            );
        }

        #[cfg(feature = "cppgc_pointer_compression")]
        pub fn trace_uncompressed<T>(&self, member: &subtle::UncompressedMember<T>) {
            let value = member.get_raw_atomic();
             if value != std::ptr::null() {
                self.trace_impl(value);
            }
        }

        pub fn trace_multiple<T>(&self, start: *const subtle::UncompressedMember<T>, len: usize) {
            // TODO: Add static asserts for size, IsGarbageCollectedOrMixinType
            self.visit_multiple_uncompressed_member(
                start as *const std::ffi::c_void,
                len,
                |ptr| T::get_trace_descriptor(ptr),
            );
        }

        pub fn trace_multiple_member<T>(&self, start: *const Member<T>, len: usize) {
            // TODO: Add static asserts for size, IsGarbageCollectedOrMixinType
            #[cfg(feature = "cppgc_pointer_compression")]
            {
               self.visit_multiple_compressed_member(
                   start as *const std::ffi::c_void,
                   len,
                   |ptr| T::get_trace_descriptor(ptr),
                );
            }
        }

        pub fn trace<T>(&self, object: &T) {
            // TODO: Add CheckObjectNotInConstruction
            T::trace(self, object);
        }

        pub fn trace_multiple<T>(&self, start: *const T, len: usize) {
             // TODO: Add CheckObjectNotInConstruction
            for i in 0..len {
                let object = unsafe { &*start.add(i) };
                 if std::any::TypeId::of::<T>() != std::any::TypeId::of::<dyn std::any::Any>() {
                    let vtable = unsafe { *(object as *const T as *const usize) };
                    if vtable == 0 {
                        continue;
                    }
                }
                T::trace(self, object);
            }
        }

        pub fn register_weak_callback_method<T, F>(&self, object: *const T, method: F)
        where
            F: Fn(&T, &LivenessBroker) -> (),
        {
            self.register_weak_callback(
                weak_callback_method_delegate::<T, F>,
                object as *const T as *const std::ffi::c_void,
            );

            unsafe extern "C" fn weak_callback_method_delegate<T, F>(
                info: &LivenessBroker,
                self_ptr: *const std::ffi::c_void,
            ) where
                F: Fn(&T, &LivenessBroker) -> (),
            {
                let object: &T = unsafe { &*(self_ptr as *const T) };
                method(object, info);
            }
        }

        pub fn trace<K, V>(&self, ephemeron_pair: &EphemeronPair<K, V>) {
            self.trace_ephemeron(&ephemeron_pair.key, &ephemeron_pair.value);
            self.register_weak_callback_method(
                ephemeron_pair as *const EphemeronPair<K, V>,
                EphemeronPair::<K, V>::clear_value_if_key_is_dead,
            );
        }

        pub fn trace_ephemeron<KeyType, ValueType>(
            &self,
            weak_member_key: &WeakMember<KeyType>,
            member_value: &Member<ValueType>,
        ) {
            let key = weak_member_key.get_raw_atomic();
            if key.is_null() {
                return;
            }

            let value = member_value.get_raw_atomic();
            if value.is_null() {
                return;
            }

            let value_desc = ValueType::get_trace_descriptor(value);
            if value_desc.base_object_payload.is_null() {
                 panic!("value_desc.base_object_payload is null");
            }
            let key_base_object_payload = KeyType::get_trace_descriptor(key).base_object_payload;

            if key_base_object_payload.is_null() {
               panic!("key_base_object_payload is null");
            }

            self.visit_ephemeron(key_base_object_payload, value, value_desc);
        }

        pub fn trace_ephemeron_value<KeyType, ValueType>(
            &self,
            weak_member_key: &WeakMember<KeyType>,
            value: *const ValueType,
        ) {
            // TODO: static_assert(!IsGarbageCollectedOrMixinTypeV<ValueType>)
            let key = weak_member_key.get_raw_atomic();
            if key.is_null() {
                return;
            }
            // `value` must always be non-null.
            assert!(!value.is_null());
            let value_desc = ValueType::get_trace_descriptor(value);
            // `value_desc.base_object_payload` must be null as this override is only
            // taken for non-garbage-collected values.
            assert!(value_desc.base_object_payload.is_null());

            // KeyType might be a GarbageCollectedMixin.
            let key_base_object_payload = KeyType::get_trace_descriptor(key).base_object_payload;
            assert!(!key_base_object_payload.is_null());

            self.visit_ephemeron(key_base_object_payload, value as *const std::ffi::c_void, value_desc);
        }

        pub fn trace_strongly<T>(&self, weak_member: &WeakMember<T>) {
            let value = weak_member.get_raw_atomic();
            if value != std::ptr::null() {
                self.trace_impl(value);
            }
        }

        pub fn trace_strong_container<T>(&self, object: *const T) {
            self.trace_impl(object as *const std::ffi::c_void);
        }

        pub fn trace_weak_container<T>(
            &self,
            object: *const T,
            callback: WeakCallback,
            callback_data: *const std::ffi::c_void,
        ) {
            if object.is_null() {
                return;
            }
            self.visit_weak_container(
                object as *const std::ffi::c_void,
                T::get_trace_descriptor(object as *const T),
                T::get_weak_trace_descriptor(object as *const T),
                callback,
                callback_data,
            );
        }

        pub fn register_movable_reference<T>(&self, slot: *const *const T) {
            // TODO: Add static asserts for IsAllocatedOnCompactableSpace, IsGarbageCollectedMixinType
            self.handle_movable_reference(slot as *const *const std::ffi::c_void);
        }

        pub fn register_weak_callback(&self, _callback: WeakCallback, _data: *const std::ffi::c_void) {}

        pub fn defer_trace_to_mutator_thread_if_concurrent(
            &self,
            _parameter: *const std::ffi::c_void,
            _callback: TraceCallback,
            _deferred_size: usize,
        ) -> bool {
            false
        }

        pub fn visit(&self, _self_ptr: *const std::ffi::c_void, _trace_descriptor: TraceDescriptor) {}
        pub fn visit_weak(
            &self,
            _self_ptr: *const std::ffi::c_void,
            _trace_descriptor: TraceDescriptor,
            _weak_callback: WeakCallback,
            _weak_member: *const std::ffi::c_void,
        ) {
        }
        pub fn visit_ephemeron(
            &self,
            _key: *const std::ffi::c_void,
            _value: *const std::ffi::c_void,
            _value_desc: TraceDescriptor,
        ) {
        }

        pub fn visit_weak_container(
            &self,
            _self_ptr: *const std::ffi::c_void,
            _strong_desc: TraceDescriptor,
            _weak_desc: TraceDescriptor,
            _callback: WeakCallback,
            _data: *const std::ffi::c_void,
        ) {
        }
        pub fn handle_movable_reference(&self, _slot: *const *const std::ffi::c_void) {}

        pub fn visit_multiple_uncompressed_member(
            &self,
            start: *const std::ffi::c_void,
            len: usize,
            get_trace_descriptor: fn(*const std::ffi::c_void) -> TraceDescriptor,
        ) {
            let mut it = start as *const u8;
            let end = unsafe { it.add(len * internal::K_SIZE_OF_UNCOMPRESSED_MEMBER) };
            while it < end {
                let current = it as *const internal::RawPointer;
                let object = unsafe { (*current).LoadAtomic() };
                if !object.is_null() {
                    self.visit(object, get_trace_descriptor(object));
                }
                it = unsafe { it.add(internal::K_SIZE_OF_UNCOMPRESSED_MEMBER) };
            }
        }

        #[cfg(feature = "cppgc_pointer_compression")]
        pub fn visit_multiple_compressed_member(
            &self,
            start: *const std::ffi::c_void,
            len: usize,
            get_trace_descriptor: fn(*const std::ffi::c_void) -> TraceDescriptor,
        ) {
            let mut it = start as *const u8;
            let end = unsafe { it.add(len * internal::K_SIZEOF_COMPRESSED_MEMBER) };
            while it < end {
                let current = it as *const internal::CompressedPointer;
                let object = unsafe { (*current).LoadAtomic() };
                if !object.is_null() {
                    self.visit(object, get_trace_descriptor(object));
                }
                it = unsafe { it.add(internal::K_SIZEOF_COMPRESSED_MEMBER) };
            }
        }

        fn trace_impl(&self, t: *const std::ffi::c_void) {
            // TODO: Add static asserts for size, IsGarbageCollectedOrMixinType

            self.visit(t, T::get_trace_descriptor(t));
        }

        // #[cfg(feature = "v8_enable_checks")]
        // fn check_object_not_in_construction(&self, address: *const std::ffi::c_void) {}
    }

    extern "C" fn handle_weak<PointerType>(info: &LivenessBroker, object: *const std::ffi::c_void) {
        let weak: &PointerType = unsafe { &*(object as *const PointerType) };
        if !info.is_heap_object_alive(weak.get_from_gc()) {
            weak.clear_from_gc();
        }
    }

    pub struct VisitorKey {}

    pub struct Member<T> {
        _phantom: std::marker::PhantomData<T>,
        _raw: *mut T,
    }

    impl<T> Member<T> {
        pub fn get_raw_atomic(&self) -> *const T {
            self._raw as *const T
        }
    }

    pub struct WeakMember<T> {
        _phantom: std::marker::PhantomData<T>,
        _raw: *mut T,
    }

    impl<T> WeakMember<T> {
        pub fn get_raw_atomic(&self) -> *const T {
            self._raw as *const T
        }
        pub fn get_from_gc(&self) -> *const T {
            self._raw as *const T
        }
        pub fn clear_from_gc(&self) {
            self._raw = std::ptr::null_mut();
        }
    }

    pub mod subtle {
        pub struct UncompressedMember<T> {
            _phantom: std::marker::PhantomData<T>,
            _raw: *mut T,
        }

        impl<T> UncompressedMember<T> {
            pub fn get_raw_atomic(&self) -> *const T {
                self._raw as *const T
            }
        }
    }

    pub struct LivenessBroker;

    impl LivenessBroker {
        pub fn is_heap_object_alive(&self, _object: *const std::ffi::c_void) -> bool {
            true // Stub
        }
    }

    pub type TraceDescriptor = *const std::ffi::c_void;
    pub type TraceCallback = extern "C" fn(*const std::ffi::c_void);
    pub type TraceDescriptorCallback = fn(*const std::ffi::c_void) -> TraceDescriptor;

    pub trait TraceTrait {
        fn get_trace_descriptor(_ptr: *const Self) -> TraceDescriptor;
        fn get_weak_trace_descriptor(_ptr: *const Self) -> TraceDescriptor {
            std::ptr::null() as TraceDescriptor
        }
        fn trace(_visitor: &Visitor, _object: &Self) {}
    }

    pub trait GarbageCollected {}

    pub trait SourceLocation {}

    pub struct EphemeronPair<K, V> {
        key: WeakMember<K>,
        value: Member<V>,
    }

    impl<K, V> EphemeronPair<K, V> {
        fn clear_value_if_key_is_dead(&self, _info: &LivenessBroker) {}
    }

    pub mod internal_traits {
        pub trait IsGarbageCollectedOrMixinType {}
        pub trait IsAllocatedOnCompactableSpace {}
    }

    pub mod root_visitor {
        use super::*;
        use std::marker::PhantomData;
        pub struct RootVisitor {
            _private: Visitor::Key,
        }

        impl RootVisitor {
            pub fn new(key: Visitor::Key) -> Self {
                RootVisitor { _private: key }
            }

            pub fn trace<AnyStrongPersistentType>(&self, p: &AnyStrongPersistentType) {
                let object = Self::extract(p);
                if !object.is_null() {
                    self.visit_root(
                        object,
                        AnyStrongPersistentType::PointeeType::get_trace_descriptor(object),
                        p.location(),
                    );
                }
            }

            pub fn trace_weak<AnyWeakPersistentType>(&self, p: &AnyWeakPersistentType) {
                let object = Self::extract(p);
                if !object.is_null() {
                    self.visit_weak_root(
                        object,
                        AnyWeakPersistentType::PointeeType::get_trace_descriptor(object),
                        handle_weak::<AnyWeakPersistentType>,
                        p as *const AnyWeakPersistentType as *const std::ffi::c_void,
                        p.location(),
                    );
                }
            }

            fn visit_root(
                &self,
                _ptr: *const std::ffi::c_void,
                _desc: TraceDescriptor,
                _location: &dyn SourceLocation,
            ) {
            }
            fn visit_weak_root(
                &self,
                _self_ptr: *const std::ffi::c_void,
                _trace_descriptor: TraceDescriptor,
                _weak_callback: WeakCallback,
                _weak_root: *const std::ffi::c_void,
                _location: &dyn SourceLocation,
            ) {
            }

            fn extract<AnyPersistentType>(p: &AnyPersistentType) -> *const std::ffi::c_void {
                p.get_from_gc()
            }
        }

        extern "C" fn handle_weak<PointerType>(info: &LivenessBroker, object: *const std::ffi::c_void) {
            let weak: &PointerType = unsafe { &*(object as *const PointerType) };
            if !info.is_heap_object_alive(weak.get_from_gc()) {
                weak.clear_from_gc();
            }
        }

        pub trait IsStrongPersistent {
            const value: bool;
        }
    }

    pub mod persistent {
        use super::*;
        pub trait AnyPersistent {
            type PointeeType: TraceTrait;
            fn get_from_gc(&self) -> *const std::ffi::c_void;
            fn location(&self) -> &dyn SourceLocation;
        }
    }
}