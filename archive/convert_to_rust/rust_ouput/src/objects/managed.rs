// Converted from V8 C++ source files:
// Header: managed.h
// Implementation: managed.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod managed {
    //use std::sync::Arc;
    //use std::sync::Mutex;
    //use std::any::Any;
    //use crate::api::api::Isolate;
    use crate::objects::foreign::Foreign;
    //use crate::handles::handles::DirectHandle;
    //use crate::heap::factory::AllocationType;
    //use crate::sandbox::external_pointer_table::ExternalPointerTable;
    //use crate::execution::isolate::Isolate;
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::objects::objects::code;
    use crate::codegen::code_stub_assembler::isolate;

    pub struct TagForManaged<CppType> {
        _phantom: std::marker::PhantomData<CppType>,
    }

    impl<CppType> TagForManaged<CppType> {
        pub const value: ExternalPointerTag = ExternalPointerTag::Other; // Provide a default value
    }

    #[macro_export]
    macro_rules! ASSIGN_EXTERNAL_POINTER_TAG_FOR_MANAGED {
        ($CppType:ty, $Tag:expr) => {
            impl TagForManaged<$CppType> {
                pub const value: ExternalPointerTag = $Tag;
            }
        };
    }

    #[derive(Debug)]
    pub enum ExternalPointerTag {
        Other,
    }

    pub struct ManagedPtrDestructor {
        estimated_size_: usize,
        prev_: *mut ManagedPtrDestructor,
        next_: *mut ManagedPtrDestructor,
        shared_ptr_ptr_: *mut std::ffi::c_void,
        destructor_: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
        global_handle_location_: *mut Address,
        external_memory_accounter_: ExternalMemoryAccounter,
    }

    impl ManagedPtrDestructor {
        pub fn new(
            estimated_size: usize,
            shared_ptr_ptr: *mut std::ffi::c_void,
            destructor: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
        ) -> Self {
            ManagedPtrDestructor {
                estimated_size_: estimated_size,
                prev_: std::ptr::null_mut(),
                next_: std::ptr::null_mut(),
                shared_ptr_ptr_: shared_ptr_ptr,
                destructor_: destructor,
                global_handle_location_: std::ptr::null_mut(),
                external_memory_accounter_: ExternalMemoryAccounter::new(),
            }
        }
    }

    pub struct ExternalMemoryAccounter {}
    impl ExternalMemoryAccounter{
        fn new() -> ExternalMemoryAccounter{
            ExternalMemoryAccounter{}
        }

        fn Decrease(&mut self, _isolate : *mut v8::Isolate, _size : usize){}
        
    }
    
    pub type Address = usize;

    pub struct Managed<CppType> {
        foreign: Foreign,
        _phantom: std::marker::PhantomData<CppType>,
    }

    impl<CppType> Managed<CppType> {
        pub fn new() -> Self {
            Managed {
                foreign: Foreign::new(0),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn from_address(ptr: Address) -> Self {
            Managed {
                foreign: Foreign::new(ptr),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn from_address_skip_type_check(ptr: Address) -> Self {
            Managed {
                foreign: Foreign::new_skip_type_check(ptr),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn operator_arrow(&mut self) -> &mut Self {
            self
        }

        pub fn operator_arrow_const(&self) -> &Self {
            self
        }

        pub fn raw(&mut self) -> *mut CppType {
            unsafe {
                let shared_ptr_ptr = self.GetSharedPtrPtr();
                if shared_ptr_ptr.is_null() {
                    std::ptr::null_mut()
                } else {
                    (*shared_ptr_ptr).as_mut().map(|ptr| ptr as *mut CppType).unwrap_or(std::ptr::null_mut())
                }
            }
        }

        pub fn get(&self) -> Option<&std::shared_ptr::SharedPtr<CppType>> {
            unsafe {
                let shared_ptr_ptr = self.GetSharedPtrPtr();
                if shared_ptr_ptr.is_null() {
                    None
                } else {
                    (*shared_ptr_ptr).as_ref()
                }
            }
        }

        pub fn estimated_size(&self) -> usize {
            unsafe {
                let destructor = self.GetDestructor();
                if destructor.is_null() {
                    0
                } else {
                    (*destructor).estimated_size_
                }
            }
        }

        //     static DirectHandle<Managed<CppType>> From(
        //     Isolate* isolate, size_t estimated_size,
        //     std::shared_ptr<CppType> shared_ptr,
        //     AllocationType allocation_type = AllocationType::kYoung);
        // }
        // pub fn from(
        //     isolate: *mut Isolate,
        //     estimated_size: usize,
        //     shared_ptr: std::shared_ptr::SharedPtr<CppType>,
        //     allocation_type: AllocationType,
        // ) -> DirectHandle<Managed<CppType>> {
        //     todo!()
        // }
        
        unsafe fn GetDestructor(&self) -> *mut ManagedPtrDestructor {
            let k_tag: ExternalPointerTag = TagForManaged::<CppType>::value;
            self.foreign.foreign_address::<ExternalPointerTag>() as *mut ManagedPtrDestructor
        }

        unsafe fn GetSharedPtrPtr(&self) -> *mut *mut std::shared_ptr::SharedPtr<CppType> {
            let destructor = self.GetDestructor();
            if destructor.is_null() {
                std::ptr::null_mut()
            } else {
                (*destructor).shared_ptr_ptr_ as *mut *mut std::shared_ptr::SharedPtr<CppType>
            }
        }
    }

    pub enum AllocationType {
        kYoung,
    }

    pub struct TrustedManaged<CppType> {
        trusted_foreign: TrustedForeign,
        _phantom: std::marker::PhantomData<CppType>,
    }

    impl<CppType> TrustedManaged<CppType> {
        pub fn new() -> Self {
            TrustedManaged {
                trusted_foreign: TrustedForeign::new(0),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn from_address(ptr: Address) -> Self {
            TrustedManaged {
                trusted_foreign: TrustedForeign::new(ptr),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn from_address_skip_type_check(ptr: Address) -> Self {
            TrustedManaged {
                trusted_foreign: TrustedForeign::new_skip_type_check(ptr),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn operator_arrow(&mut self) -> &mut Self {
            self
        }

        pub fn operator_arrow_const(&self) -> &Self {
            self
        }

        pub fn raw(&mut self) -> *mut CppType {
            unsafe {
                let shared_ptr_ptr = self.GetSharedPtrPtr();
                if shared_ptr_ptr.is_null() {
                    std::ptr::null_mut()
                } else {
                    (*shared_ptr_ptr).as_mut().map(|ptr| ptr as *mut CppType).unwrap_or(std::ptr::null_mut())
                }
            }
        }

        pub fn get(&self) -> Option<&std::shared_ptr::SharedPtr<CppType>> {
            unsafe {
                let shared_ptr_ptr = self.GetSharedPtrPtr();
                if shared_ptr_ptr.is_null() {
                    None
                } else {
                    (*shared_ptr_ptr).as_ref()
                }
            }
        }

        unsafe fn GetSharedPtrPtr(&self) -> *mut *mut std::shared_ptr::SharedPtr<CppType> {
            let destructor = self.trusted_foreign.foreign_address() as *mut ManagedPtrDestructor;
            if destructor.is_null() {
                std::ptr::null_mut()
            } else {
                (*destructor).shared_ptr_ptr_ as *mut *mut std::shared_ptr::SharedPtr<CppType>
            }
        }
    }

    pub struct TrustedForeign {
        address: Address,
    }

    impl TrustedForeign {
        pub fn new(address: Address) -> Self {
            TrustedForeign { address }
        }

        pub fn new_skip_type_check(address: Address) -> Self {
            TrustedForeign { address }
        }

        pub fn foreign_address(&self) -> Address {
            self.address
        }
    }
} // namespace v8::internal
pub mod managed_object_finalizer {
    use crate::managed::ManagedPtrDestructor;
    use v8::WeakCallbackInfo;
    use crate::execution::isolate::Isolate;
    use crate::v8::Isolate as v8Isolate;

    pub fn ManagedObjectFinalizer(data: &WeakCallbackInfo<()>) {
        unsafe {
            let destructor = data.GetParameter() as *mut ManagedPtrDestructor;
            
            //GlobalHandles::Destroy(destructor->global_handle_location_);
            // We need to do the main work as a second pass callback because
            // it can trigger garbage collection. The first pass callbacks
            // are not allowed to invoke V8 API.
            data.SetSecondPassCallback(Some(ManagedObjectFinalizerSecondPass));
        }
    }
    extern "C" fn ManagedObjectFinalizerSecondPass(data: &WeakCallbackInfo<()>) {
        unsafe {
            let destructor = data.GetParameter() as *mut ManagedPtrDestructor;
            let isolate = data.GetIsolate() as *mut Isolate;
            
            if let Some(destructor_func) = (*destructor).destructor_ {
                destructor_func((*destructor).shared_ptr_ptr_);
            }
            
            //(*destructor).external_memory_accounter_.Decrease(isolate as *mut v8Isolate, (*destructor).estimated_size_);
            //(*destructor).ZapExternalPointerTableEntry();
           
           drop(Box::from_raw(destructor));
        }
    }

}
