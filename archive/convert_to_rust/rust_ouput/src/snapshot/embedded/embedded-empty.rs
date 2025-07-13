// Converted from V8 C++ source files:
// Header: N/A
// Implementation: embedded-empty.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub static v8_Default_embedded_blob_code_: [u8; 1] = [0];
pub static mut v8_Default_embedded_blob_code_size_: u32 = 0;
pub static v8_Default_embedded_blob_data_: [u8; 1] = [0];
pub static mut v8_Default_embedded_blob_data_size_: u32 = 0;

#[cfg(feature = "v8_enable_drumbrake")]
mod drumbrake {
    type fun_ptr = fn();

    extern "C" {
        static mut Builtins_LoadI8SignExtend: Option<fun_ptr>;
        static mut Builtins_LoadI8ZeroExtend: Option<fun_ptr>;
        static mut Builtins_LoadI16SignExtend: Option<fun_ptr>;
        static mut Builtins_LoadI16ZeroExtend: Option<fun_ptr>;
        static mut Builtins_LoadI32SignExtend: Option<fun_ptr>;
        static mut Builtins_LoadI32ZeroExtend: Option<fun_ptr>;
        static mut Builtins_LoadI64SignExtend: Option<fun_ptr>;
        static mut Builtins_LoadI64ZeroExtend: Option<fun_ptr>;
        static mut Builtins_LoadFloat32: Option<fun_ptr>;
        static mut Builtins_LoadFloat64: Option<fun_ptr>;
        static mut Builtins_LoadLaneI8x16: Option<fun_ptr>;
        static mut Builtins_LoadLaneI16x8: Option<fun_ptr>;
        static mut Builtins_LoadLaneI32x4: Option<fun_ptr>;
        static mut Builtins_LoadLaneI64x2: Option<fun_ptr>;
        static mut Builtins_LoadLaneFloat32x4: Option<fun_ptr>;
        static mut Builtins_LoadLaneFloat64x2: Option<fun_ptr>;
        static mut Builtins_StoreI8: Option<fun_ptr>;
        static mut Builtins_StoreI16: Option<fun_ptr>;
        static mut Builtins_StoreI32: Option<fun_ptr>;
        static mut Builtins_StoreI64: Option<fun_ptr>;
        static mut Builtins_StoreFloat32: Option<fun_ptr>;
        static mut Builtins_StoreFloat64: Option<fun_ptr>;
        static mut Builtins_StoreLaneI8x16: Option<fun_ptr>;
        static mut Builtins_StoreLaneI16x8: Option<fun_ptr>;
        static mut Builtins_StoreLaneI32x4: Option<fun_ptr>;
        static mut Builtins_StoreLaneI64x2: Option<fun_ptr>;
        static mut Builtins_StoreLaneFloat32x4: Option<fun_ptr>;
        static mut Builtins_StoreLaneFloat64x2: Option<fun_ptr>;
    }

    pub fn initialize_builtins() {
        unsafe {
            Builtins_LoadI8SignExtend = None;
            Builtins_LoadI8ZeroExtend = None;
            Builtins_LoadI16SignExtend = None;
            Builtins_LoadI16ZeroExtend = None;
            Builtins_LoadI32SignExtend = None;
            Builtins_LoadI32ZeroExtend = None;
            Builtins_LoadI64SignExtend = None;
            Builtins_LoadI64ZeroExtend = None;
            Builtins_LoadFloat32 = None;
            Builtins_LoadFloat64 = None;
            Builtins_LoadLaneI8x16 = None;
            Builtins_LoadLaneI16x8 = None;
            Builtins_LoadLaneI32x4 = None;
            Builtins_LoadLaneI64x2 = None;
            Builtins_LoadLaneFloat32x4 = None;
            Builtins_LoadLaneFloat64x2 = None;
            Builtins_StoreI8 = None;
            Builtins_StoreI16 = None;
            Builtins_StoreI32 = None;
            Builtins_StoreI64 = None;
            Builtins_StoreFloat32 = None;
            Builtins_StoreFloat64 = None;
            Builtins_StoreLaneI8x16 = None;
            Builtins_StoreLaneI16x8 = None;
            Builtins_StoreLaneI32x4 = None;
            Builtins_StoreLaneI64x2 = None;
            Builtins_StoreLaneFloat32x4 = None;
            Builtins_StoreLaneFloat64x2 = None;
        }
    }
}
