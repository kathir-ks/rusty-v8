// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// Since there's no direct equivalent to #error in Rust, we skip this check.

use std::sync::atomic::{AtomicU8, Ordering};
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum WellKnownImport {
  // Generic:
  kUninstantiated,
  kGeneric,
  kLinkError,

  ////////////////////////////////////////////////////////
  // Compile-time "builtin" imports:
  ////////////////////////////////////////////////////////
  kFirstCompileTimeImport,

  // JS String Builtins
  // https://github.com/WebAssembly/js-string-builtins
  // TODO(14179): Rename some of these to reflect the new import names.
  kStringCast = Self::kFirstCompileTimeImport as u8,
  kStringCharCodeAt,
  kStringCodePointAt,
  kStringCompare,
  kStringConcat,
  kStringEquals,
  kStringFromCharCode,
  kStringFromCodePoint,
  kStringFromUtf8Array,
  kStringFromWtf16Array,
  kStringIntoUtf8Array,
  kStringLength,
  kStringMeasureUtf8,
  kStringSubstring,
  kStringTest,
  kStringToUtf8Array,
  kStringToWtf16Array,

  kLastCompileTimeImport = Self::kStringToWtf16Array as u8,
  ////////////////////////////////////////////////////////
  // End of compile-time "builtin" imports.
  ////////////////////////////////////////////////////////

  // DataView methods:
  kDataViewGetBigInt64,
  kDataViewGetBigUint64,
  kDataViewGetFloat32,
  kDataViewGetFloat64,
  kDataViewGetInt8,
  kDataViewGetInt16,
  kDataViewGetInt32,
  kDataViewGetUint8,
  kDataViewGetUint16,
  kDataViewGetUint32,
  kDataViewSetBigInt64,
  kDataViewSetBigUint64,
  kDataViewSetFloat32,
  kDataViewSetFloat64,
  kDataViewSetInt8,
  kDataViewSetInt16,
  kDataViewSetInt32,
  kDataViewSetUint8,
  kDataViewSetUint16,
  kDataViewSetUint32,
  kDataViewByteLength,

  // Math functions.
  kMathF64Acos,
  kMathF64Asin,
  kMathF64Atan,
  kMathF64Atan2,
  kMathF64Cos,
  kMathF64Sin,
  kMathF64Tan,
  kMathF64Exp,
  kMathF64Log,
  kMathF64Pow,
  kMathF64Sqrt, // Used by dart2wasm. f64.sqrt is equivalent.

  // String-related functions:
  kDoubleToString,
  kIntToString,
  kParseFloat,

  kStringIndexOf,
  kStringIndexOfImported,
  kStringToLocaleLowerCaseStringref,
  kStringToLowerCaseStringref,
  kStringToLowerCaseImported,
  // Fast API calls:
  kFastAPICall,
}

impl WellKnownImport {
  pub fn from_u8(value: u8) -> Option<Self> {
    match value {
      0 => Some(WellKnownImport::kUninstantiated),
      1 => Some(WellKnownImport::kGeneric),
      2 => Some(WellKnownImport::kLinkError),
      3 => Some(WellKnownImport::kFirstCompileTimeImport),
      4 => Some(WellKnownImport::kStringCast),
      5 => Some(WellKnownImport::kStringCharCodeAt),
      6 => Some(WellKnownImport::kStringCodePointAt),
      7 => Some(WellKnownImport::kStringCompare),
      8 => Some(WellKnownImport::kStringConcat),
      9 => Some(WellKnownImport::kStringEquals),
      10 => Some(WellKnownImport::kStringFromCharCode),
      11 => Some(WellKnownImport::kStringFromCodePoint),
      12 => Some(WellKnownImport::kStringFromUtf8Array),
      13 => Some(WellKnownImport::kStringFromWtf16Array),
      14 => Some(WellKnownImport::kStringIntoUtf8Array),
      15 => Some(WellKnownImport::kStringLength),
      16 => Some(WellKnownImport::kStringMeasureUtf8),
      17 => Some(WellKnownImport::kStringSubstring),
      18 => Some(WellKnownImport::kStringTest),
      19 => Some(WellKnownImport::kStringToUtf8Array),
      20 => Some(WellKnownImport::kStringToWtf16Array),
      21 => Some(WellKnownImport::kDataViewGetBigInt64),
      22 => Some(WellKnownImport::kDataViewGetBigUint64),
      23 => Some(WellKnownImport::kDataViewGetFloat32),
      24 => Some(WellKnownImport::kDataViewGetFloat64),
      25 => Some(WellKnownImport::kDataViewGetInt8),
      26 => Some(WellKnownImport::kDataViewGetInt16),
      27 => Some(WellKnownImport::kDataViewGetInt32),
      28 => Some(WellKnownImport::kDataViewGetUint8),
      29 => Some(WellKnownImport::kDataViewGetUint16),
      30 => Some(WellKnownImport::kDataViewGetUint32),
      31 => Some(WellKnownImport::kDataViewSetBigInt64),
      32 => Some(WellKnownImport::kDataViewSetBigUint64),
      33 => Some(WellKnownImport::kDataViewSetFloat32),
      34 => Some(WellKnownImport::kDataViewSetFloat64),
      35 => Some(WellKnownImport::kDataViewSetInt8),
      36 => Some(WellKnownImport::kDataViewSetInt16),
      37 => Some(WellKnownImport::kDataViewSetInt32),
      38 => Some(WellKnownImport::kDataViewSetUint8),
      39 => Some(WellKnownImport::kDataViewSetUint16),
      40 => Some(WellKnownImport::kDataViewSetUint32),
      41 => Some(WellKnownImport::kDataViewByteLength),
      42 => Some(WellKnownImport::kMathF64Acos),
      43 => Some(WellKnownImport::kMathF64Asin),
      44 => Some(WellKnownImport::kMathF64Atan),
      45 => Some(WellKnownImport::kMathF64Atan2),
      46 => Some(WellKnownImport::kMathF64Cos),
      47 => Some(WellKnownImport::kMathF64Sin),
      48 => Some(WellKnownImport::kMathF64Tan),
      49 => Some(WellKnownImport::kMathF64Exp),
      50 => Some(WellKnownImport::kMathF64Log),
      51 => Some(WellKnownImport::kMathF64Pow),
      52 => Some(WellKnownImport::kMathF64Sqrt),
      53 => Some(WellKnownImport::kDoubleToString),
      54 => Some(WellKnownImport::kIntToString),
      55 => Some(WellKnownImport::kParseFloat),
      56 => Some(WellKnownImport::kStringIndexOf),
      57 => Some(WellKnownImport::kStringIndexOfImported),
      58 => Some(WellKnownImport::kStringToLocaleLowerCaseStringref),
      59 => Some(WellKnownImport::kStringToLowerCaseStringref),
      60 => Some(WellKnownImport::kStringToLowerCaseImported),
      61 => Some(WellKnownImport::kFastAPICall),
      _ => None,
    }
  }
}

pub struct NativeModule; // Placeholder, as the implementation is not provided.

// For debugging/tracing.
pub fn well_known_import_name(wki: WellKnownImport) -> &'static str {
  match wki {
    WellKnownImport::kUninstantiated => "kUninstantiated",
    WellKnownImport::kGeneric => "kGeneric",
    WellKnownImport::kLinkError => "kLinkError",
    WellKnownImport::kFirstCompileTimeImport => "kFirstCompileTimeImport",
    WellKnownImport::kStringCast => "kStringCast",
    WellKnownImport::kStringCharCodeAt => "kStringCharCodeAt",
    WellKnownImport::kStringCodePointAt => "kStringCodePointAt",
    WellKnownImport::kStringCompare => "kStringCompare",
    WellKnownImport::kStringConcat => "kStringConcat",
    WellKnownImport::kStringEquals => "kStringEquals",
    WellKnownImport::kStringFromCharCode => "kStringFromCharCode",
    WellKnownImport::kStringFromCodePoint => "kStringFromCodePoint",
    WellKnownImport::kStringFromUtf8Array => "kStringFromUtf8Array",
    WellKnownImport::kStringFromWtf16Array => "kStringFromWtf16Array",
    WellKnownImport::kStringIntoUtf8Array => "kStringIntoUtf8Array",
    WellKnownImport::kStringLength => "kStringLength",
    WellKnownImport::kStringMeasureUtf8 => "kStringMeasureUtf8",
    WellKnownImport::kStringSubstring => "kStringSubstring",
    WellKnownImport::kStringTest => "kStringTest",
    WellKnownImport::kStringToUtf8Array => "kStringToUtf8Array",
    WellKnownImport::kStringToWtf16Array => "kStringToWtf16Array",
    WellKnownImport::kDataViewGetBigInt64 => "kDataViewGetBigInt64",
    WellKnownImport::kDataViewGetBigUint64 => "kDataViewGetBigUint64",
    WellKnownImport::kDataViewGetFloat32 => "kDataViewGetFloat32",
    WellKnownImport::kDataViewGetFloat64 => "kDataViewGetFloat64",
    WellKnownImport::kDataViewGetInt8 => "kDataViewGetInt8",
    WellKnownImport::kDataViewGetInt16 => "kDataViewGetInt16",
    WellKnownImport::kDataViewGetInt32 => "kDataViewGetInt32",
    WellKnownImport::kDataViewGetUint8 => "kDataViewGetUint8",
    WellKnownImport::kDataViewGetUint16 => "kDataViewGetUint16",
    WellKnownImport::kDataViewGetUint32 => "kDataViewGetUint32",
    WellKnownImport::kDataViewSetBigInt64 => "kDataViewSetBigInt64",
    WellKnownImport::kDataViewSetBigUint64 => "kDataViewSetBigUint64",
    WellKnownImport::kDataViewSetFloat32 => "kDataViewSetFloat32",
    WellKnownImport::kDataViewSetFloat64 => "kDataViewSetFloat64",
    WellKnownImport::kDataViewSetInt8 => "kDataViewSetInt8",
    WellKnownImport::kDataViewSetInt16 => "kDataViewSetInt16",
    WellKnownImport::kDataViewSetInt32 => "kDataViewSetInt32",
    WellKnownImport::kDataViewSetUint8 => "kDataViewSetUint8",
    WellKnownImport::kDataViewSetUint16 => "kDataViewSetUint16",
    WellKnownImport::kDataViewSetUint32 => "kDataViewSetUint32",
    WellKnownImport::kDataViewByteLength => "kDataViewByteLength",
    WellKnownImport::kMathF64Acos => "kMathF64Acos",
    WellKnownImport::kMathF64Asin => "kMathF64Asin",
    WellKnownImport::kMathF64Atan => "kMathF64Atan",
    WellKnownImport::kMathF64Atan2 => "kMathF64Atan2",
    WellKnownImport::kMathF64Cos => "kMathF64Cos",
    WellKnownImport::kMathF64Sin => "kMathF64Sin",
    WellKnownImport::kMathF64Tan => "kMathF64Tan",
    WellKnownImport::kMathF64Exp => "kMathF64Exp",
    WellKnownImport::kMathF64Log => "kMathF64Log",
    WellKnownImport::kMathF64Pow => "kMathF64Pow",
    WellKnownImport::kMathF64Sqrt => "kMathF64Sqrt",
    WellKnownImport::kDoubleToString => "kDoubleToString",
    WellKnownImport::kIntToString => "kIntToString",
    WellKnownImport::kParseFloat => "kParseFloat",
    WellKnownImport::kStringIndexOf => "kStringIndexOf",
    WellKnownImport::kStringIndexOfImported => "kStringIndexOfImported",
    WellKnownImport::kStringToLocaleLowerCaseStringref => "kStringToLocaleLowerCaseStringref",
    WellKnownImport::kStringToLowerCaseStringref => "kStringToLowerCaseStringref",
    WellKnownImport::kStringToLowerCaseImported => "kStringToLowerCaseImported",
    WellKnownImport::kFastAPICall => "kFastAPICall",
  }
}

pub fn is_compile_time_import(wki: WellKnownImport) -> bool {
  (WellKnownImport::kFirstCompileTimeImport as u8) <= (wki as u8)
    && (wki as u8) <= (WellKnownImport::kLastCompileTimeImport as u8)
}

pub struct WellKnownImportsList {
  statuses_: Vec<AtomicU8>,
  #[cfg(debug_assertions)]
  size_: i32,
}

impl WellKnownImportsList {
  pub enum UpdateResult {
    kFoundIncompatibility,
    kOK,
  }

  pub fn new() -> Self {
    WellKnownImportsList {
      statuses_: Vec::new(),
      #[cfg(debug_assertions)]
      size_: -1,
    }
  }

  pub fn initialize(&mut self, size: usize) {
    #[cfg(debug_assertions)]
    assert_eq!(self.size_, -1);

    self.statuses_ = (0..size)
      .map(|_| AtomicU8::new(WellKnownImport::kUninstantiated as u8))
      .collect();
    #[cfg(debug_assertions)]
    self.size_ = size as i32;

    // No need for explicit initialization loop since AtomicU8 initializes to 0 (kUninstantiated).
    // The C++ code used std::atomic_init, but Rust's AtomicU8 already handles this.
  }

  pub fn initialize_from_entries(&mut self, entries: &[WellKnownImport]) {
    self.statuses_ = entries
      .iter()
      .map(|&wki| AtomicU8::new(wki as u8))
      .collect();
    #[cfg(debug_assertions)]
    self.size_ = entries.len() as i32;
  }

  pub fn get(&self, index: usize) -> Option<WellKnownImport> {
    #[cfg(debug_assertions)]
    assert!(index < self.size_ as usize);
    WellKnownImport::from_u8(self.statuses_[index].load(Ordering::Relaxed))
  }

  pub fn update(&mut self, entries: &[WellKnownImport]) -> UpdateResult {
    for (i, &entry) in entries.iter().enumerate() {
      let current = self.statuses_[i].load(Ordering::Relaxed);
      if current != WellKnownImport::kUninstantiated as u8 && current != entry as u8 {
        return UpdateResult::kFoundIncompatibility;
      }
      self.statuses_[i].store(entry as u8, Ordering::Relaxed);
    }
    UpdateResult::kOK
  }
}

impl Default for WellKnownImportsList {
    fn default() -> Self {
        Self::new()
    }
}