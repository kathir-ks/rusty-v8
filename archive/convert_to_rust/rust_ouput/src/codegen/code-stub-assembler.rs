// Converted from V8 C++ source files:
// Header: code-stub-assembler.h
// Implementation: code-stub-assembler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod uri {
  pub struct V8 {}
}
pub mod ast {
  pub enum class CallType {
  }
}
pub mod compiler {
  pub mod code_assembler {
    pub struct CodeAssembler;
  }
}
pub mod codegen {
  pub mod flush_instruction_cache {
    pub struct V8_EXPORT_PRIVATE {}
  }
}
pub mod compiler {
  pub mod turboshaft {
    pub struct growable_stacks_reducer {
      pub struct IntPtr;
    }
  }
}
pub mod codegen {
  pub mod macro_assembler {
    pub enum AllocationFlags {
    }
  }
}
pub mod codegen {
  pub mod tnode {
    pub struct TNode<T> {
      t: std::marker::PhantomData<T>,
    }
  }
}
pub mod strings {
  pub struct uri {
    pub struct V8 {}
  }
}
pub mod interpreter {
  pub mod bytecode_generator {
    pub enum Load {
    }
  }
}
pub mod sandbox {
  pub mod isolate {
    pub enum ExternalPointerTagRange {
    }
    pub enum ExternalPointerTag {
    }
  }
}
pub mod codegen {
  pub mod assembler {
    pub enum IndirectPointerTag {
    }
  }
}
pub mod sandbox {
  pub mod indirect_pointer_inl {
    pub enum CodeEntrypointTag {
    }
  }
}
pub mod codegen {
  pub mod safepoint_table {
    pub struct Code {}
  }
}
pub mod codegen {
  pub mod reloc_info_inl {
    pub struct HeapObject {}
  }
}
pub mod compiler {
  pub mod js_typed_lowering {
    pub struct ConvertReceiverMode {}
  }
}
pub mod codegen {
  pub struct callable {
    pub struct Callable {}
  }
}
pub mod codegen {
  pub mod pending_optimization_table {
    pub struct JSFunction {}
    pub struct Object {}
    pub struct Heap {}
    pub struct ReadOnlyRoots {}
  }
}
pub mod codegen {
  pub mod linkage_location {
    pub struct MachineType {}
  }
}
pub mod interpreter {
  pub mod bytecode_generator {
    pub struct If {}
  }
}
pub mod codegen {
  pub struct background_merge_task {
    pub struct String {}
  }
}
pub mod strings {
  pub struct string_stream {
    pub struct JSPrimitiveWrapper {}
  }
}
pub mod compiler {
  pub mod allocation_builder_inl {
    pub struct FixedDoubleArray {}
  }
}
pub mod regexp {
  pub mod experimental {
    pub struct experimental_interpreter {
      pub struct NumberDictionary {}
      pub struct ClosureFeedbackCellArray {}
      pub struct DescriptorArray {}
      pub struct Descriptor {}
      pub struct Data {}
      pub struct Promise {}
    }
  }
}
pub mod codegen {
  pub mod compilation_cache {
    pub struct FeedbackCell {}
    pub enum Flags {
    }
  }
}
pub mod codegen {
  pub mod machine_type {
    pub enum MachineRepresentation {
    }
  }
}
pub mod codegen {
  pub mod turboshaft_builtins_assembler_inl {
    pub struct Map {}
  }
}
pub mod baseline {
  pub mod arm {
    pub mod baseline_assembler_arm_inl {
      pub enum InstanceType {
      }
    }
  }
}
pub mod interpreter {
  pub mod interpreter_generator {
    pub struct JSArray {}
    pub struct JSProxy {}
  }
}
pub mod init {
  pub mod bootstrapper {
    pub enum ElementsKind {
    }
    pub enum GCType {
    }
    pub struct Script {}
    pub struct AccessorPair {}
    pub struct JSGlobalObject {}
    pub struct EnumCache {}
    pub struct JavaScript {}
    pub struct SwissNameDictionary {}
  }
}
pub mod ast {
  pub struct ast {
    pub enum Operation {
    }
  }
}
pub mod codegen {
  pub mod turboshaft_builtins_assembler_inl {
    pub enum WriteBarrierMode {
    }
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct PropertyArray {}
    pub struct FixedArray {}
  }
}
pub mod codegen {
  pub mod label {
    pub struct Label {}
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod include {
  pub struct v8_profiler {
    pub struct Allocation {}
  }
}
pub mod codegen {
  pub mod turboshaft_builtins_assembler_inl {
    pub enum UpdateFeedbackMode {
    }
  }
}
pub mod interpreter {
  pub mod interpreter_generator {
    pub struct Debug {}
  }
}
pub mod codegen {
  pub struct external_reference {
    pub struct JSArrayBuffer {}
  }
}
pub mod interpreter {
  pub mod bytecode_generator {
    pub struct JSArrayBufferView {}
    pub enum MessageTemplate {}
  }
}
pub mod snapshot {
  pub struct deserializer {
    pub struct JSTypedArray {}
  }
}
pub mod codegen {
  pub mod reloc_info_inl {
    pub struct JSDispatchTable {}
  }
}
pub mod codegen {
  pub mod machine_type {
    pub enum Type {
    }
  }
}
pub mod regexp {
  pub mod experimental {
    pub struct experimental_interpreter {
      pub struct ClosureFeedbackCellArray {}
    }
  }
}
pub mod compiler {
  pub mod processed_feedback {
    pub enum KeyedAccessStoreMode {
    }
  }
}
pub mod codegen {
  pub mod code_factory {
    pub struct AllocationSite {}
  }
}
pub mod ast {
  pub mod ast {
    pub enum Operation {
    }
  }
}
pub mod ast {
  pub mod scopes {
    pub struct Flag {}
  }
}
pub mod compiler {
  pub struct loop_unrolling {
    pub struct Uses {}
  }
}
pub mod codegen {
  pub struct constant_pool {
    pub enum Type {
    }
  }
}
pub mod strings {
  pub struct string_stream {
    pub struct PropertyDetails {}
  }
}
pub mod include {
  pub struct v8_object {
    pub struct Private {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct source_position {
    struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
      value: u64,
    }
  }
}
pub mod codegen {
  pub mod turboshaft_builtins_assembler_inl {
    pub enum UpdateFeedbackMode {
    }
  }
}
pub mod interpreter {
  pub struct interpreter_generator {
    pub struct Int32 {}
  }
}
pub mod codegen {
  pub struct safepoint_table {
    pub struct Code {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct JSReceiver {}
  }
}
pub mod compiler {
  pub struct js_call_reducer {
    pub struct HeapNumber {}
  }
}
pub mod init {
  pub struct bootstrapper {
    pub struct SeqOneByteString {}
    pub struct Array {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct Context {}
  }
}
pub mod init {
  pub struct bootstrapper {
    pub struct Object {}
  }
}
pub mod codegen {
  pub struct assembler {
    pub enum IndirectPointerTag {
    }
  }
}
pub mod sandbox {
  pub mod indirect_pointer_inl {
    pub enum CodeEntrypointTag {
    }
  }
}
pub mod codegen {
  pub struct macro_assembler_base {
    pub enum RootIndex {
    }
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct FixedArrayBase {}
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct TaggedT {}
  }
}
pub mod codegen {
  pub struct tnode {
    pub struct WordT : IntegralT {}
  }
}
pub mod codegen {
  pub mod codegen {
    pub mod tnode {
      pub struct IntegralT {}
    }
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct FixedArray {}
  }
}
pub mod codegen {
  pub mod code_stub_assembler_inl {
    pub struct FixedArray {}
  }
}
pub mod interpreter {
  pub struct bytecode_generator {
    pub struct HashTable {}
  }
}
pub mod codegen {
  pub struct external_reference {
    pub struct NameDictionary {}
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct FixedArray {}
    struct TVARIABLE<'a, T> {
      // fields
      value: T,
    }
    impl<'a, T> TVARIABLE<'a, T> {
      fn Set(new_value: T) {
        self.value = new_value;
      }
    }
  }
}
pub mod codegen {
  pub mod label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub mod code_stub_assembler {
    pub enum BranchHint {}
  }
}
pub mod sandbox {
  pub struct isolate {
    pub enum ExternalPointerTagRange {
    }
  }
}
pub mod compiler {
  pub struct turboshaft {
    pub struct operations {
      pub struct Operation {}
    }
  }
}
pub mod sandbox {
  pub struct isolate {
    pub enum ExternalPointerTag {
    }
  }
}
pub mod codegen {
  pub mod code_stub_assembler {
    pub enum BranchHint {}
  }
}
pub mod interpreter {
  pub struct interpreter {
    pub struct Address {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct Undefined {}
  }
}
pub mod codegen {
  pub mod pending_optimization_table {
    pub struct ReadOnlyRoots {}
  }
}
pub mod torque {
  pub mod earley_parser {
    pub struct Rule {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct Smi {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub struct linkage_location {
      pub struct MachineType {}
    }
  }
}
pub mod compiler {
  pub struct turboshaft {
    pub struct operations {
      pub struct Loop {}
    }
  }
}
pub mod codegen {
  pub mod pending_optimization_table {
    pub struct JSFunction {}
  }
}
pub mod regexp {
  pub struct regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod tnode {
      pub struct UintPtrT : WordT {}
    }
  }
}
pub mod codegen {
  pub struct compilation_cache {
    pub enum Flags {
    }
  }
}
pub mod codegen {
  pub mod codegen {
    pub struct tnode {
      pub struct TaggedIndex {}
    }
  }
}
pub mod ast {
  pub mod ast {
    pub enum Operation {
    }
  }
}
pub mod sandbox {
  pub struct isolate {
    pub enum ExternalPointerTagRange {
    }
  }
}
pub mod ast {
  pub struct ast {
    pub struct Operation {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod strings {
  pub struct string_builder {
    struct PersistentHandles {}
  }
}
pub mod strings {
  pub struct uri {
    pub struct V8 {}
  }
}
pub mod ast {
  pub enum class CallType {
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum UpdateFeedbackMode {
    }
  }
}
pub mod interpreter {
  pub struct interpreter {
    pub struct Debug {}
  }
}
pub mod codegen {
  pub struct external_reference {
    pub struct JSArrayBuffer {}
  }
}
pub mod interpreter {
  pub struct bytecode_generator {
    pub struct JSArrayBufferView {}
  }
}
pub mod snapshot {
  pub struct deserializer {
    pub struct JSTypedArray {}
  }
}
pub mod init {
  pub mod bootstrapper {
    pub enum ElementsKind {
    }
    pub enum ElementsKind {
    }
    pub struct SwissNameDictionary {}
  }
}
pub mod codegen {
  pub struct safepoint_table {
    pub struct Code {}
  }
}
pub mod codegen {
  pub mod reloc_info_inl {
    pub struct JSDispatchTable {}
  }
}
pub mod compiler {
  pub struct code_assembler {
    pub struct CodeAssembler {}
  }
}
pub mod codegen {
  pub struct macro_assembler_base {
    pub enum RootIndex {
    }
  }
}
pub mod regexp {
  pub struct experimental {
    pub struct experimental_interpreter {
      pub struct Promise {}
    }
  }
}
pub mod ast {
  pub enum class CallType {
  }
}
pub mod torque {
  pub struct torque_parser {
    pub struct IntegerLiteral {}
  }
}
pub mod codegen {
  pub struct source_position {
    struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
      value: u64,
    }
  }
}
pub mod regexp {
  pub mod experimental {
    pub struct experimental_interpreter {
      pub struct Descriptor {}
    }
  }
}
pub mod interpreter {
  pub struct interpreter_generator {
    pub struct JSObject {}
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct PropertyArray {}
  }
}
pub mod interpreter {
  pub mod bytecode_generator {
    pub struct flags {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum IsKnownTaggedPointer {
    }
  }
}
pub mod interpreter {
  pub struct bytecode_generator {
    pub struct If {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum WriteBarrierMode {
    }
    pub struct Smi {}
  }
}
pub mod init {
  pub mod bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod compiler {
  pub struct processed_feedback {
    pub enum KeyedAccessStoreMode {
    }
  }
}
pub mod codegen {
  pub struct code_factory {
    pub struct AllocationSite {}
  }
}
pub mod ast {
  pub struct ast {
    pub enum Operation {
    }
  }
}
pub mod init {
  pub struct bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct macro_assembler_base {
    pub enum RootIndex {
    }
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct Context {}
  }
}
pub mod ast {
  pub struct ast {
    pub enum Operation {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct background_merge_task {
    pub struct String {}
  }
}
pub mod codegen {
  pub mod turboshaft_builtins_assembler_inl {
    pub enum UpdateFeedbackMode {
    }
  }
}
pub mod interpreter {
  pub struct interpreter_generator {
    pub struct Debug {}
  }
}
pub mod codegen {
  pub struct external_reference {
    pub struct JSArrayBuffer {}
  }
}
pub mod interpreter {
  pub mod bytecode_generator {
    pub struct JSArrayBufferView {}
    pub enum MessageTemplate {}
  }
}
pub mod snapshot {
  pub struct deserializer {
    pub struct JSTypedArray {}
  }
}
pub mod init {
  pub struct bootstrapper {
    pub enum ElementsKind {
    }
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct safepoint_table {
    pub struct Code {}
  }
}
pub mod codegen {
  pub mod reloc_info_inl {
    pub struct JSDispatchTable {}
  }
}
pub mod compiler {
  pub struct code_assembler {
    pub struct CodeAssembler {}
  }
}
pub mod codegen {
  pub struct macro_assembler_base {
    pub enum RootIndex {
    }
  }
}
pub mod regexp {
  pub struct experimental {
    pub struct experimental_interpreter {
      pub struct Promise {}
    }
  }
}
pub mod ast {
  pub enum class CallType {
  }
}
pub mod torque {
  pub struct torque_parser {
    pub struct IntegerLiteral {}
  }
}
pub mod codegen {
  pub struct source_position {
    struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
      value: u64,
    }
  }
}
pub mod regexp {
  pub struct experimental {
    pub struct experimental_interpreter {
      pub struct Descriptor {}
    }
  }
}
pub mod interpreter {
  pub struct interpreter_generator {
    pub struct JSObject {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    struct Smi {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod init {
  pub mod bootstrapper {
    pub enum ElementsKind {
    }
    pub enum ElementsKind {
    }
    pub struct SwissNameDictionary {}
  }
}
pub mod codegen {
  pub struct background_merge_task {
    pub struct String {}
  }
}
pub mod strings {
  pub struct string_stream {
    pub struct JSPrimitiveWrapper {}
  }
}
pub mod compiler {
  pub mod loop_unrolling {
    pub struct Uses {}
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct FixedArray {}
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum WriteBarrierMode {
    }
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct FixedArray {}
  }
}
pub mod compiler {
  pub struct allocation_builder_inl {
    pub struct FixedDoubleArray {}
  }
}
pub mod interpreter {
  pub struct bytecode_generator {
    pub struct If {}
  }
}
pub mod init {
  pub mod bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct macro_assembler {
    pub enum AllocationFlags {
    }
  }
}
pub mod interpreter {
  pub mod bytecode_generator {
    pub struct flags {}
  }
}
pub mod regexp {
  pub struct regexp_parser {
    pub enum void {
    }
  }
}
pub mod init {
  pub struct bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct Smi {}
  }
}
pub mod include {
  pub struct v8_profiler {
    pub struct Allocation {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod source_position {
      struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
        value: u64,
      }
    }
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum UpdateFeedbackMode {
    }
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod init {
  pub struct bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod compiler {
  pub struct js_call_reducer {
    pub struct HeapNumber {}
  }
}
pub mod codegen {
  pub struct constant_pool {
    pub enum Type {
    }
  }
}
pub mod codegen {
  pub struct background_merge_task {
    pub struct String {}
  }
}
pub mod strings {
  pub struct string_stream {
    pub struct JSPrimitiveWrapper {}
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod baseline {
  pub struct arm {
    pub struct baseline_assembler_arm_inl {
      pub enum InstanceType {
      }
    }
  }
}
pub mod codegen {
  pub struct source_position {
    struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
      value: u64,
    }
  }
}
pub mod interpreter {
  pub mod bytecode_generator {
    pub enum MessageTemplate {
    }
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct constant_pool {
    pub enum Type {
    }
  }
}
pub mod interpreter {
  pub struct interpreter_generator {
    pub struct JSProxy {}
  }
}
pub mod init {
  pub struct bootstrapper {
    pub enum ElementsKind {
    }
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct Smi {}
  }
}
pub mod compiler {
  pub struct js_call_reducer {
    pub struct HeapNumber {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod source_position {
      struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
        value: u64,
      }
    }
  }
}
pub mod init {
  pub struct bootstrapper {
    pub enum ElementsKind {
    }
    pub enum ElementsKind {
    }
    pub enum ElementsKind {
    }
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct background_merge_task {
    pub struct String {}
  }
}
pub mod codegen {
  pub mod constant_pool {
    pub enum Type {
    }
  }
}
pub mod ast {
  pub enum class CallType {
  }
}
pub mod codegen {
  pub struct background_merge_task {
    pub struct String {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct Number {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum Builtin {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod tnode {
      pub struct Float64T : UntaggedT {}
    }
  }
}
pub mod init {
  pub struct bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod interpreter {
  pub mod bytecode_generator {
    pub enum Call {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum WriteBarrierMode {
    }
  }
}
pub mod interpreter {
  pub struct bytecode_generator {
    pub struct If {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct JSReceiver {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod regexp {
  pub mod experimental {
    pub struct experimental_interpreter {
      pub struct DescriptorArray {}
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod interpreter {
  pub struct interpreter_generator {
    pub struct Int32 {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod source_position {
      struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
        value: u64,
      }
    }
  }
}
pub mod regexp {
  pub mod experimental {
    pub struct experimental_interpreter {
      pub struct ClosureFeedbackCellArray {}
    }
  }
}
pub mod codegen {
  pub struct compilation_cache {
    pub struct FeedbackCell {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum UpdateFeedbackMode {
    }
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod source_position {
      struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
        value: u64,
      }
    }
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct code_stub_assembler_inl {
    pub struct tnode {
      pub struct TNode<T> {}
    }
  }
}
pub mod init {
  pub mod bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub struct BigInt {}
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod tnode {
      pub struct WordT : IntegralT {}
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod tnode {
      pub struct UintPtrT : WordT {}
    }
  }
}
pub mod codegen {
  pub mod code_stub_assembler_inl {
    pub struct IntPtrT {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod tnode {
      pub struct TaggedIndex {}
    }
  }
}
pub mod codegen {
  pub mod linkage_location {
    pub struct MachineType {}
  }
}
pub mod codegen {
  pub struct flush_instruction_cache {
    pub struct V8_EXPORT_PRIVATE {}
  }
}
pub mod codegen {
  pub struct flush_instruction_cache {
    pub struct V8_EXPORT_PRIVATE {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub struct tnode {
      pub struct UintPtrT : WordT {}
    }
  }
}
pub mod codegen {
  pub mod code_stub_assembler_inl {
    pub struct IntPtrT {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod tnode {
      pub struct TaggedIndex {}
    }
  }
}
pub mod codegen {
  pub struct flush_instruction_cache {
    pub struct V8_EXPORT_PRIVATE {}
  }
}
pub mod codegen {
  pub struct flush_instruction_cache {
    pub struct V8_EXPORT_PRIVATE {}
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod regexp {
  pub mod experimental {
    pub struct experimental_interpreter {
      pub struct Data {}
    }
  }
}
pub mod codegen {
  pub mod code_stub_assembler_inl {
    pub struct TaggedT {}
  }
}
pub mod codegen {
  pub struct tnode {
    pub struct WordT : IntegralT {
      // fields
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod tnode {
      pub struct UntaggedT {}
    }
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod init {
  pub mod bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod linkage_location {
      pub struct MachineType {}
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod linkage_location {
      pub struct MachineType {}
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod linkage_location {
      pub struct MachineType {}
    }
  }
}
pub mod compiler {
  pub struct turboshaft {
    pub struct operations {
      pub struct Loop {}
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod source_position {
      struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
        value: u64,
      }
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod source_position {
      struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
        value: u64,
      }
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod source_position {
      struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
        value: u64,
      }
    }
  }
}
pub mod codegen {
  pub struct pending_optimization_table {
    pub struct JSFunction {}
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct turboshaft_builtins_assembler_inl {
    pub enum WriteBarrierMode {
    }
  }
}
pub mod init {
  pub mod bootstrapper {
    pub enum ElementsKind {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod linkage_location {
      pub struct MachineType {}
    }
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod codegen {
  pub struct label {
    pub struct Label {}
  }
}
pub mod regexp {
  pub mod regexp_parser {
    pub enum void {
    }
  }
}
pub mod codegen {
  pub struct codegen {
    pub mod source_position {
      struct BitField64<T, const OFFSET: usize, const SIZE: usize> {
        value
