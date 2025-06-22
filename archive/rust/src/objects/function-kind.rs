// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FunctionKind {
    // BEGIN constructable functions
    NormalFunction,
    Module,
    ModuleWithTopLevelAwait,
    // BEGIN class constructors
    // BEGIN base constructors
    BaseConstructor,
    // BEGIN default constructors
    DefaultBaseConstructor,
    // END base constructors
    // BEGIN derived constructors
    DefaultDerivedConstructor,
    // END default constructors
    DerivedConstructor,
    // END derived constructors
    // END class constructors
    // END constructable functions.
    // BEGIN accessors
    GetterFunction,
    StaticGetterFunction,
    SetterFunction,
    StaticSetterFunction,
    // END accessors
    // BEGIN arrow functions
    ArrowFunction,
    // BEGIN async functions
    AsyncArrowFunction,
    // END arrow functions
    AsyncFunction,
    // BEGIN concise methods 1
    AsyncConciseMethod,
    StaticAsyncConciseMethod,
    // BEGIN generators
    AsyncConciseGeneratorMethod,
    StaticAsyncConciseGeneratorMethod,
    // END concise methods 1
    AsyncGeneratorFunction,
    // END async functions
    GeneratorFunction,
    // BEGIN concise methods 2
    ConciseGeneratorMethod,
    StaticConciseGeneratorMethod,
    // END generators
    ConciseMethod,
    StaticConciseMethod,
    ClassMembersInitializerFunction,
    ClassStaticInitializerFunction,
    // END concise methods 2
    Invalid,

    LastFunctionKind = FunctionKind::ClassStaticInitializerFunction as u8,
}

pub const FUNCTION_KIND_BIT_SIZE: i32 = 5;

const _: () = {
    assert!(FunctionKind::LastFunctionKind as i32  < (1 << FUNCTION_KIND_BIT_SIZE));
};

impl FunctionKind {
    #[inline]
    pub fn is_arrow_function(self) -> bool {
        self >= FunctionKind::ArrowFunction && self <= FunctionKind::AsyncArrowFunction
    }

    #[inline]
    pub fn is_module(self) -> bool {
        self >= FunctionKind::Module && self <= FunctionKind::ModuleWithTopLevelAwait
    }

    #[inline]
    pub fn is_module_with_top_level_await(self) -> bool {
        self == FunctionKind::ModuleWithTopLevelAwait
    }

    #[inline]
    pub fn is_async_generator_function(self) -> bool {
        self >= FunctionKind::AsyncConciseGeneratorMethod && self <= FunctionKind::AsyncGeneratorFunction
    }

    #[inline]
    pub fn is_generator_function(self) -> bool {
        self >= FunctionKind::AsyncConciseGeneratorMethod && self <= FunctionKind::StaticConciseGeneratorMethod
    }

    #[inline]
    pub fn is_async_function(self) -> bool {
        self >= FunctionKind::AsyncArrowFunction && self <= FunctionKind::AsyncGeneratorFunction
    }

    #[inline]
    pub fn is_resumable_function(self) -> bool {
        self.is_generator_function() || self.is_async_function() || self.is_module()
    }

    #[inline]
    pub fn is_concise_method(self) -> bool {
        (self >= FunctionKind::AsyncConciseMethod && self <= FunctionKind::StaticAsyncConciseGeneratorMethod) ||
            (self >= FunctionKind::ConciseGeneratorMethod && self <= FunctionKind::ClassStaticInitializerFunction)
    }

    #[inline]
    pub fn is_strict_function_without_prototype(self) -> bool {
        (self >= FunctionKind::GetterFunction && self <= FunctionKind::AsyncArrowFunction) ||
            (self >= FunctionKind::AsyncConciseMethod && self <= FunctionKind::StaticAsyncConciseGeneratorMethod) ||
            (self >= FunctionKind::ConciseGeneratorMethod && self <= FunctionKind::ClassStaticInitializerFunction)
    }

    #[inline]
    pub fn is_getter_function(self) -> bool {
        self >= FunctionKind::GetterFunction && self <= FunctionKind::StaticGetterFunction
    }

    #[inline]
    pub fn is_setter_function(self) -> bool {
        self >= FunctionKind::SetterFunction && self <= FunctionKind::StaticSetterFunction
    }

    #[inline]
    pub fn is_accessor_function(self) -> bool {
        self >= FunctionKind::GetterFunction && self <= FunctionKind::StaticSetterFunction
    }

    #[inline]
    pub fn is_default_constructor(self) -> bool {
        self >= FunctionKind::DefaultBaseConstructor && self <= FunctionKind::DefaultDerivedConstructor
    }

    #[inline]
    pub fn is_base_constructor(self) -> bool {
        self >= FunctionKind::BaseConstructor && self <= FunctionKind::DefaultBaseConstructor
    }

    #[inline]
    pub fn is_derived_constructor(self) -> bool {
        self >= FunctionKind::DefaultDerivedConstructor && self <= FunctionKind::DerivedConstructor
    }

    #[inline]
    pub fn is_class_constructor(self) -> bool {
        self >= FunctionKind::BaseConstructor && self <= FunctionKind::DerivedConstructor
    }

    #[inline]
    pub fn is_class_members_initializer_function(self) -> bool {
        self >= FunctionKind::ClassMembersInitializerFunction && self <= FunctionKind::ClassStaticInitializerFunction
    }

    #[inline]
    pub fn is_constructable(self) -> bool {
        self >= FunctionKind::NormalFunction && self <= FunctionKind::DerivedConstructor
    }

    #[inline]
    pub fn is_static(self) -> bool {
        match self {
            FunctionKind::StaticGetterFunction |
            FunctionKind::StaticSetterFunction |
            FunctionKind::StaticConciseMethod |
            FunctionKind::StaticConciseGeneratorMethod |
            FunctionKind::StaticAsyncConciseMethod |
            FunctionKind::StaticAsyncConciseGeneratorMethod |
            FunctionKind::ClassStaticInitializerFunction => true,
            _ => false,
        }
    }

    #[inline]
    pub fn binds_super(self) -> bool {
        self.is_concise_method() || self.is_accessor_function() ||
            self.is_class_constructor()
    }

    #[inline]
    pub fn to_string(self) -> &'static str {
        match self {
            FunctionKind::NormalFunction => "NormalFunction",
            FunctionKind::ArrowFunction => "ArrowFunction",
            FunctionKind::GeneratorFunction => "GeneratorFunction",
            FunctionKind::ConciseMethod => "ConciseMethod",
            FunctionKind::StaticConciseMethod => "StaticConciseMethod",
            FunctionKind::DerivedConstructor => "DerivedConstructor",
            FunctionKind::BaseConstructor => "BaseConstructor",
            FunctionKind::GetterFunction => "GetterFunction",
            FunctionKind::StaticGetterFunction => "StaticGetterFunction",
            FunctionKind::SetterFunction => "SetterFunction",
            FunctionKind::StaticSetterFunction => "StaticSetterFunction",
            FunctionKind::AsyncFunction => "AsyncFunction",
            FunctionKind::Module => "Module",
            FunctionKind::ModuleWithTopLevelAwait => "AsyncModule",
            FunctionKind::ClassMembersInitializerFunction => "ClassMembersInitializerFunction",
            FunctionKind::ClassStaticInitializerFunction => "ClassStaticInitializerFunction",
            FunctionKind::DefaultBaseConstructor => "DefaultBaseConstructor",
            FunctionKind::DefaultDerivedConstructor => "DefaultDerivedConstructor",
            FunctionKind::AsyncArrowFunction => "AsyncArrowFunction",
            FunctionKind::AsyncConciseMethod => "AsyncConciseMethod",
            FunctionKind::StaticAsyncConciseMethod => "StaticAsyncConciseMethod",
            FunctionKind::ConciseGeneratorMethod => "ConciseGeneratorMethod",
            FunctionKind::StaticConciseGeneratorMethod => "StaticConciseGeneratorMethod",
            FunctionKind::AsyncConciseGeneratorMethod => "AsyncConciseGeneratorMethod",
            FunctionKind::StaticAsyncConciseGeneratorMethod => "StaticAsyncConciseGeneratorMethod",
            FunctionKind::AsyncGeneratorFunction => "AsyncGeneratorFunction",
            FunctionKind::Invalid => "Invalid",
        }
    }
}

impl std::fmt::Display for FunctionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}