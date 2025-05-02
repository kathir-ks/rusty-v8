// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// DataView operations that are handled as well-known imports.
macro_rules! data_view_op_list {
    ($v:ident) => {
        $v!(BigInt64);
        $v!(BigUint64);
        $v!(Float32);
        $v!(Float64);
        $v!(Int8);
        $v!(Int16);
        $v!(Int32);
        $v!(Uint8);
        $v!(Uint16);
        $v!(Uint32);
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DataViewOp {
    #[allow(non_camel_case_types)]
    kGetBigInt64,
    #[allow(non_camel_case_types)]
    kSetBigInt64,
    #[allow(non_camel_case_types)]
    kGetBigUint64,
    #[allow(non_camel_case_types)]
    kSetBigUint64,
    #[allow(non_camel_case_types)]
    kGetFloat32,
    #[allow(non_camel_case_types)]
    kSetFloat32,
    #[allow(non_camel_case_types)]
    kGetFloat64,
    #[allow(non_camel_case_types)]
    kSetFloat64,
    #[allow(non_camel_case_types)]
    kGetInt8,
    #[allow(non_camel_case_types)]
    kSetInt8,
    #[allow(non_camel_case_types)]
    kGetInt16,
    #[allow(non_camel_case_types)]
    kSetInt16,
    #[allow(non_camel_case_types)]
    kGetInt32,
    #[allow(non_camel_case_types)]
    kSetInt32,
    #[allow(non_camel_case_types)]
    kGetUint8,
    #[allow(non_camel_case_types)]
    kSetUint8,
    #[allow(non_camel_case_types)]
    kGetUint16,
    #[allow(non_camel_case_types)]
    kSetUint16,
    #[allow(non_camel_case_types)]
    kGetUint32,
    #[allow(non_camel_case_types)]
    kSetUint32,
    kByteLength,
}

impl DataViewOp {
    pub fn to_string(&self) -> &'static str {
        match self {
            DataViewOp::kGetBigInt64 => "DataView.prototype.getBigInt64",
            DataViewOp::kSetBigInt64 => "DataView.prototype.setBigInt64",
            DataViewOp::kGetBigUint64 => "DataView.prototype.getBigUint64",
            DataViewOp::kSetBigUint64 => "DataView.prototype.setBigUint64",
            DataViewOp::kGetFloat32 => "DataView.prototype.getFloat32",
            DataViewOp::kSetFloat32 => "DataView.prototype.setFloat32",
            DataViewOp::kGetFloat64 => "DataView.prototype.getFloat64",
            DataViewOp::kSetFloat64 => "DataView.prototype.setFloat64",
            DataViewOp::kGetInt8 => "DataView.prototype.getInt8",
            DataViewOp::kSetInt8 => "DataView.prototype.setInt8",
            DataViewOp::kGetInt16 => "DataView.prototype.getInt16",
            DataViewOp::kSetInt16 => "DataView.prototype.setInt16",
            DataViewOp::kGetInt32 => "DataView.prototype.getInt32",
            DataViewOp::kSetInt32 => "DataView.prototype.setInt32",
            DataViewOp::kGetUint8 => "DataView.prototype.getUint8",
            DataViewOp::kSetUint8 => "DataView.prototype.setUint8",
            DataViewOp::kGetUint16 => "DataView.prototype.getUint16",
            DataViewOp::kSetUint16 => "DataView.prototype.setUint16",
            DataViewOp::kGetUint32 => "DataView.prototype.getUint32",
            DataViewOp::kSetUint32 => "DataView.prototype.setUint32",
            DataViewOp::kByteLength => "get DataView.prototype.byteLength",
        }
    }
}

// NOTE: The UNREACHABLE macro from the original C++ code doesn't have a direct
// equivalent in Rust.  A common approach is to use `unreachable!()` or to
// return a default value combined with a `debug_assert!` that the default
// is never reached in production code. For now, I chose the first option.