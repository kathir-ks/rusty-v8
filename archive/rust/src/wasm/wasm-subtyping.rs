// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/wasm/wasm-subtyping.h (converted to module definition)
pub mod wasm_subtyping {
    use crate::wasm::canonical_types::CanonicalTypeIndex;
    use crate::wasm::wasm_module::WasmModule;
    use crate::wasm::value_type::{HeapType, Nullability, RefTypeKind, StandardType, TypeDefinition, TypeInModule, ValueType, ValueTypeBase, kNoSuperType, kNullable, kNonNullable, kWasmBottom, kWasmTop};
    use crate::wasm::function_sig::FunctionSig;
    use crate::wasm::struct_type::StructType;
    use crate::wasm::array_type::ArrayType;
    use crate::wasm::cont_type::ContType;
    use crate::wasm::type_canonicalizer::GetTypeCanonicalizer;
    use std::option::Option;
    use std::option::Option::Some;
    use std::vec::Vec;

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum GenericKind {
        kVoid, // Placeholder
        kEq,
        kAny,
        kStruct,
        kNone,
        kArray,
        kI31,
        kString,
        kFunc,
        kNoFunc,
        kExternString,
        kExtern,
        kNoExtern,
        kNoExn,
        kExn,
        kNoCont,
        kCont,
    }
    
    const K_NUM_STANDARD_TYPES: usize = 32;

    macro_rules! make_array {
        ($size:expr, $f:expr) => {{
            let mut result = Vec::with_capacity($size);
            for i in 0..$size {
                result.push($f(i));
            }
            result.into_boxed_slice()
        }};
    }

    // Helper function since C++ V8 codebase uses V8_EXPORT_PRIVATE
    // this can be replaced by pub for public functions
    pub fn is_subtype_of(
        subtype: ValueType,
        supertype: ValueType,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        IsSubtypeOfImpl(subtype, supertype, sub_module, super_module)
    }

    pub fn is_heap_subtype_of(
        subtype: HeapType,
        supertype: HeapType,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        IsSubtypeOfImpl(subtype, supertype, sub_module, super_module)
    }

    pub fn equivalent_types(
        type1: ValueType,
        type2: ValueType,
        module1: &WasmModule,
        module2: &WasmModule,
    ) -> bool {
        EquivalentTypes(type1, type2, module1, module2)
    }

    pub fn to_null_sentinel(type_in_module: TypeInModule) -> ValueType {
        ToNullSentinel(type_in_module)
    }

    pub fn is_same_type_hierarchy(
        type1: HeapType,
        type2: HeapType,
        module: &WasmModule,
    ) -> bool {
        IsSameTypeHierarchy(type1, type2, module)
    }

    pub fn intersection(
        type1: ValueType,
        type2: ValueType,
        module1: &WasmModule,
        module2: &WasmModule,
    ) -> TypeInModule {
        Intersection(type1, type2, module1, module2)
    }

    pub fn union(
        type1: ValueType,
        type2: ValueType,
        module1: &WasmModule,
        module2: &WasmModule,
    ) -> TypeInModule {
        Union(type1, type2, module1, module2)
    }

    fn equivalent_indices(
        index1: ModuleTypeIndex,
        index2: ModuleTypeIndex,
        module1: &WasmModule,
        module2: &WasmModule,
    ) -> bool {
        if index1 == index2 && module1 == module2 {
            return true;
        }
        module1.canonical_type_id(index1) == module2.canonical_type_id(index2)
    }

    fn valid_struct_subtype_definition(
        subtype_index: ModuleTypeIndex,
        supertype_index: ModuleTypeIndex,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        let sub_struct = sub_module.type_at(subtype_index).struct_type.as_ref().unwrap();
        let super_struct = super_module.type_at(supertype_index).struct_type.as_ref().unwrap();

        if sub_struct.field_count() < super_struct.field_count() {
            return false;
        }

        for i in 0..super_struct.field_count() {
            let sub_mut = sub_struct.mutability(i);
            let super_mut = super_struct.mutability(i);
            if sub_mut != super_mut
                || (sub_mut
                    && !EquivalentTypes(
                        sub_struct.field(i),
                        super_struct.field(i),
                        sub_module,
                        super_module,
                    ))
                || (!sub_mut
                    && !IsSubtypeOfImpl(
                        sub_struct.field(i),
                        super_struct.field(i),
                        sub_module,
                        super_module,
                    ))
            {
                return false;
            }
        }
        true
    }

    fn valid_array_subtype_definition(
        subtype_index: ModuleTypeIndex,
        supertype_index: ModuleTypeIndex,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        let sub_array = sub_module.type_at(subtype_index).array_type.as_ref().unwrap();
        let super_array = super_module.type_at(supertype_index).array_type.as_ref().unwrap();
        let sub_mut = sub_array.mutability();
        let super_mut = super_array.mutability();

        (sub_mut && super_mut && EquivalentTypes(
            sub_array.element_type(),
            super_array.element_type(),
            sub_module,
            super_module,
        )) || (!sub_mut && !super_mut && IsSubtypeOfImpl(
            sub_array.element_type(),
            super_array.element_type(),
            sub_module,
            super_module,
        ))
    }

    fn valid_function_subtype_definition(
        subtype_index: ModuleTypeIndex,
        supertype_index: ModuleTypeIndex,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        let sub_func = sub_module.type_at(subtype_index).function_sig.as_ref().unwrap();
        let super_func = super_module.type_at(supertype_index).function_sig.as_ref().unwrap();

        if sub_func.parameter_count() != super_func.parameter_count()
            || sub_func.return_count() != super_func.return_count()
        {
            return false;
        }

        for i in 0..sub_func.parameter_count() {
            // Contravariance for params.
            if !IsSubtypeOfImpl(
                super_func.parameters()[i],
                sub_func.parameters()[i],
                super_module,
                sub_module,
            ) {
                return false;
            }
        }
        for i in 0..sub_func.return_count() {
            // Covariance for returns.
            if !IsSubtypeOfImpl(
                sub_func.returns()[i],
                super_func.returns()[i],
                sub_module,
                super_module,
            ) {
                return false;
            }
        }

        true
    }

    fn valid_continuation_subtype_definition(
        subtype_index: ModuleTypeIndex,
        supertype_index: ModuleTypeIndex,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        let sub_cont = sub_module.type_at(subtype_index).cont_type.as_ref().unwrap();
        let super_cont = super_module.type_at(supertype_index).cont_type.as_ref().unwrap();

        IsSubtypeOfImpl(
            sub_module.heap_type(sub_cont.contfun_typeindex()),
            super_module.heap_type(super_cont.contfun_typeindex()),
            sub_module,
            super_module,
        )
    }

    // For some purposes, we can treat all custom structs like the generic
    // "structref", etc.
    fn upcast_to_standard_type(type_: ValueTypeBase) -> StandardType {
        if !type_.has_index() {
            return type_.standard_type();
        }
        match type_.ref_type_kind() {
            RefTypeKind::kStruct => StandardType::kStruct,
            RefTypeKind::kArray => StandardType::kArray,
            RefTypeKind::kFunction => StandardType::kFunc,
            RefTypeKind::kCont => StandardType::kCont,
            RefTypeKind::kOther => panic!("UNREACHABLE"),
        }
    }

    macro_rules! foreach_subtyping {
        ($V:ident) => {
            // anyref hierarchy
            $V!(Eq, Any);
            $V!(Struct, Eq);
            $V!(None, Struct);
            $V!(Array, Eq);
            $V!(None, Array);
            $V!(I31, Eq);
            $V!(None, I31);
            $V!(String, Any);
            $V!(None, String);
            // funcref hierarchy
            $V!(NoFunc, Func);
            // extern hierarchy
            $V!(ExternString, Extern);
            $V!(NoExtern, ExternString);
            // exnref hierarchy
            $V!(NoExn, Exn);
            // cont hierarchy
            $V!(NoCont, Cont);
        };
    }

    const K_NUM_NON_TRIVIAL: usize = 14;

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    enum CondensedIndices {
        kEq,
        kEqAdjustNext,
        kAny,
        kAnyAdjustNext,
        kStruct,
        kStructAdjustNext,
        kNone,
        kNoneAdjustNext,
        kArray,
        kArrayAdjustNext,
        kI31,
        kI31AdjustNext,
        kString,
        kStringAdjustNext,
        kFunc,
        kFuncAdjustNext,
        kNoFunc,
        kNoFuncAdjustNext,
        kExternString,
        kExternStringAdjustNext,
        kExtern,
        kExternAdjustNext,
        kNoExtern,
        kNoExternAdjustNext,
        kNoExn,
        kNoExnAdjustNext,
        kExn,
        kExnAdjustNext,
        kNoCont,
        kNoContAdjustNext,
        kCont,
        kContAdjustNext,
    }

    const K_NOT_RELATED_SENTINEL: u8 = 0xFF;

    fn compute_condensed_index(type_: StandardType) -> u8 {
        match type_ {
            StandardType::kI32 | StandardType::kI64 | StandardType::kF32 | StandardType::kF64 | StandardType::kS128 =>
                K_NOT_RELATED_SENTINEL,
            StandardType::kEq => CondensedIndices::kEq as u8,
            StandardType::kAny => CondensedIndices::kAny as u8,
            StandardType::kStruct => CondensedIndices::kStruct as u8,
            StandardType::kNone => CondensedIndices::kNone as u8,
            StandardType::kArray => CondensedIndices::kArray as u8,
            StandardType::kI31 => CondensedIndices::kI31 as u8,
            StandardType::kString => CondensedIndices::kString as u8,
            StandardType::kFunc => CondensedIndices::kFunc as u8,
            StandardType::kNoFunc => CondensedIndices::kNoFunc as u8,
            StandardType::kExternString => CondensedIndices::kExternString as u8,
            StandardType::kExtern => CondensedIndices::kExtern as u8,
            StandardType::kNoExtern => CondensedIndices::kNoExtern as u8,
            StandardType::kNoExn => CondensedIndices::kNoExn as u8,
            StandardType::kExn => CondensedIndices::kExn as u8,
            StandardType::kNoCont => CondensedIndices::kNoCont as u8,
            StandardType::kCont => CondensedIndices::kCont as u8,
            _ => K_NOT_RELATED_SENTINEL,
        }
    }

    fn compute_standard_type(condensed_index: u8) -> StandardType {
        if compute_condensed_index(StandardType::kEq) == condensed_index {
            return StandardType::kEq;
        }
        if compute_condensed_index(StandardType::kAny) == condensed_index {
            return StandardType::kAny;
        }
        if compute_condensed_index(StandardType::kStruct) == condensed_index {
            return StandardType::kStruct;
        }
        if compute_condensed_index(StandardType::kNone) == condensed_index {
            return StandardType::kNone;
        }
        if compute_condensed_index(StandardType::kArray) == condensed_index {
            return StandardType::kArray;
        }
        if compute_condensed_index(StandardType::kI31) == condensed_index {
            return StandardType::kI31;
        }
        if compute_condensed_index(StandardType::kString) == condensed_index {
            return StandardType::kString;
        }
         if compute_condensed_index(StandardType::kFunc) == condensed_index {
            return StandardType::kFunc;
        }
        if compute_condensed_index(StandardType::kNoFunc) == condensed_index {
            return StandardType::kNoFunc;
        }
        if compute_condensed_index(StandardType::kExternString) == condensed_index {
            return StandardType::kExternString;
        }
        if compute_condensed_index(StandardType::kExtern) == condensed_index {
            return StandardType::kExtern;
        }
        if compute_condensed_index(StandardType::kNoExtern) == condensed_index {
            return StandardType::kNoExtern;
        }
        if compute_condensed_index(StandardType::kNoExn) == condensed_index {
            return StandardType::kNoExn;
        }
        if compute_condensed_index(StandardType::kExn) == condensed_index {
            return StandardType::kExn;
        }
        if compute_condensed_index(StandardType::kNoCont) == condensed_index {
            return StandardType::kNoCont;
        }
        if compute_condensed_index(StandardType::kCont) == condensed_index {
            return StandardType::kCont;
        }
        panic!("UNREACHABLE");
    }

    lazy_static::lazy_static! {
        static ref K_CONDENSED_INDEX_LOOKUP_MAP: Box<[u8; K_NUM_STANDARD_TYPES]> = {
            make_array!(K_NUM_STANDARD_TYPES, |i| compute_condensed_index(unsafe { std::mem::transmute::<usize, StandardType>(i) }))
        };
        static ref K_CONDENSED_TO_STANDARD_MAP: Box<[u8; K_NUM_NON_TRIVIAL]> = {
            make_array!(K_NUM_NON_TRIVIAL, |i| compute_standard_type(i as u8) as u8)
        };
    }

    fn condensed_index(type_: StandardType) -> u8 {
        K_CONDENSED_INDEX_LOOKUP_MAP[type_ as usize]
    }

    fn condensed_to_standard(condensed: u8) -> StandardType {
        compute_standard_type(condensed)
    }

    fn compute_is_subtype(sub: usize, super_: usize) -> bool {
        if sub == super_ {
            return true;
        }
        macro_rules! case {
            ($a:ident, $b:ident) => {
                let raw_a = CondensedIndices::k##$a as usize;
                let raw_b = CondensedIndices::k##$b as usize;
                if sub == raw_a {
                    if super_ == raw_b {
                        return true;
                    }
                    if compute_is_subtype(raw_b, super_) {
                        return true;
                    }
                }
            };
        }
        foreach_subtyping!(case);
        false
    }

    lazy_static::lazy_static! {
        static ref K_SUBTYPE_LOOKUP_MAP2: Box<[[bool; K_NUM_NON_TRIVIAL]; K_NUM_NON_TRIVIAL]> = {
            let mut arr = Box::new([[false; K_NUM_NON_TRIVIAL]; K_NUM_NON_TRIVIAL]);
            for sub in 0..K_NUM_NON_TRIVIAL {
                for super_ in 0..K_NUM_NON_TRIVIAL {
                    arr[sub][super_] = compute_is_subtype(sub, super_);
                }
            }
            arr
        };
    }

    fn subtype_lookup(sub: StandardType, super_: StandardType) -> bool {
        if sub == StandardType::kBottom {
            return true;
        }
        if super_ == StandardType::kTop {
            return true;
        }
        let sub_condensed = condensed_index(sub);
        if sub_condensed == K_NOT_RELATED_SENTINEL {
            return sub == super_;
        }
        let super_condensed = condensed_index(super_);
        if super_condensed == K_NOT_RELATED_SENTINEL {
            return false;
        }
        K_SUBTYPE_LOOKUP_MAP2[sub_condensed as usize][super_condensed as usize]
    }

    fn compute_common_ancestor(t1: usize, t2: usize) -> StandardType {
        if K_SUBTYPE_LOOKUP_MAP2[t1][t2] {
            return condensed_to_standard(t2 as u8);
        }
        if K_SUBTYPE_LOOKUP_MAP2[t2][t1] {
            return condensed_to_standard(t1 as u8);
        }
        macro_rules! case {
            ($a:ident, $b:ident) => {
                if t1 == CondensedIndices::k##$a as usize {
                    return compute_common_ancestor(
                        CondensedIndices::k##$b as usize,
                        t2,
                    );
                }
            };
        }
        foreach_subtyping!(case);
        StandardType::kTop
    }

    lazy_static::lazy_static! {
        static ref K_COMMON_ANCESTOR_LOOKUP_MAP: Box<[[StandardType; K_NUM_NON_TRIVIAL]; K_NUM_NON_TRIVIAL]> = {
            let mut arr = Box::new([[StandardType::kBottom; K_NUM_NON_TRIVIAL]; K_NUM_NON_TRIVIAL]);
            for sub in 0..K_NUM_NON_TRIVIAL {
                for super_ in 0..K_NUM_NON_TRIVIAL {
                    arr[sub][super_] = compute_common_ancestor(sub, super_);
                }
            }
            arr
        };
    }

    fn common_ancestor_lookup(t1: StandardType, t2: StandardType) -> StandardType {
        if t1 == StandardType::kBottom {
            return t2;
        }
        if t2 == StandardType::kBottom {
            return t1;
        }
        if t1 == StandardType::kTop {
            return t1;
        }
        if t2 == StandardType::kTop {
            return t2;
        }
        let t1_condensed = condensed_index(t1);
        if t1_condensed == K_NOT_RELATED_SENTINEL {
            return if t2 == t1 { t1 } else { StandardType::kTop };
        }
        let t2_condensed = condensed_index(t2);
        if t2_condensed == K_NOT_RELATED_SENTINEL {
            return StandardType::kTop;
        }
        K_COMMON_ANCESTOR_LOOKUP_MAP[t1_condensed as usize][t2_condensed as usize]
    }

    fn null_sentinel_impl(type_: HeapType) -> HeapType {
        let standard = upcast_to_standard_type(type_);
        const CANDIDATES: [StandardType; 4] = [
            StandardType::kNone,
            StandardType::kNoFunc,
            StandardType::kNoExtern,
            StandardType::kNoExn,
        ];
        for candidate in CANDIDATES.iter() {
            if subtype_lookup(*candidate, standard) {
                return HeapType::Generic(to_generic_kind(*candidate), type_.is_shared());
            }
        }
        if type_.is_string_view() {
            // TODO(12868): This special case reflects unresolved discussion. If string
            // views aren't nullable, they shouldn't really show up here at all.
            return HeapType::Generic(GenericKind::kNone, type_.is_shared());
        }
        panic!("UNREACHABLE");
    }

    fn is_null_sentinel(type_: HeapType) -> bool {
        if type_.has_index() {
            return false;
        }
        is_null_kind(type_.generic_kind())
    }

    fn is_generic_subtype_of_indexed_types(type_: ValueTypeBase) -> bool {
        if !type_.is_generic() {
            return false;
        }
        let kind = type_.generic_kind();
        is_null_kind(kind) || kind == GenericKind::kVoid
    }

    pub fn valid_subtype_definition(
        subtype_index: ModuleTypeIndex,
        supertype_index: ModuleTypeIndex,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        let subtype = sub_module.type_at(subtype_index);
        let supertype = super_module.type_at(supertype_index);
        if subtype.kind != supertype.kind {
            return false;
        }
        if supertype.is_final {
            return false;
        }
        if subtype.is_shared != supertype.is_shared {
            return false;
        }
        match subtype.kind {
            TypeDefinition::kFunction => valid_function_subtype_definition(
                subtype_index,
                supertype_index,
                sub_module,
                super_module,
            ),
            TypeDefinition::kStruct => valid_struct_subtype_definition(
                subtype_index,
                supertype_index,
                sub_module,
                super_module,
            ),
            TypeDefinition::kArray => valid_array_subtype_definition(
                subtype_index,
                supertype_index,
                sub_module,
                super_module,
            ),
            TypeDefinition::kCont => valid_continuation_subtype_definition(
                subtype_index,
                supertype_index,
                sub_module,
                super_module,
            ),
        }
    }

    // Common parts of the implementation for ValueType and CanonicalValueType.
    fn is_subtype_of_abstract(
        subtype: ValueTypeBase,
        supertype: ValueTypeBase,
    ) -> Option<bool> {
        if subtype.is_numeric() || supertype.is_numeric() {
            return None;
        }

        if subtype.is_shared() != supertype.is_shared() {
            return Some(false);
        }
        if supertype.has_index() {
            // If both types are indexed, the specialized implementations need to
            // take care of it.
            if subtype.has_index() {
                return None;
            }
            // Subtype is generic. It can only be a subtype if it is a none-type.
            if !is_generic_subtype_of_indexed_types(subtype) {
                return Some(false);
            }
        }
        Some(subtype_lookup(
            upcast_to_standard_type(subtype),
            upcast_to_standard_type(supertype),
        ))
    }

    // Helper function since C++ V8 codebase uses V8_NOINLINE V8_EXPORT_PRIVATE
    // These functions should not be inlined and is meant to be public external
    #[no_inline]
    fn IsSubtypeOfImpl(
        subtype: HeapType,
        supertype: HeapType,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        if subtype == supertype && sub_module == super_module {
            return true;
        }

        let result = is_subtype_of_abstract(subtype, supertype);
        if let Some(res) = result {
            return res;
        }
        
        if !subtype.has_index() || !supertype.has_index() {
            return false;
        }

        let sub_index = subtype.ref_index();
        let super_canon = super_module.canonical_type_id(supertype.ref_index());

        // Comparing canonicalized type indices handles both different modules
        // and different recgroups in the same module.
        let mut current_index = sub_index;
        loop {
            if sub_module.canonical_type_id(current_index) == super_canon {
                return true;
            }
            if current_index == sub_module.supertype(current_index) {
                break;
            }
            current_index = sub_module.supertype(current_index);

            if !current_index.valid() {
                break;
            }
        }
        false
    }

    #[no_inline]
    fn IsSubtypeOfImpl(
        subtype: ValueType,
        supertype: ValueType,
        sub_module: &WasmModule,
        super_module: &WasmModule,
    ) -> bool {
        if supertype.is_top() {
            return true;
        }
        if subtype.is_numeric() {
            return subtype == supertype;
        }
        if supertype.is_numeric() {
            return subtype.is_bottom();
        }
        if subtype.is_nullable() && !supertype.is_nullable() {
            return false;
        }
        let sub_heap = subtype.heap_type();
        let super_heap = supertype.heap_type();
        if sub_heap == super_heap && sub_module == super_module {
            return true;
        }
        IsSubtypeOfImpl(sub_heap, super_heap, sub_module, super_module)
    }

    #[no_inline]
    fn IsSubtypeOfImpl(
        subtype: CanonicalValueType,
        supertype: CanonicalValueType,
    ) -> bool {
        if subtype == supertype {
            return true;
        }
        if supertype.is_top() {
            return true;
        }
        if subtype.is_numeric() {
            return false;
        }
        if supertype.is_numeric() {
            return subtype.is_bottom();
        }
        if subtype.is_nullable() && !supertype.is_nullable() {
            return false;
        }

        let result = is_subtype_of_abstract(subtype, supertype);
        if let Some(res) = result {
            return res;
        }
        
        if !subtype.has_index() || !supertype.has_index() {
            return false;
        }

        let sub_index = subtype.ref_index();
        let super_index = supertype.ref_index();

        // Can happen despite subtype != supertype, e.g. when nullability differs.
        if sub_index == super_index {
            return true;
        }

        GetTypeCanonicalizer().is_heap_subtype(sub_index, super_index)
    }

    #[no_inline]
    fn EquivalentTypes(
        type1: ValueType,
        type2: ValueType,
        module1: &WasmModule,
        module2: &WasmModule,
    ) -> bool {
        if type1 == type2 && module1 == module2 {
            return true;
        }
        if !type1.has_index() || !type2.has_index() {
            return type1 == type2;
        }
        if type1.nullability() != type2.nullability() {
            return false;
        }

        if !type1.has_index() || !module1.has_type(type1.ref_index()) || !type2.has_index() || !module2.has_type(type2.ref_index()) {
            return false;
        }
        
        equivalent_indices(type1.ref_index(), type2.ref_index(), module1, module2)
    }

    // Returns the least common ancestor of two type indices, as a type index in
    // {module1}.
    fn common_ancestor(
        type1: HeapType,
        type2: HeapType,
        module1: &WasmModule,
        module2: &WasmModule,
    ) -> HeapType {
        if !type1.has_index() || !type2.has_index() {
            return HeapType {
                encoded: kWasmTop.encoded
            };
        }
        let both_shared = type1.is_shared();
        if both_shared != type2.is_shared() {
            return HeapType {
                encoded: kWasmTop.encoded
            };
        }

        let mut type_index1 = type1.ref_index();
        let mut type_index2 = type2.ref_index();
        {
            let mut depth1 = get_subtyping_depth(module1, type_index1);
            let mut depth2 = get_subtyping_depth(module2, type_index2);
            while depth1 > depth2 {
                type_index1 = module1.supertype(type_index1);
                depth1 -= 1;
            }
            while depth2 > depth1 {
                type_index2 = module2.supertype(type_index2);
                depth2 -= 1;
            }
        }

        if type_index1 == kNoSuperType || type_index2 == kNoSuperType {
            return HeapType {
                encoded: kWasmTop.encoded
            };
        }

        while !(type_index1 == type_index2 && module1 == module2)
            && !equivalent_indices(type_index1, type_index2, module1, module2)
        {
            type_index1 = module1.supertype(type_index1);
            type_index2 = module2.supertype(type_index2);

            if type_index1 == kNoSuperType || type_index2 == kNoSuperType {
                return HeapType {
                    encoded: kWasmTop.encoded
                };
            }
        }

        if type_index1 == kNoSuperType && type_index2 == kNoSuperType {
            return HeapType {
                encoded: kWasmTop.encoded
            };
        }

        let kind1 = type1.ref_type_kind();
        let kind2 = type2.ref_type_kind();

        if type_index1 != kNoSuperType {
             if both_shared {
                return HeapType::Index(type_index1, true, kind1);
             }
             else {
                return HeapType::Index(type_index1, false, kind1);
             }
        }
        // No indexed type was found as common ancestor, so we can treat both types
        // as generic.
        let generic_ancestor = common_ancestor_lookup(
            upcast_to_standard_type(type1),
            upcast_to_standard_type(type2),
        );
        HeapType::Generic(to_generic_kind(generic_ancestor), both_shared)
    }

    // Returns the least common ancestor of an abstract heap type {type1}, and
    // another heap type {type2}.
    fn common_ancestor_with_abstract(
        heap1: HeapType,
        heap2: HeapType,
        module2: &WasmModule,
    ) -> HeapType {
        if !heap1.is_abstract_ref() {
            return HeapType {
                encoded: kWasmTop.encoded
            };
        }
        let is_shared = heap1.is_shared();
        if is_shared != heap2.is_shared() {
            return HeapType {
                encoded: kWasmTop.encoded
            };
        }

        // If {heap2} is an indexed type, then {heap1} could be a subtype of it if
        // it is a none-type. In that case, {heap2} is the common ancestor.
        let is_sub = is_subtype_of_abstract(heap1, heap2);
        if is_sub.is_none() {
             return HeapType {
