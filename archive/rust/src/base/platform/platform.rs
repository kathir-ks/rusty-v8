// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This module contains the platform-specific code. This make the rest of the
// code less dependent on operating system, compilers and runtime libraries.
// This module does specifically not deal with differences between different
// processor architecture.
// The platform classes have the same definition for all platforms. The
// implementation for a particular platform is put in platform_<os>.cc.
// The build system then uses the implementation for the target platform.
//
// This design has been chosen because it is simple and fast. Alternatively,
// the platform dependent classes could have been implemented using abstract
// superclasses with virtual methods and having specializations for each
// platform. This design was rejected because it was more complicated and
// slower. It would require factory methods for selecting the right
// implementation and the overhead of virtual methods for performance
// sensitive like mutex locking/unlocking.

use std::ffi::{c_char, c_double, c_int, c_long, c_size_t, c_uint, c_ulong, c_void};
use std::fs::File;
use std::io;
use std::mem::MaybeUninit;
use std::num::NonZeroI32;
use std::path::Path;
use std::time::Duration;
use v8::PlatformSharedMemoryHandle;

//use crate::base::abort_mode::AbortMode; // Assuming AbortMode is defined in abort_mode.rs
//use crate::base::build_config; // Assuming build_config defines V8_OS_* and V8_CC_*
//use crate::base::compiler_specific; // Assuming compiler_specific defines V8_INLINE and other compiler-specific attributes
//use crate::base::macros; // Assuming macros defines DISALLOW_IMPLICIT_CONSTRUCTORS
//use crate::base::platform::semaphore::Semaphore; // Assuming Semaphore is defined in semaphore.rs
//use crate::testing::gtest::include::gtest::gtest_prod; // Assuming this is only used for testing and can be excluded

//#[cfg(V8_OS_QNX)]
//use crate::base::qnx_math; // Assuming qnx-math.h functionality is handled here.

//#[cfg(V8_CC_MSVC)]
//use std::arch::x86_64::_rdtsc; // Assuming intrinsics are handled here.

//#[cfg(V8_OS_FUCHSIA)]
//use zx::types::zx_handle_t; // Assuming zircon types are handled appropriately.

//#[cfg(V8_USE_ADDRESS_SANITIZER)]
//use asan; // Assuming address sanitizer integration is handled here.

//#[cfg(not(V8_NO_FAST_TLS))]
//#[cfg(all(V8_CC_MSVC, V8_HOST_ARCH_IA32))]
//extern "C" {
//    fn __readfsdword(offset: c_ulong) -> c_ulong;
//}

//#[cfg(V8_OS_OPENBSD)]
//const PERMISSION_MUTABLE_SECTION: &str = "__attribute__((section(\".openbsd.mutable\")))";
//#[cfg(not(V8_OS_OPENBSD))]
const PERMISSION_MUTABLE_SECTION: &str = "";

mod heap {
    pub mod base {
        pub struct Stack {} // Placeholder, replace with actual implementation
    }
}

pub mod v8 {
    pub mod base {

        //#[cfg(not(V8_NO_FAST_TLS))]
        //#[cfg(all(V8_CC_MSVC, V8_HOST_ARCH_IA32))]
        //const V8_FAST_TLS_SUPPORTED: bool = true;

        //#[cfg(not(V8_NO_FAST_TLS))]
        //#[cfg(all(V8_CC_MSVC, V8_HOST_ARCH_IA32))]
        //#[inline]
        //fn internal_get_existing_thread_local(index: isize) -> isize {
        //    const K_TIB_INLINE_TLS_OFFSET: isize = 0xE10;
        //    const K_TIB_EXTRA_TLS_OFFSET: isize = 0xF94;
        //    const K_MAX_INLINE_SLOTS: isize = 64;
        //    const K_MAX_SLOTS: isize = K_MAX_INLINE_SLOTS + 1024;
        //    const K_SYSTEM_POINTER_SIZE: isize = std::mem::size_of::<*mut c_void>() as isize;

        //    assert!(0 <= index && index < K_MAX_SLOTS);

        //    if index < K_MAX_INLINE_SLOTS {
        //        unsafe { __readfsdword(K_TIB_INLINE_TLS_OFFSET as c_ulong + (K_SYSTEM_POINTER_SIZE * index) as c_ulong) as isize }
        //    } else {
        //        let extra = unsafe { __readfsdword(K_TIB_EXTRA_TLS_OFFSET as c_ulong) } as isize;
        //        if extra == 0 {
        //            0
        //        } else {
        //            unsafe { *(extra as *mut isize).add(((index - K_MAX_INLINE_SLOTS) * K_SYSTEM_POINTER_SIZE) as usize) }
        //        }
        //    }
        //}

        //#[cfg(all(target_os = "macos", any(target_arch = "x86", target_arch = "x86_64")))]
        //const V8_FAST_TLS_SUPPORTED: bool = true;

        //#[cfg(all(target_os = "macos", any(target_arch = "x86", target_arch = "x86_64")))]
        //#[inline]
        //unsafe fn internal_get_existing_thread_local(index: isize) -> isize {
        //    let mut result: isize = 0;
        //    if cfg!(target_arch = "x86") {
        //        std::arch::asm!(
        //            "movl %gs:(,%1,4), %0",
        //            out("r") result,
        //            in("r") index,
        //        );
        //    } else {
        //        std::arch::asm!(
        //            "movq %gs:(,%1,8), %0",
        //            out("r") result,
        //            in("r") index,
        //        );
        //    }
        //    result
        //}

        pub struct TimezoneCache {} // Placeholder, replace with actual implementation

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum MemoryPermission {
            kNoAccess,
            kRead,
            kReadWrite,
            kReadWriteExecute,
            kReadExecute,
            kNoAccessWillJitLater,
        }

        pub struct SharedLibraryAddress {
            pub library_path: String,
            pub start: usize,
            pub end: usize,
            pub aslr_slide: isize,
        }

        impl SharedLibraryAddress {
            pub fn new(library_path: String, start: usize, end: usize) -> Self {
                SharedLibraryAddress {
                    library_path,
                    start,
                    end,
                    aslr_slide: 0,
                }
            }

            pub fn new_with_slide(
                library_path: String,
                start: usize,
                end: usize,
                aslr_slide: isize,
            ) -> Self {
                SharedLibraryAddress {
                    library_path,
                    start,
                    end,
                    aslr_slide,
                }
            }
        }

        pub struct OS {}

        impl OS {
            pub fn initialize(abort_mode: i32, gc_fake_mmap: *const c_char) {
                // Placeholder
            }

            //#[cfg(V8_OS_WIN)]
            pub fn ensure_win32_memory_api_loaded() {
                // Placeholder
            }

            pub fn is_hardware_enforced_shadow_stacks_enabled() -> bool {
                false // Placeholder
            }

            pub fn get_user_time(secs: *mut u32, usecs: *mut u32) -> i32 {
                // Placeholder
                0
            }

            pub fn get_peak_memory_usage_kb() -> i32 {
                // Placeholder
                0
            }

            pub fn time_current_millis() -> f64 {
                // Placeholder
                0.0
            }

            pub fn create_timezone_cache() -> TimezoneCache {
                TimezoneCache {} // Placeholder
            }

            pub fn get_last_error() -> i32 {
                // Placeholder
                0
            }

            pub fn fopen(path: &str, mode: &str) -> Option<File> {
                File::open(path).ok() // Placeholder
            }

            pub fn remove(path: &str) -> bool {
                std::fs::remove_file(path).is_ok() // Placeholder
            }

            pub fn directory_separator() -> char {
                '/' // Placeholder
            }

            pub fn is_directory_separator(ch: char) -> bool {
                ch == '/' // Placeholder
            }

            pub fn open_temporary_file() -> Option<File> {
                // Placeholder
                None
            }

            pub const LOG_FILE_OPEN_MODE: &'static str = "w"; // Placeholder

            #[allow(unused_variables)]
            pub fn print(format: &str, args: ...) {
                // Placeholder
                println!("{}", format);
            }

            #[allow(unused_variables)]
            pub fn vprint(format: &str, args: std::fmt::Arguments) {
                // Placeholder
                println!("{}", format);
            }

            #[allow(unused_variables)]
            pub fn fprint(out: &mut File, format: &str, args: ...) {
                // Placeholder
                println!("{}", format);
            }

            #[allow(unused_variables)]
            pub fn vfprint(out: &mut File, format: &str, args: std::fmt::Arguments) {
                // Placeholder
                println!("{}", format);
            }

            #[allow(unused_variables)]
            pub fn printerror(format: &str, args: ...) {
                // Placeholder
                eprintln!("{}", format);
            }

            #[allow(unused_variables)]
            pub fn vprinterror(format: &str, args: std::fmt::Arguments) {
                // Placeholder
                eprintln!("{}", format);
            }

            pub fn create_shared_memory_handle_for_testing(size: usize) -> PlatformSharedMemoryHandle {
                // Placeholder
                0
            }

            pub fn destroy_shared_memory_handle(handle: PlatformSharedMemoryHandle) {
                // Placeholder
            }

            pub fn has_lazy_commits() -> bool {
                false // Placeholder
            }

            pub fn sleep(interval: Duration) {
                std::thread::sleep(interval); // Placeholder
            }

            pub fn abort() -> ! {
                std::process::abort() // Placeholder
            }

            pub fn debug_break() {
                #[cfg(debug_assertions)]
                std::process::exit(1); // Placeholder, replace with a debugger break
            }

            pub const K_STACK_WALK_ERROR: i32 = -1;
            pub const K_STACK_WALK_MAX_NAME_LEN: usize = 256;
            pub const K_STACK_WALK_MAX_TEXT_LEN: usize = 256;

            #[derive(Debug)]
            pub struct StackFrame {
                pub address: *mut c_void,
                pub text: [c_char; Self::K_STACK_WALK_MAX_TEXT_LEN],
            }

            pub struct MemoryMappedFile {} // Placeholder

            impl MemoryMappedFile {
                pub fn open(name: &str, mode: FileMode) -> Option<MemoryMappedFile> {
                    // Placeholder
                    None
                }

                pub fn create(name: &str, size: usize, initial: *mut c_void) -> Option<MemoryMappedFile> {
                    // Placeholder
                    None
                }
                pub fn memory(&self) -> *mut c_void {
                    std::ptr::null_mut() // Placeholder
                }

                pub fn size(&self) -> usize {
                    0
                }
            }

            #[derive(Debug, Copy, Clone)]
            pub enum FileMode {
                kReadOnly,
                kReadWrite,
            }

            #[allow(unused_variables)]
            pub fn snprintf(str: &mut [u8], format: &str, args: ...) -> i32 {
                // Placeholder
                0
            }

            #[allow(unused_variables)]
            pub fn vsnprintf(str: &mut [u8], format: &str, args: std::fmt::Arguments) -> i32 {
                // Placeholder
                0
            }

            pub fn strncpy(dest: &mut [u8], src: &str, n: usize) {
                let len = std::cmp::min(n, src.len());
                dest[..len].copy_from_slice(&src.as_bytes()[..len]);
                if len < dest.len() {
                    dest[len] = 0; // Null-terminate
                }
            }

            pub fn get_shared_library_addresses() -> Vec<SharedLibraryAddress> {
                // Placeholder
                Vec::new()
            }

            pub fn signal_code_moving_gc() {
                // Placeholder
            }

            pub fn arm_using_hard_float() -> bool {
                false // Placeholder
            }

            pub fn activation_frame_alignment() -> i32 {
                0 // Placeholder
            }

            pub fn get_current_process_id() -> i32 {
                std::process::id() as i32 // Placeholder
            }

            pub fn get_current_thread_id() -> i32 {
                Self::get_current_thread_id_internal() // Placeholder
            }

            pub fn adjust_scheduling_params() {
                // Placeholder
            }

            pub type Address = usize;

            #[derive(Debug, Copy, Clone)]
            pub struct MemoryRange {
                pub start: usize,
                pub end: usize,
            }

            pub fn get_first_free_memory_range_within(
                boundary_start: Address,
                boundary_end: Address,
                minimum_size: usize,
                alignment: usize,
            ) -> Option<MemoryRange> {
                // Placeholder
                None
            }

            pub fn exit_process(exit_code: i32) -> ! {
                std::process::exit(exit_code);
            }

            pub const fn is_remap_page_supported() -> bool {
                cfg!(all(
                    any(target_os = "macos", target_os = "linux"),
                    not(any(
                        target_arch = "powerpc64",
                        target_arch = "s390x"
                    ))
                ))
            }

            pub fn remap_pages(
                address: *const c_void,
                size: usize,
                new_address: *mut c_void,
                access: MemoryPermission,
            ) -> bool {
                // Placeholder
                false
            }

            pub fn set_data_read_only(address: *mut c_void, size: usize) {
                // Placeholder
            }

            fn get_current_thread_id_internal() -> i32 {
                // Placeholder
                0
            }

            fn allocate_page_size() -> usize {
                // Placeholder
                4096
            }

            fn commit_page_size() -> usize {
                // Placeholder
                4096
            }

            fn set_random_mmap_seed(seed: i64) {
                // Placeholder
            }

            fn get_random_mmap_addr() -> *mut c_void {
                std::ptr::null_mut() // Placeholder
            }

            fn allocate(
                address: *mut c_void,
                size: usize,
                alignment: usize,
                access: MemoryPermission,
            ) -> *mut c_void {
                // Placeholder
                std::ptr::null_mut()
            }

            fn allocate_shared(size: usize, access: MemoryPermission) -> *mut c_void {
                // Placeholder
                std::ptr::null_mut()
            }

            fn remap_shared(
                old_address: *mut c_void,
                new_address: *mut c_void,
                size: usize,
            ) -> *mut c_void {
                // Placeholder
                std::ptr::null_mut()
            }

            fn free(address: *mut c_void, size: usize) {
                // Placeholder
            }

            fn allocate_shared_with_handle(
                address: *mut c_void,
                size: usize,
                access: MemoryPermission,
                handle: PlatformSharedMemoryHandle,
                offset: u64,
            ) -> *mut c_void {
                // Placeholder
                std::ptr::null_mut()
            }

            fn free_shared(address: *mut c_void, size: usize) {
                // Placeholder
            }

            fn release(address: *mut c_void, size: usize) {
                // Placeholder
            }

            fn set_permissions(address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
                // Placeholder
                false
            }

            fn recommit_pages(
                address: *mut c_void,
                size: usize,
                access: MemoryPermission,
            ) -> bool {
                // Placeholder
                false
            }

            fn discard_system_pages(address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }

            fn decommit_pages(address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }

            fn seal_pages(address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }

            fn can_reserve_address_space() -> bool {
                // Placeholder
                false
            }

            fn create_address_space_reservation(
                hint: *mut c_void,
                size: usize,
                alignment: usize,
                max_permission: MemoryPermission,
            ) -> Option<AddressSpaceReservation> {
                // Placeholder
                None
            }

            fn free_address_space_reservation(reservation: AddressSpaceReservation) {
                // Placeholder
            }

            const MS_PER_SECOND: i32 = 1000;

            //#[cfg(V8_OS_POSIX)]
            fn get_gc_fake_mmap_file() -> &'static str {
                // Placeholder
                ""
            }
        }

        //#[cfg(V8_OS_WIN)]
        pub fn ensure_console_output_win32() {
            // Placeholder
        }

        pub fn ensure_console_output() {
            //#[cfg(V8_OS_WIN)]
            //ensure_console_output_win32();
        }

        pub struct AddressSpaceReservation {
            base_: *mut c_void,
            size_: usize,
            //#[cfg(V8_OS_FUCHSIA)]
            //vmar_: zx_handle_t,
        }

        impl AddressSpaceReservation {
            pub fn base(&self) -> *mut c_void {
                self.base_
            }

            pub fn size(&self) -> usize {
                self.size_
            }

            pub fn contains(&self, region_addr: *mut c_void, region_size: usize) -> bool {
                let base = self.base_ as usize;
                let region_base = region_addr as usize;
                (region_base >= base) && ((region_base + region_size) <= (base + self.size_))
            }

            pub fn allocate(&self, address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
                // Placeholder
                false
            }

            pub fn free(&self, address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }

            pub fn allocate_shared(
                &self,
                address: *mut c_void,
                size: usize,
                access: MemoryPermission,
                handle: PlatformSharedMemoryHandle,
                offset: u64,
            ) -> bool {
                // Placeholder
                false
            }

            pub fn free_shared(&self, address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }

            pub fn set_permissions(&self, address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
                // Placeholder
                false
            }

            pub fn recommit_pages(&self, address: *mut c_void, size: usize, access: MemoryPermission) -> bool {
                // Placeholder
                false
            }

            pub fn discard_system_pages(&self, address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }

            pub fn decommit_pages(&self, address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }

            pub fn create_sub_reservation(
                &self,
                address: *mut c_void,
                size: usize,
                max_permission: MemoryPermission,
            ) -> Option<AddressSpaceReservation> {
                // Placeholder
                None
            }

            pub fn free_sub_reservation(reservation: AddressSpaceReservation) -> bool {
                // Placeholder
                false
            }

            //#[cfg(V8_OS_WIN)]
            pub fn split_placeholder(&self, address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }

            //#[cfg(V8_OS_WIN)]
            pub fn merge_placeholders(&self, address: *mut c_void, size: usize) -> bool {
                // Placeholder
                false
            }
        }

        pub type LocalStorageKey = i32;

        pub struct Thread {
            options: Options,
            data_: *mut PlatformData,
            name_: [c_char; Self::K_MAX_THREAD_NAME_LENGTH],
            stack_size_: i32,
            priority_: Priority,
            start_semaphore_: Option<Semaphore>,
        }

        impl Thread {
            pub fn new(options: &Options) -> Self {
                let mut name_: [c_char; Self::K_MAX_THREAD_NAME_LENGTH] = [0; Self::K_MAX_THREAD_NAME_LENGTH];
                let name_bytes = options.name().as_bytes();
                let len = std::cmp::min(name_bytes.len(), Self::K_MAX_THREAD_NAME_LENGTH - 1);
                for i in 0..len {
                    name_[i] = name_bytes[i] as c_char;
                }
                name_[len] = 0;

                Thread {
                    options: Options { ..options.clone() },
                    data_: std::ptr::null_mut(), // Placeholder, replace with actual implementation
                    name_,
                    stack_size_: options.stack_size(),
                    priority_: options.priority(),
                    start_semaphore_: None,
                }
            }

            pub fn start(&mut self) -> bool {
                // Placeholder, replace with actual thread creation and starting
                false
            }

            pub fn start_synchronously(&mut self) -> bool {
                self.start_semaphore_ = Some(Semaphore::new(0));
                if !self.start() {
                    return false;
                }
                if let Some(sem) = &self.start_semaphore_ {
                    sem.wait();
                }
                self.start_semaphore_ = None;
                true
            }

            pub fn join(&self) {
                // Placeholder, replace with actual thread joining
            }

            pub fn name(&self) -> &str {
                unsafe {
                    let ptr = self.name_.as_ptr();
                    std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
                }
            }

            pub fn create_thread_local_key() -> LocalStorageKey {
                // Placeholder, replace with actual thread local key creation
                0
            }

            pub fn delete_thread_local_key(key: LocalStorageKey) {
                // Placeholder, replace with actual thread local key deletion
            }

            pub fn get_thread_local(key: LocalStorageKey) -> *mut c_void {
                // Placeholder, replace with actual thread local value retrieval
                std::ptr::null_mut()
            }

            pub fn set_thread_local(key: LocalStorageKey, value: *mut c_void) {
                // Placeholder, replace with actual thread local value setting
            }

            //#[cfg(V8_FAST_TLS_SUPPORTED)]
            pub fn get_existing_thread_local(key: LocalStorageKey) -> *mut c_void {
                // Placeholder
                Self::get_thread_local(key)
            }
            //#[cfg(not(V8_FAST_TLS_SUPPORTED))]
            //pub fn get_existing_thread_local(key: LocalStorageKey) -> *mut c_void {
            //    Self::get_thread_local(key)
            //}

            pub const K_MAX_THREAD_NAME_LENGTH: usize = 16;

            pub fn data(&mut self) -> *mut PlatformData {
                self.data_
            }

            pub fn priority(&self) -> Priority {
                self.priority_
            }

            pub fn notify_started_and_run(&mut self) {
                if let Some(sem) = &self.start_semaphore_ {
                    sem.signal();
                }
                // Placeholder: Replace this with actual run function.
                // Run();
            }
        }

        impl Drop for Thread {
            fn drop(&mut self) {
                // Placeholder, replace with actual thread cleanup
            }
        }

        pub enum Priority {
            kBestEffort,
            kUserVisible,
            kUserBlocking,
            kDefault,
        }

        #[derive(Clone)]
        pub struct Options {
            name_: String,
            priority_: Priority,
            stack_size_: i32,
        }

        impl Options {
            pub fn new(name: &str) -> Self {
                Options {
                    name_: name.to_string(),
                    priority_: Priority::kDefault,
                    stack_size_: 0,
                }
            }
            pub fn new_with_priority(name: &str, priority: Priority) -> Self {
                Options {
                    name_: name.to_string(),
                    priority,
                    stack_size_: 0,
                }
            }
            pub fn new_with_stacksize(name: &str, priority: Priority, stack_size: i32) -> Self {
                Options {
                    name_: name.to_string(),
                    priority,
                    stack_size,
                }
            }

            pub fn name(&self) -> &str {
                &self.name_
            }

            pub fn stack_size(&self) -> i32 {
                self.stack_size_
            }

            pub fn priority(&self) -> Priority {
                self.priority_.clone()
            }
        }

        pub struct PlatformData {}

        pub struct Stack {}

        impl Stack {
            pub struct StackSlot {
                value: usize,
            }

            impl StackSlot {
                pub fn new(value: *mut c_void) -> Self {
                    StackSlot {
                        value: value as usize,
                    }
                }

                pub fn from_usize(value: usize) -> Self {
                    StackSlot { value }
                }

                pub fn to_ptr(&self) -> *mut c_void {
                    self.value as *mut c_void
                }

                pub fn to_usize(&self) -> usize {
                    self.value
                }
            }

            pub fn get_stack_start() -> StackSlot {
                // Placeholder, replace with actual stack start retrieval
                StackSlot::new(std::ptr::null_mut())
            }

            // #[inline(never)]
            pub fn get_current_stack_position() -> StackSlot {
                // Placeholder, replace with actual stack position retrieval
                StackSlot::new(std::ptr::null_mut())
            }

            // #[inline]
            pub fn get_current_frame_address() -> StackSlot {
                // Placeholder, replace with actual frame address retrieval
                StackSlot::new(std::ptr::null_mut())
            }

            pub fn get_real_stack_address_for_slot(slot: StackSlot) -> StackSlot {
                // Placeholder
                slot
            }

            fn get_stack_start_unchecked() -> StackSlot {
                // Placeholder
                StackSlot::new(std::ptr::null_mut())
            }

            fn obtain_current_thread_stack_start() -> Stack::StackSlot {
                // Placeholder
                StackSlot::new(std::ptr::null_mut())
            }
        }

        //#[cfg(V8_HAS_PTHREAD_JIT_WRITE_PROTECT)]
        pub fn set_jit_write_protected(enable: i32) {
            // Placeholder
        }
    }
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

mod semaphore {
    use std::sync::{Condvar, Mutex};
    use std::time::Duration;

    pub struct Semaphore {
        lock: Mutex<i32>,
        condvar: Condvar,
        max: i32,
    }

    impl Semaphore {
        pub fn new(max: i32) -> Self {
            Semaphore {
                lock: Mutex::new(0),
                condvar: Condvar::new(),
                max,
            }
        }

        pub fn wait(&self) {
            let mut count = self.lock.lock().unwrap();
            while *count >= self.max {
                count = self.condvar.wait(count).unwrap();
            }
            *count += 1;
        }

        pub fn wait_timeout(&self, duration: Duration) -> bool {
            let mut count = self.lock.lock().unwrap();
            let result = self.condvar.wait_timeout(count, duration).unwrap();
            count = result.0;
            if result.1.timed_out() {
                return false;
            }
            *count += 1;
            true
        }

        pub fn signal(&self) {
            let mut count = self.lock.lock().unwrap();
            *count -= 1;
            if *count < self.max {
                self.condvar.notify_one();
            }
        }
    }
}