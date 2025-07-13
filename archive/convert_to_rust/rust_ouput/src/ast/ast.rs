// Converted from V8 C++ source files:
// Header: ast.h
// Implementation: ast.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/ast/ast.rs
#![allow(non_upper_case_globals)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use crate::strings::uri::V8;
use crate::init::bootstrapper::Node;
use crate::init::bootstrapper::JavaScript;
use crate::init::bootstrapper::Scope;
use crate::parsing::token::Token;
use crate::torque::cfg::Block;
use crate::compiler_dispatcher::lazy_compile_dispatcher::FunctionLiteral;
use crate::strings::uri::v8;
use crate::strings::string_stream::Context;
use crate::objects::fixed_array::FixedArray;
use crate::objects::elements::ElementsKind;
use crate::strings::uri::void;

pub struct ZoneObject {}

pub type AstBigInt = String;

pub struct AstNode;

pub struct DeclarationScope {}

pub struct Base {}

pub struct JsValue {}

pub struct JSAny {}

pub struct Value {}

pub struct Isolate {}

pub struct JSObjectRef {}

pub struct Uchar {}

pub struct Flag {}

pub struct Exceptions {}

pub struct StringView {}

pub struct Tagged<T> {}

pub struct DirectHandle<T> {}

pub struct MaybeLocal<'static, T> {}

pub struct Local<'static, T> {}

pub struct Handle<T> {}

pub struct Runtime {}

pub enum OpIndex {}

pub enum InstructionOperand {}

pub enum MachineRepresentation {}

pub enum LocationOperand {}

pub enum Control {}

pub enum JsonPosition {}

pub struct Field {}

pub struct JsonObject {}

pub enum SourceFileMapScope {}

pub struct FixedArrayBase {}

pub struct Code {}

pub struct JSHeapBroker {}

pub struct Heap {}

pub enum BrokerMode {}

pub struct StructDeclaration {}

pub struct Operation {}

pub struct Label {}

pub struct MachineType {}

pub struct Flag {}

pub struct Constant {}

pub struct Key {}

pub struct FeedbackSource {}

pub struct DirectHandle<T> {}

pub struct Register {}

pub struct WriteBarrierKind {}

pub enum ArchOpcode {}

pub mod turboshaft {
  pub struct Graph {}
  pub struct Block {}
}
pub struct AstValueFactory {}

pub mod wasm {
    pub struct ValueType {}
    pub enum ValueTypeKind {}
}

pub mod internal {
    pub struct SharedObjectConveyorHandles {}
}
pub enum AllocationType {}

pub enum AsmType {}
pub struct Element {}

pub mod turboshaft {
  pub struct Operation {}
  pub struct Graph {}
  pub struct Block {}
}
pub mod wasm {
  pub enum ValueType {}
}
pub enum ValueTypeKind {}
pub enum HeapTypeRepresentation {}
pub mod turboshaft {
  pub struct Graph {}
  pub struct Block {}
}

pub struct ArrayBoilerplateDescription {}

pub struct ObjectBoilerplateDescription {}

pub struct VirtualAddressSpace {}

pub struct Sandbox {}

pub enum LanguageMode {}

pub enum FunctionKind {}
pub struct DirectHandle<T> {}

pub mod turboshaft {
    pub struct Operation {}
    pub struct Graph {}
    pub struct Block {}
}

pub enum If {}

pub struct Array {}

pub struct Oddball {}

pub struct Smi {}

pub mod turboshaft {
    pub struct Graph {}
    pub struct Block {}
}
pub mod wasm {
    pub struct ValueType {}
}

pub mod turboshaft {
    pub struct Operation {}
    pub struct Graph {}
    pub struct Block {}
}
pub mod wasm {
    pub enum ValueType {}
}

pub mod turboshaft {
    pub struct Operation {}
    pub struct Graph {}
    pub struct Block {}
}
pub mod wasm {
    pub enum ValueType {}
}

pub struct LocalHeap {}

pub mod turboshaft {
    pub struct Graph {}
    pub struct Block {}
}
pub mod wasm {
    pub enum ValueType {}
}

pub struct Value {}
pub struct Statement {}

pub enum ValueTypeKind {}
pub enum HeapTypeRepresentation {}

pub enum class CallType {
    GLOBAL_CALL,
    WITH_CALL,
    NAMED_PROPERTY_CALL,
    KEYED_PROPERTY_CALL,
    NAMED_OPTIONAL_CHAIN_PROPERTY_CALL,
    KEYED_OPTIONAL_CHAIN_PROPERTY_CALL,
    NAMED_SUPER_PROPERTY_CALL,
    KEYED_SUPER_PROPERTY_CALL,
    PRIVATE_CALL,
    PRIVATE_OPTIONAL_CHAIN_CALL,
    SUPER_CALL,
    OTHER_CALL,
}

pub enum ModuleImportPhase {}
pub struct Expression {}

pub struct Operator {}

pub struct Declaration {}
pub enum BinaryOperation {}

pub mod base {
    pub struct BitField<T, const A: usize, const B: usize> {}
}

pub struct Stack<T> {}

pub enum Property {}

pub enum ClassSyntaxKind {}

pub mod turboshaft {
    pub struct Operation {}
    pub struct Graph {}
    pub struct Block {}
}

pub mod wasm {
    pub struct ValueType {}
}

pub struct Write {};

pub struct Root {}

pub mod turboshaft {
    pub struct Graph {}
    pub struct Block {}
}

pub mod wasm {
    pub struct ValueType {}
}

pub mod internal {
    pub struct SharedObjectConveyorHandles {}
}

pub mod compiler {
    pub mod backend {
        pub struct instruction {
            pub struct InstructionOperand {}
        }
    }
}
pub mod turboshaft {
  pub struct Graph {}
  pub struct Block {}
}
pub mod wasm {
    pub struct ValueType {}
}
pub enum Runtime {}
pub mod compiler {
    pub mod backend {
        pub struct instruction {
            pub struct InstructionOperand {}
        }
    }
}

pub mod turboshaft {
  pub struct Graph {}
  pub struct Block {}
}
pub mod wasm {
    pub struct ValueType {}
}
pub enum Runtime {}

pub mod compiler {
    pub mod backend {
        pub struct instruction {
            pub struct InstructionOperand {}
        }
    }
}

pub mod turboshaft {
  pub struct Graph {}
  pub struct Block {}
}

pub mod wasm {
    pub struct ValueType {}
}

pub mod internal {
    pub struct SharedObjectConveyorHandles {}
}

pub struct Base {}

pub struct Iterator {}

pub mod turboshaft {
  pub struct Graph {}
  pub struct Block {}
}

pub mod wasm {
    pub struct ValueType {}
}

pub struct iterator {}

pub mod base {
    pub type uc16 = u16;
}

pub enum AstNodeFunc {
    kBlock,
    kVariableDeclaration,
    kFunctionDeclaration,
    kSwitchStatement,
    kDoWhileStatement,
    kWhileStatement,
    kForStatement,
    kForInStatement,
    kExpressionStatement,
    kContinueStatement,
    kBreakStatement,
    kReturnStatement,
    kWithStatement,
    kIfStatement,
    kTryCatchStatement,
    kTryFinallyStatement,
    kDebuggerStatement,
    kEmptyStatement,
    kSloppyBlockFunctionStatement,
    kInitializeClassMembersStatement,
    kInitializeClassStaticElementsStatement,
    kRegExpLiteral,
    kObjectLiteral,
    kArrayLiteral,
    kAssignment,
    kAwait,
    kBinaryOperation,
    kNaryOperation,
    kCall,
    kCallNew,
    kCallRuntime,
    kClassLiteral,
    kCompareOperation,
    kCompoundAssignment,
    kConditional,
    kCountOperation,
    kEmptyParentheses,
    kFunctionLiteral,
    kGetTemplateObject,
    kImportCallExpression,
    kLiteral,
    kNativeFunctionLiteral,
    kProperty,
    kSpread,
    kSuperCallReference,
    kSuperPropertyReference,
    kTemplateLiteral,
    kThisExpression,
    kThrow,
    kUnaryOperation,
    kVariableProxy,
    kYield,
    kFailureExpression,
    kYieldStar,
    kConditionalChain,
    kForOfStatement,
    kSuperCallForwardArgs,
    kOptionalChain,
    kAutoAccessorGetterBody,
    kAutoAccessorSetterBody,
}
