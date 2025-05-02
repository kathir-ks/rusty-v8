// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains defines and typedefs that allow popular Windows types to
// be used without the overhead of including windows.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::{c_int, c_long, c_void};
use std::sync::{RwLock, Condvar};
use std::time::Duration;

#[cfg(target_pointer_width = "64")]
type ULONG_PTR = u64;
#[cfg(target_pointer_width = "64")]
type PULONG_PTR = *mut ULONG_PTR;

#[cfg(target_pointer_width = "32")]
type ULONG_PTR = u32;
#[cfg(target_pointer_width = "32")]
type PULONG_PTR = *mut ULONG_PTR;

pub type BOOL = c_int;
pub type DWORD = u32;
pub type LONG = c_long;
pub type LPVOID = *mut c_void;
pub type PVOID = *mut c_void;
pub type HANDLE = *mut c_void;

pub type SRWLOCK = RwLock<()>;
pub type CONDITION_VARIABLE = Condvar;

// These structs mimic the Windows structures.  We use Rust's mechanisms
// for synchronization rather than trying to directly mimic Windows.
// These structures should be used by the implementor of base::Locking for
// the windows platform to use system apis to call the synchronization methods.
pub struct CRITICAL_SECTION {
  lock: std::sync::Mutex<()>,
}

impl CRITICAL_SECTION {
  pub fn new() -> Self {
    CRITICAL_SECTION {
      lock: std::sync::Mutex::new(()),
    }
  }

  pub fn lock(&self) {
    let _guard = self.lock.lock().unwrap();
    // The guard is dropped when the function returns, releasing the lock.
  }

  pub fn unlock(&self) {
    // No explicit unlock needed, lock is released when guard is dropped.
  }

  pub fn try_lock(&self) -> bool {
    self.lock.try_lock().is_ok()
  }
}

// Declare V8 versions of some Windows structures. These are needed for
// when we need a concrete type but don't want to pull in Windows.h.

pub struct V8_SRWLOCK {
  lock: SRWLOCK,
}

impl V8_SRWLOCK {
    pub fn new() -> Self {
        V8_SRWLOCK {
            lock: RwLock::new(()),
        }
    }

    pub fn acquire_shared(&self) {
        self.lock.read().unwrap();
    }

    pub fn release_shared(&self) {
        // Reader lock is released when guard is dropped
    }

    pub fn acquire_exclusive(&self) {
        self.lock.write().unwrap();
    }

    pub fn release_exclusive(&self) {
      // Writer lock is released when guard is dropped
    }

    pub fn try_acquire_exclusive(&self) -> bool {
        self.lock.try_write().is_ok()
    }
}

pub struct V8_CONDITION_VARIABLE {
  condvar: CONDITION_VARIABLE,
}

impl V8_CONDITION_VARIABLE {
    pub fn new() -> Self {
        V8_CONDITION_VARIABLE {
            condvar: Condvar::new(),
        }
    }

    pub fn wait(&self, mutex_guard: std::sync::MutexGuard<'_, ()>) -> std::sync::MutexGuard<'_, ()> {
        self.condvar.wait(mutex_guard).unwrap()
    }

    pub fn wait_timeout(&self, mutex_guard: std::sync::MutexGuard<'_, ()>, timeout: Duration) -> Result<std::sync::MutexGuard<'_, ()>, std::sync::TryLockError>{
        match self.condvar.wait_timeout(mutex_guard, timeout) {
          Ok((guard, _)) => Ok(guard),
          Err(_) => Err(std::sync::TryLockError::WouldBlock),
        }
    }

    pub fn signal(&self) {
        self.condvar.notify_one();
    }

    pub fn signal_all(&self) {
        self.condvar.notify_all();
    }
}

pub struct V8_CRITICAL_SECTION {
  critical_section: CRITICAL_SECTION,
}

impl V8_CRITICAL_SECTION {
  pub fn new() -> Self {
      V8_CRITICAL_SECTION {
          critical_section: CRITICAL_SECTION::new(),
      }
  }

  pub fn lock(&self) {
      self.critical_section.lock();
  }

  pub fn unlock(&self) {
      self.critical_section.unlock();
  }

  pub fn try_lock(&self) -> bool {
      self.critical_section.try_lock()
  }
}

// The following functions were not required to fully translate the file to Rust.

// inline SRWLOCK* V8ToWindowsType(V8_SRWLOCK* p) {
//   return reinterpret_cast<SRWLOCK*>(p);
// }

// inline const SRWLOCK* V8ToWindowsType(const V8_SRWLOCK* p) {
//   return reinterpret_cast<const SRWLOCK*>(p);
// }

// inline CONDITION_VARIABLE* V8ToWindowsType(V8_CONDITION_VARIABLE* p) {
//   return reinterpret_cast<CONDITION_VARIABLE*>(p);
// }

// inline const CONDITION_VARIABLE* V8ToWindowsType(
//     const V8_CONDITION_VARIABLE* p) {
//   return reinterpret_cast<const CONDITION_VARIABLE*>(p);
// }

// inline CRITICAL_SECTION* V8ToWindowsType(V8_CRITICAL_SECTION* p) {
//   return reinterpret_cast<CRITICAL_SECTION*>(p);
// }

// inline const CRITICAL_SECTION* V8ToWindowsType(const V8_CRITICAL_SECTION* p) {
//   return reinterpret_cast<const CRITICAL_SECTION*>(p);
// }