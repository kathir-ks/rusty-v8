// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod v8config {
    // Placeholder for v8config.h content.  This file typically contains
    // configuration macros used during the V8 build process. Since we can't
    // directly translate these, we'll leave this module empty for now.
    //
    // In a real-world scenario, you'd replace this with Rust equivalents
    // based on your build system and how V8 is configured.
}

pub mod v8_source_location {
    // Placeholder for v8-source-location.h content.
    // In V8, source locations track the origin of code.  For now,
    // we'll stub this out.
    #[derive(Debug, Clone)]
    pub struct SourceLocation {}

    impl SourceLocation {
        pub const fn Current() -> Self {
            SourceLocation {}
        }
    }
}

pub mod v8 {
    use std::fmt::{self, Debug};
    use std::ptr::null_mut;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use crate::v8_source_location::SourceLocation;

    // Forward declare Isolate to avoid circular dependency issues.
    pub struct Isolate {}

    /// Valid priorities supported by the task scheduling infrastructure.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    pub enum TaskPriority {
        /// Best effort tasks are not critical for performance of the application. The
        /// platform implementation should preempt such tasks if higher priority tasks
        /// arrive.
        kBestEffort,
        /// User visible tasks are long running background tasks that will
        /// improve performance and memory usage of the application upon completion.
        /// Example: background compilation and garbage collection.
        kUserVisible,
        /// User blocking tasks are highest priority tasks that block the execution
        /// thread (e.g. major garbage collection). They must be finished as soon as
        /// possible.
        kUserBlocking,
        kMaxPriority, // = kUserBlocking
    }

    impl TaskPriority {
        pub const kMaxPriority: Self = Self::kUserBlocking;
    }

    /// A Task represents a unit of work.
    pub trait Task: Send + Sync {
        fn run(&mut self);
    }

    /// An IdleTask represents a unit of work to be performed in idle time.
    /// The Run method is invoked with an argument that specifies the deadline in
    /// seconds returned by MonotonicallyIncreasingTime().
    /// The idle task is expected to complete by this deadline.
    pub trait IdleTask: Send + Sync {
        fn run(&mut self, deadline_in_seconds: f64);
    }

    /// A TaskRunner allows scheduling of tasks. The TaskRunner may still be used to
    /// post tasks after the isolate gets destructed, but these tasks may not get
    /// executed anymore. All tasks posted to a given TaskRunner will be invoked in
    /// sequence. Tasks can be posted from any thread.
    pub trait TaskRunner: Send + Sync {
        /// Schedules a task to be invoked by this TaskRunner. The TaskRunner
        /// implementation takes ownership of |task|.
        fn post_task(&self, task: Box<dyn Task>, location: SourceLocation);

        /// Schedules a task to be invoked by this TaskRunner. The |task| cannot be nested
        /// within other task executions.
        fn post_non_nestable_task(&self, task: Box<dyn Task>, location: SourceLocation);

        /// Schedules a task to be invoked by this TaskRunner. The task is scheduled
        /// after the given number of seconds |delay_in_seconds|. The TaskRunner
        /// implementation takes ownership of |task|.
        fn post_delayed_task(&self, task: Box<dyn Task>, delay_in_seconds: f64, location: SourceLocation);

        /// Schedules a task to be invoked by this TaskRunner. The task is scheduled
        /// after the given number of seconds |delay_in_seconds|. The |task| cannot be nested
        /// within other task executions.
        fn post_non_nestable_delayed_task(&self, task: Box<dyn Task>, delay_in_seconds: f64, location: SourceLocation);

        /// Schedules an idle task to be invoked by this TaskRunner. The task is
        /// scheduled when the embedder is idle. Requires that
        /// |TaskRunner::IdleTasksEnabled()| is true. Idle tasks may be reordered
        /// relative to other task types and may be starved for an arbitrarily long
        /// time if no idle time is available. The TaskRunner implementation takes
        /// ownership of |task|.
        fn post_idle_task(&self, task: Box<dyn IdleTask>, location: SourceLocation);

        /// Returns true if idle tasks are enabled for this TaskRunner.
        fn idle_tasks_enabled(&self) -> bool;

        /// Returns true if non-nestable tasks are enabled for this TaskRunner.
        fn non_nestable_tasks_enabled(&self) -> bool;

        /// Returns true if non-nestable delayed tasks are enabled for this TaskRunner.
        fn non_nestable_delayed_tasks_enabled(&self) -> bool;
    }

    pub struct DefaultTaskRunner {
        idle_tasks_enabled: bool,
        non_nestable_tasks_enabled: bool,
        non_nestable_delayed_tasks_enabled: bool,
    }

    impl DefaultTaskRunner {
        pub fn new(idle_tasks_enabled: bool, non_nestable_tasks_enabled: bool, non_nestable_delayed_tasks_enabled: bool) -> Self {
            DefaultTaskRunner {
                idle_tasks_enabled,
                non_nestable_tasks_enabled,
                non_nestable_delayed_tasks_enabled,
            }
        }
    }

    impl TaskRunner for DefaultTaskRunner {
        fn post_task(&self, task: Box<dyn Task>, location: SourceLocation) {
            task.run();
        }

        fn post_non_nestable_task(&self, task: Box<dyn Task>, location: SourceLocation) {
            if self.non_nestable_tasks_enabled() {
                task.run();
            } else {
                eprintln!("Error: Non-nestable tasks are not enabled for this TaskRunner.");
            }
        }

        fn post_delayed_task(&self, mut task: Box<dyn Task>, delay_in_seconds: f64, location: SourceLocation) {
            std::thread::sleep(Duration::from_secs_f64(delay_in_seconds));
            task.run();
        }

        fn post_non_nestable_delayed_task(&self, mut task: Box<dyn Task>, delay_in_seconds: f64, location: SourceLocation) {
            if self.non_nestable_delayed_tasks_enabled() {
                std::thread::sleep(Duration::from_secs_f64(delay_in_seconds));
                task.run();
            } else {
                eprintln!("Error: Non-nestable delayed tasks are not enabled for this TaskRunner.");
            }
        }

        fn post_idle_task(&self, mut task: Box<dyn IdleTask>, location: SourceLocation) {
            if self.idle_tasks_enabled() {
                let now = SystemTime::now();
                let deadline = now.duration_since(UNIX_EPOCH).unwrap().as_secs_f64() + 1.0;
                task.run(deadline);
            } else {
                eprintln!("Error: Idle tasks are not enabled for this TaskRunner.");
            }
        }

        fn idle_tasks_enabled(&self) -> bool {
            self.idle_tasks_enabled
        }

        fn non_nestable_tasks_enabled(&self) -> bool {
            self.non_nestable_tasks_enabled
        }

        fn non_nestable_delayed_tasks_enabled(&self) -> bool {
            self.non_nestable_delayed_tasks_enabled
        }
    }

    /// Delegate that's passed to Job's worker task, providing an entry point to
    /// communicate with the scheduler.
    pub trait JobDelegate: Send + Sync {
        /// Returns true if this thread *must* return from the worker task on the
        /// current thread ASAP. Workers should periodically invoke ShouldYield (or
        /// YieldIfNeeded()) as often as is reasonable.
        /// After this method returned true, ShouldYield must not be called again.
        fn should_yield(&self) -> bool;

        /// Notifies the scheduler that max concurrency was increased, and the number
        /// of worker should be adjusted accordingly. See Platform::PostJob() for more
        /// details.
        fn notify_concurrency_increase(&self);

        /// Returns a task_id unique among threads currently running this job, such
        /// that GetTaskId() < worker count. To achieve this, the same task_id may be
        /// reused by a different thread after a worker_task returns.
        fn get_task_id(&self) -> u8;

        /// Returns true if the current task is called from the thread currently
        /// running JobHandle::Join().
        fn is_joining_thread(&self) -> bool;
    }

    /// Handle returned when posting a Job. Provides methods to control execution of
    /// the posted Job.
    pub trait JobHandle: Send + Sync {
        /// Notifies the scheduler that max concurrency was increased, and the number
        /// of worker should be adjusted accordingly. See Platform::PostJob() for more
        /// details.
        fn notify_concurrency_increase(&self);

        /// Contributes to the job on this thread. Doesn't return until all tasks have
        /// completed and max concurrency becomes 0. When Join() is called and max
        /// concurrency reaches 0, it should not increase again. This also promotes
        /// this Job's priority to be at least as high as the calling thread's
        /// priority.
        fn join(&self);

        /// Forces all existing workers to yield ASAP. Waits until they have all
        /// returned from the Job's callback before returning.
        fn cancel(&self);

        /*
         * Forces all existing workers to yield ASAP but doesn’t wait for them.
         * Warning, this is dangerous if the Job's callback is bound to or has access
         * to state which may be deleted after this call.
         */
        fn cancel_and_detach(&self);

        /// Returns true if there's any work pending or any worker running.
        fn is_active(&self) -> bool;

        /// Returns true if associated with a Job and other methods may be called.
        /// Returns false after Join() or Cancel() was called. This may return true
        /// even if no workers are running and IsCompleted() returns true
        fn is_valid(&self) -> bool;

        /// Returns true if job priority can be changed.
        fn update_priority_enabled(&self) -> bool;

        /// Update this Job's priority.
        fn update_priority(&self, new_priority: TaskPriority);
    }

    /// A JobTask represents work to run in parallel from Platform::PostJob().
    pub trait JobTask: Send + Sync {
        fn run(&self, delegate: &dyn JobDelegate);

        /// Controls the maximum number of threads calling Run() concurrently, given
        /// the number of threads currently assigned to this job and executing Run().
        /// Run() is only invoked if the number of threads previously running Run() was
        /// less than the value returned. In general, this should return the latest
        /// number of incomplete work items (smallest unit of work) left to process,
        /// including items that are currently in progress. |worker_count| is the
        /// number of threads currently assigned to this job which some callers may
        /// need to determine their return value. Since GetMaxConcurrency() is a leaf
        /// function, it must not call back any JobHandle methods.
        fn get_max_concurrency(&self, worker_count: usize) -> usize;
    }

    /// A "blocking call" refers to any call that causes the calling thread to wait
    /// off-CPU. It includes but is not limited to calls that wait on synchronous
    /// file I/O operations: read or write a file from disk, interact with a pipe or
    /// a socket, rename or delete a file, enumerate files in a directory, etc.
    /// Acquiring a low contention lock is not considered a blocking call.

    /// BlockingType indicates the likelihood that a blocking call will actually
    /// block.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BlockingType {
        // The call might block (e.g. file I/O that might hit in memory cache).
        kMayBlock,
        // The call will definitely block (e.g. cache already checked and now pinging
        // server synchronously).
        kWillBlock,
    }

    /// This class is instantiated with CreateBlockingScope() in every scope where a
    /// blocking call is made and serves as a precise annotation of the scope that
    /// may/will block. May be implemented by an embedder to adjust the thread count.
    /// CPU usage should be minimal within that scope. ScopedBlockingCalls can be
    /// nested.
    pub trait ScopedBlockingCall {}

    pub struct DefaultScopedBlockingCall {}

    impl ScopedBlockingCall for DefaultScopedBlockingCall {}

    /// The interface represents complex arguments to trace events.
    pub trait ConvertableToTraceFormat: Send + Sync {
        /// Append the class info to the provided |out| string. The appended
        /// data must be a valid JSON object. Strings must be properly quoted, and
        /// escaped. There is no processing applied to the content after it is
        /// appended.
        fn append_as_trace_format(&self, out: &mut String) -> fmt::Result;
    }

    /// V8 Tracing controller.
    ///
    /// Can be implemented by an embedder to record trace events from V8.
    ///
    /// Will become obsolete in Perfetto SDK build (v8_use_perfetto = true).
    pub trait TracingController: Send + Sync {
        // In Perfetto mode, trace events are written using Perfetto's Track Event
        // API directly without going through the embedder. However, it is still
        // possible to observe tracing being enabled and disabled.

        /// Called by TRACE_EVENT* macros, don't call this directly.
        /// The name parameter is a category group for example:
        /// TRACE_EVENT0("v8,parse", "V8.Parse")
        /// The pointer returned points to a value with zero or more of the bits
        /// defined in CategoryGroupEnabledFlags.
        fn get_category_group_enabled(&self, name: &str) -> *const u8;

        /// Adds a trace event to the platform tracing system. These function calls are
        /// usually the result of a TRACE_* macro from trace-event-no-perfetto.h when
        /// tracing and the category of the particular trace are enabled. It is not
        /// advisable to call these functions on their own; they are really only meant
        /// to be used by the trace macros. The returned handle can be used by
        /// UpdateTraceEventDuration to update the duration of COMPLETE events.
        fn add_trace_event(
            &self, phase: char, category_enabled_flag: *const u8, name: &str, scope: &str, id: u64, bind_id: u64, num_args: i32,
            arg_names: &[*const char], arg_types: &[u8], arg_values: &[u64], arg_convertables: &[*mut dyn ConvertableToTraceFormat],
            flags: u32,
        ) -> u64;
        fn add_trace_event_with_timestamp(
            &self, phase: char, category_enabled_flag: *const u8, name: &str, scope: &str, id: u64, bind_id: u64, num_args: i32,
            arg_names: &[*const char], arg_types: &[u8], arg_values: &[u64], arg_convertables: &[*mut dyn ConvertableToTraceFormat],
            flags: u32, timestamp: i64,
        ) -> u64;

        /// Sets the duration field of a COMPLETE trace event. It must be called with
        /// the handle returned from AddTraceEvent().
        fn update_trace_event_duration(&self, category_enabled_flag: *const u8, name: &str, handle: u64);

        trait TraceStateObserver: Send + Sync {
            fn on_trace_enabled(&mut self);
            fn on_trace_disabled(&mut self);
        }

        /// Adds tracing state change observer.
        /// Does nothing in Perfetto SDK build (v8_use_perfetto = true).
        fn add_trace_state_observer(&mut self, observer: *mut dyn TraceStateObserver);

        /// Removes tracing state change observer.
        /// Does nothing in Perfetto SDK build (v8_use_perfetto = true).
        fn remove_trace_state_observer(&mut self, observer: *mut dyn TraceStateObserver);
    }

    // A V8 memory page allocator.
    //
    // Can be implemented by an embedder to manage large host OS allocations.
    pub trait PageAllocator: Send + Sync {
        // Gets the page granularity for AllocatePages and FreePages. Addresses and
        // lengths for those calls should be multiples of AllocatePageSize().
        fn allocate_page_size(&self) -> usize;

        // Gets the page granularity for SetPermissions and ReleasePages. Addresses
        // and lengths for those calls should be multiples of CommitPageSize().
        fn commit_page_size(&self) -> usize;

        // Sets the random seed so that GetRandomMmapAddr() will generate repeatable
        // sequences of random mmap addresses.
        fn set_random_mmap_seed(&mut self, seed: i64);

        // Returns a randomized address, suitable for memory allocation under ASLR.
        // The address will be aligned to AllocatePageSize.
        fn get_random_mmap_addr(&mut self) -> *mut std::ffi::c_void;

        // Memory permissions.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Permission {
            kNoAccess,
            kRead,
            kReadWrite,
            kReadWriteExecute,
            kReadExecute,
            // Set this when reserving memory that will later require kReadWriteExecute
            // permissions. The resulting behavior is platform-specific, currently
            // this is used to set the MAP_JIT flag on Apple Silicon.
            // TODO(jkummerow): Remove this when Wasm has a platform-independent
            // w^x implementation.
            // TODO(saelo): Remove this once all JIT pages are allocated through the
            // VirtualAddressSpace API.
            kNoAccessWillJitLater,
        }

        // Allocates memory in range with the given alignment and permission.
        fn allocate_pages(
            &mut self, address: *mut std::ffi::c_void, length: usize, alignment: usize, permissions: Permission,
        ) -> *mut std::ffi::c_void;

        // Frees memory in a range that was allocated by a call to AllocatePages.
        fn free_pages(&mut self, address: *mut std::ffi::c_void, length: usize) -> bool;

        // Releases memory in a range that was allocated by a call to AllocatePages.
        fn release_pages(&mut self, address: *mut std::ffi::c_void, length: usize, new_length: usize) -> bool;

        // Sets permissions on pages in an allocated range.
        fn set_permissions(&mut self, address: *mut std::ffi::c_void, length: usize, permissions: Permission) -> bool;

        // Recommits discarded pages in the given range with given permissions.
        // Discarded pages must be recommitted with their original permissions
        // before they are used again.
        fn recommit_pages(&mut self, address: *mut std::ffi::c_void, length: usize, permissions: Permission) -> bool;

        // Frees memory in the given [address, address + size) range. address and size
        // should be operating system page-aligned. The next write to this
        // memory area brings the memory transparently back. This should be treated as
        // a hint to the OS that the pages are no longer needed. It does not guarantee
        // that the pages will be discarded immediately or at all.
        fn discard_system_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;

        // Decommits any wired memory pages in the given range, allowing the OS to
        // reclaim them, and marks the region as inacessible (kNoAccess). The address
        // range stays reserved and can be accessed again later by changing its
        // permissions. However, in that case the memory content is guaranteed to be
        // zero-initialized again. The memory must have been previously allocated by a
        // call to AllocatePages. Returns true on success, false otherwise.
        fn decommit_pages(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;

        // Block any modifications to the given mapping such as changing permissions
        // or unmapping the pages on supported platforms.
        // The address space reservation will exist until the process ends, but it's
        // possible to release the memory using DiscardSystemPages. Note that this
        // might require write permissions to the page as e.g. on Linux, mseal will
        // block discarding sealed anonymous memory.
        fn seal_pages(&mut self, address: *mut std::ffi::c_void, length: usize) -> bool;

        // INTERNAL ONLY: This interface has not been stabilised and may change
        // without notice from one release to another without being deprecated first.
        trait SharedMemoryMapping {
            // Implementations are expected to free the shared memory mapping in the
            // destructor.
            fn get_memory(&self) -> *mut std::ffi::c_void;
        }

        // INTERNAL ONLY: This interface has not been stabilised and may change
        // without notice from one release to another without being deprecated first.
        trait SharedMemory {
            fn remap_to(&self, new_address: *mut std::ffi::c_void) -> Option<Box<dyn SharedMemoryMapping>>;
            fn get_memory(&self) -> *mut std::ffi::c_void;
            fn get_size(&self) -> usize;
        }

        // INTERNAL ONLY: This interface has not been stabilised and may change
        // without notice from one release to another without being deprecated first.
        //
        // Reserve pages at a fixed address returning whether the reservation is
        // possible. The reserved memory is detached from the PageAllocator and so
        // should not be freed by it. It's intended for use with
        // SharedMemory::RemapTo, where ~SharedMemoryMapping would free the memory.
        fn reserve_for_shared_memory_mapping(&mut self, address: *mut std::ffi::c_void, size: usize) -> bool;

        // INTERNAL ONLY: This interface has not been stabilised and may change
        // without notice from one release to another without being deprecated first.
        //
        // Allocates shared memory pages. Not all PageAllocators need support this and
        // so this method need not be overridden.
        // Allocates a new read-only shared memory region of size |length| and copies
        // the memory at |original_address| into it.
        fn allocate_shared_pages(&mut self, length: usize, original_address: *const std::ffi::c_void) -> Option<Box<dyn SharedMemory>>;

        // INTERNAL ONLY: This interface has not been stabilised and may change
        // without notice from one release to another without being deprecated first.
        //
        // If not overridden and changed to return true, V8 will not attempt to call
        // AllocateSharedPages or RemapSharedPages. If overridden, AllocateSharedPages
        // and RemapSharedPages must also be overridden.
        fn can_allocate_shared_pages(&self) -> bool;
    }

    /// An allocator that uses per-thread permissions to protect the memory.
    ///
    /// The implementation is platform/hardware specific, e.g. using pkeys on x64.
    ///
    /// INTERNAL ONLY: This interface has not been stabilised and may change
    /// without notice from one release to another without being deprecated first.
    pub trait ThreadIsolatedAllocator: Send + Sync {
        fn allocate(&mut self, size: usize) -> *mut std::ffi::c_void;

        fn free(&mut self, object: *mut std::ffi::c_void);

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Type {
            kPkey,
        }

        fn get_type(&self) -> Type;

        // Return the pkey used to implement the thread isolation if Type == kPkey.
        fn pkey(&self) -> i32;

        // Per-thread permissions can be reset on signal handler entry. Even reading
        // ThreadIsolated memory will segfault in that case.
        // Call this function on signal handler entry to ensure that read permissions
        // are restored.
        fn set_default_permissions_for_signal_handler();
    }

    // Opaque type representing a handle to a shared memory region.
    pub type PlatformSharedMemoryHandle = isize;
    pub const kInvalidSharedMemoryHandle: PlatformSharedMemoryHandle = -1;

    // Conversion routines from the platform-dependent shared memory identifiers
    // into the opaque PlatformSharedMemoryHandle type. These use the underlying
    // types (e.g. unsigned int) instead of the typedef'd ones (e.g. mach_port_t)
    // to avoid pulling in large OS header files into this header file. Instead,
    // the users of these routines are expected to include the respecitve OS
    // headers in addition to this one.
    //
    // Note: These functions are platform-specific and require conditional compilation.
    // In a real-world scenario, these implementations should be defined in separate
    // platform-specific modules.

    #[cfg(target_os = "macos")]
    pub mod darwin {
        use super::PlatformSharedMemoryHandle;
        // Convert between a shared memory handle and a mach_port_t referencing a memory
        // entry object.
        pub fn shared_memory_handle_from_mach_memory_entry(port: u32) -> PlatformSharedMemoryHandle {
            port as PlatformSharedMemoryHandle
        }
        pub fn mach_memory_entry_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> u32 {
            handle as u32
        }
    }

    #[cfg(target_os = "fuchsia")]
    pub mod fuchsia {
        use super::PlatformSharedMemoryHandle;
        // Convert between a shared memory handle and a zx_handle_t to a VMO.
        pub fn shared_memory_handle_from_vmo(handle: u32) -> PlatformSharedMemoryHandle {
            handle as PlatformSharedMemoryHandle
        }
        pub fn vmo_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> u32 {
            handle as u32
        }
    }

    #[cfg(target_os = "windows")]
    pub mod windows {
        use super::PlatformSharedMemoryHandle;
        // Convert between a shared memory handle and a Windows HANDLE to a file mapping
        // object.
        pub fn shared_memory_handle_from_file_mapping(handle: *mut std::ffi::c_void) -> PlatformSharedMemoryHandle {
            handle as PlatformSharedMemoryHandle
        }
        pub fn file_mapping_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> *mut std::ffi::c_void {
            handle as *mut std::ffi::c_void
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "fuchsia", target_os = "windows")))]
    pub mod posix {
        use super::PlatformSharedMemoryHandle;
        // Convert between a shared memory handle and a file descriptor.
        pub fn shared_memory_handle_from_file_descriptor(fd: i32) -> PlatformSharedMemoryHandle {
            fd as PlatformSharedMemoryHandle
        }
        pub fn file_descriptor_from_shared_memory_handle(handle: PlatformSharedMemoryHandle) -> i32 {
            handle as i32
        }
    }

    /// Possible permissions for memory pages.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PagePermissions {
        kNoAccess,
        kRead,
        kReadWrite,
        kReadWriteExecute,
        kReadExecute,
    }

    /// Class to manage a virtual memory address space.
    ///
    /// This class represents a contiguous region of virtual address space in which
    /// sub-spaces and (private or shared) memory pages can be allocated, freed, and
    /// modified. This interface is meant to eventually replace the PageAllocator
    /// interface, and can be used as an alternative in the meantime.
    ///
    /// This API is not yet stable and may change without notice!
    pub trait VirtualAddressSpace: Send + Sync {
        type Address = usize;

        /// The page size used inside this space. Guaranteed to be a power of two.
        /// Used as granularity for all page-related operations except for allocation,
        /// which use the allocation_granularity(), see below.
        ///
        /// \returns the page size in bytes.
        fn page_size(&self) -> usize;

        /// The granularity of page allocations and, by extension, of subspace
        /// allocations. This is guaranteed to be a power of two and a multiple of the
        /// page_size(). In practice, this is equal to the page size on most OSes, but
        /// on Windows it is usually 64KB, while the page size is 4KB.
        ///
        /// \returns the allocation granularity in bytes.
        fn allocation_granularity(&self) -> usize;

        /// The base address of the address space managed by this instance.
        ///
        /// \returns the base address of this address space.
        fn base(&self) -> Self::Address;

        /// The size of the address space managed by this instance.
        ///
        /// \returns the size of this address space in bytes.
        fn size(&self) -> usize;

        /// The maximum page permissions that pages allocated inside this space can
        /// obtain.
        ///
        /// \returns the maximum page permissions.
        fn max_page_permissions(&self) -> PagePermissions;

        /// Whether the |address| is inside the address space managed by this instance.
        ///
        /// \returns true if it is inside the address space, false if not.
        fn contains(&self, address: Self::Address) -> bool;

        /// Sets the random seed so that GetRandomPageAddress() will generate
        /// repeatable sequences of random addresses.
        ///
        /// \param The seed for the PRNG.
        fn set_random_seed(&mut self, seed: i64);

        /// Returns a random address inside this address space, suitable for page
        /// allocations hints.
        ///
        /// \returns a random address aligned to allocation_granularity().
        fn random_page_address(&mut self) -> Self::Address;

        /// Allocates private memory pages with the given alignment and permissions.
        ///
        /// \param hint If nonzero, the allocation is attempted to be placed at the
        /// given address first. If that fails, the allocation is attempted to be
        /// placed elsewhere, possibly nearby, but that is not guaranteed. Specifying
        /// zero for the hint always causes this function to choose a random address.
        /// The hint, if specified, must be aligned to the specified alignment.
        ///
        /// \param size The size of the allocation in bytes. Must be a multiple of the
        /// allocation_granularity().
        ///
        /// \param alignment The alignment of the allocation in bytes. Must be a
        /// multiple of the allocation_granularity() and should be a power of two.
        ///
        /// \param permissions The page permissions of the newly allocated pages.
        ///
        /// \returns the start address of the allocated pages on success, zero on
        /// failure.
        const kNoHint: Self::Address = 0;
        fn allocate_pages(
            &mut self, hint: Self::Address, size: usize, alignment: usize, permissions: PagePermissions,
        ) -> Self::Address;

        /// Frees previously allocated pages.
        ///
        /// This function will terminate the process on failure as this implies a bug
        /// in the client. As such, there is no return value.
        ///
        /// \param address The start address of the pages to free. This address must
        /// have been obtained through a call to AllocatePages.
        ///
        /// \param size The size in bytes of the region to free. This must match the
        /// size passed to AllocatePages when the pages were allocated.
        fn free_pages(&mut self, address: Self::Address, size: usize);

        /// Sets permissions of all allocated pages in the given range.
        ///
        /// This operation can fail due to OOM, in which case false is returned. If
        /// the operation fails for a reason other than OOM, this function will
        /// terminate the process as this implies a bug in the client.
        ///
        /// \param address The start address of the range. Must be aligned to
        /// page_size().
        ///
        /// \param size The size in bytes of the range. Must be a multiple
        /// of page_size().
        ///
        /// \param permissions The new permissions for the range.
        ///
        /// \returns true on success, false on OOM.
        fn set_page_permissions(&mut self, address: Self::Address, size: usize, permissions: PagePermissions) -> bool;

        /// Creates a guard region at the specified address.
        ///
        /// Guard regions are guaranteed to cause a fault when accessed and generally
        /// do not count towards any memory consumption limits. Further, allocating
        /// guard regions can usually not fail in subspaces if the region does not
        /// overlap with another region, subspace, or page allocation.
        ///
        /// \param address The start address of the guard region. Must be aligned to
        /// the allocation_granularity().
        ///
        /// \param size The size of the guard region in bytes. Must be a multiple of
        /// the allocation_granularity().
        ///
        /// \returns true on success, false otherwise.
        fn allocate_guard_region(&mut self, address: Self::Address, size: usize) -> bool;

        /// Frees an existing guard region.
        ///
        /// This function will terminate the process on failure as this