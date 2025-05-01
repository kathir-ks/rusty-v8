// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This translation assumes the V8_TARGET_ARCH_IA32 is always enabled for
// this specific Rust translation. Feature flags or conditional compilation
// could be added to handle other architectures if needed.

mod base {
    use std::mem::size_of;
    use std::ptr::addr_of_mut;

    #[inline]
    pub fn read_unaligned<T: Copy>(ptr: *const T) -> T {
        unsafe {
            std::ptr::read_unaligned(ptr)
        }
    }

    #[inline]
    pub fn write_unaligned<T>(ptr: *mut T, value: T) {
        unsafe {
            std::ptr::write_unaligned(ptr, value)
        }
    }

    pub unsafe fn reinterpret_cast<T, U>(src: &T) -> &U {
        &*(src as *const T as *const U)
    }

    pub unsafe fn reinterpret_cast_mut<T, U>(src: &mut T) -> &mut U {
        &mut *(src as *mut T as *mut U)
    }

    pub fn size_of_val<T>(val: &T) -> usize {
        size_of::<T>()
    }

    pub unsafe fn addr_of<T>(location: &T) -> *const T {
        location as *const T
    }

    pub unsafe fn addr_of_mut<T>(location: &mut T) -> *mut T {
        location as *mut T
    }
}

mod deoptimizer {
    pub mod deoptimizer_arch_ia32 {
        pub const EAGER_DEOPT_EXIT_SIZE: usize = 5;
        pub const LAZY_DEOPT_EXIT_SIZE: usize = 5;
        pub const ADAPT_SHADOW_STACK_OFFSET_TO_SUBTRACT: usize = 0;
    }

    use std::any::Any;
    use std::fmt;
    use std::mem;
    use std::ops::{Index, IndexMut};

    pub trait DeoptimizerTrait {
        fn patch_to_jump(pc: usize, new_pc: usize);
    }

    pub struct Deoptimizer {}

    impl Deoptimizer {
        // static
        // Note: This function is marked as UNREACHABLE in the original C++ code.
        // In Rust, we can either panic or return an error. Here, we panic to
        // maintain the original intent.
        pub fn patch_to_jump(_pc: usize, _new_pc: usize) {
            panic!("UNREACHABLE");
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Float32(pub f32);

    #[derive(Debug, Copy, Clone)]
    pub struct Float64(pub f64);

    const SIMD128_REGISTERS_SIZE: usize = 16;

    #[derive(Debug)]
    pub struct RegisterValues {
        simd128_registers_: [u8; 16 * SIMD128_REGISTERS_SIZE], // Assuming 16 registers
    }

    impl RegisterValues {
        pub fn new() -> Self {
            RegisterValues {
                simd128_registers_: [0; 16 * SIMD128_REGISTERS_SIZE],
            }
        }

        pub fn get_float_register(&self, n: usize) -> Float32 {
            let offset = n * SIMD128_REGISTERS_SIZE;
            let ptr = &self.simd128_registers_[offset] as *const u8;
            Float32(unsafe { base::read_unaligned(ptr as *const f32) })
        }

        pub fn get_double_register(&self, n: usize) -> Float64 {
            assert!(n < 16);
            let offset = n * SIMD128_REGISTERS_SIZE;
            let ptr = &self.simd128_registers_[offset] as *const u8;
            Float64(unsafe { base::read_unaligned(ptr as *const f64) })
        }

        pub fn set_double_register(&mut self, n: usize, value: Float64) {
            assert!(n < 16);
            let offset = n * SIMD128_REGISTERS_SIZE;
            let ptr = &mut self.simd128_registers_[offset] as *mut u8;
            unsafe {
                base::write_unaligned(ptr as *mut f64, value.0);
            }
        }
    }

    const FRAME_SLOT_SIZE: usize = 8; // Size of intptr_t

    pub struct FrameDescription {
        frame_: Vec<i64>,
        pc_: i64,
    }

    impl FrameDescription {
        pub fn new(size: usize) -> Self {
             FrameDescription {
                frame_: vec![0; size],
                pc_: 0,
            }
        }

        fn set_frame_slot(&mut self, offset: usize, value: i64) {
            self.frame_[offset / FRAME_SLOT_SIZE] = value;
        }

        pub fn set_caller_pc(&mut self, offset: usize, value: i64) {
            self.set_frame_slot(offset, value);
        }

        pub fn set_caller_fp(&mut self, offset: usize, value: i64) {
            self.set_frame_slot(offset, value);
        }

        pub fn set_caller_constant_pool(&mut self, _offset: usize, _value: i64) {
            // No embedded constant pool support.
            panic!("UNREACHABLE");
        }

        pub fn set_pc(&mut self, pc: i64) {
            self.pc_ = pc;
        }
    }
} // namespace deoptimizer