// Converted from V8 C++ source files:
// Header: wasm-call-descriptors.h
// Implementation: wasm-call-descriptors.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

mod common {
    pub mod globals {}
}

mod compiler {
    pub struct CallDescriptor {}
    use crate::execution::builtins::Builtin;
    use crate::compiler::wasm_call_descriptors::StubCallMode;
    use crate::zone::Zone;

    pub fn GetBuiltinCallDescriptor(builtin: Builtin, zone: &Zone, call_mode: StubCallMode) -> *mut CallDescriptor {
        GetBuiltinCallDescriptor_internal(builtin, zone, call_mode, false)
    }

    pub fn GetBuiltinCallDescriptor_internal(builtin: Builtin, zone: &Zone, call_mode: StubCallMode, needs_frame_state: bool) -> *mut CallDescriptor {
        // Simulate creating a CallDescriptor on the zone.
        Box::into_raw(Box::new(CallDescriptor {}))
    }
}

mod execution {
    pub mod builtins {
        pub enum Builtin {
            kBigIntToI64,
            kBigIntToI32Pair,
        }
    }
}

mod zone {
    use crate::v8::internal::AccountingAllocator;
    pub struct Zone {
        allocator: *mut AccountingAllocator,
        name: String,
    }

    impl Zone {
        pub fn new(allocator: *mut AccountingAllocator, name: &str) -> Self {
            Zone {
                allocator,
                name: name.to_string(),
            }
        }
    }
}

pub mod v8 {
    pub mod internal {
        pub struct AccountingAllocator {}
    }
}

pub mod base {
    pub struct Flags {}
}

pub mod utils {
    pub mod escapes {}
}

pub mod strings {
    pub struct String_ExternalOneByteStringResource {}
}

#[derive(PartialEq, Eq)]
pub enum MaybeIndirectHandle<T> {
    None,
    Some(T),
}

pub struct V8_EXPORT_PRIVATE {}

#[cfg(target_arch = "x86_64")]
const V8_TARGET_ARCH_32_BIT: bool = false;
#[cfg(target_arch = "x86")]
const V8_TARGET_ARCH_32_BIT: bool = true;
#[cfg(target_arch = "arm")]
const V8_TARGET_ARCH_32_BIT: bool = true;

pub enum StubCallMode {
    kCallBuiltinPointer,
}

pub struct WasmCallDescriptors {
    zone_: Box<Zone>,
    bigint_to_i64_descriptor_: *mut compiler::CallDescriptor,
    bigint_to_i64_descriptor_with_framestate_: *mut compiler::CallDescriptor,

    #[cfg(all(target_arch = "x86", target_pointer_width = "32"))]
    bigint_to_i32pair_descriptor_: *mut compiler::CallDescriptor,
    #[cfg(all(target_arch = "x86", target_pointer_width = "32"))]
    bigint_to_i32pair_descriptor_with_framestate_: *mut compiler::CallDescriptor,

    #[cfg(all(target_arch = "arm", target_pointer_width = "32"))]
    bigint_to_i32pair_descriptor_: *mut compiler::CallDescriptor,
    #[cfg(all(target_arch = "arm", target_pointer_width = "32"))]
    bigint_to_i32pair_descriptor_with_framestate_: *mut compiler::CallDescriptor,
}

impl WasmCallDescriptors {
    pub fn new(allocator: *mut v8::internal::AccountingAllocator) -> Self {
        let zone = Box::new(Zone::new(allocator, "wasm_call_descriptors"));
        let bigint_to_i64_descriptor = compiler::GetBuiltinCallDescriptor(
            execution::builtins::Builtin::kBigIntToI64,
            &zone,
            StubCallMode::kCallBuiltinPointer,
        );
        let bigint_to_i64_descriptor_with_framestate = compiler::GetBuiltinCallDescriptor_internal(
            execution::builtins::Builtin::kBigIntToI64,
            &zone,
            StubCallMode::kCallBuiltinPointer,
            true,
        );

        #[cfg(all(target_arch = "x86", target_pointer_width = "32"))]
        let bigint_to_i32pair_descriptor = compiler::GetBuiltinCallDescriptor(
            execution::builtins::Builtin::kBigIntToI32Pair,
            &zone,
            StubCallMode::kCallBuiltinPointer,
        );
        #[cfg(all(target_arch = "x86", target_pointer_width = "32"))]
        let bigint_to_i32pair_descriptor_with_framestate = compiler::GetBuiltinCallDescriptor_internal(
            execution::builtins::Builtin::kBigIntToI32Pair,
            &zone,
            StubCallMode::kCallBuiltinPointer,
            true,
        );
        #[cfg(all(target_arch = "arm", target_pointer_width = "32"))]
        let bigint_to_i32pair_descriptor = compiler::GetBuiltinCallDescriptor(
            execution::builtins::Builtin::kBigIntToI32Pair,
            &zone,
            StubCallMode::kCallBuiltinPointer,
        );
        #[cfg(all(target_arch = "arm", target_pointer_width = "32"))]
        let bigint_to_i32pair_descriptor_with_framestate = compiler::GetBuiltinCallDescriptor_internal(
            execution::builtins::Builtin::kBigIntToI32Pair,
            &zone,
            StubCallMode::kCallBuiltinPointer,
            true,
        );

        WasmCallDescriptors {
            zone_: zone,
            bigint_to_i64_descriptor_: bigint_to_i64_descriptor,
            bigint_to_i64_descriptor_with_framestate_: bigint_to_i64_descriptor_with_framestate,
            #[cfg(all(target_arch = "x86", target_pointer_width = "32"))]
            bigint_to_i32pair_descriptor_: bigint_to_i32pair_descriptor,
            #[cfg(all(target_arch = "x86", target_pointer_width = "32"))]
            bigint_to_i32pair_descriptor_with_framestate_: bigint_to_i32pair_descriptor_with_framestate,
            #[cfg(all(target_arch = "arm", target_pointer_width = "32"))]
            bigint_to_i32pair_descriptor_: bigint_to_i32pair_descriptor,
            #[cfg(all(target_arch = "arm", target_pointer_width = "32"))]
            bigint_to_i32pair_descriptor_with_framestate_: bigint_to_i32pair_descriptor_with_framestate,
        }
    }

    pub fn GetBigIntToI64Descriptor(&self, needs_frame_state: bool) -> *mut compiler::CallDescriptor {
        if needs_frame_state {
            self.bigint_to_i64_descriptor_with_framestate_
        } else {
            self.bigint_to_i64_descriptor_
        }
    }

    #[cfg(all(target_arch = "x86", target_pointer_width = "32"))]
    pub fn GetLoweredCallDescriptor(
        &self,
        original: *const compiler::CallDescriptor,
    ) -> *mut compiler::CallDescriptor {
        if original == self.bigint_to_i64_descriptor_ as *const compiler::CallDescriptor {
            return self.bigint_to_i32pair_descriptor_;
        }
        if original == self.bigint_to_i64_descriptor_with_framestate_ as *const compiler::CallDescriptor {
            return self.bigint_to_i32pair_descriptor_with_framestate_;
        }
        std::ptr::null_mut()
    }
    #[cfg(all(target_arch = "arm", target_pointer_width = "32"))]
    pub fn GetLoweredCallDescriptor(
        &self,
        original: *const compiler::CallDescriptor,
    ) -> *mut compiler::CallDescriptor {
        if original == self.bigint_to_i64_descriptor_ as *const compiler::CallDescriptor {
            return self.bigint_to_i32pair_descriptor_;
        }
        if original == self.bigint_to_i64_descriptor_with_framestate_ as *const compiler::CallDescriptor {
            return self.bigint_to_i32pair_descriptor_with_framestate_;
        }
        std::ptr::null_mut()
    }

    #[cfg(not(any(all(target_arch = "x86", target_pointer_width = "32"),all(target_arch = "arm", target_pointer_width = "32"))))]
    pub fn GetLoweredCallDescriptor(
        &self,
        original: *const compiler::CallDescriptor,
    ) -> *mut compiler::CallDescriptor {
        panic!("UNREACHABLE");
    }
}
