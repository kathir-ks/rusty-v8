// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod lookup_cache {
    use crate::objects::lookup_cache::DescriptorLookupCache;
    use crate::objects::map::Map;
    use crate::objects::name::Name;
    use crate::objects::name::IsUniqueName;
    use crate::base::tagged::Tagged;
    use crate::base::tagged::kTaggedSizeLog2;

    impl DescriptorLookupCache {
        /// Calculates the hash for the given source map and name.
        pub fn hash(source: Tagged<Map>, name: Tagged<Name>) -> usize {
            debug_assert!(IsUniqueName(name));
            // Uses only lower 32 bits if pointers are larger.
            let source_hash = (source.ptr() as u32 >> kTaggedSizeLog2) as usize;
            let name_hash = name.hash() as usize;
            (source_hash ^ name_hash) % Self::kLength()
        }

        /// Looks up the result in the cache for the given source map and name.
        pub fn lookup(
            &self,
            source: Tagged<Map>,
            name: Tagged<Name>,
        ) -> i32 {
            let index = Self::hash(source, name);
            let key = &self.keys[index];
            // Pointers in the table might be stale, so use SafeEquals.
            if key.source.safe_equals(source) && key.name.safe_equals(name) {
                return self.results[index];
            }
            Self::kAbsent()
        }

        /// Updates the cache with the given source map, name, and result.
        pub fn update(
            &mut self,
            source: Tagged<Map>,
            name: Tagged<Name>,
            result: i32,
        ) {
            debug_assert_ne!(result, Self::kAbsent());
            let index = Self::hash(source, name);
            let key = &mut self.keys[index];
            key.source = source;
            key.name = name;
            self.results[index] = result;
        }
    }
} // mod lookup_cache