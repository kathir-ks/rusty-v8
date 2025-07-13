// Converted from V8 C++ source files:
// Header: v8-internal.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
use std::cmp::{PartialOrd, Ordering};
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div, BitAnd, BitOr, BitXor, Shl, Shr};
use std::os::raw::c_char;
use std::ffi::c_void;
use std::{result, fmt};
use std::any::Any;
use std::num::Wrapping;
use std::sync::atomic::AtomicBool;

pub struct Array;
pub struct Context;
pub struct Data;
pub struct Isolate;

pub mod internal {
    use super::*;
    use std::{ptr, fmt};
    use std::any::Any;
    use std::ops::{Add, Sub};
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
    use std::marker::PhantomData;
    use std::os::raw::c_char;
    use std::ffi::c_void;
    use std::sync::atomic::AtomicBool;

    pub struct Heap;
    pub struct LocalHeap;
    pub struct Isolate;
    pub struct IsolateGroup;
    pub struct LocalIsolate;

    pub type Address = usize;
    pub const kNullAddress: Address = 0;

    pub const KB: i32 = 1024;
    pub const MB: i32 = KB * 1024;
    pub const GB: i32 = MB * 1024;
    #[cfg(target_arch = "x86_64")]
    pub const TB: usize = GB as usize * 1024;

    pub const kApiSystemPointerSize: usize = mem::size_of::<*mut c_void>();
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

    pub const kIntptrAllBitsSet: isize = -1;
    pub const kUintptrAllBitsSet: usize = kIntptrAllBitsSet as usize;

    impl SmiTagging<4> {
        pub const kSmiShiftSize: i32 = 0;
        pub const kSmiValueSize: i32 = 31;

        pub const kSmiMinValue: isize =
            (kUintptrAllBitsSet << (Self::kSmiValueSize - 1)) as isize;
        pub const kSmiMaxValue: isize = -(Self::kSmiMinValue + 1);

        #[inline]
        pub const fn SmiToInt(value: Address) -> i32 {
            let shift_bits = Self::kSmiTagSize + Self::kSmiShiftSize;
            (value as u32 >> shift_bits) as i32
        }

        #[inline]
        pub const fn IsValidSmi<T: num_traits::PrimInt + num_traits::Signed>(value: T) -> bool {
            (value as usize).wrapping_sub(Self::kSmiMinValue as usize) <= (Self::kSmiMaxValue as usize).wrapping_sub(Self::kSmiMinValue as usize)
        }

        #[inline]
        pub const fn IsValidSmi_unsigned<T: num_traits::PrimInt + num_traits::Unsigned>(value: T) -> bool {
             assert!(Self::kSmiMaxValue as usize <= usize::MAX);
            (value as usize) <= Self::kSmiMaxValue as usize
        }

         #[inline]
        pub const fn IsValidSmi(value: i64) -> bool {
            (value as u64).wrapping_sub(Self::kSmiMinValue as u64) <= (Self::kSmiMaxValue as u64).wrapping_sub(Self::kSmiMinValue as u64)
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
            (kUintptrAllBitsSet << (Self::kSmiValueSize - 1)) as isize;
        pub const kSmiMaxValue: isize = -(Self::kSmiMinValue + 1);

        #[inline]
        pub const fn SmiToInt(value: Address) -> i32 {
            let shift_bits = Self::kSmiTagSize + Self::kSmiShiftSize;
            ((value as isize) >> shift_bits) as i32
        }

        #[inline]
        pub const fn IsValidSmi<T: num_traits::PrimInt + num_traits::Signed>(value: T) -> bool {
            value >= i32::MIN.into() && value <= i32::MAX.into()
        }

        #[inline]
        pub const fn IsValidSmi_unsigned<T: num_traits::PrimInt + num_traits::Unsigned>(value: T) -> bool {
            value <= i32::MAX.into()
        }
    }

    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub const kApiTaggedSize: usize = kApiSystemPointerSize;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kPtrComprCageReservationSize: usize = 1 << 32;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kPtrComprCageBaseAlignment: usize = 1 << 32;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    static_assert!(
        kApiSystemPointerSize == kApiInt64Size,
        "Pointer compression can be enabled only for 64-bit architectures"
    );
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kApiTaggedSize: usize = kApiInt32Size;

    pub const fn PointerCompressionIsEnabled() -> bool {
        kApiTaggedSize != kApiSystemPointerSize
    }

    #[cfg(feature = "V8_31BIT_SMIS_ON_64BIT_ARCH")]
    pub type PlatformSmiTagging = SmiTagging<kApiInt32Size>;
    #[cfg(not(feature = "V8_31BIT_SMIS_ON_64BIT_ARCH"))]
    pub type PlatformSmiTagging = SmiTagging<kApiTaggedSize>;

    pub const kSmiShiftSize: i32 = PlatformSmiTagging::kSmiShiftSize;
    pub const kSmiValueSize: i32 = PlatformSmiTagging::kSmiValueSize;
    pub const kSmiMinValue: i32 = PlatformSmiTagging::kSmiMinValue as i32;
    pub const kSmiMaxValue: i32 = PlatformSmiTagging::kSmiMaxValue as i32;
    pub const fn SmiValuesAre31Bits() -> bool {
        kSmiValueSize == 31
    }
    pub const fn SmiValuesAre32Bits() -> bool {
        kSmiValueSize == 32
    }
    pub const fn Is64() -> bool {
        kApiSystemPointerSize == mem::size_of::<i64>()
    }

    #[inline]
    pub const fn IntToSmi(value: i32) -> Address {
        ((value as Address) << (kSmiTagSize + kSmiShiftSize)) | kSmiTag as Address
    }

    #[inline]
    pub const fn SandBoxIsEnabled() -> bool {
        cfg!(feature = "V8_ENABLE_SANDBOX")
    }

    pub type SandboxedPointer_t = Address;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxSizeLog2: usize = 40;
    #[cfg(all(feature = "V8_ENABLE_SANDBOX", target_os = "android"))]
    pub const kSandboxSizeLog2: usize = 37;
    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxSize: usize = 1 << kSandboxSizeLog2;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxAlignment: usize = kPtrComprCageBaseAlignment;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxedPointerShift: u64 = 64 - kSandboxSizeLog2 as u64;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxGuardRegionSize: usize = 32 * GB as usize + 4 * GB as usize;
    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    static_assert!(
        kSandboxGuardRegionSize % kSandboxAlignment == 0,
        "The size of the guard regions around the sandbox must be a multiple of its required alignment."
    );

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kSandboxMinimumReservationSize: usize = 8 * GB as usize;

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    static_assert!(
        kSandboxMinimumReservationSize > kPtrComprCageReservationSize,
        "The minimum reservation size for a sandbox must be larger than the pointer compression cage contained within it."
    );

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kMaxSafeBufferSizeForSandbox: usize = 32 * GB as usize - 1;
    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    static_assert!(
        kMaxSafeBufferSizeForSandbox <= kSandboxGuardRegionSize,
        "The maximum allowed buffer size must not be larger than the sandbox's guard regions"
    );

    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    pub const kBoundedSizeShift: i32 = 29;
    #[cfg(feature = "V8_ENABLE_SANDBOX")]
    static_assert!(
        1 << (64 - kBoundedSizeShift) as usize == kMaxSafeBufferSizeForSandbox + 1,
        "The maximum size of a BoundedSize must be synchronized with the kMaxSafeBufferSizeForSandbox"
    );

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kExternalPointerTableReservationSize: usize = 256 * MB as usize;
    #[cfg(all(feature = "V8_COMPRESS_POINTERS", target_os = "android"))]
    pub const kExternalPointerTableReservationSize: usize = 256 * MB as usize;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kExternalPointerIndexShift: u32 = 7;
    #[cfg(all(feature = "V8_COMPRESS_POINTERS", target_os = "android"))]
    pub const kExternalPointerIndexShift: u32 = 7;

    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kExternalPointerTableEntrySize: i32 = 8;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kExternalPointerTableEntrySizeLog2: i32 = 3;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    pub const kMaxExternalPointers: usize =
        kExternalPointerTableReservationSize / kExternalPointerTableEntrySize as usize;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    static_assert!(
        (1 << (32 - kExternalPointerIndexShift)) as usize == kMaxExternalPointers,
        "kExternalPointerTableReservationSize and kExternalPointerIndexShift don't match"
    );

    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub const kMaxExternalPointers: usize = 0;

    pub const kExternalPointerMarkBit: u64 = 1 << 48;
    pub const kExternalPointerTagShift: u64 = 49;
    pub const kExternalPointerTagMask: u64 = 0x00fe000000000000;
    pub const kExternalPointerShiftedTagMask: u64 =
        kExternalPointerTagMask >> kExternalPointerTagShift;
    static_assert!(
        kExternalPointerShiftedTagMask << kExternalPointerTagShift == kExternalPointerTagMask
    );
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
        kCppHeapPointerTableReservationSize / kCppHeapPointerTableEntrySize as usize;
    #[cfg(feature = "V8_COMPRESS_POINTERS")]
    static_assert!(
        (1 << (32 - kCppHeapPointerIndexShift)) as usize == kMaxCppHeapPointers,
        "kCppHeapPointerTableReservationSize and kCppHeapPointerIndexShift don't match"
    );

    #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
    pub const kMaxCppHeapPointers: usize = 0;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TagRange<Tag> {
        first: Tag,
        last: Tag,
    }

    impl<Tag> TagRange<Tag>
    where
        Tag: num_traits::PrimInt + num_traits::Unsigned + Copy + From<u8>,
    {
        pub const fn new(first: Tag, last: Tag) -> Self {
            Self { first, last }
        }

        pub const fn new_single(tag: Tag) -> Self {
            Self { first: tag, last: tag }
        }

        pub const fn new_empty() -> Self {
            Self {
                first: Tag::from(0),
                last: Tag::from(0),
            }
        }

        pub const fn IsEmpty(&self) -> bool {
            self.first == Tag::from(0) && self.last == Tag::from(0)
        }

        pub const fn Size(&self) -> usize {
            if self.IsEmpty() {
                0
            } else {
                (self.last.to_u64().unwrap() - self.first.to_u64().unwrap() + 1) as usize
            }
        }

        pub const fn Contains(&self, tag: Tag) -> bool {
            tag.to_u64().unwrap().wrapping_sub(self.first.to_u64().unwrap()) <= self.last.to_u64().unwrap().wrapping_sub(self.first.to_u64().unwrap())
        }

        pub const fn Contains_range(&self, tag_range: TagRange<Tag>) -> bool {
            tag_range.first >= self.first && tag_range.last <= self.last
        }

        pub const fn hash_value(&self) -> usize {
            (self.first.to_u64().unwrap() as usize) << 16 | self.last.to_u64().unwrap() as usize
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u16)]
    pub enum ExternalPointerTag {
        kFirstExternalPointerTag = 0,
        kExternalPointerNullTag = 0,

        kFirstSharedExternalPointerTag,
        kWaiterQueueNodeTag = kFirstSharedExternalPointerTag as isize,
        kExternalStringResourceTag,
        kExternalStringResourceDataTag,
        kLastSharedExternalPointerTag = kExternalStringResourceDataTag as isize,

        kNativeContextMicrotaskQueueTag,
        kEmbedderDataSlotPayloadTag,
        kExternalObjectValueTag,
        kFirstMaybeReadOnlyExternalPointerTag,
        kFunctionTemplateInfoCallbackTag = kFirstMaybeReadOnlyExternalPointerTag as isize,
        kAccessorInfoGetterTag,
        kAccessorInfoSetterTag,
        kLastMaybeReadOnlyExternalPointerTag = kAccessorInfoSetterTag as isize,
        kWasmInternalFunctionCallTargetTag,
        kWasmTypeInfoNativeTypeTag,
        kWasmExportedFunctionDataSignatureTag,
        kWasmStackMemoryTag,
        kWasmIndirectFunctionTargetTag,

        kFirstForeignExternalPointerTag,
        kGenericForeignTag = kFirstForeignExternalPointerTag as isize,
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
        kFirstManagedExternalPointerTag = kFirstManagedResourceTag as isize,
        kGenericManagedTag = kFirstManagedExternalPointerTag as isize,
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
        kLastForeignExternalPointerTag = kD8ModuleEmbedderDataTag as isize,
        kLastManagedExternalPointerTag = kLastForeignExternalPointerTag as isize,
        kArrayBufferExtensionTag,
        kLastManagedResourceTag = kArrayBufferExtensionTag as isize,

        kExternalPointerZappedEntryTag = 0x7d,
        kExternalPointerEvacuationEntryTag = 0x7e,
        kExternalPointerFreeEntryTag = 0x7f,
        kLastExternalPointerTag = 0x7f,
    }

    impl From<u8> for ExternalPointerTag {
        fn from(value: u8) -> Self {
            match value {
                0 => ExternalPointerTag::kFirstExternalPointerTag,
                1 => ExternalPointerTag::kFirstSharedExternalPointerTag,
                2 => ExternalPointerTag::kExternalStringResourceTag,
                3 => ExternalPointerTag::kExternalStringResourceDataTag,
                4 => ExternalPointerTag::kNativeContextMicrotaskQueueTag,
                5 => ExternalPointerTag::kEmbedderDataSlotPayloadTag,
                6 => ExternalPointerTag::kExternalObjectValueTag,
                7 => ExternalPointerTag::kFirstMaybeReadOnlyExternalPointerTag,
                8 => ExternalPointerTag::kFunctionTemplateInfoCallbackTag,
                9 => ExternalPointerTag::kAccessorInfoGetterTag,
                10 => ExternalPointerTag::kAccessorInfoSetterTag,
                11 => ExternalPointerTag::kWasmInternalFunctionCallTargetTag,
                12 => ExternalPointerTag::kWasmTypeInfoNativeTypeTag,
                13 => ExternalPointerTag::kWasmExportedFunctionDataSignatureTag,
                14 => ExternalPointerTag::kWasmStackMemoryTag,
                15 => ExternalPointerTag::kWasmIndirectFunctionTargetTag,
                16 => ExternalPointerTag::kFirstForeignExternalPointerTag,
                17 => ExternalPointerTag::kApiNamedPropertyQueryCallbackTag,
                18 => ExternalPointerTag::kApiNamedPropertyGetterCallbackTag,
                19 => ExternalPointerTag::kApiNamedPropertySetterCallbackTag,
                20 => ExternalPointerTag::kApiNamedPropertyDescriptorCallbackTag,
                21 => ExternalPointerTag::kApiNamedPropertyDefinerCallbackTag,
                22 => ExternalPointerTag::kApiNamedPropertyDeleterCallbackTag,
                23 => ExternalPointerTag::kApiIndexedPropertyQueryCallbackTag,
                24 => ExternalPointerTag::kApiIndexedPropertyGetterCallbackTag,
                25 => ExternalPointerTag::kApiIndexedPropertySetterCallbackTag,
                26 => ExternalPointerTag::kApiIndexedPropertyDescriptorCallbackTag,
                27 => ExternalPointerTag::kApiIndexedPropertyDefinerCallbackTag,
                28 => ExternalPointerTag::kApiIndexedPropertyDeleterCallbackTag,
                29 => ExternalPointerTag::kApiIndexedPropertyEnumeratorCallbackTag,
                30 => ExternalPointerTag::kApiAccessCheckCallbackTag,
                31 => ExternalPointerTag::kApiAbortScriptExecutionCallbackTag,
                32 => ExternalPointerTag::kSyntheticModuleTag,
                33 => ExternalPointerTag::kMicrotaskCallbackTag,
                34 => ExternalPointerTag::kMicrotaskCallbackDataTag,
                35 => ExternalPointerTag::kCFunctionTag,
                36 => ExternalPointerTag::kCFunctionInfoTag,
                37 => ExternalPointerTag::kMessageListenerTag,
                38 => ExternalPointerTag::kWaiterQueueForeignTag,
                39 => ExternalPointerTag::kFirstManagedResourceTag,
                40 => ExternalPointerTag::kWasmWasmStreamingTag,
                41 => ExternalPointerTag::kWasmFuncDataTag,
                42 => ExternalPointerTag::kWasmManagedDataTag,
                43 => ExternalPointerTag::kWasmNativeModuleTag,
                44 => ExternalPointerTag::kIcuBreakIteratorTag,
                45 => ExternalPointerTag::kIcuUnicodeStringTag,
                46 => ExternalPointerTag::kIcuListFormatterTag,
                47 => ExternalPointerTag::kIcuLocaleTag,
                48 => ExternalPointerTag::kIcuSimpleDateFormatTag,
                49 => ExternalPointerTag::kIcuDateIntervalFormatTag,
                50 => ExternalPointerTag::kIcuRelativeDateTimeFormatterTag,
                51 => ExternalPointerTag::kIcuLocalizedNumberFormatterTag,
                52 => ExternalPointerTag::kIcuPluralRulesTag,
                53 => ExternalPointerTag::kIcuCollatorTag,
                54 => ExternalPointerTag::kDisplayNamesInternalTag,
                55 => ExternalPointerTag::kD8WorkerTag,
                56 => ExternalPointerTag::kD8ModuleEmbedderDataTag,
                57 => ExternalPointerTag::kArrayBufferExtensionTag,
                0x7d => ExternalPointerTag::kExternalPointerZappedEntryTag,
                0x7e => ExternalPointerTag::kExternalPointerEvacuationEntryTag,
                0x7f => ExternalPointerTag::kExternalPointerFreeEntryTag,
                _ => ExternalPointerTag::kLastExternalPointerTag,
            }
        }
    }
    #[allow(non_upper_case_globals)]
    pub const kAnyExternalPointerTagRange: TagRange<ExternalPointerTag> =
        TagRange::new(ExternalPointerTag::kFirstExternalPointerTag, ExternalPointerTag::kLastExternalPointerTag);
    #[allow(non_upper_case_globals)]
    pub const kAnySharedExternalPointerTagRange: TagRange<ExternalPointerTag> = TagRange::new(
        ExternalPointerTag::kFirstSharedExternalPointerTag,
        ExternalPointerTag::kLastSharedExternalPointerTag,
    );
    #[allow(non_upper_case_globals)]
    pub const kAnyForeignExternalPointerTagRange: TagRange<ExternalPointerTag> = TagRange::new(
        ExternalPointerTag::kFirstForeignExternalPointerTag,
        ExternalPointerTag::kLastForeignExternalPointerTag,
    );
    #[allow(non_upper_case_globals)]
    pub const kAnyManagedExternalPointerTagRange: TagRange<ExternalPointerTag> = TagRange::new(
        ExternalPointerTag::kFirstManagedExternalPointerTag,
        ExternalPointerTag::kLastManagedExternalPointerTag,
    );
    #[allow(non_upper_case_globals)]
    pub const kAnyMaybeReadOnlyExternalPointerTagRange: TagRange<ExternalPointerTag> = TagRange::new(
        ExternalPointerTag::kFirstMaybeReadOnlyExternalPointerTag,
        ExternalPointerTag::kLastMaybeReadOnlyExternalPointerTag,
    );
    #[allow(non_upper_case_globals)]
    pub const kAnyManagedResourceExternalPointerTag: TagRange<ExternalPointerTag> = TagRange::new(
        ExternalPointerTag::kFirstManagedResourceTag,
        ExternalPointerTag::kLastManagedResourceTag,
    );

    #[inline]
    pub const fn IsSharedExternalPointerType(tag_range: TagRange<ExternalPointerTag>) -> bool {
        kAnySharedExternalPointerTagRange.Contains_range(tag_range)
    }

    #[inline]
    pub const fn IsMaybeReadOnlyExternalPointerType(
        tag_range: TagRange<ExternalPointerTag>,
    ) -> bool {
        kAnyMaybeReadOnlyExternalPointerTagRange.Contains_range(tag_range)
    }

    #[inline]
    pub const fn IsManagedExternalPointerType(tag_range: TagRange<ExternalPointerTag>) -> bool {
        kAnyManagedResourceExternalPointerTag.Contains_range(tag_range)
    }

    #[inline]
    pub const fn ExternalPointerCanBeEmpty(tag_range: TagRange<ExternalPointerTag>) -> bool {
        tag_range.Contains(ExternalPointerTag::kArrayBufferExtensionTag)
            || tag_range.Contains(ExternalPointerTag::kEmbedderDataSlotPayloadTag)
    }

    pub type IndirectPointerHandle = u32;

    pub const kNullIndirectPointerHandle: IndirectPointerHandle = 0;

    pub type TrustedPointerHandle = IndirectPointerHandle;

    pub const kTrustedPointerTableReservationSize: usize = 64 * MB as usize;

    pub const kTrustedPointerHandleShift: u32 = 9;

    pub const kNullTrustedPointerHandle: TrustedPointerHandle = kNullIndirectPointerHandle;

    pub const kTrustedPointerTableEntrySize: i32 = 8;
    pub const kTrustedPointerTableEntrySizeLog2: i32 = 3;
    pub const kMaxTrustedPointers: usize =
        kTrustedPointerTableReservationSize / kTrustedPointerTableEntrySize as usize;
    static_assert!(
        (1 << (32 - kTrustedPointerHandleShift)) as usize == kMaxTrustedPointers,
        "kTrustedPointerTableReservationSize and kTrustedPointerHandleShift don't match"
    );

    pub type CodePointerHandle = IndirectPointerHandle;

    pub const kCodePointerTableReservationSize: usize = 128 * MB as usize;

    pub const kCodePointerHandleShift: u32 = 9;

    pub const kNullCodePointerHandle: CodePointerHandle = kNullIndirectPointerHandle;

    pub const kCodePointerHandleMarker: u32 = 0x1;
    static_assert!(kCodePointerHandleShift > 0);
    static_assert!(kTrustedPointerHandleShift > 0);

    pub const kCodePointerTableEntrySize: i32 = 16;
    pub const kCodePointerTableEntrySizeLog2: i32 = 4;
    pub const kMaxCodePointers: usize =
        kCodePointerTableReservationSize / kCodePointerTableEntrySize as usize;
    static_assert!(
        (1 << (32 - kCodePointerHandleShift)) as usize == kMaxCodePointers,
        "kCodePointerTableReservationSize and kCodePointerHandleShift don't match"
    );

    pub const kCodePointerTableEntryEntrypointOffset: i32 = 0;
    pub const kCodePointerTableEntryCodeObjectOffset: i32 = 8;

    pub const kRuntimeGeneratedCodeObjectsLiveInTrustedSpace: bool = true;
    pub const kBuiltinCodeObjectsLiveInTrustedSpace: bool = false;
    pub const kAllCodeObjectsLiveInTrustedSpace: bool =
        kRuntimeGeneratedCodeObjectsLiveInTrustedSpace && kBuiltinCodeObjectsLiveInTrustedSpace;

    extern "C" {
        pub fn IsolateFromNeverReadOnlySpaceObject(obj: Address) -> *mut internal::Isolate;
    }

    extern "C" {
        pub fn ShouldThrowOnError(isolate: *mut internal::Isolate) -> bool;
    }

    pub struct Internals {}

    impl Internals {
        pub const kHeapObjectMapOffset: i32 = 0;
        pub const kMapInstanceTypeOffset: i32 = 1 * kApiTaggedSize as i32 + kApiInt32Size;
        pub const kStringResourceOffset: i32 =
            1 * kApiTaggedSize as i32 + 2 * kApiInt32Size;

        pub const kOddballKindOffset: i32 = 4 * kApiTaggedSize as i32 + kApiDoubleSize;
        pub const kJSObjectHeaderSize: i32 = 3 * kApiTaggedSize as i32;
        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        pub const kJSAPIObjectWithEmbedderSlotsHeaderSize: i32 =
            kJSObjectHeaderSize + kApiInt32Size;
        #[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
        pub const kJSAPIObjectWithEmbedderSlotsHeaderSize: i32 =
            kJSObjectHeaderSize + kApiTaggedSize as i32;
        pub const kFixedArrayHeaderSize: i32 = 2 * kApiTaggedSize as i32;
        pub const kEmbedderDataArrayHeaderSize: i32 = 2 * kApiTaggedSize as i32;
        pub const kEmbedderDataSlotSize: i32 = kApiSystemPointerSize as i32;
        #[cfg(feature = "V8_ENABLE_SANDBOX")]
        pub const kEmbedderDataSlotExternalPointerOffset: i32 = kApiTaggedSize as i32;
        #[cfg(not(feature = "V8_ENABLE_SANDBOX"))]
        pub const kEmbedderDataSlotExternalPointerOffset: i32 = 0;
        pub const kNativeContextEmbedderDataOffset: i32 = 6 * kApiTaggedSize as i32;
        pub const kStringRepresentationAndEncodingMask: i32 = 0x0f;
        pub const kStringEncodingMask: i32 = 0x8;
        pub const kExternalTwoByteRepresentationTag: i32 = 0x02;
        pub const kExternalOneByteRepresentationTag: i32 = 0x0a;

        pub const kNumIsolateDataSlots: u32 = 4;
        pub const kStackGuardSize: i32 = 8 * kApiSystemPointerSize as i32
