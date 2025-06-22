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

pub mod v8 {
    pub mod internal {
        use sha2::{Sha256, Digest};

        pub const K_SIZE_OF_SHA256_DIGEST: usize = 32;
        pub const K_SIZE_OF_FORMATTED_SHA256_DIGEST: usize = (K_SIZE_OF_SHA256_DIGEST * 2) + 1;

        pub struct HashVTab {
            pub init: fn(&mut HashCtx),
            pub update: fn(&mut HashCtx, &[u8]),
            pub final_fn: fn(&mut HashCtx) -> Vec<u8>,
            pub hash: fn(&[u8], &mut [u8]) -> Vec<u8>,
            pub size: u32,
        }

        pub struct HashCtx {
            pub f: &'static HashVTab,
            pub count: u64,
            pub buf: [u8; 64],
            pub state: [u32; 8],
        }

        pub type LiteSha256Ctx = HashCtx;

        pub fn sha256_init(ctx: &mut LiteSha256Ctx) {
            let sha256_vtab = HashVTab {
                init: sha256_init,
                update: sha256_update,
                final_fn: sha256_final,
                hash: sha256_hash,
                size: 32,
            };
            ctx.f = &sha256_vtab;
            // Initialize the Sha256 context.
            let mut hasher = Sha256::new();
            ctx.state = hasher.state.clone(); // Directly copy the state if accessible
            ctx.count = 0;
        }

        pub fn sha256_update(ctx: &mut LiteSha256Ctx, data: &[u8]) {
            // Update the Sha256 context with the provided data.
            let mut hasher = Sha256::new();
            hasher.state = ctx.state.clone(); // Restore the internal state.
            hasher.update(data);

            ctx.state = hasher.state.clone(); // Update state
            ctx.count += data.len() as u64;
        }

        pub fn sha256_final(ctx: &mut LiteSha256Ctx) -> Vec<u8> {
            // Finalize the Sha256 computation and return the digest.
            let mut hasher = Sha256::new();
            hasher.state = ctx.state.clone();
            let result = hasher.finalize();
            ctx.state = hasher.state.clone(); // Keep state.

            result.to_vec()
        }

        pub fn sha256_hash(data: &[u8], digest: &mut [u8]) -> Vec<u8> {
            // Convenience method to compute the Sha256 hash of the provided data.
            let mut hasher = Sha256::digest(data);

            digest.copy_from_slice(hasher.as_slice());

            hasher.to_vec()
        }
    }
}