// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod machine_operator {
    use std::{
        fmt,
        hash::{Hash, Hasher},
        marker::PhantomData,
        mem,
        ops::{BitOr, BitOrAssign},
    };

    use crate::codegen::{atomic_memory_order::AtomicMemoryOrder, machine_type::MachineRepresentation, machine_type::MachineType};
    use crate::compiler::globals::kSimd128Size;
    use crate::compiler::write_barrier_kind::WriteBarrierKind;

    pub struct Operator {}

    /// For operators that are not supported on all platforms.
    pub struct OptionalOperator<'a> {
        supported: bool,
        op: &'a Operator,
    }

    impl<'a> OptionalOperator<'a> {
        pub fn new(supported: bool, op: &'a Operator) -> Self {
            OptionalOperator { supported, op }
        }

        pub fn is_supported(&self) -> bool {
            self.supported
        }

        // Gets the operator only if it is supported.
        pub fn op(&self) -> &'a Operator {
            assert!(self.supported);
            self.op
        }

        // Always gets the operator, even for unsupported operators. This is useful to
        // use the operator as a placeholder in a graph, for instance.
        pub fn placeholder(&self) -> &'a Operator {
            self.op
        }
    }

    /// A Load needs a MachineType.
    pub type LoadRepresentation = MachineType;

    pub fn load_representation_of(_op: &Operator) -> LoadRepresentation {
        unimplemented!()
    }

    /// A Word(32|64)AtomicLoad needs both a LoadRepresentation and a memory
    /// order.
    #[derive(Clone, Copy)]
    pub struct AtomicLoadParameters {
        representation: LoadRepresentation,
        order: AtomicMemoryOrder,
        kind: MemoryAccessKind,
    }

    impl AtomicLoadParameters {
        pub fn new(
            representation: LoadRepresentation,
            order: AtomicMemoryOrder,
            kind: MemoryAccessKind,
        ) -> Self {
            AtomicLoadParameters {
                representation,
                order,
                kind,
            }
        }

        pub fn representation(&self) -> LoadRepresentation {
            self.representation
        }
        pub fn order(&self) -> AtomicMemoryOrder {
            self.order
        }
        pub fn kind(&self) -> MemoryAccessKind {
            self.kind
        }
    }

    impl PartialEq for AtomicLoadParameters {
        fn eq(&self, other: &Self) -> bool {
            self.representation == other.representation
                && self.order == other.order
                && self.kind == other.kind
        }
    }

    impl Eq for AtomicLoadParameters {}

    impl Hash for AtomicLoadParameters {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.representation.hash(state);
            self.order.hash(state);
            self.kind.hash(state);
        }
    }

    impl fmt::Display for AtomicLoadParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "AtomicLoadParameters {{ representation: {:?}, order: {:?}, kind: {:?} }}",
                self.representation, self.order, self.kind
            )
        }
    }

    pub fn atomic_load_parameters_of(_op: &Operator) -> AtomicLoadParameters {
        unimplemented!()
    }

    #[derive(Clone, Copy)]
    pub struct AtomicOpParameters {
        r#type: MachineType,
        kind: MemoryAccessKind,
    }

    impl AtomicOpParameters {
        pub fn new(r#type: MachineType, kind: MemoryAccessKind) -> Self {
            AtomicOpParameters { r#type, kind }
        }

        pub fn r#type(&self) -> MachineType {
            self.r#type
        }
        pub fn kind(&self) -> MemoryAccessKind {
            self.kind
        }
    }

    impl PartialEq for AtomicOpParameters {
        fn eq(&self, other: &Self) -> bool {
            self.r#type == other.r#type && self.kind == other.kind
        }
    }

    impl Eq for AtomicOpParameters {}

    impl Hash for AtomicOpParameters {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.r#type.hash(state);
            self.kind.hash(state);
        }
    }

    impl fmt::Display for AtomicOpParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "AtomicOpParameters {{ type: {:?}, kind: {:?} }}",
                self.r#type, self.kind
            )
        }
    }

    pub fn atomic_op_parameters_of(_op: &Operator) -> AtomicOpParameters {
        unimplemented!()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum LoadTransformation {
        // 128-bit LoadSplats must be first.
        S128Load8Splat,
        S128Load16Splat,
        S128Load32Splat,
        S128Load64Splat,
        // 128-bit LoadExtend.
        S128Load8x8S,
        S128Load8x8U,
        S128Load16x4S,
        S128Load16x4U,
        S128Load32x2S,
        S128Load32x2U,
        S128Load32Zero,
        S128Load64Zero,
        // 256-bit transformations must be last.
        S256Load8Splat,
        S256Load16Splat,
        S256Load32Splat,
        S256Load64Splat,
        S256Load8x16S,
        S256Load8x16U,
        S256Load8x8U,
        S256Load16x8S,
        S256Load16x8U,
        S256Load32x4S,
        S256Load32x4U,
    }

    impl fmt::Display for LoadTransformation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct LoadTransformParameters {
        pub kind: MemoryAccessKind,
        pub transformation: LoadTransformation,
    }

    impl fmt::Display for LoadTransformParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "LoadTransformParameters {{ kind: {:?}, transformation: {:?} }}",
                self.kind, self.transformation
            )
        }
    }

    pub fn load_transform_parameters_of(_op: &Operator) -> &LoadTransformParameters {
        unimplemented!()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct LoadLaneParameters {
        pub kind: MemoryAccessKind,
        pub rep: LoadRepresentation,
        pub laneidx: u8,
    }

    impl fmt::Display for LoadLaneParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "LoadLaneParameters {{ kind: {:?}, rep: {:?}, laneidx: {} }}",
                self.kind, self.rep, self.laneidx
            )
        }
    }

    pub fn load_lane_parameters_of(_op: &Operator) -> &LoadLaneParameters {
        unimplemented!()
    }

    /// A Store needs a MachineType and a WriteBarrierKind in order to emit the
    /// correct write barrier, and needs to state whether it is storing into the
    /// header word, so that the value can be packed, if necessary.
    #[derive(Clone, Copy)]
    pub struct StoreRepresentation {
        representation: MachineRepresentation,
        write_barrier_kind: WriteBarrierKind,
    }

    impl StoreRepresentation {
        pub fn new(representation: MachineRepresentation, write_barrier_kind: WriteBarrierKind) -> Self {
            StoreRepresentation {
                representation,
                write_barrier_kind,
            }
        }

        pub fn representation(&self) -> MachineRepresentation {
            self.representation
        }
        pub fn write_barrier_kind(&self) -> WriteBarrierKind {
            self.write_barrier_kind
        }
    }

    #[derive(Clone, Copy)]
    pub struct StorePairRepresentation {
        first: StoreRepresentation,
        second: StoreRepresentation,
    }

    impl StorePairRepresentation {
        pub fn new(first: StoreRepresentation, second: StoreRepresentation) -> Self {
            StorePairRepresentation { first, second }
        }
    }

    impl fmt::Display for StorePairRepresentation {
        fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(
                out,
                "StorePairRepresentation {{ first: {}, second: {} }}",
                self.first, self.second
            )
        }
    }

    impl PartialEq for StoreRepresentation {
        fn eq(&self, other: &Self) -> bool {
            self.representation == other.representation
                && self.write_barrier_kind == other.write_barrier_kind
        }
    }

    impl Eq for StoreRepresentation {}

    impl Hash for StoreRepresentation {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.representation.hash(state);
            self.write_barrier_kind.hash(state);
        }
    }

    impl fmt::Display for StoreRepresentation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "StoreRepresentation {{ representation: {:?}, write_barrier_kind: {:?} }}",
                self.representation, self.write_barrier_kind
            )
        }
    }

    pub fn store_representation_of(_op: &Operator) -> &StoreRepresentation {
        unimplemented!()
    }

    pub fn store_pair_representation_of(_op: &Operator) -> &StorePairRepresentation {
        unimplemented!()
    }

    /// A Word(32|64)AtomicStore needs both a StoreRepresentation and a memory order.
    #[derive(Clone, Copy)]
    pub struct AtomicStoreParameters {
        store_representation: StoreRepresentation,
        order: AtomicMemoryOrder,
        kind: MemoryAccessKind,
    }

    impl AtomicStoreParameters {
        pub fn new(
            representation: MachineRepresentation,
            write_barrier_kind: WriteBarrierKind,
            order: AtomicMemoryOrder,
            kind: MemoryAccessKind,
        ) -> Self {
            let store_representation = StoreRepresentation::new(representation, write_barrier_kind);
            AtomicStoreParameters {
                store_representation,
                order,
                kind,
            }
        }

        pub fn representation(&self) -> MachineRepresentation {
            self.store_representation.representation()
        }
        pub fn write_barrier_kind(&self) -> WriteBarrierKind {
            self.store_representation.write_barrier_kind()
        }
        pub fn order(&self) -> AtomicMemoryOrder {
            self.order
        }
        pub fn kind(&self) -> MemoryAccessKind {
            self.kind
        }

        pub fn store_representation(&self) -> StoreRepresentation {
            self.store_representation
        }
    }

    impl PartialEq for AtomicStoreParameters {
        fn eq(&self, other: &Self) -> bool {
            self.store_representation == other.store_representation
                && self.order == other.order
                && self.kind == other.kind
        }
    }

    impl Eq for AtomicStoreParameters {}

    impl Hash for AtomicStoreParameters {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.store_representation.hash(state);
            self.order.hash(state);
            self.kind.hash(state);
        }
    }

    impl fmt::Display for AtomicStoreParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "AtomicStoreParameters {{ store_representation: {}, order: {:?}, kind: {:?} }}",
                self.store_representation, self.order, self.kind
            )
        }
    }

    pub fn atomic_store_parameters_of(_op: &Operator) -> &AtomicStoreParameters {
        unimplemented!()
    }

    /// An UnalignedStore needs a MachineType.
    pub type UnalignedStoreRepresentation = MachineRepresentation;

    pub fn unaligned_store_representation_of(_op: &Operator) -> &UnalignedStoreRepresentation {
        unimplemented!()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct StoreLaneParameters {
        pub kind: MemoryAccessKind,
        pub rep: MachineRepresentation,
        pub laneidx: u8,
    }

    impl fmt::Display for StoreLaneParameters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "StoreLaneParameters {{ kind: {:?}, rep: {:?}, laneidx: {} }}",
                self.kind, self.rep, self.laneidx
            )
        }
    }

    pub fn store_lane_parameters_of(_op: &Operator) -> &StoreLaneParameters {
        unimplemented!()
    }

    #[derive(Clone, Copy, Debug)]
    pub struct StackSlotRepresentation {
        size: i32,
        alignment: i32,
        is_tagged: bool,
    }

    impl StackSlotRepresentation {
        pub fn new(size: i32, alignment: i32, is_tagged: bool) -> Self {
            StackSlotRepresentation {
                size,
                alignment,
                is_tagged,
            }
        }

        pub fn size(&self) -> i32 {
            self.size
        }
        pub fn alignment(&self) -> i32 {
            self.alignment
        }
        pub fn is_tagged(&self) -> bool {
            self.is_tagged
        }
    }

    impl PartialEq for StackSlotRepresentation {
        fn eq(&self, other: &Self) -> bool {
            self.size == other.size && self.alignment == other.alignment && self.is_tagged == other.is_tagged
        }
    }

    impl Eq for StackSlotRepresentation {}

    impl Hash for StackSlotRepresentation {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.size.hash(state);
            self.alignment.hash(state);
            self.is_tagged.hash(state);
        }
    }

    impl fmt::Display for StackSlotRepresentation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "StackSlotRepresentation {{ size: {}, alignment: {}, is_tagged: {} }}",
                self.size, self.alignment, self.is_tagged
            )
        }
    }

    pub fn stack_slot_representation_of(_op: &Operator) -> &StackSlotRepresentation {
        unimplemented!()
    }

    pub fn atomic_op_type(_op: &Operator) -> MachineType {
        unimplemented!()
    }

    pub struct SimdImmediateParameter<const SIMD_SIZE: usize> {
        immediate: [u8; SIMD_SIZE],
        _marker: PhantomData<[(); SIMD_SIZE]>,
    }

    impl<const SIMD_SIZE: usize> SimdImmediateParameter<SIMD_SIZE> {
        pub fn new(immediate: [u8; SIMD_SIZE]) -> Self {
            SimdImmediateParameter {
                immediate,
                _marker: PhantomData,
            }
        }

        pub fn immediate(&self) -> &[u8; SIMD_SIZE] {
            &self.immediate
        }

        pub fn data(&self) -> *const u8 {
            self.immediate.as_ptr()
        }

        pub fn get(&self, x: usize) -> u8 {
            self.immediate[x]
        }
    }

    impl<const SIMD_SIZE: usize> PartialEq for SimdImmediateParameter<SIMD_SIZE> {
        fn eq(&self, other: &Self) -> bool {
            self.immediate() == other.immediate()
        }
    }

    impl<const SIMD_SIZE: usize> Eq for SimdImmediateParameter<SIMD_SIZE> {}

    impl<const SIMD_SIZE: usize> Hash for SimdImmediateParameter<SIMD_SIZE> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.immediate.hash(state);
        }
    }

    impl<const SIMD_SIZE: usize> fmt::Display for SimdImmediateParameter<SIMD_SIZE> {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            for i in 0..SIMD_SIZE {
                let separator = if i < SIMD_SIZE - 1 { "," } else { "" };
                write!(os, "{}{}", self.immediate[i] as u32, separator)?;
            }
            Ok(())
        }
    }

    pub type S128ImmediateParameter = SimdImmediateParameter<kSimd128Size>;
    pub type S256ImmediateParameter = SimdImmediateParameter<32>; // Assuming kSimd256Size = 32

    pub fn s128_immediate_parameter_of(_op: &Operator) -> &S128ImmediateParameter {
        unimplemented!()
    }

    pub fn s256_immediate_parameter_of(_op: &Operator) -> &S256ImmediateParameter {
        unimplemented!()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum StackCheckKind {
        Normal,
        FunctionEntry,
    }

    pub fn stack_check_kind_of(_op: &Operator) -> StackCheckKind {
        unimplemented!()
    }

    /// ShiftKind::kShiftOutZeros means that it is guaranteed that the bits shifted
    /// out of the left operand are all zeros. If this is not the case, undefined
    /// behavior (i.e., incorrect optimizations) will happen.
    /// This is mostly useful for Smi untagging.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum ShiftKind {
        Normal,
        ShiftOutZeros,
    }

    impl fmt::Display for ShiftKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub fn shift_kind_of(_op: &Operator) -> ShiftKind {
        unimplemented!()
    }

    /// TruncateKind::kSetOverflowToMin sets the result of a saturating float-to-int
    /// conversion to INT_MIN if the conversion returns INT_MAX due to overflow. This
    /// makes it easier to detect an overflow. This parameter is ignored on platforms
    /// like x64 and ia32 where a range overflow does not result in INT_MAX.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum TruncateKind {
        ArchitectureDefault,
        SetOverflowToMin,
    }

    impl fmt::Display for TruncateKind {
        fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TruncateKind::ArchitectureDefault => write!(os, "ArchitectureDefault"),
                TruncateKind::SetOverflowToMin => write!(os, "SetOverflowToMin"),
            }
        }
    }

    // Interface for building machine-level operators. These operators are
    // machine-level but machine-independent and thus define a language suitable
    // for generating code to run on architectures such as ia32, x64, arm, etc.
    pub struct MachineOperatorBuilder {
        zone: (), // Replace with actual Zone type if available
        cache: MachineOperatorGlobalCache,
        word: MachineRepresentation,
        flags: Flags,
        alignment_requirements: AlignmentRequirements,
    }

    impl MachineOperatorBuilder {
        pub fn new(
            _zone: (), // Replace with actual Zone type if available
            word: MachineRepresentation,
            supported_operators: Flags,
            alignment_requirements: AlignmentRequirements,
        ) -> Self {
            MachineOperatorBuilder {
                zone: (),
                cache: MachineOperatorGlobalCache {},
                word,
                flags: supported_operators,
                alignment_requirements,
            }
        }

        pub fn comment(&self, _msg: &str) -> &Operator {
            unimplemented!()
        }
        pub fn abort_csadcheck(&self) -> &Operator {
            unimplemented!()
        }
        pub fn debug_break(&self) -> &Operator {
            unimplemented!()
        }

        pub fn word32_and(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_or(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_xor(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_shl(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_shr(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_sar(&self, kind: ShiftKind) -> &Operator {
            unimplemented!()
        }
        pub fn word32_rol(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word32_ror(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_equal(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_clz(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_ctz(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word32_popcnt(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word64_popcnt(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word32_reverse_bits(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word64_reverse_bits(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word32_reverse_bytes(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_reverse_bytes(&self) -> &Operator {
            unimplemented!()
        }
        pub fn simd128_reverse_bytes(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_abs_with_overflow(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn int64_abs_with_overflow(&self) -> OptionalOperator {
            unimplemented!()
        }

        // Return true if the target's Word32 shift implementation is directly
        // compatible with JavaScript's specification. Otherwise, we have to manually
        // generate a mask with 0x1f on the amount ahead of generating the shift.
        pub fn word32_shift_is_safe(&self) -> bool {
            self.flags.contains(Flag::Word32ShiftIsSafe)
        }

        // Return true if the target's implementation of float-to-int-conversions is a
        // saturating conversion rounding towards 0. Otherwise, we have to manually
        // generate the correct value if a saturating conversion is requested.
        pub fn sat_conversion_is_safe(&self) -> bool {
            self.flags.contains(Flag::SatConversionIsSafe)
        }

        // Return true if the target suppoerts performing a pair of loads/stores in
        // a single operation.
        pub fn supports_load_store_pairs(&self) -> bool {
            //  return !v8_flags.enable_unconditional_write_barriers &&
            //        (self.flags & kLoadStorePairs);
            //TODO Implement v8_flags, also replace the flag constaints
            self.flags.contains(Flag::LoadStorePairs)
        }

        pub fn word64_and(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_or(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_xor(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_shl(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_shr(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_sar(&self, kind: ShiftKind) -> &Operator {
            unimplemented!()
        }

        // 64-bit rol, ror, clz and ctz operators have two versions: the non-suffixed
        // ones are meant to be used in 64-bit systems and have no control input. The
        // "Lowerable"-suffixed ones are meant to be temporary operators in 32-bit
        // systems and will be lowered to 32-bit operators. They have a control input
        // to enable the lowering.
        pub fn word64_rol(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word64_ror(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_clz(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_ctz(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word64_rol_lowerable(&self) -> OptionalOperator {
            unimplemented!()
        }
        pub fn word64_ror_lowerable(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_clz_lowerable(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word64_ctz_lowerable(&self) -> OptionalOperator {
            unimplemented!()
        }

        pub fn word64_equal(&self) -> &Operator {
            unimplemented!()
        }

        pub fn int32_pair_add(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_pair_sub(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_pair_mul(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_pair_shl(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_pair_shr(&self) -> &Operator {
            unimplemented!()
        }
        pub fn word32_pair_sar(&self) -> &Operator {
            unimplemented!()
        }

        pub fn int32_add(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_add_with_overflow(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_sub(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_sub_with_overflow(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_mul(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_mul_with_overflow(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_mul_high(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_div(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_mod(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_less_than(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_less_than_or_equal(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint32_div(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint32_less_than(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint32_less_than_or_equal(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint32_mod(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint32_mul_high(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int32_div_is_safe(&self) -> bool {
            self.flags.contains(Flag::Int32DivIsSafe)
        }
        pub fn uint32_div_is_safe(&self) -> bool {
            self.flags.contains(Flag::Uint32DivIsSafe)
        }

        pub fn int64_add(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_add_with_overflow(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_sub(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_sub_with_overflow(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_mul(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_mul_high(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_mul_with_overflow(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_div(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_mod(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_less_than(&self) -> &Operator {
            unimplemented!()
        }
        pub fn int64_less_than_or_equal(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint64_div(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint64_less_than(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint64_less_than_or_equal(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint64_mod(&self) -> &Operator {
            unimplemented!()
        }
        pub fn uint64_mul_high(&self) -> &Operator {
            unimplemented!()
        }

        // This operator reinterprets the bits of a tagged pointer as a word.
        pub fn bitcast_tagged_to_word(&self) -> &Operator {
            unimplemented!()
        }

        // This operator reinterprets the bits of a tagged value as a word preserving
        // non-pointer bits (all the bits that are not modified by GC):
        // 1) smi tag
        // 2) weak tag
        // 3) smi payload if the tagged value is a smi.
        // Note, that it's illegal to "look" at the pointer bits of non-smi values.
        pub fn bitcast_tagged_to_word_for_tag_and_smi_bits(&self) -> &Operator {
            unimplemented!()
        }

        // This operator reinterprets the bits of a tagged Tagged<MaybeObject> pointer
        // as word.
        pub fn bitcast_maybe_object_to_word(&self) -> &Operator {
            unimplemented!()
        }

        // This operator reinterprets the bits of a word as tagged pointer.
        pub fn bitcast_word_to_tagged(&self) -> &Operator {
            unimplemented!()
        }

        // This operator reinterprets the bits of a word as a Smi.
        pub fn bitcast_word_to_tagged_signed(&self) -> &Operator {
            unimplemented!()
        }

        // JavaScript float64 to int32/uint32 truncation.
        pub fn truncate_float64_to_word32(&self) -> &Operator {
            unimplemented!()
        }

        // These operators change the representation of numbers while preserving the
        // value of the number. Narrowing operators assume the input is representable
        // in the target type and are *not* defined for other inputs.
        // Use narrowing change operators only when there is a static guarantee that
        // the input value is representable in the target value.
        //
        // Some operators can have the behaviour on overflow change through specifying
        // TruncateKind. The exact semantics are documented in the tests in
        // test/cctest/compiler/test-run-machops.cc .
        pub fn change_float32_to_float64(&self) -> &Operator {
            unimplemented!()
        }
        pub fn change_float64_to_int32(&self) -> &Operator {
            unimplemented!()
        } // narrowing
        pub fn change_float64_to_int64(&self) -> &Operator {
            unimplemented!()
        }
        pub fn change_float64_to_uint32(&self) -> &Operator {
            unimplemented!()
        } // narrowing
        pub fn change_float64_to_uint64(&self) -> &Operator {
            unimplemented!()
        }
        pub fn truncate_float64_to_int64(&self, kind: TruncateKind) -> &Operator {
            unimplemented!()
        }
        pub fn truncate_float64_to_uint32(&self) -> &Operator {
            unimplemented!()
        