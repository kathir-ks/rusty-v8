// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod kythe_data {
    use std::collections::{HashMap, HashSet};
    use std::any::Any;

    // Placeholder for base::ContextualClass.  Needs a custom implementation
    // or external crate integration for full fidelity.  For now, we just
    // stub it out.
    pub struct Contextual<T>(T);

    impl<T> Contextual<T> {
        pub fn new(value: T) -> Self {
            Contextual(value)
        }

        pub fn get(&self) -> &T {
            &self.0
        }

        pub fn get_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    // Dummy types mirroring C++ Torque AST structures
    pub struct Value;
    pub struct Callable;
    pub struct Field;
    pub struct Binding<T> {
        _phantom: std::marker::PhantomData<T>,
    }
    pub struct LocalValue;
    pub struct LocalLabel;
    pub struct Declarable;
    pub struct SourcePosition; //  Needs a real implementation
    pub struct GlobalContext; // Dummy type.

    #[derive(Debug, Clone)]
    pub struct KythePosition {
        pub file_path: String,
        pub start_offset: u64,
        pub end_offset: u64,
    }

    pub type kythe_entity_t = u64;

    pub trait KytheConsumer {
        fn add_definition(&mut self, kind: Kind, name: String, pos: KythePosition) -> kythe_entity_t;
        fn add_use(&mut self, kind: Kind, entity: kythe_entity_t, use_pos: KythePosition);
        fn add_call(&mut self, kind: Kind, caller_entity: kythe_entity_t, call_pos: KythePosition, callee_entity: kythe_entity_t);
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Kind {
        Unspecified,
        Constant,
        Function,
        ClassField,
        Variable,
        Type,
    }

    pub struct KytheData {
        consumer_: Option<Box<dyn KytheConsumer>>,
        constants_: HashMap<*const Value, kythe_entity_t>,
        callables_: HashMap<*const Callable, kythe_entity_t>,
        field_uses_: HashMap<*const Field, HashSet<SourcePosition>>,
        local_bindings_: HashMap<u64, kythe_entity_t>,
        types_: HashMap<*const Declarable, kythe_entity_t>,
        class_fields_: HashMap<*const Field, kythe_entity_t>,
    }

    impl KytheData {
        pub fn new() -> Self {
            KytheData {
                consumer_: None,
                constants_: HashMap::new(),
                callables_: HashMap::new(),
                field_uses_: HashMap::new(),
                local_bindings_: HashMap::new(),
                types_: HashMap::new(),
                class_fields_: HashMap::new(),
            }
        }

        pub fn set_consumer(&mut self, consumer: Box<dyn KytheConsumer>) {
            self.consumer_ = Some(consumer);
        }

        pub fn add_constant_definition(&mut self, constant: *const Value) -> kythe_entity_t {
            if self.constants_.contains_key(&constant) {
                return *self.constants_.get(&constant).unwrap();
            }

            let name = format!("Constant_{:p}", constant);

            let pos = KythePosition {
                file_path: "unknown".to_string(),
                start_offset: 0,
                end_offset: 0,
            };

            let entity = match &mut self.consumer_ {
                Some(consumer) => consumer.add_definition(Kind::Constant, name, pos),
                None => 0, // or some default value or error handling
            };

            self.constants_.insert(constant, entity);
            entity
        }

        pub fn add_constant_use(&mut self, use_position: SourcePosition, constant: *const Value) {
            if let Some(&entity) = self.constants_.get(&constant) {
                let pos = KythePosition {
                    file_path: "unknown".to_string(),
                    start_offset: 0,
                    end_offset: 0,
                };
                if let Some(consumer) = &mut self.consumer_ {
                    consumer.add_use(Kind::Constant, entity, pos);
                }
            }
        }

        pub fn add_function_definition(&mut self, callable: *const Callable) -> kythe_entity_t {
            if self.callables_.contains_key(&callable) {
                return *self.callables_.get(&callable).unwrap();
            }

            let name = format!("Callable_{:p}", callable);

            let pos = KythePosition {
                file_path: "unknown".to_string(),
                start_offset: 0,
                end_offset: 0,
            };

            let entity = match &mut self.consumer_ {
                Some(consumer) => consumer.add_definition(Kind::Function, name, pos),
                None => 0, // or some default value or error handling
            };

            self.callables_.insert(callable, entity);
            entity
        }

        pub fn add_call(&mut self, caller: *const Callable, call_position: SourcePosition, callee: *const Callable) {
            if let (Some(&caller_entity), Some(&callee_entity)) = (self.callables_.get(&caller), self.callables_.get(&callee)) {

                let pos = KythePosition {
                    file_path: "unknown".to_string(),
                    start_offset: 0,
                    end_offset: 0,
                };

                if let Some(consumer) = &mut self.consumer_ {
                    consumer.add_call(Kind::Function, caller_entity, pos, callee_entity);
                }
            }
        }

        pub fn add_class_field_definition(&mut self, field: *const Field) -> kythe_entity_t {
            if self.class_fields_.contains_key(&field) {
                return *self.class_fields_.get(&field).unwrap();
            }

            let name = format!("Field_{:p}", field);

            let pos = KythePosition {
                file_path: "unknown".to_string(),
                start_offset: 0,
                end_offset: 0,
            };

            let entity = match &mut self.consumer_ {
                Some(consumer) => consumer.add_definition(Kind::ClassField, name, pos),
                None => 0, // or some default value or error handling
            };

            self.class_fields_.insert(field, entity);
            entity
        }

        pub fn add_class_field_use(&mut self, use_position: SourcePosition, field: *const Field) {
             if let Some(&entity) = self.class_fields_.get(&field) {

                let pos = KythePosition {
                    file_path: "unknown".to_string(),
                    start_offset: 0,
                    end_offset: 0,
                };

                if let Some(consumer) = &mut self.consumer_ {
                    consumer.add_use(Kind::ClassField, entity, pos);
                }
            } else {
                 let mut uses = self.field_uses_.entry(field).or_insert(HashSet::new());
                 uses.insert(use_position);
             }
        }

        pub fn add_binding_definition<T>(&mut self, binding: *const Binding<T>) -> kythe_entity_t {
            let binding_index = binding as u64;
            let name = format!("Binding_{:p}", binding);
            let ident_pos = SourcePosition; // Dummy

            self.add_binding_definition_impl(binding_index, name, ident_pos)
        }

        pub fn add_type_definition(&mut self, type_decl: *const Declarable) -> kythe_entity_t {
            if self.types_.contains_key(&type_decl) {
                return *self.types_.get(&type_decl).unwrap();
            }

            let name = format!("Type_{:p}", type_decl);

            let pos = KythePosition {
                file_path: "unknown".to_string(),
                start_offset: 0,
                end_offset: 0,
            };

            let entity = match &mut self.consumer_ {
                Some(consumer) => consumer.add_definition(Kind::Type, name, pos),
                None => 0, // or some default value or error handling
            };

            self.types_.insert(type_decl, entity);
            entity
        }

         pub fn add_type_use(&mut self, use_position: SourcePosition, type_decl: *const Declarable) {
            if let Some(&entity) = self.types_.get(&type_decl) {

                let pos = KythePosition {
                    file_path: "unknown".to_string(),
                    start_offset: 0,
                    end_offset: 0,
                };

                if let Some(consumer) = &mut self.consumer_ {
                    consumer.add_use(Kind::Type, entity, pos);
                }
            }
        }


        fn add_binding_definition_impl(
            &mut self,
            binding_index: u64,
            name: String,
            ident_pos: SourcePosition,
        ) -> kythe_entity_t {
             if self.local_bindings_.contains_key(&binding_index) {
                return *self.local_bindings_.get(&binding_index).unwrap();
            }


            let pos = KythePosition {
                file_path: "unknown".to_string(),
                start_offset: 0,
                end_offset: 0,
            };

            let entity = match &mut self.consumer_ {
                Some(consumer) => consumer.add_definition(Kind::Variable, name, pos),
                None => 0, // or some default value or error handling
            };
           self.local_bindings_.insert(binding_index, entity);
            entity
        }

        pub fn add_binding_use<T>(&mut self, use_position: SourcePosition, binding: *const Binding<T>) {
            let binding_index = binding as u64;
            if let Some(&entity) = self.local_bindings_.get(&binding_index) {

                let pos = KythePosition {
                    file_path: "unknown".to_string(),
                    start_offset: 0,
                    end_offset: 0,
                };

                if let Some(consumer) = &mut self.consumer_ {
                    consumer.add_use(Kind::Variable, entity, pos);
                }
            }
        }

    }

}