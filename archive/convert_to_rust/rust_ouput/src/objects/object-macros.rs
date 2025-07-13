// Converted from V8 C++ source files:
// Header: object-macros.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct RelaxedLoadTag {}
pub struct RelaxedStoreTag {}
pub struct AcquireLoadTag {}
pub struct ReleaseStoreTag {}

macro_rules! _Pragma {
    ($x:expr) => {
        ()
    };
}

macro_rules! V8_OBJECT_PUSH {
    () => {
        #[repr(packed, C, align(4))]
    };
}

macro_rules! V8_OBJECT_POP {
    () => {
    };
}

macro_rules! V8_OBJECT {
    ($vis:vis struct $name:ident : $parent:ty { $($body:tt)* }) => {
        V8_OBJECT_PUSH!();
        $vis struct $name {
            $($body)*
        }
    };
}

macro_rules! V8_OBJECT_END {
    () => {
        V8_OBJECT_POP!();
    };
}

macro_rules! V8_OBJECT_INNER_CLASS {
    () => {
        V8_OBJECT_POP!();
    };
}

macro_rules! V8_OBJECT_INNER_CLASS_END {
    () => {
        V8_OBJECT_PUSH!();
    };
}

macro_rules! OBJECT_CONSTRUCTORS {
    ($Type:ident, $($arg:tt)*) => {
        impl $Type {
            pub const fn new() -> Self {
                Self { $($arg)* }
            }

            pub fn operator_arrow(&self) -> &Self {
                self
            }

            pub fn operator_arrow_const(&self) -> &Self {
                self
            }
        }

        impl $Type {
            fn check_type_on_cast(&self) {}
            fn new_with_address(_ptr: usize) -> Self {
                Self::new()
            }
        }
    };
}

macro_rules! OBJECT_CONSTRUCTORS_IMPL {
    ($Type:ident, $Super:ident) => {
        impl $Type {
            fn check_type_on_cast(&self) {
                // Implement your type checking logic here
            }

             fn new_with_address(ptr: usize) -> Self {
                 todo!()
             }
        }
    };
}

macro_rules! NEVER_READ_ONLY_SPACE {
    () => {
        fn get_heap(&self) -> usize;
        fn get_isolate(&self) -> usize;
    };
}

macro_rules! NEVER_READ_ONLY_SPACE_IMPL {
    ($Type:ident) => {
        impl $Type {
            fn get_heap(&self) -> usize {
                0
            }
            fn get_isolate(&self) -> usize {
                0
            }
        }
    };
}

macro_rules! DECL_PRIMITIVE_GETTER {
    ($name:ident, $type:ty) => {
        fn $name(&self) -> $type;
    };
}

macro_rules! DECL_PRIMITIVE_SETTER {
    ($name:ident, $type:ty) => {
        fn set_$name(&mut self, value: $type);
    };
}

macro_rules! DECL_PRIMITIVE_ACCESSORS {
    ($name:ident, $type:ty) => {
        DECL_PRIMITIVE_GETTER!($name, $type);
        DECL_PRIMITIVE_SETTER!($name, $type);
    };
}

macro_rules! DECL_BOOLEAN_ACCESSORS {
    ($name:ident) => {
        DECL_PRIMITIVE_ACCESSORS!($name, bool);
    };
}

macro_rules! DECL_INT_ACCESSORS {
    ($name:ident) => {
        DECL_PRIMITIVE_ACCESSORS!($name, i32);
    };
}

macro_rules! DECL_INT32_ACCESSORS {
    ($name:ident) => {
        DECL_PRIMITIVE_ACCESSORS!($name, i32);
    };
}

macro_rules! DECL_SANDBOXED_POINTER_ACCESSORS {
    ($name:ident, $type:ty) => {
        DECL_PRIMITIVE_ACCESSORS!($name, $type);
    };
}

macro_rules! DECL_UINT16_ACCESSORS {
    ($name:ident) => {
        DECL_PRIMITIVE_ACCESSORS!($name, u16);
    };
}

macro_rules! DECL_INT16_ACCESSORS {
    ($name:ident) => {
        DECL_PRIMITIVE_ACCESSORS!($name, i16);
    };
}

macro_rules! DECL_UINT8_ACCESSORS {
    ($name:ident) => {
        DECL_PRIMITIVE_ACCESSORS!($name, u8);
    };
}

macro_rules! DECL_RELAXED_PRIMITIVE_ACCESSORS {
    ($name:ident, $type:ty) => {
        fn $name(&self, tag: RelaxedLoadTag) -> $type;
        fn set_$name(&mut self, value: $type, tag: RelaxedStoreTag);
    };
}

macro_rules! DECL_RELAXED_INT32_ACCESSORS {
    ($name:ident) => {
        DECL_RELAXED_PRIMITIVE_ACCESSORS!($name, i32);
    };
}

macro_rules! DECL_RELAXED_UINT32_ACCESSORS {
    ($name:ident) => {
        DECL_RELAXED_PRIMITIVE_ACCESSORS!($name, u32);
    };
}

macro_rules! DECL_RELAXED_UINT16_ACCESSORS {
    ($name:ident) => {
        DECL_RELAXED_PRIMITIVE_ACCESSORS!($name, u16);
    };
}

macro_rules! DECL_RELAXED_UINT8_ACCESSORS {
    ($name:ident) => {
        DECL_RELAXED_PRIMITIVE_ACCESSORS!($name, u8);
    };
}

macro_rules! DECL_GETTER {
    ($name:ident, $($arg:tt)*) => {
        fn $name(&self) -> $($arg)*;
        fn $name(&self, cage_base: PtrComprCageBase) -> $($arg)*;
    };
}

macro_rules! DEF_GETTER {
    ($holder:ident, $name:ident, $($arg:tt)*) => {
        impl $holder {
            $($arg)* fn $name(&self) -> $($arg)* {
                let cage_base = PtrComprCageBase {}; // Assuming GetPtrComprCageBase returns PtrComprCageBase
                self.$name(cage_base)
            }

            $($arg)* fn $name(&self, cage_base: PtrComprCageBase) -> $($arg)* {
                // Implement the getter logic here
                 todo!()
            }
        }
    };
}

macro_rules! DEF_RELAXED_GETTER {
    ($holder:ident, $name:ident, $($arg:tt)*) => {
        impl $holder {
            $($arg)* fn $name(&self, tag: RelaxedLoadTag) -> $($arg)* {
                let cage_base = PtrComprCageBase {}; // Assuming GetPtrComprCageBase returns PtrComprCageBase
                self.$name(cage_base, tag)
            }

            $($arg)* fn $name(&self, cage_base: PtrComprCageBase, tag: RelaxedLoadTag) -> $($arg)* {
                // Implement the relaxed getter logic here
                todo!()
            }
        }
    };
}

macro_rules! DEF_ACQUIRE_GETTER {
    ($holder:ident, $name:ident, $($arg:tt)*) => {
        impl $holder {
            $($arg)* fn $name(&self, tag: AcquireLoadTag) -> $($arg)* {
                let cage_base = PtrComprCageBase {}; // Assuming GetPtrComprCageBase returns PtrComprCageBase
                self.$name(cage_base, tag)
            }

            $($arg)* fn $name(&self, cage_base: PtrComprCageBase, tag: AcquireLoadTag) -> $($arg)* {
                // Implement the acquire getter logic here
                 todo!()
            }
        }
    };
}

macro_rules! DEF_HEAP_OBJECT_PREDICATE {
    ($holder:ident, $name:ident) => {
        fn $name(obj: Tagged<$holder>) -> bool {
            let cage_base = PtrComprCageBase {};
            $name(obj, cage_base)
        }

        fn $name(obj: Tagged<$holder>, cage_base: PtrComprCageBase) -> bool {
            // Implement the heap object predicate logic here
            todo!()
        }
    };
}

macro_rules! TQ_FIELD_TYPE {
    ($name:ident, $tq_type:expr) => {
        const $name: &str = $tq_type;
    };
}

macro_rules! DECL_FIELD_OFFSET_TQ {
    ($name:ident, $value:expr, $tq_type:expr) => {
        const $name: i32 = $value;
        TQ_FIELD_TYPE!($name, $tq_type);
    };
}

macro_rules! DECL_SETTER {
    ($name:ident, $($arg:tt)*) => {
        fn set_$name(&mut self, value: $($arg)*, mode: WriteBarrierMode);
    };
}

macro_rules! DECL_ACCESSORS {
    ($name:ident, $($arg:tt)*) => {
        DECL_GETTER!($name, $($arg)*);
        DECL_SETTER!($name, $($arg)*);
    };
}

macro_rules! DECL_ACCESSORS_LOAD_TAG {
    ($name:ident, $type:ty, $tag_type:ty) => {
        fn $name(&self, tag: $tag_type) -> $type;
        fn $name(&self, cage_base: PtrComprCageBase, tag: $tag_type) -> $type;
    };
}

macro_rules! DECL_ACCESSORS_STORE_TAG {
    ($name:ident, $type:ty, $tag_type:ty) => {
        fn set_$name(&mut self, value: $type, tag: $tag_type, mode: WriteBarrierMode);
    };
}

macro_rules! DECL_RELAXED_GETTER {
    ($name:ident, $($arg:tt)*) => {
        DECL_ACCESSORS_LOAD_TAG!($name, ($($arg)*), RelaxedLoadTag);
    };
}

macro_rules! DECL_RELAXED_SETTER {
    ($name:ident, $($arg:tt)*) => {
        DECL_ACCESSORS_STORE_TAG!($name, ($($arg)*), RelaxedStoreTag);
    };
}

macro_rules! DECL_RELAXED_ACCESSORS {
    ($name:ident, $($arg:tt)*) => {
        DECL_RELAXED_GETTER!($name, $($arg)*);
        DECL_RELAXED_SETTER!($name, $($arg)*);
    };
}

macro_rules! DECL_ACQUIRE_GETTER {
    ($name:ident, $($arg:tt)*) => {
        DECL_ACCESSORS_LOAD_TAG!($name, ($($arg)*), AcquireLoadTag);
    };
}

macro_rules! DECL_RELEASE_SETTER {
    ($name:ident, $($arg:tt)*) => {
        DECL_ACCESSORS_STORE_TAG!($name, ($($arg)*), ReleaseStoreTag);
    };
}

macro_rules! DECL_RELEASE_ACQUIRE_ACCESSORS {
    ($name:ident, $($arg:tt)*) => {
        DECL_ACQUIRE_GETTER!($name, $($arg)*);
        DECL_RELEASE_SETTER!($name, $($arg)*);
    };
}

macro_rules! DEF_PRIMITIVE_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr, $type:ty) => {
        impl $holder {
            fn $name(&self) -> $type {
                // Implement reading from offset logic here
                 todo!()
            }
            fn set_$name(&mut self, value: $type) {
                // Implement writing to offset logic here
                 todo!()
            }
        }
    };
}

macro_rules! INT_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        DEF_PRIMITIVE_ACCESSORS!($holder, $name, $offset, i32);
    };
}

macro_rules! INT32_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        DEF_PRIMITIVE_ACCESSORS!($holder, $name, $offset, i32);
    };
}

macro_rules! UINT16_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        DEF_PRIMITIVE_ACCESSORS!($holder, $name, $offset, u16);
    };
}

macro_rules! UINT8_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        DEF_PRIMITIVE_ACCESSORS!($holder, $name, $offset, u8);
    };
}

macro_rules! RELAXED_INT32_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> i32 {
                // Implement relaxed read from offset logic here
                 todo!()
            }
            fn set_$name(&mut self, value: i32, _tag: RelaxedStoreTag) {
                // Implement relaxed write to offset logic here
                 todo!()
            }
        }
    };
}

macro_rules! RELAXED_UINT32_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> u32 {
                // Implement relaxed read from offset logic here
                 todo!()
            }
            fn set_$name(&mut self, value: u32, _tag: RelaxedStoreTag) {
                // Implement relaxed write to offset logic here
                 todo!()
            }
        }
    };
}

macro_rules! RELAXED_UINT16_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> u16 {
                // Implement relaxed read from offset logic here
                 todo!()
            }
            fn set_$name(&mut self, value: u16, _tag: RelaxedStoreTag) {
                // Implement relaxed write to offset logic here
                 todo!()
            }
        }
    };
}

macro_rules! RELAXED_UINT8_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> u8 {
                // Implement relaxed read from offset logic here
                 todo!()
            }
            fn set_$name(&mut self, value: u8, _tag: RelaxedStoreTag) {
                // Implement relaxed write to offset logic here
                 todo!()
            }
        }
    };
}

macro_rules! ACCESSORS_CHECKED2 {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr, $set_condition:expr) => {
        DEF_GETTER!($holder, $name, $type);
        impl $holder {
            fn set_$name(&mut self, value: $type, mode: WriteBarrierMode) {
                if $set_condition {
                    // Implement writing to offset logic here with write barrier
                }
                 todo!()
            }
        }
    };
}

macro_rules! ACCESSORS_CHECKED {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $condition:expr) => {
        ACCESSORS_CHECKED2!($holder, $name, $type, $offset, $condition, $condition);
    };
}

macro_rules! ACCESSORS {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        ACCESSORS_CHECKED!($holder, $name, $type, $offset, true);
    };
}

macro_rules! ACCESSORS_NOCAGE {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        impl $holder {
            fn $name(&self) -> $type {
                // Implement reading from offset logic here
                 todo!()
            }
            fn set_$name(&mut self, value: $type, mode: WriteBarrierMode) {
                // Implement writing to offset logic here with write barrier
                 todo!()
            }
        }
    };
}

macro_rules! RENAME_TORQUE_ACCESSORS {
    ($holder:ident, $name:ident, $torque_name:ident, $type:ty) => {
        impl $holder {
            fn $name(&self) -> $type {
                // Implement calling TorqueGeneratedClass::torque_name logic here
                 todo!()
            }
            fn $name(&self, _cage_base: PtrComprCageBase) -> $type {
                // Implement calling TorqueGeneratedClass::torque_name logic here with cage_base
                 todo!()
            }
            fn set_$name(&mut self, value: $type, mode: WriteBarrierMode) {
                // Implement calling TorqueGeneratedClass::set_torque_name logic here
                 todo!()
            }
        }
    };
}

macro_rules! RENAME_PRIMITIVE_TORQUE_ACCESSORS {
    ($holder:ident, $name:ident, $torque_name:ident, $type:ty) => {
        impl $holder {
            fn $name(&self) -> $type {
                // Implement calling TorqueGeneratedClass::torque_name logic here
                 todo!()
            }
            fn set_$name(&mut self, value: $type) {
                // Implement calling TorqueGeneratedClass::set_torque_name logic here
                 todo!()
            }
        }
    };
}

macro_rules! ACCESSORS_RELAXED_CHECKED2 {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr, $set_condition:expr) => {
        impl $holder {
            fn $name(&self) -> $type {
                let cage_base = PtrComprCageBase {};
                self.$name(cage_base)
            }
            fn $name(&self, cage_base: PtrComprCageBase) -> $type {
                // Implement relaxed load logic with checks
                 todo!()
            }
            fn set_$name(&mut self, value: $type, mode: WriteBarrierMode) {
                if $set_condition {
                    // Implement relaxed store logic with checks and write barrier
                }
                 todo!()
            }
        }
    };
}

macro_rules! ACCESSORS_RELAXED_CHECKED {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $condition:expr) => {
        ACCESSORS_RELAXED_CHECKED2!($holder, $name, $type, $offset, $condition, $condition);
    };
}

macro_rules! ACCESSORS_RELAXED {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        ACCESSORS_RELAXED_CHECKED!($holder, $name, $type, $offset, true);
    };
}

macro_rules! RELAXED_ACCESSORS_CHECKED2 {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr, $set_condition:expr) => {
        impl $holder {
            fn $name(&self, tag: RelaxedLoadTag) -> $type {
                let cage_base = PtrComprCageBase {};
                self.$name(cage_base, tag)
            }

            fn $name(&self, cage_base: PtrComprCageBase, tag: RelaxedLoadTag) -> $type {
                // Implement relaxed load logic with checks
                 todo!()
            }

            fn set_$name(&mut self, value: $type, tag: RelaxedStoreTag, mode: WriteBarrierMode) {
                if $set_condition {
                    // Implement relaxed store logic with checks and write barrier
                }
                 todo!()
            }
        }
    };
}

macro_rules! RELAXED_ACCESSORS_CHECKED {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $condition:expr) => {
        RELAXED_ACCESSORS_CHECKED2!($holder, $name, $type, $offset, $condition, $condition);
    };
}

macro_rules! RELAXED_ACCESSORS {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        RELAXED_ACCESSORS_CHECKED!($holder, $name, $type, $offset, true);
    };
}

macro_rules! RELEASE_ACQUIRE_GETTER_CHECKED {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr) => {
        DEF_ACQUIRE_GETTER!($holder, $name, $type);
        impl $holder {
            fn $name(&self, cage_base: PtrComprCageBase, tag: AcquireLoadTag) -> $type {
                 todo!()
            }
        }
    };
}

macro_rules! RELEASE_ACQUIRE_SETTER_CHECKED {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $set_condition:expr) => {
        impl $holder {
            fn set_$name(&mut self, value: $type, tag: ReleaseStoreTag, mode: WriteBarrierMode) {
                if $set_condition {
                     todo!()
                }
                 todo!()
            }
        }
    };
}

macro_rules! RELEASE_ACQUIRE_ACCESSORS_CHECKED2 {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr, $set_condition:expr) => {
        RELEASE_ACQUIRE_GETTER_CHECKED!($holder, $name, $type, $offset, $get_condition);
        RELEASE_ACQUIRE_SETTER_CHECKED!($holder, $name, $type, $offset, $set_condition);
    };
}

macro_rules! RELEASE_ACQUIRE_ACCESSORS_CHECKED {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $condition:expr) => {
        RELEASE_ACQUIRE_ACCESSORS_CHECKED2!($holder, $name, $type, $offset, $condition, $condition);
    };
}

macro_rules! RELEASE_ACQUIRE_ACCESSORS {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        RELEASE_ACQUIRE_ACCESSORS_CHECKED!($holder, $name, $type, $offset, true);
    };
}

macro_rules! SMI_ACCESSORS_CHECKED {
    ($holder:ident, $name:ident, $offset:expr, $condition:expr) => {
        impl $holder {
            fn $name(&self) -> i32 {
                if $condition {
                     todo!()
                }
                 todo!()
            }

            fn set_$name(&mut self, value: i32) {
                if $condition {
                     todo!()
                }
                 todo!()
            }
        }
    };
}

macro_rules! SMI_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        SMI_ACCESSORS_CHECKED!($holder, $name, $offset, true);
    };
}

macro_rules! DECL_RELEASE_ACQUIRE_INT_ACCESSORS {
    ($name:ident) => {
        fn $name(&self, tag: AcquireLoadTag) -> i32;
        fn set_$name(&mut self, value: i32, tag: ReleaseStoreTag);
    };
}

macro_rules! RELEASE_ACQUIRE_SMI_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: AcquireLoadTag) -> i32 {
                 todo!()
            }

            fn set_$name(&mut self, value: i32, _tag: ReleaseStoreTag) {
                 todo!()
            }
        }
    };
}

macro_rules! DECL_RELAXED_INT_ACCESSORS {
    ($name:ident) => {
        fn $name(&self, tag: RelaxedLoadTag) -> i32;
        fn set_$name(&mut self, value: i32, tag: RelaxedStoreTag);
    };
}

macro_rules! RELAXED_SMI_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> i32 {
                 todo!()
            }
            fn set_$name(&mut self, value: i32, _tag: RelaxedStoreTag) {
                 todo!()
            }
        }
    };
}

macro_rules! BOOL_GETTER {
    ($holder:ident, $field:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self) -> bool {
                 todo!()
            }
        }
    };
}

macro_rules! BOOL_ACCESSORS {
    ($holder:ident, $field:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self) -> bool {
                 todo!()
            }
            fn set_$name(&mut self, value: bool) {
                 todo!()
            }
        }
    };
}

macro_rules! DECL_RELAXED_BOOL_ACCESSORS {
    ($name:ident) => {
        fn $name(&self, tag: RelaxedLoadTag) -> bool;
        fn set_$name(&mut self, value: bool, tag: RelaxedStoreTag);
    };
}

macro_rules! RELAXED_BOOL_ACCESSORS {
    ($holder:ident, $field:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> bool {
                 todo!()
            }
            fn set_$name(&mut self, value: bool, _tag: RelaxedStoreTag) {
                 todo!()
            }
        }
    };
}

macro_rules! DECL_EXTERNAL_POINTER_ACCESSORS_MAYBE_READ_ONLY_HOST {
    ($name:ident, $type:ty) => {
        fn $name(&self, isolate: i::IsolateForSandbox) -> $type;
        fn init_$name(&mut self, isolate: i::IsolateForSandbox, initial_value: $type);
        fn set_$name(&mut self, isolate: i::IsolateForSandbox, value: $type);
    };
}

macro_rules! EXTERNAL_POINTER_ACCESSORS_MAYBE_READ_ONLY_HOST {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $tag:ident) => {
        impl $holder {
            fn $name(&self, isolate: i::IsolateForSandbox) -> $type {
                 todo!()
            }

            fn init_$name(&mut self, isolate: i::IsolateForSandbox, initial_value: $type) {
                 todo!()
            }

            fn set_$name(&mut self, isolate: i::IsolateForSandbox, value: $type) {
                 todo!()
            }
        }
    };
}

macro_rules! DECL_EXTERNAL_POINTER_ACCESSORS {
    ($name:ident, $type:ty) => {
        fn $name(&self) -> $type;
        DECL_EXTERNAL_POINTER_ACCESSORS_MAYBE_READ_ONLY_HOST!($name, $type);
    };
}

macro_rules! EXTERNAL_POINTER_ACCESSORS {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $tag:ident) => {
        impl $holder {
            fn $name(&self) -> $type {
                 todo!()
            }
        }
        EXTERNAL_POINTER_ACCESSORS_MAYBE_READ_ONLY_HOST!($holder, $name, $type, $offset, $tag);
    };
}

macro_rules! DECL_TRUSTED_POINTER_GETTERS {
    ($name:ident, $type:ty) => {
        fn $name(&self, isolate: IsolateForSandbox) -> Tagged<$type>;
        fn $name(&self, isolate: IsolateForSandbox, tag: AcquireLoadTag) -> Tagged<$type>;
        fn has_$name(&self) -> bool;
        fn has_${name}_unpublished(&self, isolate: IsolateForSandbox) -> bool;
    };
}

macro_rules! DECL_TRUSTED_POINTER_SETTERS {
    ($name:ident, $type:ty) => {
        fn set_$name(&mut self, value: Tagged<$type>, mode: WriteBarrierMode);
        fn set_$name(&mut self, value: Tagged<$type>, tag: ReleaseStoreTag, mode: WriteBarrierMode);
        fn clear_$name(&mut self);
    };
}

macro_rules! DECL_TRUSTED_POINTER_ACCESSORS {
    ($name:ident, $type:ty) => {
        DECL_TRUSTED_POINTER_GETTERS!($name, $type);
        DECL_TRUSTED_POINTER_SETTERS!($name, $type);
    };
}

macro_rules! TRUSTED_POINTER_ACCESSORS {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $tag:ident) => {
        impl $holder {
            fn $name(&self, isolate: IsolateForSandbox) -> Tagged<$type> {
                self.$name(isolate, AcquireLoadTag {})
            }
            fn $name(&self, isolate: IsolateForSandbox, _tag: AcquireLoadTag) -> Tagged<$type> {
                 todo!()
            }
            fn set_$name(&mut self, value: Tagged<$type>, mode: WriteBarrierMode) {
                self.set_$name(value, ReleaseStoreTag {}, mode)
            }
            fn set_$name(&mut self, value: Tagged<$type>, _tag: ReleaseStoreTag, mode: WriteBarrierMode) {
                 todo!()
            }
            fn has_$name(&self) -> bool {
                 todo!()
            }
            fn has_${name}_unpublished(&self, isolate: IsolateForSandbox) -> bool {
                 todo!()
            }
            fn clear_$name(&mut self) {
                 todo!()
            }
        }
    };
}

macro_rules! DECL_CODE_POINTER_ACCESSORS {
    ($name:ident) => {
        DECL_TRUSTED_POINTER_ACCESSORS!($name, Code);
    };
}

macro_rules! CODE_POINTER_ACCESSORS {
    ($holder:ident, $name:ident, $offset:expr) => {
        TRUSTED_POINTER_ACCESSORS!($holder, $name, Code, $offset, kCodeIndirectPointerTag);
    };
}

macro_rules! DECL_PROTECTED_POINTER_ACCESSORS {
    ($name:ident, $type:ty) => {
        fn $name(&self) -> Tagged<$type>;
        fn set_$name(&mut self, value: Tagged<$type>, mode: WriteBarrierMode);
        fn has_$name(&self) -> bool;
        fn clear_$name(&mut self);
    };
}

macro_rules! PROTECTED_POINTER_ACCESSORS {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        impl $holder {
            fn $name(&self) -> Tagged<$type> {
                 todo!()
            }
            fn set_$name(&mut self, value: Tagged<$type>, mode: WriteBarrierMode) {
                 todo!()
            }
            fn has_$name(&self) -> bool {
                 todo!()
            }
            fn clear_$name(&mut self) {
                 todo!()
            }
        }
    };
}

macro_rules! DECL_RELEASE_ACQUIRE_PROTECTED_POINTER_ACCESSORS {
    ($name:ident, $type:ty) => {
        fn $name(&self, tag: AcquireLoadTag) -> Tagged<$type>;
        fn set_$name(&mut self, value: Tagged<$type>, tag: ReleaseStoreTag, mode: WriteBarrierMode);
        fn has_$name(&self, tag: AcquireLoadTag) -> bool;
        fn clear_$name(&mut self, tag: ReleaseStoreTag);
    };
}

macro_rules! RELEASE_ACQUIRE_PROTECTED_POINTER_ACCESSORS {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        impl $holder {
            fn $name(&self, tag: AcquireLoadTag) -> Tagged<$type> {
                 todo!()
            }
            fn set_$name(&mut self, value: Tagged<$type>, tag: ReleaseStoreTag, mode: WriteBarrierMode) {
                 todo!()
            }
            fn has_$name(&self, tag: AcquireLoadTag) -> bool {
                 todo!()
            }
            fn clear_$name(&mut self, tag: ReleaseStoreTag) {
                 todo!()
            }
        }
    };
}

macro_rules! BIT_FIELD_ACCESSORS2 {
    ($holder
