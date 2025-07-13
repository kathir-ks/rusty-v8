// Converted from V8 C++ source files:
// Header: v8-platform.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{ptr, result};

use libc::c_char;

//use crate::v8::{Local, Name, Data, PropertyAttribute, Value};
pub struct SourceLocation {}
impl SourceLocation {
    pub fn Current() -> Self {
        SourceLocation {}
    }
}

pub struct Isolate;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TaskPriority {
    kBestEffort,
    kUserVisible,
    kUserBlocking,
}

pub trait Task {
    fn run(&mut self);
}

pub trait IdleTask {
    fn run(&mut self, deadline_in_seconds: f64);
}

pub trait TaskRunner {
    fn post_task(&self, task: Box<dyn Task>, location: &SourceLocation);
    fn post_non_nestable_task(&self, task: Box<dyn Task>, location: &SourceLocation);
    fn post_delayed_task(&self, task: Box<dyn Task>, delay_in_seconds: f64, location: &SourceLocation);
    fn post_non_nestable_delayed_task(&self, task: Box<dyn Task>, delay_in_seconds: f64, location: &SourceLocation);
    fn post_idle_task(&self, task: Box<dyn IdleTask>, location: &SourceLocation);
    fn idle_tasks_enabled(&self) -> bool;
    fn non_nestable_tasks_enabled(&self) -> bool;
    fn non_nestable_delayed_tasks_enabled(&self) -> bool;
}

pub trait JobDelegate {
    fn should_yield(&self) -> bool;
    fn notify_concurrency_increase(&self);
    fn get_task_id(&self) -> u8;
    fn is_joining_thread(&self) -> bool;
}

pub trait JobHandle {
    fn notify_concurrency_increase(&self);
    fn join(&self);
    fn cancel(&self);
    fn cancel_and_detach(&self);
    fn is_active(&self) -> bool;
    fn is_valid(&self) -> bool;
    fn update_priority_enabled(&self) -> bool;
    fn update_priority(&self, new_priority: TaskPriority);
}

pub trait JobTask {
    fn run(&mut self, delegate: &dyn JobDelegate);
    fn get_max_concurrency(&self, worker_count: usize) -> usize;
}

pub enum BlockingType {
    kMayBlock,
    kWillBlock,
}

pub trait ScopedBlockingCall {}

pub trait ConvertableToTraceFormat {
    fn append_as_trace_format(&self, out: &mut String) ;
}

pub trait TracingController {
    fn get_category_group_enabled(&self, name: &str) -> *const u8;
    fn add_trace_event(
        &self,
        phase: char,
        category_enabled_flag: *const u8,
        name: &str,
        scope: &str,
        id: u64,
        bind_id: u64,
        num_args: i32,
        arg_names: &[*const c_char],
        arg_types: *const u8,
        arg_values: *const u64,
        arg_convertables: &mut [*mut dyn ConvertableToTraceFormat],
        flags: u32,
    ) -> u64;
    fn add_trace_event_with_timestamp(
        &self,
        phase: char,
        category_enabled_flag: *const u8,
        name: &str,
        scope: &str,
        id: u64,
        bind_id: u64,
        num_args: i32,
        arg_names: &[*const c_char],
        arg_types: *const u8,
        arg_values: *const u64,
        arg_convertables: &mut [*mut dyn ConvertableToTraceFormat],
        flags: u32,
        timestamp: i64,
    ) -> u64;
    fn update_trace_event_duration(&self, category_enabled_flag: *const u8, name: &str, handle: u64);
    type TraceStateObserver: TraceStateObserver;
    fn add_trace_state_observer(&self, observer: &Self::TraceStateObserver);
    fn remove_trace_state_observer(&self, observer: &Self::TraceStateObserver);
}

pub trait TraceStateObserver {
    fn on_trace_enabled(&self);
    fn on_trace_disabled(&self);
}

pub trait PageAllocator {
    fn allocate_page_size(&self) -> usize;
    fn commit_page_size(&self) -> usize;
    fn set_random_mmap_seed(&self, seed: i64);
    fn get_random_mmap_addr(&self) -> *mut void;
    type Permission;
    fn allocate_pages(
        &self,
        address: *mut void,
        length: usize,
        alignment: usize,
        permissions: Self::Permission,
    ) -> *mut void;
    fn free_pages(&self, address: *mut void, length: usize) -> bool;
    fn release_pages(&self, address: *mut void, length: usize, new_length: usize) -> bool;
    fn set_permissions(&self, address: *mut void, length: usize, permissions: Self::Permission) -> bool;
    fn recommit_pages(&self, address: *mut void, length: usize, permissions: Self::Permission) -> bool;
    fn discard_system_pages(&self, address: *mut void, size: usize) -> bool;
    fn decommit_pages(&self, address: *mut void, size: usize) -> bool;
    fn seal_pages(&self, address: *mut void, length: usize) -> bool;
    type SharedMemoryMapping: SharedMemoryMapping;
    type SharedMemory: SharedMemory;
    fn reserve_for_shared_memory_mapping(&self, address: *mut void, size: usize) -> bool;
    fn allocate_shared_pages(&self, length: usize, original_address: *const void) -> Option<Self::SharedMemory>;
    fn can_allocate_shared_pages(&self) -> bool;
}

pub trait SharedMemoryMapping {
    fn get_memory(&self) -> *mut void;
}

pub trait SharedMemory {
    type SharedMemoryMapping: SharedMemoryMapping;
    fn remap_to(&self, new_address: *mut void) -> Option<Self::SharedMemoryMapping>;
    fn get_memory(&self) -> *mut void;
    fn get_size(&self) -> usize;
}

pub trait ThreadIsolatedAllocator {
    fn allocate(&self, size: usize) -> *mut void;
    fn free(&self, object: *mut void);
    type Type;
    fn get_type(&self) -> Self::Type;
    fn pkey(&self) -> i32;
    fn set_default_permissions_for_signal_handler();
}

pub type PlatformSharedMemoryHandle = isize;
pub const kInvalidSharedMemoryHandle: PlatformSharedMemoryHandle = -1;

#[cfg(target_os = "darwin")]
pub fn shared_memory_handle_from_mach_memory_entry(port: u32) -> PlatformSharedMemoryHandle {
    port as PlatformSharedMemoryHandle
}

#[cfg(target_os = "darwin")]
pub fn mach_memory_entry_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> u32 {
    handle as u32
}

#[cfg(target_os = "fuchsia")]
pub fn shared_memory_handle_from_vmo(handle: u32) -> PlatformSharedMemoryHandle {
    handle as PlatformSharedMemoryHandle
}

#[cfg(target_os = "fuchsia")]
pub fn vmo_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> u32 {
    handle as u32
}

#[cfg(target_os = "windows")]
pub fn shared_memory_handle_from_file_mapping(handle: *mut void) -> PlatformSharedMemoryHandle {
    handle as PlatformSharedMemoryHandle
}

#[cfg(target_os = "windows")]
pub fn file_mapping_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> *mut void {
    handle as *mut void
}

#[cfg(not(any(target_os = "darwin", target_os = "fuchsia", target_os = "windows")))]
pub fn shared_memory_handle_from_file_descriptor(fd: i32) -> PlatformSharedMemoryHandle {
    fd as PlatformSharedMemoryHandle
}

#[cfg(not(any(target_os = "darwin", target_os = "fuchsia", target_os = "windows")))]
pub fn file_descriptor_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> i32 {
    handle as i32
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PagePermissions {
    kNoAccess,
    kRead,
    kReadWrite,
    kReadWriteExecute,
    kReadExecute,
}

pub trait VirtualAddressSpace {
    type Address;
    fn page_size(&self) -> usize;
    fn allocation_granularity(&self) -> usize;
    fn base(&self) -> Self::Address;
    fn size(&self) -> usize;
    fn max_page_permissions(&self) -> PagePermissions;
    fn contains(&self, address: Self::Address) -> bool;
    fn set_random_seed(&mut self, seed: i64);
    fn random_page_address(&mut self) -> Self::Address;
    fn allocate_pages(
        &mut self,
        hint: Self::Address,
        size: usize,
        alignment: usize,
        permissions: PagePermissions,
    ) -> Result<Self::Address, String>;
    fn free_pages(&mut self, address: Self::Address, size: usize) -> Result<(), String>;
    fn set_page_permissions(
        &mut self,
        address: Self::Address,
        size: usize,
        permissions: PagePermissions,
    ) -> Result<bool, String>;
    fn allocate_guard_region(&mut self, address: Self::Address, size: usize) -> Result<bool, String>;
    fn free_guard_region(&mut self, address: Self::Address, size: usize) -> Result<(), String>;
    fn allocate_shared_pages(
        &mut self,
        hint: Self::Address,
        size: usize,
        permissions: PagePermissions,
        handle: PlatformSharedMemoryHandle,
        offset: u64,
    ) -> Result<Self::Address, String>;
    fn free_shared_pages(&mut self, address: Self::Address, size: usize) -> Result<(), String>;
    fn can_allocate_subspaces(&self) -> bool;
    type Subspace: VirtualAddressSpace;
    fn allocate_subspace(
        &mut self,
        hint: Self::Address,
        size: usize,
        alignment: usize,
        max_page_permissions: PagePermissions,
    ) -> Result<Self::Subspace, String>;
    fn recommit_pages(
        &mut self,
        address: Self::Address,
        size: usize,
        permissions: PagePermissions,
    ) -> Result<bool, String>;
    fn discard_system_pages(&mut self, address: Self::Address, size: usize) -> Result<bool, String>;
    fn decommit_pages(&mut self, address: Self::Address, size: usize) -> Result<bool, String>;
}

pub trait HighAllocationThroughputObserver {
    fn enter_section(&self) {}
    fn leave_section(&self) {}
}

pub trait Platform {
    type PageAllocator: PageAllocator;
    fn get_page_allocator(&self) -> Option<&dyn PageAllocator>;
    type ThreadIsolatedAllocator: ThreadIsolatedAllocator;
    fn get_thread_isolated_allocator(&self) -> Option<&dyn ThreadIsolatedAllocator>;
    fn on_critical_memory_pressure(&self);
    fn number_of_worker_threads(&self) -> i32;
    type TaskRunner: TaskRunner;
    fn get_foreground_task_runner(&self, isolate: *mut Isolate, priority: TaskPriority) -> Box<dyn TaskRunner>;
    fn post_task_on_worker_thread(
        &self,
        priority: TaskPriority,
        task: Box<dyn Task>,
        location: &SourceLocation,
    );
    fn post_delayed_task_on_worker_thread(
        &self,
        priority: TaskPriority,
        task: Box<dyn Task>,
        delay_in_seconds: f64,
        location: &SourceLocation,
    );
    fn idle_tasks_enabled(&self, isolate: *mut Isolate) -> bool;
    type JobHandle: JobHandle;
    type JobTask: JobTask;
    fn post_job(
        &self,
        priority: TaskPriority,
        job_task: Box<dyn JobTask>,
        location: &SourceLocation,
    ) -> Box<dyn JobHandle>;
    fn create_job(
        &self,
        priority: TaskPriority,
        job_task: Box<dyn JobTask>,
        location: &SourceLocation,
    ) -> Box<dyn JobHandle>;
    type ScopedBlockingCall: ScopedBlockingCall;
    fn create_blocking_scope(&self, blocking_type: BlockingType) -> Option<Self::ScopedBlockingCall>;
    fn monotonically_increasing_time(&self) -> f64;
    fn current_clock_time_milliseconds(&self) -> i64;
    fn current_clock_time_millis(&self) -> f64;
    fn current_clock_time_milliseconds_high_resolution(&self) -> f64;
    type StackTracePrinter;
    fn get_stack_trace_printer(&self) -> Option<Self::StackTracePrinter>;
    type TracingController: TracingController;
    fn get_tracing_controller(&self) -> &dyn TracingController;
    fn dump_without_crashing(&self);
    type HighAllocationThroughputObserver: HighAllocationThroughputObserver;
    fn get_high_allocation_throughput_observer(&self) -> &dyn HighAllocationThroughputObserver;
}

pub mod platform_support {
    use super::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub fn system_clock_time_millis() -> f64 {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_secs() as f64 * 1000.0 + since_the_epoch.subsec_nanos() as f64 / 1_000_000.0
    }
}
