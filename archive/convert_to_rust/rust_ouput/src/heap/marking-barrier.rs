// Converted from V8 C++ source files:
// Header: marking-barrier.h
// Implementation: marking-barrier.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap {
pub mod marking_barrier {
use crate::heap::MarkingState;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;
use std::sync::{Mutex, MutexGuard};

pub struct Heap {}
pub struct Isolate {}
pub struct LocalHeap {
    heap_: *mut Heap,
    is_main_thread_: bool,
}
impl LocalHeap {
    pub fn heap(&self) -> &mut Heap {
        unsafe { &mut *self.heap_ }
    }
    pub fn is_main_thread(&self) -> bool {
        self.is_main_thread_
    }
    pub fn marking_barrier(&mut self) -> &mut MarkingBarrier {
        unsafe { &mut *(&mut self.marking_barrier_ as *mut MarkingBarrier) }
    }
}
pub struct IncrementalMarking {}
pub struct PagedSpace {}
pub struct NewSpace {}
pub struct MarkCompactCollector {}
impl MarkCompactCollector {
    pub fn epoch(&self) -> u32 {
        1
    }
    pub fn RecordRelocSlot(
        host: Tagged<InstructionStream>,
        reloc_info: *mut RelocInfo,
        value: Tagged<HeapObject>,
    ) {
    }
    pub fn marking_worklists(&self) -> &MarkingWorklists {
        unsafe { std::mem::transmute(0usize) }
    }
}
pub struct RelocInfo {}
pub struct InstructionStream {}
pub struct JSArrayBuffer {}
pub struct ArrayBufferExtension {}
pub struct DescriptorArray {}
pub struct HeapObject {}
pub struct IndirectPointerSlot {}
impl IndirectPointerSlot {
    pub fn load(&self, _isolate: &Isolate) -> Tagged<HeapObject> {
        unsafe { Tagged::<HeapObject>::assume_valid(MaybeUninit::zeroed().assume_init()) }
    }
}
pub struct MutablePageMetadata {
    mutex_: Mutex<()>,
}
impl MutablePageMetadata {
    pub fn mutex(&self) -> &Mutex<()> {
        &self.mutex_
    }
}
pub struct TypedSlots {}
pub struct OldSpace {}
pub struct SharedSpace {}
pub struct TrustedSpace {}
pub struct CodeSpace {}
pub struct OldLargeObjectSpace {}
pub struct SharedLargeObjectSpace {}
pub struct TrustedLargeObjectSpace {}
pub struct CodeLargeObjectSpace {}
pub struct NewLargeObjectSpace {}
pub struct MemoryChunk {}
impl MemoryChunk {
    pub fn FromHeapObject(host: Tagged<HeapObject>) -> *mut MemoryChunk {
        null_mut()
    }
    pub fn Metadata(&self) -> &MutablePageMetadata {
        unsafe { std::mem::transmute(0usize) }
    }
    pub fn IsMarking(&self) -> bool {
        false
    }
    pub fn IsLargePage(&self) -> bool {
        false
    }
}
pub struct HeapLayout {}
impl HeapLayout {
    pub fn InWritableSharedSpace(host: Tagged<HeapObject>) -> bool {
        false
    }
    pub fn InReadOnlySpace(value: Tagged<HeapObject>) -> bool {
        false
    }
    pub fn InYoungGeneration(value: Tagged<HeapObject>) -> bool {
        false
    }
    pub fn InBlackAllocatedPage(descriptor_array: Tagged<DescriptorArray>) -> bool {
        false
    }
}
pub struct MarkingWorklistsLocal {}
impl MarkingWorklistsLocal {
    pub fn Push(&self, descriptor_array: Tagged<DescriptorArray>) {}
    pub fn IsEmpty(&self) -> bool {
        true
    }
    pub fn Publish(&self) {}
}
pub struct RwxMemoryWriteScope {
    _dummy: String,
}
impl RwxMemoryWriteScope {
    pub fn new(_dummy: &str) -> Self {
        Self {
            _dummy: String::from(""),
        }
    }
}
impl Drop for RwxMemoryWriteScope {
    fn drop(&mut self) {}
}
pub struct Safepoint {}
impl Safepoint {
    pub fn IterateLocalHeaps<F>(&self, mut f: F)
    where
        F: FnMut(&mut LocalHeap),
    {
    }
}
pub struct RememberedSet<T> {
    _dummy: i32,
    phantom: PhantomData<T>,
}
impl<T> RememberedSet<T> {
    pub fn MergeTyped(
        memory_chunk: *mut MutablePageMetadata,
        typed_slots: std::unique_ptr<TypedSlots>,
    ) {
    }
}
pub struct MarkingHelper {}
impl MarkingHelper {
    pub enum WorklistTarget {
        kRegular,
    }
    pub fn ShouldMarkObject(
        heap_: *mut Heap,
        descriptor_array: Tagged<DescriptorArray>,
    ) -> Option<MarkingHelper::WorklistTarget> {
        None
    }
}
pub struct GlobalSafepoint {}
impl GlobalSafepoint {
    pub fn IterateClientIsolates<F>(&self, mut f: F)
    where
        F: FnMut(&mut Isolate),
    {
    }
}
pub struct MarkingWorklists {}
pub struct Space {}
pub struct CodePointerHandle {}
pub struct CodePointerTable {}
impl CodePointerTable {
    pub const kSupportsCompaction: bool = false;
}
pub struct TrustedPointerTable {}
impl TrustedPointerTable {
    pub const kSupportsCompaction: bool = false;
}
pub struct WriteBarrier {}
impl WriteBarrier {
    pub fn CurrentMarkingBarrier(
        verification_candidate: Tagged<HeapObject>,
    ) -> *mut MarkingBarrier {
        null_mut()
    }
}
pub struct DescriptorArrayMarkingState {}
impl DescriptorArrayMarkingState {
    pub fn TryUpdateIndicesToMark(
        gc_epoch: u32,
        descriptor_array: Tagged<DescriptorArray>,
        number_of_own_descriptors: i32,
    ) -> bool {
        false
    }
}
pub struct MinorMarkSweepCollector {}
impl MinorMarkSweepCollector {
    pub fn marking_worklists(&self) -> &MarkingWorklists {
        unsafe { std::mem::transmute(0usize) }
    }
}
#[derive(Clone, Copy)]
pub struct Tagged<T> {
    _dummy: i32,
    phantom: PhantomData<T>,
}
impl<T> Tagged<T> {
    pub unsafe fn assume_valid(val: T) -> Self {
        Self {
            _dummy: 0,
            phantom: PhantomData,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum MarkingMode {
    kNoMarking,
    kMajorMarking,
    kMinorMarking,
}
impl Default for MarkingMode {
    fn default() -> Self {
        MarkingMode::kNoMarking
    }
}
pub struct V8_EXPORT_PRIVATE {}
pub struct HeapFlags {}
impl HeapFlags {
    pub fn sticky_mark_bits(&self) -> bool {
        false
    }
    pub fn black_allocated_pages(&self) -> bool {
        false
    }
}
pub struct V8 {}
impl V8 {
    pub fn flags() -> HeapFlags {
        HeapFlags {}
    }
}
pub static v8_flags: HeapFlags = HeapFlags {};
pub const SHARED_TRUSTED_SPACE: i32 = 0;
impl Isolate {
    pub fn has_shared_space(&self) -> bool {
        false
    }
    pub fn is_shared_space_isolate(&self) -> bool {
        false
    }
    pub fn shared_space_isolate(&mut self) -> &mut Isolate {
        unsafe { &mut *(&mut self as *mut Isolate) }
    }
    pub fn heap(&mut self) -> &mut Heap {
        unsafe { &mut *(&mut self.heap_ as *mut Heap) }
    }
}

pub struct MarkingBarrier {
    heap_: *mut Heap,
    major_collector_: *mut MarkCompactCollector,
    minor_collector_: *mut MinorMarkSweepCollector,
    incremental_marking_: *mut IncrementalMarking,
    current_worklists_: std::unique_ptr<MarkingWorklists::Local>,
    shared_heap_worklists_: Option<MarkingWorklists::Local>,
    marking_state_: MarkingState,
    typed_slots_map_: HashMap<
        *mut MutablePageMetadata,
        std::unique_ptr<TypedSlots>,
    >,
    is_compacting_: bool,
    is_activated_: bool,
    is_main_thread_barrier_: bool,
    uses_shared_heap_: bool,
    is_shared_space_isolate_: bool,
    marking_mode_: MarkingMode,
}

impl MarkingBarrier {
    pub fn new(local_heap: &mut LocalHeap) -> Self {
        MarkingBarrier {
            heap_: local_heap.heap(),
            major_collector_: unsafe {
                &mut *(((*local_heap.heap()).mark_compact_collector())
                    as *mut MarkCompactCollector)
            },
            minor_collector_: unsafe {
                &mut *(((*local_heap.heap()).minor_mark_sweep_collector())
                    as *mut MinorMarkSweepCollector)
            },
            incremental_marking_: unsafe {
                &mut *(((*local_heap.heap()).incremental_marking()) as *mut IncrementalMarking)
            },
            marking_state_: MarkingState {},
            current_worklists_: std::unique_ptr::new(MarkingWorklists::Local {}),
            shared_heap_worklists_: None,
            typed_slots_map_: HashMap::new(),
            is_compacting_: false,
            is_activated_: false,
            is_main_thread_barrier_: local_heap.is_main_thread(),
            uses_shared_heap_: unsafe { ((*local_heap.heap()).isolate()).has_shared_space() },
            is_shared_space_isolate_: unsafe {
                ((*local_heap.heap()).isolate()).is_shared_space_isolate()
            },
            marking_mode_: MarkingMode::kNoMarking,
        }
    }

    pub fn write(&mut self, host: Tagged<HeapObject>, slot: IndirectPointerSlot) {
        if !self.is_current_marking_barrier(host) {
            return;
        }
        if !(self.is_activated_ || self.shared_heap_worklists_.is_some()) {
            return;
        }
        if !unsafe {
            (*MemoryChunk::FromHeapObject(host)).IsMarking()
        } {
            return;
        }
        let value = slot.load(self.isolate());
        if unsafe {
            HeapLayout::InReadOnlySpace(value)
        } {
            return;
        }
        if unsafe {
            HeapLayout::InYoungGeneration(value)
        } {
            return;
        }

        if self.uses_shared_heap_ && !self.is_shared_space_isolate_ {
            if unsafe {
                HeapLayout::InWritableSharedSpace(value)
            } {
                unsafe {
                    assert!(HeapLayout::InWritableSharedSpace(host));
                }
                if !unsafe {
                    (*MemoryChunk::FromHeapObject(value)).IsTrusted()
                } {
                    return;
                }
                self.mark_value_shared(value);
            } else {
                self.mark_value_local(value);
            }
        } else {
            self.mark_value_local(value);
        }
    }

    pub fn write_without_host(&mut self, value: Tagged<HeapObject>) {
        if !self.is_main_thread_barrier_ {
            return;
        }
        if !self.is_activated_ {
            return;
        }

        if self.uses_shared_heap_ && !self.is_shared_space_isolate_ {
            if unsafe {
                HeapLayout::InWritableSharedSpace(value)
            } {
                return;
            }
        }
        if unsafe {
            HeapLayout::InReadOnlySpace(value)
        } {
            return;
        }
        self.mark_value_local(value);
    }

    pub fn write(
        &mut self,
        host: Tagged<InstructionStream>,
        reloc_info: *mut RelocInfo,
        value: Tagged<HeapObject>,
    ) {
        if !self.is_current_marking_barrier(host) {
            return;
        }
        if unsafe {
            HeapLayout::InWritableSharedSpace(host)
        } {
            return;
        }
        if !(self.is_activated_ || self.shared_heap_worklists_.is_some()) {
            return;
        }
        if !unsafe {
            (*MemoryChunk::FromHeapObject(host)).IsMarking()
        } {
            return;
        }

        self.mark_value(host, value);

        if self.is_compacting_ {
            assert!(self.is_major());
            if self.is_main_thread_barrier_ {
                unsafe {
                    (&mut *self.major_collector_).RecordRelocSlot(host, reloc_info, value);
                }
            } else {
                self.record_reloc_slot(host, reloc_info, value);
            }
        }
    }

    pub fn write(&mut self, host: Tagged<JSArrayBuffer>, extension: *mut ArrayBufferExtension) {
        if !self.is_current_marking_barrier(host) {
            return;
        }
        if unsafe {
            HeapLayout::InWritableSharedSpace(host)
        } {
            return;
        }
        if !unsafe {
            (*MemoryChunk::FromHeapObject(host)).IsMarking()
        } {
            return;
        }

        if self.is_minor() {
            if unsafe {
                HeapLayout::InYoungGeneration(host)
            } {
                unsafe {
                    (*extension).YoungMark();
                }
            }
        } else {
            unsafe {
                (*extension).Mark();
            }
        }
    }

    pub fn write(&mut self, descriptor_array: Tagged<DescriptorArray>, number_of_own_descriptors: i32) {
        if !self.is_current_marking_barrier(descriptor_array) {
            return;
        }
        if unsafe {
            HeapLayout::InReadOnlySpace(descriptor_array.map())
        } {
            return;
        }
        if !unsafe {
            (*MemoryChunk::FromHeapObject(descriptor_array)).IsMarking()
        } {
            return;
        }

        if self.is_minor() || self.is_strong_descriptor_array(descriptor_array) {
            self.mark_value_local(descriptor_array);
            return;
        }

        let (gc_epoch, worklist) = if self.uses_shared_heap_
            && unsafe {
                HeapLayout::InWritableSharedSpace(descriptor_array)
            }
            && !self.is_shared_space_isolate_
        {
            let shared_isolate = unsafe {
                self.isolate()
                    .shared_space_isolate()
            };
            let gc_epoch = unsafe {
                shared_isolate
                    .heap()
                    .mark_compact_collector()
                    .epoch()
            };
            let worklist = self
                .shared_heap_worklists_
                .as_mut()
                .expect("shared_heap_worklists_ should have value");
            (gc_epoch, worklist)
        } else {
            let gc_epoch = unsafe { (&mut *self.major_collector_).epoch() };
            let worklist = self
                .current_worklists_
                .as_mut()
                .expect("current_worklists_ should have value");
            (gc_epoch, worklist)
        };

        if v8_flags.black_allocated_pages() {
            if MarkingHelper::ShouldMarkObject(self.heap_, descriptor_array).is_some() {
                self.marking_state_.TryMark(descriptor_array);
            }
        } else {
            self.marking_state_.TryMark(descriptor_array);
        }

        if DescriptorArrayMarkingState::TryUpdateIndicesToMark(
            gc_epoch,
            descriptor_array,
            number_of_own_descriptors,
        ) {
            worklist.Push(descriptor_array);
        }
    }

    fn record_reloc_slot(
        &mut self,
        host: Tagged<InstructionStream>,
        rinfo: *mut RelocInfo,
        target: Tagged<HeapObject>,
    ) {
        if !self.is_current_marking_barrier(host) {
            return;
        }
        if !MarkCompactCollector::ShouldRecordRelocSlot(host, rinfo, target) {
            return;
        }

        let info = MarkCompactCollector::ProcessRelocInfo(host, rinfo, target);

        let typed_slots = self
            .typed_slots_map_
            .entry(info.page_metadata)
            .or_insert_with(|| std::unique_ptr::new(TypedSlots {}));

        typed_slots.Insert(info.slot_type, info.offset);
    }

    pub fn activate_all(heap: &mut Heap, is_compacting: bool) {
        Self::activate_spaces(heap, MarkingMode::kMajorMarking);

        heap.safepoint().IterateLocalHeaps(
            |local_heap: &mut LocalHeap| {
                local_heap.marking_barrier().activate(
                    is_compacting,
                    MarkingMode::kMajorMarking,
                );
            },
        );

        if unsafe {
            heap.isolate().is_shared_space_isolate()
        } {
            unsafe {
                heap.isolate()
                    .shared_space_isolate()
                    .global_safepoint()
                    .IterateClientIsolates(|client: &mut Isolate| {
                        client.heap().SetIsMarkingFlag(true);
                        client.heap().safepoint().IterateLocalHeaps(
                            |local_heap: &mut LocalHeap| {
                                local_heap.marking_barrier().activate_shared();
                            },
                        );
                    });
            }
        }
    }

    pub fn activate_young(heap: &mut Heap) {
        Self::activate_spaces(heap, MarkingMode::kMinorMarking);

        heap.safepoint().IterateLocalHeaps(|local_heap: &mut LocalHeap| {
            local_heap
                .marking_barrier()
                .activate(false, MarkingMode::kMinorMarking);
        });
    }

    pub fn activate(&mut self, is_compacting: bool, marking_mode: MarkingMode) {
        assert!(!self.is_activated_);
        self.is_compacting_ = is_compacting;
        self.marking_mode_ = marking_mode;
        self.current_worklists_ =
            std::unique_ptr::new(MarkingWorklists::Local {});
        self.is_activated_ = true;
    }

    pub fn activate_shared(&mut self) {
        assert!(self.shared_heap_worklists_.is_none());
        let shared_isolate = unsafe {
            self.isolate()
                .shared_space_isolate()
        };
        self.shared_heap_worklists_ =
            Some(MarkingWorklists::Local {});
    }

    pub fn deactivate_all(heap: &mut Heap) {
        Self::deactivate_spaces(heap, MarkingMode::kMajorMarking);

        heap.safepoint().IterateLocalHeaps(|local_heap: &mut LocalHeap| {
            local_heap.marking_barrier().deactivate();
        });

        if unsafe {
            heap.isolate().is_shared_space_isolate()
        } {
            unsafe {
                heap.isolate()
                    .shared_space_isolate()
                    .global_safepoint()
                    .IterateClientIsolates(|client: &mut Isolate| {
                        let is_marking = client.heap().incremental_marking().IsMarking();
                        client.heap().SetIsMarkingFlag(is_marking);
                        client.heap().safepoint().IterateLocalHeaps(
                            |local_heap: &mut LocalHeap| {
                                local_heap.marking_barrier().deactivate_shared();
                            },
                        );
                    });
            }
        }
    }

    pub fn deactivate_young(heap: &mut Heap) {
        Self::deactivate_spaces(heap, MarkingMode::kMinorMarking);

        heap.safepoint().IterateLocalHeaps(|local_heap: &mut LocalHeap| {
            local_heap.marking_barrier().deactivate();
        });
    }

    pub fn deactivate(&mut self) {
        assert!(self.is_activated_);
        self.is_activated_ = false;
        self.is_compacting_ = false;
        self.marking_mode_ = MarkingMode::kNoMarking;
        assert!(self
            .typed_slots_map_
            .is_empty());
        assert!(self
            .current_worklists_
            .as_ref()
            .map_or(true, |wl| wl.IsEmpty()));
        self.current_worklists_.take();
    }

    pub fn deactivate_shared(&mut self) {
        assert!(self
            .shared_heap_worklists_
            .as_ref()
            .map_or(true, |wl| wl.IsEmpty()));
        self.shared_heap_worklists_.take();
    }

    pub fn publish_all(heap: &mut Heap) {
        heap.safepoint().IterateLocalHeaps(|local_heap: &mut LocalHeap| {
            local_heap.marking_barrier().publish_if_needed();
        });

        if unsafe {
            heap.isolate().is_shared_space_isolate()
        } {
            unsafe {
                heap.isolate()
                    .shared_space_isolate()
                    .global_safepoint()
                    .IterateClientIsolates(|client: &mut Isolate| {
                        client.heap().safepoint().IterateLocalHeaps(
                            |local_heap: &mut LocalHeap| {
                                local_heap
                                    .marking_barrier()
                                    .publish_shared_if_needed();
                            },
                        );
                    });
            }
        }
    }

    pub fn publish_young(heap: &mut Heap) {
        heap.safepoint().IterateLocalHeaps(|local_heap: &mut LocalHeap| {
            local_heap.marking_barrier().publish_if_needed();
        });
    }

    pub fn publish_if_needed(&mut self) {
        if self.is_activated_ {
            if let Some(worklists) = self.current_worklists_.as_mut() {
                worklists.Publish();
            }

            for (memory_chunk, typed_slots) in self.typed_slots_map_.drain() {
                let guard: MutexGuard<()> = unsafe {
                    (*memory_chunk).mutex().lock().unwrap()
                };
                RememberedSet::<OLD_TO_OLD>::MergeTyped(
                    memory_chunk,
                    typed_slots,
                );
            }
        }
    }

    pub fn publish_shared_if_needed(&mut self) {
        if let Some(worklists) = self.shared_heap_worklists_.as_mut() {
            worklists.Publish();
        }
    }

    fn is_current_marking_barrier(&self, verification_candidate: Tagged<HeapObject>) -> bool {
        WriteBarrier::CurrentMarkingBarrier(verification_candidate) as *const _ == self as *const _
    }

    fn isolate(&mut self) -> &mut Isolate {
        unsafe { &mut *((*self.heap_).isolate() as *mut Isolate) }
    }

    fn is_minor(&self) -> bool {
        self.marking_mode_ == MarkingMode::kMinorMarking
    }

    fn is_major(&self) -> bool {
        self.marking_mode_ == MarkingMode::kMajorMarking
    }

    fn mark_value(&mut self, _host: Tagged<HeapObject>, value: Tagged<HeapObject>) {
        self.mark_value_local(value);
    }
    fn mark_value_shared(&mut self, value: Tagged<HeapObject>) {}
    fn mark_value_local(&mut self, value: Tagged<HeapObject>) {}
    fn is_strong_descriptor_array(&mut self, _descriptor_array: Tagged<DescriptorArray>) -> bool {
        false
    }

    fn activate_spaces(heap: &mut Heap, marking_mode: MarkingMode) {
        Self::activate_space(heap.old_space(), marking_mode);
        Self::activate_space(heap.lo_space(), marking_mode);
        if heap.new_space() != std::ptr::null_mut() {
            assert!(!v8_flags.sticky_mark_bits());
            Self::activate_space(heap.new_space(), marking_mode);
        }
        Self::activate_space(heap.new_lo_space(), marking_mode);
        {
            let _scope = RwxMemoryWriteScope::new("For writing flags.");
            Self::activate_space(heap.code_space(), marking_mode);
            Self::activate_space(heap.code_lo_space(), marking_mode);
        }

        if marking_mode == MarkingMode::kMajorMarking {
            if heap.shared_space() != std::ptr::null_mut() {
                Self::activate_space(heap.shared_space(), marking_mode);
            }
            if heap.shared_lo_space() != std::ptr::null_mut() {
                Self::activate_space(heap.shared_lo_space(), marking_mode);
            }
        }

        Self::activate_space(heap.trusted_space(), marking_mode);
        Self::activate_space(heap.trusted_lo_space(), marking_mode);
    }

    fn deactivate_spaces(heap: &mut Heap, marking_mode: MarkingMode) {
        Self::deactivate_space(heap.old_space());
        Self::deactivate_space(heap.lo_space());
        if heap.new_space() != std::ptr::null_mut() {
            assert!(!v8_flags.sticky_mark_bits());
            Self::deactivate_space(heap.new_space());
        }
        Self::deactivate_space(heap.new_lo_space());
        {
            let _scope = RwxMemoryWriteScope::new("For writing flags.");
            Self::deactivate_space(heap.code_space());
            Self::deactivate_space(heap.code_lo_space());
        }

        if marking_mode == MarkingMode::kMajorMarking {
            if heap.shared_space() != std::ptr::null_mut() {
                Self::deactivate_space(heap.shared_space());
            }
            if heap.shared_lo_space() != std::ptr::null_mut() {
                Self::deactivate_space(heap.shared_lo_space());
            }
        }

        Self::deactivate_space(heap.trusted_space());
        Self::deactivate_space(heap.trusted_lo_space());
    }

    fn activate_space<SpaceType>(space: *mut SpaceType, marking_mode: MarkingMode) {
        unsafe {
            if space != std::ptr::null_mut() {
                (*space).SetGenerationPageFlags(marking_mode);
            }
        }
    }

    fn deactivate_space<SpaceType>(space: *mut SpaceType) {
        unsafe {
            if space != std::ptr::null_mut() {
                (*space).SetGenerationPageFlags(MarkingMode::kNoMarking);
            }
        }
    }
}

impl Drop for MarkingBarrier {
    fn drop(&mut self) {
        assert!(self.typed_slots_map_.is_empty());
    }
}

pub trait GenerationPageFlags {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode);
}

impl GenerationPageFlags for OldSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for SharedSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for TrustedSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for CodeSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for OldLargeObjectSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for SharedLargeObjectSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for TrustedLargeObjectSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for CodeLargeObjectSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for NewSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl GenerationPageFlags for NewLargeObjectSpace {
    fn SetGenerationPageFlags(&mut self, marking_mode: MarkingMode) {}
}

impl Heap {
    pub fn mark_compact_collector(&self) -> *mut MarkCompactCollector {
        null_mut()
    }
    pub fn minor_mark_sweep_collector(&self) -> *mut MinorMarkSweepCollector {
        null_mut()
    }
    pub fn incremental_marking(&self) -> *mut IncrementalMarking {
        null_mut()
    }
    pub fn safepoint(&self) -> &Safepoint {
        unsafe { std::mem::transmute(0usize) }
    }
    pub fn old_space(&mut self) -> *mut OldSpace {
        null_mut()
    }
    pub fn lo_space(&mut self) -> *mut OldLargeObjectSpace {
        null_mut()
    }
    pub fn new_space(&mut self) -> *mut NewSpace {
        null_mut()
    }
    pub fn new_lo_space(&mut self) -> *mut NewLargeObjectSpace {
        null_mut()
    }
    pub fn code_space(&mut self) -> *mut CodeSpace {
        null_mut()
    }
    pub fn code_lo_space(&mut self) -> *mut CodeLargeObjectSpace {
        null_mut()
    }
    pub fn shared_space(&mut self) -> *mut SharedSpace {
        null_mut()
    }
    pub fn shared_lo_space(&mut self) -> *mut SharedLargeObjectSpace {
        null_mut()
    }
    pub fn trusted_space(&mut self) -> *mut TrustedSpace {
        null_mut()
    }
    pub fn trusted_lo_space(&mut self) -> *mut TrustedLargeObjectSpace {
        null_mut()
    }
    pub fn isolate(&mut self) -> &mut Isolate {
        unsafe { &mut *(&mut self.isolate_ as *mut Isolate) }
    }
    pub fn SetIsMarkingFlag(&mut self, _is_marking: bool) {}
    pub fn incremental_marking(&mut self) -> &mut IncrementalMarking {
        unsafe { &mut *(&mut self.incremental_marking_ as *mut IncrementalMarking) }
    }
}

pub struct OLD_TO_OLD {}
pub trait Insert {
    fn Insert(&mut self, slot_type: i32, offset: usize);
}

impl Insert for TypedSlots {
    fn Insert(&mut self, slot_type: i32, offset: usize) {}
}
} // namespace marking_barrier
} // namespace heap
