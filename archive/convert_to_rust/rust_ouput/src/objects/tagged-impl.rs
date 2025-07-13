// Converted from V8 C++ source files:
// Header: tagged-impl.h
// Implementation: tagged-impl.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod tagged_impl {
    #![allow(dead_code)]
    #![allow(mutable_transmutes)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(missing_docs)]
    use crate::base::export_template::EXPORT_TEMPLATE_DEFINE;
    use crate::base::export_template::EXPORT_TEMPLATE_DECLARE;
    use crate::objects::js_duration_format::FieldStyle;
    use crate::objects::js_duration_format::V8;
    use crate::objects::js_list_format_inl::Tagged;
    use crate::objects::js_segment_iterator_inl::GranularityBits;
    use crate::objects::string::v8;
    use crate::objects::tagged_field::HeapObject;
    use crate::objects::tagged_field::Object;
    use crate::objects::tagged_field::Smi;
    use crate::objects::tagged_impl_inl::TaggedField;
    use crate::objects::tagged_impl_inl::UseScratchRegisterScope;
    use crate::objects::objects::Address;
    use crate::objects::objects::code;
    use crate::objects::objects::If;
    use crate::objects::objects::This;
    use crate::codegen::code_stub_assembler::isolate;
    use crate::wasm::module_decoder_impl::OFStream;
    use crate::codegen::loong64::assembler_loong64::StdoutStream;
    use crate::codegen::code_stub_assembler_inl::Union;
    use std::any::Any;
    use std::io::Write;
    use std::mem::size_of;
    use std::{fmt, io};

    pub const kSystemPointerSize: usize = std::mem::size_of::<usize>();

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum HeapObjectReferenceType {
        STRONG,
        WEAK,
    }

    pub type Tagged_t = usize;

    const kClearedWeakHeapObjectLower32: u32 = 0x0;

    macro_rules! HAS_SMI_TAG {
        ($ptr_:expr) => {
            ($ptr_ as usize) & 1 == 1
        };
    }
    macro_rules! HAS_STRONG_HEAP_OBJECT_TAG {
        ($ptr_:expr) => {
            ($ptr_ as usize) & 1 == 0
        };
    }
    macro_rules! HAS_WEAK_HEAP_OBJECT_TAG {
        ($ptr_:expr) => {
            false
        };
    }

    pub struct Isolate {}

    pub struct PtrComprCageBase {}

    pub struct V8HeapCompressionScheme {}
    impl V8HeapCompressionScheme {
        pub fn GetPtrComprCageBaseAddress(_ptr: usize) -> usize {
            0
        }
        pub fn base() -> usize {
            0
        }
    }

    pub struct DirectHandle<T> {
        dummy : i32
    }

    pub struct FixedArray {}

    pub struct Script {}

    pub struct Operand {}

    pub struct Register {}
    pub struct CPURegister {}

    pub struct OpIndex {}

    pub struct InstructionOperand {}

    pub struct WasmInternalFunction {}

    pub struct MachineType {}
    pub struct IrregexpImplementation {}

    pub struct OperationType {}
    pub enum Condition {}
    pub enum Bytecode {}
    pub struct Macro {}
    pub struct Identifier {}

    pub struct IndirectHandle<T> {
        dummy : i32
    }

    pub struct Local<'a, T> {
        dummy : i32,
        _marker: std::marker::PhantomData<&'a T>,
    }

    pub struct Context {}

    pub struct Function {}

    pub struct Error {}

    #[repr(C)]
    pub struct TaggedImpl<const kRefType: HeapObjectReferenceType, StorageType> {
        ptr_: StorageType,
    }

    impl<const kRefType: HeapObjectReferenceType, StorageType> TaggedImpl<kRefType, StorageType> {
        pub const kIsFull: bool = size_of::<StorageType>() == size_of::<usize>();
        pub const kCanBeWeak: bool =
            match kRefType {
                HeapObjectReferenceType::WEAK => true,
                HeapObjectReferenceType::STRONG => false,
            };

        #[inline]
        pub const fn new() -> Self {
            Self { ptr_: unsafe { std::mem::zeroed() } }
        }
        #[inline]
        pub const fn new_with_ptr(ptr: StorageType) -> Self {
            Self { ptr_: ptr }
        }

        // Make clang on Linux catch what MSVC complains about on Windows:
        //explicit operator bool() const = delete;

        // Don't use this operator for comparing with stale or invalid pointers
        // because CheckObjectComparisonAllowed() might crash when trying to access
        // the object's page header. Use SafeEquals() instead.
        pub fn operator_equal<const kOtherRefType: HeapObjectReferenceType, U>(
            &self,
            other: &TaggedImpl<kOtherRefType, U>,
        ) -> bool
        where
            U: Copy + PartialEq,
            StorageType: Copy + PartialEq,
        {
            assert!(size_of::<U>() == size_of::<usize>() || size_of::<U>() == size_of::<Tagged_t>());
            assert!(size_of::<StorageType>() == size_of::<usize>() || size_of::<StorageType>() == size_of::<Tagged_t>());

            if cfg!(any(feature = "V8_EXTERNAL_CODE_SPACE", feature = "V8_ENABLE_SANDBOX")) {
                if size_of::<StorageType>() == size_of::<usize>() && size_of::<U>() == size_of::<usize>() {
                    if let Some(a) = any_as_address(self.ptr_) {
                        if let Some(b) = any_as_address(other.ptr_) {
                            if !check_object_comparison_allowed(a,b) {
                                return false;
                            }
                        }
                    }
                }
            }
            let self_ptr_value: Tagged_t = any_as_tagged_t(self.ptr_).unwrap_or_default();
            let other_ptr_value: Tagged_t = any_as_tagged_t(other.ptr_).unwrap_or_default();
            self_ptr_value == other_ptr_value
        }

        // Don't use this operator for comparing with stale or invalid pointers
        // because CheckObjectComparisonAllowed() might crash when trying to access
        // the object's page header. Use SafeEquals() instead.
        pub fn operator_not_equal<const kOtherRefType: HeapObjectReferenceType, U>(
            &self,
            other: &TaggedImpl<kOtherRefType, U>,
        ) -> bool
        where
            U: Copy + PartialEq,
            StorageType: Copy + PartialEq,
        {
            assert!(size_of::<U>() == size_of::<usize>() || size_of::<U>() == size_of::<Tagged_t>());
            assert!(size_of::<StorageType>() == size_of::<usize>() || size_of::<StorageType>() == size_of::<Tagged_t>());
            if cfg!(any(feature = "V8_EXTERNAL_CODE_SPACE", feature = "V8_ENABLE_SANDBOX")) {
                if size_of::<StorageType>() == size_of::<usize>() && size_of::<U>() == size_of::<usize>() {
                    if let Some(a) = any_as_address(self.ptr_) {
                        if let Some(b) = any_as_address(other.ptr_) {
                            if !check_object_comparison_allowed(a,b) {
                                return false;
                            }
                        }
                    }
                }
            }
            let self_ptr_value: Tagged_t = any_as_tagged_t(self.ptr_).unwrap_or_default();
            let other_ptr_value: Tagged_t = any_as_tagged_t(other.ptr_).unwrap_or_default();
            self_ptr_value != other_ptr_value
        }

        // A variant of operator== which allows comparing objects in different
        // pointer compression cages. In particular, this should be used when
        // comparing objects in trusted- or code space with objects in the main
        // pointer compression cage.
        pub fn safe_equals<const kOtherRefType: HeapObjectReferenceType>(
            &self,
            other: &TaggedImpl<kOtherRefType, StorageType>,
        ) -> bool
        where
            StorageType: Copy + PartialEq,
        {
            assert_eq!(size_of::<StorageType>(), size_of::<usize>(), "Safe comparison is allowed only for full tagged values");
            if cfg!(feature = "V8_EXTERNAL_CODE_SPACE_BOOL") || cfg!(feature = "V8_ENABLE_SANDBOX_BOOL") {
                if let Some(self_ptr) = any_as_address(self.ptr_) {
                    if let Some(other_ptr) = any_as_address(other.ptr_) {
                        return self_ptr == other_ptr;
                    }
                }

            }
            self.operator_equal(other)
        }

        // For using in std::set and std::map.
        pub fn operator_less_than(&self, other: &Self) -> bool
        where
            StorageType: Copy,
        {
            if cfg!(any(feature = "V8_EXTERNAL_CODE_SPACE", feature = "V8_ENABLE_SANDBOX")) {
                if size_of::<StorageType>() == size_of::<usize>() {
                    if let Some(a) = any_as_address(self.ptr_) {
                        if let Some(b) = any_as_address(other.ptr_) {
                            if !check_object_comparison_allowed(a,b) {
                                return false;
                            }
                        }
                    }
                }
            }

            let self_ptr_value: Tagged_t = any_as_tagged_t(self.ptr_).unwrap_or_default();
            let other_ptr_value: Tagged_t = any_as_tagged_t(other.ptr_).unwrap_or_default();

            self_ptr_value < other_ptr_value
        }

        #[inline]
        pub const fn ptr(&self) -> StorageType
        where
            StorageType: Copy,
        {
            self.ptr_
        }

        // Returns true if this tagged value is a strong pointer to a HeapObject or
        // Smi.
        #[inline]
        pub const fn is_object(&self) -> bool {
            !self.is_weak_or_cleared()
        }

        // Returns true if this tagged value is a Smi.
        pub const fn is_smi(&self) -> bool {
            HAS_SMI_TAG!(self.ptr_)
        }

        pub fn to_smi(&self) -> Result<Tagged<Smi>, &'static str> {
            if self.is_smi() {
                let value = unsafe { std::mem::transmute_copy(&self.ptr_) };
                Ok(Tagged { dummy: value })
            } else {
                Err("Not a Smi")
            }
        }
        

        // Returns true if this tagged value is a strong pointer to a HeapObject.
        #[inline]
        pub const fn is_heap_object(&self) -> bool {
            self.is_strong()
        }

        // Returns true if this tagged value is a cleared weak reference.
        #[inline]
        pub const fn is_cleared(&self) -> bool {
            Self::kCanBeWeak
                && any_as_tagged_t(self.ptr_).unwrap_or_default() as u32 == kClearedWeakHeapObjectLower32
        }

        // Returns true if this tagged value is a strong or weak pointer to a
        // HeapObject.
        #[inline]
        pub const fn is_strong_or_weak(&self) -> bool {
            !self.is_smi() && !self.is_cleared()
        }

        // Returns true if this tagged value is a strong pointer to a HeapObject.
        #[inline]
        pub const fn is_strong(&self) -> bool {
            if Self::kCanBeWeak {
               HAS_STRONG_HEAP_OBJECT_TAG!(self.ptr_)
            } else {
                !self.is_smi()
            }
        }

        // Returns true if this tagged value is a strong pointer to a HeapObject, or a
        // Smi.
        #[inline]
        pub const fn is_strong_or_smi(&self) -> bool {
            !Self::kCanBeWeak || !HAS_WEAK_HEAP_OBJECT_TAG!(self.ptr_)
        }

        // Returns true if this tagged value is a weak pointer to a HeapObject.
        #[inline]
        pub const fn is_weak(&self) -> bool {
            self.is_weak_or_cleared() && !self.is_cleared()
        }

        // Returns true if this tagged value is a weak pointer to a HeapObject or
        // cleared weak reference.
        #[inline]
        pub const fn is_weak_or_cleared(&self) -> bool {
            Self::kCanBeWeak && HAS_WEAK_HEAP_OBJECT_TAG!(self.ptr_)
        }

        #[cfg(feature = "V8_COMPRESS_POINTERS")]
        #[inline]
        pub fn is_in_main_cage_base(&self) -> bool {
            assert!(!self.is_smi());
            using S = V8HeapCompressionScheme;
            S::GetPtrComprCageBaseAddress(any_as_tagged_t(self.ptr_).unwrap_or_default()) == S::GetPtrComprCageBaseAddress(S::base())
        }

        pub fn get_heap_object_if_strong(
            &self,
        ) -> Option<Tagged<HeapObject>> {
            if self.is_strong() {
                Some(Tagged { dummy: 0 })
            } else {
                None
            }
        }
        pub fn get_heap_object_if_strong_with_isolate(
            &self,
            _isolate: *mut Isolate,
        ) -> Option<Tagged<HeapObject>> {
            if self.is_strong() {
                Some(Tagged { dummy: 0 })
            } else {
                None
            }
        }

        pub fn get_heap_object_assume_strong(&self) -> Tagged<HeapObject> {
            assert!(self.is_strong());
            Tagged { dummy: 0 }
        }

        pub fn get_heap_object_assume_strong_with_isolate(
            &self,
            _isolate: *mut Isolate,
        ) -> Tagged<HeapObject> {
            assert!(self.is_strong());
            Tagged { dummy: 0 }
        }

        pub fn get_heap_object_if_weak(
            &self,
        ) -> Option<Tagged<HeapObject>> {
            if self.is_weak() {
                Some(Tagged { dummy: 0 })
            } else {
                None
            }
        }

        pub fn get_heap_object_if_weak_with_isolate(
            &self,
            _isolate: *mut Isolate,
        ) -> Option<Tagged<HeapObject>> {
            if self.is_weak() {
                Some(Tagged { dummy: 0 })
            } else {
                None
            }
        }

        pub fn get_heap_object_assume_weak(&self) -> Tagged<HeapObject> {
            assert!(self.is_weak());
            Tagged { dummy: 0 }
        }

        pub fn get_heap_object_assume_weak_with_isolate(
            &self,
            _isolate: *mut Isolate,
        ) -> Tagged<HeapObject> {
            assert!(self.is_weak());
            Tagged { dummy: 0 }
        }

        pub fn get_heap_object(&self) -> Option<Tagged<HeapObject>> {
            if self.is_strong_or_weak() {
                Some(Tagged { dummy: 0 })
            } else {
                None
            }
        }

        pub fn get_heap_object_with_isolate(
            &self,
            _isolate: *mut Isolate,
        ) -> Option<Tagged<HeapObject>> {
            if self.is_strong_or_weak() {
                Some(Tagged { dummy: 0 })
            } else {
                None
            }
        }

        pub fn get_heap_object_with_reference_type(
            &self,
            reference_type: &mut HeapObjectReferenceType,
        ) -> Option<Tagged<HeapObject>> {
            if self.is_strong() {
                *reference_type = HeapObjectReferenceType::STRONG;
                Some(Tagged { dummy: 0 })
            } else if self.is_weak() {
                *reference_type = HeapObjectReferenceType::WEAK;
                Some(Tagged { dummy: 0 })
            } else {
                None
            }
        }

        pub fn get_heap_object_with_isolate_and_reference_type(
            &self,
            _isolate: *mut Isolate,
            reference_type: &mut HeapObjectReferenceType,
        ) -> Option<Tagged<HeapObject>> {
            if self.is_strong() {
                *reference_type = HeapObjectReferenceType::STRONG;
                Some(Tagged { dummy: 0 })
            } else if self.is_weak() {
                *reference_type = HeapObjectReferenceType::WEAK;
                Some(Tagged { dummy: 0 })
            } else {
                None
            }
        }

        pub fn get_heap_object_unchecked(&self) -> Tagged<HeapObject> {
            assert!(self.is_strong_or_weak());
            Tagged { dummy: 0 }
        }

        pub fn get_heap_object_unchecked_with_isolate(
            &self,
            _isolate: *mut Isolate,
        ) -> Tagged<HeapObject> {
            assert!(self.is_strong_or_weak());
            Tagged { dummy: 0 }
        }

        pub fn get_heap_object_or_smi(&self) -> Tagged<Object> {
            assert!(self.is_strong_or_weak() || self.is_smi());
            Tagged { dummy: 0 }
        }

        pub fn get_heap_object_or_smi_with_isolate(
            &self,
            _isolate: *mut Isolate,
        ) -> Tagged<Object> {
            assert!(self.is_strong_or_weak() || self.is_smi());
            Tagged { dummy: 0 }
        }

        pub fn cast<T>(&self) -> Tagged<T> {
            assert!(Self::kIsFull);
            assert!(!HAS_WEAK_HEAP_OBJECT_TAG!(self.ptr_));
            let obj = Tagged::<Object> { dummy: any_as_tagged_t(self.ptr_).unwrap_or_default() };
            crate::objects::casting_inl::Cast::<T>(obj)
        }

        pub fn ptr_location(&mut self) -> *mut StorageType {
            &mut self.ptr_
        }

        pub fn ptr_location_const(&self) -> *const StorageType {
            &self.ptr_
        }
    }

    fn any_as_address<T>(data: T) -> Option<Address> {
        if size_of::<T>() == size_of::<Address>() {
            let transmuted_data: Address = unsafe { std::mem::transmute_copy(&data) };
            Some(transmuted_data)
        } else {
            None
        }
    }

    fn any_as_tagged_t<T>(data: T) -> Option<Tagged_t> {
        if size_of::<T>() == size_of::<Tagged_t>() {
            let transmuted_data: Tagged_t = unsafe { std::mem::transmute_copy(&data) };
            Some(transmuted_data)
        } else {
            None
        }
    }

    fn check_object_comparison_allowed(a: Address, b: Address) -> bool {
        if !HAS_STRONG_HEAP_OBJECT_TAG!(a) || !HAS_STRONG_HEAP_OBJECT_TAG!(b) {
            return true;
        }
        let obj_a = unsafe { std::mem::transmute::<Address, Tagged<HeapObject>>(a) };
        let obj_b = unsafe { std::mem::transmute::<Address, Tagged<HeapObject>>(b) };
        // This check might fail when we try to compare objects in different pointer
        // compression cages (e.g. the one used by code space or trusted space) with
        // each other. The main legitimate case when such "mixed" comparison could
        // happen is comparing two AbstractCode objects. If that's the case one must
        // use AbstractCode's == operator instead of Object's one or SafeEquals().
        //CHECK_EQ(HeapLayout::InCodeSpace(obj_a), HeapLayout::InCodeSpace(obj_b));
        #[cfg(feature = "V8_ENABLE_SANDBOX")]
        {
            //CHECK_EQ(HeapLayout::InTrustedSpace(obj_a), HeapLayout::InTrustedSpace(obj_b));
        }
        true
    }

    pub trait BriefPrint {
        fn brief(&self) -> String;
    }

    impl<const kRefType: HeapObjectReferenceType, StorageType> BriefPrint for TaggedImpl<kRefType, StorageType>
    where
        StorageType: Copy + fmt::Debug,
    {
        fn brief(&self) -> String {
            format!("{:?}", self.ptr_)
        }
    }

    pub fn short_print<const kRefType: HeapObjectReferenceType, StorageType>(
        ptr: TaggedImpl<kRefType, StorageType>,
        out: &mut dyn Write,
    ) -> Result<(), io::Error>
    where
        StorageType: Copy + fmt::Debug,
    {
        let os = ptr.brief();
        out.write_all(os.as_bytes())?;
        Ok(())
    }
    
    pub fn short_print_to_string_stream<const kRefType: HeapObjectReferenceType, StorageType>(
        ptr: TaggedImpl<kRefType, StorageType>,
        accumulator: &mut String,
    ) where
        StorageType: Copy + fmt::Debug,
    {
        let os = ptr.brief();
        accumulator.push_str(&os);
    }
    

    pub fn short_print_to_ostream<const kRefType: HeapObjectReferenceType, StorageType>(
        ptr: TaggedImpl<kRefType, StorageType>,
        os: &mut std::ostream,
    ) -> Result<(), std::io::Error>
    where
        StorageType: Copy + fmt::Debug,
    {
        let brief = ptr.brief();
        write!(os, "{}", brief)
    }

    pub fn print<const kRefType: HeapObjectReferenceType, StorageType>(
        ptr: TaggedImpl<kRefType, StorageType>,
    ) where
        StorageType: Copy + fmt::Debug,
    {
        let mut os = StdoutStream {};
        print_to_ostream(ptr, &mut os).expect("print failed");
    }

    pub fn print_to_ostream<const kRefType: HeapObjectReferenceType, StorageType>(
        ptr: TaggedImpl<kRefType, StorageType>,
        os: &mut dyn Write,
    ) -> Result<(), io::Error>
    where
        StorageType: Copy + fmt::Debug,
    {
        if ptr.is_smi() {
            let value = any_as_tagged_t(ptr.ptr()).unwrap_or_default();
            let smi_value = value as i32;
            write!(os, "Smi: 0x{:x} ({})\n", smi_value, smi_value)?;
        } else if ptr.is_cleared() {
            write!(os, "[cleared]")?;
        } else if ptr.is_weak() {
            write!(os, "[weak] ")?;
            //heap_object.heap_object_print(os);
        } else if ptr.is_strong() {
            //heap_object.heap_object_print(os);
        } else {
            panic!("UNREACHABLE");
        }
        Ok(())
    }

    impl From<u64> for TaggedImpl<HeapObjectReferenceType::STRONG, Address> {
        fn from(value: u64) -> Self {
            TaggedImpl { ptr_: value as usize }
        }
    }
    impl From<u64> for TaggedImpl<HeapObjectReferenceType::WEAK, Address> {
        fn from(value: u64) -> Self {
            TaggedImpl { ptr_: value as usize }
        }
    }
    
    impl From<usize> for TaggedImpl<HeapObjectReferenceType::WEAK, Address> {
        fn from(value: usize) -> Self {
            TaggedImpl { ptr_: value }
        }
    }
    impl From<usize> for TaggedImpl<HeapObjectReferenceType::STRONG, Address> {
        fn from(value: usize) -> Self {
            TaggedImpl { ptr_: value }
        }
    }

    impl From<Tagged_t> for TaggedImpl<HeapObjectReferenceType::WEAK, Address> {
        fn from(value: Tagged_t) -> Self {
            TaggedImpl { ptr_: value }
        }
    }
    impl From<Tagged_t> for TaggedImpl<HeapObjectReferenceType::STRONG, Address> {
        fn from(value: Tagged_t) -> Self {
            TaggedImpl { ptr_: value }
        }
    }

    unsafe impl Send for TaggedImpl<HeapObjectReferenceType::STRONG, Address> {}
    unsafe impl Sync for TaggedImpl<HeapObjectReferenceType::STRONG, Address> {}
    unsafe impl Send for TaggedImpl<HeapObjectReferenceType::WEAK, Address> {}
    unsafe impl Sync for TaggedImpl<HeapObjectReferenceType::WEAK, Address> {}
    
}
