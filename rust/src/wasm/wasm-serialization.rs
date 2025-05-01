// src/wasm/wasm-serialization.rs

//use std::arch::asm;
//use std::arch::x86_64::*;
use std::cmp;
use std::collections::HashMap;
use std::convert::TryInto;
use std::mem;
use std::ptr;
use std::sync::{Arc, Mutex};

//use byteorder::{ByteOrder, LittleEndian};

//use crate::base::vector::{Vector, VectorOf};
//use crate::codegen::assembler_arch::Assembler;
//use crate::codegen::assembler_inl::Assembler;
//use crate::debug::debug::Debug;
//use crate::runtime::runtime::Runtime;
//use crate::snapshot::snapshot_data::SerializedData;
//use crate::utils::ostreams::StdoutStream;
//use crate::utils::version::Version;
//use crate::wasm::code_space_access::CodeSpaceAccess;
//use crate::wasm::function_compiler::FunctionCompiler;
//use crate::wasm::module_compiler::ModuleCompiler;
//use crate::wasm::module_decoder::{DecodeWasmModule, ModuleResult};
//use crate::wasm::wasm_code_manager::WasmCodeManager;
//use crate::wasm::wasm_engine::{GetWasmEngine, WasmEngine};
//use crate::wasm::wasm_module::WasmModule;
//use crate::wasm::wasm_objects_inl::WasmModuleObject;
//use crate::wasm::wasm_objects::WasmCode;
//use crate::wasm::wasm_result::WasmResult;
//use crate::wasm::well_known_imports::WellKnownImport;
//use crate::wasm::enabled_features::WasmEnabledFeatures;
//use crate::flag_list::FlagList;
//use crate::cpu_features::CpuFeatures;
//use crate::wasm::compilation_environment::CompilationEnviornment;
//use crate::wasm::wasm_objects::ExecutionTier;
//use crate::wasm::compilation_state::CompilationState;
//use crate::base::owned_vector::OwnedVector;

const K_LAZY_FUNCTION: u8 = 2;
const K_EAGER_FUNCTION: u8 = 3;
const K_TURBO_FAN_FUNCTION: u8 = 4;

// TODO: Reimplement trace_wasm_serialization using tracing or logging crate.
const TRACE_WASM_SERIALIZATION: bool = false;

struct Writer<'a> {
    start: *mut u8,
    end: *mut u8,
    pos: *mut u8,
    buffer: &'a mut [u8],
}

impl<'a> Writer<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        let start = buffer.as_mut_ptr();
        let end = unsafe { start.add(buffer.len()) };
        Writer {
            start,
            end,
            pos: start,
            buffer,
        }
    }

    fn bytes_written(&self) -> usize {
        unsafe { self.pos.offset_from(self.start) as usize }
    }

    fn current_location(&self) -> *mut u8 {
        self.pos
    }

    fn current_size(&self) -> usize {
        unsafe { self.end.offset_from(self.pos) as usize }
    }

    fn current_buffer(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.pos, self.current_size()) }
    }

    fn write<T>(&mut self, value: T) {
        let size = std::mem::size_of::<T>();
        assert!(self.current_size() >= size);
        unsafe {
            std::ptr::write_unaligned(self.pos as *mut T, value);
            self.pos = self.pos.add(size);
        }

        if TRACE_WASM_SERIALIZATION {
            println!("wrote: {} sized: {}", value as usize, size);
        }
    }

    fn write_vector(&mut self, v: &[u8]) {
        assert!(self.current_size() >= v.len());
        if !v.is_empty() {
            unsafe {
                std::ptr::copy_nonoverlapping(v.as_ptr(), self.pos, v.len());
                self.pos = self.pos.add(v.len());
            }
        }

        if TRACE_WASM_SERIALIZATION {
            println!(
                "wrote vector of {} elements (total size {} bytes)",
                v.len(),
                v.len()
            );
        }
    }

    fn skip(&mut self, size: usize) {
        self.pos = unsafe { self.pos.add(size) };
    }
}

struct Reader<'a> {
    start: *const u8,
    end: *const u8,
    pos: *const u8,
    buffer: &'a [u8],
}

impl<'a> Reader<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        let start = buffer.as_ptr();
        let end = unsafe { start.add(buffer.len()) };
        Reader {
            start,
            end,
            pos: start,
            buffer,
        }
    }

    fn bytes_read(&self) -> usize {
        unsafe { self.pos.offset_from(self.start) as usize }
    }

    fn current_location(&self) -> *const u8 {
        self.pos
    }

    fn current_size(&self) -> usize {
        unsafe { self.end.offset_from(self.pos) as usize }
    }

    fn current_buffer(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.pos, self.current_size()) }
    }

    fn read<T: Copy>(&mut self) -> T {
        let size = std::mem::size_of::<T>();
        assert!(self.current_size() >= size);
        let value = unsafe { std::ptr::read_unaligned(self.pos as *const T) };
        self.pos = unsafe { self.pos.add(size) };

        if TRACE_WASM_SERIALIZATION {
            println!("read: {} sized: {}", value as usize, size);
        }

        value
    }

    fn read_vector<T: Copy>(&mut self, size: usize) -> &[T] {
        let byte_size = size * std::mem::size_of::<T>();
        assert!(self.current_size() >= byte_size);

        let bytes = unsafe { std::slice::from_raw_parts(self.pos as *const T, size) };
        self.pos = unsafe { self.pos.add(byte_size) };

        if TRACE_WASM_SERIALIZATION {
            println!(
                "read vector of {} elements of size {} (total size {} bytes)",
                size,
                std::mem::size_of::<T>(),
                byte_size
            );
        }
        bytes
    }

    fn skip(&mut self, size: usize) {
        self.pos = unsafe { self.pos.add(size) };
    }
}

fn write_header(writer: &mut Writer, enabled_features: u32) {
    assert_eq!(0, writer.bytes_written());
    writer.write(0xDEADBEEF_u32); //SerializedData::kMagicNumber); // Placeholder
    writer.write(0xCAFEBABE_u32); //Version::Hash()); // Placeholder
    writer.write(0x12345678_u32); //CpuFeatures::SupportedFeatures()); // Placeholder
    writer.write(0x87654321_u32); //FlagList::Hash()); // Placeholder
    writer.write(enabled_features); //enabled_features.ToIntegral());
    assert_eq(WASM_SERIALIZER_HEADER_SIZE, writer.bytes_written());
}

// This function cannot be directly translated because it depends on V8 internal
// reloc info details and target architecture specific assembly instruction manipulation.
// TODO: Reimplement the functionality based on Rust's memory manipulation and
// architecture-specific assembly instruction generation capabilities.
fn _set_wasm_callee_tag(_rinfo: usize, _tag: u32) {
    // Placeholder implementation
    println!("set_wasm_callee_tag");
}

// This function cannot be directly translated because it depends on V8 internal
// reloc info details and target architecture specific assembly instruction manipulation.
// TODO: Reimplement the functionality based on Rust's memory manipulation and
// architecture-specific assembly instruction generation capabilities.
fn _get_wasm_callee_tag(_rinfo: usize) -> u32 {
    // Placeholder implementation
    println!("get_wasm_callee_tag");
    0
}

const WASM_CODE_HEADER_SIZE: usize = std::mem::size_of::<u8>() // code kind
    + std::mem::size_of::<i32>() // offset of constant pool
    + std::mem::size_of::<i32>() // offset of safepoint table
    + std::mem::size_of::<i32>() // offset of handler table
    + std::mem::size_of::<i32>() // offset of code comments
    + std::mem::size_of::<i32>() // unpadded binary size
    + std::mem::size_of::<i32>() // stack slots
    + std::mem::size_of::<i32>() // ool slots
    + std::mem::size_of::<i32>() // tagged parameter slots
    + std::mem::size_of::<i32>() // code size
    + std::mem::size_of::<i32>() // reloc size
    + std::mem::size_of::<i32>() // source positions size
    + std::mem::size_of::<i32>() // inlining positions size
    + std::mem::size_of::<i32>() // deopt data size
    + std::mem::size_of::<i32>() // protected instructions size
    + std::mem::size_of::<u32>() // WasmCode::Kind // fix this.
    + std::mem::size_of::<u32>(); // ExecutionTier; // fix this

// The below struct and it's functions cannot be directly translated due to dependency
// on V8 specific data structures.
// TODO: Implement replacement of the V8 specific structs with standard Rust
// implementations where applicable.
struct ExternalReferenceList {}

impl ExternalReferenceList {
    fn tag_from_address(&self, _ext_ref_address: usize) -> u32 {
        println!("ExternalReferenceList::tag_from_address");
        0
    }

    fn address_from_tag(&self, _tag: u32) -> usize {
        println!("ExternalReferenceList::address_from_tag");
        0
    }

    fn get() -> &'static ExternalReferenceList {
        println!("ExternalReferenceList::Get");
        static LIST: ExternalReferenceList = ExternalReferenceList {};
        &LIST
    }
}

const WASM_SERIALIZER_HEADER_SIZE: usize = 20;

struct NativeModuleSerializer<'a> {
    native_module: &'a NativeModule,
    code_table: Vec<&'a WasmCode>,
    import_statuses: Vec<u32>, // &'a [WellKnownImport],
    canonical_sig_ids_to_module_local_ids: HashMap<u32, u32>,
    write_called: bool,
    total_written_code: usize,
    num_turbofan_functions: i32,
}

impl<'a> NativeModuleSerializer<'a> {
    fn new(
        native_module: &'a NativeModule,
        code_table: Vec<&'a WasmCode>,
        import_statuses: Vec<u32>, // &'a [WellKnownImport],
    ) -> Self {
        NativeModuleSerializer {
            native_module,
            code_table,
            import_statuses,
            canonical_sig_ids_to_module_local_ids: HashMap::new(),
            write_called: false,
            total_written_code: 0,
            num_turbofan_functions: 0,
        }
    }

    fn measure_code(&self, code: &WasmCode) -> usize {
        println!("NativeModuleSerializer::measure_code");
        if code.instructions.is_empty() {
            return std::mem::size_of::<u8>();
        }
        //assert_eq!(WasmCode::kWasmFunction, code.kind());
        if code.tier != 3 { //ExecutionTier::kTurbofan
            return std::mem::size_of::<u8>();
        }

        WASM_CODE_HEADER_SIZE + code.instructions.len() + code.reloc_info.len() + code.source_positions.len()
            + code.inlining_positions.len()
            + code.protected_instructions_data.len()
            + code.deopt_data.len()
    }

    fn measure(&self) -> usize {
        println!("NativeModuleSerializer::measure");

        let mut size = std::mem::size_of::<u32>() // WasmDetectedFeatures::StorageType
            + std::mem::size_of::<usize>()  // total code size
            + std::mem::size_of::<bool>() // all functions validated
            + std::mem::size_of::<u32>() // CompileTimeImportFlags::StorageType
            + std::mem::size_of::<u32>() // length of constants_module.
            + self.native_module.compile_imports.constants_module.len()
            + self.import_statuses.len() * std::mem::size_of::<u32>(); // WellKnownImport

        for code in &self.code_table {
            size += self.measure_code(code);
        }

        size += self.native_module.module.num_declared_functions * std::mem::size_of::<u32>();

        size
    }

    fn write_header(&self, writer: &mut Writer, total_code_size: usize) {
        println!("NativeModuleSerializer::write_header");

        writer.write(self.native_module.compilation_state.detected_features); //ToIntegral());
        writer.write(total_code_size);

        let fully_validated = true; // !v8_flags.wasm_lazy_validation;
        writer.write(fully_validated);

        writer.write(self.native_module.compile_imports.flags); //ToIntegral());
        writer.write(self.native_module.compile_imports.constants_module.len() as u32);
        writer.write_vector(self.native_module.compile_imports.constants_module.as_bytes());
        writer.write_vector(&self.import_statuses.iter().map(|&x| x as u8).collect::<Vec<_>>()); //&self.import_statuses);
    }

    fn write_code(
        &mut self,
        code: &WasmCode,
        writer: &mut Writer,
        _function_index_map: &HashMap<u32, u32>,
    ) {
        println!("NativeModuleSerializer::write_code");

        if code.instructions.is_empty() {
            writer.write(K_LAZY_FUNCTION);
            return;
        }

        //assert_eq!(WasmCode::kWasmFunction, code.kind());
        if code.tier != 3 { //ExecutionTier::kTurbofan
                             //NativeModule* native_module = code->native_module();
            let budget = self.native_module.tiering_budget[0]; // declared_function_index( native_module->module(), code->index())].load(std::memory_order_relaxed);
                                                                //let eager = budget == static_cast::<u32>(40000.0 as u32);
            writer.write(K_LAZY_FUNCTION); // if eager { K_LAZY_FUNCTION } else { K_EAGER_FUNCTION }); // TODO: review this.
            return;
        }

        self.num_turbofan_functions += 1;
        writer.write(K_TURBO_FAN_FUNCTION);
        writer.write(code.constant_pool_offset);
        writer.write(code.safepoint_table_offset);
        writer.write(code.handler_table_offset);
        writer.write(code.code_comments_offset);
        writer.write(code.unpadded_binary_size);
        writer.write(code.stack_slots);
        writer.write(code.ool_spills);
        writer.write(code.tagged_parameter_slots); //raw_tagged_parameter_slots_for_serialization());
        writer.write(code.instructions.len() as i32);
        writer.write(code.reloc_info.len() as i32);
        writer.write(code.source_positions.len() as i32);
        writer.write(code.inlining_positions.len() as i32);
        writer.write(code.deopt_data.len() as i32);
        writer.write(code.protected_instructions_data.len() as i32);
        writer.write(code.kind);
        writer.write(code.tier);

        let serialized_code_start = writer.current_buffer().as_ptr() as *mut u8;
        let mut code_start = serialized_code_start;
        let code_size = code.instructions.len();
        writer.skip(code_size);

        writer.write_vector(&code.reloc_info);
        writer.write_vector(&code.source_positions);
        writer.write_vector(&code.inlining_positions);
        writer.write_vector(&code.deopt_data);
        writer.write_vector(&code.protected_instructions_data);

        unsafe {
            std::ptr::copy_nonoverlapping(code.instructions.as_ptr(), code_start, code_size);
        }

        // This block cannot be directly translated because it depends on V8 internal
        // reloc info details and target architecture specific assembly instruction manipulation.
        // TODO: Reimplement the functionality based on Rust's memory manipulation and
        // architecture-specific assembly instruction generation capabilities.
        let _k_mask = 0;
        // RelocIterator orig_iter(code->instructions(), code->reloc_info(),
        //                         code->constant_pool(), kMask);

        // WritableJitAllocation jit_allocation =
        //     WritableJitAllocation::ForNonExecutableMemory(
        //         reinterpret_cast<Address>(code_start), code->instructions().size(),
        //         ThreadIsolation::JitAllocationType::kWasmCode);

        // for (WritableRelocIterator iter(
        //          jit_allocation, {code_start, code->instructions().size()},
        //          code->reloc_info(),
        //          reinterpret_cast<Address>(code_start) + code->constant_pool_offset(),
        //          kMask);
        //      !iter.done(); iter.next(), orig_iter.next()) {
        //     RelocInfo::Mode mode = orig_iter.rinfo()->rmode();
        //     match (mode) {
        //         RelocInfo::WASM_CALL => {
        //             let orig_target = 0; //orig_iter.rinfo()->wasm_call_address();
        //             let tag = self.native_module.get_function_index_from_jump_table_slot(orig_target);
        //             _set_wasm_callee_tag(0, tag); //iter.rinfo(), tag);
        //         }
        //         RelocInfo::WASM_STUB_CALL => {
        //             //Address target = orig_iter.rinfo()->wasm_stub_call_address();
        //             //uint32_t tag = static_cast<uint32_t>(native_module_->GetBuiltinInJumptableSlot(target));
        //             //SetWasmCalleeTag(iter.rinfo(), tag);
        //         }
        //         RelocInfo::WASM_CANONICAL_SIG_ID => {
        //             //uint32_t canonical_sig_id = orig_iter.rinfo()->wasm_canonical_sig_id();
        //             //uint32_t module_local_sig_id = self.canonical_sig_id_to_module_local_type_id(canonical_sig_id);
        //             //iter.rinfo()->set_wasm_canonical_sig_id(module_local_sig_id);
        //         }
        //         RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY => {
        //             //WasmCodePointer target = orig_iter.rinfo()->wasm_code_pointer_table_entry();
        //             //uint32_t function_index = function_index_map.at(target);
        //             //iter.rinfo()->set_wasm_code_pointer_table_entry(
        //             //    WasmCodePointer{function_index}, SKIP_ICACHE_FLUSH);
        //         }
        //         RelocInfo::EXTERNAL_REFERENCE => {
        //             //Address orig_target = orig_iter.rinfo()->target_external_reference();
        //             //uint32_t ext_ref_tag = ExternalReferenceList::Get().tag_from_address(orig_target);
        //             //SetWasmCalleeTag(iter.rinfo(), ext_ref_tag);
        //         }
        //         RelocInfo::INTERNAL_REFERENCE => {}
        //         RelocInfo::INTERNAL_REFERENCE_ENCODED => {}
        //         _ => {
        //             println!("UNREACHABLE");
        //             //UNREACHABLE();
        //         }
        //     }
        // }

        self.total_written_code += code_size;
    }

    fn write_tiering_budget(&self, writer: &mut Writer) {
        println!("NativeModuleSerializer::write_tiering_budget");
        for i in 0..self.native_module.module.num_declared_functions {
            writer.write(self.native_module.tiering_budget[i]);
        }
    }

    fn canonical_sig_id_to_module_local_type_id(&mut self, canonical_sig_id: u32) -> u32 {
        println!("NativeModuleSerializer::canonical_sig_id_to_module_local_type_id");

        if self.canonical_sig_ids_to_module_local_ids.is_empty() {
            //const WasmModule* module = native_module_->module();
            //DCHECK_GE(kMaxUInt32, module->isorecursive_canonical_type_ids.size());
            let num_types = self.native_module.module.types.len(); //size();
                                                                      //DCHECK_EQ(num_types, module->isorecursive_canonical_type_ids.size());
            for local_id in 0..num_types as u32 {
                // Only add function signatures.
                //if (!module->has_signature(ModuleTypeIndex{local_id})) continue;
                //CanonicalTypeIndex canonical_id = module->canonical_sig_id(ModuleTypeIndex{local_id});
                // Try to emplace, skip if an entry exists already. It does not matter
                // which local type ID we use if multiple types got canonicalized to the
                // same ID.
                self.canonical_sig_ids_to_module_local_ids.insert(local_id, local_id); //std::make_pair(canonical_id.index, local_id));
            }
        }

        let it = self.canonical_sig_ids_to_module_local_ids.get(&canonical_sig_id).unwrap();
        return *it;
    }

    fn write(&mut self, buffer: &mut [u8]) -> bool {
        println!("NativeModuleSerializer::write");

        assert!(!self.write_called);
        self.write_called = true;

        let mut total_code_size = 0;
        for code in &self.code_table {
            if code.tier == 3 { // ExecutionTier::kTurbofan
                                 //DCHECK(IsAligned(code->instructions().size(), kCodeAlignment));
                total_code_size += code.instructions.len();
            }
        }

        let mut writer = Writer::new(buffer);
        self.write_header(&writer, total_code_size);

        //NativeModule::CallIndirectTargetMap function_index_map = native_module_->CreateIndirectCallTargetToFunctionIndexMap();
        let function_index_map = HashMap::new();
        for code in &self.code_table {
            self.write_code(code, &mut writer, &function_index_map);
        }

        // No TurboFan-compiled functions in jitless mode.
        //if (!v8_flags.wasm_jitless) {
        // If not a single function was written, serialization was not successful.
        if self.num_turbofan_functions == 0 {
            return false;
        }
        //}

        // Make sure that the serialized total code size was correct.
        assert_eq!(self.total_written_code, total_code_size);

        self.write_tiering_budget(&mut writer);
        return true;
    }
}

struct WasmSerializer<'a> {
    native_module: &'a NativeModule,
    code_table: Vec<&'a WasmCode>,
    import_statuses: Vec<u32>, //Vec<WellKnownImport>,
}

impl<'a> WasmSerializer<'a> {
    fn new(native_module: &'a NativeModule) -> Self {
        println!("WasmSerializer::new");

        let code_table: Vec<&WasmCode> = native_module.code_table.iter().collect();
        let import_statuses: Vec<u32> = native_module.import_statuses.iter().map(|&x| x as u32).collect();
        WasmSerializer {
            native_module,
            code_table,
            import_statuses,
        }
    }

    fn get_serialized_native_module_size(&self) -> usize {
        println!("WasmSerializer::get_serialized_native_module_size");

        let serializer = NativeModuleSerializer::new(
            self.native_module,
            self.code_table.clone(),
            self.import_statuses.clone(),
        );
        WASM_SERIALIZER_HEADER_SIZE + serializer.measure()
    }

    fn serialize_native_module(&self, buffer: &mut [u8]) -> bool {
        println!("WasmSerializer::serialize_native_module");

        let serializer = NativeModuleSerializer::new(
            self.native_module,
            self.code_table.clone(),
            self.import_statuses.clone(),
        );
        let measured_size = WASM_SERIALIZER_HEADER_SIZE + serializer.measure();
        if buffer.len() < measured_size {
            return false;
        }

        let enabled_features = self.native_module.compilation_state.detected_features; // TODO: Fix

        let mut writer = Writer::new(buffer);
        write_header(&mut writer, enabled_features);

        if !serializer.write(&mut writer) {
            return false;
        }
        assert_eq!(measured_size, writer.bytes_written());
        return true;
    }
}

struct DeserializationUnit {
    src_code_buffer: Vec<u8>,
    code: WasmCode,
    jump_tables: u32, //NativeModule::JumpTablesRef, //TODO: Fix this.
}

struct DeserializationQueue {
    queue: Arc<Mutex<Vec<Vec<DeserializationUnit>>>>,
}

impl DeserializationQueue {
    fn new() -> Self {
        DeserializationQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn add(&self, batch: Vec<DeserializationUnit>) {
        assert!(!batch.is_empty());
        let mut guard = self.queue.lock().unwrap();
        guard.push(batch);
    }

    fn pop(&self) -> Vec<DeserializationUnit> {
        let mut guard = self.queue.lock().unwrap();
        if guard.is_empty() {
            return Vec::new();
        }
        guard.remove(0)
    }

    fn pop_all(&self) -> Vec<DeserializationUnit> {
        let mut guard = self.queue.lock().unwrap();
        if guard.is_empty() {
            return Vec::new();
        }

        let mut units = guard.remove(0);

        while !guard.is_empty() {
            let mut front = guard.remove(0);
            units.append(&mut front);
        }
        units
    }

    fn num_batches(&self) -> usize {
        let guard = self.queue.lock().unwrap();
        guard.len()
    }
}

struct NativeModuleDeserializer<'a> {
    native_module: &'a mut NativeModule,
    remaining_code_size: usize,
    all_functions_validated: bool,
    compile_imports: CompileTimeImports,
    current_code_space: Vec<u8>,
    current_jump_tables: u32, // NativeModule::JumpTablesRef, //TODO: Fix this.
    lazy_functions: Vec<i32>,
    eager_functions: Vec<i32>,
    read_called: bool,
}

impl<'a> NativeModuleDeserializer<'a> {
    fn new(native_module: &'a mut NativeModule) -> Self {
        println!("NativeModuleDeserializer::new");
        NativeModuleDeserializer {
            native_module,
            remaining_code_size: 0,
            all_functions_validated: false,
            compile_imports: CompileTimeImports::default(),
            current_code_space: Vec::new(),
            current_jump_tables: 0, //NativeModule::JumpTablesRef::default(),
            lazy_functions: Vec::new(),
            eager_functions: Vec::new(),
            read_called: false,
        }
    }

    fn read(&mut self, buffer: &[u8]) -> bool {
        println!("NativeModuleDeserializer::read");

        assert!(!self.read_called);
        self.read_called = true;

        let mut reader = Reader::new(buffer);

        self.read_header(&mut reader);
        if self.compile_imports.compare(&self.native_module.compile_imports) != 0 {
            return false;
        }

        let total_fns = self.native_module.module.num_declared_functions;
        let first_wasm_fn = self.native_module.module.num_imported_functions;

        if self.all_functions_validated {
            //self.native_module.module.set_all_functions_validated();
        }

        let reloc_queue = DeserializationQueue::new();

        // Create a new job without any workers; those are spawned on
        // {NotifyConcurrencyIncrease}.
        //std::unique_ptr<JobHandle> job_handle = V8::GetCurrentPlatform()->CreateJob(
        //    TaskPriority::kUserVisible,
        //    std::make_unique<DeserializeCodeTask>(this, &reloc_queue));

        // Choose a batch size such that we do not create too small batches (>=100k
        // code bytes), but also not too many (<=100 batches).
        let _k_min_batch_size_in_bytes = 100000;
        let _batch_limit = self.remaining_code_size; // std::max(kMinBatchSizeInBytes, remaining_code_size_ / 100);

        let mut batch: Vec<DeserializationUnit> = Vec::new();
        //let mut batch_size = 0;
        for i in first_wasm_fn..total_fns {
            let unit = self.read_code(i as i32, &mut reader);
            if unit.code.instructions.is_empty() {
                continue;
            }
            //batch_size += unit.code.instructions.len();
            batch.push(unit);
            //if batch_size >= batch_limit {
            //    reloc_queue.add(batch); //std::move(batch));
            //    batch = Vec::new();
            //    //batch_size = 0;
            //    //job_handle->NotifyConcurrencyIncrease();
            //}
        }

        // We should have read the expected amount of code now, and should have fully
        // utilized the allocated code space.
        assert_eq!(0, self.remaining_code_size);
        assert_eq!(0, self.current_code_space.len());

        if !batch.is_empty() {
            reloc_queue.add(batch); //std::move(batch));
                                     //job_handle->NotifyConcurrencyIncrease();
        }

        // Wait for all tasks to finish, while participating in their work.
        //job_handle->Join();

        let all = reloc_queue.pop_all();
        for unit in all {
            self.copy_and_relocate(&unit);
            self.publish(vec![unit]);
        }

        self.read_tiering_budget(&mut reader);
        return reader.current_size() == 0;
    }

    fn read_header(&mut self, reader: &mut Reader) {
        println!("NativeModuleDeserializer::read_header");

        let detected_features = reader.read::<u32>();

        self.remaining_code_size = reader.read::<usize>();

        self.all_functions_validated = reader.read::<bool>();

        let compile_imports_flags = reader.read::<u32>();
        let constants_module_size = reader.read::<u32>();
        let constants_module_data = reader.read_vector::<u8>(constants_module_size as usize).to_vec();
        self.compile_imports = CompileTimeImports {
            flags: compile_imports_flags,
            constants_module: String::from_utf8(constants_module_data).unwrap(),
        };

        let imported = self.native_module.module.num_imported_functions;
        if imported > 