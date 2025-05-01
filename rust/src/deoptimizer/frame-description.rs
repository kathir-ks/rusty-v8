// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod frame_description {
    use std::mem;
    use std::ptr;

    //use crate::base::memory::*; // Requires more context on base::memory
    //use crate::base::platform::memory::*; // Requires more context on base::platform::memory
    //use crate::codegen::register::*; // Requires more context on codegen::register
    //use crate::common::simd128::*; // Requires more context on common::simd128
    //use crate::execution::frame_constants::*; // Requires more context on execution::frame_constants
    //use crate::utils::boxed_float::*; // Requires more context on utils::boxed_float

    // Placeholder types.  Need to be replaced with actual implementations.
    type Address = usize; // Placeholder
    type Isolate = usize; // Placeholder
    type Float32 = f32;
    type Float64 = f64;
    type Simd128 = [u8; 16]; // Assuming it's a 128-bit value
    const kSimd128Size: usize = 16;
    const kSystemPointerSize: usize = mem::size_of::<usize>();

    //TODO: Implement Register enum properly to reflect the C++ implementation
    #[derive(Debug, Copy, Clone)]
    pub struct Register {
        num: usize,
    }
    impl Register {
        pub const kNumRegisters: usize = 32; // example value
    }

    #[derive(Debug, Copy, Clone)]
    pub struct DoubleRegister {
        num: usize,
    }
    impl DoubleRegister {
        pub const kNumRegisters: usize = 32; // example value
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Simd128Register {
        num: usize,
    }
    impl Simd128Register {
        pub const kNumRegisters: usize = 32; // example value
    }

    const kZapUint32: usize = 0xbeeddead; // converted to usize

    #[derive(Debug)]
    pub struct RegisterValues {
        pub registers_: [usize; Register::kNumRegisters],
        // Generated code writes directly into the following array, make sure the
        // element size matches what the machine instructions expect.
        // TODO: static_assert(sizeof(Simd128) == kSimd128Size, "size mismatch");

        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        pub double_registers_: [Float64; DoubleRegister::kNumRegisters],
        pub simd128_registers_: [Simd128; Simd128Register::kNumRegisters],

        #[cfg(not(any(target_arch = "riscv64", target_arch = "riscv32")))]
        pub simd128_registers_: [Simd128; Simd128Register::kNumRegisters],
    }

    impl RegisterValues {
        pub fn get_register(&self, n: usize) -> usize {
            assert!(n < self.registers_.len());
            self.registers_[n]
        }

        pub fn get_float_register(&self, _n: usize) -> Float32 {
            todo!()
        }
        pub fn get_double_register(&self, _n: usize) -> Float64 {
            todo!()
        }

        pub fn set_double_register(&mut self, _n: usize, _value: Float64) {
             todo!()
        }

        pub fn get_simd128_register(&self, n: usize) -> Simd128 {
            assert!(n < self.simd128_registers_.len());
            self.simd128_registers_[n]
        }

        pub fn set_register(&mut self, n: usize, value: usize) {
            assert!(n < self.registers_.len());
            self.registers_[n] = value;
        }

        pub fn set_simd128_register(&mut self, n: usize, value: Simd128) {
            assert!(n < self.simd128_registers_.len());
            self.simd128_registers_[n] = value;
        }
    }

    pub struct FrameDescription {
        frame_size_: usize, // Number of bytes.
        parameter_count_: i32,
        register_values_: RegisterValues,
        top_: usize,
        pc_: usize,
        fp_: usize,
        constant_pool_: usize,
        caller_pc_: usize,
        isolate_: Isolate,
        continuation_: usize,
        frame_content_: Vec<usize>, // intptr_t frame_content_[1]; needs to be dynamically sized
    }

    impl FrameDescription {
        pub fn create(frame_size: u32, parameter_count: i32, isolate: Isolate) -> Box<FrameDescription> {
            let frame_size_usize = frame_size as usize;
            let total_size = mem::size_of::<FrameDescription>() + frame_size_usize - kSystemPointerSize;

            // Allocate the memory using Box::new and a slice.
            let mut buffer: Vec<u8> = vec![0u8; total_size];

            // Initialize the FrameDescription struct.  This is unsafe because we're
            // transmuting a raw pointer into a FrameDescription.  This is fine as long
            // as the FrameDescription is POD (plain old data).
            let frame_description = {
                let ptr = buffer.as_mut_ptr() as *mut FrameDescription;
                unsafe {
                    ptr::write(ptr, FrameDescription {
                        frame_size_: frame_size_usize,
                        parameter_count_: parameter_count,
                        register_values_: RegisterValues {
                            registers_: [0; Register::kNumRegisters],
                            #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
                            double_registers_: [0.0; DoubleRegister::kNumRegisters],
                            simd128_registers_: [[0; 16]; Simd128Register::kNumRegisters],
                            #[cfg(not(any(target_arch = "riscv64", target_arch = "riscv32")))]
                            simd128_registers_: [[0; 16]; Simd128Register::kNumRegisters],
                        },
                        top_: kZapUint32,
                        pc_: kZapUint32,
                        fp_: kZapUint32,
                        constant_pool_: kZapUint32,
                        caller_pc_: 0,
                        isolate_: isolate,
                        continuation_: 0,
                        // Create frame content Vec
                        frame_content_: vec![0; frame_size_usize / kSystemPointerSize],
                    });
                    Box::from_raw(ptr)
                }
            };

            // Zero out registers after creating FrameDescription
            for r in 0..Register::kNumRegisters {
                frame_description.set_register(r, kZapUint32);
            }

            // Zero out frame slots after creating FrameDescription
            for o in (0..frame_size).step_by(kSystemPointerSize) {
                frame_description.set_frame_slot(o as u32, kZapUint32);
            }

            frame_description
        }

        pub fn get_frame_size(&self) -> u32 {
            assert_eq!(self.frame_size_ as u32, self.frame_size_ as u32);
            self.frame_size_ as u32
        }

        pub fn get_frame_slot(&self, offset: u32) -> usize {
            *self.get_frame_slot_pointer(offset)
        }

        pub fn get_last_argument_slot_offset(&self, pad_arguments: bool) -> u32 {
            let mut parameter_slots = self.parameter_count() as usize;
            if pad_arguments {
                parameter_slots = self.add_argument_padding_slots(parameter_slots);
            }
            (self.get_frame_size() as usize - parameter_slots * kSystemPointerSize) as u32
        }

        pub fn get_frame_pointer_address(&self) -> Address {
             //TODO: Implement StandardFrameConstants

            // We should not pad arguments in the bottom frame, since this
            // already contains a padding if necessary and it might contain
            // extra arguments (actual argument count > parameter count).
            const PAD_ARGUMENTS_BOTTOM_FRAME: bool = false;

            //TODO: Implement StandardFrameConstants::kCallerSPOffset. This is a placeholder
            const K_CALLER_SP_OFFSET: u32 = 8;

            let fp_offset = self.get_last_argument_slot_offset(PAD_ARGUMENTS_BOTTOM_FRAME) as i32 -
                            K_CALLER_SP_OFFSET as i32;
            let frame_slot_ptr = self.get_frame_slot_pointer(fp_offset as u32);
            frame_slot_ptr as Address
        }

        pub fn get_register_values(&mut self) -> &mut RegisterValues {
            &mut self.register_values_
        }

        pub fn set_frame_slot(&mut self, offset: u32, value: usize) {
            *self.get_frame_slot_pointer(offset) = value;
        }

        // Same as SetFrameSlot but only writes 32 bits. This is needed as liftoff
        // has 32 bit frame slots.
        pub fn set_liftoff_frame_slot32(&mut self, offset: u32, value: i32) {
            //TODO: Implement base::WriteUnalignedValue if necessary
            let ptr = self.get_frame_slot_pointer(offset) as *mut i32;
            unsafe {
                ptr::write_unaligned(ptr, value);
            }
        }

        // Same as SetFrameSlot but also supports the offset to be unaligned (4 Byte
        // aligned) as liftoff doesn't align frame slots if they aren't references.
        pub fn set_liftoff_frame_slot64(&mut self, offset: u32, value: i64) {
            //TODO: Implement base::WriteUnalignedValue if necessary
            let ptr = self.get_frame_slot_pointer(offset) as *mut i64;
            unsafe {
                ptr::write_unaligned(ptr, value);
            }
        }

        pub fn set_liftoff_frame_slot_pointer(&mut self, offset: u32, value: usize) {
            if mem::size_of::<usize>() == 8 {
                self.set_liftoff_frame_slot64(offset, value as i64);
            } else {
                self.set_liftoff_frame_slot32(offset, value as i32);
            }
        }

        pub fn set_caller_pc(&mut self, _offset: u32, _value: usize) {
             todo!()
        }

        pub fn set_caller_fp(&mut self, _offset: u32, _value: usize) {
            todo!()
        }

        pub fn set_caller_constant_pool(&mut self, _offset: u32, _value: usize) {
             todo!()
        }

        pub fn get_register(&self, n: usize) -> usize {
            self.register_values_.get_register(n)
        }

        pub fn get_double_register(&self, n: usize) -> Float64 {
            self.register_values_.get_double_register(n)
        }

        pub fn set_register(&mut self, n: usize, value: usize) {
            self.register_values_.set_register(n, value);
        }

        pub fn set_double_register(&mut self, n: usize, value: Float64) {
             self.register_values_.set_double_register(n, value);
        }

        pub fn set_simd128_register(&mut self, n: usize, value: Simd128) {
            self.register_values_.set_simd128_register(n, value);
        }

        pub fn get_top(&self) -> usize {
            self.top_
        }

        pub fn set_top(&mut self, top: usize) {
            self.top_ = top;
        }

        pub fn get_pc(&self) -> usize {
            self.pc_
        }

        pub fn set_pc(&mut self, pc: usize) {
            self.pc_ = pc;
        }

        pub fn get_fp(&self) -> usize {
            self.fp_
        }

        pub fn set_fp(&mut self, frame_pointer: usize) {
            self.fp_ = frame_pointer;
        }

        pub fn get_constant_pool(&self) -> usize {
            self.constant_pool_
        }

        pub fn set_constant_pool(&mut self, constant_pool: usize) {
            self.constant_pool_ = constant_pool;
        }

        pub fn has_caller_pc(&self) -> bool {
            self.caller_pc_ != 0
        }

        pub fn get_caller_pc(&self) -> usize {
            self.caller_pc_
        }

        pub fn set_continuation(&mut self, pc: usize) {
            self.continuation_ = pc;
        }

        pub fn get_continuation(&self) -> usize {
            self.continuation_
        }

        // Argument count, including receiver.
        pub fn parameter_count(&self) -> i32 {
            self.parameter_count_
        }

        pub fn registers_offset() -> usize {
            let dummy = unsafe { mem::zeroed::<FrameDescription>() };
            let field_ptr = &dummy.register_values_.registers_ as *const _ as usize;
            let struct_ptr = &dummy as *const FrameDescription as usize;
            field_ptr - struct_ptr
        }
        
        #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
        pub const fn double_registers_offset() -> usize {
            let dummy = unsafe { mem::zeroed::<FrameDescription>() };
            let field_ptr = &dummy.register_values_.double_registers_ as *const _ as usize;
            let struct_ptr = &dummy as *const FrameDescription as usize;
            field_ptr - struct_ptr
        }

        pub const fn simd128_registers_offset() -> usize {
            let dummy = unsafe { mem::zeroed::<FrameDescription>() };
            let field_ptr = &dummy.register_values_.simd128_registers_ as *const _ as usize;
            let struct_ptr = &dummy as *const FrameDescription as usize;
            field_ptr - struct_ptr
        }

        pub fn frame_size_offset() -> usize {
            let dummy = unsafe { mem::zeroed::<FrameDescription>() };
            let field_ptr = &dummy.frame_size_ as *const _ as usize;
            let struct_ptr = &dummy as *const FrameDescription as usize;
            field_ptr - struct_ptr
        }

        pub fn pc_offset() -> usize {
            let dummy = unsafe { mem::zeroed::<FrameDescription>() };
            let field_ptr = &dummy.pc_ as *const _ as usize;
            let struct_ptr = &dummy as *const FrameDescription as usize;
            field_ptr - struct_ptr
        }

        pub fn continuation_offset() -> usize {
            let dummy = unsafe { mem::zeroed::<FrameDescription>() };
            let field_ptr = &dummy.continuation_ as *const _ as usize;
            let struct_ptr = &dummy as *const FrameDescription as usize;
            field_ptr - struct_ptr
        }

        pub fn frame_content_offset() -> usize {
           let dummy = unsafe { mem::zeroed::<FrameDescription>() };
           let field_ptr = &dummy.frame_content_ as *const _ as usize;
           let struct_ptr = &dummy as *const FrameDescription as usize;
           field_ptr - struct_ptr
        }

        fn add_argument_padding_slots(&self, parameter_slots: usize) -> usize {
            //Placeholder implementation
            parameter_slots
        }

        fn get_frame_slot_pointer(&mut self, offset: u32) -> *mut usize {
            assert!(offset as usize <= self.frame_size_);
            let base_ptr = self as *mut FrameDescription as usize;
            let slot_ptr = base_ptr + Self::frame_content_offset() + offset as usize;
            slot_ptr as *mut usize
        }
    }
}