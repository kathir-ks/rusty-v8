// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod asm_types {
    use std::fmt;
    use std::fmt::Formatter;

    pub const K_NOT_HEAP_TYPE: i32 = -1;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct AsmValueType {
        bitset: u32,
    }

    impl AsmValueType {
        pub const ASM_NONE: u32 = 1 << 0;
        pub const ASM_INT: u32 = 1 << 1;
        pub const ASM_DOUBLE: u32 = 1 << 2;
        pub const ASM_FLOAT: u32 = 1 << 3;
        pub const ASM_SIGNED: u32 = 1 << 4;
        pub const ASM_UNSIGNED: u32 = 1 << 5;
        pub const ASM_INTISH: u32 = 1 << 6;
        pub const ASM_FLOATISH: u32 = 1 << 7;
        pub const ASM_FLOAT_Q: u32 = 1 << 8;
        pub const ASM_DOUBLE_Q: u32 = 1 << 9;
        pub const ASM_FLOATISH_DOUBLE_Q: u32 = 1 << 10;
        pub const ASM_INT8_ARRAY: u32 = 1 << 11;
        pub const ASM_UINT8_ARRAY: u32 = 1 << 12;
        pub const ASM_INT16_ARRAY: u32 = 1 << 13;
        pub const ASM_UINT16_ARRAY: u32 = 1 << 14;
        pub const ASM_INT32_ARRAY: u32 = 1 << 15;
        pub const ASM_UINT32_ARRAY: u32 = 1 << 16;
        pub const ASM_FLOAT32_ARRAY: u32 = 1 << 17;
        pub const ASM_FLOAT64_ARRAY: u32 = 1 << 18;

        pub fn new(bitset: u32) -> Self {
            AsmValueType { bitset }
        }

        pub fn bitset(&self) -> u32 {
            self.bitset
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum AsmType {
        Value(AsmValueType),
        Callable(Box<dyn AsmCallableType>),
    }

    impl AsmType {
        pub fn as_value_type(&self) -> Option<&AsmValueType> {
            match self {
                AsmType::Value(avt) => Some(avt),
                _ => None,
            }
        }

        pub fn as_callable_type(&self) -> Option<&dyn AsmCallableType> {
            match self {
                AsmType::Callable(act) => Some(act.as_ref()),
                _ => None,
            }
        }

        pub fn name(&self) -> String {
            match self.as_value_type() {
                Some(avt) => match avt.bitset() {
                    AsmValueType::ASM_NONE => "none".to_string(),
                    AsmValueType::ASM_INT => "int".to_string(),
                    AsmValueType::ASM_DOUBLE => "double".to_string(),
                    AsmValueType::ASM_FLOAT => "float".to_string(),
                    AsmValueType::ASM_SIGNED => "signed".to_string(),
                    AsmValueType::ASM_UNSIGNED => "unsigned".to_string(),
                    AsmValueType::ASM_INTISH => "intish".to_string(),
                    AsmValueType::ASM_FLOATISH => "floatish".to_string(),
                    AsmValueType::ASM_FLOAT_Q => "floatQ".to_string(),
                    AsmValueType::ASM_DOUBLE_Q => "doubleQ".to_string(),
                    AsmValueType::ASM_FLOATISH_DOUBLE_Q => "floatishDoubleQ".to_string(),
                    AsmValueType::ASM_INT8_ARRAY => "int8array".to_string(),
                    AsmValueType::ASM_UINT8_ARRAY => "uint8array".to_string(),
                    AsmValueType::ASM_INT16_ARRAY => "int16array".to_string(),
                    AsmValueType::ASM_UINT16_ARRAY => "uint16array".to_string(),
                    AsmValueType::ASM_INT32_ARRAY => "int32array".to_string(),
                    AsmValueType::ASM_UINT32_ARRAY => "uint32array".to_string(),
                    AsmValueType::ASM_FLOAT32_ARRAY => "float32array".to_string(),
                    AsmValueType::ASM_FLOAT64_ARRAY => "float64array".to_string(),
                    _ => panic!("UNREACHABLE"),
                },
                None => self.as_callable_type().unwrap().name(),
            }
        }

        pub fn is_exactly(&self, other: &AsmType) -> bool {
            match (self.as_value_type(), other.as_value_type()) {
                (Some(avt_self), Some(avt_other)) => avt_self.bitset() == avt_other.bitset(),
                (None, None) => std::ptr::eq(self, other),
                _ => false,
            }
        }

        pub fn is_a(&self, that: &AsmType) -> bool {
            if let Some(avt) = self.as_value_type() {
                if let Some(tavt) = that.as_value_type() {
                    return (avt.bitset() & tavt.bitset()) == tavt.bitset();
                }
                return false;
            }

            if let Some(as_callable) = self.as_callable_type() {
                return as_callable.is_a(that);
            }

            panic!("UNREACHABLE");
        }

        pub fn element_size_in_bytes(&self) -> i32 {
            match self.as_value_type() {
                Some(value) => match value.bitset() {
                    AsmValueType::ASM_INT8_ARRAY | AsmValueType::ASM_UINT8_ARRAY => 1,
                    AsmValueType::ASM_INT16_ARRAY | AsmValueType::ASM_UINT16_ARRAY => 2,
                    AsmValueType::ASM_INT32_ARRAY
                    | AsmValueType::ASM_UINT32_ARRAY
                    | AsmValueType::ASM_FLOAT32_ARRAY => 4,
                    AsmValueType::ASM_FLOAT64_ARRAY => 8,
                    _ => K_NOT_HEAP_TYPE,
                },
                None => K_NOT_HEAP_TYPE,
            }
        }

        pub fn load_type(&self) -> AsmType {
            match self.as_value_type() {
                Some(value) => match value.bitset() {
                    AsmValueType::ASM_INT8_ARRAY
                    | AsmValueType::ASM_UINT8_ARRAY
                    | AsmValueType::ASM_INT16_ARRAY
                    | AsmValueType::ASM_UINT16_ARRAY
                    | AsmValueType::ASM_INT32_ARRAY
                    | AsmValueType::ASM_UINT32_ARRAY => AsmType::Value(AsmValueType::new(AsmValueType::ASM_INTISH)),
                    AsmValueType::ASM_FLOAT32_ARRAY => AsmType::Value(AsmValueType::new(AsmValueType::ASM_FLOAT_Q)),
                    AsmValueType::ASM_FLOAT64_ARRAY => AsmType::Value(AsmValueType::new(AsmValueType::ASM_DOUBLE_Q)),
                    _ => AsmType::Value(AsmValueType::new(AsmValueType::ASM_NONE)),
                },
                None => AsmType::Value(AsmValueType::new(AsmValueType::ASM_NONE)),
            }
        }

        pub fn store_type(&self) -> AsmType {
            match self.as_value_type() {
                Some(value) => match value.bitset() {
                    AsmValueType::ASM_INT8_ARRAY
                    | AsmValueType::ASM_UINT8_ARRAY
                    | AsmValueType::ASM_INT16_ARRAY
                    | AsmValueType::ASM_UINT16_ARRAY
                    | AsmValueType::ASM_INT32_ARRAY
                    | AsmValueType::ASM_UINT32_ARRAY => AsmType::Value(AsmValueType::new(AsmValueType::ASM_INTISH)),
                    AsmValueType::ASM_FLOAT32_ARRAY => AsmType::Value(AsmValueType::new(AsmValueType::ASM_FLOATISH_DOUBLE_Q)),
                    AsmValueType::ASM_FLOAT64_ARRAY => AsmType::Value(AsmValueType::new(AsmValueType::ASM_FLOAT_Q | AsmValueType::ASM_DOUBLE_Q)),
                    _ => AsmType::Value(AsmValueType::new(AsmValueType::ASM_NONE)),
                },
                None => AsmType::Value(AsmValueType::new(AsmValueType::ASM_NONE)),
            }
        }

        pub fn none() -> AsmType {
            AsmType::Value(AsmValueType::new(AsmValueType::ASM_NONE))
        }

        pub fn intish() -> AsmType {
            AsmType::Value(AsmValueType::new(AsmValueType::ASM_INTISH))
        }

        pub fn float_q() -> AsmType {
            AsmType::Value(AsmValueType::new(AsmValueType::ASM_FLOAT_Q))
        }

        pub fn double_q() -> AsmType {
            AsmType::Value(AsmValueType::new(AsmValueType::ASM_DOUBLE_Q))
        }

        pub fn floatish_double_q() -> AsmType {
            AsmType::Value(AsmValueType::new(AsmValueType::ASM_FLOATISH_DOUBLE_Q))
        }

        pub fn floatish() -> AsmType {
            AsmType::Value(AsmValueType::new(AsmValueType::ASM_FLOATISH))
        }

        pub fn signed() -> AsmType {
            AsmType::Value(AsmValueType::new(AsmValueType::ASM_SIGNED))
        }

        pub fn unsigned() -> AsmType {
            AsmType::Value(AsmValueType::new(AsmValueType::ASM_UNSIGNED))
        }

        pub fn fround_type() -> AsmType {
            AsmType::Callable(Box::new(AsmFroundType::new()))
        }

        pub fn min_max_type(dest: &AsmType, src: &AsmType) -> AsmType {
            AsmType::Callable(Box::new(AsmMinMaxType::new(dest, src)))
        }
    }

    pub trait AsmCallableType: fmt::Debug {
        fn can_be_invoked_with(&self, return_type: &AsmType, args: &[&AsmType]) -> bool;
        fn name(&self) -> String;
        fn is_a(&self, other: &AsmType) -> bool;
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct AsmFunctionType {
        return_type: AsmType,
        args: Vec<AsmType>,
    }

    impl AsmFunctionType {
        pub fn new(return_type: AsmType, args: Vec<AsmType>) -> Self {
            AsmFunctionType {
                return_type,
                args,
            }
        }
    }

    impl AsmCallableType for AsmFunctionType {
        fn can_be_invoked_with(&self, return_type: &AsmType, args: &[&AsmType]) -> bool {
            if !self.return_type.is_exactly(return_type) {
                return false;
            }

            if self.args.len() != args.len() {
                return false;
            }

            for (ii, arg) in args.iter().enumerate() {
                if !arg.is_a(&self.args[ii]) {
                    return false;
                }
            }

            true
        }

        fn name(&self) -> String {
            let mut ret = String::new();
            ret += "(";
            for (ii, arg) in self.args.iter().enumerate() {
                ret += &arg.name();
                if ii != self.args.len() - 1 {
                    ret += ", ";
                }
            }
            ret += ") -> ";
            ret += &self.return_type.name();
            ret
        }

        fn is_a(&self, other: &AsmType) -> bool {
            match other.as_callable_type() {
                Some(that) => match that.name().as_str() {
                    name if name == self.name() => true,
                    _ => false,
                },
                None => false,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct AsmFroundType {}

    impl AsmFroundType {
        fn new() -> Self {
            AsmFroundType {}
        }
    }

    impl AsmCallableType for AsmFroundType {
        fn can_be_invoked_with(&self, _return_type: &AsmType, args: &[&AsmType]) -> bool {
            if args.len() != 1 {
                return false;
            }

            let arg = args[0];
            if !arg.is_a(&AsmType::Value(AsmValueType::new(AsmValueType::ASM_FLOATISH)))
                && !arg.is_a(&AsmType::Value(AsmValueType::new(AsmValueType::ASM_DOUBLE_Q)))
                && !arg.is_a(&AsmType::Value(AsmValueType::new(AsmValueType::ASM_SIGNED)))
                && !arg.is_a(&AsmType::Value(AsmValueType::new(AsmValueType::ASM_UNSIGNED)))
            {
                return false;
            }

            true
        }

        fn name(&self) -> String {
            "fround".to_string()
        }

        fn is_a(&self, other: &AsmType) -> bool {
            match other.as_callable_type() {
                Some(that) => match that.name().as_str() {
                    name if name == "fround" => true,
                    _ => false,
                },
                None => false,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct AsmMinMaxType {
        return_type: AsmType,
        arg: AsmType,
    }

    impl AsmMinMaxType {
        fn new(dest: &AsmType, src: &AsmType) -> Self {
            AsmMinMaxType {
                return_type: dest.clone(),
                arg: src.clone(),
            }
        }
    }

    impl AsmCallableType for AsmMinMaxType {
        fn can_be_invoked_with(&self, return_type: &AsmType, args: &[&AsmType]) -> bool {
            if !self.return_type.is_exactly(return_type) {
                return false;
            }

            if args.len() < 2 {
                return false;
            }

            for arg in args {
                if !arg.is_a(&self.arg) {
                    return false;
                }
            }

            true
        }

        fn name(&self) -> String {
            format!("({}, ...) -> {}", self.arg.name(), self.return_type.name())
        }

        fn is_a(&self, other: &AsmType) -> bool {
            match other.as_callable_type() {
                Some(that) => match that.name().as_str() {
                    name if name == self.name() => true,
                    _ => false,
                },
                None => false,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct AsmOverloadedFunctionType {
        overloads: Vec<AsmType>,
    }

    impl AsmOverloadedFunctionType {
        pub fn new() -> Self {
            AsmOverloadedFunctionType {
                overloads: Vec::new(),
            }
        }

        pub fn can_be_invoked_with(&self, return_type: &AsmType, args: &[&AsmType]) -> bool {
            for overload in &self.overloads {
                if overload
                    .as_callable_type()
                    .unwrap()
                    .can_be_invoked_with(return_type, args)
                {
                    return true;
                }
            }

            false
        }

        pub fn name(&self) -> String {
            let mut ret = String::new();

            for (ii, overload) in self.overloads.iter().enumerate() {
                if ii != 0 {
                    ret += " /\\ ";
                }
                ret += &overload.name();
            }

            ret
        }

        pub fn add_overload(&mut self, overload: AsmType) {
            // The original code does a CHECK_NOT_NULL(overload->AsCallableType());
            // Since there's no easy way to CHECK here, just unwrap, and let it panic
            // if that's a problem.
            overload.as_callable_type().expect("must be callable");
            self.overloads.push(overload);
        }
    }
}