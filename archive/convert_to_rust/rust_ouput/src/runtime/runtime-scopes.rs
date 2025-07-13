// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-scopes.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct V8 {}
pub struct HandleScope {}
pub struct Isolate {}
pub struct Object {}
pub struct String {}
pub struct Smi {}
pub struct JSFunction {}
pub struct ScopeInfo {}
pub struct Context {}
pub struct FixedArray {}
pub struct JSReceiver {}
pub struct Heap {}
pub struct JSGlobalObject {}
pub struct ScriptContextTable {}
pub struct VariableLookupResult {}
pub struct LookupIterator {}
pub struct Maybe<T> {
    value: Option<T>,
}
impl<T> Maybe<T> {
    pub fn IsNothing(&self) -> bool {
        self.value.is_none()
    }
    pub fn FromJust(self) -> T {
        self.value.unwrap()
    }
    pub fn IsNothing_clone(&self) -> bool{
        self.value.is_none()
    }
    pub fn clone(&self) -> Maybe<T> where T:Clone{
        Maybe{value: self.value.clone()}
    }
}
pub struct PropertyAttributes {}
pub struct ReadOnlyRoots {}
pub struct LookupIteratorState {}
pub struct JSObject {}
pub struct DirectHandle<T> {
    value: T,
}
impl<T> DirectHandle<T> {
    pub fn new(value: T) -> Self {
        DirectHandle { value }
    }
    pub fn clone(&self) -> DirectHandle<T> where T:Clone{
        DirectHandle{value: self.value.clone()}
    }
}
pub struct JSDisposableStackBase {}
pub struct JSAny {}
pub struct SharedFunctionInfo {}
pub struct FeedbackCell {}
pub struct ClosureFeedbackCellArray {}
pub struct Tagged<T> {
    value: T,
}
pub struct DependentCode {}
pub struct Script {}
pub struct FullObjectSlot {}
pub struct LanguageMode {}
pub struct EvalError {}
pub struct JSContextExtensionObject {}
pub struct DirectHandleVector<T> {
    data: Vec<DirectHandle<T>>,
}
impl<T> DirectHandleVector<T> {
    pub fn data(&self) -> &[DirectHandle<T>] {
        &self.data
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn get(&self, index: usize) -> &DirectHandle<T> {
        &self.data[index]
    }
}
pub struct WasmInternalFunction {}
pub struct Operand {}
pub struct Register {}
pub struct Condition {}
pub enum StoreRepresentation {}
pub struct Node {}
pub enum AbortReason {}
pub struct FPUControlRegister {}
pub struct StructDeclaration {}
pub struct Space {}
pub struct CodePointerHandle {}
pub struct Root {}
pub struct RootIndex {}
pub struct Tagged_t {}
pub struct Turboshaft {}
pub struct WasmFrame {}
pub struct WasmModule {}
pub struct BranchHint {}
pub struct JSReceiver {}
pub struct LookupIteratorState {}
pub struct Local<T> {}
pub struct JSObject {}
pub struct RegExpData {}
pub struct Code {}
pub struct UnoptimizedCompileFlags {}
pub struct AllocationSite {}
pub struct TaggedString {}
pub struct WasmInternalFunction {}
pub struct WasmModule {}
pub struct WasmFrame {}
pub struct JsonObject {}
pub struct AstNodeSourceRanges {}
pub struct ZoneObject {}
pub struct Declaration {}
pub struct Cancelable {}
pub struct turboshaft {
    pub struct Graph {}
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowConstAssignError(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    let isolate = unsafe { &mut * (isolate as *mut Isolate)};
    let err = new_type_error(isolate, MessageTemplate::kConstAssign);
    let err = unsafe { std::mem::transmute::<_, *mut Object>(Box::new(err)) };
    err
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowUsingAssignError(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    let isolate = unsafe { &mut * (isolate as *mut Isolate)};
    let err = new_type_error(isolate, MessageTemplate::kUsingAssign);
    let err = unsafe { std::mem::transmute::<_, *mut Object>(Box::new(err)) };
    err
}

enum RedeclarationType {
    kSyntaxError = 0,
    kTypeError = 1,
}

fn throw_redeclaration_error(
    isolate: &mut Isolate,
    name: &DirectHandle<String>,
    redeclaration_type: RedeclarationType,
) -> Tagged<Object> {
    let mut scope = HandleScope {};
    if let RedeclarationType::kSyntaxError = redeclaration_type {
        let err = new_syntax_error(isolate, MessageTemplate::kVarRedeclaration, name);
        return Tagged{value: unsafe { std::mem::transmute::<_, Object>(err) }};
    } else {
        let err = new_type_error(isolate, MessageTemplate::kVarRedeclaration, name);
        return Tagged{value: unsafe { std::mem::transmute::<_, Object>(err) }};
    }
}

fn declare_global(
    isolate: &mut Isolate,
    global: &DirectHandle<JSGlobalObject>,
    name: &DirectHandle<String>,
    value: &DirectHandle<Object>,
    attr: PropertyAttributes,
    is_var: bool,
    redeclaration_type: RedeclarationType,
) -> Tagged<Object> {
    let mut script_contexts = DirectHandle::new(global.value.native_context.script_context_table);
    let mut lookup = VariableLookupResult {};
    if script_contexts.value.Lookup(name, &mut lookup) && IsLexicalVariableMode(lookup.mode) {
        return throw_redeclaration_error(isolate, name, RedeclarationType::kSyntaxError);
    }
    let lookup_config = if !is_var {
        LookupIterator::Configuration::OWN
    } else {
        LookupIterator::Configuration::OWN_SKIP_INTERCEPTOR
    };
    let mut it = LookupIterator::new(isolate, global, name, global, lookup_config);
    let maybe = JSReceiver::GetPropertyAttributes(&mut it);
    if maybe.IsNothing_clone() {
        return ReadOnlyRoots {}.exception();
    }
    if it.IsFound() {
        let old_attributes = maybe.clone().FromJust();
        if is_var {
            return ReadOnlyRoots {}.undefined_value();
        }
        if (old_attributes.dont_delete != 0) {
            assert_eq!(attr.read_only & 0, 0);
            if (old_attributes.read_only != 0) || (old_attributes.dont_enum != 0) || (it.state() == LookupIterator::ACCESSOR) {
                return throw_redeclaration_error(isolate, name, redeclaration_type);
            }
            attr = old_attributes;
        }
        if it.state() == LookupIterator::ACCESSOR {
            it.Delete();
        }
    }
    if !is_var {
        it.Restart();
    }
    match JSObject::DefineOwnPropertyIgnoreAttributes(&mut it, value, attr) {
        Ok(_) => ReadOnlyRoots {}.undefined_value(),
        Err(_) => ReadOnlyRoots {}.exception(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_DeclareModuleExports(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(2, args_length);
    let args = unsafe { &*(args_object as *mut Arguments) };
    let declarations = DirectHandle::new(unsafe { &*(args.at(0) as *mut FixedArray) }.clone());
    let closure = DirectHandle::new(unsafe { &*(args.at(1) as *mut JSFunction) }.clone());

    let closure_feedback_cell_array = if closure.value.has_feedback_vector() {
        DirectHandle::new(closure.value.feedback_vector.closure_feedback_cell_array)
    } else {
        DirectHandle::new(closure.value.closure_feedback_cell_array)
    };
    let isolate = unsafe { &mut *(isolate as *mut Isolate) };
    let context = DirectHandle::new(unsafe { &*(isolate.context as *mut Context) }.clone());
    assert!(context.value.IsModuleContext());
    let exports = DirectHandle::new(unsafe {
        &*(std::mem::transmute::<_, *mut SourceTextModule>(
            context.value.extension as *mut Object,
        ))
        .regular_exports as *mut FixedArray
    }.clone());
    let length = declarations.value.length();
    for i in 0..length {
        let decl = declarations.value.get(i);
        let index;
        let value;
        if decl.IsSmi() {
            index = decl.to_smi().ToInt();
            value = ReadOnlyRoots {}.the_hole_value();
        } else {
            let sfi = DirectHandle::new(unsafe { &*std::mem::transmute::<_, *mut SharedFunctionInfo>(declarations.value.get(i) as *mut Object) }.clone());
            let feedback_index = declarations.value.get(i + 1).to_smi().ToInt();
            i += 1;
            index = declarations.value.get(i + 1).to_smi().ToInt();
            i += 1;
            let feedback_cell = DirectHandle::new(unsafe { &*std::mem::transmute::<_, *mut FeedbackCell>(closure_feedback_cell_array.value.get(feedback_index) as *mut Object) }.clone());
            value = Factory::JSFunctionBuilder(isolate, &sfi, &context)
                .set_feedback_cell(feedback_cell)
                .Build();
        }
        unsafe {
        std::mem::transmute::<_, *mut Cell>(exports.value.get(index - 1) as *mut Object).set_value(value);
        }
    }
    ReadOnlyRoots {}.undefined_value()
}

#[no_mangle]
pub extern "C" fn Runtime_DeclareGlobals(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(2, args_length);

    let args = unsafe { &*(args_object as *mut Arguments) };

    let declarations = DirectHandle::new(unsafe { &*(args.at(0) as *mut FixedArray) }.clone());
    let closure = DirectHandle::new(unsafe { &*(args.at(1) as *mut JSFunction) }.clone());
    let isolate = unsafe { &mut *(isolate as *mut Isolate)};
    let global = DirectHandle::new(unsafe { &*(isolate.global_object as *mut JSGlobalObject) }.clone());
    let context = DirectHandle::new(unsafe { &*(isolate.context as *mut Context) }.clone());
    let closure_feedback_cell_array = if closure.value.has_feedback_vector() {
        DirectHandle::new(closure.value.feedback_vector.closure_feedback_cell_array)
    } else {
        DirectHandle::new(closure.value.closure_feedback_cell_array)
    };
    let length = declarations.value.length();
    for i in 0..length {
        let decl = DirectHandle::new(declarations.value.get(i));
        let name;
        let value;
        let is_var = decl.value.IsString();
        if is_var {
            name = unsafe { std::mem::transmute::<_, DirectHandle<String>>(decl) };
            value = DirectHandle::new(isolate.factory.undefined_value());
        } else {
            let sfi = DirectHandle::new(unsafe { std::mem::transmute::<_, &mut SharedFunctionInfo>(decl.value as *mut Object) }.clone());
            name = DirectHandle::new(sfi.value.Name());
            let index = declarations.value.get(i + 1).to_smi().ToInt();
            i += 1;
            let feedback_cell = DirectHandle::new(unsafe { std::mem::transmute::<_, &mut FeedbackCell>(closure_feedback_cell_array.value.get(index) as *mut Object) }.clone());
            value = DirectHandle::new(Factory::JSFunctionBuilder(isolate, &sfi, &context).set_feedback_cell(feedback_cell).Build());
        }
        let script = unsafe { std::mem::transmute::<_, Script>(closure.value.shared.script) };
        let attr = if script.compilation_type == Script::CompilationType::kEval {
            PropertyAttributes::NONE
        } else {
            PropertyAttributes::DONT_DELETE
        };
        let result = declare_global(isolate, &global, &name, &value, attr, is_var, RedeclarationType::kSyntaxError);
        if result.IsException() {
            return unsafe { std::mem::transmute::<_, *mut Object>(result) };
        }
    }
    ReadOnlyRoots {}.undefined_value()
}

#[no_mangle]
pub extern "C" fn Runtime_InitializeDisposableStack(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(0, args_length);

    let isolate = unsafe { &mut * (isolate as *mut Isolate)};
    let disposable_stack = DirectHandle::new(isolate.factory.NewJSDisposableStackBase());
    JSDisposableStackBase::InitializeJSDisposableStackBase(isolate, &disposable_stack);
    unsafe { std::mem::transmute::<_, *mut Object>(disposable_stack.value) }
}

enum DisposeMethodCallType {
    kValueIsReceiver,
}

enum DisposeMethodHint {
    kSyncDispose,
    kAsyncDispose,
}

fn add_to_disposable_stack(
    isolate: &mut Isolate,
    stack: &DirectHandle<JSDisposableStackBase>,
    value: &DirectHandle<JSAny>,
    method_call_type: DisposeMethodCallType,
    method_hint: DisposeMethodHint,
) -> Result<bool, ()> {
    let mut method: DirectHandle<Object>;
    match JSDisposableStackBase::CheckValueAndGetDisposeMethod(isolate, value, method_hint) {
        Ok(m) => method = m,
        Err(_) => return Err(()),
    }
    JSDisposableStackBase::Add(isolate, stack, value, &method, method_call_type, method_hint);
    Ok(true)
}

#[no_mangle]
pub extern "C" fn Runtime_AddDisposableValue(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(2, args_length);

    let args = unsafe { &*(args_object as *mut Arguments) };

    let stack = DirectHandle::new(unsafe { &*(args.at(0) as *mut JSDisposableStackBase) }.clone());
    let value = DirectHandle::new(unsafe { &*(args.at(1) as *mut JSAny) }.clone());
    if !value.value.IsNullOrUndefined() {
        match add_to_disposable_stack(
            unsafe { &mut * (isolate as *mut Isolate)},
            &stack,
            &value,
            DisposeMethodCallType::kValueIsReceiver,
            DisposeMethodHint::kSyncDispose,
        ) {
            Ok(_) => {}
            Err(_) => return ReadOnlyRoots {}.exception() as *mut Object,
        };
    }
    unsafe { std::mem::transmute::<_, *mut Object>(value.value) }
}

#[no_mangle]
pub extern "C" fn Runtime_AddAsyncDisposableValue(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(2, args_length);

    let args = unsafe { &*(args_object as *mut Arguments) };

    let stack = DirectHandle::new(unsafe { &*(args.at(0) as *mut JSDisposableStackBase) }.clone());
    let value = DirectHandle::new(unsafe { &*(args.at(1) as *mut JSAny) }.clone());
    let value_to_add = if value.value.IsNullOrUndefined() {
        DirectHandle::new(unsafe { &mut *(isolate as *mut Isolate)}.factory.undefined_value())
    } else {
        value
    };
    match add_to_disposable_stack(
        unsafe { &mut * (isolate as *mut Isolate)},
        &stack,
        &value_to_add,
        DisposeMethodCallType::kValueIsReceiver,
        DisposeMethodHint::kAsyncDispose,
    ) {
        Ok(_) => {}
        Err(_) => return ReadOnlyRoots {}.exception() as *mut Object,
    };
    unsafe { std::mem::transmute::<_, *mut Object>(value.value) }
}

#[no_mangle]
pub extern "C" fn Runtime_DisposeDisposableStack(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(5, args_length);

    let args = unsafe { &*(args_object as *mut Arguments) };

    let disposable_stack = DirectHandle::new(unsafe { &*(args.at(0) as *mut JSDisposableStackBase) }.clone());
    let continuation_token = DirectHandle::new(unsafe { &*(args.at(1) as *mut Smi) }.clone());
    let continuation_error = DirectHandle::new(unsafe { &*(args.at(2) as *mut Object) }.clone());
    let continuation_message = DirectHandle::new(unsafe { &*(args.at(3) as *mut Object) }.clone());
    let has_await_using = DirectHandle::new(unsafe { &*(args.at(4) as *mut Smi) }.clone());
    if disposable_stack.value.state() != DisposableStackState::kDisposed && continuation_token.value == Smi::FromInt(TryFinallyContinuationToken::kRethrowToken as i32) {
        unsafe { &mut *disposable_stack.value.error = *continuation_error };
        unsafe { &mut *disposable_stack.value.error_message = *continuation_message };
    }
    let has_await_using_enum_val = unsafe { std::mem::transmute::<_, DisposableStackResourcesType>(has_await_using.value.ToInt() as i32) };
    assert!(!(disposable_stack.value.state() == DisposableStackState::kDisposed) || has_await_using_enum_val == DisposableStackResourcesType::kAtLeastOneAsync);
    disposable_stack.value.set_state(DisposableStackState::kDisposed);
    let isolate = unsafe { &mut * (isolate as *mut Isolate)};
    let result: DirectHandle<Object>;
    match JSDisposableStackBase::DisposeResources(
        isolate,
        &disposable_stack,
        unsafe { std::mem::transmute::<_, DisposableStackResourcesType>(has_await_using.value.ToInt() as i32)},
    ) {
        Ok(res) => result = DirectHandle::new(res),
        Err(_) => return ReadOnlyRoots {}.exception() as *mut Object,
    };
    unsafe { std::mem::transmute::<_, *mut Object>(result.value) }
}

#[no_mangle]
pub extern "C" fn Runtime_HandleExceptionsInDisposeDisposableStack(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(3, args_length);

    let args = unsafe { &*(args_object as *mut Arguments) };

    let disposable_stack = DirectHandle::new(unsafe { &*(args.at(0) as *mut JSDisposableStackBase) }.clone());
    let exception = DirectHandle::new(unsafe { &*(args.at(1) as *mut Object) }.clone());
    let message = DirectHandle::new(unsafe { &*(args.at(2) as *mut Object) }.clone());
    let isolate = unsafe { &mut * (isolate as *mut Isolate)};
    if !isolate.is_catchable_by_javascript(*exception) {
        return isolate.Throw(*exception);
    }
    JSDisposableStackBase::HandleErrorInDisposal(isolate, &disposable_stack, &exception, &message);
    unsafe { std::mem::transmute::<_, *mut Object>(disposable_stack.value) }
}

fn declare_eval_helper(
    isolate: &mut Isolate,
    name: &DirectHandle<String>,
    value: &DirectHandle<Object>,
) -> Tagged<Object> {
    let context = DirectHandle::new(unsafe { &*(isolate.context.declaration_context as *mut Context) }.clone());
    let is_debug_evaluate_in_module = isolate.context.IsDebugEvaluateContext() && context.value.IsModuleContext();
    assert!(context.value.IsFunctionContext() || context.value.IsNativeContext() || context.value.IsScriptContext() || context.value.IsEvalContext() || (context.value.IsBlockContext() && context.value.scope_info.is_declaration_scope()) || is_debug_evaluate_in_module);
    let is_var = value.value.IsUndefined(isolate);
    assert!(!(is_var) || value.value.IsJSFunction());
    let mut index;
    let attributes;
    let init_flag;
    let mode;
    let holder = Context::Lookup(
        &context,
        name,
        DONT_FOLLOW_CHAINS,
        &mut index,
        &mut attributes,
        &mut init_flag,
        &mut mode,
    );
    assert!(holder.is_null() || !holder.IsSourceTextModule());
    assert!(!isolate.has_exception());
    let mut object;
    if attributes != PropertyAttributes::ABSENT && holder.IsJSGlobalObject() {
        return declare_global(isolate, unsafe { &mut *(holder.as_ptr() as *mut JSGlobalObject) }.into(), name, value, PropertyAttributes::NONE, is_var, RedeclarationType::kTypeError);
    }
    if context.value.has_extension() && holder.IsJSGlobalObject() {
        let global = DirectHandle::new(unsafe { &*(context.value.extension as *mut JSGlobalObject) }.clone());
        return declare_global(isolate, &global, name, value, PropertyAttributes::NONE, is_var, RedeclarationType::kTypeError);
    } else if context.value.IsScriptContext() {
        assert!(holder.IsJSGlobalObject());
        let global = DirectHandle::new(unsafe { &*(context.value.global_object as *mut JSGlobalObject) }.clone());
        return declare_global(isolate, &global, name, value, PropertyAttributes::NONE, is_var, RedeclarationType::kTypeError);
    }
    if attributes != PropertyAttributes::ABSENT {
        assert_eq!(PropertyAttributes::NONE, attributes);
        if is_var {
            return ReadOnlyRoots {}.undefined_value();
        }
        if index != Context::kNotFound {
            assert!(holder.is_identical_to(context));
            unsafe{ std::mem::transmute::<_, *mut Context>(holder.as_ptr()).set(index, *value) };
            return ReadOnlyRoots {}.undefined_value();
        }
        object = unsafe { std::mem::transmute::<_, JSObject>(holder.as_ptr()) };
    } else if context.value.has_extension() && !is_debug_evaluate_in_module {
        object = DirectHandle::new(unsafe { std::mem::transmute::<_, JSContextExtensionObject>(context.value.extension_object as *mut Object) });
        assert!(object.value.IsJSContextExtensionObject());
    } else if context.value.scope_info.HasContextExtensionSlot() && !is_debug_evaluate_in_module {
        assert!(context.value.IsBlockContext() && context.value.scope_info.is_declaration_scope() || context.value.IsFunctionContext());
        assert!(context.value.scope_info.SloppyEvalCanExtendVars());
        object = DirectHandle::new(isolate.factory.NewJSObject(isolate.context_extension_function));
        unsafe{ std::mem::transmute::<_, *mut Context>(holder.as_ptr()).set_extension(*object) };
        let scope_info = context.value.scope_info;
        if !scope_info.SomeContextHasExtension() {
            scope_info.mark_some_context_has_extension();
            DependentCode::DeoptimizeDependencyGroups(
                isolate,
                &scope_info,
                DependentCode::kEmptyContextExtensionGroup,
            );
        }
    } else {
        let err = new_eval_error(isolate, MessageTemplate::kVarNotAllowedInEvalScope, name);
        unsafe{ std::mem::transmute::<_, *mut Object>(err) };
        return ReadOnlyRoots {}.exception();
    }
    match JSObject::SetOwnPropertyIgnoreAttributes(&object, name, value, PropertyAttributes::NONE) {
        Ok(_) => ReadOnlyRoots {}.undefined_value(),
        Err(_) => ReadOnlyRoots {}.exception(),
    }
}

#[no_mangle]
pub extern "C" fn Runtime_DeclareEvalFunction(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(2, args_length);

    let args = unsafe { &*(args_object as *mut Arguments) };

    let name = DirectHandle::new(unsafe { &*(args.at(0) as *mut String) }.clone());
    let value = DirectHandle::new(unsafe { &*(args.at(1) as *mut Object) }.clone());
    let isolate = unsafe { &mut * (isolate as *mut Isolate)};
    unsafe { std::mem::transmute::<_, *mut Object>(declare_eval_helper(isolate, &name, &value)) }
}

#[no_mangle]
pub extern "C" fn Runtime_DeclareEvalVar(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(1, args_length);
    let args = unsafe { &*(args_object as *mut Arguments) };
    let name = DirectHandle::new(unsafe { &*(args.at(0) as *mut String) }.clone());
    let isolate = unsafe { &mut * (isolate as *mut Isolate)};
    let value = DirectHandle::new(isolate.factory.undefined_value());
    unsafe { std::mem::transmute::<_, *mut Object>(declare_eval_helper(isolate, &name, &value)) }
}

fn get_caller_arguments(isolate: &mut Isolate) -> DirectHandleVector<Object> {
    let mut it = JavaScriptStackFrameIterator::new(isolate);
    let frame = it.frame();
    let functions = frame.GetFunctions();
    if functions.len() > 1 {
        let inlined_jsframe_index = functions.len() as i32 - 1;
        let translated_values = TranslatedState::new(frame);
        translated_values.Prepare(frame.fp());
        let mut argument_count = 0;
        let translated_frame = translated_values.GetArgumentsInfoFromJSFrameIndex(inlined_jsframe_index, &mut argument_count);
        let mut iter = translated_frame.begin();
        iter += 1;
        iter += 1;
        argument_count -= 1;
        let mut param_data = DirectHandleVector::new(isolate, argument_count);
        let mut should_deoptimize = false;
        for i in 0..argument_count {
            should_deoptimize = should_deoptimize || iter.IsMaterializedObject();
            let value = iter.GetValue();
            param_data.data[i] = value;
            iter += 1;
        }
        if should_deoptimize {
            translated_values.StoreMaterializedValuesAndDeopt(frame);
        }
        return param_data;
    } else {
        let args_count = frame.GetActualArgumentCount();
        let mut param_data = DirectHandleVector::new(isolate, args_count);
        for i in 0..args_count {
            let val = DirectHandle::new(frame.GetParameter(i));
            param_data.data[i] = val;
        }
        return param_data;
    }
}

fn new_sloppy_arguments(
    isolate: &mut Isolate,
    callee: &DirectHandle<JSFunction>,
    parameters: &HandleArguments,
    argument_count: i32,
) -> DirectHandle<JSObject> {
    assert!(!callee.value.shared.IsDerivedConstructor());
    assert!(callee.value.shared.has_simple_parameters());
    let result = DirectHandle::new(isolate.factory.NewArgumentsObject(callee, argument_count));
    let parameter_count = callee.value.shared.internal_formal_parameter_count_without_receiver();
    if argument_count > 0 {
        if parameter_count > 0 {
            let mapped_count = std::cmp::min(argument_count, parameter_count);
            let context = DirectHandle::new(isolate.context.clone());
            let arguments = DirectHandle::new(isolate.factory.NewFixedArray(argument_count, AllocationType::kYoung));
            let parameter_map = DirectHandle::new(isolate.factory.NewSloppyArgumentsElements(mapped_count, &context, &arguments, AllocationType::kYoung));
            unsafe{std::mem::transmute::<_,*mut JSObject>(result.value.set_map(isolate, &isolate.native_context.fast_aliased_arguments_map))};
            unsafe{std::mem::transmute::<_,*mut JSObject>(result.value.set_elements(*parameter_map))};
            let mut index = argument_count - 1;
            while index >= mapped_count {
                arguments.value.set(index as usize, parameters[index as usize]);
                index -= 1;
            }
            let scope_info = DirectHandle::new(callee.value.shared.scope_info.clone());
            let roots = ReadOnlyRoots {};
            for i in 0..mapped_count {
                arguments.value.set(i as usize, parameters[i as usize]);
                unsafe{parameter_map.value.set_mapped_entries(i as usize, isolate.factory.the_hole_value())};
            }
            for i in 0..scope_info.value.ContextLocalCount() {
                if !scope_info.value.ContextLocalIsParameter(i) {
                    continue;
                }
                let parameter = scope_info.value.ContextLocalParameterNumber(i);
                if parameter >= mapped_count {
                    continue;
                }
                arguments.value.set_the_hole(&roots, parameter as usize);
                let slot = Smi::FromInt(scope_info.value.ContextHeaderLength() + i);
                unsafe{parameter_map.value.set_mapped_entries(parameter as usize, slot)};
            }
        } else {
            let elements = DirectHandle::new(isolate.factory.NewFixedArray(argument_count, AllocationType::kYoung));
            unsafe{std::mem::transmute::<_,*mut JSObject>(result.value.set_elements(*elements))};
            for i in 0..argument_count {
                elements.value.set(i as usize, parameters[i as usize]);
            }
        }
    }
    result
}

struct HandleArguments {
    array: Vec<Tagged<Object>>,
}
impl HandleArguments {
    fn new(array: Vec<Tagged<Object>>) -> Self {
        HandleArguments { array }
    }
    fn get(&self, index: usize) -> &Tagged<Object> {
        &self.array[index]
    }
}

impl std::ops::Index<usize> for HandleArguments {
    type Output = Tagged<Object>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

struct ParameterArguments {
    parameters: *mut Object,
}
impl ParameterArguments {
    fn new(parameters: *mut Object) -> Self {
        ParameterArguments { parameters }
    }
    fn get(&self, index: usize) -> &Tagged<Object> {
        unsafe { &*((self.parameters as usize - (index + 1) * std::mem::size_of::<*mut Object>()) as *mut Tagged<Object>) }
    }
}

impl std::ops::Index<usize> for ParameterArguments {
    type Output = Tagged<Object>;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*((self.parameters as usize - (index + 1) * std::mem::size_of::<*mut Object>()) as *mut Tagged<Object>) }
    }
}

#[no_mangle]
pub extern "C" fn Runtime_NewSloppyArguments(
    args_length: i32,
    args_object: *mut Object,
    isolate: *mut Isolate,
) -> *mut Object {
    let mut scope = HandleScope {};
    assert_eq!(1, args_length);

    let args = unsafe { &*(args_object as *mut Arguments) };

    
