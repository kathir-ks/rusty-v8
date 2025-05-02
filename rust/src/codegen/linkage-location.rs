// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Determine if there's a Rust equivalent for __declspec(noinline).
// For now, it's ignored.

pub mod linkage_location {
    use std::marker::Copy;
    use std::marker::Clone;

    pub use crate::codegen::machine_type::*;
    pub use crate::execution::frame_constants::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct LinkageLocation {
        bit_field: i32,
        machine_type: MachineType,
    }

    impl LinkageLocation {
        pub fn is_same_location(a: &LinkageLocation, b: &LinkageLocation) -> bool {
            (a.bit_field == b.bit_field) &&
                (is_subtype(a.machine_type.representation, b.machine_type.representation) ||
                 is_subtype(b.machine_type.representation, a.machine_type.representation))
        }

        pub fn for_null_register(reg: i32, type_: MachineType) -> LinkageLocation {
            LinkageLocation::new(LocationType::Register, reg, type_)
        }

        pub fn for_any_register(type_: MachineType) -> LinkageLocation {
            LinkageLocation::new(LocationType::Register, ANY_REGISTER, type_)
        }

        pub fn for_register(reg: i32, type_: MachineType) -> LinkageLocation {
            debug_assert!(reg >= 0);
            LinkageLocation::new(LocationType::Register, reg, type_)
        }

        pub fn for_caller_frame_slot(slot: i32, type_: MachineType) -> LinkageLocation {
            debug_assert!(slot < 0);
            LinkageLocation::new(LocationType::StackSlot, slot, type_)
        }

        pub fn for_callee_frame_slot(slot: i32, type_: MachineType) -> LinkageLocation {
            debug_assert!(slot >= 0 && slot < MAX_STACK_SLOT);
            LinkageLocation::new(LocationType::StackSlot, slot, type_)
        }

        pub fn for_saved_caller_return_address() -> LinkageLocation {
            LinkageLocation::for_callee_frame_slot(
                (STANDARD_FRAME_CONSTANTS.k_caller_pc_offset - STANDARD_FRAME_CONSTANTS.k_caller_pc_offset) / k_system_pointer_size,
                MachineType::Pointer
            )
        }

        pub fn for_saved_caller_frame_ptr() -> LinkageLocation {
            LinkageLocation::for_callee_frame_slot(
                (STANDARD_FRAME_CONSTANTS.k_caller_pc_offset - STANDARD_FRAME_CONSTANTS.k_caller_fp_offset) / k_system_pointer_size,
                MachineType::Pointer
            )
        }

        pub fn for_saved_caller_constant_pool() -> LinkageLocation {
            if !V8_EMBEDDED_CONSTANT_POOL_BOOL {
                panic!("V8_EMBEDDED_CONSTANT_POOL_BOOL is false"); // Or handle it differently, e.g., return an Option
            }
            LinkageLocation::for_callee_frame_slot(
                (STANDARD_FRAME_CONSTANTS.k_caller_pc_offset - STANDARD_FRAME_CONSTANTS.k_constant_pool_offset) / k_system_pointer_size,
                MachineType::AnyTagged
            )
        }

        pub fn for_saved_caller_function() -> LinkageLocation {
            LinkageLocation::for_callee_frame_slot(
                (STANDARD_FRAME_CONSTANTS.k_caller_pc_offset - STANDARD_FRAME_CONSTANTS.k_function_offset) / k_system_pointer_size,
                MachineType::AnyTagged
            )
        }

        pub fn convert_to_tail_caller_location(caller_location: LinkageLocation, stack_param_delta: i32) -> LinkageLocation {
            if !caller_location.is_register() {
                LinkageLocation::new(
                    LocationType::StackSlot,
                    caller_location.get_location() + stack_param_delta,
                    caller_location.get_type(),
                )
            } else {
                caller_location
            }
        }

        pub fn get_type(&self) -> MachineType {
            self.machine_type
        }

        pub fn get_size_in_pointers(&self) -> i32 {
            element_size_in_pointers(self.get_type().representation)
        }

        pub fn get_location(&self) -> i32 {
            self.bit_field
        }

        pub fn is_null_register(&self) -> bool {
            self.is_register() && self.get_location() < ANY_REGISTER
        }

        pub fn is_register(&self) -> bool {
            match self.get_location_type() {
                LocationType::Register => true,
                LocationType::StackSlot => false,
            }
        }

        pub fn is_any_register(&self) -> bool {
            self.is_register() && self.get_location() == ANY_REGISTER
        }

        pub fn is_caller_frame_slot(&self) -> bool {
            !self.is_register() && self.get_location() < 0
        }

        pub fn is_callee_frame_slot(&self) -> bool {
            !self.is_register() && self.get_location() >= 0
        }

        pub fn as_register(&self) -> i32 {
            debug_assert!(self.is_register());
            self.get_location()
        }

        pub fn as_caller_frame_slot(&self) -> i32 {
            debug_assert!(self.is_caller_frame_slot());
            self.get_location()
        }

        pub fn as_callee_frame_slot(&self) -> i32 {
            debug_assert!(self.is_callee_frame_slot());
            self.get_location()
        }

        fn new(type_: LocationType, location: i32, machine_type: MachineType) -> Self {
            LinkageLocation {
                bit_field: type_.encode() | ((location as u32) << LOCATION_SHIFT) as i32,
                machine_type,
            }
        }

        fn get_location_type(&self) -> LocationType {
            if (self.bit_field & TYPE_MASK) == 0 {
                LocationType::Register
            } else {
                LocationType::StackSlot
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum LocationType {
        Register,
        StackSlot,
    }

    impl LocationType {
        const REGISTER_VALUE: i32 = 0;
        const STACK_SLOT_VALUE: i32 = 1;

        fn encode(&self) -> i32 {
            match self {
                LocationType::Register => Self::REGISTER_VALUE,
                LocationType::StackSlot => Self::STACK_SLOT_VALUE,
            }
        }
    }

    const ANY_REGISTER: i32 = -1;
    const MAX_STACK_SLOT: i32 = 32767;

    const TYPE_SHIFT: i32 = 0;
    const TYPE_MASK: i32 = 0x1 << TYPE_SHIFT;
    const LOCATION_SHIFT: i32 = 1;
    const LOCATION_MASK: i32 = !(TYPE_MASK) as i32;

    // TODO: Implement Signature
    pub type LocationSignature = Vec<LinkageLocation>;
}

pub mod base {
    pub mod bit_field {
        // TODO: Implement BitField
    }
}

pub mod codegen {
    pub mod machine_type {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct MachineType {
            pub representation: Representation,
        }

        impl MachineType {
            pub const None: MachineType = MachineType { representation: Representation::None };
            pub const Pointer: MachineType = MachineType { representation: Representation::Pointer };
            pub const AnyTagged: MachineType = MachineType { representation: Representation::AnyTagged };
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Representation {
            None,
            Integer32,
            Integer64,
            Float32,
            Float64,
            Simd128,
            Pointer,
            Smi,
            HeapObject,
            TaggedSigned,
            TaggedPointer,
            AnyTagged,
        }

        pub fn is_subtype(a: Representation, b: Representation) -> bool {
            a == b // Basic equality for now
        }

        pub fn element_size_in_pointers(_rep: Representation) -> i32 {
            1 // Placeholder
        }
    }
}

pub mod execution {
    pub mod frame_constants {
        pub const k_system_pointer_size: i32 = 8;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct StandardFrameConstants {
            pub k_caller_pc_offset: i32,
            pub k_caller_fp_offset: i32,
            pub k_constant_pool_offset: i32,
            pub k_function_offset: i32,
        }

        pub const STANDARD_FRAME_CONSTANTS: StandardFrameConstants = StandardFrameConstants {
            k_caller_pc_offset: 16,
            k_caller_fp_offset: 8,
            k_constant_pool_offset: 24,
            k_function_offset: 32,
        };
    }
}

const V8_EMBEDDED_CONSTANT_POOL_BOOL: bool = true; // Or false, depending on the desired behavior