// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod turboshaft {
    use std::cmp::Ordering;
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::marker::PhantomData;
    use std::mem;
    use std::num::ParseIntError;
    use std::ops::{Deref, DerefMut};
    use std::any::Any;
    

    // Operations are stored in possibly multiple sequential storage slots.
    pub type OperationStorageSlot = u64;
    // Operations occupy at least 2 slots, therefore we assign one id per two slots.
    pub const SLOTS_PER_ID: usize = 2;

    // TODO: Define ConstOrV (and other types) stubs as needed. For now, they are commented out to allow the module to compile.
    // template <typename T, typename C>
    // class ConstOrV;

    /// `OpIndex` is an offset from the beginning of the operations buffer.
    /// Compared to `Operation*`, it is more memory efficient (32bit) and stable when
    /// the operations buffer is re-allocated.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpIndex {
        offset: u32,
    }

    impl OpIndex {
        pub const fn from_offset(offset: u32) -> Self {
            OpIndex { offset }
        }

        pub const fn new() -> Self {
            OpIndex {
                offset: u32::MAX,
            }
        }

        pub fn id(&self) -> u32 {
            // Operations are stored at an offset that's a multiple of
            // `sizeof(OperationStorageSlot)`. In addition, an operation occupies at
            // least `kSlotsPerId` many `OperationSlot`s. Therefore, we can assign id's
            // by dividing by `kSlotsPerId`. A compact id space is important, because it
            // makes side-tables smaller.
            debug_assert!(self.check_invariants());
            self.offset / mem::size_of::<OperationStorageSlot>() as u32 / SLOTS_PER_ID as u32
        }

        pub fn hash(&self) -> u32 {
            // It can be useful to hash OpIndex::Invalid(), so we have this `hash`
            // function, which returns the id, but without DCHECKing that Invalid is
            // valid.
            if self.valid() {
                debug_assert!(self.check_invariants());
            }
            self.offset / mem::size_of::<OperationStorageSlot>() as u32 / SLOTS_PER_ID as u32
        }

        pub fn offset(&self) -> u32 {
            debug_assert!(self.check_invariants());
            #[cfg(debug_assertions)]
            {
                self.offset & Self::UNMASK_GENERATION_MASK
            }
            #[cfg(not(debug_assertions))]
            {
                self.offset
            }
        }

        pub const fn valid(&self) -> bool {
            *self != Self::invalid()
        }

        pub const fn invalid() -> Self {
            OpIndex::new()
        }

        // Encode a sea-of-nodes node id in the `OpIndex` type.
        // Only used for node origins that actually point to sea-of-nodes graph nodes.
        pub fn encode_turbofan_node_id(id: u32) -> Self {
            let mut result = OpIndex::from_offset(id * mem::size_of::<OperationStorageSlot>() as u32);
            result.offset += Self::TURBOFAN_NODE_ID_FLAG;
            result
        }

        pub fn decode_turbofan_node_id(&self) -> u32 {
            debug_assert!(self.is_turbofan_node_id());
            self.offset / mem::size_of::<OperationStorageSlot>() as u32
        }

        pub fn is_turbofan_node_id(&self) -> bool {
            self.offset % mem::size_of::<OperationStorageSlot>() as u32 == Self::TURBOFAN_NODE_ID_FLAG
        }

        #[cfg(debug_assertions)]
        fn generation_mod2(&self) -> i32 {
            ((self.offset & Self::GENERATION_MASK) >> Self::GENERATION_MASK_SHIFT) as i32
        }

        #[cfg(debug_assertions)]
        fn set_generation_mod2(&mut self, generation_mod2: i32) {
            debug_assert!(generation_mod2 <= 1);
            self.offset |= (generation_mod2 as u32) << Self::GENERATION_MASK_SHIFT;
        }

        #[cfg(debug_assertions)]
        const fn check_invariants(&self) -> bool {
            if !self.valid() {
                return true;
            }
            // The second lowest significant bit of the offset is used to store the
            // graph generation modulo 2. The lowest and 3rd lowest bits should always
            // be 0 (as long as sizeof(OperationStorageSlot) is 8).
            debug_assert!(mem::size_of::<OperationStorageSlot>() == 8);
            (self.offset & 0b101) == 0
        }

        const GENERATION_MASK_SHIFT: u32 = 1;
        const GENERATION_MASK: u32 = 1 << Self::GENERATION_MASK_SHIFT;
        const UNMASK_GENERATION_MASK: u32 = !Self::GENERATION_MASK;

        const TURBOFAN_NODE_ID_FLAG: u32 = 1;
    }

    impl fmt::Display for OpIndex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.offset)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OptionalOpIndex(OpIndex);

    impl OptionalOpIndex {
        pub const fn new(other: OpIndex) -> Self {
            OptionalOpIndex(other)
        }

        pub const fn nullopt() -> Self {
            OptionalOpIndex { 0: OpIndex::invalid() }
        }

        pub fn hash(&self) -> u32 {
            self.0.hash()
        }

        pub const fn has_value(&self) -> bool {
            self.valid()
        }

        pub const fn value(&self) -> OpIndex {
            debug_assert!(self.has_value());
            self.0
        }

        pub const fn value_or_invalid(&self) -> OpIndex {
            self.0
        }
    }

    impl From<OpIndex> for OptionalOpIndex {
        fn from(op_index: OpIndex) -> Self {
            OptionalOpIndex(op_index)
        }
    }
    
    impl Deref for OptionalOpIndex {
        type Target = OpIndex;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl DerefMut for OptionalOpIndex {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl fmt::Display for OptionalOpIndex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value_or_invalid())
        }
    }

    // Dummy value for abstract representation classes that don't have a
    // RegisterRepresentation.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NullRep;
    pub const NULLREP: NullRep = NullRep;

    impl PartialEq<RegisterRepresentation> for NullRep {
        fn eq(&self, _other: &RegisterRepresentation) -> bool {
            false
        }
    }

    impl PartialEq<NullRep> for RegisterRepresentation {
        fn eq(&self, _other: &NullRep) -> bool {
            false
        }
    }

    // Abstract tag classes for V<>.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Any;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct None;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WordWithBits<const BITS: usize>;
    impl<const BITS: usize> WordWithBits<BITS> {
      //This is a little hacky. It does guarantee that there are only a few allowed bit sizes
      pub const fn new() -> Self {
        assert!(BITS == 32 || BITS == 64 || BITS == 128 || BITS == 256);
        Self {}
      }
    }

    pub type Word32 = WordWithBits<32>;
    pub type Word64 = WordWithBits<64>;
    #[cfg(target_pointer_width = "64")]
    pub type WordPtr = Word64;
    #[cfg(target_pointer_width = "32")]
    pub type WordPtr = Word32;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FloatWithBits<const BITS: usize>;
    impl<const BITS: usize> FloatWithBits<BITS> {
      pub const fn new() -> Self {
        assert!(BITS == 32 || BITS == 64);
        Self {}
      }
    }

    pub type Float32 = FloatWithBits<32>;
    pub type Float64 = FloatWithBits<64>;

    pub type Simd128 = WordWithBits<128>;
    pub type Simd256 = WordWithBits<256>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Compressed;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct InternalTag;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FrameState;

    // A Union type for untagged values. For Tagged types use `Union` for now.
    // TODO(nicohartmann@): We should think about a more uniform solution some day.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UntaggedUnion<T>(PhantomData<T>);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Tuple<T>(PhantomData<T>);

    // Traits classes `v_traits<T>` to provide additional T-specific information for
    // V<T> and ConstOrV<T>. If you need to provide non-default conversion behavior
    // for a specific type, specialize the corresponding v_traits<>.
    pub trait VTraits {
        const IS_ABSTRACT_TAG: bool;
        type RepType;
        const REP: NullRep; //TODO: use a proper representation type here
        fn allows_representation(_maybe_allowed_rep: RegisterRepresentation) -> bool;

        //TODO: impl ImplicitlyConstructibleFrom
        // template <typename U>
        // struct implicitly_constructible_from : std::true_type {};
    }

    pub struct AnyVTraits;
    impl VTraits for Any {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = RegisterRepresentation;
        const REP: NullRep = NULLREP;
        fn allows_representation(_maybe_allowed_rep: RegisterRepresentation) -> bool {
            true
        }
    }

    pub struct NoneVTraits;
    impl VTraits for None {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = NullRep;
        const REP: NullRep = NULLREP;
        fn allows_representation(_maybe_allowed_rep: RegisterRepresentation) -> bool {
            false
        }
    }

    pub struct CompressedVTraits;
    impl VTraits for Compressed {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = RegisterRepresentation;
        const REP: NullRep = NULLREP; // TODO: change to RegisterRepresentation::Compressed();
        fn allows_representation(maybe_allowed_rep: RegisterRepresentation) -> bool {
            maybe_allowed_rep == RegisterRepresentation::Compressed
        }
    }

    pub struct Word32VTraits;
    impl VTraits for Word32 {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = WordRepresentation;
        const REP: NullRep = NULLREP; // TODO: change to WordRepresentation::Word32();
        fn allows_representation(maybe_allowed_rep: RegisterRepresentation) -> bool {
            maybe_allowed_rep == RegisterRepresentation::Word32
        }
    }

    pub struct Word64VTraits;
    impl VTraits for Word64 {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = WordRepresentation;
        const REP: NullRep = NULLREP; // TODO: change to WordRepresentation::Word64();
        fn allows_representation(maybe_allowed_rep: RegisterRepresentation) -> bool {
            maybe_allowed_rep == RegisterRepresentation::Word64
        }
    }

    pub struct Float32VTraits;
    impl VTraits for Float32 {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = FloatRepresentation;
        const REP: NullRep = NULLREP; // TODO: change to FloatRepresentation::Float32();
        fn allows_representation(maybe_allowed_rep: RegisterRepresentation) -> bool {
            maybe_allowed_rep == RegisterRepresentation::Float32
        }
    }

    pub struct Float64VTraits;
    impl VTraits for Float64 {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = FloatRepresentation;
        const REP: NullRep = NULLREP; // TODO: change to FloatRepresentation::Float64();
        fn allows_representation(maybe_allowed_rep: RegisterRepresentation) -> bool {
            maybe_allowed_rep == RegisterRepresentation::Float64
        }
    }

    pub struct Simd128VTraits;
    impl VTraits for Simd128 {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = RegisterRepresentation;
        const REP: NullRep = NULLREP; // TODO: change to RegisterRepresentation::Simd128();
        fn allows_representation(maybe_allowed_rep: RegisterRepresentation) -> bool {
            maybe_allowed_rep == RegisterRepresentation::Simd128
        }
    }

    pub struct Simd256VTraits;
    impl VTraits for Simd256 {
        const IS_ABSTRACT_TAG: bool = true;
        type RepType = RegisterRepresentation;
        const REP: NullRep = NULLREP; // TODO: change to RegisterRepresentation::Simd256();
        fn allows_representation(maybe_allowed_rep: RegisterRepresentation) -> bool {
            maybe_allowed_rep == RegisterRepresentation::Simd256
        }
    }

    // TODO: Implement the rest of the v_traits specializations

    // TODO: Implement the detail namespace

    // TODO: Implement ShadowyOpIndex and ShadowyOpIndexVectorWrapper

    /// `BlockIndex` is the index of a bound block.
    /// A dominating block always has a smaller index.
    /// It corresponds to the ordering of basic blocks in the operations buffer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BlockIndex {
        id: u32,
    }

    impl BlockIndex {
        pub const fn new(id: u32) -> Self {
            BlockIndex { id }
        }

        pub const fn default() -> Self {
            BlockIndex {
                id: u32::MAX,
            }
        }

        pub fn id(&self) -> u32 {
            self.id
        }

        pub const fn valid(&self) -> bool {
            *self != Self::invalid()
        }

        pub const fn invalid() -> Self {
            BlockIndex::default()
        }
    }

    impl fmt::Display for BlockIndex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.id)
        }
    }

    // TODO: Implement DEFINE_STRONG_ORDERING_COMPARISON macro

    // Dummy types to make the code compile
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum RegisterRepresentation {
        Word32,
        Word64,
        Float32,
        Float64,
        Tagged,
        Compressed,
        Simd128,
        Simd256
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum WordRepresentation {
        Word32,
        Word64
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FloatRepresentation {
        Float32,
        Float64
    }

    pub type Boolean = bool;
    pub type Null = ();
    pub type Undefined = ();
    pub type Number = f64;
    pub type String = std::string::String;
    pub type FixedArray = Vec<u8>;
    pub type FixedDoubleArray = Vec<f64>;
    pub type Symbol = u32;
    pub type BigInt = i128;
    pub type Code = u32;
    pub type JSFunction = u32;
    pub type Object = u32; //This should probably be some smart pointer type
    pub type WasmArray = Vec<u8>;
    pub type WasmNull = ();
    pub type WasmStruct = Vec<u8>;

    pub fn is_subtype_v<T,U>() -> bool {
        true //TODO: Properly implement this
    }
    pub type UnionOf<T, U> = (T, U);
    
    
    pub const K_SIMD128_SIZE: usize = 16;
    pub const K_SIMD256_SIZE: usize = 32;
    
    pub fn is_64() -> bool {
        std::mem::size_of::<usize>() == 8
    }
    
    // V<> represents an SSA-value that is parameterized with the type of the value.
    // Types from the `Object` hierarchy can be provided as well as the abstract
    // representation classes (`Word32`, ...) defined above.
    // Prefer using V<> instead of a plain OpIndex where possible.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct V<T> {
        index: OpIndex,
        _phantom: PhantomData<T>,
    }
    
    impl<T> V<T> {
        pub fn new(index: OpIndex) -> Self {
            V {
                index,
                _phantom: PhantomData,
            }
        }
    
        pub fn invalid() -> Self {
            V {
                index: OpIndex::invalid(),
                _phantom: PhantomData,
            }
        }
    
        pub fn cast<U>(index: V<U>) -> Self {
            V {
                index: index.index,
                _phantom: PhantomData,
            }
        }
    
        pub fn cast_from_opindex(index: OpIndex) -> Self {
            V {
                index,
                _phantom: PhantomData,
            }
        }
    
        pub fn allows_representation(_maybe_allowed_rep: RegisterRepresentation) -> bool {
            true //TODO: Properly implement this
        }
    
    }
    
    impl<T> Deref for V<T> {
        type Target = OpIndex;
    
        fn deref(&self) -> &Self::Target {
            &self.index
        }
    }
    
    impl<T> DerefMut for V<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.index
        }
    }
    
    // OptionalV represents an optional SSA-value that is parameterized with the type of the value.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OptionalV<T> {
        index: OptionalOpIndex,
        _phantom: PhantomData<T>,
    }
    
    impl<T> OptionalV<T> {
        pub fn new(index: OptionalOpIndex) -> Self {
            OptionalV {
                index,
                _phantom: PhantomData,
            }
        }
    
        pub fn nullopt() -> Self {
            OptionalV {
                index: OptionalOpIndex::nullopt(),
                _phantom: PhantomData,
            }
        }
    
        pub fn value(&self) -> V<T> {
            V::cast_from_opindex(self.index.value())
        }
    
        pub fn value_or_invalid(&self) -> V<T> {
            V::cast_from_opindex(self.index.value_or_invalid())
        }
    
        pub fn cast<U>(index: OptionalV<U>) -> Self {
            OptionalV {
                index: index.index,
                _phantom: PhantomData,
            }
        }
    
        pub fn cast_from_optional_opindex(index: OptionalOpIndex) -> Self {
            OptionalV {
                index,
                _phantom: PhantomData,
            }
        }
    }
    
    impl<T> Deref for OptionalV<T> {
        type Target = OptionalOpIndex;
    
        fn deref(&self) -> &Self::Target {
            &self.index
        }
    }
    
    impl<T> DerefMut for OptionalV<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.index
        }
    }
    
    impl<T> From<V<T>> for OptionalV<T> {
        fn from(v: V<T>) -> Self {
            OptionalV {
                index: OptionalOpIndex::from(v.index),
                _phantom: PhantomData,
            }
        }
    }
    
}