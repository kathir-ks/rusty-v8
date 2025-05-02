// Note 1: Any file that includes this one should include object-macros-undef.h
// at the bottom.
// Note 2: This file is deliberately missing the include guards (the undeffing
// approach wouldn't work otherwise).
//
// PRESUBMIT_INTENTIONALLY_MISSING_INCLUDE_GUARD

// The accessors with RELAXED_, ACQUIRE_, and RELEASE_ prefixes should be used
// for fields that can be written to and read from multiple threads at the same
// time. See comments in src/base/atomicops.h for the memory ordering sematics.

// First, ensure that we do not include object-macros.h twice without including
// object-macros-undef.h in between.
// #ifdef V8_OBJECT_MACROS_DEFINED
// #error Include object-macros-undef.h before including object-macros.h again
// #endif
// #define V8_OBJECT_MACROS_DEFINED

// #include "src/base/memory.h" // Assuming no direct equivalent needed

// V8 objects are defined as:
//
//     V8_OBJECT class Foo : public Base {
//       ...
//     } V8_OBJECT_END;
//
// These macros are to enable packing down to 4-byte alignment (i.e. int32
// alignment, since we have int32 fields), and to add warnings which ensure that
// there is no unwanted within-object padding.

// Note: Rust doesn't have a direct equivalent to `#pragma pack`.
// Memory layout control is generally handled through explicit struct definitions
// and potentially using crates like `repr_offset`.
// The `-Wpadded` compiler warning is also not directly translatable to Rust,
// as Rust's compiler provides different mechanisms for ensuring memory layout
// correctness.

macro_rules! v8_object_push {
    () => {
        // TODO: Implement struct packing if needed.
    };
}

macro_rules! v8_object_pop {
    () => {
        // TODO: Implement struct packing cleanup if needed.
    };
}

macro_rules! v8_object {
    () => {
        v8_object_push!();
    };
}

macro_rules! v8_object_end {
    () => {
        v8_object_pop!();
        static_assert!(true);
    };
}

macro_rules! v8_object_inner_class {
    () => {
        v8_object_pop!();
    };
}

macro_rules! v8_object_inner_class_end {
    () => {
        v8_object_push!();
        static_assert!(true);
    };
}

// Since this changes visibility, it should always be last in a class
// definition.

macro_rules! object_constructors {
    ($Type:ident, $($arg:tt)*) => {
        impl $Type {
            pub const fn new() -> Self {
                Self { $($arg)* }
            }

            /* For every object, add a `->` operator which returns a pointer to this
               object. This will allow smoother transition between T and Tagged<T>. */
            pub fn as_ptr(&self) -> *const Self {
                self as *const Self
            }

            pub fn as_mut_ptr(&mut self) -> *mut Self {
                self as *mut Self
            }
        }

        impl $Type {
            // Special constructor for constexpr construction which allows skipping type
            // checks.
            // explicit constexpr V8_INLINE Type(Address ptr, HeapObject::SkipTypeCheckTag)
            //     : __VA_ARGS__(ptr, HeapObject::SkipTypeCheckTag()) {}

            fn check_type_on_cast(&self) {
                // SLOW_DCHECK(Is##Type(*this));
                unimplemented!()
            }
            // explicit inline Type(Address ptr)
            // Assuming Address is usize or similar
            // impl From<usize> for $Type {
            //     fn from(ptr: usize) -> Self {
            //         let mut s = Self {  };
            //         s.check_type_on_cast();
            //         s
            //     }
            // }
        }
    };
}

macro_rules! object_constructors_impl {
    ($Type:ident, $Super:ident) => {
        impl $Type {
            fn check_type_on_cast(&self) {
                unimplemented!()
            }

            // Assuming Address is usize or similar and Super implements From<usize>
            // impl From<usize> for $Type {
            //     fn from(ptr: usize) -> Self {
            //         let mut s = $Super::from(ptr);
            //         Self{ super_field: s, ..Default::default() }
            //     }
            // }
        }
    };
}

macro_rules! never_read_only_space {
    () => {
        //   inline Heap* GetHeap() const;
        //   inline Isolate* GetIsolate() const;
        impl SomeType { //replace SomeType with the actual type
            fn get_heap(&self) -> *mut u8 { //replace u8 with the correct type
                unimplemented!()
            }
            fn get_isolate(&self) -> *mut u8 { //replace u8 with the correct type
                unimplemented!()
            }
        }
    };
}

macro_rules! never_read_only_space_impl {
    ($Type:ident) => {
        impl $Type {
            fn get_heap(&self) -> *mut u8 { //replace u8 with the correct type
                // return GetHeapFromWritableObject(*this);
                unimplemented!()
            }
            fn get_isolate(&self) -> *mut u8 { //replace u8 with the correct type
                // return GetIsolateFromWritableObject(*this);
                unimplemented!()
            }
        }
    };
}

macro_rules! decl_primitive_getter {
    ($name:ident, $type:ty) => {
        fn $name(&self) -> $type;
    };
}

macro_rules! decl_primitive_setter {
    ($name:ident, $type:ty) => {
        fn set_$name(&mut self, value: $type);
    };
}

macro_rules! decl_primitive_accessors {
    ($name:ident, $type:ty) => {
        decl_primitive_getter!($name, $type);
        decl_primitive_setter!($name, $type);
    };
}

macro_rules! decl_boolean_accessors {
    ($name:ident) => {
        decl_primitive_accessors!($name, bool);
    };
}

macro_rules! decl_int_accessors {
    ($name:ident) => {
        decl_primitive_accessors!($name, i32);
    };
}

macro_rules! decl_int32_accessors {
    ($name:ident) => {
        decl_primitive_accessors!($name, i32);
    };
}

macro_rules! decl_sandboxed_pointer_accessors {
    ($name:ident, $type:ty) => {
        decl_primitive_getter!($name, $type);
        decl_primitive_setter!($name, $type);
    };
}

macro_rules! decl_uint16_accessors {
    ($name:ident) => {
        decl_primitive_accessors!($name, u16);
    };
}

macro_rules! decl_int16_accessors {
    ($name:ident) => {
        decl_primitive_accessors!($name, i16);
    };
}

macro_rules! decl_uint8_accessors {
    ($name:ident) => {
        decl_primitive_accessors!($name, u8);
    };
}

macro_rules! decl_relaxed_primitive_accessors {
    ($name:ident, $type:ty) => {
        fn $name(&self, tag: RelaxedLoadTag) -> $type;
        fn set_$name(&mut self, value: $type, tag: RelaxedStoreTag);
    };
}

macro_rules! decl_relaxed_int32_accessors {
    ($name:ident) => {
        decl_relaxed_primitive_accessors!($name, i32);
    };
}

macro_rules! decl_relaxed_uint32_accessors {
    ($name:ident) => {
        decl_relaxed_primitive_accessors!($name, u32);
    };
}

macro_rules! decl_relaxed_uint16_accessors {
    ($name:ident) => {
        decl_relaxed_primitive_accessors!($name, u16);
    };
}

macro_rules! decl_relaxed_uint8_accessors {
    ($name:ident) => {
        decl_relaxed_primitive_accessors!($name, u8);
    };
}

macro_rules! decl_getter {
    ($name:ident, $($arg:tt)*) => {
        fn $name(&self) -> $($arg)*;
        fn $name_cage(&self, cage_base: PtrComprCageBase) -> $($arg)*;
    };
}

macro_rules! def_getter {
    ($holder:ident, $name:ident, $($arg:tt)*) => {
        impl $holder {
            fn $name(&self) -> $($arg)* {
                let cage_base = self.get_ptr_compr_cage_base();
                self.$name_cage(cage_base)
            }

            fn $name_cage(&self, cage_base: PtrComprCageBase) -> $($arg)* {
                // Implementation here
                unimplemented!()
            }
        }
    };
}

macro_rules! def_relaxed_getter {
    ($holder:ident, $name:ident, $($arg:tt)*) => {
        impl $holder {
            fn $name(&self, tag: RelaxedLoadTag) -> $($arg)* {
                let cage_base = self.get_ptr_compr_cage_base();
                self.$name_cage(cage_base, tag)
            }

            fn $name_cage(&self, cage_base: PtrComprCageBase, tag: RelaxedLoadTag) -> $($arg)* {
                // Implementation here
                unimplemented!()
            }
        }
    };
}

macro_rules! def_acquire_getter {
    ($holder:ident, $name:ident, $($arg:tt)*) => {
        impl $holder {
            fn $name(&self, tag: AcquireLoadTag) -> $($arg)* {
                let cage_base = self.get_ptr_compr_cage_base();
                self.$name_cage(cage_base, tag)
            }

            fn $name_cage(&self, cage_base: PtrComprCageBase, tag: AcquireLoadTag) -> $($arg)* {
                // Implementation here
                unimplemented!()
            }
        }
    };
}

// TODO(leszeks): Add checks in the factory that we never allocate these
// objects in RO space.

macro_rules! def_heap_object_predicate {
    ($holder:ident, $name:ident) => {
        fn $name(obj: Tagged<$holder>) -> bool {
            let cage_base = obj.get_ptr_compr_cage_base();
            $name(obj, cage_base)
        }

        fn $name(obj: Tagged<$holder>, cage_base: PtrComprCageBase) -> bool {
            // Implementation here
            unimplemented!()
        }
    };
}

macro_rules! tq_field_type {
    ($name:ident, $tq_type:expr) => {
        const K_$name_TQ_FIELD_TYPE: &'static str = $tq_type;
    };
}

macro_rules! decl_field_offset_tq {
    ($name:ident, $value:expr, $tq_type:expr) => {
        const K_$name_OFFSET: i32 = $value;
        tq_field_type!($name, $tq_type);
    };
}

macro_rules! decl_setter {
    ($name:ident, $($arg:tt)*) => {
        fn set_$name(&mut self, value: $($arg)*, mode: WriteBarrierMode);
    };
}

macro_rules! decl_accessors {
    ($name:ident, $($arg:tt)*) => {
        decl_getter!($name, $($arg)*);
        decl_setter!($name, $($arg)*);
    };
}

macro_rules! decl_accessors_load_tag {
    ($name:ident, $type:ty, $tag_type:ty) => {
        fn $name(&self, tag: $tag_type) -> $type;
        fn $name_cage(&self, cage_base: PtrComprCageBase, tag: $tag_type) -> $type;
    };
}

macro_rules! decl_accessors_store_tag {
    ($name:ident, $type:ty, $tag_type:ty) => {
        fn set_$name(&mut self, value: $type, tag: $tag_type, mode: WriteBarrierMode);
    };
}

macro_rules! decl_relaxed_getter {
    ($name:ident, $($arg:tt)*) => {
        decl_accessors_load_tag!($name, $($arg)*, RelaxedLoadTag);
    };
}

macro_rules! decl_relaxed_setter {
    ($name:ident, $($arg:tt)*) => {
        decl_accessors_store_tag!($name, $($arg)*, RelaxedStoreTag);
    };
}

macro_rules! decl_relaxed_accessors {
    ($name:ident, $($arg:tt)*) => {
        decl_relaxed_getter!($name, $($arg)*);
        decl_relaxed_setter!($name, $($arg)*);
    };
}

macro_rules! decl_acquire_getter {
    ($name:ident, $($arg:tt)*) => {
        decl_accessors_load_tag!($name, $($arg)*, AcquireLoadTag);
    };
}

macro_rules! decl_release_setter {
    ($name:ident, $($arg:tt)*) => {
        decl_accessors_store_tag!($name, $($arg)*, ReleaseStoreTag);
    };
}

macro_rules! decl_release_acquire_accessors {
    ($name:ident, $($arg:tt)*) => {
        decl_acquire_getter!($name, $($arg)*);
        decl_release_setter!($name, $($arg)*);
    };
}

macro_rules! def_primitive_accessors {
    ($holder:ident, $name:ident, $offset:expr, $type:ty) => {
        impl $holder {
            fn $name(&self) -> $type {
                // return ReadField::<$type>($offset);
                unimplemented!()
            }
            fn set_$name(&mut self, value: $type) {
                // WriteField::<$type>($offset, value);
                unimplemented!()
            }
        }
    };
}

macro_rules! int_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        def_primitive_accessors!($holder, $name, $offset, i32);
    };
}

macro_rules! int32_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        def_primitive_accessors!($holder, $name, $offset, i32);
    };
}

macro_rules! uint16_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        def_primitive_accessors!($holder, $name, $offset, u16);
    };
}

macro_rules! uint8_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        def_primitive_accessors!($holder, $name, $offset, u8);
    };
}

macro_rules! relaxed_int32_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> i32 {
                // RELAXED_READ_INT32_FIELD(*this, offset);
                unimplemented!()
            }
            fn set_$name(&mut self, value: i32, _tag: RelaxedStoreTag) {
                // RELAXED_WRITE_INT32_FIELD(*this, offset, value);
                unimplemented!()
            }
        }
    };
}

macro_rules! relaxed_uint32_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> u32 {
                // RELAXED_READ_UINT32_FIELD(*this, offset);
                unimplemented!()
            }
            fn set_$name(&mut self, value: u32, _tag: RelaxedStoreTag) {
                // RELAXED_WRITE_UINT32_FIELD(*this, offset, value);
                unimplemented!()
            }
        }
    };
}

macro_rules! relaxed_uint16_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> u16 {
                // RELAXED_READ_UINT16_FIELD(*this, offset);
                unimplemented!()
            }
            fn set_$name(&mut self, value: u16, _tag: RelaxedStoreTag) {
                // RELAXED_WRITE_UINT16_FIELD(*this, offset, value);
                unimplemented!()
            }
        }
    };
}

macro_rules! relaxed_uint8_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, _tag: RelaxedLoadTag) -> u8 {
                // RELAXED_READ_UINT8_FIELD(*this, offset);
                unimplemented!()
            }
            fn set_$name(&mut self, value: u8, _tag: RelaxedStoreTag) {
                // RELAXED_WRITE_UINT8_FIELD(*this, offset, value);
                unimplemented!()
            }
        }
    };
}

macro_rules! accessors_checked2 {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr, $set_condition:expr) => {
        impl $holder {
            fn $name(&self) -> $type {
                let cage_base = self.get_ptr_compr_cage_base();
                self.$name_cage(cage_base)
            }

            fn $name_cage(&self, cage_base: PtrComprCageBase) -> $type {
                // UNPAREN(type)
                // value = TaggedField::<$type, $offset>::load(cage_base, *this);
                // DCHECK(get_condition);
                unimplemented!()
            }
            fn set_$name(&mut self, value: $type, mode: WriteBarrierMode) {
                // DCHECK(set_condition);
                // TaggedField::<$type, $offset>::store(*this, value);
                // CONDITIONAL_WRITE_BARRIER(*this, $offset, value, mode);
                unimplemented!()
            }
        }
    };
}

macro_rules! accessors_checked {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $condition:expr) => {
        accessors_checked2!($holder, $name, $type, $offset, $condition, $condition);
    };
}

macro_rules! accessors {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        accessors_checked!($holder, $name, $type, $offset, true);
    };
}

// TODO(jgruber): Eventually, all accessors should be ported to the NOCAGE
// variant (which doesn't define a PtrComprCageBase overload). Once that's
// done, remove the cage-ful macros (e.g. ACCESSORS) and rename the cage-less
// macros (e.g. ACCESSORS_NOCAGE).
macro_rules! accessors_nocage {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        impl $holder {
            fn $name(&self) -> $type {
                let cage_base = self.get_ptr_compr_cage_base();
                // TaggedField::<$type, $offset>::load(cage_base, *this);
                unimplemented!()
            }
            fn set_$name(&mut self, value: $type, mode: WriteBarrierMode) {
                // TaggedField::<$type, $offset>::store(*this, value);
                // CONDITIONAL_WRITE_BARRIER(*this, $offset, value, mode);
                unimplemented!()
            }
        }
    };
}

macro_rules! rename_torque_accessors {
    ($holder:ident, $name:ident, $torque_name:ident, $type:ty) => {
        impl $holder {
            fn $name(&self) -> $type {
                TorqueGeneratedClass::$torque_name()
            }
            fn $name_cage(&self, cage_base: PtrComprCageBase) -> $type {
                TorqueGeneratedClass::$torque_name_cage(cage_base)
            }
            fn set_$name(&mut self, value: $type, mode: WriteBarrierMode) {
                TorqueGeneratedClass::set_$torque_name(value, mode);
            }
        }
    };
}

macro_rules! rename_primitive_torque_accessors {
    ($holder:ident, $name:ident, $torque_name:ident, $type:ty) => {
        impl $holder {
            fn $name(&self) -> $type {
                TorqueGeneratedClass::$torque_name()
            }
            fn set_$name(&mut self, value: $type) {
                TorqueGeneratedClass::set_$torque_name(value);
            }
        }
    };
}

macro_rules! accessors_relaxed_checked2 {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr, $set_condition:expr) => {
        impl $holder {
             fn $name(&self) -> $type {
                let cage_base = self.get_ptr_compr_cage_base();
                self.$name_cage(cage_base)
            }
            fn $name_cage(&self, cage_base: PtrComprCageBase) -> $type {
                // type value = TaggedField::<$type, $offset>::Relaxed_Load(cage_base, *this);
                // DCHECK(get_condition);
                unimplemented!()
            }
            fn set_$name(&mut self, value: $type, mode: WriteBarrierMode) {
                // DCHECK(set_condition);
                // TaggedField::<$type, $offset>::Relaxed_Store(*this, value);
                // CONDITIONAL_WRITE_BARRIER(*this, $offset, value, mode);
                unimplemented!()
            }
        }
    };
}

macro_rules! accessors_relaxed_checked {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $condition:expr) => {
        accessors_relaxed_checked2!($holder, $name, $type, $offset, $condition, $condition);
    };
}

macro_rules! accessors_relaxed {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        accessors_relaxed_checked!($holder, $name, $type, $offset, true);
    };
}

// Similar to ACCESSORS_RELAXED above but with respective relaxed tags.
macro_rules! relaxed_accessors_checked2 {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr, $set_condition:expr) => {
        impl $holder {
            fn $name(&self, tag: RelaxedLoadTag) -> $type {
                let cage_base = self.get_ptr_compr_cage_base();
                self.$name_cage(cage_base, tag)
            }

            fn $name_cage(&self, cage_base: PtrComprCageBase, tag: RelaxedLoadTag) -> $type {
                // UNPAREN(type)
                // value = TaggedField::<$type, $offset>::Relaxed_Load(cage_base, *this);
                // DCHECK(get_condition);
                unimplemented!()
            }
            fn set_$name(&mut self, value: $type, tag: RelaxedStoreTag, mode: WriteBarrierMode) {
                // DCHECK(set_condition);
                // TaggedField::<$type, $offset>::Relaxed_Store(*this, value);
                // CONDITIONAL_WRITE_BARRIER(*this, $offset, value, mode);
                unimplemented!()
            }
        }
    };
}

macro_rules! relaxed_accessors_checked {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $condition:expr) => {
        relaxed_accessors_checked2!($holder, $name, $type, $offset, $condition, $condition);
    };
}

macro_rules! relaxed_accessors {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        relaxed_accessors_checked!($holder, $name, $type, $offset, true);
    };
}

macro_rules! release_acquire_getter_checked {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr) => {
        impl $holder {
           fn $name(&self, tag: AcquireLoadTag) -> $type {
                let cage_base = self.get_ptr_compr_cage_base();
                self.$name_cage(cage_base, tag)
            }

            fn $name_cage(&self, cage_base: PtrComprCageBase, tag: AcquireLoadTag) -> $type {
                // UNPAREN(type)
                // value = TaggedField::<$type, $offset>::Acquire_Load(cage_base, *this);
                // DCHECK(get_condition);
                unimplemented!()
            }
        }
    };
}

macro_rules! release_acquire_setter_checked {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $set_condition:expr) => {
        impl $holder {
            fn set_$name(&mut self, value: $type, tag: ReleaseStoreTag, mode: WriteBarrierMode) {
                // DCHECK(set_condition);
                // TaggedField::<$type, $offset>::Release_Store(*this, value);
                // CONDITIONAL_WRITE_BARRIER(*this, $offset, value, mode);
                unimplemented!()
            }
        }
    };
}

macro_rules! release_acquire_accessors_checked2 {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $get_condition:expr, $set_condition:expr) => {
        release_acquire_getter_checked!($holder, $name, $type, $offset, $get_condition);
        release_acquire_setter_checked!($holder, $name, $type, $offset, $set_condition);
    };
}

macro_rules! release_acquire_accessors_checked {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $condition:expr) => {
        release_acquire_accessors_checked2!($holder, $name, $type, $offset, $condition, $condition);
    };
}

macro_rules! release_acquire_accessors {
    ($holder:ident, $name:ident, $type:ty, $offset:expr) => {
        release_acquire_accessors_checked!($holder, $name, $type, $offset, true);
    };
}

// Getter that returns a Smi as an int and writes an int as a Smi.
macro_rules! smi_accessors_checked {
    ($holder:ident, $name:ident, $offset:expr, $condition:expr) => {
        impl $holder {
            fn $name(&self) -> i32 {
                // DCHECK(condition);
                // Tagged::<Smi> value = TaggedField::<Smi, offset>::load(*this);
                // value.value()
                unimplemented!()
            }
            fn set_$name(&mut self, value: i32) {
                // DCHECK(condition);
                // TaggedField::<Smi, offset>::store(*this, Smi::FromInt(value));
                unimplemented!()
            }
        }
    };
}

macro_rules! smi_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        smi_accessors_checked!($holder, $name, $offset, true);
    };
}

macro_rules! decl_release_acquire_int_accessors {
    ($name:ident) => {
        fn $name(&self, tag: AcquireLoadTag) -> i32;
        fn set_$name(&mut self, value: i32, tag: ReleaseStoreTag);
    };
}

macro_rules! release_acquire_smi_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, tag: AcquireLoadTag) -> i32 {
                // Tagged::<Smi> value = TaggedField::<Smi, offset>::Acquire_Load(*this);
                // value.value()
                unimplemented!()
            }
            fn set_$name(&mut self, value: i32, tag: ReleaseStoreTag) {
                // TaggedField::<Smi, offset>::Release_Store(*this, Smi::FromInt(value));
                unimplemented!()
            }
        }
    };
}

macro_rules! decl_relaxed_int_accessors {
    ($name:ident) => {
        fn $name(&self, tag: RelaxedLoadTag) -> i32;
        fn set_$name(&mut self, value: i32, tag: RelaxedStoreTag);
    };
}

macro_rules! relaxed_smi_accessors {
    ($holder:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, tag: RelaxedLoadTag) -> i32 {
                // Tagged::<Smi> value = TaggedField::<Smi, offset>::Relaxed_Load(*this);
                // value.value()
                unimplemented!()
            }
            fn set_$name(&mut self, value: i32, tag: RelaxedStoreTag) {
                // TaggedField::<Smi, offset>::Relaxed_Store(*this, Smi::FromInt(value));
                unimplemented!()
            }
        }
    };
}

macro_rules! bool_getter {
    ($holder:ident, $field:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self) -> bool {
                BooleanBit::get(self.$field(), $offset)
            }
        }
    };
}

macro_rules! bool_accessors {
    ($holder:ident, $field:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self) -> bool {
                BooleanBit::get(self.$field(), $offset)
            }
            fn set_$name(&mut self, value: bool) {
                self.set_$field(BooleanBit::set(self.$field(), $offset, value));
            }
        }
    };
}

macro_rules! decl_relaxed_bool_accessors {
    ($name:ident) => {
        fn $name(&self, tag: RelaxedLoadTag) -> bool;
        fn set_$name(&mut self, value: bool, tag: RelaxedStoreTag);
    };
}

macro_rules! relaxed_bool_accessors {
    ($holder:ident, $field:ident, $name:ident, $offset:expr) => {
        impl $holder {
            fn $name(&self, tag: RelaxedLoadTag) -> bool {
                BooleanBit::get(self.$field(RelaxedLoadTag {}), $offset)
            }
            fn set_$name(&mut self, value: bool, tag: RelaxedStoreTag) {
                self.set_$field(BooleanBit::set(self.$field(RelaxedLoadTag {}), $offset, value), tag);
            }
        }
    };
}

// Host objects in ReadOnlySpace can't define the isolate-less accessor.
macro_rules! decl_external_pointer_accessors_maybe_read_only_host {
    ($name:ident, $type:ty) => {
        fn $name(&self, isolate: IsolateForSandbox) -> $type;
        fn init_$name(&mut self, isolate: IsolateForSandbox, initial_value: $type);
        fn set_$name(&mut self, isolate: IsolateForSandbox, value: $type);
    };
}

// Host objects in ReadOnlySpace can't define the isolate-less accessor.
macro_rules! external_pointer_accessors_maybe_read_only_host {
    ($holder:ident, $name:ident, $type:ty, $offset:expr, $tag:ident) => {
        impl $holder {
            fn $name(&self, isolate: IsolateForSandbox) -> $type {
                //This is a workaround for MSVC error C2440 not allowing
                //reinterpret casts to the same type.
                // struct C2440 {};
                // Address result = HeapObject::ReadExternalPointerField::<$tag>