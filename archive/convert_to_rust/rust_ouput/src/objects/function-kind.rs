// Converted from V8 C++ source files:
// Header: function-kind.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub fn is_in_range<T: PartialOrd>(value: T, lower: T, upper: T) -> bool {
        value >= lower && value <= upper
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FunctionKind {
    // BEGIN constructable functions
    kNormalFunction,
    kModule,
    kModuleWithTopLevelAwait,
    // BEGIN class constructors
    // BEGIN base constructors
    kBaseConstructor,
    // BEGIN default constructors
    kDefaultBaseConstructor,
    // END base constructors
    // BEGIN derived constructors
    kDefaultDerivedConstructor,
    // END default constructors
    kDerivedConstructor,
    // END derived constructors
    // END class constructors
    // END constructable functions.
    // BEGIN accessors
    kGetterFunction,
    kStaticGetterFunction,
    kSetterFunction,
    kStaticSetterFunction,
    // END accessors
    // BEGIN arrow functions
    kArrowFunction,
    // BEGIN async functions
    kAsyncArrowFunction,
    // END arrow functions
    kAsyncFunction,
    // BEGIN concise methods 1
    kAsyncConciseMethod,
    kStaticAsyncConciseMethod,
    // BEGIN generators
    kAsyncConciseGeneratorMethod,
    kStaticAsyncConciseGeneratorMethod,
    // END concise methods 1
    kAsyncGeneratorFunction,
    // END async functions
    kGeneratorFunction,
    // BEGIN concise methods 2
    kConciseGeneratorMethod,
    kStaticConciseGeneratorMethod,
    // END generators
    kConciseMethod,
    kStaticConciseMethod,
    kClassMembersInitializerFunction,
    kClassStaticInitializerFunction,
    // END concise methods 2
    kInvalid,

    kLastFunctionKind = kClassStaticInitializerFunction,
}

const K_FUNCTION_KIND_BIT_SIZE: i32 = 5;

impl FunctionKind {
    pub fn is_arrow_function(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kArrowFunction,
            FunctionKind::kAsyncArrowFunction,
        )
    }

    pub fn is_module(self) -> bool {
        base::is_in_range(self, FunctionKind::kModule, FunctionKind::kModuleWithTopLevelAwait)
    }

    pub fn is_module_with_top_level_await(self) -> bool {
        self == FunctionKind::kModuleWithTopLevelAwait
    }

    pub fn is_async_generator_function(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kAsyncConciseGeneratorMethod,
            FunctionKind::kAsyncGeneratorFunction,
        )
    }

    pub fn is_generator_function(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kAsyncConciseGeneratorMethod,
            FunctionKind::kStaticConciseGeneratorMethod,
        )
    }

    pub fn is_async_function(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kAsyncArrowFunction,
            FunctionKind::kAsyncGeneratorFunction,
        )
    }

    pub fn is_resumable_function(self) -> bool {
        self.is_generator_function() || self.is_async_function() || self.is_module()
    }

    pub fn is_concise_method(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kAsyncConciseMethod,
            FunctionKind::kStaticAsyncConciseGeneratorMethod,
        ) || base::is_in_range(
            self,
            FunctionKind::kConciseGeneratorMethod,
            FunctionKind::kClassStaticInitializerFunction,
        )
    }

    pub fn is_strict_function_without_prototype(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kGetterFunction,
            FunctionKind::kAsyncArrowFunction,
        ) || base::is_in_range(
            self,
            FunctionKind::kAsyncConciseMethod,
            FunctionKind::kStaticAsyncConciseGeneratorMethod,
        ) || base::is_in_range(
            self,
            FunctionKind::kConciseGeneratorMethod,
            FunctionKind::kClassStaticInitializerFunction,
        )
    }

    pub fn is_getter_function(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kGetterFunction,
            FunctionKind::kStaticGetterFunction,
        )
    }

    pub fn is_setter_function(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kSetterFunction,
            FunctionKind::kStaticSetterFunction,
        )
    }

    pub fn is_accessor_function(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kGetterFunction,
            FunctionKind::kStaticSetterFunction,
        )
    }

    pub fn is_default_constructor(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kDefaultBaseConstructor,
            FunctionKind::kDefaultDerivedConstructor,
        )
    }

    pub fn is_base_constructor(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kBaseConstructor,
            FunctionKind::kDefaultBaseConstructor,
        )
    }

    pub fn is_derived_constructor(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kDefaultDerivedConstructor,
            FunctionKind::kDerivedConstructor,
        )
    }

    pub fn is_class_constructor(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kBaseConstructor,
            FunctionKind::kDerivedConstructor,
        )
    }

    pub fn is_class_members_initializer_function(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kClassMembersInitializerFunction,
            FunctionKind::kClassStaticInitializerFunction,
        )
    }

    pub fn is_constructable(self) -> bool {
        base::is_in_range(
            self,
            FunctionKind::kNormalFunction,
            FunctionKind::kDerivedConstructor,
        )
    }

    pub fn is_static(self) -> bool {
        match self {
            FunctionKind::kStaticGetterFunction
            | FunctionKind::kStaticSetterFunction
            | FunctionKind::kStaticConciseMethod
            | FunctionKind::kStaticConciseGeneratorMethod
            | FunctionKind::kStaticAsyncConciseMethod
            | FunctionKind::kStaticAsyncConciseGeneratorMethod
            | FunctionKind::kClassStaticInitializerFunction => true,
            _ => false,
        }
    }

    pub fn binds_super(self) -> bool {
        self.is_concise_method() || self.is_accessor_function() || self.is_class_constructor()
    }

    pub fn to_string(self) -> &'static str {
        match self {
            FunctionKind::kNormalFunction => "NormalFunction",
            FunctionKind::kArrowFunction => "ArrowFunction",
            FunctionKind::kGeneratorFunction => "GeneratorFunction",
            FunctionKind::kConciseMethod => "ConciseMethod",
            FunctionKind::kStaticConciseMethod => "StaticConciseMethod",
            FunctionKind::kDerivedConstructor => "DerivedConstructor",
            FunctionKind::kBaseConstructor => "BaseConstructor",
            FunctionKind::kGetterFunction => "GetterFunction",
            FunctionKind::kStaticGetterFunction => "StaticGetterFunction",
            FunctionKind::kSetterFunction => "SetterFunction",
            FunctionKind::kStaticSetterFunction => "StaticSetterFunction",
            FunctionKind::kAsyncFunction => "AsyncFunction",
            FunctionKind::kModule => "Module",
            FunctionKind::kModuleWithTopLevelAwait => "AsyncModule",
            FunctionKind::kClassMembersInitializerFunction => "ClassMembersInitializerFunction",
            FunctionKind::kClassStaticInitializerFunction => "ClassStaticInitializerFunction",
            FunctionKind::kDefaultBaseConstructor => "DefaultBaseConstructor",
            FunctionKind::kDefaultDerivedConstructor => "DefaultDerivedConstructor",
            FunctionKind::kAsyncArrowFunction => "AsyncArrowFunction",
            FunctionKind::kAsyncConciseMethod => "AsyncConciseMethod",
            FunctionKind::kStaticAsyncConciseMethod => "StaticAsyncConciseMethod",
            FunctionKind::kConciseGeneratorMethod => "ConciseGeneratorMethod",
            FunctionKind::kStaticConciseGeneratorMethod => "StaticConciseGeneratorMethod",
            FunctionKind::kAsyncConciseGeneratorMethod => "AsyncConciseGeneratorMethod",
            FunctionKind::kStaticAsyncConciseGeneratorMethod => {
                "StaticAsyncConciseGeneratorMethod"
            }
            FunctionKind::kAsyncGeneratorFunction => "AsyncGeneratorFunction",
            FunctionKind::kInvalid => "Invalid",
        }
    }
}
