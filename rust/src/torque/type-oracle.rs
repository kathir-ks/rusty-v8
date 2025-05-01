// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Mutex;

use once_cell::sync::Lazy;

//use crate::base::contextual::ContextualClass; // Assuming a Rust equivalent for ContextualClass exists
use crate::torque::constants::*;
use crate::torque::declarable::*;
use crate::torque::declarations::*;
use crate::torque::types::*;
use crate::torque::utils::*;

mod base {
    pub mod contextual {
        pub trait ContextualClass<T> {
            fn get() -> &'static T;
        }
    }
}

pub struct TypeOracle {
    function_pointer_types_: Deduplicator<BuiltinPointerType>,
    all_builtin_pointer_types_: Vec<*const BuiltinPointerType>,
    union_types_: Deduplicator<UnionType>,
    nominal_types_: Vec<Box<AbstractType>>,
    aggregate_types_: Vec<Box<AggregateType>>,
    bit_field_struct_types_: Vec<Box<BitFieldStructType>>,
    top_types_: Vec<Box<TopType>>,
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

    pub fn get_abstract_type(
        parent: *const Type,
        name: String,
        flags: AbstractTypeFlags,
        generated: String,
        non_constexpr_version: *const Type,
        specialized_from: MaybeSpecializationKey,
    ) -> *const AbstractType {
        let mut ptr = Box::new(AbstractType::new(
            parent,
            flags,
            name,
            generated,
            non_constexpr_version,
            specialized_from,
        ));
        let result: *const AbstractType = ptr.as_ref();
        if !non_constexpr_version.is_null() {
            unsafe {
                debug_assert!((*ptr).is_constexpr());
                (*non_constexpr_version).set_constexpr_version(result);
            }
        }
        Self::get_mut().nominal_types_.push(ptr);
        result
    }

    pub fn get_struct_type(
        decl: *const StructDeclaration,
        specialized_from: MaybeSpecializationKey,
    ) -> *mut StructType {
        let mut ptr = Box::new(StructType::new(
            current_namespace(),
            decl,
            specialized_from,
        ));
        let result: *mut StructType = ptr.as_mut();
        Self::get_mut().aggregate_types_.push(ptr);
        result
    }

    pub fn get_bit_field_struct_type(
        parent: *const Type,
        decl: *const BitFieldStructDeclaration,
    ) -> *mut BitFieldStructType {
        let mut ptr = Box::new(BitFieldStructType::new(
            current_namespace(),
            parent,
            decl,
        ));
        let result: *mut BitFieldStructType = ptr.as_mut();
        Self::get_mut().bit_field_struct_types_.push(ptr);
        result
    }

    pub fn get_class_type(
        parent: *const Type,
        name: &str,
        flags: ClassFlags,
        generates: &str,
        decl: *mut ClassDeclaration,
        alias: *const TypeAlias,
    ) -> *mut ClassType {
        let mut type_ = Box::new(ClassType::new(
            parent,
            current_namespace(),
            name.to_string(),
            flags,
            generates.to_string(),
            decl,
            alias,
        ));
        let result: *mut ClassType = type_.as_mut();
        Self::get_mut().aggregate_types_.push(type_);
        result
    }

    pub fn get_builtin_pointer_type(
        argument_types: TypeVector,
        return_type: *const Type,
    ) -> *const BuiltinPointerType {
        let mut self_ = Self::get_mut();
        let builtin_type = self_.get_builtin_type(BUILTIN_POINTER_TYPE_STRING);
        let result = self_.function_pointer_types_.add(BuiltinPointerType::new(
            builtin_type,
            argument_types,
            return_type,
            self_.all_builtin_pointer_types_.len(),
        ));

        if result.function_pointer_type_id() == self_.all_builtin_pointer_types_.len() {
            self_.all_builtin_pointer_types_.push(result);
        }
        result
    }

    pub fn get_generic_type_instance(
        generic_type: *mut GenericType,
        arg_types: TypeVector,
    ) -> *const Type {
        unsafe { (*generic_type).get_instance(arg_types) }
    }

    pub fn get_reference_generic(is_const: bool) -> *mut GenericType {
        Declarations::lookup_unique_generic_type(QualifiedName::new(
            vec![TORQUE_INTERNAL_NAMESPACE_STRING.to_string()],
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
        is_const: &mut bool,
    ) -> Option<*const Type> {
        unsafe {
            if let Some(generic_type) =
                Declarations::find_generic_for_type(reference_type, CONST_REFERENCE_TYPE_STRING)
            {
                *is_const = true;
                return Some(generic_type);
            }
            if let Some(generic_type) =
                Declarations::find_generic_for_type(reference_type, MUTABLE_REFERENCE_TYPE_STRING)
            {
                *is_const = false;
                return Some(generic_type);
            }
        }
        None
    }

    pub fn get_mutable_slice_generic() -> *mut GenericType {
        Declarations::lookup_unique_generic_type(QualifiedName::new(
            vec![MUTABLE_SLICE_TYPE_STRING.to_string()],
            "".to_string(),
        ))
    }
    pub fn get_const_slice_generic() -> *mut GenericType {
        Declarations::lookup_unique_generic_type(QualifiedName::new(
            vec![CONST_SLICE_TYPE_STRING.to_string()],
            "".to_string(),
        ))
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
        Self::get_generic_type_instance(Self::get_reference_generic(is_const), vec![referenced_type])
    }

    pub fn get_const_reference_type(referenced_type: *const Type) -> *const Type {
        Self::get_reference_type(referenced_type, true)
    }

    pub fn get_mutable_reference_type(referenced_type: *const Type) -> *const Type {
        Self::get_reference_type(referenced_type, false)
    }

    pub fn get_mutable_slice_type(referenced_type: *const Type) -> *const Type {
        Self::get_generic_type_instance(Self::get_mutable_slice_generic(), vec![referenced_type])
    }

    pub fn get_const_slice_type(referenced_type: *const Type) -> *const Type {
        Self::get_generic_type_instance(Self::get_const_slice_generic(), vec![referenced_type])
    }

    pub fn all_builtin_pointer_types() -> &'static Vec<*const BuiltinPointerType> {
        &Self::get().all_builtin_pointer_types_
    }

    pub fn get_union_type(type_: UnionType) -> *const Type {
        if let Some(single) = type_.get_single_member() {
            return single;
        }
        Self::get_mut().union_types_.add(type_)
    }

    pub fn get_union_type_types(a: *const Type, b: *const Type) -> *const Type {
        unsafe {
            if (*a).is_subtype_of(b) {
                return b;
            }
            if (*b).is_subtype_of(a) {
                return a;
            }
        }
        let mut result = UnionType::from_type(a);
        result.extend(b);
        Self::get_union_type(result)
    }

    pub fn get_top_type(reason: String, source_type: *const Type) -> *const TopType {
        let mut type_ = Box::new(TopType::new(reason, source_type));
        let result: *const TopType = Box::leak(type_);
        Self::get_mut().top_types_.push(Box::from_raw(result as *mut TopType));
        result
    }

    pub fn get_arguments_type() -> *const Type {
        Self::get().get_builtin_type(ARGUMENTS_TYPE_STRING)
    }

    pub fn get_bool_type() -> *const Type {
        Self::get().get_builtin_type(BOOL_TYPE_STRING)
    }

    pub fn get_constexpr_bool_type() -> *const Type {
        Self::get().get_builtin_type(CONSTEXPR_BOOL_TYPE_STRING)
    }

    pub fn get_constexpr_string_type() -> *const Type {
        Self::get().get_builtin_type(CONSTEXPR_STRING_TYPE_STRING)
    }

    pub fn get_constexpr_intptr_type() -> *const Type {
        Self::get().get_builtin_type(CONSTEXPR_INTPTR_TYPE_STRING)
    }

    pub fn get_constexpr_instance_type_type() -> *const Type {
        Self::get().get_builtin_type(CONSTEXPR_INSTANCE_TYPE_TYPE_STRING)
    }

    pub fn get_void_type() -> *const Type {
        Self::get().get_builtin_type(VOID_TYPE_STRING)
    }

    pub fn get_raw_ptr_type() -> *const Type {
        Self::get().get_builtin_type(RAWPTR_TYPE_STRING)
    }

    pub fn get_external_pointer_type() -> *const Type {
        Self::get().get_builtin_type(EXTERNALPTR_TYPE_STRING)
    }

    pub fn get_cpp_heap_pointer_type() -> *const Type {
        Self::get().get_builtin_type(CPPHEAPPTR_TYPE_STRING)
    }

    pub fn get_trusted_pointer_type() -> *const Type {
        Self::get().get_builtin_type(TRUSTEDPTR_TYPE_STRING)
    }

    pub fn get_protected_pointer_type() -> *const Type {
        Self::get().get_builtin_type(PROTECTEDPTR_TYPE_STRING)
    }

    pub fn get_dispatch_handle_type() -> *const Type {
        Self::get().get_builtin_type(DISPATCH_HANDLE_TYPE_STRING)
    }

    pub fn get_map_type() -> *const Type {
        Self::get().get_builtin_type(MAP_TYPE_STRING)
    }

    pub fn get_object_type() -> *const Type {
        Self::get().get_builtin_type(OBJECT_TYPE_STRING)
    }

    pub fn get_heap_object_type() -> *const Type {
        Self::get().get_builtin_type(HEAP_OBJECT_TYPE_STRING)
    }

    pub fn get_tagged_zero_pattern_type() -> *const Type {
        Self::get().get_builtin_type(TAGGED_ZERO_PATTERN_TYPE_STRING)
    }

    pub fn get_js_any_type() -> *const Type {
        Self::get().get_builtin_type(JSANY_TYPE_STRING)
    }

    pub fn get_js_object_type() -> *const Type {
        Self::get().get_builtin_type(JSOBJECT_TYPE_STRING)
    }

    pub fn get_tagged_type() -> *const Type {
        Self::get().get_builtin_type(TAGGED_TYPE_STRING)
    }

    pub fn get_strong_tagged_type() -> *const Type {
        Self::get().get_builtin_type(STRONG_TAGGED_TYPE_STRING)
    }

    pub fn get_uninitialized_type() -> *const Type {
        Self::get().get_builtin_type(UNINITIALIZED_TYPE_STRING)
    }

    pub fn get_uninitialized_heap_object_type() -> *const Type {
        Self::get().get_builtin_type(QualifiedName::new(
            vec![TORQUE_INTERNAL_NAMESPACE_STRING.to_string()],
            UNINITIALIZED_HEAP_OBJECT_TYPE_STRING.to_string(),
        ))
    }

    pub fn get_smi_type() -> *const Type {
        Self::get().get_builtin_type(SMI_TYPE_STRING)
    }

    pub fn get_const_string_type() -> *const Type {
        Self::get().get_builtin_type(CONST_STRING_TYPE_STRING)
    }

    pub fn get_string_type() -> *const Type {
        Self::get().get_builtin_type(STRING_TYPE_STRING)
    }

    pub fn get_number_type() -> *const Type {
        Self::get().get_builtin_type(NUMBER_TYPE_STRING)
    }

    pub fn get_intptr_type() -> *const Type {
        Self::get().get_builtin_type(INTPTR_TYPE_STRING)
    }

    pub fn get_uintptr_type() -> *const Type {
        Self::get().get_builtin_type(UINTPTR_TYPE_STRING)
    }

    pub fn get_int64_type() -> *const Type {
        Self::get().get_builtin_type(INT64_TYPE_STRING)
    }

    pub fn get_uint64_type() -> *const Type {
        Self::get().get_builtin_type(UINT64_TYPE_STRING)
    }

    pub fn get_int32_type() -> *const Type {
        Self::get().get_builtin_type(INT32_TYPE_STRING)
    }

    pub fn get_uint32_type() -> *const Type {
        Self::get().get_builtin_type(UINT32_TYPE_STRING)
    }

    pub fn get_uint31_type() -> *const Type {
        Self::get().get_builtin_type(UINT31_TYPE_STRING)
    }

    pub fn get_int16_type() -> *const Type {
        Self::get().get_builtin_type(INT16_TYPE_STRING)
    }

    pub fn get_uint16_type() -> *const Type {
        Self::get().get_builtin_type(UINT16_TYPE_STRING)
    }

    pub fn get_int8_type() -> *const Type {
        Self::get().get_builtin_type(INT8_TYPE_STRING)
    }

    pub fn get_uint8_type() -> *const Type {
        Self::get().get_builtin_type(UINT8_TYPE_STRING)
    }

    pub fn get_float64_type() -> *const Type {
        Self::get().get_builtin_type(FLOAT64_TYPE_STRING)
    }

    pub fn get_float64_or_undefined_or_hole_type() -> *const Type {
        Self::get().get_builtin_type(FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING)
    }

    pub fn get_const_float64_type() -> *const Type {
        Self::get().get_builtin_type(CONST_FLOAT64_TYPE_STRING)
    }

    pub fn get_integer_literal_type() -> *const Type {
        Self::get().get_builtin_type(INTEGER_LITERAL_TYPE_STRING)
    }

    pub fn get_never_type() -> *const Type {
        Self::get().get_builtin_type(NEVER_TYPE_STRING)
    }

    pub fn get_const_int31_type() -> *const Type {
        Self::get().get_builtin_type(CONST_INT31_TYPE_STRING)
    }

    pub fn get_const_int32_type() -> *const Type {
        Self::get().get_builtin_type(CONST_INT32_TYPE_STRING)
    }

    pub fn get_context_type() -> *const Type {
        Self::get().get_builtin_type(CONTEXT_TYPE_STRING)
    }

    pub fn get_no_context_type() -> *const Type {
        Self::get().get_builtin_type(NO_CONTEXT_TYPE_STRING)
    }

    pub fn get_native_context_type() -> *const Type {
        Self::get().get_builtin_type(NATIVE_CONTEXT_TYPE_STRING)
    }

    pub fn get_js_function_type() -> *const Type {
        Self::get().get_builtin_type(JS_FUNCTION_TYPE_STRING)
    }

    pub fn get_uninitialized_iterator_type() -> *const Type {
        Self::get().get_builtin_type(UNINITIALIZED_ITERATOR_TYPE_STRING)
    }

    pub fn get_fixed_array_base_type() -> *const Type {
        Self::get().get_builtin_type(FIXED_ARRAY_BASE_TYPE_STRING)
    }

    pub fn implicitly_convertable_from(to: *const Type, from: *const Type) -> Option<*const Type> {
        let mut current_from = from;
        while !current_from.is_null() {
            for from_constexpr in Declarations::lookup_generic(kFromConstexprMacroName) {
                unsafe {
                    if let Some(specialization) =
                        (*from_constexpr).get_specialization(vec![to, current_from])
                    {
                        if (*specialization).signature().get_explicit_types() == vec![from] {
                            return Some(from);
                        }
                    }
                }
            }
            unsafe {
                current_from = (*current_from).parent();
            }
        }
        None
    }

    pub fn get_aggregate_types() -> &'static Vec<Box<AggregateType>> {
        &Self::get().aggregate_types_
    }

    pub fn get_bit_field_struct_types() -> &'static Vec<Box<BitFieldStructType>> {
        &Self::get().bit_field_struct_types_
    }

    // By construction, this list of all classes is topologically sorted w.r.t.
    // inheritance.
    pub fn get_classes() -> Vec<*const ClassType> {
        Self::get()
            .aggregate_types_
            .iter()
            .filter_map(|aggregate_type| {
                if let AggregateType::Class(class_type) = aggregate_type {
                    Some(class_type as *const ClassType)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn finalize_aggregate_types() {
        todo!()
    }

    pub fn fresh_type_id() -> usize {
        let mut self_ = Self::get_mut();
        let id = self_.next_type_id_;
        self_.next_type_id_ += 1;
        id
    }

    pub fn create_generic_type_instantiation_namespace() -> *mut Namespace {
        todo!()
    }

    fn get_builtin_type(&self, name: &str) -> *const Type {
        Declarations::lookup_global_type(QualifiedName::new(vec![name.to_string()], "".to_string()))
    }

    fn get_builtin_type_qualified(&self, name: QualifiedName) -> *const Type {
        Declarations::lookup_global_type(name)
    }

    fn get() -> &'static TypeOracle {
        static ORACLE: Lazy<TypeOracle> = Lazy::new(|| TypeOracle::new());
        &ORACLE
    }

    fn get_mut() -> &'static mut TypeOracle {
        static mut ORACLE: Lazy<TypeOracle> = Lazy::new(|| TypeOracle::new());
        unsafe { &mut ORACLE }
    }
}

pub struct Deduplicator<T> {
    map: Mutex<HashMap<u64, *const T>>,
}

impl<T: Hash> Deduplicator<T> {
    pub fn new() -> Self {
        Deduplicator {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(&mut self, value: T) -> *const T {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let key = hasher.finish();

        let mut map = self.map.lock().unwrap();
        if let Some(ptr) = map.get(&key) {
            return *ptr;
        }

        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        map.insert(key, ptr);
        ptr
    }
}

unsafe impl<T> Sync for Deduplicator<T> {}

// Placeholder functions - replace with actual implementations
fn current_namespace() -> *const Namespace {
    std::ptr::null() // Replace with actual namespace retrieval logic
}

mod torque {
    pub mod utils {
        pub type TypeVector = Vec<*const super::Type>; // Replace with actual Type definition
    }
}