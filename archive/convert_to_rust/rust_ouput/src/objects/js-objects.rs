// Converted from V8 C++ source files:
// Header: js-objects.h
// Implementation: js-objects.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::cell::RefCell;
use std::rc::Rc;

use crate::codegen::code_stub_assembler::AllocationPolicy;
use crate::codegen::code_stub_assembler::LanguageMode;
use crate::codegen::code_stub_assembler::ShouldThrow;
use crate::compiler::c_linkage::Properties;
use crate::compiler::loop_analysis::IsCompiledScope;
use crate::heap::heap::Heap;
use crate::init::bootstrapper::Bootstrapper;
use crate::init::v8::V8;
use crate::interpreter::bytecode_generator::FunctionTemplate;
use crate::objects::property_descriptor::PropertyDescriptor;
use crate::objects::property_descriptor::PropertyKey;
use crate::objects::slots_inl::DisallowGarbageCollection;
use crate::runtime::runtime_regexp::ShouldThrow;
use crate::runtime::runtime_scopes::LanguageMode;
use crate::sandbox::indirect_pointer_inl::AcquireLoadTag;
use crate::sandbox::trusted_pointer_table::IndirectPointerTag;
use crate::codegen::code_stub_assembler::isolate;

pub struct JSReceiver {}

pub struct JSObject {}

pub struct JSGlobalProxy {}

pub struct LookupIterator {}

pub struct PropertyDescriptor {}

pub struct PropertyKey {}

pub struct NativeContext {}

pub struct IsCompiledScope {}

pub struct StackTraceInfo {}

pub struct SwissNameDictionary {}

pub struct ElementsAccessor {}

pub struct Undefined {}

pub struct Null {}

pub struct String {}

pub struct JSFunction {}

pub struct Object {}

pub struct Smi {}

pub enum MessageTemplate {}

pub struct JSAsyncFromSyncIterator {}

pub struct JSStringIterator {}

pub struct JSValidIteratorWrapper {}

pub enum class BranchHint{};

pub enum class BranchSemantics{};

pub struct JSPrimitiveWrapper {}

pub struct JSDate {}

pub struct JSMessageObject {}

pub struct JSTypedArray {}

pub struct PropertyArray {}

pub enum PropertiesEnumerationMode {}

pub struct JSGlobalObject {}

pub struct ShouldThrow {}

pub struct InterceptorInfo {}

pub enum InterceptorResult {}

pub struct AccessorInfo {}

pub enum EnforceDefineSemantics {}

pub enum StoreOrigin {}

pub struct DirectHandle<T> {}

pub struct HeapObject {}

pub struct JSProxy {}

pub struct JSAny {}

pub struct PropertyAttributes {}

pub struct Number {}

pub struct JSArray {}

pub enum CppHeapPointerTag {}

pub struct Name {}

pub struct BigInt {}

pub struct Symbol {}

pub struct Script {}

pub struct FixedArray {}

pub struct WeakArrayList {}

pub struct SloppyArgumentsElements {}

pub struct VRegister {}

pub struct JSWeakMap {}

pub struct JSWeakSet {}

pub struct ReadOnlyRoots {}

pub struct JSGlobalObject {}

pub struct PropertyDetails {}

pub struct JSFinalizationRegistry {}

pub struct V8_EXPORT_PRIVATE {}

pub enum HeapPointerTag {}

pub struct GlobalDictionary {}

pub struct NameDictionary {}

pub enum OpIndex {}

pub struct Operand {}

pub struct Immediate {}

pub struct Label {}

pub struct Address {}

pub struct PropertyCell {}

pub struct FixedDoubleArray {}

pub struct DirectHandle<T> {}

pub struct FieldIndex {}

pub struct Map {}

pub struct V8_PRESERVE_MOST {}

pub struct PropertyCellType {}

pub enum SeqCstAccessTag {}

pub struct v8 {}

pub struct ExposedTrustedObject {}

pub struct PtrComprCageBase {}

pub struct HandleScope {}

pub struct List {}

pub struct v8 {}

pub struct JSPrototype {}

pub struct MapWord {}

pub struct JSPluralRules {}

pub struct Local<T> {}

pub struct Value {}

pub struct ExposedTrustedObject {}

pub struct CallInterfaceDescriptorData {}

pub struct CallInterfaceDescriptor {}

pub struct WasmTableObject {}

pub struct WasmInternalFunction {}

pub enum PropertiesEnumerationMode {}

pub enum SeqCstAccessTag {}

pub enum Access {
    kHasNoSideEffect,
}

pub struct Js_objects {}

pub struct JSProxy {}

pub struct JSFunction {}

pub struct JSObject {}

pub struct ShouldThrow {}

pub struct PropertiesEnumerationMode {}

pub struct InterceptorInfo {}

pub enum MessageTemplate {}

pub struct StackTraceInfo {}

pub struct ObjectSlot {}

pub struct AcquireLoadTag {}

pub struct PropertyFilter {}

pub struct ElementsKind {}

pub struct AccessorPair {}

pub enum PropertyConstness {}

pub enum SeqCstAccessTag {}

pub struct NumberDictionary {}

pub struct HeapNumber {}

pub struct Bootstrapper {}

pub struct JSStringIterator {}

pub struct Arguments {}

pub struct Isolate {}

pub struct HandleScope {}

pub struct Properties {}

pub struct Maybe<T> {}

pub struct ShouldThrow {}

pub struct FunctionTemplate {}

pub struct JSMessageObject {}

pub struct JSProxy {}

pub struct GlobalDictionary {}

pub struct NativeContext {}

pub struct Js_receiver {}

pub struct JSReceiver {}

pub struct GlobalDictionary {}

pub struct LookupIterator {}

pub struct SwissNameDictionary {}

pub struct Isolate {}

pub struct This{
    dummy : i32
}

pub struct FixedArray {}

pub struct PropertyDescriptor {}

pub struct HeapObject {}

pub struct PropertyKey {}

pub struct PropertyAttributes {}

pub struct ShouldThrow {}

pub struct PropertyKind {}

pub struct Representation {}

pub struct FieldIndex {}

pub enum  MessageTemplate {}

pub struct ReadOnlyRoots {}

pub enum PropertyDetails {}

pub struct InternalIndex {}

pub struct Message {}

pub struct Code {}

pub struct MaybeHandle<T> {}

pub struct GlobalDictionary {}

pub struct Code {}

pub struct v8 {}

pub enum AllocationPolicy {
    kAllocationAllowed,
    kAllocationDisallowed,
}

pub struct If {}

pub struct v8 {}

pub struct Internal {}

pub struct DirectHandleVector {}

pub struct Property {
    value : i32
}

pub struct Iterable {}

pub struct BodyDescriptor {}

pub struct prototype {}

pub struct JSObject {}

pub struct PropertyKind {}

pub struct iterator {}

pub struct WasmCodePointer {}

pub struct TaggedObject {}

pub struct WasmSuspendingObject {}

pub struct ValueType {}

pub struct String {}

pub struct OpIndex {}

pub struct InstructionOperand {}

pub struct DirectHandle<T> {}

pub struct FileEvent {}

pub struct Diamond {}

pub enum BranchHint {}

pub struct JsAny {}

pub struct Space {}

pub struct Entry {}

pub struct Segment {}

pub struct FreelistHead {}

pub struct ExposedTrustedObject {}

pub enum READ_ONLY {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/literal-objects.h
pub enum ReleaseStoreTag {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/string-inl.h
pub enum WriteBarrierMode {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup-inl.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/property-descriptor.h
pub struct FieldIndex {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum class FPUControlRegister{
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/megadom-handler.h
pub struct BodyDescriptor;
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum class FPUControlRegister{
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/ppc/macro-assembler-ppc.h
struct UseScratchRegisterScope{dummy : i32}
}

// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/literal-objects.h
pub enum ReleaseStoreTag {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/string-inl.h
pub enum WriteBarrierMode {
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/lookup-inl.h
pub enum void {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/property-descriptor.h
pub struct FieldIndex {}
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum class FPUControlRegister{
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/megadom-handler.h
pub struct BodyDescriptor;
// From /home/kathirks_gc/v8_go/archive/codebase/src/objects/intl-objects.h
pub enum class FPUControlRegister{
// From /home/kathirks_gc/v8_go/archive/codebase/src/codegen/ppc/macro-assembler-ppc.h
struct UseScratchRegisterScope{dummy : i32}
    #[allow(non_upper_case_globals)]
    #[no_mangle]
    pub static ReadOnly : i32 = 0;
}
}
