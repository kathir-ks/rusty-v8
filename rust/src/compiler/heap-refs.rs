#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(clippy::all)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};

// Placeholder for C++ API includes - replace with relevant Rust equivalents
// For now, define some dummy types
type Address = usize;
type Tagged<T> = *mut T;
type IndirectHandle<T> = *mut T;
type DirectHandle<T> = *mut T;
type Handle<T> = *mut T;
type Object = u64;
type HeapObject = u64;
type Smi = i32;
type Map = u64;
type PropertyCell = u64;
type JSReceiver = u64;
type JSObject = u64;
type JSTypedArray = u64;
type JSDataView = u64;
type JSPrimitiveWrapper = u64;
type JSBoundFunction = u64;
type JSFunction = u64;
type BigInt = u64;
type FixedArrayBase = u64;
type FixedArray = u64;
type ScriptContextTable = u64;
type JSArray = u64;
type JSGlobalObject = u64;
type JSGlobalProxy = u64;
type String = u64;
type ArrayBoilerplateDescription = u64;
type FixedDoubleArray = u64;
type BytecodeArray = u64;
type AllocationSite = u64;
type RegExpBoilerplateDescription = u64;
type FunctionTemplateInfo = u64;
type ScopeInfo = u64;
type SharedFunctionInfo = u64;
type FeedbackCell = u64;
type FeedbackVector = u64;
type Code = u64;
type SourceTextModule = u64;
type PrototypeInfo = u64;
type NativeContext = u64;
type TrustedByteArray = u64;
type PropertyArray = u64;
type MaybeObject = u64;

const MAP_TYPE: u32 = 1;
type Float64 = f64;
type ElementsKind = u32;
type PropertyDetails = u32;
type Builtin = u32;

const FAST_STRING_WRAPPER_ELEMENTS: ElementsKind = 1;
const SLOW_STRING_WRAPPER_ELEMENTS: ElementsKind = 2;
const RAB_GSAB_BIGUINT64_ELEMENTS: ElementsKind = 3;
const RAB_GSAB_BIGINT64_ELEMENTS: ElementsKind = 4;
const BIGINT64_ELEMENTS: ElementsKind = 5;
const BIGUINT64_ELEMENTS: ElementsKind = 6;
const PACKED_SMI_ELEMENTS: ElementsKind = 7;
const HOLEY_SMI_ELEMENTS: ElementsKind = 8;
const PACKED_DOUBLE_ELEMENTS: ElementsKind = 9;
const HOLEY_DOUBLE_ELEMENTS: ElementsKind = 10;
const PACKED_ELEMENTS: ElementsKind = 11;
const HOLEY_ELEMENTS: ElementsKind = 12;

const JS_ARRAY_TYPE: u32 = 1;
const ODDBALL_TYPE: u32 = 2;
const LAST_PRIMITIVE_HEAP_OBJECT_TYPE: u32 = 3;

const CONTEXT_SIDE_TABLE_PROPERTY_INDEX: i32 = 10;

const kSystemPointerSize: usize = 8;

macro_rules! CHECK {
    ($cond:expr) => {
        if !$cond {
            panic!("Check failed: {}", stringify!($cond));
        }
    };
}

macro_rules! CHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("Check failed: {} != {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK {
    ($cond:expr) => {
        if cfg!(debug_assertions) && !$cond {
            panic!("DCheck failed: {}", stringify!($cond));
        }
    };
}

macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) && $left != $right {
            panic!("DCheck failed: {} != {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! DCHECK_NE {
    ($left:expr, $right:expr) => {
        if cfg!(debug_assertions) && $left == $right {
            panic!("DCheck failed: {} == {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! CHECK_LT {
    ($left:expr, $right:expr) => {
        if $left >= $right {
            panic!("Check failed: {} < {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! CHECK_LE {
    ($left:expr, $right:expr) => {
        if $left > $right {
            panic!("Check failed: {} <= {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! CHECK_GT {
    ($left:expr, $right:expr) => {
        if $left <= $right {
            panic!("Check failed: {} > {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! SBXCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("SBXCheck failed: {} != {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("Unreachable code reached");
    };
}

macro_rules! TRACE_BROKER {
    ($broker:expr, $x:expr) => {
        if v8_flags::concurrent_recompilation {
            println!("TRACE: {}", $x);
        }
    };
}

macro_rules! TRACE_BROKER_MISSING {
    ($broker:expr, $x:expr) => {
        if v8_flags::concurrent_recompilation {
            println!("TRACE_MISSING: {}", $x);
        }
    };
}

macro_rules! HOLE_LIST {
  ($callback:ident) => {
    $callback!(TheHole, the_hole, TheHole)
    $callback!(PropertyCellHole, property_cell_hole, PropertyCellHole)
    $callback!(HashTableHole, hash_table_hole, HashTableHole)
  };
}

mod v8_flags {
    pub static concurrent_recompilation: bool = true; // Or false, based on your needs.
}

mod internal {
    pub mod compiler {
        use super::super::*;
        use std::any::Any;
        use std::fmt;
        use std::fmt::{Debug, Formatter};
        use std::ops::BitAnd;

        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        enum ObjectDataKind {
            Smi,
            BackgroundSerializedHeapObject,
            UnserializedHeapObject,
            NeverSerializedHeapObject,
            UnserializedReadOnlyHeapObject,
        }

        // Placeholder for ZoneObject - replace with appropriate Rust equivalent if needed
        trait ZoneObject {}

        // Placeholder for base::Flags - replace with appropriate Rust equivalent if needed
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        struct Flags<T: Copy + Clone + PartialEq + Eq + Debug>(u32, std::marker::PhantomData<T>);

        impl<T: Copy + Clone + PartialEq + Eq + Debug> Flags<T> {
            const EMPTY: Self = Flags(0, std::marker::PhantomData);

            fn new(value: u32) -> Self {
                Flags(value, std::marker::PhantomData)
            }

            fn contains(&self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            fn insert(&mut self, other: Self) {
                self.0 |= other.0;
            }

            fn remove(&mut self, other: Self) {
                self.0 &= !other.0;
            }

            fn is_empty(&self) -> bool {
                self.0 == 0
            }
        }

        impl<T: Copy + Clone + PartialEq + Eq + Debug> BitAnd for Flags<T> {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Flags(self.0 & rhs.0, std::marker::PhantomData)
            }
        }

        impl<T: Copy + Clone + PartialEq + Eq + Debug> std::ops::BitOr for Flags<T> {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Flags(self.0 | rhs.0, std::marker::PhantomData)
            }
        }

        impl<T: Copy + Clone + PartialEq + Eq + Debug> std::ops::BitAndAssign for Flags<T> {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl<T: Copy + Clone + PartialEq + Eq + Debug> std::ops::BitOrAssign for Flags<T> {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        struct AddressMatcher {}

        impl AddressMatcher {
            fn eq(&self, a: &Address, b: &Address) -> bool {
                a == b
            }
        }

        const kInitialRefsBucketCount: usize = 64;

        #[derive(Default)]
        struct TraceScope<'a> {
            broker: Option<&'a JSHeapBroker<'a>>,
            object_data: Option<*const ObjectData>,
            message: String,
        }

        impl<'a> TraceScope<'a> {
            fn new(broker: &'a JSHeapBroker<'a>, object_data: *const ObjectData, message: &str) -> Self {
                TraceScope {
                    broker: Some(broker),
                    object_data: Some(object_data),
                    message: message.to_string(),
                }
            }
        }

        impl<'a> Drop for TraceScope<'a> {
            fn drop(&mut self) {
                if let Some(broker) = self.broker {
                    if v8_flags::concurrent_recompilation {
                        println!("Exiting TraceScope: {} for {:?}", self.message, self.object_data);
                    }
                }
            }
        }

        struct RefsMap<'a> {
            map: Mutex<HashMap<Address, ObjectData>>,
            address_matcher: AddressMatcher,
            zone: &'a Zone,
        }

        impl<'a> RefsMap<'a> {
            fn new(bucket_count: usize, address_matcher: AddressMatcher, zone: &'a Zone) -> Self {
                RefsMap {
                    map: Mutex::new(HashMap::with_capacity(bucket_count)),
                    address_matcher,
                    zone,
                }
            }

            fn lookup(&self, address: Address) -> Option<&ObjectData> {
                let map = self.map.lock().unwrap();
                map.get(&address)
            }

            fn lookup_or_insert(&self, address: Address) -> &mut ObjectData {
                let mut map = self.map.lock().unwrap();
                map.entry(address).or_insert_with(|| {
                    // Placeholder data, will be overwritten in ObjectData constructor
                    ObjectData::new(
                        // Replace with a safe, empty JSHeapBroker if necessary
                        unsafe { &*(self.zone as *const Zone as *const JSHeapBroker) },
                        ptr::null_mut(), // Dummy storage pointer
                        address as IndirectHandle<Object>,
                        ObjectDataKind::UnserializedHeapObject,
                    )
                })
            }

            fn clear(&self) {
                let mut map = self.map.lock().unwrap();
                map.clear();
            }
        }

        impl<'a> fmt::Debug for RefsMap<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let map = self.map.lock().unwrap();
                f.debug_struct("RefsMap")
                    .field("map", &map)
                    .field("address_matcher", &self.address_matcher)
                    .field("zone", &self.zone)
                    .finish()
            }
        }

        #[derive(Debug)]
        struct Zone {
            // Add fields for tracking allocated memory, etc.
        }

        impl Zone {
            fn new() -> Self {
                Zone {}
            }

            fn new_object_data(&self,
                               broker: &JSHeapBroker,
                               storage: *mut *mut ObjectData,
                               object: IndirectHandle<Object>,
                               kind: ObjectDataKind) -> *mut ObjectData {
                let data = ObjectData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_property_cell_data(&self,
                                       broker: &JSHeapBroker,
                                       storage: *mut *mut ObjectData,
                                       object: IndirectHandle<PropertyCell>,
                                       kind: ObjectDataKind) -> *mut ObjectData {
                let data = PropertyCellData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_js_function_data(&self,
                                     broker: &JSHeapBroker,
                                     storage: *mut *mut ObjectData,
                                     object: IndirectHandle<JSFunction>,
                                     kind: ObjectDataKind) -> *mut ObjectData {
                let data = JSFunctionData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_bigint_data(&self,
                                 broker: &JSHeapBroker,
                                 storage: *mut *mut ObjectData,
                                 object: IndirectHandle<BigInt>,
                                 kind: ObjectDataKind) -> *mut ObjectData {
                let data = BigIntData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_map_data(&self,
                               broker: &JSHeapBroker,
                               storage: *mut *mut ObjectData,
                               object: IndirectHandle<Map>,
                               kind: ObjectDataKind) -> *mut ObjectData {
                let data = MapData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_fixed_array_data(&self,
                                      broker: &JSHeapBroker,
                                      storage: *mut *mut ObjectData,
                                      object: IndirectHandle<FixedArray>,
                                      kind: ObjectDataKind) -> *mut ObjectData {
                let data = FixedArrayData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_script_context_table_data(&self,
                                                broker: &JSHeapBroker,
                                                storage: *mut *mut ObjectData,
                                                object: IndirectHandle<ScriptContextTable>,
                                                kind: ObjectDataKind) -> *mut ObjectData {
                let data = ScriptContextTableData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_js_array_data(&self,
                                  broker: &JSHeapBroker,
                                  storage: *mut *mut ObjectData,
                                  object: IndirectHandle<JSArray>,
                                  kind: ObjectDataKind) -> *mut ObjectData {
                let data = JSArrayData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_js_global_object_data(&self,
                                           broker: &JSHeapBroker,
                                           storage: *mut *mut ObjectData,
                                           object: IndirectHandle<JSGlobalObject>,
                                           kind: ObjectDataKind) -> *mut ObjectData {
                let data = JSGlobalObjectData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_js_global_proxy_data(&self,
                                          broker: &JSHeapBroker,
                                          storage: *mut *mut ObjectData,
                                          object: IndirectHandle<JSGlobalProxy>,
                                          kind: ObjectDataKind) -> *mut ObjectData {
                let data = JSGlobalProxyData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_scope_info_data(&self,
                                     broker: &JSHeapBroker,
                                     storage: *mut *mut ObjectData,
                                     object: IndirectHandle<ScopeInfo>,
                                     kind: ObjectDataKind) -> *mut ObjectData {
                let data = ScopeInfoData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_shared_function_info_data(&self,
                                                broker: &JSHeapBroker,
                                                storage: *mut *mut ObjectData,
                                                object: IndirectHandle<SharedFunctionInfo>,
                                                kind: ObjectDataKind) -> *mut ObjectData {
                let data = SharedFunctionInfoData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_fixed_array_base_data(&self,
                                           broker: &JSHeapBroker,
                                           storage: *mut *mut ObjectData,
                                           object: IndirectHandle<FixedArrayBase>,
                                           kind: ObjectDataKind) -> *mut ObjectData {
                let data = FixedArrayBaseData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_feedback_cell_data(&self,
                                        broker: &JSHeapBroker,
                                        storage: *mut *mut ObjectData,
                                        object: IndirectHandle<FeedbackCell>,
                                        kind: ObjectDataKind) -> *mut ObjectData {
                let data = FeedbackCellData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_allocation_site_data(&self,
                                          broker: &JSHeapBroker,
                                          storage: *mut *mut ObjectData,
                                          object: IndirectHandle<AllocationSite>,
                                          kind: ObjectDataKind) -> *mut ObjectData {
                let data = AllocationSiteData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_feedback_vector_data(&self,
                                          broker: &JSHeapBroker,
                                          storage: *mut *mut ObjectData,
                                          object: IndirectHandle<FeedbackVector>,
                                          kind: ObjectDataKind) -> *mut ObjectData {
                let data = FeedbackVectorData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_descriptor_array_data(&self,
                                           broker: &JSHeapBroker,
                                           storage: *mut *mut ObjectData,
                                           object: IndirectHandle<DescriptorArray>,
                                           kind: ObjectDataKind) -> *mut ObjectData {
                let data = DescriptorArrayData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_code_data(&self,
                              broker: &JSHeapBroker,
                              storage: *mut *mut ObjectData,
                              object: IndirectHandle<Code>,
                              kind: ObjectDataKind) -> *mut ObjectData {
                let data = CodeData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_native_context_data(&self,
                                        broker: &JSHeapBroker,
                                        storage: *mut *mut ObjectData,
                                        object: IndirectHandle<NativeContext>,
                                        kind: ObjectDataKind) -> *mut ObjectData {
                let data = NativeContextData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_string_data(&self,
                                 broker: &JSHeapBroker,
                                 storage: *mut *mut ObjectData,
                                 object: IndirectHandle<String>,
                                 kind: ObjectDataKind) -> *mut ObjectData {
                let data = StringData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_function_template_info_data(&self,
                                                 broker: &JSHeapBroker,
                                                 storage: *mut *mut ObjectData,
                                                 object: IndirectHandle<FunctionTemplateInfo>,
                                                 kind: ObjectDataKind) -> *mut ObjectData {
                let data = FunctionTemplateInfoData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_js_data_view_data(&self,
                                      broker: &JSHeapBroker,
                                      storage: *mut *mut ObjectData,
                                      object: IndirectHandle<JSDataView>,
                                      kind: ObjectDataKind) -> *mut ObjectData {
                let data = JSDataViewData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_js_typed_array_data(&self,
                                        broker: &JSHeapBroker,
                                        storage: *mut *mut ObjectData,
                                        object: IndirectHandle<JSTypedArray>,
                                        kind: ObjectDataKind) -> *mut ObjectData {
                let data = JSTypedArrayData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_js_primitive_wrapper_data(&self,
                                                broker: &JSHeapBroker,
                                                storage: *mut *mut ObjectData,
                                                object: IndirectHandle<JSPrimitiveWrapper>,
                                                kind: ObjectDataKind) -> *mut ObjectData {
                let data = JSPrimitiveWrapperData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_js_bound_function_data(&self,
                                             broker: &JSHeapBroker,
                                             storage: *mut *mut ObjectData,
                                             object: IndirectHandle<JSBoundFunction>,
                                             kind: ObjectDataKind) -> *mut ObjectData {
                let data = JSBoundFunctionData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_array_boilerplate_description_data(&self,
                                                          broker: &JSHeapBroker,
                                                          storage: *mut *mut ObjectData,
                                                          object: IndirectHandle<ArrayBoilerplateDescription>,
                                                          kind: ObjectDataKind) -> *mut ObjectData {
                let data = ArrayBoilerplateDescriptionData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_reg_exp_boilerplate_description_data(&self,
                                                            broker: &JSHeapBroker,
                                                            storage: *mut *mut ObjectData,
                                                            object: IndirectHandle<RegExpBoilerplateDescription>,
                                                            kind: ObjectDataKind) -> *mut ObjectData {
                let data = RegExpBoilerplateDescriptionData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            fn new_source_text_module_data(&self,
                                              broker: &JSHeapBroker,
                                              storage: *mut *mut ObjectData,
                                              object: IndirectHandle<SourceTextModule>,
                                              kind: ObjectDataKind) -> *mut ObjectData {
                let data = SourceTextModuleData::new(broker, storage, object, kind);
                let boxed = Box::new(data);
                Box::into_raw(boxed)
            }

            // TODO: add the other constructors here
            fn alloc<T>(&self) -> *mut T {
                Box::into_raw(Box::new(unsafe { mem::zeroed() }))
            }

            fn new_vector<T>(&self, len: usize) -> Vec<T> {
                vec![unsafe { mem::zeroed() }; len]
            }

            fn new<T>(&self, value: T) -> Box<T> {
                Box::new(value)
            }

        }

        /// A trait representing an object that can be serialized by the JSHeapBroker.
        trait BrokerObject {
            /// The data type associated with this object for serialization.
            type DataType: ZoneObject;
            /// The reference type associated with this object for safe access.
            type RefType;
        }

        struct ObjectData {
            object_: IndirectHandle<Object>,
            kind_: ObjectDataKind,
            #[cfg(debug_assertions)]
            broker_: *const JSHeapBroker<'static>,
        }

        impl ObjectData {
            fn new(
                broker: &JSHeapBroker,
                storage: *mut *mut ObjectData,
                object: IndirectHandle<Object>,
                kind: ObjectDataKind,
            ) -> Self {
                unsafe {
                    *storage = Box::into_raw(Box::new(ObjectData {
                        object_: object,
                        kind_: kind,
                        #[cfg(debug_assertions)]
                        broker_: broker,
                    }));
                }

                if v8_flags::concurrent_recompilation {
                    println!(
                        "Creating data {:p} for handle {:p} ",
                        unsafe { *storage },
                        object
                    );
                }

                let isolate = broker.isolate;
                let isolate_option = unsafe { isolate.as_ref() };

                if let Some(isolate_ref) = isolate_option {
                    DCHECK!(broker.mode == JSHeapBrokerMode::Disabled ||
                            broker.mode == JSHeapBrokerMode::Serializing ||
                            isolate_ref.persistent_handles_scope_is_active() &&
                            broker.is_canonical_handle(object));
                }

                DCHECK!(broker.mode != JSHeapBrokerMode::Serialized ||
                        kind == ObjectDataKind::UnserializedReadOnlyHeapObject ||
                        kind == ObjectDataKind::Smi ||
                        kind == ObjectDataKind::NeverSerializedHeapObject ||
                        kind == ObjectDataKind::BackgroundSerializedHeapObject);
                DCHECK!(kind != ObjectDataKind::UnserializedReadOnlyHeapObject ||
                        unsafe { i::is_heap_object(object) } &&
                        ReadOnlyHeap::contains(object as HeapObject));

                unsafe {
                    (**storage)
                }
            }

            fn is_js_array(&self) -> bool {
                if self.should_access_heap() {
                    unsafe { i::is_js_array(self.object_) }
                } else if self.is_smi() {
                    false
                } else {
                    let instance_type = unsafe { (self as *const Self as *const HeapObjectData).read().get_map_instance_type() };
                    InstanceTypeChecker::is_js_array(instance_type)
                }
            }

            fn is_smi(&self) -> bool {
                self.kind_ == ObjectDataKind::Smi
            }

            fn should_access_heap(&self) -> bool {
                self.kind_ == ObjectDataKind::UnserializedHeapObject
                    || self.kind_ == ObjectDataKind::NeverSerializedHeapObject
                    || self.kind_ == ObjectDataKind::UnserializedReadOnlyHeapObject
            }

            fn is_null(&self) -> bool {
                unsafe { i::is_null(self.object_) }
            }

            fn object(&self) -> IndirectHandle<Object> {
                self.object_
            }

            fn kind(&self) -> ObjectDataKind {
                self.kind_
            }

            fn as_map(&self) -> &MapData {
                CHECK!(self.is_map());
                CHECK!(self.kind_ == ObjectDataKind::BackgroundSerializedHeapObject);
                unsafe { &*(self as *const Self as *const MapData) }
            }
        }

        impl fmt::Debug for ObjectData {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.debug_struct("ObjectData")
                    .field("object_", &self.object_)
                    .field("kind_", &self.kind_)
                    .finish()
            }
        }

        struct HeapObjectData {
            base: ObjectData,
            map_: *mut ObjectData,
        }

        impl HeapObjectData {
            fn new(
                broker: &JSHeapBroker,
                storage: *mut *mut ObjectData,
                object: IndirectHandle<HeapObject>,
                kind: ObjectDataKind,
            ) -> Self {
                let mut data = HeapObjectData {
                    base: ObjectData {
                        object_: object as IndirectHandle<Object>,
                        kind_: kind,
                        #[cfg(debug_assertions)]
                        broker_: broker,
                    },
                    map_: ptr::null_mut(), // Initialize to null
                };

                unsafe {
                    *storage = Box::into_raw(Box::new(data.base));
                    let base_ptr: *mut ObjectData = *storage;

                    let heap_data_ptr = base_ptr as *mut HeapObjectData;
                    let heap_data = &mut *heap_data_ptr;

                    heap_data.map_ = broker.get_or_create_data(
                        (*(object as *mut HeapObject)).map(broker.cage_base, AcquireLoadTag),
                        GetOrCreateDataFlags::AssumeMemoryFence,
                    );
                }

                // Now we write the correct HeapObjectData to the storage.
                unsafe {
                  let base_ptr: *mut ObjectData = *storage;
                  let mut heap_data = HeapObjectData {
                      base: (**storage), // Copy ObjectData
                      map_: broker.get_or_create_data(
                          (*(object as *mut HeapObject)).map(broker.cage_base, AcquireLoadTag),
                          GetOrCreateDataFlags::AssumeMemoryFence,
                      ),
                  };
                  ptr::write(base_ptr as *mut HeapObjectData, heap_data);
                }

                // The base pointer now points to the correct `HeapObjectData` object
                unsafe {
                    (**storage)
                }
            }

            fn try_get_boolean_value(
                &self,
                broker: &JSHeapBroker,
            ) -> Result<Option<bool>, String> {
                let result = self.try_get_boolean_value_impl(broker)?;
                if let Some(result_value) = result {
                    if let Some(isolate_ref) = unsafe { broker.isolate.as_ref() } {
                        let expected = unsafe { Object::boolean_value(self.base.object_, isolate_ref) };
                        DCHECK_EQ!(broker.is_main_thread() && result.is_some(), expected);
                    }
                }
                Ok(result)
            }

            fn try_get_boolean_value_impl(&self, broker: &JSHeapBroker) -> Result<Option<bool>, String> {
                // Keep in sync with Object::BooleanValue.
                let no_gc = DisallowGarbageCollection {};
                unsafe {
                    let o = self.base.object_;
                    let isolate = broker.isolate;
                    if i::is_true(o, isolate) {
                        return Ok(Some(true));
                    } else if i::is_false(o, isolate) {
                        return Ok(Some(false));
                    } else if i::is_null_or_undefined(o, isolate) {
                        return Ok(Some(false));
                    }
                    let map_ref = MapRef { data_: self.map_ };
                    if map_ref.is_undetectable() {
                        return Ok(Some(false));
                    }
                    let t = self.get_map_instance_type();
                    if InstanceTypeChecker::is_string(t) {
                        return Ok(None);
                    } else if InstanceTypeChecker::is_heap_number(t) {
                        return Ok(None);
                    } else if InstanceTypeChecker::is_big_int(t) {
                        return Ok(None);
                    }
                }

                Ok(Some(true))
            }

            fn map(&self) -> *mut ObjectData {
                self.map_
            }

            fn get_map_instance_type(&self) -> InstanceType {
                let map_data = unsafe { &*self.map_ };
                if map_data.base.should_access_heap() {
                    SBXCHECK_EQ!(map_data.base.kind(), ObjectDataKind::UnserializedReadOnlyHeapObject);
                    return unsafe { (*(map_data.base.object_ as *mut Map)).instance_type() };
                }
                if self as *const Self as *const ObjectData == self.map_ as *const ObjectData {
                    return MAP_TYPE;
                }
                unsafe {