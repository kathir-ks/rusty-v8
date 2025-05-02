// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file defines all the ETW Provider functions.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;

// Re-export windows-sys crate for use in other modules
pub use windows_sys as windows;

pub mod etw_provider {
    use super::*;
    use windows::Win32::Foundation::HRESULT;
    use windows::Win32::System::Diagnostics::Etw::*;
    use windows::core::{GUID, PCWSTR};
    use std::ptr;

    pub const V8_ETW_GUID: GUID = GUID {
        Data1: 0x57277741,
        Data2: 0x3638,
        Data3: 0x4A4B,
        Data4: [0xBD, 0xBA, 0x0A, 0xC6, 0xE4, 0x5D, 0xA5, 0x6C],
    };

    // Adapted from TRACELOGGING_DECLARE_PROVIDER
    #[macro_export]
    macro_rules! V8_DECLARE_TRACELOGGING_PROVIDER {
        ($v8Provider:ident) => {
            extern "C" {
                static $v8Provider: TRACEHANDLE;
            }
        };
    }

    // Adapted from TRACELOGGING_DEFINE_PROVIDER
    #[macro_export]
    macro_rules! V8_DEFINE_TRACELOGGING_PROVIDER {
        ($v8Provider:ident) => {
            #[no_mangle]
            static mut $v8Provider: TRACEHANDLE = 0;

            pub unsafe fn register_$v8Provider() -> HRESULT {
                let provider_name = "V8.js\0";
                let provider_guid = super::etw_provider::V8_ETW_GUID;
                let mut reg_handle: TRACEHANDLE = 0;

                let status = RegisterTraceGuidsW(
                    Some(EtwNotificationRegistration),
                    Some(&mut reg_handle),
                    &provider_guid,
                    1,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                if status as u32 != windows::Win32::Foundation::ERROR_SUCCESS {
                  return HRESULT::from(status);
                }

                $v8Provider = reg_handle;
                HRESULT::from(0)  // S_OK
            }

            pub unsafe fn unregister_$v8Provider() -> HRESULT {
                let mut reg_handle: TRACEHANDLE = $v8Provider;
                $v8Provider = 0;
                let status = UnregisterTraceGuids(reg_handle);
                if status as u32 != windows::Win32::Foundation::ERROR_SUCCESS {
                    return HRESULT::from(status);
                }

                HRESULT::from(0)  // S_OK
            }

        };
    }
}