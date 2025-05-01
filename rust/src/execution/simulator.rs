// Copyright 2009 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/execution/simulator.h

#![allow(dead_code)] // Suppress warnings for unused code

use std::ops::Deref;

use crate::common::globals::*;
use crate::objects::code::Code;

#[cfg(not(feature = "simulator"))]
use crate::base::platform::Platform;
#[cfg(not(feature = "simulator"))]
use crate::execution::isolate::Isolate;
#[cfg(not(feature = "simulator"))]
use crate::utils::utils::Utils;

#[cfg(all(
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    target_arch = "aarch64",
))]
mod arm64;

#[cfg(all(
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    target_arch = "arm",
))]
mod arm;

#[cfg(all(
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    target_arch = "powerpc64",
))]
mod ppc;

#[cfg(all(
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    target_arch = "mips64",
))]
mod mips64;

#[cfg(all(
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    target_arch = "loongarch64",
))]
mod loong64;

#[cfg(all(
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    target_arch = "s390x",
))]
mod s390;

#[cfg(all(
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    any(target_arch = "riscv32", target_arch = "riscv64"),
))]
mod riscv;

#[cfg(all(
    target_arch = "x86",
    not(target_arch = "x86_64"),
))]
compile_error!("Unsupported target architecture.");

#[cfg(all(
    not(target_arch = "x86"),
    target_arch = "x86_64",
))]
compile_error!("Unsupported target architecture.");

pub mod heap {
    pub mod base {
        pub struct StackVisitor {}
    }
}

pub mod v8 {
    pub mod internal {
        use crate::common::globals::*;
        use crate::objects::code::Code;
        use std::any::Any;

        #[cfg(not(feature = "simulator"))]
        use crate::execution::isolate::Isolate;

        pub trait AllStatic {
            // Placeholder trait; can be implemented by types that were AllStatic in C++.
        }

        /// Represents the simulator stack.
        pub struct SimulatorStack {}

        impl SimulatorStack {
            /// Converts a C stack limit to a JS stack limit.
            pub fn js_limit_from_c_limit(isolate: *mut Isolate, c_limit: usize) -> usize {
                #[cfg(feature = "simulator")]
                {
                    //Simulator::current(isolate).stack_limit(c_limit)
                    // TODO: Implement Simulator::current and stack_limit in simulator module
                    c_limit
                }
                #[cfg(not(feature = "simulator"))]
                {
                    let _ = isolate;
                    c_limit
                }
            }

            #[cfg(feature = "wasm")]
            pub fn get_central_stack_view(isolate: *mut Isolate) -> Vec<u8> {
                #[cfg(feature = "simulator")]
                {
                    //Simulator::current(isolate).get_central_stack_view()
                    // TODO: Implement Simulator::current and get_central_stack_view in simulator module
                    Vec::new()
                }
                #[cfg(not(feature = "simulator"))]
                {
                    let upper_bound = crate::base::platform::get_stack_start();
                    let size = unsafe {
                        (*isolate).stack_size() + 256 * KB
                    }; // wasm::StackMemory::kJSLimitOffsetKB * KB
                    let lower_bound = upper_bound - size;
                    (lower_bound as *mut u8).offset(0).read_volatile().into()
                    //TODO: Implement base::VectorOf
                }
            }

            /// Iterates the simulator registers and stack for conservative stack scanning.
            pub fn iterate_registers_and_stack(
                isolate: *mut Isolate,
                visitor: &mut crate::heap::base::StackVisitor,
            ) {
                #[cfg(feature = "simulator")]
                {
                    // assert!(!isolate.is_null());
                    //Simulator::current(isolate).iterate_registers_and_stack(visitor);
                    // TODO: Implement Simulator::current and iterate_registers_and_stack in simulator module
                }
                #[cfg(not(feature = "simulator"))]
                {
                    // Do nothing
                }
            }

            /// Determines if the C stack should be switched when switching stacks for Wasm.
            pub fn should_switch_c_stack_for_wasm_stack_switching() -> bool {
                #[cfg(feature = "simulator")]
                {
                    false
                }
                #[cfg(not(feature = "simulator"))]
                {
                    true
                }
            }

            /// Returns the current stack address on the simulator stack frame.
            /// The returned address is comparable with JS stack address.
            pub fn register_js_stack_comparable_address(isolate: *mut Isolate) -> usize {
                #[cfg(feature = "simulator")]
                {
                    //const K_PLACE_HOLDER: usize = 0x4A535350; // "JSSP" in ASCII
                    //Simulator::current(isolate).push_address(K_PLACE_HOLDER)
                    // TODO: Implement Simulator::current and push_address in simulator module
                    0
                }
                #[cfg(not(feature = "simulator"))]
                {
                   let _ = isolate;
                    crate::internal::get_current_stack_position()
                }
            }

            pub fn unregister_js_stack_comparable_address(isolate: *mut Isolate) {
                #[cfg(feature = "simulator")]
                {
                    //Simulator::current(isolate).pop_address();
                    // TODO: Implement Simulator::current and pop_address in simulator module
                }
                #[cfg(not(feature = "simulator"))]
                {
                    let _ = isolate;
                }
            }
        }

        impl AllStatic for SimulatorStack {}

        /// Represents generated code.
        pub struct GeneratedCode<Return, Args> {
            isolate_: *mut Isolate,
            fn_ptr_: *mut dyn Fn(Args) -> Return,
            _phantom: std::marker::PhantomData<(Return, Args)>,
        }

        impl<Return: 'static, Args: 'static> GeneratedCode<Return, Args> {
            // Alias for the function signature type
            pub type Signature = fn(Args) -> Return;
            
            pub fn from_address(isolate: *mut Isolate, addr: usize) -> Self {
                unsafe {
                    GeneratedCode {
                        isolate_: isolate,
                        fn_ptr_: addr as *mut dyn Fn(Args) -> Return,
                        _phantom: std::marker::PhantomData,
                    }
                }
            }
            
            pub fn from_buffer(isolate: *mut Isolate, buffer: *mut u8) -> Self {
                unsafe {
                    GeneratedCode {
                        isolate_: isolate,
                        fn_ptr_: buffer as *mut dyn Fn(Args) -> Return,
                        _phantom: std::marker::PhantomData,
                    }
                }
            }
        
            pub fn from_code(isolate: *mut Isolate, code: *mut Code) -> Self {
                unsafe {
                    GeneratedCode::from_address(isolate, (*code).instruction_start() as usize)
                }
            }

            #[cfg(feature = "simulator")]
            pub fn call(&self, args: Args) -> Return {
                // TODO: Implement Simulator::current and call in simulator module
                // Simulator::current(self.isolate_).call(self.fn_ptr_, args)
                panic!("Simulator::current not implemented")
            }

            #[cfg(not(feature = "simulator"))]
            pub fn call(&self, args: Args) -> Return {
                unsafe {
                    let fn_ptr: Signature = std::mem::transmute(self.fn_ptr_);
                    fn_ptr(args)
                }
            }
        }
        
        pub struct GeneratedCodeFn<Return, Args> {
            inner: GeneratedCode<Return, Args>
        }
        
        impl<Return: 'static, Args: 'static> From<GeneratedCode<Return, Args>> for GeneratedCodeFn<Return, Args> {
            fn from(other: GeneratedCode<Return, Args>) -> Self {
                GeneratedCodeFn { inner: other }
            }
        }
        
        impl<Return: 'static, Args: 'static> GeneratedCodeFn<Return, Args> {
            pub fn call(&self, args: Args) -> Return {
                self.inner.call(args)
            }
        }

        pub fn get_current_stack_position() -> usize {
            let dummy: i32 = 0;
            let addr: *const i32 = &dummy;
            addr as usize
        }
    }
}