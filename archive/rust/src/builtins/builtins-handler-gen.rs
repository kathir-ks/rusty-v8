// src/builtins/builtins-handler-gen.rs

//use std::any::Any;
//use std::convert::TryInto;
//use std::mem::transmute;
//use std::rc::Rc;

//use crate::builtins::builtins_utils_gen::*; // Assuming a Rust equivalent exists
//use crate::builtins::builtins::*; // Assuming a Rust equivalent exists
//use crate::codegen::code_stub_assembler::*; // Assuming a Rust equivalent exists
//use crate::codegen::code_stub_assembler_inl::*; // Assuming a Rust equivalent exists
//use crate::ic::ic::*; // Assuming a Rust equivalent exists
//use crate::ic::keyed_store_generic::*; // Assuming a Rust equivalent exists
//use crate::objects::objects::*; // Assuming a Rust equivalent exists
//use crate::objects::objects_inl::*; // Assuming a Rust equivalent exists
//use crate::torque_generated::exported_macros_assembler::*; // Assuming a Rust equivalent exists

//#[macro_use]
//use crate::codegen::define_code_stub_assembler_macros::*; // Assuming a Rust equivalent exists

//pub mod define_code_stub_assembler_macros {
//    #[macro_export]
//    macro_rules! TF_BUILTIN {
//        ($name:ident, $assembler:ident) => {
//            pub fn $name(context: &mut crate::codegen::code_assembler::CodeAssemblerState) {
//                let assembler = $assembler::new(context);
//                assembler.generate();
//            }
//        };
//    }
//}

//pub mod undef_code_stub_assembler_macros {} // Placeholder - No direct equivalent for #undef

pub mod builtins {
    // Builtins::Generate_KeyedStoreIC_Megamorphic
    pub fn generate_keyed_store_ic_megamorphic(
        state: &mut crate::codegen::code_assembler::CodeAssemblerState,
    ) {
        crate::ic::keyed_store_generic::KeyedStoreMegamorphicGenerator::generate(state);
    }

    // Builtins::Generate_DefineKeyedOwnIC_Megamorphic
    pub fn generate_define_keyed_own_ic_megamorphic(
        state: &mut crate::codegen::code_assembler::CodeAssemblerState,
    ) {
        crate::ic::keyed_store_generic::DefineKeyedOwnGenericGenerator::generate(state);
    }

    // Builtins::Generate_StoreIC_NoFeedback
    pub fn generate_store_ic_no_feedback(
        state: &mut crate::codegen::code_assembler::CodeAssemblerState,
    ) {
        crate::ic::keyed_store_generic::StoreICNoFeedbackGenerator::generate(state);
    }

    // Builtins::Generate_DefineNamedOwnIC_NoFeedback
    pub fn generate_define_named_own_ic_no_feedback(
        state: &mut crate::codegen::code_assembler::CodeAssemblerState,
    ) {
        crate::ic::keyed_store_generic::DefineNamedOwnICNoFeedbackGenerator::generate(state);
    }
}

pub mod handler_builtins_assembler {
    use std::convert::TryInto;

    use crate::codegen::code_assembler::*; // Assuming a Rust equivalent exists
    use crate::codegen::code_stub_assembler::*;
    //use crate::objects::objects::*; // Assuming a Rust equivalent exists
    //use crate::objects::objects_inl::*; // Assuming a Rust equivalent exists

    // Placeholder enums and structs.  These would need to be defined
    // based on what the original C++ code uses.  I've tried to make
    // reasonable assumptions.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum KeyedAccessStoreMode {
        kInBounds,
        kGrowAndHandleCOW,
        kIgnoreTypedArrayOOB,
        kHandleCOW,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ElementsKind {
        PACKED_SMI_ELEMENTS,
        HOLEY_SMI_ELEMENTS,
        PACKED_DOUBLE_ELEMENTS,
        HOLEY_DOUBLE_ELEMENTS,
        PACKED_ELEMENTS,
        HOLEY_ELEMENTS,
        PACKED_NONEXTENSIBLE_ELEMENTS,
        PACKED_SEALED_ELEMENTS,
        SHARED_ARRAY_ELEMENTS,
        UINT8_ELEMENTS,
        INT8_ELEMENTS,
        UINT16_ELEMENTS,
        INT16_ELEMENTS,
        UINT32_ELEMENTS,
        INT32_ELEMENTS,
        FLOAT16_ELEMENTS,
        FLOAT32_ELEMENTS,
        FLOAT64_ELEMENTS,
        UINT8_CLAMPED_ELEMENTS,
        BIGUINT64_ELEMENTS,
        BIGINT64_ELEMENTS,
        RAB_GSAB_UINT8_ELEMENTS,
        RAB_GSAB_INT8_ELEMENTS,
        RAB_GSAB_UINT16_ELEMENTS,
        RAB_GSAB_INT16_ELEMENTS,
        RAB_GSAB_UINT32_ELEMENTS,
        RAB_GSAB_INT32_ELEMENTS,
        RAB_GSAB_FLOAT16_ELEMENTS,
        RAB_GSAB_FLOAT32_ELEMENTS,
        RAB_GSAB_FLOAT64_ELEMENTS,
        RAB_GSAB_UINT8_CLAMPED_ELEMENTS,
        RAB_GSAB_BIGUINT64_ELEMENTS,
        RAB_GSAB_BIGINT64_ELEMENTS,
    }

    pub struct StoreTransitionDescriptor; // Placeholder
    impl StoreTransitionDescriptor {
        pub const kReceiver: usize = 0;
        pub const kName: usize = 1;
        pub const kValue: usize = 2;
        pub const kMap: usize = 3;
        pub const kSlot: usize = 4;
        pub const kVector: usize = 5;
        pub const kContext: usize = 6;
    }

    pub struct StoreWithVectorDescriptor; // Placeholder
    impl StoreWithVectorDescriptor {
        pub const kReceiver: usize = 0;
        pub const kName: usize = 1;
        pub const kValue: usize = 2;
        pub const kSlot: usize = 3;
        pub const kVector: usize = 4;
        pub const kContext: usize = 5;
    }

    //impl ElementsKind {
    //    pub fn is_typed_array_or_rab_gsab_typed_array(self) -> bool {
    //        use ElementsKind::*;
    //        matches!(
    //            self,
    //            UINT8_ELEMENTS
    //                | INT8_ELEMENTS
    //                | UINT16_ELEMENTS
    //                | INT16_ELEMENTS
    //                | UINT32_ELEMENTS
    //                | INT32_ELEMENTS
    //                | FLOAT32_ELEMENTS
    //                | FLOAT64_ELEMENTS
    //                | UINT8_CLAMPED_ELEMENTS
    //                | BIGUINT64_ELEMENTS
    //                | BIGINT64_ELEMENTS
    //                | RAB_GSAB_UINT8_ELEMENTS
    //                | RAB_GSAB_INT8_ELEMENTS
    //                | RAB_GSAB_UINT16_ELEMENTS
    //                | RAB_GSAB_INT16_ELEMENTS
    //                | RAB_GSAB_UINT32_ELEMENTS
    //                | RAB_GSAB_INT32_ELEMENTS
    //                | RAB_GSAB_FLOAT32_ELEMENTS
    //                | RAB_GSAB_FLOAT64_ELEMENTS
    //                | RAB_GSAB_UINT8_CLAMPED_ELEMENTS
    //                | RAB_GSAB_BIGUINT64_ELEMENTS
    //                | RAB_GSAB_BIGINT64_ELEMENTS
    //        )
    //    }
    //}

    pub struct HandlerBuiltinsAssembler<'a> {
        assembler: &'a mut CodeAssemblerState,
    }

    impl<'a> HandlerBuiltinsAssembler<'a> {
        pub fn new(state: &'a mut CodeAssemblerState) -> Self {
            HandlerBuiltinsAssembler { assembler: state }
        }

        fn generate_keyed_store_ic_sloppy_arguments(&mut self) {
            //using Descriptor = StoreWithVectorDescriptor;
            //auto receiver = Parameter<JSObject>(Descriptor::kReceiver);
            //auto key = Parameter<Object>(Descriptor::kName);
            //auto value = Parameter<JSAny>(Descriptor::kValue);
            //auto slot = Parameter<Smi>(Descriptor::kSlot);
            //auto vector = Parameter<HeapObject>(Descriptor::kVector);
            //auto context = Parameter<Context>(Descriptor::kContext);

            //Label miss(this);

            //SloppyArgumentsStore(receiver, key, value, &miss);
            //Return(value);

            //BIND(&miss);
            //TailCallRuntime(Runtime::kKeyedStoreIC_Miss, context, value, slot, vector,
            //                  receiver, key);
            todo!()
        }

        fn generate_elements_transition_and_store(&mut self, store_mode: KeyedAccessStoreMode) {
            //using Descriptor = StoreTransitionDescriptor;
            //auto receiver = Parameter<JSObject>(Descriptor::kReceiver);
            //auto key = Parameter<Object>(Descriptor::kName);
            //auto value = Parameter<Object>(Descriptor::kValue);
            //auto map = Parameter<Map>(Descriptor::kMap);
            //auto slot = Parameter<Smi>(Descriptor::kSlot);
            //auto vector = Parameter<FeedbackVector>(Descriptor::kVector);
            //auto context = Parameter<Context>(Descriptor::kContext);

            //Comment("ElementsTransitionAndStore: store_mode=", store_mode);

            //Label miss(this);

            //if (v8_flags.trace_elements_transitions) {
            // Tracing elements transitions is the job of the runtime.
            //  Goto(&miss);
            //} else {
            // TODO(v8:8481): Pass from_kind and to_kind in feedback vector slots.
            //  DispatchForElementsKindTransition(
            //      LoadElementsKind(receiver), LoadMapElementsKind(map),
            //      [=, this, &miss](ElementsKind from_kind, ElementsKind to_kind) {
            //        TransitionElementsKind(receiver, map, from_kind, to_kind, &miss);
            //        EmitElementStore(receiver, key, value, to_kind, store_mode, &miss,
            //                         context, nullptr);
            //      });
            //  Return(value);
            //}

            //BIND(&miss);
            //TailCallRuntime(Runtime::kElementsTransitionAndStoreIC_Miss, context,
            //                receiver, key, value, map, slot, vector);
            todo!()
        }

        fn generate_store_fast_element_ic(&mut self, store_mode: KeyedAccessStoreMode) {
            //using Descriptor = StoreWithVectorDescriptor;
            //auto receiver = Parameter<JSObject>(Descriptor::kReceiver);
            //auto key = Parameter<Object>(Descriptor::kName);
            //auto value = Parameter<Object>(Descriptor::kValue);
            //auto slot = Parameter<Smi>(Descriptor::kSlot);
            //auto vector = Parameter<HeapObject>(Descriptor::kVector);
            //auto context = Parameter<Context>(Descriptor::kContext);

            //Comment("StoreFastElementStub: store_mode=", store_mode);

            //Label miss(this);

            // For typed arrays maybe_converted_value contains the value obtained after
            // calling ToNumber. We should pass the converted value to the runtime to
            // avoid doing the user visible conversion again.
            //TVARIABLE(Object, maybe_converted_value, value);
            // TODO(v8:8481): Pass elements_kind in feedback vector slots.
            //DispatchByElementsKind(
            //LoadElementsKind(receiver),
            //[=, this, &miss, &maybe_converted_value](ElementsKind elements_kind) {
            //  EmitElementStore(receiver, key, value, elements_kind, store_mode, &miss,
            //                   context, &maybe_converted_value);
            //},
            //StoreModeSupportsTypeArray(store_mode));
            //Return(value);

            //BIND(&miss);
            //TailCallRuntime(Runtime::kKeyedStoreIC_Miss, context,
            // maybe_converted_value.value(), slot, vector, receiver, key);
            todo!()
        }

        fn dispatch_for_elements_kind_transition(
            &mut self,
            from_kind: i32,
            to_kind: i32,
            case_function: &dyn Fn(ElementsKind, ElementsKind),
        ) {
            //static_assert(sizeof(ElementsKind) == sizeof(uint8_t));

            //Label next(this), if_unknown_type(this, Label::kDeferred);

            //int32_t combined_elements_kinds[] = {
            //#define ELEMENTS_KINDS_CASE(FROM, TO) (FROM << kBitsPerByte) | TO,
            //  ELEMENTS_KIND_TRANSITIONS(ELEMENTS_KINDS_CASE)
            //#undef ELEMENTS_KINDS_CASE
            //};

            //#define ELEMENTS_KINDS_CASE(FROM, TO) Label if_##FROM##_##TO(this);
            //ELEMENTS_KIND_TRANSITIONS(ELEMENTS_KINDS_CASE)
            //#undef ELEMENTS_KINDS_CASE

            //Label* elements_kind_labels[] = {
            //#define ELEMENTS_KINDS_CASE(FROM, TO) &if_##FROM##_##TO,
            //  ELEMENTS_KIND_TRANSITIONS(ELEMENTS_KINDS_CASE)
            //#undef ELEMENTS_KINDS_CASE
            //};
            //static_assert(arraysize(combined_elements_kinds) ==
            //          arraysize(elements_kind_labels));

            //TNode<Int32T> combined_elements_kind =
            //  Word32Or(Word32Shl(from_kind, Int32Constant(kBitsPerByte)), to_kind);

            //Switch(combined_elements_kind, &if_unknown_type, combined_elements_kinds,
            //     elements_kind_labels, arraysize(combined_elements_kinds));

            //#define ELEMENTS_KINDS_CASE(FROM, TO) \
            //BIND(&if_##FROM##_##TO);            \
            //{                                   \
            //  case_function(FROM, TO);          \
            //  Goto(&next);                      \
            //}
            //ELEMENTS_KIND_TRANSITIONS(ELEMENTS_KINDS_CASE)
            //#undef ELEMENTS_KINDS_CASE

            //BIND(&if_unknown_type);
            //Unreachable();

            //BIND(&next);
            todo!()
        }

        fn dispatch_by_elements_kind(
            &mut self,
            elements_kind: i32,
            case_function: &dyn Fn(ElementsKind),
            handle_typed_elements_kind: bool,
        ) {
            //Label next(this), if_unknown_type(this, Label::kDeferred);

            //int32_t elements_kinds[] = {
            //#define ELEMENTS_KINDS_CASE(KIND) KIND,
            //  ELEMENTS_KINDS(ELEMENTS_KINDS_CASE)
            //#undef ELEMENTS_KINDS_CASE
            //};

            //#define ELEMENTS_KINDS_CASE(KIND) Label if_##KIND(this);
            //ELEMENTS_KINDS(ELEMENTS_KINDS_CASE)
            //#undef ELEMENTS_KINDS_CASE

            //Label* elements_kind_labels[] = {
            //#define ELEMENTS_KINDS_CASE(KIND) &if_##KIND,
            //  ELEMENTS_KINDS(ELEMENTS_KINDS_CASE)
            //#undef ELEMENTS_KINDS_CASE
            //};
            //static_assert(arraysize(elements_kinds) == arraysize(elements_kind_labels));

            //// TODO(mythria): Do not emit cases for typed elements kind when
            //// handle_typed_elements is false to decrease the size of the jump table.
            //Switch(elements_kind, &if_unknown_type, elements_kinds, elements_kind_labels,
            //    arraysize(elements_kinds));

            //#define ELEMENTS_KINDS_CASE(KIND)                            \
            //BIND(&if_##KIND);                                          \
            //{                                                          \
            //  if (!handle_typed_elements_kind &&                       \
            //      IsTypedArrayOrRabGsabTypedArrayElementsKind(KIND)) { \
            //    Unreachable();                                         \
            //  } else {                                                 \
            //    case_function(KIND);                                   \
            //    Goto(&next);                                           \
            //  }                                                        \
            //}
            //ELEMENTS_KINDS(ELEMENTS_KINDS_CASE)
            //#undef ELEMENTS_KINDS_CASE

            //BIND(&if_unknown_type);
            //Unreachable();

            //BIND(&next);
            todo!()
        }
    }
}

// Below are the generated functions.

pub mod load_ic_string_length {
    use crate::codegen::code_assembler::*;

    //TF_BUILTIN(LoadIC_StringLength, CodeStubAssembler) {
    //    auto string = Parameter<String>(Descriptor::kReceiver);
    //    Return(LoadStringLengthAsSmi(string));
    //}
    pub fn load_ic_string_length(context: &mut CodeAssemblerState) {
        todo!()
    }
}

pub mod load_ic_string_wrapper_length {
    use crate::codegen::code_assembler::*;
    //TF_BUILTIN(LoadIC_StringWrapperLength, CodeStubAssembler) {
    //    auto value = Parameter<JSPrimitiveWrapper>(Descriptor::kReceiver);
    //    TNode<String> string = CAST(LoadJSPrimitiveWrapperValue(value));
    //    Return(LoadStringLengthAsSmi(string));
    //}
    pub fn load_ic_string_wrapper_length(context: &mut CodeAssemblerState) {
        todo!()
    }
}

pub mod elements_transition_and_store_in_bounds {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(ElementsTransitionAndStore_InBounds, HandlerBuiltinsAssembler) {
    //    Generate_ElementsTransitionAndStore(KeyedAccessStoreMode::kInBounds);
    //}
    pub fn elements_transition_and_store_in_bounds(context: &mut CodeAssemblerState) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_elements_transition_and_store(KeyedAccessStoreMode::kInBounds);
    }
}

pub mod elements_transition_and_store_no_transition_grow_and_handle_cow {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(ElementsTransitionAndStore_NoTransitionGrowAndHandleCOW,
    //           HandlerBuiltinsAssembler) {
    //    Generate_ElementsTransitionAndStore(KeyedAccessStoreMode::kGrowAndHandleCOW);
    //}
    pub fn elements_transition_and_store_no_transition_grow_and_handle_cow(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_elements_transition_and_store(KeyedAccessStoreMode::kGrowAndHandleCOW);
    }
}

pub mod elements_transition_and_store_no_transition_ignore_typed_array_oob {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(ElementsTransitionAndStore_NoTransitionIgnoreTypedArrayOOB,
    //           HandlerBuiltinsAssembler) {
    //    Generate_ElementsTransitionAndStore(
    //        KeyedAccessStoreMode::kIgnoreTypedArrayOOB);
    //}
    pub fn elements_transition_and_store_no_transition_ignore_typed_array_oob(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_elements_transition_and_store(
            KeyedAccessStoreMode::kIgnoreTypedArrayOOB,
        );
    }
}

pub mod elements_transition_and_store_no_transition_handle_cow {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;
    //TF_BUILTIN(ElementsTransitionAndStore_NoTransitionHandleCOW,
    //           HandlerBuiltinsAssembler) {
    //    Generate_ElementsTransitionAndStore(KeyedAccessStoreMode::kHandleCOW);
    //}
    pub fn elements_transition_and_store_no_transition_handle_cow(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_elements_transition_and_store(KeyedAccessStoreMode::kHandleCOW);
    }
}

pub mod store_fast_element_ic_in_bounds {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(StoreFastElementIC_InBounds, HandlerBuiltinsAssembler) {
    //    Generate_StoreFastElementIC(KeyedAccessStoreMode::kInBounds);
    //}
    pub fn store_fast_element_ic_in_bounds(context: &mut CodeAssemblerState) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_store_fast_element_ic(KeyedAccessStoreMode::kInBounds);
    }
}

pub mod store_fast_element_ic_no_transition_grow_and_handle_cow {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(StoreFastElementIC_NoTransitionGrowAndHandleCOW,
    //           HandlerBuiltinsAssembler) {
    //    Generate_StoreFastElementIC(KeyedAccessStoreMode::kGrowAndHandleCOW);
    //}
    pub fn store_fast_element_ic_no_transition_grow_and_handle_cow(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_store_fast_element_ic(KeyedAccessStoreMode::kGrowAndHandleCOW);
    }
}

pub mod store_fast_element_ic_no_transition_ignore_typed_array_oob {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;
    //TF_BUILTIN(StoreFastElementIC_NoTransitionIgnoreTypedArrayOOB,
    //           HandlerBuiltinsAssembler) {
    //    Generate_StoreFastElementIC(
    //        KeyedAccessStoreMode::kIgnoreTypedArrayOOB);
    //}
    pub fn store_fast_element_ic_no_transition_ignore_typed_array_oob(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_store_fast_element_ic(KeyedAccessStoreMode::kIgnoreTypedArrayOOB);
    }
}

pub mod store_fast_element_ic_no_transition_handle_cow {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(StoreFastElementIC_NoTransitionHandleCOW, HandlerBuiltinsAssembler) {
    //    Generate_StoreFastElementIC(KeyedAccessStoreMode::kHandleCOW);
    //}
    pub fn store_fast_element_ic_no_transition_handle_cow(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_store_fast_element_ic(KeyedAccessStoreMode::kHandleCOW);
    }
}

pub mod load_ic_function_prototype {
    use crate::codegen::code_assembler::*;

    //TF_BUILTIN(LoadIC_FunctionPrototype, CodeStubAssembler) {
    //    auto receiver = Parameter<JSFunction>(Descriptor::kReceiver);
    //    auto name = Parameter<Name>(Descriptor::kName);
    //    auto slot = Parameter<Smi>(Descriptor::kSlot);
    //    auto vector = Parameter<FeedbackVector>(Descriptor::kVector);
    //    auto context = Parameter<Context>(Descriptor::kContext);

    //    Label miss(this, Label::kDeferred);
    //    Return(LoadJSFunctionPrototype(receiver, &miss));

    //    BIND(&miss);
    //    TailCallRuntime(Runtime::kLoadIC_Miss, context, receiver, name, slot, vector);
    //}
    pub fn load_ic_function_prototype(context: &mut CodeAssemblerState) {
        todo!()
    }
}

pub mod store_global_ic_slow {
    use crate::codegen::code_assembler::*;

    //TF_BUILTIN(StoreGlobalIC_Slow, CodeStubAssembler) {
    //    auto receiver = Parameter<Object>(Descriptor::kReceiver);
    //    auto name = Parameter<Name>(Descriptor::kName);
    //    auto value = Parameter<Object>(Descriptor::kValue);
    //    auto slot = Parameter<Smi>(Descriptor::kSlot);
    //    auto vector = Parameter<FeedbackVector>(Descriptor::kVector);
    //    auto context = Parameter<Context>(Descriptor::kContext);

    //    // The slow case calls into the runtime to complete the store without causing
    //    // an IC miss that would otherwise cause a transition to the generic stub.
    //    TailCallRuntime(Runtime::kStoreGlobalIC_Slow, context, value, slot, vector,
    //                    receiver, name);
    //}
    pub fn store_global_ic_slow(context: &mut CodeAssemblerState) {
        todo!()
    }
}

pub mod keyed_load_ic_sloppy_arguments {
    use crate::codegen::code_assembler::*;

    //TF_BUILTIN(KeyedLoadIC_SloppyArguments, HandlerBuiltinsAssembler) {
    //    auto receiver = Parameter<JSObject>(Descriptor::kReceiver);
    //    auto key = Parameter<Object>(Descriptor::kName);
    //    auto slot = Parameter<Smi>(Descriptor::kSlot);
    //    auto vector = Parameter<HeapObject>(Descriptor::kVector);
    //    auto context = Parameter<Context>(Descriptor::kContext);

    //    Label miss(this);

    //    TNode<Object> result = SloppyArgumentsLoad(receiver, key, &miss);
    //    Return(result);

    //    BIND(&miss);
    //    {
    //      Comment("Miss");
    //      TailCallRuntime(Runtime::kKeyedLoadIC_Miss, context, receiver, key, slot,
    //                      vector);
    //    }
    //}
    pub fn keyed_load_ic_sloppy_arguments(context: &mut CodeAssemblerState) {
        todo!()
    }
}

pub mod keyed_store_ic_sloppy_arguments_in_bounds {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(KeyedStoreIC_SloppyArguments_InBounds, HandlerBuiltinsAssembler) {
    //    Generate_KeyedStoreIC_SloppyArguments();
    //}
    pub fn keyed_store_ic_sloppy_arguments_in_bounds(context: &mut CodeAssemblerState) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_keyed_store_ic_sloppy_arguments();
    }
}

pub mod keyed_store_ic_sloppy_arguments_no_transition_grow_and_handle_cow {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(KeyedStoreIC_SloppyArguments_NoTransitionGrowAndHandleCOW,
    //           HandlerBuiltinsAssembler) {
    //    Generate_KeyedStoreIC_SloppyArguments();
    //}
    pub fn keyed_store_ic_sloppy_arguments_no_transition_grow_and_handle_cow(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_keyed_store_ic_sloppy_arguments();
    }
}

pub mod keyed_store_ic_sloppy_arguments_no_transition_ignore_typed_array_oob {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(KeyedStoreIC_SloppyArguments_NoTransitionIgnoreTypedArrayOOB,
    //           HandlerBuiltinsAssembler) {
    //    Generate_KeyedStoreIC_SloppyArguments();
    //}
    pub fn keyed_store_ic_sloppy_arguments_no_transition_ignore_typed_array_oob(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_keyed_store_ic_sloppy_arguments();
    }
}

pub mod keyed_store_ic_sloppy_arguments_no_transition_handle_cow {
    use crate::codegen::code_assembler::*;
    use crate::handler_builtins_assembler::*;

    //TF_BUILTIN(KeyedStoreIC_SloppyArguments_NoTransitionHandleCOW,
    //           HandlerBuiltinsAssembler) {
    //    Generate_KeyedStoreIC_SloppyArguments();
    //}
    pub fn keyed_store_ic_sloppy_arguments_no_transition_handle_cow(
        context: &mut CodeAssemblerState,
    ) {
        let mut assembler = HandlerBuiltinsAssembler::new(context);
        assembler.generate_keyed_store_ic_sloppy_arguments();
    }
}

pub mod load_indexed_interceptor_ic {
    use crate::codegen::code_assembler::*;

    //TF_BUILTIN(LoadIndexedInterceptorIC, CodeStubAssembler) {
    //    auto receiver = Parameter<JSObject>(Descriptor::kReceiver);
    //    auto key = Parameter<Object>(Descriptor::kName);
    //    auto slot = Parameter<Smi>(Descriptor::kSlot);
    //    auto vector = Parameter<HeapObject>(Descriptor::kVector);
    //    auto context = Parameter<Context>(Descriptor::kContext);

    //    Label if_keyispositivesmi(this), if_keyisinvalid(this);
    //    Branch(TaggedIsPositiveSmi(key), &if_keyispositivesmi, &if_keyisinvalid);
    //    BIND(&if_keyispositivesmi);
    //    TailCallRuntime(Runtime::kLoadElementWithInterceptor, context, receiver, key);

    //    BIND(&if_keyisinvalid);
    //    TailCallRuntime(Runtime::kKeyedLoadIC_Miss, context, receiver, key, slot,
    //                    vector);
    //}
    pub fn load_indexed_interceptor_ic(context: &mut CodeAssemblerState) {
        todo!()
    }
}

pub mod keyed_has_ic_sloppy_arguments {
    use crate::codegen::code_assembler::*;

    //TF_BUILTIN(KeyedHasIC_SloppyArguments, HandlerBuiltinsAssembler) {
    //    auto receiver = Parameter<JSObject>(Descriptor::kReceiver);
    //    auto key = Parameter<Object>(Descriptor::kName);
    //    auto slot = Parameter<Smi>(Descriptor::kSlot);
    //    auto vector = Parameter<HeapObject>(Descriptor::kVector);
    //    auto context = Parameter<Context>(Descriptor::kContext);

    //    Label miss(this);

    //    TNode<Object> result = SloppyArgumentsHas(receiver, key, &miss);
    //    Return(result);

    //    BIND(&miss);
    //    {
    //      Comment("Miss");
    //      TailCallRuntime(Runtime::kKeyedHasIC_Miss, context, receiver, key, slot,
    //                      vector);
    //    }
    //}
    pub fn keyed_has_ic_sloppy_arguments(context: &mut CodeAssemblerState) {
        todo!()
    }
}

pub mod has_indexed_interceptor_ic {
    use crate::codegen::code_assembler::*;

    //TF_BUILTIN(HasIndexedInterceptorIC, CodeStubAssembler) {
    //    auto receiver = Parameter<JSObject>(Descriptor::kReceiver);
    //    auto key = Parameter<Object>(Descriptor::kName);
    //    auto slot = Parameter<Smi>(Descriptor::kSlot);
    //    auto vector = Parameter<HeapObject>(Descriptor::kVector);
    //    auto context = Parameter<Context>(Descriptor::kContext);

    //    Label if_keyispositivesmi(this), if_keyisinvalid(this);
    //    Branch(TaggedIsPositiveSmi(key), &if_keyispositivesmi, &if_keyisinvalid);
    //    BIND(&if_keyispositivesmi);
    //    TailCallRuntime(Runtime::kHasElementWithInterceptor, context, receiver, key);

    //    BIND(&if_keyisinvalid);
    //    TailCallRuntime(Runtime::kKeyedHasIC_Miss, context, receiver, key, slot,
    //                    vector);
    //}
    pub fn has_indexed_interceptor_ic(context: &mut CodeAssemblerState) {
        todo!()
    }
}