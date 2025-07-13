// Converted from V8 C++ source files:
// Header: instance-type-checker.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod instance_type_checker {
    use crate::objects::map::Map;
    use crate::objects::object_macros::*;
    use crate::roots::roots::*;
    use crate::roots::static_roots::*;

    pub struct V8 {}

    macro_rules! instance_type_checkers_single {
        ($V:ident) => {
            // Assuming these macros expand to similar V(...) calls
            instance_type_checkers_single_fully_defined!($V);
            instance_type_checkers_single_only_declared!($V);
            $V!(BigInt, BIGINT_TYPE);
            $V!(FixedArrayExact, FIXED_ARRAY_TYPE);
        };
    }

    macro_rules! instance_type_checkers_range {
        ($V:ident) => {
            // Assuming these macros expand to similar V(...) calls
            instance_type_checkers_range_fully_defined!($V);
            instance_type_checkers_range_only_declared!($V);
            $V!(
                CallableJSFunction,
                FIRST_CALLABLE_JS_FUNCTION_TYPE,
                LAST_CALLABLE_JS_FUNCTION_TYPE
            );
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

    macro_rules! instance_type_checkers_single_fully_defined {
        ($V:ident) => {};
    }

    macro_rules! instance_type_checkers_single_only_declared {
        ($V:ident) => {};
    }

    macro_rules! instance_type_checkers_range_fully_defined {
        ($V:ident) => {};
    }

    macro_rules! instance_type_checkers_range_only_declared {
        ($V:ident) => {};
    }

    pub mod instance_type_checker {
        use super::*;
        use crate::objects::instance_type::InstanceType;
        use crate::roots::roots::RootIndex;
        use crate::roots::static_roots::StaticReadOnlyRoot;
        use crate::roots::static_roots::StaticReadOnlyRootsPointerTable;

        pub trait InstanceTypeCheck {
            fn is_type(&self, instance_type: InstanceType) -> bool;
        }

        macro_rules! is_type_function_decl {
            ($Type:ident, $($rest:tt)*) => {
                pub const fn is_##$Type(instance_type: InstanceType) -> bool {
                    // Provide a default implementation that always returns false
                    false
                }

                pub fn is_##$Type(map: &Map) -> bool {
                    // Provide a default implementation that always returns false
                    false
                }
            };
            ($Type:ident) => {
                pub const fn is_##$Type(instance_type: InstanceType) -> bool {
                    // Provide a default implementation that always returns false
                    false
                }

                pub fn is_##$Type(map: &Map) -> bool {
                    // Provide a default implementation that always returns false
                    false
                }
            };
        }

        instance_type_checkers!(is_type_function_decl);

        #[cfg(v8_static_roots_bool)]
        {
            // Maps for primitive objects and a select few JS objects are allocated in r/o
            // space. All JS_RECEIVER maps must come after primitive object maps, i.e. they
            // have a compressed address above the last primitive object map root. If we
            // have a receiver and need to distinguish whether it is either a primitive
            // object or a JS receiver, it suffices to check if its map is allocated above
            // the following limit address.
            pub const K_NON_JS_RECEIVER_MAP_LIMIT: usize = unsafe {
                StaticReadOnlyRootsPointerTable[RootIndex::KFirstJSReceiverMapRoot as usize]
                    as usize
                    & !0xFFF
            };

            // Maps for strings allocated as the first maps in r/o space, so their lower
            // bound is zero.
            pub const K_STRING_MAP_LOWER_BOUND: usize = 0;
            // If we have a receiver and need to distinguish whether it is a string or not,
            // it suffices to check whether it is less-than-equal to the following value.
            pub const K_STRING_MAP_UPPER_BOUND: StaticReadOnlyRoot =
                StaticReadOnlyRoot::KThinOneByteStringMap;

            macro_rules! assert_is_last_string_map {
                ($instance_type:ident, $size:ident, $name:ident, $Name:ident) => {
                    const _: () = assert!(
                        StaticReadOnlyRoot::KThinOneByteStringMap as usize
                            <= K_STRING_MAP_UPPER_BOUND as usize
                    );
                };
            }

        }

        #[cfg(not(v8_static_roots_bool))]
        {
            pub const K_NON_JS_RECEIVER_MAP_LIMIT: usize = 0x0;
            pub const K_STRING_MAP_LOWER_BOUND: usize = 0;
        }
    }
}
