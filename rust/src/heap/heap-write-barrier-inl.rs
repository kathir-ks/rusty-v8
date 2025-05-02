// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Clients of this interface shouldn't depend on lots of heap internals.

mod heap_write_barrier {
    use crate::heap::heap_layout;
    use crate::heap::marking_barrier::MarkingBarrier;
    use crate::heap::memory_chunk::MemoryChunk;
    use crate::objects::maybe_object::MaybeObject;
    use crate::objects::maybe_object::MaybeObject::Cleared;
    use crate::objects::maybe_object::MaybeObject::Object;
    use crate::objects::{
        compressed_slots, descriptor_array::DescriptorArray, ephemeron_hash_table::EphemeronHashTable,
    };
    use crate::objects::{object::Object, shared::TrustedObject, smi::Smi};
    use crate::utils::flag_utils::v8_flags;

    use std::ptr::NonNull;

    // Placeholder types and functions for V8's internal types.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Tagged<T>(pub T);

    impl<T> Tagged<T> {
        pub fn new(value: T) -> Self {
            Tagged(value)
        }
    }

    impl Tagged<HeapObject> {
        pub fn get_heap_object(&self) -> Result<Tagged<HeapObject>, &'static str> {
            Ok(*self)
        }
    }

    pub type HeapObjectSlot = ObjectSlot;
    pub type MaybeObjectSlot = ObjectSlot;
    pub type ObjectSlot = NonNull<Object>;
    pub type ExternalPointerSlot = ObjectSlot;
    pub type IndirectPointerSlot = ObjectSlot;
    pub type ProtectedPointerSlot = ObjectSlot;
    pub type CppHeapPointerSlot = ObjectSlot;
    pub type JSDispatchHandle = ObjectSlot;
    pub type TaggedMemberBase = ObjectSlot;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HeapObject(pub usize);

    impl HeapObject {
        pub fn new(value: usize) -> Self {
            HeapObject(value)
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InstructionStream(pub usize);

    impl InstructionStream {
        pub fn new(value: usize) -> Self {
            InstructionStream(value)
        }
    }

    // Placeholder enums for WriteBarrierMode
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WriteBarrierMode {
        SkipWriteBarrier,
        UpdateWriteBarrier,
        UnsafeSkipWriteBarrier,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RelocInfo {}

    pub fn has_weak_heap_object_tag<T>(_value: T) -> bool {
        false
    }

    // Placeholder functions
    pub fn page_flags_are_consistent<T>(_object: T) -> bool {
        true
    }

    pub fn is_smi<T>(_value: T) -> bool {
        false
    }

    pub mod read_only_heap {
        use crate::heap::heap_write_barrier::HeapObject;
        use crate::heap::heap_write_barrier::Tagged;
        pub fn contains(_object: Tagged<HeapObject>) -> bool {
            false
        }
    }

    pub mod cpp_heap {
        use crate::heap::heap_write_barrier::HeapObject;
        use crate::heap::heap_write_barrier::Tagged;

        pub struct CppHeap {}

        impl CppHeap {
            pub fn from(_cpp_heap: *mut CppHeap) -> *mut CppHeap {
                _cpp_heap
            }
            pub fn remember_cross_heap_reference_if_needed(
                &mut self,
                _host: Tagged<crate::objects::js_object::JSObject>,
                _value: *mut std::ffi::c_void,
            ) {
            }
        }
    }

    pub mod current_marking_barrier {
        use crate::heap::heap_write_barrier::{HeapObject, MarkingBarrier, Tagged};
        pub fn current_marking_barrier(_host: Tagged<HeapObject>) -> *mut MarkingBarrier {
            // TODO: Implement
            panic!("CurrentMarkingBarrier needs implementation")
        }
    }

    pub mod flag_utils {
        pub struct V8Flags {
            pub disable_write_barriers: bool,
            pub sticky_mark_bits: bool,
        }

        impl V8Flags {
            pub fn new() -> Self {
                V8Flags {
                    disable_write_barriers: false,
                    sticky_mark_bits: false,
                }
            }
        }
    }

    lazy_static::lazy_static! {
        pub static ref v8_flags: flag_utils::V8Flags = flag_utils::V8Flags::new();
    }

    pub struct WriteBarrier {}

    impl WriteBarrier {
        pub fn combined_write_barrier_internal(
            host: Tagged<HeapObject>,
            slot: HeapObjectSlot,
            value: Tagged<HeapObject>,
            mode: WriteBarrierMode,
        ) {
            assert_eq!(mode, WriteBarrierMode::UpdateWriteBarrier);

            let host_chunk = MemoryChunk::from_heap_object(host);
            let value_chunk = MemoryChunk::from_heap_object(value);

            let is_marking = host_chunk.is_marking();

            if v8_flags.sticky_mark_bits {
                // TODO(333906585): Support shared barrier.
                if !heap_layout::in_young_generation(&host_chunk, host)
                    && heap_layout::in_young_generation(&value_chunk, value)
                {
                    // Generational or shared heap write barrier (old-to-new or
                    // old-to-shared).
                    Self::combined_generational_and_shared_barrier_slow(
                        host,
                        slot.as_ptr(),
                        value,
                    );
                }
            } else {
                let pointers_from_here_are_interesting = !host_chunk.is_young_or_shared_chunk();
                if pointers_from_here_are_interesting && value_chunk.is_young_or_shared_chunk() {
                    // Generational or shared heap write barrier (old-to-new or
                    // old-to-shared).
                    Self::combined_generational_and_shared_barrier_slow(
                        host,
                        slot.as_ptr(),
                        value,
                    );
                }
            }

            // Marking barrier: mark value & record slots when marking is on.
            if is_marking {
                Self::marking_slow(host, HeapObjectSlot(slot), value);
            }
        }

        pub fn get_write_barrier_mode_for_object(
            object: Tagged<HeapObject>,
            _promise: &DisallowGarbageCollection,
        ) -> WriteBarrierMode {
            if v8_flags.disable_write_barriers {
                WriteBarrierMode::SkipWriteBarrier
            } else {
                assert!(page_flags_are_consistent(object));
                let chunk = MemoryChunk::from_heap_object(object);
                if chunk.is_marking() {
                    WriteBarrierMode::UpdateWriteBarrier
                } else if heap_layout::in_young_generation(&chunk, object) {
                    WriteBarrierMode::SkipWriteBarrier
                } else {
                    WriteBarrierMode::UpdateWriteBarrier
                }
            }
        }

        pub fn is_immortal_immovable_heap_object(object: Tagged<HeapObject>) -> bool {
            // All objects in readonly space are immortal and immovable.
            heap_layout::in_read_only_space(object)
        }

        pub fn for_reloc_info(
            host: Tagged<InstructionStream>,
            rinfo: &mut RelocInfo,
            value: Tagged<HeapObject>,
            mode: WriteBarrierMode,
        ) {
            if mode == WriteBarrierMode::SkipWriteBarrier {
                //SLOW_DCHECK(!WriteBarrier::IsRequired(host, value));
                return;
            }

            // Used during InstructionStream initialization where we update the write
            // barriers together separate from the field writes.
            if mode == WriteBarrierMode::UnsafeSkipWriteBarrier {
                assert!(!DisallowGarbageCollection::is_allowed());
                return;
            }

            assert_eq!(mode, WriteBarrierMode::UpdateWriteBarrier);
            Self::generational_for_reloc_info(host, rinfo, value);
            Self::shared_for_reloc_info(host, rinfo, value);
            Self::marking_for_reloc_info(host, rinfo, value);
        }

        pub fn for_value<T>(
            host: Tagged<HeapObject>,
            slot: MaybeObjectSlot,
            value: Tagged<T>,
            mode: WriteBarrierMode,
        ) where
            T: Copy,
        {
            if mode == WriteBarrierMode::SkipWriteBarrier {
                //SLOW_DCHECK(!WriteBarrier::IsRequired(host, value));
                return;
            }

            let value_object = match value.get_heap_object() {
                Ok(value_object) => value_object,
                Err(_e) => return,
            };
            Self::combined_write_barrier_internal(
                host,
                HeapObjectSlot(slot),
                value_object,
                mode,
            );
        }

        pub fn for_value_heap_object_layout<T>(
            host: &mut HeapObjectLayout,
            slot: &mut TaggedMemberBase,
            value: Tagged<T>,
            mode: WriteBarrierMode,
        ) where
            T: Copy,
        {
            if mode == WriteBarrierMode::SkipWriteBarrier {
                //SLOW_DCHECK(!WriteBarrier::IsRequired(host, value));
                return;
            }
            let value_object = match value.get_heap_object() {
                Ok(value_object) => value_object,
                Err(_e) => return,
            };
            Self::combined_write_barrier_internal(
                Tagged(HeapObject(host as *mut _ as usize)),
                HeapObjectSlot(*slot),
                value_object,
                mode,
            );
        }

        pub fn for_ephemeron_hash_table(
            host: Tagged<EphemeronHashTable>,
            slot: ObjectSlot,
            value: Tagged<Object>,
            mode: WriteBarrierMode,
        ) {
            if mode == WriteBarrierMode::SkipWriteBarrier {
                //SLOW_DCHECK(!WriteBarrier::IsRequired(host, value));
                return;
            }

            assert_eq!(mode, WriteBarrierMode::UpdateWriteBarrier);
            if !value.0.is_heap_object() {
                return;
            }

            let host_chunk = MemoryChunk::from_heap_object(host);

            let heap_object_value = HeapObject(value.0 as usize);
            let heap_object_value = Tagged::new(heap_object_value);
            let value_chunk = MemoryChunk::from_heap_object(heap_object_value);

            let pointers_from_here_are_interesting = !host_chunk.is_young_or_shared_chunk();
            let is_marking = host_chunk.is_marking();

            if pointers_from_here_are_interesting && value_chunk.is_young_or_shared_chunk() {
                Self::combined_generational_and_shared_ephemeron_barrier_slow(
                    host,
                    slot.as_ptr(),
                    heap_object_value,
                );
            }

            // Marking barrier: mark value & record slots when marking is on.
            if is_marking {
                Self::marking_slow(host, HeapObjectSlot(slot), heap_object_value);
            }
        }

        pub fn for_external_pointer(
            host: Tagged<HeapObject>,
            slot: ExternalPointerSlot,
            mode: WriteBarrierMode,
        ) {
            if mode == WriteBarrierMode::SkipWriteBarrier {
                //SLOW_DCHECK(HeapLayout::InYoungGeneration(host));
                return;
            }
            Self::marking(host, slot);
        }

        pub fn for_indirect_pointer(
            host: Tagged<HeapObject>,
            slot: IndirectPointerSlot,
            value: Tagged<HeapObject>,
            mode: WriteBarrierMode,
        ) {
            // Indirect pointers are only used when the sandbox is enabled.
            assert!(true); //DCHECK(V8_ENABLE_SANDBOX_BOOL);
            if mode == WriteBarrierMode::SkipWriteBarrier {
                //SLOW_DCHECK(!WriteBarrier::IsRequired(host, value));
                return;
            }
            // Objects referenced via indirect pointers are currently never allocated in
            // the young generation.
            if !v8_flags.sticky_mark_bits {
                assert!(!MemoryChunk::from_heap_object(value).in_young_generation());
            }
            Self::marking(host, slot);
        }

        pub fn for_js_dispatch_handle(
            host: Tagged<HeapObject>,
            handle: JSDispatchHandle,
            mode: WriteBarrierMode,
        ) {
            assert!(true); //DCHECK(V8_ENABLE_LEAPTIERING_BOOL);
                            //SLOW_DCHECK(
                            //    WriteBarrier::VerifyDispatchHandleMarkingState(host, handle, mode));
            if mode == WriteBarrierMode::SkipWriteBarrier {
                return;
            }
            Self::marking(host, handle);
        }

        pub fn for_protected_pointer(
            host: Tagged<TrustedObject>,
            slot: ProtectedPointerSlot,
            value: Tagged<TrustedObject>,
            mode: WriteBarrierMode,
        ) {
            if mode == WriteBarrierMode::SkipWriteBarrier {
                //SLOW_DCHECK(!WriteBarrier::IsRequired(host, value));
                return;
            }
            // Protected pointers are only used within trusted and shared trusted space.
            assert!(!v8_flags.sticky_mark_bits || !MemoryChunk::from_heap_object(value).in_young_generation());
            if MemoryChunk::from_heap_object(value).in_writable_shared_space() {
                Self::shared_slow(host, slot, value);
            }
            Self::marking(host, slot, value);
        }

        pub fn generational_for_reloc_info(
            host: Tagged<InstructionStream>,
            rinfo: &mut RelocInfo,
            object: Tagged<HeapObject>,
        ) {
            if !heap_layout::in_young_generation(object) {
                return;
            }
            Self::generational_barrier_for_code_slow(host, rinfo, object);
        }

        pub fn is_marking(object: Tagged<HeapObject>) -> bool {
            MemoryChunk::from_heap_object(object).is_marking()
        }

        pub fn marking_for_testing(host: Tagged<HeapObject>, slot: ObjectSlot, value: Tagged<Object>) {
            assert!(!has_weak_heap_object_tag(value));
            if !value.0.is_heap_object() {
                return;
            }
            let heap_object_value = HeapObject(value.0 as usize);
            let heap_object_value = Tagged::new(heap_object_value);
            Self::marking(host, HeapObjectSlot(slot), heap_object_value);
        }

        pub fn marking(host: Tagged<HeapObject>, slot: MaybeObjectSlot, value: Tagged<MaybeObject>) {
            let value_heap_object = match value {
                Object(value_heap_object) => value_heap_object,
                Cleared => return,
            };
            let value_heap_object = Tagged::new(value_heap_object);

            assert!(!heap_layout::in_code_space(value_heap_object));
            Self::marking(host, HeapObjectSlot(slot), value_heap_object);
        }

        pub fn marking(host: Tagged<HeapObject>, slot: HeapObjectSlot, value: Tagged<HeapObject>) {
            if !Self::is_marking(host) {
                return;
            }
            Self::marking_slow(host, slot, value);
        }

        pub fn marking_for_reloc_info(
            host: Tagged<InstructionStream>,
            reloc_info: &mut RelocInfo,
            value: Tagged<HeapObject>,
        ) {
            if !Self::is_marking(host) {
                return;
            }
            Self::marking_slow(host, reloc_info, value);
        }

        pub fn shared_for_reloc_info(
            host: Tagged<InstructionStream>,
            reloc_info: &mut RelocInfo,
            value: Tagged<HeapObject>,
        ) {
            let value_chunk = MemoryChunk::from_heap_object(value);
            if !value_chunk.in_writable_shared_space() {
                return;
            }
            Self::shared_slow(host, reloc_info, value);
        }

        pub fn for_array_buffer_extension(
            host: Tagged<crate::objects::js_array_buffer::JSArrayBuffer>,
            extension: *mut std::ffi::c_void, //ArrayBufferExtension*
        ) {
            if extension.is_null() || !Self::is_marking(host) {
                return;
            }
            Self::marking_slow(host, extension);
        }

        pub fn for_descriptor_array(descriptor_array: Tagged<DescriptorArray>, number_of_own_descriptors: i32) {
            if !Self::is_marking(descriptor_array) {
                return;
            }
            Self::marking_slow(descriptor_array, number_of_own_descriptors);
        }

        pub fn marking(host: Tagged<HeapObject>, slot: ExternalPointerSlot) {
            if !Self::is_marking(host) {
                return;
            }
            Self::marking_slow(host, slot);
        }

        pub fn marking(host: Tagged<HeapObject>, slot: IndirectPointerSlot) {
            if !Self::is_marking(host) {
                return;
            }
            Self::marking_slow(host, slot);
        }

        pub fn marking(host: Tagged<TrustedObject>, slot: ProtectedPointerSlot, value: Tagged<TrustedObject>) {
            if !Self::is_marking(host) {
                return;
            }
            Self::marking_slow(host, slot, value);
        }

        pub fn marking(host: Tagged<HeapObject>, handle: JSDispatchHandle) {
            if !Self::is_marking(host) {
                return;
            }
            Self::marking_slow(host, handle);
        }

        pub fn marking_from_traced_handle(value: Tagged<Object>) {
            if !value.0.is_heap_object() {
                return;
            }
            let heap_object_value = HeapObject(value.0 as usize);
            let heap_object_value = Tagged::new(heap_object_value);

            Self::marking_slow_from_traced_handle(heap_object_value);
        }

        pub fn for_cpp_heap_pointer(
            host: Tagged<crate::objects::js_object::JSObject>,
            slot: CppHeapPointerSlot,
            value: *mut std::ffi::c_void,
        ) {
            // Note: this is currently a combined barrier for marking both the
            // CppHeapPointerTable entry and the referenced object.

            if !Self::is_marking(host) {
                //There is no young-gen CppHeapPointerTable space so we should not mark
                // the table entry in this case.
                if !value.is_null() {
                    Self::generational_barrier_for_cpp_heap_pointer(host, value);
                }
                return;
            }
            let marking_barrier_ptr =
                crate::heap::heap_write_barrier::current_marking_barrier::current_marking_barrier(
                    host,
                );
            let marking_barrier = unsafe { &mut *marking_barrier_ptr };

            if marking_barrier.is_minor() {
                // TODO(v8:13012): We do not currently mark Oilpan objects while MinorMS is
                // active. Once Oilpan uses a generational GC with incremental marking and
                // unified heap, this barrier will be needed again.
                return;
            }
            Self::marking_slow_from_cpp_heap_wrappable(marking_barrier.heap(), host, slot, value);
        }

        pub fn generational_barrier_for_cpp_heap_pointer(
            host: Tagged<crate::objects::js_object::JSObject>,
            value: *mut std::ffi::c_void,
        ) {
            if value.is_null() {
                return;
            }
            let memory_chunk = MemoryChunk::from_heap_object(host);
            if heap_layout::in_young_generation(&memory_chunk, host) {
                return;
            }
            let cpp_heap_ptr = memory_chunk.get_heap().cpp_heap();
            let cpp_heap = unsafe { &mut *crate::heap::heap_write_barrier::cpp_heap::CppHeap::from(cpp_heap_ptr) };
            cpp_heap.remember_cross_heap_reference_if_needed(host, value);
        }

        // Dummy methods implementations

        fn combined_generational_and_shared_barrier_slow(
            _host: Tagged<HeapObject>,
            _slot: *mut Object,
            _value: Tagged<HeapObject>,
        ) {
            // TODO: Implement
        }

        fn combined_generational_and_shared_ephemeron_barrier_slow(
            _host: Tagged<EphemeronHashTable>,
            _slot: *mut Object,
            _value: Tagged<HeapObject>,
        ) {
            // TODO: Implement
        }
        fn marking_slow(_host: Tagged<HeapObject>, _slot: HeapObjectSlot, _value: Tagged<HeapObject>) {
            // TODO: Implement
        }

        fn marking_slow(_host: Tagged<HeapObject>, _slot: &mut RelocInfo, _value: Tagged<HeapObject>) {
            // TODO: Implement
        }

        fn generational_barrier_for_code_slow(
            _host: Tagged<InstructionStream>,
            _rinfo: &mut RelocInfo,
            _object: Tagged<HeapObject>,
        ) {
            // TODO: Implement
        }

        fn shared_slow(_host: Tagged<InstructionStream>, _reloc_info: &mut RelocInfo, _value: Tagged<HeapObject>) {
            // TODO: Implement
        }

        fn marking_slow_from_traced_handle(_value: Tagged<HeapObject>) {
            // TODO: Implement
        }

        fn marking_slow_from_cpp_heap_wrappable(
            _heap: *mut std::ffi::c_void, // TODO: Type CppHeap
            _host: Tagged<crate::objects::js_object::JSObject>,
            _slot: CppHeapPointerSlot,
            _value: *mut std::ffi::c_void,
        ) {
            // TODO: Implement
        }

        fn shared_slow(
            _host: Tagged<TrustedObject>,
            _slot: ProtectedPointerSlot,
            _value: Tagged<TrustedObject>,
        ) {
            // TODO: Implement
        }

        fn marking_slow(_host: Tagged<HeapObject>, _slot: *mut std::ffi::c_void) {
            // TODO: Implement
        }

        fn marking_slow(_host: Tagged<HeapObject>, _slot: ExternalPointerSlot) {
            // TODO: Implement
        }

        fn marking_slow(_host: Tagged<HeapObject>, _slot: IndirectPointerSlot) {
            // TODO: Implement
        }

        fn marking_slow(_host: Tagged<TrustedObject>, _slot: ProtectedPointerSlot, _value: Tagged<TrustedObject>) {
            // TODO: Implement
        }

        fn marking_slow(_host: Tagged<HeapObject>, _handle: JSDispatchHandle) {
            // TODO: Implement
        }

        fn marking_slow(_descriptor_array: Tagged<DescriptorArray>, _number_of_own_descriptors: i32) {
            // TODO: Implement
        }
    }

    pub struct DisallowGarbageCollection {}

    impl DisallowGarbageCollection {
        pub fn is_allowed() -> bool {
            false
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HeapObjectLayout {}

    impl HeapObjectLayout {
        pub fn new() -> Self {
            HeapObjectLayout {}
        }
    }
}

mod heap {
    pub mod heap_layout {
        use crate::heap::heap_write_barrier::{HeapObject, MemoryChunk, Tagged};

        pub fn in_young_generation(_chunk: &MemoryChunk, _object: Tagged<HeapObject>) -> bool {
            false
        }

        pub fn in_code_space(_value_heap_object: Tagged<HeapObject>) -> bool {
            false
        }

        pub fn in_read_only_space(_object: Tagged<HeapObject>) -> bool {
            false
        }
    }

    pub mod marking_barrier {
        pub struct MarkingBarrier {
            heap: *mut std::ffi::c_void, // Replace with the actual type for Heap
            is_minor: bool,
        }

        impl MarkingBarrier {
            pub fn new(heap: *mut std::ffi::c_void, is_minor: bool) -> Self {
                MarkingBarrier { heap, is_minor }
            }
            pub fn is_minor(&self) -> bool {
                self.is_minor
            }

            pub fn heap(&self) -> *mut std::ffi::c_void {
                self.heap
            }
        }
    }

    pub mod memory_chunk {
        use crate::heap::heap_write_barrier::{HeapObject, Tagged};

        pub struct MemoryChunk {
            marking: bool,
            young_or_shared_chunk: bool,
            writable_shared_space: bool,
            heap: *mut std::ffi::c_void,
            in_young_generation: bool,
        }

        impl MemoryChunk {
            pub fn from_heap_object(_object: Tagged<HeapObject>) -> Self {
                MemoryChunk {
                    marking: false,
                    young_or_shared_chunk: false,
                    writable_shared_space: false,
                    heap: std::ptr::null_mut(),
                    in_young_generation: false,
                }
            }

            pub fn is_marking(&self) -> bool {
                self.marking
            }

            pub fn is_young_or_shared_chunk(&self) -> bool {
                self.young_or_shared_chunk
            }

            pub fn in_writable_shared_space(&self) -> bool {
                self.writable_shared_space
            }

            pub fn get_heap(&self) -> &Self {
                self
            }

            pub fn cpp_heap(&self) -> *mut std::ffi::c_void {
                std::ptr::null_mut()
            }
            pub fn in_young_generation(&self) -> bool {
                self.in_young_generation
            }
        }
    }
}

mod objects {
    pub mod compressed_slots {}

    pub mod descriptor_array {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct DescriptorArray {}

        impl DescriptorArray {
            pub fn new() -> Self {
                DescriptorArray {}
            }
        }
    }

    pub mod ephemeron_hash_table {
        use crate::heap::heap_write_barrier::HeapObject;
        use crate::heap::heap_write_barrier::Tagged;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct EphemeronHashTable(pub usize);

        impl EphemeronHashTable {
            pub fn new(value: usize) -> Self {
                EphemeronHashTable(value)
            }
        }
    }

    pub mod js_object {
        use crate::heap::heap_write_barrier::HeapObject;
        use crate::heap::heap_write_barrier::Tagged;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct JSObject(pub usize);

        impl JSObject {
            pub fn new(value: usize) -> Self {
                JSObject(value)
            }
        }
    }

    pub mod js_array_buffer {
        use crate::heap::heap_write_barrier::Tagged;
        use crate::objects::js_object::JSObject;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct JSArrayBuffer(pub JSObject);

        impl JSArrayBuffer {
            pub fn new(js_object: JSObject) -> Self {
                JSArrayBuffer(js_object)
            }
        }
    }

    pub mod maybe_object {
        use crate::heap::heap_write_barrier::HeapObject;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MaybeObject {
            Object(HeapObject),
            Cleared,
        }
    }

    pub mod object {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Object(pub usize);

        impl Object {
            pub fn new(value: usize) -> Self {
                Object(value)
            }
            pub fn is_heap_object(&self) -> bool {
                true
            }
        }
    }

    pub mod shared {
        use crate::heap::heap_write_barrier::HeapObject;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct TrustedObject(pub HeapObject);
    }

    pub mod smi {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Smi(pub usize);
    }
}

mod utils {
    pub mod flag_utils {
        pub struct V8Flags {
            pub disable_write_barriers: bool,
            pub sticky_mark_bits: bool,
        }

        impl V8Flags {
            pub fn new() -> Self {
                V8Flags {
                    disable_write_barriers: false,
                    sticky_mark_bits: false,
                }
            }
        }
    }
}