// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod instance_type_checker {
    use crate::objects::instance_type::InstanceType;
    // use crate::roots::roots::RootIndex; // TODO: Implement roots
    // use crate::roots::static_roots::StaticReadOnlyRoot; // TODO: Implement static roots
    // use crate::roots::static_roots::StaticReadOnlyRootsPointerTable; // TODO: Implement static roots
    // use crate::objects::object_macros; // TODO: Implement object macros

    //Placeholder type
    pub struct Map {}

    macro_rules! instance_type_checkers_single {
        ($V:ident) => {
            // TORQUE_INSTANCE_CHECKERS_SINGLE_FULLY_DEFINED!($V); // TODO: Torque
            // TORQUE_INSTANCE_CHECKERS_SINGLE_ONLY_DECLARED!($V); // TODO: Torque
            $V!(BigInt, InstanceType::BigInt);
            $V!(FixedArrayExact, InstanceType::FixedArray);
        };
    }

    macro_rules! instance_type_checkers_range {
        ($V:ident) => {
            // TORQUE_INSTANCE_CHECKERS_RANGE_FULLY_DEFINED!($V); // TODO: Torque
            // TORQUE_INSTANCE_CHECKERS_RANGE_ONLY_DECLARED!($V); // TODO: Torque
            $V!(CallableJSFunction, InstanceType::CallableJSFunction, InstanceType::LastCallableJSFunction);
        };
    }

    macro_rules! instance_type_checkers_custom {
        ($V:ident) => {
            $V!(AbstractCode);
            $V!(ExternalString);
            $V!(FreeSpaceOrFiller);
            $V!(GcSafeCode);
            $V!(InternalizedString);
            $V!(MaybeReadOnlyJSObject);
            $V!(PropertyDictionary);
        };
    }

    macro_rules! instance_type_checkers {
        ($V:ident) => {
            instance_type_checkers_single!($V);
            instance_type_checkers_range!($V);
            instance_type_checkers_custom!($V);
        };
    }

    pub mod instance_type_checker {
        use super::*;
        use crate::objects::instance_type::InstanceType;

        macro_rules! is_type_function_decl {
            ($Type:ident, $($rest:tt)*) => {
                #[inline]
                pub const fn is_$Type(instance_type: InstanceType) -> bool;
                #[inline]
                pub fn is_$Type(map: &Map) -> bool;
            };
        }

        instance_type_checkers!(is_type_function_decl);

        // Cleanup macro
        macro_rules! undef_is_type_function_decl {
            () => {};
        }

        undef_is_type_function_decl!();


        // #[cfg(V8_STATIC_ROOTS_BOOL)]
        // const K_NON_JS_RECEIVER_MAP_LIMIT: Tagged_t =
        //     StaticReadOnlyRootsPointerTable[RootIndex::KFirstJSReceiverMapRoot as usize] & !0xFFF;

        // #[cfg(not(V8_STATIC_ROOTS_BOOL))]
        pub const K_NON_JS_RECEIVER_MAP_LIMIT: usize = 0x0;

        // #[cfg(V8_STATIC_ROOTS_BOOL)]
        // pub const K_STRING_MAP_LOWER_BOUND: Tagged_t = 0;
        // #[cfg(V8_STATIC_ROOTS_BOOL)]
        // pub const K_STRING_MAP_UPPER_BOUND: Tagged_t = StaticReadOnlyRoot::KThinOneByteStringMap;

        //TODO: implement string types
        // macro_rules! assert_is_last_string_map {
        //   ($instance_type:ident, $size:ident, $name:ident, $Name:ident) => {
        //     const _: () = assert!(StaticReadOnlyRoot::K##Name##Map <= K_STRING_MAP_UPPER_BOUND);
        //   };
        // }

        // STRING_TYPE_LIST!(assert_is_last_string_map);

        // macro_rules! undef_assert_is_last_string_map {
        //   () => {};
        // }
        // undef_assert_is_last_string_map!();
    }
}