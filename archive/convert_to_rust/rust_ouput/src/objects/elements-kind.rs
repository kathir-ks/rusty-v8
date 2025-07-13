// Converted from V8 C++ source files:
// Header: elements-kind.h
// Implementation: elements-kind.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod elements_kind {
    use std::fmt;
    use std::ops::RangeInclusive;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    pub enum ElementsKind {
        // The "fast" kind for elements that only contain SMI values. Must be first
        // to make it possible to efficiently check maps for this kind.
        PACKED_SMI_ELEMENTS = 0,
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
        UINT8_ELEMENTS,
        INT8_ELEMENTS,
        UINT16_ELEMENTS,
        INT16_ELEMENTS,
        UINT32_ELEMENTS,
        INT32_ELEMENTS,
        BIGUINT64_ELEMENTS,
        BIGINT64_ELEMENTS,
        UINT8_CLAMPED_ELEMENTS,
        FLOAT32_ELEMENTS,
        FLOAT64_ELEMENTS,
        FLOAT16_ELEMENTS,
        RabGsabUint8_ELEMENTS,
        RabGsabInt8_ELEMENTS,
        RabGsabUint16_ELEMENTS,
        RabGsabInt16_ELEMENTS,
        RabGsabUint32_ELEMENTS,
        RabGsabInt32_ELEMENTS,
        RabGsabBigUint64_ELEMENTS,
        RabGsabBigInt64_ELEMENTS,
        RabGsabUint8Clamped_ELEMENTS,
        RabGsabFloat32_ELEMENTS,
        RabGsabFloat64_ELEMENTS,
        RabGsabFloat16_ELEMENTS,

        // WasmObject elements kind. The actual elements type is read from the
        // respective WasmTypeInfo.
        WASM_ARRAY_ELEMENTS,

        // Sentinel ElementsKind for objects with no elements.
        NO_ELEMENTS,
    }

    impl ElementsKind {
        pub const FIRST_ELEMENTS_KIND: ElementsKind = ElementsKind::PACKED_SMI_ELEMENTS;
        pub const LAST_ELEMENTS_KIND: ElementsKind = ElementsKind::RabGsabFloat16_ELEMENTS;
        pub const FIRST_FAST_ELEMENTS_KIND: ElementsKind = ElementsKind::PACKED_SMI_ELEMENTS;
        pub const LAST_FAST_ELEMENTS_KIND: ElementsKind = ElementsKind::HOLEY_DOUBLE_ELEMENTS;
        pub const FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND: ElementsKind = ElementsKind::UINT8_ELEMENTS;
        pub const LAST_FIXED_TYPED_ARRAY_ELEMENTS_KIND: ElementsKind = ElementsKind::FLOAT16_ELEMENTS;
        pub const FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND: ElementsKind = ElementsKind::RabGsabUint8_ELEMENTS;
        pub const LAST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND: ElementsKind = ElementsKind::RabGsabFloat16_ELEMENTS;
        pub const TERMINAL_FAST_ELEMENTS_KIND: ElementsKind = ElementsKind::HOLEY_ELEMENTS;
        pub const FIRST_ANY_NONEXTENSIBLE_ELEMENTS_KIND: ElementsKind = ElementsKind::PACKED_NONEXTENSIBLE_ELEMENTS;
        pub const LAST_ANY_NONEXTENSIBLE_ELEMENTS_KIND: ElementsKind = ElementsKind::SHARED_ARRAY_ELEMENTS;
        pub const FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND: ElementsKind = ElementsKind::UINT8_ELEMENTS;
        pub const LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND: ElementsKind = ElementsKind::BIGINT64_ELEMENTS;

        #[cfg(not(feature = "v8_compress_pointers"))]
        pub const SYSTEM_POINTER_ELEMENTS: ElementsKind = ElementsKind::PACKED_ELEMENTS;
        #[cfg(feature = "v8_compress_pointers")]
        pub const SYSTEM_POINTER_ELEMENTS: ElementsKind = ElementsKind::PACKED_DOUBLE_ELEMENTS;

        pub const ELEMENTS_KIND_COUNT: i32 = ElementsKind::LAST_ELEMENTS_KIND as i32 - ElementsKind::FIRST_ELEMENTS_KIND as i32 + 1;
        pub const FAST_ELEMENTS_KIND_COUNT: i32 = ElementsKind::LAST_FAST_ELEMENTS_KIND as i32 - ElementsKind::FIRST_FAST_ELEMENTS_KIND as i32 + 1;

        // The number to add to a packed elements kind to reach a holey elements kind
        pub const FAST_ELEMENTS_KIND_PACKED_TO_HOLEY: i32 = ElementsKind::HOLEY_SMI_ELEMENTS as i32 - ElementsKind::PACKED_SMI_ELEMENTS as i32;

        pub const ELEMENTS_KIND_BITS: i32 = 6;
        pub const FAST_ELEMENTS_KIND_BITS: i32 = 3;

        pub fn elements_kind_to_shift_size(self) -> i32 {
            match self {
                ElementsKind::UINT8_ELEMENTS |
                ElementsKind::INT8_ELEMENTS |
                ElementsKind::UINT8_CLAMPED_ELEMENTS |
                ElementsKind::RabGsabUint8_ELEMENTS |
                ElementsKind::RabGsabInt8_ELEMENTS |
                ElementsKind::RabGsabUint8Clamped_ELEMENTS => 0,
                ElementsKind::UINT16_ELEMENTS |
                ElementsKind::INT16_ELEMENTS |
                ElementsKind::FLOAT16_ELEMENTS |
                ElementsKind::RabGsabFloat16_ELEMENTS |
                ElementsKind::RabGsabUint16_ELEMENTS |
                ElementsKind::RabGsabInt16_ELEMENTS => 1,
                ElementsKind::UINT32_ELEMENTS |
                ElementsKind::INT32_ELEMENTS |
                ElementsKind::FLOAT32_ELEMENTS |
                ElementsKind::RabGsabUint32_ELEMENTS |
                ElementsKind::RabGsabInt32_ELEMENTS |
                ElementsKind::RabGsabFloat32_ELEMENTS => 2,
                ElementsKind::PACKED_DOUBLE_ELEMENTS |
                ElementsKind::HOLEY_DOUBLE_ELEMENTS |
                ElementsKind::FLOAT64_ELEMENTS |
                ElementsKind::BIGINT64_ELEMENTS |
                ElementsKind::BIGUINT64_ELEMENTS |
                ElementsKind::RabGsabFloat64_ELEMENTS |
                ElementsKind::RabGsabBigInt64_ELEMENTS |
                ElementsKind::RabGsabBigUint64_ELEMENTS => 3,
                ElementsKind::PACKED_SMI_ELEMENTS |
                ElementsKind::PACKED_ELEMENTS |
                ElementsKind::PACKED_FROZEN_ELEMENTS |
                ElementsKind::PACKED_SEALED_ELEMENTS |
                ElementsKind::PACKED_NONEXTENSIBLE_ELEMENTS |
                ElementsKind::HOLEY_SMI_ELEMENTS |
                ElementsKind::HOLEY_ELEMENTS |
                ElementsKind::HOLEY_FROZEN_ELEMENTS |
                ElementsKind::HOLEY_SEALED_ELEMENTS |
                ElementsKind::HOLEY_NONEXTENSIBLE_ELEMENTS |
                ElementsKind::DICTIONARY_ELEMENTS |
                ElementsKind::FAST_SLOPPY_ARGUMENTS_ELEMENTS |
                ElementsKind::SLOW_SLOPPY_ARGUMENTS_ELEMENTS |
                ElementsKind::FAST_STRING_WRAPPER_ELEMENTS |
                ElementsKind::SLOW_STRING_WRAPPER_ELEMENTS |
                ElementsKind::SHARED_ARRAY_ELEMENTS => 3, // Assuming kTaggedSizeLog2 == 3
                ElementsKind::WASM_ARRAY_ELEMENTS |
                ElementsKind::NO_ELEMENTS => panic!("CONSTEXPR_UNREACHABLE"),
            }
        }

        pub fn elements_kind_to_byte_size(self) -> i32 {
            1 << self.elements_kind_to_shift_size()
        }

        pub fn get_initial_fast_elements_kind() -> ElementsKind {
            ElementsKind::PACKED_SMI_ELEMENTS
        }

        pub fn get_fast_elements_kind_from_sequence_index(sequence_number: i32) -> ElementsKind {
            let sequence = [
                ElementsKind::PACKED_SMI_ELEMENTS,
                ElementsKind::HOLEY_SMI_ELEMENTS,
                ElementsKind::PACKED_DOUBLE_ELEMENTS,
                ElementsKind::HOLEY_DOUBLE_ELEMENTS,
                ElementsKind::PACKED_ELEMENTS,
                ElementsKind::HOLEY_ELEMENTS,
            ];
            sequence[sequence_number as usize]
        }

        pub fn get_sequence_index_from_fast_elements_kind(elements_kind: ElementsKind) -> i32 {
            match elements_kind {
                ElementsKind::PACKED_SMI_ELEMENTS => 0,
                ElementsKind::HOLEY_SMI_ELEMENTS => 1,
                ElementsKind::PACKED_DOUBLE_ELEMENTS => 2,
                ElementsKind::HOLEY_DOUBLE_ELEMENTS => 3,
                ElementsKind::PACKED_ELEMENTS => 4,
                ElementsKind::HOLEY_ELEMENTS => 5,
                _ => panic!("UNREACHABLE"),
            }
        }

        pub fn get_next_transition_elements_kind(elements_kind: ElementsKind) -> ElementsKind {
            let index = ElementsKind::get_sequence_index_from_fast_elements_kind(elements_kind);
            ElementsKind::get_fast_elements_kind_from_sequence_index(index + 1)
        }

        pub fn is_dictionary_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::DICTIONARY_ELEMENTS
        }

        pub fn is_fast_arguments_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::FAST_SLOPPY_ARGUMENTS_ELEMENTS
        }

        pub fn is_slow_arguments_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::SLOW_SLOPPY_ARGUMENTS_ELEMENTS
        }

        pub fn is_sloppy_arguments_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::FAST_SLOPPY_ARGUMENTS_ELEMENTS as u8..=ElementsKind::SLOW_SLOPPY_ARGUMENTS_ELEMENTS as u8).contains(&(kind as u8))
        }

        pub fn is_string_wrapper_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::FAST_STRING_WRAPPER_ELEMENTS as u8..=ElementsKind::SLOW_STRING_WRAPPER_ELEMENTS as u8).contains(&(kind as u8))
        }

        pub fn is_typed_array_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8..=ElementsKind::LAST_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8).contains(&(kind as u8))
        }

        pub fn is_rab_gsab_typed_array_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8..=ElementsKind::LAST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8).contains(&(kind as u8))
        }

        pub fn is_typed_array_or_rab_gsab_typed_array_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8..=ElementsKind::LAST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8).contains(&(kind as u8))
        }

        pub fn is_big_int_typed_array_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::BIGINT64_ELEMENTS || kind == ElementsKind::BIGUINT64_ELEMENTS ||
                kind == ElementsKind::RabGsabBigInt64_ELEMENTS ||
                kind == ElementsKind::RabGsabBigUint64_ELEMENTS
        }

        pub fn is_float16_typed_array_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::FLOAT16_ELEMENTS || kind == ElementsKind::RabGsabFloat16_ELEMENTS
        }

        pub fn is_float_typed_array_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::FLOAT16_ELEMENTS || kind == ElementsKind::RabGsabFloat16_ELEMENTS ||
                kind == ElementsKind::FLOAT32_ELEMENTS || kind == ElementsKind::FLOAT64_ELEMENTS ||
                kind == ElementsKind::RabGsabFloat32_ELEMENTS || kind == ElementsKind::RabGsabFloat64_ELEMENTS
        }

        pub fn is_signed_int_typed_array_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::INT8_ELEMENTS || kind == ElementsKind::RabGsabInt8_ELEMENTS ||
                kind == ElementsKind::INT16_ELEMENTS || kind == ElementsKind::RabGsabInt16_ELEMENTS ||
                kind == ElementsKind::INT32_ELEMENTS || kind == ElementsKind::RabGsabInt32_ELEMENTS
        }

        pub fn is_unsigned_int_typed_array_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::UINT8_CLAMPED_ELEMENTS ||
                kind == ElementsKind::RabGsabUint8Clamped_ELEMENTS || kind == ElementsKind::UINT8_ELEMENTS ||
                kind == ElementsKind::RabGsabUint8_ELEMENTS || kind == ElementsKind::UINT16_ELEMENTS ||
                kind == ElementsKind::RabGsabUint16_ELEMENTS || kind == ElementsKind::UINT32_ELEMENTS ||
                kind == ElementsKind::RabGsabUint32_ELEMENTS
        }

        pub fn is_wasm_array_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::WASM_ARRAY_ELEMENTS
        }

        pub fn is_shared_array_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::SHARED_ARRAY_ELEMENTS
        }

        pub fn is_terminal_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::TERMINAL_FAST_ELEMENTS_KIND ||
                ElementsKind::is_typed_array_or_rab_gsab_typed_array_elements_kind(kind) ||
                ElementsKind::is_rab_gsab_typed_array_elements_kind(kind)
        }

        pub fn is_fast_elements_kind(kind: ElementsKind) -> bool {
            kind as u8 <= ElementsKind::LAST_FAST_ELEMENTS_KIND as u8
        }

        pub fn is_transition_elements_kind(kind: ElementsKind) -> bool {
            ElementsKind::is_fast_elements_kind(kind) ||
                ElementsKind::is_typed_array_or_rab_gsab_typed_array_elements_kind(kind) ||
                kind == ElementsKind::FAST_SLOPPY_ARGUMENTS_ELEMENTS ||
                kind == ElementsKind::FAST_STRING_WRAPPER_ELEMENTS
        }

        pub fn is_double_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::PACKED_DOUBLE_ELEMENTS as u8..=ElementsKind::HOLEY_DOUBLE_ELEMENTS as u8).contains(&(kind as u8))
        }

        // This predicate is used for disabling respective functionality in builtins.
        pub fn is_any_nonextensible_elements_kind_unchecked(kind: ElementsKind) -> bool {
            (ElementsKind::FIRST_ANY_NONEXTENSIBLE_ELEMENTS_KIND as u8..=ElementsKind::LAST_ANY_NONEXTENSIBLE_ELEMENTS_KIND as u8).contains(&(kind as u8))
        }

        pub fn is_any_nonextensible_elements_kind(kind: ElementsKind) -> bool {
            ElementsKind::is_any_nonextensible_elements_kind_unchecked(kind)
        }

        pub fn is_nonextensible_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::PACKED_NONEXTENSIBLE_ELEMENTS as u8..=ElementsKind::HOLEY_NONEXTENSIBLE_ELEMENTS as u8).contains(&(kind as u8))
        }

        pub fn is_sealed_elements_kind(kind: ElementsKind) -> bool {
            ElementsKind::is_shared_array_elements_kind(kind) ||
                (ElementsKind::PACKED_SEALED_ELEMENTS as u8..=ElementsKind::HOLEY_SEALED_ELEMENTS as u8).contains(&(kind as u8))
        }

        pub fn is_frozen_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::PACKED_FROZEN_ELEMENTS as u8..=ElementsKind::HOLEY_FROZEN_ELEMENTS as u8).contains(&(kind as u8))
        }

        pub fn is_fast_or_nonextensible_or_sealed_elements_kind(kind: ElementsKind) -> bool {
            let result = kind as u8 <= ElementsKind::HOLEY_SEALED_ELEMENTS as u8;
            if result {
                debug_assert!(ElementsKind::is_fast_elements_kind(kind) ||
                              ElementsKind::is_nonextensible_elements_kind(kind) ||
                              ElementsKind::is_sealed_elements_kind(kind));
                debug_assert!(!ElementsKind::is_frozen_elements_kind(kind));
            }
            result
        }

        pub fn is_smi_or_object_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::PACKED_SMI_ELEMENTS as u8..=ElementsKind::HOLEY_ELEMENTS as u8).contains(&(kind as u8))
        }

        pub fn is_smi_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::PACKED_SMI_ELEMENTS as u8..=ElementsKind::HOLEY_SMI_ELEMENTS as u8).contains(&(kind as u8))
        }

        pub fn is_fast_number_elements_kind(kind: ElementsKind) -> bool {
            ElementsKind::is_smi_elements_kind(kind) || ElementsKind::is_double_elements_kind(kind)
        }

        pub fn is_object_elements_kind(kind: ElementsKind) -> bool {
            (ElementsKind::PACKED_ELEMENTS as u8..=ElementsKind::HOLEY_ELEMENTS as u8).contains(&(kind as u8))
        }

        pub fn is_any_holey_nonextensible_elements_kind(kind: ElementsKind) -> bool {
            kind == ElementsKind::HOLEY_NONEXTENSIBLE_ELEMENTS ||
                kind == ElementsKind::HOLEY_SEALED_ELEMENTS || kind == ElementsKind::HOLEY_FROZEN_ELEMENTS
        }

        pub fn is_holey_elements_kind(kind: ElementsKind) -> bool {
            (kind as i32) % 2 == 1 && (kind as u8) <= ElementsKind::HOLEY_DOUBLE_ELEMENTS as u8
        }

        pub fn is_holey_elements_kind_for_read(kind: ElementsKind) -> bool {
            (kind as i32) % 2 == 1 && (kind as u8) <= ElementsKind::HOLEY_FROZEN_ELEMENTS as u8
        }

        pub fn is_holey_or_dictionary_elements_kind(kind: ElementsKind) -> bool {
            ElementsKind::is_holey_elements_kind_for_read(kind) || kind == ElementsKind::DICTIONARY_ELEMENTS
        }

        pub fn is_fast_packed_elements_kind(kind: ElementsKind) -> bool {
            (kind as i32) % 2 == 0 && (kind as u8) <= ElementsKind::PACKED_DOUBLE_ELEMENTS as u8
        }

        pub fn get_packed_elements_kind(holey_kind: ElementsKind) -> ElementsKind {
            if holey_kind == ElementsKind::HOLEY_SMI_ELEMENTS {
                return ElementsKind::PACKED_SMI_ELEMENTS;
            }
            if holey_kind == ElementsKind::HOLEY_DOUBLE_ELEMENTS {
                return ElementsKind::PACKED_DOUBLE_ELEMENTS;
            }
            if holey_kind == ElementsKind::HOLEY_ELEMENTS {
                return ElementsKind::PACKED_ELEMENTS;
            }
            holey_kind
        }

        pub fn get_holey_elements_kind(packed_kind: ElementsKind) -> ElementsKind {
            if packed_kind == ElementsKind::PACKED_SMI_ELEMENTS {
                return ElementsKind::HOLEY_SMI_ELEMENTS;
            }
            if packed_kind == ElementsKind::PACKED_DOUBLE_ELEMENTS {
                return ElementsKind::HOLEY_DOUBLE_ELEMENTS;
            }
            if packed_kind == ElementsKind::PACKED_ELEMENTS {
                return ElementsKind::HOLEY_ELEMENTS;
            }
            if packed_kind == ElementsKind::PACKED_NONEXTENSIBLE_ELEMENTS {
                return ElementsKind::HOLEY_NONEXTENSIBLE_ELEMENTS;
            }
            packed_kind
        }

        pub fn get_corresponding_rab_gsab_elements_kind(typed_array_kind: ElementsKind) -> ElementsKind {
            debug_assert!(ElementsKind::is_typed_array_elements_kind(typed_array_kind));
            ElementsKind::from(typed_array_kind as u8 - ElementsKind::FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8 +
                ElementsKind::FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8)
        }

        pub fn get_corresponding_non_rab_gsab_elements_kind(typed_array_kind: ElementsKind) -> ElementsKind {
            debug_assert!(ElementsKind::is_rab_gsab_typed_array_elements_kind(typed_array_kind));
            ElementsKind::from(typed_array_kind as u8 -
                ElementsKind::FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8 +
                ElementsKind::FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND as u8)
        }

        pub fn union_elements_kind_upto_packedness(a_out: &mut ElementsKind, b: ElementsKind) -> bool {
            // Assert that the union of two ElementKinds can be computed via std::max.
            debug_assert!(ElementsKind::PACKED_SMI_ELEMENTS < ElementsKind::HOLEY_SMI_ELEMENTS,
                          "ElementsKind union not computable via std::max.");
            debug_assert!(ElementsKind::PACKED_ELEMENTS < ElementsKind::HOLEY_ELEMENTS,
                          "ElementsKind union not computable via std::max.");
            debug_assert!(ElementsKind::PACKED_DOUBLE_ELEMENTS < ElementsKind::HOLEY_DOUBLE_ELEMENTS,
                          "ElementsKind union not computable via std::max.");

            let a = *a_out;
            match a {
                ElementsKind::HOLEY_SMI_ELEMENTS | ElementsKind::PACKED_SMI_ELEMENTS => {
                    if b == ElementsKind::PACKED_SMI_ELEMENTS || b == ElementsKind::HOLEY_SMI_ELEMENTS {
                        *a_out = std::cmp::max(a, b);
                        return true;
                    }
                }
                ElementsKind::PACKED_ELEMENTS | ElementsKind::HOLEY_ELEMENTS => {
                    if b == ElementsKind::PACKED_ELEMENTS || b == ElementsKind::HOLEY_ELEMENTS {
                        *a_out = std::cmp::max(a, b);
                        return true;
                    }
                }
                ElementsKind::PACKED_DOUBLE_ELEMENTS | ElementsKind::HOLEY_DOUBLE_ELEMENTS => {
                    if b == ElementsKind::PACKED_DOUBLE_ELEMENTS || b == ElementsKind::HOLEY_DOUBLE_ELEMENTS {
                        *a_out = std::cmp::max(a, b);
                        return true;
                    }
                }
                _ => {}
            }
            false
        }

        pub fn union_elements_kind_upto_size(a_out: &mut ElementsKind, b: ElementsKind) -> bool {
            // Assert that the union of two ElementKinds can be computed via std::max.
            debug_assert!(ElementsKind::PACKED_SMI_ELEMENTS < ElementsKind::HOLEY_SMI_ELEMENTS,
                          "ElementsKind union not computable via std::max.");
            debug_assert!(ElementsKind::HOLEY_SMI_ELEMENTS < ElementsKind::PACKED_ELEMENTS,
                          "ElementsKind union not computable via std::max.");
            debug_assert!(ElementsKind::PACKED_ELEMENTS < ElementsKind::HOLEY_ELEMENTS,
                          "ElementsKind union not computable via std::max.");
            debug_assert!(ElementsKind::PACKED_DOUBLE_ELEMENTS < ElementsKind::HOLEY_DOUBLE_ELEMENTS,
                          "ElementsKind union not computable via std::max.");

            let a = *a_out;
            match a {
                ElementsKind::PACKED_SMI_ELEMENTS => {
                    match b {
                        ElementsKind::PACKED_SMI_ELEMENTS |
                        ElementsKind::HOLEY_SMI_ELEMENTS |
                        ElementsKind::PACKED_ELEMENTS |
                        ElementsKind::HOLEY_ELEMENTS => {
                            *a_out = b;
                            return true;
                        }
                        _ => return false,
                    }
                }
                ElementsKind::HOLEY_SMI_ELEMENTS => {
                    match b {
                        ElementsKind::PACKED_SMI_ELEMENTS |
                        ElementsKind::HOLEY_SMI_ELEMENTS => {
                            *a_out = ElementsKind::HOLEY_SMI_ELEMENTS;
                            return true;
                        }
                        ElementsKind::PACKED_ELEMENTS |
                        ElementsKind::HOLEY_ELEMENTS => {
                            *a_out = ElementsKind::HOLEY_ELEMENTS;
                            return true;
                        }
                        _ => return false,
                    }
                }
                ElementsKind::PACKED_ELEMENTS => {
                    match b {
                        ElementsKind::PACKED_SMI_ELEMENTS |
                        ElementsKind::PACKED_ELEMENTS => {
                            *a_out = ElementsKind::PACKED_ELEMENTS;
                            return true;
                        }
                        ElementsKind::HOLEY_SMI_ELEMENTS |
                        ElementsKind::HOLEY_ELEMENTS => {
                            *a_out = ElementsKind::HOLEY_ELEMENTS;
                            return true;
                        }
                        _ => return false,
                    }
                }
                ElementsKind::HOLEY_ELEMENTS => {
                    match b {
                        ElementsKind::PACKED_SMI_ELEMENTS |
                        ElementsKind::HOLEY_SMI_ELEMENTS |
                        ElementsKind::PACKED_ELEMENTS |
                        ElementsKind::HOLEY_ELEMENTS => {
                            *a_out = ElementsKind::HOLEY_ELEMENTS;
                            return true;
                        }
                        _ => return false,
                    }
                }
                ElementsKind::PACKED_DOUBLE_ELEMENTS => {
                    match b {
                        ElementsKind::PACKED_DOUBLE_ELEMENTS |
                        ElementsKind::HOLEY_DOUBLE_ELEMENTS => {
                            *a_out = b;
                            return true;
                        }
                        _ => return false,
                    }
                }
                ElementsKind::HOLEY_DOUBLE_ELEMENTS => {
                    match b {
                        ElementsKind::PACKED_DOUBLE_ELEMENTS |
                        ElementsKind::HOLEY_DOUBLE_ELEMENTS => {
                            *a_out = ElementsKind::HOLEY_DOUBLE_ELEMENTS;
                            return true;
                        }
                        _ => return false,
                    }
                }
                _ => return false,
            }
        }

        pub fn fast_smi_to_object_elements_kind(from_kind: ElementsKind) -> ElementsKind {
            debug_assert!(ElementsKind::is_smi_elements_kind(from_kind));
            if from_kind == ElementsKind::PACKED_SMI_ELEMENTS {
                ElementsKind::PACKED_ELEMENTS
            } else {
                ElementsKind::HOLEY_ELEMENTS
            }
        }

        pub fn is_simple_map_change_transition(from_kind: ElementsKind, to_kind: ElementsKind) -> bool {
            ElementsKind::get_holey_elements_kind(from_kind) == to_kind ||
                (ElementsKind::is_smi_elements_kind(from_kind) && ElementsKind::is_object_elements_kind(to_kind))
        }

        pub fn is_more_general_elements_kind_transition(from_kind: ElementsKind, to_kind: ElementsKind) -> bool {
            if !ElementsKind::is_fast_elements_kind(from_kind) {
                return false;
            }
            if !ElementsKind::is_fast_transition_target(to_kind) {
                return false;
            }

            match from_kind {
                ElementsKind::PACKED_SMI_ELEMENTS => to_kind != ElementsKind::PACKED_SMI_ELEMENTS,
                ElementsKind::HOLEY_SMI_ELEMENTS => {
                    to_kind != ElementsKind::PACKED_SMI_ELEMENTS && to_kind != ElementsKind::HOLEY_SMI_ELEMENTS
                }
                ElementsKind::PACKED_DOUBLE_ELEMENTS => {
                    to_kind != ElementsKind::PACKED_SMI_ELEMENTS && to_kind != ElementsKind::HOLEY_SMI_ELEMENTS &&
                        to_kind != ElementsKind::PACKED_DOUBLE_ELEMENTS
                }
                ElementsKind::HOLEY_DOUBLE_ELEMENTS => to_kind == ElementsKind::PACKED_ELEMENTS || to_kind == ElementsKind::HOLEY_ELEMENTS,
                ElementsKind::PACKED_ELEMENTS => to_kind == ElementsKind::HOLEY_ELEMENTS,
                ElementsKind::HOLEY_ELEMENTS => false,
                _ => false,
            }
        }

        pub fn get_more_general_elements_kind(from_kind: ElementsKind, to_kind: ElementsKind) -> ElementsKind {
            if ElementsKind::is_more_general_elements_kind_transition(from_kind, to_kind) {
                return to_kind;
            }
            from_kind
        }

        pub fn is_transitionable_fast_elements_kind(from_kind: ElementsKind) -> bool {
            ElementsKind::is_fast_elements_kind(from_kind) &&
                from_kind != ElementsKind::TERMINAL_FAST_ELEMENTS_KIND
        }

        pub fn elements_kind_equal(a: ElementsKind, b: ElementsKind) -> bool {
            a == b
        }

        fn from(value: u8) -> Self {
            match value {
                0 => ElementsKind::PACKED_SMI_ELEMENTS,
                1 => ElementsKind::HOLEY_SMI_ELEMENTS,
                2 => ElementsKind::PACKED_ELEMENTS,
                3 => ElementsKind::HOLEY_ELEMENTS,
                4 => ElementsKind::PACKED_DOUBLE_ELEMENTS,
                5 => ElementsKind::HOLEY_DOUBLE_ELEMENTS,
                6 => ElementsKind::PACKED_NONEXTENSIBLE_ELEMENTS,
                7 => ElementsKind::HOLEY_NONEXTENSIBLE_ELEMENTS,
                8 => ElementsKind::PACKED_SEALED_ELEMENTS,
                9 => ElementsKind::HOLEY_SEALED_ELEMENTS,
                10 => ElementsKind::PACKED_FROZEN_ELEMENTS,
                11 => ElementsKind::HOLEY_FROZEN_ELEMENTS,
                12 => ElementsKind::SHARED_ARRAY_ELEMENTS,
                13 => ElementsKind::DICTIONARY_ELEMENTS,
                14 => ElementsKind::FAST_SLOPPY_ARGUMENTS_ELEMENTS,
                15 => ElementsKind::SLOW_SLOPPY_ARGUMENTS_ELEMENTS,
                16 => ElementsKind::FAST_STRING_WRAPPER_ELEMENTS,
                17 => ElementsKind::SLOW_STRING_WRAPPER_ELEMENTS,
                18 => ElementsKind::UINT8_ELEMENTS,
                19 => ElementsKind::INT8_ELEMENTS,
                20 => ElementsKind::UINT16_ELEMENTS,
                21 => ElementsKind::INT16_ELEMENTS,
                22 => ElementsKind::UINT32_ELEMENTS,
                23 => ElementsKind::INT32_ELEMENTS,
                24 => ElementsKind::BIGUINT64_ELEMENTS,
                25 => ElementsKind::BIGINT64_ELEMENTS,
                26 => ElementsKind::UINT8_CLAMPED_ELEMENTS,
                27
