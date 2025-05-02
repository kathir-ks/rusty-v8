// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod heap_object_inl {
    use crate::common::ptr_compr::PtrComprCageBase;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::instance_type::InstanceTypeChecker;
    use crate::objects::map::Map;
    use crate::objects::tagged::Tagged;

    macro_rules! instance_type_checkers {
        ($checker:ident) => {
            $checker!(JS_PROXY);
            $checker!(JS_OBJECT);
            $checker!(JS_CONTEXT);
            $checker!(FIXED_ARRAY);
            $checker!(STRING);
            $checker!(SYMBOL);
            $checker!(BIGINT);
            $checker!(JS_FUNCTION);
            $checker!(CODE);
            $checker!(BYTE_ARRAY);
            $checker!(DESCRIPTOR_ARRAY);
            $checker!(FREE_SPACE);
            $checker!(MAP);
            $checker!(WEAK_MAP);
            $checker!(WEAK_SET);
            $checker!(JS_PRIMITIVE_WRAPPER);
            $checker!(JS_ARRAY);
            $checker!(JS_TYPED_ARRAY);
            $checker!(JS_DATA_VIEW);
        };
    }

    macro_rules! type_checker {
        ($type:ident) => {
            #[allow(dead_code)]
            pub fn is_$type(obj: Tagged<HeapObject>) -> bool {
                // IsBlah() predicates needs to load the map and thus they require the
                // main cage base.
                let cage_base = PtrComprCageBase::get_ptr_compr_cage_base();
                is_$type(obj, cage_base)
            }

            // The cage_base passed here must be the base of the main pointer
            // compression cage, i.e. the one where the Map space is allocated.
            #[allow(dead_code)]
            pub fn is_$type(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
                let map_object = obj.map(cage_base);
                InstanceTypeChecker::is_$type(map_object)
            }
        };
    }

    instance_type_checkers!(type_checker);

    // These macros can't be translated without knowing the actual definitions
    // of the INSTANCE_TYPE_CHECKERS and TYPE_CHECKER
    // The implementation is based on the existing C++ code structure.
}