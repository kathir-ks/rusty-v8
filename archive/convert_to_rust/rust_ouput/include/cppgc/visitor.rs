// Converted from V8 C++ source files:
// Header: visitor.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {
pub struct RawPointer {}
pub struct CompressedPointer {}
}
pub use self::internal::RawPointer;
pub use self::internal::CompressedPointer;
pub use std::marker::PhantomData;
pub use std::sync::atomic::AtomicPtr;
pub use std::sync::atomic::Ordering;

pub struct LivenessBroker {}

pub struct SourceLocation {}

pub struct TraceDescriptor {
    pub base_object_payload: *const std::ffi::c_void,
}

impl TraceDescriptor {
    pub fn new(base_object_payload: *const std::ffi::c_void) -> Self {
        TraceDescriptor {
            base_object_payload,
        }
    }
}

pub type TraceCallback = fn(parameter: *const std::ffi::c_void);
pub type TraceDescriptorCallback = fn(object: *const std::ffi::c_void) -> TraceDescriptor;

pub struct Member<T> {
    raw_ptr: AtomicPtr<T>,
}

impl<T> Member<T> {
    pub fn new(ptr: *mut T) -> Self {
        Member {
            raw_ptr: AtomicPtr::new(ptr),
        }
    }

    pub fn get_raw(&self) -> *mut T {
        self.raw_ptr.load(Ordering::Relaxed)
    }

    pub fn get_raw_atomic(&self) -> *const T {
        self.raw_ptr.load(Ordering::Relaxed) as *const T
    }

    pub fn store(&self, ptr: *mut T) {
        self.raw_ptr.store(ptr, Ordering::Relaxed);
    }
}

pub struct WeakMember<T> {
    raw_ptr: AtomicPtr<T>,
}

impl<T> WeakMember<T> {
    pub fn new(ptr: *mut T) -> Self {
        WeakMember {
            raw_ptr: AtomicPtr::new(ptr),
        }
    }

    pub fn get_raw(&self) -> *mut T {
        self.raw_ptr.load(Ordering::Relaxed)
    }

    pub fn get_raw_atomic(&self) -> *const T {
        self.raw_ptr.load(Ordering::Relaxed) as *const T
    }

    pub fn store(&self, ptr: *mut T) {
        self.raw_ptr.store(ptr, Ordering::Relaxed);
    }

    pub fn clear_from_gc(&self) {
        self.store(std::ptr::null_mut());
    }

    pub fn get_from_gc(&self) -> *mut T {
        self.get_raw()
    }
}

pub mod subtle {
    use super::*;

    pub struct UncompressedMember<T> {
        raw_ptr: AtomicPtr<T>,
    }

    impl<T> UncompressedMember<T> {
        pub fn new(ptr: *mut T) -> Self {
            UncompressedMember {
                raw_ptr: AtomicPtr::new(ptr),
            }
        }

        pub fn get_raw_atomic(&self) -> *const T {
            self.raw_ptr.load(Ordering::Relaxed) as *const T
        }
    }

    pub struct CompressedMember<T> {
        raw_ptr: AtomicPtr<T>,
    }

    impl<T> CompressedMember<T> {
        pub fn new(ptr: *mut T) -> Self {
            CompressedMember {
                raw_ptr: AtomicPtr::new(ptr),
            }
        }

        pub fn get_raw_atomic(&self) -> *const T {
            self.raw_ptr.load(Ordering::Relaxed) as *const T
        }
    }
}

pub use self::subtle::UncompressedMember;
pub use self::subtle::CompressedMember;

pub struct EphemeronPair<K, V> {
    pub key: WeakMember<K>,
    pub value: Member<V>,
}

impl<K, V> EphemeronPair<K, V> {
    pub fn new(key: *mut K, value: *mut V) -> Self {
        EphemeronPair {
            key: WeakMember::new(key),
            value: Member::new(value),
        }
    }

    pub fn clear_value_if_key_is_dead(&self, info: &LivenessBroker) {
        if !info.is_heap_object_alive(self.key.get_from_gc()) {
            self.value.store(std::ptr::null_mut());
        }
    }
}

impl<K, V> EphemeronPair<K, V> {
    pub fn key(&self) -> &WeakMember<K> {
        &self.key
    }
    pub fn value(&self) -> &Member<V> {
        &self.value
    }
}

pub trait IsGarbageCollectedOrMixinType<T> {}

pub trait IsAllocatedOnCompactableSpace<T> {}

pub trait IsGarbageCollectedMixinType<T> {}

pub struct V8_EXPORT {}

pub struct Visitor {
    // Key type is private, so Visitor can only be constructed via VisitorFactory
    // This is a zero-sized type used for compile-time access control.
    _private: std::marker::PhantomData<Key>,
}

impl Visitor {
    pub struct Key {
        _private: (),
    }

    pub fn new() -> Self {
        Visitor {
            _private: std::marker::PhantomData,
        }
    }

    /**
     * Trace method for Member.
     *
     * \param member Member reference retaining an object.
     */
    pub fn trace<T>(&mut self, member: &Member<T>) {
        let value = member.get_raw_atomic();
        if value.is_null() {
            return;
        }
        self.trace_impl(value);
    }

    /**
     * Trace method for WeakMember.
     *
     * \param weak_member WeakMember reference weakly retaining an object.
     */
    pub fn trace_weak<T>(&mut self, weak_member: &WeakMember<T>) {
        let value = weak_member.get_raw_atomic();

        if value.is_null() {
            return;
        }

        let trace_descriptor = TraceTrait::<T>::get_trace_descriptor(value);
        self.visit_weak(value, trace_descriptor, &Self::handle_weak::<WeakMember<T>>, weak_member);
    }

    #[cfg(feature = "cppgc_pointer_compression")]
    /**
     * Trace method for UncompressedMember.
     *
     * \param member UncompressedMember reference retaining an object.
     */
    pub fn trace_uncompressed<T>(&mut self, member: &subtle::UncompressedMember<T>) {
        let value = member.get_raw_atomic();
        if value.is_null() {
            return;
        }
        self.trace_impl(value);
    }

    pub fn trace_multiple<T>(&mut self, start: *const subtle::UncompressedMember<T>, len: usize) {
        self.visit_multiple_uncompressed_member(start as *const std::ffi::c_void, len, &TraceTrait::<T>::get_trace_descriptor);
    }

    pub fn trace_multiple_member<T>(&mut self, start: *const Member<T>, len: usize) {
        #[cfg(feature = "cppgc_pointer_compression")]
        {
            self.visit_multiple_compressed_member(start as *const std::ffi::c_void, len, &TraceTrait::<T>::get_trace_descriptor);
        }
        #[cfg(not(feature = "cppgc_pointer_compression"))]
        {
             println!("CPPGC_POINTER_COMPRESSION is disabled");
        }
    }

    /**
     * Trace method for inlined objects that are not allocated themselves but
     * otherwise follow managed heap layout and have a Trace() method.
     *
     * \param object reference of the inlined object.
     */
    pub fn trace_inline<T>(&mut self, object: &T) {
        TraceTrait::<T>::trace(self, object as *const T as *const std::ffi::c_void);
    }

    pub fn trace_multiple_inline<T>(&mut self, start: *const T, len: usize) {
        for i in 0..len {
            let object = unsafe { &*start.add(i) };
            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<dyn std::any::Any>() {
                let object_ptr = object as *const T as *const usize;
                let vtable = unsafe { *object_ptr };
                if vtable == 0 {
                    continue;
                }
            }
            TraceTrait::<T>::trace(self, object as *const T as *const std::ffi::c_void);
        }
    }

    /**
     * Registers a weak callback method on the object of type T. See
     * LivenessBroker for an usage example.
     *
     * \param object of type T specifying a weak callback method.
     */
    pub fn register_weak_callback_method<T, F>(&mut self, object: *const T, method: F)
        where F: Fn(&T, &LivenessBroker)
    {
        self.register_weak_callback(Self::weak_callback_method_delegate::<T, F>, object as *const std::ffi::c_void);
    }

    /**
     * Trace method for EphemeronPair.
     *
     * \param ephemeron_pair EphemeronPair reference weakly retaining a key object
     * and strongly retaining a value object in case the key object is alive.
     */
    pub fn trace_ephemeron<K, V>(&mut self, ephemeron_pair: &EphemeronPair<K, V>) {
        self.trace_ephemeron_raw(&ephemeron_pair.key, &ephemeron_pair.value);
        self.register_weak_callback_method(ephemeron_pair as *const EphemeronPair<K, V>, EphemeronPair::<K, V>::clear_value_if_key_is_dead);
    }

    /**
     * Trace method for a single ephemeron. Used for tracing a raw ephemeron in
     * which the `key` and `value` are kept separately.
     *
     * \param weak_member_key WeakMember reference weakly retaining a key object.
     * \param member_value Member reference with ephemeron semantics.
     */
    pub fn trace_ephemeron_raw<KeyType, ValueType>(&mut self, weak_member_key: &WeakMember<KeyType>, member_value: &Member<ValueType>) {
        let key = weak_member_key.get_raw_atomic();
        if key.is_null() {
            return;
        }

        let value = member_value.get_raw_atomic();
        if value.is_null() {
            return;
        }

        let value_desc = TraceTrait::<ValueType>::get_trace_descriptor(value);
        let key_base_object_payload = TraceTrait::<KeyType>::get_trace_descriptor(key).base_object_payload;

        self.visit_ephemeron(key_base_object_payload, value, value_desc);
    }

    /**
     * Trace method for a single ephemeron. Used for tracing a raw ephemeron in
     * which the `key` and `value` are kept separately. Note that this overload
     * is for non-GarbageCollected `value`s that can be traced though.
     *
     * \param key `WeakMember` reference weakly retaining a key object.
     * \param value Reference weakly retaining a value object. Note that
     *   `ValueType` here should not be `Member`. It is expected that
     *   `TraceTrait<ValueType>::GetTraceDescriptor(value)` returns a
     *   `TraceDescriptor` with a null base pointer but a valid trace method.
     */
    pub fn trace_ephemeron_raw_value<KeyType, ValueType>(&mut self, weak_member_key: &WeakMember<KeyType>, value: *const ValueType) {
        let key = weak_member_key.get_raw_atomic();
        if key.is_null() {
            return;
        }

        let value_desc = TraceTrait::<ValueType>::get_trace_descriptor(value);
        let key_base_object_payload = TraceTrait::<KeyType>::get_trace_descriptor(key).base_object_payload;

        self.visit_ephemeron(key_base_object_payload, value as *const std::ffi::c_void, value_desc);
    }

    /**
     * Trace method that strongifies a WeakMember.
     *
     * \param weak_member WeakMember reference retaining an object.
     */
    pub fn trace_strongly<T>(&mut self, weak_member: &WeakMember<T>) {
        let value = weak_member.get_raw_atomic();
        if value.is_null() {
            return;
        }
        self.trace_impl(value);
    }

    /**
     * Trace method for retaining containers strongly.
     *
     * \param object reference to the container.
     */
    pub fn trace_strong_container<T>(&mut self, object: *const T) {
        self.trace_impl(object);
    }

    /**
     * Trace method for retaining containers weakly. Note that weak containers
     * should emit write barriers.
     *
     * \param object reference to the container.
     * \param callback to be invoked.
     * \param callback_data custom data that is passed to the callback.
     */
    pub fn trace_weak_container<T>(&mut self, object: *const T, callback: WeakCallback, callback_data: *const std::ffi::c_void) {
        if object.is_null() {
            return;
        }
        let strong_desc = TraceTrait::<T>::get_trace_descriptor(object);
        let weak_desc = TraceTrait::<T>::get_weak_trace_descriptor(object);
        self.visit_weak_container(object, strong_desc, weak_desc, callback, callback_data);
    }

    /**
     * Registers a slot containing a reference to an object allocated on a
     * compactable space. Such references maybe be arbitrarily moved by the GC.
     *
     * \param slot location of reference to object that might be moved by the GC.
     * The slot must contain an uncompressed pointer.
     */
    pub fn register_movable_reference<T>(&mut self, slot: *const *const T) {
        self.handle_movable_reference(slot as *const *const std::ffi::c_void);
    }

    /**
     * Registers a weak callback that is invoked during garbage collection.
     *
     * \param callback to be invoked.
     * \param data custom data that is passed to the callback.
     */
    pub fn register_weak_callback(&mut self, callback: WeakCallback, data: *const std::ffi::c_void) {
        // Default implementation does nothing.
    }

    /**
     * Defers tracing an object from a concurrent thread to the mutator thread.
     * Should be called by Trace methods of types that are not safe to trace
     * concurrently.
     *
     * \param parameter tells the trace callback which object was deferred.
     * \param callback to be invoked for tracing on the mutator thread.
     * \param deferred_size size of deferred object.
     *
     * \returns false if the object does not need to be deferred (i.e. currently
     * traced on the mutator thread) and true otherwise (i.e. currently traced on
     * a concurrent thread).
     */
    pub fn defer_trace_to_mutator_thread_if_concurrent(&mut self, _parameter: *const std::ffi::c_void, _callback: TraceCallback, _deferred_size: usize) -> bool {
        // By default tracing is not deferred.
        false
    }

    fn visit(&mut self, self_: *const std::ffi::c_void, _trace_descriptor: TraceDescriptor) {
        // Default implementation does nothing.
    }

    fn visit_weak(&mut self, self_: *const std::ffi::c_void, _trace_descriptor: TraceDescriptor, _weak_callback: WeakCallback, _weak_member: &WeakMember<()>) {
        // Default implementation does nothing.
    }

    fn visit_ephemeron(&mut self, key: *const std::ffi::c_void, value: *const std::ffi::c_void, _value_desc: TraceDescriptor) {
        // Default implementation does nothing.
    }

    fn visit_weak_container(&mut self, self_: *const std::ffi::c_void, _strong_desc: TraceDescriptor, _weak_desc: TraceDescriptor, _callback: WeakCallback, _data: *const std::ffi::c_void) {
        // Default implementation does nothing.
    }

    fn handle_movable_reference(&mut self, _slot: *const *const std::ffi::c_void) {
        // Default implementation does nothing.
    }

    fn visit_multiple_uncompressed_member(&mut self, start: *const std::ffi::c_void, len: usize, get_trace_descriptor: &TraceDescriptorCallback) {
        let mut it = start as *const i8;
        let end = unsafe { it.add(len * std::mem::size_of::<usize>()) };
        while it < end {
            let current = it as *const *const std::ffi::c_void;
            let object = unsafe { *current };
            if !object.is_null() {
                self.visit(object, get_trace_descriptor(object));
            }
            unsafe { it = it.add(std::mem::size_of::<usize>()) };
        }
    }

    #[cfg(feature = "cppgc_pointer_compression")]
    fn visit_multiple_compressed_member(&mut self, start: *const std::ffi::c_void, len: usize, get_trace_descriptor: &TraceDescriptorCallback) {
        let mut it = start as *const i8;
        let end = unsafe { it.add(len * std::mem::size_of::<usize>()) };
        while it < end {
            let current = it as *const *const std::ffi::c_void;
            let object = unsafe { *current };
            if !object.is_null() {
                self.visit(object, get_trace_descriptor(object));
            }
            unsafe { it = it.add(std::mem::size_of::<usize>()) };
        }
    }

    fn weak_callback_method_delegate<T, F>(info: &LivenessBroker, self_: *const std::ffi::c_void)
        where F: Fn(&T, &LivenessBroker)
    {
        let object = self_ as *const T;
        let object_ref = unsafe { &*object };
        method(object_ref, info);
    }

    fn handle_weak<PointerType>(info: &LivenessBroker, object: *const std::ffi::c_void) {
        let weak = object as *const PointerType;
        let weak_ref = unsafe { &*weak };
        if !info.is_heap_object_alive(weak_ref.get_from_gc()) {
            weak_ref.clear_from_gc();
        }
    }

    fn trace_impl(&mut self, t: *const std::ffi::c_void) {
        let trace_descriptor = TraceTrait::<()>::get_trace_descriptor(t);
        self.visit(t, trace_descriptor);
    }
}

impl Drop for Visitor {
    fn drop(&mut self) {
        // Default implementation does nothing.
    }
}

pub type WeakCallback = fn(&LivenessBroker, *const std::ffi::c_void);

pub struct RootVisitor {
    _private: std::marker::PhantomData<Visitor::Key>,
}

impl RootVisitor {
    pub fn new() -> Self {
        RootVisitor {
            _private: std::marker::PhantomData,
        }
    }

    pub fn trace<AnyStrongPersistentType>(&mut self, p: &AnyStrongPersistentType)
    {
        let object = Self::extract(p);
        if object.is_null() {
            return;
        }
        let trace_descriptor = TraceTrait::<()>::get_trace_descriptor(object);
        self.visit_root(object, trace_descriptor, SourceLocation {});
    }

    pub fn trace_weak<AnyWeakPersistentType>(&mut self, p: &AnyWeakPersistentType)
    {
        let object = Self::extract(p);
        if object.is_null() {
            return;
        }
        let trace_descriptor = TraceTrait::<()>::get_trace_descriptor(object);
        self.visit_weak_root(object, trace_descriptor, &Self::handle_weak::<AnyWeakPersistentType>, p as *const _ as *const std::ffi::c_void, SourceLocation {});
    }

    fn visit_root(&mut self, _self_: *const std::ffi::c_void, _trace_descriptor: TraceDescriptor, _location: SourceLocation) {
        // Default implementation does nothing.
    }

    fn visit_weak_root(&mut self, self_: *const std::ffi::c_void, _trace_descriptor: TraceDescriptor, weak_callback: WeakCallback, weak_root: *const std::ffi::c_void, _location: SourceLocation) {
        // Default implementation does nothing.
    }

    fn extract<AnyPersistentType>(p: &AnyPersistentType) -> *const std::ffi::c_void {
        p.get_from_gc()
    }

    fn handle_weak<PointerType>(info: &LivenessBroker, object: *const std::ffi::c_void) {
        let weak = object as *const PointerType;
        let weak_ref = unsafe { &*weak };
        if !info.is_heap_object_alive(weak_ref.get_from_gc()) {
            weak_ref.clear_from_gc();
        }
    }
}

impl Drop for RootVisitor {
    fn drop(&mut self) {
        // Default implementation does nothing.
    }
}

pub trait GetFromGC {
    fn get_from_gc(&self) -> *const std::ffi::c_void;
}

pub trait ClearFromGC {
    fn clear_from_gc(&self);
}

pub trait IsStrongPersistent {
    const VALUE: bool;
}

pub trait Location {
    fn location(&self) -> SourceLocation;
}

pub mod trace_trait {
    use super::*;
    pub trait Trace {
        unsafe fn trace(&self, visitor: &mut Visitor);
    }
}
}
