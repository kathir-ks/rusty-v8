// Converted from V8 C++ source files:
// Header: constants-ia32.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod constants_ia32 {

// The actual value of the kRootRegister is offset from the IsolateData's start
// to take advantage of negative displacement values.
pub const K_ROOT_REGISTER_BIAS: i32 = 128;

// The maximum size of the code range s.t. pc-relative calls are possible
// between all Code objects in the range.
pub const K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB: usize = 0;

} // namespace internal
   //} // namespace v8
