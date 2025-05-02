mod builtins_utils_gen {
    // Placeholder for builtins-utils-gen.h content.
}

mod builtins {
    // Placeholder for builtins.h content.
    pub struct CodeAssemblerState {} //Dummy Struct
    pub struct Builtins {}
    impl Builtins {
        pub fn generate_load_ic(_state: &CodeAssemblerState) {}
        pub fn generate_load_ic_megamorphic(_state: &CodeAssemblerState) {}
        pub fn generate_load_ic_noninlined(_state: &CodeAssemblerState) {}
        pub fn generate_load_ic_no_feedback(_state: &CodeAssemblerState) {}
        pub fn generate_load_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_load_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_load_ic_trampoline_megamorphic(_state: &CodeAssemblerState) {}
        pub fn generate_load_super_ic(_state: &CodeAssemblerState) {}
        pub fn generate_load_super_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_load_ic(_state: &CodeAssemblerState) {}
        pub fn generate_enumerated_keyed_load_ic(_state: &CodeAssemblerState) {}
        pub fn generate_enumerated_keyed_load_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_load_ic_megamorphic(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_load_ic_polymorphic_name(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_load_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_load_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_load_ic_trampoline_megamorphic(_state: &CodeAssemblerState) {}
        pub fn generate_load_global_ic_no_feedback(_state: &CodeAssemblerState) {}
        pub fn generate_store_global_ic(_state: &CodeAssemblerState) {}
        pub fn generate_store_global_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_store_global_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_store_ic(_state: &CodeAssemblerState) {}
        pub fn generate_store_ic_megamorphic(_state: &CodeAssemblerState) {}
        pub fn generate_store_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_store_ic_trampoline_megamorphic(_state: &CodeAssemblerState) {}
        pub fn generate_store_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_define_named_own_ic(_state: &CodeAssemblerState) {}
        pub fn generate_define_named_own_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_define_named_own_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_store_ic(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_store_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_store_ic_trampoline_megamorphic(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_store_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_define_keyed_own_ic(_state: &CodeAssemblerState) {}
        pub fn generate_define_keyed_own_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_define_keyed_own_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_store_in_array_literal_ic(_state: &CodeAssemblerState) {}
        pub fn generate_store_in_array_literal_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_clone_object_ic(_state: &CodeAssemblerState) {}
        pub fn generate_clone_object_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_clone_object_ic_slow(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_has_ic(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_has_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_has_ic_megamorphic(_state: &CodeAssemblerState) {}
        pub fn generate_keyed_has_ic_polymorphic_name(_state: &CodeAssemblerState) {}
        pub fn generate_load_global_ic(_state: &CodeAssemblerState) {}
        pub fn generate_load_global_ic_inside_typeof(_state: &CodeAssemblerState) {}
        pub fn generate_load_global_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_load_global_ic_inside_typeof_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_load_global_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_load_global_ic_inside_typeof_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_global_ic(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_global_ic_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_global_ic_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_global_ic_inside_typeof(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_global_ic_inside_typeof_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_global_ic_inside_typeof_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_context_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_script_context_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_context_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_script_context_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_context_inside_typeof_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_script_context_inside_typeof_trampoline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_context_inside_typeof_baseline(_state: &CodeAssemblerState) {}
        pub fn generate_lookup_script_context_inside_typeof_baseline(_state: &CodeAssemblerState) {}
    }
}

mod codegen {
    // Placeholder for codegen/code-stub-assembler-inl.h content.
}

mod ic {
    // Placeholder for ic/accessor-assembler.h content.
    use crate::builtins::CodeAssemblerState;
    struct AccessorAssembler {
        state: *mut CodeAssemblerState,
    }

    impl AccessorAssembler {
        fn new(state: *mut CodeAssemblerState) -> Self {
            AccessorAssembler { state }
        }

        fn generate_load_ic(&self) {}
        fn generate_load_ic_megamorphic(&self) {}
        fn generate_load_ic_noninlined(&self) {}
        fn generate_load_ic_no_feedback(&self) {}
        fn generate_load_ic_trampoline(&self) {}
        fn generate_load_ic_baseline(&self) {}
        fn generate_load_ic_trampoline_megamorphic(&self) {}
        fn generate_load_super_ic(&self) {}
        fn generate_load_super_ic_baseline(&self) {}
        fn generate_keyed_load_ic(&self) {}
        fn generate_enumerated_keyed_load_ic(&self) {}
        fn generate_enumerated_keyed_load_ic_baseline(&self) {}
        fn generate_keyed_load_ic_megamorphic(&self) {}
        fn generate_keyed_load_ic_polymorphic_name(&self) {}
        fn generate_keyed_load_ic_trampoline(&self) {}
        fn generate_keyed_load_ic_baseline(&self) {}
        fn generate_keyed_load_ic_trampoline_megamorphic(&self) {}
        fn generate_load_global_ic_no_feedback(&self) {}
        fn generate_store_global_ic(&self) {}
        fn generate_store_global_ic_trampoline(&self) {}
        fn generate_store_global_ic_baseline(&self) {}
        fn generate_store_ic(&self) {}
        fn generate_store_ic_megamorphic(&self) {}
        fn generate_store_ic_trampoline(&self) {}
        fn generate_store_ic_trampoline_megamorphic(&self) {}
        fn generate_store_ic_baseline(&self) {}
        fn generate_define_named_own_ic(&self) {}
        fn generate_define_named_own_ic_trampoline(&self) {}
        fn generate_define_named_own_ic_baseline(&self) {}
        fn generate_keyed_store_ic(&self) {}
        fn generate_keyed_store_ic_trampoline(&self) {}
        fn generate_keyed_store_ic_trampoline_megamorphic(&self) {}
        fn generate_keyed_store_ic_baseline(&self) {}
        fn generate_define_keyed_own_ic(&self) {}
        fn generate_define_keyed_own_ic_trampoline(&self) {}
        fn generate_define_keyed_own_ic_baseline(&self) {}
        fn generate_store_in_array_literal_ic(&self) {}
        fn generate_store_in_array_literal_ic_baseline(&self) {}
        fn generate_clone_object_ic(&self) {}
        fn generate_clone_object_ic_baseline(&self) {}
        fn generate_clone_object_ic_slow(&self) {}
        fn generate_keyed_has_ic(&self) {}
        fn generate_keyed_has_ic_baseline(&self) {}
        fn generate_keyed_has_ic_megamorphic(&self) {}
        fn generate_keyed_has_ic_polymorphic_name(&self) {}

        fn generate_load_global_ic(&self, _typeof_mode: TypeofMode) {}
        fn generate_load_global_ic_trampoline(&self, _typeof_mode: TypeofMode) {}
        fn generate_load_global_ic_baseline(&self, _typeof_mode: TypeofMode) {}
        fn generate_lookup_global_ic(&self, _typeof_mode: TypeofMode) {}
        fn generate_lookup_global_ic_trampoline(&self, _typeof_mode: TypeofMode) {}
        fn generate_lookup_global_ic_baseline(&self, _typeof_mode: TypeofMode) {}
        fn generate_lookup_context_trampoline(&self, _typeof_mode: TypeofMode, _context_kind: ContextKind) {}
        fn generate_lookup_context_baseline(&self, _typeof_mode: TypeofMode, _context_kind: ContextKind) {}
    }

    enum TypeofMode {
        kNotInside,
        kInside,
    }

    enum ContextKind {
        kDefault,
        kScriptContext,
    }
}

pub mod builtins_ic_gen {
    use crate::builtins;
    use crate::ic::AccessorAssembler;
    use crate::builtins::CodeAssemblerState;
    

    impl builtins::Builtins {
        pub fn generate_load_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_ic();
        }
        pub fn generate_load_ic_megamorphic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_ic_megamorphic();
        }
        pub fn generate_load_ic_noninlined(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_ic_noninlined();
        }
        pub fn generate_load_ic_no_feedback(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_ic_no_feedback();
        }
        pub fn generate_load_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_ic_trampoline();
        }
        pub fn generate_load_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_ic_baseline();
        }
        pub fn generate_load_ic_trampoline_megamorphic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_ic_trampoline_megamorphic();
        }
        pub fn generate_load_super_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_super_ic();
        }
        pub fn generate_load_super_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_super_ic_baseline();
        }
        pub fn generate_keyed_load_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_load_ic();
        }
        pub fn generate_enumerated_keyed_load_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_enumerated_keyed_load_ic();
        }
        pub fn generate_enumerated_keyed_load_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_enumerated_keyed_load_ic_baseline();
        }
        pub fn generate_keyed_load_ic_megamorphic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_load_ic_megamorphic();
        }
        pub fn generate_keyed_load_ic_polymorphic_name(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_load_ic_polymorphic_name();
        }
        pub fn generate_keyed_load_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_load_ic_trampoline();
        }
        pub fn generate_keyed_load_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_load_ic_baseline();
        }
        pub fn generate_keyed_load_ic_trampoline_megamorphic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_load_ic_trampoline_megamorphic();
        }
        pub fn generate_load_global_ic_no_feedback(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_global_ic_no_feedback();
        }
        pub fn generate_store_global_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_global_ic();
        }
        pub fn generate_store_global_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_global_ic_trampoline();
        }
        pub fn generate_store_global_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_global_ic_baseline();
        }
        pub fn generate_store_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_ic();
        }
        pub fn generate_store_ic_megamorphic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_ic_megamorphic();
        }
        pub fn generate_store_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_ic_trampoline();
        }
        pub fn generate_store_ic_trampoline_megamorphic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_ic_trampoline_megamorphic();
        }
        pub fn generate_store_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_ic_baseline();
        }
        pub fn generate_define_named_own_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_define_named_own_ic();
        }
        pub fn generate_define_named_own_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_define_named_own_ic_trampoline();
        }
        pub fn generate_define_named_own_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_define_named_own_ic_baseline();
        }
        pub fn generate_keyed_store_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_store_ic();
        }
        pub fn generate_keyed_store_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_store_ic_trampoline();
        }
        pub fn generate_keyed_store_ic_trampoline_megamorphic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_store_ic_trampoline_megamorphic();
        }
        pub fn generate_keyed_store_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_store_ic_baseline();
        }
        pub fn generate_define_keyed_own_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_define_keyed_own_ic();
        }
        pub fn generate_define_keyed_own_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_define_keyed_own_ic_trampoline();
        }
        pub fn generate_define_keyed_own_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_define_keyed_own_ic_baseline();
        }
        pub fn generate_store_in_array_literal_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_in_array_literal_ic();
        }
        pub fn generate_store_in_array_literal_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_store_in_array_literal_ic_baseline();
        }
        pub fn generate_clone_object_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_clone_object_ic();
        }
        pub fn generate_clone_object_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_clone_object_ic_baseline();
        }
        pub fn generate_clone_object_ic_slow(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_clone_object_ic_slow();
        }
        pub fn generate_keyed_has_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_has_ic();
        }
        pub fn generate_keyed_has_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_has_ic_baseline();
        }
        pub fn generate_keyed_has_ic_megamorphic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_has_ic_megamorphic();
        }
        pub fn generate_keyed_has_ic_polymorphic_name(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_keyed_has_ic_polymorphic_name();
        }

        pub fn generate_load_global_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_global_ic(crate::ic::TypeofMode::kNotInside);
        }

        pub fn generate_load_global_ic_inside_typeof(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_global_ic(crate::ic::TypeofMode::kInside);
        }

        pub fn generate_load_global_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_global_ic_trampoline(crate::ic::TypeofMode::kNotInside);
        }

        pub fn generate_load_global_ic_inside_typeof_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_global_ic_trampoline(crate::ic::TypeofMode::kInside);
        }

        pub fn generate_load_global_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_global_ic_baseline(crate::ic::TypeofMode::kNotInside);
        }

        pub fn generate_load_global_ic_inside_typeof_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_load_global_ic_baseline(crate::ic::TypeofMode::kInside);
        }

        pub fn generate_lookup_global_ic(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_global_ic(crate::ic::TypeofMode::kNotInside);
        }

        pub fn generate_lookup_global_ic_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_global_ic_trampoline(crate::ic::TypeofMode::kNotInside);
        }

        pub fn generate_lookup_global_ic_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_global_ic_baseline(crate::ic::TypeofMode::kNotInside);
        }

        pub fn generate_lookup_global_ic_inside_typeof(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_global_ic(crate::ic::TypeofMode::kInside);
        }

        pub fn generate_lookup_global_ic_inside_typeof_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_global_ic_trampoline(crate::ic::TypeofMode::kInside);
        }

        pub fn generate_lookup_global_ic_inside_typeof_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_global_ic_baseline(crate::ic::TypeofMode::kInside);
        }

        pub fn generate_lookup_context_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_context_trampoline(
                crate::ic::TypeofMode::kNotInside,
                crate::ic::ContextKind::kDefault,
            );
        }

        pub fn generate_lookup_script_context_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_context_trampoline(
                crate::ic::TypeofMode::kNotInside,
                crate::ic::ContextKind::kScriptContext,
            );
        }

        pub fn generate_lookup_context_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_context_baseline(
                crate::ic::TypeofMode::kNotInside,
                crate::ic::ContextKind::kDefault,
            );
        }

        pub fn generate_lookup_script_context_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_context_baseline(
                crate::ic::TypeofMode::kNotInside,
                crate::ic::ContextKind::kScriptContext,
            );
        }

        pub fn generate_lookup_context_inside_typeof_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_context_trampoline(
                crate::ic::TypeofMode::kInside,
                crate::ic::ContextKind::kDefault,
            );
        }

        pub fn generate_lookup_script_context_inside_typeof_trampoline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_context_trampoline(
                crate::ic::TypeofMode::kInside,
                crate::ic::ContextKind::kScriptContext,
            );
        }

        pub fn generate_lookup_context_inside_typeof_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_context_baseline(
                crate::ic::TypeofMode::kInside,
                crate::ic::ContextKind::kDefault,
            );
        }

        pub fn generate_lookup_script_context_inside_typeof_baseline(state: &CodeAssemblerState) {
            let assembler = AccessorAssembler::new(state as *const _ as *mut _);
            assembler.generate_lookup_context_baseline(
                crate::ic::TypeofMode::kInside,
                crate::ic::ContextKind::kScriptContext,
            );
        }
    }
}