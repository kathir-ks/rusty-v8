// src/heap/cppgc/gc_info.rs

mod gc_info_table;
use gc_info_table::GlobalGCInfoTable;

use std::sync::atomic::{AtomicUsize, Ordering};

/// Represents a GCInfo index.
pub type GCInfoIndex = usize;

/// Represents a trace callback function.
pub type TraceCallback = fn(*mut std::ffi::c_void);

/// Represents a finalization callback function.
pub type FinalizationCallback = fn(*mut std::ffi::c_void);

/// Represents a name callback function.
pub type NameCallback = fn(*const std::ffi::c_void, HeapObjectNameForUnnamedObject) -> HeapObjectName;

/// Enum for specifying how to retrieve the heap object name for unnamed objects.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum HeapObjectNameForUnnamedObject {
    kDoNotUseHiddenName,
    kUseHiddenName,
}

/// Represents a heap object name.
#[derive(Copy, Clone, Debug)]
pub struct HeapObjectName {
    name: NameProvider,
    use_hidden_name: bool,
}

/// Enum for name providers.
#[derive(Copy, Clone, Debug)]
pub enum NameProvider {
    kHiddenName,
}

/// Represents GC information.
#[derive(Clone)]
pub struct GCInfo {
    finalization_callback: Option<FinalizationCallback>,
    trace_callback: Option<TraceCallback>,
    name_callback: NameCallback,
}

impl GCInfo {
    fn new(
        finalization_callback: Option<FinalizationCallback>,
        trace_callback: Option<TraceCallback>,
        name_callback: NameCallback,
    ) -> Self {
        GCInfo {
            finalization_callback,
            trace_callback,
            name_callback,
        }
    }
}

mod internal {
    use super::*;

    fn get_hidden_name(
        _: *const std::ffi::c_void,
        name_retrieval_mode: HeapObjectNameForUnnamedObject,
    ) -> HeapObjectName {
        HeapObjectName {
            name: NameProvider::kHiddenName,
            use_hidden_name: name_retrieval_mode == HeapObjectNameForUnnamedObject::kUseHiddenName,
        }
    }

    pub struct EnsureGCInfoIndexTrait {}

    impl EnsureGCInfoIndexTrait {
        pub fn ensure_gc_info_index(
            registered_index: &AtomicUsize,
            trace_callback: Option<TraceCallback>,
            finalization_callback: Option<FinalizationCallback>,
            name_callback: NameCallback,
        ) -> GCInfoIndex {
            GlobalGCInfoTable::get_mutable().register_new_gc_info(
                registered_index,
                GCInfo::new(
                    finalization_callback,
                    trace_callback,
                    name_callback,
                ),
            )
        }

        pub fn ensure_gc_info_index_no_name(
            registered_index: &AtomicUsize,
            trace_callback: Option<TraceCallback>,
            finalization_callback: Option<FinalizationCallback>,
        ) -> GCInfoIndex {
            GlobalGCInfoTable::get_mutable().register_new_gc_info(
                registered_index,
                GCInfo::new(
                    finalization_callback,
                    trace_callback,
                    get_hidden_name,
                ),
            )
        }

        pub fn ensure_gc_info_index_no_finalization(
            registered_index: &AtomicUsize,
            trace_callback: Option<TraceCallback>,
            name_callback: NameCallback,
        ) -> GCInfoIndex {
            GlobalGCInfoTable::get_mutable().register_new_gc_info(
                registered_index,
                GCInfo::new(
                    None,
                    trace_callback,
                    name_callback,
                ),
            )
        }

        pub fn ensure_gc_info_index_trace_only(
            registered_index: &AtomicUsize,
            trace_callback: Option<TraceCallback>,
        ) -> GCInfoIndex {
            GlobalGCInfoTable::get_mutable().register_new_gc_info(
                registered_index,
                GCInfo::new(
                    None,
                    trace_callback,
                    get_hidden_name,
                ),
            )
        }
    }
}