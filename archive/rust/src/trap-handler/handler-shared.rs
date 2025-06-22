// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// PLEASE READ BEFORE CHANGING THIS FILE!
//
// This file contains code that is used both inside and outside the out of
// bounds trap handler. Because this code runs in a trap handler context,
// use extra care when modifying this file. Here are some rules to follow.
//
// 1. Do not introduce any new external dependencies. This file needs
//    to be self contained so it is easy to audit everything that a
//    trap handler might do.
//
// 2. Any changes must be reviewed by someone from the crash reporting
//    or security team. See OWNERS for suggested reviewers.
//
// For more information, see https://goo.gl/yMeyUY.

// src/trap-handler/trap-handler-internal.h is not provided, so stubbing it.
mod trap_handler_internal {
    pub struct CodeProtectionInfoListEntry {}
    pub struct SandboxRecord {}
}

use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicPtr, Ordering};
use std::thread_local;

thread_local! {
    /// We declare this as int rather than bool as a workaround for a glibc bug, in
    /// which the dynamic loader cannot handle executables whose TLS area is only
    /// 1 byte in size; see https://sourceware.org/bugzilla/show_bug.cgi?id=14898.
    static G_THREAD_IN_WASM_CODE: i32 = 0;
}

pub static mut G_NUM_CODE_OBJECTS: usize = 0;
pub static mut G_CODE_OBJECTS: *mut trap_handler_internal::CodeProtectionInfoListEntry = std::ptr::null_mut();
pub static mut G_SANDBOX_RECORDS_HEAD: *mut trap_handler_internal::SandboxRecord = std::ptr::null_mut();
pub static G_RECOVERED_TRAP_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static G_LANDING_PAD: AtomicUsize = AtomicUsize::new(0);

pub struct MetadataLock {
    spinlock_: AtomicBool,
}

impl MetadataLock {
    pub const fn new() -> Self {
        MetadataLock {
            spinlock_: AtomicBool::new(false),
        }
    }
    pub fn lock(&self) {
      G_THREAD_IN_WASM_CODE.with(|in_wasm| {
          if *in_wasm != 0 {
              std::process::abort();
          }
      });
      while self.spinlock_.swap(true, Ordering::Acquire) {
          std::hint::spin_loop();
      }
    }

    pub fn unlock(&self) {
        G_THREAD_IN_WASM_CODE.with(|in_wasm| {
            if *in_wasm != 0 {
                std::process::abort();
            }
        });
        self.spinlock_.store(false, Ordering::Release);
    }
}

unsafe impl Sync for MetadataLock {}
unsafe impl Send for MetadataLock {}

pub struct SandboxRecordsLock {
    spinlock_: AtomicBool,
}

impl SandboxRecordsLock {
    pub const fn new() -> Self {
        SandboxRecordsLock {
            spinlock_: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.spinlock_.swap(true, Ordering::Acquire) {
          std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.spinlock_.store(false, Ordering::Release);
    }
}

unsafe impl Sync for SandboxRecordsLock {}
unsafe impl Send for SandboxRecordsLock {}