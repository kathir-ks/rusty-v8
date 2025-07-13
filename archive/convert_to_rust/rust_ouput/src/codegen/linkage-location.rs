// Converted from V8 C++ source files:
// Header: linkage-location.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct BitField<T, const OFFSET: usize, const SIZE: usize>;
}

pub mod codegen {
    pub use super::internal::MachineType;
    pub struct MachineType {
        representation: Representation,
    }

    impl MachineType {
        pub fn None() -> Self {
            MachineType {
                representation: Representation::None,
            }
        }

        pub fn Pointer() -> Self {
            MachineType {
                representation: Representation::Pointer,
            }
        }

        pub fn AnyTagged() -> Self {
            MachineType {
                representation: Representation::AnyTagged,
            }
        }

        pub fn representation(&self) -> Representation {
            self.representation
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum Representation {
        None,
        Integer32,
        Integer64,
        Float32,
        Float64,
        Simd128,
        Pointer,
        AnyTagged,
        Smi,
        Double,
    }
}

pub mod execution {
    pub mod frame_constants {
        pub const kCallerPCOffset: i32 = 8;
        pub const kCallerFPOffset: i32 = 16;
        pub const kConstantPoolOffset: i32 = 24;
        pub const kFunctionOffset: i32 = 32;
    }
}

pub mod internal {
    use crate::base::BitField;
    use crate::codegen::MachineType;
    use crate::codegen::Representation;
    use crate::execution::frame_constants::*;

    #[derive(Copy, Clone, Debug)]
    pub struct LinkageLocation {
        bit_field_: i32,
        machine_type_: MachineType,
    }

    impl PartialEq for LinkageLocation {
        fn eq(&self, other: &Self) -> bool {
            self.bit_field_ == other.bit_field_ && self.machine_type_ == other.machine_type_
        }
    }

    impl Eq for LinkageLocation {}

    impl LinkageLocation {
        pub fn operator_ne(&self, other: &LinkageLocation) -> bool {
            !(*self == *other)
        }

        pub fn is_same_location(a: &LinkageLocation, b: &LinkageLocation) -> bool {
            (a.bit_field_ == b.bit_field_)
                && (LinkageLocation::is_subtype(
                    a.machine_type_.representation(),
                    b.machine_type_.representation(),
                ) || LinkageLocation::is_subtype(
                    b.machine_type_.representation(),
                    a.machine_type_.representation(),
                ))
        }

        fn is_subtype(a: Representation, b: Representation) -> bool {
            a == b || a == Representation::Smi && b == Representation::AnyTagged
        }

        pub fn for_null_register(reg: i32, type_: MachineType) -> LinkageLocation {
            LinkageLocation::new(LocationType::REGISTER, reg, type_)
        }

        pub fn for_any_register(type_: MachineType) -> LinkageLocation {
            LinkageLocation::new(LocationType::REGISTER, LinkageLocation::ANY_REGISTER, type_)
        }

        pub fn for_register(reg: i32, type_: MachineType) -> LinkageLocation {
            assert!(reg >= 0);
            LinkageLocation::new(LocationType::REGISTER, reg, type_)
        }

        pub fn for_caller_frame_slot(slot: i32, type_: MachineType) -> LinkageLocation {
            assert!(slot < 0);
            LinkageLocation::new(LocationType::STACK_SLOT, slot, type_)
        }

        pub fn for_callee_frame_slot(slot: i32, type_: MachineType) -> LinkageLocation {
            assert!(slot >= 0 && slot < LinkageLocation::MAX_STACK_SLOT);
            LinkageLocation::new(LocationType::STACK_SLOT, slot, type_)
        }

        pub fn for_saved_caller_return_address() -> LinkageLocation {
            LinkageLocation::for_callee_frame_slot(
                (kCallerPCOffset - kCallerPCOffset) / kSystemPointerSize,
                MachineType::Pointer(),
            )
        }

        pub fn for_saved_caller_frame_ptr() -> LinkageLocation {
            LinkageLocation::for_callee_frame_slot(
                (kCallerPCOffset - kCallerFPOffset) / kSystemPointerSize,
                MachineType::Pointer(),
            )
        }

        pub fn for_saved_caller_constant_pool() -> LinkageLocation {
            LinkageLocation::for_callee_frame_slot(
                (kCallerPCOffset - kConstantPoolOffset) / kSystemPointerSize,
                MachineType::AnyTagged(),
            )
        }

        pub fn for_saved_caller_function() -> LinkageLocation {
            LinkageLocation::for_callee_frame_slot(
                (kCallerPCOffset - kFunctionOffset) / kSystemPointerSize,
                MachineType::AnyTagged(),
            )
        }

        pub fn convert_to_tail_caller_location(
            caller_location: LinkageLocation,
            stack_param_delta: i32,
        ) -> LinkageLocation {
            if !caller_location.is_register() {
                LinkageLocation::new(
                    LocationType::STACK_SLOT,
                    caller_location.get_location() + stack_param_delta,
                    caller_location.get_type(),
                )
            } else {
                caller_location
            }
        }

        pub fn get_type(&self) -> MachineType {
            self.machine_type_
        }

        pub fn get_size_in_pointers(&self) -> i32 {
            LinkageLocation::element_size_in_pointers(self.get_type().representation())
        }

        pub fn get_location(&self) -> i32 {
            (self.bit_field_ & LinkageLocation::LocationField::kMask)
                >> LinkageLocation::LocationField::kShift
        }

        pub fn is_null_register(&self) -> bool {
            self.is_register() && self.get_location() < LinkageLocation::ANY_REGISTER
        }

        pub fn is_register(&self) -> bool {
            LinkageLocation::TypeField::decode(self.bit_field_) == LocationType::REGISTER
        }

        pub fn is_any_register(&self) -> bool {
            self.is_register() && self.get_location() == LinkageLocation::ANY_REGISTER
        }

        pub fn is_caller_frame_slot(&self) -> bool {
            !self.is_register() && self.get_location() < 0
        }

        pub fn is_callee_frame_slot(&self) -> bool {
            !self.is_register() && self.get_location() >= 0
        }

        pub fn as_register(&self) -> i32 {
            assert!(self.is_register());
            self.get_location()
        }

        pub fn as_caller_frame_slot(&self) -> i32 {
            assert!(self.is_caller_frame_slot());
            self.get_location()
        }

        pub fn as_callee_frame_slot(&self) -> i32 {
            assert!(self.is_callee_frame_slot());
            self.get_location()
        }

        fn new(type_: LocationType, location: i32, machine_type: MachineType) -> LinkageLocation {
            let bit_field_ = LinkageLocation::TypeField::encode(type_)
                | ((location as u32) << LinkageLocation::LocationField::kShift
                    & LinkageLocation::LocationField::kMask);
            LinkageLocation {
                bit_field_: bit_field_ as i32,
                machine_type_: machine_type,
            }
        }

        fn element_size_in_pointers(representation: Representation) -> i32 {
            match representation {
                Representation::None => 0,
                Representation::Integer32 => 1,
                Representation::Integer64 => 1,
                Representation::Float32 => 1,
                Representation::Float64 => 1,
                Representation::Simd128 => 4,
                Representation::Pointer => 1,
                Representation::AnyTagged => 1,
                Representation::Smi => 1,
                Representation::Double => 1,
            }
        }

        type TypeField = BitField<LocationType, 0, 1>;
        type LocationField = BitField<i32, { TypeField::OFFSET + TypeField::SIZE }, 31>;

        const ANY_REGISTER: i32 = -1;
        const MAX_STACK_SLOT: i32 = 32767;
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    enum LocationType {
        REGISTER,
        STACK_SLOT,
    }

    pub struct LocationSignature {}

    const kSystemPointerSize: i32 = 8;
}
