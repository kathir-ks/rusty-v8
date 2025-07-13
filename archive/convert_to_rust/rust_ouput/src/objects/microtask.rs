// Converted from V8 C++ source files:
// Header: microtask.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct StructBodyDescriptor {}

mod microtask_tq {
    #![allow(dead_code)]
    #![allow(non_camel_case_types)]
    use crate::v8::internal::Object;
    pub struct Microtask {
        dummy: i32,
    }
    impl Microtask {
        pub fn cast(obj: &Object) -> &Self {
            unsafe { std::mem::transmute(obj) }
        }
    }
    pub struct CallbackTask {
        dummy: i32,
    }
    impl CallbackTask {
        pub fn cast(obj: &Object) -> &Self {
            unsafe { std::mem::transmute(obj) }
        }
    }
    pub struct CallableTask {
        dummy: i32,
    }
    impl CallableTask {
        pub fn cast(obj: &Object) -> &Self {
            unsafe { std::mem::transmute(obj) }
        }
    }
}

use microtask_tq::*;

macro_rules! TQ_OBJECT_CONSTRUCTORS {
    ($class:ident) => {
        impl $class {
            pub fn new() -> Self {
                Self { dummy: 0 }
            }
        }
    };
}

pub(crate) use TQ_OBJECT_CONSTRUCTORS;

pub struct Microtask {
    dummy: i32,
}

impl Microtask {
    TQ_OBJECT_CONSTRUCTORS!(Microtask);
}

pub struct CallbackTask {
    dummy: i32,
}

impl CallbackTask {
    pub type BodyDescriptor = StructBodyDescriptor;
    TQ_OBJECT_CONSTRUCTORS!(CallbackTask);
}

pub struct CallableTask {
    dummy: i32,
}

impl CallableTask {
    pub type BodyDescriptor = StructBodyDescriptor;
    TQ_OBJECT_CONSTRUCTORS!(CallableTask);

    pub fn verify(_task: &CallableTask) -> bool {
        true
    }

    pub fn brief_print_details(&self, os: &mut std::fmt::Write) {
        write!(os, "CallableTask").unwrap();
    }
}

impl std::fmt::Display for CallableTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallableTask")
    }
}
