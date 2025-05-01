// src/logging/counters.rs

use std::sync::atomic::{AtomicI32, AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Duration;

// Placeholder for base::ElapsedTimer and base::TimeDelta
// Need to find suitable Rust equivalents or implementations.
#[derive(Debug, Clone, Copy)]
pub struct ElapsedTimer {
    start: std::time::Instant,
    elapsed: Option<std::time::Duration>,
}

impl ElapsedTimer {
    pub fn new() -> Self {
        ElapsedTimer {
            start: std::time::Instant::now(),
            elapsed: None,
        }
    }

    pub fn start(&mut self) {
        self.start = std::time::Instant::now();
        self.elapsed = None;
    }

    pub fn stop(&mut self) {
        self.elapsed = Some(self.start.elapsed());
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed.unwrap_or(self.start.elapsed())
    }

    pub fn is_started(&self) -> bool {
        self.elapsed.is_none()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimeDelta {
    duration: Duration,
}

impl TimeDelta {
    pub fn from_micros(micros: i64) -> Self {
        TimeDelta {
            duration: Duration::from_micros(micros as u64),
        }
    }
    pub fn from_millis(millis: i64) -> Self {
        TimeDelta {
            duration: Duration::from_millis(millis as u64),
        }
    }
    pub fn as_micros(&self) -> i64 {
        self.duration.as_micros() as i64
    }
    pub fn as_millis(&self) -> i64 {
        self.duration.as_millis() as i64
    }
    pub fn max() -> Self {
        TimeDelta {
            duration: Duration::from_secs(i64::MAX as u64),
        }
    }
}

// Placeholder for builtins-definitions.h, execution/isolate.h, execution/thread-id.h, logging/log-inl.h and logging/log.h
// These would likely be separate Rust modules.

pub type CounterLookupCallback = Box<dyn Fn(&str) -> Option<*mut i32> + Send + Sync>;
pub type CreateHistogramCallback = Box<dyn Fn(&str, i32, i32, i32) -> *mut std::ffi::c_void + Send + Sync>;
pub type AddHistogramSampleCallback = Box<dyn Fn(*mut std::ffi::c_void, i32) + Send + Sync>;

pub struct StatsTable {
    lookup_function_: Option<CounterLookupCallback>,
    create_histogram_function_: Option<CreateHistogramCallback>,
    add_histogram_sample_function_: Option<AddHistogramSampleCallback>,
}

impl StatsTable {
    pub fn new() -> Self {
        StatsTable {
            lookup_function_: None,
            create_histogram_function_: None,
            add_histogram_sample_function_: None,
        }
    }

    pub fn set_counter_function(&mut self, f: CounterLookupCallback) {
        self.lookup_function_ = Some(f);
    }

    pub fn set_create_histogram_function(&mut self, f: CreateHistogramCallback) {
        self.create_histogram_function_ = Some(f);
    }

    pub fn set_add_histogram_sample_function(&mut self, f: AddHistogramSampleCallback) {
        self.add_histogram_sample_function_ = Some(f);
    }

    pub fn lookup_function(&self) -> &Option<CounterLookupCallback> {
        &self.lookup_function_
    }

    pub fn create_histogram_function(&self) -> &Option<CreateHistogramCallback> {
        &self.create_histogram_function_
    }

    pub fn add_histogram_sample_function(&self) -> &Option<AddHistogramSampleCallback> {
        &self.add_histogram_sample_function_
    }
}

static UNUSED_COUNTER_DUMP: AtomicI32 = AtomicI32::new(0);

pub struct StatsCounter {
    ptr_: AtomicPtr<i32>,
    name_: &'static str,
    counters_: *mut Counters,
}

impl StatsCounter {
    pub fn new(name: &'static str, counters_: *mut Counters) -> Self {
        StatsCounter {
            ptr_: AtomicPtr::new(&UNUSED_COUNTER_DUMP as *const AtomicI32 as *mut i32),
            name_: name,
            counters_: counters_,
        }
    }

    pub fn enabled(&self) -> bool {
        self.ptr_.load(Ordering::Relaxed) as *const AtomicI32 != &UNUSED_COUNTER_DUMP as *const AtomicI32
    }

    pub fn setup_ptr_from_stats_table(&self) -> *mut AtomicI32 {
        // {Init} must have been called.
        assert!(!self.counters_.is_null());
        assert!(!self.name_.is_empty());

        let counters = unsafe { &*self.counters_ };
        let location = counters.find_location(self.name_);

        let ptr: *mut AtomicI32 = match location {
            Some(loc) => loc as *mut i32 as *mut AtomicI32,
            None => &UNUSED_COUNTER_DUMP as *const AtomicI32 as *mut AtomicI32,
        };

        self.ptr_.store(ptr as *mut i32, Ordering::Release);

        ptr
    }

    pub fn reset(&self) {
        if self.enabled() {
            let ptr = self.ptr_.load(Ordering::Relaxed);
            if !ptr.is_null() {
                unsafe { *ptr = 0 };
            }
        }
    }
}

#[derive(Debug)]
pub struct Histogram {
    name_: &'static str,
    min_: i32,
    max_: i32,
    num_buckets_: i32,
    histogram_: *mut std::ffi::c_void,
    counters_: *mut Counters,
}

impl Histogram {
    pub fn new(name_: &'static str, min_: i32, max_: i32, num_buckets_: i32, counters_: *mut Counters) -> Self {
        Histogram {
            name_: name_,
            min_: min_,
            max_: max_,
            num_buckets_: num_buckets_,
            histogram_: std::ptr::null_mut(),
            counters_: counters_,
        }
    }

    pub fn initialize(&mut self, name_: &'static str, min_: i32, max_: i32, num_buckets_: i32, counters_: *mut Counters) {
        self.name_ = name_;
        self.min_ = min_;
        self.max_ = max_;
        self.num_buckets_ = num_buckets_;
        self.counters_ = counters_;
    }

    pub fn enabled(&self) -> bool {
        !self.counters_.is_null()
    }

    pub fn add_sample(&self, sample: i32) {
        if self.enabled() {
            let counters = unsafe { &*self.counters_ };
            counters.add_histogram_sample(self.histogram_, sample);
        }
    }

    pub fn create_histogram(&mut self) -> *mut std::ffi::c_void {
        let counters = unsafe { &*self.counters_ };
        self.histogram_ = counters.create_histogram(self.name_, self.min_, self.max_, self.num_buckets_);
        self.histogram_
    }

    pub fn reset(&self) {
        //Need implementation if reset functionality is required for Histogram
    }
}

#[derive(Debug)]
pub struct TimedHistogram {
    histogram: Histogram,
    resolution_: TimedHistogramResolution,
}

#[derive(Debug, Clone, Copy)]
pub enum TimedHistogramResolution {
    MICROSECOND,
    MILLISECOND,
}

impl TimedHistogram {
    pub fn new(name: &'static str, min: i32, max: i32, resolution: TimedHistogramResolution, num_buckets: i32, counters: *mut Counters) -> Self {
        TimedHistogram {
            histogram: Histogram::new(name, min, max, num_buckets, counters),
            resolution_: resolution,
        }
    }
    pub fn initialize(&mut self, name_: &'static str, min_: i32, max_: i32, res: TimedHistogramResolution, num_buckets_: i32, counters_: *mut Counters) {
      self.histogram.initialize(name_, min_, max_, num_buckets_, counters_);
      self.resolution_ = res;
    }

    pub fn enabled(&self) -> bool {
        self.histogram.enabled()
    }

    pub fn stop(&mut self, timer: &mut ElapsedTimer) {
        assert!(self.enabled());
        self.add_timed_sample(timer.elapsed());
        timer.stop();
    }

    fn add_timed_sample(&self, sample: Duration) {
        if self.enabled() {
            let sample_int: i64 = match self.resolution_ {
                TimedHistogramResolution::MICROSECOND => sample.as_micros() as i64,
                TimedHistogramResolution::MILLISECOND => sample.as_millis() as i64,
            };
            self.histogram.add_sample(sample_int as i32);
        }
    }

    //Placeholder. Requires V8FileLogger and LogEventStatus.
    pub fn record_abandon(&self, timer: &mut ElapsedTimer, isolate: *mut Isolate) {
        if self.enabled() {
            assert!(timer.is_started());
            timer.stop();
            let sample: i64 = match self.resolution_ {
                TimedHistogramResolution::MICROSECOND => TimeDelta::max().as_micros(),
                TimedHistogramResolution::MILLISECOND => TimeDelta::max().as_millis(),
            };
            self.histogram.add_sample(sample as i32);
        }
        if !isolate.is_null() {
            //V8FileLogger::CallEventLogger(isolate, self.histogram.name_(), v8::LogEventStatus::kEnd, true);
            //Placeholder: Need to implement V8FileLogger and LogEventStatus equivalent
        }
    }
}

#[derive(Debug)]
pub struct NestedTimedHistogram {
    histogram: TimedHistogram,
}

impl NestedTimedHistogram {
    pub fn new(name: &'static str, max: i32, resolution: TimedHistogramResolution, counters: *mut Counters) -> Self {
        NestedTimedHistogram {
            histogram: TimedHistogram::new(name, 0, max, resolution, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, counters),
        }
    }
    pub fn initialize(&mut self, name_: &'static str, min_: i32, max_: i32, res: TimedHistogramResolution, num_buckets_: i32, counters_: *mut Counters) {
        self.histogram.initialize(name_, min_, max_, res, num_buckets_, counters_);
      }
}

#[derive(Debug)]
pub struct AggregatableHistogramTimer {
    histogram: Histogram,
}

impl AggregatableHistogramTimer {
    pub fn new(name: &'static str, counters: *mut Counters) -> Self {
        AggregatableHistogramTimer {
            histogram: Histogram::new(name, 0, 10000000, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, counters),
        }
    }

    pub fn initialize(&mut self, name_: &'static str, min_: i32, max_: i32, num_buckets_: i32, counters_: *mut Counters) {
        self.histogram.initialize(name_, min_, max_, num_buckets_, counters_);
    }
}

#[derive(Debug)]
pub struct PercentageHistogram {
    histogram: Histogram,
}

impl PercentageHistogram {
    pub fn new(name: &'static str, counters: *mut Counters) -> Self {
        PercentageHistogram {
            histogram: Histogram::new(name, 0, 101, 100, counters),
        }
    }

    pub fn initialize(&mut self, name_: &'static str, min_: i32, max_: i32, num_buckets_: i32, counters_: *mut Counters) {
        self.histogram.initialize(name_, min_, max_, num_buckets_, counters_);
    }
}

#[derive(Debug)]
pub struct LegacyMemoryHistogram {
    histogram: Histogram,
}

impl LegacyMemoryHistogram {
    pub fn new(name: &'static str, counters: *mut Counters) -> Self {
        // Exponential histogram assigns bucket limits to points
        // p[1], p[2], ... p[n] such that p[i+1] / p[i] = constant.
        // The constant factor is equal to the n-th root of (high / low),
        // where the n is the number of buckets, the low is the lower limit,
        // the high is the upper limit.
        // For n = 50, low = 1000, high = 500000: the factor = 1.13.
        LegacyMemoryHistogram {
            histogram: Histogram::new(name, 1000, 500000, 50, counters),
        }
    }
    pub fn initialize(&mut self, name_: &'static str, min_: i32, max_: i32, num_buckets_: i32, counters_: *mut Counters) {
        self.histogram.initialize(name_, min_, max_, num_buckets_, counters_);
    }
}

const DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS: i32 = 50;

pub struct CountersInitializer<'a> {
    counters_: *mut Counters,
    //PhantomData: PhantomData<&'a Counters> // To tie lifetime to Counters
}

impl<'a> CountersInitializer<'a> {
    pub fn new(counters_: *mut Counters) -> Self {
        CountersInitializer {
            counters_: counters_,
            //PhantomData
        }
    }

    pub fn counters(&self) -> *mut Counters {
        self.counters_
    }

    pub fn visit_histogram(&self, histogram: &mut Histogram, caption: &'static str, min: i32, max: i32, num_buckets: i32) {
        histogram.initialize(caption, min, max, num_buckets, self.counters_);
    }

    pub fn visit_percentage_histogram(&self, histogram: &mut PercentageHistogram, caption: &'static str) {
        histogram.initialize(caption, 0, 101, 100, self.counters_);
    }

    pub fn visit_legacy_memory_histogram(&self, histogram: &mut LegacyMemoryHistogram, caption: &'static str) {
        histogram.initialize(caption, 1000, 500000, 50, self.counters_);
    }

    pub fn visit_timed_histogram(&self, histogram: &mut TimedHistogram, caption: &'static str, max: i32, res: TimedHistogramResolution) {
        histogram.initialize(caption, 0, max, res, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, self.counters_);
    }

    pub fn visit_nested_timed_histogram(&self, histogram: &mut NestedTimedHistogram, caption: &'static str, max: i32, res: TimedHistogramResolution) {
        histogram.initialize(caption, 0, max, res, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, self.counters_);
    }

    pub fn visit_aggregatable_histogram_timer(&self, histogram: &mut AggregatableHistogramTimer, caption: &'static str) {
        histogram.initialize(caption, 0, 10000000, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, self.counters_);
    }

    pub fn visit_stats_counter(&self, counter: &mut StatsCounter, caption: &'static str) {
        counter.name_ = caption;
        counter.counters_ = self.counters_;
        counter.setup_ptr_from_stats_table();
    }

    pub fn start(&self) {
        let counters = unsafe { &mut *self.counters_ };
        counters.initialize_counters(self);
    }
}

pub struct StatsCounterResetter<'a> {
  counters_: *mut Counters,
  //PhantomData: PhantomData<&'a Counters>
}

impl<'a> StatsCounterResetter<'a> {
    pub fn new(counters_: *mut Counters) -> Self {
        StatsCounterResetter {
            counters_: counters_,
            //PhantomData,
        }
    }

    pub fn visit_stats_counter(&self, counter: &mut StatsCounter, caption: &'static str) {
        counter.reset();
    }

    pub fn start(&self) {
      let counters = unsafe { &mut *self.counters_ };
        counters.reset_counters(self);
    }
}

pub struct HistogramResetter<'a> {
  counters_: *mut Counters,
  //PhantomData: PhantomData<&'a Counters>
}

impl<'a> HistogramResetter<'a> {
    pub fn new(counters_: *mut Counters) -> Self {
        HistogramResetter {
            counters_: counters_,
            //PhantomData,
        }
    }

    pub fn visit_histogram(&self, histogram: &mut Histogram, caption: &'static str) {
        histogram.reset();
    }

    pub fn start(&self) {
        let counters = unsafe { &mut *self.counters_ };
        counters.reset_histograms(self);
    }
}

pub struct CountersVisitor<'a> {
    counters_: *mut Counters,
    //PhantomData: PhantomData<&'a Counters>
}

impl<'a> CountersVisitor<'a> {
    pub fn new(counters_: *mut Counters) -> Self {
        CountersVisitor {
            counters_: counters_,
            //PhantomData,
        }
    }

    pub fn counters(&self) -> *mut Counters {
        self.counters_
    }

    pub fn visit(&self, histogram: &mut Histogram, caption: &'static str, min: i32, max: i32, num_buckets: i32) {
        self.visit_histogram(histogram, caption);
    }
    pub fn visit(&self, histogram: &mut TimedHistogram, caption: &'static str, max: i32, res: TimedHistogramResolution) {
        self.visit_histogram(histogram, caption);
    }
    pub fn visit(&self, histogram: &mut NestedTimedHistogram, caption: &'static str, max: i32, res: TimedHistogramResolution) {
        self.visit_histogram(histogram, caption);
    }
    pub fn visit(&self, histogram: &mut AggregatableHistogramTimer, caption: &'static str) {
        self.visit_histogram(histogram, caption);
    }
    pub fn visit(&self, histogram: &mut PercentageHistogram, caption: &'static str) {
        self.visit_histogram(histogram, caption);
    }
    pub fn visit(&self, histogram: &mut LegacyMemoryHistogram, caption: &'static str) {
        self.visit_histogram(histogram, caption);
    }
    pub fn visit(&self, counter: &mut StatsCounter, caption: &'static str) {
        self.visit_stats_counter(counter, caption);
    }

    fn visit_histograms(&self, counters: &mut Counters) {
        counters.histogram_range_list.iter_mut().for_each(|(name, caption, min, max, num_buckets, histogram)| {
            histogram.initialize(caption, *min, *max, *num_buckets, self.counters_);
        });

        counters.histogram_range_list_slow.iter_mut().for_each(|(name, caption, min, max, num_buckets, histogram)| {
            histogram.initialize(caption, *min, *max, *num_buckets, self.counters_);
        });

        counters.histogram_percentage_list.iter_mut().for_each(|(name, caption, histogram)| {
            histogram.initialize(caption, 0, 101, 100, self.counters_);
        });

        counters.histogram_legacy_memory_list.iter_mut().for_each(|(name, caption, histogram)| {
            histogram.initialize(caption, 1000, 500000, 50, self.counters_);
        });

        counters.nested_timed_histogram_list.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            histogram.initialize(caption, 0, *max, *res, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, self.counters_);
        });

        counters.nested_timed_histogram_list_slow.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            histogram.initialize(caption, 0, *max, *res, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, self.counters_);
        });

        counters.timed_histogram_list.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            histogram.initialize(caption, 0, *max, *res, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, self.counters_);
        });

        counters.aggregatable_histogram_timer_list.iter_mut().for_each(|(name, caption, histogram)| {
            histogram.initialize(caption, 0, 10000000, DEFAULT_TIMED_HISTOGRAM_NUM_BUCKETS, self.counters_);
        });
    }

    fn visit_stats_counters(&self, counters: &mut Counters) {
        counters.stats_counter_list.iter_mut().for_each(|(name, caption, counter)| {
            counter.name_ = caption;
            counter.counters_ = self.counters_;
            counter.setup_ptr_from_stats_table();
        });

        counters.stats_counter_native_code_list.iter_mut().for_each(|(name, caption, counter)| {
            counter.name_ = caption;
            counter.counters_ = self.counters_;
            counter.setup_ptr_from_stats_table();
        });
    }

    pub fn visit_histogram(&self, histogram: &mut Histogram, caption: &'static str) {
    }

    pub fn visit_stats_counter(&self, counter: &mut StatsCounter, caption: &'static str) {
    }

    pub fn start(&self) {
        let counters = unsafe { &mut *self.counters_ };
        self.visit_stats_counters(counters);
        self.visit_histograms(counters);
    }
}

// Placeholder for Isolate.
#[derive(Debug)]
pub struct Isolate {}

#[derive(Debug)]
pub struct Counters {
    stats_table_: Arc<Mutex<StatsTable>>,
    counter_values: Arc<Mutex<HashMap<String, i32>>>,
    isolate_: *mut Isolate,

    // Lists of histograms and counters
    histogram_range_list: Vec<(&'static str, &'static str, i32, i32, i32, Histogram)>,
    histogram_range_list_slow: Vec<(&'static str, &'static str, i32, i32, i32, Histogram)>,
    histogram_percentage_list: Vec<(&'static str, &'static str, PercentageHistogram)>,
    histogram_legacy_memory_list: Vec<(&'static str, &'static str, LegacyMemoryHistogram)>,
    nested_timed_histogram_list: Vec<(&'static str, &'static str, i32, TimedHistogramResolution, NestedTimedHistogram)>,
    nested_timed_histogram_list_slow: Vec<(&'static str, &'static str, i32, TimedHistogramResolution, NestedTimedHistogram)>,
    timed_histogram_list: Vec<(&'static str, &'static str, i32, TimedHistogramResolution, TimedHistogram)>,
    aggregatable_histogram_timer_list: Vec<(&'static str, &'static str, AggregatableHistogramTimer)>,
    stats_counter_list: Vec<(&'static str, &'static str, StatsCounter)>,
    stats_counter_native_code_list: Vec<(&'static str, &'static str, StatsCounter)>,
}

impl Counters {
    pub fn new(isolate: *mut Isolate) -> Self {
        let stats_table_ = Arc::new(Mutex::new(StatsTable::new()));
        let counter_values = Arc::new(Mutex::new(HashMap::new()));

        let mut counters = Counters {
            stats_table_: stats_table_.clone(),
            counter_values: counter_values.clone(),
            isolate_: isolate,
            histogram_range_list: Vec::new(),
            histogram_range_list_slow: Vec::new(),
            histogram_percentage_list: Vec::new(),
            histogram_legacy_memory_list: Vec::new(),
            nested_timed_histogram_list: Vec::new(),
            nested_timed_histogram_list_slow: Vec::new(),
            timed_histogram_list: Vec::new(),
            aggregatable_histogram_timer_list: Vec::new(),
            stats_counter_list: Vec::new(),
            stats_counter_native_code_list: Vec::new(),
        };

        counters
    }

    fn initialize_counters(&mut self, init: &CountersInitializer) {
        // Initialize StatsCounters
        self.stats_counter_list.iter_mut().for_each(|(name, caption, counter)| {
            init.visit_stats_counter(counter, caption);
        });

        self.stats_counter_native_code_list.iter_mut().for_each(|(name, caption, counter)| {
            init.visit_stats_counter(counter, caption);
        });

        //Initialize Histograms
        self.histogram_range_list.iter_mut().for_each(|(name, caption, min, max, num_buckets, histogram)| {
            init.visit_histogram(histogram, caption, *min, *max, *num_buckets);
        });

        self.histogram_range_list_slow.iter_mut().for_each(|(name, caption, min, max, num_buckets, histogram)| {
            init.visit_histogram(histogram, caption, *min, *max, *num_buckets);
        });

        self.histogram_percentage_list.iter_mut().for_each(|(name, caption, histogram)| {
            init.visit_percentage_histogram(histogram, caption);
        });

        self.histogram_legacy_memory_list.iter_mut().for_each(|(name, caption, histogram)| {
            init.visit_legacy_memory_histogram(histogram, caption);
        });

        self.nested_timed_histogram_list.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            init.visit_nested_timed_histogram(histogram, caption, *max, *res);
        });

        self.nested_timed_histogram_list_slow.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            init.visit_nested_timed_histogram(histogram, caption, *max, *res);
        });

        self.timed_histogram_list.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            init.visit_timed_histogram(histogram, caption, *max, *res);
        });

        self.aggregatable_histogram_timer_list.iter_mut().for_each(|(name, caption, histogram)| {
            init.visit_aggregatable_histogram_timer(histogram, caption);
        });
    }

    fn reset_counters(&mut self, resetter: &StatsCounterResetter) {
        self.stats_counter_list.iter_mut().for_each(|(name, caption, counter)| {
            resetter.visit_stats_counter(counter, caption);
        });

        self.stats_counter_native_code_list.iter_mut().for_each(|(name, caption, counter)| {
            resetter.visit_stats_counter(counter, caption);
        });
    }

    fn reset_histograms(&mut self, resetter: &HistogramResetter) {
        self.histogram_range_list.iter_mut().for_each(|(name, caption, min, max, num_buckets, histogram)| {
            resetter.visit_histogram(histogram, caption);
        });

        self.histogram_range_list_slow.iter_mut().for_each(|(name, caption, min, max, num_buckets, histogram)| {
            resetter.visit_histogram(histogram, caption);
        });

        self.histogram_percentage_list.iter_mut().for_each(|(name, caption, histogram)| {
            resetter.visit_histogram(histogram, caption);
        });

        self.histogram_legacy_memory_list.iter_mut().for_each(|(name, caption, histogram)| {
            resetter.visit_histogram(histogram, caption);
        });

        self.nested_timed_histogram_list.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            resetter.visit_histogram(histogram.histogram.histogram, caption);
        });

        self.nested_timed_histogram_list_slow.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            resetter.visit_histogram(histogram.histogram.histogram, caption);
        });

        self.timed_histogram_list.iter_mut().for_each(|(name, caption, max, res, histogram)| {
            resetter.visit_histogram(histogram.histogram, caption);
        });

        self.aggregatable_histogram_timer_list.iter_mut().for_each(|(name, caption, histogram)| {
            resetter.visit_histogram(histogram.histogram, caption);
        });
    }

    pub fn stats_table(&self) -> Arc<Mutex<StatsTable>> {
        self.stats_table_.clone()
    }

    pub fn find_location(&self, name: &str) -> Option<*mut i32> {
        let stats_table = self.stats_table_.lock().unwrap();
        match &stats_table.lookup_function() {
            Some(lookup_function) => lookup_function(name),
            None => None,
        }
    }

    pub fn create_histogram(&self, name: &str, min: i32, max: i32, num_buckets: i32) -> *mut std::ffi::c_void {
        let stats_table = self.stats_table_.lock().unwrap();
        match &stats_table.create_histogram_function() {
            Some(create_histogram_function) => create_histogram_function(name, min, max, num_buckets),
            None => std::ptr::null_mut(),
        }
    }

    pub fn add_histogram_sample(&self, histogram: *mut std::ffi::c_void, sample: i32) {
        let stats_table = self.stats_table_.lock().unwrap();
        match &stats_table.add_histogram_sample_function() {
            Some(add_histogram_sample_function) => add_histogram_sample_function(histogram, sample),
            None => (),
        }
    }

    pub fn reset_counter_function(&mut self, f: CounterLookupCallback) {
        let mut stats_table = self.stats_table_.lock().unwrap();
        stats_table.set_counter_function(f);
        let resetter = StatsCounterResetter::new(self);
        resetter.start();
    }

    pub fn reset_create_histogram_function(&mut self, f: CreateHistogramCallback) {
        let mut stats_table = self.stats_table_.lock().unwrap();
        stats_table.set_create_histogram_function(f);
        let resetter = HistogramResetter::new(self);
        resetter.start();
    }
}

// Macros
macro_rules! histogram_range_list {
    ($($name:ident, $caption:literal, $min:expr, $max:expr, $num_buckets:expr);*) => {
        $(
            pub fn $name(&mut self, counters: *mut Counters) {
                self.histogram_range_list.push((stringify!($name), $caption, $min, $max, $num_buckets, Histogram::new($caption, $min, $max, $num_buckets, counters)));
            }
        )*
    };
}

macro_rules! histogram_range_list_slow {
    ($($name:ident, $caption:literal, $min:expr, $max:expr, $num_buckets:expr);*) => {
        $(
            pub fn $name(&mut self, counters: *mut Counters) {
                self.histogram_range_list_slow.push((stringify!($name), $caption, $min, $max, $num_buckets, Histogram::new($caption, $min, $max, $num_buckets, counters)));
            }
        )*
    };
}

macro_rules! histogram_percentage_list {
    ($($name:ident, $caption:literal);*) => {
        $(
            pub fn $name(&mut self, counters: *mut Counters) {
                self.histogram_percentage_list.push((stringify!($name), $caption, PercentageHistogram::new($caption, counters)));
            }
        )*
    };
}

macro_rules! histogram_legacy_memory_list {
    ($($name:ident, $caption:literal);*) => {
        $(
            pub fn $name(&mut self, counters: *mut Counters) {
                self.histogram_legacy_memory_list.push((stringify!($name), $caption, LegacyMemoryHistogram::new($caption, counters)));
            }
        )*
    };
}

macro_rules! nested_timed_histogram_list {
    ($($name:ident, $caption:literal, $max:expr, $res:ident);*) => {
        $(
            pub fn $name(&mut self, counters: *mut Counters) {
                self.nested_timed_histogram_list.push((stringify!($name), $caption, $max, TimedHistogramResolution::$res, NestedTimedHistogram::new($caption, $