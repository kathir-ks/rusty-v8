// Converted from V8 C++ source files:
// Header: name.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub struct BitField<T, const START_BIT: usize, const SIZE: usize>;
impl<T, const START_BIT: usize, const SIZE: usize> BitField<T, START_BIT, SIZE> {
    pub fn encode(_value: T) -> u32 {
        0
    }
    pub fn kMask() -> u32 {
        0
    }
}
}
pub mod common {
pub mod globals {}
}
pub mod objects {
pub struct PrimitiveHeapObject {}
pub struct MaybeObject {}
pub struct Object {}
pub struct String {}
pub struct Heap {}
}
pub mod utils {
pub mod utils {}
}
pub mod torque_generated {
pub mod bit_fields {}
}
use std::sync::atomic::{AtomicU32, Ordering};
use std::marker::PhantomData;
pub struct SharedStringAccessGuardIfNeeded {}
pub struct Name {
    raw_hash_field_: AtomicU32,
}
impl Name {
    pub fn HasHashCode(&self) -> bool {
        self.TryGetHash(&mut 0)
    }
    pub fn HasForwardingIndex(&self, _arg: AcquireLoadTag) -> bool {
        Self::IsForwardingIndex(self.raw_hash_field(AcquireLoadTag))
    }
    pub fn HasInternalizedForwardingIndex(&self, _arg: AcquireLoadTag) -> bool {
        Self::IsInternalizedForwardingIndex(self.raw_hash_field(AcquireLoadTag))
    }
    pub fn HasExternalForwardingIndex(&self, _arg: AcquireLoadTag) -> bool {
        Self::IsExternalForwardingIndex(self.raw_hash_field(AcquireLoadTag))
    }
    pub fn raw_hash_field(&self) -> u32 {
        self.raw_hash_field_.load(Ordering::Relaxed)
    }
    pub fn raw_hash_field_AcquireLoadTag(&self) -> u32 {
        self.raw_hash_field_.load(Ordering::Acquire)
    }
    pub fn set_raw_hash_field(&self, hash: u32) {
        self.raw_hash_field_.store(hash, Ordering::Relaxed);
    }
    pub fn set_raw_hash_field_ReleaseStoreTag(&self, hash: u32) {
        self.raw_hash_field_.store(hash, Ordering::Release);
    }
    pub fn set_raw_hash_field_if_empty(&self, hash: u32) {
        let mut current = self.raw_hash_field_.load(Ordering::Relaxed);
        while current == Self::kEmptyHashField as u32 {
            match self.raw_hash_field_.compare_exchange_weak(
                current,
                hash,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(x) => current = x,
            }
        }
    }
    pub fn hash(&self) -> u32 {
        self.raw_hash_field()
    }
    pub fn TryGetHash(&self, hash: &mut u32) -> bool {
        let raw_hash = self.raw_hash_field();
        if Self::IsHash(raw_hash) {
            *hash = raw_hash;
            true
        } else {
            false
        }
    }
    pub fn Equals(&self, other: TaggedName) -> bool {
        let mut hash1 = 0;
        let mut hash2 = 0;
        if self.TryGetHash(&mut hash1) && other.TryGetHash(&mut hash2) {
            return hash1 == hash2;
        }
        false
    }
    pub fn Equals_static(isolate: &mut Isolate, one: DirectHandleName, two: DirectHandleName) -> bool {
        one.Equals(*two)
    }
    pub fn IsArrayIndex(&self) -> bool {
        let raw_hash = self.raw_hash_field();
        Self::IsIntegerIndex(raw_hash)
    }
    pub fn AsArrayIndex(&self, index: &mut u32) -> bool {
        let raw_hash = self.raw_hash_field();
        if Self::IsIntegerIndex(raw_hash) {
            *index = Self::ArrayIndexValueBits::decode(raw_hash);
            true
        } else {
            false
        }
    }
    pub fn AsIntegerIndex(&self, _index: &mut usize) -> bool {
        false
    }
    pub fn IsInteresting(&self, _isolate: &mut Isolate) -> bool {
        false
    }
    pub fn IsPrivate(&self) -> bool {
        false
    }
    pub fn IsPrivateName(&self) -> bool {
        false
    }
    pub fn IsPrivateBrand(&self) -> bool {
        false
    }
    pub fn ContainsCachedArrayIndex(_hash: u32) -> bool {
        false
    }
    pub fn ToFunctionName(
        _isolate: &mut Isolate,
        _name: DirectHandleName,
    ) -> Result<DirectHandleString, Error> {
        Err(Error::New)
    }
    pub fn ToFunctionName_prefix(
        _isolate: &mut Isolate,
        _name: DirectHandleName,
        _prefix: DirectHandleString,
    ) -> Result<DirectHandleString, Error> {
        Err(Error::New)
    }
    pub fn EnsureHash(&self) -> u32 {
        let mut hash = 0;
        if self.TryGetHash(&mut hash) {
            return hash;
        }
        0
    }
    pub fn EnsureHash_sharedstringaccessguardifneeded(
        &self,
        _: &SharedStringAccessGuardIfNeeded,
    ) -> u32 {
        let mut hash = 0;
        if self.TryGetHash(&mut hash) {
            return hash;
        }
        0
    }
    pub fn EnsureRawHash(&self) -> u32 {
        let mut hash = 0;
        if self.TryGetHash(&mut hash) {
            return hash;
        }
        0
    }
    pub fn EnsureRawHash_sharedstringaccessguardifneeded(
        &self,
        _: &SharedStringAccessGuardIfNeeded,
    ) -> u32 {
        let mut hash = 0;
        if self.TryGetHash(&mut hash) {
            return hash;
        }
        0
    }
    pub fn RawHash(&self) -> u32 {
        self.raw_hash_field()
    }
    pub fn IsHashFieldComputed(raw_hash_field: u32) -> bool {
        raw_hash_field & 1 == 0
    }
    pub fn IsHash(raw_hash_field: u32) -> bool {
        Self::HashFieldTypeBits::decode(raw_hash_field) == HashFieldType::kHash
    }
    pub fn IsIntegerIndex(raw_hash_field: u32) -> bool {
        Self::HashFieldTypeBits::decode(raw_hash_field) == HashFieldType::kIntegerIndex
    }
    pub fn IsForwardingIndex(raw_hash_field: u32) -> bool {
        Self::HashFieldTypeBits::decode(raw_hash_field) == HashFieldType::kForwardingIndex
    }
    pub fn IsInternalizedForwardingIndex(raw_hash_field: u32) -> bool {
        Self::IsInternalizedForwardingIndexBit::decode(raw_hash_field)
    }
    pub fn IsExternalForwardingIndex(raw_hash_field: u32) -> bool {
        Self::IsExternalForwardingIndexBit::decode(raw_hash_field)
    }
    pub fn CreateHashFieldValue(hash: u32, type_: HashFieldType) -> u32 {
        Self::HashFieldTypeBits::encode(type_) | hash
    }
    pub fn CreateInternalizedForwardingIndex(index: u32) -> u32 {
        Self::IsInternalizedForwardingIndexBit::encode(true) | index
    }
    pub fn CreateExternalForwardingIndex(index: u32) -> u32 {
        Self::IsExternalForwardingIndexBit::encode(true) | index
    }
    fn GetRawHashFromForwardingTable(&self, raw_hash: u32) -> u32 {
        raw_hash
    }
}
pub fn IsUniqueName(obj: TaggedName) -> bool {
    IsUniqueName_PtrComprCageBase(obj, PtrComprCageBase {})
}
pub fn IsUniqueName_PtrComprCageBase(_obj: TaggedName, _cage_base: PtrComprCageBase) -> bool {
    false
}
pub struct Symbol {
    flags_: u32,
    description_: TaggedMemberPrimitiveHeapObject,
}
impl Symbol {
    pub fn description(&self) -> TaggedPrimitiveHeapObject {
        self.description_.value()
    }
    pub fn set_description(&self, value: TaggedPrimitiveHeapObject, _mode: WriteBarrierMode) {
        self.description_.set(value);
    }
    pub fn is_private(&self) -> bool {
        Self::IsPrivateBit::decode(self.flags_)
    }
    pub fn set_is_private(&self, value: bool) {
        self.flags_ = Self::IsPrivateBit::update(value, self.flags_);
    }
    pub fn is_well_known_symbol(&self) -> bool {
        Self::IsWellKnownSymbolBit::decode(self.flags_)
    }
    pub fn set_is_well_known_symbol(&self, value: bool) {
        self.flags_ = Self::IsWellKnownSymbolBit::update(value, self.flags_);
    }
    pub fn is_interesting_symbol(&self) -> bool {
        Self::IsInterestingSymbolBit::decode(self.flags_)
    }
    pub fn set_is_interesting_symbol(&self, value: bool) {
        self.flags_ = Self::IsInterestingSymbolBit::update(value, self.flags_);
    }
    pub fn is_in_public_symbol_table(&self) -> bool {
        Self::IsInPublicSymbolTableBit::decode(self.flags_)
    }
    pub fn set_is_in_public_symbol_table(&self, value: bool) {
        self.flags_ = Self::IsInPublicSymbolTableBit::update(value, self.flags_);
    }
    pub fn is_private_name(&self) -> bool {
        Self::IsPrivateNameBit::decode(self.flags_)
    }
    pub fn set_is_private_name(&mut self) {
        self.set_is_private(true);
        self.flags_ = Self::IsPrivateNameBit::update(true, self.flags_);
    }
    pub fn is_private_brand(&self) -> bool {
        Self::IsPrivateBrandBit::decode(self.flags_)
    }
    pub fn set_is_private_brand(&mut self) {
        self.set_is_private(true);
        self.flags_ = Self::IsPrivateBrandBit::update(true, self.flags_);
    }
    fn PrivateSymbolToName(&self) -> &str {
        ""
    }
}
pub struct ObjectTraitsSymbol {}
pub struct OffsetsForDebug {}
pub struct FixedBodyDescriptor<const A: usize, const B: usize, const C: usize>;
pub struct TaggedMember<T> {
    dummy: i32,
    phantom: PhantomData<T>,
}
impl<T> TaggedMember<T> {
    pub fn value(&self) -> T {
        panic!()
    }
    pub fn set(&self, _value: T) {}
}
pub enum class HashFieldType: u32 {
    kHash = 0b10,
    kIntegerIndex = 0b00,
    kForwardingIndex = 0b01,
    kEmpty = 0b11,
}
pub struct PtrComprCageBase {}
pub struct Isolate {}
pub struct DirectHandle<T>(PhantomData<T>);
impl<T> DirectHandle<T> {
    fn Equals(&self, _other: Self) -> bool {
        false
    }
}
pub struct IndirectHandle<T>(PhantomData<T>);
pub struct Tagged<T>(PhantomData<T>);
pub struct WriteBarrierMode {}
pub struct AcquireLoadTag {}
pub struct ReleaseStoreTag {}
pub struct Error {
    dummy: i32,
}
impl Error {
    pub fn New() -> Error {
        Error { dummy: 0 }
    }
}
impl From<std::io::Error> for Error {
    fn from(_error: std::io::Error) -> Self {
        Error::New()
    }
}
pub struct Factory;
pub struct CodeStubAssembler;
pub struct StringBuiltinsAssembler;
pub struct SandboxTesting;
pub mod maglev {
pub struct MaglevGraphBuilder;
pub struct MaglevAssembler;
}
pub mod compiler {
pub struct AccessBuilder;
pub struct WasmGraphBuilder;
}
pub type DirectHandleString = DirectHandle<String>;
pub type DirectHandleName = DirectHandle<Name>;
pub type TaggedName = Tagged<Name>;
pub type TaggedPrimitiveHeapObject = Tagged<PrimitiveHeapObject>;
pub struct TaggedMemberPrimitiveHeapObject(TaggedMember<PrimitiveHeapObject>);
impl TaggedMemberPrimitiveHeapObject {
    pub fn set(&self, _value: TaggedPrimitiveHeapObject) {}
    pub fn value(&self) -> TaggedPrimitiveHeapObject {
        TaggedPrimitiveHeapObject(PhantomData)
    }
}
const kBitsPerInt: usize = 32;
impl Name {
    pub struct HashFieldTypeBits;
    pub struct HashBits;
    pub const kHashNotComputedMask: i32 = 1;
    pub const kEmptyHashField: i32 = Self::HashFieldTypeBits::encode(HashFieldType::kEmpty) as i32;
    pub struct IsInternalizedForwardingIndexBit;
    pub struct IsExternalForwardingIndexBit;
    pub struct ForwardingIndexValueBits;
    pub const kMaxCachedArrayIndexLength: i32 = 7;
    pub const kMaxArrayIndex: u32 = u32::MAX - 1;
    pub const kMaxArrayIndexSize: i32 = 10;
    pub const kArrayIndexValueBits: i32 = 24;
    pub const kArrayIndexLengthBits: i32 = kBitsPerInt as i32 - Self::kArrayIndexValueBits - 2;
    pub struct ArrayIndexValueBits;
    pub struct ArrayIndexLengthBits;
    pub const kDoesNotContainCachedArrayIndexMask: u32 =
        (!Self::kMaxCachedArrayIndexLength as u32 << Self::ArrayIndexLengthBits::kShift)
            | Self::HashFieldTypeBits::kMask();
    pub const kDoesNotContainIntegerOrForwardingIndexMask: u32 = 0b10;
}
fn TenToThe(exponent: i32) -> u32 {
    match exponent {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        4 => 10000,
        5 => 100000,
        6 => 1000000,
        7 => 10000000,
        8 => 100000000,
        9 => 1000000000,
        _ => panic!("Exponent too large"),
    }
}
impl Symbol {
    pub struct IsPrivateBit;
    pub struct IsWellKnownSymbolBit;
    pub struct IsInPublicSymbolTableBit;
    pub struct IsInterestingSymbolBit;
    pub struct IsPrivateNameBit;
    pub struct IsPrivateBrandBit;
}
impl<T, const START_BIT: usize, const SIZE: usize> Name::HashFieldTypeBits {
    const kSize: i32 = 2;
    const kShift: i32 = 0;
    pub fn encode(value: HashFieldType) -> u32 {
        value as u32
    }
    pub fn decode(value: u32) -> HashFieldType {
        match value & 0b11 {
            0b10 => HashFieldType::kHash,
            0b00 => HashFieldType::kIntegerIndex,
            0b01 => HashFieldType::kForwardingIndex,
            0b11 => HashFieldType::kEmpty,
            _ => panic!("Invalid HashFieldType"),
        }
    }
    pub fn kMask() -> u32 {
        0b11
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Name::IsInternalizedForwardingIndexBit {
    const kSize: i32 = 1;
    const kShift: i32 = 2;
    pub fn encode(value: bool) -> u32 {
        if value {
            1 << Self::kShift
        } else {
            0
        }
    }
    pub fn decode(value: u32) -> bool {
        (value >> Self::kShift) & 1 != 0
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Name::IsExternalForwardingIndexBit {
    const kSize: i32 = 1;
    const kShift: i32 = 3;
    pub fn encode(value: bool) -> u32 {
        if value {
            1 << Self::kShift
        } else {
            0
        }
    }
    pub fn decode(value: u32) -> bool {
        (value >> Self::kShift) & 1 != 0
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Name::ArrayIndexValueBits {
    const kSize: i32 = 24;
    const kShift: i32 = 2;
    pub fn encode(value: u32) -> u32 {
        value << Self::kShift
    }
    pub fn decode(value: u32) -> u32 {
        (value >> Self::kShift) & ((1 << Self::kSize) - 1)
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Name::ArrayIndexLengthBits {
    const kSize: i32 = kBitsPerInt as i32 - Name::ArrayIndexValueBits::kSize - 2;
    const kShift: i32 = Name::ArrayIndexValueBits::kShift + Name::ArrayIndexValueBits::kSize;
    pub fn encode(value: u32) -> u32 {
        value << Self::kShift
    }
    pub fn decode(value: u32) -> u32 {
        (value >> Self::kShift) & ((1 << Self::kSize) - 1)
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Symbol::IsPrivateBit {
    const kSize: i32 = 1;
    const kShift: i32 = 0;
    pub fn encode(value: bool) -> u32 {
        if value {
            1 << Self::kShift
        } else {
            0
        }
    }
    pub fn decode(value: u32) -> bool {
        (value >> Self::kShift) & 1 != 0
    }
    pub fn update(value: bool, flags: u32) -> u32 {
        let mask = 1 << Self::kShift;
        if value {
            flags | mask
        } else {
            flags & !mask
        }
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Symbol::IsWellKnownSymbolBit {
    const kSize: i32 = 1;
    const kShift: i32 = 1;
    pub fn encode(value: bool) -> u32 {
        if value {
            1 << Self::kShift
        } else {
            0
        }
    }
    pub fn decode(value: u32) -> bool {
        (value >> Self::kShift) & 1 != 0
    }
    pub fn update(value: bool, flags: u32) -> u32 {
        let mask = 1 << Self::kShift;
        if value {
            flags | mask
        } else {
            flags & !mask
        }
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Symbol::IsInPublicSymbolTableBit {
    const kSize: i32 = 1;
    const kShift: i32 = 2;
    pub fn encode(value: bool) -> u32 {
        if value {
            1 << Self::kShift
        } else {
            0
        }
    }
    pub fn decode(value: u32) -> bool {
        (value >> Self::kShift) & 1 != 0
    }
    pub fn update(value: bool, flags: u32) -> u32 {
        let mask = 1 << Self::kShift;
        if value {
            flags | mask
        } else {
            flags & !mask
        }
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Symbol::IsInterestingSymbolBit {
    const kSize: i32 = 1;
    const kShift: i32 = 3;
    pub fn encode(value: bool) -> u32 {
        if value {
            1 << Self::kShift
        } else {
            0
        }
    }
    pub fn decode(value: u32) -> bool {
        (value >> Self::kShift) & 1 != 0
    }
    pub fn update(value: bool, flags: u32) -> u32 {
        let mask = 1 << Self::kShift;
        if value {
            flags | mask
        } else {
            flags & !mask
        }
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Symbol::IsPrivateNameBit {
    const kSize: i32 = 1;
    const kShift: i32 = 4;
    pub fn encode(value: bool) -> u32 {
        if value {
            1 << Self::kShift
        } else {
            0
        }
    }
    pub fn decode(value: u32) -> bool {
        (value >> Self::kShift) & 1 != 0
    }
    pub fn update(value: bool, flags: u32) -> u32 {
        let mask = 1 << Self::kShift;
        if value {
            flags | mask
        } else {
            flags & !mask
        }
    }
}
impl<T, const START_BIT: usize, const SIZE: usize> Symbol::IsPrivateBrandBit {
    const kSize: i32 = 1;
    const kShift: i32 = 5;
    pub fn encode(value: bool) -> u32 {
        if value {
            1 << Self::kShift
        } else {
            0
        }
    }
    pub fn decode(value: u32) -> bool {
        (value >> Self::kShift) & 1 != 0
    }
    pub fn update(value: bool, flags: u32) -> u32 {
        let mask = 1 << Self::kShift;
        if value {
            flags | mask
        } else {
            flags & !mask
        }
    }
}
