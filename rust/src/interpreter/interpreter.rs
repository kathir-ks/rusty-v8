// src/interpreter/interpreter.rs

use std::collections::HashMap;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

// Placeholder types and functions.  These would need to be defined
// elsewhere to provide complete functionality.
type Address = usize; // Or a more appropriate address type
const kNullAddress: Address = 0;

// Builtins-generated/bytecodes-builtins-list.h
//This is a placeholder, real implementation will have a macro like OPERAND_SCALE_LIST and BYTECODE_LIST to define enums
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OperandScale {
    kSingle,
    kDouble,
    kQuadruple,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Bytecode {
    kLdaZero, // Example
    kLdaSmi, // Example
    kStar, // Example
    kFirstShortStar,
    kLastShortStar,
    kLast,
}

mod bytecodes {
    use super::{Bytecode, OperandScale};
    pub const kShortStarCount: i32 = 1;
    pub const kBytecodeCount: i32 = 2; // Placeholder value

    pub fn is_short_star(bytecode: Bytecode) -> bool {
        bytecode == Bytecode::kStar
    }
    
    pub fn from_byte(index: i32) -> Bytecode {
        match index {
            0 => Bytecode::kLdaZero,
            1 => Bytecode::kLdaSmi,
            _ => Bytecode::kLdaZero
        }
    }

    pub fn to_byte(bytecode: Bytecode) -> i32 {
        match bytecode {
            Bytecode::kLdaZero => 0,
            Bytecode::kLdaSmi => 1,
            _ => 0
        }
    }

    pub fn to_string(bytecode: Bytecode) -> String {
        format!("{:?}", bytecode)
    }

    pub fn to_string_with_scale(bytecode: Bytecode, operand_scale: OperandScale, additional_info: &str) -> String {
        format!("{:?} {:?} {}", bytecode, operand_scale, additional_info)
    }

    pub fn bytecode_has_handler(bytecode: Bytecode, operand_scale: OperandScale) -> bool {
        true
    }

    pub fn from_byte_test(index: i32) -> Bytecode {
        match index {
            0 => Bytecode::kLdaZero,
            1 => Bytecode::kLdaSmi,
            _ => Bytecode::kLdaZero
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Builtin {
    kIllegalHandler,
    kFirstBytecodeHandler,
    kLastBytecodeHandler,
}

impl Builtin {
    fn from_int(index: i32) -> Self {
        match index {
            0 => Builtin::kIllegalHandler,
            1 => Builtin::kFirstBytecodeHandler,
            _ => Builtin::kFirstBytecodeHandler, // Replace with proper mapping
        }
    }
}

mod builtins {
    use super::Builtin;
    #[derive(Debug)]
    pub struct Builtins {}

    impl Builtins {
        pub fn code(&self, builtin: Builtin) -> TaggedCode {
            TaggedCode{}
        }

        pub fn is_initialized(&self) -> bool {
            true
        }

        pub fn name(builtin: Builtin) -> &'static str {
            match builtin {
                Builtin::kIllegalHandler => "IllegalHandler",
                Builtin::kFirstBytecodeHandler => "FirstBytecodeHandler",
                _ => "UnknownBuiltin",
            }
        }
    }
}

// src/ast/prettyprinter.h
struct AstPrinter {} // Placeholder

impl AstPrinter {
    fn new(stack_limit: usize) -> Self {
        AstPrinter{}
    }
}

// src/ast/scopes.h
struct DeclarationScope {} // Placeholder

impl DeclarationScope {
    fn allocate_scope_infos(parse_info: &ParseInfo, script: &Script, isolate: &Isolate) {
        // Placeholder
    }
}

// src/codegen/compiler.h
struct Compiler {} // Placeholder

// src/codegen/unoptimized-compilation-info.h
struct UnoptimizedCompilationInfo<'a> {
    bytecode_array_: Option<Handle<BytecodeArray>>,
    zone_: &'a Zone,
    literal_: FunctionLiteral,
    source_position_recording_mode_: SourcePositionTableBuilder::RecordingMode,
    parse_info_: &'a ParseInfo,
}

impl <'a> UnoptimizedCompilationInfo<'a> {
    fn new(zone: &'a Zone, parse_info: &'a ParseInfo, literal: FunctionLiteral) -> Self {
        UnoptimizedCompilationInfo {
            bytecode_array_: None,
            zone_: zone,
            literal_: literal,
            source_position_recording_mode_: SourcePositionTableBuilder::RecordingMode::NO_SOURCE_POSITIONS,
            parse_info_: parse_info,
        }
    }
    fn bytecode_array(&self) -> &Option<Handle<BytecodeArray>> {
        &self.bytecode_array_
    }

    fn set_bytecode_array(&mut self, bytecodes: Handle<BytecodeArray>) {
        self.bytecode_array_ = Some(bytecodes);
    }

    fn source_position_recording_mode(&self) -> SourcePositionTableBuilder::RecordingMode {
        self.source_position_recording_mode_
    }

    fn literal(&self) -> &FunctionLiteral {
        &self.literal_
    }
}

// src/common/globals.h
// Placeholder for global flags
mod v8_flags {
    pub static mut print_ast: bool = false;
    pub static mut print_bytecode: bool = false;
    pub static mut print_bytecode_filter: &str = "";
}

// src/execution/local-isolate.h
struct LocalIsolate {} // Placeholder

impl LocalIsolate {
    fn park_if_on_background_and_execute<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        f(); // Just execute for now
    }
}

// src/heap/parked-scope.h
struct ParkedScope {} // Placeholder

// src/init/setup-isolate.h
struct SetupIsolate {} // Placeholder

// src/interpreter/bytecode-generator.h
struct BytecodeGenerator<'a> {
    local_isolate_: *mut LocalIsolate,
    zone_: &'a Zone,
    compilation_info_: *mut UnoptimizedCompilationInfo<'a>,
    stack_overflow_: bool,
}

impl <'a> BytecodeGenerator<'a> {
    fn new(local_isolate: *mut LocalIsolate, zone: &'a Zone, compilation_info: *mut UnoptimizedCompilationInfo<'a>, ast_string_constants: (), eager_inner_literals: *mut Vec<FunctionLiteral>, script: Handle<Script>) -> Self {
        BytecodeGenerator {
            local_isolate_: local_isolate,
            zone_: zone,
            compilation_info_: compilation_info,
            stack_overflow_: false,
        }
    }
    fn generate_bytecode(&mut self, stack_limit: usize) {
        // Placeholder implementation
    }

    fn finalize_bytecode(&mut self, isolate: &Isolate, script: Handle<Script>) -> Handle<BytecodeArray> {
        // Placeholder implementation
        Handle::new(BytecodeArray{})
    }

    fn finalize_source_position_table(&self, isolate: &Isolate) -> Handle<TrustedByteArray> {
        // Placeholder implementation
        Handle::new(TrustedByteArray{})
    }

    fn has_stack_overflow(&self) -> bool {
        self.stack_overflow_
    }
}

// src/logging/runtime-call-stats-scope.h
struct RuntimeCallStatsScope {} // Placeholder

// src/objects/objects-inl.h
struct JSObject {} // Placeholder

impl JSObject {
    fn add_property(isolate: &Isolate, object: Handle<JSObject>, name: String, value: Handle<Object>, attribute: i32) {
        // Placeholder implementation
    }
}

// src/objects/shared-function-info.h
struct SharedFunctionInfo {} // Placeholder

impl SharedFunctionInfo {
    fn is_toplevel(&self) -> bool {
        false
    }

    fn passes_filter(&self, filter: &str) -> bool {
        true
    }

    fn script(&self) -> *mut Script {
        // Placeholder
        ptr::null_mut()
    }
}

// src/parsing/parse-info.h
struct ParseInfo {
    stack_limit_: usize,
    ast_value_factory_: AstValueFactory,
} // Placeholder

impl ParseInfo {
    fn new(stack_limit: usize, ast_value_factory: AstValueFactory) -> Self {
        ParseInfo { stack_limit_: stack_limit, ast_value_factory_: ast_value_factory }
    }
    fn runtime_call_stats(&self) -> i32 {
        0 // Placeholder
    }

    fn stack_limit(&self) -> usize {
        self.stack_limit_
    }

    fn ast_value_factory(&self) -> &AstValueFactory {
        &self.ast_value_factory_
    }
}

// src/utils/ostreams.h
struct StdoutStream {} // Placeholder

impl StdoutStream {
    fn os_stream(&self) -> &Self {
        self
    }
}

// Additional required structs and enums, mimicking V8's codebase
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CodeKind {
    BYTECODE_HANDLER,
}

#[derive(Debug)]
struct TaggedCode {} // Placeholder

impl TaggedCode {
    fn has_instruction_stream(&self) -> bool {
        false
    }
    fn kind(&self) -> CodeKind {
        CodeKind::BYTECODE_HANDLER
    }
    fn instruction_start(&self) -> Address {
        0
    }
}

struct Isolate {
    builtins_: Builtins,
    factory_: Factory,
} // Placeholder

impl Isolate {
    fn builtins(&self) -> &Builtins {
        &self.builtins_
    }

    fn factory(&self) -> &Factory {
        &self.factory_
    }
}

struct Factory {} // Placeholder

impl Factory {
    fn new_js_object_with_null_proto(&self) -> Handle<JSObject> {
        Handle::new(JSObject{}) // Placeholder
    }

    fn new_number_from_size(&self, size: usize) -> Handle<Object> {
        Handle::new(Object{}) // Placeholder
    }
}

struct Script {} // Placeholder

impl Script {
    fn get_name_or_source_url(&self) -> TaggedObject {
        TaggedObject{}
    }
}

struct TaggedObject{}

impl TaggedObject {
    
}

struct FunctionLiteral {} // Placeholder

impl FunctionLiteral {
    fn get_debug_name(&self) -> Box<[u8]> {
        "placeholder".as_bytes().to_vec().into_boxed_slice() // Placeholder
    }

    fn get_name(&self, isolate: &Isolate) -> MaybeDirectHandle<String> {
        MaybeDirectHandle::Empty
    }

    fn start_position(&self) -> usize {
        0
    }
    fn shared_function_info(&self) -> *mut SharedFunctionInfo {
        ptr::null_mut()
    }
    fn set_shared_function_info(&self, indirect_handle: IndirectHandle<SharedFunctionInfo>) {
        // Placeholder implementation
    }
}

struct BytecodeArray {} // Placeholder

impl BytecodeArray {
    fn length(&self) -> usize {
        0 // Placeholder
    }

    fn disassemble(&self, os: StdoutStream) {
        // Placeholder implementation
    }

    fn set_source_position_table(&self, source_position_table: TrustedByteArray, kReleaseStore: i32) {
        
    }
}

struct TrustedByteArray {} // Placeholder

#[derive(Debug)]
struct Handle<T> {
    _ptr: Box<T>,
} // Placeholder

impl <T> Handle<T> {
    fn new(obj: T) -> Self {
        Handle{ _ptr: Box::new(obj)}
    }
}

#[derive(Debug)]
struct DirectHandle<T> {
    _ptr: Box<T>,
} // Placeholder

impl <T> DirectHandle<T> {
    fn new(obj: T) -> Self {
        DirectHandle{ _ptr: Box::new(obj)}
    }
}

#[derive(Debug)]
struct MaybeDirectHandle<T> {
    _ptr: Option<Box<T>>,
}

impl <T> MaybeDirectHandle<T> {
    const Empty: MaybeDirectHandle<T> = MaybeDirectHandle { _ptr: None };

    fn to_handle(&self, handle: &mut DirectHandle<T>) -> bool {
        match &self._ptr {
            Some(boxed) => {
                *handle = DirectHandle::new(unsafe { ptr::read(boxed.as_ref()) });
                true
            }
            None => false,
        }
    }
}

struct IndirectHandle<T> {
    _ptr: Box<T>,
}

impl <T> IndirectHandle<T> {
    
}

fn indirect_handle<T>(shared_info: DirectHandle<T>, isolate: &Isolate) -> IndirectHandle<T> {
    IndirectHandle{ _ptr: shared_info._ptr }
}

struct Object {} // Placeholder

struct AstValueFactory {}

impl AstValueFactory {
    fn internalize(&self, isolate: &Isolate) {

    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RecordingMode {
    NO_SOURCE_POSITIONS,
    RECORD_SOURCE_POSITIONS
}

mod source_position_table_builder {
    use super::RecordingMode;
    pub struct SourcePositionTableBuilder {}
    impl SourcePositionTableBuilder {
        pub enum RecordingMode {
            NO_SOURCE_POSITIONS,
            RECORD_SOURCE_POSITIONS
        }
    }
}

use source_position_table_builder::SourcePositionTableBuilder;

struct Zone {
    name: String,
} // Placeholder

impl Zone {
    fn new(name: String) -> Self {
        Zone { name }
    }
}

struct AccountingAllocator {} // Placeholder

impl AccountingAllocator {
    fn new() -> Self {
        AccountingAllocator {}
    }
}

//START OF InterpreterCompilationJob

struct InterpreterCompilationJob<'a> {
    compilation_job: UnoptimizedCompilationJob,
    zone_: Zone,
    compilation_info_: UnoptimizedCompilationInfo<'a>,
    local_isolate_: *mut LocalIsolate,
    generator_: BytecodeGenerator<'a>,
}

impl<'a> InterpreterCompilationJob<'a> {
    fn new(
        parse_info: &'a ParseInfo,
        literal: FunctionLiteral,
        script: Handle<Script>,
        allocator: &AccountingAllocator,
        eager_inner_literals: *mut Vec<FunctionLiteral>,
        local_isolate: *mut LocalIsolate,
    ) -> Self {
        let zone_ = Zone::new(String::from("ZONE_NAME"));
        let compilation_info_ = UnoptimizedCompilationInfo::new(&zone_, parse_info, literal);
        let compilation_job = UnoptimizedCompilationJob::new(parse_info.stack_limit(), parse_info, &compilation_info_);
        let generator_ = BytecodeGenerator::new(local_isolate, &zone_, &compilation_info_ as *const _ as *mut _, (), eager_inner_literals, script);

        InterpreterCompilationJob {
            compilation_job: compilation_job,
            zone_: zone_,
            compilation_info_: compilation_info_,
            local_isolate_: local_isolate,
            generator_: generator_,
        }
    }

    fn execute_job_impl(&mut self) -> Result<(), String> {
        // RCS_SCOPE, TRACE_EVENT0, MaybePrintAst implementations...

        unsafe {
            (*self.local_isolate_).park_if_on_background_and_execute(|| {
                self.generator_.generate_bytecode(self.compilation_job.stack_limit);
            });
        }

        if self.generator_.has_stack_overflow() {
            return Err("Stack overflow during bytecode generation".to_string());
        }
        Ok(())
    }

    fn finalize_job_impl(&mut self, shared_info: DirectHandle<SharedFunctionInfo>, isolate: &Isolate) -> Result<(), String> {
        // RCS_SCOPE, TRACE_EVENT0 implementations...
        self.do_finalize_job_impl(shared_info, isolate)
    }

    fn finalize_job_impl_local(&mut self, shared_info: DirectHandle<SharedFunctionInfo>, isolate: *mut LocalIsolate) -> Result<(), String> {
        // RCS_SCOPE, TRACE_EVENT0 implementations...
        self.do_finalize_job_impl(shared_info, unsafe { &*(isolate as *const Isolate) })
    }

    fn do_finalize_job_impl<IsolateT>(&mut self, shared_info: DirectHandle<SharedFunctionInfo>, isolate: &Isolate) -> Result<(), String> {
        let mut bytecodes = self.compilation_info_.bytecode_array().cloned();
        if bytecodes.is_none() {
            bytecodes = Some(unsafe { self.generator_.finalize_bytecode(isolate, Handle::new(Script{})) });

            if self.generator_.has_stack_overflow() {
                return Err("Stack overflow during bytecode finalization".to_string());
            }
            self.compilation_info_.set_bytecode_array(bytecodes.clone().unwrap());
        }

        if self.compilation_info_.source_position_recording_mode() ==
            SourcePositionTableBuilder::RecordingMode::RECORD_SOURCE_POSITIONS {
            let source_position_table = self.generator_.finalize_source_position_table(isolate);
            // bytecodes.set_source_position_table(*source_position_table, 0); // Assuming 0 is the equivalent of kReleaseStore
        }

        // ShouldPrintBytecode, CheckAndPrintBytecodeMismatch implementations...

        Ok(())
    }
}

struct UnoptimizedCompilationJob {
    stack_limit: usize,
    parse_info: *const ParseInfo,
    compilation_info: *const UnoptimizedCompilationInfo<'static>,
}

impl UnoptimizedCompilationJob {
    fn new(stack_limit: usize, parse_info: *const ParseInfo, compilation_info: *const UnoptimizedCompilationInfo<'static>) -> Self {
        UnoptimizedCompilationJob {
            stack_limit: stack_limit,
            parse_info: parse_info,
            compilation_info: compilation_info,
        }
    }
}

//END OF InterpreterCompilationJob

// Dispatch table constants.
const kBitsPerByte: usize = 8;
const kNumberOfBytecodeHandlers: usize = 1;
const kNumberOfWideBytecodeHandlers: usize = 1;
const kIllegalBytecodeHandlerEncoding: u8 = 0xFF;

static kWideBytecodeToBuiltinsMapping: [u8; 2] = [kIllegalBytecodeHandlerEncoding, kIllegalBytecodeHandlerEncoding];
const kNumberOfBytecodes: usize = 2;
const V8_IGNITION_DISPATCH_COUNTING_BOOL: bool = false;

/// The interpreter implementation.
pub struct Interpreter {
    isolate_: *mut Isolate,
    interpreter_entry_trampoline_instruction_start_: Address,
    dispatch_table_: [Address; 256 * 3], // Assuming OperandScale has 3 values
    bytecode_dispatch_counters_table_: Option<Box<[usize]>>,
}

impl Interpreter {
    /// Creates a new Interpreter instance.
    pub fn new(isolate: *mut Isolate) -> Self {
        Interpreter {
            isolate_: isolate,
            interpreter_entry_trampoline_instruction_start_: kNullAddress,
            dispatch_table_: [0; 256 * 3],
            bytecode_dispatch_counters_table_: if V8_IGNITION_DISPATCH_COUNTING_BOOL {
                Some(Self::init_dispatch_counters())
            } else {
                None
            },
        }
    }

    fn init_dispatch_counters() -> Box<[usize]> {
        let k_bytecode_count = bytecodes::kBytecodeCount as usize;
        let table_size = k_bytecode_count * k_bytecode_count;
        vec![0; table_size].into_boxed_slice()
    }

    /// Returns the bytecode handler for the given bytecode and operand scale.
    pub fn get_bytecode_handler(&self, bytecode: Bytecode, operand_scale: OperandScale) -> TaggedCode {
        let builtin = Self::builtin_index_from_bytecode(bytecode, operand_scale);
        unsafe { (*self.isolate_).builtins().code(builtin) }
    }

    /// Sets the bytecode handler for the given bytecode and operand scale.
    pub fn set_bytecode_handler(&mut self, bytecode: Bytecode, operand_scale: OperandScale, handler: TaggedCode) {
        assert!(!handler.has_instruction_stream());
        assert_eq!(handler.kind(), CodeKind::BYTECODE_HANDLER);
        let index = Self::get_dispatch_table_index(bytecode, operand_scale);
        self.dispatch_table_[index] = handler.instruction_start();
    }

    /// Returns the dispatch table index for the given bytecode and operand scale.
    pub fn get_dispatch_table_index(bytecode: Bytecode, operand_scale: OperandScale) -> usize {
        const K_ENTRIES_PER_OPERAND_SCALE: usize = 1 << kBitsPerByte;
        let index = bytecodes::to_byte(bytecode) as usize;
        index + Self::operand_scale_as_index(operand_scale) * K_ENTRIES_PER_OPERAND_SCALE
    }

    fn operand_scale_as_index(operand_scale: OperandScale) -> usize {
        match operand_scale {
            OperandScale::kSingle => 0,
            OperandScale::kDouble => 1,
            OperandScale::kQuadruple => 2,
        }
    }

    fn builtin_index_from_bytecode(bytecode: Bytecode, operand_scale: OperandScale) -> Builtin {
        let mut index = bytecodes::to_byte(bytecode) as usize;

        if operand_scale == OperandScale::kSingle {
            if bytecodes::is_short_star(bytecode) {
                index = bytecodes::to_byte(Bytecode::kFirstShortStar) as usize;
            } else if bytecodes::to_byte(bytecode) > bytecodes::to_byte(Bytecode::kLastShortStar) {
                index -= bytecodes::kShortStarCount as usize - 1;
            }
        } else {
            let offset = kWideBytecodeToBuiltinsMapping[index];
            if offset == kIllegalBytecodeHandlerEncoding {
                return Builtin::kIllegalHandler;
            } else {
                index = kNumberOfBytecodeHandlers + offset as usize;
                if operand_scale == OperandScale::kQuadruple {
                    index += kNumberOfWideBytecodeHandlers;
                }
            }
        }

        Builtin::from_int(Builtin::kFirstBytecodeHandler as i32 + index as i32)
    }

    /// Creates a new compilation job.
    pub fn new_compilation_job(
        parse_info: &ParseInfo,
        literal: FunctionLiteral,
        script: Handle<Script>,
        allocator: &AccountingAllocator,
        eager_inner_literals: *mut Vec<FunctionLiteral>,
        local_isolate: *mut LocalIsolate,
    ) -> Box<InterpreterCompilationJob<'static>> {
        Box::new(InterpreterCompilationJob::new(parse_info, literal, script, allocator, eager_inner_literals, local_isolate))
    }

    pub fn new_source_position_collection_job(
        parse_info: &ParseInfo,
        literal: FunctionLiteral,
        existing_bytecode: Handle<BytecodeArray>,
        allocator: &AccountingAllocator,
        local_isolate: *mut LocalIsolate,
    ) -> Box<InterpreterCompilationJob<'static>> {
        let mut job = InterpreterCompilationJob::new(parse_info, literal, Handle::new(Script{}), allocator, ptr::null_mut(), local_isolate);
        job.compilation_info_.set_bytecode_array(existing_bytecode);
        Box::new(job)
    }

    /// Executes a function for each bytecode and operand scale.
    pub fn for_each_bytecode<F>(&self, f: F)
    where
        F: Fn(Bytecode, OperandScale),
    {
        const OPERAND_SCALES: [OperandScale; 3] = [
            OperandScale::kSingle,
            OperandScale::kDouble,
            OperandScale::kQuadruple,
        ];

        for operand_scale in OPERAND_SCALES.iter() {
            for i in 0..bytecodes::kBytecodeCount {
                f(bytecodes::from_byte_test(i), *operand_scale);
            }
        }
    }

    /// Initializes the interpreter.
    pub fn initialize(&mut self) {
        let builtins = unsafe { (*self.isolate_).builtins() };

        // Set the interpreter entry trampoline entry point now that builtins are
        // initialized.
        // let code = builtins.code(Builtin::InterpreterEntryTrampoline); // Assuming Builtin has an InterpreterEntryTrampoline variant
        let code = TaggedCode{};
        assert!(builtins.is_initialized());
        assert!(!code.has_instruction_stream());
        self.interpreter_entry_trampoline_instruction_start_ = code.instruction_start();

        // Initialize the dispatch table.
        self.for_each_bytecode(|bytecode, operand_scale| {
            let builtin = Self::builtin_index_from_bytecode(bytecode, operand_scale);
            let handler = builtins.code(builtin);

            if bytecodes::bytecode_has_handler(bytecode, operand_scale) {
                // Debug checks...
            }

            self.set_bytecode_handler(bytecode, operand_scale, handler);
        });
        assert!(self.is_dispatch_table_initialized());
    }

    /// Returns whether the dispatch table is initialized.
    pub fn is_dispatch_table_initialized(&self) -> bool {
        self.dispatch_table_[0] != kNullAddress
    }

    pub fn get_dispatch_counter(&self, from: Bytecode, to: Bytecode) -> usize {
        let from_index = bytecodes::to_byte(from) as usize;
        let to_index = bytecodes::to_byte(to) as usize;
        assert!(self.bytecode_dispatch_counters_table_.is_some(), "Dispatch counters require building with v8_enable_ignition_dispatch_counting");

        self.bytecode_dispatch_counters_table_.as_ref().unwrap()[from_index * kNumberOfBytecodes + to_index]
    }

    pub fn get_dispatch_counters_object(&self) -> Handle<JSObject> {
        let counters_map = unsafe { (*self.isolate_).factory().new_js_object_with_null_proto() };

        for from_index in 0..kNumberOfBytecodes {
            let from_bytecode = bytecodes::from_byte(from_index as i32);
            let counters_row = unsafe { (*self.isolate_).factory().new_js_object_with_null_proto() };

            for to_index in 0..kNumberOfBytecodes {
                let to_bytecode = bytecodes::from_byte(to_index as i32);
                let counter = self.get_dispatch_counter(from_bytecode, to_bytecode);

                if counter > 0 {
                    let value = unsafe { (*self.isolate_).factory().new_number_from_size(counter) };
                    JSObject::add_property(unsafe { &*self.isolate_ }, counters_row, bytecodes::to_string(to_bytecode), value, 0);
                }
            }

            JSObject::add_property(unsafe { &*self.isolate_ }, counters_map, bytecodes::to_string(from_bytecode), counters_row, 0);
        }

        counters_map
    }
}