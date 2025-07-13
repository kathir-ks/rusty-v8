// Converted from V8 C++ source files:
// Header: pointer-authentication-dummy.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]
use crate::Address;
use crate::Isolate;
use v8_flags;

pub struct PointerAuthentication {}

impl PointerAuthentication {
    // Load return address from {pc_address} and return it.
    #[inline]
    pub fn authenticate_pc(pc_address: &mut Address, _: u32) -> Address {
        *pc_address
    }

    // Return {pc} unmodified.
    #[inline]
    pub fn strip_pac(pc: Address) -> Address {
        pc
    }

    // Store {new_pc} to {pc_address} without signing.
    #[inline]
    pub fn replace_pc(pc_address: &mut Address, new_pc: Address, _: i32) {
        *pc_address = new_pc;
    }

    // Return {pc} unmodified.
    #[inline]
    pub fn sign_and_check_pc(_isolate: &Isolate, pc: Address, _: Address) -> Address {
        pc
    }

    #[inline]
    pub fn move_signed_pc(_isolate: &Isolate, pc: Address, _: Address, _: Address) -> Address {
        #[cfg(feature = "V8_ENABLE_WEBASSEMBLY")]
        {
            // Only used by wasm deoptimizations and growable stacks.
            if v8_flags::v8_flags().wasm_deopt || v8_flags::v8_flags().experimental_wasm_growable_stacks {
                return pc;
            } else {
				panic!("UNREACHABLE");
			}
        }

        #[cfg(not(feature = "V8_ENABLE_WEBASSEMBLY"))]
        {
            panic!("UNREACHABLE");
        }
    }
}
