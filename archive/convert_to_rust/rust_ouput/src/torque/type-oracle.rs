// Converted from V8 C++ source files:
// Header: type-oracle.h
// Implementation: type-oracle.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod torque {
    pub mod type_oracle {
        use std::cell::Ref;
        use std::collections::HashMap;
        use std::sync::Mutex;
        use crate::torque::constants::*;
        use crate::torque::declarable::*;
        use crate::torque::declarations::*;
        use crate::torque::types::*;
        use crate::torque::utils::*;
        use crate::v8::internal::torque::types::{Type, TypeVector};

        pub struct TypeOracle {
            function_pointer_types_: Deduplicator<BuiltinPointerType>,
            all_builtin_pointer_types_: Vec<*const BuiltinPointerType>,
            union_types_: Deduplicator<UnionType>,
            nominal_types_: Vec<Box<AbstractType>>,
            aggregate_types_: Vec<Box<AggregateType>>,
            bit_field_struct_types_: Vec<Box<BitFieldStructType>>,
            top_types_: Vec<Box<Type>>,
            generic_type_instantiation_namespaces_: Vec<Box<Namespace>>,
            next_type_id_: usize,
        }

        impl TypeOracle {
            pub fn new() -> Self {
                TypeOracle {
                    function_pointer_types_: Deduplicator::new(),
                    all_builtin_pointer_types_: Vec::new(),
                    union_types_: Deduplicator::new(),
                    nominal_types_: Vec::new(),
                    aggregate_types_: Vec::new(),
                    bit_field_struct_types_: Vec::new(),
                    top_types_: Vec::new(),
                    generic_type_instantiation_namespaces_: Vec::new(),
                    next_type_id_: 0,
                }
            }

             fn with_context<F, R>(f: F) -> R
                where
                    F: FnOnce(&mut TypeOracle) -> R,
            {
                thread_local! {
                    static TYPE_ORACLE: Mutex<TypeOracle> = Mutex::new(TypeOracle::new());
                }
                TYPE_ORACLE.with(|oracle| {
                    let mut guard = oracle.lock().unwrap();
                    f(&mut guard)
                })
            }
            pub fn get() -> &'static Mutex<TypeOracle> {
                thread_local! {
                    static TYPE_ORACLE: Mutex<TypeOracle> = Mutex::new(TypeOracle::new());
                }
                TYPE_ORACLE.with(|x| x)
            }


            pub fn get_abstract_type(
                parent: *const Type,
                name: String,
                flags: AbstractTypeFlags,
                generated: String,
                non_constexpr_version: *const Type,
                specialized_from: MaybeSpecializationKey,
            ) -> *const AbstractType {
                Self::with_context(|oracle| {
                    let mut ptr = Box::new(AbstractType::new(
                        parent,
                        flags,
                        name.clone(),
                        generated.clone(),
                        non_constexpr_version,
                        specialized_from,
                    ));
                    let result = &*ptr as *const AbstractType;
                    if !non_constexpr_version.is_null() {
                         unsafe {
                            if let Some(non_constexpr_version_ref) = non_constexpr_version.as_ref() {
                                assert!(ptr.is_constexpr());
                                non_constexpr_version_ref.set_constexpr_version(result);
                            }
                        }
                    }
                    oracle.nominal_types_.push(ptr);
                    result
                })
            }

            pub fn get_struct_type(
                decl: *const StructDeclaration,
                specialized_from: MaybeSpecializationKey,
            ) -> *mut StructType {
                Self::with_context(|oracle| {
                    let mut ptr = Box::new(StructType::new(
                        CurrentNamespace::get(),
                        decl,
                        specialized_from,
                    ));
                    let result = &mut *ptr as *mut StructType;
                    oracle.aggregate_types_.push(ptr);
                    result
                })
            }

            pub fn get_bit_field_struct_type(
                parent: *const Type,
                decl: *const BitFieldStructDeclaration,
            ) -> *mut BitFieldStructType {
                Self::with_context(|oracle| {
                    let mut ptr = Box::new(BitFieldStructType::new(
                        CurrentNamespace::get(),
                        parent,
                        decl,
                    ));
                    let result = &mut *ptr as *mut BitFieldStructType;
                    oracle.bit_field_struct_types_.push(ptr);
                    result
                })
            }

            pub fn get_class_type(
                parent: *const Type,
                name: &str,
                flags: ClassFlags,
                generates: &str,
                decl: *mut ClassDeclaration,
                alias: *const TypeAlias,
            ) -> *mut ClassType {
                 Self::with_context(|oracle| {
                    let mut type_ = Box::new(ClassType::new(
                        parent,
                        CurrentNamespace::get(),
                        name.to_string(),
                        flags,
                        generates.to_string(),
                        decl,
                        alias,
                    ));
                    let result = &mut *type_ as *mut ClassType;
                    oracle.aggregate_types_.push(type_);
                    result
                })
            }

            pub fn get_builtin_pointer_type(
                argument_types: TypeVector,
                return_type: *const Type,
            ) -> *const BuiltinPointerType {
                Self::with_context(|oracle| {
                    let builtin_type =
                        oracle.get_builtin_type(&QualifiedName(BUILTIN_POINTER_TYPE_STRING.to_string()));
                    let result = oracle.function_pointer_types_.deduplicate(BuiltinPointerType::new(
                        builtin_type,
                        argument_types,
                        return_type,
                        oracle.all_builtin_pointer_types_.len(),
                    ));
                    if result.function_pointer_type_id() == oracle.all_builtin_pointer_types_.len() {
                        oracle.all_builtin_pointer_types_.push(result);
                    }
                    result
                })
            }

            pub fn get_generic_type_instance(
                generic_type: *mut GenericType,
                arg_types: TypeVector,
            ) -> *const Type {
                unsafe {
                    if generic_type.is_null() {
                        eprintln!("Error: Generic type is null");
                        return std::ptr::null();
                    }
                    if let Some(generic_type_ref) = generic_type.as_mut() {
                         let params = generic_type_ref.generic_parameters();

                        if params.len() != arg_types.len() {
                             ReportError(&format!("Generic struct takes {} parameters, but {} were given", params.len(), arg_types.len()));
                            return std::ptr::null();
                        }

                         if let Some(specialization) = generic_type_ref.get_specialization(arg_types.clone()) {
                            return specialization;
                        } else {
                            let type_: *const Type;
                            {
                                let requester_scope = CurrentScope::get();
                                let generic_scope = generic_type_ref.parent_scope();
                                CurrentScope::scope(generic_scope, || {
                                     type_ = TypeVisitor::compute_type(
                                        generic_type_ref.declaration(),
                                        vec![(generic_type, arg_types.clone())],
                                        requester_scope,
                                    );
                                });

                            }
                            generic_type_ref.add_specialization(arg_types, type_);
                            return type_;
                        }
                    } else {
                        eprintln!("Error: Generic type is null");
                        return std::ptr::null();
                    }
                }
            }

            pub fn get_reference_generic(is_const: bool) -> *mut GenericType {
                Declarations::lookup_unique_generic_type(&QualifiedName(
                    TORQUE_INTERNAL_NAMESPACE_STRING.to_string(),
                    if is_const {
                        CONST_REFERENCE_TYPE_STRING.to_string()
                    } else {
                        MUTABLE_REFERENCE_TYPE_STRING.to_string()
                    },
                ))
            }

            pub fn get_const_reference_generic() -> *mut GenericType {
                Self::get_reference_generic(true)
            }

            pub fn get_mutable_reference_generic() -> *mut GenericType {
                Self::get_reference_generic(false)
            }

            pub fn match_reference_generic(
                reference_type: *const Type,
                is_const: *mut bool,
            ) -> Option<*const Type> {
                 unsafe {
                    if let Some(type_) = Type::match_unary_generic(
                        reference_type,
                        Self::get_mutable_reference_generic(),
                    ) {
                        if !is_const.is_null() {
                            *is_const = false;
                        }
                        return Some(type_);
                    }
                    if let Some(type_) = Type::match_unary_generic(
                        reference_type,
                        Self::get_const_reference_generic(),
                    ) {
                        if !is_const.is_null() {
                            *is_const = true;
                        }
                        return Some(type_);
                    }
                }
                None
            }

            pub fn get_mutable_slice_generic() -> *mut GenericType {
                Declarations::lookup_unique_generic_type(&QualifiedName(MUTABLE_SLICE_TYPE_STRING.to_string()))
            }

            pub fn get_const_slice_generic() -> *mut GenericType {
                Declarations::lookup_unique_generic_type(&QualifiedName(CONST_SLICE_TYPE_STRING.to_string()))
            }

            pub fn get_weak_generic() -> *mut GenericType {
                Declarations::lookup_global_unique_generic_type(WEAK_TYPE_STRING)
            }

            pub fn get_smi_tagged_generic() -> *mut GenericType {
                Declarations::lookup_global_unique_generic_type(SMI_TAGGED_TYPE_STRING)
            }

            pub fn get_lazy_generic() -> *mut GenericType {
                Declarations::lookup_global_unique_generic_type(LAZY_TYPE_STRING)
            }

            pub fn get_reference_type(referenced_type: *const Type, is_const: bool) -> *const Type {
                Self::get_generic_type_instance(
                    Self::get_reference_generic(is_const),
                    vec![referenced_type],
                )
            }

            pub fn get_const_reference_type(referenced_type: *const Type) -> *const Type {
                Self::get_reference_type(referenced_type, true)
            }

            pub fn get_mutable_reference_type(referenced_type: *const Type) -> *const Type {
                Self::get_reference_type(referenced_type, false)
            }

            pub fn get_mutable_slice_type(referenced_type: *const Type) -> *const Type {
                Self::get_generic_type_instance(
                    Self::get_mutable_slice_generic(),
                    vec![referenced_type],
                )
            }

            pub fn get_const_slice_type(referenced_type: *const Type) -> *const Type {
                Self::get_generic_type_instance(Self::get_const_slice_generic(), vec![referenced_type])
            }

            pub fn all_builtin_pointer_types() -> &'static Vec<*const BuiltinPointerType> {
                Self::with_context(|oracle| &oracle.all_builtin_pointer_types_)
            }

            pub fn get_union_type_union(type_: UnionType) -> *const Type {
                Self::with_context(|oracle| {
                    if let Some(single) = type_.get_single_member() {
                        return single;
                    }
                    oracle.union_types_.deduplicate(type_)
                })
            }

            pub fn get_union_type(a: *const Type, b: *const Type) -> *const Type {
                unsafe {
                    if a.is_null() || b.is_null() {
                        return std::ptr::null();
                    }
                    if let Some(a_ref) = a.as_ref() {
                        if let Some(b_ref) = b.as_ref() {
                             if a_ref.is_subtype_of(b) {
                                return b;
                            }
                            if b_ref.is_subtype_of(a) {
                                return a;
                            }
                            let mut result = UnionType::from_type(a);
                            result.extend(b);
                            return Self::get_union_type_union(result);
                        } else {
                            return std::ptr::null();
                        }
                    } else {
                        return std::ptr::null();
                    }
                }
            }

            pub fn get_top_type(reason: String, source_type: *const Type) -> *const Type {
                Self::with_context(|oracle| {
                    let mut type_ = Box::new(TopType::new(reason, source_type));
                    let result = &*type_ as *const Type;
                    oracle.top_types_.push(type_);
                    result
                })
            }

            pub fn get_arguments_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(ARGUMENTS_TYPE_STRING.to_string())))
            }

            pub fn get_bool_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(BOOL_TYPE_STRING.to_string())))
            }

            pub fn get_constexpr_bool_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONSTEXPR_BOOL_TYPE_STRING.to_string())))
            }

            pub fn get_constexpr_string_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONSTEXPR_STRING_TYPE_STRING.to_string())))
            }

            pub fn get_constexpr_intptr_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONSTEXPR_INTPTR_TYPE_STRING.to_string())))
            }

            pub fn get_constexpr_instance_type_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONSTEXPR_INSTANCE_TYPE_TYPE_STRING.to_string())))
            }

            pub fn get_void_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(VOID_TYPE_STRING.to_string())))
            }

            pub fn get_raw_ptr_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(RAWPTR_TYPE_STRING.to_string())))
            }

            pub fn get_external_pointer_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(EXTERNALPTR_TYPE_STRING.to_string())))
            }

            pub fn get_cpp_heap_pointer_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CPPHEAPPTR_TYPE_STRING.to_string())))
            }

            pub fn get_trusted_pointer_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(TRUSTEDPTR_TYPE_STRING.to_string())))
            }

            pub fn get_protected_pointer_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(PROTECTEDPTR_TYPE_STRING.to_string())))
            }

            pub fn get_dispatch_handle_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(DISPATCH_HANDLE_TYPE_STRING.to_string())))
            }

            pub fn get_map_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(MAP_TYPE_STRING.to_string())))
            }

            pub fn get_object_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(OBJECT_TYPE_STRING.to_string())))
            }

            pub fn get_heap_object_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(HEAP_OBJECT_TYPE_STRING.to_string())))
            }

            pub fn get_tagged_zero_pattern_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(TAGGED_ZERO_PATTERN_TYPE_STRING.to_string())))
            }

            pub fn get_js_any_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(JSANY_TYPE_STRING.to_string())))
            }

            pub fn get_js_object_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(JSOBJECT_TYPE_STRING.to_string())))
            }

            pub fn get_tagged_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(TAGGED_TYPE_STRING.to_string())))
            }

            pub fn get_strong_tagged_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(STRONG_TAGGED_TYPE_STRING.to_string())))
            }

            pub fn get_uninitialized_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(UNINITIALIZED_TYPE_STRING.to_string())))
            }

            pub fn get_uninitialized_heap_object_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(
                    QualifiedName(TORQUE_INTERNAL_NAMESPACE_STRING.to_string(), UNINITIALIZED_HEAP_OBJECT_TYPE_STRING.to_string()))
                ))
            }

            pub fn get_smi_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(SMI_TYPE_STRING.to_string())))
            }

            pub fn get_const_string_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONST_STRING_TYPE_STRING.to_string())))
            }

            pub fn get_string_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(STRING_TYPE_STRING.to_string())))
            }

            pub fn get_number_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(NUMBER_TYPE_STRING.to_string())))
            }

            pub fn get_intptr_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(INTPTR_TYPE_STRING.to_string())))
            }

            pub fn get_uintptr_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(UINTPTR_TYPE_STRING.to_string())))
            }

            pub fn get_int64_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(INT64_TYPE_STRING.to_string())))
            }

            pub fn get_uint64_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(UINT64_TYPE_STRING.to_string())))
            }

            pub fn get_int32_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(INT32_TYPE_STRING.to_string())))
            }

            pub fn get_uint32_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(UINT32_TYPE_STRING.to_string())))
            }

            pub fn get_uint31_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(UINT31_TYPE_STRING.to_string())))
            }

            pub fn get_int16_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(INT16_TYPE_STRING.to_string())))
            }

            pub fn get_uint16_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(UINT16_TYPE_STRING.to_string())))
            }

            pub fn get_int8_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(INT8_TYPE_STRING.to_string())))
            }

            pub fn get_uint8_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(UINT8_TYPE_STRING.to_string())))
            }

            pub fn get_float64_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(FLOAT64_TYPE_STRING.to_string())))
            }

            pub fn get_float64_or_undefined_or_hole_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING.to_string())))
            }

            pub fn get_const_float64_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONST_FLOAT64_TYPE_STRING.to_string())))
            }

            pub fn get_integer_literal_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(INTEGER_LITERAL_TYPE_STRING.to_string())))
            }

            pub fn get_never_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(NEVER_TYPE_STRING.to_string())))
            }

            pub fn get_const_int31_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONST_INT31_TYPE_STRING.to_string())))
            }

            pub fn get_const_int32_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONST_INT32_TYPE_STRING.to_string())))
            }

            pub fn get_context_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(CONTEXT_TYPE_STRING.to_string())))
            }

            pub fn get_no_context_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(NO_CONTEXT_TYPE_STRING.to_string())))
            }

            pub fn get_native_context_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(NATIVE_CONTEXT_TYPE_STRING.to_string())))
            }

            pub fn get_js_function_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(JS_FUNCTION_TYPE_STRING.to_string())))
            }

            pub fn get_uninitialized_iterator_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(UNINITIALIZED_ITERATOR_TYPE_STRING.to_string())))
            }

            pub fn get_fixed_array_base_type() -> *const Type {
                Self::with_context(|oracle| oracle.get_builtin_type(&QualifiedName(FIXED_ARRAY_BASE_TYPE_STRING.to_string())))
            }

            pub fn implicitly_convertable_from(
                to: *const Type,
                from: *const Type,
            ) -> Option<*const Type> {
                 unsafe {
                    let mut current_from = from;
                    while !current_from.is_null() {
                         if let Some(from_constexprs) = Declarations::lookup_generic(kFromConstexprMacroName) {
                            for from_constexpr in from_constexprs {
                                 if let Some(specialization) =
                                    from_constexpr.get_specialization(vec![to, current_from])
                                {
                                     if (*specialization).signature().get_explicit_types() == vec![current_from] {
                                        return Some(current_from);
                                    }
                                }
                            }
                        }
                        if let Some(current_from_ref) = current_from.as_ref() {
                            current_from = current_from_ref.parent();
                        } else {
                            break;
                        }

                    }
                 }
                None
            }

            pub fn get_aggregate_types() -> &'static Vec<Box<AggregateType>> {
                Self::with_context(|oracle| &oracle.aggregate_types_)
            }

            pub fn get_bit_field_struct_types() -> &'static Vec<Box<BitFieldStructType>> {
                Self::with_context(|oracle| &oracle.bit_field_struct_types_)
            }

            pub fn get_classes() -> Vec<*const ClassType> {
                 Self::with_context(|oracle| {
                    let mut result = Vec::new();
                    for t in &oracle.aggregate_types_ {
                        if let Some(class_type) = t.downcast_ref::<ClassType>() {
                            result.push(class_type);
                        }
                    }
                    result
                })
            }

            pub fn finalize_aggregate_types() {
                Self::with_context(|oracle| {
                    let mut current = 0;
                    while current != oracle.aggregate_types_.len() {
                        oracle.aggregate_types_[current].finalize();
                        current += 1;
                    }
                })
            }

            pub fn fresh_type_id() -> usize {
                Self::with_context(|oracle| {
                    let id = oracle.next_type_id_;
                    oracle.next_type_id_ += 1;
                    id
                })
            }

            pub fn create_generic_type_instantiation_namespace() -> *mut Namespace {
                Self::with_context(|oracle| {
                    let mut namespace = Box::new(Namespace::new(GENERIC_TYPE_INSTANTIATION_NAMESPACE_STRING.to_string()));
                    let result = &mut *namespace as *mut Namespace;
                    oracle.generic_type_instantiation_namespaces_.push(namespace);
                    result
                })
            }
        }

        impl TypeOracle {
            fn get_builtin_type(&mut self, name: &QualifiedName) -> *const Type {
                Declarations::lookup_global_type(name)
            }
        }
        use std::any::Any;

        trait Downcastable {
            fn as_any(&self) -> &dyn Any;
        }

        impl<T: Any> Downcastable for T {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        trait Downcast: Downcastable {
            fn downcast_ref<T: Any>(&self) -> Option<&T> {
                self.as_any().downcast_ref::<T>()
            }
        }

        impl<T: Downcastable> Downcast for T {}

        impl AggregateType {
            fn downcast_ref<T: Any>(&self) -> Option<&T> {
                (self as &dyn Downcast).downcast_ref()
            }
        }
    }
}
