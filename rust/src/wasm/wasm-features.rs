// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "wasm"))]
// compile_error!("This header should only be included if WebAssembly is enabled.");

use std::fmt;
use std::cmp::Ordering;

mod base {
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::ops::{BitOr, BitOrAssign, BitAnd, BitAndAssign, Sub, SubAssign};

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub struct EnumSet<T>
    where
        T: Copy + Eq + Hash + Into<u32> + From<u32>,
    {
        bits: HashSet<T>,
    }

    impl<T> EnumSet<T>
    where
        T: Copy + Eq + Hash + Into<u32> + From<u32>,
    {
        pub const fn new() -> Self {
            Self { bits: HashSet::new() }
        }

        pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut set = HashSet::new();
            for item in iter {
                set.insert(item);
            }
            Self { bits: set }
        }

        pub fn insert(&mut self, value: T) {
            self.bits.insert(value);
        }

        pub fn remove(&mut self, value: &T) {
            self.bits.remove(value);
        }

        pub fn contains(&self, value: T) -> bool {
            self.bits.contains(&value)
        }

        pub fn is_empty(&self) -> bool {
            self.bits.is_empty()
        }

        pub fn len(&self) -> usize {
            self.bits.len()
        }

        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> {
            self.bits.iter()
        }

        pub fn to_integral(&self) -> u32 {
            let mut result: u32 = 0;
            for &item in &self.bits {
                result |= item.into();
            }
            result
        }

        pub fn from_integral(value: u32) -> Self {
            let mut bits = HashSet::new();
            for i in 0..32 {
                if (value >> i) & 1 != 0 {
                    bits.insert(T::from(i));
                }
            }
            Self { bits }
        }

    }

    impl<T> BitOr for EnumSet<T>
    where
        T: Copy + Eq + Hash + Into<u32> + From<u32>,
    {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            let mut result = self.clone();
            result |= other;
            result
        }
    }

    impl<T> BitOrAssign for EnumSet<T>
    where
        T: Copy + Eq + Hash + Into<u32> + From<u32>,
    {
        fn bitor_assign(&mut self, other: Self) {
            for item in other.bits {
                self.bits.insert(item);
            }
        }
    }

    impl<T> BitAnd for EnumSet<T>
    where
        T: Copy + Eq + Hash + Into<u32> + From<u32>,
    {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            let mut result = Self::new();
            for item in self.bits {
                if other.bits.contains(&item) {
                    result.bits.insert(item);
                }
            }
            result
        }
    }

    impl<T> BitAndAssign for EnumSet<T>
    where
        T: Copy + Eq + Hash + Into<u32> + From<u32>,
    {
        fn bitand_assign(&mut self, other: Self) {
            self.bits = self.bits.intersection(&other.bits).cloned().collect();
        }
    }

    impl<T> Sub for EnumSet<T>
    where
        T: Copy + Eq + Hash + Into<u32> + From<u32>,
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            let mut result = self.clone();
            result -= other;
            result
        }
    }

    impl<T> SubAssign for EnumSet<T>
    where
        T: Copy + Eq + Hash + Into<u32> + From<u32>,
    {
        fn sub_assign(&mut self, other: Self) {
            for item in other.bits {
                self.bits.remove(&item);
            }
        }
    }

}

mod common {
    pub mod globals {
        // Placeholder for globals.h
        pub type Isolate = u32;
        pub type NativeContext = u32;
    }
}

mod wasm {
    use super::*;
    use self::base::EnumSet;
    use std::fmt;
    use std::string::String;

    pub mod wasm_feature_flags {
        // Placeholder for wasm-feature-flags.h
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum FeatureFlag {
            BulkMemory,
            Exceptions,
            MutableGlobal,
            Trunc提案, // Example of a non-ascii name, replace with suitable name
            SatConv,
            SignExt,
            // Add other feature flags here
        }
    }

    macro_rules! foreach_wasm_non_flag_feature {
        ($v:ident) => {
            $v!(shared_memory)
            $v!(reftypes)
            $v!(simd)
            $v!(threads)
            $v!(return_call)
            $v!(extended_const)
            $v!(relaxed_simd)
            $v!(gc)
            $v!(typed_funcref)
            $v!(js_inlining)
            $v!(multi_memory)
            $v!(memory64)
        };
    }

    macro_rules! foreach_wasm_feature_flag {
        ($v:ident) => {
            $v!(bulk_memory)
            $v!(exceptions)
            $v!(mutable_global)
            $v!(trunc_提案) // Example of a non-ascii name, replace with suitable name
            $v!(sat_conv)
            $v!(sign_ext)
        };
    }

    macro_rules! foreach_wasm_feature {
        ($v:ident) => {
            foreach_wasm_feature_flag!($v);
            foreach_wasm_non_flag_feature!($v);
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum WasmEnabledFeature {
        // List of enabled features, mirroring DECL_FEATURE_ENUM from C++
        #[allow(non_camel_case_types)]
        bulk_memory,
        #[allow(non_camel_case_types)]
        exceptions,
        #[allow(non_camel_case_types)]
        mutable_global,
        #[allow(non_camel_case_types)]
        trunc_提案,
        #[allow(non_camel_case_types)]
        sat_conv,
        #[allow(non_camel_case_types)]
        sign_ext,
    }

    impl From<WasmEnabledFeature> for u32 {
        fn from(feature: WasmEnabledFeature) -> Self {
            match feature {
                WasmEnabledFeature::bulk_memory => 1 << 0,
                WasmEnabledFeature::exceptions => 1 << 1,
                WasmEnabledFeature::mutable_global => 1 << 2,
                WasmEnabledFeature::trunc_提案 => 1 << 3,
                WasmEnabledFeature::sat_conv => 1 << 4,
                WasmEnabledFeature::sign_ext => 1 << 5,
            }
        }
    }

    impl From<u32> for WasmEnabledFeature {
        fn from(value: u32) -> Self {
            match value {
                0 => WasmEnabledFeature::bulk_memory,
                1 => WasmEnabledFeature::exceptions,
                2 => WasmEnabledFeature::mutable_global,
                3 => WasmEnabledFeature::trunc_提案,
                4 => WasmEnabledFeature::sat_conv,
                5 => WasmEnabledFeature::sign_ext,
                _ => panic!("Invalid WasmEnabledFeature value: {}", value),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum WasmDetectedFeature {
        // List of detected features, mirroring DECL_FEATURE_ENUM from C++
        #[allow(non_camel_case_types)]
        bulk_memory,
        #[allow(non_camel_case_types)]
        exceptions,
        #[allow(non_camel_case_types)]
        mutable_global,
        #[allow(non_camel_case_types)]
        trunc_提案,
        #[allow(non_camel_case_types)]
        sat_conv,
        #[allow(non_camel_case_types)]
        sign_ext,
        #[allow(non_camel_case_types)]
        shared_memory,
        #[allow(non_camel_case_types)]
        reftypes,
        #[allow(non_camel_case_types)]
        simd,
        #[allow(non_camel_case_types)]
        threads,
        #[allow(non_camel_case_types)]
        return_call,
        #[allow(non_camel_case_types)]
        extended_const,
        #[allow(non_camel_case_types)]
        relaxed_simd,
        #[allow(non_camel_case_types)]
        gc,
        #[allow(non_camel_case_types)]
        typed_funcref,
        #[allow(non_camel_case_types)]
        js_inlining,
        #[allow(non_camel_case_types)]
        multi_memory,
        #[allow(non_camel_case_types)]
        memory64,
    }

    impl From<WasmDetectedFeature> for u32 {
        fn from(feature: WasmDetectedFeature) -> Self {
            match feature {
                WasmDetectedFeature::bulk_memory => 1 << 0,
                WasmDetectedFeature::exceptions => 1 << 1,
                WasmDetectedFeature::mutable_global => 1 << 2,
                WasmDetectedFeature::trunc_提案 => 1 << 3,
                WasmDetectedFeature::sat_conv => 1 << 4,
                WasmDetectedFeature::sign_ext => 1 << 5,
                WasmDetectedFeature::shared_memory => 1 << 6,
                WasmDetectedFeature::reftypes => 1 << 7,
                WasmDetectedFeature::simd => 1 << 8,
                WasmDetectedFeature::threads => 1 << 9,
                WasmDetectedFeature::return_call => 1 << 10,
                WasmDetectedFeature::extended_const => 1 << 11,
                WasmDetectedFeature::relaxed_simd => 1 << 12,
                WasmDetectedFeature::gc => 1 << 13,
                WasmDetectedFeature::typed_funcref => 1 << 14,
                WasmDetectedFeature::js_inlining => 1 << 15,
                WasmDetectedFeature::multi_memory => 1 << 16,
                WasmDetectedFeature::memory64 => 1 << 17,
            }
        }
    }

    impl From<u32> for WasmDetectedFeature {
        fn from(value: u32) -> Self {
            match value {
                0 => WasmDetectedFeature::bulk_memory,
                1 => WasmDetectedFeature::exceptions,
                2 => WasmDetectedFeature::mutable_global,
                3 => WasmDetectedFeature::trunc_提案,
                4 => WasmDetectedFeature::sat_conv,
                5 => WasmDetectedFeature::sign_ext,
                6 => WasmDetectedFeature::shared_memory,
                7 => WasmDetectedFeature::reftypes,
                8 => WasmDetectedFeature::simd,
                9 => WasmDetectedFeature::threads,
                10 => WasmDetectedFeature::return_call,
                11 => WasmDetectedFeature::extended_const,
                12 => WasmDetectedFeature::relaxed_simd,
                13 => WasmDetectedFeature::gc,
                14 => WasmDetectedFeature::typed_funcref,
                15 => WasmDetectedFeature::js_inlining,
                16 => WasmDetectedFeature::multi_memory,
                17 => WasmDetectedFeature::memory64,
                _ => panic!("Invalid WasmDetectedFeature value: {}", value),
            }
        }
    }

    /// Set of enabled features. This only includes features that have a flag.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub struct WasmEnabledFeatures(EnumSet<WasmEnabledFeature>);

    impl WasmEnabledFeatures {
        pub const fn new() -> Self {
            Self(EnumSet::new())
        }

        pub fn from_iter<I: IntoIterator<Item = WasmEnabledFeature>>(iter: I) -> Self {
            Self(EnumSet::from_iter(iter))
        }

        pub fn all() -> Self {
            let mut features = EnumSet::new();
            macro_rules! list_feature {
                ($feat:ident) => {
                    features.insert(WasmEnabledFeature::$feat);
                };
            }
            foreach_wasm_feature_flag!(list_feature);
            WasmEnabledFeatures(features)
        }

        pub fn none() -> Self {
            Self(EnumSet::new())
        }

        pub fn for_asmjs() -> Self {
            Self(EnumSet::new())
        }

        // Retuns optional features that are enabled by flags, plus features that are
        // not enabled by a flag and are always on.
        pub fn from_flags() -> Self {
            // TODO: Implement feature flag logic based on V8 flags
            Self::none()
        }

        pub fn from_isolate(_isolate: &common::globals::Isolate) -> Self {
            // TODO: Implement feature flag logic based on Isolate
            Self::none()
        }

        pub fn from_context(
            _isolate: &common::globals::Isolate,
            _context: &common::globals::NativeContext,
        ) -> Self {
            // TODO: Implement feature flag logic based on Context
            Self::none()
        }
    }

    macro_rules! decl_feature_getter {
        ($feat:ident) => {
            #[allow(non_snake_case)]
            pub fn has_$feat(&self) -> bool {
                self.0.contains(WasmEnabledFeature::$feat)
            }
        };
    }
    foreach_wasm_feature_flag!(decl_feature_getter);

    /// Set of detected features. This includes features that have a flag plus
    /// features in FOREACH_WASM_NON_FLAG_FEATURE.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub struct WasmDetectedFeatures(EnumSet<WasmDetectedFeature>);

    impl WasmDetectedFeatures {
        pub const fn new() -> Self {
            Self(EnumSet::new())
        }

        // Construct from an enum set.
        pub const fn from_enum_set(features: EnumSet<WasmDetectedFeature>) -> Self {
            Self(features)
        }
    }

    macro_rules! decl_feature_getter {
        ($feat:ident) => {
            #[allow(non_snake_case)]
            pub fn add_$feat(&mut self) {
                self.0.insert(WasmDetectedFeature::$feat);
            }
            #[allow(non_snake_case)]
            pub fn has_$feat(&self) -> bool {
                self.0.contains(WasmDetectedFeature::$feat)
            }
        };
    }
    foreach_wasm_feature!(decl_feature_getter);

    impl fmt::Display for WasmEnabledFeature {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", name(*self))
        }
    }

    impl fmt::Display for WasmDetectedFeature {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", name(*self))
        }
    }

    pub fn name(feature: WasmEnabledFeature) -> &'static str {
        match feature {
            WasmEnabledFeature::bulk_memory => "bulk_memory",
            WasmEnabledFeature::exceptions => "exceptions",
            WasmEnabledFeature::mutable_global => "mutable_global",
            WasmEnabledFeature::trunc_提案 => "trunc_提案",
            WasmEnabledFeature::sat_conv => "sat_conv",
            WasmEnabledFeature::sign_ext => "sign_ext",
        }
    }

    pub fn name(feature: WasmDetectedFeature) -> &'static str {
        match feature {
            WasmDetectedFeature::bulk_memory => "bulk_memory",
            WasmDetectedFeature::exceptions => "exceptions",
            WasmDetectedFeature::mutable_global => "mutable_global",
            WasmDetectedFeature::trunc_提案 => "trunc_提案",
            WasmDetectedFeature::sat_conv => "sat_conv",
            WasmDetectedFeature::sign_ext => "sign_ext",
            WasmDetectedFeature::shared_memory => "shared_memory",
            WasmDetectedFeature::reftypes => "reftypes",
            WasmDetectedFeature::simd => "simd",
            WasmDetectedFeature::threads => "threads",
            WasmDetectedFeature::return_call => "return_call",
            WasmDetectedFeature::extended_const => "extended_const",
            WasmDetectedFeature::relaxed_simd => "relaxed_simd",
            WasmDetectedFeature::gc => "gc",
            WasmDetectedFeature::typed_funcref => "typed_funcref",
            WasmDetectedFeature::js_inlining => "js_inlining",
            WasmDetectedFeature::multi_memory => "multi_memory",
            WasmDetectedFeature::memory64 => "memory64",
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum CompileTimeImport {
        JsString,
        StringConstants,
        TextEncoder,
        TextDecoder,
        DisableDenormalFloats, // Not really an import
    }

    impl fmt::Display for CompileTimeImport {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", *self as i32)
        }
    }

    pub type CompileTimeImportFlags = EnumSet<CompileTimeImport>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct CompileTimeImports {
        bits: CompileTimeImportFlags,
        constants_module_: String,
    }

    impl CompileTimeImports {
        pub fn new() -> Self {
            CompileTimeImports {
                bits: EnumSet::new(),
                constants_module_: String::new(),
            }
        }

        pub fn from_serialized(flags: u32, constants_module: Vec<u8>) -> Self {
            CompileTimeImports {
                bits: EnumSet::from_integral(flags),
                constants_module_: String::from_utf8(constants_module).unwrap(),
            }
        }

        pub fn empty(&self) -> bool {
            self.bits.is_empty()
        }

        pub fn has_string_constants(&self, name: &[u8]) -> bool {
            self.bits.contains(CompileTimeImport::StringConstants)
                && self.constants_module_.len() == name.len()
                && self.constants_module_.as_bytes() == name
        }

        pub fn contains(&self, imp: CompileTimeImport) -> bool {
            self.bits.contains(imp)
        }

        pub fn compare(&self, other: &CompileTimeImports) -> Ordering {
            let bits_cmp = self.bits.to_integral().cmp(&other.bits.to_integral());
            if bits_cmp != Ordering::Equal {
                return bits_cmp;
            }
            self.constants_module_.cmp(&other.constants_module_)
        }

        pub fn add(&mut self, imp: CompileTimeImport) {
            self.bits.insert(imp);
        }

        pub fn constants_module(&mut self) -> &mut String {
            &mut self.constants_module_
        }

        pub fn get_constants_module(&self) -> &String {
            &self.constants_module_
        }

        pub fn flags(&self) -> CompileTimeImportFlags {
            self.bits.clone()
        }
    }
}