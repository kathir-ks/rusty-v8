// src/heap/cppgc/heap-page.rs

//use std::alloc::{alloc, dealloc, Layout};
//use std::cmp::Ordering;
//use std::marker::PhantomData;
//use std::mem::{align_of, size_of, transmute};
//use std::ptr::NonNull;
//use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
//use std::sync::{Mutex, MutexGuard};

//use crate::base::logging::DCHECK;
//use crate::base::platform::mutex::Mutex;
//use crate::heap::cppgc::globals::kAllocationGranularity;
//use crate::heap::cppgc::heap_object_header::HeapObjectHeader;
//use crate::heap::cppgc::heap_space::BaseSpace;
//use crate::heap::cppgc::heap::HeapBase;
//use crate::heap::cppgc::memory::{Address, ConstAddress};
//use crate::heap::cppgc::object_start_bitmap::ObjectStartBitmap;
//use crate::heap::cppgc::page_memory::PageBackend;
//use crate::heap::cppgc::raw_heap::RawHeap;
//use crate::heap::cppgc::remembered_set::SlotSet;
//use crate::heap::cppgc::stats_collector::StatsCollector;

mod api_constants {
    pub const kMaxSupportedAlignment: usize = 64; // Example value
}

const kGuaranteedObjectAlignment: usize = 8; // Example value

pub type Address = *mut u8;
pub type ConstAddress = *const u8;

//use v8::base;

fn align_address(address: Address, alignment: usize) -> Address {
    let address_int = address as usize;
    let aligned_address_int = (address_int + alignment - 1) & !(alignment - 1);
    aligned_address_int as Address
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PageType {
    kNormal,
    kLarge,
}

pub trait BasePageTrait {
    fn heap(&self) -> &HeapBase;
    fn space(&self) -> &BaseSpace;
    fn page_type(&self) -> PageType;
    fn is_large(&self) -> bool;
    fn payload_start(&self) -> Address;
    fn payload_end(&self) -> Address;
    fn allocated_size(&self) -> usize;
    fn allocated_bytes_at_last_gc(&self) -> usize;
    fn try_object_header_from_inner_address(&self, address: Address) -> Option<*mut HeapObjectHeader>;
}

pub struct BasePageHandle {
    //heap_handle_: &'a HeapBase, //RawHeap,
}

impl BasePageHandle {
    fn new() -> Self {
        BasePageHandle {}
    }
}

pub struct BasePage {
    heap_handle_: Box<HeapBase>, // Replace with appropriate ownership mechanism (Box, Arc, etc.)
    space_: Box<BaseSpace>,
    type_: PageType,
    //TODO: Figure out slot set stuff
    //slot_set_: Option<Box<SlotSet>>,
    discarded_memory_: usize, //TODO: Figure out how to represent this
}

impl BasePage {
    pub fn new(heap: Box<HeapBase>, space: Box<BaseSpace>, page_type: PageType) -> Self {
        assert_eq!((&heap.raw_heap() as *const _) , (&space.raw_heap() as *const _));
        BasePage {
            heap_handle_: heap,
            space_: space,
            type_: page_type,
            //slot_set_: None,
            discarded_memory_: 0,
        }
    }

    pub fn heap(&self) -> &HeapBase {
        &self.heap_handle_
    }

    pub fn space(&self) -> &BaseSpace {
        &self.space_
    }

    pub fn page_type(&self) -> PageType {
        self.type_
    }

    pub fn is_large(&self) -> bool {
        self.type_ == PageType::kLarge
    }

    pub fn from_inner_address(heap: &HeapBase, address: Address) -> *const BasePage {
        heap.page_backend().lookup(address) as *const BasePage
    }

    pub fn destroy(page: *mut BasePage) {
        unsafe {
            let page_ref = &*page;
            if page_ref.discarded_memory() > 0 {
                page_ref.space()
                    .raw_heap()
                    .heap()
                    .stats_collector()
                    .decrement_discarded_memory(page_ref.discarded_memory());
            }

            if page_ref.is_large() {
                LargePage::destroy(LargePage::from(page));
            } else {
                NormalPage::destroy(NormalPage::from(page));
            }
        }
    }

    pub fn payload_start(&mut self) -> Address {
        if self.is_large() {
            unsafe { LargePage::from(self as *mut BasePage).as_mut().unwrap().payload_start() }
        } else {
            unsafe { NormalPage::from(self as *mut BasePage).as_mut().unwrap().payload_start() }
        }
    }

    pub fn payload_start_const(&self) -> ConstAddress {
        unsafe {
            let page = self as *const Self as *mut Self;
            if (*page).is_large() {
                LargePage::from(page).as_ref().unwrap().payload_start() as ConstAddress
            } else {
                NormalPage::from(page).as_ref().unwrap().payload_start() as ConstAddress
            }
        }
    }

    pub fn payload_end(&mut self) -> Address {
        if self.is_large() {
             unsafe { LargePage::from(self as *mut BasePage).as_mut().unwrap().payload_end() }
        } else {
            unsafe { NormalPage::from(self as *mut BasePage).as_mut().unwrap().payload_end() }
        }
    }

    pub fn payload_end_const(&self) -> ConstAddress {
        unsafe {
            let page = self as *const Self as *mut Self;
            if (*page).is_large() {
                LargePage::from(page).as_ref().unwrap().payload_end() as ConstAddress
            } else {
                NormalPage::from(page).as_ref().unwrap().payload_end() as ConstAddress
            }
        }
    }

    pub fn allocated_size(&self) -> usize {
        if self.is_large() {
            unsafe { LargePage::page_header_size() + LargePage::from(self as *const Self as *mut Self).as_ref().unwrap().payload_size() }
        } else {
            //TODO: Figure out allocation granularity
            unsafe { NormalPage::from(self as *const Self as *mut Self).as_ref().unwrap().payload_size() + round_up(std::mem::size_of::<NormalPage>(), 8) }
        }
    }

    pub fn allocated_bytes_at_last_gc(&self) -> usize {
        if self.is_large() {
            unsafe { LargePage::from(self as *const Self as *mut Self).as_ref().unwrap().allocated_bytes_at_last_gc() }
        } else {
            unsafe { NormalPage::from(self as *const Self as *mut Self).as_ref().unwrap().allocated_bytes_at_last_gc() }
        }
    }

    pub fn try_object_header_from_inner_address(&self, address: Address) -> Option<*mut HeapObjectHeader> {
       unsafe {
            let page = self as *const Self as *mut Self;
            if (*page).is_large() {
                if !LargePage::from(page).as_ref().unwrap().payload_contains(address as ConstAddress) {
                    return None;
                }
            } else {
                let normal_page = NormalPage::from(page).as_ref().unwrap();
                if !normal_page.payload_contains(address as ConstAddress) {
                    return None;
                }
                // Check that the space has no linear allocation buffer.
                assert!(NormalPageSpace::from(normal_page.space()).linear_allocation_buffer().size() == 0);
            }

            // |address| is on the heap, so we FromInnerAddress can get the header.
            let header = Self::object_header_from_inner_address_impl(self as *const Self, address);
            if (*header).is_free() {
                return None;
            }
            assert_ne!((*header).get_gc_info_index(), 255); //kFreeListGCInfoIndex
            Some(header)
        }
    }
    fn object_header_from_inner_address_impl(page: *const Self, address: Address) -> *mut HeapObjectHeader {
        address as *mut HeapObjectHeader //TODO: Implement this
    }

    pub fn change_owner(&mut self, space: &mut BaseSpace) {
        assert_eq!(self.space().raw_heap() as *const _, space.raw_heap() as *const _);
        self.space_ = Box::new(BaseSpace::new(space.raw_heap().clone()));
    }

    fn discarded_memory(&self) -> usize {
        self.discarded_memory_
    }
}

pub struct NormalPage {
    base_page: BasePage,
    object_start_bitmap_: ObjectStartBitmap,
}

impl NormalPage {
    pub fn new(heap: Box<HeapBase>, space: Box<BaseSpace>) -> Self {
        let page = BasePage::new(heap, space, PageType::kNormal);
        let size = page.payload_end() as usize - page.payload_start() as usize;
        assert!(1024 < size as usize);
        NormalPage {
            base_page: page,
            object_start_bitmap_: ObjectStartBitmap::new(),
        }
    }

     pub fn try_create(page_backend: &mut PageBackend, space: &mut NormalPageSpace) -> Option<Box<NormalPage>> {
        let memory = page_backend.try_allocate_normal_page_memory()?;
        //TODO: Fix alignment issues
        let normal_page = Box::new(NormalPage::new(Box::new(HeapBase::new(space.raw_heap().clone())), Box::new(BaseSpace::new(space.raw_heap().clone()))));

        let heap = normal_page.base_page.heap();
        heap.stats_collector().notify_allocated_memory(4096); //kPageSize

        // Memory is zero initialized as
        // a) memory retrieved from the OS is zeroed;
        // b) memory retrieved from the page pool was swept and thus is zeroed except
        //    for the first header which will anyways serve as header again.
        //
        // The following is a subset of SetMemoryInaccessible() to establish the
        // invariant that memory is in the same state as it would be after sweeping.
        // This allows to return newly allocated pages to go into that LAB and back
        // into the free list.
        let begin = normal_page.payload_start() as usize + std::mem::size_of::<HeapObjectHeader>();
        let size = normal_page.payload_size() - std::mem::size_of::<HeapObjectHeader>();
        // TODO: Implement poisoning/zapping

        Some(normal_page)
    }

    pub fn destroy(page: *mut NormalPage) {
         unsafe {
            let page_ref = &*page;
            let heap = page_ref.heap();
            let space = page_ref.space();
            //assert_eq!(space.end(), std::find(space.begin(), space.end(), page));
            //TODO: Implement this
            let backend = heap.page_backend();
            heap.stats_collector().notify_freed_memory(4096); //kPageSize
            backend.free_normal_page_memory(page as Address);
        }
    }

    pub fn from(page: *mut BasePage) -> *mut NormalPage {
        page as *mut NormalPage
    }

    pub fn payload_start(&mut self) -> Address {
       align_address((self as *mut Self as usize + std::mem::size_of::<Self>()) as Address, 8) //kAllocationGranularity
    }

    pub fn payload_end(&mut self) -> Address {
        self.payload_start() as Address + self.payload_size() as usize
    }

    pub fn payload_contains(&self, address: ConstAddress) -> bool {
        address >= self.payload_start() as ConstAddress && address < self.payload_end() as ConstAddress
    }

    fn heap(&self) -> &HeapBase {
        self.base_page.heap()
    }

    fn space(&self) -> &BaseSpace {
        self.base_page.space()
    }
    pub fn allocated_bytes_at_last_gc(&self) -> usize {
        1024 //TODO: Implement this
    }
    fn payload_size(&self) -> usize {
        4096 //TODO: Implement this
    }
    fn linear_allocation_buffer(&self) -> LinearAllocationBuffer {
        LinearAllocationBuffer::new() //TODO: Implement this
    }
}

pub struct NormalPageSpace {
   // base_space: BaseSpace,
}

impl NormalPageSpace {
    pub fn from(space: &BaseSpace) -> &NormalPageSpace {
        unsafe {
            std::mem::transmute::<&BaseSpace, &NormalPageSpace>(space)
        }
    }
    pub fn linear_allocation_buffer(&self) -> LinearAllocationBuffer {
        LinearAllocationBuffer::new() //TODO: Implement this
    }
}

pub struct LinearAllocationBuffer {
    size_: usize,
    start_: Address,
}
impl LinearAllocationBuffer {
    pub fn new() -> Self {
        LinearAllocationBuffer{
            size_: 0,
            start_: 0 as Address,
        }
    }
    pub fn size(&self) -> usize {
        self.size_
    }
    pub fn start(&self) -> Address {
        self.start_
    }
}

pub struct LargePage {
    base_page: BasePage,
    payload_size_: usize,
}

impl LargePage {
     pub fn new(heap: Box<HeapBase>, space: Box<BaseSpace>, size: usize) -> Self {
        LargePage {
            base_page: BasePage::new(heap, space, PageType::kLarge),
            payload_size_: size,
        }
    }

    pub fn allocation_size(payload_size: usize) -> usize {
        Self::page_header_size() + payload_size
    }

    pub fn try_create(page_backend: &mut PageBackend, space: &mut LargePageSpace, size: usize) -> Option<Box<LargePage>> {
        assert!(8 <= api_constants::kMaxSupportedAlignment);
        assert_eq!(api_constants::kMaxSupportedAlignment % 8, 0);
        assert!(1024 <= size);
        let allocation_size = Self::allocation_size(size);
        let memory = page_backend.try_allocate_large_page_memory(allocation_size)?;

        let large_page = Box::new(LargePage::new(Box::new(HeapBase::new(space.raw_heap().clone())), Box::new(BaseSpace::new(space.raw_heap().clone())), size));
        let heap = large_page.base_page.heap();
        heap.stats_collector().notify_allocated_memory(allocation_size);
        Some(large_page)
    }

    pub fn destroy(page: *mut LargePage) {
        unsafe {
            let page_ref = &*page;
            let heap = page_ref.heap();
            let payload_size = page_ref.payload_size();
            let backend = heap.page_backend();
            heap.stats_collector().notify_freed_memory(Self::allocation_size(payload_size));
            backend.free_large_page_memory(page as Address);
        }
    }

    pub fn from(page: *mut BasePage) -> *mut LargePage {
        page as *mut LargePage
    }

    pub fn object_header(&mut self) -> *mut HeapObjectHeader {
        self.payload_start() as *mut HeapObjectHeader
    }

    pub fn payload_start(&mut self) -> Address {
        (self as *mut Self as usize + Self::page_header_size()) as Address
    }

    pub fn payload_end(&mut self) -> Address {
        self.payload_start() as Address + self.payload_size()
    }

    pub fn page_header_size() -> usize {
        std::mem::size_of::<LargePage>()
    }

    pub fn payload_size(&self) -> usize {
        self.payload_size_
    }

    pub fn payload_contains(&self, address: ConstAddress) -> bool {
        address >= self.payload_start() as ConstAddress && address < self.payload_end() as ConstAddress
    }

     fn heap(&self) -> &HeapBase {
        self.base_page.heap()
    }

    pub fn allocated_bytes_at_last_gc(&self) -> usize {
        1024 //TODO: Implement this
    }
}

pub struct LargePageSpace {
   // base_space: BaseSpace,
}
impl LargePageSpace {
    pub fn from(space: &BaseSpace) -> &LargePageSpace {
        unsafe {
            std::mem::transmute::<&BaseSpace, &LargePageSpace>(space)
        }
    }
}

//Helper function
fn round_up(x: usize, alignment: usize) -> usize {
    (x + alignment - 1) & !(alignment - 1)
}

//Dummy structs
pub struct HeapBase {}
impl HeapBase {
    pub fn new(raw_heap: RawHeap) -> Self {
        HeapBase {}
    }
    pub fn page_backend(&self) -> &PageBackend {
        &PageBackend{} //TODO: Implement this
    }
    pub fn raw_heap(&self) -> &RawHeap {
       &RawHeap{} //TODO: Implement this
    }
    pub fn stats_collector(&self) -> &StatsCollector {
        &StatsCollector{} //TODO: Implement this
    }
}

pub struct RawHeap {}
impl RawHeap {
    pub fn heap(&self) -> &HeapBase {
        &HeapBase{} //TODO: Implement this
    }
}

pub struct StatsCollector {}
impl StatsCollector {
    pub fn notify_allocated_memory(&self, size: usize) {
        //TODO: Implement this
    }
    pub fn notify_freed_memory(&self, size: usize) {
        //TODO: Implement this
    }
    pub fn decrement_discarded_memory(&self, size: usize) {
        //TODO: Implement this
    }
}

pub struct HeapObjectHeader {}
impl HeapObjectHeader {
    pub fn is_free(&self) -> bool {
        true //TODO: Implement this
    }
    pub fn get_gc_info_index(&self) -> u8 {
        0 //TODO: Implement this
    }
}

pub struct PageBackend {}
impl PageBackend {
    pub fn lookup(&self, address: ConstAddress) -> Address {
        address as Address //TODO: Implement this
    }

    pub fn try_allocate_normal_page_memory(&mut self) -> Option<Address> {
        Some(0 as Address) //TODO: Implement this
    }

    pub fn free_normal_page_memory(&mut self, address: Address) {
        //TODO: Implement this
    }
    pub fn try_allocate_large_page_memory(&mut self, size: usize) -> Option<Address> {
        Some(0 as Address) //TODO: Implement this
    }
    pub fn free_large_page_memory(&mut self, address: Address) {
        //TODO: Implement this
    }
}

pub struct ObjectStartBitmap {}
impl ObjectStartBitmap {
    pub fn new() -> Self {
        ObjectStartBitmap{} //TODO: Implement this
    }
}