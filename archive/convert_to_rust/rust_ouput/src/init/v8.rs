// Converted from V8 C++ source files:
// Header: v8.h
// Implementation: v8.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
    pub struct OOMDetails {
        pub is_heap_oom: bool,
        pub detail: *const i8,
    }

    pub trait Platform {
        fn GetStackTracePrinter(&self) -> Option<fn(usize, *const usize)>;
        fn GetThreadIsolatedAllocator(&self) -> *mut internal::ThreadIsolatedAllocator;
    }

    pub struct StartupData {
        pub data: *const u8,
        pub raw_size: i32,
    }

    pub mod internal {
        use std::sync::atomic::{AtomicPtr, Ordering};
        use std::sync::{Once, Mutex};

        pub struct Isolate {
            // Add fields as needed, based on the C++ Isolate class.
        }
        impl Isolate{
            pub fn GetTurboCfgFileName(arg: *mut Isolate) -> String{
                return String::from("turbocfg");
            }
        }

        pub struct V8 {}

        impl V8 {
            pub fn Initialize() {
                V8::initialize()
            }
            pub fn Dispose() {
                V8::dispose()
            }
            pub fn FatalProcessOutOfMemory(
                isolate: *mut Isolate,
                location: *const i8,
                details: &OOMDetails,
            ) -> ! {
                unsafe {
                    let location_str = std::ffi::CStr::from_ptr(location).to_str().unwrap();
                    eprintln!("FATAL: Out of memory in {}!", location_str);
                    eprintln!("Heap OOM: {}", details.is_heap_oom);
                    if !details.detail.is_null() {
                        let detail_str = std::ffi::CStr::from_ptr(details.detail).to_str().unwrap();
                        eprintln!("Detail: {}", detail_str);
                    }
                }
                std::process::abort();
            }
            pub fn FatalProcessOutOfMemoryDetail(
                isolate: *mut Isolate,
                location: *const i8,
                detail: *const i8,
            ) -> ! {
                unsafe {
                    let location_str = std::ffi::CStr::from_ptr(location).to_str().unwrap();
                    let detail_str = std::ffi::CStr::from_ptr(detail).to_str().unwrap();
                    eprintln!("FATAL: Out of memory in {}!", location_str);
                    eprintln!("Detail: {}", detail_str);
                }
                std::process::abort();
            }
            pub const kNoOOMDetails: OOMDetails = OOMDetails {
                is_heap_oom: false,
                detail: std::ptr::null(),
            };
            pub const kHeapOOM: OOMDetails = OOMDetails {
                is_heap_oom: true,
                detail: std::ptr::null(),
            };

            pub fn InitializePlatform(platform: *mut dyn super::Platform) {
                V8::initialize_platform(platform)
            }
            pub fn InitializePlatformForTesting(platform: *mut dyn super::Platform) {
                V8::initialize_platform_for_testing(platform)
            }

            pub fn DisposePlatform() {
                V8::dispose_platform()
            }
            pub fn GetCurrentPlatform() -> *mut dyn super::Platform {
                V8::get_current_platform()
            }
            pub fn SetPlatformForTesting(platform: *mut dyn super::Platform) {
                V8::set_platform_for_testing(platform)
            }
            pub fn SetSnapshotBlob(snapshot_blob: *mut super::StartupData) {
                V8::set_snapshot_blob(snapshot_blob)
            }

            static_accessor!(platform, AtomicPtr<dyn super::Platform>, Ordering::Relaxed, *mut dyn super::Platform);

            static_initializer!(initialize_platform, |platform: *mut dyn super::Platform| {
                println!("V8::InitializePlatform");
                if V8::platform().load(Ordering::Relaxed).is_null() {
                    V8::platform().store(platform, Ordering::Relaxed);
                    unsafe{
                        v8::base::SetPrintStackTrace(platform.as_ref().unwrap().GetStackTracePrinter());
                    }
                    v8::tracing::TracingCategoryObserver::SetUp();
                    CppHeap::InitializeOncePerProcess();
                    return Ok(());
                }
                Err("Platform already initialized".to_string())
            });
            static_initializer!(initialize_platform_for_testing, |platform: *mut dyn super::Platform| {
                println!("V8::InitializePlatformForTesting");
                if V8::state().load(Ordering::Relaxed) != V8StartupState::kIdle as i32 {
                    return Err("The platform was initialized before. Note that running multiple tests in the same process is not supported.".to_string());
                }
                V8::initialize_platform(platform)
            });
            static_initializer!(initialize, || {
                println!("V8::Initialize");
                FlagList::EnforceFlagImplications();
                FlagList::Hash();
                if v8_flags::trace_turbo() {
                    std::fs::File::create(Isolate::GetTurboCfgFileName(std::ptr::null_mut())).unwrap();
                }
                if v8_flags::interpreted_frames_native_stack() && v8_flags::jitless() {
                    return Err("The --jitless and --interpreted-frames-native-stack flags are incompatible since the latter requires code generation while the former prohibits code generation.".to_string());
                }
                let abort_mode = if v8_flags::sandbox_fuzzing() || v8_flags::hole_fuzzing() {
                    base::AbortMode::kExitWithFailureAndIgnoreDcheckFailures
                } else if v8_flags::sandbox_testing() {
                    base::AbortMode::kExitWithSuccessAndIgnoreDcheckFailures
                } else if v8_flags::hard_abort() {
                    base::AbortMode::kImmediateCrash
                } else {
                    base::AbortMode::kDefault
                };
                base::OS::Initialize(abort_mode, v8_flags::gc_fake_mmap());
                if let Some(seed) = v8_flags::random_seed() {
                    GetPlatformPageAllocator().SetRandomMmapSeed(seed);
                    GetPlatformVirtualAddressSpace().SetRandomSeed(seed);
                }
                if v8_flags::print_flag_values() {
                    FlagList::PrintValues();
                }
                ThreadIsolation::Initialize(
                    unsafe{V8::platform().load(Ordering::Relaxed).as_ref().unwrap().GetThreadIsolatedAllocator()}
                );
                Sandbox::InitializeDefaultOncePerProcess(GetPlatformVirtualAddressSpace());
                if Sandbox::current().size() != kSandboxSize {
                    return Err("Sandbox size mismatch".to_string());
                }
                if v8_flags::sandbox_testing() || v8_flags::sandbox_fuzzing() {
                    let mode = if v8_flags::sandbox_testing() {
                        SandboxTesting::Mode::kForTesting
                    } else {
                        SandboxTesting::Mode::kForFuzzing
                    };
                    SandboxTesting::Enable(mode);
                }

                IsolateGroup::InitializeOncePerProcess();
                Isolate::InitializeOncePerProcess();
                Simulator::InitializeOncePerProcess();
                CpuFeatures::Probe(false);
                ElementsAccessor::InitializeOncePerProcess();
                Bootstrapper::InitializeOncePerProcess();
                CallDescriptors::InitializeOncePerProcess();
                WasmEngine::InitializeOncePerProcess();
                ExternalReferenceTable::InitializeOncePerIsolateGroup(
                    IsolateGroup::current().external_ref_table()
                );
                return Ok(());
            });
            static_initializer!(dispose, ||{
                println!("V8::Dispose");
                WasmEngine::GlobalTearDown();
                Simulator::GlobalTearDown();
                CallDescriptors::TearDown();
                ElementsAccessor::TearDown();
                RegisteredExtension::UnregisterAll();
                FlagList::ReleaseDynamicAllocations();
                IsolateGroup::TearDownOncePerProcess();
                return Ok(());
            });
            static_initializer!(dispose_platform, ||{
                println!("V8::DisposePlatform");
                v8::tracing::TracingCategoryObserver::TearDown();
                unsafe{v8::base::SetPrintStackTrace(None)};
                Sandbox::TearDownDefault();
                V8::platform().store(std::ptr::null_mut(), Ordering::Relaxed);
                ThreadIsolation::CheckTrackedMemoryEmpty();
                return Ok(());
            });
            static_initializer!(get_current_platform, ||{
                println!("V8::GetCurrentPlatform");
                let platform = V8::platform().load(Ordering::Relaxed);
                if platform.is_null() {
                    return Err("Platform is null".to_string());
                }
                return Ok(platform);
            });
            static_initializer!(set_platform_for_testing, |platform: *mut dyn super::Platform|{
                println!("V8::SetPlatformForTesting");
                V8::platform().store(platform, Ordering::Relaxed);
                return Ok(());
            });
            static_initializer!(set_snapshot_blob, |snapshot_blob: *mut super::StartupData|{
                println!("V8::SetSnapshotBlob");
                SetSnapshotFromFile(unsafe{snapshot_blob.as_mut().unwrap()});
                return Ok(());
            });

            static_initializer!(initialize_once, || {
                println!("V8::init_snapshot_once");
                return Ok(());
            });
            static_initializer!(SetSnapshotFromFile, |snapshot_blob: *mut super::StartupData|{
                println!("V8::SetSnapshotFromFile");
                return Ok(());
            });
        }

        pub mod v8_flags {
            use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

            static_atomic!(trace_turbo, bool, Ordering::Relaxed);
            static_atomic!(interpreted_frames_native_stack, bool, Ordering::Relaxed);
            static_atomic!(jitless, bool, Ordering::Relaxed);
            static_atomic!(gc_fake_mmap, bool, Ordering::Relaxed);
            static_atomic!(random_seed, u32, Ordering::Relaxed);
            static_atomic!(print_flag_values, bool, Ordering::Relaxed);
            static_atomic!(sandbox_fuzzing, bool, Ordering::Relaxed);
            static_atomic!(hole_fuzzing, bool, Ordering::Relaxed);
            static_atomic!(sandbox_testing, bool, Ordering::Relaxed);
            static_atomic!(hard_abort, bool, Ordering::Relaxed);
            static_atomic!(freeze_flags_after_init, bool, Ordering::Relaxed);

            pub fn trace_turbo() -> bool {
                trace_turbo().load(Ordering::Relaxed)
            }
            pub fn set_trace_turbo(value: bool) {
                trace_turbo().store(value, Ordering::Relaxed)
            }
            pub fn interpreted_frames_native_stack() -> bool {
                interpreted_frames_native_stack().load(Ordering::Relaxed)
            }
            pub fn set_interpreted_frames_native_stack(value: bool) {
                interpreted_frames_native_stack().store(value, Ordering::Relaxed)
            }
            pub fn jitless() -> bool {
                jitless().load(Ordering::Relaxed)
            }
            pub fn set_jitless(value: bool) {
                jitless().store(value, Ordering::Relaxed)
            }
            pub fn gc_fake_mmap() -> bool {
                gc_fake_mmap().load(Ordering::Relaxed)
            }
            pub fn set_gc_fake_mmap(value: bool) {
                gc_fake_mmap().store(value, Ordering::Relaxed)
            }
            pub fn random_seed() -> Option<u32> {
                let value = random_seed().load(Ordering::Relaxed);
                if value == 0 {
                    None
                } else {
                    Some(value)
                }
            }
            pub fn set_random_seed(value: u32) {
                random_seed().store(value, Ordering::Relaxed)
            }
            pub fn print_flag_values() -> bool {
                print_flag_values().load(Ordering::Relaxed)
            }
            pub fn set_print_flag_values(value: bool) {
                print_flag_values().store(value, Ordering::Relaxed)
            }
            pub fn sandbox_fuzzing() -> bool {
                sandbox_fuzzing().load(Ordering::Relaxed)
            }
            pub fn set_sandbox_fuzzing(value: bool) {
                sandbox_fuzzing().store(value, Ordering::Relaxed)
            }
             pub fn hole_fuzzing() -> bool {
                hole_fuzzing().load(Ordering::Relaxed)
            }
            pub fn set_hole_fuzzing(value: bool) {
                hole_fuzzing().store(value, Ordering::Relaxed)
            }
            pub fn sandbox_testing() -> bool {
                sandbox_testing().load(Ordering::Relaxed)
            }
            pub fn set_sandbox_testing(value: bool) {
                sandbox_testing().store(value, Ordering::Relaxed)
            }
            pub fn hard_abort() -> bool {
                hard_abort().load(Ordering::Relaxed)
            }
            pub fn set_hard_abort(value: bool) {
                hard_abort().store(value, Ordering::Relaxed)
            }
            pub fn freeze_flags_after_init() -> bool {
                freeze_flags_after_init().load(Ordering::Relaxed)
            }
            pub fn set_freeze_flags_after_init(value: bool) {
                freeze_flags_after_init().store(value, Ordering::Relaxed)
            }
        }

        pub mod base {
            pub enum AbortMode {
                kDefault,
                kExitWithFailureAndIgnoreDcheckFailures,
                kExitWithSuccessAndIgnoreDcheckFailures,
                kImmediateCrash,
            }
            pub struct OS {}
            impl OS {
                pub fn Initialize(abort_mode: AbortMode, gc_fake_mmap: bool) {}
                pub fn TimeCurrentMillis() -> f64 {
                    0.0
                }
            }

            pub fn SetPrintStackTrace(f: Option<fn(usize, *const usize)>){}
        }
        pub mod tracing {
            pub struct TracingCategoryObserver {}
            impl TracingCategoryObserver {
                pub fn SetUp(){}
                pub fn TearDown(){}
            }
        }

        pub mod cppgc {
            pub struct CppHeap {}
            impl CppHeap {
                pub fn InitializeOncePerProcess(){}
            }
        }

        pub mod FlagList {
            pub fn EnforceFlagImplications(){}
            pub fn Hash(){}
            pub fn PrintValues(){}
            pub fn ReleaseDynamicAllocations(){}
            pub fn FreezeFlags(){}
        }

        pub mod ThreadIsolation {
            pub fn Initialize(arg: *mut ThreadIsolatedAllocator){}
            pub fn CheckTrackedMemoryEmpty(){}
        }

        pub mod Sandbox {
            pub struct Sandbox {}
            impl Sandbox {
                pub fn InitializeDefaultOncePerProcess(space: VirtualAddressSpace){}
                pub fn TearDownDefault(){}
                pub fn current() -> Sandbox {
                    Sandbox{}
                }
                pub fn size(&self) -> usize{
                    0
                }
            }
        }

        pub mod SandboxTesting {
            pub enum Mode {
                kForTesting,
                kForFuzzing,
            }
            pub fn Enable(mode: Mode){}
        }
        pub mod IsolateGroup {
            pub struct IsolateGroup {}
            impl IsolateGroup {
                pub fn InitializeOncePerProcess(){}
                pub fn TearDownOncePerProcess(){}
                pub fn current() -> IsolateGroup {
                    IsolateGroup{}
                }
                pub fn external_ref_table(&self) -> ExternalReferenceTable{
                    ExternalReferenceTable{}
                }
            }
        }
        pub mod Simulator {
            pub fn InitializeOncePerProcess(){}
            pub fn GlobalTearDown(){}
        }
        pub mod CpuFeatures {
            pub fn Probe(arg: bool){}
        }
        pub mod ElementsAccessor {
            pub fn InitializeOncePerProcess(){}
            pub fn TearDown(){}
        }
        pub mod Bootstrapper {
            pub fn InitializeOncePerProcess(){}
        }
        pub mod CallDescriptors {
            pub fn InitializeOncePerProcess(){}
            pub fn TearDown(){}
        }
        pub mod WasmEngine {
            pub fn InitializeOncePerProcess(){}
            pub fn GlobalTearDown(){}
        }
        pub mod RegisteredExtension {
            pub fn UnregisterAll(){}
        }
        pub struct ExternalReferenceTable {}

        pub struct VirtualAddressSpace{}
        pub fn GetPlatformVirtualAddressSpace() -> VirtualAddressSpace {
            VirtualAddressSpace{}
        }

        pub struct PageAllocator{}
        pub fn GetPlatformPageAllocator() -> PageAllocator{
            PageAllocator{}
        }
        impl PageAllocator {
            pub fn SetRandomMmapSeed(&self, seed: u32){}
        }

        pub struct ThreadIsolatedAllocator{}

        static_atomic!(state, i32, Ordering::Relaxed);

        #[derive(PartialEq, Eq)]
        pub enum V8StartupState {
            kIdle = 0,
            kPlatformInitializing,
            kPlatformInitialized,
            kV8Initializing,
            kV8Initialized,
            kV8Disposing,
            kV8Disposed,
            kPlatformDisposing,
            kPlatformDisposed,
        }

        impl V8 {
            fn initialize() -> Result<(), String> {
                V8::transition_state(V8StartupState::kV8Initializing)?;
                let result = V8::INITIALIZE.call(());
                V8::transition_state(V8StartupState::kV8Initialized)?;
                result
            }
            fn dispose() -> Result<(), String> {
                V8::transition_state(V8StartupState::kV8Disposing)?;
                 let result = V8::DISPOSE.call(());
                V8::transition_state(V8StartupState::kV8Disposed)?;
                result
            }

             fn initialize_platform(platform: *mut dyn super::Platform) -> Result<(), String> {
                 V8::transition_state(V8StartupState::kPlatformInitializing)?;
                let result = V8::INITIALIZE_PLATFORM.call(platform);
                 V8::transition_state(V8StartupState::kPlatformInitialized)?;
                result
            }

            fn initialize_platform_for_testing(platform: *mut dyn super::Platform) -> Result<(), String> {
                V8::transition_state(V8StartupState::kPlatformInitializing)?;
                let result = V8::INITIALIZE_PLATFORM_FOR_TESTING.call(platform);
                 V8::transition_state(V8StartupState::kPlatformInitialized)?;
                result
            }
             fn dispose_platform() -> Result<(), String> {
                V8::transition_state(V8StartupState::kPlatformDisposing)?;
                let result =  V8::DISPOSE_PLATFORM.call(());
                V8::transition_state(V8StartupState::kPlatformDisposed)?;
                result
            }

             fn get_current_platform() -> Result<*mut dyn super::Platform, String> {
                 V8::transition_state(V8StartupState::kV8Initialized)?;
                 let result = V8::GET_CURRENT_PLATFORM.call(());
                 V8::transition_state(V8StartupState::kV8Initialized)?;
                 result
            }

            fn set_platform_for_testing(platform: *mut dyn super::Platform) -> Result<(), String> {
                 V8::transition_state(V8StartupState::kV8Initialized)?;
                 let result =  V8::SET_PLATFORM_FOR_TESTING.call(platform);
                 V8::transition_state(V8StartupState::kV8Initialized)?;
                 result
            }
            fn set_snapshot_blob(snapshot_blob: *mut super::StartupData) -> Result<(), String> {
                V8::transition_state(V8StartupState::kV8Initialized)?;
                let result = V8::SET_SNAPSHOT_BLOB.call(snapshot_blob);
                V8::transition_state(V8StartupState::kV8Initialized)?;
                result
            }
            fn transition_state(expected_next_state: V8StartupState) -> Result<(), String> {
                let current_state = V8::state().load(Ordering::Relaxed) as V8StartupState;
                if current_state == V8StartupState::kPlatformDisposed {
                    return Err("V8 is already disposed".to_string());
                }

                let next_state = match current_state {
                    V8StartupState::kIdle => V8StartupState::kPlatformInitializing,
                    V8StartupState::kPlatformInitializing => V8StartupState::kPlatformInitialized,
                    V8StartupState::kPlatformInitialized => V8StartupState::kV8Initializing,
                    V8StartupState::kV8Initializing => V8StartupState::kV8Initialized,
                    V8StartupState::kV8Initialized => V8StartupState::kV8Disposing,
                    V8StartupState::kV8Disposing => V8StartupState::kV8Disposed,
                    V8StartupState::kV8Disposed => V8StartupState::kPlatformDisposing,
                    V8StartupState::kPlatformDisposing => V8StartupState::kPlatformDisposed,
                    V8StartupState::kPlatformDisposed => return Err("V8 is already disposed".to_string()),
                };

                if next_state != expected_next_state {
                    return Err(format!(
                        "Wrong initialization order: from {:?} to {:?}, expected to {:?}!",
                        current_state, next_state, expected_next_state
                    ));
                }

                if V8::state().compare_exchange(
                    current_state as i32,
                    next_state as i32,
                    Ordering::Strong,
                    Ordering::Relaxed,
                ).is_err() {
                    return Err(format!(
                        "Multiple threads are initializing V8 in the wrong order: expected {:?} got {:?}!",
                        current_state,
                        V8::state().load(Ordering::Relaxed)
                    ));
                }

                Ok(())
            }
        }

        macro_rules! static_atomic {
            ($name:ident, $type:ty, $ordering:expr) => {
                static $name: std::sync::atomic::Atomic<$type> = std::sync::atomic::Atomic::new(Default::default());
                impl Default for $type {
                    fn default() -> Self {
                        unsafe { std::mem::zeroed() }
                    }
                }
            };
        }

        macro_rules! static_initializer {
            ($name:ident, $body:expr) => {
                lazy_static::lazy_static! {
                    static ref $name: StaticInit<Result<(), String>> = StaticInit::new();
                }
                impl V8 {
                    #[allow(dead_code)]
                    pub fn $name() -> Result<(), String> {
                         $name.call(|| $body())
                    }
                }
            };
            ($name:ident, $args:ty, $body:expr) => {
                lazy_static::lazy_static! {
                    static ref $name: StaticInit<Result<(), String>> = StaticInit::new();
                }
                impl V8 {
                    #[allow(dead_code)]
                    pub fn $name(arg: $args) -> Result<(), String> {
                        $name.call(move || $body(arg))
                    }
                }
            };
        }

        macro_rules! static_accessor {
            ($name:ident, $type:ty, $ordering:expr, $return_type:ty) => {
                impl V8 {
                    #[allow(dead_code)]
                    pub fn $name() -> &'static $type {
                        lazy_static::lazy_static! {
                            static ref $name: $type = <$type>::new(std::ptr::null_mut());
                        }
                        & $name
                    }
                }
            };
        }

        struct StaticInit<T> {
            once: std::sync::Once,
            result: std::sync::Mutex<Option<T>>,
        }

        impl<T> StaticInit<T> {
            const fn new() -> Self {
                Self {
                    once: std::sync::Once::new(),
                    result: std::sync::Mutex::new(None),
                }
            }

            fn call<F>(&self, f: F) -> T
            where
                F: FnOnce() -> T,
                T: Clone,
            {
                let mut result = self.result.lock().unwrap();
                self.once.call_once(|| {
                    *result = Some(f());
                });

                result.as_ref().unwrap().clone()
            }
        }

        const kSandboxSize: usize = 1024;
        fn SetSnapshotFromFile(snapshot_blob: *mut super::StartupData){}
    }
    pub mod base {
        pub fn SetPrintStackTrace(arg: Option<fn(usize, *const usize)>){}
    }
    pub mod tracing {
        pub struct TracingCategoryObserver {}
        impl TracingCategoryObserver {
            pub fn SetUp(){}
            pub fn TearDown(){}
        }
    }
}

mod lazy_static {
    macro_rules! lazy_static {
        ($(#[$attr:meta])* static ref $NAME:ident : $T:ty = $e:expr ;) => {
            $(#[$attr])*
            static $NAME: lazy_static::Lazy<$T> = lazy_static::Lazy::new(|| { $e });
        };
    }

    use std::mem;
    use std::ops::Deref;
    use std::sync::{Once, ONCE_INIT};

    pub struct Lazy<T: Sync> {
        lock: Once,
        ptr: *mut T,
    }

    unsafe impl<T: Sync> Sync for Lazy<T> {}

    impl<T: Sync> Lazy<T> {
        #[inline]
        pub const fn new(f: fn() -> T) -> Lazy<T> {
            Lazy {
                lock: ONCE_INIT,
                ptr: unsafe {
                    let ptr = ::libc::malloc(mem::size_of::<T>()) as *mut T;
                    assert!(!ptr.is_null(), "Failed to allocate memory for lazy static");
                    let func = f as *mut dyn Fn() -> T;
                    ptr
                },
            }
        }
    }

    impl<T: Sync> Deref for Lazy<T> {
        type Target = T;

        fn deref(&self) -> &T {
            self.lock.call_once(|| unsafe {
                 println!("Lazy::deref");
            });

            unsafe { &*self.ptr }
        }
    }
}
