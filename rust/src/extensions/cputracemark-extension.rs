// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::arch::asm;

// Placeholder for v8 crate.  Needs a proper definition for actual usage.
mod v8 {
    pub mod internal {
        #[allow(dead_code)]
        pub struct Isolate {}

        #[allow(dead_code)]
        pub struct FunctionCallbackInfo<'a> {
            pub args: Vec<Local<'a, Value>>,
            pub isolate: &'a mut Isolate,
        }

        #[allow(dead_code)]
        impl<'a> FunctionCallbackInfo<'a> {
            pub fn Length(&self) -> usize {
                self.args.len()
            }

            pub fn GetIsolate(&mut self) -> &mut Isolate {
                self.isolate
            }
        }

        pub mod String {
            #[derive(Debug, Clone)]
            pub struct String {
                value: String
            }

            impl String {
                pub fn NewFromUtf8(isolate: &mut super::Isolate, data: &[u8]) -> Local<'_, String> {
                   Local(String { value: String::from_utf8(data.to_vec()).unwrap() })
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct Local<'a, T>(pub T, pub std::marker::PhantomData<&'a T>);

        pub mod Value {
            #[derive(Debug, Clone)]
            pub enum Value {
                Uint32(u32),
                Other
            }
        }

        pub type Local<'a, T> = super::Local<'a, T>;

        #[allow(dead_code)]
        impl<'a> Local<'a, Value::Value> {
            pub fn IsUint32(&self) -> bool {
                match &self.0 {
                    Value::Value::Uint32(_) => true,
                    _ => false,
                }
            }

            pub fn Uint32Value(&self, _context: &mut Isolate) -> Result<u32, String> {
                match &self.0 {
                    Value::Value::Uint32(val) => Ok(*val),
                    _ => Err("Value is not Uint32".to_string()),
                }
            }
        }


        #[allow(dead_code)]
        pub mod FunctionTemplate {
            use super::{Isolate, Local, String};

            pub struct FunctionTemplate {}

            impl FunctionTemplate {
                pub fn New<'a>(isolate: &mut Isolate, callback: fn(&super::FunctionCallbackInfo)) -> Local<'a, FunctionTemplate> {
                   Local(FunctionTemplate{}, std::marker::PhantomData)
                }
            }
        }

        #[allow(dead_code)]
        impl Isolate {
            pub fn ThrowError(&mut self, message: &str) {
                eprintln!("Error thrown in V8: {}", message);
            }

            pub fn GetCurrentContext(&mut self) -> &mut Isolate {
                self
            }
        }

        // Placeholder for ValidateCallbackInfo.  Needs a proper implementation.
        #[allow(dead_code)]
        fn ValidateCallbackInfo(_info: &FunctionCallbackInfo) -> bool {
            true
        }
    }
}

pub struct CpuTraceMarkExtension {}

impl CpuTraceMarkExtension {
    #[allow(dead_code)]
    pub fn get_native_function_template<'a>(
        isolate: &mut v8::internal::Isolate,
        str_: v8::internal::Local<'a, v8::internal::String::String>,
    ) -> v8::internal::Local<'a, v8::internal::FunctionTemplate::FunctionTemplate> {
        v8::internal::FunctionTemplate::FunctionTemplate::New(isolate, CpuTraceMarkExtension::mark)
    }

    #[allow(dead_code)]
    pub fn mark(info: &v8::internal::FunctionCallbackInfo) {
        if !v8::internal::ValidateCallbackInfo(info) {
            return;
        }

        if info.Length() < 1 || !info.args[0].IsUint32() {
            info.GetIsolate().ThrowError("First parameter to cputracemark() must be a unsigned int32.");
            return;
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            let param_result = info.args[0].Uint32Value(info.GetIsolate().GetCurrentContext());

            match param_result {
                Ok(param) => {
                    let magic_number: u32 = 0x4711 | (param << 16);
                    let mut dummy: i32;
                    unsafe {
                        #[cfg(all(target_arch = "x86", target_feature = "pic"))]
                        {
                            asm!(
                                "push ebx",
                                "cpuid",
                                "pop ebx",
                                inout("eax") magic_number as i32 => dummy,
                                out("ecx") _,
                                out("edx") _,
                            );
                        }
                        #[cfg(not(all(target_arch = "x86", target_feature = "pic")))]
                        {
                            asm!(
                                "cpuid",
                                inout("eax") magic_number as i32 => dummy,
                                out("ecx") _,
                                out("edx") _,
                                out("ebx") _,
                            );
                        }
                    }
                }
                Err(err) => {
                    info.GetIsolate().ThrowError(&format!("Error getting Uint32 value: {}", err));
                }
            }
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
             // V8_HOST_ARCH_IA32 || V8_HOST_ARCH_X64 not defined
             // No implementation for other architectures.
        }
    }
}