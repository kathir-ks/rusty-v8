// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/libsampler/sampler.h (Rust module definition)
pub mod sampler {
    use std::sync::{atomic::{AtomicBool, Ordering}, Mutex, MutexGuard, PoisonError, RwLock};
    use std::collections::HashMap;
    use std::thread;
    use std::time::Duration;
    use std::sync::Arc;
    use lazy_static::lazy_static;

    extern crate libc;

    // Define macro for platform checks.  Since we can't get target_os! without a build script,
    // we'll leave platform-specific code under cfg! macros.
    #[cfg(target_os = "linux")]
    #[cfg(target_os = "android")]
    #[cfg(target_os = "macos")]
    #[cfg(target_os = "ios")]
    #[cfg(target_os = "freebsd")]
    #[cfg(target_os = "netbsd")]
    #[cfg(target_os = "openbsd")]
    #[cfg(target_os = "solaris")]
    #[cfg(target_os = "qnx")]
    #[cfg(target_os = "aix")]
    #[cfg(target_os = "zos")]
    #[cfg(target_os = "windows")]
    #[cfg(target_os = "cygwin")]
    #[cfg(target_os = "fuchsia")]
    macro_rules! use_signals {
        () => {
            true
        };
    }

    #[cfg(not(any(target_os = "linux", target_os = "android", target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", target_os = "solaris", target_os = "qnx", target_os = "aix", target_os = "zos", target_os = "windows", target_os = "cygwin", target_os = "fuchsia")))]
    macro_rules! use_signals {
        () => {
            false
        };
    }

    #[derive(Debug, Default)]
    pub struct RegisterState {
        pub pc: *mut libc::c_void,
        pub sp: *mut libc::c_void,
        pub fp: *mut libc::c_void,
        pub lr: *mut libc::c_void,
    }

    #[derive(Debug)]
    pub struct AtomicGuard<'a> {
        atomic_: &'a AtomicBool,
        is_success_: bool,
    }

    impl<'a> AtomicGuard<'a> {
        pub fn new(atomic: &'a AtomicBool, is_blocking: bool) -> Self {
            let mut is_success_ = false;
            loop {
                let expected = false;
                // We have to use the strong version here for the case where is_blocking
                // is false, and we will only attempt the exchange once.
                if atomic.compare_exchange_strong(expected, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    is_success_ = true;
                    break;
                }
                if !is_blocking {
                    break;
                }
            }
            AtomicGuard {
                atomic_: atomic,
                is_success_: is_success_,
            }
        }

        pub fn is_success(&self) -> bool {
            self.is_success_
        }
    }

    impl<'a> Drop for AtomicGuard<'a> {
        fn drop(&mut self) {
            if !self.is_success_ {
                return;
            }
            self.atomic_.store(false, Ordering::SeqCst);
        }
    }

    #[cfg(use_signals!())]
    pub struct Sampler {
        isolate_: *mut v8::Isolate,
        data_: Box<PlatformData>,
        is_active_: Arc<AtomicBool>,
        should_record_sample_: Arc<AtomicBool>,
    }

    #[cfg(not(use_signals!()))]
    pub struct Sampler {
        isolate_: *mut v8::Isolate,
        data_: Box<PlatformData>,
        is_active_: Arc<AtomicBool>,
    }

    impl Sampler {
        pub fn new(isolate: *mut v8::Isolate) -> Self {
            Sampler {
                isolate_: isolate,
                data_: Box::new(PlatformData::new()),
                is_active_: Arc::new(AtomicBool::new(false)),
                #[cfg(use_signals!())]
                should_record_sample_: Arc::new(AtomicBool::new(false)),
            }
        }

        pub fn isolate(&self) -> *mut v8::Isolate {
            self.isolate_
        }

        pub fn platform_data(&self) -> &PlatformData {
            &self.data_
        }

        pub fn is_active(&self) -> bool {
            self.is_active_.load(Ordering::SeqCst)
        }

        fn set_active(&self, active: bool) {
            self.is_active_.store(active, Ordering::SeqCst);
        }

        #[cfg(use_signals!())]
        pub fn should_record_sample(&self) -> bool {
            self.should_record_sample_.load(Ordering::SeqCst)
        }

        #[cfg(use_signals!())]
        pub fn set_should_record_sample(&self) {
            self.should_record_sample_.store(true, Ordering::SeqCst);
        }

        #[cfg(use_signals!())]
        pub fn clear_should_record_sample(&self) {
            self.should_record_sample_.store(false, Ordering::SeqCst);
        }

        pub fn start(&self) {
            if self.is_active() {
                return;
            }

            self.set_active(true);

            #[cfg(use_signals!())]
            {
                unsafe { SignalHandler::increase_sampler_count() };
                SamplerManager::instance().add_sampler(self);
            }
        }

        pub fn stop(&self) {
            #[cfg(use_signals!())]
            {
                SamplerManager::instance().remove_sampler(self);
                unsafe { SignalHandler::decrease_sampler_count() };
            }

            if !self.is_active() {
                return;
            }

            self.set_active(false);
        }

        #[cfg(use_signals!())]
        pub fn do_sample(&self) {
            unsafe {
                let guard = SignalHandler::mutex().lock().unwrap();
                if !SignalHandler::installed() {
                    drop(guard);
                    return;
                }
                self.set_should_record_sample();
                libc::pthread_kill(self.platform_data().vm_tself(), libc::SIGPROF);
                drop(guard);
                self.clear_should_record_sample();
            }
        }

        #[cfg(not(use_signals!()))]
        pub fn do_sample(&self) {
            //Platform specific sampling
            #[cfg(target_os = "windows")]
            {
                let profiled_thread = self.platform_data().profiled_thread();
                if profiled_thread.is_null() {
                    return;
                }

                const SUSPEND_FAILED: u32 = u32::MAX;
                if unsafe { libc::SuspendThread(profiled_thread) } == SUSPEND_FAILED {
                    return;
                }

                let mut context: libc::CONTEXT = unsafe {std::mem::zeroed()};
                context.ContextFlags = libc::CONTEXT_ALL as u32;

                if unsafe { libc::GetThreadContext(profiled_thread, &mut context) } != 0 {
                    let mut state = RegisterState::default();

                    #[cfg(target_arch = "x86_64")]
                    {
                        state.pc = context.Rip as *mut libc::c_void;
                        state.sp = context.Rsp as *mut libc::c_void;
                        state.fp = context.Rbp as *mut libc::c_void;
                    }

                    #[cfg(target_arch = "aarch64")]
                    {
                        state.pc = context.Pc as *mut libc::c_void;
                        state.sp = context.Sp as *mut libc::c_void;
                        state.fp = context.Fp as *mut libc::c_void;
                    }

                    #[cfg(target_arch = "x86")]
                    {
                         state.pc = context.Eip as *mut libc::c_void;
                         state.sp = context.Esp as *mut libc::c_void;
                         state.fp = context.Ebp as *mut libc::c_void;
                    }
                    self.sample_stack(&state);
                }

                unsafe { libc::ResumeThread(profiled_thread) };
            }

            #[cfg(target_os = "fuchsia")]
            {
                let profiled_thread = self.platform_data().profiled_thread();
                if profiled_thread == 0 {
                    return;
                }

                let mut suspend_token: zx::Handle = zx::Handle::invalid();
                if zx::task::suspend_token(zx::AsHandleRef::borrow(&profiled_thread), &mut suspend_token).is_err() {
                    return;
                }

                let signals = profiled_thread.wait_one(zx::Signals::SUSPENDED | zx::Signals::TERMINATED, zx::Duration::from_millis(100));

                if signals.is_err() || (signals.unwrap() & zx::Signals::SUSPENDED).is_empty() {
                    suspend_token.close();
                    return;
                }

                let mut thread_state: zx::ThreadStateGeneralRegs = unsafe {std::mem::zeroed()};

                if zx::thread::read_state(&profiled_thread, zx::ThreadStateKind::GeneralRegs, &mut thread_state).is_ok() {
                    let mut state = RegisterState::default();

                    #[cfg(target_arch = "x86_64")]
                    {
                        state.pc = thread_state.rip as *mut libc::c_void;
                        state.sp = thread_state.rsp as *mut libc::c_void;
                        state.fp = thread_state.rbp as *mut libc::c_void;
                    }

                    #[cfg(target_arch = "aarch64")]
                    {
                        state.pc = thread_state.pc as *mut libc::c_void;
                        state.sp = thread_state.sp as *mut libc::c_void;
                        state.fp = thread_state.r[29] as *mut libc::c_void;
                    }

                    self.sample_stack(&state);
                }
                suspend_token.close();
            }
        }


        pub fn sample_stack(&self, state: &RegisterState) {
            //TODO: Implement
            println!("Sampling stack at PC: {:p}, SP: {:p}, FP: {:p}", state.pc, state.sp, state.fp);
        }
    }

    #[cfg(use_signals!())]
    impl Drop for Sampler {
        fn drop(&mut self) {
            if self.is_active() {
                self.stop();
            }
        }
    }

    #[cfg(use_signals!())]
    unsafe impl Send for Sampler {}
    #[cfg(use_signals!())]
    unsafe impl Sync for Sampler {}


    #[derive(Debug)]
    pub struct PlatformData {
        #[cfg(use_signals!())]
        vm_tid_: i32,
        #[cfg(use_signals!())]
        vm_tself_: libc::pthread_t,

        #[cfg(target_os = "windows")]
        profiled_thread_: libc::HANDLE,

        #[cfg(target_os = "fuchsia")]
        profiled_thread_: zx::Handle,
    }

    impl PlatformData {
        #[cfg(use_signals!())]
        pub fn new() -> Self {
            PlatformData {
                vm_tid_: unsafe { libc::syscall(libc::SYS_gettid) as i32 },
                vm_tself_: unsafe { libc::pthread_self() },
            }
        }

        #[cfg(use_signals!())]
        pub fn vm_tid(&self) -> i32 {
            self.vm_tid_
        }

        #[cfg(use_signals!())]
        pub fn vm_tself(&self) -> libc::pthread_t {
            self.vm_tself_
        }

        #[cfg(target_os = "windows")]
        pub fn new() -> Self {
            let current_process = unsafe { libc::GetCurrentProcess() };
            let mut profiled_thread_: libc::HANDLE = std::ptr::null_mut();

            let result = unsafe {
                libc::DuplicateHandle(
                    current_process,
                    libc::GetCurrentThread(),
                    current_process,
                    &mut profiled_thread_,
                    libc::THREAD_GET_CONTEXT | libc::THREAD_SUSPEND_RESUME | libc::THREAD_QUERY_INFORMATION,
                    libc::FALSE,
                    0,
                )
            };

            assert!(result != 0, "DuplicateHandle failed");
            PlatformData {
                profiled_thread_: profiled_thread_,
            }
        }

        #[cfg(target_os = "windows")]
        pub fn profiled_thread(&self) -> libc::HANDLE {
            self.profiled_thread_
        }

        #[cfg(target_os = "fuchsia")]
        pub fn new() -> Self {
            let mut profiled_thread_: zx::Handle = zx::Handle::invalid();
            zx::duplicate_handle(zx::Handle::from(unsafe {zx::Thread::self_handle()}), zx::Rights::SAME_RIGHTS, &mut profiled_thread_).unwrap();
            PlatformData {
                profiled_thread_: profiled_thread_,
            }
        }

        #[cfg(target_os = "fuchsia")]
        pub fn profiled_thread(&self) -> zx::Handle {
            self.profiled_thread_
        }
    }

    #[cfg(target_os = "windows")]
    impl Drop for PlatformData {
         fn drop(&mut self) {
            if !self.profiled_thread_.is_null() {
                unsafe {
                    libc::CloseHandle(self.profiled_thread_);
                }
            }
        }
    }

    #[cfg(target_os = "fuchsia")]
    impl Drop for PlatformData {
        fn drop(&mut self) {
            if self.profiled_thread_.is_valid() {
                self.profiled_thread_.close();
            }
        }
    }

    type SamplerList = Vec<*const Sampler>;

    #[derive(Debug)]
    pub struct SamplerManager {
        sampler_map_: RwLock<HashMap<i32, SamplerList>>,
        samplers_access_counter_: AtomicBool,
    }

    impl SamplerManager {
        fn new() -> Self {
            SamplerManager {
                sampler_map_: RwLock::new(HashMap::new()),
                samplers_access_counter_: AtomicBool::new(false),
            }
        }

        fn add_sampler(&self, sampler: &Sampler) {
            let atomic_guard = AtomicGuard::new(&self.samplers_access_counter_, true);
            if !atomic_guard.is_success() {
                return;
            }
            let thread_id = sampler.platform_data().vm_tid();
            let mut sampler_map = self.sampler_map_.write().unwrap();
            match sampler_map.get_mut(&thread_id) {
                Some(samplers) => {
                    if !samplers.contains(&(sampler as *const Sampler)) {
                        samplers.push(sampler as *const Sampler);
                    }
                }
                None => {
                    let mut samplers = SamplerList::new();
                    samplers.push(sampler as *const Sampler);
                    sampler_map.insert(thread_id, samplers);
                }
            }
        }

        fn remove_sampler(&self, sampler: &Sampler) {
            let atomic_guard = AtomicGuard::new(&self.samplers_access_counter_, true);
             if !atomic_guard.is_success() {
                return;
            }
            let thread_id = sampler.platform_data().vm_tid();
            let mut sampler_map = self.sampler_map_.write().unwrap();
            if let Some(samplers) = sampler_map.get_mut(&thread_id) {
                samplers.retain(|&s| s != sampler as *const Sampler);
                if samplers.is_empty() {
                    sampler_map.remove(&thread_id);
                }
            }
        }

        fn do_sample(&self, state: &RegisterState) {
            let atomic_guard = AtomicGuard::new(&self.samplers_access_counter_, false);
            if !atomic_guard.is_success() {
                return;
            }
            let thread_id = unsafe { libc::syscall(libc::SYS_gettid) as i32 };
            let sampler_map = self.sampler_map_.read().unwrap();
            if let Some(samplers) = sampler_map.get(&thread_id) {
                for &sampler_ptr in samplers {
                    let sampler: &Sampler = unsafe { &*sampler_ptr };
                    #[cfg(use_signals!())]
                    if !sampler.should_record_sample() {
                        continue;
                    }
                    let isolate = sampler.isolate();
                    if isolate.is_null() || unsafe { (*isolate).is_in_use() != 0 } {
                        continue;
                    }
                    sampler.sample_stack(state);
                }
            }
        }

        pub fn instance() -> &'static SamplerManager {
            lazy_static! {
                static ref INSTANCE: SamplerManager = SamplerManager::new();
            }
            &INSTANCE
        }
    }

    extern "C" {
        fn v8_thread_isolated_allocator_set_default_permissions_for_signal_handler();
        fn v8_set_mutex_deadlock_detection_mode(mode: i32);
        fn v8_isolate_is_in_use(isolate: *mut v8::Isolate) -> i32;
    }

    impl Isolate {
        unsafe fn is_in_use(&self) -> i32 {
            v8_isolate_is_in_use(self)
        }
    }

    // Dummy Isolate struct to satisfy the pointer cast. This is just a simulation;
    // the actual Isolate would be defined by the v8 crate.
    #[repr(C)]
    pub struct Isolate {
        _private: [u8; 0],
    }

    #[cfg(use_signals!())]
    struct SignalHandler {}

    #[cfg(use_signals!())]
    impl SignalHandler {
        lazy_static! {
            static ref MUTEX: Mutex<()> = Mutex::new(());
            static ref CLIENT_COUNT: Mutex<i32> = Mutex::new(0);
            static ref SIGNAL_HANDLER_INSTALLED: AtomicBool = AtomicBool::new(false);
            static ref OLD_SIGNAL_HANDLER: Mutex<libc::sigaction> = Mutex::new(unsafe { std::mem::zeroed() });
        }

        unsafe fn increase_sampler_count() {
            let _lock = MUTEX.lock().unwrap();
            let mut client_count = CLIENT_COUNT.lock().unwrap();
            *client_count += 1;
            if *client_count == 1 {
                Self::install();
            }
        }

        unsafe fn decrease_sampler_count() {
            let _lock = MUTEX.lock().unwrap();
            let mut client_count = CLIENT_COUNT.lock().unwrap();
            *client_count -= 1;
            if *client_count == 0 {
                Self::restore();
            }
        }

        fn installed() -> bool {
            SIGNAL_HANDLER_INSTALLED.load(Ordering::SeqCst)
        }

        fn mutex() -> &'static Mutex<()> {
            &MUTEX
        }

        unsafe fn install() {
            let mut sa: libc::sigaction = std::mem::zeroed();
            sa.sa_sigaction = Self::handle_profiler_signal as usize as *mut _;
            libc::sigemptyset(&mut sa.sa_mask);
            sa.sa_flags = libc::SA_RESTART | libc::SA_SIGINFO | libc::SA_ONSTACK;

            let mut old_signal_handler = OLD_SIGNAL_HANDLER.lock().unwrap();
            let result = libc::sigaction(libc::SIGPROF, &sa, &mut *old_signal_handler);
            if result == 0 {
                SIGNAL_HANDLER_INSTALLED.store(true, Ordering::SeqCst);
            }
        }

        unsafe fn restore() {
            if SIGNAL_HANDLER_INSTALLED.load(Ordering::SeqCst) {
                SIGNAL_HANDLER_INSTALLED.store(false, Ordering::SeqCst);
                #[cfg(any(target_os = "aix", target_arch = "s390x"))]
                std::thread::sleep(Duration::from_micros(10));
                let old_signal_handler = OLD_SIGNAL_HANDLER.lock().unwrap();
                libc::sigaction(libc::SIGPROF, &*old_signal_handler, std::ptr::null_mut());
            }
        }

        extern "C" fn handle_profiler_signal(signal: libc::c_int, info: *mut libc::siginfo_t, context: *mut libc::c_void) {
            unsafe {
                v8_thread_isolated_allocator_set_default_permissions_for_signal_handler();
            }
            if signal != libc::SIGPROF {
                return;
            }

            let mut state = RegisterState::default();
            Self::fill_register_state(context, &mut state);
            SamplerManager::instance().do_sample(&state);
        }

        #[cfg(target_os = "linux")]
        #[cfg(target_os = "android")]
        unsafe fn fill_register_state(context: *mut libc::c_void, state: &mut RegisterState) {
            let ucontext = context as *mut libc::ucontext_t;
            let mcontext = &mut (*ucontext).uc_mcontext;

            #[cfg(target_arch = "x86")]
            {
                state.pc = (*mcontext).gregs[14] as *mut libc::c_void;  //REG_EIP
                state.sp = (*mcontext).gregs[7] as *mut libc::c_void;   //REG_ESP
                state.fp = (*mcontext).gregs[6] as *mut libc::c_void;   //REG_EBP
            }

            #[cfg(target_arch = "x86_64")]
            {
                state.pc = (*mcontext).gregs[16] as *mut libc::c_void; //REG_RIP
                state.sp = (*mcontext).gregs[15] as *mut libc::c_void; //REG_RSP
                state.fp = (*mcontext).gregs[10] as *mut libc::c_void; //REG_RBP
            }

            #[cfg(target_arch = "arm")]
            {
                state.pc = (*mcontext).arm_pc as *mut libc::c_void;
                state.sp = (*mcontext).arm_sp as *mut libc::c_void;
                state.fp = (*mcontext).arm_fp as *mut libc::c_void;
                state.lr = (*mcontext).arm_lr as *mut libc::c_void;
            }

            #[cfg(target_arch = "aarch64")]
            {
                state.pc = (*mcontext).pc as *mut libc::c_void;
                state.sp = (*mcontext).sp as *mut libc::c_void;
                state.fp = (*mcontext).regs[29] as *mut libc::c_void;
                state.lr = (*mcontext).regs[30] as *mut libc::c_void;
            }

            #[cfg(target_arch = "mips64")]
            {
                state.pc = (*mcontext).pc as *mut libc::c_void;
                state.sp = (*mcontext).gregs[29] as *mut libc::c_void;
                state.fp = (*mcontext).gregs[30] as *mut libc::c_void;
            }

            #[cfg(target_arch = "loongarch64")]
            {
                state.pc = (*mcontext).__pc as *mut libc::c_void;
                state.sp = (*mcontext).__gregs[3] as *mut libc::c_void;
                state.fp = (*mcontext).__gregs[22] as *mut libc::c_void;
            }

            #[cfg(target_arch = "powerpc64")]
            {
               state.pc = (*(*ucontext).uc_mcontext.regs).nip as *mut libc::c_void;
               state.sp = (*(*ucontext).uc_mcontext.regs).gpr[1] as *mut libc::c_void;
               state.fp = (*(*ucontext).uc_mcontext.regs).gpr[31] as *mut libc::c_void;
               state.lr = (*(*ucontext).uc_mcontext.regs).link as *mut libc::c_void;
            }

            #[cfg(target_arch = "s390x")]
            {
                state.pc = (*ucontext).uc_mcontext.psw.addr as *mut libc::c_void;
                state.sp = (*ucontext).uc_mcontext.gregs[15] as *mut libc::c_void;
                state.fp = (*ucontext).uc_mcontext.gregs[11] as *mut libc::c_void;
                state.lr = (*ucontext).uc_mcontext.gregs[14] as *mut libc::c_void;
            }

            #[cfg(target_arch = "riscv64")]
            {
                 state.pc = (*mcontext).__gregs[34] as *mut libc::c_void; //REG_PC
                 state.sp = (*mcontext).__gregs[2] as *mut libc::c_void;  //REG_SP
                 state.fp = (*mcontext).__gregs[8] as *mut libc::c_void;  //REG_S0/FP
                 state.lr = (*mcontext).__gregs[1] as *mut libc::c_void;  //REG_RA
            }
        }
         #[cfg(target_os = "macos")]
         #[cfg(target_os = "ios")]
         unsafe fn fill_register_state(context: *mut libc::c_void, state: &mut RegisterState) {
            let ucontext = context as *mut libc::ucontext_t;
            let mcontext = &mut (*ucontext).uc_mcontext;

            #[cfg(target_arch = "x86_64")] {
                state.pc = (*mcontext).__ss.__rip as *mut libc::c_void;
                state.sp = (*mcontext).__ss.__rsp as *mut libc::c_void;
                state.fp = (*mcontext).__ss.__rbp as *mut libc::c_void;
            }

            #[cfg(target_arch = "aarch64")] {
                use mach2::arm::*;
                state.pc = arm_thread_state64_get_pc((*mcontext).__ss) as *mut libc::c_void;
                state.sp = arm_thread_state64_get_sp((*mcontext).__ss) as *mut libc::c_void;
                state.fp = arm_thread_state64_get_fp((*mcontext).__ss) as *mut libc::c_void;
            }
         }
    }

    #[cfg(target_os = "fuchsia")]
    mod zx {
        #![allow(dead_code)]
        use std::mem;
        use std::ptr;
        use std::time::Duration;

        use bitflags::bitflags;

        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct Handle(u32);

        impl Handle {
            pub const INVALID: Self = Handle(0);

            pub fn invalid() -> Self {
                Self::INVALID
            }

            pub fn is_valid(&self) -> bool {
                self.0 != Self::INVALID.0
            }

            pub fn close(&self) {
                unsafe {
                    zx_handle_close(self.0);
                }
            }

            pub fn wait_one(&self, signals: Signals, deadline: Duration) -> Result<Signals, ()> {
                let mut observed: u32 = 0;
                let status = unsafe {
                    zx_object_wait_one(self.0, signals.bits(), duration_to_nanos(deadline), &mut observed)
                };
                if status == 0 {
                    Ok(Signals::from_bits_truncate(observed))
                } else {
                    Err(())
                }
            }
        }

        impl AsHandleRef for Handle {}

        impl Drop for Handle {
            fn drop(&mut self) {
                if self.is_valid() {
                    self.close();
                }
            }
        }

        #[derive(Copy, Clone, Debug)]
        pub struct AsHandleRefBorrowed<'a>(&'a Handle);

        impl<'a> AsHandleRefBorrowed<'a> {
            pub fn new(handle: &'a Handle) -> Self {
                Self(handle)
            }
        }

        impl<'a> AsHandleRef for AsHandleRefBorrowed<'a> {
            fn borrow(&self) -> Handle {
                self.0.clone()
            }
        }

        pub trait AsHandleRef {
            fn borrow(&self) -> Handle;
        }

        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Duration(i64);

        impl Duration {
            pub const INFINITE: Self = Duration(i64::MAX);

            pub fn from_nanos(nanos: i64) -> Self {
                Self(nanos)
            }

            pub fn from_millis(millis: i64) -> Self {
                Self(millis * 1_000_000)
            }

            pub fn from_secs(secs: i64) -> Self {
                Self(secs * 1_000_000_000)
            }

            pub fn from_minutes(minutes: i64) -> Self {
                Self(minutes * 60 * 1_000_000_000)
            }

            pub fn from_hours(hours: i64) -> Self {
                Self(hours * 60 * 60 * 1_000_000_000)
            }

            pub fn from_days(days: i64) -> Self {
                Self(days * 24 * 60 * 60 * 1_000_000_000)
            }
        }

        bitflags! {
            #[repr(transparent)]
            #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct Rights: u32 {
                const NONE = 0;
                const DUPLICATE = 1 << 0;
                const TRANSFER = 1 << 1;
                const READ = 1 << 2;
                const WRITE = 1 << 3;
                const EXECUTE = 1 << 4;
                const MAP = 1 << 5;
                const GET_PROPERTY = 1 << 6;
                const SET_PROPERTY = 1 << 7;
                const ENUMERATE = 1 << 8;
                const DESTROY = 1 << 9;
                const SET_POLICY = 1 << 10;
                const GET_POLICY = 1 << 11;
                const SIGNAL = 1 << 12;
                const WAIT = 1 << 13;
                const INSPECT = 1 << 14;
                const MANAGE_JOB = 1 << 16;
                const MANAGE_PROCESS = 1 << 17;
                const MANAGE_THREAD = 1 << 18;
                const SUSPEND = 1 << 19;
                const RESUME = 1 << 20;
                const GET_CPU_MAP = 1 << 21;
                const SET_DEBUG_REGS = 1 << 22;
                const GET_DEBUG_REGS = 1 << 23;
                const SAME_RIGHTS = 0x7fff_ffff;
            }
        }

        bitflags! {
            #[repr(transparent)]
            #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct Signals: u32 {
                const NONE = 0;
                const READABLE = 1 << 0;
                const WRITABLE = 1 << 1;
                const PEER_CLOSED = 1 << 2;
                const SIGNALED = 1 << 3;
                const THREAD_SUSPENDED = 1 << 4;
                const THREAD_RUNNING = 1 << 5;
                const THREAD_TERMINATED = 1 << 6;
                const PROCESS_TERMINATED = 1 << 7;
                const JOB_TERMINATED = 1 << 8;
                const OBJECT_OBSERVED = 1 << 9;
                const POLICY_CHANGED = 1 << 10;
                const KERNEL_OBJECT_ALL = 0x7fff_ffff;
            }
        }

        #[repr(C)]
        #[derive(Default, Debug, Copy, Clone)]
        pub struct ThreadStateGeneralRegs {
            pub