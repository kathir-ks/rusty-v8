// Converted from V8 C++ source files:
// Header: ptr-compr-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc, sync::Mutex};

use crate::Address;
use crate::PtrComprCageBase;
use crate::Tagged_t;

const V8_COMPRESS_POINTERS: bool = true;
const V8_EXTERNAL_CODE_SPACE: bool = true;
const V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES: bool = true;
const V8_COMPRESS_POINTERS_IN_SHARED_CAGE: bool = true;

const CHAR_BIT: usize = 8;

pub struct IsolateGroup {}
impl IsolateGroup {
    fn current() -> &'static RefCell<Option<usize>> {
        static CURRENT: RefCell<Option<usize>> = RefCell::new(None);
        &CURRENT
    }

    fn set_current(group_id: usize) {
        *IsolateGroup::current().borrow_mut() = Some(group_id);
    }
    pub fn sandbox(&self) -> &'static Sandbox {
        static SANDBOX: Sandbox = Sandbox {};
        &SANDBOX
    }
}
pub struct Isolate {
    cage_base_: Address,
    code_cage_base_: Address,
    isolate_group_: usize,
}

impl Isolate {
    pub fn cage_base(&self) -> Address {
        self.cage_base_
    }
    pub fn code_cage_base(&self) -> Address {
        self.code_cage_base_
    }
    pub fn isolate_group(&self) -> usize {
        self.isolate_group_
    }
}
pub struct Sandbox {}
impl Sandbox {
    fn current() -> &'static RefCell<Option<usize>> {
        static CURRENT: RefCell<Option<usize>> = RefCell::new(None);
        &CURRENT
    }
    fn set_current(group_id: usize) {
        *Sandbox::current().borrow_mut() = Some(group_id);
    }
}

pub struct LocalIsolate {
    cage_base_: Address,
}

impl LocalIsolate {
    pub fn cage_base(&self) -> Address {
        self.cage_base_
    }
}

pub trait CageTrait {
    fn base_non_inlined() -> Address;
    fn set_base_non_inlined(base: Address);
}

pub struct V8HeapCompressionScheme {}
const kPtrComprCageBaseAlignment: usize = 256;
const kNullAddress: Address = 0;

impl V8HeapCompressionScheme {
    pub fn GetPtrComprCageBaseAddress(on_heap_addr: Address) -> Address {
        (on_heap_addr / kPtrComprCageBaseAlignment) * kPtrComprCageBaseAlignment
    }
    pub fn InitBase(base: Address) {}
    pub fn base() -> Address {
        0 // Implement based on actual requirements.
    }
}
pub struct V8HeapCompressionSchemeImpl<Cage> {
    _phantom: std::marker::PhantomData<Cage>,
}

impl<Cage> V8HeapCompressionSchemeImpl<Cage>
where
    Cage: CageTrait,
{
    const kPtrComprCageBaseMask: Address = !(kPtrComprCageBaseAlignment as Address - 1);
    pub fn GetPtrComprCageBaseAddress(on_heap_addr: Address) -> Address {
        (on_heap_addr / kPtrComprCageBaseAlignment as Address) * kPtrComprCageBaseAlignment as Address
    }

    pub fn GetPtrComprCageBaseAddress_cage(cage_base: PtrComprCageBase) -> Address {
        let base = cage_base.address();
        if (base & Self::kPtrComprCageBaseMask) != base {
            panic!("Alignment error");
        }
        base
    }

    pub fn InitBase(base: Address) {
        if base != Self::GetPtrComprCageBaseAddress(base) {
            panic!("Base address not aligned");
        }
        #[cfg(
            all(
                feature = "USING_V8_SHARED_PRIVATE",
                feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"
            )
        )]
        Cage::set_base_non_inlined(base);
        #[cfg(not(all(
            feature = "USING_V8_SHARED_PRIVATE",
            feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"
        )))]
        {
            //Cage::base_ = base;
            Cage::set_base_non_inlined(base);
        }
    }

    pub fn base() -> Address {
        #[cfg(
            all(
                feature = "USING_V8_SHARED_PRIVATE",
                feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"
            )
        )]
        let base = Cage::base_non_inlined();
        #[cfg(not(all(
            feature = "USING_V8_SHARED_PRIVATE",
            feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"
        )))]
        let base = Cage::base_non_inlined();
        if (base & Self::kPtrComprCageBaseMask) != base {
            panic!("Alignment error");
        }
        base
    }

    pub fn CompressObject(tagged: Address) -> Tagged_t {
        #[cfg(feature = "V8_COMPRESS_POINTERS_IN_SHARED_CAGE")]
        {
            if (tagged & Self::kPtrComprCageBaseMask) != Self::base() {
            }
        }
        tagged as Tagged_t
    }

    pub const fn CompressAny(tagged: Address) -> Tagged_t {
        tagged as Tagged_t
    }

    pub fn DecompressTaggedSigned(raw_value: Tagged_t) -> Address {
        raw_value as Address
    }

    pub fn DecompressTagged<TOnHeapAddress>(on_heap_addr: TOnHeapAddress, raw_value: Tagged_t) -> Address {
        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        let cage_base = Self::base();
        #[cfg(feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")]
        if cage_base == kNullAddress {
            panic!("V8HeapCompressionSchemeImpl::base is not initialized for current thread");
        }
        #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
        let cage_base = Self::GetPtrComprCageBaseAddress(on_heap_addr);
        let result = cage_base + raw_value as Address;
        if (result as u32) != raw_value as u32 {
            panic!("Value out of range");
        }
        result
    }

    pub fn ProcessIntermediatePointers<ProcessPointerCallback>(
        cage_base: PtrComprCageBase,
        raw_value: Address,
        callback: ProcessPointerCallback,
    ) where
        ProcessPointerCallback: Fn(Address),
    {
        let decompressed_low =
            V8HeapCompressionSchemeImpl::<Cage>::DecompressTagged(cage_base, raw_value as Tagged_t);
        callback(decompressed_low);
        let decompressed_high = V8HeapCompressionSchemeImpl::<Cage>::DecompressTagged(
            cage_base,
            (raw_value >> (std::mem::size_of::<Tagged_t>() * CHAR_BIT)) as Tagged_t,
        );
        callback(decompressed_high);
    }
}

pub struct ExternalCodeCompressionScheme {}
impl ExternalCodeCompressionScheme {
    pub fn PrepareCageBaseAddress(on_heap_addr: Address) -> Address {
        (on_heap_addr / kPtrComprCageBaseAlignment as Address) * kPtrComprCageBaseAlignment as Address
    }

    pub fn GetPtrComprCageBaseAddress(cage_base: PtrComprCageBase) -> Address {
        let base = cage_base.address();
        if (base & V8HeapCompressionSchemeImpl::<FakeCage>::kPtrComprCageBaseMask) != base {
            panic!("Alignment error");
        }
        base
    }

    pub fn InitBase(base: Address) {
        if base != Self::PrepareCageBaseAddress(base) {
            panic!("Base address not aligned");
        }
        #[cfg(
            all(
                feature = "USING_V8_SHARED_PRIVATE",
                feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"
            )
        )]
        Self::set_base_non_inlined(base);
        #[cfg(not(all(
            feature = "USING_V8_SHARED_PRIVATE",
            feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"
        )))]
        {
            Self::set_base_non_inlined(base);
        }
    }

    pub fn base() -> Address {
        #[cfg(
            all(
                feature = "USING_V8_SHARED_PRIVATE",
                feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"
            )
        )]
        let base = Self::base_non_inlined();
        #[cfg(not(all(
            feature = "USING_V8_SHARED_PRIVATE",
            feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES"
        )))]
        let base = Self::base_non_inlined();
        if (base & V8HeapCompressionSchemeImpl::<FakeCage>::kPtrComprCageBaseMask) != base {
            panic!("Alignment error");
        }
        base
    }

    pub fn CompressObject(tagged: Address) -> Tagged_t {
        #[cfg(feature = "V8_COMPRESS_POINTERS_IN_SHARED_CAGE")]
        {
            if (tagged & V8HeapCompressionSchemeImpl::<FakeCage>::kPtrComprCageBaseMask) != Self::base() {
            }
        }
        tagged as Tagged_t
    }

    pub const fn CompressAny(tagged: Address) -> Tagged_t {
        tagged as Tagged_t
    }

    pub fn DecompressTaggedSigned(raw_value: Tagged_t) -> Address {
        raw_value as Address
    }

    pub fn DecompressTagged<TOnHeapAddress>(on_heap_addr: TOnHeapAddress, raw_value: Tagged_t) -> Address {
        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        let cage_base = Self::base();
        #[cfg(feature = "V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES")]
        if cage_base == kNullAddress {
            panic!("ExternalCodeCompressionScheme::base is not initialized for current thread");
        }
        #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
        let cage_base = Self::GetPtrComprCageBaseAddress(
            PtrComprCageBase {
                address_: on_heap_addr,
            },
        );
        let result = cage_base + raw_value as Address;
        if (result as u32) != raw_value as u32 {
            panic!("Value out of range");
        }
        result
    }

    pub fn ProcessIntermediatePointers<ProcessPointerCallback>(
        cage_base: PtrComprCageBase,
        raw_value: Address,
        callback: ProcessPointerCallback,
    ) where
        ProcessPointerCallback: Fn(Address),
    {
        let decompressed_low = ExternalCodeCompressionScheme::DecompressTagged(cage_base, raw_value as Tagged_t);
        callback(decompressed_low);
        let decompressed_high = ExternalCodeCompressionScheme::DecompressTagged(
            cage_base,
            (raw_value >> (std::mem::size_of::<Tagged_t>() * CHAR_BIT)) as Tagged_t,
        );
        callback(decompressed_high);
    }
    thread_local! {
        static BASE: RefCell<Address> = RefCell::new(0);
    }

    fn base_non_inlined() -> Address {
        Self::BASE.with(|f| *f.borrow())
    }
    fn set_base_non_inlined(base: Address) {
        Self::BASE.with(|f| *f.borrow_mut() = base);
    }
}

pub fn GetPtrComprCageBaseFromOnHeapAddress(address: Address) -> PtrComprCageBase {
    PtrComprCageBase {
        address_: V8HeapCompressionScheme::GetPtrComprCageBaseAddress(address),
    }
}

pub fn GetPtrComprCageBase() -> PtrComprCageBase {
    PtrComprCageBase {
        address_: V8HeapCompressionScheme::base(),
    }
}
pub struct HeapObject {}
pub struct Tagged<T> {
    _phantom: std::marker::PhantomData<T>,
    ptr: Address,
}
impl<T> Tagged<T> {
    pub fn ptr(&self) -> Address {
        self.ptr
    }
}
pub fn GetPtrComprCageBase_obj(object: Tagged<HeapObject>) -> PtrComprCageBase {
    GetPtrComprCageBaseFromOnHeapAddress(object.ptr())
}

pub struct PtrComprCageAccessScope {
    cage_base_: Address,
    code_cage_base_: Address,
    saved_current_isolate_group_: usize,
    saved_current_sandbox_: usize,
}

impl PtrComprCageAccessScope {
    pub fn new(isolate: &Isolate) -> Self {
        let cage_base_ = V8HeapCompressionScheme::base();
        let code_cage_base_ = ExternalCodeCompressionScheme::base();
        let saved_current_isolate_group_ = IsolateGroup::current().borrow().unwrap_or(0);
        let saved_current_sandbox_ = Sandbox::current().borrow().unwrap_or(0);
        V8HeapCompressionScheme::InitBase(isolate.cage_base());
        ExternalCodeCompressionScheme::InitBase(isolate.code_cage_base());
        IsolateGroup::set_current(isolate.isolate_group());
        Sandbox::set_current(isolate.isolate_group());

        Self {
            cage_base_,
            code_cage_base_,
            saved_current_isolate_group_,
            saved_current_sandbox_,
        }
    }
}

impl Drop for PtrComprCageAccessScope {
    fn drop(&mut self) {
        V8HeapCompressionScheme::InitBase(self.cage_base_);
        ExternalCodeCompressionScheme::InitBase(self.code_cage_base_);
        IsolateGroup::set_current(self.saved_current_isolate_group_);
        Sandbox::set_current(self.saved_current_sandbox_);
    }
}
struct FakeCage {}
impl CageTrait for FakeCage {
    fn base_non_inlined() -> Address {
        0
    }
    fn set_base_non_inlined(_base: Address) {}
}
