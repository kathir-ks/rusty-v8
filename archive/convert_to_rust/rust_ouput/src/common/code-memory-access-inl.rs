// Converted from V8 C++ source files:
// Header: code-memory-access-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use std::{fmt, io};

use crate::internal::Address;

pub mod base {
    pub struct Address {}
    impl Address {
        pub fn new() -> Self {
            Address {}
        }
    }
    pub struct Mutex {}

    impl Mutex {
        pub fn new() -> Self {
            Mutex {}
        }
        pub fn lock(&self) {}
        pub fn unlock(&self) {}
    }
    pub struct Thread {
        name: String,
    }
    impl Thread {
        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }
    }
    pub fn set_jit_write_protected(value: i32) {}
    pub fn write_unaligned_value<T>(address: Address, value: T) {}
    pub struct Memory<T>(T);
}

pub mod flags {
    pub struct Flags {}
    impl Flags {
        pub fn new() -> Self {
            Flags {}
        }
    }
    pub static mut v8_flags: Flags = Flags {  };
}

pub mod objects {
    use crate::internal::Address;

    pub struct InstructionStream {}
    impl InstructionStream {
        pub const kCodeOffset: usize = 0;
        pub const kRelocationInfoOffset: usize = 8;
    }

    pub struct HeapObject {
        address: Address,
    }

    impl HeapObject {
        pub const kMapOffset: usize = 0;
        pub fn from_address(address: Address) -> Self {
            HeapObject { address }
        }
    }
}

pub mod thread_isolation {
    use crate::internal::Address;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    pub struct JitPageKey {
        addr: Address,
        size: usize,
    }
    pub enum JitAllocationType {
        kWasmJumpTable,
        kWasmFarJumpTable,
        kInstructionStream,
    }
    pub enum JitAllocationSource {
        kRegister,
    }
    pub struct JitPage {
        addr: Address,
        size: usize,
        allocations: Vec<JitAllocation>,
        mutex: Mutex<()>,
    }

    impl JitPage {
        fn new(addr: Address, size: usize) -> Self {
            JitPage {
                addr,
                size,
                allocations: Vec::new(),
                mutex: Mutex::new(()),
            }
        }

        pub fn register_allocation(
            &self,
            addr: Address,
            size: usize,
            allocation_type: JitAllocationType,
        ) -> JitAllocationRegistration {
            JitAllocationRegistration {
                size: size,
                allocation_type: allocation_type,
            }
        }

        pub fn lookup_allocation(
            &self,
            addr: Address,
            size: usize,
            allocation_type: JitAllocationType,
        ) -> JitAllocationRegistration {
            JitAllocationRegistration {
                size: size,
                allocation_type: allocation_type,
            }
        }

        pub fn unregister_range(&self, addr: Address, size: usize) {}
        pub fn allocation_containing(&self, addr: Address) -> (Address, JitAllocationRegistration) {
            (Address {}, JitAllocationRegistration{size: 0, allocation_type: JitAllocationType::kInstructionStream})
        }
    }

    pub struct JitAllocationRegistration {
        size: usize,
        allocation_type: JitAllocationType,
    }

    impl JitAllocationRegistration {
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn Type(&self) -> JitAllocationType {
            self.allocation_type
        }
    }

    pub struct ThreadIsolationData {
        jit_pages: Mutex<HashMap<JitPageKey, JitPage>>,
    }

    impl ThreadIsolationData {
        fn new() -> Self {
            ThreadIsolationData {
                jit_pages: Mutex::new(HashMap::new()),
            }
        }
    }

    static mut THREAD_ISOLATION_DATA: Option<ThreadIsolationData> = None;
    static mut INITIALIZED: bool = false;

    pub fn initialize() {
        unsafe {
            if !INITIALIZED {
                THREAD_ISOLATION_DATA = Some(ThreadIsolationData::new());
                INITIALIZED = true;
            }
        }
    }

    pub fn initialized() -> bool {
        unsafe { INITIALIZED }
    }

    pub fn lookup_jit_page(addr: Address, size: usize) -> JitPageRef {
        unsafe {
            if !INITIALIZED {
                initialize();
            }

            let key = JitPageKey { addr, size };

            let mut data = THREAD_ISOLATION_DATA.as_mut().unwrap();
            let mut jit_pages = data.jit_pages.lock().unwrap();

            if !jit_pages.contains_key(&key) {
                jit_pages.insert(key, JitPage::new(addr, size));
            }

            JitPageRef {
                page: jit_pages.get(&key).unwrap()
            }
        }
    }

    pub struct JitPageRef {
        page: &'static JitPage,
    }
    impl JitPageRef{
        pub fn RegisterAllocation(
            &self,
            addr: Address,
            size: usize,
            allocation_type: JitAllocationType,
        ) -> JitAllocationRegistration{
            self.page.register_allocation(addr, size, allocation_type)
        }
        pub fn LookupAllocation(
            &self,
            addr: Address,
            size: usize,
            allocation_type: JitAllocationType,
        ) -> JitAllocationRegistration {
            self.page.lookup_allocation(addr, size, allocation_type)
        }
        pub fn AllocationContaining(&self, addr: Address) -> (Address, JitAllocationRegistration){
            self.page.allocation_containing(addr)
        }
        pub fn UnregisterRange(&self, addr: Address, size: usize){
            self.page.unregister_range(addr, size);
        }
    }

    pub fn pkey_is_available() -> bool {
        true
    }

    pub fn pkey() -> i32 {
        1
    }

    pub fn split_jit_pages(
        far_jump_table_address: Address,
        far_jump_table_size: usize,
        jump_table_address: Address,
        jump_table_size: usize,
    ) -> Option<(JitPageRef, JitPageRef)> {
        Some((lookup_jit_page(far_jump_table_address, far_jump_table_size), lookup_jit_page(jump_table_address, jump_table_size)))
    }
}

pub mod platform {
    pub struct MemoryProtectionKey {}
    impl MemoryProtectionKey {
        pub const kNoMemoryProtectionKey: i32 = -1;
        pub const kNoRestrictions: i32 = 0;
        pub const kDisableWrite: i32 = 1;

        pub fn get_key_permission(key: i32) -> i32 {
            MemoryProtectionKey::kNoRestrictions
        }

        pub fn set_permissions_for_key(key: i32, permission: i32) {}
    }
}

pub struct RwxMemoryWriteScope {
    comment: &'static str,
}

impl RwxMemoryWriteScope {
    pub fn new(comment: &'static str) -> Self {
        if !unsafe { flags::v8_flags.jitless } {
            Self::set_writable();
        }
        RwxMemoryWriteScope { comment }
    }

    pub fn is_supported() -> bool {
        #[cfg(all(not(V8_HAS_PTHREAD_JIT_WRITE_PROTECT), not(V8_HAS_BECORE_JIT_WRITE_PROTECT), not(V8_HAS_PKU_JIT_WRITE_PROTECT)))]
        return false;
        #[cfg(any(V8_HAS_PTHREAD_JIT_WRITE_PROTECT, V8_HAS_BECORE_JIT_WRITE_PROTECT, V8_HAS_PKU_JIT_WRITE_PROTECT))]
        return true;
    }

    pub fn set_writable() {
        #[cfg(V8_HAS_PTHREAD_JIT_WRITE_PROTECT)]
        base::set_jit_write_protected(0);

        #[cfg(V8_HAS_BECORE_JIT_WRITE_PROTECT)]
        unsafe {
            be_memory_inline_jit_restrict_rwx_to_rw_with_witness();
        }

        #[cfg(V8_HAS_PKU_JIT_WRITE_PROTECT)]
        {
            if !thread_isolation::initialized() {
                thread_isolation::initialize();
            }

            if !Self::is_supported() {
                return;
            }

            if platform::MemoryProtectionKey::get_key_permission(thread_isolation::pkey())
                != platform::MemoryProtectionKey::kNoRestrictions
            {
                panic!("");
            }

            platform::MemoryProtectionKey::set_permissions_for_key(
                thread_isolation::pkey(),
                platform::MemoryProtectionKey::kNoRestrictions,
            );
        }
    }

    pub fn set_executable() {
        #[cfg(V8_HAS_PTHREAD_JIT_WRITE_PROTECT)]
        base::set_jit_write_protected(1);

        #[cfg(V8_HAS_BECORE_JIT_WRITE_PROTECT)]
        unsafe {
            be_memory_inline_jit_restrict_rwx_to_rx_with_witness();
        }

        #[cfg(V8_HAS_PKU_JIT_WRITE_PROTECT)]
        {
            if !thread_isolation::initialized() {
                thread_isolation::initialize();
            }
            if !Self::is_supported() {
                return;
            }
            if platform::MemoryProtectionKey::get_key_permission(thread_isolation::pkey())
                != platform::MemoryProtectionKey::kNoRestrictions
            {
                panic!("");
            }
            platform::MemoryProtectionKey::set_permissions_for_key(
                thread_isolation::pkey(),
                platform::MemoryProtectionKey::kDisableWrite,
            );
        }
    }
}

impl Drop for RwxMemoryWriteScope {
    fn drop(&mut self) {
        if !unsafe { flags::v8_flags.jitless } {
            Self::set_executable();
        }
    }
}

pub struct WritableJitAllocation {
    address_: Address,
    size_: usize,
    write_scope_: Option<RwxMemoryWriteScope>,
    page_ref_: Option<thread_isolation::JitPageRef>,
    allocation_: JitAllocationRegistration,
    enforce_write_api_: bool,
}

impl WritableJitAllocation {
    pub fn new(
        addr: Address,
        size: usize,
        allocation_type: thread_isolation::JitAllocationType,
        source: thread_isolation::JitAllocationSource,
        enforce_write_api: bool,
    ) -> Self {
        let mut write_scope_ = None;
        if !unsafe { flags::v8_flags.jitless } {
            write_scope_ = Some(RwxMemoryWriteScope::new("WritableJitAllocation"));
        }
        let page_ref_ = Some(thread_isolation::lookup_jit_page(addr, size));
        let allocation_ = match source {
            thread_isolation::JitAllocationSource::kRegister => page_ref_.as_ref().unwrap().RegisterAllocation(addr, size, allocation_type),
            _ => page_ref_.as_ref().unwrap().LookupAllocation(addr, size, allocation_type),
        };
        let mut res = WritableJitAllocation {
            address_: addr,
            size_: size,
            write_scope_: write_scope_,
            page_ref_: page_ref_,
            allocation_: allocation_,
            enforce_write_api_: enforce_write_api,
        };
        if enforce_write_api {
            res.write_scope_ = None;
        }
        res
    }

    pub fn new2(addr: Address, size: usize, allocation_type: thread_isolation::JitAllocationType, enforce_write_api: bool) -> Self {
        WritableJitAllocation {
            address_: addr,
            size_: size,
            write_scope_: None,
            page_ref_: None,
            allocation_: JitAllocationRegistration{size: size, allocation_type: allocation_type},
            enforce_write_api_: enforce_write_api,
        }
    }

    pub fn for_non_executable_memory(
        addr: Address,
        size: usize,
        type_: thread_isolation::JitAllocationType,
    ) -> Self {
        WritableJitAllocation::new2(addr, size, type_, false)
    }

    pub fn write_scope_for_api_enforcement(&self) -> Option<RwxMemoryWriteScope> {
        if self.enforce_write_api_ {
            Some(RwxMemoryWriteScope::new("WriteScopeForApiEnforcement"))
        } else {
            None
        }
    }

    pub fn write_header_slot<T, const OFFSET: usize>(&self, value: T) {
        let write_scope = self.write_scope_for_api_enforcement();
    }

    pub fn write_header_slot2<T, const OFFSET: usize>(&self, value: Tagged<T>, tag: ReleaseStoreTag) {
        let write_scope = self.write_scope_for_api_enforcement();
    }

    pub fn write_header_slot3<T, const OFFSET: usize>(&self, value: Tagged<T>, tag: RelaxedStoreTag) {
        let write_scope = self.write_scope_for_api_enforcement();
    }

    pub fn write_protected_pointer_header_slot<T, const OFFSET: usize>(
        &self,
        value: Tagged<T>,
        tag: RelaxedStoreTag,
    ) {
        let write_scope = self.write_scope_for_api_enforcement();
    }

    pub fn write_protected_pointer_header_slot2<T, const OFFSET: usize>(
        &self,
        value: Tagged<T>,
        tag: ReleaseStoreTag,
    ) {
        let write_scope = self.write_scope_for_api_enforcement();
    }

    pub fn write_header_slot4<T>(&self, address: Address, value: T, tag: RelaxedStoreTag) {
        if self.allocation_.Type() != thread_isolation::JitAllocationType::kInstructionStream {
            panic!("");
        }
        let offset = address as usize - self.address_.address() as usize;
        let tagged: Tagged<T> = Tagged::new(value);
        match offset {
            objects::InstructionStream::kCodeOffset => {
                self.write_protected_pointer_header_slot::<T, objects::InstructionStream::kCodeOffset>(tagged, tag);
            }
            objects::InstructionStream::kRelocationInfoOffset => {
                self.write_protected_pointer_header_slot::<T, objects::InstructionStream::kRelocationInfoOffset>(tagged, tag);
            }
            _ => {
                panic!("");
            }
        }
    }

    pub fn write_unaligned_value<T>(&self, address: Address, value: T) {
        let write_scope = self.write_scope_for_api_enforcement();
        base::write_unaligned_value(address, value);
    }

    pub fn write_value<T>(&self, address: Address, value: T) {
        let write_scope = self.write_scope_for_api_enforcement();
        unsafe {
            let ptr = address.address() as *mut T;
            *ptr = value;
        }
    }

    pub fn write_value2<T>(&self, address: Address, value: T, tag: RelaxedStoreTag) {
        let write_scope = self.write_scope_for_api_enforcement();
        unsafe {
            let ptr = address.address() as *mut std::sync::atomic::AtomicPtr<T>;
            (*ptr).store(value as *mut T, std::sync::atomic::Ordering::Relaxed);
        }
    }

    pub fn copy_code(&self, dst_offset: usize, src: *const u8, num_bytes: usize) {
        let write_scope = self.write_scope_for_api_enforcement();
        self.copy_bytes(
            (self.address_.address() as usize + dst_offset) as *mut u8,
            src,
            num_bytes,
        );
    }

    pub fn copy_data(&self, dst_offset: usize, src: *const u8, num_bytes: usize) {
        let write_scope = self.write_scope_for_api_enforcement();
        self.copy_bytes(
            (self.address_.address() as usize + dst_offset) as *mut u8,
            src,
            num_bytes,
        );
    }

    fn copy_bytes(&self, dst: *mut u8, src: *const u8, num_bytes: usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, num_bytes);
        }
    }

    pub fn clear_bytes(&self, offset: usize, len: usize) {
        let write_scope = self.write_scope_for_api_enforcement();
        unsafe {
            std::ptr::write_bytes((self.address_.address() as usize + offset) as *mut u8, 0, len);
        }
    }

    pub fn address(&self) -> Address {
        self.address_
    }
    pub fn size(&self) -> usize {
        self.size_
    }
    pub fn allocation_type(&self) -> thread_isolation::JitAllocationType {
        self.allocation_.Type()
    }
}

impl Drop for WritableJitAllocation {
    fn drop(&mut self) {
        if self.enforce_write_api_ && self.page_ref_.is_some() {
            self.write_scope_ = Some(RwxMemoryWriteScope::new("~WritableJitAllocation"));
        }
    }
}

pub struct WritableJumpTablePair {
    writable_jump_table_: WritableJitAllocation,
    writable_far_jump_table_: WritableJitAllocation,
    write_scope_: RwxMemoryWriteScope,
    jump_table_pages_: Option<(thread_isolation::JitPageRef, thread_isolation::JitPageRef)>,
}

impl WritableJumpTablePair {
    pub fn new(
        jump_table_address: Address,
        jump_table_size: usize,
        far_jump_table_address: Address,
        far_jump_table_size: usize,
    ) -> Self {
        let writable_jump_table_ = WritableJitAllocation::new(
            jump_table_address,
            jump_table_size,
            thread_isolation::JitAllocationType::kWasmJumpTable,
            thread_isolation::JitAllocationSource::kRegister,
            true,
        );
        let writable_far_jump_table_ = WritableJitAllocation::new(
            far_jump_table_address,
            far_jump_table_size,
            thread_isolation::JitAllocationType::kWasmFarJumpTable,
            thread_isolation::JitAllocationSource::kRegister,
            true,
        );
        let mut write_scope_ = RwxMemoryWriteScope::new("WritableJumpTablePair");
        let jump_table_pages_ = thread_isolation::split_jit_pages(
            far_jump_table_address,
            far_jump_table_size,
            jump_table_address,
            jump_table_size,
        );

        let (first, second) = jump_table_pages_.as_ref().unwrap();
        assert!(second.RegisterAllocation(jump_table_address, jump_table_size, thread_isolation::JitAllocationType::kWasmJumpTable).Type() == thread_isolation::JitAllocationType::kWasmJumpTable);
        assert!(first.RegisterAllocation(far_jump_table_address, far_jump_table_size, thread_isolation::JitAllocationType::kWasmFarJumpTable).Type() == thread_isolation::JitAllocationType::kWasmFarJumpTable);
        RwxMemoryWriteScope::set_executable();
        WritableJumpTablePair {
            writable_jump_table_,
            writable_far_jump_table_,
            write_scope_: write_scope_,
            jump_table_pages_: jump_table_pages_,
        }
    }
}

struct Tagged<T> {
    value: T,
}

impl<T> Tagged<T> {
    fn new(value: T) -> Self {
        Tagged { value }
    }
}

struct ReleaseStoreTag {}
struct RelaxedStoreTag {}

pub struct WritableJitPage {
    write_scope_: RwxMemoryWriteScope,
    page_ref_: thread_isolation::JitPageRef,
}

impl WritableJitPage {
    pub fn new(addr: Address, size: usize) -> Self {
        WritableJitPage {
            write_scope_: RwxMemoryWriteScope::new("WritableJitPage"),
            page_ref_: thread_isolation::lookup_jit_page(addr, size),
        }
    }

    pub fn lookup_allocation_containing(&self, addr: Address) -> WritableJitAllocation {
        let pair = self.page_ref_.AllocationContaining(addr);
        WritableJitAllocation::new2(pair.0, pair.1.size(), pair.1.Type(), false)
    }

    pub fn free_range(&self, addr: Address, size: usize) -> WritableFreeSpace {
        self.page_ref_.UnregisterRange(addr, size);
        WritableFreeSpace::new(addr, size, true)
    }
}

pub struct WritableFreeSpace {
    address_: Address,
    size_: i32,
    executable_: bool,
}

impl WritableFreeSpace {
    pub fn new(addr: Address, size: usize, executable: bool) -> Self {
        WritableFreeSpace {
            address_: addr,
            size_: size as i32,
            executable_: executable,
        }
    }

    pub fn for_non_executable_memory(addr: Address, size: usize) -> Self {
        WritableFreeSpace::new(addr, size, false)
    }

    pub fn write_header_slot<T, const OFFSET: usize>(
        &self,
        value: Tagged<T>,
        tag: RelaxedStoreTag,
    ) where T: Copy {
    }
}

impl Drop for WritableJitPage {
    fn drop(&mut self) {}
}

impl Drop for WritableFreeSpace {
    fn drop(&mut self) {}
}

#[cfg(V8_HAS_BECORE_JIT_WRITE_PROTECT)]
extern "C" {
    pub fn be_memory_inline_jit_restrict_with_witness_supported() -> i32;
    pub fn be_memory_inline_jit_restrict_rwx_to_rw_with_witness();
    pub fn be_memory_inline_jit_restrict_rwx_to_rx_with_witness();
}
