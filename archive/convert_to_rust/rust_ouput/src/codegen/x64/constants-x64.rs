// Converted from V8 C++ source files:
// Header: constants-x64.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    // The actual value of the kRootRegister is offset from the IsolateData's start
    // to take advantage of negative displacement values.
    // On x64, the smallest operand encoding allows int8 offsets, thus we select the
    // bias s.t. the first 32 8-byte slots of IsolateData are can be encoded this
    // way.
    pub const K_ROOT_REGISTER_BIAS: i32 = 128;

    // The maximum size of the code range s.t. pc-relative calls are possible
    // between all Code objects in the range.
    pub const K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB: usize = 2048;
}
