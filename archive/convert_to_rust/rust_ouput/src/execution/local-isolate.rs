// Converted from V8 C++ source files:
// Header: local-isolate.h
// Implementation: local-isolate.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod local_isolate {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::base::macros::UNREACHABLE;
    use crate::execution::isolate::Isolate;
    use crate::execution::isolate::Address;
    use crate::execution::isolate::ReadOnlyHeap;
    use crate::execution::isolate::RootsTable;
    use crate::execution::isolate::RootIndex;
    use crate::execution::isolate::V8FileLogger;
    use crate::execution::v8threads::ThreadId;
    use crate::heap::local_factory::LocalFactory;
    use crate::heap::local_heap::LocalHeap;
    use crate::handles::handles::Object;
    use crate::handles::handles::Handle;
    use crate::logging::runtime_call_stats::RuntimeCallStats;
    use crate::execution::mutex_guard_if_off_thread::MutexGuardIfOffThread;
    use std::sync::Mutex;

    pub struct HiddenLocalFactory {
        local_factory: LocalFactory,
    }

    impl HiddenLocalFactory {
        pub fn new(isolate: &Isolate) -> Self {
            HiddenLocalFactory {
                local_factory: LocalFactory::new(isolate),
            }
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum ThreadKind {
        kMain,
        kBackground,
    }

    pub struct LocalIsolate {
        heap_: LocalHeap,
        isolate_: *mut Isolate,
        logger_: Box<LocalLogger>,
        thread_id_: ThreadId,
        stack_limit_: Address,
        bigint_processor_: Option<Box<bigint::Processor>>,
        runtime_call_stats_: *mut RuntimeCallStats,

        //default_locale_: String,
    }

    impl LocalIsolate {
        pub fn new(isolate: *mut Isolate, kind: ThreadKind) -> LocalIsolate {
            let isolate_ref = unsafe { &*isolate };

            LocalIsolate {
                heap_: LocalHeap::new(isolate_ref.heap(), kind),
                isolate_: isolate,
                logger_: Box::new(LocalLogger::new(isolate_ref)),
                thread_id_: ThreadId::Current(),
                stack_limit_: Address {}, // initialize with dummy value
                bigint_processor_: None,
                runtime_call_stats_: isolate_ref.counters().runtime_call_stats(),
            }
        }

         // Kinda sketchy.
        pub fn from_heap(heap: &mut LocalHeap) -> *mut LocalIsolate {
            let heap_ptr = heap as *mut LocalHeap as usize;
            let offset = std::mem::offset_of!(LocalIsolate, heap_);
            (heap_ptr - offset) as *mut LocalIsolate
        }

        pub fn is_main_thread(&self) -> bool {
            self.heap_.is_main_thread()
        }

        pub fn heap(&mut self) -> &mut LocalHeap {
            &mut self.heap_
        }

        pub fn heap_const(&self) -> &LocalHeap {
            &self.heap_
        }

        pub fn cage_base(&self) -> Address {
            Address {}
        }

        pub fn code_cage_base(&self) -> Address {
            Address {}
        }

        pub fn read_only_heap(&self) -> &ReadOnlyHeap {
            unsafe {
                (&(*self.isolate_)).read_only_heap()
            }
        }

        pub fn roots_table(&mut self) -> &mut RootsTable {
            unsafe {
                (&mut (*self.isolate_)).roots_table()
            }
        }

        pub fn roots_table_const(&self) -> &RootsTable {
            unsafe {
                (&(*self.isolate_)).roots_table()
            }
        }

        pub fn root(&self, index: RootIndex) -> &Object {
            unsafe {
                (&(*self.isolate_)).root(index)
            }
        }

        pub fn root_handle(&self, index: RootIndex) -> Handle<Object> {
            unsafe {
                (&(*self.isolate_)).root_handle(index)
            }
        }

        pub fn fuzzer_rng(&self) -> &base::RandomNumberGenerator {
            unsafe {
                (&(*self.isolate_)).fuzzer_rng()
            }
        }

        pub fn string_table(&self) -> &StringTable {
             unsafe {
                (&(*self.isolate_)).string_table()
            }
        }

        pub fn internalized_string_access(&self) -> &Mutex {
            unsafe {
                (&(*self.isolate_)).internalized_string_access()
            }
        }

        pub fn shared_function_info_access(&self) -> &Mutex {
            unsafe {
                (&(*self.isolate_)).shared_function_info_access()
            }
        }

        pub fn ast_string_constants(&self) -> &AstStringConstants {
            unsafe {
                (&(*self.isolate_)).ast_string_constants()
            }
        }

        pub fn lazy_compile_dispatcher(&self) -> &LazyCompileDispatcher {
            unsafe {
                (&(*self.isolate_)).lazy_compile_dispatcher()
            }
        }

        pub fn main_thread_logger(&mut self) -> *mut V8FileLogger {
            unsafe {
                (&mut (*self.isolate_)).v8_file_logger()
            }
        }

        pub fn is_precise_binary_code_coverage(&self) -> bool {
            unsafe {
                (&(*self.isolate_)).is_precise_binary_code_coverage()
            }
        }

        pub fn factory(&mut self) -> &mut LocalFactory {
            unsafe {
                &mut ((*(self as *mut Self as *mut HiddenLocalFactory)).local_factory)
            }
        }

        pub fn isolate_group(&self) -> &IsolateGroup {
            unsafe {
                (&(*self.isolate_)).isolate_group()
            }
        }

        pub fn allocator(&self) -> &AccountingAllocator {
            unsafe {
                (&(*self.isolate_)).allocator()
            }
        }

        pub fn has_exception(&self) -> bool {
            false
        }

        pub fn serializer_enabled(&self) -> bool {
             unsafe {
                (&(*self.isolate_)).serializer_enabled()
            }
        }

        pub fn register_deserializer_started(&mut self) {
            unsafe {
                (&mut (*self.isolate_)).RegisterDeserializerStarted();
            }
        }

        pub fn register_deserializer_finished(&mut self) {
            unsafe {
                (&mut (*self.isolate_)).RegisterDeserializerFinished();
            }
        }

        pub fn has_active_deserializer(&self) -> bool {
             unsafe {
                (&(*self.isolate_)).has_active_deserializer()
            }
        }

        pub fn throw(&self, _exception: &Object) -> ! {
            UNREACHABLE()
        }

        pub fn fatal_process_out_of_heap_memory(&self, _location: &str) -> ! {
            UNREACHABLE()
        }

        pub fn get_next_script_id(&mut self) -> i32 {
            unsafe {
                (&mut (*self.isolate_)).GetNextScriptId()
            }
        }

        pub fn get_and_inc_next_unique_sfi_id(&mut self) -> u32 {
            unsafe {
                (&mut (*self.isolate_)).GetAndIncNextUniqueSfiId()
            }
        }

        pub fn v8_file_logger(&self) -> &LocalLogger {
            self.logger_.as_ref()
        }

        pub fn thread_id(&self) -> ThreadId {
            self.thread_id_
        }

        pub fn stack_limit(&self) -> Address {
            self.stack_limit_
        }

        pub fn runtime_call_stats(&self) -> *mut RuntimeCallStats {
            self.runtime_call_stats_
        }

        pub fn bigint_processor(&mut self) -> &mut bigint::Processor {
            if self.bigint_processor_.is_none() {
                self.initialize_bigint_processor();
            }
            self.bigint_processor_.as_mut().unwrap()
        }

        pub fn get_js_dispatch_table_space_for(&self, owning_slot: Address) -> &JSDispatchTable::Space {
            unsafe {
                (&(*self.isolate_)).GetJSDispatchTableSpaceFor(owning_slot)
            }
        }

        pub fn as_isolate(&mut self) -> &mut Isolate {
            unsafe {
                DCHECK_EQ!(self.thread_id_, (&mut (*self.isolate_)).thread_id());
                &mut (*self.isolate_)
            }
        }

        pub fn as_local_isolate(&mut self) -> &mut LocalIsolate {
            self
        }

        pub fn shared_space_isolate(&self) -> &LocalIsolate {
            unsafe {
                (&(*self.isolate_)).shared_space_isolate().main_thread_local_isolate()
            }
        }

        pub fn get_main_thread_isolate_unsafe(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn snapshot_blob(&self) -> &v8::StartupData {
            unsafe {
                (&(*self.isolate_)).snapshot_blob()
            }
        }

        pub fn pending_message_address(&mut self) -> &mut Object {
            unsafe {
                (&mut (*self.isolate_)).pending_message_address()
            }
        }

        pub fn next_optimization_id(&mut self) -> i32 {
            unsafe {
                (&mut (*self.isolate_)).NextOptimizationId()
            }
        }

        fn initialize_bigint_processor(&mut self) {
            self.bigint_processor_ = Some(Box::new(bigint::Processor::New(Box::new(bigint::Platform{}))));
        }
    }

    impl Drop for LocalIsolate {
        fn drop(&mut self) {
            if let Some(mut processor) = self.bigint_processor_.take() {
                processor.Destroy();
            }
        }
    }

    pub struct LocalLogger {
        isolate: *mut Isolate,
    }

    impl LocalLogger {
        pub fn new(isolate: &Isolate) -> Self {
            LocalLogger {
                isolate: isolate as *const Isolate as *mut Isolate,
            }
        }
    }

    pub mod base {
        pub struct RandomNumberGenerator {}
        pub struct Mutex {}
        pub struct MutexGuard {}
    }

    pub mod bigint {
        pub struct Processor {}
        impl Processor {
            pub fn New(_platform: Box<Platform>) -> Self {
                Processor{}
            }
            pub fn Destroy(&mut self) {}
        }

        pub struct Platform {}
    }

    pub mod v8 {
        pub struct StartupData {}
    }

    pub mod logging {
        pub struct Counters {

        }
        impl Counters {
            pub fn runtime_call_stats(&self) -> *mut RuntimeCallStats {
                std::ptr::null_mut()
            }
        }
        pub struct RuntimeCallStatsScope {}
    }

    pub mod heap {
        pub struct Heap {}
    }

    pub mod handles {
        pub struct MaybeHandle<T> {
            value: Option<T>,
        }
        impl<T> MaybeHandle<T> {
            pub fn new(value: Option<T>) -> Self {
                MaybeHandle { value }
            }

            pub fn to_handle(self) -> Option<T> {
                self.value
            }
        }
    }

    pub mod isolate {
        use crate::execution::isolate::Address;
        use crate::execution::isolate::ReadOnlyHeap;
        use crate::execution::isolate::RootsTable;
        use crate::execution::isolate::RootIndex;
        use crate::execution::v8threads::ThreadId;
        use crate::heap::local_heap::LocalHeap;
        use crate::logging::runtime_call_stats::RuntimeCallStats;
        use crate::execution::local_isolate::LocalIsolate;
        use crate::execution::local_isolate::ThreadKind;
        use std::sync::Mutex;
        use crate::execution::isolate::V8FileLogger;

        pub struct Isolate {
            heap_: LocalHeap,
            thread_id_: ThreadId,
            stack_limit_: Address,
            read_only_heap_: ReadOnlyHeap,
            roots_table_: RootsTable,
            string_table_: StringTable,
            internalized_string_access_: Mutex,
            shared_function_info_access_: Mutex,
            ast_string_constants_: AstStringConstants,
            lazy_compile_dispatcher_: LazyCompileDispatcher,
            v8_file_logger_: V8FileLogger,
            counters_: Counters,
            serializer_enabled_: bool,
            next_script_id_: i32,
            next_unique_sfi_id_: u32,
            isolate_group_: IsolateGroup,
            allocator_: AccountingAllocator,
            fuzzer_rng_: RandomNumberGenerator,
            shared_space_isolate_: Box<Isolate>,
            snapshot_blob_: v8::StartupData,
            pending_message_address_: Object,
            next_optimization_id_: i32,
            is_precise_binary_code_coverage_: bool,
            has_active_deserializer_: bool,

            //default_locale_: String,
        }

        impl Isolate {
            pub fn new() -> Isolate {
                Isolate {
                    heap_: LocalHeap::new(&Heap{}, ThreadKind::kMain),
                    thread_id_: ThreadId::Current(),
                    stack_limit_: Address {},
                    read_only_heap_: ReadOnlyHeap {},
                    roots_table_: RootsTable {},
                    string_table_: StringTable {},
                    internalized_string_access_: Mutex::new(()),
                    shared_function_info_access_: Mutex::new(()),
                    ast_string_constants_: AstStringConstants {},
                    lazy_compile_dispatcher_: LazyCompileDispatcher {},
                    v8_file_logger_: V8FileLogger{},
                    counters_: Counters {},
                    serializer_enabled_: false,
                    next_script_id_: 0,
                    next_unique_sfi_id_: 0,
                    isolate_group_: IsolateGroup {},
                    allocator_: AccountingAllocator {},
                    fuzzer_rng_: RandomNumberGenerator {},
                    shared_space_isolate_: Box::new(Isolate::new()),
                    snapshot_blob_: v8::StartupData {},
                    pending_message_address_: Object {},
                    next_optimization_id_: 0,
                    is_precise_binary_code_coverage_: false,
                    has_active_deserializer_: false,
                    //default_locale_: String::new(),

                }
            }

            pub fn heap(&self) -> &LocalHeap {
                &self.heap_
            }

            pub fn thread_id(&self) -> ThreadId {
                self.thread_id_
            }

            pub fn stack_guard(&self) -> &StackGuard {
                &StackGuard{}
            }

            pub fn read_only_heap(&self) -> &ReadOnlyHeap {
                &self.read_only_heap_
            }

            pub fn roots_table(&mut self) -> &mut RootsTable {
                &mut self.roots_table_
            }

            pub fn root(&self, _index: RootIndex) -> &Object {
                &Object {}
            }
            pub fn root_handle(&self, _index: RootIndex) -> Handle<Object> {
                Handle::new()
            }

            pub fn string_table(&self) -> &StringTable {
                &self.string_table_
            }
            pub fn internalized_string_access(&self) -> &Mutex {
                &self.internalized_string_access_
            }
             pub fn shared_function_info_access(&self) -> &Mutex {
                &self.shared_function_info_access_
            }
            pub fn ast_string_constants(&self) -> &AstStringConstants {
                &self.ast_string_constants_
            }
             pub fn lazy_compile_dispatcher(&self) -> &LazyCompileDispatcher {
                &self.lazy_compile_dispatcher_
            }

            pub fn v8_file_logger(&mut self) -> *mut V8FileLogger {
                &mut self.v8_file_logger_
            }

            pub fn counters(&self) -> &Counters {
                &self.counters_
            }

            pub fn serializer_enabled(&self) -> bool {
                self.serializer_enabled_
            }

            pub fn GetNextScriptId(&mut self) -> i32 {
                self.next_script_id_ += 1;
                self.next_script_id_
            }

             pub fn GetAndIncNextUniqueSfiId(&mut self) -> u32 {
                self.next_unique_sfi_id_ += 1;
                self.next_unique_sfi_id_
            }

            pub fn isolate_group(&self) -> &IsolateGroup {
                &self.isolate_group_
            }
            pub fn allocator(&self) -> &AccountingAllocator {
                &self.allocator_
            }
            pub fn fuzzer_rng(&self) -> &RandomNumberGenerator {
                &self.fuzzer_rng_
            }

            pub fn shared_space_isolate(&self) -> &Isolate {
                &self.shared_space_isolate_
            }
            pub fn snapshot_blob(&self) -> &v8::StartupData {
                &self.snapshot_blob_
            }
            pub fn pending_message_address(&mut self) -> &mut Object {
                &mut self.pending_message_address_
            }
            pub fn NextOptimizationId(&mut self) -> i32 {
                self.next_optimization_id_ += 1;
                self.next_optimization_id_
            }

            pub fn is_precise_binary_code_coverage(&self) -> bool {
                self.is_precise_binary_code_coverage_
            }

            pub fn RegisterDeserializerStarted(&mut self) {
                self.has_active_deserializer_ = true;
            }

            pub fn RegisterDeserializerFinished(&mut self) {
                self.has_active_deserializer_ = false;
            }

            pub fn has_active_deserializer(&self) -> bool {
                self.has_active_deserializer_
            }

            pub fn GetJSDispatchTableSpaceFor(&self, _owning_slot: Address) -> &JSDispatchTable::Space {
                &JSDispatchTable::Space {}
            }

            pub fn main_thread_local_isolate(&self) -> &LocalIsolate {
                unsafe {
                    std::mem::transmute::<&Isolate, &LocalIsolate>(self)
                }
            }
        }

        pub struct StackGuard {}
        impl StackGuard {
            pub fn real_climit(&self) -> Address {
                Address {}
            }
        }
        pub struct AccountingAllocator {}
        pub struct RandomNumberGenerator {}
        pub struct IsolateGroup {}

    }
    pub struct StringTable {}
    pub struct AstStringConstants {}
    pub struct LazyCompileDispatcher {}
    pub struct JSDispatchTable {}
    impl JSDispatchTable {
        pub struct Space {}
    }

    #[macro_export]
    macro_rules! DCHECK {
        ($x:expr) => {
            if !$x {
                panic!("DCHECK failed: {}", stringify!($x));
            }
        };
    }

    extern "C" {
        fn GetCurrentStackPosition() -> Address;
    }

    pub struct StackLimitCheck {}
    impl StackLimitCheck {
        pub fn HasOverflowed(local_isolate: &LocalIsolate) -> bool {
            unsafe {
                GetCurrentStackPosition() < local_isolate.stack_limit()
            }
        }
    }
}
