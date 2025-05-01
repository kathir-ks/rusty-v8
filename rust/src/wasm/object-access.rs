// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use crate::common::globals::*;
//use crate::objects::fixed_array::*;
//use crate::objects::js_function::*;
//use crate::objects::shared_function_info::*;

// Replicate only the necessary parts for compilation.
const K_HEAP_OBJECT_TAG: i32 = 1;

/// Provides static methods for accessing object fields with tagged offsets.
pub struct ObjectAccess;

impl ObjectAccess {
    /// Converts an offset into an object to an offset into a tagged object.
    pub const fn to_tagged(offset: i32) -> i32 {
        offset - K_HEAP_OBJECT_TAG
    }

    /// Gets the offset into a fixed array for a given {index}.
    pub const fn element_offset_in_tagged_fixed_array(index: i32) -> i32 {
        Self::to_tagged(fixed_array_offset_of_element_at(index))
    }

    /// Gets the offset into a fixed uint8 array for a given {index}.
    pub const fn element_offset_in_tagged_fixed_uint8_array(index: i32) -> i32 {
        Self::to_tagged(fixed_uint8_array_offset_of_element_at(index))
    }

    /// Gets the offset into a fixed uint32 array for a given {index}.
    pub const fn element_offset_in_tagged_fixed_uint32_array(index: i32) -> i32 {
        Self::to_tagged(fixed_uint32_array_offset_of_element_at(index))
    }

    /// Gets the offset into a fixed address array for a given {index}.
    pub const fn element_offset_in_tagged_fixed_address_array(index: i32) -> i32 {
        Self::to_tagged(fixed_address_array_offset_of_element_at(index))
    }

    /// Gets the offset into a trusted fixed address array for a given {index}.
    pub const fn element_offset_in_tagged_trusted_fixed_address_array(index: i32) -> i32 {
        Self::to_tagged(trusted_fixed_address_array_offset_of_element_at(index))
    }

    /// Gets the offset into a ProtectedFixedArray for a given {index}.
    pub const fn element_offset_in_protected_fixed_array(index: i32) -> i32 {
        Self::to_tagged(protected_fixed_array_offset_of_element_at(index))
    }

    /// Gets the offset of the context stored in a {JSFunction} object.
    pub const fn context_offset_in_tagged_js_function() -> i32 {
        Self::to_tagged(js_function_k_context_offset())
    }

    /// Gets the offset of the shared function info in a {JSFunction} object.
    pub const fn shared_function_info_offset_in_tagged_js_function() -> i32 {
        Self::to_tagged(js_function_k_shared_function_info_offset())
    }

    /// Gets the offset of the flags in a {SharedFunctionInfo} object.
    pub const fn flags_offset_in_shared_function_info() -> i32 {
        Self::to_tagged(shared_function_info_k_flags_offset())
    }
}

// Dummy functions for compilation, replace with actual implementations.
const fn fixed_array_offset_of_element_at(_index: i32) -> i32 {
    0
}

const fn fixed_uint8_array_offset_of_element_at(_index: i32) -> i32 {
    0
}

const fn fixed_uint32_array_offset_of_element_at(_index: i32) -> i32 {
    0
}

const fn fixed_address_array_offset_of_element_at(_index: i32) -> i32 {
    0
}

const fn trusted_fixed_address_array_offset_of_element_at(_index: i32) -> i32 {
    0
}

const fn protected_fixed_array_offset_of_element_at(_index: i32) -> i32 {
    0
}

const fn js_function_k_context_offset() -> i32 {
    0
}

const fn js_function_k_shared_function_info_offset() -> i32 {
    0
}

const fn shared_function_info_k_flags_offset() -> i32 {
    0
}