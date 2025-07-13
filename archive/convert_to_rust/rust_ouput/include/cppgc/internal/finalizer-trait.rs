// Converted from V8 C++ source files:
// Header: finalizer-trait.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {

pub type FinalizationCallback = Option<unsafe extern "C" fn(arg1: *mut std::ffi::c_void) -> ()>;

pub struct HasFinalizeGarbageCollectedObject<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> HasFinalizeGarbageCollectedObject<T> {
    pub const VALUE: bool = false;
}

pub trait FinalizeGarbageCollected {
    fn finalize_garbage_collected_object(&mut self);
}

pub struct FinalizerTraitImpl<T, const IS_FINALIZED: bool> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> FinalizerTraitImpl<T, true> {
    unsafe extern "C" fn custom_call(obj: *mut std::ffi::c_void) {
        let obj_ref = (obj as *mut T).as_mut().unwrap();
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<dyn FinalizeGarbageCollected>() {
          let trait_object = (obj as *mut dyn FinalizeGarbageCollected).as_mut().unwrap();
          trait_object.finalize_garbage_collected_object();
        } else {
          let obj_ref = (obj as *mut T).as_mut().unwrap();
          if let Some(obj) = obj_ref.downcast_mut::<T>() {
             obj.finalize_garbage_collected_object();
          }

        }
    }

    unsafe extern "C" fn destructor_call(obj: *mut std::ffi::c_void) {
        let obj_ref = (obj as *mut T).as_mut().unwrap();
        std::ptr::drop_in_place(obj_ref);
    }

    pub unsafe fn finalize(obj: *mut std::ffi::c_void) {
        assert_ne!(std::mem::size_of::<T>(), 0, "T must be fully defined");
        if <HasFinalizeGarbageCollectedObject<T>>::VALUE {
            Self::custom_call(obj);
        } else {
            Self::destructor_call(obj);
        }
    }
}

impl<T> FinalizerTraitImpl<T, false> {
    pub unsafe fn finalize(_obj: *mut std::ffi::c_void) {
        assert_ne!(std::mem::size_of::<T>(), 0, "T must be fully defined");
    }
}

pub struct FinalizerTrait<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> FinalizerTrait<T> {
    const K_NON_TRIVIAL_FINALIZER: bool = {
       <HasFinalizeGarbageCollectedObject<T>>::VALUE || !std::mem::needs_drop::<T>()
    };

    unsafe extern "C" fn finalize(obj: *mut std::ffi::c_void) {
        FinalizerTraitImpl::<T, { Self::K_NON_TRIVIAL_FINALIZER }>::finalize(obj);
    }

    pub const fn has_finalizer() -> bool {
        Self::K_NON_TRIVIAL_FINALIZER
    }

    pub const K_CALLBACK: FinalizationCallback = if Self::K_NON_TRIVIAL_FINALIZER {
        Some(Self::finalize)
    } else {
        None
    };
}
} // namespace internal
} // namespace cppgc
