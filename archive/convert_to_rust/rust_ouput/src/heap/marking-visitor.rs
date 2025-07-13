// Converted from V8 C++ source files:
// Header: marking-visitor.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub use std::collections::hash_map::DefaultHasher;

    pub trait KeyEqualSafe {
        fn key_equal_safe(&self, other: &Self) -> bool;
    }

    pub mod small_vector {
        use std::vec::Vec;

        pub struct SmallVector<T, const N: usize> {
            data: Vec<T>,
        }

        impl<T, const N: usize> SmallVector<T, N> {
            pub fn new() -> Self {
                SmallVector { data: Vec::new() }
            }

            pub fn push(&mut self, value: T) {
                self.data.push(value);
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }

            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }

            pub fn get(&self, index: usize) -> Option<&T> {
                self.data.get(index)
            }
        }
    }
}

pub mod common {
    pub mod globals {
        pub type Address = usize;
    }
}

pub mod execution {
    pub struct Isolate {
        is_backgrounded: bool,
        external_pointer_table: std::sync::Mutex<usize>,
        shared_external_pointer_table: std::sync::Mutex<usize>,
        shared_external_pointer_space: std::sync::Mutex<usize>,
        cpp_heap_pointer_table: std::sync::Mutex<usize>,
        trusted_pointer_table: std::sync::Mutex<usize>,
        shared_trusted_pointer_table: std::sync::Mutex<usize>,
    }

    impl Isolate {
        pub fn new(is_backgrounded: bool) -> Self {
            Isolate {
                is_backgrounded,
                external_pointer_table: std::sync::Mutex::new(0),
                shared_external_pointer_table: std::sync::Mutex::new(0),
                shared_external_pointer_space: std::sync::Mutex::new(0),
                cpp_heap_pointer_table: std::sync::Mutex::new(0),
                trusted_pointer_table: std::sync::Mutex::new(0),
                shared_trusted_pointer_table: std::sync::Mutex::new(0),
            }
        }

        pub fn is_backgrounded(&self) -> bool {
            self.is_backgrounded
        }

        pub fn external_pointer_table(&self) -> &std::sync::Mutex<usize> {
            &self.external_pointer_table
        }
        pub fn shared_external_pointer_table(&self) -> &std::sync::Mutex<usize> {
            &self.shared_external_pointer_table
        }
        pub fn shared_external_pointer_space(&self) -> &std::sync::Mutex<usize> {
            &self.shared_external_pointer_space
        }
        pub fn cpp_heap_pointer_table(&self) -> &std::sync::Mutex<usize> {
            &self.cpp_heap_pointer_table
        }
        pub fn trusted_pointer_table(&self) -> &std::sync::Mutex<usize> {
            &self.trusted_pointer_table
        }
        pub fn shared_trusted_pointer_table(&self) -> &std::sync::Mutex<usize> {
            &self.shared_trusted_pointer_table
        }
    }
}

pub mod heap {
    use crate::{
        base::small_vector::SmallVector, common::globals::Address,
        execution::Isolate,
    };
    use std::{
        cell::RefCell,
        collections::HashMap,
        hash::{Hash, Hasher},
        rc::Rc,
        sync::Mutex,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Tagged<T> {
        address: Address,
        phantom: std::marker::PhantomData<T>,
    }

    impl<T> Tagged<T> {
        pub fn new(address: Address) -> Self {
            Tagged {
                address,
                phantom: std::marker::PhantomData,
            }
        }

        pub fn address(&self) -> Address {
            self.address
        }
    }

    pub trait HeapObjectTrait {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct HeapObject {
        address: Address,
    }

    impl HeapObject {
        pub fn new(address: Address) -> Self {
            HeapObject { address }
        }

        pub fn address(&self) -> Address {
            self.address
        }

        pub fn map<'a>(&self, _cage_base: usize) -> Tagged<Map> {
            Tagged::new(0)
        }
        pub fn map_slot<'a>(&self) -> ObjectSlot {
            ObjectSlot {}
        }
    }

    impl HeapObjectTrait for HeapObject {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Map {
        address: Address,
    }

    impl Map {
        pub fn new(address: Address) -> Self {
            Map { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DescriptorArray {
        address: Address,
    }

    impl DescriptorArray {
        pub fn new(address: Address) -> Self {
            DescriptorArray { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EphemeronHashTable {
        address: Address,
    }

    impl EphemeronHashTable {
        pub fn new(address: Address) -> Self {
            EphemeronHashTable { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FixedArray {
        address: Address,
    }

    impl FixedArray {
        pub fn new(address: Address) -> Self {
            FixedArray { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct JSArrayBuffer {
        address: Address,
    }

    impl JSArrayBuffer {
        pub fn new(address: Address) -> Self {
            JSArrayBuffer { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct JSFunction {
        address: Address,
    }

    impl JSFunction {
        pub fn new(address: Address) -> Self {
            JSFunction { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct JSWeakRef {
        address: Address,
    }

    impl JSWeakRef {
        pub fn new(address: Address) -> Self {
            JSWeakRef { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SharedFunctionInfo {
        address: Address,
    }

    impl SharedFunctionInfo {
        pub fn new(address: Address) -> Self {
            SharedFunctionInfo { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TransitionArray {
        address: Address,
    }

    impl TransitionArray {
        pub fn new(address: Address) -> Self {
            TransitionArray { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WeakCell {
        address: Address,
    }

    impl WeakCell {
        pub fn new(address: Address) -> Self {
            WeakCell { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct InstructionStream {
        address: Address,
    }

    impl InstructionStream {
        pub fn new(address: Address) -> Self {
            InstructionStream { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Code {
        address: Address,
    }

    impl Code {
        pub fn new(address: Address) -> Self {
            Code { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TrustedObject {
        address: Address,
    }

    impl TrustedObject {
        pub fn new(address: Address) -> Self {
            TrustedObject { address }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct String {
        address: Address,
    }

    impl String {
        pub fn new(address: Address) -> Self {
            String { address }
        }
    }

    pub struct ObjectSlot {}
    pub struct MaybeObjectSlot {}
    pub struct InstructionStreamSlot {}
    pub struct ExternalPointerSlot {}
    pub struct CppHeapPointerSlot {}
    pub struct IndirectPointerSlot {}
    pub struct ProtectedPointerSlot {}
    pub struct ProtectedMaybeObjectSlot {}

    pub enum IndirectPointerMode {}

    pub struct JSDispatchHandle {}

    pub struct RelocInfo {}

    pub mod marking {
        use super::*;
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Color {
            WHITE,
            GREY,
            BLACK,
        }

        #[derive(Debug, Default)]
        pub struct MarkingState {
            color: Mutex<HashMap<Address, Color>>,
        }

        impl MarkingState {
            pub fn new() -> Self {
                MarkingState {
                    color: Mutex::new(HashMap::new()),
                }
            }

            pub fn get_color(&self, address: Address) -> Option<Color> {
                self.color.lock().unwrap().get(&address).copied()
            }

            pub fn set_color(&self, address: Address, color: Color) {
                self.color.lock().unwrap().insert(address, color);
            }
        }

        #[derive(Debug, Default)]
        pub struct MarkingWorklist {
            objects: RefCell<Vec<Address>>,
        }

        impl MarkingWorklist {
            pub fn push(&self, address: Address) {
                self.objects.borrow_mut().push(address);
            }

            pub fn pop(&self) -> Option<Address> {
                self.objects.borrow_mut().pop()
            }

            pub fn is_empty(&self) -> bool {
                self.objects.borrow().is_empty()
            }
        }

        pub struct MarkingWorklists {
            pub local: Local,
        }

        impl MarkingWorklists {
            pub fn new() -> Self {
                MarkingWorklists {
                    local: Local::new(),
                }
            }
        }

        pub struct Local {
            main: MarkingWorklist,
            other: MarkingWorklist,
        }

        impl Local {
            pub fn new() -> Self {
                Local {
                    main: MarkingWorklist::default(),
                    other: MarkingWorklist::default(),
                }
            }

            pub fn main(&mut self) -> &MarkingWorklist {
                &self.main
            }

            pub fn other(&mut self) -> &MarkingWorklist {
                &self.other
            }
        }

        pub enum WorklistTarget {
            MAIN,
            OTHER,
        }

        pub struct MarkingHelper {}

        impl MarkingHelper {
            pub fn is_marked(
                marking_state: &MarkingState,
                object: Tagged<HeapObject>,
            ) -> bool {
                marking_state.get_color(object.address()).is_some()
            }
        }
    }

    pub struct WeakObjects {}

    impl WeakObjects {
        pub fn new() -> Self {
            WeakObjects {}
        }
    }

    pub mod weak_object_worklists {
        pub struct Local {}

        impl Local {
            pub fn new() -> Self {
                Local {}
            }
        }
    }

    pub struct PretenuringHandler {}

    impl PretenuringHandler {
        pub fn new() -> Self {
            PretenuringHandler {}
        }
    }

    pub mod spaces {
        pub struct Spaces {}

        impl Spaces {
            pub fn new() -> Self {
                Spaces {}
            }
        }
    }

    pub enum CodeFlushMode {
        FlushNothing,
        FlushEager,
        FlushLazy,
    }

    pub struct Heap {
        isolate: *mut Isolate,
        marking_state: MarkingState,
    }

    impl Heap {
        pub fn new(isolate: *mut Isolate) -> Self {
            Heap {
                isolate,
                marking_state: MarkingState::new(),
            }
        }

        pub fn isolate(&self) -> &Isolate {
            unsafe { &*self.isolate }
        }

        pub fn marking_state(&self) -> &MarkingState {
            &self.marking_state
        }
    }

    pub struct ExternalPointerTable {}

    pub struct CppHeapPointerTable {}

    pub struct TrustedPointerTable {}

    pub mod memory_chunk {
        use crate::heap::HeapObject;

        pub struct MemoryChunk {}

        impl MemoryChunk {
            pub fn FromHeapObject(_heap_object: Tagged<HeapObject>) -> MemoryChunk {
                MemoryChunk {}
            }
            pub fn SynchronizedLoad(&self){}
        }
    }
}

pub mod objects {
    use crate::heap::{HeapObject, Map, String, Tagged};

    impl crate::base::KeyEqualSafe for Tagged<HeapObject> {
        fn key_equal_safe(&self, other: &Self) -> bool {
            self.address() == other.address()
        }
    }

    pub mod visitors {
        pub struct ObjectVisitorWithCageBases {}
    }
    
    pub struct Script {}
    pub struct TrustedByteArray {}
}

pub mod v8 {
    pub use crate::heap::HeapObject;
}

pub mod absl {
    pub mod flat_hash_map {
        use std::collections::HashMap;
        use std::hash::{Hash, Hasher};

        pub struct FlatHashMap<K, V, H, E> {
            map: HashMap<K, V>,
            hasher: H,
            key_equal: E,
        }

        impl<K, V, H, E> FlatHashMap<K, V, H, E>
        where
            K: Hash + Eq,
            H: Fn() -> std::collections::hash_map::DefaultHasher,
            E: Fn(&K, &K) -> bool,
        {
            pub fn new(hasher: H, key_equal: E) -> Self {
                FlatHashMap {
                    map: HashMap::new(),
                    hasher,
                    key_equal,
                }
            }

            pub fn insert(&mut self, key: K, value: V) {
                self.map.insert(key, value);
            }

            pub fn get(&self, key: &K) -> Option<&V> {
                self.map.get(key)
            }

            pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
                self.map.get_mut(key)
            }

            pub fn remove(&mut self, key: &K) -> Option<V> {
                self.map.remove(key)
            }
        }
    }
}

pub mod heap_visitor {
    use crate::{heap::{HeapObject, Map}, objects::visitors::ObjectVisitorWithCageBases};

    pub trait ObjectVisitor {
        fn visit_map_pointer(&mut self, host: Tagged<HeapObject>);
    }

    pub trait ConcurrentHeapVisitorTrait<ConcreteVisitor> {
        fn map(&self) -> Tagged<Map>;
    }
}

pub mod marking_visitor {
    use super::{
        absl::flat_hash_map::FlatHashMap,
        base::{small_vector::SmallVector, KeyEqualSafe},
        execution::Isolate,
        heap::{
            self, memory_chunk::MemoryChunk, Heap, HeapObject, HeapObjectTrait,
            InstructionStream, InstructionStreamSlot, JSDispatchHandle, Map,
            MarkingState, ObjectSlot, RelocInfo, Tagged,
            TrustedObject, ExternalPointerSlot, CppHeapPointerSlot, IndirectPointerSlot, IndirectPointerMode, ProtectedPointerSlot, ProtectedMaybeObjectSlot
        },
        heap::{
            Code, DescriptorArray, EphemeronHashTable, FixedArray, JSArrayBuffer, JSFunction,
            JSWeakRef, MarkingWorklists, SharedFunctionInfo, TransitionArray, WeakCell,
        },
        heap::marking::{MarkingHelper, WorklistTarget},
        heap::spaces::CodeFlushMode,
        heap::weak_object_worklists::Local as WeakObjectsLocal,
        objects::Script,
        heap_visitor::ObjectVisitorWithCageBases,
        heap_visitor::ConcurrentHeapVisitorTrait,
        objects::TrustedByteArray,
    };
    use std::{
        marker::PhantomData,
        sync::Mutex,
    };

    pub struct ConcurrentHeapVisitor<ConcreteVisitor> {
        isolate: *mut Isolate,
        phantom: PhantomData<ConcreteVisitor>,
    }

    impl<ConcreteVisitor> ConcurrentHeapVisitor<ConcreteVisitor> {
        pub fn new(isolate: *mut Isolate) -> Self {
            ConcurrentHeapVisitor {
                isolate,
                phantom: PhantomData,
            }
        }
        fn cage_base(&self) -> usize {
            0
        }
    }

    impl<ConcreteVisitor> ConcurrentHeapVisitorTrait<ConcreteVisitor> for ConcurrentHeapVisitor<ConcreteVisitor> {
        fn map(&self) -> Tagged<Map> {
            Tagged::new(0)
        }
    }

    pub type KeyToValues = FlatHashMap<
        Tagged<HeapObject>,
        SmallVector<Tagged<HeapObject>, 1>,
        fn() -> std::collections::hash_map::DefaultHasher,
        fn(&Tagged<HeapObject>, &Tagged<HeapObject>) -> bool,
    >;

    pub trait ConcreteMarkingVisitor {
        fn can_update_values_in_heap(&self) -> bool;
        fn add_strong_reference_for_reference_summarizer(
            &mut self,
            host: Tagged<HeapObject>,
            obj: Tagged<HeapObject>,
        );
        fn add_weak_reference_for_reference_summarizer(
            &mut self,
            host: Tagged<HeapObject>,
            obj: Tagged<HeapObject>,
        );
        fn marking_state(&self) -> &MarkingState;
        fn mark_pointer_table_entry(&mut self, obj: Tagged<HeapObject>, slot: IndirectPointerSlot);
        fn record_slot(&mut self);
        fn record_reloc_slot(&mut self);
    }

    pub struct MarkingVisitorBase<ConcreteVisitor> {
        base: ConcurrentHeapVisitor<ConcreteVisitor>,
        local_marking_worklists: *mut MarkingWorklists::Local,
        local_weak_objects: *mut WeakObjectsLocal,
        key_to_values: Mutex<Option<KeyToValues>>,
        heap: *mut Heap,
        mark_compact_epoch: u32,
        code_flush_mode: std::collections::HashSet<CodeFlushMode>,
        should_keep_ages_unchanged: bool,
        code_flushing_increase: u16,
        isolate_in_background: bool,
        external_pointer_table: Mutex<usize>,
        shared_external_pointer_table: Mutex<usize>,
        shared_external_pointer_space: Mutex<usize>,
        cpp_heap_pointer_table: Mutex<usize>,
        trusted_pointer_table: Mutex<usize>,
        shared_trusted_pointer_table: Mutex<usize>,
        concrete_visitor: ConcreteVisitor,
    }

    impl<ConcreteVisitor> MarkingVisitorBase<ConcreteVisitor>
    where ConcreteVisitor: ConcreteMarkingVisitor {
        pub fn new(
            local_marking_worklists: *mut MarkingWorklists::Local,
            local_weak_objects: *mut WeakObjectsLocal,
            heap: *mut Heap,
            mark_compact_epoch: u32,
            code_flush_mode: std::collections::HashSet<CodeFlushMode>,
            should_keep_ages_unchanged: bool,
            code_flushing_increase: u16,
            concrete_visitor: ConcreteVisitor,
        ) -> Self {
            let isolate = unsafe { (*heap).isolate() };
            MarkingVisitorBase {
                base: ConcurrentHeapVisitor::new(heap as *mut Isolate),
                local_marking_worklists,
                local_weak_objects,
                key_to_values: Mutex::new(None),
                heap,
                mark_compact_epoch,
                code_flush_mode,
                should_keep_ages_unchanged,
                code_flushing_increase,
                isolate_in_background: unsafe { (*isolate).is_backgrounded() },
                external_pointer_table: Mutex::new(0),
                shared_external_pointer_table: Mutex::new(0),
                shared_external_pointer_space: Mutex::new(0),
                cpp_heap_pointer_table: Mutex::new(0),
                trusted_pointer_table: Mutex::new(0),
                shared_trusted_pointer_table: Mutex::new(0),
                concrete_visitor,
            }
        }

        pub fn visit_descriptor_array_strongly(
            &self,
            map: Tagged<Map>,
            object: Tagged<DescriptorArray>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_descriptor_array(
            &self,
            map: Tagged<Map>,
            object: Tagged<DescriptorArray>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_ephemeron_hash_table(
            &self,
            map: Tagged<Map>,
            object: Tagged<EphemeronHashTable>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_fixed_array(
            &self,
            map: Tagged<Map>,
            object: Tagged<FixedArray>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_js_array_buffer(
            &self,
            map: Tagged<Map>,
            object: Tagged<JSArrayBuffer>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_js_function(
            &self,
            map: Tagged<Map>,
            object: Tagged<JSFunction>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_js_weak_ref(
            &self,
            map: Tagged<Map>,
            object: Tagged<JSWeakRef>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_map(
            &self,
            map: Tagged<Map>,
            object: Tagged<Map>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_shared_function_info(
            &self,
            map: Tagged<Map>,
            object: Tagged<SharedFunctionInfo>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_transition_array(
            &self,
            map: Tagged<Map>,
            object: Tagged<TransitionArray>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_weak_cell(
            &self,
            map: Tagged<Map>,
            object: Tagged<WeakCell>,
            _object_size: MaybeObjectSize,
        ) -> usize {
            0
        }

        pub fn visit_map_pointer(&self, host: Tagged<HeapObject>) {
            let map = host.map(ObjectVisitorWithCageBases::cage_base());
            self.process_strong_heap_object(host, host.map_slot(), map);
        }

        pub fn visit_pointer(&self, host: Tagged<HeapObject>, p: ObjectSlot) {
            self.visit_pointers_impl(host, p, p);
        }

        pub fn visit_pointer_maybe_object_slot(
            &self,
            host: Tagged<HeapObject>,
            p: MaybeObjectSlot,
        ) {
            self.visit_pointers_impl(host, p, p);
        }

        pub fn visit_pointers(&self, host: Tagged<HeapObject>, start: ObjectSlot, end: ObjectSlot) {
            self.visit_pointers_impl(host, start, end);
        }

        pub fn visit_pointers_maybe_object_slot(
            &self,
            host: Tagged<HeapObject>,
            start: MaybeObjectSlot,
            end: MaybeObjectSlot,
        ) {
            self.visit_pointers_impl(host, start, end);
        }

        pub fn visit_instruction_stream_pointer(
            &self,
            host: Tagged<Code>,
            slot: InstructionStreamSlot,
        ) {
            self.visit_strong_pointer_impl(host, slot);
        }

        pub fn visit_embedded_pointer(&self, _host: Tagged<InstructionStream>, _rinfo: &RelocInfo) {}

        pub fn visit_code_target(&self, _host: Tagged<InstructionStream>, _rinfo: &RelocInfo) {}

        pub fn visit_custom_weak_pointers(
            &self,
            _host: Tagged<HeapObject>,
            _start: ObjectSlot,
            _end: ObjectSlot,
        ) {
        }

        pub fn visit_external_pointer(&self, host: Tagged<HeapObject>, slot: ExternalPointerSlot) {}

        pub fn visit_cpp_heap_pointer(&self, host: Tagged<HeapObject>, slot: CppHeapPointerSlot) {}

        pub fn visit_indirect_pointer(&self, host: Tagged<HeapObject>, slot: IndirectPointerSlot, mode: IndirectPointerMode) {}

        pub fn visit_trusted_pointer_table_entry(
            &self,
            host: Tagged<HeapObject>,
            slot: IndirectPointerSlot,
        ) {
        }

        pub fn visit_js_dispatch_table_entry(
            &self,
            host: Tagged<HeapObject>,
            _handle: JSDispatchHandle,
        ) {
        }

        pub fn visit_protected_pointer(&self, host: Tagged<TrustedObject>, slot: ProtectedPointerSlot) {
            self.visit_strong_pointer_impl(host, slot);
        }

        pub fn visit_protected_pointer_maybe_object_slot(&self, host: Tagged<TrustedObject>, slot: ProtectedMaybeObjectSlot) {
            self.visit_pointers_impl(host, slot, slot);
        }

        pub fn synchronize_page_access(&self, _heap_object: Tagged<HeapObject>) {}

        pub fn mark_object(
            &self,
            host: Tagged<HeapObject>,
            obj: Tagged<HeapObject>,
            target_worklist: WorklistTarget,
        ) -> bool {
            let marking_state = unsafe { &(*self.heap).marking_state };
            if MarkingHelper::is_marked(marking_state, obj) {
                return false;
            }

            let color = marking_state.get_color(obj.address()).unwrap_or(heap::marking::Color::WHITE);
            if color == heap::marking::Color::BLACK {
                return false;
            }

            let worklist = unsafe {
                match target_worklist {
                    WorklistTarget::MAIN => (*self.local_marking_worklists).main(),
                    WorklistTarget::OTHER => (*self.local_marking_worklists).other(),
                }
            };

            marking_state.set_color(obj.address(), heap::marking::Color::GREY);
            worklist.push(obj.address());
            true
        }

        pub const fn should_visit_read_only_map_pointer() -> bool {
            false
        }

        pub const fn can_encounter_filler_or_free_space() -> bool {
            false
        }

        pub fn is_trivial_weak_reference_value(
            _host: Tagged<HeapObject>,
            _heap_object: Tagged<HeapObject>,
        ) -> bool {
            false
        }

        pub fn set_key_to_values(&self, key_to_values: &mut KeyToValues) {
            let mut guard = self.key_to_values.lock().unwrap();
            *guard = Some(KeyToValues {
                map: key_to_values.map.clone(),
                hasher: key_to_values.hasher,
                key_equal: key_to_values.key_equal,
            });
        }

        fn process_strong_heap_object<THeapObjectSlot>(
            &self,
            host: Tagged<HeapObject>,
            _slot: THeapObjectSlot,
            heap_object: Tagged<HeapObject>,
        ) {
            self.mark_object(host, heap_object, WorklistTarget::MAIN);
            self.concrete_visitor.add_strong_reference_for_reference_summarizer(host, heap_object);
        }

        fn process_weak_heap_object<THeapObjectSlot>(
            &self,
            host: Tagged<HeapObject>,
            _slot: THeapObjectSlot,
            heap_object: Tagged<HeapObject>,
        ) {
            self.concrete_visitor.add_weak_reference_for_reference_summarizer(host, heap_object);
        }

        fn visit_pointers_impl<TSlot>(&self, host: Tagged<HeapObject>, start: TSlot, end: TSlot) {
            //Placeholder since implementation details are not available
        }

        fn visit_strong_pointer_impl<TSlot>(&self, host: Tagged<HeapObject>, slot: TSlot) {
            //Placeholder since implementation details are not available
        }

        fn visit_descriptors_for_map(&self, _map: Tagged<Map>) {}

        fn visit_fixed_array_with_progress_tracker(
            &self,
            _map: Tagged<Map>,
            _object: Tagged<FixedArray>,
            _progress_tracker: MarkingProgressTracker,
        ) -> usize {
            0
        }

        fn should_flush_code(&self, _sfi: Tagged<SharedFunctionInfo>) -> bool {
            false
        }

        fn should_flush_baseline_code(&self, _js_function: Tagged<JSFunction>) -> bool {
            false
        }

        fn has_bytecode_array_for_flushing(&self, _sfi: Tagged<SharedFunctionInfo>) -> bool {
            false
        }

        fn is_old(&self, _sfi: Tagged<SharedFunctionInfo>) -> bool {
            false
        }

        fn make_older(&self, _sfi: Tagged<SharedFunctionInfo>) {}
    }

    pub struct FullMarkingVisitorBase<ConcreteVisitor> {
        base: MarkingVisitorBase<ConcreteVisitor>,
        marking_state: *mut MarkingState,
    }

    impl<ConcreteVisitor> FullMarkingVisitorBase<ConcreteVisitor>
    where ConcreteVisitor: ConcreteMarkingVisitor {
        pub fn new(
            local_marking_worklists: *mut MarkingWorklists::Local,
            local_weak_objects: *mut WeakObjectsLocal,
            heap: *mut Heap,
            mark_compact_epoch: u32,
            code_flush_mode: std::collections::HashSet<CodeFlushMode>,
            should_keep_ages_unchanged: bool,
            code_flushing_increase: u16,
            concrete_visitor: ConcreteVisitor,
        ) -> Self {
            let marking_state = unsafe { (*heap).marking_state() as *mut MarkingState };
            FullMarkingVisitorBase {
                base: MarkingVisitorBase::new(
                    local_marking_worklists,
                    local_weak_objects,
                    heap,
                    mark_compact_epoch,
                    code_flush_mode,
                    should_keep_ages_unchanged,
                    code_flushing_increase,
                    concrete_visitor,
                ),
                marking_state,
            }
        }

        pub fn add_strong_reference_for_reference_summarizer(
            &mut self,
            _host: Tagged<HeapObject>,
            _obj: Tagged<HeapObject>,
        ) {
        }

        pub fn add_weak_reference_for_reference_summarizer(
            &mut self,
            _host: Tagged<HeapObject>,
            _obj: Tagged<HeapObject>,
        ) {
        }

        pub const fn can_update_values_in_heap() -> bool {
            true
        }

        pub fn marking_state(&self) -> *mut MarkingState {
            self.marking_state
        }

        pub fn mark_pointer_table_entry(&mut self, _obj: Tagged<HeapObject>, _slot: IndirectPointerSlot) {}
    }

    pub struct MarkingProgressTracker {}

    impl MarkingProgressTracker {
        pub fn new() -> Self {
            MarkingProgressTracker {}
        }
    }
}
