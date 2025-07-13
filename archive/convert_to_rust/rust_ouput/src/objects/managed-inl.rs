// Converted from V8 C++ source files:
// Header: managed-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod managed_inl {
    use crate::handles::global_handles_inl::GlobalHandles;
    use crate::objects::managed::Managed;
    use std::ptr::null_mut;
    use std::sync::{Arc, Mutex};
    use v8::WeakCallbackType;

    pub struct Isolate {
        global_handles: Mutex<GlobalHandles>,
        managed_ptr_destructors: Mutex<Vec<Box<ManagedPtrDestructor>>>,
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                global_handles: Mutex::new(GlobalHandles::new()),
                managed_ptr_destructors: Mutex::new(Vec::new()),
            }
        }

        pub fn global_handles(&self) -> &Mutex<GlobalHandles> {
            &self.global_handles
        }

        pub fn register_managed_ptr_destructor(&self, destructor: Box<ManagedPtrDestructor>) {
            self.managed_ptr_destructors.lock().unwrap().push(destructor);
        }
    }

    pub struct Factory {
        isolate: *mut Isolate,
    }

    impl Factory {
        pub fn new(isolate: *mut Isolate) -> Self {
            Factory { isolate }
        }

        pub fn new_foreign<T>(
            &self,
            address: usize,
            _allocation_type: AllocationType,
        ) -> DirectHandle<Managed<T>> {
            DirectHandle {
                ptr: address as *mut Managed<T>,
            }
        }

        pub fn new_trusted_foreign<T>(&self, address: usize) -> DirectHandle<TrustedManaged<T>> {
            DirectHandle {
                ptr: address as *mut TrustedManaged<T>,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum AllocationType {}

    pub struct DirectHandle<T> {
        ptr: *mut T,
    }

    impl<T> DirectHandle<T> {
        pub fn location(&self) -> usize {
            self.ptr as usize
        }
    }

    pub struct IndirectHandle<T> {
        location: usize,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> IndirectHandle<T> {
        pub fn location(&self) -> usize {
            self.location
        }
    }

    pub struct ManagedPtrDestructor {
        estimated_size: usize,
        shared_ptr: *mut dyn std::any::Any,
        destructor: fn(ptr: *mut std::ffi::c_void),
        pub external_memory_accounter_: ExternalMemoryAccounter,
        pub global_handle_location_: usize,
    }

    impl ManagedPtrDestructor {
        pub fn new<CppType: 'static>(
            estimated_size: usize,
            shared_ptr: std::shared_ptr::SharedPtr<CppType>,
            destructor: fn(ptr: *mut std::ffi::c_void),
        ) -> Self {
            ManagedPtrDestructor {
                estimated_size,
                shared_ptr: Box::into_raw(Box::new(shared_ptr)) as *mut dyn std::any::Any,
                destructor,
                external_memory_accounter_: ExternalMemoryAccounter::new(),
                global_handle_location_: 0,
            }
        }

        pub fn destroy(&mut self) {
            (self.destructor)(self.shared_ptr as *mut std::ffi::c_void);
        }
    }

    impl Drop for ManagedPtrDestructor {
        fn drop(&mut self) {
            (self.destructor)(self.shared_ptr as *mut std::ffi::c_void);
        }
    }

    pub struct ExternalMemoryAccounter {}

    impl ExternalMemoryAccounter {
        pub fn new() -> Self {
            ExternalMemoryAccounter {}
        }
        pub fn increase(&mut self, _isolate: *mut v8::Isolate, _estimated_size: usize) {}
    }

    pub struct TagForManaged<CppType> {
        _phantom: std::marker::PhantomData<CppType>,
    }

    impl<CppType> TagForManaged<CppType> {
        pub const value: ExternalPointerTag = ExternalPointerTag::Managed;
    }

    pub enum ExternalPointerTag {
        Managed,
        TrustedManaged,
    }

    pub fn is_managed_external_pointer_type(tag: ExternalPointerTag) -> bool {
        match tag {
            ExternalPointerTag::Managed => true,
            _ => false,
        }
    }

    pub struct Object {}

    pub fn cast<T>(_obj: *mut Object) -> *mut T {
        null_mut() as *mut T
    }

    pub struct V8 {}

    pub mod detail {
        pub fn destructor<CppType>(ptr: *mut std::ffi::c_void) {
            unsafe {
                let shared_ptr_ptr =
                    ptr as *mut std::shared_ptr::SharedPtr<CppType>;
                drop(Box::from_raw(shared_ptr_ptr));
            }
        }
    }

    pub trait BaseManaged<CppType> {
        fn from_impl(
            isolate: *mut Isolate,
            estimated_size: usize,
            shared_ptr: std::shared_ptr::SharedPtr<CppType>,
            allocation_type: Option<AllocationType>,
            trusted: bool,
        ) -> DirectHandle<Self>
        where
            Self: Sized;
    }

    impl<CppType: 'static> BaseManaged<CppType> for Managed<CppType> {
        fn from_impl(
            isolate: *mut Isolate,
            estimated_size: usize,
            shared_ptr: std::shared_ptr::SharedPtr<CppType>,
            allocation_type: Option<AllocationType>,
            trusted: bool,
        ) -> DirectHandle<Self>
        where
            Self: Sized,
        {
            let isolate_safe = unsafe { &mut *isolate };
            let factory = Factory::new(isolate);
            let destructor = Box::new(ManagedPtrDestructor::new(
                estimated_size,
                shared_ptr,
                detail::destructor::<CppType>,
            ));

            destructor
                .external_memory_accounter_
                .increase(unsafe { std::mem::transmute(isolate) }, estimated_size);

            let address = Box::into_raw(destructor) as usize;

            let handle: DirectHandle<Self> = if trusted {
                DirectHandle {
                    ptr: factory.new_trusted_foreign::<CppType>(address).ptr as *mut Self,
                }
            } else {
                let alloc_type = allocation_type.unwrap();
                DirectHandle {
                    ptr: factory.new_foreign::<CppType>(address, alloc_type).ptr as *mut Self,
                }
            };

            let global_handle_mutex = isolate_safe.global_handles();
            let mut global_handles = global_handle_mutex.lock().unwrap();
            let global_handle: IndirectHandle<Object> =
                global_handles.create(handle.ptr as *mut Object);

            let destructor_ptr = unsafe { (handle.ptr as *mut Self).as_mut().unwrap() };
            let boxed_destructor = unsafe { Box::from_raw(address as *mut ManagedPtrDestructor) };
            boxed_destructor.global_handle_location_ = global_handle.location();

            GlobalHandles::make_weak(
                boxed_destructor.global_handle_location_,
                Box::into_raw(boxed_destructor) as *mut std::ffi::c_void,
                &managed_object_finalizer,
                v8::WeakCallbackType::kParameter,
            );

            let boxed_destructor = unsafe { Box::from_raw(address as *mut ManagedPtrDestructor) };
            isolate_safe.register_managed_ptr_destructor(boxed_destructor);

            handle
        }
    }

    impl<CppType: 'static> Managed<CppType> {
        pub fn from(
            isolate: *mut Isolate,
            estimated_size: usize,
            shared_ptr: std::shared_ptr::SharedPtr<CppType>,
            allocation_type: AllocationType,
        ) -> DirectHandle<Self> {
            <Self as BaseManaged<CppType>>::from_impl(
                isolate,
                estimated_size,
                shared_ptr,
                Some(allocation_type),
                false,
            )
        }
    }

    impl<CppType: 'static> TrustedManaged<CppType> {
        pub fn from(
            isolate: *mut Isolate,
            estimated_size: usize,
            shared_ptr: std::shared_ptr::SharedPtr<CppType>,
        ) -> DirectHandle<Self> {
            <Self as BaseManaged<CppType>>::from_impl(
                isolate,
                estimated_size,
                shared_ptr,
                None,
                true,
            )
        }
    }

    pub struct TrustedManaged<CppType> {
        _phantom: std::marker::PhantomData<CppType>,
    }

    extern "C" fn managed_object_finalizer(_parameter: *const std::ffi::c_void) {}

    pub mod std {
        pub mod shared_ptr {
            use std::rc::Rc;

            #[derive(Debug)]
            pub struct SharedPtr<T>(Rc<T>);

            impl<T> SharedPtr<T> {
                pub fn new(value: T) -> Self {
                    SharedPtr(Rc::new(value))
                }
            }

            impl<T> Clone for SharedPtr<T> {
                fn clone(&self) -> Self {
                    SharedPtr(self.0.clone())
                }
            }

            impl<T> std::ops::Deref for SharedPtr<T> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    self.0.as_ref()
                }
            }
        }
    }
}
