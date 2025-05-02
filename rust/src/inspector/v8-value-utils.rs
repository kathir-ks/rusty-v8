// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8_value_utils {
    /// Re-export the protocol module for use within this module
    pub use crate::inspector::protocol;

    /// A wrapper around `v8::Maybe<bool>` to represent success or failure.
    pub type V8MaybeBoolResult = Result<bool, ()>;

    /// Creates a data property on a V8 object.
    ///
    /// # Arguments
    ///
    /// * `context` - The V8 context.
    /// * `object` - The V8 object to create the property on.
    /// * `key` - The key of the property.
    /// * `value` - The value of the property.
    pub fn create_data_property(
        context: &v8::Context,
        object: &v8::Object,
        key: &v8::Name,
        value: &v8::Value,
    ) -> V8MaybeBoolResult {
        let result = object.define_property(
            context,
            key.into(),
            value.into(),
            v8::PropertyAttribute::None,
        );

        match result {
            Some(_) => Ok(true),
            None => Err(()),
        }
    }

    /// Creates a data property on a V8 array at a specific index.
    ///
    /// # Arguments
    ///
    /// * `context` - The V8 context.
    /// * `array` - The V8 array to create the property on.
    /// * `index` - The index of the property.
    /// * `value` - The value of the property.
    pub fn create_data_property_index(
        context: &v8::Context,
        array: &v8::Array,
        index: u32,
        value: &v8::Value,
    ) -> V8MaybeBoolResult {
        let result = array.define_element(context, index, value.into());
        match result {
            Some(_) => Ok(true),
            None => Err(()),
        }
    }
}
pub mod inspector {
    pub mod protocol {
        // Placeholder for the protocol module
    }
}
extern crate v8;