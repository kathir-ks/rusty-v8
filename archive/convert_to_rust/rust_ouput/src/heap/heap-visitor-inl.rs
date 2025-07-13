// Converted from V8 C++ source files:
// Header: heap-visitor-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap_visitor_inl {
use std::marker::PhantomData;
use std::{
    cell::RefCell,
    ops::Range,
    ptr::null_mut,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};
use crate::heap::safepoint::*;
use crate::heap::stress_scavenge_observer::*;
use crate::heap::heap_verifier::*;
use crate::objects::objects::*;
use crate::objects::fixed_array_inl::*;
use crate::objects::prototype_info::*;
use crate::objects::js_array_buffer_inl::*;
use crate::heap::concurrent_marking::*;
use crate::objects::js_plural_rules::*;
use crate::heap::young_generation_marking_visitor_inl::*;
use crate::codegen::arm::assembler_arm_inl::*;
use crate::heap::live_object_range_inl::*;
use crate::torque::ls::message::*;
use crate::wasm::struct_types::*;
use crate::objects::source_text_module::*;
use crate::objects::property_details::*;
use crate::heap::incremental_marking::*;
use crate::objects::js_segments::*;
use crate::runtime::runtime_wasm::*;
use crate::heap::marking_worklist::*;
use crate::objects::string_set::*;
use crate::heap::list::*;
use crate::objects::managed::*;
use crate::objects::feedback_vector_inl::*;
use crate::heap::marking_inl::*;
use crate::interpreter::interpreter_assembler::*;
use crate::heap::evacuation_allocator::*;
use crate::objects::call_site_info_inl::*;
use crate::zone::zone::*;
use crate::heap::base_space::*;
use crate::heap::pretenuring_handler_inl::*;
use crate::heap::memory_chunk::*;
use crate::torque::earley_parser::*;
use crate::objects::oddball::*;
use crate::objects::shared_function_info::*;
use crate::objects::visitors::*;
use crate::objects::free_space_inl::*;
use crate::heap::heap_layout_inl::*;
use crate::heap::heap_visitor::*;
use crate::heap::mark_compact::*;
use crate::heap::object_lock_inl::*;
use crate::objects::arguments::*;
use crate::objects::data_handler_inl::*;
use crate::objects::js_weak_refs_inl::*;
use crate::objects::literal_objects_inl::*;
use crate::objects::map::*;
use crate::objects::module_inl::*;
use crate::objects::objects_body_descriptors_inl::*;
use crate::objects::objects_inl::*;
use crate::objects::ordered_hash_table::*;
use crate::objects::synthetic_module_inl::*;
use crate::objects::torque_defined_classes::*;
use crate::wasm::wasm_objects::*;
// Assuming these are defined elsewhere or are standard Rust types
pub struct Isolate {}
pub struct Heap {}
pub struct ObjectVisitorWithCageBases {}
pub struct LocalIsolate {}
pub struct String {}
pub struct JSPluralRules {
    dummy: i32,
}
impl JSPluralRules {
    pub enum Type {
        Ordinal,
        Cardinal,
    }
}
pub struct JSHeapBroker {}
pub struct MapRef {}
pub struct Handle<T> {}
pub struct DirectHandle<T> {}
pub struct IndirectHandle<T> {}
pub struct CpuFeatures {}
pub struct Condition {}
pub struct Register {}
pub struct Operand {}
pub struct GCType {}
pub struct MarkingBitmap {}
pub struct JSAPIObjectWithEmbedderSlots {}
impl JSAPIObjectWithEmbedderSlots {
    pub struct BodyDescriptor {}
}
pub struct ConsString {}
pub struct Flag {}
pub struct MaybeObject {}
pub struct ObjectTraits<T> {
    dummy: i32,
}
impl<T> ObjectTraits<T> {
    pub struct BodyDescriptor {
        dummy: i32,
    }
}
pub struct JSObject {}
impl JSObject {
    pub struct FastBodyDescriptor {}
    fn GetHeaderSize(_map: Tagged<Map>) -> i32 {
        16
    }
}
pub struct FreeSpace {}
pub struct StructBodyDescriptor {}
impl StructBodyDescriptor {
    fn IterateBody(_map: Tagged<Map>, _object: Tagged<HeapObject>, _size: i32, _visitor: &mut impl ObjectVisitorInterface) {}
}
pub struct JSArrayBuffer {}
pub struct MaybeObjectSize {
    size: Option<usize>,
}
impl MaybeObjectSize {
    fn IsNone(&self) -> bool {
        self.size.is_none()
    }
    fn AssumeSize(&self) -> usize {
        self.size.unwrap_or(0)
    }
}
impl From<usize> for MaybeObjectSize {
    fn from(size: usize) -> Self {
        MaybeObjectSize { size: Some(size) }
    }
}
impl From<Option<usize>> for MaybeObjectSize {
    fn from(size: Option<usize>) -> Self {
        MaybeObjectSize { size }
    }
}
impl MaybeObjectSize {
    fn new() -> Self {
        MaybeObjectSize { size: None }
    }
}
pub trait ObjectVisitorInterface {
    fn VisitMapPointer(&mut self, _host: Tagged<HeapObject>);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VisitorId {
    kVisitFixedArray,
    kVisitFixedDoubleArray,
    kVisitWeakFixedArray,
    kVisitHeapObject,
    kVisitJSObject,
    kVisitStruct,
    kVisitFiller,
    kVisitFreeSpace,
    kVisitConsString,
    kVisitJSArrayBuffer,
    kVisitShortcutCandidate,
    kVisitJSObjectFast,
    kVisitJSApiObject,
    kDataOnlyVisitorIdCount,
    kVisitorIdCount,
}

macro_rules! typed_visitor_id_list {
    ($callback:ident) => {
        $callback!(FixedArray);
        $callback!(FixedDoubleArray);
        $callback!(WeakFixedArray);
        $callback!(HeapObject);
        $callback!(JSArrayBuffer);
    };
}
macro_rules! typed_visitor_with_slack_id_list {
    ($callback:ident) => {
        $callback!(JSObject);
    };
}
macro_rules! torque_visitor_id_list {
    ($callback:ident) => {};
}
macro_rules! trusted_visitor_id_list {
    ($callback:ident) => {};
}
macro_rules! safe_string_transition_sources {
    ($callback:ident) => {};
}
macro_rules! unsafe_string_transition_sources {
    ($callback:ident) => {};
}
macro_rules! unsafe_string_transition_targets {
    ($callback:ident) => {};
}
pub(crate) use safe_string_transition_sources;
pub(crate) use typed_visitor_id_list;
pub(crate) use typed_visitor_with_slack_id_list;
pub(crate) use torque_visitor_id_list;
pub(crate) use trusted_visitor_id_list;
pub(crate) use unsafe_string_transition_sources;
pub(crate) use unsafe_string_transition_targets;

impl From<usize> for VisitorId {
    fn from(_: usize) -> Self {
        VisitorId::kVisitHeapObject
    }
}
impl VisitorId {
    pub fn as_usize(&self) -> usize {
        match self {
            VisitorId::kVisitFixedArray => 0,
            VisitorId::kVisitFixedDoubleArray => 1,
            VisitorId::kVisitWeakFixedArray => 2,
            VisitorId::kVisitHeapObject => 3,
            VisitorId::kVisitJSObject => 4,
            VisitorId::kVisitStruct => 5,
            VisitorId::kVisitFiller => 6,
            VisitorId::kVisitFreeSpace => 7,
            VisitorId::kVisitConsString => 8,
            VisitorId::kVisitJSArrayBuffer => 9,
            VisitorId::kVisitShortcutCandidate => 10,
            VisitorId::kVisitJSObjectFast => 11,
            VisitorId::kVisitJSApiObject => 12,
            VisitorId::kDataOnlyVisitorIdCount => 13,
            VisitorId::kVisitorIdCount => 14,
        }
    }
}

const _: () = {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    assert_send::<VisitorId>();
    assert_sync::<VisitorId>();
};

impl From<VisitorId> for usize {
    fn from(visitor_id: VisitorId) -> Self {
        visitor_id.as_usize()
    }
}

fn unreachable() -> ! {
    panic!("This should be unreachable");
}

fn dcheck(_condition: bool) {}

fn sbxcheck(_condition: bool) {}

fn fast_in_read_only_space_or_small_smi(_raw: Tagged_t) -> bool {
    false
}

pub struct HeapVisitor<ConcreteVisitor> {
    object_visitor_with_cage_bases: ObjectVisitorWithCageBases,
    heap_: *mut Heap,
    _phantom: PhantomData<ConcreteVisitor>,
}

impl<ConcreteVisitor> HeapVisitor<ConcreteVisitor> {
    fn new_from_local_isolate(isolate: &mut LocalIsolate) -> Self {
        let cage_base = PtrComprCageBase {};
        let code_cage_base = PtrComprCageBase {};
        let object_visitor_with_cage_bases = ObjectVisitorWithCageBases {};
        HeapVisitor {
            object_visitor_with_cage_bases,
            heap_: unsafe { isolate.heap().heap() },
            _phantom: PhantomData,
        }
    }

    fn new_from_isolate(isolate: &mut Isolate) -> Self {
        let object_visitor_with_cage_bases = ObjectVisitorWithCageBases {};
        HeapVisitor {
            object_visitor_with_cage_bases,
            heap_: unsafe { isolate.heap() },
            _phantom: PhantomData,
        }
    }

    fn new_from_heap(heap: &mut Heap) -> Self {
        let object_visitor_with_cage_bases = ObjectVisitorWithCageBases {};
        HeapVisitor {
            object_visitor_with_cage_bases,
            heap_: heap,
            _phantom: PhantomData,
        }
    }
}

impl<ConcreteVisitor> HeapVisitor<ConcreteVisitor> {
    fn cast<T>(_object: Tagged<HeapObject>, _heap: *mut Heap) -> Tagged<T> {
        Tagged {}
    }
}
impl<ConcreteVisitor: ObjectVisitorInterface> HeapVisitor<ConcreteVisitor> {
    fn visit(&mut self, object: Tagged<HeapObject>) -> usize {
        self.visit_internal(object.map(PtrComprCageBase {}), object, MaybeObjectSize::new())
    }
    fn visit_from_map(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>) -> usize {
        self.visit_internal(map, object, MaybeObjectSize::new())
    }
    fn visit_from_map_and_size(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>, object_size: usize) -> usize {
        self.visit_internal(map, object, MaybeObjectSize::from(object_size))
    }
    fn visit_internal(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>, maybe_object_size: MaybeObjectSize) -> usize {
        let visitor: &mut ConcreteVisitor = unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) };
        let visitor_id = map.visitor_id();
        match visitor_id {
            VisitorId::kVisitFixedArray => {
                dcheck(!InstanceTypeChecker::IsTrustedObject(map) || !HeapLayout::InTrustedSpace(object));
                let obj: Tagged<FixedArray> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_fixed_array_internal(map, obj, maybe_object_size)
            }
            VisitorId::kVisitFixedDoubleArray => {
                dcheck(!InstanceTypeChecker::IsTrustedObject(map) || !HeapLayout::InTrustedSpace(object));
                let obj: Tagged<FixedDoubleArray> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_fixed_double_array_internal(map, obj, maybe_object_size)
            }
            VisitorId::kVisitWeakFixedArray => {
                dcheck(!InstanceTypeChecker::IsTrustedObject(map) || !HeapLayout::InTrustedSpace(object));
                let obj: Tagged<WeakFixedArray> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_weak_fixed_array_internal(map, obj, maybe_object_size)
            }
            VisitorId::kVisitHeapObject => {
                dcheck(!InstanceTypeChecker::IsTrustedObject(map) || !HeapLayout::InTrustedSpace(object));
                let obj: Tagged<HeapObject> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_heap_object_internal(map, obj, maybe_object_size)
            }
            VisitorId::kVisitJSArrayBuffer => {
                dcheck(!InstanceTypeChecker::IsTrustedObject(map) || !HeapLayout::InTrustedSpace(object));
                let obj: Tagged<JSArrayBuffer> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_js_array_buffer_internal(map, obj, maybe_object_size)
            }
            VisitorId::kVisitJSObject => {
                dcheck(!InstanceTypeChecker::IsTrustedObject(map) || !HeapLayout::InTrustedSpace(object));
                let obj: Tagged<JSObject> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_js_object_internal(map, obj, maybe_object_size)
            }
            VisitorId::kVisitStruct => {
                let obj: Tagged<HeapObject> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_struct(map, object, maybe_object_size)
            }
            VisitorId::kVisitFiller => {
                let obj: Tagged<HeapObject> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_filler(map, object, maybe_object_size)
            }
            VisitorId::kVisitFreeSpace => {
                let obj: Tagged<FreeSpace> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_free_space(map, obj, maybe_object_size)
            }
            VisitorId::kVisitConsString => {
                dcheck(!InstanceTypeChecker::IsTrustedObject(map) || !HeapLayout::InTrustedSpace(object));
                let obj: Tagged<ConsString> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_cons_string_internal(map, obj, maybe_object_size)
            }
            VisitorId::kVisitShortcutCandidate => {
                let obj: Tagged<ConsString> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_shortcut_candidate(map, obj, maybe_object_size)
            }
            VisitorId::kVisitJSObjectFast => {
                let obj: Tagged<JSObject> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_js_object_fast(map, obj, maybe_object_size)
            }
            VisitorId::kVisitJSApiObject => {
                let obj: Tagged<JSObject> = HeapVisitor::<ConcreteVisitor>::cast(object, self.heap_);
                self.visit_js_api_object(map, obj, maybe_object_size)
            }
            _ => {
                unreachable();
            }
        }
    }

    fn visit_map_pointer_if_needed<const VISITOR_ID: VisitorId>(&mut self, host: Tagged<HeapObject>) {
        if !ConcreteVisitor::SHOULD_VISIT_MAP_POINTER {
            return;
        }
        if !ConcreteVisitor::SHOULD_VISIT_READ_ONLY_MAP_POINTER && contains_read_only_map::<VISITOR_ID>(PtrComprCageBase {}, host) {
            return;
        }
        let visitor: &mut ConcreteVisitor = unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) };
        visitor.VisitMapPointer(host);
    }

    fn visit_fixed_array_internal(&mut self, map: Tagged<Map>, object: Tagged<FixedArray>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<VisitorId::kVisitFixedArray, FixedArray, ObjectTraits<FixedArray>::BodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_fixed_double_array_internal(&mut self, map: Tagged<Map>, object: Tagged<FixedDoubleArray>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<VisitorId::kVisitFixedDoubleArray, FixedDoubleArray, ObjectTraits<FixedDoubleArray>::BodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_weak_fixed_array_internal(&mut self, map: Tagged<Map>, object: Tagged<WeakFixedArray>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<VisitorId::kVisitWeakFixedArray, WeakFixedArray, ObjectTraits<WeakFixedArray>::BodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_heap_object_internal(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<VisitorId::kVisitHeapObject, HeapObject, ObjectTraits<HeapObject>::BodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_cons_string_internal(&mut self, map: Tagged<Map>, object: Tagged<ConsString>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<VisitorId::kVisitConsString, ConsString, ObjectTraits<ConsString>::BodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_js_array_buffer_internal(&mut self, map: Tagged<Map>, object: Tagged<JSArrayBuffer>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_with_body_descriptor::<VisitorId::kVisitJSArrayBuffer, JSArrayBuffer, ObjectTraits<JSArrayBuffer>::BodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_js_object_internal(&mut self, map: Tagged<Map>, object: Tagged<JSObject>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_js_object_subclass::<JSObject, JSObject::FastBodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_with_body_descriptor<const VISITOR_ID: VisitorId, T, TBodyDescriptor>(
        &mut self,
        map: Tagged<Map>,
        object: Tagged<T>,
        maybe_object_size: MaybeObjectSize,
    ) -> usize {
        dcheck(!map.IsInobjectSlackTrackingInProgress());
        let visitor: &mut ConcreteVisitor = unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) };
        HeapVisitor::<ConcreteVisitor>::visit_map_pointer_if_needed::<VISITOR_ID>(self, object);
        let size = if ConcreteVisitor::USE_PRECOMPUTED_OBJECT_SIZE {
            maybe_object_size.AssumeSize() as i32
        } else {
            TBodyDescriptor::SizeOf(map, object) as i32
        };
        TBodyDescriptor::IterateBody(map, object, size as i32, visitor);
        size
    }
    fn visit_shortcut_candidate(&mut self, map: Tagged<Map>, object: Tagged<ConsString>, maybe_object_size: MaybeObjectSize) -> usize {
        let visitor: &mut ConcreteVisitor = unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) };
        self.visit_cons_string_internal(map, object, maybe_object_size)
    }
    fn visit_filler(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>, maybe_object_size: MaybeObjectSize) -> usize {
        let visitor: &mut ConcreteVisitor = unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) };
        if !ConcreteVisitor::CAN_ENCOUNTER_FILLER_OR_FREE_SPACE {
            unreachable();
        }
        visitor.VisitMapPointer(object);
        if ConcreteVisitor::USE_PRECOMPUTED_OBJECT_SIZE {
            maybe_object_size.AssumeSize()
        } else {
            map.instance_size()
        }
    }
    fn visit_free_space(&mut self, map: Tagged<Map>, object: Tagged<FreeSpace>, maybe_object_size: MaybeObjectSize) -> usize {
        let visitor: &mut ConcreteVisitor = unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) };
        if !ConcreteVisitor::CAN_ENCOUNTER_FILLER_OR_FREE_SPACE {
            unreachable();
        }
        visitor.VisitMapPointer(object);
        object.size(kRelaxedLoad)
    }
    fn visit_js_object_fast(&mut self, map: Tagged<Map>, object: Tagged<JSObject>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_js_object_subclass::<JSObject, JSObject::FastBodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_js_api_object(&mut self, map: Tagged<Map>, object: Tagged<JSObject>, maybe_object_size: MaybeObjectSize) -> usize {
        self.visit_js_object_subclass::<JSObject, JSAPIObjectWithEmbedderSlots::BodyDescriptor>(map, object, maybe_object_size)
    }
    fn visit_struct(&mut self, map: Tagged<Map>, object: Tagged<HeapObject>, maybe_object_size: MaybeObjectSize) -> usize {
        let visitor: &mut ConcreteVisitor = unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) };
        let size = if ConcreteVisitor::USE_PRECOMPUTED_OBJECT_SIZE {
            maybe_object_size.AssumeSize() as i32
        } else {
            map.instance_size()
        };
        visitor.VisitMapPointer(object);
        StructBodyDescriptor::IterateBody(map, object, size, visitor);
        size as usize
    }
    fn visit_js_object_subclass<T, TBodyDescriptor>(
        &mut self,
        map: Tagged<Map>,
        object: Tagged<T>,
        maybe_object_size: MaybeObjectSize,
    ) -> usize {
        let visitor: &mut ConcreteVisitor = unsafe { &mut *(self as *mut Self as *mut ConcreteVisitor) };
        visitor.VisitMapPointer(object);
        let size = if ConcreteVisitor::USE_PRECOMPUTED_OBJECT_SIZE {
            maybe_object_size.AssumeSize()
        } else {
            TBodyDescriptor::SizeOf(map, object)
        };
        let mut visitation_size = size as i32;
        TBodyDescriptor::IterateBody(map, object, visitation_size, visitor);
        size
    }
    fn get_object_filter_read_only_and_smi_fast<TSlot>(&self, slot: TSlot) -> Option<Tagged<Object>> {
        None
    }
}

pub struct ConcurrentHeapVisitor<ConcreteVisitor> {
    heap_visitor: HeapVisitor<ConcreteVisitor>,
}
impl<ConcreteVisitor> ConcurrentHeapVisitor<ConcreteVisitor> {
    fn new(isolate: &mut Isolate) -> Self {
        ConcurrentHeapVisitor {
            heap_visitor: HeapVisitor::new_from_isolate(isolate),
        }
    }
    fn cast<T>(_object: Tagged<HeapObject>, _heap: *mut Heap) -> Tagged<T> {
        Tagged {}
    }
    fn visit_string_locked<T>(&mut self, object: Tagged<T>) -> usize {
        0
    }
}

pub struct NewSpaceVisitor<ConcreteVisitor> {
    concurrent_heap_visitor: ConcurrentHeapVisitor<ConcreteVisitor>,
}
impl<ConcreteVisitor> NewSpaceVisitor<ConcreteVisitor> {
    fn new(isolate: &mut Isolate) -> Self {
        NewSpaceVisitor {
            concurrent_heap_visitor: ConcurrentHeapVisitor::new(isolate),
        }
    }
}

pub struct InstanceTypeChecker {}
impl InstanceTypeChecker {
    fn IsTrustedObject(_map: Tagged<Map>) -> bool {
        false
    }
}

pub struct Object {}
impl Object {
    pub fn map(&self) -> Tagged<Map> {
        Tagged {}
    }
}

pub struct kRelaxedLoad {}
impl FreeSpace {
    fn size(&self, _relaxed_load: kRelaxedLoad) -> usize {
        0
    }
}

pub struct Tagged_t {}

pub struct ObjectLockGuard {}
impl ObjectLockGuard {
    fn new(_object: Tagged<HeapObject>) -> Self {
        ObjectLockGuard {}
    }
}

fn contains_read_only_map<const VISITOR_ID: VisitorId>(_cage_base: PtrComprCageBase, _object: Tagged<HeapObject>) -> bool {
    false
}

macro_rules! define_read_only_map_specialization {
    ($visitor_id_type:ident) => {
        fn contains_read_only_map<const VISITOR_ID: VisitorId>(_cage_base: PtrComprCageBase, object: Tagged<HeapObject>) -> bool {
            if VISITOR_ID == VisitorId::kVisitHeapObject {
                dcheck(HeapLayout::InReadOnlySpace(object.map(PtrComprCageBase {})));
                return true;
            }
            false
        }
    };
}

trusted_visitor_id_list!(define_read_only_map_specialization);

pub const fn OutsideSandboxOrInReadonlySpace(_object: Tagged<HeapObject>) -> bool {
    true
}
}
