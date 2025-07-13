// Converted from V8 C++ source files:
// Header: member.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {

pub mod subtle {
pub struct HeapConsistency {}
}  // namespace subtle

pub struct Visitor {}

pub mod internal {

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct MemberBase<StorageType> {
    raw_: StorageType,
}

impl<StorageType> MemberBase<StorageType> {
    pub fn new() -> Self {
        Self {
            raw_: StorageType::default(),
        }
    }

    pub fn with_value(value: *const std::ffi::c_void) -> Self {
        Self {
            raw_: StorageType::from_ptr(value as *mut std::ffi::c_void),
        }
    }

    pub fn with_value_atomic(value: *const std::ffi::c_void) -> Self
    where
        StorageType: AtomicStorage,
    {
        Self {
            raw_: StorageType::from_ptr_atomic(value as *mut std::ffi::c_void),
        }
    }

    pub fn with_raw(raw: StorageType) -> Self {
        Self { raw_: raw }
    }

    pub fn with_nullptr() -> Self {
        Self {
            raw_: StorageType::default(),
        }
    }

    pub fn with_sentinel(s: SentinelPointer) -> Self
    where
        StorageType: From<SentinelPointer>,
    {
        Self { raw_: s.into() }
    }

    pub fn get_raw_slot(&self) -> *mut *const std::ffi::c_void {
        self as *const Self as *mut *const std::ffi::c_void
    }

    pub fn get_raw(&self) -> *const std::ffi::c_void {
        self.raw_.load() as *const std::ffi::c_void
    }

    pub fn set_raw(&mut self, value: *mut std::ffi::c_void) {
        self.raw_.store(value);
    }

    pub fn get_raw_atomic(&self) -> *const std::ffi::c_void
    where
        StorageType: AtomicStorage,
    {
        self.raw_.load_atomic() as *const std::ffi::c_void
    }

    pub fn set_raw_atomic(&self, value: *const std::ffi::c_void)
    where
        StorageType: AtomicStorage,
    {
        self.raw_.store_atomic(value as *mut std::ffi::c_void);
    }

    pub fn get_raw_storage(&self) -> StorageType {
        self.raw_.clone()
    }

    pub fn set_raw_storage_atomic(&self, other: StorageType)
    where
        StorageType: AtomicStorage,
    {
        self.raw_.store_atomic_relaxed(other.load() as *mut std::ffi::c_void);
    }

    pub fn is_cleared(&self) -> bool {
        self.raw_.is_null()
    }

    pub fn clear_from_gc(&self)
    where
        StorageType: Clearable,
    {
        self.raw_.clear();
    }
}

pub trait Clearable {
    fn clear(&self);
    fn is_null(&self) -> bool;
}

pub trait AtomicStorage {
    fn load_atomic(&self) -> *mut std::ffi::c_void;
    fn store_atomic(&self, value: *mut std::ffi::c_void);
    fn store_atomic_relaxed(&self, value: *mut std::ffi::c_void);
}

#[derive(Default, Clone, Copy)]
pub struct RawStorage {
    ptr: *mut std::ffi::c_void,
}

impl RawStorage {
    fn from_ptr(ptr: *mut std::ffi::c_void) -> Self {
        RawStorage { ptr }
    }
    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl AtomicStorage for RawStorage {
    fn load_atomic(&self) -> *mut std::ffi::c_void {
        self.ptr
    }
    fn store_atomic(&self, value: *mut std::ffi::c_void) {
        unsafe {
            let ptr_mut = &self.ptr as *const *mut std::ffi::c_void as *mut *mut std::ffi::c_void;
            *ptr_mut = value;
        }
    }
    fn store_atomic_relaxed(&self, value: *mut std::ffi::c_void) {
        unsafe {
            let ptr_mut = &self.ptr as *const *mut std::ffi::c_void as *mut *mut std::ffi::c_void;
            *ptr_mut = value;
        }
    }
}
impl Clearable for RawStorage {
    fn clear(&self) {
        unsafe {
            let ptr_mut = &self.ptr as *const *mut std::ffi::c_void as *mut *mut std::ffi::c_void;
            *ptr_mut = std::ptr::null_mut();
        }
    }

    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}
impl From<SentinelPointer> for RawStorage {
    fn from(_: SentinelPointer) -> Self {
        RawStorage {
            ptr: std::ptr::null_mut(), // Sentinel pointer is represented as null
        }
    }
}

#[derive(Clone)]
pub struct AtomicRawStorage {
    ptr: AtomicPtr<std::ffi::c_void>,
}

impl Default for AtomicRawStorage {
    fn default() -> Self {
        AtomicRawStorage {
            ptr: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
}

impl AtomicRawStorage {
    fn from_ptr(ptr: *mut std::ffi::c_void) -> Self {
        AtomicRawStorage {
            ptr: AtomicPtr::new(ptr),
        }
    }
}
impl Clearable for AtomicRawStorage {
    fn clear(&self) {
        self.ptr.store(std::ptr::null_mut(), Ordering::Relaxed);
    }
    fn is_null(&self) -> bool {
        self.ptr.load(Ordering::Relaxed).is_null()
    }
}

impl AtomicStorage for AtomicRawStorage {
    fn load_atomic(&self) -> *mut std::ffi::c_void {
        self.ptr.load(Ordering::Relaxed)
    }
    fn store_atomic(&self, value: *mut std::ffi::c_void) {
        self.ptr.store(value, Ordering::Relaxed);
    }
    fn store_atomic_relaxed(&self, value: *mut std::ffi::c_void) {
        self.ptr.store(value, Ordering::Relaxed);
    }
}

#[derive(Default, Copy, Clone)]
pub struct StrongMemberTag {}
#[derive(Default, Copy, Clone)]
pub struct WeakMemberTag {}
#[derive(Default, Copy, Clone)]
pub struct UntracedMemberTag {}

pub struct DijkstraWriteBarrierPolicy {}

impl DijkstraWriteBarrierPolicy {
    pub fn initializing_barrier(_slot: *mut *const std::ffi::c_void, _value: *mut std::ffi::c_void) {}
    pub fn assigning_barrier<const K: usize>(_slot: *mut *const std::ffi::c_void, _value: *mut std::ffi::c_void) {}
}
pub struct NoWriteBarrierPolicy {}
impl NoWriteBarrierPolicy {
    pub fn initializing_barrier(_slot: *mut *const std::ffi::c_void, _value: *mut std::ffi::c_void) {}
    pub fn assigning_barrier<const K: usize>(_slot: *mut *const std::ffi::c_void, _value: *mut std::ffi::c_void) {}
}

pub struct DefaultMemberCheckingPolicy {}

impl DefaultMemberCheckingPolicy {
    pub fn check_pointer<T>(_value: *mut T) {}
}

#[derive(Default, Copy, Clone)]
pub struct DefaultMemberStorage {}

pub struct BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> {
    base: MemberBase<StorageType>,
    checking_policy: PhantomData<CheckingPolicy>,
    weakness_tag: PhantomData<WeaknessTag>,
    write_barrier_policy: PhantomData<WriteBarrierPolicy>,
    phantom: PhantomData<T>,
}

impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
    BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
{
    pub const fn new() -> Self {
        BasicMember {
            base: MemberBase::new(),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        }
    }

    pub const fn with_nullptr() -> Self {
        BasicMember {
            base: MemberBase::with_nullptr(),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        }
    }

    pub fn with_sentinel(s: SentinelPointer) -> Self
    where
        StorageType: From<SentinelPointer>,
    {
        BasicMember {
            base: MemberBase::with_sentinel(s),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        }
    }

    pub fn with_raw(raw: *mut T) -> Self {
        let mut member = BasicMember {
            base: MemberBase::with_value(raw as *mut std::ffi::c_void),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        };
        member.initializing_write_barrier(raw);
        member.check_pointer(raw);
        member
    }

    pub fn with_raw_ref(raw: &mut T) -> Self {
        Self::with_raw(raw as *mut T)
    }

    pub fn with_nullptr_atomic(atomic: AtomicInitializerTag) -> Self
    where
        StorageType: AtomicStorage,
    {
        BasicMember {
            base: MemberBase::with_value_atomic(std::ptr::null_mut()),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        }
    }

    pub fn with_sentinel_atomic(s: SentinelPointer, atomic: AtomicInitializerTag) -> Self
    where
        StorageType: From<SentinelPointer> + AtomicStorage,
    {
        BasicMember {
            base: MemberBase::with_value_atomic(s.into() as *mut std::ffi::c_void),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        }
    }

    pub fn with_raw_atomic(raw: *mut T, atomic: AtomicInitializerTag) -> Self
    where
        StorageType: AtomicStorage,
    {
        let mut member = BasicMember {
            base: MemberBase::with_value_atomic(raw as *mut std::ffi::c_void),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        };
        member.initializing_write_barrier(raw);
        member.check_pointer(raw);
        member
    }

    pub fn with_raw_ref_atomic(raw: &mut T, atomic: AtomicInitializerTag) -> Self
    where
        StorageType: AtomicStorage,
    {
        Self::with_raw_atomic(raw as *mut T, atomic)
    }

    pub fn copy_from(other: &Self) -> Self
    where
        StorageType: Clone,
    {
        BasicMember {
            base: MemberBase::with_raw(other.base.get_raw_storage()),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        }
    }

    pub fn move_from(other: &mut Self) -> Self
    where
        StorageType: Clone + Clearable,
    {
        let mut new_member = BasicMember {
            base: MemberBase::with_raw(other.base.get_raw_storage()),
            checking_policy: PhantomData,
            weakness_tag: PhantomData,
            write_barrier_policy: PhantomData,
            phantom: PhantomData,
        };
        other.clear();
        new_member
    }
    pub fn get(&self) -> *mut T {
        self.base.get_raw() as *mut T
    }
    pub fn clear(&mut self)
    where
        StorageType: Clearable,
    {
        self.base.clear_from_gc();
    }
    pub fn initializing_write_barrier(&mut self, value: *mut T) {
        WriteBarrierPolicy::initializing_barrier(self.base.get_raw_slot(), value as *mut std::ffi::c_void);
    }

    pub fn check_pointer(&self, value: *mut T) {
        CheckingPolicy::check_pointer(value);
    }
}

impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> Deref
    for BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.get() }
    }
}

impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> DerefMut
    for BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.get() }
    }
}

#[derive(Default, Copy, Clone)]
pub struct AtomicInitializerTag {}

}  // namespace internal

pub use internal::BasicMember;
pub use internal::DefaultMemberCheckingPolicy;
pub use internal::DefaultMemberStorage;
pub use internal::DijkstraWriteBarrierPolicy;
pub use internal::NoWriteBarrierPolicy;
pub use internal::RawStorage;
pub use internal::SentinelPointer;
pub use internal::StrongMemberTag;
pub use internal::UntracedMemberTag;
pub use internal::WeakMemberTag;

pub type Member<T> = internal::BasicMember<
    T,
    internal::StrongMemberTag,
    internal::DijkstraWriteBarrierPolicy,
    internal::DefaultMemberCheckingPolicy,
    internal::RawStorage,
>;

pub type WeakMember<T> = internal::BasicMember<
    T,
    internal::WeakMemberTag,
    internal::DijkstraWriteBarrierPolicy,
    internal::DefaultMemberCheckingPolicy,
    internal::RawStorage,
>;

pub type UntracedMember<T> = internal::BasicMember<
    T,
    internal::UntracedMemberTag,
    internal::NoWriteBarrierPolicy,
    internal::DefaultMemberCheckingPolicy,
    internal::RawStorage,
>;

pub mod subtle {
    use super::internal;
    pub type UncompressedMember<T> = internal::BasicMember<
        T,
        internal::StrongMemberTag,
        internal::DijkstraWriteBarrierPolicy,
        internal::DefaultMemberCheckingPolicy,
        internal::RawStorage,
    >;

    #[cfg(CPPGC_POINTER_COMPRESSION)]
    pub type CompressedMember<T> = internal::BasicMember<
        T,
        internal::StrongMemberTag,
        internal::DijkstraWriteBarrierPolicy,
        internal::DefaultMemberCheckingPolicy,
        internal::RawStorage,
    >;
}  // namespace subtle

pub mod internal {
    #[derive(Default, Copy, Clone)]
    pub struct Dummy;

    pub const K_SIZE_OF_MEMBER: usize = std::mem::size_of::<super::Member<Dummy>>();
    pub const K_SIZE_OF_UNCOMPRESSED_MEMBER: usize =
        std::mem::size_of::<super::subtle::UncompressedMember<Dummy>>();
    #[cfg(CPPGC_POINTER_COMPRESSION)]
    pub const K_SIZEOF_COMPRESSED_MEMBER: usize =
        std::mem::size_of::<super::subtle::CompressedMember<Dummy>>();
}  // namespace internal
}  // namespace cppgc

impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType, TQ, UQ>
    std::basic_common_reference::BasicCommonReference<
        cppgc::internal::BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>,
        *mut T,
        TQ,
        UQ,
    > for cppgc::internal::BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
{
    type Type = *mut T;
}

impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType, TQ, UQ>
    std::basic_common_reference::BasicCommonReference<
        *mut T,
        cppgc::internal::BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>,
        TQ,
        UQ,
    > for *mut T
{
    type Type = *mut T;
}
impl<T> From<*mut T> for cppgc::internal::RawStorage {
    fn from(ptr: *mut T) -> Self {
        cppgc::internal::RawStorage {
            ptr: ptr as *mut std::ffi::c_void,
        }
    }
}

use cppgc::internal::SentinelPointer;
impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> PartialEq<SentinelPointer>
    for cppgc::internal::BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
where
    StorageType: cppgc::internal::Clearable,
{
    fn eq(&self, _other: &SentinelPointer) -> bool {
        self.base.is_cleared()
    }
}

impl<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> PartialEq<*mut T>
    for cppgc::internal::BasicMember<T, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
where
    StorageType: From<*mut T> + cppgc::internal::Clearable,
{
    fn eq(&self, other: *mut T) -> bool {
        let raw_storage: StorageType = other.into();
        self.base.get_raw_storage().ptr == raw_storage.ptr
    }
}

impl<T, U, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType> PartialEq<*mut T>
    for cppgc::internal::BasicMember<U, WeaknessTag, WriteBarrierPolicy, CheckingPolicy, StorageType>
where
    StorageType: cppgc::internal::Clearable,
{
    fn eq(&self, other: *mut T) -> bool {
        self.base.get_raw() == (other as *mut std::ffi::c_void)
    }
}
