// Converted from V8 C++ source files:
// Header: heap-object-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap_object_inl {
    use crate::common::ptr_compr_inl::PtrComprCageBase;
    use crate::objects::heap_object::HeapObject;
    use crate::objects::instance_type_inl::InstanceTypeChecker;
    use crate::objects::objects::Map;
    use crate::objects::objects_inl::Tagged;

    macro_rules! type_checker {
        ($type:ident, $($args:tt)*) => {
            pub fn is_$type(obj: Tagged<HeapObject>) -> bool {
                let cage_base = PtrComprCageBase {};
                is_$type(obj, cage_base)
            }

            pub fn is_$type(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
                let map_object = obj.map(cage_base);
                InstanceTypeChecker::is_$type(map_object)
            }
        };
    }

    macro_rules! instance_type_checkers {
        ($macro:ident) => {
            $macro!(array);
            $macro!(string);
            $macro!(number);
            $macro!(boolean);
            $macro!(symbol);
            $macro!(function);
            $macro!(object);
            $macro!(regexp);
            $macro!(date);
            $macro!(proxy);
            $macro!(weakref);
        };
    }

    instance_type_checkers!(type_checker);

    pub fn is_array(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_array(obj, cage_base)
    }

    pub fn is_array(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_array(map_object)
    }

    pub fn is_string(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_string(obj, cage_base)
    }

    pub fn is_string(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_string(map_object)
    }
    
    pub fn is_number(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_number(obj, cage_base)
    }

    pub fn is_number(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_number(map_object)
    }

    pub fn is_boolean(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_boolean(obj, cage_base)
    }

    pub fn is_boolean(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_boolean(map_object)
    }

    pub fn is_symbol(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_symbol(obj, cage_base)
    }

    pub fn is_symbol(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_symbol(map_object)
    }

    pub fn is_function(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_function(obj, cage_base)
    }

    pub fn is_function(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_function(map_object)
    }

     pub fn is_object(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_object(obj, cage_base)
    }

    pub fn is_object(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_object(map_object)
    }

    pub fn is_regexp(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_regexp(obj, cage_base)
    }

    pub fn is_regexp(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_regexp(map_object)
    }

    pub fn is_date(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_date(obj, cage_base)
    }

    pub fn is_date(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_date(map_object)
    }

    pub fn is_proxy(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_proxy(obj, cage_base)
    }

    pub fn is_proxy(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_proxy(map_object)
    }

     pub fn is_weakref(obj: Tagged<HeapObject>) -> bool {
        let cage_base = PtrComprCageBase {};
        is_weakref(obj, cage_base)
    }

    pub fn is_weakref(obj: Tagged<HeapObject>, cage_base: PtrComprCageBase) -> bool {
        let map_object = obj.map(cage_base);
        InstanceTypeChecker::is_weakref(map_object)
    }
}
