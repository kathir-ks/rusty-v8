// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This is a simplified translation, and some parts might need
// further adaptation based on the full context of the V8 engine.

use std::option::Option;

// Placeholder types and modules.  These would need to be defined
// according to the actual V8 codebase.
mod codegen {
    pub mod code_stub_assembler;
}

mod compiler {
    pub mod code_assembler {
        pub type TVariable<T> = std::cell::RefCell<T>;
    }
    pub struct CodeAssemblerState {}
}

mod objects {
    pub mod dictionary;
}

use codegen::code_stub_assembler::CodeStubAssembler;
use compiler::code_assembler::TVariable;
use objects::dictionary::PropertyDictionary;

pub struct ExitPoint<'a> {
    asm: &'a CodeStubAssembler<'a>,
    indirect_return_handler: Option<Box<dyn Fn(TNode<Object>) + 'a>>,
}

impl<'a> ExitPoint<'a> {
    pub fn new(asm: &'a CodeStubAssembler<'a>) -> Self {
        ExitPoint {
            asm,
            indirect_return_handler: None,
        }
    }

    pub fn new_with_handler<F>(asm: &'a CodeStubAssembler<'a>, indirect_return_handler: F) -> Self
    where
        F: Fn(TNode<Object>) + 'a,
        F: 'static,
    {
        ExitPoint {
            asm,
            indirect_return_handler: Some(Box::new(indirect_return_handler)),
        }
    }

    pub fn new_with_label(
        asm: &'a CodeStubAssembler<'a>,
        out: *mut compiler::code_assembler::TVariable<Object>,
    ) -> Self {
        // TODO: Proper handling of mutable raw pointer `out`.
        ExitPoint {
            asm,
            indirect_return_handler: None, //Some(Box::new(move |result: TNode<Object>| {
                                            //   unsafe { *out.as_mut().unwrap() = result; }
                                            //   asm.goto(todo!("Implement Goto"))
                                            // })),
        }
    }

    pub fn return_call_runtime<TArgs>(&self, function: RuntimeFunctionId, context: TNode<Context>, args: TArgs)
    where
        TArgs: std::fmt::Debug,
    {
        if self.is_direct() {
             self.asm.tail_call_runtime(function, context, args);
        } else {
            if let Some(handler) = &self.indirect_return_handler {
                let result = self.asm.call_runtime(function, context, args);
                handler(result);
            }
        }
    }

    pub fn return_call_builtin<TArgs>(&self, builtin: Builtin, context: TNode<Context>, args: TArgs)
    where
        TArgs: std::fmt::Debug,
    {
        if self.is_direct() {
             self.asm.tail_call_builtin(builtin, context, args);
        } else {
            if let Some(handler) = &self.indirect_return_handler {
                let result = self.asm.call_builtin(builtin, context, args);
                handler(result);
            }
        }
    }

    pub fn return_call_stub<TArgs>(
        &self,
        descriptor: CallInterfaceDescriptor,
        target: TNode<Code>,
        context: TNode<Context>,
        args: TArgs,
    ) where
        TArgs: std::fmt::Debug,
    {
        if self.is_direct() {
             self.asm.tail_call_stub(descriptor, target, context, args);
        } else {
            if let Some(handler) = &self.indirect_return_handler {
                let result = self.asm.call_stub(descriptor, target, context, args);
                handler(result);
            }
        }
    }

    pub fn return_value(&self, result: TNode<Object>) {
        if self.is_direct() {
             self.asm.return_value(result);
        } else {
            if let Some(handler) = &self.indirect_return_handler {
                handler(result);
            }
        }
    }

    pub fn is_direct(&self) -> bool {
        self.indirect_return_handler.is_none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeofMode {
    Normal,
    Boolean,
    Number,
    String,
    Symbol,
    BigInt,
    Object,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextKind {
    Any,
    Block,
    Catch,
    Eval,
    Function,
    Module,
    Script,
    With,
}

// Placeholder types
#[derive(Debug, Clone, Copy)]
pub struct TNode<T>(std::marker::PhantomData<T>);

impl<T> TNode<T> {
    pub fn new() -> Self {
        TNode(std::marker::PhantomData)
    }
}

// Define dummy structs/enums to represent V8 types
#[derive(Debug, Clone, Copy)]
pub struct JSAny;
#[derive(Debug, Clone, Copy)]
pub struct Object;
#[derive(Debug, Clone, Copy)]
pub struct HeapObject;
#[derive(Debug, Clone, Copy)]
pub struct Name;
#[derive(Debug, Clone, Copy)]
pub struct TaggedIndex;
#[derive(Debug, Clone, Copy)]
pub struct Context;
#[derive(Debug, Clone, Copy)]
pub struct Smi;
#[derive(Debug, Clone, Copy)]
pub struct Map;
#[derive(Debug, Clone, Copy)]
pub struct IntPtrT;
#[derive(Debug, Clone, Copy)]
pub struct MaybeObject;
#[derive(Debug, Clone, Copy)]
pub struct Uint32T;
#[derive(Debug, Clone, Copy)]
pub struct PropertyCell;
#[derive(Debug, Clone, Copy)]
pub struct Float64T;
#[derive(Debug, Clone, Copy)]
pub struct DescriptorArray;
#[derive(Debug, Clone, Copy)]
pub struct Word32T;
#[derive(Debug, Clone, Copy)]
pub struct BoolT;
#[derive(Debug, Clone, Copy)]
pub struct JSObject;
#[derive(Debug, Clone, Copy)]
pub struct SharedStruct;
#[derive(Debug, Clone, Copy)]
pub struct FunctionTemplateInfo;
#[derive(Debug, Clone, Copy)]
pub struct DataHandler;
#[derive(Debug, Clone, Copy)]
pub struct Int32T;
#[derive(Debug, Clone, Copy)]
pub struct WasmObject;
#[derive(Debug, Clone, Copy)]
pub struct JSProxy;
#[derive(Debug, Clone, Copy)]
pub struct PropertyArray;
#[derive(Debug, Clone, Copy)]
pub struct FixedArrayBase;
#[derive(Debug, Clone, Copy)]
pub struct Code;
#[derive(Debug, Clone, Copy)]
pub struct FeedbackVector;
#[derive(Debug, Clone, Copy)]
pub struct HeapObjectReference;
#[derive(Debug, Clone, Copy)]
pub struct WeakFixedArray;
#[derive(Debug, Clone, Copy)]
pub struct JSAnyNotSmi;
#[derive(Debug, Clone, Copy)]
pub struct NativeContext;
#[derive(Debug, Clone, Copy)]
pub struct StoreHandler;

// Dummy Enums
#[derive(Debug, Clone, Copy)]
pub enum Builtin {}
#[derive(Debug, Clone, Copy)]
pub enum RuntimeFunctionId {}
#[derive(Debug, Clone, Copy)]
pub struct CallInterfaceDescriptor {}
#[derive(Debug, Clone, Copy)]
pub enum Representation {}
#[derive(Debug, Clone, Copy)]
pub enum OnNonExistent {
    kReturnUndefined,
    kThrow,
}

// Dummy macro replacements for feature flags.
const V8_ENABLE_WEBASSEMBLY: bool = false;

#[derive(Debug, Clone, Copy)]
pub struct LazyNode<T> {
    // Using Box<dyn Fn() -> T> to represent the lazy evaluation.
    // Using a function pointer instead of a closure that captures anything by value as the closure may be used only at later stages.
    thunk: fn() -> T,
}

impl<T> LazyNode<T> {
    pub fn new(thunk: fn() -> T) -> Self {
        LazyNode { thunk }
    }

    pub fn resolve(&self) -> T {
        (self.thunk)()
    }
}

impl<T> std::ops::Deref for LazyNode<T> {
    type Target = fn() -> T;

    fn deref(&self) -> &Self::Target {
        &self.thunk
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StubCache;

impl StubCache {
    pub fn primary_offset(name: TNode<Name>, map: TNode<Map>) -> TNode<IntPtrT> {
        TNode::new()
    }
    pub fn secondary_offset(name: TNode<Name>, map: TNode<Map>) -> TNode<IntPtrT> {
        TNode::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Isolate;

impl Isolate {
    pub fn define_own_stub_cache(&self) -> &StubCache {
        unimplemented!()
    }

    pub fn store_stub_cache(&self) -> &StubCache {
        unimplemented!()
    }
}

pub struct AccessorAssembler<'a> {
    code_stub_assembler: CodeStubAssembler<'a>,
}

impl<'a> AccessorAssembler<'a> {
    pub fn new(state: &'a compiler::CodeAssemblerState) -> Self {
        AccessorAssembler {
            code_stub_assembler: CodeStubAssembler::new(state),
        }
    }

    pub fn generate_load_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_ic_megamorphic(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_ic_noninlined(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_ic_no_feedback(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_global_ic_no_feedback(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_ic_trampoline(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_ic_trampoline_megamorphic(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_super_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_super_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_load_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_enumerated_keyed_load_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_load_ic_megamorphic(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_load_ic_polymorphic_name(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_load_ic_trampoline(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_load_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_enumerated_keyed_load_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_load_ic_trampoline_megamorphic(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_ic_megamorphic(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_ic_trampoline(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_ic_trampoline_megamorphic(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_define_named_own_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_define_named_own_ic_trampoline(&mut self) {
        unimplemented!()
    }

    pub fn generate_define_named_own_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_global_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_global_ic_trampoline(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_global_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_clone_object_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_clone_object_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_clone_object_ic_slow(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_has_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_has_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_has_ic_megamorphic(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_has_ic_polymorphic_name(&mut self) {
        unimplemented!()
    }

    pub fn generate_load_global_ic(&mut self, typeof_mode: TypeofMode) {
        unimplemented!()
    }

    pub fn generate_load_global_ic_trampoline(&mut self, typeof_mode: TypeofMode) {
        unimplemented!()
    }

    pub fn generate_load_global_ic_baseline(&mut self, typeof_mode: TypeofMode) {
        unimplemented!()
    }

    pub fn generate_lookup_global_ic(&mut self, typeof_mode: TypeofMode) {
        unimplemented!()
    }

    pub fn generate_lookup_global_ic_trampoline(&mut self, typeof_mode: TypeofMode) {
        unimplemented!()
    }

    pub fn generate_lookup_global_ic_baseline(&mut self, typeof_mode: TypeofMode) {
        unimplemented!()
    }

    pub fn generate_lookup_context_trampoline(&mut self, typeof_mode: TypeofMode, context_kind: ContextKind) {
        unimplemented!()
    }

    pub fn generate_lookup_context_baseline(&mut self, typeof_mode: TypeofMode, context_kind: ContextKind) {
        unimplemented!()
    }

    pub fn generate_keyed_store_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_store_ic_trampoline(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_store_ic_trampoline_megamorphic(&mut self) {
        unimplemented!()
    }

    pub fn generate_keyed_store_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_define_keyed_own_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_define_keyed_own_ic_trampoline(&mut self) {
        unimplemented!()
    }

    pub fn generate_define_keyed_own_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_in_array_literal_ic(&mut self) {
        unimplemented!()
    }

    pub fn generate_store_in_array_literal_ic_baseline(&mut self) {
        unimplemented!()
    }

    pub fn try_probe_stub_cache(
        &mut self,
        stub_cache: &StubCache,
        lookup_start_object: TNode<JSAny>,
        lookup_start_object_map: TNode<Map>,
        name: TNode<Name>,
        if_handler: &mut Label,
        var_handler: &mut TVariable<MaybeObject>,
        if_miss: &mut Label,
    ) {
        unimplemented!()
    }

    pub fn stub_cache_primary_offset_for_testing(&mut self, name: TNode<Name>, map: TNode<Map>) -> TNode<IntPtrT> {
        StubCache::primary_offset(name, map)
    }
    pub fn stub_cache_secondary_offset_for_testing(&mut self, name: TNode<Name>, map: TNode<Map>) -> TNode<IntPtrT> {
        StubCache::secondary_offset(name, map)
    }

    pub fn load_global_ic(
        &mut self,
        maybe_feedback_vector: TNode<HeapObject>,
        lazy_slot: &LazyNode<TaggedIndex>,
        lazy_context: &LazyNode<Context>,
        lazy_name: &LazyNode<Name>,
        typeof_mode: TypeofMode,
        exit_point: &mut ExitPoint,
    ) {
        unimplemented!()
    }

    pub fn load_ic_bytecode_handler(&mut self, p: &LazyLoadICParameters, exit_point: &mut ExitPoint) {
        unimplemented!()
    }

    pub fn load_handler_data_field(&self, handler: TNode<DataHandler>, data_index: i32) -> TNode<MaybeObject> {
        unimplemented!()
    }

    fn load_ic(&mut self, p: &LoadICParameters) {
        unimplemented!()
    }

    fn load_ic_noninlined(
        &mut self,
        p: &LoadICParameters,
        lookup_start_object_map: TNode<Map>,
        feedback: TNode<HeapObject>,
        var_handler: &mut TVariable<MaybeObject>,
        if_handler: &mut Label,
        miss: &mut Label,
        exit_point: &mut ExitPoint,
    ) {
        unimplemented!()
    }

    fn load_super_ic(&mut self, p: &LoadICParameters) {
        unimplemented!()
    }

    fn load_descriptor_value(&mut self, map: TNode<Map>, descriptor_entry: TNode<IntPtrT>) -> TNode<Object> {
        unimplemented!()
    }
    fn load_descriptor_value_or_field_type(
        &mut self,
        map: TNode<Map>,
        descriptor_entry: TNode<IntPtrT>,
    ) -> TNode<MaybeObject> {
        unimplemented!()
    }

    fn load_ic_no_feedback(&mut self, p: &LoadICParameters, smi_typeof_mode: TNode<Smi>) {
        unimplemented!()
    }
    fn load_super_ic_no_feedback(&mut self, p: &LoadICParameters) {
        unimplemented!()
    }
    fn load_global_ic_no_feedback(&mut self, context: TNode<Context>, name: TNode<Object>, smi_typeof_mode: TNode<Smi>) {
        unimplemented!()
    }

    fn keyed_load_ic(&mut self, p: &LoadICParameters, access_mode: LoadAccessMode) {
        unimplemented!()
    }
    fn keyed_load_ic_generic(&mut self, p: &LoadICParameters) {
        unimplemented!()
    }
    fn keyed_load_ic_polymorphic_name(&mut self, p: &LoadICParameters, access_mode: LoadAccessMode) {
        unimplemented!()
    }

    fn store_ic(&mut self, p: &StoreICParameters) {
        unimplemented!()
    }
    fn store_global_ic(&mut self, p: &StoreICParameters) {
        unimplemented!()
    }
    fn store_global_ic_property_cell_case(
        &mut self,
        property_cell: TNode<PropertyCell>,
        value: TNode<Object>,
        exit_point: &mut ExitPoint,
        miss: &mut Label,
    ) {
        unimplemented!()
    }
    fn keyed_store_ic(&mut self, p: &StoreICParameters) {
        unimplemented!()
    }
    fn define_keyed_own_ic(&mut self, p: &StoreICParameters) {
        unimplemented!()
    }
    fn store_in_array_literal_ic(&mut self, p: &StoreICParameters) {
        unimplemented!()
    }

    fn lookup_global_ic(
        &mut self,
        lazy_name: LazyNode<Object>,
        depth: TNode<TaggedIndex>,
        lazy_slot: LazyNode<TaggedIndex>,
        context: TNode<Context>,
        lazy_feedback_vector: LazyNode<FeedbackVector>,
        typeof_mode: TypeofMode,
    ) {
        unimplemented!()
    }
    fn lookup_context(
        &mut self,
        lazy_name: LazyNode<Object>,
        depth: TNode<TaggedIndex>,
        lazy_slot: LazyNode<TaggedIndex>,
        context: TNode<Context>,
        typeof_mode: TypeofMode,
        context_kind: ContextKind,
    ) {
        unimplemented!()
    }

    fn goto_if_not_same_number_bit_pattern(&mut self, left: TNode<Float64T>, right: TNode<Float64T>, miss: &mut Label) {
        unimplemented!()
    }

    fn try_monomorphic_case(
        &mut self,
        slot: TNode<TaggedIndex>,
        vector: TNode<FeedbackVector>,
        weak_lookup_start_object_map: TNode<HeapObjectReference>,
        if_handler: &mut Label,
        var_handler: &mut TVariable<MaybeObject>,
        if_miss: &mut Label,
    ) -> TNode<HeapObjectReference> {
        unimplemented!()
    }
    fn handle_polymorphic_case(
        &mut self,
        weak_lookup_start_object_map: TNode<HeapObjectReference>,
        feedback: TNode<WeakFixedArray>,
        if_handler: &mut Label,
        var_handler: &mut TVariable<MaybeObject>,
        if_miss: &mut Label,
    ) {
        unimplemented!()
    }

    fn try_mega_dom_case(
        &mut self,
        lookup_start_object: TNode<Object>,
        lookup_start_object_map: TNode<Map>,
        var_handler: &mut TVariable<MaybeObject>,
        vector: TNode<Object>,
        slot: TNode<TaggedIndex>,
        miss: &mut Label,
        exit_point: &mut ExitPoint,
    ) {
        unimplemented!()
    }

    fn try_enumerated_keyed_load(&mut self, p: &LoadICParameters, lookup_start_object_map: TNode<Map>, exit_point: &mut ExitPoint) {
        unimplemented!()
    }

    fn handle_load_ic_handler_case(
        &mut self,
        p: &LazyLoadICParameters,
        handler: TNode<MaybeObject>,
        miss: &mut Label,
        exit_point: &mut ExitPoint,
        ic_mode: ICMode,
        on_nonexistent: OnNonExistent,
        support_elements: ElementSupport,
        access_mode: LoadAccessMode,
    ) {
        unimplemented!()
    }

    fn handle_load_ic_smi_handler_case(
        &mut self,
        p: &LazyLoadICParameters,
        holder: TNode<Object>,
        smi_handler: TNode<Smi>,
        handler: TNode<MaybeObject>,
        miss: &mut Label,
        exit_point: &mut ExitPoint,
        ic_mode: ICMode,
        on_nonexistent: OnNonExistent,
        support_elements: ElementSupport,
        access_mode: LoadAccessMode,
    ) {
        unimplemented!()
    }

    fn handle_load_ic_proto_handler(
        &mut self,
        p: &LazyLoadICParameters,
        handler: TNode<DataHandler>,
        var_holder: &mut TVariable<Object>,
        var_smi_handler: &mut TVariable<MaybeObject>,
        if_smi_handler: &mut Label,
        miss: &mut Label,
        exit_point: &mut ExitPoint,
        ic_mode: ICMode,
        access_mode: LoadAccessMode,
    ) {
        unimplemented!()
    }

    fn handle_load_callback_property(
        &mut self,
        p: &LazyLoadICParameters,
        holder: TNode<JSObject>,
        handler_word: TNode<Word32T>,
        exit_point: &mut ExitPoint,
    ) {
        unimplemented!()
    }

    fn handle_load_accessor(
        &mut self,
        p: &LazyLoadICParameters,
        function_template_info: TNode<FunctionTemplateInfo>,
        handler_word: TNode<Word32T>,
        handler: TNode<DataHandler>,
        handler_kind: TNode<Uint32T>,
        exit_point: &mut ExitPoint,
    ) {
        unimplemented!()
    }

    fn handle_load_field(
        &mut self,
        holder: TNode<JSObject>,
        handler_word: TNode<Word32T>,
        var_double_value: &mut TVariable<Float64T>,
        rebox_double: &mut Label,
        miss: &mut Label,
        exit_point: &mut ExitPoint,
    ) {
        unimplemented!()
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    fn handle_load_wasm_field(
        &mut self,
        holder: TNode<WasmObject>,
        wasm_value_type: TNode<Int32T>,
        field_offset: TNode<IntPtrT>,
        var_double_value: &mut TVariable<Float64T>,
        rebox_double: &mut Label,
        exit_point: &mut ExitPoint,
    ) {
        unimplemented!()
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    fn handle_load_wasm_field2(
        &mut self,
        holder: TNode<WasmObject>,
        handler_word: TNode<Word32T>,
        var_double_value: &mut TVariable<Float64T>,
        rebox_double: &mut Label,
        exit_point: &mut ExitPoint,
    ) {
        unimplemented!()
    }

    fn emit_access_check(
        &mut self,
        expected_native_context: TNode<Context>,
        context: TNode<Context>,
        receiver: TNode<Object>,
        can_access: &mut Label,
        miss: &mut Label,
    ) {
        unimplemented!()
    }

    fn handle_load_ic_smi_handler_load_named_case(
        &mut self,
        p: &LazyLoadICParameters,
        holder: TNode<Object>,
        handler_kind: TNode<Uint32T>,
        handler_word: TNode<Word32T>,
        rebox_double: &mut Label,
        var_double_value: &mut TVariable<Float64T>,
        handler: TNode<MaybeObject>,
        miss: &mut Label,
        exit_point: &mut ExitPoint,
        ic_mode: ICMode,
        on_nonexistent: OnNonExistent,
        support_elements: ElementSupport,
    ) {
        unimplemented!()
    }

    fn handle_load_ic_smi_handler_has_named_case(
        &mut self,
        p: &LazyLoadICParameters,
        holder: TNode<Object>,
        handler_kind: TNode<Uint32T>,
        miss: &mut Label,
        exit_point: &mut ExitPoint,
        ic_mode: ICMode,
    ) {
        unimplemented!()
    }

    fn load_global_ic_try_property_cell_case(
        &mut self,
        vector: TNode<FeedbackVector>,
        slot: TNode<TaggedIndex>,
        lazy_context: &LazyNode<Context>,
        exit_point: &mut ExitPoint,
        try_handler: &mut Label,
        miss: &mut Label,
    ) {
        unimplemented!()
    }

    fn load_global_ic_try_handler_case(
        &mut self,
        vector: TNode<FeedbackVector>,
        slot: TNode<TaggedIndex>,
        lazy_context: &LazyNode<Context>,
        lazy_name: &LazyNode<Name>,
        typeof_mode: TypeofMode,
        exit_point: &mut ExitPoint,
        miss: &mut Label,
    ) {
        unimplemented!()
    }

    fn script_context_table_lookup(
        &mut self,
        name: TNode<Name>,
        native_context: TNode<NativeContext>,
        found_hole: &mut Label,
        not_found: &mut Label,
    ) {
        unimplemented!()
    }

    fn handle_store_ic_proto_handler(
        &mut self,
        p: &StoreICParameters,
        handler: TNode<StoreHandler>,
        slow: &mut Label,
        miss: &mut Label,
        ic_mode: ICMode,
        support_elements: ElementSupport,
    ) {
        unimplemented!()
    }
    fn handle_store_ic_smi_handler_case(
        &mut self,
        handler_word: TNode<Word32T>,
        holder: TNode<JSObject>,
        value: TNode<Object>,
        miss: &mut Label,
    ) {
        unimplemented!()
    }
    fn handle_store_ic_smi_handler_js_shared_struct_field_case(
        &mut self,
        context: TNode<Context>,
        handler_word: TNode<Word32T>,
        holder: TNode<JSObject>,
        value: TNode<Object>,
    ) {
        unimplemented!()
    }
    fn handle_store_field_and_return(
        &mut self,
        handler_word: TNode<Word32T>,
        holder: TNode<JSObject>,
        value: TNode<Object>,
        double_value: Option<TNode<Float64T>>,
        representation: Representation,
        miss: &mut Label,
    ) {
        unimplemented!()
    }

    fn check_prototype_validity_cell(&mut self, maybe_validity_cell: TNode<Object>, miss: &mut Label) {
        unimplemented!()
    }
    fn handle_store_ic_native_data_property(&mut self, p: &StoreICParameters, holder: TNode<HeapObject>, handler_word: TNode<Word32T>) {
        unimplemented!()
    }

    fn handle_store_to_proxy(
        &mut self,
        p: &StoreICParameters,
        proxy: TNode<JSProxy>,
        miss: &mut Label,
        support_elements: ElementSupport,
    ) {
        unimplemented!()
    }

    fn generic_element_load(
        &mut self,
        lookup_start_object: TNode<JSAnyNotSmi>,
        lookup_start_object_map: TNode<Map>,
        lookup_start_object_instance_type: TNode<Int32T>,
        index: TNode<IntPtrT>,
        slow: &mut Label,
    ) {
        unimplemented!()
    }

    fn generic_property_load(
        &mut self,
        lookup_start_object: TNode<JSAnyNotSmi>,
        lookup_start_object_map: TNode<Map>,
        lookup_start_object_instance_type: TNode<Int32T>,
        p: &LoadICParameters,
        slow: &mut Label,
        use_stub_cache: UseStubCache,
    ) {
        unimplemented!()
    }

    fn handle_proto_handler<ICHandler, ICParameters>(
        &mut self,
        p: &ICParameters,
        handler: TNode<DataHandler>,
        on_code_handler: impl Fn(TNode<Code>),
        on_found_on_lookup_start_object: impl Fn(TNode<PropertyDictionary>, TNode<IntPtrT>),
        miss: &mut Label,
        ic_mode: ICMode,
    ) -> TNode<Object> {
        unimplemented!()
    }

    fn check_heap_object_type_matches_descriptor(
        &mut self,
        handler_word: TNode<Word32T>,
        holder: TNode<JSObject>,
        value: TNode<Object>,
        bailout: &mut Label,
    ) {
        unimplemented!()
    }

    fn check_descriptor_considers_numbers_mutable(
        &mut self,
        handler_word: TNode<Word32T>,
        holder: TNode<JSObject>,
        bailout: &mut Label,
    ) {
        unimplemented!()
    }

    fn extend_properties_backing_store(&mut self, object: TNode<HeapObject>, index: TNode<IntPtrT>) -> TNode<PropertyArray> {
        unimplemented!()
    }

    fn emit_fast_elements_bounds_check(
        &mut self,
        object: TNode<JSObject>,
        elements: TNode<FixedArrayBase>,
        intptr