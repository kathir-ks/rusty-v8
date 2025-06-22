// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod elements_kind {
    use std::{fmt, ops::RangeInclusive};

    macro_rules! typed_arrays_base {
        ($V:ident) => {
            $V!(Uint8, uint8, UINT8, u8);
            $V!(Int8, int8, INT8, i8);
            $V!(Uint16, uint16, UINT16, u16);
            $V!(Int16, int16, INT16, i16);
            $V!(Uint32, uint32, UINT32, u32);
            $V!(Int32, int32, INT32, i32);
            $V!(BigUint64, biguint64, BIGUINT64, u64);
            $V!(BigInt64, bigint64, BIGINT64, i64);
            $V!(Uint8Clamped, uint8_clamped, UINT8_CLAMPED, u8);
            $V!(Float32, float32, FLOAT32, f32);
            $V!(Float64, float64, FLOAT64, f64);
        };
    }

    macro_rules! typed_arrays_float16 {
        ($V:ident) => {
            $V!(Float16, float16, FLOAT16, u16);
        };
    }

    macro_rules! typed_arrays {
        ($V:ident) => {
            typed_arrays_base!($V);
            typed_arrays_float16!($V);
        };
    }

    macro_rules! rab_gsab_typed_arrays_base {
        ($V:ident) => {
            $V!(RabGsabUint8, rab_gsab_uint8, RAB_GSAB_UINT8, u8);
            $V!(RabGsabInt8, rab_gsab_int8, RAB_GSAB_INT8, i8);
            $V!(RabGsabUint16, rab_gsab_uint16, RAB_GSAB_UINT16, u16);
            $V!(RabGsabInt16, rab_gsab_int16, RAB_GSAB_INT16, i16);
            $V!(RabGsabUint32, rab_gsab_uint32, RAB_GSAB_UINT32, u32);
            $V!(RabGsabInt32, rab_gsab_int32, RAB_GSAB_INT32, i32);
            $V!(RabGsabBigUint64, rab_gsab_biguint64, RAB_GSAB_BIGUINT64, u64);
            $V!(RabGsabBigInt64, rab_gsab_bigint64, RAB_GSAB_BIGINT64, i64);
            $V!(RabGsabUint8Clamped, rab_gsab_uint8_clamped, RAB_GSAB_UINT8_CLAMPED, u8);
            $V!(RabGsabFloat32, rab_gsab_float32, RAB_GSAB_FLOAT32, f32);
            $V!(RabGsabFloat64, rab_gsab_float64, RAB_GSAB_FLOAT64, f64);
        };
    }

    macro_rules! rab_gsab_typed_arrays_float16 {
        ($V:ident) => {
            $V!(RabGsabFloat16, rab_gsab_float16, RAB_GSAB_FLOAT16, u16);
        };
    }

    macro_rules! rab_gsab_typed_arrays {
        ($V:ident) => {
            rab_gsab_typed_arrays_base!($V);
            rab_gsab_typed_arrays_float16!($V);
        };
    }

    macro_rules! rab_gsab_typed_arrays_with_typed_array_type_base {
        ($V:ident) => {
            $V!(Uint8, rab_gsab_uint8, RAB_GSAB_UINT8, u8);
            $V!(Int8, rab_gsab_int8, RAB_GSAB_INT8, i8);
            $V!(Uint16, rab_gsab_uint16, RAB_GSAB_UINT16, u16);
            $V!(Int16, rab_gsab_int16, RAB_GSAB_INT16, i16);
            $V!(Uint32, rab_gsab_uint32, RAB_GSAB_UINT32, u32);
            $V!(Int32, rab_gsab_int32, RAB_GSAB_INT32, i32);
            $V!(BigUint64, rab_gsab_biguint64, RAB_GSAB_BIGUINT64, u64);
            $V!(BigInt64, rab_gsab_bigint64, RAB_GSAB_BIGINT64, i64);
            $V!(Uint8Clamped, rab_gsab_uint8_clamped, RAB_GSAB_UINT8_CLAMPED, u8);
            $V!(Float32, rab_gsab_float32, RAB_GSAB_FLOAT32, f32);
            $V!(Float64, rab_gsab_float64, RAB_GSAB_FLOAT64, f64);
        };
    }

    macro_rules! rab_gsab_typed_arrays_with_typed_array_type_float16 {
        ($V:ident) => {
            $V!(Float16, rab_gsab_float16, RAB_GSAB_FLOAT16, u16);
        };
    }

    macro_rules! rab_gsab_typed_arrays_with_typed_array_type {
        ($V:ident) => {
            rab_gsab_typed_arrays_with_typed_array_type_base!($V);
            rab_gsab_typed_arrays_with_typed_array_type_float16!($V);
        };
    }

    macro_rules! rab_gsab_typed_arrays_with_non_rab_gsab_elements_kind_base {
        ($V:ident) => {
            $V!(RabGsabUint8, rab_gsab_uint8, RAB_GSAB_UINT8, u8, UINT8);
            $V!(RabGsabInt8, rab_gsab_int8, RAB_GSAB_INT8, i8, INT8);
            $V!(RabGsabUint16, rab_gsab_uint16, RAB_GSAB_UINT16, u16, UINT16);
            $V!(RabGsabInt16, rab_gsab_int16, RAB_GSAB_INT16, i16, INT16);
            $V!(RabGsabUint32, rab_gsab_uint32, RAB_GSAB_UINT32, u32, UINT32);
            $V!(RabGsabInt32, rab_gsab_int32, RAB_GSAB_INT32, i32, INT32);
            $V!(RabGsabBigUint64, rab_gsab_biguint64, RAB_GSAB_BIGUINT64, u64, BIGUINT64);
            $V!(RabGsabBigInt64, rab_gsab_bigint64, RAB_GSAB_BIGINT64, i64, BIGINT64);
            $V!(RabGsabUint8Clamped, rab_gsab_uint8_clamped, RAB_GSAB_UINT8_CLAMPED, u8, UINT8_CLAMPED);
            $V!(RabGsabFloat32, rab_gsab_float32, RAB_GSAB_FLOAT32, f32, FLOAT32);
            $V!(RabGsabFloat64, rab_gsab_float64, RAB_GSAB_FLOAT64, f64, FLOAT64);
        };
    }

    macro_rules! rab_gsab_typed_arrays_with_non_rab_gsab_elements_kind_float16 {
        ($V:ident) => {
            $V!(RabGsabFloat16, rab_gsab_float16, RAB_GSAB_FLOAT16, u16, FLOAT16);
        };
    }

    macro_rules! rab_gsab_typed_arrays_with_non_rab_gsab_elements_kind {
        ($V:ident) => {
            rab_gsab_typed_arrays_with_non_rab_gsab_elements_kind_base!($V);
            rab_gsab_typed_arrays_with_non_rab_gsab_elements_kind_float16!($V);
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    pub enum ElementsKind {
        // The "fast" kind for elements that only contain SMI values. Must be first
        // to make it possible to efficiently check maps for this kind.
        PACKED_SMI_ELEMENTS,
        HOLEY_SMI_ELEMENTS,

        // The "fast" kind for tagged values. Must be second to make it possible to
        // efficiently check maps for this and the PACKED_SMI_ELEMENTS kind
        // together at once.
        PACKED_ELEMENTS,
        HOLEY_ELEMENTS,

        // The "fast" kind for unwrapped, non-tagged double values.
        PACKED_DOUBLE_ELEMENTS,
        HOLEY_DOUBLE_ELEMENTS,

        // The nonextensible kind for elements.
        PACKED_NONEXTENSIBLE_ELEMENTS,
        HOLEY_NONEXTENSIBLE_ELEMENTS,

        // The sealed kind for elements.
        PACKED_SEALED_ELEMENTS,
        HOLEY_SEALED_ELEMENTS,

        // The frozen kind for elements.
        PACKED_FROZEN_ELEMENTS,
        HOLEY_FROZEN_ELEMENTS,

        // SharedArray elements kind. A FAST_SEALED_ELEMENTS variation useful to
        // code specific paths for SharedArrays.
        SHARED_ARRAY_ELEMENTS,

        // The "slow" kind.
        DICTIONARY_ELEMENTS,

        // Elements kind of the "arguments" object (only in sloppy mode).
        FAST_SLOPPY_ARGUMENTS_ELEMENTS,
        SLOW_SLOPPY_ARGUMENTS_ELEMENTS,

        // For string wrapper objects ("new String('...')"), the string's characters
        // are overlaid onto a regular elements backing store.
        FAST_STRING_WRAPPER_ELEMENTS,
        SLOW_STRING_WRAPPER_ELEMENTS,

        // Fixed typed arrays.
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        #[allow(clippy::upper_case_acronyms)]
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        
        
        
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        
        

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_variables)]
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }
        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $TYPE##_ELEMENTS,
            };
        }

        #[allow(unused_macros)]
        #[macro_use]
        macro_rules! rab_gsab_typed_array_elements_kind {
            ($Type:ident, $type:ident, $TYPE:ident, $ctype:ty) => {
                $