// Converted from V8 C++ source files:
// Header: accounting-allocator.h
// Implementation: accounting-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct BoundedPageAllocator {}
    impl BoundedPageAllocator {
        pub fn AllocatePagesAt(
            &mut self,
            _reservation_start: Address,
            _k_zone_page_size: usize,
            _k_no_access: i32,
        ) {
        }
    }
    pub enum PageInitializationMode {
        kAllocatedPagesCanBeUninitialized,
    }
    pub enum PageFreeingMode {
        kMakeInaccessible,
    }
}

pub mod logging {
    pub fn CHECK(_condition: bool) {}
}

pub mod utils {
    pub mod allocation {
        pub struct AllocResult {
            pub ptr: *mut u8,
            pub count: usize,
        }

        pub fn AllocAtLeastWithRetry(bytes: usize) -> AllocResult {
            let ptr = unsafe { libc::malloc(bytes) as *mut u8 };
            if ptr.is_null() {
                AllocResult { ptr: std::ptr::null_mut(), count: 0 }
            } else {
                AllocResult { ptr, count: bytes }
            }
        }
    }
}

pub mod zone {
    pub mod zone_compression {
        pub const kReservationSize: usize = 256 * 1024 * 1024;
        pub const kReservationAlignment: usize = 2 * 1024 * 1024;
    }
    pub mod zone_segment {
        use std::alloc::{alloc, dealloc, Layout};
        use std::mem::size_of;
        use std::ptr::NonNull;

        #[derive(Debug)]
        pub struct Segment {
            total_size_: usize,
        }

        impl Segment {
            pub fn new(size: usize) -> NonNull<Segment> {
                let layout = Layout::new::<Segment>();
                unsafe {
                    let ptr = alloc(layout) as *mut Segment;
                    if ptr.is_null() {
                        panic!("Allocation failed in Segment::new");
                    }
                    (*ptr).total_size_ = size;
                    NonNull::new(ptr).unwrap()
                }
            }

            pub fn total_size(&self) -> usize {
                self.total_size_
            }

            pub fn zap_contents(&mut self) {}
            pub fn zap_header(&mut self) {}
        }

        impl Drop for Segment {
            fn drop(&mut self) {
                let layout = Layout::new::<Segment>();
                unsafe {
                    dealloc(self as *mut Segment as *mut u8, layout);
                }
            }
        }
    }
}

use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::null_mut;
use crate::base;
use crate::logging;
use crate::utils::allocation::AllocResult;
use crate::utils::allocation::AllocAtLeastWithRetry;
use crate::zone::zone_segment::Segment;
use crate::zone::zone_compression;
use crate::V8;

const KB: usize = 1024;

const COMPRESS_ZONES_BOOL: bool = false;

pub struct VirtualMemory {
    address_: *mut u8,
    size_: usize,
}

impl VirtualMemory {
    pub fn new(_platform_allocator: *mut V8, size: usize, hint: *mut std::ffi::c_void, alignment: usize) -> VirtualMemory {
        VirtualMemory {
            address_: hint as *mut u8,
            size_: size
        }
    }

    pub fn IsReserved(&self) -> bool {
        !self.address_.is_null()
    }

    pub fn address(&self) -> *mut u8 {
        self.address_
    }
}

fn RoundDown(x: usize, alignment: usize) -> usize {
    x & !(alignment - 1)
}

fn IsAligned(x: usize, alignment: usize) -> bool {
    x % alignment == 0
}

fn RoundUp(x: usize, alignment: usize) -> usize {
    if x % alignment == 0 {
        x
    } else {
        x + (alignment - (x % alignment))
    }
}

fn GetPlatformPageAllocator() -> *mut V8 {
    null_mut()
}

fn AllocatePages(
    _allocator: *mut base::BoundedPageAllocator,
    _hint: *mut std::ffi::c_void,
    size: usize,
    _page_size: usize,
    _access: i32,
) -> *mut u8 {
    unsafe { libc::malloc(size) as *mut u8 }
}

fn FreePages(_allocator: *mut base::BoundedPageAllocator, ptr: *mut Segment, size: usize) {
    unsafe { libc::free(ptr as *mut std::ffi::c_void) }
}

const ZoneCompression__kReservationSize: usize = 0;
const ZoneCompression__kReservationAlignment: usize = 0;
const kZonePageSize: usize = 256 * KB;

fn ReserveAddressSpace(_platform_allocator: *mut V8) -> VirtualMemory {
    let hint = null_mut();
    VirtualMemory { address_: hint as *mut u8, size_: 0 }
}

fn CreateBoundedAllocator(
    _platform_allocator: *mut V8,
    reservation_start: *mut u8,
) -> Box<base::BoundedPageAllocator> {
    let mut allocator = Box::new(base::BoundedPageAllocator {});
    allocator.AllocatePagesAt(reservation_start as Address, kZonePageSize, 0);
    allocator
}

pub struct AccountingAllocator {
    current_memory_usage_: AtomicUsize,
    max_memory_usage_: AtomicUsize,
    reserved_area_: Option<Box<VirtualMemory>>,
    bounded_page_allocator_: Option<Box<base::BoundedPageAllocator>>,
}

impl AccountingAllocator {
    pub fn new() -> Self {
        let mut reserved_area_: Option<Box<VirtualMemory>> = None;
        let mut bounded_page_allocator_: Option<Box<base::BoundedPageAllocator>> = None;

        if COMPRESS_ZONES_BOOL {
            let platform_page_allocator = GetPlatformPageAllocator();
            let memory = ReserveAddressSpace(platform_page_allocator);
            reserved_area_ = Some(Box::new(memory));
            bounded_page_allocator_ = Some(Box::new(CreateBoundedAllocator(
                platform_page_allocator,
                reserved_area_.as_ref().unwrap().address(),
            )));
        }

        AccountingAllocator {
            current_memory_usage_: AtomicUsize::new(0),
            max_memory_usage_: AtomicUsize::new(0),
            reserved_area_,
            bounded_page_allocator_,
        }
    }

    pub fn allocate_segment(&self, bytes: usize, supports_compression: bool) -> *mut Segment {
        let memory: *mut std::ffi::c_void;
        let mut alloc_bytes = bytes;

        if COMPRESS_ZONES_BOOL && supports_compression {
            alloc_bytes = RoundUp(bytes, kZonePageSize);
            memory = match self.bounded_page_allocator_.as_ref() {
                Some(allocator) => unsafe {
                    AllocatePages(
                        allocator as *const base::BoundedPageAllocator as *mut base::BoundedPageAllocator,
                        null_mut(),
                        alloc_bytes,
                        kZonePageSize,
                        1,
                    ) as *mut std::ffi::c_void
                },
                None => null_mut(),
            };
        } else {
            let result: AllocResult = AllocAtLeastWithRetry(bytes);
            memory = result.ptr as *mut std::ffi::c_void;
            alloc_bytes = result.count;
        }

        if memory.is_null() {
            return null_mut();
        }

        let current = self
            .current_memory_usage_
            .fetch_add(alloc_bytes, Ordering::Relaxed)
            + alloc_bytes;
        let mut max = self.max_memory_usage_.load(Ordering::Relaxed);

        while current > max
            && !self.max_memory_usage_.compare_exchange_weak(
                max,
                current,
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
        {
            max = self.max_memory_usage_.load(Ordering::Relaxed);
        }

        if alloc_bytes < std::mem::size_of::<Segment>() {
            return null_mut();
        }

        unsafe {
            let segment = Segment::new(alloc_bytes);
            segment.as_ptr()
        }
    }

    pub fn return_segment(&self, segment: *mut Segment, supports_compression: bool) {
        if segment.is_null() {
            return;
        }
        unsafe {
            let segment_ref = &mut *segment;
            segment_ref.zap_contents();

            let segment_size = segment_ref.total_size();
            self.current_memory_usage_.fetch_sub(segment_size, Ordering::Relaxed);
            segment_ref.zap_header();
        }

        if COMPRESS_ZONES_BOOL && supports_compression {
            if let Some(allocator) = &self.bounded_page_allocator_ {
                unsafe {
                    FreePages(
                        allocator.as_ref() as *const base::BoundedPageAllocator as *mut base::BoundedPageAllocator,
                        segment,
                        (unsafe { (*segment).total_size() }),
                    );
                }
            }
        } else {
            unsafe {
                libc::free(segment as *mut std::ffi::c_void);
            }
        }
    }

    pub fn get_current_memory_usage(&self) -> usize {
        self.current_memory_usage_.load(Ordering::Relaxed)
    }

    pub fn get_max_memory_usage(&self) -> usize {
        self.max_memory_usage_.load(Ordering::Relaxed)
    }

    pub fn trace_zone_creation(&self, _zone: *const Zone) {
        if !TracingFlags::is_zone_stats_enabled() {
            return;
        }
        self.trace_zone_creation_impl(_zone);
    }

    pub fn trace_zone_destruction(&self, _zone: *const Zone) {
        if !TracingFlags::is_zone_stats_enabled() {
            return;
        }
        self.trace_zone_destruction_impl(_zone);
    }

    pub fn trace_allocate_segment(&self, _segment: *mut Segment) {
        if !TracingFlags::is_zone_stats_enabled() {
            return;
        }
        self.trace_allocate_segment_impl(_segment);
    }

    fn trace_zone_creation_impl(&self, _zone: *const Zone) {}
    fn trace_zone_destruction_impl(&self, _zone: *const Zone) {}
    fn trace_allocate_segment_impl(&self, _segment: *mut Segment) {}
}

impl Drop for AccountingAllocator {
    fn drop(&mut self) {}
}

pub struct Zone {}

pub mod TracingFlags {
    pub fn is_zone_stats_enabled() -> bool {
        false
    }
}

pub type Address = *mut u8;
