// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::alloc::Layout;
use std::cmp::{Ordering, PartialOrd};
use std::marker::PhantomData;
use std::mem;
use std::num::NonZeroU32;
use std::ops::{Add, Deref, DerefMut, Not, Sub};
use std::ptr;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering::Relaxed};
use std::{
    any::Any,
    borrow::Borrow,
    fmt,
    hash::{Hash, Hasher},
    iter::FusedIterator,
    marker::PhantomPinned,
    pin::Pin,
    task::{Context, Poll},
};

//mod v8config; // Assuming v8config defines configuration constants

pub mod v8 {
    pub struct Array {}
    pub struct Context {}
    pub struct Data {}
    pub struct Isolate {}
}

pub mod internal {
    use std::{
        fmt,
        mem::MaybeUninit,
        num::NonZeroU32,
        ptr::{null_mut, NonNull},
        sync::atomic::{AtomicU64, Ordering::Relaxed},
    };

    pub struct Heap {}
    pub struct LocalHeap {}
    pub struct Isolate {}
    pub struct IsolateGroup {}
    pub struct LocalIsolate {}

    pub type Address = usize;
    pub const kNullAddress: Address = 0;

    pub const KB: i32 = 1024;
    pub const MB: i32 = KB * 1024;
    pub const GB: i32 = MB * 1024;
    #[cfg(target_arch = "x86_64")]
    pub const TB: usize = GB as usize * 1024;

    pub const kApiSystemPointerSize: usize = mem::size_of::<*mut std::ffi::c_void>();
    pub const kApiDoubleSize: usize = mem::size_of::<f64>();
    pub const kApiInt32Size: usize = mem::size_of::<i32>();
    pub const kApiInt64Size: usize = mem::size_of::<i64>();
    pub const kApiSizetSize: usize = mem::size_of::<usize>();

    pub const kHeapObjectTag: i32 = 1;
    pub const kWeakHeapObjectTag: i32 = 3;
    pub const kHeapObjectTagSize: i32 = 2;
    pub const kHeapObjectTagMask: isize = (1 << kHeapObjectTagSize) - 1;
    pub const kHeapObjectReferenceTagMask: isize = 1 << (kHeapObjectTagSize - 1);

    pub const kForwardingTag: i32 = 0;
    pub const kForwardingTagSize: i32 = 2;
    pub const kForwardingTagMask: isize = (1 << kForwardingTagSize) - 1;

    pub const kSmiTag: i32 = 0;
    pub const kSmiTagSize: i32 = 1;
    pub const kSmiTagMask: isize = (1 << kSmiTagSize) - 1;

    pub struct SmiTagging<const tagged_ptr_size: usize>;

    impl SmiTagging<4> {
        pub const kSmiShiftSize: i32 = 0;
        pub const kSmiValueSize: i32 = 31;

        pub const kSmiMinValue: isize =
            ((usize::MAX as isize) << (Self::kSmiValueSize - 1)) as isize;
        pub const kSmiMaxValue: isize = -(Self::kSmiMinValue + 1);

        #[inline]
        pub const fn SmiToInt(value: Address) -> i32 {
            let shift_bits = Self::kSmiTagSize + Self::kSmiShiftSize;
            (value as u32 >> shift_bits) as i32
        }

        #[inline]
        pub const fn IsValidSmi<T>(value: T) -> bool
        where
            T: num_traits::PrimInt + num_traits::Signed,
        {
            (value as usize).wrapping_sub(Self::kSmiMinValue as usize)
                <= (Self::kSmiMaxValue as usize).wrapping_sub(Self::kSmiMinValue as usize)
        }

        #[inline]
        pub const fn IsValidSmi_unsigned<T>(value: T) -> bool
        where
            T: num_traits::PrimInt + num_traits::Unsigned,
        {
            assert!(Self::kSmiMaxValue as usize <= usize::MAX);
            (value as usize) <= Self::kSmiMaxValue as usize
        }

        #[inline]
        pub const fn IsValidSmi_i64(value: i64) -> bool {
            (value as u64).wrapping_sub(Self::kSmiMinValue as u64)
                <= (Self::kSmiMaxValue as u64).wrapping_sub(Self::kSmiMinValue as u64)
        }

        #[inline]
        pub const fn IsValidSmi_u64(value: u64) -> bool {
            assert!(Self::kSmiMaxValue as u64 <= u64::MAX);
            value <= Self::kSmiMaxValue as u64
        }
    }

    impl SmiTagging<8> {
        pub const kSmiShiftSize: i32 = 31;
        pub const kSmiValueSize: i32 = 32;

        pub const kSmiMinValue: isize =
            ((usize::MAX as isize) << (Self::kSmiValueSize - 1)) as isize;
        pub const kSmiMaxValue: isize = -(Self::kSmiMinValue + 1);

        #[inline]
        pub const fn SmiToInt(value: Address) -> i32 {
            let shift_bits = Self::kSmiTagSize + Self::kSmiShiftSize;
            ((value as isize) >> shift_bits) as i32
        }

        #[inline]
        pub const fn IsValidSmi<T>(value: T) -> bool
        where
            T: num_traits::PrimInt + num_traits::Signed,
        {
            i32::MIN as i64 <= value.to_i64().unwrap() && value.to_i64().unwrap() <= i32::MAX as i64
        }

        #[inline]
        pub const fn IsValidSmi_unsigned<T>(value: T) -> bool
        where
            T: num_traits::PrimInt + num_traits::Unsigned,
        {
            value.to_u64().unwrap() <= i32::MAX as u64
        }
    }

    pub const kIntptrAllBitsSet: isize = -1;
    pub const kUintptrAllBitsSet: usize = kIntptrAllBitsSet as usize;

    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub const kApiTaggedSize: usize = kApiSystemPointerSize;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kApiTaggedSize: usize = kApiInt32Size;

    #[inline]
    pub const fn PointerCompressionIsEnabled() -> bool {
        kApiTaggedSize != kApiSystemPointerSize
    }

    #[cfg(feature = "V8_31BIT_SMIS_ON_64BIT_ARCH")]
    pub type PlatformSmiTagging = SmiTagging<4>;
    #[cfg(not(feature = "V8_31BIT_SMIS_ON_64BIT_ARCH"))]
    pub type PlatformSmiTagging = SmiTagging<{ kApiTaggedSize }>;

    pub const kSmiShiftSize: i32 = PlatformSmiTagging::kSmiShiftSize;
    pub const kSmiValueSize: i32 = PlatformSmiTagging::kSmiValueSize;
    pub const kSmiMinValue: i32 = PlatformSmiTagging::kSmiMinValue as i32;
    pub const kSmiMaxValue: i32 = PlatformSmiTagging::kSmiMaxValue as i32;

    #[inline]
    pub const fn SmiValuesAre31Bits() -> bool {
        kSmiValueSize == 31
    }

    #[inline]
    pub const fn SmiValuesAre32Bits() -> bool {
        kSmiValueSize == 32
    }

    #[inline]
    pub const fn Is64() -> bool {
        kApiSystemPointerSize == mem::size_of::<i64>()
    }

    #[inline]
    pub const fn IntToSmi(value: i32) -> Address {
        ((value as Address) << (kSmiTagSize + kSmiShiftSize)) | (kSmiTag as Address)
    }

    #[inline]
    pub const fn SandboxIsEnabled() -> bool {
        cfg!(feature = "V8_ENABLE_SANDBOX")
    }

    pub type SandboxedPointer_t = Address;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxSizeLog2: usize = {
        #[cfg(target_os = "android")]
        {
            37 // 128 GB
        }
        #[cfg(not(target_os = "android"))]
        {
            40 // 1 TB
        }
    };
    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxSize: usize = 1 << kSandboxSizeLog2;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxAlignment: usize = {
        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        {
            1 << 32
        }
        #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
        {
            1
        }
    };

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxedPointerShift: u64 = 64 - (kSandboxSizeLog2 as u64);

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxGuardRegionSize: usize = 32 * (GB as usize) + 4 * (GB as usize);

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxMinimumReservationSize: usize = 8 * (GB as usize);

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kMaxSafeBufferSizeForSandbox: usize = 32 * (GB as usize) - 1;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kBoundedSizeShift: usize = 29;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kPtrComprCageReservationSize: usize = 1 << 32;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kPtrComprCageBaseAlignment: usize = 1 << 32;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kExternalPointerTableReservationSize: usize = {
        #[cfg(target_os = "android")]
        {
            256 * (MB as usize)
        }
        #[cfg(not(target_os = "android"))]
        {
            512 * (MB as usize)
        }
    };

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kExternalPointerIndexShift: u32 = {
        #[cfg(target_os = "android")]
        {
            7
        }
        #[cfg(not(target_os = "android"))]
        {
            6
        }
    };

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kExternalPointerTableEntrySize: i32 = 8;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kExternalPointerTableEntrySizeLog2: i32 = 3;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kMaxExternalPointers: usize =
        kExternalPointerTableReservationSize / (kExternalPointerTableEntrySize as usize);

    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub const kMaxExternalPointers: usize = 0;

    pub const kExternalPointerMarkBit: u64 = 1 << 48;
    pub const kExternalPointerTagShift: u64 = 49;
    pub const kExternalPointerTagMask: u64 = 0x00fe000000000000;
    pub const kExternalPointerShiftedTagMask: u64 = kExternalPointerTagMask >> kExternalPointerTagShift;
    pub const kExternalPointerTagAndMarkbitMask: u64 = 0x00ff000000000000;
    pub const kExternalPointerPayloadMask: u64 = 0xff00ffffffffffff;

    pub type ExternalPointerHandle = u32;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub type ExternalPointer_t = ExternalPointerHandle;
    #[cfg(not(feature = "V8_ENABLE_SANDBOX"))]
    pub type ExternalPointer_t = Address;

    pub const kNullExternalPointer: ExternalPointer_t = 0;
    pub const kNullExternalPointerHandle: ExternalPointerHandle = 0;

    pub type CppHeapPointerHandle = u32;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub type CppHeapPointer_t = CppHeapPointerHandle;
    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub type CppHeapPointer_t = Address;

    pub const kNullCppHeapPointer: CppHeapPointer_t = 0;
    pub const kNullCppHeapPointerHandle: CppHeapPointerHandle = 0;

    pub const kCppHeapPointerMarkBit: u64 = 1;
    pub const kCppHeapPointerTagShift: u64 = 1;
    pub const kCppHeapPointerPayloadShift: u64 = 16;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kCppHeapPointerTableReservationSize: usize = kExternalPointerTableReservationSize;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kCppHeapPointerIndexShift: u32 = kExternalPointerIndexShift;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kCppHeapPointerTableEntrySize: i32 = 8;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kCppHeapPointerTableEntrySizeLog2: i32 = 3;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kMaxCppHeapPointers: usize =
        kCppHeapPointerTableReservationSize / (kCppHeapPointerTableEntrySize as usize);

    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub const kMaxCppHeapPointers: usize = 0;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TagRange<Tag: num_traits::PrimInt> {
        first: Tag,
        last: Tag,
    }

    impl<Tag: num_traits::PrimInt + num_traits::Unsigned + Copy> TagRange<Tag> {
        pub const fn new(first: Tag, last: Tag) -> Self {
            TagRange { first, last }
        }

        pub const fn new_single(tag: Tag) -> Self {
            TagRange { first: tag, last: tag }
        }

        pub const fn new_empty() -> Self {
            TagRange {
                first: Tag::zero(),
                last: Tag::zero(),
            }
        }

        pub const fn is_empty(&self) -> bool {
            self.first == Tag::zero() && self.last == Tag::zero()
        }

        pub const fn size(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                (self.last.to_usize().unwrap() - self.first.to_usize().unwrap() + 1) as usize
            }
        }

        pub const fn contains(&self, tag: Tag) -> bool {
            (tag.to_u32().unwrap()).wrapping_sub(self.first.to_u32().unwrap())
                <= (self.last.to_u32().unwrap()).wrapping_sub(self.first.to_u32().unwrap())
        }

        pub const fn contains_range(&self, tag_range: &TagRange<Tag>) -> bool {
            tag_range.first >= self.first && tag_range.last <= self.last
        }
    }

    impl<Tag: num_traits::PrimInt + num_traits::Unsigned + Copy> From<Tag> for TagRange<Tag> {
        fn from(tag: Tag) -> Self {
            TagRange::new_single(tag)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[repr(u16)]
    pub enum ExternalPointerTag {
        kFirstExternalPointerTag = 0,
        kExternalPointerNullTag = 0,
        kFirstSharedExternalPointerTag,
        kWaiterQueueNodeTag = Self::kFirstSharedExternalPointerTag as isize,
        kExternalStringResourceTag,
        kExternalStringResourceDataTag,
        kLastSharedExternalPointerTag = Self::kExternalStringResourceDataTag as isize,
        kNativeContextMicrotaskQueueTag,
        kEmbedderDataSlotPayloadTag,
        kExternalObjectValueTag,
        kFirstMaybeReadOnlyExternalPointerTag,
        kFunctionTemplateInfoCallbackTag = Self::kFirstMaybeReadOnlyExternalPointerTag as isize,
        kAccessorInfoGetterTag,
        kAccessorInfoSetterTag,
        kLastMaybeReadOnlyExternalPointerTag = Self::kAccessorInfoSetterTag as isize,
        kWasmInternalFunctionCallTargetTag,
        kWasmTypeInfoNativeTypeTag,
        kWasmExportedFunctionDataSignatureTag,
        kWasmStackMemoryTag,
        kWasmIndirectFunctionTargetTag,
        kFirstForeignExternalPointerTag,
        kGenericForeignTag = Self::kFirstForeignExternalPointerTag as isize,
        kApiNamedPropertyQueryCallbackTag,
        kApiNamedPropertyGetterCallbackTag,
        kApiNamedPropertySetterCallbackTag,
        kApiNamedPropertyDescriptorCallbackTag,
        kApiNamedPropertyDefinerCallbackTag,
        kApiNamedPropertyDeleterCallbackTag,
        kApiIndexedPropertyQueryCallbackTag,
        kApiIndexedPropertyGetterCallbackTag,
        kApiIndexedPropertySetterCallbackTag,
        kApiIndexedPropertyDescriptorCallbackTag,
        kApiIndexedPropertyDefinerCallbackTag,
        kApiIndexedPropertyDeleterCallbackTag,
        kApiIndexedPropertyEnumeratorCallbackTag,
        kApiAccessCheckCallbackTag,
        kApiAbortScriptExecutionCallbackTag,
        kSyntheticModuleTag,
        kMicrotaskCallbackTag,
        kMicrotaskCallbackDataTag,
        kCFunctionTag,
        kCFunctionInfoTag,
        kMessageListenerTag,
        kWaiterQueueForeignTag,
        kFirstManagedResourceTag,
        kFirstManagedExternalPointerTag = Self::kFirstManagedResourceTag as isize,
        kGenericManagedTag = Self::kFirstManagedExternalPointerTag as isize,
        kWasmWasmStreamingTag,
        kWasmFuncDataTag,
        kWasmManagedDataTag,
        kWasmNativeModuleTag,
        kIcuBreakIteratorTag,
        kIcuUnicodeStringTag,
        kIcuListFormatterTag,
        kIcuLocaleTag,
        kIcuSimpleDateFormatTag,
        kIcuDateIntervalFormatTag,
        kIcuRelativeDateTimeFormatterTag,
        kIcuLocalizedNumberFormatterTag,
        kIcuPluralRulesTag,
        kIcuCollatorTag,
        kDisplayNamesInternalTag,
        kD8WorkerTag,
        kD8ModuleEmbedderDataTag,
        kLastForeignExternalPointerTag = Self::kD8ModuleEmbedderDataTag as isize,
        kLastManagedExternalPointerTag = Self::kLastForeignExternalPointerTag as isize,
        kArrayBufferExtensionTag,
        kLastManagedResourceTag = Self::kArrayBufferExtensionTag as isize,
        kExternalPointerZappedEntryTag = 0x7d,
        kExternalPointerEvacuationEntryTag = 0x7e,
        kExternalPointerFreeEntryTag = 0x7f,
        kLastExternalPointerTag = 0x7f,
    }

    pub type ExternalPointerTagRange = TagRange<ExternalPointerTag>;

    pub const kAnyExternalPointerTagRange: ExternalPointerTagRange =
        ExternalPointerTagRange::new(
            ExternalPointerTag::kFirstExternalPointerTag,
            ExternalPointerTag::kLastExternalPointerTag,
        );
    pub const kAnySharedExternalPointerTagRange: ExternalPointerTagRange =
        ExternalPointerTagRange::new(
            ExternalPointerTag::kFirstSharedExternalPointerTag,
            ExternalPointerTag::kLastSharedExternalPointerTag,
        );
    pub const kAnyForeignExternalPointerTagRange: ExternalPointerTagRange =
        ExternalPointerTagRange::new(
            ExternalPointerTag::kFirstForeignExternalPointerTag,
            ExternalPointerTag::kLastForeignExternalPointerTag,
        );
    pub const kAnyManagedExternalPointerTagRange: ExternalPointerTagRange =
        ExternalPointerTagRange::new(
            ExternalPointerTag::kFirstManagedExternalPointerTag,
            ExternalPointerTag::kLastManagedExternalPointerTag,
        );
    pub const kAnyMaybeReadOnlyExternalPointerTagRange: ExternalPointerTagRange =
        ExternalPointerTagRange::new(
            ExternalPointerTag::kFirstMaybeReadOnlyExternalPointerTag,
            ExternalPointerTag::kLastMaybeReadOnlyExternalPointerTag,
        );
    pub const kAnyManagedResourceExternalPointerTag: ExternalPointerTagRange =
        ExternalPointerTagRange::new(
            ExternalPointerTag::kFirstManagedResourceTag,
            ExternalPointerTag::kLastManagedResourceTag,
        );

    #[inline]
    pub const fn IsSharedExternalPointerType(tag_range: ExternalPointerTagRange) -> bool {
        kAnySharedExternalPointerTagRange.contains_range(&tag_range)
    }

    #[inline]
    pub const fn IsMaybeReadOnlyExternalPointerType(tag_range: ExternalPointerTagRange) -> bool {
        kAnyMaybeReadOnlyExternalPointerTagRange.contains_range(&tag_range)
    }

    #[inline]
    pub const fn IsManagedExternalPointerType(tag_range: ExternalPointerTagRange) -> bool {
        kAnyManagedResourceExternalPointerTag.contains_range(&tag_range)
    }

    #[inline]
    pub const fn ExternalPointerCanBeEmpty(tag_range: ExternalPointerTagRange) -> bool {
        tag_range.contains(ExternalPointerTag::kArrayBufferExtensionTag)
            || tag_range.contains(ExternalPointerTag::kEmbedderDataSlotPayloadTag)
    }

    pub type IndirectPointerHandle = u32;

    pub const kNullIndirectPointerHandle: IndirectPointerHandle = 0;

    pub type TrustedPointerHandle = IndirectPointerHandle;

    pub const kTrustedPointerTableReservationSize: usize = 64 * (MB as usize);

    pub const kTrustedPointerHandleShift: u32 = 9;

    pub const kNullTrustedPointerHandle: TrustedPointerHandle = kNullIndirectPointerHandle;

    pub const kTrustedPointerTableEntrySize: i32 = 8;
    pub const kTrustedPointerTableEntrySizeLog2: i32 = 3;
    pub const kMaxTrustedPointers: usize =
        kTrustedPointerTableReservationSize / (kTrustedPointerTableEntrySize as usize);

    pub type CodePointerHandle = IndirectPointerHandle;

    pub const kCodePointerTableReservationSize: usize = 128 * (MB as usize);

    pub const kCodePointerHandleShift: u32 = 9;

    pub const kNullCodePointerHandle: CodePointerHandle = kNullIndirectPointerHandle;

    pub const kCodePointerHandleMarker: u32 = 0x1;

    pub const kCodePointerTableEntrySize: i32 = 16;
    pub const kCodePointerTableEntrySizeLog2: i32 = 4;
    pub const kMaxCodePointers: usize =
        kCodePointerTableReservationSize / (kCodePointerTableEntrySize as usize);

    pub const kCodePointerTableEntryEntrypointOffset: i32 = 0;
    pub const kCodePointerTableEntryCodeObjectOffset: i32 = 8;

    pub const kRuntimeGeneratedCodeObjectsLiveInTrustedSpace: bool = true;
    pub const kBuiltinCodeObjectsLiveInTrustedSpace: bool = false;
    pub const kAllCodeObjectsLiveInTrustedSpace: bool =
        kRuntimeGeneratedCodeObjectsLiveInTrustedSpace && kBuiltinCodeObjectsLiveInTrustedSpace;

    extern "C" {
        pub fn IsolateFromNeverReadOnlySpaceObject(obj: Address) -> *mut Isolate;
        pub fn ShouldThrowOnError(isolate: *mut Isolate) -> bool;
    }

    pub struct Internals {}

    impl Internals {
        pub const kHeapObjectMapOffset: i32 = 0;
        pub const kMapInstanceTypeOffset: i32 = kApiTaggedSize as i32 + kApiInt32Size as i32;
        pub const kStringResourceOffset: i32 = kApiTaggedSize as i32 + 2 * (kApiInt32Size as i32);

        pub const kOddballKindOffset: i32 = 4 * (kApiTaggedSize as i32) + kApiDoubleSize as i32;
        pub const kJSObjectHeaderSize: i32 = 3 * (kApiTaggedSize as i32);
        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        pub const kJSAPIObjectWithEmbedderSlotsHeaderSize: i32 =
            Self::kJSObjectHeaderSize + kApiInt32Size as i32;
        #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
        pub const kJSAPIObjectWithEmbedderSlotsHeaderSize: i32 =
            Self::kJSObjectHeaderSize + kApiTaggedSize as i32;
        pub const kFixedArrayHeaderSize: i32 = 2 * (kApiTaggedSize as i32);
        pub const kEmbedderDataArrayHeaderSize: i32 = 2 * (kApiTaggedSize as i32);
        pub const kEmbedderDataSlotSize: i32 = kApiSystemPointerSize as i32;

        #[cfg(feature = "V8_ENABLE_SANDBOX")]
        pub const kEmbedderDataSlotExternalPointerOffset: i32 = kApiTaggedSize as i32;
        #[cfg(not(feature = "V8_ENABLE_SANDBOX"))]
        pub const kEmbedderDataSlotExternalPointerOffset: i32 = 0;

        pub const kNativeContextEmbedderDataOffset: i32 = 6 * (kApiTaggedSize as i32);
        pub const kStringRepresentationAndEncodingMask: i32 = 0x0f;
        pub const kStringEncodingMask: i32 = 0x8;
        pub const kExternalTwoByteRepresentationTag: i32 = 0x02;
        pub const kExternalOneByteRepresentationTag: i32 = 0x0a;

        pub const kNumIsolateDataSlots: u32 = 4;
        pub const kStackGuardSize: i32 = 8 * (kApiSystemPointerSize as i32);
        pub const kNumberOfBooleanFlags: i32 = 6;
        pub const kErrorMessageParamSize: i32 = 1;
        pub const kTablesAlignmentPaddingSize: i32 = 1;
        pub const kRegExpStaticResultOffsetsVectorSize: i32 = kApiSystemPointerSize as i32;
        pub const kBuiltinTier0EntryTableSize: i32 = 7 * (kApiSystemPointerSize as i32);
        pub const kBuiltinTier0TableSize: i32 = 7 * (kApiSystemPointerSize as i32);
        pub const kLinearAllocationAreaSize: i32 = 3 * (kApiSystemPointerSize as i32);
        pub const kThreadLocalTopSize: i32 = 30 * (kApiSystemPointerSize as i32);
        pub const kHandleScopeDataSize: i32 =
            2 * (kApiSystemPointerSize as i32) + 2 * (kApiInt32Size as i32);

        pub const kExternalPointerTableBasePointerOffset: i32 = 0;
        pub const kExternalPointerTableSize: i32 = 2 * (kApiSystemPointerSize as i32);
        pub const kTrustedPointerTableSize: i32 = 2 * (kApiSystemPointerSize as i32);
        pub const kTrustedPointerTableBasePointerOffset: i32 = 0;

        pub const kIsolateCageBaseOffset: i32 = 0;
        pub const kIsolateStackGuardOffset: i32 =
            Self::kIsolateCageBaseOffset + kApiSystemPointerSize as i32;
        pub const kVariousBooleanFlagsOffset: i32 =
            Self::kIsolateStackGuardOffset + Self::kStackGuardSize;
        pub const kErrorMessageParamOffset: i32 =
            Self::kVariousBooleanFlagsOffset + Self::kNumberOfBooleanFlags;
        pub const kBuiltinTier0EntryTableOffset: i32 = Self::kErrorMessageParamOffset
            + Self::kErrorMessageParamSize
            + Self::kTablesAlignmentPaddingSize
            + Self::kRegExpStaticResultOffsetsVectorSize;
        pub const kBuiltinTier0TableOffset: i32 =
            Self::kBuiltinTier0EntryTableOffset + Self::kBuiltinTier0EntryTableSize;
        pub const kNewAllocationInfoOffset: i32 =
            Self::kBuiltinTier0TableOffset + Self::kBuiltinTier0TableSize;
        pub const kOldAllocationInfoOffset: i32 =
            Self::kNewAllocationInfoOffset + Self::kLinearAllocationAreaSize;

        pub const kFastCCallAlignmentPaddingSize: i32 =
            if kApiSystemPointerSize == 8 {
                5 * kApiSystemPointerSize as i32
            } else {
                1 * kApiSystemPointerSize as i32
            };
        pub const kIsolateFastCCallCallerFpOffset: i32 = Self::kOldAllocationInfoOffset
            + Self::kLinearAllocationAreaSize
            + Self::kFastCCallAlignmentPaddingSize;
        pub const kIsolateFastCCallCallerPcOffset: i32 =
            Self::kIsolateFastCCallCallerFpOffset + kApiSystemPointerSize as i32;
        pub const kIsolateFastApiCallTargetOffset: i32 =
            Self::kIsolateFastCCallCallerPcOffset + kApiSystemPointerSize as i32;
        pub const kIsolateLongTaskStatsCounterOffset: i32 =
            Self::kIsolateFastApiCallTargetOffset + kApiSizetSize as i32;
        pub const kIsolateThreadLocalTopOffset: i32 =
            Self::kIsolateLongTaskStatsCounterOffset + kApiSizetSize as i32;
        pub const kIsolateHandleScopeDataOffset: i32 =
            Self::kIsolateThreadLocalTopOffset + Self::kThreadLocalTopSize;
        pub const kIsolateEmbedderDataOffset: i32 =
            Self::kIsolateHandleScopeDataOffset + Self::kHandleScopeDataSize;

        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        pub const kIsolateExternalPointerTableOffset: i32 = Self::kIsolateEmbedderDataOffset
            + (Self::kNumIsolateDataSlots as i32) * (kApiSystemPointerSize as i32);
        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        pub const kIsolateSharedExternalPointerTableAddressOffset: i32 =
            Self::kIsolateExternalPointerTableOffset + Self::kExternalPointerTableSize;
        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        pub const kIsolateCppHeapPointerTableOffset: i32 =
            Self::kIsolateSharedExternalPointerTableAddressOffset + kApiSystemPointerSize as i32;

        #[cfg(all(feature = "V8_COMPRESS_POINTERS", feature = "V8_ENABLE_SANDBOX"))]
        pub const kIsolateTrustedCageBaseOffset: i32 =
            Self::kIsolateCppHeapPointerTableOffset + Self::kExternalPointerTableSize;
        #[cfg(all(feature = "V8_COMPRESS_POINTERS", feature = "V8_ENABLE_SANDBOX"))]
        pub const kIsolateTrustedPointerTableOffset: i32 =
            Self::kIsolateTrustedCageBaseOffset + kApiSystemPointerSize as i32;
        #[cfg(all(feature = "V8_COMPRESS_POINTERS", feature = "V8_ENABLE_SANDBOX"))]
        pub const kIsolateSharedTrustedPointerTableAddressOffset: i32 =
            Self::kIsolateTrustedPointerTableOffset + Self::kTrustedPointerTableSize;
        #[cfg(all(feature = "V8_COMPRESS_POINTERS", feature = "V8_ENABLE_SANDBOX"))]
        pub const kIsolateTrustedPointerPublishingScopeOffset: i32 =
            Self::kIsolateSharedTrustedPointerTableAddressOffset + kApiSystemPointerSize as i32;
        #[cfg(all(feature = "V8_COMPRESS_POINTERS", feature = "V8_ENABLE