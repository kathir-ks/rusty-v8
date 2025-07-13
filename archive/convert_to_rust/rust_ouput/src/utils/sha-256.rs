// Converted from V8 C++ source files:
// Header: sha-256.h
// Implementation: sha-256.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Copyright 2013 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// ========================================================================
//
// This code originates from the Omaha installer for Windows:
//   https://github.com/google/omaha
// The following changes were made:
//  - Combined the hash-internal.h and sha256.h headers together to form
//    this one.
//  - Eliminated conditional definitions related to LITE_EMULATED_64BIT_OPS
//  - Eliminated `extern "C"` definitions as these aren't exported
//  - Eliminated `SHA512_SUPPORT` as we only support SHA256
//  - Eliminated generic `HASH_` definitions as unnecessary
//  - Moved the hashing functions into `namespace v8::internal`
//
// This is intended simply to provide a minimal-impact SHA256 hash utility
// to support the stack trace source hash functionality.

use std::mem::size_of;

const kSizeOfSha256Digest: usize = 32;
const kSizeOfFormattedSha256Digest: usize = (kSizeOfSha256Digest * 2) + 1;

pub mod v8 {
    pub mod internal {
        pub type HashInitFunc = fn(&mut HASH_CTX);
        pub type HashUpdateFunc = fn(&mut HASH_CTX, *const std::ffi::c_void, usize);
        pub type HashFinalFunc = fn(&mut HASH_CTX) -> *const u8;
        pub type HashHashFunc = fn(*const std::ffi::c_void, usize, *mut u8) -> *const u8;

        #[derive(Debug)]
        pub struct HASH_VTAB {
            pub init: HashInitFunc,
            pub update: HashUpdateFunc,
            pub final_: HashFinalFunc,
            pub hash: HashHashFunc,
            pub size: u32,
        }

        #[derive(Debug)]
        #[repr(C)]
        pub struct HASH_CTX {
            pub f: *const HASH_VTAB,
            pub count: u64,
            pub buf: [u8; 64],
            pub state: [u32; 8], // upto SHA2-256
        }

        pub type LITE_SHA256_CTX = HASH_CTX;

        extern "C" {
            fn memcpy(dest: *mut std::ffi::c_void, src: *const std::ffi::c_void, n: usize) -> *mut std::ffi::c_void;
        }

        macro_rules! ror {
            ($value:expr, $bits:expr) => {
                (($value >> $bits) | ($value << (32 - $bits)))
            };
        }

        macro_rules! shr {
            ($value:expr, $bits:expr) => {
                ($value >> $bits)
            };
        }

        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
            0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
            0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
            0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
            0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
            0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
            0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
            0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
            0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
        ];

        fn sha256_transform(ctx: &mut LITE_SHA256_CTX) {
            let mut w = [0u32; 64];
            let mut a: u32;
            let mut b: u32;
            let mut c: u32;
            let mut d: u32;
            let mut e: u32;
            let mut f: u32;
            let mut g: u32;
            let mut h: u32;
            let mut p = 0;

            for t in 0..16 {
                let mut tmp: u32 = (ctx.buf[p] as u32) << 24;
                p += 1;
                tmp |= (ctx.buf[p] as u32) << 16;
                p += 1;
                tmp |= (ctx.buf[p] as u32) << 8;
                p += 1;
                tmp |= ctx.buf[p] as u32;
                p += 1;
                w[t] = tmp;
            }

            for t in 16..64 {
                let s0 = ror!(w[t - 15], 7) ^ ror!(w[t - 15], 18) ^ shr!(w[t - 15], 3);
                let s1 = ror!(w[t - 2], 17) ^ ror!(w[t - 2], 19) ^ shr!(w[t - 2], 10);
                w[t] = w[t - 16].wrapping_add(s0).wrapping_add(w[t - 7]).wrapping_add(s1);
            }

            a = ctx.state[0];
            b = ctx.state[1];
            c = ctx.state[2];
            d = ctx.state[3];
            e = ctx.state[4];
            f = ctx.state[5];
            g = ctx.state[6];
            h = ctx.state[7];

            for t in 0..64 {
                let s0 = ror!(a, 2) ^ ror!(a, 13) ^ ror!(a, 22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let t2 = s0.wrapping_add(maj);
                let s1 = ror!(e, 6) ^ ror!(e, 11) ^ ror!(e, 25);
                let ch = (e & f) ^ ((!e) & g);
                let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[t]).wrapping_add(w[t]);

                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }

            ctx.state[0] = ctx.state[0].wrapping_add(a);
            ctx.state[1] = ctx.state[1].wrapping_add(b);
            ctx.state[2] = ctx.state[2].wrapping_add(c);
            ctx.state[3] = ctx.state[3].wrapping_add(d);
            ctx.state[4] = ctx.state[4].wrapping_add(e);
            ctx.state[5] = ctx.state[5].wrapping_add(f);
            ctx.state[6] = ctx.state[6].wrapping_add(g);
            ctx.state[7] = ctx.state[7].wrapping_add(h);
        }

        unsafe extern "C" fn sha256_init_c(ctx: *mut LITE_SHA256_CTX) {
            if ctx.is_null() {
                return;
            }
            let ctx = &mut *ctx;
            let sha256_vtab_ptr: *const HASH_VTAB = &SHA256_VTAB;
            ctx.f = sha256_vtab_ptr;
            ctx.state[0] = 0x6a09e667;
            ctx.state[1] = 0xbb67ae85;
            ctx.state[2] = 0x3c6ef372;
            ctx.state[3] = 0xa54ff53a;
            ctx.state[4] = 0x510e527f;
            ctx.state[5] = 0x9b05688c;
            ctx.state[6] = 0x1f83d9ab;
            ctx.state[7] = 0x5be0cd19;
            ctx.count = 0;
        }

        pub fn SHA256_init(ctx: &mut LITE_SHA256_CTX) {
            unsafe {
                sha256_init_c(ctx);
            }
        }

        unsafe extern "C" fn sha256_update_c(ctx: *mut LITE_SHA256_CTX, data: *const std::ffi::c_void, len: usize) {
            if ctx.is_null() || data.is_null() {
                return;
            }

            let ctx = &mut *ctx;
            let mut i = (ctx.count & 63) as usize;
            let mut p = data as *const u8;
            let mut len_remaining = len;

            ctx.count += len_remaining as u64;

            while len_remaining > 0 {
                ctx.buf[i] = *p;
                i += 1;
                p = p.add(1);
                len_remaining -= 1;

                if i == 64 {
                    sha256_transform(ctx);
                    i = 0;
                }
            }
        }

        pub fn SHA256_update(ctx: &mut LITE_SHA256_CTX, data: *const std::ffi::c_void, len: usize) {
            unsafe {
                sha256_update_c(ctx, data, len);
            }
        }

        unsafe extern "C" fn sha256_final_c(ctx: *mut LITE_SHA256_CTX) -> *const u8 {
            if ctx.is_null() {
                return std::ptr::null();
            }

            let ctx = &mut *ctx;
            let mut p = ctx.buf.as_mut_ptr();
            let mut cnt = ctx.count << 3;
            let i: usize;

            let completion: [u8; 2] = [0x80, 0];

            SHA256_update(ctx, completion.as_ptr() as *const std::ffi::c_void, 1);
            while (ctx.count & 63) != 56 {
                SHA256_update(ctx, completion[1..].as_ptr() as *const std::ffi::c_void, 1);
            }

            for i in 0..8 {
                let tmp = (cnt >> 56) as u8;
                cnt <<= 8;
                SHA256_update(ctx, &tmp as *const u8 as *const std::ffi::c_void, 1);
            }

            for i in 0..8 {
                let tmp = ctx.state[i];
                *p = (tmp >> 24) as u8;
                p = p.add(1);
                *p = (tmp >> 16) as u8;
                p = p.add(1);
                *p = (tmp >> 8) as u8;
                p = p.add(1);
                *p = (tmp >> 0) as u8;
                p = p.add(1);
            }

            ctx.buf.as_ptr()
        }

        pub fn SHA256_final(ctx: &mut LITE_SHA256_CTX) -> *const u8 {
            unsafe {
                sha256_final_c(ctx)
            }
        }

        unsafe extern "C" fn sha256_hash_c(data: *const std::ffi::c_void, len: usize, digest: *mut u8) -> *const u8 {
            if data.is_null() || digest.is_null() {
                return std::ptr::null();
            }

            let mut ctx: LITE_SHA256_CTX = std::mem::zeroed();
            SHA256_init(&mut ctx);
            SHA256_update(&mut ctx, data, len);

            let result = SHA256_final(&mut ctx);
            if result.is_null() {
                return std::ptr::null();
            }
            std::ptr::copy_nonoverlapping(result, digest, super::kSizeOfSha256Digest);

            digest as *const u8
        }

        pub fn SHA256_hash(data: *const std::ffi::c_void, len: usize, digest: *mut u8) -> *const u8 {
            unsafe {
                sha256_hash_c(data, len, digest)
            }
        }

        static SHA256_VTAB: HASH_VTAB = HASH_VTAB {
            init: sha256_init,
            update: sha256_update,
            final_: sha256_final,
            hash: sha256_hash,
            size: super::kSizeOfSha256Digest as u32,
        };

        fn sha256_init(ctx: &mut HASH_CTX) {
            let sha256_vtab_ptr: *const HASH_VTAB = &SHA256_VTAB;
            ctx.f = sha256_vtab_ptr;
            ctx.state[0] = 0x6a09e667;
            ctx.state[1] = 0xbb67ae85;
            ctx.state[2] = 0x3c6ef372;
            ctx.state[3] = 0xa54ff53a;
            ctx.state[4] = 0x510e527f;
            ctx.state[5] = 0x9b05688c;
            ctx.state[6] = 0x1f83d9ab;
            ctx.state[7] = 0x5be0cd19;
            ctx.count = 0;
        }

        fn sha256_update(ctx: &mut HASH_CTX, data: *const std::ffi::c_void, len: usize) {
            unsafe {
                let mut i = (ctx.count & 63) as usize;
                let mut p = data as *const u8;
                let mut len_remaining = len;

                ctx.count += len_remaining as u64;

                while len_remaining > 0 {
                    ctx.buf[i] = *p;
                    i += 1;
                    p = p.add(1);
                    len_remaining -= 1;

                    if i == 64 {
                        sha256_transform(ctx as *mut HASH_CTX as *mut LITE_SHA256_CTX);
                        i = 0;
                    }
                }
            }
        }

        fn sha256_final(ctx: &mut HASH_CTX) -> *const u8 {
            unsafe {
                let mut p = ctx.buf.as_mut_ptr();
                let mut cnt = ctx.count << 3;
                let i: usize;

                let completion: [u8; 2] = [0x80, 0];

                sha256_update(ctx, completion.as_ptr() as *const std::ffi::c_void, 1);
                while (ctx.count & 63) != 56 {
                    sha256_update(ctx, completion[1..].as_ptr() as *const std::ffi::c_void, 1);
                }

                for i in 0..8 {
                    let tmp = (cnt >> 56) as u8;
                    cnt <<= 8;
                    sha256_update(ctx, &tmp as *const u8 as *const std::ffi::c_void, 1);
                }

                for i in 0..8 {
                    let tmp = ctx.state[i];
                    *p = (tmp >> 24) as u8;
                    p = p.add(1);
                    *p = (tmp >> 16) as u8;
                    p = p.add(1);
                    *p = (tmp >> 8) as u8;
                    p = p.add(1);
                    *p = (tmp >> 0) as u8;
                    p = p.add(1);
                }

                ctx.buf.as_ptr()
            }
        }

        fn sha256_hash(data: *const std::ffi::c_void, len: usize, digest: *mut u8) -> *const u8 {
            unsafe {
                let mut ctx: LITE_SHA256_CTX = std::mem::zeroed();
                SHA256_init(&mut ctx);
                SHA256_update(&mut ctx, data, len);
                let result = SHA256_final(&mut ctx);
                if result.is_null() {
                    return std::ptr::null();
                }
                std::ptr::copy_nonoverlapping(result, digest, super::kSizeOfSha256Digest);
                digest as *const u8
            }
        }
    }
}
