// Converted from V8 C++ source files:
// Header: build_config.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct V8 {}
#[allow(dead_code)]
pub const CAN_USE_ARMV7_INSTRUCTIONS: i32 = 1;
#[allow(dead_code)]
pub const CAN_USE_SUDIV: i32 = 1;
#[allow(dead_code)]
pub const CAN_USE_VFP3_INSTRUCTIONS: i32 = 1;
#[allow(dead_code)]
pub const CAN_USE_ARMV8_INSTRUCTIONS: i32 = 1;
#[allow(dead_code)]
pub const V8_HAS_PTHREAD_JIT_WRITE_PROTECT: i32 = 0;
#[allow(dead_code)]
pub const V8_HAS_BECORE_JIT_WRITE_PROTECT: i32 = 0;
#[allow(dead_code)]
pub const V8_HAS_PKU_JIT_WRITE_PROTECT: i32 = 0;
#[allow(dead_code)]
pub const V8_TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK: bool = false;
#[allow(dead_code)]
pub const kReturnAddressStackSlotCount: i32 = if V8_TARGET_ARCH_STORES_RETURN_ADDRESS_ON_STACK {
    1
} else {
    0
};
#[allow(dead_code)]
pub const kPageSizeBits: i32 = 18;
#[allow(dead_code)]
pub const kRegularPageSize: i32 = 1 << kPageSizeBits;
#[allow(dead_code)]
pub const kMinimumOSPageSize: i32 = 4 * 1024;
