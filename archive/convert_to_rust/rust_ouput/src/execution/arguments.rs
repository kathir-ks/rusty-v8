// Converted from V8 C++ source files:
// Header: arguments.h
// Implementation: arguments.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod arguments {
    use crate::execution::isolate::Isolate;
    use crate::objects::objects::ObjectPair;
    use crate::execution::frames::Address;
    use std::ptr::null_mut;
    use crate::execution::isolate::Tagged;
    use crate::execution::isolate::MaybeObject;
    use crate::objects::slots::FullObjectSlot;
    use crate::execution::isolate::MessageTemplate;
    use crate::strings::string_external::String_ExternalOneByteStringResource;
    use crate::execution::code::Code;
    use std::sync::atomic::AtomicU16;
    use crate::cppgc::internal::gc_info::GCInfoIndex;
    use crate::heap::heap::Heap;
    use crate::local_handles::Local;
    use crate::cppgc::cross_thread_persistent::PersistentBase;
    use crate::execution::source_location::SourceLocation;
    use crate::bigint::div_helpers::digit_t;

    #[derive(PartialEq, Eq)]
    pub enum ArgumentsType {
        kJS,
        kNative,
    }

    pub struct Arguments<const ARGUMENTS_TYPE: ArgumentsType> {
        length_: isize,
        arguments_: *mut Address,
    }

    impl<const ARGUMENTS_TYPE: ArgumentsType> Arguments<ARGUMENTS_TYPE> {
        pub fn new(length: i32, arguments: *mut Address) -> Self {
            assert!(length >= 0);
            Arguments {
                length_: length as isize,
                arguments_: arguments,
            }
        }

        pub fn index(&self, index: i32) -> Tagged<MaybeObject> {
            unsafe {
                let addr = self.address_of_arg_at(index).read();
                Tagged {
                    ptr: addr as *mut Object,
                }
            }
        }

        pub fn at<S>(&self, index: i32) -> Result<Local<S>, String> {
            let obj = Local::<Object>::new(self.address_of_arg_at(index) as *mut Object);
            // Attempt to cast the object to type S.  If this fails, return an error.
            // Note:  This cast is purely conceptual in Rust, and amounts to type
            // verification rather than any actual data manipulation.
            if std::any::TypeId::of::<S>() == std::any::TypeId::of::<Object>() {
                // If S is Object, the cast is always valid.
                Ok(unsafe { obj.cast::<S>() })
            } else {
                // Otherwise, return an error.
                Err("Unable to cast to the type".to_string())
            }
        }

        pub fn slot_from_address_at(&self, index: i32, offset: i32) -> FullObjectSlot {
            unsafe {
                let location = *(self.address_of_arg_at(index) as *mut *mut Address);
                FullObjectSlot {
                    address: location.add(offset as usize)
                }
            }
        }

        pub fn smi_value_at(&self, index: i32) -> i32 {
            unsafe { *(self.address_of_arg_at(index) as *mut i32) }
        }

        pub fn positive_smi_value_at(&self, index: i32) -> u32 {
            self.smi_value_at(index) as u32
        }

        pub fn tagged_index_value_at(&self, index: i32) -> i32 {
            self.smi_value_at(index)
        }

        pub fn number_value_at(&self, index: i32) -> f64 {
            unsafe { *(self.address_of_arg_at(index) as *mut f64) }
        }

        pub fn at_or_undefined(&self, isolate: &mut Isolate, index: i32) -> Local<Object> {
            if index >= 0 && index < self.length() {
                Local::<Object>::new(self.address_of_arg_at(index) as *mut Object)
            } else {
                unsafe {
                    Local::<Object>::new(isolate.undefined_value() as *mut Object)
                }
            }
        }

        pub fn address_of_arg_at(&self, index: i32) -> *mut Address {
            assert!(index as isize <= self.length_);
            let offset = index as isize * std::mem::size_of::<Address>() as isize;
            let final_offset = if ARGUMENTS_TYPE == ArgumentsType::kJS {
                (self.length_ - index as isize - 1) * std::mem::size_of::<Address>() as isize
            } else {
                offset
            };

            unsafe {
                (self.arguments_ as usize).wrapping_sub(final_offset as usize) as *mut Address
            }
        }

        pub fn length(&self) -> i32 {
            self.length_ as i32
        }
    }

    pub struct ChangeValueScope<'a> {
        location_: *mut Address,
        old_value_: Box<Local<'a, Object>>,
    }

    impl<'a> ChangeValueScope<'a> {
        pub fn new(isolate: &'a Isolate, args: &mut Arguments<{ArgumentsType::kJS}>, index: i32, value: Tagged<Object>) -> Self {
            let location_ = args.address_of_arg_at(index);
            let old_value_ = unsafe {
                let old_value_ptr = location_.read();
                Box::new(Local::<'a, Object>::new(old_value_ptr as *mut Object))
            };
            unsafe {
                location_.write(value.ptr as Address);
            }

            ChangeValueScope {
                location_: location_,
                old_value_: old_value_,
            }
        }
    }

    impl<'a> Drop for ChangeValueScope<'a> {
        fn drop(&mut self) {
            unsafe {
                self.location_.write(self.old_value_.ptr() as Address);
            }
        }
    }

    #[macro_export]
    macro_rules! clobber_double_registers {
        () => {
            #[cfg(debug_assertions)]
            {
                //ClobberDoubleRegisters(1, 2, 3, 4);
                println!("Clobbering double registers");
            }
            #[cfg(not(debug_assertions))]
            {}
        };
    }

    #[macro_export]
    macro_rules! runtime_function_returns_type {
        ($Type:ty, $InternalType:ty, $Convert:expr, $Name:ident, $($arg:ident: $arg_type:ty),*) => {
            ::paste::item! {
                #[allow(non_snake_case)]
                static mut [<RT_impl_ $Name>]: Option<unsafe extern "C" fn($($arg_type),*) -> $InternalType> = None;

                #[allow(non_snake_case)]
                #[inline(never)]
                pub unsafe extern "C" fn $Name(args_length: i32, args_object: *mut Address, isolate: *mut Isolate) -> $Type {
                    #[cfg(feature = "v8_runtime_call_stats")]
                    {
                        todo!();
                        //Stats_##Name(args_length, args_object, isolate)
                    }
                    #[cfg(not(feature = "v8_runtime_call_stats"))]
                    {
                        // Empty
                    }
                    let args = Arguments::<{ArgumentsType::kJS}>::new(args_length, args_object);
                    let result = [<RT_impl_ $Name>].unwrap()( $($arg),* );
                    $Convert(result)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! runtime_function {
        ($Name:ident) => {
            runtime_function_returns_type!(Address, Tagged<Object>, builtin_convert_result, $Name);
        };
    }

    #[macro_export]
    macro_rules! runtime_function_return_pair {
        ($Name:ident) => {
            runtime_function_returns_type!(ObjectPair, ObjectPair, builtin_convert_result_pair, $Name);
        };
    }

    pub fn builtin_convert_result(x: Tagged<Object>) -> Address {
        #[cfg(debug_assertions)]
        {
            //isolate.VerifyBuiltinsResult(x).ptr()
            println!("VerifyBuiltinsResult");
            x.ptr as Address
        }
        #[cfg(not(debug_assertions))]
        {
            x.ptr as Address
        }
    }

    pub fn builtin_convert_result_pair(x: ObjectPair) -> ObjectPair {
        #[cfg(debug_assertions)]
        {
            //isolate.VerifyBuiltinsResult(x)
            println!("VerifyBuiltinsResult");
            x
        }
        #[cfg(not(debug_assertions))]
        {
            x
        }
    }
}
