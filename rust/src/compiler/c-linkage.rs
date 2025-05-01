// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add appropriate Rust crates for any C++ libraries used

// TODO: For header files (.h, .hpp), create appropriate Rust module definitions and public interfaces

// TODO: Convert classes to Rust structs with impl blocks for methods

// TODO: Handle memory management appropriately (raw pointers to Box, Arc, Rc, etc.)

// TODO: Convert C++ templates to Rust generics where applicable

// TODO: Transform error handling to use Rust's Result type

// TODO: Adapt any preprocessor macros to Rust macro_rules! or const values

// TODO: Add appropriate Rust documentation comments

//use std::convert::TryInto;

//mod codegen; // Assuming codegen module exists
//mod compiler; // Assuming compiler module exists
//mod zone;    // Assuming zone module exists

//use codegen::{Assembler, MacroAssembler};
//use compiler::{globals, linkage};
//use zone::Zone;

// Placeholder for the architecture-specific definitions
mod architecture {
    #[cfg(target_arch = "x86")]
    pub const ARCH: &str = "ia32";
    #[cfg(target_arch = "x86_64")]
    pub const ARCH: &str = "x64";
    #[cfg(target_arch = "arm")]
    pub const ARCH: &str = "arm";
    #[cfg(target_arch = "aarch64")]
    pub const ARCH: &str = "arm64";
    #[cfg(target_arch = "mips64")]
    pub const ARCH: &str = "mips64";
    #[cfg(target_arch = "loongarch64")]
    pub const ARCH: &str = "loong64";
    #[cfg(target_arch = "powerpc64")]
    pub const ARCH: &str = "ppc64";
    #[cfg(target_arch = "s390x")]
    pub const ARCH: &str = "s390x";
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    pub const ARCH: &str = "riscv";
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "mips64",
        target_arch = "loongarch64",
        target_arch = "powerpc64",
        target_arch = "s390x",
        target_arch = "riscv32",
        target_arch = "riscv64"
    )))]
    pub const ARCH: &str = "unknown";
}

// Placeholder for registers
mod registers {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register {
        code: i32,
    }

    impl Register {
        pub const fn new(code: i32) -> Self {
            Register { code }
        }

        pub fn code(&self) -> i32 {
            self.code
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DoubleRegister {
        code: i32,
    }

    impl DoubleRegister {
        pub const fn new(code: i32) -> Self {
            DoubleRegister { code }
        }

        pub fn code(&self) -> i32 {
            self.code
        }
    }
    
    // Example definitions, extend as needed for different architectures
    pub const kReturnRegister0: Register = Register::new(10);
    pub const kReturnRegister1: Register = Register::new(11);

    #[cfg(target_arch = "x86_64")]
    pub mod x64 {
        use super::*;
        // Example definitions for x64
        pub const rcx: Register = Register::new(1);
        pub const rdx: Register = Register::new(2);
        pub const r8: Register = Register::new(8);
        pub const r9: Register = Register::new(9);
        pub const xmm0: DoubleRegister = DoubleRegister::new(100);
        pub const xmm1: DoubleRegister = DoubleRegister::new(101);
        pub const xmm2: DoubleRegister = DoubleRegister::new(102);
        pub const xmm3: DoubleRegister = DoubleRegister::new(103);
        pub const xmm4: DoubleRegister = DoubleRegister::new(104);
        pub const xmm5: DoubleRegister = DoubleRegister::new(105);
        pub const xmm6: DoubleRegister = DoubleRegister::new(106);
        pub const xmm7: DoubleRegister = DoubleRegister::new(107);
    }

    #[cfg(target_arch = "arm")]
    pub mod arm {
        use super::*;
        pub const r0: Register = Register::new(0);
        pub const r1: Register = Register::new(1);
        pub const r2: Register = Register::new(2);
        pub const r3: Register = Register::new(3);
    }

    #[cfg(target_arch = "aarch64")]
    pub mod arm64 {
        use super::*;
        pub const x0: Register = Register::new(0);
        pub const x1: Register = Register::new(1);
        pub const x2: Register = Register::new(2);
        pub const x3: Register = Register::new(3);
        pub const x4: Register = Register::new(4);
        pub const x5: Register = Register::new(5);
        pub const x6: Register = Register::new(6);
        pub const x7: Register = Register::new(7);
        pub const d0: DoubleRegister = DoubleRegister::new(0);
        pub const d1: DoubleRegister = DoubleRegister::new(1);
        pub const d2: DoubleRegister = DoubleRegister::new(2);
        pub const d3: DoubleRegister = DoubleRegister::new(3);
        pub const d4: DoubleRegister = DoubleRegister::new(4);
        pub const d5: DoubleRegister = DoubleRegister::new(5);
        pub const d6: DoubleRegister = DoubleRegister::new(6);
        pub const d7: DoubleRegister = DoubleRegister::new(7);
    }

    #[cfg(target_arch = "loongarch64")]
    pub mod loongarch64 {
        use super::*;
        pub const a0: Register = Register::new(0);
        pub const a1: Register = Register::new(1);
        pub const a2: Register = Register::new(2);
        pub const a3: Register = Register::new(3);
        pub const a4: Register = Register::new(4);
        pub const a5: Register = Register::new(5);
        pub const a6: Register = Register::new(6);
        pub const a7: Register = Register::new(7);
        pub const f0: DoubleRegister = DoubleRegister::new(0);
        pub const f1: DoubleRegister = DoubleRegister::new(1);
        pub const f2: DoubleRegister = DoubleRegister::new(2);
        pub const f3: DoubleRegister = DoubleRegister::new(3);
        pub const f4: DoubleRegister = DoubleRegister::new(4);
        pub const f5: DoubleRegister = DoubleRegister::new(5);
        pub const f6: DoubleRegister = DoubleRegister::new(6);
        pub const f7: DoubleRegister = DoubleRegister::new(7);
    }

    #[cfg(target_arch = "mips64")]
    pub mod mips64 {
        use super::*;
        pub const a0: Register = Register::new(4);
        pub const a1: Register = Register::new(5);
        pub const a2: Register = Register::new(6);
        pub const a3: Register = Register::new(7);
        pub const a4: Register = Register::new(8);
        pub const a5: Register = Register::new(9);
        pub const a6: Register = Register::new(10);
        pub const a7: Register = Register::new(11);
        pub const f12: DoubleRegister = DoubleRegister::new(12);
        pub const f13: DoubleRegister = DoubleRegister::new(13);
        pub const f14: DoubleRegister = DoubleRegister::new(14);
        pub const f15: DoubleRegister = DoubleRegister::new(15);
        pub const f16: DoubleRegister = DoubleRegister::new(16);
        pub const f17: DoubleRegister = DoubleRegister::new(17);
        pub const f18: DoubleRegister = DoubleRegister::new(18);
        pub const f19: DoubleRegister = DoubleRegister::new(19);
        pub const f0: DoubleRegister = DoubleRegister::new(0);
    }

    #[cfg(target_arch = "powerpc64")]
    pub mod ppc64 {
        use super::*;
        pub const r3: Register = Register::new(3);
        pub const r4: Register = Register::new(4);
        pub const r5: Register = Register::new(5);
        pub const r6: Register = Register::new(6);
        pub const r7: Register = Register::new(7);
        pub const r8: Register = Register::new(8);
        pub const r9: Register = Register::new(9);
        pub const r10: Register = Register::new(10);
    }

    #[cfg(target_arch = "s390x")]
    pub mod s390x {
        use super::*;
        pub const r2: Register = Register::new(2);
        pub const r3: Register = Register::new(3);
        pub const r4: Register = Register::new(4);
        pub const r5: Register = Register::new(5);
        pub const r6: Register = Register::new(6);
    }

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    pub mod riscv {
        use super::*;
        pub const a0: Register = Register::new(10);
        pub const a1: Register = Register::new(11);
        pub const a2: Register = Register::new(12);
        pub const a3: Register = Register::new(13);
        pub const a4: Register = Register::new(14);
        pub const a5: Register = Register::new(15);
        pub const a6: Register = Register::new(16);
        pub const a7: Register = Register::new(17);
        pub const fa0: DoubleRegister = DoubleRegister::new(10);
        pub const fa1: DoubleRegister = DoubleRegister::new(11);
        pub const fa2: DoubleRegister = DoubleRegister::new(12);
        pub const fa3: DoubleRegister = DoubleRegister::new(13);
        pub const fa4: DoubleRegister = DoubleRegister::new(14);
        pub const fa5: DoubleRegister = DoubleRegister::new(15);
        pub const fa6: DoubleRegister = DoubleRegister::new(16);
        pub const fa7: DoubleRegister = DoubleRegister::new(17);
    }
}

mod linkage {
    //use super::codegen::{Assembler, MacroAssembler};
    //use super::compiler::globals;
    //use super::zone::Zone;
    use super::registers::*;
    use std::vec::Vec;
    //use super::*; // Import everything from the parent module

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum MachineRepresentation {
        Word8,
        Word16,
        Word32,
        Word64,
        Float32,
        Float64,
        Simd128,
        Pointer,
        Tagged,
        Bit, // Represents a single bit
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MachineType {
        representation: MachineRepresentation,
    }

    impl MachineType {
        pub const fn new(representation: MachineRepresentation) -> Self {
            MachineType { representation }
        }

        pub const fn pointer() -> Self {
            MachineType {
                representation: MachineRepresentation::Pointer,
            }
        }

        pub fn representation(&self) -> MachineRepresentation {
            self.representation
        }
    }

    fn is_floating_point(representation: MachineRepresentation) -> bool {
        representation == MachineRepresentation::Float32 || representation == MachineRepresentation::Float64
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MachineSignature {
        returns: Vec<MachineType>,
        params: Vec<MachineType>,
    }

    impl MachineSignature {
        pub fn new(returns: Vec<MachineType>, params: Vec<MachineType>) -> Self {
            MachineSignature { returns, params }
        }

        pub fn return_count(&self) -> usize {
            self.returns.len()
        }

        pub fn parameter_count(&self) -> usize {
            self.params.len()
        }

        pub fn get_return(&self, index: usize) -> MachineType {
            self.returns[index]
        }

        pub fn get_param(&self, index: usize) -> MachineType {
            self.params[index]
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum LinkageLocation {
        ForRegister { reg_code: i32, typ: MachineType },
        ForCallerFrameSlot { offset: i32, typ: MachineType },
        ForAnyRegister { typ: MachineType },
        ForNullRegister { reg_code: i32, typ: MachineType },
    }

    impl LinkageLocation {
        pub fn for_register(reg_code: i32, typ: MachineType) -> Self {
            LinkageLocation::ForRegister { reg_code, typ }
        }

        pub fn for_caller_frame_slot(offset: i32, typ: MachineType) -> Self {
            LinkageLocation::ForCallerFrameSlot { offset, typ }
        }

        pub fn for_any_register(typ: MachineType) -> Self {
            LinkageLocation::ForAnyRegister { typ }
        }
        
        pub fn for_null_register(reg_code: i32, typ: MachineType) -> Self {
            LinkageLocation::ForNullRegister { reg_code, typ }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LocationSignature {
        returns: Vec<LinkageLocation>,
        params: Vec<LinkageLocation>,
    }

    impl LocationSignature {
        fn new(returns: Vec<LinkageLocation>, params: Vec<LinkageLocation>) -> Self {
            LocationSignature { returns, params }
        }

        fn get(&self) -> &Self {
            self
        }
    }

    pub struct LocationSignatureBuilder<'a> {
        locations: LocationSignature,
        zone: &'a Zone,
    }

    impl<'a> LocationSignatureBuilder<'a> {
        pub fn new(zone: &'a Zone, return_count: usize, parameter_count: usize) -> Self {
            LocationSignatureBuilder {
                locations: LocationSignature {
                    returns: Vec::with_capacity(return_count),
                    params: Vec::with_capacity(parameter_count),
                },
                zone,
            }
        }

        pub fn add_return(&mut self, location: LinkageLocation) {
            self.locations.returns.push(location);
        }

        pub fn add_param(&mut self, location: LinkageLocation) {
            self.locations.params.push(location);
        }

        pub fn get(self) -> LocationSignature {
            self.locations
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CallDescriptor {
        kind: CallKind,
        tag: CodeEntrypointTag,
        target_type: MachineType,
        target_loc: LinkageLocation,
        location_sig: LocationSignature,
        stack_parameter_count: usize,
        properties: OperatorProperties,
        callee_save_registers: Vec<i32>, //RegList, // Replace with appropriate type if needed
        callee_save_fp_registers: Vec<i32>, //DoubleRegList, // Replace with appropriate type if needed
        flags: Flags,
        debug_name: String,
    }

    impl CallDescriptor {
        pub fn new(
            kind: CallKind,
            tag: CodeEntrypointTag,
            target_type: MachineType,
            target_loc: LinkageLocation,
            location_sig: LocationSignature,
            stack_parameter_count: usize,
            properties: OperatorProperties,
            callee_save_registers: Vec<i32>, //RegList,
            callee_save_fp_registers: Vec<i32>, //DoubleRegList,
            flags: Flags,
            debug_name: String,
        ) -> Self {
            CallDescriptor {
                kind,
                tag,
                target_type,
                target_loc,
                location_sig,
                stack_parameter_count,
                properties,
                callee_save_registers,
                callee_save_fp_registers,
                flags,
                debug_name,
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CallKind {
        CallCode,
        CallAddress,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CodeEntrypointTag {
        NoTag,
        // Add other tags as needed
    }

    const kDefaultCodeEntrypointTag: CodeEntrypointTag = CodeEntrypointTag::NoTag;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum OperatorProperties {
        NoThrow,
        Pure,
        // Add other properties as needed
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Flags(u32);

    impl Flags {
        pub const fn new(flags: u32) -> Self {
            Flags(flags)
        }

        pub fn contains(&self, other: Flags) -> bool {
            (self.0 & other.0) == other.0
        }
    }
    impl std::ops::BitOr for Flags {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            Flags(self.0 | other.0)
        }
    }

    impl Flags {
        pub const kNoAllocate: Flags = Flags::new(1 << 0);
    }

    const kMaxCParameters: i32 = 8;

    pub struct Linkage {}

    impl Linkage {
        pub fn get_simplified_c_descriptor(
            zone: &Zone,
            msig: &MachineSignature,
            flags: Flags,
            properties: OperatorProperties,
        ) -> Box<CallDescriptor> {
            // This method should not be called on unknown architectures.
            if architecture::ARCH == "unknown"{
                panic!("requested C call descriptor on unsupported architecture");
            }

            assert!(properties == OperatorProperties::NoThrow || properties == OperatorProperties::Pure);
            assert!(msig.parameter_count() <= kMaxCParameters as usize);

            let mut locations = LocationSignatureBuilder::new(zone, msig.return_count(), msig.parameter_count());

            #[cfg(not(feature = "v8_enable_fp_params_in_c_linkage"))]
            {
                // Check the types of the signature.
                for i in 0..msig.parameter_count() {
                    let typ = msig.get_param(i);
                    assert!(!is_floating_point(typ.representation()));
                }

                // Check the return types.
                for i in 0..locations.locations.returns.capacity() {
                    let typ = msig.get_return(i);
                    assert!(!is_floating_point(typ.representation()));
                }
            }

            assert!(locations.locations.returns.capacity() <= 2);
            if locations.locations.returns.capacity() > 0 {
                let reg =
                if cfg!(feature = "fp_return_register") {
                    let k_fp_return_register = get_fp_return_register(); // TODO: Implement this function based on arch
                    if is_floating_point(msig.get_return(0).representation()) {
                        k_fp_return_register
                    } else {
                        kReturnRegister0.code()
                    }
                } else {
                    kReturnRegister0.code()
                };

                locations.add_return(LinkageLocation::ForRegister {
                    reg_code: reg,
                    typ: msig.get_return(0),
                });
            }

            if locations.locations.returns.capacity() > 1 {
                assert!(!is_floating_point(msig.get_return(0).representation()));

                locations.add_return(LinkageLocation::ForRegister {
                    reg_code: kReturnRegister1.code(),
                    typ: msig.get_return(1),
                });
            }

            build_parameter_locations(msig, &mut locations);

            let k_callee_save_registers = get_callee_save_registers();
            let k_callee_save_fp_registers = get_callee_save_fp_registers();

            // The target for C calls is always an address (i.e. machine pointer).
            let target_type = MachineType::pointer();
            let target_loc = LinkageLocation::ForAnyRegister { typ: target_type };
            let flags = flags | CallDescriptor::kNoAllocate;

            // TODO(saelo): here we probably want to use a c-call specific tag.
            Box::new(CallDescriptor::new(
                CallDescriptor::CallKind::CallAddress,
                kDefaultCodeEntrypointTag,
                target_type,
                target_loc,
                locations.get(),
                0,
                properties,
                k_callee_save_registers,
                k_callee_save_fp_registers,
                flags,
                "c-call".to_string(),
            ))
        }
    }

    fn build_parameter_locations(msig: &MachineSignature, out_locations: &mut LocationSignatureBuilder) {
        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        build_parameter_locations_windows_x64(msig, out_locations);

        #[cfg(target_arch = "mips64")]
        build_parameter_locations_mips64(msig, out_locations);

        #[cfg(target_arch = "loongarch64")]
        build_parameter_locations_loongarch64(msig, out_locations);

        #[cfg(not(any(all(target_os = "windows", target_arch = "x86_64"), target_arch = "mips64", target_arch = "loongarch64")))]
        build_parameter_locations_general(msig, out_locations);
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    fn build_parameter_locations_windows_x64(
        msig: &MachineSignature,
        out_locations: &mut LocationSignatureBuilder,
    ) {
        let k_fp_param_registers = get_fp_param_registers();
        let k_param_registers = get_param_registers();
        let k_fp_param_register_count = k_fp_param_registers.len();
        let k_param_register_count = k_param_registers.len();
        let stack_shadow_words = 4;
        let mut stack_offset = stack_shadow_words;

        assert_eq!(k_fp_param_register_count, k_param_register_count);

        for i in 0..msig.parameter_count() {
            let typ = msig.get_param(i);
            let spill = i >= k_param_register_count;
            if spill {
                out_locations.add_param(LinkageLocation::ForCallerFrameSlot {
                    offset: -1 - stack_offset as i32,
                    typ,
                });
                stack_offset += 1;
            } else {
                if is_floating_point(typ.representation()) {
                    out_locations.add_param(LinkageLocation::ForRegister {
                        reg_code: k_fp_param_registers[i].code(),
                        typ,
                    });
                } else {
                    out_locations.add_param(LinkageLocation::ForRegister {
                        reg_code: k_param_registers[i].code(),
                        typ,
                    });
                }
            }
        }
    }

    #[cfg(target_arch = "mips64")]
    fn build_parameter_locations_mips64(
        msig: &MachineSignature,
        out_locations: &mut LocationSignatureBuilder,
    ) {
        let k_fp_param_registers = get_fp_param_registers();
        let k_param_registers = get_param_registers();
        let k_fp_param_register_count = k_fp_param_registers.len();
        let k_param_register_count = k_param_registers.len();
        let stack_shadow_words = 0; // MIPS ABI doesn't define a stack shadow
        let mut stack_offset = stack_shadow_words;

        assert_eq!(k_fp_param_register_count, k_param_register_count);

        for i in 0..msig.parameter_count() {
            let typ = msig.get_param(i);
            let spill = i >= k_param_register_count;
            if spill {
                out_locations.add_param(LinkageLocation::ForCallerFrameSlot {
                    offset: -1 - stack_offset as i32,
                    typ,
                });
                stack_offset += 1;
            } else {
                if is_floating_point(typ.representation()) {
                    out_locations.add_param(LinkageLocation::ForRegister {
                        reg_code: k_fp_param_registers[i].code(),
                        typ,
                    });
                } else {
                    out_locations.add_param(LinkageLocation::ForRegister {
                        reg_code: k_param_registers[i].code(),
                        typ,
                    });
                }
            }
        }
    }

    #[cfg(target_arch = "loongarch64")]
    fn build_parameter_locations_loongarch64(
        msig: &MachineSignature,
        out_locations: &mut LocationSignatureBuilder,
    ) {
        let k_fp_param_registers = get_fp_param_registers();
        let k_param_registers = get_param_registers();
        let k_fp_param_register_count = k_fp_param_registers.len();
        let k_param_register_count = k_param_registers.len();

        let stack_shadow_words = 0;
        let mut stack_offset = stack_shadow_words;
        let mut num_params = 0;
        let mut num_fp_params = 0;

        for i in 0..msig.parameter_count() {
            let typ = msig.get_param(i);
            if is_floating_point(typ.representation()) {
                if num_fp_params < k_fp_param_register_count {
                    out_locations.add_param(LinkageLocation::ForRegister {
                        reg_code: k_fp_param_registers[num_fp_params].code(),
                        typ,
                    });
                    num_fp_params += 1;
                } else if num_params < k_param_register_count {
                    out_locations.add_param(LinkageLocation::ForNullRegister {
                        reg_code: -k_param_registers[num_params].code(),
                        typ,
                    });
                    num_params += 1;
                } else {
                    out_locations.add_param(LinkageLocation::ForCallerFrameSlot {
                        offset: -1 - stack_offset as i32,
                        typ,
                    });
                    stack_offset += 1;
                }
            } else {
                if num_params < k_param_register_count {
                    out_locations.add_param(LinkageLocation::ForRegister {
                        reg_code: k_param_registers[num_params].code(),
                        typ,
                    });
                    num_params += 1;
                } else {
                    out_locations.add_param(LinkageLocation::ForCallerFrameSlot {
                        offset: -1 - stack_offset as i32,
                        typ,
                    });
                    stack_offset += 1;
                }
            }
        }
    }

    #[cfg(not(any(all(target_os = "windows", target_arch = "x86_64"), target_arch = "mips64", target_arch = "loongarch64")))]
    fn build_parameter_locations_general(
        msig: &MachineSignature,
        out_locations: &mut LocationSignatureBuilder,
    ) {
        let k_fp_param_registers = get_fp_param_registers();
        let k_param_registers = get_param_registers();
        let k_fp_param_register_count = k_fp_param_registers.len();
        let k_param_register_count = k_param_registers.len();

        let stack_shadow_words = 0;
        let mut stack_offset = stack_shadow_words;
        let mut num_params = 0;
        let mut num_fp_params = 0;

        for i in 0..msig.parameter_count() {
            let typ = msig.get_param(i);
            let spill = if is_floating_point(typ.representation()) {
                num_fp_params >= k_fp_param_register_count
            } else {
                num_params >= k_param_register_count
            };
            if spill {
                out_locations.add_param(LinkageLocation::ForCallerFrameSlot {
                    offset: -1 - stack_offset as i32,
                    typ,
                });
                stack_offset += 1;
            } else {
                if is_floating_point(typ.representation()) {
                    out_locations.add_param(LinkageLocation::ForRegister {
                        reg_code: k_fp_param_registers[num_fp_params].code(),
                        typ,
                    });
                    num_fp_params += 1;
                } else {
                    out_locations.add_param(LinkageLocation::ForRegister {
                        reg_code: k_param_registers[num_params].code(),
                        typ,
                    });
                    num_params += 1;
                }
            }
        }
    }

    // Architecture-specific register lists.  These should be functions to avoid
    // initialization order issues.
    fn get_param_registers() -> Vec<Register> {
        match architecture::ARCH {
            "x64" => {
                #[cfg(target_os = "windows")]
                {
                    vec![
                        registers::x64::rcx,
                        registers::x64::rdx,
                        registers::x64::r8,
                        registers::x64::r9,
                    ]
                }
                #[cfg(not(target_os = "windows"))]
                {
                    vec![
                        Register::new(1), //rdi
                        Register::new(2), //rsi
                        registers::x64::rdx,
                        registers::x64::rcx,
                        registers::x64::r8,
                        registers::x64::r9,
                    ]
                }
            }
            "arm" => vec![registers::arm::r0, registers::arm::r1, registers::arm::r2, registers::arm::r3],
            "arm64" => vec![registers::arm64::x0, registers::arm64::x1, registers::arm64::x2, registers::arm64::x3, registers::arm64::x4, registers::arm64::x5, registers::arm64::x6, registers::arm64::x7],
            "mips64" => vec![registers::mips64::a0, registers::mips64::a1, registers::mips64::a2, registers::mips64::a3, registers::mips64::a4, registers::mips64::a5, registers::mips64::a6, registers::mips64::a7],
            "loong64" => vec![registers::loongarch64::a0, registers::loongarch64::a1, registers::loongarch64::a2, registers::loongarch64::a3, registers::loongarch64::a4, registers::loongarch64::a5, registers::loongarch64::a6, registers::loongarch64::a7],
            "ppc64" => vec![registers::ppc64::r3, registers::ppc64::r4, registers::ppc64::r5, registers::ppc64::r6, registers::ppc64::r7, registers::ppc64::r8, registers::ppc64::r9, registers::ppc64::r10],
            "s390x" => vec![registers::s390x::r2, registers::s390x::r3, registers::s390x::r4, registers::s390x::r5, registers::s390x::r6],
            "riscv" => vec![registers::riscv::a0, registers::riscv::a1, registers::riscv::a2, registers::riscv::a3, registers::riscv