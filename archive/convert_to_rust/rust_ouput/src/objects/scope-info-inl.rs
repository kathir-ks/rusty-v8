// Converted from V8 C++ source files:
// Header: scope-info-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_imports)]

use std::marker::PhantomData;

use crate::v8::internal::ObjectSlot;
use crate::v8::internal::ReadOnlyRoots;
use crate::v8::internal::V8;
use crate::v8::internal::code;
use crate::v8::internal::v8;
use crate::v8::internal::BitField64;
use crate::v8::internal::FPUControlRegister;
use crate::v8::internal::Iterator;
use crate::v8::internal::InternalIndex;
use crate::v8::internal::PtrComprCageBase;
use crate::v8::internal::DisallowGarbageCollection;

mod scope_info_generated;

pub struct ScopeInfo {
    flags: u32,
    parameter_count: i32,
    context_local_count: i32,
    dependent_code: *mut DependentCode,
    context_local_names_hashtable: *mut NameToIndexHashTable,
}

impl ScopeInfo {
    pub fn IsAsmModule(&self) -> bool {
        IsAsmModuleBit::decode(self.flags)
    }

    pub fn HasSimpleParameters(&self) -> bool {
        HasSimpleParametersBit::decode(self.flags)
    }

    pub fn Flags(&self) -> u32 {
        self.flags
    }

    pub fn ParameterCount(&self) -> i32 {
        self.parameter_count
    }

    pub fn ContextLocalCount(&self) -> i32 {
        self.context_local_count
    }

    pub fn dependent_code(&self) -> *mut DependentCode {
        self.dependent_code
    }

    pub fn data_start(&self) -> ObjectSlot {
        ObjectSlot {}
    }

    pub fn HasInlinedLocalNames(&self) -> bool {
        self.ContextLocalCount() < kScopeInfoMaxInlinedLocalNamesSize
    }

    pub fn IterateLocalNames(scope_info: *mut ScopeInfo) -> LocalNamesRange<*mut ScopeInfo> {
        LocalNamesRange::<*mut ScopeInfo>::new(scope_info)
    }

    pub fn IterateLocalNames_tagged(
        scope_info: Tagged<ScopeInfo>,
        _no_gc: &DisallowGarbageCollection,
    ) -> LocalNamesRange<Tagged<ScopeInfo>> {
        LocalNamesRange::<Tagged<ScopeInfo>>::new(scope_info)
    }

    fn flags(&self, _relaxed_load: ()) -> u32 {
        self.flags
    }

    fn context_local_names_hashtable(&self) -> *mut NameToIndexHashTable {
        self.context_local_names_hashtable
    }

    fn ContextInlinedLocalName(
        &self,
        _cage_base: PtrComprCageBase,
        index: i32,
    ) -> Tagged<String> {
        Tagged::<String>::empty()
    }
}

const kScopeInfoMaxInlinedLocalNamesSize: i32 = 16;

struct IsAsmModuleBit {}

impl IsAsmModuleBit {
    const OFFSET: usize = 0;
    const SIZE: usize = 1;

    fn decode(flags: u32) -> bool {
        (flags >> Self::OFFSET & ((1 << Self::SIZE) - 1)) != 0
    }
}

struct HasSimpleParametersBit {}

impl HasSimpleParametersBit {
    const OFFSET: usize = 1;
    const SIZE: usize = 1;

    fn decode(flags: u32) -> bool {
        (flags >> Self::OFFSET & ((1 << Self::SIZE) - 1)) != 0
    }
}

pub struct LocalNamesRange<ScopeInfoPtr> {
    scope_info_: ScopeInfoPtr,
    _phantom: PhantomData<ScopeInfoPtr>,
}

impl<ScopeInfoPtr> LocalNamesRange<ScopeInfoPtr> {
    fn new(scope_info_: ScopeInfoPtr) -> Self {
        LocalNamesRange {
            scope_info_: scope_info_,
            _phantom: PhantomData,
        }
    }
}

impl<ScopeInfoPtr> LocalNamesRange<ScopeInfoPtr>
where
    ScopeInfoPtr: Copy,
    LocalNamesRange<ScopeInfoPtr>: LocalNamesRangeTrait<ScopeInfoPtr>,
{
    pub fn begin(&self) -> <Self as LocalNamesRangeTrait<ScopeInfoPtr>>::Iterator {
        <Self as LocalNamesRangeTrait<ScopeInfoPtr>>::Iterator::new(self, InternalIndex { value: 0 })
    }

    pub fn end(&self) -> <Self as LocalNamesRangeTrait<ScopeInfoPtr>>::Iterator {
        <Self as LocalNamesRangeTrait<ScopeInfoPtr>>::Iterator::new(self, self.max_index())
    }
}

trait LocalNamesRangeTrait<ScopeInfoPtr> {
    type Iterator;
    fn inlined(&self) -> bool;
    fn max_index(&self) -> InternalIndex;
}

impl LocalNamesRangeTrait<*mut ScopeInfo> for LocalNamesRange<*mut ScopeInfo> {
    type Iterator = LocalNamesRangeIterator<*mut ScopeInfo>;

    fn inlined(&self) -> bool {
        unsafe { (*self.scope_info_).HasInlinedLocalNames() }
    }

    fn max_index(&self) -> InternalIndex {
        let max = if self.inlined() {
            unsafe { (*self.scope_info_).ContextLocalCount() }
        } else {
            unsafe {
                (*(*self.scope_info_).context_local_names_hashtable()).Capacity() as i32
            }
        };
        InternalIndex { value: max }
    }
}

impl LocalNamesRangeTrait<Tagged<ScopeInfo>> for LocalNamesRange<Tagged<ScopeInfo>> {
    type Iterator = LocalNamesRangeIterator<Tagged<ScopeInfo>>;

    fn inlined(&self) -> bool {
        self.scope_info_.HasInlinedLocalNames()
    }

    fn max_index(&self) -> InternalIndex {
        let max = if self.inlined() {
            self.scope_info_.ContextLocalCount()
        } else {
            unsafe {
                (*self.scope_info_.context_local_names_hashtable).Capacity() as i32
            }
        };
        InternalIndex { value: max }
    }
}

pub struct LocalNamesRangeIterator<ScopeInfoPtr> {
    range_: *const LocalNamesRange<ScopeInfoPtr>,
    index_: InternalIndex,
    _phantom: PhantomData<ScopeInfoPtr>,
}

impl<ScopeInfoPtr> LocalNamesRangeIterator<ScopeInfoPtr>
where
    ScopeInfoPtr: Copy,
{
    fn new(range_: *const LocalNamesRange<ScopeInfoPtr>, index_: InternalIndex) -> Self {
        assert!(!range_.is_null());
        if !unsafe { (*range_).inlined() } {
        }

        LocalNamesRangeIterator {
            range_: range_,
            index_: index_,
            _phantom: PhantomData,
        }
    }

    fn scope_info(&self) -> ScopeInfoPtr {
        unsafe { (*self.range_).scope_info_ }
    }
}

impl<ScopeInfoPtr> LocalNamesRangeIterator<ScopeInfoPtr>
where
    ScopeInfoPtr: Copy,
    LocalNamesRange<ScopeInfoPtr>: LocalNamesRangeTrait<ScopeInfoPtr>,
{
    pub fn name(&self) -> Tagged<String> {
        let cage_base = PtrComprCageBase {};
        self.name_with_cage_base(cage_base)
    }

    pub fn index(&self) -> i32 {
        if unsafe { (*self.range_).inlined() } {
            self.index_.value
        } else {
            unsafe { (*(*LocalNamesRangeIterator::scope_info(self) as *mut ScopeInfo).context_local_names_hashtable()).IndexAt(self.index_) }
        }
    }
}

impl<ScopeInfoPtr> LocalNamesRangeIterator<ScopeInfoPtr>
where
    ScopeInfoPtr: Copy,
{
    fn name_with_cage_base(&self, _cage_base: PtrComprCageBase) -> Tagged<String> {
        assert!(self.index_.value < unsafe { (*self.range_).max_index().value });
        if unsafe { (*self.range_).inlined() } {
            let scope_info = unsafe { (*self.range_).scope_info_ };
            match scope_info {
                _ => {
                    let scope_info_ptr = scope_info as *mut ScopeInfo;
                    unsafe { (*scope_info_ptr).ContextInlinedLocalName(_cage_base, self.index_.value) }
                }
            }
        } else {
            let table_ptr = unsafe { (*(unsafe { (*self.range_).scope_info_ } as *mut ScopeInfo)).context_local_names_hashtable() };
            let key = unsafe { (*table_ptr).KeyAt(_cage_base, self.index_) };
            Tagged::<String>::empty()
        }
    }

    fn table(&self) -> *mut NameToIndexHashTable {
        unsafe { (*(unsafe { (*self.range_).scope_info_ } as *mut ScopeInfo)).context_local_names_hashtable() }
    }
}

impl<ScopeInfoPtr> LocalNamesRangeIterator<ScopeInfoPtr>
where
    ScopeInfoPtr: Copy,
{
    fn advance_hashtable_index(&mut self) {
        let roots = ReadOnlyRoots {};
        let max = unsafe { (*self.range_).max_index() };

        while self.index_.value < max.value {
            let table_ptr = self.table();
            let key = unsafe { (*table_ptr).KeyAt(PtrComprCageBase {}, self.index_) };
            if unsafe { (*table_ptr).IsKey(roots, key) } {
                break;
            }
            self.index_.value += 1;
        }
    }
}

impl<ScopeInfoPtr> PartialEq for LocalNamesRangeIterator<ScopeInfoPtr>
where
    ScopeInfoPtr: Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.range_ as *const _ == other.range_ as *const _ && self.index_ == other.index_
    }
}

impl<ScopeInfoPtr> Eq for LocalNamesRangeIterator<ScopeInfoPtr> where ScopeInfoPtr: Copy {}

impl<ScopeInfoPtr> Iterator for LocalNamesRangeIterator<ScopeInfoPtr>
where
    ScopeInfoPtr: Copy,
    LocalNamesRange<ScopeInfoPtr>: LocalNamesRangeTrait<ScopeInfoPtr>,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_.value < unsafe { (*self.range_).max_index().value } {
            self.index_.value += 1;
            if !unsafe { (*self.range_).inlined() } {
                self.advance_hashtable_index();
            }
            Some(Self {
                range_: self.range_,
                index_: self.index_,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tagged<T> {
    _phantom: PhantomData<T>,
    value: usize,
}

impl<T> Tagged<T> {
    pub fn empty() -> Self {
        Tagged {
            _phantom: PhantomData,
            value: 0,
        }
    }
}

pub struct DependentCode {}

pub struct NameToIndexHashTable {}

impl NameToIndexHashTable {
    fn Capacity(&self) -> usize {
        16
    }
    fn KeyAt(&self, _cage_base: PtrComprCageBase, _index: InternalIndex) -> *mut Object {
        std::ptr::null_mut()
    }

    fn IsKey(&self, _roots: ReadOnlyRoots, _key: *mut Object) -> bool {
        true
    }

    fn IndexAt(&self, _index: InternalIndex) -> i32 {
        0
    }
}

pub struct Object {}
