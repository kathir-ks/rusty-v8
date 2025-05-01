// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is a placeholder for the actual V8 data structures and functionalities.
// Many of the types and methods used here are simplified or stubbed out for
// illustrative purposes.

// Replicates some bits from src/builtins/builtins.h
mod builtins {
    pub struct Code {}
}

// Replicates some bits from src/execution/isolate.h
mod execution {
    pub struct Isolate {}
}

// Replicates some bits from src/handles/handles-inl.h
mod handles {
    pub struct Handle<T>(T);
    pub struct DirectHandle<T>(T);

    impl<T> Handle<T> {
        pub fn new(value: T) -> Self {
            Handle(value)
        }
    }
    impl<T> DirectHandle<T> {
        pub fn new(value: T) -> Self {
            DirectHandle(value)
        }
    }
}

// Replicates some bits from src/ic/handler-configuration.h
mod ic {
    pub mod handler_configuration {
        pub enum Kind {
            kNormal,
            kGlobal,
            kInterceptor,
            kSlow,
            kField,
            kConstantFromPrototype,
            kAccessorFromPrototype,
            kProxy,
            kNativeDataProperty,
            kApiGetter,
            kModuleExport,
            kNonExistent,
            kElement,
            kIndexedString,
            kGlobalProxy,
            kApiSetter,
            kConstField,
            kSharedStructField,
        }
    }
}

// Replicates some bits from src/objects/data-handler-inl.h
mod objects {
    pub mod data_handler {
        pub struct DataHandler {}
    }
    pub mod field_index_inl {
        pub struct FieldIndex {
            index: i32,
            is_inobject: bool,
            is_double: bool,
        }

        impl FieldIndex {
            pub fn new(index: i32, is_inobject: bool, is_double: bool) -> Self {
                FieldIndex {
                    index,
                    is_inobject,
                    is_double,
                }
            }

            pub fn index(&self) -> i32 {
                self.index
            }

            pub fn is_inobject(&self) -> bool {
                self.is_inobject
            }

            pub fn is_double(&self) -> bool {
                self.is_double
            }
        }
    }
    pub mod objects_inl {}
    pub mod smi {
        #[derive(Clone, Copy)]
        pub struct Smi(i32);

        impl Smi {
            pub fn from_int(value: i32) -> Self {
                Smi(value)
            }

            pub fn value(&self) -> i32 {
                self.0
            }
        }
    }
}

use crate::builtins::*;
use crate::execution::*;
use crate::handles::*;
use crate::ic::handler_configuration::Kind;
use crate::objects::data_handler::DataHandler;
use crate::objects::field_index_inl::FieldIndex;
use crate::objects::smi::Smi;

// Defines macros that manipulate an object with its size and fields.
// #[macro_export]
// macro_rules! OBJECT_CONSTRUCTORS_IMPL {
//     ($name:ident, $base:ident) => {
//         impl $name {
//             // Constructor-like method (Rust doesn't have constructors)
//             pub fn new() -> Self {
//                 Self {} // Initialization logic here. Replace {} with appropriate initialization.
//             }
//         }
//     };
// }

mod load_handler {
    use super::*;

    // Bitfield encoding constants
    const KIND_BITS_SHIFT: i32 = 0;
    const KIND_BITS_MASK: i32 = 0xF; // Example mask, adjust as needed

    const IS_INOBJECT_BITS_SHIFT: i32 = 4;
    const IS_INOBJECT_BITS_MASK: i32 = 0x1 << IS_INOBJECT_BITS_SHIFT;

    const IS_DOUBLE_BITS_SHIFT: i32 = 5;
    const IS_DOUBLE_BITS_MASK: i32 = 0x1 << IS_DOUBLE_BITS_SHIFT;

    const FIELD_INDEX_BITS_SHIFT: i32 = 6;
    const FIELD_INDEX_BITS_MASK: i32 = 0x3FF << FIELD_INDEX_BITS_SHIFT; // Example, adjust

    const IS_WASM_STRUCT_BITS_SHIFT: i32 = 16;
    const IS_WASM_STRUCT_BITS_MASK: i32 = 0x1 << IS_WASM_STRUCT_BITS_SHIFT;

    const WASM_FIELD_TYPE_BITS_SHIFT: i32 = 17;
    const WASM_FIELD_TYPE_BITS_MASK: i32 = 0x7 << WASM_FIELD_TYPE_BITS_SHIFT; // Example

    const WASM_FIELD_OFFSET_BITS_SHIFT: i32 = 20;
    const WASM_FIELD_OFFSET_BITS_MASK: i32 = 0xFFF << WASM_FIELD_OFFSET_BITS_SHIFT; // Example

    const DESCRIPTOR_BITS_SHIFT: i32 = 4;
    const DESCRIPTOR_BITS_MASK: i32 = 0xFFF << DESCRIPTOR_BITS_SHIFT;

    const EXPORTS_INDEX_BITS_SHIFT: i32 = 4;
    const EXPORTS_INDEX_BITS_MASK: i32 = 0xFFF << EXPORTS_INDEX_BITS_SHIFT;

    const ALLOW_OUT_OF_BOUNDS_BITS_SHIFT: i32 = 4;
    const ALLOW_OUT_OF_BOUNDS_BITS_MASK: i32 = 0x1 << ALLOW_OUT_OF_BOUNDS_BITS_SHIFT;

    const ELEMENTS_KIND_BITS_SHIFT: i32 = 5;
    const ELEMENTS_KIND_BITS_MASK: i32 = 0x7 << ELEMENTS_KIND_BITS_SHIFT;

    const ALLOW_HANDLING_HOLE_BITS_SHIFT: i32 = 8;
    const ALLOW_HANDLING_HOLE_BITS_MASK: i32 = 0x1 << ALLOW_HANDLING_HOLE_BITS_SHIFT;

    const IS_JS_ARRAY_BITS_SHIFT: i32 = 9;
    const IS_JS_ARRAY_BITS_MASK: i32 = 0x1 << IS_JS_ARRAY_BITS_SHIFT;

    const IS_WASM_ARRAY_BITS_SHIFT: i32 = 4;
    const IS_WASM_ARRAY_BITS_MASK: i32 = 0x1 << IS_WASM_ARRAY_BITS_SHIFT;

    const WASM_ARRAY_TYPE_BITS_SHIFT: i32 = 5;
    const WASM_ARRAY_TYPE_BITS_MASK: i32 = 0x7 << WASM_ARRAY_TYPE_BITS_SHIFT;

    pub struct LoadHandler {}

    impl LoadHandler {
        // Bitfield encoding utilities
        fn encode(value: i32, shift: i32, mask: i32) -> i32 {
            (value << shift) & mask
        }

        fn decode(encoded_value: i32, shift: i32, mask: i32) -> i32 {
            (encoded_value & mask) >> shift
        }

        mod kind_bits {
            use super::*;

            pub fn encode(kind: Kind) -> i32 {
                match kind {
                    Kind::kNormal => LoadHandler::encode(0, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kGlobal => LoadHandler::encode(1, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kInterceptor => LoadHandler::encode(2, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kSlow => LoadHandler::encode(3, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kField => LoadHandler::encode(4, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kConstantFromPrototype => {
                        LoadHandler::encode(5, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kAccessorFromPrototype => {
                        LoadHandler::encode(6, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kProxy => LoadHandler::encode(7, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kNativeDataProperty => {
                        LoadHandler::encode(8, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kApiGetter => LoadHandler::encode(9, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kModuleExport => {
                        LoadHandler::encode(10, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kNonExistent => {
                        LoadHandler::encode(11, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kElement => LoadHandler::encode(12, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kIndexedString => {
                        LoadHandler::encode(13, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    _ => 0, //Placeholder for other kinds. Should not happen.
                }
            }

            pub fn decode(smi_value: i32) -> Kind {
                match LoadHandler::decode(smi_value, KIND_BITS_SHIFT, KIND_BITS_MASK) {
                    0 => Kind::kNormal,
                    1 => Kind::kGlobal,
                    2 => Kind::kInterceptor,
                    3 => Kind::kSlow,
                    4 => Kind::kField,
                    5 => Kind::kConstantFromPrototype,
                    6 => Kind::kAccessorFromPrototype,
                    7 => Kind::kProxy,
                    8 => Kind::kNativeDataProperty,
                    9 => Kind::kApiGetter,
                    10 => Kind::kModuleExport,
                    11 => Kind::kNonExistent,
                    12 => Kind::kElement,
                    13 => Kind::kIndexedString,
                    _ => Kind::kNormal, // Default, or handle error
                }
            }
        }

        mod is_inobject_bits {
            use super::*;

            pub fn encode(is_inobject: bool) -> i32 {
                if is_inobject {
                    LoadHandler::encode(1, IS_INOBJECT_BITS_SHIFT, IS_INOBJECT_BITS_MASK)
                } else {
                    0
                }
            }
        }

        mod is_double_bits {
            use super::*;

            pub fn encode(is_double: bool) -> i32 {
                if is_double {
                    LoadHandler::encode(1, IS_DOUBLE_BITS_SHIFT, IS_DOUBLE_BITS_MASK)
                } else {
                    0
                }
            }
        }

        mod field_index_bits {
            use super::*;

            pub fn encode(index: i32) -> i32 {
                LoadHandler::encode(index, FIELD_INDEX_BITS_SHIFT, FIELD_INDEX_BITS_MASK)
            }
        }

        mod is_wasm_struct_bits {
            use super::*;

            pub fn encode(is_wasm_struct: bool) -> i32 {
                if is_wasm_struct {
                    LoadHandler::encode(1, IS_WASM_STRUCT_BITS_SHIFT, IS_WASM_STRUCT_BITS_MASK)
                } else {
                    0
                }
            }
        }

        mod wasm_field_type_bits {
            use super::*;

            pub fn encode(wasm_value_type: WasmValueType) -> i32 {
                match wasm_value_type {
                    WasmValueType::kI8 => LoadHandler::encode(0, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kI16 => LoadHandler::encode(1, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kI32 => LoadHandler::encode(2, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kU32 => LoadHandler::encode(3, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kI64 => LoadHandler::encode(4, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kF32 => LoadHandler::encode(5, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kF64 => LoadHandler::encode(6, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kS128 => LoadHandler::encode(7, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kRef => LoadHandler::encode(8, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    WasmValueType::kRefNull => LoadHandler::encode(9, WASM_FIELD_TYPE_BITS_SHIFT, WASM_FIELD_TYPE_BITS_MASK),
                    _ => 0, // Placeholder for other value types, handle properly
                }
            }
        }

        mod wasm_field_offset_bits {
            use super::*;

            pub fn encode(offset: i32) -> i32 {
                LoadHandler::encode(offset, WASM_FIELD_OFFSET_BITS_SHIFT, WASM_FIELD_OFFSET_BITS_MASK)
            }
        }

        mod descriptor_bits {
            use super::*;

            pub fn encode(descriptor: i32) -> i32 {
                LoadHandler::encode(descriptor, DESCRIPTOR_BITS_SHIFT, DESCRIPTOR_BITS_MASK)
            }
        }

        mod exports_index_bits {
            use super::*;

            pub fn encode(index: i32) -> i32 {
                LoadHandler::encode(index, EXPORTS_INDEX_BITS_SHIFT, EXPORTS_INDEX_BITS_MASK)
            }
        }

        mod allow_out_of_bounds_bits {
            use super::*;

            pub fn encode(allow_oob: bool) -> i32 {
                if allow_oob {
                    LoadHandler::encode(1, ALLOW_OUT_OF_BOUNDS_BITS_SHIFT, ALLOW_OUT_OF_BOUNDS_BITS_MASK)
                } else {
                    0
                }
            }
        }

        mod elements_kind_bits {
            use super::*;

            pub fn encode(elements_kind: ElementsKind) -> i32 {
                 match elements_kind {
                    ElementsKind::PackedSmiElements => LoadHandler::encode(0, ELEMENTS_KIND_BITS_SHIFT, ELEMENTS_KIND_BITS_MASK),
                    ElementsKind::HoleySmiElements => LoadHandler::encode(1, ELEMENTS_KIND_BITS_SHIFT, ELEMENTS_KIND_BITS_MASK),
                    ElementsKind::PackedDoubleElements => LoadHandler::encode(2, ELEMENTS_KIND_BITS_SHIFT, ELEMENTS_KIND_BITS_MASK),
                    ElementsKind::HoleyDoubleElements => LoadHandler::encode(3, ELEMENTS_KIND_BITS_SHIFT, ELEMENTS_KIND_BITS_MASK),
                    ElementsKind::PackedElement => LoadHandler::encode(4, ELEMENTS_KIND_BITS_SHIFT, ELEMENTS_KIND_BITS_MASK),
                    ElementsKind::HoleyElement => LoadHandler::encode(5, ELEMENTS_KIND_BITS_SHIFT, ELEMENTS_KIND_BITS_MASK),
                    _ => 0, // Handle all ElementsKind variants appropriately
                }
            }
        }

        mod allow_handling_hole {
            use super::*;

            pub fn encode(allow_hole: bool) -> i32 {
                 if allow_hole {
                    LoadHandler::encode(1, ALLOW_HANDLING_HOLE_BITS_SHIFT, ALLOW_HANDLING_HOLE_BITS_MASK)
                } else {
                    0
                }
            }
        }

        mod is_js_array_bits {
            use super::*;

            pub fn encode(is_js_array: bool) -> i32 {
                if is_js_array {
                    LoadHandler::encode(1, IS_JS_ARRAY_BITS_SHIFT, IS_JS_ARRAY_BITS_MASK)
                } else {
                    0
                }
            }
        }

        mod is_wasm_array_bits {
            use super::*;

            pub fn encode(is_wasm_array: bool) -> i32 {
                if is_wasm_array {
                    LoadHandler::encode(1, IS_WASM_ARRAY_BITS_SHIFT, IS_WASM_ARRAY_BITS_MASK)
                } else {
                    0
                }
            }
        }

        mod wasm_array_type_bits {
            use super::*;

            pub fn encode(wasm_value_type: WasmValueType) -> i32 {
                match wasm_value_type {
                    WasmValueType::kI8 => LoadHandler::encode(0, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kI16 => LoadHandler::encode(1, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kI32 => LoadHandler::encode(2, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kU32 => LoadHandler::encode(3, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kI64 => LoadHandler::encode(4, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kF32 => LoadHandler::encode(5, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kF64 => LoadHandler::encode(6, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kS128 => LoadHandler::encode(7, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kRef => LoadHandler::encode(8, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    WasmValueType::kRefNull => LoadHandler::encode(9, WASM_ARRAY_TYPE_BITS_SHIFT, WASM_ARRAY_TYPE_BITS_MASK),
                    _ => 0, // Placeholder for other value types, handle properly
                }
            }
        }
        /// Decodes kind from Smi-handler.
        pub fn get_handler_kind(smi_handler: Smi) -> Kind {
            kind_bits::decode(smi_handler.value())
        }

        pub fn load_normal(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kNormal);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_global(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kGlobal);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_interceptor(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kInterceptor);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_slow(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kSlow);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_field(isolate: &Isolate, field_index: FieldIndex) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kField)
                | is_inobject_bits::encode(field_index.is_inobject())
                | is_double_bits::encode(field_index.is_double())
                | field_index_bits::encode(field_index.index());
            Handle::new(Smi::from_int(config))
        }

        pub fn load_wasm_struct_field(
            isolate: &Isolate,
            value_type: WasmValueType,
            offset: i32,
        ) -> DirectHandle<Smi> {
            let config = kind_bits::encode(Kind::kField)
                | is_wasm_struct_bits::encode(true)
                | wasm_field_type_bits::encode(value_type)
                | wasm_field_offset_bits::encode(offset);
            DirectHandle::new(Smi::from_int(config))
        }

        pub fn load_constant_from_prototype(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kConstantFromPrototype);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_accessor_from_prototype(isolate: &Isolate) -> DirectHandle<Smi> {
            let config = kind_bits::encode(Kind::kAccessorFromPrototype);
            DirectHandle::new(Smi::from_int(config))
        }

        pub fn load_proxy(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kProxy);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_native_data_property(isolate: &Isolate, descriptor: i32) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kNativeDataProperty) | descriptor_bits::encode(descriptor);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_api_getter(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kApiGetter);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_module_export(isolate: &Isolate, index: i32) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kModuleExport) | exports_index_bits::encode(index);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_non_existent(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kNonExistent);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_element(
            isolate: &Isolate,
            elements_kind: ElementsKind,
            is_js_array: bool,
            load_mode: KeyedAccessLoadMode,
        ) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kElement)
                | allow_out_of_bounds_bits::encode(load_mode.handles_oob())
                | elements_kind_bits::encode(elements_kind)
                | allow_handling_hole::encode(load_mode.handles_holes())
                | is_js_array_bits::encode(is_js_array);
            Handle::new(Smi::from_int(config))
        }

        pub fn load_indexed_string(isolate: &Isolate, load_mode: KeyedAccessLoadMode) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kIndexedString) | allow_out_of_bounds_bits::encode(load_mode.handles_oob());
            Handle::new(Smi::from_int(config))
        }

        pub fn load_wasm_array_element(
            isolate: &Isolate,
            value_type: WasmValueType,
        ) -> DirectHandle<Smi> {
            let config = kind_bits::encode(Kind::kElement)
                | is_wasm_array_bits::encode(true)
                | wasm_array_type_bits::encode(value_type);
            DirectHandle::new(Smi::from_int(config))
        }
    }
}

mod store_handler {
    use super::*;
    use KeyedAccessStoreMode::*;

    // Bitfield encoding constants - Replicated from LoadHandler, adjust if needed
    const KIND_BITS_SHIFT: i32 = 0;
    const KIND_BITS_MASK: i32 = 0xF; // Example mask, adjust as needed

    const DESCRIPTOR_BITS_SHIFT: i32 = 4;
    const DESCRIPTOR_BITS_MASK: i32 = 0xFFF << DESCRIPTOR_BITS_SHIFT;

    const IS_INOBJECT_BITS_SHIFT: i32 = 16;
    const IS_INOBJECT_BITS_MASK: i32 = 0x1 << IS_INOBJECT_BITS_SHIFT;

    const REPRESENTATION_BITS_SHIFT: i32 = 17;
    const REPRESENTATION_BITS_MASK: i32 = 0x7 << REPRESENTATION_BITS_SHIFT; // Example

    const FIELD_INDEX_BITS_SHIFT: i32 = 20;
    const FIELD_INDEX_BITS_MASK: i32 = 0xFFF << FIELD_INDEX_BITS_SHIFT; // Example

    const KEYED_ACCESS_STORE_MODE_BITS_SHIFT: i32 = 4;
    const KEYED_ACCESS_STORE_MODE_BITS_MASK: i32 = 0x3 << KEYED_ACCESS_STORE_MODE_BITS_SHIFT; // Example

    pub struct StoreHandler {}

    impl StoreHandler {
        // Bitfield encoding utilities - Replicated from LoadHandler, adjust if needed
        fn encode(value: i32, shift: i32, mask: i32) -> i32 {
            (value << shift) & mask
        }

        mod kind_bits {
            use super::*;

            pub fn encode(kind: Kind) -> i32 {
                match kind {
                    Kind::kNormal => StoreHandler::encode(0, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kGlobal => StoreHandler::encode(1, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kInterceptor => StoreHandler::encode(2, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kSlow => StoreHandler::encode(3, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kField => StoreHandler::encode(4, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kConstantFromPrototype => {
                        StoreHandler::encode(5, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kAccessorFromPrototype => {
                        StoreHandler::encode(6, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kProxy => StoreHandler::encode(7, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kNativeDataProperty => {
                        StoreHandler::encode(8, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kApiGetter => StoreHandler::encode(9, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kModuleExport => {
                        StoreHandler::encode(10, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kNonExistent => {
                        StoreHandler::encode(11, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kElement => StoreHandler::encode(12, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kIndexedString => {
                        StoreHandler::encode(13, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kGlobalProxy => {
                        StoreHandler::encode(14, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    Kind::kApiSetter => StoreHandler::encode(15, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kConstField => StoreHandler::encode(16, KIND_BITS_SHIFT, KIND_BITS_MASK),
                    Kind::kSharedStructField => {
                        StoreHandler::encode(17, KIND_BITS_SHIFT, KIND_BITS_MASK)
                    }
                    _ => 0, //Placeholder for other kinds. Should not happen.
                }
            }
        }

        mod descriptor_bits {
            use super::*;

            pub fn encode(descriptor: i32) -> i32 {
                StoreHandler::encode(descriptor, DESCRIPTOR_BITS_SHIFT, DESCRIPTOR_BITS_MASK)
            }
        }

        mod is_inobject_bits {
            use super::*;

            pub fn encode(is_inobject: bool) -> i32 {
                if is_inobject {
                    StoreHandler::encode(1, IS_INOBJECT_BITS_SHIFT, IS_INOBJECT_BITS_MASK)
                } else {
                    0
                }
            }
        }

        mod representation_bits {
            use super::*;

            pub fn encode(representation_kind: RepresentationKind) -> i32 {
                match representation_kind {
                    RepresentationKind::None => StoreHandler::encode(0, REPRESENTATION_BITS_SHIFT, REPRESENTATION_BITS_MASK),
                    RepresentationKind::Smi => StoreHandler::encode(1, REPRESENTATION_BITS_SHIFT, REPRESENTATION_BITS_MASK),
                    RepresentationKind::Double => StoreHandler::encode(2, REPRESENTATION_BITS_SHIFT, REPRESENTATION_BITS_MASK),
                    RepresentationKind::HeapObject => StoreHandler::encode(3, REPRESENTATION_BITS_SHIFT, REPRESENTATION_BITS_MASK),
                    RepresentationKind::TaggedSigned => StoreHandler::encode(4, REPRESENTATION_BITS_SHIFT, REPRESENTATION_BITS_MASK),
                    RepresentationKind::Tagged => StoreHandler::encode(5, REPRESENTATION_BITS_SHIFT, REPRESENTATION_BITS_MASK),
                    RepresentationKind::UnsignedSmall => StoreHandler::encode(6, REPRESENTATION_BITS_SHIFT, REPRESENTATION_BITS_MASK),
                }
            }
        }

        mod field_index_bits {
            use super::*;

            pub fn encode(index: i32) -> i32 {
                StoreHandler::encode(index, FIELD_INDEX_BITS_SHIFT, FIELD_INDEX_BITS_MASK)
            }
        }

        mod keyed_access_store_mode_bits {
            use super::*;

            pub fn encode(store_mode: KeyedAccessStoreMode) -> i32 {
                match store_mode {
                    KeyedAccessStoreMode::kInBounds => StoreHandler::encode(0, KEYED_ACCESS_STORE_MODE_BITS_SHIFT, KEYED_ACCESS_STORE_MODE_BITS_MASK),
                    KeyedAccessStoreMode::kGrowAndHandleCOW => StoreHandler::encode(1, KEYED_ACCESS_STORE_MODE_BITS_SHIFT, KEYED_ACCESS_STORE_MODE_BITS_MASK),
                    KeyedAccessStoreMode::kIgnoreTypedArrayOOB => StoreHandler::encode(2, KEYED_ACCESS_STORE_MODE_BITS_SHIFT, KEYED_ACCESS_STORE_MODE_BITS_MASK),
                    KeyedAccessStoreMode::kHandleCOW => StoreHandler::encode(3, KEYED_ACCESS_STORE_MODE_BITS_SHIFT, KEYED_ACCESS_STORE_MODE_BITS_MASK),
                }
            }
        }

        pub fn store_global_proxy(isolate: &Isolate) -> DirectHandle<Smi> {
            let config = kind_bits::encode(Kind::kGlobalProxy);
            DirectHandle::new(Smi::from_int(config))
        }

        pub fn store_normal(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kNormal);
            Handle::new(Smi::from_int(config))
        }

        pub fn store_interceptor(isolate: &Isolate) -> Handle<Smi> {
            let config = kind_bits::encode(Kind::kInterceptor);
            Handle::new(Smi::from_int(config))
        }

        pub fn store_sloppy_arguments_builtin(
            isolate: &Isolate,
            mode: KeyedAccessStoreMode,
        ) -> Handle<Code> {
            match mode {
                KeyedAccessStoreMode::kInBounds => {
                    Handle::new(BUILTIN_CODE(isolate, KeyedStoreIC_SloppyArguments_InBounds))
                }
                KeyedAccessStoreMode::kGrowAndHandleCOW => Handle::new(BUILTIN_CODE(
                    isolate,
                    KeyedStoreIC_SloppyArguments_NoTransitionGrowAndHandleCOW,
                )),
                KeyedAccessStoreMode::kIgnoreTypedArrayOOB => Handle::new(BUILTIN_CODE(
                    isolate,
                    KeyedStoreIC_SloppyArguments_NoTransitionIgnoreTypedArrayOOB,
                )),
                KeyedAccessStoreMode::kHandleCOW => Handle::new(BUILTIN_CODE(
                    isolate,
                    KeyedStoreIC_SloppyArguments_NoTransitionHandleCOW,
                )),
            }
        }

        pub fn store_fast_element_builtin(isolate: &Isolate, mode: KeyedAccessStoreMode) -> Handle<Code> {
            match mode {
                KeyedAccessStoreMode::kInBounds => {
                    Handle::new(BUILTIN_CODE(isolate, StoreFastElementIC_InBounds))
                }
                KeyedAccessStoreMode::kGrowAndHandleCOW => {
                    Handle::new(BUILTIN_CODE(isolate, StoreFastElementIC_NoTransitionGrowAndHandleCOW))
                }
                KeyedAccessStoreMode::kIgnoreTypedArrayOOB => Handle::new(BUILTIN