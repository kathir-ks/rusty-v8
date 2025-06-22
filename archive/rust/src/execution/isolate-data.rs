// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: Many of the V8-specific types and functionalities are not directly
// translatable to Rust without the full V8 context.  This translation provides
// a structural equivalent where possible, but V8's internal workings are
// largely abstracted.

mod builtins;
mod codegen;
mod execution;
mod heap;
mod init;
mod roots;
mod sandbox;
mod utils;

#[cfg(test)]
mod gtest;

use std::{
    cell::Cell,
    marker::PhantomData,
    mem::{size_of, transmute},
    ptr::null_mut,
    sync::atomic::{AtomicU8, Ordering},
};

use self::{
    builtins::Builtin,
    codegen::{constants_arch::kSystemPointerSize, external_reference_table::ExternalReferenceTable},
    execution::{stack_guard::StackGuard, thread_local_top::ThreadLocalTop},
    heap::linear_allocation_area::LinearAllocationArea,
    init::isolate_group::IsolateGroup,
    roots::roots::RootsTable,
    sandbox::{
        code_pointer_table::CodePointerTable, cppheap_pointer_table::CppHeapPointerTable,
        external_pointer_table::ExternalPointerTable, trusted_pointer_table::TrustedPointerTable,
    },
};

// Placeholder for v8::internal::Isolate
pub struct Isolate {
    // Add fields as needed for compatibility.
}

// Placeholder for v8::internal::TrustedPointerPublishingScope
pub struct TrustedPointerPublishingScope {}

#[cfg(target_arch = "x86_64")]
pub const K_FAST_C_CALL_ALIGNMENT_PADDING_COUNT: usize = 5;
#[cfg(not(target_arch = "x86_64"))]
pub const K_FAST_C_CALL_ALIGNMENT_PADDING_COUNT: usize = 1;

macro_rules! isolate_data_fast_c_call_padding {
    ($V:ident) => {
        $V!(
            kFastCCallAlignmentPaddingOffset,
            K_FAST_C_CALL_ALIGNMENT_PADDING_COUNT * kSystemPointerSize,
            fast_c_call_alignment_padding
        );
    };
}

#[cfg(feature = "leap_tiering")]
mod leap_tiering {
    macro_rules! builtins_with_dispatch_adapter {
        ($V:ident, $CamelName:ident, $underscore_name:ident, $($rest:tt)*) => {
            $V!($CamelName, $CamelName##SharedFun);
        };
    }

    #[cfg(feature = "static_roots")]
    const V8_STATIC_DISPATCH_HANDLES_BOOL: bool = true;
    #[cfg(not(feature = "static_roots"))]
    const V8_STATIC_DISPATCH_HANDLES_BOOL: bool = false;

    macro_rules! builtins_with_dispatch_list {
        ($V:ident) => {
            builtins::builtins_with_sfi_list_generator!(builtins_with_dispatch_adapter, $V);
        };
    }

    pub struct JSBuiltinDispatchHandleRoot {}

    impl JSBuiltinDispatchHandleRoot {
        pub enum Idx {
            // TODO: Replace with real values when Builtins are available
            #[allow(dead_code)]
            Placeholder,
            Count,
            First,
        }
        pub const K_PADDING: usize =
            (Self::Idx::Count as usize * size_of::<JSDispatchHandle>()) % kSystemPointerSize / size_of::<JSDispatchHandle>();
        pub const K_TABLE_SIZE: usize = Self::Idx::Count as usize + Self::K_PADDING;

        #[inline]
        pub fn to_builtin(_idx: Self::Idx) -> Builtin {
            // Dummy implementation
            Builtin::kNoBuiltinId
        }

        #[inline]
        pub fn to_idx(_builtin: Builtin) -> Self::Idx {
            // Dummy implementation
            Self::Idx::Placeholder
        }

        #[inline]
        pub fn to_idx_root_index(_root_idx: roots::roots::RootIndex) -> Self::Idx {
            // Dummy implementation
            Self::Idx::Placeholder
        }
    }

    #[derive(Clone, Copy)]
    pub struct JSDispatchHandle {
        // Placeholder data
        data: usize,
    }
}

macro_rules! isolate_data_fields {
    ($V:ident) => {
        /* Misc. fields. */
        $V!(CageBase, kSystemPointerSize, cage_base);
        $V!(StackGuard, size_of::<StackGuard>(), stack_guard);
        $V!(IsMarkingFlag, size_of::<u8>(), is_marking_flag);
        $V!(IsMinorMarkingFlag, size_of::<u8>(), is_minor_marking_flag);
        $V!(IsSharedSpaceIsolateFlag, size_of::<u8>(), is_shared_space_isolate_flag);
        $V!(UsesSharedHeapFlag, size_of::<u8>(), uses_shared_heap_flag);
        $V!(ExecutionMode, size_of::<u8>(), execution_mode);
        $V!(StackIsIterable, size_of::<u8>(), stack_is_iterable);
        $V!(ErrorMessageParam, size_of::<u8>(), error_message_param);
        $V!(TablesAlignmentPadding, 1, tables_alignment_padding);
        $V!(RegExpStaticResultOffsetsVector, kSystemPointerSize, regexp_static_result_offsets_vector);
        /* Tier 0 tables (small but fast access). */
        $V!(
            BuiltinTier0EntryTable,
            builtins::Builtins::K_BUILTIN_TIER0_COUNT * kSystemPointerSize,
            builtin_tier0_entry_table
        );
        $V!(
            BuiltinsTier0Table,
            builtins::Builtins::K_BUILTIN_TIER0_COUNT * kSystemPointerSize,
            builtin_tier0_table
        );
        /* Misc. fields. */
        $V!(NewAllocationInfo, size_of::<LinearAllocationArea>(), new_allocation_info);
        $V!(OldAllocationInfo, size_of::<LinearAllocationArea>(), old_allocation_info);
        isolate_data_fast_c_call_padding!($V);
        $V!(FastCCallCallerFP, kSystemPointerSize, fast_c_call_caller_fp);
        $V!(FastCCallCallerPC, kSystemPointerSize, fast_c_call_caller_pc);
        $V!(FastApiCallTarget, kSystemPointerSize, fast_api_call_target);
        $V!(LongTaskStatsCounter, size_of::<usize>(), long_task_stats_counter);
        $V!(ThreadLocalTop, size_of::<ThreadLocalTop>(), thread_local_top);
        $V!(HandleScopeData, size_of::<HandleScopeData>(), handle_scope_data);
        $V!(
            EmbedderData,
            utils::utils::Internals::K_NUM_ISOLATE_DATA_SLOTS * kSystemPointerSize,
            embedder_data
        );
        isolate_data_fields_pointer_compression!($V);
        isolate_data_fields_sandbox!($V);
        $V!(ApiCallbackThunkArgument, kSystemPointerSize, api_callback_thunk_argument);
        $V!(RegexpExecVectorArgument, kSystemPointerSize, regexp_exec_vector_argument);
        $V!(
            ContinuationPreservedEmbedderData,
            kSystemPointerSize,
            continuation_preserved_embedder_data
        );
        /* Full tables (arbitrary size, potentially slower access). */
        $V!(
            RootsTable,
            roots::roots::RootsTable::K_ENTRIES_COUNT * kSystemPointerSize,
            roots_table
        );
        $V!(
            ExternalReferenceTable,
            size_of::<ExternalReferenceTable>(),
            external_reference_table
        );
        $V!(
            BuiltinEntryTable,
            builtins::Builtins::K_BUILTIN_COUNT * kSystemPointerSize,
            builtin_entry_table
        );
        $V!(
            BuiltinTable,
            builtins::Builtins::K_BUILTIN_COUNT * kSystemPointerSize,
            builtin_table
        );
        isolate_data_fields_leap_tiering!($V);
    };
}

macro_rules! isolate_data_fields_pointer_compression {
    ($V:ident) => {
        $V!(
            ExternalPointerTable,
            size_of::<ExternalPointerTable>(),
            external_pointer_table
        );
        $V!(SharedExternalPointerTable, kSystemPointerSize, shared_external_pointer_table);
        $V!(
            CppHeapPointerTable,
            size_of::<CppHeapPointerTable>(),
            cpp_heap_pointer_table
        );
    };
}

macro_rules! isolate_data_fields_sandbox {
    ($V:ident) => {
        $V!(TrustedCageBase, kSystemPointerSize, trusted_cage_base);
        $V!(
            TrustedPointerTable,
            size_of::<TrustedPointerTable>(),
            trusted_pointer_table
        );
        $V!(
            SharedTrustedPointerTable,
            kSystemPointerSize,
            shared_trusted_pointer_table
        );
        $V!(
            TrustedPointerPublishingScope,
            kSystemPointerSize,
            trusted_pointer_publishing_scope
        );
        $V!(
            CodePointerTableBaseAddress,
            kSystemPointerSize,
            code_pointer_table_base_address
        );
    };
}

macro_rules! isolate_data_fields_leap_tiering {
    ($V:ident) => {};
}

macro_rules! external_reference_list_isolate_fields {
    ($V:ident) => {
        $V!(isolate_address, "isolate address", IsolateAddress);
        $V!(jslimit_address, "jslimit address", JsLimitAddress);
    };
}

const K_NUM_ISOLATE_FIELD_IDS: u8 = 0;

macro_rules! plus_1 {
    ($($tt:tt)*) => {
        1
    };
}

const _: u8 = 0
    $(
        + external_reference_list_isolate_fields!(plus_1)
    )*
    $(
        + isolate_data_fields!(plus_1)
    )*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum IsolateFieldId {
    KUnknown = 0,
    #[allow(dead_code)]
    IsolateAddress,
    #[allow(dead_code)]
    JsLimitAddress,
    #[allow(dead_code)]
    CageBase,
    #[allow(dead_code)]
    StackGuard,
    #[allow(dead_code)]
    IsMarkingFlag,
    #[allow(dead_code)]
    IsMinorMarkingFlag,
    #[allow(dead_code)]
    IsSharedSpaceIsolateFlag,
    #[allow(dead_code)]
    UsesSharedHeapFlag,
    #[allow(dead_code)]
    ExecutionMode,
    #[allow(dead_code)]
    StackIsIterable,
    #[allow(dead_code)]
    ErrorMessageParam,
    #[allow(dead_code)]
    TablesAlignmentPadding,
    #[allow(dead_code)]
    RegExpStaticResultOffsetsVector,
    #[allow(dead_code)]
    BuiltinTier0EntryTable,
    #[allow(dead_code)]
    BuiltinsTier0Table,
    #[allow(dead_code)]
    NewAllocationInfo,
    #[allow(dead_code)]
    OldAllocationInfo,
    #[allow(dead_code)]
    FastCCallAlignmentPadding,
    #[allow(dead_code)]
    FastCCallCallerFP,
    #[allow(dead_code)]
    FastCCallCallerPC,
    #[allow(dead_code)]
    FastApiCallTarget,
    #[allow(dead_code)]
    LongTaskStatsCounter,
    #[allow(dead_code)]
    ThreadLocalTop,
    #[allow(dead_code)]
    HandleScopeData,
    #[allow(dead_code)]
    EmbedderData,
    #[allow(dead_code)]
    ExternalPointerTable,
    #[allow(dead_code)]
    SharedExternalPointerTable,
    #[allow(dead_code)]
    CppHeapPointerTable,
    #[allow(dead_code)]
    TrustedCageBase,
    #[allow(dead_code)]
    TrustedPointerTable,
    #[allow(dead_code)]
    SharedTrustedPointerTable,
    #[allow(dead_code)]
    TrustedPointerPublishingScope,
    #[allow(dead_code)]
    CodePointerTableBaseAddress,
    #[allow(dead_code)]
    ApiCallbackThunkArgument,
    #[allow(dead_code)]
    RegexpExecVectorArgument,
    #[allow(dead_code)]
    ContinuationPreservedEmbedderData,
    #[allow(dead_code)]
    RootsTable,
    #[allow(dead_code)]
    ExternalReferenceTable,
    #[allow(dead_code)]
    BuiltinEntryTable,
    #[allow(dead_code)]
    BuiltinTable,
}

// Placeholder type
pub type Address = *mut u8;
// Placeholder type
pub type Tagged<T> = *mut T;
// Placeholder type
pub type Object = u8;
// Placeholder type
pub type Smi = i32;

pub const K_ROOT_REGISTER_BIAS: isize = 0; // Replace with actual value

pub struct IsolateData {
    cage_base_: Address,
    stack_guard_: StackGuard,
    is_marking_flag_: Cell<bool>,
    is_minor_marking_flag_: Cell<bool>,
    is_shared_space_isolate_flag_: Cell<bool>,
    uses_shared_heap_flag_: Cell<bool>,
    execution_mode_: AtomicU8,
    stack_is_iterable_: Cell<u8>,
    error_message_param_: Cell<u8>,
    tables_alignment_padding_: [u8; 1],
    regexp_static_result_offsets_vector_: *mut i32,
    builtin_tier0_entry_table_: [*mut u8; builtins::Builtins::K_BUILTIN_TIER0_COUNT],
    builtin_tier0_table_: [*mut u8; builtins::Builtins::K_BUILTIN_TIER0_COUNT],
    new_allocation_info_: LinearAllocationArea,
    old_allocation_info_: LinearAllocationArea,
    fast_c_call_alignment_padding_: [*mut u8; K_FAST_C_CALL_ALIGNMENT_PADDING_COUNT],
    fast_c_call_caller_fp_: Address,
    fast_c_call_caller_pc_: Address,
    fast_api_call_target_: Address,
    long_task_stats_counter_: usize,
    thread_local_top_: ThreadLocalTop,
    handle_scope_data_: HandleScopeData,
    embedder_data_: [*mut std::ffi::c_void; utils::utils::Internals::K_NUM_ISOLATE_DATA_SLOTS],
    external_pointer_table_: ExternalPointerTable,
    shared_external_pointer_table_: *mut ExternalPointerTable,
    cpp_heap_pointer_table_: CppHeapPointerTable,
    trusted_cage_base_: Address,
    trusted_pointer_table_: TrustedPointerTable,
    shared_trusted_pointer_table_: *mut TrustedPointerTable,
    trusted_pointer_publishing_scope_: *mut TrustedPointerPublishingScope,
    code_pointer_table_base_address_: Address,
    api_callback_thunk_argument_: Address,
    regexp_exec_vector_argument_: Address,
    continuation_preserved_embedder_data_: Tagged<Object>,
    roots_table_: RootsTable,
    external_reference_table_: ExternalReferenceTable,
    builtin_entry_table_: [*mut u8; builtins::Builtins::K_BUILTIN_COUNT],
    builtin_table_: [*mut u8; builtins::Builtins::K_BUILTIN_COUNT],
    padding_: [char; 8], // Placeholder for actual padding calculation
}

impl IsolateData {
    pub fn new(isolate: *mut Isolate, group: *mut IsolateGroup) -> Self {
        // TODO: Initialize fields based on the isolate and group
        IsolateData {
            cage_base_: Address::default(),
            stack_guard_: StackGuard::new(unsafe { &mut *isolate }),
            is_marking_flag_: Cell::new(false),
            is_minor_marking_flag_: Cell::new(false),
            is_shared_space_isolate_flag_: Cell::new(false),
            uses_shared_heap_flag_: Cell::new(false),
            execution_mode_: AtomicU8::new(IsolateExecutionModeFlag::K_NO_FLAGS as u8),
            stack_is_iterable_: Cell::new(1),
            error_message_param_: Cell::new(0),
            tables_alignment_padding_: [0; 1],
            regexp_static_result_offsets_vector_: null_mut(),
            builtin_tier0_entry_table_: [*mut u8::default(); builtins::Builtins::K_BUILTIN_TIER0_COUNT],
            builtin_tier0_table_: [*mut u8::default(); builtins::Builtins::K_BUILTIN_TIER0_COUNT],
            new_allocation_info_: LinearAllocationArea::default(),
            old_allocation_info_: LinearAllocationArea::default(),
            fast_c_call_alignment_padding_: [*mut u8::default(); K_FAST_C_CALL_ALIGNMENT_PADDING_COUNT],
            fast_c_call_caller_fp_: Address::default(),
            fast_c_call_caller_pc_: Address::default(),
            fast_api_call_target_: Address::default(),
            long_task_stats_counter_: 0,
            thread_local_top_: ThreadLocalTop::default(),
            handle_scope_data_: HandleScopeData::default(),
            embedder_data_: [*mut std::ffi::c_void::default(); utils::utils::Internals::K_NUM_ISOLATE_DATA_SLOTS],
            external_pointer_table_: ExternalPointerTable::default(),
            shared_external_pointer_table_: null_mut(),
            cpp_heap_pointer_table_: CppHeapPointerTable::default(),
            trusted_cage_base_: Address::default(),
            trusted_pointer_table_: TrustedPointerTable::default(),
            shared_trusted_pointer_table_: null_mut(),
            trusted_pointer_publishing_scope_: null_mut(),
            code_pointer_table_base_address_: Address::default(),
            api_callback_thunk_argument_: Address::default(),
            regexp_exec_vector_argument_: Address::default(),
            continuation_preserved_embedder_data_: Smi::default() as *mut Object,
            roots_table_: RootsTable::default(),
            external_reference_table_: ExternalReferenceTable::default(),
            builtin_entry_table_: [*mut u8::default(); builtins::Builtins::K_BUILTIN_COUNT],
            builtin_table_: [*mut u8::default(); builtins::Builtins::K_BUILTIN_COUNT],
            padding_: ['\0'; 8],
        }
    }

    pub const K_ISOLATE_ROOT_BIAS: isize = K_ROOT_REGISTER_BIAS;

    pub fn isolate_root(&self) -> Address {
        (self as *const Self as usize as isize + Self::K_ISOLATE_ROOT_BIAS) as Address
    }

    macro_rules! define_offset_functions {
        ($($camel_name:ident, $size:expr, $hacker_name:ident),*) => {
            $(
                pub const fn $hacker_name##_offset() -> isize {
                    offset_of!(IsolateData, $hacker_name_) as isize - Self::K_ISOLATE_ROOT_BIAS
                }
            )*
        };
    }

    isolate_data_fields!(define_offset_functions);

    pub const fn root_slot_offset(root_index: roots::roots::RootIndex) -> isize {
        Self::roots_table_offset() as isize + RootsTable::offset_of(root_index) as isize
    }

    pub const fn builtin_entry_slot_offset(id: Builtin) -> isize {
        if builtins::Builtins::is_tier0(id) {
            Self::builtin_tier0_entry_table_offset() as isize
        } else {
            Self::builtin_entry_table_offset() as isize
        } + builtins::Builtins::to_int(id) as isize * kSystemPointerSize as isize
    }

    pub const fn builtin_slot_offset(builtin_index: i32) -> isize {
        Self::builtin_slot_offset_builtin(builtins::Builtins::from_int(builtin_index))
    }

    pub const fn builtin_slot_offset_builtin(id: Builtin) -> isize {
        if builtins::Builtins::is_tier0(id) {
            Self::builtin_tier0_table_offset() as isize
        } else {
            Self::builtin_table_offset() as isize
        } + builtins::Builtins::to_int(id) as isize * kSystemPointerSize as isize
    }

    pub const fn jslimit_offset() -> isize {
        Self::stack_guard_offset() as isize + StackGuard::jslimit_offset() as isize
    }

    pub const fn real_jslimit_offset() -> isize {
        Self::stack_guard_offset() as isize + StackGuard::real_jslimit_offset() as isize
    }

    macro_rules! define_address_functions {
        ($($camel_name:ident, $size:expr, $hacker_name:ident),*) => {
            $(
                pub fn $hacker_name##_address(&self) -> Address {
                    unsafe { transmute(&self.$hacker_name_) }
                }
            )*
        };
    }

    isolate_data_fields!(define_address_functions);

    pub fn fast_c_call_caller_fp(&self) -> Address {
        self.fast_c_call_caller_fp_
    }
    pub fn fast_c_call_caller_pc(&self) -> Address {
        self.fast_c_call_caller_pc_
    }
    pub fn fast_api_call_target(&self) -> Address {
        self.fast_api_call_target_
    }

    pub const fn exception_offset() -> isize {
        Self::thread_local_top_offset() as isize + ThreadLocalTop::exception_offset() as isize
    }

    pub fn cage_base(&self) -> Address {
        self.cage_base_
    }
    pub fn stack_guard(&mut self) -> &mut StackGuard {
        &mut self.stack_guard_
    }

    pub fn regexp_static_result_offsets_vector(&self) -> *mut i32 {
        self.regexp_static_result_offsets_vector_
    }
    pub fn set_regexp_static_result_offsets_vector(&mut self, value: *mut i32) {
        self.regexp_static_result_offsets_vector_ = value;
    }
    pub fn builtin_tier0_entry_table(&mut self) -> &mut [*mut u8; builtins::Builtins::K_BUILTIN_TIER0_COUNT] {
        &mut self.builtin_tier0_entry_table_
    }
    pub fn builtin_tier0_table(&mut self) -> &mut [*mut u8; builtins::Builtins::K_BUILTIN_TIER0_COUNT] {
        &mut self.builtin_tier0_table_
    }
    pub fn roots(&mut self) -> &mut RootsTable {
        &mut self.roots_table_
    }
    pub fn api_callback_thunk_argument(&self) -> Address {
        self.api_callback_thunk_argument_
    }
    pub fn regexp_exec_vector_argument(&self) -> Address {
        self.regexp_exec_vector_argument_
    }
    pub fn continuation_preserved_embedder_data(&self) -> Tagged<Object> {
        self.continuation_preserved_embedder_data_
    }
    pub fn set_continuation_preserved_embedder_data(&mut self, data: Tagged<Object>) {
        self.continuation_preserved_embedder_data_ = data;
    }
    pub fn roots_const(&self) -> &RootsTable {
        &self.roots_table_
    }
    pub fn external_reference_table(&mut self) -> &mut ExternalReferenceTable {
        &mut self.external_reference_table_
    }
    pub fn thread_local_top(&mut self) -> &mut ThreadLocalTop {
        &mut self.thread_local_top_
    }
    pub fn thread_local_top_const(&self) -> &ThreadLocalTop {
        &self.thread_local_top_
    }
    pub fn builtin_entry_table(&mut self) -> &mut [*mut u8; builtins::Builtins::K_BUILTIN_COUNT] {
        &mut self.builtin_entry_table_
    }
    pub fn builtin_table(&mut self) -> &mut [*mut u8; builtins::Builtins::K_BUILTIN_COUNT] {
        &mut self.builtin_table_
    }
    pub fn stack_is_iterable(&self) -> bool {
        self.stack_is_iterable_.get() != 0
    }

    pub fn is_marking(&self) -> bool {
        self.is_marking_flag_.get()
    }

    pub fn contains(&self, address: Address) -> bool {
        let start = self as *const Self as usize;
        let addr = address as usize;
        (addr.wrapping_sub(start)) < size_of::<Self>()
    }

    macro_rules! thread_local_top_member_offset {
        ($name:ident) => {
            pub const fn $name##_offset() -> u32 {
                Self::thread_local_top_offset() as u32 + offset_of!(ThreadLocalTop, $name##_) as u32
            }
        };
    }

    thread_local_top_member_offset!(topmost_script_having_context);
    thread_local_top_member_offset!(is_on_central_stack_flag);
    thread_local_top_member_offset!(context);

    pub const fn get_offset(id: IsolateFieldId) -> isize {
        match id {
            IsolateFieldId::KUnknown => panic!("UNREACHABLE"),
            IsolateFieldId::IsolateAddress => -Self::K_ISOLATE_ROOT_BIAS,
            IsolateFieldId::JsLimitAddress => Self::jslimit_offset(),
            IsolateFieldId::CageBase => Self::cage_base_offset(),
            IsolateFieldId::StackGuard => Self::stack_guard_offset(),
            IsolateFieldId::IsMarkingFlag => Self::is_marking_flag_offset(),
            IsolateFieldId::IsMinorMarkingFlag => Self::is_minor_marking_flag_offset(),
            IsolateFieldId::IsSharedSpaceIsolateFlag => Self::is_shared_space_isolate_flag_offset(),
            IsolateFieldId::UsesSharedHeapFlag => Self::uses_shared_heap_flag_offset(),
            IsolateFieldId::ExecutionMode => Self::execution_mode_offset(),
            IsolateFieldId::StackIsIterable => Self::stack_is_iterable_offset(),
            IsolateFieldId::ErrorMessageParam => Self::error_message_param_offset(),
            IsolateFieldId::TablesAlignmentPadding => Self::tables_alignment_padding_offset(),
            IsolateFieldId::RegExpStaticResultOffsetsVector => Self::regexp_static_result_offsets_vector_offset(),
            IsolateFieldId::BuiltinTier0EntryTable => Self::builtin_tier0_entry_table_offset(),
            IsolateFieldId::BuiltinsTier0Table => Self::builtin_tier0_table_offset(),
            IsolateFieldId::NewAllocationInfo => Self::new_allocation_info_offset(),
            IsolateFieldId::OldAllocationInfo => Self::old_allocation_info_offset(),
            IsolateFieldId::FastCCallAlignmentPadding => Self::fast_c_call_alignment_padding_offset(),
            IsolateFieldId::FastCCallCallerFP => Self::fast_c_call_caller_fp_offset(),
            IsolateFieldId::FastCCallCallerPC => Self::fast_c_call_caller_pc_offset(),
            IsolateFieldId::FastApiCallTarget => Self::fast_api_call_target_offset(),
            IsolateFieldId::LongTaskStatsCounter => Self::long_task_stats_counter_offset(),
            IsolateFieldId::ThreadLocalTop => Self::thread_local_top_offset(),
            IsolateFieldId::HandleScopeData => Self::handle_scope_data_offset(),
            IsolateFieldId::EmbedderData => Self::embedder_data_offset(),
            IsolateFieldId::ExternalPointerTable => Self::external_pointer_table_offset(),
            IsolateFieldId::SharedExternalPointerTable => Self::shared_external_pointer_table_offset(),
            IsolateFieldId::CppHeapPointerTable => Self::cpp_heap_pointer_table_offset(),
            IsolateFieldId::TrustedCageBase => Self::trusted_cage_base_offset(),
            IsolateFieldId::TrustedPointerTable => Self::trusted_pointer_table_offset(),
            IsolateFieldId::SharedTrustedPointerTable => Self::shared_trusted_pointer_table_offset(),
            IsolateFieldId::TrustedPointerPublishingScope => Self::trusted_pointer_publishing_scope_offset(),
            IsolateFieldId::CodePointerTableBaseAddress => Self::code_pointer_table_base_address_offset(),
            IsolateFieldId::ApiCallbackThunkArgument => Self::api_callback_thunk_argument_offset(),
            IsolateFieldId::RegexpExecVectorArgument => Self::regexp_exec_vector_argument_offset(),
            IsolateFieldId::ContinuationPreservedEmbedderData => Self::continuation_preserved_embedder_data_offset(),
            IsolateFieldId::RootsTable => Self::roots_table_offset(),
            IsolateFieldId::ExternalReferenceTable => Self::external_reference_table_offset(),
            IsolateFieldId::BuiltinEntryTable => Self::builtin_entry_table_offset(),
            IsolateFieldId::BuiltinTable => Self::builtin_table_offset(),
        }
    }

    pub fn assert_predictable_layout() {
        assert_eq!(size_of::<StackGuard>(), 8);
        assert_eq!(size_of::<RootsTable>(), 8);
        assert_eq!(size_of::<ThreadLocalTop>(), 8);
        assert_eq!(size_of::<ExternalReferenceTable>(), 8);
        assert_eq!(size_of::<IsolateData>(), 8);
        assert_eq!(size_of::<LinearAllocationArea>(), 8);

        macro_rules! assert_field_layout {
            ($($pure_name:ident, $size:expr, $name:ident),*) => {
                $(
                    assert_eq!(offset_of!(IsolateData, $name_), IsolateData::$pure_name##_offset() as usize);
                )*
            };
        }

        isolate_data_fields!(assert_field_layout);

        assert_eq!(size_of::<IsolateData>(), offset_of!(IsolateData, padding_) + 8);
    }
}

macro_rules! offset_of {
    ($struct:path, $field:tt) => {{
        let base = std::mem::MaybeUninit::<$struct>::uninit();
        let ptr = unsafe {
            let reference: &$struct = &*base.as_ptr();
            &reference.$field as *const _ as *const u8
        };
        let base_ptr = base.as_ptr() as *const u8;
        (ptr as usize).wrapping_sub(base_ptr as usize)
    }};
}

#[derive(Default, Copy, Clone)]
struct HandleScopeData {
    // Add fields as needed for compatibility.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IsolateExecutionModeFlag {
    K_NO_FLAGS = 0,
    K_PROFILING = 1 << 0,
    K_SHOULD_CHECK_SIDE_EFFECTS = 1 << 1,
}

#[derive(Default, Copy, Clone)]
struct Padding {}