// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::marker::PhantomData;

// Placeholder for v8-source-location.h
pub mod v8 {
    pub struct SourceLocation {}

    impl SourceLocation {
        pub fn Current() -> Self {
            SourceLocation {}
        }
    }
}

// Placeholder for src/base/logging.h
macro_rules! DCHECK {
    ($condition:expr) => {
        if !($condition) {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! DCHECK_WITH_MSG_AND_LOC {
    ($condition:expr, $msg:expr, $loc:expr) => {
        if !($condition) {
            panic!("DCHECK failed: {}. Message: {}", stringify!($condition), $msg);
        }
    };
}

macro_rules! V8_PRETTY_FUNCTION_VALUE_OR {
    ($fallback:expr) => {
        $fallback
    };
}

// Placeholder for src/objects/tagged.h
pub struct Tagged<T> {
    ptr: *mut (),
    _phantom: PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn new(ptr: *mut ()) -> Self {
        Tagged {
            ptr,
            _phantom: PhantomData,
        }
    }

    pub fn ptr(&self) -> *mut () {
        self.ptr
    }

    pub fn is_smi(&self) -> bool {
        // Placeholder implementation
        false
    }

    pub fn is_heap_object(&self) -> bool {
        // Placeholder implementation
        true
    }
    pub fn is_strong_or_smi(&self) -> bool {
        true
    }
    pub fn is_cleared(&self) -> bool {
        false
    }
}

pub struct IndirectHandle<T> {
    _phantom: PhantomData<T>,
}

pub struct MaybeIndirectHandle<T> {
    _phantom: PhantomData<T>,
}

pub struct DirectHandle<T> {
    _phantom: PhantomData<T>,
}

pub struct MaybeDirectHandle<T> {
    _phantom: PhantomData<T>,
}

pub struct Object {}
pub struct HeapObject {}
pub struct Smi {}
pub struct MaybeWeak<T> {
    _phantom: PhantomData<T>,
}

pub struct Union<T> {
    _phantom: PhantomData<T>,
}

pub trait IsSubtype<T> {}

pub mod internal {
    use super::*;

    pub trait CastableTo<T> {}
    impl CastableTo<Object> for Object {}
    impl CastableTo<HeapObject> for HeapObject {}
    impl CastableTo<Smi> for Smi {}

    pub trait V8Type {}
    impl V8Type for Object {}
    impl V8Type for HeapObject {}
    impl V8Type for Smi {}
    impl<T> V8Type for MaybeWeak<T> {}

    // CastTraits<T> is a type trait that defines type checking behaviour for
    // tagged object casting.
    pub trait CastTraits<To> {
        fn allow_from<From>(value: Tagged<From>) -> bool;
    }

    // `Is<T>(value)` checks whether `value` is a tagged object of type `T`.
    pub fn is<T: V8Type, U>(value: Tagged<U>) -> bool
    where
        T: CastTraits<T>,
    {
        T::allow_from(value)
    }

    pub fn is_indirect<T: V8Type, U>(_value: IndirectHandle<U>) -> bool
    {
        todo!()
    }

    pub fn is_maybe_indirect<T: V8Type, U>(_value: MaybeIndirectHandle<U>) -> bool
    {
        todo!()
    }

    pub fn is_direct<T: V8Type, U>(_value: DirectHandle<U>) -> bool
    {
        todo!()
    }

    pub fn is_maybe_direct<T: V8Type, U>(_value: MaybeDirectHandle<U>) -> bool
    {
        todo!()
    }

    // `UncheckedCast<T>(value)` casts `value` to a tagged object of type `T`,
    // without checking the type of value.
    pub fn unchecked_cast<To, From>(value: Tagged<From>) -> Tagged<To> {
        Tagged::<To>::new(value.ptr())
    }

    pub fn unchecked_cast_indirect<To, From>(_value: IndirectHandle<From>) -> IndirectHandle<To>
    {
        todo!()
    }

    pub fn unchecked_cast_maybe_indirect<To, From>(_value: MaybeIndirectHandle<From>) -> MaybeIndirectHandle<To>
    {
        todo!()
    }

    pub fn unchecked_cast_direct<To, From>(_value: DirectHandle<From>) -> DirectHandle<To>
    {
        todo!()
    }

    pub fn unchecked_cast_maybe_direct<To, From>(_value: MaybeDirectHandle<From>) -> MaybeDirectHandle<To>
    {
        todo!()
    }

    // `TryCast<T>(value, &out)` casts `value` to a tagged object of type `T` and
    // writes the value to `out`, returning true if the cast succeeded and false if
    // it failed.
    pub fn try_cast<To: V8Type, From>(value: Tagged<From>, out: &mut Tagged<To>) -> bool
    where
        To: CastTraits<To>,
    {
        if !is::<To, From>(value) {
            return false;
        }
        *out = unchecked_cast::<To, From>(value);
        true
    }

    pub fn try_cast_indirect<To: V8Type, From>(_value: IndirectHandle<From>, _out: &mut IndirectHandle<To>) -> bool
    {
        todo!()
    }
    pub fn try_cast_indirect_to_direct<To: V8Type, From>(_value: IndirectHandle<From>, _out: &mut DirectHandle<To>) -> bool
    {
        todo!()
    }
    pub fn try_cast_direct<To: V8Type, From>(_value: DirectHandle<From>, _out: &mut DirectHandle<To>) -> bool
    {
        todo!()
    }
    pub fn try_cast_maybe_indirect<To: V8Type, From>(_value: MaybeIndirectHandle<From>, _out: &mut MaybeIndirectHandle<To>) -> bool
    {
        todo!()
    }
    pub fn try_cast_maybe_indirect_to_maybe_direct<To: V8Type, From>(_value: MaybeIndirectHandle<From>, _out: &mut MaybeDirectHandle<To>) -> bool
    {
        todo!()
    }
    pub fn try_cast_maybe_direct<To: V8Type, From>(_value: MaybeDirectHandle<From>, _out: &mut MaybeDirectHandle<To>) -> bool
    {
        todo!()
    }

    // Only initialise the SourceLocation in debug mode.
    #[cfg(debug_assertions)]
    const INIT_SOURCE_LOCATION_IN_DEBUG: v8::SourceLocation = v8::SourceLocation::Current();
    #[cfg(not(debug_assertions))]
    const INIT_SOURCE_LOCATION_IN_DEBUG: v8::SourceLocation = v8::SourceLocation {};

    // TODO(leszeks): Implement GCAwareObjectTypeCheck
    #[cfg(debug_assertions)]
    pub fn gc_aware_object_type_check<T>(_object: Tagged<Object>, _heap: *const ()) -> bool {
        true
    }

    // `GCSafeCast<T>(value)` casts `object` to a tagged object of type `T` and
    // should be used when the cast can be called from within a GC. The case all
    // includes a debug check that `object` is either a tagged object of type `T`,
    // or one of few special cases possible during GC (see GCAwareObjectTypeCheck):
    // 1) `object` was already evacuated and the forwarding address refers to a
    //     tagged object of type `T`.
    // 2) During Scavenger, `object` is a large object.
    // 3) During a conservative Scavenger, `object` is a pinned object.
    #[cfg(debug_assertions)]
    pub fn gc_safe_cast<T>(object: Tagged<Object>, heap: *const ()) -> Tagged<T> {
        DCHECK!(gc_aware_object_type_check::<T>(object, heap));
        unchecked_cast::<T, Object>(object)
    }

    #[cfg(not(debug_assertions))]
    pub fn gc_safe_cast<T>(object: Tagged<Object>, _heap: *const ()) -> Tagged<T> {
        unchecked_cast::<T, Object>(object)
    }

    // `Cast<T>(value)` casts `value` to a tagged object of type `T`, with a debug
    // check that `value` is a tagged object of type `T`.
    pub fn cast<To: V8Type, From>(
        value: Tagged<From>,
        loc: &v8::SourceLocation,
    ) -> Tagged<To>
    where
        To: CastTraits<To>,
    {
        DCHECK_WITH_MSG_AND_LOC!(
            is::<To, From>(value),
            V8_PRETTY_FUNCTION_VALUE_OR!("Cast type check"),
            loc
        );
        unchecked_cast::<To, From>(value)
    }

    pub fn cast_indirect<To: V8Type, From>(
        value: IndirectHandle<From>,
        loc: &v8::SourceLocation,
    ) -> IndirectHandle<To>
    {
        todo!()
    }
    pub fn cast_direct<To: V8Type, From>(
        value: DirectHandle<From>,
        loc: &v8::SourceLocation,
    ) -> DirectHandle<To>
    {
        todo!()
    }
    pub fn cast_maybe_indirect<To: V8Type, From>(
        value: MaybeIndirectHandle<From>,
        loc: &v8::SourceLocation,
    ) -> MaybeIndirectHandle<To>
    {
        todo!()
    }
    pub fn cast_maybe_direct<To: V8Type, From>(
        value: MaybeDirectHandle<From>,
        loc: &v8::SourceLocation,
    ) -> MaybeDirectHandle<To>
    {
        todo!()
    }

    // TODO(leszeks): Figure out a way to make these cast to actual pointers rather
    // than Tagged.
    pub fn unchecked_cast_from_ptr<To, From>(value: *const From) -> Tagged<To> {
        unchecked_cast::<To, From>(Tagged::new(value as *mut ()))
    }
    pub fn cast_from_ptr<To: V8Type, From>(
        value: *const From,
        loc: &v8::SourceLocation,
    ) -> Tagged<To>
    where
        To: CastTraits<To>,
    {
        cast::<To, From>(Tagged::new(value as *mut ()), loc)
    }
    pub fn unchecked_cast_from_value<To, From>(value: From) -> Tagged<To> {
        unchecked_cast::<To, From>(Tagged::new(&value as *const _ as *mut ()))
    }
    pub fn cast_from_value<To: V8Type, From>(
        value: From,
        loc: &v8::SourceLocation,
    ) -> Tagged<To>
    where
        To: CastTraits<To>,
    {
        cast::<To, From>(Tagged::new(&value as *const _ as *mut ()), loc)
    }

    // `Is<T>(maybe_weak_value)` specialization for possible weak values and strong
    // target `T`, that additionally first checks whether `maybe_weak_value` is
    // actually a strong value (or a Smi, which can't be weak).
    pub fn is_maybe_weak<T: V8Type, U>(value: Tagged<MaybeWeak<U>>) -> bool
    where
        T: CastTraits<T>,
    {
        if !std::any::TypeId::of::<T>() == std::any::TypeId::of::<MaybeWeak<T>>() {
            if !value.is_strong_or_smi() {
                return false;
            }
            <T as CastTraits<T>>::allow_from(Tagged::<U>::new(value.ptr()))
        } else {
            <T as CastTraits<T>>::allow_from(value)
        }
    }

    pub fn is_union<T: V8Type, U>(value: Tagged<Union<U>>) -> bool
        where
            T: CastTraits<T>,
    {
        todo!()
    }

    // Specialization for maybe weak cast targets, which first converts the incoming
    // value to a strong reference and then checks if the cast to the strong T
    // is allowed. Cleared weak references always return true.
    impl<T> CastTraits<MaybeWeak<T>> for MaybeWeak<T>
    where
        T: V8Type + CastTraits<T>,
    {
        fn allow_from<U>(value: Tagged<U>) -> bool {
            if std::any::TypeId::of::<U>() == std::any::TypeId::of::<MaybeWeak<T>>() {
                let value: Tagged<MaybeWeak<T>> = unsafe { std::mem::transmute(value) };
                // Cleared values are always ok.
                if value.is_cleared() {
                    return true;
                }
                // TODO(leszeks): Skip Smi check for values that are known to not be Smi.
                if value.is_smi() {
                    return <T as CastTraits<T>>::allow_from(Tagged::<Smi>::new(value.ptr()));
                }
                //return CastTraits::<T>::AllowFrom(make_strong(value));
                todo!()
            } else {
                <T as CastTraits<T>>::allow_from(value)
            }
        }
    }

    impl CastTraits<Object> for Object {
        fn allow_from<From>(_value: Tagged<From>) -> bool {
            true
        }
    }

    impl CastTraits<Smi> for Smi {
        fn allow_from<From>(value: Tagged<From>) -> bool {
             let object_tag : Tagged<Object> = unsafe { std::mem::transmute(value) };

            object_tag.is_smi()
        }
    }

    impl CastTraits<HeapObject> for HeapObject {
        fn allow_from<From>(value: Tagged<From>) -> bool {
            let object_tag : Tagged<Object> = unsafe { std::mem::transmute(value) };
            object_tag.is_heap_object()
        }
    }

} // namespace v8::internal