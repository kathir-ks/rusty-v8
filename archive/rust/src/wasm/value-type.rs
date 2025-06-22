// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/wasm/value-type.h equivalent (module definition)
pub mod value_type {
    use std::fmt;
    use std::fmt::Display;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum NumericKind {
        I32,
        I64,
        F32,
        F64,
        V128,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum GenericKind {
        None,
        Extern,
        Func,
        Any,
        Eq,
        Struct,
        Array,
        I31Ref,
        StringViewUtf8,
        StringViewUtf16,
        StringViewWtf8,
        NoExtern,
        NoFunc,
        NoExn,
        AnyRef,
        DataRef,
        Cont,
        NoCont,
        Void,
        Bottom,
        Top,
    }

    pub const kGenericKindMask: u32 = 0xFF;
    pub const kNumericKindMask: u32 = 0x0F;

    macro_rules! check_mask_generic {
        ($kind:ident) => {
            const _: () = assert!(GenericKind::k_$kind as u32 == (GenericKind::k_$kind as u32 & kGenericKindMask));
        };
    }

    macro_rules! check_mask_numeric {
        ($kind:ident) => {
            const _: () = assert!(NumericKind::k_$kind as u32 == (NumericKind::k_$kind as u32 & kNumericKindMask));
        };
    }

    macro_rules! foreach_generic_type {
        ($macro:ident) => {
            $macro!(None);
            $macro!(Extern);
            $macro!(Func);
            $macro!(Any);
            $macro!(Eq);
            $macro!(Struct);
            $macro!(Array);
            $macro!(I31Ref);
            $macro!(StringViewUtf8);
            $macro!(StringViewUtf16);
            $macro!(StringViewWtf8);
            $macro!(NoExtern);
            $macro!(NoFunc);
            $macro!(NoExn);
            $macro!(AnyRef);
            $macro!(DataRef);
            $macro!(Cont);
            $macro!(NoCont);
            $macro!(Void);
            $macro!(Bottom);
            $macro!(Top);
        };
    }

    macro_rules! foreach_numeric_value_type {
        ($macro:ident) => {
            $macro!(I32);
            $macro!(I64);
            $macro!(F32);
            $macro!(F64);
            $macro!(V128);
        };
    }

    foreach_generic_type!(check_mask_generic);
    foreach_numeric_value_type!(check_mask_numeric);

    pub fn to_zero_based_index(kind: NumericKind) -> u32 {
        match kind {
            NumericKind::I32 => 0,
            NumericKind::I64 => 1,
            NumericKind::F32 => 2,
            NumericKind::F64 => 3,
            NumericKind::V128 => 4,
        }
    }

    #[test]
    fn test_to_zero_based_index() {
        assert_eq!(to_zero_based_index(NumericKind::I32), 0);
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ValueTypeCode {
        I32,
        I64,
        F32,
        F64,
        V128,
        AnyRef,
        DataRef,
        FuncRef,
        ExternRef,
        ExnRef,
        StringViewUtf8,
        StringViewUtf16,
        StringViewWtf8,
        I31Ref,
        EqRef,
        StructRef,
        ArrayRef,
        ContRef,
        Bottom,
        Top,
        Void,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ValueTypeBase {
        kind: ValueTypeKind,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    enum ValueTypeKind {
        Numeric(NumericKind),
        Generic(GenericKind, bool, bool), // GenericKind, nullable, shared
        Indexed(u32, bool), // index, nullable
    }

    impl ValueTypeBase {
        pub const fn numeric(kind: NumericKind) -> Self {
            ValueTypeBase { kind: ValueTypeKind::Numeric(kind) }
        }

        pub const fn generic(kind: GenericKind, nullable: bool, shared: bool) -> Self {
            ValueTypeBase { kind: ValueTypeKind::Generic(kind, nullable, shared) }
        }

        pub const fn indexed(index: u32, nullable: bool) -> Self {
            ValueTypeBase { kind: ValueTypeKind::Indexed(index, nullable) }
        }

        pub fn is_numeric(&self) -> bool {
            matches!(self.kind, ValueTypeKind::Numeric(_))
        }

        pub fn is_generic(&self) -> bool {
            matches!(self.kind, ValueTypeKind::Generic(_, _, _))
        }

        pub fn is_bottom(&self) -> bool {
            match self.kind {
                ValueTypeKind::Generic(GenericKind::Bottom, _, _) => true,
                _ => false,
            }
        }

        pub fn is_top(&self) -> bool {
            match self.kind {
                ValueTypeKind::Generic(GenericKind::Top, _, _) => true,
                _ => false,
            }
        }

        pub fn is_nullable(&self) -> bool {
            match self.kind {
                ValueTypeKind::Generic(_, nullable, _) => nullable,
                ValueTypeKind::Indexed(_, nullable) => nullable,
                _ => false,
            }
        }

        pub fn is_shared(&self) -> bool {
            match self.kind {
                ValueTypeKind::Generic(_, _, shared) => shared,
                _ => false,
            }
        }

        pub fn is_sentinel(&self) -> bool {
            match self.kind {
                ValueTypeKind::Generic(kind, _, _) => {
                    kind == GenericKind::NoExtern || kind == GenericKind::NoFunc || kind == GenericKind::NoExn || kind == GenericKind::NoCont
                }
                _ => false,
            }
        }

        pub fn is_string_view(&self) -> bool {
            match self.kind {
                ValueTypeKind::Generic(kind, _, _) => {
                    kind == GenericKind::StringViewUtf8 || kind == GenericKind::StringViewUtf16 || kind == GenericKind::StringViewWtf8
                }
                _ => false,
            }
        }

        pub fn has_index(&self) -> bool {
            matches!(self.kind, ValueTypeKind::Indexed(_, _))
        }

        pub fn raw_index(&self) -> IndexValue {
            match self.kind {
                ValueTypeKind::Indexed(index, _) => IndexValue { index },
                _ => panic!("raw_index called on non-indexed type"),
            }
        }

        pub fn numeric_kind(&self) -> NumericKind {
            match self.kind {
                ValueTypeKind::Numeric(kind) => kind,
                _ => panic!("numeric_kind called on non-numeric type"),
            }
        }

        pub fn generic_kind(&self) -> GenericKind {
            match self.kind {
                ValueTypeKind::Generic(kind, _, _) => kind,
                _ => panic!("generic_kind called on non-generic type"),
            }
        }

        pub fn value_type_code_numeric(&self) -> ValueTypeCode {
            match self.numeric_kind() {
                NumericKind::I32 => ValueTypeCode::I32,
                NumericKind::I64 => ValueTypeCode::I64,
                NumericKind::F32 => ValueTypeCode::F32,
                NumericKind::F64 => ValueTypeCode::F64,
                NumericKind::V128 => ValueTypeCode::V128,
            }
        }

        pub fn value_type_code_generic(&self) -> ValueTypeCode {
            match self.generic_kind() {
                GenericKind::AnyRef => ValueTypeCode::AnyRef,
                GenericKind::DataRef => ValueTypeCode::DataRef,
                GenericKind::Extern => ValueTypeCode::ExternRef,
                GenericKind::Func => ValueTypeCode::FuncRef,
                GenericKind::Eq => ValueTypeCode::EqRef,
                GenericKind::StringViewUtf8 => ValueTypeCode::StringViewUtf8,
                GenericKind::StringViewUtf16 => ValueTypeCode::StringViewUtf16,
                GenericKind::StringViewWtf8 => ValueTypeCode::StringViewWtf8,
                GenericKind::I31Ref => ValueTypeCode::I31Ref,
                GenericKind::Struct => ValueTypeCode::StructRef,
                GenericKind::Array => ValueTypeCode::ArrayRef,
                GenericKind::ExnRef => ValueTypeCode::ExnRef,
                GenericKind::Cont => ValueTypeCode::ContRef,
                GenericKind::Bottom => ValueTypeCode::Bottom,
                GenericKind::Top => ValueTypeCode::Top,
                GenericKind::Void => ValueTypeCode::Void,
                _ => panic!("Unreachable: value_type_code_generic called with invalid generic kind"),
            }
        }

        pub fn generic_heaptype_name(&self) -> String {
            match &self.kind {
                ValueTypeKind::Generic(kind, _, shared) => {
                    let mut buf = String::new();
                    if *shared {
                        buf.push_str("shared ");
                    }
                    match kind {
                        GenericKind::None => buf.push_str("none"),
                        GenericKind::Extern => buf.push_str("extern"),
                        GenericKind::Func => buf.push_str("func"),
                        GenericKind::Any => buf.push_str("any"),
                        GenericKind::Eq => buf.push_str("eq"),
                        GenericKind::Struct => buf.push_str("struct"),
                        GenericKind::Array => buf.push_str("array"),
                        GenericKind::I31Ref => buf.push_str("i31"),
                        GenericKind::StringViewUtf8 => buf.push_str("stringview_utf8"),
                        GenericKind::StringViewUtf16 => buf.push_str("stringview_utf16"),
                        GenericKind::StringViewWtf8 => buf.push_str("stringview_wtf8"),
                        GenericKind::NoExtern => buf.push_str("noextern"),
                        GenericKind::NoFunc => buf.push_str("nofunc"),
                        GenericKind::NoExn => buf.push_str("noexn"),
                        GenericKind::AnyRef => buf.push_str("anyref"),
                        GenericKind::DataRef => buf.push_str("dataref"),
                        GenericKind::Cont => buf.push_str("cont"),
                        GenericKind::NoCont => buf.push_str("nocont"),
                        GenericKind::Void => buf.push_str("void"),
                        GenericKind::Bottom => buf.push_str("bottom"),
                        GenericKind::Top => buf.push_str("top"),
                    }
                    buf
                }
                _ => panic!("generic_heaptype_name called on non-generic type"),
            }
        }

        pub fn name(&self) -> String {
            if self.is_numeric() {
                match self.numeric_kind() {
                    NumericKind::I32 => "i32".to_string(),
                    NumericKind::I64 => "i64".to_string(),
                    NumericKind::F32 => "f32".to_string(),
                    NumericKind::F64 => "f64".to_string(),
                    NumericKind::V128 => "v128".to_string(),
                }
            } else {
                let mut buf = String::new();
                if self.has_index() {
                    buf.push_str("(ref ");
                    if self.is_nullable() {
                        buf.push_str("null ");
                    }
                    buf.push_str(&format!("{}", self.raw_index().index));
                    buf.push_str(")");
                } else {
                    debug_assert!(self.is_generic());
                    if self.is_nullable() {
                        let kind = self.generic_kind();
                        if kind == GenericKind::None {
                            return "nullref".to_string();
                        } else if kind == GenericKind::NoExn {
                            return "nullexnref".to_string();
                        } else if kind == GenericKind::NoExtern {
                            return "nullexternref".to_string();
                        } else if kind == GenericKind::NoFunc {
                            return "nullfuncref".to_string();
                        } else if kind == GenericKind::NoCont {
                            return "nullcontref".to_string();
                        }
                    }
                    let shorthand = self.is_nullable() || self.is_sentinel() || self.is_string_view();
                    let append_ref = self.is_nullable() && !self.is_sentinel();
                    if !shorthand {
                        buf.push_str("(ref ");
                    }
                    buf.push_str(&self.generic_heaptype_name());
                    if append_ref {
                        buf.push_str("ref");
                    }
                    if !shorthand {
                        buf.push_str(")");
                    }
                }
                buf
            }
        }
    }

    impl fmt::Debug for ValueTypeBase {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ValueTypeBase {{ {} }}", self.name())
        }
    }

    impl Display for ValueTypeBase {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.name())
        }
    }

    pub struct IndexValue {
        pub index: u32,
    }

    pub const kWasmBottom: ValueTypeBase = ValueTypeBase::generic(GenericKind::Bottom, false, false);
    pub const kWasmTop: ValueTypeBase = ValueTypeBase::generic(GenericKind::Top, false, false);
    pub const kWasmI32: ValueTypeBase = ValueTypeBase::numeric(NumericKind::I32);
    pub const kWasmI64: ValueTypeBase = ValueTypeBase::numeric(NumericKind::I64);

    // Remaining code that depends on external V8 types.
    // For now, stubs are created to allow compilation.
    // These stubs should be replaced with the actual implementations when the corresponding V8 components are translated to Rust.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ValueKind {
        I32,
        I64,
        F32,
        F64,
        V128,
        AnyRef,
        DataRef,
        FuncRef,
        ExternRef,
        ExnRef,
        StringViewUtf8,
        StringViewUtf16,
        StringViewWtf8,
        I31Ref,
        EqRef,
        StructRef,
        ArrayRef,
        ContRef,
    }

    #[derive(Debug)]
    pub struct CanonicalSig {
        return_types: Vec<CanonicalValueType>,
        parameter_types: Vec<CanonicalValueType>,
        signature_hash_: u32,
    }

    impl CanonicalSig {
        pub fn return_count(&self) -> usize {
            self.return_types.len()
        }

        pub fn parameter_count(&self) -> usize {
            self.parameter_types.len()
        }

        pub fn GetReturn(&self, index: usize) -> CanonicalValueType {
            self.return_types[index]
        }

        pub fn all(&self) -> Vec<&CanonicalValueType> {
            let mut all_types: Vec<&CanonicalValueType> = Vec::new();
            for param in &self.parameter_types {
                all_types.push(param);
            }
            for ret in &self.return_types {
                all_types.push(ret);
            }
            all_types
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CanonicalValueType {
        kind: ValueKind,
    }

    impl CanonicalValueType {
        pub fn kind(&self) -> ValueKind {
            self.kind
        }

        pub fn is_numeric(&self) -> bool {
            match self.kind {
                ValueKind::I32 | ValueKind::I64 | ValueKind::F32 | ValueKind::F64 | ValueKind::V128 => true,
                _ => false,
            }
        }
    }

    pub mod signature_hashing {
        use super::CanonicalSig;
        use std::hash::{Hash, Hasher};

        pub struct SignatureHasher {}

        impl SignatureHasher {
            pub fn Hash(sig: &CanonicalSig) -> u32 {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                sig.hash(&mut hasher);
                hasher.finish() as u32
            }
        }
    }

    impl Hash for CanonicalSig {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.return_types.hash(state);
            self.parameter_types.hash(state);
        }
    }

    #[derive(Debug)]
    pub struct FunctionSig {
        return_types: Vec<ValueTypeBase>,
        parameter_types: Vec<ValueTypeBase>,
    }

    impl FunctionSig {
        pub fn return_count(&self) -> usize {
            self.return_types.len()
        }

        pub fn parameter_count(&self) -> usize {
            self.parameter_types.len()
        }

        pub fn GetParam(&self, index: usize) -> ValueTypeBase {
            self.parameter_types[index]
        }

        pub fn GetReturn(&self) -> ValueTypeBase {
            self.return_types[0] // Assuming only one return type as per the C++ code.
        }

        pub fn all(&self) -> Vec<ValueTypeBase> {
            let mut all_types: Vec<ValueTypeBase> = Vec::new();
            all_types.extend_from_slice(&self.parameter_types);
            all_types.extend_from_slice(&self.return_types);
            all_types
        }

        pub fn parameters(&self) -> &Vec<ValueTypeBase> {
            &self.parameter_types
        }

        pub fn returns(&self) -> &Vec<ValueTypeBase> {
            &self.return_types
        }
    }

    pub struct CanonicalSigBuilder {
        return_types: Vec<CanonicalValueType>,
        parameter_types: Vec<CanonicalValueType>,
    }

    impl CanonicalSigBuilder {
        pub fn new(return_count: usize, parameter_count: usize) -> Self {
            CanonicalSigBuilder {
                return_types: Vec::with_capacity(return_count),
                parameter_types: Vec::with_capacity(parameter_count),
            }
        }

        pub fn AddReturn(&mut self, return_type: CanonicalValueType) {
            self.return_types.push(return_type);
        }

        pub fn AddParam(&mut self, param_type: CanonicalValueType) {
            self.parameter_types.push(param_type);
        }

        pub fn Get(&self) -> CanonicalSig {
            let mut sig = CanonicalSig {
                return_types: self.return_types.clone(),
                parameter_types: self.parameter_types.clone(),
                signature_hash_: 0, // Initialize to 0, the correct value is set in Get() method
            };
            sig.signature_hash_ = signature_hashing::SignatureHasher::Hash(&sig);
            sig
        }
    }

    pub fn wasm_return_type_from_signature(wasm_signature: &CanonicalSig) -> Option<ValueKind> {
        if wasm_signature.return_count() == 0 {
            return None;
        }

        debug_assert_eq!(wasm_signature.return_count(), 1);
        let return_type = wasm_signature.GetReturn(0);
        Some(return_type.kind())
    }

    pub fn equivalent_numeric_sig(a: &CanonicalSig, b: &FunctionSig) -> bool {
        if a.parameter_count() != b.parameter_count() {
            return false;
        }
        if a.return_count() != b.return_count() {
            return false;
        }

        let a_types: Vec<&CanonicalValueType> = a.all();
        let b_types: Vec<ValueTypeBase> = b.all();

        for i in 0..a_types.len() {
            if !a_types[i].is_numeric() {
                return false;
            }
            let a_kind = a_types[i].kind();
            let b_kind = match b_types[i] {
                _ if b_types[i].is_numeric() => match b_types[i].numeric_kind() {
                    NumericKind::I32 => ValueKind::I32,
                    NumericKind::I64 => ValueKind::I64,
                    NumericKind::F32 => ValueKind::F32,
                    NumericKind::F64 => ValueKind::F64,
                    NumericKind::V128 => ValueKind::V128,
                },
                _ => return false,
            };

            if a_kind != b_kind {
                return false;
            }
        }
        true
    }

    // Placeholder for printing function signature.
    #[cfg(debug_assertions)]
    pub fn print_function_sig(sig: &FunctionSig) {
        println!("{:?}", sig);
    }

    fn replace_type_in_sig(
        sig: &FunctionSig,
        from: ValueTypeBase,
        to: ValueTypeBase,
        num_replacements: usize,
    ) -> FunctionSig {
        let param_occurrences = sig.parameters().iter().filter(|&p| *p == from).count();
        let return_occurrences = sig.returns().iter().filter(|&r| *r == from).count();

        if param_occurrences == 0 && return_occurrences == 0 {
            return FunctionSig {
                return_types: sig.return_types.clone(),
                parameter_types: sig.parameter_types.clone(),
            };
        }

        let mut return_types = Vec::new();
        let mut parameter_types = Vec::new();

        for &ret in sig.returns() {
            if ret == from {
                for _ in 0..num_replacements {
                    return_types.push(to);
                }
            } else {
                return_types.push(ret);
            }
        }

        for &param in sig.parameters() {
            if param == from {
                for _ in 0..num_replacements {
                    parameter_types.push(to);
                }
            } else {
                parameter_types.push(param);
            }
        }

        FunctionSig {
            return_types,
            parameter_types,
        }
    }

    pub fn get_i32_sig(sig: &FunctionSig) -> FunctionSig {
        replace_type_in_sig(sig, kWasmI64, kWasmI32, 2)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_value_type_base_creation() {
            let i32_type = ValueTypeBase::numeric(NumericKind::I32);
            assert_eq!(i32_type.name(), "i32");

            let anyref_type = ValueTypeBase::generic(GenericKind::AnyRef, false, false);
            assert_eq!(anyref_type.name(), "(ref any)");

            let nullable_anyref_type = ValueTypeBase::generic(GenericKind::AnyRef, true, false);
            assert_eq!(nullable_anyref_type.name(), "(ref anyref)");
        }

        #[test]
        fn test_wasm_return_type_from_signature() {
            let mut builder = CanonicalSigBuilder::new(1, 1);
            builder.AddReturn(CanonicalValueType { kind: ValueKind::I32 });
            builder.AddParam(CanonicalValueType { kind: ValueKind::I64 });
            let sig = builder.Get();

            let return_type = wasm_return_type_from_signature(&sig);
            assert_eq!(return_type, Some(ValueKind::I32));

            let mut empty_builder = CanonicalSigBuilder::new(0, 0);
            let empty_sig = empty_builder.Get();
            let empty_return_type = wasm_return_type_from_signature(&empty_sig);
            assert_eq!(empty_return_type, None);
        }

        #[test]
        fn test_equivalent_numeric_sig() {
            let mut a_builder = CanonicalSigBuilder::new(1, 1);
            a_builder.AddReturn(CanonicalValueType { kind: ValueKind::I32 });
            a_builder.AddParam(CanonicalValueType { kind: ValueKind::I64 });
            let a_sig = a_builder.Get();

            let b_sig = FunctionSig {
                return_types: vec![kWasmI32],
                parameter_types: vec![kWasmI64],
            };

            assert!(equivalent_numeric_sig(&a_sig, &b_sig));

            let mut c_builder = CanonicalSigBuilder::new(1, 1);
            c_builder.AddReturn(CanonicalValueType { kind: ValueKind::F32 });
            c_builder.AddParam(CanonicalValueType { kind: ValueKind::I64 });
            let c_sig = c_builder.Get();
            assert!(!equivalent_numeric_sig(&c_sig, &b_sig));
        }

        #[test]
        fn test_replace_type_in_sig() {
            let sig = FunctionSig {
                return_types: vec![kWasmI64],
                parameter_types: vec![kWasmI64, kWasmI32, kWasmI64],
            };

            let new_sig = replace_type_in_sig(&sig, kWasmI64, kWasmI32, 2);

            assert_eq!(new_sig.return_types, vec![kWasmI32, kWasmI32]);
            assert_eq!(new_sig.parameter_types, vec![kWasmI32, kWasmI32, kWasmI32, kWasmI32, kWasmI32, kWasmI32]);

            let new_sig_no_replace = replace_type_in_sig(&sig, ValueTypeBase::generic(GenericKind::AnyRef, true, false), kWasmI32, 2);
            assert_eq!(new_sig_no_replace.return_types, sig.return_types);
            assert_eq!(new_sig_no_replace.parameter_types, sig.parameter_types);

        }

        #[test]
        fn test_get_i32_sig() {
            let sig = FunctionSig {
                return_types: vec![kWasmI64],
                parameter_types: vec![kWasmI64, kWasmI32, kWasmI64],
            };

            let i32_sig = get_i32_sig(&sig);

            assert_eq!(i32_sig.return_types, vec![kWasmI32]);
            assert_eq!(i32_sig.parameter_types, vec![kWasmI32, kWasmI32, kWasmI32]);
        }

        #[test]
        fn test_generic_heaptype_name() {
            let anyref_type = ValueTypeBase::generic(GenericKind::AnyRef, false, false);
            assert_eq!(anyref_type.generic_heaptype_name(), "anyref");

            let shared_externref_type = ValueTypeBase::generic(GenericKind::Extern, false, true);
            assert_eq!(shared_externref_type.generic_heaptype_name(), "shared extern");

            let nullable_dataref_type = ValueTypeBase::generic(GenericKind::DataRef, true, false);
            assert_eq!(nullable_dataref_type.generic_heaptype_name(), "dataref");
        }

        #[test]
        fn test_is_shared_nullable() {
            let shared_anyref = ValueTypeBase::generic(GenericKind::AnyRef, false, true);
            assert_eq!(shared_anyref.is_shared(), true);
            assert_eq!(shared_anyref.is_nullable(), false);

            let nullable_anyref = ValueTypeBase::generic(GenericKind::AnyRef, true, false);
            assert_eq!(nullable_anyref.is_shared(), false);
            assert_eq!(nullable_anyref.is_nullable(), true);

            let not_shared_not_nullable = ValueTypeBase::generic(GenericKind::AnyRef, false, false);
            assert_eq!(not_shared_not_nullable.is_shared(), false);
            assert_eq!(not_shared_not_nullable.is_nullable(), false);
        }

        #[test]
        fn test_sentinel_types() {
            let noextern_type = ValueTypeBase::generic(GenericKind::NoExtern, false, false);
            let nofunc_type = ValueTypeBase::generic(GenericKind::NoFunc, false, false);
            let noexn_type = ValueTypeBase::generic(GenericKind::NoExn, false, false);
            let nocont_type = ValueTypeBase::generic(GenericKind::NoCont, false, false);

            assert_eq!(noextern_type.is_sentinel(), true);
            assert_eq!(nofunc_type.is_sentinel(), true);
            assert_eq!(noexn_type.is_sentinel(), true);
            assert_eq!(nocont_type.is_sentinel(), true);

            assert_eq!(ValueTypeBase::generic(GenericKind::AnyRef, false, false).is_sentinel(), false);
            assert_eq!(ValueTypeBase::numeric(NumericKind::I32).is_sentinel(), false);
        }

        #[test]
        fn test_nullable_sentinel_names() {
             assert_eq!(ValueTypeBase::generic(GenericKind::None, true, false).name(), "nullref");
             assert_eq!(ValueTypeBase::generic(GenericKind::NoExn, true, false).name(), "nullexnref");
             assert_eq!(ValueTypeBase::generic(GenericKind::NoExtern, true, false).name(), "nullexternref");
             assert_eq!(ValueTypeBase::generic(GenericKind::NoFunc, true, false).name(), "nullfuncref");
             assert_eq!(ValueTypeBase::generic(GenericKind::NoCont, true, false).name(), "nullcontref");
        }
    }
}