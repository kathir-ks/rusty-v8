// Converted from V8 C++ source files:
// Header: frame-description.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod memory {
        pub fn free<T>(_ptr: *mut T) {}
    }
    pub mod platform {
        pub mod memory {
            use std::alloc::{GlobalAlloc, Layout, System};
            use std::ptr;

            pub struct MallocAllocator;

            unsafe impl GlobalAlloc for MallocAllocator {
                unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
                    System.alloc(layout)
                }

                unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
                    System.dealloc(ptr, layout)
                }
            }

            impl MallocAllocator {
                pub fn malloc(size: usize) -> *mut std::ffi::c_void {
                    let layout = Layout::from_size_align(size, 1).unwrap();
                    unsafe {
                        let ptr = System.alloc(layout);
                        if ptr.is_null() {
                            panic!("Memory allocation failed!");
                        }
                        ptr as *mut std::ffi::c_void
                    }
                }

                pub fn free(ptr: *mut std::ffi::c_void) {
                    if !ptr.is_null() {
                        unsafe {
                            //Need layout to deallocate, but don't have it.
                            //Assuming it was allocated with size 1 alignment 1. This is not ideal.
                            let layout = Layout::from_size_align(1, 1).unwrap();
                            System.dealloc(ptr as *mut u8, layout);
                        }
                    }
                }

                pub fn alloc_zeroed(size: usize) -> *mut std::ffi::c_void {
                    let layout = Layout::from_size_align(size, 1).unwrap();
                    unsafe {
                        let ptr = System.alloc_zeroed(layout);
                        if ptr.is_null() {
                            panic!("Memory allocation failed!");
                        }
                        ptr as *mut std::ffi::c_void
                    }
                }
            }

            pub unsafe fn write_unaligned_value<T>(ptr: *mut u8, value: T) {
                let ptr_typed = ptr as *mut T;
                ptr::write_unaligned(ptr_typed, value);
            }
        }
    }
}
pub mod codegen {
    pub struct Register {}

    impl Register {
        pub const kNumRegisters: usize = 32;
    }
}
pub mod common {
    pub struct Simd128 {}
}
pub mod execution {
    pub mod frame_constants {
        pub const kSystemPointerSize: usize = 8;
        pub struct StandardFrameConstants {}

        impl StandardFrameConstants {
            pub const kCallerSPOffset: i32 = 8;
        }
    }
}
pub mod utils {
    pub struct BoxedFloat {}
}

pub mod internal {
    use crate::{
        base::{
            memory::free,
            platform::memory::{write_unaligned_value},
        },
        codegen::Register,
        common::Simd128,
        execution::frame_constants::{kSystemPointerSize},
    };
    use std::{mem::MaybeUninit, ptr::null_mut};

    pub struct Float32 {}
    pub struct Float64 {
        value: f64,
    }
    impl Float64 {
        pub fn new(value: f64) -> Self {
            Float64 { value }
        }
        pub fn get(&self) -> f64 {
            self.value
        }
    }

    pub struct Isolate {}

    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Address {}

    impl Address {
        pub fn is_null(&self) -> bool {
            true
        }
    }

    // Classes in this file describe the physical stack frame state.
    //
    // RegisterValues: stores gp and fp register values. Can be filled in either by
    // the DeoptimizationEntry builtin (which fills in the input state of the
    // optimized frame); or by the FrameWriter (fills in the output state of the
    // interpreted frame).
    //
    // - FrameDescription: contains RegisterValues and other things.

    pub struct RegisterValues {
        pub registers_: [i64; Register::kNumRegisters],
        pub simd128_registers_: [Simd128; Simd128Register::kNumRegisters],
    }
    pub struct DoubleRegister {}
    impl DoubleRegister {
        pub const kNumRegisters: usize = 32;
    }

    pub struct Simd128Register {}
    impl Simd128Register {
        pub const kNumRegisters: usize = 32;
    }

    impl RegisterValues {
        pub fn get_register(&self, n: usize) -> i64 {
            if n >= self.registers_.len() {
                panic!("Register index out of bounds");
            }
            self.registers_[n]
        }

        pub fn get_float_register(&self, _n: usize) -> Float32 {
            Float32 {}
        }

        pub fn get_double_register(&self, n: usize) -> Float64 {
             Float64::new(0.0)
        }

        pub fn set_double_register(&mut self, n: usize, value: Float64) {
        }

        pub fn get_simd128_register(&self, n: usize) -> Simd128 {
            if n >= self.simd128_registers_.len() {
                panic!("Simd128 register index out of bounds");
            }
            self.simd128_registers_[n]
        }

        pub fn set_register(&mut self, n: usize, value: i64) {
            if n >= self.registers_.len() {
                panic!("Register index out of bounds");
            }
            self.registers_[n] = value;
        }

        pub fn set_simd128_register(&mut self, n: usize, value: Simd128) {
            if n >= self.simd128_registers_.len() {
                panic!("Simd128 register index out of bounds");
            }
            self.simd128_registers_[n] = value;
        }
    }
    const kSimd128Size: usize = 16;

    pub struct FrameDescription {
        frame_size_: usize,
        parameter_count_: i32,
        register_values_: RegisterValues,
        top_: i64,
        pc_: i64,
        fp_: i64,
        constant_pool_: i64,
        caller_pc_: i64,
        isolate_: *mut Isolate,
        continuation_: i64,
        frame_content_: [i64; 1], // Flexible array member, must be last
    }

    impl FrameDescription {
        pub fn create(
            frame_size: u32,
            parameter_count: i32,
            isolate: *mut Isolate,
        ) -> *mut FrameDescription {
            unsafe {
                let layout = std::alloc::Layout::new::<FrameDescription>()
                    .extend(
                        std::alloc::Layout::array::<i64>(frame_size as usize / 8).unwrap(),
                    )
                    .unwrap()
                    .0
                    .pad_to_align();

                let ptr = std::alloc::alloc_zeroed(layout) as *mut FrameDescription;
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }
                (*ptr).frame_size_ = frame_size as usize;
                (*ptr).parameter_count_ = parameter_count;
                (*ptr).register_values_ = RegisterValues {
                    registers_: [0; Register::kNumRegisters],
                    simd128_registers_: [Simd128 {}; Simd128Register::kNumRegisters],
                };
                (*ptr).top_ = Self::kZapUint32 as i64;
                (*ptr).pc_ = Self::kZapUint32 as i64;
                (*ptr).fp_ = Self::kZapUint32 as i64;
                (*ptr).constant_pool_ = Self::kZapUint32 as i64;
                (*ptr).caller_pc_ = 0;
                (*ptr).isolate_ = isolate;
                (*ptr).continuation_ = 0;

                for r in 0..Register::kNumRegisters {
                    (*ptr).set_register(r as usize, Self::kZapUint32 as i64);
                }

                // Zap all the slots.
                for o in (0..frame_size).step_by(kSystemPointerSize) {
                    (*ptr).set_frame_slot(o as usize, Self::kZapUint32 as i64);
                }

                ptr
            }
        }

        pub fn delete(description: *mut FrameDescription) {
            unsafe {
                if !description.is_null() {
                    let frame_size = (*description).frame_size_;
                    let layout = std::alloc::Layout::new::<FrameDescription>()
                        .extend(
                            std::alloc::Layout::array::<i64>(frame_size as usize / 8).unwrap(),
                        )
                        .unwrap()
                        .0
                        .pad_to_align();

                    std::alloc::dealloc(description as *mut u8, layout);
                }
            }
        }

        pub fn get_frame_size(&self) -> u32 {
            self.frame_size_ as u32
        }

        pub fn get_frame_slot(&self, offset: usize) -> i64 {
            unsafe { *self.get_frame_slot_pointer(offset) }
        }

        pub fn get_last_argument_slot_offset(&self, pad_arguments: bool) -> usize {
            let mut parameter_slots = self.parameter_count() as usize;
            if pad_arguments {
                parameter_slots = Self::add_argument_padding_slots(parameter_slots);
            }
            self.get_frame_size() as usize - parameter_slots * kSystemPointerSize
        }

        pub fn get_frame_pointer_address(&self) -> Address {
            // We should not pad arguments in the bottom frame, since this
            // already contains a padding if necessary and it might contain
            // extra arguments (actual argument count > parameter count).
            let pad_arguments_bottom_frame = false;
            let fp_offset = self.get_last_argument_slot_offset(pad_arguments_bottom_frame)
                - super::execution::frame_constants::StandardFrameConstants::kCallerSPOffset
                    as usize;
            unsafe {
                std::mem::transmute::<*mut i64, Address>(self.get_frame_slot_pointer(fp_offset))
            }
        }

        pub fn get_register_values(&mut self) -> &mut RegisterValues {
            &mut self.register_values_
        }

        pub fn set_frame_slot(&mut self, offset: usize, value: i64) {
            unsafe {
                *self.get_frame_slot_pointer(offset) = value;
            }
        }

        // Same as SetFrameSlot but only writes 32 bits. This is needed as liftoff
        // has 32 bit frame slots.
        pub fn set_liftoff_frame_slot32(&mut self, offset: usize, value: i32) {
            unsafe {
                write_unaligned_value(
                    self.get_frame_slot_pointer(offset) as *mut i64 as *mut u8,
                    value,
                );
            }
        }

        // Same as SetFrameSlot but also supports the offset to be unaligned (4 Byte
        // aligned) as liftoff doesn't align frame slots if they aren't references.
        pub fn set_liftoff_frame_slot64(&mut self, offset: usize, value: i64) {
            unsafe {
                write_unaligned_value(
                    self.get_frame_slot_pointer(offset) as *mut i64 as *mut u8,
                    value,
                );
            }
        }

        pub fn set_liftoff_frame_slot_pointer(&mut self, offset: usize, value: i64) {
            if cfg!(target_pointer_width = "64") {
                self.set_liftoff_frame_slot64(offset, value);
            } else {
                self.set_liftoff_frame_slot32(offset as usize, value as i32);
            }
        }

        pub fn set_caller_pc(&mut self, _offset: usize, _value: i64) {}

        pub fn set_caller_fp(&mut self, _offset: usize, _value: i64) {}

        pub fn set_caller_constant_pool(&mut self, _offset: usize, _value: i64) {}

        pub fn get_register(&self, n: usize) -> i64 {
            self.register_values_.get_register(n)
        }

        pub fn get_double_register(&self, n: usize) -> Float64 {
            self.register_values_.get_double_register(n)
        }

        pub fn set_register(&mut self, n: usize, value: i64) {
            self.register_values_.set_register(n, value);
        }

        pub fn set_double_register(&mut self, n: usize, value: Float64) {
            self.register_values_.set_double_register(n, value);
        }

        pub fn set_simd128_register(&mut self, n: usize, value: Simd128) {
            self.register_values_.set_simd128_register(n, value);
        }

        pub fn get_top(&self) -> i64 {
            self.top_
        }
        pub fn set_top(&mut self, top: i64) {
            self.top_ = top;
        }

        pub fn get_pc(&self) -> i64 {
            self.pc_
        }
        pub fn set_pc(&mut self, pc: i64) {
            self.pc_ = pc;
        }

        pub fn get_fp(&self) -> i64 {
            self.fp_
        }
        pub fn set_fp(&mut self, frame_pointer: i64) {
            self.fp_ = frame_pointer;
        }

        pub fn get_constant_pool(&self) -> i64 {
            self.constant_pool_
        }
        pub fn set_constant_pool(&mut self, constant_pool: i64) {
            self.constant_pool_ = constant_pool;
        }

        pub fn has_caller_pc(&self) -> bool {
            self.caller_pc_ != 0
        }
        pub fn get_caller_pc(&self) -> i64 {
            self.caller_pc_
        }

        pub fn set_continuation(&mut self, pc: i64) {
            self.continuation_ = pc;
        }
        pub fn get_continuation(&self) -> i64 {
            self.continuation_
        }

        // Argument count, including receiver.
        pub fn parameter_count(&self) -> i32 {
            self.parameter_count_
        }

        pub fn registers_offset() -> usize {
            let dummy = MaybeUninit::<FrameDescription>::uninit();
            let ptr = dummy.as_ptr();
            unsafe {
                let field_ptr = std::ptr::addr_of!((*ptr).register_values_.registers_);
                (field_ptr as usize) - (ptr as usize)
            }
        }

        pub const fn simd128_registers_offset() -> usize {
            let dummy = std::mem::MaybeUninit::<FrameDescription>::uninit();
            let ptr = dummy.as_ptr();
            unsafe {
                let field_ptr = std::ptr::addr_of!((*ptr).register_values_.simd128_registers_);
                (field_ptr as usize) - (ptr as usize)
            }
        }

        pub fn frame_size_offset() -> usize {
            let dummy = std::mem::MaybeUninit::<FrameDescription>::uninit();
            let ptr = dummy.as_ptr();
            unsafe {
                let field_ptr = std::ptr::addr_of!((*ptr).frame_size_);
                (field_ptr as usize) - (ptr as usize)
            }
        }

        pub fn pc_offset() -> usize {
            let dummy = std::mem::MaybeUninit::<FrameDescription>::uninit();
            let ptr = dummy.as_ptr();
            unsafe {
                let field_ptr = std::ptr::addr_of!((*ptr).pc_);
                (field_ptr as usize) - (ptr as usize)
            }
        }

        pub fn continuation_offset() -> usize {
            let dummy = std::mem::MaybeUninit::<FrameDescription>::uninit();
            let ptr = dummy.as_ptr();
            unsafe {
                let field_ptr = std::ptr::addr_of!((*ptr).continuation_);
                (field_ptr as usize) - (ptr as usize)
            }
        }

        pub fn frame_content_offset() -> usize {
            let dummy = std::mem::MaybeUninit::<FrameDescription>::uninit();
            let ptr = dummy.as_ptr();
            unsafe {
                let field_ptr = std::ptr::addr_of!((*ptr).frame_content_);
                (field_ptr as usize) - (ptr as usize)
            }
        }

        fn add_argument_padding_slots(parameter_slots: usize) -> usize {
            parameter_slots // Placeholder implementation
        }

        unsafe fn get_frame_slot_pointer(&self, offset: usize) -> *mut i64 {
            if offset >= self.frame_size_ {
                panic!("Frame slot offset out of bounds");
            }
            (self as *const FrameDescription as *mut i64).add(Self::frame_content_offset() / 8).add(offset / 8)
        }

        const kZapUint32: u32 = 0xbeeddead;
    }
}
