// Converted from V8 C++ source files:
// Header: v8-statistics.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub enum MeasureMemoryMode {
    kSummary,
    kDetailed,
}

pub enum MeasureMemoryExecution {
    kDefault,
    kEager,
    kLazy,
}

pub struct Result {
    pub contexts: Vec<*const Context>,
    pub sizes_in_bytes: Vec<usize>,
    pub unattributed_size_in_bytes: usize,
    pub wasm_code_size_in_bytes: usize,
    pub wasm_metadata_size_in_bytes: usize,
}

pub trait MeasureMemoryDelegateTrait {
    fn should_measure(&self, context: *const Context) -> bool;
    fn measurement_complete(&mut self, result: Result);
}

pub struct MeasureMemoryDelegate {
    inner: Box<dyn MeasureMemoryDelegateTrait>,
}

impl MeasureMemoryDelegate {
    pub fn new(inner: Box<dyn MeasureMemoryDelegateTrait>) -> Self {
        MeasureMemoryDelegate { inner }
    }

    pub fn should_measure(&self, context: *const Context) -> bool {
        self.inner.should_measure(context)
    }

    pub fn measurement_complete(&mut self, result: Result) {
        self.inner.measurement_complete(result)
    }

    pub fn default(
        isolate: *mut Isolate,
        context: Local<Context>,
        promise_resolver: Local<Promise::Resolver>,
        mode: MeasureMemoryMode,
    ) -> Box<dyn MeasureMemoryDelegateTrait> {
        Box::new(DefaultMeasureMemoryDelegate {
            isolate,
            context,
            promise_resolver,
            mode,
        })
    }
}

struct DefaultMeasureMemoryDelegate {
    isolate: *mut Isolate,
    context: Local<Context>,
    promise_resolver: Local<Promise::Resolver>,
    mode: MeasureMemoryMode,
}

impl MeasureMemoryDelegateTrait for DefaultMeasureMemoryDelegate {
    fn should_measure(&self, _context: *const Context) -> bool {
        true // Default implementation measures all contexts
    }

    fn measurement_complete(&mut self, result: Result) {
        // In a real implementation, this would resolve the promise
        // with the measurement result. For now, we just print the result.
        println!("Measurement complete:");
        println!("  Unattributed size: {}", result.unattributed_size_in_bytes);
        println!("  Wasm code size: {}", result.wasm_code_size_in_bytes);
        println!("  Wasm metadata size: {}", result.wasm_metadata_size_in_bytes);
        println!("  Number of contexts: {}", result.contexts.len());
    }
}

#[derive(Default)]
pub struct SharedMemoryStatistics {
    read_only_space_size_: usize,
    read_only_space_used_size_: usize,
    read_only_space_physical_size_: usize,
}

impl SharedMemoryStatistics {
    pub fn new() -> Self {
        SharedMemoryStatistics {
            read_only_space_size_: 0,
            read_only_space_used_size_: 0,
            read_only_space_physical_size_: 0,
        }
    }
    pub fn read_only_space_size(&self) -> usize {
        self.read_only_space_size_
    }
    pub fn read_only_space_used_size(&self) -> usize {
        self.read_only_space_used_size_
    }
    pub fn read_only_space_physical_size(&self) -> usize {
        self.read_only_space_physical_size_
    }
}

#[derive(Default)]
pub struct HeapStatistics {
    total_heap_size_: usize,
    total_heap_size_executable_: usize,
    total_physical_size_: usize,
    total_available_size_: usize,
    used_heap_size_: usize,
    heap_size_limit_: usize,
    malloced_memory_: usize,
    external_memory_: usize,
    peak_malloced_memory_: usize,
    does_zap_garbage_: bool,
    number_of_native_contexts_: usize,
    number_of_detached_contexts_: usize,
    total_global_handles_size_: usize,
    used_global_handles_size_: usize,
}

impl HeapStatistics {
    pub fn new() -> Self {
        HeapStatistics {
            total_heap_size_: 0,
            total_heap_size_executable_: 0,
            total_physical_size_: 0,
            total_available_size_: 0,
            used_heap_size_: 0,
            heap_size_limit_: 0,
            malloced_memory_: 0,
            external_memory_: 0,
            peak_malloced_memory_: 0,
            does_zap_garbage_: false,
            number_of_native_contexts_: 0,
            number_of_detached_contexts_: 0,
            total_global_handles_size_: 0,
            used_global_handles_size_: 0,
        }
    }
    pub fn total_heap_size(&self) -> usize {
        self.total_heap_size_
    }
    pub fn total_heap_size_executable(&self) -> usize {
        self.total_heap_size_executable_
    }
    pub fn total_physical_size(&self) -> usize {
        self.total_physical_size_
    }
    pub fn total_available_size(&self) -> usize {
        self.total_available_size_
    }
    pub fn total_global_handles_size(&self) -> usize {
        self.total_global_handles_size_
    }
    pub fn used_global_handles_size(&self) -> usize {
        self.used_global_handles_size_
    }
    pub fn used_heap_size(&self) -> usize {
        self.used_heap_size_
    }
    pub fn heap_size_limit(&self) -> usize {
        self.heap_size_limit_
    }
    pub fn malloced_memory(&self) -> usize {
        self.malloced_memory_
    }
    pub fn external_memory(&self) -> usize {
        self.external_memory_
    }
    pub fn peak_malloced_memory(&self) -> usize {
        self.peak_malloced_memory_
    }
    pub fn number_of_native_contexts(&self) -> usize {
        self.number_of_native_contexts_
    }
    pub fn number_of_detached_contexts(&self) -> usize {
        self.number_of_detached_contexts_
    }
    pub fn does_zap_garbage(&self) -> usize {
        if self.does_zap_garbage_ {
            1
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct HeapSpaceStatistics {
    space_name_: &'static str,
    space_size_: usize,
    space_used_size_: usize,
    space_available_size_: usize,
    physical_space_size_: usize,
}

impl HeapSpaceStatistics {
    pub fn new() -> Self {
        HeapSpaceStatistics {
            space_name_: "",
            space_size_: 0,
            space_used_size_: 0,
            space_available_size_: 0,
            physical_space_size_: 0,
        }
    }
    pub fn space_name(&self) -> &str {
        self.space_name_
    }
    pub fn space_size(&self) -> usize {
        self.space_size_
    }
    pub fn space_used_size(&self) -> usize {
        self.space_used_size_
    }
    pub fn space_available_size(&self) -> usize {
        self.space_available_size_
    }
    pub fn physical_space_size(&self) -> usize {
        self.physical_space_size_
    }
}

#[derive(Default)]
pub struct HeapObjectStatistics {
    object_type_: &'static str,
    object_sub_type_: &'static str,
    object_count_: usize,
    object_size_: usize,
}

impl HeapObjectStatistics {
    pub fn new() -> Self {
        HeapObjectStatistics {
            object_type_: "",
            object_sub_type_: "",
            object_count_: 0,
            object_size_: 0,
        }
    }
    pub fn object_type(&self) -> &str {
        self.object_type_
    }
    pub fn object_sub_type(&self) -> &str {
        self.object_sub_type_
    }
    pub fn object_count(&self) -> usize {
        self.object_count_
    }
    pub fn object_size(&self) -> usize {
        self.object_size_
    }
}

#[derive(Default)]
pub struct HeapCodeStatistics {
    code_and_metadata_size_: usize,
    bytecode_and_metadata_size_: usize,
    external_script_source_size_: usize,
    cpu_profiler_metadata_size_: usize,
}

impl HeapCodeStatistics {
    pub fn new() -> Self {
        HeapCodeStatistics {
            code_and_metadata_size_: 0,
            bytecode_and_metadata_size_: 0,
            external_script_source_size_: 0,
            cpu_profiler_metadata_size_: 0,
        }
    }
    pub fn code_and_metadata_size(&self) -> usize {
        self.code_and_metadata_size_
    }
    pub fn bytecode_and_metadata_size(&self) -> usize {
        self.bytecode_and_metadata_size_
    }
    pub fn external_script_source_size(&self) -> usize {
        self.external_script_source_size_
    }
    pub fn cpu_profiler_metadata_size(&self) -> usize {
        self.cpu_profiler_metadata_size_
    }
}
