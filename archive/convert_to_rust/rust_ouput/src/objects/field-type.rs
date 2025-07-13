// Converted from V8 C++ source files:
// Header: field-type.h
// Implementation: field-type.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct FieldType {}

impl FieldType {
    pub const kFieldTypesCanBeClearedOnGC: bool = true;

    pub fn None() -> Tagged<FieldType> {
        Tagged::<FieldType>::new(Smi::FromInt(2).ptr as *mut FieldType)
    }

    pub fn Any() -> Tagged<FieldType> {
        Tagged::<FieldType>::new(Smi::FromInt(1).ptr as *mut FieldType)
    }

    pub fn None_isolate(_isolate: *mut Isolate) -> DirectHandle<FieldType> {
        DirectHandle::<FieldType>::new(FieldType::None())
    }

    pub fn Any_isolate(_isolate: *mut Isolate) -> DirectHandle<FieldType> {
        DirectHandle::<FieldType>::new(FieldType::Any())
    }

    pub fn Class(map: Tagged<Map>) -> Tagged<FieldType> {
        unsafe { std::mem::transmute(map) }
    }

    pub fn Class_isolate(map: DirectHandle<Map>, isolate: *mut Isolate) -> DirectHandle<FieldType> {
        DirectHandle::<FieldType>::new(FieldType::Class(map.location))
    }

    pub fn NowContains(type_: Tagged<FieldType>, value: Tagged<Object>) -> bool {
        if type_.ptr == FieldType::Any().ptr {
            return true;
        }
        if type_.ptr == FieldType::None().ptr {
            return false;
        }

        if !IsHeapObject(value) {
            return false;
        }
        let heap_object: Tagged<HeapObject> = unsafe { std::mem::transmute(value) };
        let map: Tagged<Map> = unsafe { std::mem::transmute(type_) };
        heap_object.map() == map
    }

    pub fn NowContains_handle(type_: Tagged<FieldType>, value: DirectHandle<Object>) -> bool {
        FieldType::NowContains(type_, value.location)
    }

    pub fn AsClass(type_: Tagged<FieldType>) -> Tagged<Map> {
        assert!(IsClass(type_));
        unsafe { std::mem::transmute(type_) }
    }

    pub fn AsClass_handle(type_: DirectHandle<FieldType>) -> DirectHandle<Map> {
        assert!(IsClass(type_.location));
        DirectHandle::<Map>::new(unsafe { std::mem::transmute(type_.location) })
    }

    pub fn NowStable(type_: Tagged<FieldType>) -> bool {
        !IsClass(type_) || FieldType::AsClass(type_).is_stable()
    }

    pub fn NowIs(type_: Tagged<FieldType>, other: Tagged<FieldType>) -> bool {
        if IsAny(other) {
            return true;
        }
        if IsNone(type_) {
            return true;
        }
        if IsNone(other) {
            return false;
        }
        if IsAny(type_) {
            return false;
        }
        assert!(IsClass(type_));
        assert!(IsClass(other));
        type_.ptr == other.ptr
    }

    pub fn Equals(type_: Tagged<FieldType>, other: Tagged<FieldType>) -> bool {
        if IsAny(type_) && IsAny(other) {
            return true;
        }
        if IsNone(type_) && IsNone(other) {
            return true;
        }
        if IsClass(type_) && IsClass(other) {
            return type_.ptr == other.ptr;
        }
        false
    }

    pub fn NowIs_handle(type_: Tagged<FieldType>, other: DirectHandle<FieldType>) -> bool {
        FieldType::NowIs(type_, other.location)
    }

    pub fn PrintTo(type_: Tagged<FieldType>, os: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if IsAny(type_) {
            write!(os, "Any")
        } else if IsNone(type_) {
            write!(os, "None")
        } else {
            assert!(IsClass(type_));
            let ptr_val = FieldType::AsClass(type_).ptr() as *const _;
            write!(os, "Class({:p})", ptr_val)
        }
    }
}

pub fn IsClass(obj: Tagged<FieldType>) -> bool {
    IsMap(unsafe { std::mem::transmute(obj) })
}

pub fn IsNone(obj: Tagged<FieldType>) -> bool {
    obj.ptr == FieldType::None().ptr
}

pub fn IsAny(obj: Tagged<FieldType>) -> bool {
    obj.ptr == FieldType::Any().ptr
}

use crate::v8::internal::Isolate;
use crate::v8::internal::Tagged;
use crate::v8::internal::Map;
use crate::v8::internal::Object;
use crate::v8::internal::HeapObject;
use crate::v8::internal::Smi;
use crate::v8::internal::direct_handle;
use crate::v8::internal::DirectHandle;

pub fn IsHeapObject(obj: Tagged<Object>) -> bool {
    let address = obj.ptr() as usize;
    address & 1 == 0
}

pub fn IsMap(obj: Tagged<Object>) -> bool {
    true
}
