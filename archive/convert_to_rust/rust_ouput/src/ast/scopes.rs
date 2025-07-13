// Converted from V8 C++ source files:
// Header: scopes.h
// Implementation: scopes.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod scopes {
    //use crate::ast::ast::*;
    //use crate::base::compiler_specific::*;
    //use crate::base::hashmap::*;
    //use crate::base::pointer_with_payload::*;
    //use crate::base::threaded_list::*;
    //use crate::common::globals::*;
    //use crate::objects::function_kind::*;
    //use crate::zone::zone_hashmap::*;
    //use crate::zone::zone::*;
    //use std::numeric::*;

    //use crate::ast::ast::AstRawString;
    //use crate::ast::ast::Declaration;
    //use crate::ast::ast::ParseInfo;
    //use crate::ast::ast::VariableProxy;
    //use crate::ast::ast::ZoneObject;
    //use crate::base::threaded_list::ThreadedList;
    //use crate::common::globals::Context;
    //use crate::init::bootstrapper::Scope;
    //use crate::objects::objects::ScopeInfo;
    //use crate::parsing::parser::AstNodeFactory;
    //use crate::parsing::parser::SloppyBlockFunctionStatement;
    //use crate::strings::string_set::StringSet;
    //use crate::zone::zone::Zone;
    //use std::collections::HashMap;

    use std::collections::HashMap;
    use std::ptr;
    use std::sync::Mutex;

    //use v8::internal::base::PointerWithPayloadTraits;
    use crate::ast::ast::*;
    use crate::init::bootstrapper::*;

    pub struct VariableMap {}
    pub struct base {}
    pub struct VariableProxy {}
    pub struct AstRawString {}
    pub struct PreparseDataBuilder {}
    pub struct SloppyBlockFunctionStatement {}
    pub struct StringSet {}
    pub struct Statement {}
    pub struct Declaration {}
    pub struct Parser {}
    pub struct ZoneObject {}
    pub enum ScopeType {}
    pub enum LanguageMode {}
    pub struct DeclarationScope {}
    pub struct ModuleScope {}
    pub struct ClassScope {}
    pub struct AstNodeFactory {}
    pub struct AstValueFactory {}
    pub struct ParseInfo {}
    pub struct FunctionKind {}
    pub struct Script {}
    pub struct AstStringConstants {}
    pub struct DirectHandle<T> {}
    pub struct Address {}
    pub struct Variable {}
    pub struct IndirectHandle<T> {}
    pub enum VariableMode {}
    pub enum VariableKind {}
    pub enum InitializationFlag {}
    pub enum MaybeAssignedFlag {}
    pub enum IsStaticFlag {}
    pub struct Handle<T> {}
    pub struct FixedArray {}
    pub struct base::iterator_range<T> {}
    pub struct Graph {}
    pub struct OpIndex {}
    pub struct InstructionOperand {}
    pub struct turboshaft {}
    pub struct Label {}
    pub struct Control {}
    pub struct OptimizedCompilationInfo {}
    pub struct ClassLiteralPropertyKind {}
    pub struct Symbol {}
    pub struct Item {}
    pub struct MachineType {}
    pub struct StructDeclaration {}
    pub struct SourcePosition {}
    pub struct Flag {}
    pub struct Vector<T> {}
    pub struct Context {}
    pub struct FmtElm {}
    pub struct Global<T> {}
    pub struct HeapObject {}
    pub struct Key {}
    pub struct ControlFlowGraph {}
    pub struct Op {}
    pub struct LocationOperand {}
    pub struct MemOperand {}
    pub struct Tagged<T> {}
    pub struct Write {}
    pub struct Isolate {}
    pub struct Class {}
    pub struct Parameter {}
    pub struct Map {}
    pub struct Field {}
    pub struct AsmType {}
    pub struct AsmTypeVector {}
    pub struct Root {}
    pub struct Location {}
    pub struct JSHeapBroker {}
    pub struct MapRef {}
    pub struct Block {}
    pub struct MachineRepresentation {}
    pub struct SharedFunctionInfo {}
    pub struct Heap {}
    pub enum AtomicMemoryOrder {}
    pub struct Tagged_t {}
    pub struct Debug {}
    pub struct FunctionLiteral {}
    pub struct Local<'a, T> {}
    pub struct MaybeLocal<'a, T> {}
    pub struct Error {}
    pub struct JsonPosition {}
    pub struct JsonObject {}
    pub struct ZonePtrList<T> {}
    pub struct CaseClause {}
    pub struct SourceTextModuleDescriptor {}
    pub struct LocalHeap {}
    pub struct VisitResult {}
    pub struct Binding<T> {}
    pub struct Value {}
    pub enum ClassLiteralPropertyKind {}
    pub struct Edge {}
    pub struct Node {}
    pub enum OpIndexKind {}
    pub struct AsmTypeGraph {}
    pub struct ClassDescriptor {}
    pub struct JsonArray {}
    pub struct Source {}
    pub struct SourceRange {}
    pub struct LocalValue {}
    pub struct SaveOptions {}
    pub struct InnerPointerToCodeCacheEntry {}

    pub struct ScopeInfo {}
    pub struct SourceRangeAstVisitor {}
    pub struct turboshaft::BlockInstrumentationReducer {}
    pub enum StoreOp {}
    pub enum MemoryRepresentation {}
    pub enum WriteBarrierKind {}
    pub struct AsmGraphAssembler {}
    pub struct NodeId {}
    pub struct wasm {}
    pub enum BytecodeArray {}
    pub struct FrameDescription {}
    pub struct RootIndex {}
    pub struct AsmTyper {}
    pub struct Function {};
    pub struct SourceTextModule {}
    pub struct FlagList {}
    pub struct AddressRegion {}
    pub struct AddressMap {}
    pub struct Object {}
    pub struct RegisterConfiguration {}
    pub struct Register {}
    pub struct SignalHandler {}
    pub struct Cluster {}
    pub struct AstStringConstants {}
    pub struct ItemVector {}
    pub struct VectorSlot<T> {}
    pub struct ZoneObjectList<T> {}
    pub struct ClusterData {}
    pub struct ScopeInfoData {}
    pub struct ZoneList<T> {}
    pub struct Type {}
    pub struct TypeVector {}
    pub struct CallContext {}
    pub struct ConstantCache {}
    pub struct TypeCache {}
    pub struct FunctionType {}
    pub struct JSCreateClosureParameters {}
    pub struct TFGraph {}
    pub struct GraphCache {}
    pub struct JsonValue {}
    pub struct StringStream {}
    pub struct Number {}
    pub struct JsonWriter {}
    pub struct RelocInfo {}
    pub struct Code {}
    pub struct TaggedField<T> {}
    pub struct Constant {}
    pub struct MaybeObject {}
    pub struct WeakFixedArray {}
    pub struct DebugInfo {}
    pub struct DebugFunctionData {}
    pub struct CodeEntry {}
    pub struct OffsetTable {}
    pub struct CodeDataContainer {}
    pub struct StackFrameData {}
    pub struct StackFrameInfo {}
    pub struct CodeTableOffsetEntry {}
    pub struct LocationData {}
    pub struct Name {}
    pub struct AddressTableEntry {}
    pub struct Slot {}
    pub struct OffsetAndSize {}
    pub struct LocalIsolate {}
    pub struct Assembler {}
    pub struct AsmTypeBuilder {}
    pub struct DirectHandleList {}
    pub struct VectorBuffer {}
    pub struct SourceRangeAstVisitorMethods {}
    pub struct ClusterMap {}
    pub struct DirectHandleListAllocator {}
    pub struct StringKey {}
    pub struct SharedStringTable {}
    pub struct DirectHandleData {}
    pub struct EdgeInfo {}
    pub struct AstNode {}
    pub struct List{}
    pub enum String{}
    pub struct IsolateGroup{}
    pub struct BytecodeLoopAssignments {}
    pub enum ClassLiteralPropertyKind{}
    pub struct FrameAndOopAndDelta{}
    pub enum RelocInfoMode{}
    pub struct StringInputBuffer {}
    pub enum LookupHoistingMode{}
    pub struct Assignment {}
    pub enum REPLMode{}
    pub struct Weak<T> {}
    pub struct ObjectRef {}

}
