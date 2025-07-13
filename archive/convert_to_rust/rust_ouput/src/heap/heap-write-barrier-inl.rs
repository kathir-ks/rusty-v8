// Converted from V8 C++ source files:
// Header: heap-write-barrier-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap_write_barrier_inl {
use std::sync::Mutex;

use crate::heap::heap_layout_inl::*;
use crate::heap::heap_write_barrier::*;
use crate::heap::marking_barrier::*;
use crate::heap::memory_chunk::*;
use crate::objects::compressed_slots_inl::*;
use crate::objects::descriptor_array::*;
use crate::objects::maybe_object_inl::*;
use crate::V8;

// static
pub fn combined_write_barrier_internal(
    host: Tagged<HeapObject>,
    slot: HeapObjectSlot,
    value: Tagged<HeapObject>,
    mode: WriteBarrierMode,
) {
    if mode != WriteBarrierMode::UPDATE_WRITE_BARRIER {
        return;
    }

    let host_chunk = MemoryChunk::from_heap_object(host);
    let value_chunk = MemoryChunk::from_heap_object(value);

    let is_marking = host_chunk.is_marking();

    if V8::flags().sticky_mark_bits {
        if !HeapLayout::in_young_generation(&host_chunk, host)
            && HeapLayout::in_young_generation(&value_chunk, value)
        {
            combined_generational_and_shared_barrier_slow(host, slot.address(), value);
        }
    } else {
        let pointers_from_here_are_interesting = !host_chunk.is_young_or_shared_chunk();
        if pointers_from_here_are_interesting && value_chunk.is_young_or_shared_chunk() {
            combined_generational_and_shared_barrier_slow(host, slot.address(), value);
        }
    }

    if is_marking {
        marking_slow(host, HeapObjectSlot(slot), value);
    }
}

// static
pub fn get_write_barrier_mode_for_object(
    object: Tagged<HeapObject>,
    _promise: &DisallowGarbageCollection,
) -> WriteBarrierMode {
    if V8::flags().disable_write_barriers {
        return WriteBarrierMode::SKIP_WRITE_BARRIER;
    }

    if !page_flags_are_consistent(object) {
        return WriteBarrierMode::UPDATE_WRITE_BARRIER; // Provide a default value if page flags are inconsistent.
    }

    let chunk = MemoryChunk::from_heap_object(object);
    if chunk.is_marking() {
        return WriteBarrierMode::UPDATE_WRITE_BARRIER;
    }
    if HeapLayout::in_young_generation(&chunk, object) {
        return WriteBarrierMode::SKIP_WRITE_BARRIER;
    }
    WriteBarrierMode::UPDATE_WRITE_BARRIER
}

// static
pub fn is_immortal_immovable_heap_object(object: Tagged<HeapObject>) -> bool {
    HeapLayout::in_read_only_space(object)
}

// static
pub fn for_reloc_info(
    host: Tagged<InstructionStream>,
    rinfo: *mut RelocInfo,
    value: Tagged<HeapObject>,
    mode: WriteBarrierMode,
) {
    if mode == WriteBarrierMode::SKIP_WRITE_BARRIER {
        return;
    }

    if mode == WriteBarrierMode::UNSAFE_SKIP_WRITE_BARRIER {
        return;
    }

    if mode != WriteBarrierMode::UPDATE_WRITE_BARRIER {
        return;
    }
    generational_for_reloc_info(host, rinfo, value);
    shared_for_reloc_info(host, rinfo, value);
    marking_for_reloc_info(host, rinfo, value);
}

// static
pub fn for_value<T>(
    host: Tagged<HeapObject>,
    slot: MaybeObjectSlot,
    value: Tagged<T>,
    mode: WriteBarrierMode,
) {
    if mode == WriteBarrierMode::SKIP_WRITE_BARRIER {
        return;
    }
    let mut value_object: Tagged<HeapObject> = Tagged { ptr: 0 };

    if !value.get_heap_object(&mut value_object) {
        return;
    }
    combined_write_barrier_internal(host, HeapObjectSlot(slot), value_object, mode);
}

// static
pub fn for_value_layout<T>(
    host: *mut HeapObjectLayout,
    slot: *mut TaggedMemberBase,
    value: Tagged<T>,
    mode: WriteBarrierMode,
) {
    if mode == WriteBarrierMode::SKIP_WRITE_BARRIER {
        return;
    }
    let mut value_object: Tagged<HeapObject> = Tagged { ptr: 0 };
    if !value.get_heap_object(&mut value_object) {
        return;
    }
    unsafe {
    combined_write_barrier_internal(
        Tagged(*host),
        HeapObjectSlot(ObjectSlot(*slot)),
        value_object,
        mode,
    );
    }
}

//   static
pub fn for_ephemeron_hash_table(
    host: Tagged<EphemeronHashTable>,
    slot: ObjectSlot,
    value: Tagged<Object>,
    mode: WriteBarrierMode,
) {
    if mode == WriteBarrierMode::SKIP_WRITE_BARRIER {
        return;
    }

    if mode != WriteBarrierMode::UPDATE_WRITE_BARRIER {
        return;
    }
    if !value.is_heap_object() {
        return;
    }

    let host_chunk = MemoryChunk::from_heap_object(host);

    let heap_object_value: Tagged<HeapObject> = unsafe { std::mem::transmute(value) };
    let value_chunk = MemoryChunk::from_heap_object(heap_object_value);

    let pointers_from_here_are_interesting = !host_chunk.is_young_or_shared_chunk();
    let is_marking = host_chunk.is_marking();

    if pointers_from_here_are_interesting && value_chunk.is_young_or_shared_chunk() {
        combined_generational_and_shared_ephemeron_barrier_slow(
            host,
            slot.address(),
            heap_object_value,
        );
    }

    if is_marking {
        marking_slow(host, HeapObjectSlot(slot), heap_object_value);
    }
}

// static
pub fn for_external_pointer(
    host: Tagged<HeapObject>,
    slot: ExternalPointerSlot,
    mode: WriteBarrierMode,
) {
    if mode == WriteBarrierMode::SKIP_WRITE_BARRIER {
        if !HeapLayout::in_young_generation(&MemoryChunk::from_heap_object(host), host) {
            return;
        }
    }
    marking(host, slot);
}

// static
pub fn for_indirect_pointer(
    host: Tagged<HeapObject>,
    slot: IndirectPointerSlot,
    value: Tagged<HeapObject>,
    mode: WriteBarrierMode,
) {
    if mode == WriteBarrierMode::SKIP_WRITE_BARRIER {
        return;
    }
    marking(host, slot);
}

// static
pub fn for_js_dispatch_handle(
    host: Tagged<HeapObject>,
    handle: JSDispatchHandle,
    mode: WriteBarrierMode,
) {
    if mode == WriteBarrierMode::SKIP_WRITE_BARRIER {
        return;
    }
    marking(host, handle);
}

// static
pub fn for_protected_pointer(
    host: Tagged<TrustedObject>,
    slot: ProtectedPointerSlot,
    value: Tagged<TrustedObject>,
    mode: WriteBarrierMode,
) {
    if mode == WriteBarrierMode::SKIP_WRITE_BARRIER {
        return;
    }
    let value_chunk = MemoryChunk::from_heap_object(value);
    if value_chunk.in_writable_shared_space() {
        shared_slow(host, slot, value);
    }
    marking(host, slot, value);
}

// static
pub fn generational_for_reloc_info(
    host: Tagged<InstructionStream>,
    rinfo: *mut RelocInfo,
    object: Tagged<HeapObject>,
) {
    if !HeapLayout::in_young_generation(
        &MemoryChunk::from_heap_object(object),
        object,
    ) {
        return;
    }
    generational_barrier_for_code_slow(host, rinfo, object);
}

// static
pub fn is_marking(object: Tagged<HeapObject>) -> bool {
    MemoryChunk::from_heap_object(object).is_marking()
}

pub fn marking_for_testing(
    host: Tagged<HeapObject>,
    slot: ObjectSlot,
    value: Tagged<Object>,
) {
    if value.has_weak_heap_object_tag() {
        return;
    }
    if !value.is_heap_object() {
        return;
    }
    let value_heap_object: Tagged<HeapObject> = unsafe { std::mem::transmute(value) };
    marking(host, HeapObjectSlot(slot), value_heap_object);
}

pub fn marking(host: Tagged<HeapObject>, slot: MaybeObjectSlot, value: Tagged<MaybeObject>) {
    let mut value_heap_object: Tagged<HeapObject> = Tagged { ptr: 0 };
    if !value.get_heap_object(&mut value_heap_object) {
        return;
    }
    if HeapLayout::in_code_space(value_heap_object) {
        return;
    }
    marking(host, HeapObjectSlot(slot), value_heap_object);
}

pub fn marking(host: Tagged<HeapObject>, slot: HeapObjectSlot, value: Tagged<HeapObject>) {
    if !is_marking(host) {
        return;
    }
    marking_slow(host, slot, value);
}

pub fn marking_for_reloc_info(
    host: Tagged<InstructionStream>,
    reloc_info: *mut RelocInfo,
    value: Tagged<HeapObject>,
) {
    if !is_marking(host) {
        return;
    }
    marking_slow(host, reloc_info, value);
}

pub fn shared_for_reloc_info(
    host: Tagged<InstructionStream>,
    reloc_info: *mut RelocInfo,
    value: Tagged<HeapObject>,
) {
    let value_chunk = MemoryChunk::from_heap_object(value);
    if !value_chunk.in_writable_shared_space() {
        return;
    }
    shared_slow(host, reloc_info, value);
}

pub fn for_array_buffer_extension(host: Tagged<JSArrayBuffer>, extension: *mut ArrayBufferExtension) {
    if extension.is_null() || !is_marking(host) {
        return;
    }
    marking_slow(host, extension);
}

pub fn for_descriptor_array(
    descriptor_array: Tagged<DescriptorArray>,
    number_of_own_descriptors: i32,
) {
    if !is_marking(descriptor_array) {
        return;
    }
    marking_slow(descriptor_array, number_of_own_descriptors);
}

pub fn marking(host: Tagged<HeapObject>, slot: ExternalPointerSlot) {
    if !is_marking(host) {
        return;
    }
    marking_slow(host, slot);
}

pub fn marking(host: Tagged<HeapObject>, slot: IndirectPointerSlot) {
    if !is_marking(host) {
        return;
    }
    marking_slow(host, slot);
}

pub fn marking(host: Tagged<TrustedObject>, slot: ProtectedPointerSlot, value: Tagged<TrustedObject>) {
    if !is_marking(host) {
        return;
    }
    marking_slow(host, slot, value);
}

pub fn marking(host: Tagged<HeapObject>, handle: JSDispatchHandle) {
    if !is_marking(host) {
        return;
    }
    marking_slow(host, handle);
}

// static
pub fn marking_from_traced_handle(value: Tagged<Object>) {
    if !value.is_heap_object() {
        return;
    }
    marking_slow_from_traced_handle(unsafe { std::mem::transmute(value) });
}

// static
pub fn for_cpp_heap_pointer(host: Tagged<JSObject>, slot: CppHeapPointerSlot, value: *mut std::ffi::c_void) {
    if !is_marking(host) {
        if value.is_null() {
          return;
        }
        unsafe {
          generational_barrier_for_cpp_heap_pointer(host, value);
        }
        return;
    }
    unsafe {
        let marking_barrier = current_marking_barrier(host);
        if marking_barrier.is_minor() {
            return;
        }
        marking_slow_from_cpp_heap_wrappable(marking_barrier.heap(), host, slot, value);
    }
}

// static
pub fn generational_barrier_for_cpp_heap_pointer(host: Tagged<JSObject>, value: *mut std::ffi::c_void) {
    if value.is_null() {
        return;
    }
    let memory_chunk = MemoryChunk::from_heap_object(host);
    if HeapLayout::in_young_generation(&memory_chunk, host) {
        return;
    }
    unsafe {
        let cpp_heap = memory_chunk.get_heap().cpp_heap();
        v8::internal::cpp_heap::CppHeap::from(cpp_heap).remember_cross_heap_reference_if_needed(
            host, value,
        );
    }
}

fn page_flags_are_consistent(object: Tagged<HeapObject>) -> bool {
    true
}
} // mod heap_write_barrier_inl

mod v8 {
  pub mod internal {
    pub mod cpp_heap {
      pub struct CppHeap {}

      impl CppHeap {
        pub fn from(_ptr: *mut super::CppHeap) -> CppHeap {
          CppHeap {}
        }
        pub fn remember_cross_heap_reference_if_needed<T>(&self, _host: T, _value: *mut std::ffi::c_void) {}
      }
    }

    pub struct CppHeap {}
  }
}
