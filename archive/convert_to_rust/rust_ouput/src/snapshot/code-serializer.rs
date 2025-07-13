// Converted from V8 C++ source files:
// Header: code-serializer.h
// Implementation: code-serializer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
pub mod internal {
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

pub struct AlignedCachedData {
    owns_data_: bool,
    rejected_: bool,
    data_: *const u8,
    length_: i32,
}

impl AlignedCachedData {
    pub fn new(data: *const u8, length: i32) -> Self {
        let mut aligned_data = AlignedCachedData {
            owns_data_: false,
            rejected_: false,
            data_: data,
            length_: length,
        };
        if !Self::is_aligned(data as usize, 8) {
            let mut copy: Vec<u8> = vec![0; length as usize];
            let data_slice = unsafe { std::slice::from_raw_parts(data, length as usize) };
            copy.copy_from_slice(data_slice);
            aligned_data.data_ = copy.as_ptr();
            aligned_data.acquire_data_ownership();
        }
        aligned_data
    }

    fn is_aligned(ptr: usize, alignment: usize) -> bool {
        ptr % alignment == 0
    }

    pub fn data(&self) -> *const u8 {
        self.data_
    }

    pub fn length(&self) -> i32 {
        self.length_
    }

    pub fn rejected(&self) -> bool {
        self.rejected_
    }

    pub fn reject(&mut self) {
        self.rejected_ = true;
    }

    pub fn has_data_ownership(&self) -> bool {
        self.owns_data_
    }

    pub fn acquire_data_ownership(&mut self) {
        assert!(!self.owns_data_);
        self.owns_data_ = true;
    }

    pub fn release_data_ownership(&mut self) {
        assert!(self.owns_data_);
        self.owns_data_ = false;
    }
}

impl Drop for AlignedCachedData {
    fn drop(&mut self) {
        if self.owns_data_ {
            let data_ptr = self.data_ as *mut u8;
            unsafe {
                let _ = Vec::from_raw_parts(data_ptr, self.length_ as usize, self.length_ as usize);
            }
        }
    }
}

pub enum SerializedCodeSanityCheckResult {
    kSuccess,
    kMagicNumberMismatch,
    kVersionMismatch,
    kSourceMismatch,
    kFlagsMismatch,
    kChecksumMismatch,
    kInvalidHeader,
    kLengthMismatch,
    kReadOnlySnapshotChecksumMismatch,
    kLast,
}

pub struct PersistentHandles {}
pub struct BackgroundMergeTask {}
pub struct ScriptCompiler {}
impl ScriptCompiler {
    pub struct CachedData {
        data_: *const u8,
        length_: i32,
        buffer_policy_: CachedDataPolicy,
    }

    impl CachedData {
        pub fn new(data: *const u8, length: i32, buffer_policy: CachedDataPolicy) -> Self {
            CachedData {
                data_: data,
                length_: length,
                buffer_policy_: buffer_policy,
            }
        }
    }

    pub enum CachedDataPolicy {
        BufferOwned,
    }
}
pub struct Isolate {
    counters_: Counters,
    snapshot_blob_: *mut SnapshotBlob,
}

impl Isolate {
    pub fn counters(&self) -> &Counters {
        &self.counters_
    }

    pub fn snapshot_blob(&self) -> *mut SnapshotBlob {
        self.snapshot_blob_
    }

    pub fn needs_source_positions(&self) -> bool {
        false
    }
    pub fn factory(&self) -> Factory{
        Factory{}
    }
    pub fn interpreted_frames_native_stack(&self) -> bool {
        false
    }

    pub fn is_logging_code_creation(&self) -> bool {
        false
    }
    pub fn as_local_isolate(&self) -> &LocalIsolate{
        &LocalIsolate{isolate_: self}
    }

    pub fn heap(&self) -> Heap {
        Heap{}
    }
    pub fn set_root_script_list(&self, _list: WeakArrayList){}
}

pub struct Counters {
    code_cache_reject_reason_: CodeCacheRejectReason,
    compile_serialize_: CompileSerialize,
}

impl Counters {
    pub fn code_cache_reject_reason(&self) -> &CodeCacheRejectReason {
        &self.code_cache_reject_reason_
    }
    pub fn compile_serialize(&self) -> &CompileSerialize{
        &self.compile_serialize_
    }
}

pub struct CodeCacheRejectReason {}

impl CodeCacheRejectReason {
    pub fn add_sample(&self, _sample: i32) {}
}

pub struct CompileSerialize {}

impl CompileSerialize {
    pub fn increment(&self) {}
}

pub struct SnapshotBlob {}
pub struct HandleScope {}
pub struct CodeSerializer {
    isolate_: *mut Isolate,
    source_hash_: u32,
    sink_: RefCell<Sink>,
    no_gc_: DisallowGarbageCollection,
    serializer_: Serializer,
    reference_map_: RefCell<ReferenceMap>,
}

impl CodeSerializer {
    pub fn serialize(isolate: *mut Isolate, info: Handle<SharedFunctionInfo>) -> *mut ScriptCompiler::CachedData {
        let timer = ElapsedTimer::new();
        let source = unsafe { (*(*info).script_).source_ };
        let wrapped_arguments = unsafe { (*(*info).script_).wrapped_arguments_ };
        let mut scope = HandleScope {};
        let cs = CodeSerializer::new(isolate,SerializedCodeData::source_hash(source, wrapped_arguments,ScriptOriginOptions{}));
        let mut no_gc = DisallowGarbageCollection {};
        let cached_data = cs.serialize_shared_function_info(info);
        let result = Box::into_raw(Box::new(ScriptCompiler::CachedData {
            data_: cached_data.data(),
            length_: cached_data.length(),
            buffer_policy_: ScriptCompiler::CachedDataPolicy::BufferOwned,
        }));
        cached_data.release_data_ownership();
        drop(cached_data);
        result
    }
    fn new(isolate: *mut Isolate, source_hash: u32) -> Self {
        CodeSerializer {
            isolate_: isolate,
            source_hash_: source_hash,
            sink_: RefCell::new(Sink::new()),
            no_gc_: DisallowGarbageCollection{},
            serializer_: Serializer::new(isolate, Snapshot::kDefaultSerializerFlags),
            reference_map_: RefCell::new(ReferenceMap::new())
        }
    }
    pub fn serialize_shared_function_info(&self, info: Handle<SharedFunctionInfo>) -> AlignedCachedData {
        let mut no_gc = DisallowGarbageCollection {};
        self.serializer_.visit_root_pointer(Root::kHandleScope, 0,FullObjectSlot{});
        self.serializer_.serialize_deferred_objects();
        self.serializer_.pad();
        let data = SerializedCodeData::new2(&self.sink_.borrow().data_, self);
        data.get_script_data()
    }
    pub fn source_hash(&self) -> u32 {
        self.source_hash_
    }
    pub fn reference_map(&self) -> &RefCell<ReferenceMap>{
        &self.reference_map_
    }
}

impl Drop for CodeSerializer {
    fn drop(&mut self) {
        self.serializer_.output_statistics("CodeSerializer");
    }
}

pub struct ScriptData {}
pub struct SerializedCodeData {
    data_: *mut u8,
    size_: i32,
    owns_data_: bool
}

impl SerializedCodeData {
    pub const kVersionHashOffset: u32 = SerializedCodeData::kMagicNumberOffset + SerializedCodeData::kUInt32Size;
    pub const kSourceHashOffset: u32 = SerializedCodeData::kVersionHashOffset + SerializedCodeData::kUInt32Size;
    pub const kFlagHashOffset: u32 = SerializedCodeData::kSourceHashOffset + SerializedCodeData::kUInt32Size;
    pub const kReadOnlySnapshotChecksumOffset: u32 = SerializedCodeData::kFlagHashOffset + SerializedCodeData::kUInt32Size;
    pub const kPayloadLengthOffset: u32 = SerializedCodeData::kReadOnlySnapshotChecksumOffset + SerializedCodeData::kUInt32Size;
    pub const kChecksumOffset: u32 = SerializedCodeData::kPayloadLengthOffset + SerializedCodeData::kUInt32Size;
    pub const kUnalignedHeaderSize: u32 = SerializedCodeData::kChecksumOffset + SerializedCodeData::kUInt32Size;
    pub const kHeaderSize: u32 = 32;
    pub const kMagicNumberOffset: u32 = 0;
    const kUInt32Size: u32 = 4;
    const kMagicNumber: u32 = 123456789;

    pub fn from_cached_data(
        isolate: *mut Isolate,
        cached_data: &mut AlignedCachedData,
        expected_source_hash: u32,
        rejection_result: &mut SerializedCodeSanityCheckResult,
    ) -> SerializedCodeData {
        let mut no_gc = DisallowGarbageCollection {};
        let mut scd = SerializedCodeData::new1(cached_data);
        *rejection_result = scd.sanity_check(
            Snapshot::extract_read_only_snapshot_checksum(unsafe { (*isolate).snapshot_blob() }),
            expected_source_hash,
        );
        if let SerializedCodeSanityCheckResult::kSuccess = *rejection_result {
        } else {
            cached_data.reject();
            return SerializedCodeData::new(std::ptr::null_mut(), 0);
        }
        scd
    }

    pub fn new(data: *mut u8, size: i32) -> Self {
        SerializedCodeData {
            data_: data,
            size_: size,
            owns_data_: false
        }
    }
    pub fn new1(data: &mut AlignedCachedData) -> Self {
        SerializedCodeData {
            data_: data.data_ as *mut u8,
            size_: data.length_,
            owns_data_: false
        }
    }

    pub fn new2(payload: &Vec<u8>, cs: &CodeSerializer) -> Self {
        let mut no_gc = DisallowGarbageCollection {};
        let size = SerializedCodeData::kHeaderSize + payload.len() as u32;
        let mut data: Vec<u8> = vec![0; size as usize];
        unsafe {
            (data.as_mut_ptr() as *mut SerializedCodeData).write(SerializedCodeData {
                data_: data.as_mut_ptr(),
                size_: size as i32,
                owns_data_: true
            });
        }
        let mut serialized_code_data = unsafe { data.as_mut_ptr() as *mut SerializedCodeData };
        unsafe { (*serialized_code_data).set_magic_number() };
        unsafe { (*serialized_code_data).set_header_value(SerializedCodeData::kVersionHashOffset, Version::hash()) };
        unsafe { (*serialized_code_data).set_header_value(SerializedCodeData::kSourceHashOffset, cs.source_hash()) };
        unsafe { (*serialized_code_data).set_header_value(SerializedCodeData::kFlagHashOffset, FlagList::hash()) };
        unsafe { (*serialized_code_data).set_header_value(SerializedCodeData::kReadOnlySnapshotChecksumOffset, Snapshot::extract_read_only_snapshot_checksum(unsafe { (*cs.isolate_).snapshot_blob() })) };
        unsafe { (*serialized_code_data).set_header_value(SerializedCodeData::kPayloadLengthOffset, payload.len() as u32) };
        unsafe { std::ptr::copy_nonoverlapping(payload.as_ptr(), (*serialized_code_data).data_.offset(SerializedCodeData::kHeaderSize as isize), payload.len()) };
        let checksum = if v8_flags.verify_snapshot_checksum {
             unsafe { (*serialized_code_data).checksum((*serialized_code_data).checksummed_content()) }
        } else { 0 };
        unsafe { (*serialized_code_data).set_header_value(SerializedCodeData::kChecksumOffset, checksum) };
        SerializedCodeData {
            data_: data.as_mut_ptr(),
            size_: size as i32,
            owns_data_: true
        }
    }

    pub fn sanity_check(
        &mut self,
        expected_ro_snapshot_checksum: u32,
        expected_source_hash: u32,
    ) -> SerializedCodeSanityCheckResult {
        let result = self.sanity_check_without_source(expected_ro_snapshot_checksum);
        if let SerializedCodeSanityCheckResult::kSuccess = result {
            self.sanity_check_just_source(expected_source_hash)
        } else {
            result
        }
    }

    pub fn sanity_check_just_source(&self, expected_source_hash: u32) -> SerializedCodeSanityCheckResult {
        let source_hash = self.get_header_value(SerializedCodeData::kSourceHashOffset);
        if source_hash != expected_source_hash {
            SerializedCodeSanityCheckResult::kSourceMismatch
        } else {
            SerializedCodeSanityCheckResult::kSuccess
        }
    }

    pub fn sanity_check_without_source(&self, expected_ro_snapshot_checksum: u32) -> SerializedCodeSanityCheckResult {
        if self.size_ < SerializedCodeData::kHeaderSize as i32 {
            return SerializedCodeSanityCheckResult::kInvalidHeader;
        }
        let magic_number = self.get_magic_number();
        if magic_number != SerializedCodeData::kMagicNumber {
            return SerializedCodeSanityCheckResult::kMagicNumberMismatch;
        }
        let version_hash = self.get_header_value(SerializedCodeData::kVersionHashOffset);
        if version_hash != Version::hash() {
            return SerializedCodeSanityCheckResult::kVersionMismatch;
        }
        let flags_hash = self.get_header_value(SerializedCodeData::kFlagHashOffset);
        if flags_hash != FlagList::hash() {
            return SerializedCodeSanityCheckResult::kFlagsMismatch;
        }
        let ro_snapshot_checksum = self.get_header_value(SerializedCodeData::kReadOnlySnapshotChecksumOffset);
        if ro_snapshot_checksum != expected_ro_snapshot_checksum {
            return SerializedCodeSanityCheckResult::kReadOnlySnapshotChecksumMismatch;
        }
        let payload_length = self.get_header_value(SerializedCodeData::kPayloadLengthOffset);
        let max_payload_length = self.size_ as u32 - SerializedCodeData::kHeaderSize;
        if payload_length > max_payload_length {
            return SerializedCodeSanityCheckResult::kLengthMismatch;
        }
        if v8_flags.verify_snapshot_checksum {
            let checksum = self.get_header_value(SerializedCodeData::kChecksumOffset);
            if self.checksum(self.checksummed_content()) != checksum {
                return SerializedCodeSanityCheckResult::kChecksumMismatch;
            }
        }
        SerializedCodeSanityCheckResult::kSuccess
    }

    pub fn source_hash(source: *mut String, wrapped_arguments: *mut FixedArray, origin_options: ScriptOriginOptions) -> u32 {
        let length = unsafe { (*source).length_ };
        let has_wrapped_arguments = !wrapped_arguments.is_null();
        let is_module = origin_options.is_module();
        let mut hash: u32 = 0;
        hash = Self::update_length(hash, length);
        hash = Self::update_has_wrapped_arguments(hash, has_wrapped_arguments);
        hash = Self::update_is_module(hash, is_module);
        hash
    }

    fn update_length(hash: u32, length: i32) -> u32 {
        hash | (length as u32 & 0x1FFFFFFF)
    }

    fn update_has_wrapped_arguments(hash: u32, has_wrapped_arguments: bool) -> u32 {
        if has_wrapped_arguments {
            hash | 0x20000000
        } else {
            hash
        }
    }

    fn update_is_module(hash: u32, is_module: bool) -> u32 {
        if is_module {
            hash | 0x40000000
        } else {
            hash
        }
    }

    pub fn get_script_data(mut self) -> AlignedCachedData {
        assert!(self.owns_data_);
        let result = AlignedCachedData::new(self.data_, self.size_);
        self.owns_data_ = false;
        self.data_ = std::ptr::null_mut();
        result
    }

    pub fn payload(&self) -> Vec<u8> {
        let payload_ptr = unsafe { self.data_.offset(SerializedCodeData::kHeaderSize as isize) };
        let length = self.get_header_value(SerializedCodeData::kPayloadLengthOffset) as usize;
        let mut payload: Vec<u8> = Vec::with_capacity(length);
        unsafe {
            std::ptr::copy_nonoverlapping(payload_ptr, payload.as_mut_ptr(), length);
            payload.set_len(length);
        }
        payload
    }

    fn allocate_data(&mut self, size: u32) {
        self.size_ = size as i32;
        self.data_ = unsafe {
            let mut data: Vec<u8> = vec![0; size as usize];
            self.owns_data_ = true;
            data.as_mut_ptr()
        };
    }
    fn checksummed_content(&self) -> Vec<u8>{
        unsafe { std::slice::from_raw_parts(self.data_.offset(SerializedCodeData::kHeaderSize as isize), (self.size_ as u32 - SerializedCodeData::kHeaderSize) as usize) }.to_vec()
    }
    fn set_magic_number(&mut self) {
        unsafe {
            *(self.data_ as *mut u32) = SerializedCodeData::kMagicNumber;
        }
    }

    fn set_header_value(&mut self, offset: u32, value: u32) {
        unsafe {
            *((self.data_ as usize + offset as usize) as *mut u32) = value;
        }
    }
    fn get_magic_number(&self) -> u32 {
        unsafe { *(self.data_ as *const u32) }
    }
    fn get_header_value(&self, offset: u32) -> u32 {
        unsafe { *((self.data_ as usize + offset as usize) as *const u32) }
    }

    fn checksum(&self, payload: Vec<u8>) -> u32 {
        if payload.is_empty(){
            return 0;
        }
        let mut checksum: u32 = 0;
        for byte in payload {
            checksum = checksum.wrapping_add(byte as u32);
        }
        checksum
    }
    pub fn from_cached_data_without_source(
        local_isolate: *mut LocalIsolate,
        cached_data: &mut AlignedCachedData,
        rejection_result: &mut SerializedCodeSanityCheckResult,
    ) -> SerializedCodeData {
        let mut no_gc = DisallowGarbageCollection {};
        let mut scd = SerializedCodeData::new1(cached_data);
        *rejection_result =
            scd.sanity_check_without_source(Snapshot::extract_read_only_snapshot_checksum(unsafe {
                (*local_isolate).snapshot_blob()
            }));
        if let SerializedCodeSanityCheckResult::kSuccess = *rejection_result {
        } else {
            cached_data.reject();
            return SerializedCodeData::new(std::ptr::null_mut(), 0);
        }
        scd
    }
    pub fn from_partially_sanity_checked_cached_data(
        cached_data: &mut AlignedCachedData,
        expected_source_hash: u32,
        rejection_result: &mut SerializedCodeSanityCheckResult,
    ) -> SerializedCodeData {
        let mut no_gc = DisallowGarbageCollection {};
        if let SerializedCodeSanityCheckResult::kSuccess = *rejection_result {
        } else {
            cached_data.reject();
            return SerializedCodeData::new(std::ptr::null_mut(), 0);
        }
        let mut scd = SerializedCodeData::new1(cached_data);
        *rejection_result = scd.sanity_check_just_source(expected_source_hash);
        if let SerializedCodeSanityCheckResult::kSuccess = *rejection_result {
        } else {
            cached_data.reject();
            return SerializedCodeData::new(std::ptr::null_mut(), 0);
        }
        scd
    }
}
impl Drop for SerializedCodeData{
    fn drop(&mut self) {
        if self.owns_data_ {
            unsafe {
                let _ = Vec::from_raw_parts(self.data_, self.size_ as usize, self.size_ as usize);
            }
        }
    }
}
pub struct ScriptOriginOptions {}
impl ScriptOriginOptions{
    pub fn is_module(&self) -> bool {
        false
    }
}
pub struct String {
    length_: i32,
}
pub struct FixedArray {}
pub struct Version {}
impl Version {
    pub fn hash() -> u32 {
        12345
    }
}

pub struct FlagList {}
impl FlagList {
    pub fn hash() -> u32 {
        54321
    }
}

pub struct Snapshot {}
impl Snapshot {
    pub const kDefaultSerializerFlags: i32 = 0;
    pub fn extract_read_only_snapshot_checksum(_snapshot_blob: *mut SnapshotBlob) -> u32 {
        98765
    }
}

pub struct DisallowGarbageCollection {}
pub struct SharedFunctionInfo {
    script_: *mut Script,
}
pub struct Handle<T> {
    value_: T,
}
impl<T> Handle<T>{
    
}
pub struct Script {
    source_: *mut String,
    wrapped_arguments_: *mut FixedArray,
    origin_options_: ScriptOriginOptions,
}
pub struct Sink {
    data_: Vec<u8>,
}
impl Sink{
    fn new() -> Self{
        Sink{data_: Vec::new()}
    }
}
pub enum Root {
    kHandleScope
}
pub struct FullObjectSlot {}
pub struct Serializer {
    isolate_: *mut Isolate
}
impl Serializer{
    fn new(isolate: *mut Isolate, _flag: i32) -> Self {
        Serializer{isolate_: isolate}
    }
    fn visit_root_pointer(&self, _root: Root, _i: i32, _slot: FullObjectSlot){}
    fn serialize_deferred_objects(&self){}
    fn pad(&self){}
    fn output_statistics(&self, _name: &str){}
}
pub struct ReferenceMap{

}
impl ReferenceMap{
    fn new() -> Self{
        ReferenceMap{}
    }
    fn add_attached_reference(&self, _s: *mut String){}
}
pub struct ElapsedTimer{

}
impl ElapsedTimer{
    fn new() -> Self{
        ElapsedTimer{}
    }
    fn start(&self){}
    fn elapsed(&self) -> Milliseconds{
        Milliseconds{}
    }
}
pub struct Milliseconds{}
impl Milliseconds{
    fn in_milliseconds_f(&self) -> f64{
        1.0
    }
}

pub struct LocalIsolate{
    isolate_: *mut Isolate
}
impl LocalIsolate{
    pub fn heap(&self) -> Heap {
        Heap{}
    }
    pub fn snapshot_blob(&self) -> *mut SnapshotBlob {
        unsafe { (*self.isolate_).snapshot_blob()}
    }
}
pub struct ObjectDeserializer{}
impl ObjectDeserializer{
    pub fn deserialize_shared_function_info(_isolate: *mut Isolate, _data: &SerializedCodeData, _source: *mut String) -> MaybeDirectHandle<SharedFunctionInfo>{
        MaybeDirectHandle{handle_: SharedFunctionInfo{script_: std::ptr::null_mut()}}
    }
}
pub struct MaybeDirectHandle<T> {
    handle_: T,
}
impl<T> MaybeDirectHandle<T> {
    pub fn to_handle(&self) -> Result<&T, ()> {
        Ok(&self.handle_)
    }
}
pub struct FunctionEvent{}
impl FunctionEvent{
    
}

pub struct SharedFunctionInfoRef{}
pub struct WeakArrayList{}

impl WeakArrayList{
    pub fn add_to_end(_isolate: *mut Isolate, _list: &WeakArrayList, _script: MaybeObjectDirectHandle) -> WeakArrayList{
        WeakArrayList{}
    }
}
pub struct MaybeObjectDirectHandle{}
impl MaybeObjectDirectHandle{
    pub fn weak(_script: &Script) -> Self{
        MaybeObjectDirectHandle{}
    }
}
pub struct Profile{
}

pub enum CodeTag{
    kFunction,
    kScript
}

pub struct OptimizedCompilationInfo{}
pub struct DebugInfo{}
pub enum CachedTieringDecision{
    kEarlySparkplug,
}
pub struct UncompiledDataWithoutPreparseDataWithJob{}
pub struct UncompiledDataWithPreparseDataAndJob{}
pub struct ScopeInfo{}
pub struct DependentCode{}
impl DependentCode{
    pub fn empty_dependent_code(_roots:ReadOnlyRoots) -> Self{
        DependentCode{}
    }
}
pub struct ReadOnlyRoots{

}
impl ReadOnlyRoots{
    pub fn undefined_value(&self) -> UnionOf<Smi, Symbol, Undefined> {
        UnionOf::Undefined(Undefined{})
    }
    pub fn uninitialized_symbol(&self) -> UnionOf<Smi, Symbol, Undefined> {
        UnionOf::Symbol(Symbol{})
    }
    pub fn empty_fixed_array(&self) -> FixedArray{
        FixedArray{}
    }
    pub fn empty_string(&self) -> String{
        String{length_: 0}
    }
}
pub enum UnionOf<Smi, Symbol, Undefined>{
    Smi(Smi),
    Symbol(Symbol),
    Undefined(Undefined)
}
pub struct Smi{}
pub struct Symbol{}
pub struct Undefined{}
pub struct InterpreterData{}
pub struct BytecodeArray{}
pub struct Code{
}
impl Code{
    
}
pub struct Builtins{}
impl Builtins{
    pub fn create_interpreter_entry_trampoline_for_profiling(_isolate: *mut Isolate) -> Handle<Code>{
        Handle{value_:Code{}}
    }
}
pub struct Factory{}
impl Factory{
    pub fn new_interpreter_data(&self, _bytecode: Handle<BytecodeArray>, _code: Handle<Code>) -> Handle<InterpreterData>{
        Handle{value_: InterpreterData{}}
    }
    pub fn script_list(&self) -> Handle<WeakArrayList> {
        Handle{value_: WeakArrayList{}}
    }
}
pub struct V8Assembler<'a>{
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> V8Assembler<'a> {
    pub fn new() -> Self {
        V8Assembler {
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct CanDeoptimize{}
pub struct TSCallDescriptor{}

pub struct V<T>(T);

impl<T> V<T> {
    pub fn new(t: T) -> Self {
        V(t)
    }
}

pub enum Word32 {}
pub enum HeapObject {}
pub struct StoreOp {}
impl StoreOp{
    pub enum Kind{}
}
pub enum MemoryRepresentation{}
pub enum WriteBarrierKind{}

pub struct Operation{}
pub struct Debug{}
pub struct Heap{}

pub struct Zone{}
pub struct InstructionSequence{}
pub enum ArchOpcode{}
pub enum AtomicMemoryOrder{}
pub enum BrokerMode{}
pub struct ZoneSnapshot{}
pub struct InstructionOperand{}

}  // namespace internal
}  // namespace v8

mod base {
    pub struct ElapsedTimer {
        start_time: std::time::Instant,
    }

    impl ElapsedTimer {
        pub fn new() -> Self {
            ElapsedTimer {
                start_time: std::time::Instant::now(),
            }
        }

        pub fn start(&mut self) {
            self.start_time = std::time::Instant::now();
        }

        pub fn elapsed(&self) -> std::time::Duration {
            self.start_time.elapsed()
        }
    }
}

static mut v8_flags: V8Flags = V8Flags {
    profile_deserialization: false,
    verify_snapshot_checksum: false,
    log_function_events: false,
    stress_background_compile: false,
    concurrent_sparkplug: false,
    baseline_batch_compilation: false,
};

#[derive(Clone, Copy)]
struct V8Flags {
    profile_deserialization: bool,
    verify_snapshot_checksum: bool,
    log_function_events: bool,
    stress_background_compile: bool,
    concurrent_sparkplug: bool,
    baseline_batch_compilation: bool,
}
