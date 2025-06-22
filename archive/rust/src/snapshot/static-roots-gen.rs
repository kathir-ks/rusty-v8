// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/snapshot/static-roots-gen.h equivalent
pub mod static_roots_gen {
    use std::collections::{HashMap, BTreeMap};
    use std::fs::File;
    use std::io::{Write, Result};
    use std::f64;

    // Mocked types/constants/functions for V8 compatibility
    pub type Tagged_t = usize; // Adjust the size accordingly based on the target architecture
    pub type RootIndex = usize; // Replace with a more specific enum if needed

    const V8_STATIC_ROOTS_BOOL: bool = false; // Replace with actual condition if needed
    const V8_STATIC_ROOTS_GENERATION_BOOL: bool = true; // Replace with actual condition if needed
    const V8_ENABLE_WEBASSEMBLY: bool = true; // Replace with actual condition if needed
    const V8_INTL_SUPPORT: bool = true; // Replace with actual condition if needed
    const K_REGULAR_PAGE_SIZE: usize = 65536; //Example value; needs to be the real value

    pub struct Isolate {}

    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }

    pub struct ReadOnlyRoots<'a> {
        isolate: &'a Isolate,
    }

    impl<'a> ReadOnlyRoots<'a> {
        pub fn new(isolate: &'a Isolate) -> Self {
            ReadOnlyRoots { isolate }
        }
        fn unchecked_null(&self) -> FakeRootObject {
            FakeRootObject { ptr: 0 }
        }
        fn unchecked_the_hole(&self) -> FakeRootObject {
            FakeRootObject { ptr: 1 }
        }

        fn unchecked_undefined_value(&self) -> FakeRootObject {
            FakeRootObject {ptr: 2}
        }

        fn unchecked_uninitialized_symbol(&self) -> FakeRootObject {
            FakeRootObject {ptr: 3}
        }
        fn unchecked_empty_string(&self) -> FakeRootObject {
            FakeRootObject{ptr: 4}
        }
        fn unchecked_single_character_string(&self) -> FakeRootObject {
            FakeRootObject{ptr: 5}
        }

        fn unchecked_one_byte_string_table(&self) -> FakeRootObject {
            FakeRootObject{ptr: 6}
        }

        fn unchecked_number_string_cache(&self) -> FakeRootObject {
            FakeRootObject{ptr: 7}
        }

        fn unchecked_empty_fixed_array(&self) -> FakeRootObject {
            FakeRootObject{ptr: 8}
        }
        fn unchecked_empty_weak_fixed_array(&self) -> FakeRootObject {
            FakeRootObject{ptr: 9}
        }

        fn unchecked_empty_descriptor_array(&self) -> FakeRootObject {
            FakeRootObject{ptr: 10}
        }

        fn unchecked_empty_ordered_hash_table(&self) -> FakeRootObject {
            FakeRootObject{ptr: 11}
        }

        fn unchecked_empty_fixed_double_array(&self) -> FakeRootObject {
            FakeRootObject{ptr: 12}
        }

        fn unchecked_empty_script_list(&self) -> FakeRootObject {
            FakeRootObject{ptr: 13}
        }

        fn unchecked_empty_array(&self) -> FakeRootObject {
            FakeRootObject{ptr: 14}
        }

        fn unchecked_empty_context(&self) -> FakeRootObject {
            FakeRootObject{ptr: 15}
        }

        fn unchecked_native_context_constructor(&self) -> FakeRootObject {
            FakeRootObject{ptr: 16}
        }

        fn unchecked_allocation_site_without_feedback(&self) -> FakeRootObject {
            FakeRootObject{ptr: 17}
        }

        fn unchecked_empty_prototype_info(&self) -> FakeRootObject {
            FakeRootObject{ptr: 18}
        }

        fn unchecked_optimism_disabled(&self) -> FakeRootObject {
            FakeRootObject{ptr: 19}
        }

        fn unchecked_termination_exception(&self) -> FakeRootObject {
            FakeRootObject{ptr: 20}
        }

        fn unchecked_out_of_memory_exception(&self) -> FakeRootObject {
            FakeRootObject{ptr: 21}
        }
    }
    // Fake type
    pub struct FakeRootObject {
        pub ptr: usize
    }

    // Fake impl for a missing trait
    impl FakeRootObject {
        pub fn ptr(&self) -> usize {
            self.ptr
        }
    }

    pub mod v8heapcompressionscheme {
        use super::Tagged_t;
        pub fn compress_object(ptr: usize) -> Tagged_t {
            ptr as Tagged_t // Dummy implementation
        }
    }

    // Mock READ_ONLY_ROOT_LIST macro
    macro_rules! read_only_root_list {
        ($add_root:ident) => {
            $add_root!(_, null, Null);
            $add_root!(_, the_hole, TheHole);
            $add_root!(_, undefined_value, UndefinedValue);
            $add_root!(_, uninitialized_symbol, UninitializedSymbol);
            $add_root!(_, empty_string, EmptyString);
            $add_root!(_, single_character_string, SingleCharacterString);
            $add_root!(_, one_byte_string_table, OneByteStringTable);
            $add_root!(_, number_string_cache, NumberStringCache);
            $add_root!(_, empty_fixed_array, EmptyFixedArray);
            $add_root!(_, empty_weak_fixed_array, EmptyWeakFixedArray);
            $add_root!(_, empty_descriptor_array, EmptyDescriptorArray);
            $add_root!(_, empty_ordered_hash_table, EmptyOrderedHashTable);
            $add_root!(_, empty_fixed_double_array, EmptyFixedDoubleArray);
            $add_root!(_, empty_script_list, EmptyScriptList);
            $add_root!(_, empty_array, EmptyArray);
            $add_root!(_, empty_context, EmptyContext);
            $add_root!(_, native_context_constructor, NativeContextConstructor);
            $add_root!(_, allocation_site_without_feedback, AllocationSiteWithoutFeedback);
            $add_root!(_, empty_prototype_info, EmptyPrototypeInfo);
            $add_root!(_, optimism_disabled, OptimismDisabled);
            $add_root!(_, termination_exception, TerminationException);
            $add_root!(_, out_of_memory_exception, OutOfMemoryException);
        };
    }

    const K_READ_ONLY_ROOTS_COUNT: usize = 22;
    struct StaticRootsTableGenImpl<'a> {
        sorted_roots: BTreeMap<Tagged_t, Vec<RootIndex>>,
        camel_names: HashMap<RootIndex, String>,
        isolate: &'a Isolate,
    }

    impl<'a> StaticRootsTableGenImpl<'a> {
        pub fn new(isolate: &'a Isolate) -> Self {
            let mut sorted_roots: BTreeMap<Tagged_t, Vec<RootIndex>> = BTreeMap::new();
            let mut camel_names: HashMap<RootIndex, String> = HashMap::new();

            // Collect all roots
            let ro_roots = ReadOnlyRoots::new(isolate);
            {
                let mut pos: RootIndex = 0;

                macro_rules! add_root {
                    (_, $value:ident, $camel_name:ident) => {
                        let ptr = v8heapcompressionscheme::compress_object(ro_roots.unchecked_$value().ptr());
                        sorted_roots.entry(ptr).or_insert(Vec::new()).push(pos);
                        camel_names.insert(pos, stringify!($camel_name).to_string());
                        pos += 1;
                    };
                }
                read_only_root_list!(add_root);
            }

            StaticRootsTableGenImpl {
                sorted_roots,
                camel_names,
                isolate
            }
        }

        pub fn sorted_roots(&self) -> &BTreeMap<Tagged_t, Vec<RootIndex>> {
            &self.sorted_roots
        }

        pub fn camel_name(&self, idx: RootIndex) -> &String {
            self.camel_names.get(&idx).unwrap()
        }
    }

    pub struct StaticRootsTableGen {}

    impl StaticRootsTableGen {
        pub fn write(isolate: &Isolate, file: &str) -> Result<()> {
            if V8_STATIC_ROOTS_BOOL {
                panic!("Re-generating the table of roots is only supported in builds with v8_enable_static_roots disabled");
            }
            if !V8_STATIC_ROOTS_GENERATION_BOOL {
                panic!("V8_STATIC_ROOTS_GENERATION_BOOL must be true");
            }

            let mut out = File::create(file)?;

            let header = "// Copyright 2022 the V8 project authors. All rights reserved.\n\
                // Use of this source code is governed by a BSD-style license that can be\n\
                // found in the LICENSE file.\n\n\
                // This file is automatically generated by `tools/dev/gen-static-roots.py`. Do\n\
                // not edit manually.\n\n\
                #ifndef V8_ROOTS_STATIC_ROOTS_H_\n\
                #define V8_ROOTS_STATIC_ROOTS_H_\n\n\
                #include \"src/common/globals.h\"\n\n\
                #if V8_STATIC_ROOTS_BOOL\n\n\
                #include \"src/roots/roots.h\"\n\n\
                // Disabling Wasm or Intl invalidates the contents of static-roots.h.\n\
                // TODO(olivf): To support static roots for multiple build configurations we\n\
                //              will need to generate target specific versions of this file.\n\
                static_assert(V8_ENABLE_WEBASSEMBLY);\n\
                static_assert(V8_INTL_SUPPORT);\n\n\
                namespace v8 {\n\
                namespace internal {\n\n\
                struct StaticReadOnlyRoot {\n";
            out.write_all(header.as_bytes())?;

            // Output a symbol for every root. Ordered by ptr to make it easier to see the
            // memory layout of the read only page.
            let size = K_READ_ONLY_ROOTS_COUNT;
            let gen = StaticRootsTableGenImpl::new(isolate);

            for (ptr, roots) in gen.sorted_roots().iter() {
                if *ptr >= K_REGULAR_PAGE_SIZE {
                    panic!("ptr must be less than K_REGULAR_PAGE_SIZE");
                }

                for root in roots {
                    let k_pre_string = "  static constexpr Tagged_t k";
                    let name = gen.camel_name(*root);
                    let ptr_len = (f64::ln(*ptr as f64) / f64::ln(2.0) / 4.0).ceil() as usize;
                    // Full line is: "kPreString|name = 0x.....;"
                    let len = k_pre_string.len() + name.len() + 5 + ptr_len + 1;
                    write!(out, "{} {} =", k_pre_string, name)?;
                    if len > 80 {
                        write!(out, "\n     ")?;
                    }
                    write!(out, " 0x{:x};\n", ptr)?;
                }
            }

            let first_root = gen.sorted_roots().iter().next().map(|(k, _)| k).unwrap();
            let last_root = gen.sorted_roots().iter().next_back().map(|(k, _)| k).unwrap();
            write!(out, "\n")?;
            write!(out, "  static constexpr Tagged_t kFirstAllocatedRoot = 0x{:x};\n", first_root)?;
            write!(out, "  static constexpr Tagged_t kLastAllocatedRoot = 0x{:x};\n", last_root)?;
            write!(out, "}};\n")?;

            // Output in order of roots table
            write!(out, "\nstatic constexpr std::array<Tagged_t, {}> StaticReadOnlyRootsPointerTable = {{\n", size)?;

            macro_rules! entry {
                ($(_1:tt, _2:tt, $camel_name:ident),*) => {
                    $(
                        write!(out, "    StaticReadOnlyRoot::k{},\n", stringify!($camel_name))?;
                    )*
                };
            }
            write!(out, "")?;
            macro_rules! add_entry {
                ($(_1:tt, _2:tt, $CamelName:ident),*) => {
                  $(
                      write!(out, "    StaticReadOnlyRoot::k{},\n", stringify!($CamelName))?;
                  )*
                };
            }
            macro_rules! generate_table_entries {
              ($macro:ident) => {
                $macro!{
                    (_, null, Null),
                    (_, the_hole, TheHole),
                    (_, undefined_value, UndefinedValue),
                    (_, uninitialized_symbol, UninitializedSymbol),
                    (_, empty_string, EmptyString),
                    (_, single_character_string, SingleCharacterString),
                    (_, one_byte_string_table, OneByteStringTable),
                    (_, number_string_cache, NumberStringCache),
                    (_, empty_fixed_array, EmptyFixedArray),
                    (_, empty_weak_fixed_array, EmptyWeakFixedArray),
                    (_, empty_descriptor_array, EmptyDescriptorArray),
                    (_, empty_ordered_hash_table, EmptyOrderedHashTable),
                    (_, empty_fixed_double_array, EmptyFixedDoubleArray),
                    (_, empty_script_list, EmptyScriptList),
                    (_, empty_array, EmptyArray),
                    (_, empty_context, EmptyContext),
                    (_, native_context_constructor, NativeContextConstructor),
                    (_, allocation_site_without_feedback, AllocationSiteWithoutFeedback),
                    (_, empty_prototype_info, EmptyPrototypeInfo),
                    (_, optimism_disabled, OptimismDisabled),
                    (_, termination_exception, TerminationException),
                    (_, out_of_memory_exception, OutOfMemoryException)
                  }
              };
            }
            generate_table_entries!(add_entry);
            write!(out, "}};\n")?;

            let footer = "\n}  // namespace internal\n\
                }  // namespace v8\n\
                #endif  // V8_STATIC_ROOTS_BOOL\n\
                #endif  // V8_ROOTS_STATIC_ROOTS_H_\n";
            out.write_all(footer.as_bytes())?;

            Ok(())
        }
    }
}