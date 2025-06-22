// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Platform-specific code for Win32.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::{c_char, c_double, c_int, c_longlong, c_uchar, c_uint, c_void};
use std::io::{Error, Result};
use std::mem::{self, size_of, zeroed, MaybeUninit};
use std::num::NonZeroU32;
use std::ops::Range;
use std::ptr::{null, null_mut};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lazy_static::lazy_static;
use winapi::ctypes::c_long;
use winapi::shared::basetsd::{DWORD64, SIZE_T, ULONG64};
use winapi::shared::minwindef::{
    BOOL, DWORD, FILETIME, HANDLE, HMODULE, LPVOID, PDWORD, PSTR, UCHAR, UINT, ULONG, ULONG_PTR,
};
use winapi::shared::ntdef::{NTSTATUS, PVOID};
use winapi::shared::winerror::{
    ERROR_COMMITMENT_LIMIT, ERROR_INVALID_HANDLE, ERROR_MOD_NOT_FOUND,
    ERROR_NOT_ENOUGH_MEMORY,
};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::heapapi::GetProcessHeap;
use winapi::um::ioapiset::CreateFileW;
use winapi::um::memoryapi::{
    MapViewOfFile, MapViewOfFileEx, UnmapViewOfFile, VirtualAlloc, VirtualFree,
    VirtualProtect, FILE_MAP_ALL_ACCESS, FILE_MAP_READ, FILE_MAP_WRITE, MEM_COMMIT,
    MEM_DECOMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE,
    MEM_RESET, GetCurrentThreadStackLimits,
};
use winapi::um::processthreadsapi::{
    GetCurrentProcess, GetCurrentThread, GetCurrentThreadId, GetThreadTimes, TerminateProcess,
};
use winapi::um::profileapi::QueryPerformanceCounter;
use winapi::um::synchapi::{
    CreateWaitableTimerExW, SetWaitableTimer, WaitForSingleObject, INFINITE,
    TIMER_MODIFY_STATE, SYNCHRONIZE,
};
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
use winapi::um::sysinfoapi::{GetSystemTimeAsFileTime, Sleep};
use winapi::um::timezoneapi::{GetTimeZoneInformation, TIME_ZONE_INFORMATION, TIME_ZONE_ID_INVALID};
use winapi::um::winbase::{
    CREATE_WAITABLE_TIMER_HIGH_RESOLUTION, MEM_COALESCE_PLACEHOLDERS,
    MEM_PRESERVE_PLACEHOLDER, MEM_REPLACE_PLACEHOLDER, MEM_RESERVE_PLACEHOLDER,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING, OPEN_ALWAYS, STD_OUTPUT_HANDLE,
    INVALID_HANDLE_VALUE as WINBASE_INVALID_HANDLE_VALUE,
};
use winapi::um::fileapi::{GetFileSize, GetTempPathA, GetTempFileNameA};
use winapi::um::consoleapi::{GetStdHandle, GetFileType};
use winapi::um::fileapi::DeleteFileA;
use winapi::um::winnt::{FILE_TYPE_UNKNOWN, GENERIC_READ, GENERIC_WRITE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE};

use winapi::um::debugapi::{OutputDebugStringA, DebugBreak};
use winapi::um::handleapi::CloseHandle as Win32CloseHandle;
use winapi::um::libraryloader::{GetModuleHandleW, GetProcAddress, LoadLibraryW};
use winapi::um::processthreadsapi::{GetCurrentProcessId, GetProcessMitigationPolicy};

use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W,
    TH32CS_SNAPMODULE,
};

use winapi::um::winnt::{
    NtCurrentTeb, NT_TIB, NT_TIB64, ProcessUserShadowStackPolicy,
    USER_CET_ENVIRONMENT_WIN32_PROCESS, PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY,
};
use winapi::um::winuser::SetErrorMode;
use winapi::um::winuser::{SEM_FAILCRITICALERRORS, SEM_NOGPFAULTERRORBOX, SEM_NOOPENFILEERRORBOX};

mod base {
    pub mod platform {
        pub mod time {
            pub fn now() -> std::time::SystemTime {
                std::time::SystemTime::now()
            }
        }
    }
}

mod src {
    pub mod base {
        pub mod platform {
            pub mod mutex {
                use std::sync::{Mutex, MutexGuard, PoisonError};
                pub struct MutexWrapper(Mutex<()>);

                impl MutexWrapper {
                    pub fn new() -> Self {
                        MutexWrapper(Mutex::new(()))
                    }

                    pub fn lock(&self) -> Result<MutexGuard<()>, PoisonError<()>> {
                        self.0.lock()
                    }
                }
            }
            pub mod platform {
                use std::sync::{Mutex, MutexGuard, PoisonError};
                pub struct ConditionVariableWrapper(std::sync::Condvar);

                impl ConditionVariableWrapper {
                    pub fn new() -> Self {
                        ConditionVariableWrapper(std::sync::Condvar::new())
                    }

                    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> Result<MutexGuard<'a, T>, PoisonError<T>> {
                        let result = self.0.wait(guard);
                        match result {
                            Ok(g) => Ok(g),
                            Err(poisoned) => Err(PoisonError::new(poisoned.into_inner())),
                        }
                    }

                    pub fn notify_one(&self) {
                        self.0.notify_one();
                    }

                    pub fn notify_all(&self) {
                        self.0.notify_all();
                    }
                }

            }
        }
    }
}

const V8_CONDITION_VARIABLE_SIZE: usize = std::mem::size_of::<std::sync::Condvar>();
const V8_CONDITION_VARIABLE_ALIGN: usize = std::mem::align_of::<std::sync::Condvar>();
const V8_SRWLOCK_SIZE: usize = std::mem::size_of::<std::sync::RwLock<()>>();
const V8_SRWLOCK_ALIGN: usize = std::mem::align_of::<std::sync::RwLock<()>>();
const V8_CRITICAL_SECTION_SIZE: usize = std::mem::size_of::<std::sync::Mutex<()>>();
const V8_CRITICAL_SECTION_ALIGN: usize = std::mem::align_of::<std::sync::Mutex<()>>();

// Define macros similar to C++

macro_rules! static_assert {
    ($cond:expr) => {
        const _: [(); 0 - !($cond) as usize] = [];
    };
}

macro_rules! CHECK {
    ($cond:expr) => {
        if !($cond) {
            panic!("Check failed: {}", stringify!($cond));
        }
    };
}

macro_rules! CHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("Check failed: {} == {}. Left: {}, Right: {}", stringify!($left), stringify!($right), $left, $right);
        }
    };
}

macro_rules! CHECK_NE {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("Check failed: {} != {}. Left: {}, Right: {}", stringify!($left), stringify!($right), $left, $right);
        }
    };
}

macro_rules! CHECK_GT {
    ($left:expr, $right:expr) => {
        if $left <= $right {
            panic!("Check failed: {} > {}. Left: {}, Right: {}", stringify!($left), stringify!($right), $left, $right);
        }
    };
}

macro_rules! DCHECK {
    ($cond:expr) => {
        if cfg!(debug_assertions) && !($cond) {
            panic!("DCheck failed: {}", stringify!($cond));
        }
    };
}

macro_rules! DCHECK_IMPLIES {
    ($cond1:expr, $cond2:expr) => {
        if cfg!(debug_assertions) && $cond1 && !($cond2) {
            panic!("DCheck implies failed: {} implies {}", stringify!($cond1), stringify!($cond2));
        }
    };
}

macro_rules! USE {
    ($expr:expr) => {
        let _ = $expr;
    };
}

macro_rules! UNREACHABLE {
    () => {
        panic!("Unreachable code reached");
    };
}

// TODO: Implement OS::SNPrintF, OS::VSNPrintF, OS::StrNCpy, RandomNumberGenerator, MemoryPermission enum

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemoryPermission {
    kNoAccess,
    kNoAccessWillJitLater,
    kRead,
    kReadWrite,
    kReadWriteExecute,
    kReadExecute,
}

lazy_static! {
    static ref RNG_MUTEX: Mutex<()> = Mutex::new(());
}

struct RandomNumberGenerator {
    seed: u64,
}

impl RandomNumberGenerator {
    fn new() -> Self {
        RandomNumberGenerator { seed: 0 }
    }

    fn set_seed(&mut self, seed: i64) {
        self.seed = seed as u64;
    }

    fn next_bytes(&mut self, buffer: *mut u8, size: usize) {
        // Simple LCG for demonstration purposes
        let mut current = self.seed;
        for i in 0..size {
            current = current.wrapping_mul(6364136223846793005).wrapping_add(1);
            unsafe { *buffer.add(i) = (current >> 32) as u8 };
        }
        self.seed = current;
    }
}

struct WindowsTimezoneCache {
    initialized_: bool,
    std_tz_name_: [c_char; 128],
    dst_tz_name_: [c_char; 128],
    tzinfo_: TIME_ZONE_INFORMATION,
}

impl WindowsTimezoneCache {
    fn new() -> Self {
        WindowsTimezoneCache {
            initialized_: false,
            std_tz_name_: [0; 128],
            dst_tz_name_: [0; 128],
            tzinfo_: unsafe { zeroed() },
        }
    }

    fn clear(&mut self, _detection: TimeZoneDetection) {
        self.initialized_ = false;
    }

    fn local_timezone(&self, time: c_double) -> *const c_char {
        Win32Time::new(time).local_timezone(self)
    }

    fn local_time_offset(&self, time: c_double, is_utc: bool) -> c_double {
        // Ignore is_utc and time_ms for now. That way, the behavior wouldn't
        // change with icu_timezone_data disabled.
        // Use current time, rounded to the millisecond.
        let t = Win32Time::new(OS::time_current_millis());
        // Time::LocalOffset inlcudes any daylight savings offset, so subtract it.
        (t.local_offset(self) - t.daylight_savings_offset(self)) as c_double
    }

    fn daylight_savings_offset(&self, time: c_double) -> c_double {
        Win32Time::new(time).daylight_savings_offset(self) as c_double
    }

    fn initialize_if_needed(&mut self) {
        if self.initialized_ {
            return;
        }

        unsafe {
            _tzset();
        }

        unsafe {
            std::ptr::write_bytes(&mut self.tzinfo_, 0, size_of::<TIME_ZONE_INFORMATION>());

            if GetTimeZoneInformation(&mut self.tzinfo_) == TIME_ZONE_ID_INVALID {
                self.tzinfo_.Bias = -60;
                self.tzinfo_.StandardDate.wMonth = 10;
                self.tzinfo_.StandardDate.wDay = 5;
                self.tzinfo_.StandardDate.wHour = 3;
                self.tzinfo_.StandardBias = 0;
                self.tzinfo_.DaylightDate.wMonth = 3;
                self.tzinfo_.DaylightDate.wDay = 5;
                self.tzinfo_.DaylightDate.wHour = 2;
                self.tzinfo_.DaylightBias = -60;
            }

            let mut std_tz_name_vec: Vec<u8> = Vec::with_capacity(128);
            let mut dst_tz_name_vec: Vec<u8> = Vec::with_capacity(128);

            let std_tz_name_result = WideCharToMultiByte(
                CP_UTF8,
                0,
                self.tzinfo_.StandardName.as_ptr(),
                -1,
                std_tz_name_vec.as_mut_ptr() as *mut i8,
                128,
                null_mut(),
                null_mut(),
            );

            if std_tz_name_result > 0 {
                std_tz_name_vec.set_len(std_tz_name_result as usize - 1);
            } else {
                std_tz_name_vec.clear();
            }

            let dst_tz_name_result = WideCharToMultiByte(
                CP_UTF8,
                0,
                self.tzinfo_.DaylightName.as_ptr(),
                -1,
                dst_tz_name_vec.as_mut_ptr() as *mut i8,
                128,
                null_mut(),
                null_mut(),
            );

            if dst_tz_name_result > 0 {
                dst_tz_name_vec.set_len(dst_tz_name_result as usize - 1);
            } else {
                dst_tz_name_vec.clear();
            }


            if std_tz_name_vec.is_empty() || std_tz_name_vec.get(0) == Some(&b'@') {
                let guess = self.guess_timezone_name_from_bias(self.tzinfo_.Bias);
                let name = format!("{} Standard Time", guess);
                OS::snprintf(&mut self.std_tz_name_, name.as_str());
            } else {
                for (i, &byte) in std_tz_name_vec.iter().enumerate() {
                    self.std_tz_name_[i] = byte as c_char;
                }
                self.std_tz_name_[std_tz_name_vec.len()] = 0;
            }

            if dst_tz_name_vec.is_empty() || dst_tz_name_vec.get(0) == Some(&b'@') {
                let guess = self.guess_timezone_name_from_bias(self.tzinfo_.Bias);
                let name = format!("{} Daylight Time", guess);
                OS::snprintf(&mut self.dst_tz_name_, name.as_str());
            } else {
                for (i, &byte) in dst_tz_name_vec.iter().enumerate() {
                    self.dst_tz_name_[i] = byte as c_char;
                }
                self.dst_tz_name_[dst_tz_name_vec.len()] = 0;
            }

            self.initialized_ = true;
        }
    }

    fn guess_timezone_name_from_bias(&self, bias: i32) -> &'static str {
        let k_hour = 60;
        match -bias {
            -9 * k_hour => "Alaska",
            -8 * k_hour => "Pacific",
            -7 * k_hour => "Mountain",
            -6 * k_hour => "Central",
            -5 * k_hour => "Eastern",
            -4 * k_hour => "Atlantic",
            0 * k_hour => "GMT",
            1 * k_hour => "Central Europe",
            2 * k_hour => "Eastern Europe",
            3 * k_hour => "Russia",
            5 * k_hour + 30 => "India",
            8 * k_hour => "China",
            9 * k_hour => "Japan",
            12 * k_hour => "New Zealand",
            _ => "Local",
        }
    }
}

trait TimezoneCache {
    fn clear(&mut self, detection: TimeZoneDetection);
    fn local_timezone(&self, time: c_double) -> *const c_char;
    fn local_time_offset(&self, time: c_double, is_utc: bool) -> c_double;
    fn daylight_savings_offset(&self, time: c_double) -> c_double;
}

impl TimezoneCache for WindowsTimezoneCache {
    fn clear(&mut self, detection: TimeZoneDetection) {
        self.clear(detection);
    }

    fn local_timezone(&self, time: c_double) -> *const c_char {
        self.local_timezone(time)
    }

    fn local_time_offset(&self, time: c_double, is_utc: bool) -> c_double {
        self.local_time_offset(time, is_utc)
    }

    fn daylight_savings_offset(&self, time: c_double) -> c_double {
        self.daylight_savings_offset(time)
    }
}

#[derive(Debug, Clone, Copy)]
enum TimeZoneDetection {
    System,
    ICU,
}

struct Win32Time {
    time_: TimeStamp,
}

#[repr(C)]
union TimeStamp {
    ft_: FILETIME,
    t_: i64,
}

impl Win32Time {
    const K_TIME_EPOC: i64 = 116444736000000000;
    const K_TIME_SCALER: i64 = 10000;
    const K_MS_PER_MINUTE: i64 = 60000;
    const K_SHORT_TZ_NAMES: bool = false;

    fn new() -> Self {
        Win32Time {
            time_: TimeStamp { t_: 0 },
        }
    }

    fn new_from_jstime(jstime: c_double) -> Self {
        Win32Time {
            time_: TimeStamp {
                t_: (jstime as i64) * Self::K_TIME_SCALER + Self::K_TIME_EPOC,
            },
        }
    }

    fn new_from_components(year: c_int, mon: c_int, day: c_int, hour: c_int, min: c_int, sec: c_int) -> Self {
        unsafe {
            let mut st: SYSTEMTIME = zeroed();
            st.wYear = year as u16;
            st.wMonth = mon as u16;
            st.wDay = day as u16;
            st.wHour = hour as u16;
            st.wMinute = min as u16;
            st.wSecond = sec as u16;
            st.wMilliseconds = 0;

            let mut ft: FILETIME = zeroed();
            SystemTimeToFileTime(&st, &mut ft);

            Win32Time {
                time_: TimeStamp { ft_: ft },
            }
        }
    }

    fn to_js_time(&self) -> c_double {
        (self.t() - Self::K_TIME_EPOC) as c_double / Self::K_TIME_SCALER as c_double
    }

    fn set_to_current_time(&mut self) {
        static mut INITIALIZED: bool = false;
        static mut INIT_TIME: TimeStamp = TimeStamp { t_: 0 };
        static mut INIT_TICKS: DWORD = 0;
        const K_HUNDRED_NANOSECONDS_PER_SECOND: i64 = 10000000;
        const K_MAX_CLOCK_ELAPSED_TIME: i64 = 60 * K_HUNDRED_NANOSECONDS_PER_SECOND; // 1 minute

        let mut needs_resync;
        unsafe {
            needs_resync = !INITIALIZED;
        }

        let mut time_now: TimeStamp = unsafe { zeroed() };
        unsafe {
            GetSystemTimeAsFileTime(&mut time_now.ft_);
        }

        let ticks_now = unsafe { timeGetTime() };

        unsafe {
            needs_resync |= ticks_now < INIT_TICKS;
        }

        unsafe {
            needs_resync |= (time_now.t_ - INIT_TIME.t_) > K_MAX_CLOCK_ELAPSED_TIME;
        }

        unsafe {
            needs_resync |= time_now.t_ < INIT_TIME.t_;
        }

        if needs_resync {
            unsafe {
                GetSystemTimeAsFileTime(&mut INIT_TIME.ft_);
                INIT_TICKS = ticks_now;
                INITIALIZED = true;
            }
        }

        let elapsed = unsafe { ticks_now - INIT_TICKS };

        unsafe {
            self.time_.t_ = INIT_TIME.t_ + (elapsed as i64) * 10000;
        }
    }

    fn local_offset(&self, cache: &WindowsTimezoneCache) -> i64 {
        unsafe {
            let mut cache = WindowsTimezoneCache::new();
            cache.initialize_if_needed();

            let mut rounded_to_second = *self;
            rounded_to_second.t() = rounded_to_second.t() / 1000 / Self::K_TIME_SCALER * 1000 * Self::K_TIME_SCALER;

            let unchecked_posix_time = rounded_to_second.to_js_time() / 1000.0;
            if unchecked_posix_time > i32::MAX as f64 || unchecked_posix_time < 0.0 {
                return 0;
            }

            let posix_time = unchecked_posix_time as i64;

            let mut posix_local_time_struct: tm = std::mem::zeroed();
            if localtime_s(&mut posix_local_time_struct, &posix_time) != 0 {
                return 0;
            }

            if posix_local_time_struct.tm_isdst > 0 {
                (cache.tzinfo_.Bias as i64 + cache.tzinfo_.DaylightBias as i64) * -Self::K_MS_PER_MINUTE
            } else if posix_local_time_struct.tm_isdst == 0 {
                (cache.tzinfo_.Bias as i64 + cache.tzinfo_.StandardBias as i64) * -Self::K_MS_PER_MINUTE
            } else {
                cache.tzinfo_.Bias as i64 * -Self::K_MS_PER_MINUTE
            }
        }
    }

    fn in_dst(&self, cache: &WindowsTimezoneCache) -> bool {
        unsafe {
            let mut cache = WindowsTimezoneCache::new();
            cache.initialize_if_needed();

            let mut in_dst = false;

            if cache.tzinfo_.StandardDate.wMonth != 0 || cache.tzinfo_.DaylightDate.wMonth != 0 {
                let offset = self.local_offset(cache);

                let dstofs = -(cache.tzinfo_.Bias as i64 + cache.tzinfo_.DaylightBias as i64) * Self::K_MS_PER_MINUTE;

                in_dst = offset == dstofs;
            }

            in_dst
        }
    }

    fn daylight_savings_offset(&self, cache: &WindowsTimezoneCache) -> i64 {
        if self.in_dst(cache) {
            60 * Self::K_MS_PER_MINUTE
        } else {
            0
        }
    }

    fn local_timezone(&self, cache: &WindowsTimezoneCache) -> *const c_char {
        if self.in_dst(cache) {
            unsafe { cache.dst_tz_name_.as_ptr() }
        } else {
            unsafe { cache.std_tz_name_.as_ptr() }
        }
    }

    fn ft(&mut self) -> &mut FILETIME {
        unsafe { &mut self.time_.ft_ }
    }

    fn t(&mut self) -> &mut i64 {
        unsafe { &mut self.time_.t_ }
    }
}

#[link(name = "kernel32")]
extern "system" {
    fn SystemTimeToFileTime(lpSystemTime: *const SYSTEMTIME, lpFileTime: *mut FILETIME) -> BOOL;
}

#[repr(C)]
struct SYSTEMTIME {
    wYear: u16,
    wMonth: u16,
    wDayOfWeek: u16,
    wDay: u16,
    wHour: u16,
    wMinute: u16,
    wSecond: u16,
    wMilliseconds: u16,
}

#[link(name = "msvcrt")]
extern "C" {
    fn _tzset();
}

#[link(name = "Mincrt")]
extern "C" {
    fn localtime_r(time: *const i64, out_tm: *mut tm) -> *mut tm;
}

#[repr(C)]
struct tm {
    tm_sec: c_int,   // seconds after the minute - [0,59]
    tm_min: c_int,   // minutes after the hour - [0,59]
    tm_hour: c_int,  // hours since midnight - [0,23]
    tm_mday: c_int,  // day of the month - [1,31]
    tm_mon: c_int,   // months since January - [0,11]
    tm_year: c_int,  // years since 1900
    tm_wday: c_int,  // days since Sunday - [0,6]
    tm_yday: c_int,  // days since January 1 - [0,365]
    tm_isdst: c_int, // daylight savings time flag
}

#[link(name = "kernel32", wide = true)]
extern "system" {
    fn WideCharToMultiByte(
        CodePage: UINT,
        dwFlags: DWORD,
        lpWideCharStr: *const u16,
        cchWideChar: c_int,
        lpMultiByteStr: *mut i8,
        cbMultiByte: c_int,
        lpDefaultChar: PSTR,
        lpUsedDefaultChar: *mut BOOL,
    ) -> c_int;
}

const CP_UTF8: u32 = 65001;

#[link(name = "winmm")]
extern "system" {
    fn timeGetTime() -> DWORD;
}

struct OS {}

impl OS {
    fn get_user_time(secs: &mut u32, usecs: &mut u32) -> i32 {
        unsafe {
            let mut dummy: FILETIME = std::mem::zeroed();
            let mut usertime: u64 = 0;

            if GetThreadTimes(
                GetCurrentThread(),
                &mut dummy,
                &mut dummy,
                &mut dummy,
                (&mut usertime) as *mut _ as *mut FILETIME,
            ) == 0
            {
                return -1;
            }

            usertime /= 10;

            *secs = (usertime / 1000000) as u32;
            *usecs = (usertime % 1000000) as u32;

            0
        }
    }

    fn get_peak_memory_usage_kb() -> i32 {
        const KB: i32 = 1024;

        unsafe {
            let mut mem_counters: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
            let ret = GetProcessMemoryInfo(
                GetCurrentProcess(),
                &mut mem_counters,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            );

            if ret == 0 {
                return -1;
            }

            (mem_counters.PeakWorkingSetSize / KB as SIZE_T) as i32
        }
    }

    fn time_current_millis() -> c_double {
        let now = base::platform::time::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        since_epoch.as_secs_f64() * 1000.0
    }

    fn create_timezone_cache() -> Box<dyn TimezoneCache> {
        Box::new(WindowsTimezoneCache::new())
    }

    fn get_last_error() -> c_int {
        unsafe { GetLastError() as i32 }
    }

    fn get_current_process_id() -> c_int {
        unsafe { GetCurrentProcessId() as i32 }
    }

    fn get_current_thread_id_internal() -> c_int {
        unsafe { GetCurrentThreadId() as i32 }
    }

    fn exit_process(exit_code: c_int) {
        unsafe {
            // Use TerminateProcess to avoid races between isolate threads and
            // static destructors.
            std::io::stdout().flush().unwrap();
            std::io::stderr().flush().unwrap();
            TerminateProcess(GetCurrentProcess(), exit_code as u32);
            // Termination the current process does not return. {TerminateProcess} is not
            // marked [[noreturn]] though, since it can also be used to terminate another
            // process.
            UNREACHABLE!();
        }
    }

    fn fopen(path: &str, mode: &str) -> *mut std::ffi::c_void {
        unsafe {
            let wide_path = utf8_to_utf16(path);
            let wide_mode = utf8_to_utf16(mode);

            let mut file: *mut std::ffi::c_void = null_mut();
            if _wfopen_s(&mut file, wide_path.as_ptr(), wide_mode.as_ptr()) == 0 {
                file
            } else {
                null_mut()
            }
        }
    }

    fn remove(path: &str) -> bool {
        unsafe { DeleteFileA(path.as_ptr() as *const i8) != 0 }
    }

    fn directory_separator() -> char {
        '\\'
    }

    fn is_directory_separator(ch: char) -> bool {
        ch == '/' || ch == '\\'
    }

    fn open_temporary_file() -> *mut std::ffi::c_void {
        unsafe {
            let mut temp_path_buffer: [i8; MAX_PATH as usize] = [0; MAX_PATH as usize];
            let path_result = GetTempPathA(MAX_PATH as u32, temp_path_buffer.as_mut_ptr());

            if path_result > MAX_PATH as u32 || path_result == 0 {
                return null_mut();
            }

            let mut temp_name_buffer: [i8; MAX_PATH as usize] = [0; MAX_PATH as usize];
            let name_result = GetTempFileNameA(
                temp_path_buffer.as_ptr(),
                "".as_ptr() as *const i8,
                0,
                temp_name_buffer.as_mut_ptr(),
            );

            if name_result == 0 {
                return null_mut();
            }

            let path = std::ffi::CStr::from_ptr(temp_name_buffer.as_ptr()).to_str().unwrap();

            let result = OS::fopen(path, "w+");
            if !result.is_null() {
                OS::remove(path);
            }
            result
        }
    }

    const LOG_FILE_OPEN_MODE: &'static str = "wb+";

    fn print(format: &str, args: std::fmt::Arguments) {
        OS::vprint(format, args);
    }

    fn vprint(format: &str, args: std::fmt::Arguments) {
        OS::vprint_helper(std::io::stdout().as_mut(), format, args);
    }

    fn fprint(out: &mut dyn std::io::Write, format: &str, args: std::fmt::Arguments) {
        OS::vfprint(out, format, args);
    }

    fn vfprint(out: &mut dyn std::io::Write, format: &str, args: std::fmt::Arguments) {
        OS::vprint_helper(out, format, args);
    }

    fn print_error(format: &str, args: std::fmt::Arguments) {
        