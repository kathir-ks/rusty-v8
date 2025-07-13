// Converted from V8 C++ source files:
// Header: bytecode-array-builder.h
// Implementation: bytecode-array-builder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interpreter {
    use std::cell::RefCell;
    use std::cmp;
    use std::fmt;
    use std::mem;
    use std::rc::Rc;

    use crate::ast::ast::{CallType, Statement};
    use crate::ast::scopes::{Constant, Flag};
    use crate::execution::isolate::Exception;
    use crate::init::bootstrapper::JavaScript;
    use crate::interpreter::bytecode_array_writer::{
        BytecodeArrayWriter, ImplicitRegisterUse,
    };
    use crate::interpreter::bytecode_flags_and_tokens::{
        LanguageMode, LookupHoistingMode,
    };
    use crate::interpreter::bytecode_generator::{Call, If, Load, Statement};
    use crate::interpreter::bytecode_jump_table::ConstantArrayBuilder;
    use crate::interpreter::bytecode_label::BytecodeArrayBuilder;
    use crate::interpreter::bytecode_node::BytecodeNode;
    use crate::interpreter::bytecode_register_optimizer::{
        BytecodeRegisterAllocator, Register, RegisterList,
    };
    use crate::interpreter::bytecode_source_info::BytecodeSourceInfo;
    use crate::interpreter::bytecodes::Bytecode;
    use crate::interpreter::handler_table_builder::HandlerTableBuilder;
    use crate::interpreter::interpreter_generator::{
        ContextKind, JSFunction, SharedFunctionInfo, Type, TypeofMode,
    };
    use crate::objects::fixed_array::Tagged;
    use crate::regexp::regexp_parser::void;
    use crate::strings::uri::V8;
    use crate::v8::internal::{Isolate, LocalIsolate};
    use crate::wasm::module_decoder_impl::is;
    use crate::v8::{Context, Function, Global, Local, Promise, Value};

    pub struct BytecodeArray {}

    pub struct BytecodeLabel {}

    pub struct BytecodeLoopHeader {}

    pub struct BytecodeJumpTable {}

    pub struct SourcePositionTableBuilder {
        recording_mode: RecordingMode,
    }

    impl SourcePositionTableBuilder {
        pub const RECORD_SOURCE_POSITIONS: RecordingMode = RecordingMode::Record;
    }

    pub enum RecordingMode {
        Record,
    }

    pub struct FeedbackVectorSpec {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum DefineKeyedOwnPropertyFlags {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum DefineKeyedOwnPropertyInLiteralFlags {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum AbortReason {
        kNoReason,
        kLastErrorMessage,
    }
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum FeedbackSlotKind {
        kLoadGlobalInsideTypeof,
        kLoadGlobal,
        kLoadNamedProperty,
        kLoadKeyedProperty,
        kStoreGlobal,
        kStoreNamedProperty,
        kStoreKeyedProperty,
        kDefineNamedOwn,
        kDefineKeyedOwn,
        kCall,
        kConstruct,
        kBinaryOp,
        kCountOp,
        kToNumber,
        kTestEqual,
        kTestEqualStrict,
        kTestLessThan,
        kTestGreaterThan,
        kTestLessThanOrEqual,
        kTestGreaterThanOrEqual,
        kInstanceOf,
        kIn,
    }

    pub struct BytecodeRegisterOptimizer {
        builder: BytecodeArrayBuilder,
        source_info: BytecodeSourceInfo,
    }

    pub struct RegisterTransferWriter {}

    pub struct BytecodeArrayBuilder {
        zone: Rc<RefCell<Zone>>,
        feedback_vector_spec: *mut FeedbackVectorSpec,
        bytecode_generated: bool,
        constant_array_builder: ConstantArrayBuilder,
        handler_table_builder: HandlerTableBuilder,
        parameter_count: u16,
        max_arguments: u16,
        local_register_count: i32,
        register_allocator: BytecodeRegisterAllocator,
        bytecode_array_writer: BytecodeArrayWriter,
        register_optimizer: *mut BytecodeRegisterOptimizer, //Option<Box<BytecodeRegisterOptimizer>>,
        latest_source_info: BytecodeSourceInfo,
        deferred_source_info: BytecodeSourceInfo,
    }

    impl BytecodeArrayBuilder {
        pub fn new(
            zone: Rc<RefCell<Zone>>,
            parameter_count: i32,
            locals_count: i32,
            feedback_vector_spec: *mut FeedbackVectorSpec,
            source_position_mode: RecordingMode,
        ) -> BytecodeArrayBuilder {
            let fixed_register_count = locals_count;
            let mut builder = BytecodeArrayBuilder {
                zone: zone.clone(),
                feedback_vector_spec,
                bytecode_generated: false,
                constant_array_builder: ConstantArrayBuilder::new(zone.clone()),
                handler_table_builder: HandlerTableBuilder::new(zone.clone()),
                parameter_count: parameter_count as u16,
                max_arguments: 0,
                local_register_count: locals_count,
                register_allocator: BytecodeRegisterAllocator::new(fixed_register_count),
                bytecode_array_writer: BytecodeArrayWriter::new(
                    zone.clone(),
                    &ConstantArrayBuilder::new(zone.clone()),
                    source_position_mode,
                ),
                register_optimizer: std::ptr::null_mut(),
                //register_optimizer: None,
                latest_source_info: BytecodeSourceInfo::default(),
                deferred_source_info: BytecodeSourceInfo::default(),
            };
            unsafe {
                if v8_flags.ignition_reo {
                    //let transfer_writer = RegisterTransferWriter::new(&builder);
                    //let transfer_writer_ptr = Box::into_raw(Box::new(transfer_writer));

                    let optimizer = ZoneObject::new(zone.clone(), Box::new(BytecodeRegisterOptimizer {builder: builder, source_info:BytecodeSourceInfo::default()})); //zone.borrow().New::<BytecodeRegisterOptimizer>(BytecodeRegisterOptimizer::new(&builder));
                    builder.register_optimizer = Box::into_raw(Box::new(BytecodeRegisterOptimizer {builder: builder, source_info:BytecodeSourceInfo::default()}));
                }
            }
            builder
        }

        pub fn parameter_count(&self) -> u16 {
            self.parameter_count
        }
        pub fn max_arguments(&self) -> u16 {
            self.max_arguments
        }

        pub fn update_max_arguments(&mut self, max_arguments: u16) {
            self.max_arguments = cmp::max(self.max_arguments, max_arguments);
        }

        pub fn locals_count(&self) -> i32 {
            assert!(self.local_register_count >= 0);
            self.local_register_count
        }

        pub fn fixed_register_count(&self) -> i32 {
            self.locals_count()
        }

        pub fn total_register_count(&self) -> i32 {
            assert!(self.fixed_register_count() <= self.register_allocator.maximum_register_count());
            self.register_allocator.maximum_register_count()
        }

        pub fn local(&self, index: i32) -> Register {
            assert!(index < self.locals_count());
            Register::new(index)
        }

        pub fn parameter(&self, parameter_index: i32) -> Register {
            assert!(parameter_index >= 0);
            Register::from_parameter_index(parameter_index + 1)
        }

        pub fn receiver(&self) -> Register {
            Register::from_parameter_index(0)
        }

        pub fn load_constant_pool_entry(&mut self, entry: usize) -> &mut Self {
            self.output_lda_constant(entry);
            self
        }

        pub fn load_literal_smi(&mut self, smi: Tagged<i32>) -> &mut Self {
            let raw_smi = smi.get();
            if raw_smi == 0 {
                self.output_lda_zero();
            } else {
                self.output_lda_smi(raw_smi);
            }
            self
        }

        pub fn load_literal_double(&mut self, value: f64) -> &mut Self {
            if let Some(smi) = f64_to_smi(value) {
                self.load_literal_smi(Tagged::new(smi));
            } else {
                let entry = self.get_constant_pool_entry_double(value);
                self.output_lda_constant(entry);
            }
            self
        }

        pub fn load_literal_ast_raw_string(&mut self, raw_string: &str) -> &mut Self {
            let entry = self.get_constant_pool_entry_ast_raw_string(raw_string);
            self.output_lda_constant(entry);
            self
        }

        pub fn load_literal_ast_cons_string(&mut self, cons_string: &str) -> &mut Self {
            let entry = self.get_constant_pool_entry_ast_cons_string(cons_string);
            self.output_lda_constant(entry);
            self
        }

        pub fn load_literal_scope(&mut self, scope: &str) -> &mut Self {
            let entry = self.get_constant_pool_entry_scope(scope);
            self.output_lda_constant(entry);
            self
        }

        pub fn load_literal_ast_big_int(&mut self, big_int: i64) -> &mut Self {
            let entry = self.get_constant_pool_entry_ast_big_int(big_int);
            self.output_lda_constant(entry);
            self
        }

        pub fn load_undefined(&mut self) -> &mut Self {
            self.output_lda_undefined();
            self
        }

        pub fn load_null(&mut self) -> &mut Self {
            self.output_lda_null();
            self
        }

        pub fn load_the_hole(&mut self) -> &mut Self {
            self.output_lda_the_hole();
            self
        }

        pub fn load_true(&mut self) -> &mut Self {
            self.output_lda_true();
            self
        }

        pub fn load_false(&mut self) -> &mut Self {
            self.output_lda_false();
            self
        }

        pub fn load_boolean(&mut self, value: bool) -> &mut Self {
            if value {
                self.load_true()
            } else {
                self.load_false()
            }
        }

        pub fn load_global(
            &mut self,
            name: &str,
            feedback_slot: i32,
            typeof_mode: TypeofMode,
        ) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            match typeof_mode {
                TypeofMode::kInside => self.output_lda_global_inside_typeof(name_index, feedback_slot),
                TypeofMode::kNotInside => self.output_lda_global(name_index, feedback_slot),
            };
            self
        }

        pub fn store_global(&mut self, name: &str, feedback_slot: i32) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            self.output_sta_global(name_index, feedback_slot);
            self
        }

        pub fn load_context_slot(
            &mut self,
            context: Register,
            variable: &str,
            depth: i32,
            mutability: ContextSlotMutability,
        ) -> &mut Self {
            let slot_index = 0; // variable.index()
            match mutability {
                ContextSlotMutability::kImmutableSlot => {
                    if context.is_current_context() && depth == 0 {
                        self.output_lda_immutable_current_context_slot(slot_index);
                    } else {
                        self.output_lda_immutable_context_slot(context, slot_index, depth);
                    }
                }
                ContextSlotMutability::kMutableSlot => {
                    if false { //v8_flags.script_context_mutable_heap_number && variable.scope().is_script_scope()
                        if context.is_current_context() && depth == 0 {
                            self.output_lda_current_script_context_slot(slot_index);
                        } else {
                            self.output_lda_script_context_slot(context, slot_index, depth);
                        }
                    } else {
                        if context.is_current_context() && depth == 0 {
                            self.output_lda_current_context_slot(slot_index);
                        } else {
                            self.output_lda_context_slot(context, slot_index, depth);
                        }
                    }
                }
            }
            self
        }

        pub fn store_context_slot(
            &mut self,
            context: Register,
            variable: &str,
            depth: i32,
        ) -> &mut Self {
            let slot_index = 0; //variable.index();
            if false { //(v8_flags.script_context_mutable_heap_number || (v8_flags.const_tracking_let && variable.mode() == VariableMode::kLet)) && variable.scope().is_script_scope()
                if context.is_current_context() && depth == 0 {
                    self.output_sta_current_script_context_slot(slot_index);
                } else {
                    self.output_sta_script_context_slot(context, slot_index, depth);
                }
            } else {
                if context.is_current_context() && depth == 0 {
                    self.output_sta_current_context_slot(slot_index);
                } else {
                    self.output_sta_context_slot(context, slot_index, depth);
                }
            }
            self
        }

        pub fn load_module_variable(&mut self, cell_index: i32, depth: i32) -> &mut Self {
            self.output_lda_module_variable(cell_index, depth);
            self
        }

        pub fn store_module_variable(&mut self, cell_index: i32, depth: i32) -> &mut Self {
            self.output_sta_module_variable(cell_index, depth);
            self
        }

        pub fn load_accumulator_with_register(&mut self, reg: Register) -> &mut Self {
            if !self.register_optimizer.is_null() {
                 unsafe {
                // Defer source info so that if we elide the bytecode transfer, we attach
                // the source info to a subsequent bytecode if it exists.
                self.set_deferred_source_info(self.current_source_position(Bytecode::kLdar));
                (&mut *self.register_optimizer).source_info= self.latest_source_info;
                //self.latest_source_info.set_invalid();
                //self.register_optimizer.unwrap().do_ldar(reg);
            }
            } else {
                self.output_ldar(reg);
            }
            self
        }

        pub fn store_accumulator_in_register(&mut self, reg: Register) -> &mut Self {
            if !self.register_optimizer.is_null() {
                   unsafe {
                // Defer source info so that if we elide the bytecode transfer, we attach
                // the source info to a subsequent bytecode if it exists.
                self.set_deferred_source_info(self.current_source_position(Bytecode::kStar));
                  (&mut *self.register_optimizer).source_info= self.latest_source_info;
                //self.latest_source_info.set_invalid();
               // self.register_optimizer.unwrap().do_star(reg);
            }
            } else {
                self.output_star_raw(reg);
            }
            self
        }

        pub fn move_register(&mut self, from: Register, to: Register) -> &mut Self {
            assert!(from != to);
            if !self.register_optimizer.is_null() {
                     unsafe {
                // Defer source info so that if we elide the bytecode transfer, we attach
                // the source info to a subsequent bytecode if it exists.
                self.set_deferred_source_info(self.current_source_position(Bytecode::kMov));
                     (&mut *self.register_optimizer).source_info= self.latest_source_info;
                //self.latest_source_info.set_invalid();
                //self.register_optimizer.unwrap().do_mov(from, to);
            }
            } else {
                self.output_mov(from, to);
            }
            self
        }

        pub fn load_lookup_slot(&mut self, name: &str, typeof_mode: TypeofMode) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            match typeof_mode {
                TypeofMode::kInside => self.output_lda_lookup_slot_inside_typeof(name_index),
                TypeofMode::kNotInside => self.output_lda_lookup_slot(name_index),
            };
            self
        }

        pub fn load_lookup_context_slot(
            &mut self,
            name: &str,
            typeof_mode: TypeofMode,
            context_kind: ContextKind,
            slot_index: i32,
            depth: i32,
        ) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            if false { //v8_flags.script_context_mutable_heap_number && context_kind == ContextKind::kScriptContext
                match typeof_mode {
                    TypeofMode::kInside => self.output_lda_lookup_script_context_slot_inside_typeof(name_index, slot_index, depth),
                    TypeofMode::kNotInside => self.output_lda_lookup_script_context_slot(name_index, slot_index, depth),
                };
            } else {
                match typeof_mode {
                    TypeofMode::kInside => self.output_lda_lookup_context_slot_inside_typeof(name_index, slot_index, depth),
                    TypeofMode::kNotInside => self.output_lda_lookup_context_slot(name_index, slot_index, depth),
                };
            }
            self
        }

        pub fn load_lookup_global_slot(
            &mut self,
            name: &str,
            typeof_mode: TypeofMode,
            feedback_slot: i32,
            depth: i32,
        ) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            match typeof_mode {
                TypeofMode::kInside => self.output_lda_lookup_global_slot_inside_typeof(name_index, feedback_slot, depth),
                TypeofMode::kNotInside => self.output_lda_lookup_global_slot(name_index, feedback_slot, depth),
            };
            self
        }

        pub fn store_lookup_slot(
            &mut self,
            name: &str,
            language_mode: LanguageMode,
            lookup_hoisting_mode: LookupHoistingMode,
        ) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            let flags = 0; //StoreLookupSlotFlags::encode(language_mode, lookup_hoisting_mode);
            self.output_sta_lookup_slot(name_index, flags);
            self
        }

        pub fn load_named_property(
            &mut self,
            object: Register,
            name: &str,
            feedback_slot: i32,
        ) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            self.output_get_named_property(object, name_index, feedback_slot);
            self
        }

        pub fn load_named_property_from_super(
            &mut self,
            object: Register,
            name: &str,
            feedback_slot: i32,
        ) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            self.output_get_named_property_from_super(object, name_index, feedback_slot);
            self
        }

        pub fn load_keyed_property(&mut self, object: Register, feedback_slot: i32) -> &mut Self {
            self.output_get_keyed_property(object, feedback_slot);
            self
        }

        pub fn load_enumerated_keyed_property(
            &mut self,
            object: Register,
            enum_index: Register,
            cache_type: Register,
            feedback_slot: i32,
        ) -> &mut Self {
            self.output_get_enumerated_keyed_property(object, enum_index, cache_type, feedback_slot);
            self
        }

        pub fn load_iterator_property(&mut self, object: Register, feedback_slot: i32) -> &mut Self {
            let name_index = self.iterator_symbol_constant_pool_entry();
            self.output_get_named_property(object, name_index, feedback_slot);
            self
        }

        pub fn get_iterator(&mut self, object: Register, load_feedback_slot: i32, call_feedback_slot: i32) -> &mut Self {
            self.output_get_iterator(object, load_feedback_slot, call_feedback_slot);
            self
        }

        pub fn load_async_iterator_property(&mut self, object: Register, feedback_slot: i32) -> &mut Self {
            let name_index = self.async_iterator_symbol_constant_pool_entry();
            self.output_get_named_property(object, name_index, feedback_slot);
            self
        }

        pub fn define_keyed_own_property_in_literal(
            &mut self,
            object: Register,
            name: Register,
            flags: DefineKeyedOwnPropertyInLiteralFlags,
            feedback_slot: i32,
        ) -> &mut Self {
            self.output_define_keyed_own_property_in_literal(object, name, flags, feedback_slot);
            self
        }

        pub fn set_named_property(
            &mut self,
            object: Register,
            name_index: usize,
            feedback_slot: i32,
            language_mode: LanguageMode,
        ) -> &mut Self {
           // assert_eq!(get_language_mode_from_slot_kind(self.feedback_vector_spec.get_kind(feedback_slot)), language_mode); //Todo
            self.output_set_named_property(object, name_index, feedback_slot);
            self
        }

        pub fn set_named_property_ast_raw_string(
            &mut self,
            object: Register,
            name: &str,
            feedback_slot: i32,
            language_mode: LanguageMode,
        ) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            self.set_named_property(object, name_index, feedback_slot, language_mode);
            self
        }

        pub fn define_named_own_property(
            &mut self,
            object: Register,
            name: &str,
            feedback_slot: i32,
        ) -> &mut Self {
            let name_index = self.get_constant_pool_entry_ast_raw_string(name);
            //assert_eq!(FeedbackSlotKind::kDefineNamedOwn, self.feedback_vector_spec.get_kind(feedback_slot));
            self.output_define_named_own_property(object, name_index, feedback_slot);
            self
        }

        pub fn set_keyed_property(
            &mut self,
            object: Register,
            key: Register,
            feedback_slot: i32,
            language_mode: LanguageMode,
        ) -> &mut Self {
            //assert_eq!(get_language_mode_from_slot_kind(self.feedback_vector_spec.get_kind(feedback_slot)), language_mode);
            self.output_set_keyed_property(object, key, feedback_slot);
            self
        }

        pub fn define_keyed_own_property(
            &mut self,
            object: Register,
            key: Register,
            flags: DefineKeyedOwnPropertyFlags,
            feedback_slot: i32,
        ) -> &mut Self {
           // assert_eq!(get_language_mode_from_slot_kind(self.feedback_vector_spec.get_kind(feedback_slot)), LanguageMode::kStrict);
            self.output_define_keyed_own_property(object, key, flags, feedback_slot);
            self
        }

        pub fn store_in_array_literal(
            &mut self,
            array: Register,
            index: Register,
            feedback_slot: i32,
        ) -> &mut Self {
            self.output_sta_in_array_literal(array, index, feedback_slot);
            self
        }

        pub fn store_class_fields_initializer(&mut self, constructor: Register, feedback_slot: i32) -> &mut Self {
            let name_index = self.class_fields_symbol_constant_pool_entry();
            self.set_named_property(constructor, name_index, feedback_slot, LanguageMode::kStrict);
            self
        }

        pub fn load_class_fields_initializer(&mut self, constructor: Register, feedback_slot: i32) -> &mut Self {
            let name_index = self.class_fields_symbol_constant_pool_entry();
            self.output_get_named_property(constructor, name_index, feedback_slot);
            self
        }

        pub fn create_closure(
            &mut self,
            shared_function_info_entry: usize,
            slot: i32,
            flags: i32,
        ) -> &mut Self {
            self.output_create_closure(shared_function_info_entry, slot, flags);
            self
        }

        pub fn create_block_context(&mut self, scope: &str) -> &mut Self {
            let entry = self.get_constant_pool_entry_scope(scope);
            self.output_create_block_context(entry);
            self
        }

        pub fn create_catch_context(&mut self, exception: Register, scope: &str) -> &mut Self {
            let scope_index = self.get_constant_pool_entry_scope(scope);
            self.output_create_catch_context(exception, scope_index);
            self
        }

        pub fn create_function_context(&mut self, scope: &str, slots: i32) -> &mut Self {
            let scope_index = self.get_constant_pool_entry_scope(scope);
            self.output_create_function_context(scope_index, slots);
            self
        }

        pub fn create_eval_context(&mut self, scope: &str, slots: i32) -> &mut Self {
            let scope_index = self.get_constant_pool_entry_scope(scope);
            self.output_create_eval_context(scope_index, slots);
            self
        }

        pub fn create_with_context(&mut self, object: Register, scope: &str) -> &mut Self {
            let scope_index = self.get_constant_pool_entry_scope(scope);
            self.output_create_with_context(object, scope_index);
            self
        }

        pub fn create_arguments(&mut self, type_arguments: CreateArgumentsType) -> &mut Self {
            match type_arguments {
                CreateArgumentsType::kMappedArguments => self.output_create_mapped_arguments(),
                CreateArgumentsType::kUnmappedArguments => self.output_create_unmapped_arguments(),
                CreateArgumentsType::kRestParameter => self.output_create_rest_parameter(),
            };
            self
        }

        pub fn create_reg_exp_literal(
            &mut self,
            pattern: &str,
            literal_index: i32,
            flags: i32,
        ) -> &mut Self {
            let pattern_entry = self.get_constant_pool_entry_ast_raw_string(pattern);
            self.output_create_reg_exp_literal(pattern_entry, literal_index, flags);
            self
        }

        pub fn create_empty_array_literal(&mut self, literal_index: i32) -> &mut Self {
            self.output_create_empty_array_literal(literal_index);
            self
        }

        pub fn create_array_literal(
            &mut self,
            constant_elements_entry: usize,
            literal_index: i32,
            flags: i32,
        ) -> &mut Self {
            self.output_create_array_literal(constant_elements_entry, literal_index, flags);
            self
        }

        pub fn create_array_from_iterable(&mut self) -> &mut Self {
            self.output_create_array_from_iterable();
            self
        }

        pub fn create_object_literal(
            &mut self,
            constant_properties_entry: usize,
            literal_index: i32,
            flags: i32,
        ) -> &mut Self {
            self.output_create_object_literal(constant_properties_entry, literal_index, flags);
            self
        }

        pub fn create_empty_object_literal(&mut self) -> &mut Self {
            self.output_create_empty_object_literal();
            self
        }

        pub fn clone_object(&mut self, source: Register, flags: i32, feedback_slot: i32) -> &mut Self {
            self.output_clone_object(source, flags, feedback_slot);
            self
        }

        pub fn get_template_object(
            &mut self,
            template_object_description_entry: usize,
            feedback_slot: i32,
        ) -> &mut Self {
            self.output_get_template_object(template_object_description_entry, feedback_slot);
            self
        }

        pub fn push_context(&mut self, context: Register) -> &mut Self {
            self.output_push_context(context);
            self
        }

        pub fn pop_context(&mut self, context: Register) -> &mut Self {
            self.output_pop_context(context);
            self
        }

        pub fn to_object(&mut self, out: Register) -> &mut Self {
            self.output_to_object(out);
            self
        }

        pub fn to_name(&mut self) -> &mut Self {
            self.output_to_name();
            self
        }

        pub fn to_string(&mut self) -> &mut Self {
            self.output_to_string();
            self
        }

        pub fn to_boolean(&mut self, mode: ToBooleanMode) -> &mut Self {
            match mode {
                ToBooleanMode::kAlreadyBoolean => {}
                ToBooleanMode::kConvertToBoolean => self.output_to_boolean(),
            };
            self
        }

        pub fn to_number(&mut self, feedback_slot: i32) -> &mut Self {
            self.output_to_number(feedback_slot);
            self
        }

        pub fn to_numeric(&mut self, feedback_slot: i32) -> &mut Self {
            self.output_to_numeric(feedback_slot);
            self
        }

        pub fn bind(&mut self, label: *mut BytecodeLabel) -> &mut Self {
            // Don't generate code for a label which hasn't had a corresponding forward
            // jump generated already. For backwards jumps, use BindLoopHeader.
            unsafe{
                if !label.is_null() {
                   // if (!(*label).has_referrer_jump()) {
                  //      return self;
                 //   }

                    // Flush the register optimizer when binding a label to ensure all
                    // expected registers are valid when jumping to this label.
                    if !self.register_optimizer.is_null() {
                      //  self.register_optimizer().Flush();
                       // self.register_optimizer().ResetTypeHintForAccumulator();
                    }
                    //self.bytecode_array_writer.bind_label(label);
                }
            }

            self
        }

        pub fn bind_loop(&mut self, loop_header: *mut BytecodeLoopHeader) -> &mut Self {
            // Flush the register optimizer when starting a loop to ensure all expected
            // registers are valid when jumping to the loop header.
            unsafe {
            if !self.register_optimizer.is_null() {
              //  self.register_optimizer().Flush();
             //   self.register_optimizer().ResetTypeHintForAccumulator();
            }
           // self.bytecode_array_writer.bind_loop_header(loop_header);
            }
            self
        }

        pub fn bind_jump_table(&mut self, jump_table: *mut BytecodeJumpTable, case_value: i32) -> &mut Self {
            // Flush the register optimizer when binding a jump table entry to ensure
            // all expected registers are valid when jumping to this location.
          unsafe {
            if !self.register_optimizer.is_null() {
              //  self.register_optimizer().Flush();
              //  self.register
