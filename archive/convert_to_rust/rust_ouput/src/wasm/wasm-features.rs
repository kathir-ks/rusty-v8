// Converted from V8 C++ source files:
// Header: wasm-features.h
// Implementation: wasm-features.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

mod base {
    pub struct EnumSet<T, const N: usize> {
        // Placeholder for EnumSet data
    }
    impl<T, const N: usize> EnumSet<T, N> {
        pub fn empty(&self) -> bool {
            true // Placeholder
        }
        pub fn ToIntegral(&self) -> u32 {
            0 // Placeholder
        }
        pub fn contains(&self, _imp: T) -> bool {
            false // Placeholder
        }
        pub fn Add(&mut self, _imp: T) {}
        pub fn FromIntegral(_flags: u32) -> Self {
            EnumSet {  } // Placeholder
        }
    }
}

mod flags {
    pub struct flags {
    }
}

mod handles {
    pub struct handles {
    }
}

mod objects {
    pub struct string {
    }
}

mod execution {
    pub struct isolate_inl {
    }
}

mod v8 {
    pub struct DirectHandle<T> {
        // Placeholder for DirectHandle data
    }
    impl<T> DirectHandle<T> {
        
    }
}
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::io;
use std::string::String;
use std::vec::Vec;

pub struct V8_EXPORT_PRIVATE {}

pub struct Isolate {}
impl Isolate {
    fn native_context(&self) -> v8::DirectHandle<NativeContext> {
        v8::DirectHandle{}
    }
    fn IsWasmStringRefEnabled(&self, _context: v8::DirectHandle<NativeContext>) -> bool { false }
    fn IsWasmImportedStringsEnabled(&self, _context: v8::DirectHandle<NativeContext>) -> bool { false }
    fn IsWasmJSPIEnabled(&self, _context: v8::DirectHandle<NativeContext>) -> bool { false }
}

pub struct NativeContext {}

pub struct V8_NOEXCEPT {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WasmEnabledFeature {
    legacy_eh,
    stringref,
    imported_strings,
    jspi,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WasmDetectedFeature {
    shared_memory,
    reftypes,
    simd,
    threads,
    return_call,
    extended_const,
    relaxed_simd,
    gc,
    typed_funcref,
    js_inlining,
    multi_memory,
    memory64,
    stringref,
    imported_strings,
    jspi,
}

pub struct WasmEnabledFeatures(base::EnumSet<WasmEnabledFeature, 4>);

impl WasmEnabledFeatures {
    pub const fn new() -> Self {
        WasmEnabledFeatures(base::EnumSet {  })
    }
    pub const fn from_list(features: &[WasmEnabledFeature]) -> Self {
        let mut set = base::EnumSet {  };
        for &feature in features {
            set.Add(feature);
        }
        WasmEnabledFeatures(set)
    }

    pub const fn has_legacy_eh(&self) -> bool {
        self.0.contains(WasmEnabledFeature::legacy_eh)
    }
    pub const fn has_stringref(&self) -> bool {
        self.0.contains(WasmEnabledFeature::stringref)
    }
    pub const fn has_imported_strings(&self) -> bool {
        self.0.contains(WasmEnabledFeature::imported_strings)
    }
    pub const fn has_jspi(&self) -> bool {
        self.0.contains(WasmEnabledFeature::jspi)
    }

    pub fn All() -> Self {
        WasmEnabledFeatures::from_list(&[
            WasmEnabledFeature::legacy_eh,
            WasmEnabledFeature::stringref,
            WasmEnabledFeature::imported_strings,
            WasmEnabledFeature::jspi,
        ])
    }

    pub fn None() -> Self {
        WasmEnabledFeatures::new()
    }

    pub fn ForAsmjs() -> Self {
        WasmEnabledFeatures::new()
    }

    pub fn FromFlags() -> Self {
        let mut features = WasmEnabledFeatures::None();
        features
    }

    pub fn FromIsolate(isolate: *mut Isolate) -> Self {
        
        let isolate = unsafe { &mut *isolate };
        Self::FromContext(isolate, isolate.native_context())
    }

    pub fn FromContext(isolate: *mut Isolate, context: v8::DirectHandle<NativeContext>) -> Self {
       
        let isolate = unsafe { &mut *isolate };

        let mut features = WasmEnabledFeatures::FromFlags();
    
            if isolate.IsWasmStringRefEnabled(context) {
                features.0.Add(WasmEnabledFeature::stringref);
            }
            if isolate.IsWasmImportedStringsEnabled(context) {
                features.0.Add(WasmEnabledFeature::imported_strings);
            }
            if isolate.IsWasmJSPIEnabled(context) {
                features.0.Add(WasmEnabledFeature::jspi);
            }
        
        features
    }
}

pub struct WasmDetectedFeatures(base::EnumSet<WasmDetectedFeature, 12>);

impl WasmDetectedFeatures {
    pub const fn new() -> Self {
        WasmDetectedFeatures(base::EnumSet {  })
    }

    pub const fn from_enumset(features: base::EnumSet<WasmDetectedFeature, 12>) -> Self {
        WasmDetectedFeatures(features)
    }

    pub const fn add_shared_memory(&mut self) {
        self.0.Add(WasmDetectedFeature::shared_memory);
    }
    pub const fn has_shared_memory(&self) -> bool {
        self.0.contains(WasmDetectedFeature::shared_memory)
    }

    pub const fn add_reftypes(&mut self) {
        self.0.Add(WasmDetectedFeature::reftypes);
    }
    pub const fn has_reftypes(&self) -> bool {
        self.0.contains(WasmDetectedFeature::reftypes)
    }

    pub const fn add_simd(&mut self) {
        self.0.Add(WasmDetectedFeature::simd);
    }
    pub const fn has_simd(&self) -> bool {
        self.0.contains(WasmDetectedFeature::simd)
    }

    pub const fn add_threads(&mut self) {
        self.0.Add(WasmDetectedFeature::threads);
    }
    pub const fn has_threads(&self) -> bool {
        self.0.contains(WasmDetectedFeature::threads)
    }

    pub const fn add_return_call(&mut self) {
        self.0.Add(WasmDetectedFeature::return_call);
    }
    pub const fn has_return_call(&self) -> bool {
        self.0.contains(WasmDetectedFeature::return_call)
    }

    pub const fn add_extended_const(&mut self) {
        self.0.Add(WasmDetectedFeature::extended_const);
    }
    pub const fn has_extended_const(&self) -> bool {
        self.0.contains(WasmDetectedFeature::extended_const)
    }

    pub const fn add_relaxed_simd(&mut self) {
        self.0.Add(WasmDetectedFeature::relaxed_simd);
    }
    pub const fn has_relaxed_simd(&self) -> bool {
        self.0.contains(WasmDetectedFeature::relaxed_simd)
    }

    pub const fn add_gc(&mut self) {
        self.0.Add(WasmDetectedFeature::gc);
    }
    pub const fn has_gc(&self) -> bool {
        self.0.contains(WasmDetectedFeature::gc)
    }

    pub const fn add_typed_funcref(&mut self) {
        self.0.Add(WasmDetectedFeature::typed_funcref);
    }
    pub const fn has_typed_funcref(&self) -> bool {
        self.0.contains(WasmDetectedFeature::typed_funcref)
    }

    pub const fn add_js_inlining(&mut self) {
        self.0.Add(WasmDetectedFeature::js_inlining);
    }
    pub const fn has_js_inlining(&self) -> bool {
        self.0.contains(WasmDetectedFeature::js_inlining)
    }
    pub const fn add_multi_memory(&mut self) {
        self.0.Add(WasmDetectedFeature::multi_memory);
    }
    pub const fn has_multi_memory(&self) -> bool {
        self.0.contains(WasmDetectedFeature::multi_memory)
    }
    pub const fn add_memory64(&mut self) {
        self.0.Add(WasmDetectedFeature::memory64);
    }
    pub const fn has_memory64(&self) -> bool {
        self.0.contains(WasmDetectedFeature::memory64)
    }
    pub const fn add_stringref(&mut self) {
        self.0.Add(WasmDetectedFeature::stringref);
    }
    pub const fn has_stringref(&self) -> bool {
        self.0.contains(WasmDetectedFeature::stringref)
    }
    pub const fn add_imported_strings(&mut self) {
        self.0.Add(WasmDetectedFeature::imported_strings);
    }
    pub const fn has_imported_strings(&self) -> bool {
        self.0.contains(WasmDetectedFeature::imported_strings)
    }
    pub const fn add_jspi(&mut self) {
        self.0.Add(WasmDetectedFeature::jspi);
    }
    pub const fn has_jspi(&self) -> bool {
        self.0.contains(WasmDetectedFeature::jspi)
    }
}

pub fn name_wasm_enabled_feature(feature: WasmEnabledFeature) -> &'static str {
    match feature {
        WasmEnabledFeature::legacy_eh => "legacy_eh",
        WasmEnabledFeature::stringref => "stringref",
        WasmEnabledFeature::imported_strings => "imported_strings",
        WasmEnabledFeature::jspi => "jspi",
    }
}

impl std::fmt::Display for WasmEnabledFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", name_wasm_enabled_feature(*self))
    }
}

pub fn name_wasm_detected_feature(feature: WasmDetectedFeature) -> &'static str {
    match feature {
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
        WasmDetectedFeature::stringref => "stringref",
        WasmDetectedFeature::imported_strings => "imported_strings",
        WasmDetectedFeature::jspi => "jspi",
    }
}

impl std::fmt::Display for WasmDetectedFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", name_wasm_detected_feature(*self))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CompileTimeImport {
    kJsString,
    kStringConstants,
    kTextEncoder,
    kTextDecoder,
    kDisableDenormalFloats,
}

impl std::fmt::Display for CompileTimeImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

pub struct CompileTimeImportFlags(base::EnumSet<CompileTimeImport, 5>);

impl CompileTimeImportFlags {
    pub fn new() -> Self {
        CompileTimeImportFlags(base::EnumSet {  })
    }
    pub fn FromIntegral(flags: u32) -> Self {
        CompileTimeImportFlags(base::EnumSet::FromIntegral(flags))
    }
    pub fn ToIntegral(&self) -> u32 {
        self.0.ToIntegral()
    }
    pub fn contains(&self, imp: CompileTimeImport) -> bool {
        self.0.contains(imp)
    }
    pub fn Add(&mut self, imp: CompileTimeImport) {
        self.0.Add(imp);
    }
    pub fn empty(&self) -> bool {
        self.0.empty()
    }
}

pub struct CompileTimeImports {
    bits_: CompileTimeImportFlags,
    constants_module_: String,
}

impl CompileTimeImports {
    pub fn new() -> Self {
        CompileTimeImports {
            bits_: CompileTimeImportFlags::new(),
            constants_module_: String::new(),
        }
    }

    pub fn from_serialized(flags: u32, constants_module: Vec<char>) -> Self {
        let mut result = CompileTimeImports::new();
        result.bits_ = CompileTimeImportFlags::FromIntegral(flags);
        result.constants_module_ = constants_module.iter().collect();
        result
    }

    pub fn empty(&self) -> bool {
        self.bits_.empty()
    }

    pub fn has_string_constants(&self, name: &[u8]) -> bool {
        self.bits_.contains(CompileTimeImport::kStringConstants)
            && self.constants_module_.len() == name.len()
            && name.iter()
                .zip(self.constants_module_.as_bytes().iter())
                .all(|(a, b)| *a == *b)
    }

    pub fn contains(&self, imp: CompileTimeImport) -> bool {
        self.bits_.contains(imp)
    }

    pub fn compare(&self, other: &CompileTimeImports) -> i32 {
        if self.bits_.ToIntegral() < other.bits_.ToIntegral() {
            return -1;
        }
        if self.bits_.ToIntegral() > other.bits_.ToIntegral() {
            return 1;
        }
        self.constants_module_.cmp(&other.constants_module_) as i32
    }

    pub fn add(&mut self, imp: CompileTimeImport) {
        self.bits_.Add(imp);
    }

    pub fn constants_module(&mut self) -> &mut String {
        &mut self.constants_module_
    }

    pub fn get_constants_module(&self) -> &String {
        &self.constants_module_
    }

    pub fn flags(&self) -> &CompileTimeImportFlags {
        &self.bits_
    }
}
