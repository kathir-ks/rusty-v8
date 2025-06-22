// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/profiler/cpu-profiler.h (Module definition)
mod cpu_profiler {
    use std::time::Duration;

    pub struct CpuProfilingOptions {
        pub record_samples: bool,
        // Other options...
    }

    pub enum CpuProfilingStatus {
        Started,
        AlreadyStarted,
        Failed,
    }

    pub struct CpuProfilingResult {
        pub status: CpuProfilingStatus,
        // Other results...
    }

    pub trait DiscardedSamplesDelegate {
        fn discarded_samples(&self, count: usize);
    }

    pub enum CpuProfilingNamingMode {
        UserProvided,
        SourceText,
    }

    pub enum CpuProfilingLoggingMode {
        EagerLogging,
        LazyLogging,
    }

    pub struct CpuProfile;

    pub type ProfilerId = i32;

    pub trait CpuProfilerInterface {
        fn get_profiles_count(&self) -> i32;
        fn get_profile(&self, index: usize) -> Option<&CpuProfile>;
        fn delete_all_profiles(&mut self);
        fn delete_profile(&mut self, profile: &CpuProfile);
        fn start_profiling(
            &mut self,
            options: CpuProfilingOptions,
            delegate: Box<dyn DiscardedSamplesDelegate>,
        ) -> CpuProfilingResult;
        fn stop_profiling(&mut self, id: ProfilerId) -> Option<&CpuProfile>;
    }
}

// src/profiler/cpu-profiler.cc (Implementation)

use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Instant;
//use v8::support::AlignedAllocWithRetry;
//use v8::support::AlignedFree;

//use crate::base::lazy_instance::LazyInstance;
//use crate::base::template_utils::USE;
//use crate::debug::debug::StackFrameIterator;
//use crate::execution::frames_inl::StackFrame;
//use crate::execution::v8threads::ThreadId;
//use crate::execution::vm_state_inl::VMState;
//use crate::libsampler::sampler::Sampler;
//use crate::logging::counters::RuntimeCallStats;
//use crate::logging::log::LogEventListener;
//use crate::profiler::cpu_profiler_inl::{
//    CodeEventsContainer, CodeEventRecord, CodeEntryStorage,
//};
//use crate::profiler::profiler_stats::ProfilerStats;
//use crate::profiler::symbolizer::Symbolizer;
//use crate::utils::locked_queue_inl::LockedQueue;

//use self::cpu_profiler::CpuProfile;
use self::cpu_profiler::{
    CpuProfilingLoggingMode, CpuProfilingNamingMode, CpuProfilingOptions,
    CpuProfilingResult, CpuProfilingStatus, DiscardedSamplesDelegate, ProfilerId,
};

const KB: usize = 1024;
const MB: usize = 1024 * KB;
const kProfilerStackSize: usize = 256 * KB;

//#[cfg(V8_ENABLE_WEBASSEMBLY)]
//use crate::wasm::wasm_engine::GetWasmEngine;

struct CpuSampler {
    isolate: *mut Isolate, // Assuming Isolate is defined elsewhere
    processor: *mut SamplingEventsProcessor,
    per_thread_data: *mut PerIsolateThreadData, // Assuming PerIsolateThreadData is defined elsewhere
}

impl CpuSampler {
    fn new(isolate: *mut Isolate, processor: *mut SamplingEventsProcessor) -> Self {
        CpuSampler {
            isolate,
            processor,
            per_thread_data: unsafe { (*isolate).find_per_thread_data_for_this_thread() },
        }
    }

    fn sample_stack(&mut self, regs: &RegisterState) {
        let isolate = self.isolate;
        if unsafe { (*isolate).was_locker_ever_used() }
            && (!unsafe { (*isolate).thread_manager().is_locked_by_thread((*self.per_thread_data).thread_id()) }
                || unsafe { (*self.per_thread_data).thread_state() != std::ptr::null_mut() })
        {
           // ProfilerStats::Instance().AddReason(ProfilerStats::Reason::kIsolateNotLocked);
            return;
        }

        // #[cfg(V8_HEAP_USE_PKU_JIT_WRITE_PROTECT)]
        // i::RwxMemoryWriteScope::SetDefaultPermissionsForSignalHandler();

        let sample = unsafe { (*self.processor).start_tick_sample() };
        if sample.is_null() {
            // ProfilerStats::Instance().AddReason(ProfilerStats::Reason::kTickBufferFull);
            return;
        }

        // Every bailout up until here resulted in a dropped sample. From now on,
        // the sample is created in the buffer.
        unsafe {
            (*sample).init(
                isolate,
                regs,
                TickSample::kIncludeCEntryFrame,
                true, // update_stats
                true, // use_simulator_reg_state
                (*self.processor).period(),
            );
        }

        //  if self.is_counting_samples_ && !sample.timestamp.IsNull() {
        //      if sample.state == JS { self.js_sample_count_ += 1; }
        //      if sample.state == EXTERNAL { self.external_sample_count_ += 1; }
        //  }

        unsafe { (*self.processor).finish_tick_sample() };
    }
}

struct ProfilingScope {
    isolate: *mut Isolate,
    listener: *mut ProfilerListener,
}

impl ProfilingScope {
    fn new(isolate: *mut Isolate, listener: *mut ProfilerListener) -> Self {
        unsafe {
            let mut profiler_count = (*isolate).num_cpu_profilers();
            profiler_count += 1;
            (*isolate).set_num_cpu_profilers(profiler_count);
            (*isolate).set_is_profiling(true);

            // #[cfg(V8_ENABLE_WEBASSEMBLY)]
            // GetWasmEngine().EnableCodeLogging(isolate);

            //  CHECK((*isolate).logger().AddListener(listener));
            //   V8FileLogger* file_logger = (*isolate).v8_file_logger();
            // Populate the ProfilerCodeObserver with the initial functions and
            // callbacks on the heap.
            // DCHECK((*isolate).heap().HasBeenSetUp());

            //  if !v8_flags.prof_browser_mode {
            //      file_logger.LogCodeObjects();
            //  }
            //  file_logger.LogCompiledFunctions();
            //  file_logger.LogAccessorCallbacks();
        }

        ProfilingScope { isolate, listener }
    }
}

impl Drop for ProfilingScope {
    fn drop(&mut self) {
        unsafe {
            // CHECK((*self.isolate).logger().RemoveListener(self.listener));

            let mut profiler_count = (*self.isolate).num_cpu_profilers();
            //  DCHECK_GT(profiler_count, 0);
            profiler_count -= 1;
            (*self.isolate).set_num_cpu_profilers(profiler_count);
            if profiler_count == 0 {
                (*self.isolate).set_is_profiling(false);
            }
        }
    }
}

struct ProfilerEventsProcessor {
    thread: Option<thread::JoinHandle<()>>,
    symbolizer: *mut Symbolizer,
    code_observer: *mut ProfilerCodeObserver,
    profiles: *mut CpuProfilesCollection,
    last_code_event_id: u64,
    last_processed_code_event_id: u64,
    isolate: *mut Isolate,
    events_buffer: LockedQueue<CodeEventsContainer>,
    ticks_from_vm_buffer: LockedQueue<TickSampleEventRecord>,
    running_: Arc<AtomicBool>,
    running_mutex_: Arc<Mutex<()>>,
    running_cond_: Arc<Condvar>,
}

impl ProfilerEventsProcessor {
    fn new(
        isolate: *mut Isolate,
        symbolizer: *mut Symbolizer,
        code_observer: *mut ProfilerCodeObserver,
        profiles: *mut CpuProfilesCollection,
    ) -> Self {
        // DCHECK(!code_observer_->processor());
        // code_observer_->set_processor(this);
        unsafe { (*code_observer).set_processor(std::ptr::null_mut()) };

        let running_ = Arc::new(AtomicBool::new(false));
        let running_mutex_ = Arc::new(Mutex::new(()));
        let running_cond_ = Arc::new(Condvar::new());

        ProfilerEventsProcessor {
            thread: None,
            symbolizer,
            code_observer,
            profiles,
            last_code_event_id: 0,
            last_processed_code_event_id: 0,
            isolate,
            events_buffer: LockedQueue::new(),
            ticks_from_vm_buffer: LockedQueue::new(),
            running_,
            running_mutex_,
            running_cond_,
        }
    }

    fn enqueue(&mut self, mut event: CodeEventsContainer) {
        self.last_code_event_id += 1;
        event.generic.order = self.last_code_event_id;
        self.events_buffer.enqueue(event);
    }

    fn add_deopt_stack(&mut self, from: usize, fp_to_sp_delta: i32) {
        let mut record = TickSampleEventRecord::new(self.last_code_event_id);
        let mut regs = RegisterState::default();
        let fp = unsafe { (*self.isolate).c_entry_fp((*self.isolate).thread_local_top()) };
        regs.sp = (fp as isize - fp_to_sp_delta as isize) as *mut std::ffi::c_void;
        regs.fp = fp as *mut std::ffi::c_void;
        regs.pc = from as *mut std::ffi::c_void;
        unsafe {
            record.sample.init(
                self.isolate,
                &regs,
                TickSample::kSkipCEntryFrame,
                false,
                false,
            );
        }
        self.ticks_from_vm_buffer.enqueue(record);
    }

    fn add_current_stack(
        &mut self,
        update_stats: bool,
        trace_id: Option<u64>,
    ) {
        let mut record = TickSampleEventRecord::new(self.last_code_event_id);
        let mut regs = RegisterState::default();
        // Assuming StackFrameIterator and related types are defined elsewhere
        // let it = StackFrameIterator::new(self.isolate, (*self.isolate).thread_local_top(), StackFrameIterator::NoHandles);
        // if !it.done() {
        //   let frame = it.frame();
        //   regs.sp = frame.sp() as *mut std::ffi::c_void;
        //   regs.fp = frame.fp() as *mut std::ffi::c_void;
        //   regs.pc = frame.pc() as *mut std::ffi::c_void;
        // }
        unsafe {
            record.sample.init(
                self.isolate,
                &regs,
                TickSample::kSkipCEntryFrame,
                update_stats,
                false,
                Duration::from_secs(0),
                trace_id,
            );
        }
        self.ticks_from_vm_buffer.enqueue(record);
    }

    fn add_sample(&mut self, sample: TickSample) {
        let mut record = TickSampleEventRecord::new(self.last_code_event_id);
        record.sample = sample;
        self.ticks_from_vm_buffer.enqueue(record);
    }

    fn stop_synchronously(&mut self) {
        let expected = true;
        if !self
            .running_
            .compare_exchange(expected, false, Ordering::Relaxed, Ordering::Relaxed)
        {
            return;
        }
        let guard = self.running_mutex_.lock().unwrap();
        self.running_cond_.notify_one();
        drop(guard); // Explicitly drop the guard before joining
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }

    fn process_code_event(&mut self) -> bool {
        if let Some(mut record) = self.events_buffer.dequeue() {
            if record.generic.event_type == CodeEventRecordType::NativeContextMove {
                // let nc_record = record.NativeContextMoveEventRecord_;
                // unsafe {
                //     (*self.profiles).UpdateNativeContextAddressForCurrentProfiles(
                //         nc_record.from_address,
                //         nc_record.to_address,
                //     );
                // }
            } else {
                unsafe { (*self.code_observer).code_event_handler_internal(record) };
            }
            self.last_processed_code_event_id = record.generic.order;
            true
        } else {
            false
        }
    }

    fn code_event_handler(&mut self, evt_rec: &CodeEventsContainer) {
        match evt_rec.generic.event_type {
            CodeEventRecordType::CodeCreation
            | CodeEventRecordType::CodeMove
            | CodeEventRecordType::CodeDisableOpt
            | CodeEventRecordType::CodeDelete
            | CodeEventRecordType::NativeContextMove => {
                let evt_rec_copy = evt_rec.clone();
                self.enqueue(evt_rec_copy);
            }
            CodeEventRecordType::CodeDeopt => {
                // let rec = &evt_rec.CodeDeoptEventRecord_;
                // let pc = rec.pc;
                // let fp_to_sp_delta = rec.fp_to_sp_delta;
                let evt_rec_copy = evt_rec.clone();
                self.enqueue(evt_rec_copy);
                // self.add_deopt_stack(pc, fp_to_sp_delta);
            }
            CodeEventRecordType::NoEvent | CodeEventRecordType::ReportBuiltin => {
                // UNREACHABLE(); // No direct equivalent in Rust, consider logging or panicking
            }
        }
    }

    fn run(&mut self) {
        let running_mutex = Arc::clone(&self.running_mutex_);
        let running_cond = Arc::clone(&self.running_cond_);
        let running = Arc::clone(&self.running_);

        let mut guard = running_mutex.lock().unwrap();

        while running.load(Ordering::Relaxed) {
            // Simulate work with a sleep.
            thread::sleep(Duration::from_millis(100));

            let (new_guard, timeout_result) =
                running_cond.wait_timeout(guard, Duration::from_secs(1)).unwrap();
            guard = new_guard;

            match timeout_result.timed_out() {
                true => {
                    println!("Timed out, doing more work.");
                }
                false => {
                    if !running.load(Ordering::Relaxed) {
                        break;
                    }
                    println!("Spurious wakeup");
                }
            }
        }
        println!("Thread is exiting.");
    }

    fn start_synchronously(&mut self) -> Result<(), String> {
        let running = Arc::clone(&self.running_);
        let running_mutex = Arc::clone(&self.running_mutex_);
        let running_cond = Arc::clone(&self.running_cond_);

        if self.running_.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.running_.store(true, Ordering::Relaxed);

        let mut this = self;

        let builder = thread::Builder::new().name("v8:ProfEvntProc".into()).stack_size(kProfilerStackSize);
        self.thread = Some(builder.spawn(move || {
            this.run();
        }).map_err(|e| e.to_string())?);

        Ok(())
    }
}

impl Drop for ProfilerEventsProcessor {
    fn drop(&mut self) {
        unsafe {
            //  DCHECK_EQ((*self.code_observer).processor(), self);
            //   (*self.code_observer).clear_processor();
        }
    }
}

struct SamplingEventsProcessor {
    base: ProfilerEventsProcessor,
    sampler: Box<CpuSampler>,
    period: Duration,
    use_precise_sampling: bool,
    //#[cfg(V8_OS_WIN)]
    //precise_sleep_timer_: PreciseSleepTimer,
}

impl SamplingEventsProcessor {
    fn new(
        isolate: *mut Isolate,
        symbolizer: *mut Symbolizer,
        code_observer: *mut ProfilerCodeObserver,
        profiles: *mut CpuProfilesCollection,
        period: Duration,
        use_precise_sampling: bool,
    ) -> Box<Self> {
        let mut this = Self {
            base: ProfilerEventsProcessor::new(isolate, symbolizer, code_observer, profiles),
            sampler: Box::new(CpuSampler::new(isolate, std::ptr::null_mut())), // Temp value
            period,
            use_precise_sampling,
            //#[cfg(V8_OS_WIN)]
            //precise_sleep_timer_: PreciseSleepTimer::new(),
        };
        this.base.running_.store(true, Ordering::Relaxed);
        unsafe { this.sampler = Box::new(CpuSampler::new(isolate, &mut this.base as *mut ProfilerEventsProcessor as *mut SamplingEventsProcessor)) };
        //this.sampler.start();

        Box::new(this)
    }

    unsafe fn start_tick_sample(&mut self) -> *mut TickSample {
        //Placeholder
        std::ptr::null_mut()
    }

    unsafe fn finish_tick_sample(&mut self) {
        //Placeholder
    }

    fn period(&self) -> Duration {
        self.period
    }

    fn symbolize_and_add_to_profiles(&mut self, record: &TickSampleEventRecord) {
        //Placeholder
        // let tick_sample = &record.sample;
        // let symbolized = unsafe {
        //     (*self.base.symbolizer).symbolize_tick_sample(tick_sample)
        // };
        // unsafe {
        //     (*self.base.profiles).add_path_to_current_profiles(
        //         tick_sample.timestamp,
        //         symbolized.stack_trace,
        //         symbolized.src_line,
        //         tick_sample.update_stats_,
        //         tick_sample.sampling_interval_,
        //         tick_sample.state,
        //         tick_sample.embedder_state,
        //         tick_sample.context as usize,
        //         tick_sample.embedder_context as usize,
        //         tick_sample.trace_id_,
        //     );
        // }
    }

    fn process_one_sample(&mut self) -> SampleProcessingResult {
        //Placeholder
        SampleProcessingResult::NoSamplesInQueue
    }

    fn run(&mut self) {
        //Placeholder
    }

    fn set_sampling_interval(&mut self, period: Duration) {
        if self.period == period {
            return;
        }

        self.base.stop_synchronously();
        self.period = period;

        self.base.running_.store(true, Ordering::Relaxed);
        self.base.start_synchronously().unwrap();
    }

    //TODO Implement Operator New and Delete
}

impl Drop for SamplingEventsProcessor {
    fn drop(&mut self) {
        //self.sampler.stop();
    }
}

#[derive(PartialEq)]
enum SampleProcessingResult {
    OneSampleProcessed,
    NoSamplesInQueue,
    FoundSampleForNextCodeEvent,
}

struct ProfilerCodeObserver {
    isolate: *mut Isolate,
    code_entries: CodeEntryStorage,
    code_map: CodeMap,
    weak_code_registry: WeakCodeRegistry,
    processor: *mut ProfilerEventsProcessor,
}

impl ProfilerCodeObserver {
    fn new(isolate: *mut Isolate, storage: CodeEntryStorage) -> Self {
        let mut observer = ProfilerCodeObserver {
            isolate,
            code_entries: storage,
            code_map: CodeMap::new(),
            weak_code_registry: WeakCodeRegistry::new(isolate),
            processor: std::ptr::null_mut(),
        };
        // observer.create_entries_for_runtime_call_stats();
        // observer.log_builtins();
        observer
    }

    fn clear_code_map(&mut self) {
        self.weak_code_registry.clear();
        self.code_map.clear();
    }

    fn code_event_handler(&mut self, evt_rec: &CodeEventsContainer) {
        if !self.processor.is_null() {
            unsafe { (*self.processor).code_event_handler(evt_rec) };
            return;
        }
        self.code_event_handler_internal(evt_rec);
    }

    fn get_estimated_memory_usage(&self) -> usize {
        // To avoid race condition in codemap,
        // for now limit computation in kEagerLogging mode
        if self.processor.is_null() {
            0 // Placeholder
              // return std::mem::size_of(self) + self.code_map.get_estimated_memory_usage() +
              //        self.code_entries.strings().get_string_size();
        } else {
            0
        }
    }

    fn code_event_handler_internal(&mut self, evt_rec: CodeEventsContainer) {
        let mut record = evt_rec;
        match evt_rec.generic.event_type {
            CodeEventRecordType::CodeCreation => {
                //record.CodeCreationEventRecord_.UpdateCodeMap(&self.code_map);
            }
            CodeEventRecordType::CodeMove => {
                //record.CodeMoveEventRecord_.UpdateCodeMap(&self.code_map);
            }
            CodeEventRecordType::CodeDisableOpt => {
                //record.CodeDisableOptEventRecord_.UpdateCodeMap(&self.code_map);
            }
            CodeEventRecordType::CodeDelete => {
                //record.CodeDeleteEventRecord_.UpdateCodeMap(&self.code_map);
            }
            CodeEventRecordType::NativeContextMove => {
                //record.NativeContextMoveEventRecord_.UpdateCodeMap(&self.code_map);
            }
            _ => {}
        }
    }

    fn create_entries_for_runtime_call_stats(&mut self) {
        //Placeholder
    }

    fn log_builtins(&mut self) {
        //Placeholder
    }

    unsafe fn set_processor(&mut self, processor: *mut ProfilerEventsProcessor) {
        self.processor = processor;
    }
}

struct CpuProfiler {
    isolate: *mut Isolate,
    naming_mode: CpuProfilingNamingMode,
    logging_mode: CpuProfilingLoggingMode,
    base_sampling_interval: Duration,
    code_observer: Box<ProfilerCodeObserver>,
    profiles: Box<CpuProfilesCollection>,
    symbolizer: Option<Box<Symbolizer>>,
    processor: Option<Box<SamplingEventsProcessor>>,
    is_profiling: bool,
    profiler_listener: Option<Box<ProfilerListener>>,
    profiling_scope: Option<ProfilingScope>,
    use_precise_sampling_: bool,
    code_entries_: CodeEntryStorage,
}

impl CpuProfiler {
    fn new(
        isolate: *mut Isolate,
        naming_mode: CpuProfilingNamingMode,
        logging_mode: CpuProfilingLoggingMode,
    ) -> Self {
        CpuProfiler::new_with_components(
            isolate,
            naming_mode,
            logging_mode,
            Box::new(CpuProfilesCollection::new(isolate)),
            None,
            None,
        )
    }

    fn new_with_components(
        isolate: *mut Isolate,
        naming_mode: CpuProfilingNamingMode,
        logging_mode: CpuProfilingLoggingMode,
        test_profiles: Box<CpuProfilesCollection>,
        test_symbolizer: Option<Box<Symbolizer>>,
        test_processor: Option<Box<SamplingEventsProcessor>>,
    ) -> Self {
        let code_entries_ = CodeEntryStorage::new();
        let test_code_observer = ProfilerCodeObserver::new(isolate, code_entries_.clone());
        let mut profiler = CpuProfiler {
            isolate,
            naming_mode,
            logging_mode,
            base_sampling_interval: Duration::from_micros(1000), // Replace with v8_flags.cpu_profiler_sampling_interval
            code_observer: Box::new(test_code_observer),
            profiles: test_profiles,
            symbolizer: test_symbolizer,
            processor: test_processor,
            is_profiling: false,
            profiler_listener: None,
            profiling_scope: None,
            use_precise_sampling_: false,
            code_entries_: code_entries_.clone(),
        };
        unsafe {
            (*profiler.profiles).set_cpu_profiler(&mut profiler);
        }
        Self::get_profilers_manager().add_profiler(isolate, &profiler);

        if logging_mode == CpuProfilingLoggingMode::EagerLogging {
            profiler.enable_logging();
        }

        profiler
    }

    fn set_sampling_interval(&mut self, value: Duration) {
        //  DCHECK(!self.is_profiling_);
        self.base_sampling_interval = value;
    }

    fn set_use_precise_sampling(&mut self, value: bool) {
        // DCHECK(!self.is_profiling_);
        self.use_precise_sampling_ = value;
    }

    fn reset_profiles(&mut self) {
        self.profiles = Box::new(CpuProfilesCollection::new(self.isolate));
        unsafe { (*self.profiles).set_cpu_profiler(self) };
    }

    fn enable_logging(&mut self) {
        if self.profiling_scope.is_some() {
            return;
        }

        if self.profiler_listener.is_none() {
            self.profiler_listener = Some(Box::new(ProfilerListener::new(
                self.isolate,
                &*self.code_observer,
                self.code_observer.code_entries.clone(),
                self.code_observer.weak_code_registry.clone(),
                self.naming_mode,
            )));
        }

        if let Some(listener) = self.profiler_listener.as_mut() {
            self.profiling_scope = Some(ProfilingScope::new(self.isolate, listener.as_mut()));
        }
    }

    fn disable_logging(&mut self) {
        if self.profiling_scope.is_none() {
            return;
        }

        //  DCHECK(self.profiler_listener.is_some());
        self.profiling_scope = None;
        self.profiler_listener = None;
        self.code_observer.clear_code_map();
    }

    fn compute_sampling_interval(&mut self) -> Duration {
        unsafe { (*self.profiles).get_common_sampling_interval() }
    }

    fn adjust_sampling_interval(&mut self) {
        if self.processor.is_none() {
            return;
        }

        let base_interval = self.compute_sampling_interval();
        if let Some(processor) = self.processor.as_mut() {
            processor.set_sampling_interval(base_interval);
        }
    }

    fn collect_sample(isolate: *mut Isolate, trace_id: Option<u64>) {
        Self::get_profilers_manager().call_collect_sample(isolate, trace_id);
    }

    fn collect_sample(&mut self, trace_id: Option<u64>) {
        if let Some(processor) = &mut self.processor {
            processor.base.add_current_stack(false, trace_id);
        }
    }

    fn get_all_profilers_memory_size(isolate: *mut Isolate) -> usize {
        Self::get_profilers_manager().get_all_profilers_memory_size(isolate)
    }

    fn get_estimated_memory_usage(&self) -> usize {
        self.code_observer.get_estimated_memory_usage()
    }

    fn start_profiling(
        &mut self,
        options: CpuProfilingOptions,
        delegate: Box<dyn DiscardedSamplesDelegate>,
    ) -> CpuProfilingResult {
        self.start_profiling_with_title(None, options, delegate)
    }

    fn start_profiling_with_title(
        &mut self,
        title: Option<&str>,
        options: CpuProfilingOptions,
        delegate: Box<dyn DiscardedSamplesDelegate>,
    ) -> CpuProfilingResult {
        let result = unsafe { (*self.profiles).start_profiling(title, options, delegate) };

        // TODO(nicodubus): Revisit logic for if we want to do anything different for
        // kAlreadyStarted
        if result.status == CpuProfilingStatus::Started
            || result.status == CpuProfilingStatus::AlreadyStarted
        {
            // TRACE_EVENT0("v8", "CpuProfiler::StartProfiling");
            self.adjust_sampling_interval();
            self.start_processor_if_not_started();

            // Collect script rundown at the start of profiling if trace category is
            // turned on
            //  bool source_rundown_trace_enabled;
            //  bool source_rundown_sources_trace_enabled;
            //  TRACE_EVENT_CATEGORY_GROUP_ENABLED(
            //      TRACE_DISABLED_BY_DEFAULT("devtools.v8-source-rundown"),
            //      &source_rundown_trace_enabled);
            //  TRACE_EVENT_CATEGORY_GROUP_ENABLED(
            //      TRACE_DISABLED_BY_DEFAULT("devtools.v8-source-rundown-sources"),
            //      &source_rundown_sources_trace_enabled);
            //  if (source_rundown_trace_enabled || source_rundown_sources_trace_enabled) {
            //    Handle<WeakArrayList> script_objects = isolate_->factory()->script_list();
            //    for (int i = 0; i < script_objects->length(); i++) {
            //      if (Tagged<HeapObject> script_object;
            //          script_objects->get(i).GetHeapObjectIfWeak(&script_object)) {
            //        Tagged<Script> script(Cast<Script>(script_object));
            //        if (source_rundown_trace_enabled) {
            //          script->TraceScriptRundown();
            //        }
            //        if (source_rundown_sources_trace_enabled) {
            //          script->TraceScriptRundownSources();
            //        }
            //      }
            //    }
            //  }
        }
        result
    }

    fn start_processor_if_not_started(&mut self) {
        if self.processor.is_some() {
            self.processor.as_mut().unwrap().base.add_current_stack(false, None);
            return;
        }

        if self.profiling_scope.is_none() {
            //   DCHECK_EQ(self.logging_mode, CpuProfilingLoggingMode::LazyLogging);
            self.enable_logging();
        }

        if self.symbolizer.is_none() {
            // self.symbolizer =
            //     Some(Box::new(Symbolizer::new(self.code_observer.instruction_stream_map())));
        }

        let sampling_interval = self.compute_sampling_interval();
        // if let Some(symbolizer) = &self.symbolizer {
        self.processor = Some(SamplingEventsProcessor::new(
            self.isolate,
            std::ptr::null_mut(),
            &*self.code_observer,
            &*self.profiles,
            sampling_interval,
            self.use_precise_sampling_,
        ));
        // }

        self.is_profiling = true;

        // Enable stack sampling.
        if let Some(processor) = self.processor.as_mut() {
            processor.base.add_current_stack(false, None);
            processor.base.start_synchronously().unwrap();
        }
    }

    fn stop_profiling(&mut self, id: ProfilerId) -> Option<&CpuProfile> {
        if !self.is_profiling {
            return None;
        }

        let last_profile = unsafe { (*self.profiles).is_last_profile_left(id) };
        if last_profile {
            self.stop_processor();
        }

        let profile = unsafe { (*self.profiles).stop_profiling(id) };

        self.adjust_sampling_interval();

        //   DCHECK(self.profiling_scope.is_some());
        if last_profile && self.logging_mode == CpuProfilingLoggingMode::LazyLogging {
            self.disable_logging();
        }

        profile
    }

    fn stop_processor(&mut self) {
        self.is_profiling = false;
        if let Some(mut processor) = self.processor.take() {
            processor.base.stop_synchronously();
        }
    }

    fn get_profiles_count(&self) -> i32 {
        unsafe {
             (*self.profiles).profiles().len() as i32
        }
    }

    fn get_profile(&self, index: usize) -> Option<&CpuProfile> {
        unsafe {
            (*self.profiles).profiles().get(index).map(|profile| profile.as_ref()).map