// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-shadow-realm-gen.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod builtins_shadow_realm_gen {
    use crate::builtins::builtins_utils_gen::*;
    use crate::builtins::builtins::*;
    use crate::codegen::code_stub_assembler_inl::*;
    use crate::objects::descriptor_array::*;
    use crate::objects::js_shadow_realm::*;
    use crate::objects::module::*;
    use crate::codegen::code_stub_assembler::CodeStubAssembler;

    pub struct ShadowRealmBuiltinsAssembler {
        assembler: CodeStubAssembler,
    }

    impl ShadowRealmBuiltinsAssembler {
        pub fn new(state: &mut compiler::CodeAssemblerState) -> Self {
            ShadowRealmBuiltinsAssembler {
                assembler: CodeStubAssembler::new(state),
            }
        }

        pub enum ImportValueFulfilledFunctionContextSlot {
            kEvalContextSlot = Context::MIN_CONTEXT_SLOTS as isize,
            kSpecifierSlot,
            kExportNameSlot,
            kContextLength,
        }

        pub fn allocate_js_wrapped_function(&mut self, context: &TNode<Context>, target: &TNode<Object>) -> TNode<JSObject> {
            let native_context = self.load_native_context(context);
            let map = self.load_context_element::<Map>(&native_context, Context::WRAPPED_FUNCTION_MAP_INDEX as usize);
            let wrapped = self.assembler.allocate_js_object_from_map(&map);
            self.assembler.store_object_field_no_write_barrier(&wrapped, JSWrappedFunction::kWrappedTargetFunctionOffset as usize, target);
            self.assembler.store_object_field_no_write_barrier(&wrapped, JSWrappedFunction::kContextOffset as usize, context);
            wrapped
        }

        pub fn create_import_value_fulfilled_function_context(
            &mut self,
            caller_context: &TNode<NativeContext>,
            eval_context: &TNode<NativeContext>,
            specifier: &TNode<String>,
            export_name: &TNode<String>,
        ) -> TNode<Context> {
            let context = self.assembler.allocate_synthetic_function_context(
                caller_context,
                ImportValueFulfilledFunctionContextSlot::kContextLength as usize,
            );
            self.assembler.store_context_element_no_write_barrier(
                &context,
                ImportValueFulfilledFunctionContextSlot::kEvalContextSlot as usize,
                eval_context,
            );
            self.assembler.store_context_element_no_write_barrier(
                &context,
                ImportValueFulfilledFunctionContextSlot::kSpecifierSlot as usize,
                specifier,
            );
            self.assembler.store_context_element_no_write_barrier(
                &context,
                ImportValueFulfilledFunctionContextSlot::kExportNameSlot as usize,
                export_name,
            );
            context
        }

        pub fn allocate_import_value_fulfilled_function(
            &mut self,
            caller_context: &TNode<NativeContext>,
            eval_context: &TNode<NativeContext>,
            specifier: &TNode<String>,
            export_name: &TNode<String>,
        ) -> TNode<JSFunction> {
            let function_context = self.create_import_value_fulfilled_function_context(
                caller_context,
                eval_context,
                specifier,
                export_name,
            );
            self.assembler.allocate_root_function_with_context(
                RootIndex::kShadowRealmImportValueFulfilledSharedFun,
                &function_context,
                &{},
            )
        }

        pub fn check_accessor(
            &mut self,
            array: &TNode<DescriptorArray>,
            index: &TNode<IntPtrT>,
            name: &TNode<Name>,
            bailout: &mut Label,
        ) {
            let key = self.load_key_by_descriptor_entry(array, index);
            self.assembler.goto_if_not(self.assembler.tagged_equal(&key, name), bailout);
            let value = self.load_value_by_descriptor_entry(array, index);
            self.assembler.goto_if_not(self.assembler.is_accessor_info(&value.unchecked_cast()), bailout);
        }

        pub fn shadow_realm_throw(
            &mut self,
            context: &TNode<Context>,
            fallback_message: MessageTemplate,
            exception: &TNode<Object>,
        ) {
            let template_index = self.assembler.smi_constant(fallback_message as i32);
            self.assembler.call_runtime(
                Runtime::kShadowRealmThrow,
                context,
                template_index,
                exception,
            );
            self.assembler.unreachable();
        }

        pub fn load_native_context(&self, context: &TNode<Context>) -> TNode<NativeContext> {
            TNode::<NativeContext>{}
        }

        pub fn load_context_element<T>(&self, native_context: &TNode<NativeContext>, index: usize) -> TNode<T> {
            TNode::<T>{}
        }

        pub fn load_key_by_descriptor_entry(&self, array: &TNode<DescriptorArray>, index: &TNode<IntPtrT>) -> TNode<Name> {
            TNode::<Name>{}
        }

        pub fn load_value_by_descriptor_entry(&self, array: &TNode<DescriptorArray>, index: &TNode<IntPtrT>) -> TNode<Object> {
            TNode::<Object>{}
        }

    }

    // https://tc39.es/proposal-shadowrealm/#sec-getwrappedvalue
    pub fn shadow_realm_get_wrapped_value(
        assembler: &mut ShadowRealmBuiltinsAssembler,
    ) -> BuiltinResult {
        let context = assembler.assembler.parameter::<Context>(Descriptor::kContext);
        let creation_context = assembler.assembler.parameter::<Context>(Descriptor::kCreationContext);
        let target_context = assembler.assembler.parameter::<Context>(Descriptor::kTargetContext);
        let value = assembler.assembler.parameter::<Object>(Descriptor::kValue);

        let mut if_primitive = Label::new("if_primitive");
        let mut if_callable = Label::new("if_callable");
        let mut unwrap = Label::new("unwrap");
        let mut wrap = Label::new("wrap");
        let mut slow_wrap = Label::new_deferred("slow_wrap");
        let mut bailout = Label::new_deferred("bailout");

        // 2. Return value.
        assembler.assembler.goto_if(assembler.assembler.tagged_is_smi(&value), &mut if_primitive);
        assembler.assembler.goto_if_not(assembler.assembler.js_any_is_not_primitive(&value.unchecked_cast()), &mut if_primitive);

        // 1. If Type(value) is Object, then
        // 1a. If IsCallable(value) is false, throw a TypeError exception.
        // 1b. Return ? WrappedFunctionCreate(callerRealm, value).
        assembler.assembler.branch(assembler.assembler.is_callable(&value.unchecked_cast()), &mut if_callable, &mut bailout);

        assembler.assembler.bind(&mut if_primitive);
        assembler.assembler.return_(&value);

        assembler.assembler.bind(&mut if_callable);
        let mut target = TVARIABLE::<Object>::new("target");
        target.set(&value);
        // WrappedFunctionCreate
        // https://tc39.es/proposal-shadowrealm/#sec-wrappedfunctioncreate
        assembler.assembler.branch(assembler.assembler.is_js_wrapped_function(&value.unchecked_cast()), &mut unwrap, &mut wrap);

        assembler.assembler.bind(&mut unwrap);
        // The intermediate wrapped functions are not user-visible. And calling a
        // wrapped function won't cause a side effect in the creation realm.
        // Unwrap here to avoid nested unwrapping at the call site.
        let target_wrapped_function: TNode<JSWrappedFunction> = value.unchecked_cast();
        target.set(&assembler.assembler.load_object_field(&target_wrapped_function, JSWrappedFunction::kWrappedTargetFunctionOffset as usize));
        assembler.assembler.goto(&mut wrap);

        assembler.assembler.bind(&mut wrap);
        // Disallow wrapping of slow-mode functions. We need to figure out
        // whether the length and name property are in the original state.
        let map = assembler.assembler.load_map(&target.value().unchecked_cast());
        assembler.assembler.goto_if(assembler.assembler.is_dictionary_map(&map), &mut slow_wrap);

        // Check whether the length and name properties are still present as
        // AccessorInfo objects. If so, their value can be recomputed even if
        // the actual value on the object changes.
        let bit_field3 = assembler.assembler.load_map_bit_field3(&map);
        let number_of_own_descriptors = assembler.assembler.signed_(
            assembler.assembler.decode_word_from_word32::<Map::Bits3::NumberOfOwnDescriptorsBits>(&bit_field3),
        );
        assembler.assembler.goto_if(assembler.assembler.intptr_less_than(
            &number_of_own_descriptors,
            &assembler.assembler.intptr_constant(JSFunction::kMinDescriptorsForFastBindAndWrap as i64),
        ), &mut slow_wrap);

        // We don't need to check the exact accessor here because the only case
        // custom accessor arise is with function templates via API, and in that
        // case the object is in dictionary mode
        let descriptors = assembler.assembler.load_map_instance_descriptors(&map);
        assembler.check_accessor(
            &descriptors,
            &assembler.assembler.intptr_constant(
                JSFunctionOrBoundFunctionOrWrappedFunction::kLengthDescriptorIndex as i64,
            ),
            &assembler.assembler.length_string_constant(),
            &mut slow_wrap,
        );
        assembler.check_accessor(
            &descriptors,
            &assembler.assembler.intptr_constant(
                JSFunctionOrBoundFunctionOrWrappedFunction::kNameDescriptorIndex as i64,
            ),
            &assembler.assembler.name_string_constant(),
            &mut slow_wrap,
        );

        // Verify that prototype matches the function prototype of the target
        // context.
        let prototype = assembler.assembler.load_map_prototype(&map);
        let function_map = assembler.load_context_element::<Object>(&target_context, Context::WRAPPED_FUNCTION_MAP_INDEX as usize);
        let function_prototype = assembler.assembler.load_map_prototype(&function_map.unchecked_cast());
        assembler.assembler.goto_if(assembler.assembler.tagged_not_equal(&prototype, &function_prototype), &mut slow_wrap);

        // 1. Let internalSlotsList be the internal slots listed in Table 2, plus
        // [[Prototype]] and [[Extensible]].
        // 2. Let wrapped be ! MakeBasicObject(internalSlotsList).
        // 3. Set wrapped.[[Prototype]] to
        // callerRealm.[[Intrinsics]].[[%Function.prototype%]].
        // 4. Set wrapped.[[Call]] as described in 2.1.
        // 5. Set wrapped.[[WrappedTargetFunction]] to Target.
        // 6. Set wrapped.[[Realm]] to callerRealm.
        // 7. Let result be CopyNameAndLength(wrapped, Target, "wrapped").
        // 8. If result is an Abrupt Completion, throw a TypeError exception.
        // Installed with default accessors.
        let wrapped = assembler.allocate_js_wrapped_function(&creation_context, &target.value());

        // 9. Return wrapped.
        assembler.assembler.return_(&wrapped);

        assembler.assembler.bind(&mut slow_wrap);
        {
            let result = assembler.assembler.call_runtime(
                Runtime::kShadowRealmWrappedFunctionCreate,
                &context,
                &creation_context,
                &target.value(),
            );
            assembler.assembler.return_(&result);
        }

        assembler.assembler.bind(&mut bailout);
        assembler.assembler.throw_type_error(&context, MessageTemplate::kNotCallable, &value);
    }

    // https://tc39.es/proposal-shadowrealm/#sec-wrapped-function-exotic-objects-call-thisargument-argumentslist
    pub fn call_wrapped_function(
        assembler: &mut ShadowRealmBuiltinsAssembler,
    ) -> BuiltinResult {
        let argc = assembler.assembler.unchecked_parameter::<Int32T>(Descriptor::kActualArgumentsCount);
        let argc_ptr = assembler.assembler.change_int32_to_intptr(&argc);
        let wrapped_function = assembler.assembler.parameter::<JSWrappedFunction>(Descriptor::kFunction);
        let context = assembler.assembler.parameter::<Context>(Descriptor::kContext);

        assembler.assembler.perform_stack_check(&context);

        let mut call_exception = Label::new_deferred("call_exception");
        let mut target_not_callable = Label::new_deferred("target_not_callable");

        // 1. Let target be F.[[WrappedTargetFunction]].
        let target: TNode<JSReceiver> = assembler.assembler.load_object_field::<JSReceiver>(
            &wrapped_function,
            JSWrappedFunction::kWrappedTargetFunctionOffset as usize,
        );
        // 2. Assert: IsCallable(target) is true.
        assembler.assembler.csa_dcheck(assembler.assembler.is_callable(&target));

        // 4. Let callerRealm be ? GetFunctionRealm(F).
        let caller_context = assembler.assembler.load_object_field::<Context>(
            &wrapped_function,
            JSWrappedFunction::kContextOffset as usize,
        );
        // 3. Let targetRealm be ? GetFunctionRealm(target).
        let target_context = assembler.get_function_realm(
            &caller_context,
            &target,
            &mut target_not_callable,
        );
        // 5. NOTE: Any exception objects produced after this point are associated
        // with callerRealm.

        let args = CodeStubArguments::new(&mut assembler.assembler, &argc_ptr);
        let receiver = args.get_receiver();

        // 6. Let wrappedArgs be a new empty List.
        let wrapped_args: TNode<FixedArray> = assembler.assembler.allocate_fixed_array(
            ElementsKind::PACKED_ELEMENTS,
            &argc_ptr,
        );
        // Fill the fixed array so that heap verifier doesn't complain about it.
        assembler.fill_fixed_array_with_value(
            ElementsKind::PACKED_ELEMENTS,
            &wrapped_args,
            &assembler.assembler.intptr_constant(0),
            &argc_ptr,
            RootIndex::kUndefinedValue,
        );

        // 8. Let wrappedThisArgument to ? GetWrappedValue(targetRealm, thisArgument).
        // Create wrapped value in the target realm.
        let wrapped_receiver = assembler.assembler.call_builtin(
            Builtin::kShadowRealmGetWrappedValue,
            &caller_context,
            &target_context,
            &caller_context,
            &receiver,
        );
        assembler.store_fixed_array_element(&wrapped_args, 0, &wrapped_receiver);
        // 7. For each element arg of argumentsList, do
        assembler.build_fast_loop::<IntPtrT>(
            &assembler.assembler.intptr_constant(0),
            args.get_length_without_receiver(),
            &mut |index: &TNode<IntPtrT>| {
                // 7a. Let wrappedValue be ? GetWrappedValue(targetRealm, arg).
                // Create wrapped value in the target realm.
                let wrapped_value = assembler.assembler.call_builtin(
                    Builtin::kShadowRealmGetWrappedValue,
                    &caller_context,
                    &target_context,
                    &caller_context,
                    &args.at_index(index),
                );
                // 7b. Append wrappedValue to wrappedArgs.
                assembler.store_fixed_array_element(
                    &wrapped_args,
                    &assembler.assembler.intptr_add(index, &assembler.assembler.intptr_constant(1)),
                    &wrapped_value,
                );
            },
            1,
            LoopUnrollingMode::kNo,
            IndexAdvanceMode::kPost,
        );

        let mut var_exception = TVARIABLE::<Object>::new("var_exception");
        let result: TNode<Object>;
        {
            let handler = compiler::ScopedExceptionHandler::new(
                &mut assembler.assembler,
                &mut call_exception,
                &mut var_exception,
            );
            let args_count = assembler.assembler.int32_constant(0); // args already on the stack

            // 9. Let result be the Completion Record of Call(target,
            // wrappedThisArgument, wrappedArgs).
            result = assembler.assembler.call_builtin(
                Builtin::kCallVarargs,
                &target_context,
                &target,
                &args_count,
                &argc,
                &wrapped_args,
            );
        }

        // 10. If result.[[Type]] is normal or result.[[Type]] is return, then
        // 10a. Return ? GetWrappedValue(callerRealm, result.[[Value]]).
        let wrapped_result: TNode<JSAny> = assembler.assembler.call_builtin(
            Builtin::kShadowRealmGetWrappedValue,
            &caller_context,
            &caller_context,
            &target_context,
            &result,
        );
        args.pop_and_return(&wrapped_result);

        // 11. Else,
        assembler.assembler.bind(&mut call_exception);
        // 11a. Throw a TypeError exception.
        assembler.shadow_realm_throw(
            &context,
            MessageTemplate::kCallWrappedFunctionThrew,
            &var_exception.value(),
        );

        assembler.assembler.bind(&mut target_not_callable);
        // A wrapped value should not be non-callable.
        assembler.assembler.unreachable();
    }

    // https://tc39.es/proposal-shadowrealm/#sec-shadowrealm.prototype.importvalue
    pub fn shadow_realm_prototype_import_value(
        assembler: &mut ShadowRealmBuiltinsAssembler,
    ) -> BuiltinResult {
        let k_method_name = "ShadowRealm.prototype.importValue";
        let context = assembler.assembler.parameter::<Context>(Descriptor::kContext);
        // 1. Let O be this value.
        let o = assembler.assembler.parameter::<Object>(Descriptor::kReceiver);
        // 2. Perform ? ValidateShadowRealmObject(O).
        assembler.throw_if_not_instance_type(
            &context,
            &o,
            InstanceType::JS_SHADOW_REALM_TYPE,
            k_method_name,
        );

        // 3. Let specifierString be ? ToString(specifier).
        let specifier = assembler.assembler.parameter::<Object>(Descriptor::kSpecifier);
        let specifier_string = assembler.to_string_inline(&context, &specifier);
        // 4. Let exportNameString be ? ToString(exportName).
        let export_name = assembler.assembler.parameter::<Object>(Descriptor::kExportName);
        let export_name_string = assembler.to_string_inline(&context, &export_name);
        // 5. Let callerRealm be the current Realm Record.
        let caller_context = assembler.load_native_context(&context);
        // 6. Let evalRealm be O.[[ShadowRealm]].
        // 7. Let evalContext be O.[[ExecutionContext]].
        let eval_context: TNode<NativeContext> = assembler.assembler.load_object_field::<NativeContext>(
            &o.unchecked_cast(),
            JSShadowRealm::kNativeContextOffset as usize,
        );
        // 8. Return ? ShadowRealmImportValue(specifierString, exportNameString,
        // callerRealm, evalRealm, evalContext).
        let result = assembler.import_value(
            &caller_context,
            &eval_context,
            &specifier_string,
            &export_name_string,
        );
        assembler.assembler.return_(&result);
    }

    // https://tc39.es/proposal-shadowrealm/#sec-shadowrealmimportvalue
    impl ShadowRealmBuiltinsAssembler {
        pub fn import_value(
            &mut self,
            caller_context: &TNode<NativeContext>,
            eval_context: &TNode<NativeContext>,
            specifier: &TNode<String>,
            export_name: &TNode<String>,
        ) -> TNode<Object> {
            // 1. Assert: evalContext is an execution context associated to a ShadowRealm
            // instance's [[ExecutionContext]].
            // 2. Let innerCapability be ! NewPromiseCapability(%Promise%).
            // 3. Let runningContext be the running execution context.
            // 4. If runningContext is not already suspended, suspend runningContext.
            // 5. Push evalContext onto the execution context stack; evalContext is now
            // the running execution context.
            // 6. Perform ! HostImportModuleDynamically(null, specifierString,
            // innerCapability).
            // 7. Suspend evalContext and remove it from the execution context stack.
            // 8. Resume the context that is now on the top of the execution context stack
            // as the running execution context.
            let inner_capability = assembler.assembler.call_runtime(
                Runtime::kShadowRealmImportValue,
                eval_context,
                specifier,
            );

            // 9. Let steps be the steps of an ExportGetter function as described below.
            // 10. Let onFulfilled be ! CreateBuiltinFunction(steps, 1, "", «
            // [[ExportNameString]] », callerRealm).
            // 11. Set onFulfilled.[[ExportNameString]] to exportNameString.
            let on_fulfilled = self.allocate_import_value_fulfilled_function(
                caller_context,
                eval_context,
                specifier,
                export_name,
            );

            let on_rejected: TNode<JSFunction> =
                self.load_context_element::<JSFunction>(caller_context, Context::SHADOW_REALM_IMPORT_VALUE_REJECTED_INDEX as usize);
            // 12. Let promiseCapability be ! NewPromiseCapability(%Promise%).
            let promise: TNode<JSPromise> = self.new_js_promise(caller_context);
            // 13. Return ! PerformPromiseThen(innerCapability.[[Promise]], onFulfilled,
            // callerRealm.[[Intrinsics]].[[%ThrowTypeError%]], promiseCapability).
            assembler.assembler.call_builtin(
                Builtin::kPerformPromiseThen,
                caller_context,
                &inner_capability,
                &on_fulfilled,
                &on_rejected,
                &promise,
            )
        }

        fn get_function_realm(
            &mut self,
            caller_context: &TNode<Context>,
            target: &TNode<JSReceiver>,
            target_not_callable: &mut Label,
        ) -> TNode<Context> {
             TNode::<Context>{}
        }

        fn fill_fixed_array_with_value(
            &mut self,
            elements_kind: ElementsKind,
            wrapped_args: &TNode<FixedArray>,
            intptr_constant: &TNode<IntPtrT>,
            argc_ptr: &TNode<IntPtrT>,
            k_undefined_value: RootIndex,
        ) {
            
        }

        fn build_fast_loop<T>(
            &mut self,
            intptr_constant: &TNode<IntPtrT>,
            length_without_receiver: &TNode<IntPtrT>,
            f: &mut dyn FnMut(&TNode<IntPtrT>),
            i: i32,
            k_no: LoopUnrollingMode,
            k_post: IndexAdvanceMode,
        ) {
           
        }

        fn store_fixed_array_element(
            &mut self,
            wrapped_args: &TNode<FixedArray>,
            i: i32,
            wrapped_receiver: &TNode<Object>,
        ) {
           
        }

        fn new_js_promise(&mut self, caller_context: &TNode<NativeContext>) -> TNode<JSPromise> {
            TNode::<JSPromise>{}
        }

        fn to_string_inline(
            &mut self,
            context: &TNode<Context>,
            specifier: &TNode<Object>,
        ) -> TNode<String> {
            TNode::<String>{}
        }

        fn throw_if_not_instance_type(
            &mut self,
            context: &TNode<Context>,
            o: &TNode<Object>,
            js_shadow_realm_type: InstanceType,
            k_method_name: &str,
        ) {
           
        }
    }

    // ExportGetter of
    // https://tc39.es/proposal-shadowrealm/#sec-shadowrealmimportvalue
    pub fn shadow_realm_import_value_fulfilled(
        assembler: &mut ShadowRealmBuiltinsAssembler,
    ) -> BuiltinResult {
        // An ExportGetter function is an anonymous built-in function with a
        // [[ExportNameString]] internal slot. When an ExportGetter function is called
        // with argument exports, it performs the following steps:
        // 8. Let realm be f.[[Realm]].
        let context = assembler.assembler.parameter::<Context>(Descriptor::kContext);
        let eval_context: TNode<Context> = assembler.assembler.load_context_element::<Context>(
            &context,
            ImportValueFulfilledFunctionContextSlot::kEvalContextSlot as usize,
        );

        let mut get_export_exception = Label::new_deferred("get_export_exception");

        // 2. Let f be the active function object.
        // 3. Let string be f.[[ExportNameString]].
        // 4. Assert: Type(string) is String.
        let export_name_string: TNode<String> = assembler.assembler.load_context_element::<String>(
            &context,
            ImportValueFulfilledFunctionContextSlot::kExportNameSlot as usize,
        );

        // 1. Assert: exports is a module namespace exotic object.
        let exports = assembler.assembler.parameter::<JSModuleNamespace>(Descriptor::kExports);

        // 5. Let hasOwn be ? HasOwnProperty(exports, string).
        // 6. If hasOwn is false, throw a TypeError exception.
        // 7. Let value be ? Get(exports, string).

        // The only exceptions thrown by Runtime::kGetModuleNamespaceExport are
        // either the export is not found or the module is not initialized.
        let mut var_exception = TVARIABLE::<Object>::new("var_exception");
        let value: TNode<Object>;
        {
            let handler = compiler::ScopedExceptionHandler::new(
                &mut assembler.assembler,
                &mut get_export_exception,
                &mut var_exception,
            );
            value = assembler.assembler.call_runtime(
                Runtime::kGetModuleNamespaceExport,
                &eval_context,
                &exports,
                &export_name_string,
            );
        }

        // 9. Return ? GetWrappedValue(realm, value).
        let caller_context = assembler.load_native_context(&context);
        let wrapped_result = assembler.assembler.call_builtin(
            Builtin::kShadowRealmGetWrappedValue,
            &caller_context,
            &caller_context,
            &eval_context,
            &value,
        );
        assembler.assembler.return_(&wrapped_result);

        assembler.assembler.bind(&mut get_export_exception);
        {
            let specifier_string: TNode<String> = assembler.assembler.load_context_element::<String>(
                &context,
                ImportValueFulfilledFunctionContextSlot::kSpecifierSlot as usize,
            );
            assembler.assembler.throw_type_error(
                &context,
                MessageTemplate::kUnresolvableExport,
                &specifier_string,
                &export_name_string,
            );
        }
    }

    pub fn shadow_realm_import_value_rejected(
        assembler: &mut ShadowRealmBuiltinsAssembler,
    ) -> BuiltinResult {
        let context = assembler.assembler.parameter::<Context>(Descriptor::kContext);
        let exception = assembler.assembler.parameter::<Object>(Descriptor::kException);
        assembler.shadow_realm_throw(
            &context,
            MessageTemplate::kImportShadowRealmRejected,
            &exception,
        );
    }
}
