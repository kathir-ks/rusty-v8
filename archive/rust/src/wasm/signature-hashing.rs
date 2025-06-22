// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides signature hashing functionality for WebAssembly.

// TODO: Define feature flag for enabling WebAssembly.
// #[cfg(feature = "wasm")]
// pub mod wasm {

use std::marker::PhantomData;

// Re-export necessary types from other modules.
pub use crate::codegen::{
    AddArgumentPaddingSlots, IsAnyTagged, IsFloatingPoint, IsIntegral, LinkageLocation,
    LinkageLocationAllocator, MachineRepresentation, MachineType,
};
pub use crate::wasm_linkage::*;

/// Gets the machine representation of a ValueTypeBase.
pub fn get_machine_representation_value_type(type_: ValueTypeBase) -> MachineRepresentation {
    type_.machine_representation()
}

/// Gets the machine representation of a MachineType.
pub fn get_machine_representation_machine_type(type_: MachineType) -> MachineRepresentation {
    type_.representation()
}

/// Iterates through the signature and collects linkage locations.
///
/// This shared helper ensures that `GetWasmCallDescriptor` and
/// `SignatureHasher::Hash` remain in sync.
///
/// # Type Parameters
///
/// * `ResultCollector`: The type of the result collector, matching the
///   `LocationSignature::Builder` class. It must support:
///     * `add_param_at(index: usize, location: LinkageLocation)`
///     * `add_return_at(index: usize, location: LinkageLocation)`
/// * `SigType`: The type of the signature, matching the `Signature` class.
///   It must support:
///     * `parameter_count() -> usize`
///     * `return_count() -> usize`
///     * `get_param(index: usize) -> T` for `T` in `{ValueType, MachineType}`
///     * `get_return(index: usize) -> T` for `T` in `{ValueType, MachineType}`
///
/// # Arguments
///
/// * `sig`: The signature to iterate through.
/// * `extra_callable_param`: Whether to add an implicit "callable" parameter
///   that import call wrappers have, hard-coded to use the `kJSFunctionRegister`.
/// * `locations`: The result collector to add the linkage locations to.
/// * `untagged_parameter_slots`: Output parameter, stores the number of untagged parameter slots.
/// * `total_parameter_slots`: Output parameter, stores the total number of parameter slots.
/// * `untagged_return_slots`: Output parameter, stores the number of untagged return slots.
/// * `total_return_slots`: Output parameter, stores the total number of return slots.
pub fn iterate_signature_impl<ResultCollector, SigType>(
    sig: &SigType,
    extra_callable_param: bool,
    locations: &mut ResultCollector,
    untagged_parameter_slots: &mut i32,
    total_parameter_slots: &mut i32,
    untagged_return_slots: &mut i32,
    total_return_slots: &mut i32,
) where
    ResultCollector: LocationSignatureBuilder,
    SigType: SignatureLike,
{
    const PARAMS_SLOT_OFFSET: usize = 0;
    let mut params = LinkageLocationAllocator::new(
        &kGpParamRegisters,
        &kFpParamRegisters,
        PARAMS_SLOT_OFFSET,
    );

    // The instance object.
    locations.add_param_at(0, params.next(MachineRepresentation::TaggedPointer));
    let param_offset = 1; // Actual params start here.

    // Parameters are separated into two groups (first all untagged, then all
    // tagged parameters). This allows for easy iteration of tagged parameters
    // during frame iteration. It also allows for easy signature verification
    // based on counts.
    let parameter_count = sig.parameter_count();
    let mut has_tagged_param = false;
    for i in 0..parameter_count {
        let param = get_machine_representation_machine_type(sig.get_param(i));
        // Skip tagged parameters (e.g. any-ref).
        if IsAnyTagged(param) {
            has_tagged_param = true;
            continue;
        }
        locations.add_param_at(i + param_offset, params.next(param));
    }
    params.end_slot_area(); // End the untagged area. Tagged slots come after.
    *untagged_parameter_slots = params.num_stack_slots() as i32;
    if has_tagged_param {
        for i in 0..parameter_count {
            let param = get_machine_representation_machine_type(sig.get_param(i));
            if !IsAnyTagged(param) {
                continue; // Skip untagged parameters.
            }
            locations.add_param_at(i + param_offset, params.next(param));
        }
    }

    // Import call wrappers have an additional (implicit) parameter, the callable.
    // For consistency with JS, we use the JSFunction register.
    if extra_callable_param {
        locations.add_param_at(
            parameter_count + param_offset,
            LinkageLocation::for_register(kJSFunctionRegister.code(), MachineType::TaggedPointer),
        );
    }

    let params_stack_height = AddArgumentPaddingSlots(params.num_stack_slots()) as i32;
    *total_parameter_slots = params_stack_height;

    // Add return location(s).
    // For efficient signature verification, order results by taggedness, such
    // that all untagged results appear first in registers and on the stack,
    // followed by tagged results. That way, we can simply check the size of
    // each section, rather than needing a bit map.
    let mut rets = LinkageLocationAllocator::new(
        &kGpReturnRegisters,
        &kFpReturnRegisters,
        params_stack_height as usize,
    );

    let return_count = sig.return_count();
    let mut has_tagged_result = false;
    for i in 0..return_count {
        let ret = get_machine_representation_machine_type(sig.get_return(i));
        if IsAnyTagged(ret) {
            has_tagged_result = true;
            continue;
        }
        locations.add_return_at(i, rets.next(ret));
    }
    rets.end_slot_area(); // End the untagged area.
    *untagged_return_slots = rets.num_stack_slots() as i32;
    if has_tagged_result {
        for i in 0..return_count {
            let ret = get_machine_representation_machine_type(sig.get_return(i));
            if !IsAnyTagged(ret) {
                continue;
            }
            locations.add_return_at(i, rets.next(ret));
        }
    }
    *total_return_slots = rets.num_stack_slots() as i32;
}

/// Trait for signature types, mirroring the requirements from the C++ code.
pub trait SignatureLike {
    fn parameter_count() -> usize;
    fn return_count() -> usize;
    fn get_param(&self, index: usize) -> MachineType;
    fn get_return(&self, index: usize) -> MachineType;
}

/// Trait for result collector types, mirroring the requirements from the C++ code.
pub trait LocationSignatureBuilder {
    fn add_param_at(&mut self, index: usize, location: LinkageLocation);
    fn add_return_at(&mut self, index: usize, location: LinkageLocation);
}

//#[cfg(feature = "sandbox")]
pub mod sandbox {
    use super::*;
    use bitfield::bitfield;
    use std::mem::size_of;
    use crate::wasm::kV8MaxWasmFunctionParams;
    use crate::wasm::kV8MaxWasmFunctionReturns;

    /// Computes a "signature hash" for sandbox hardening.
    ///
    /// Two functions should have the same "signature hash" if and only if mixing them
    /// up (due to in-sandbox corruption) cannot possibly lead to a sandbox escape.
    /// That means in particular that we must ensure the following properties:
    /// - there must be no tagged/untagged mixups among parameters passed in GP registers.
    /// - there must be no tagged/untagged mixups among parameters passed on the stack.
    /// - there must be no mismatch in the sizes of the stack regions used for passing parameters.
    /// - these same properties must hold for return values.
    ///
    /// To achieve this, we simulate the linkage locations that
    /// `GetWasmCallDescriptor` would assign, and collect the counts of
    /// tagged/untagged parameters in registers and on the stack, respectively.
    pub struct SignatureHasher {
        params: Counts,
        rets: Counts,
    }

    impl SignatureHasher {
        /// Generates the hash for a given signature.
        pub fn hash<SigType>(sig: &SigType) -> u64
        where
            SigType: SignatureLike,
        {
            let mut hasher = SignatureHasher {
                params: Counts::default(),
                rets: Counts::default(),
            };
            let mut total_param_stack_slots = 0;
            let mut total_return_stack_slots = 0;
            iterate_signature_impl(
                sig,
                false, /* no extra callable parameter */
                &mut hasher,
                &mut hasher.params.untagged_on_stack,
                &mut total_param_stack_slots,
                &mut hasher.rets.untagged_on_stack,
                &mut total_return_stack_slots,
            );

            hasher.params.tagged_on_stack =
                total_param_stack_slots - hasher.params.untagged_on_stack;
            hasher.rets.tagged_on_stack =
                total_return_stack_slots - hasher.rets.untagged_on_stack;

            hasher.get_hash()
        }

        fn get_hash(&self) -> u64 {
            (self.rets.get_hash() << Self::K_TOTAL_WIDTH) | self.params.get_hash()
        }

        fn count_if_register(&self, loc: LinkageLocation, counts: &mut Counts) {
            if !loc.is_register() {
                debug_assert!(loc.is_caller_frame_slot());
                return;
            }
            let type_ = loc.get_type();
            if type_.is_tagged() {
                counts.tagged_in_reg += 1;
            } else if IsIntegral(type_.representation()) {
                counts.untagged_in_reg += 1;
            } else {
                debug_assert!(IsFloatingPoint(type_.representation()));
                // No need to count FP registers.
            }
        }

        const K_UNTAGGED_IN_REG_BITS: usize = 3;
        const K_TAGGED_IN_REG_BITS: usize = 3;
        const K_UNTAGGED_ON_STACK_BITS: usize = 11;
        const K_TAGGED_ON_STACK_BITS: usize = 10;

        const K_TOTAL_WIDTH: usize = Self::K_TAGGED_ON_STACK_BITS + Self::K_UNTAGGED_ON_STACK_BITS + Self::K_TAGGED_IN_REG_BITS + Self::K_UNTAGGED_IN_REG_BITS;
    }

    impl LocationSignatureBuilder for SignatureHasher {
        fn add_param_at(&mut self, index: usize, location: LinkageLocation) {
            if index == 0 {
                return; // Skip the instance object.
            }
            self.count_if_register(location, &mut self.params);
        }

        fn add_return_at(&mut self, index: usize, location: LinkageLocation) {
            self.count_if_register(location, &mut self.rets);
        }
    }

    #[derive(Default)]
    struct Counts {
        tagged_in_reg: i32,
        untagged_in_reg: i32,
        tagged_on_stack: i32,
        untagged_on_stack: i32,
    }

    impl Counts {
        fn get_hash(&self) -> u64 {
            let mut result: u64 = 0;
            result |= UntaggedInReg::new(self.untagged_in_reg as u32) as u64;
            result |= (TaggedInReg::new(self.tagged_in_reg as u32) as u64) << SignatureHasher::K_UNTAGGED_IN_REG_BITS;
            result |= (UntaggedOnStack::new(self.untagged_on_stack as u32) as u64) << (SignatureHasher::K_UNTAGGED_IN_REG_BITS + SignatureHasher::K_TAGGED_IN_REG_BITS);
            result |= (TaggedOnStack::new(self.tagged_on_stack as u32) as u64) << (SignatureHasher::K_UNTAGGED_IN_REG_BITS + SignatureHasher::K_TAGGED_IN_REG_BITS + SignatureHasher::K_UNTAGGED_ON_STACK_BITS);
            result
        }
    }

    bitfield! {
        #[derive(Clone, Copy)]
        struct UntaggedInReg(u32);
        impl Debug;
        untagged_in_reg, set_untagged_in_reg: 2, 0;
    }

    bitfield! {
        #[derive(Clone, Copy)]
        struct TaggedInReg(u32);
        impl Debug;
        tagged_in_reg, set_tagged_in_reg: 5, 3;
    }

    bitfield! {
        #[derive(Clone, Copy)]
        struct UntaggedOnStack(u32);
        impl Debug;
        untagged_on_stack, set_untagged_on_stack: 16, 6;
    }

    bitfield! {
        #[derive(Clone, Copy)]
        struct TaggedOnStack(u32);
        impl Debug;
        tagged_on_stack, set_tagged_on_stack: 26, 17;
    }

    const _: () = {
        assert!(kGpParamRegisters.len() <= 2_i32.pow(SignatureHasher::K_UNTAGGED_IN_REG_BITS as u32) as usize);
        assert!(kGpParamRegisters.len() <= 2_i32.pow(SignatureHasher::K_TAGGED_IN_REG_BITS as u32) as usize);
        assert!(kGpReturnRegisters.len() <= 2_i32.pow(SignatureHasher::K_UNTAGGED_IN_REG_BITS as u32) as usize);
        assert!(kGpReturnRegisters.len() <= 2_i32.pow(SignatureHasher::K_TAGGED_IN_REG_BITS as u32) as usize);
        const K_MAX_VALUE_SIZE_IN_POINTERS: usize =
            K_MAX_VALUE_TYPE_SIZE / size_of::<usize>();
        assert!(kV8MaxWasmFunctionParams * K_MAX_VALUE_SIZE_IN_POINTERS <= 2_i32.pow(SignatureHasher::K_UNTAGGED_ON_STACK_BITS as u32) as usize);
        assert!(kV8MaxWasmFunctionParams <= 2_i32.pow(SignatureHasher::K_TAGGED_ON_STACK_BITS as u32) as usize);
        assert!(kV8MaxWasmFunctionReturns * K_MAX_VALUE_SIZE_IN_POINTERS <= 2_i32.pow(SignatureHasher::K_UNTAGGED_ON_STACK_BITS as u32) as usize);
        assert!(kV8MaxWasmFunctionReturns <= 2_i32.pow(SignatureHasher::K_TAGGED_ON_STACK_BITS as u32) as usize);
    };

    // Implement Debug for the bitfield structs
    impl std::fmt::Debug for Counts {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Counts")
                .field("tagged_in_reg", &self.tagged_in_reg)
                .field("untagged_in_reg", &self.untagged_in_reg)
                .field("tagged_on_stack", &self.tagged_on_stack)
                .field("untagged_on_stack", &self.untagged_on_stack)
                .finish()
        }
    }
}

//#[cfg(not(feature = "sandbox"))]
pub mod nosandbox {
    use super::*;

    /// Dummy SignatureHasher that always returns 0 as the hash.
    pub struct SignatureHasher {}

    impl SignatureHasher {
        /// Returns 0 as the hash for any signature.
        pub fn hash<SigType>(_sig: &SigType) -> u64
        where
            SigType: SignatureLike,
        {
            0
        }
    }
}

// The following constants need to be defined somewhere (likely in a codegen or wasm_linkage file),
// along with implementations for the functions used: IsAnyTagged, AddArgumentPaddingSlots.

// Example constants (replace with actual values)
const K_MAX_VALUE_TYPE_SIZE: usize = 8;
const K_SYSTEM_POINTER_SIZE: usize = 8;

lazy_static::lazy_static! {
    pub static ref kGpParamRegisters: [usize; 4] = [0, 1, 2, 3];
    pub static ref kFpParamRegisters: [usize; 4] = [4, 5, 6, 7];
    pub static ref kGpReturnRegisters: [usize; 2] = [8, 9];
    pub static ref kFpReturnRegisters: [usize; 2] = [10, 11];
    pub static ref kJSFunctionRegister: Register = Register { code: 123 };
}

#[derive(Debug, Copy, Clone)]
pub struct Register {
    code: i32,
}