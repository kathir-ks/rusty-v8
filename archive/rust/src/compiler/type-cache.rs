// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/compiler/type-cache.rs

use lazy_static::lazy_static;

/// A cache of commonly used types.
pub struct TypeCache {
    // Add fields representing cached types here, e.g.:
    // any: Type,
}

impl TypeCache {
    /// Returns a reference to the global TypeCache instance.
    pub fn get() -> &'static TypeCache {
        static TYPE_CACHE: lazy_static::Lazy<TypeCache> = lazy_static::Lazy::new(|| {
            TypeCache {
                // Initialize cached types here, e.g.:
                // any: Type::Any(),
            }
        });
        &TYPE_CACHE
    }
}