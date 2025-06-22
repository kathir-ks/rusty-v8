// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8_fast_api_calls {
    use std::{
        marker::PhantomData,
        mem::{self, MaybeUninit},
        ptr,
        sync::atomic::{AtomicBool, Ordering},
    };

    /// Opaque type representing a V8 Isolate.  This is a stand-in.  In a real
    /// project, you'd need to bind to the real V8.
    pub struct Isolate {
        // In reality this would need fields relevant for isolating V8 contexts
        // and execution, but for a minimal example, this serves as a placeholder.
        embedder_wrapper_type_index: usize,
        embedder_wrapper_object_index: usize,
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                embedder_wrapper_type_index: 0,
                embedder_wrapper_object_index: 0,
            }
        }

        pub fn set_embedder_wrapper_type_index(&mut self, index: usize) {
            self.embedder_wrapper_type_index = index;
        }

        pub fn set_embedder_wrapper_object_index(&mut self, index: usize) {
            self.embedder_wrapper_object_index = index;
        }
    }

    /// Opaque type representing a V8 Context. Stand-in.
    pub struct Context {}

    /// Opaque type representing a V8 Local.  Stand-in.
    #[derive(Copy, Clone)]
    pub struct Local<'a, T> {
        _phantom: PhantomData<&'a T>,
        // In a real project, this would contain the actual pointer to the V8 object.
    }

    impl<'a, T> Local<'a, T> {
        pub fn empty() -> Self {
            Local {
                _phantom: PhantomData,
            }
        }

        pub fn cast<U>(&self) -> Local<'a, U> {
            Local {
                _phantom: PhantomData,
            }
        }
    }

    /// Opaque type representing a V8 Value. Stand-in.
    pub struct Value {}

    /// Opaque type representing a V8 Object. Stand-in.
    pub struct Object {}

    impl Object {
        pub fn set_aligned_pointer_in_internal_field(&self, _index: usize, _ptr: *mut std::ffi::c_void) {}
        pub fn get_aligned_pointer_from_internal_field(&self, _index: usize) -> *mut std::ffi::c_void {
            ptr::null_mut()
        }
        pub fn internal_field_count(&self) -> usize {
            0
        }
    }

    /// Opaque type representing a V8 Array. Stand-in.
    pub struct Array {}

    /// Opaque type representing a V8 FunctionTemplate. Stand-in.
    pub struct FunctionTemplate {}

    impl FunctionTemplate {
        pub fn new<'a>(
            _isolate: &mut Isolate,
            _slow_callback: fn(&FunctionCallbackInfo<Value>),
            _data: Local<'a, Value>,
            _signature: Local<'a, Signature>,
            _length: i32,
            _constructor_behavior: ConstructorBehavior,
            _side_effect_type: SideEffectType,
            c_function: &CFunction,
        ) -> Local<'a, FunctionTemplate> {
            let _ = c_function.get_int64_representation(); // Force evaluation of const expression.
            Local {
                _phantom: PhantomData,
            }
        }

        pub fn new_with_c_function_overloads<'a>(
            _isolate: &mut Isolate,
            _slow_callback: fn(&FunctionCallbackInfo<Value>),
            _data: Local<'a, Value>,
            _signature: Local<'a, Signature>,
            _length: i32,
            _constructor_behavior: ConstructorBehavior,
            _side_effect_type: SideEffectType,
            overloads: CFunctionOverloads,
        ) -> Local<'a, FunctionTemplate> {
             // Force evaluation of const expression.
            let _ = overloads.functions[0].get_int64_representation();
            Local {
                _phantom: PhantomData,
            }
        }
    }

    /// Opaque type representing a V8 ObjectTemplate. Stand-in.
    pub struct ObjectTemplate {}

    impl ObjectTemplate {
        pub fn new(_isolate: &mut Isolate) -> ObjectTemplate {
            ObjectTemplate {}
        }

        pub fn set_internal_field_count(&mut self, _count: usize) {}

        pub fn set<'a>(
            &mut self,
            _isolate: &mut Isolate,
            _name: &str,
            _value: Local<'a, FunctionTemplate>,
        ) {
        }

        pub fn new_instance<'a>(&self, _context: Local<'a, Context>) -> Result<Local<'a, Object>, ()> {
            Ok(Local {
                _phantom: PhantomData,
            })
        }
    }

    /// Opaque type representing a V8 Signature. Stand-in.
    pub struct Signature {}

    /// Opaque type representing FunctionCallbackInfo. Stand-in.
    pub struct FunctionCallbackInfo<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> FunctionCallbackInfo<T> {
        pub fn holder(&self) -> Local<Object> {
            Local {
                _phantom: PhantomData,
            }
        }
    }

    /// Enum mirroring v8::ConstructorBehavior.
    pub enum ConstructorBehavior {
        kAllow,
    }

    /// Enum mirroring v8::SideEffectType.
    pub enum SideEffectType {
        kHasSideEffect,
    }

    /// Mirrors v8::CTypeInfo
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum CType {
        kVoid,
        kBool,
        kUint8,
        kInt32,
        kUint32,
        kInt64,
        kUint64,
        kFloat32,
        kFloat64,
        kPointer,
        kV8Value,
        kSeqOneByteString,
        kApiObject, // This will be deprecated once all users have
        kAny,       // This is added to enable untyped representation of fast
                    // call arguments for test purposes.
    }

    impl CType {
        pub const kCallbackOptionsType: Self = unsafe { mem::transmute(255u8) };
        pub const kScalar: SequenceType = SequenceType::kScalar;

        pub const fn is_integral_type(self) -> bool {
            matches!(
                self,
                CType::kUint8 | CType::kInt32 | CType::kUint32 | CType::kInt64 | CType::kUint64
            )
        }

        pub const fn is_floating_point_type(self) -> bool {
            matches!(self, CType::kFloat32 | CType::kFloat64)
        }

        pub const fn is_primitive(self) -> bool {
            self.is_integral_type() || self.is_floating_point_type() || self == CType::kBool
        }
    }

    /// Mirrors v8::CTypeInfo::SequenceType
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum SequenceType {
        kScalar,
        kIsSequence,   // sequence<T>
        kIsArrayBuffer, // ArrayBuffer
    }

    /// Mirrors v8::CTypeInfo::Flags
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Flags {
        kNone = 0,
        kAllowSharedBit = 1 << 0,   // Must be an ArrayBuffer or TypedArray
        kEnforceRangeBit = 1 << 1,  // T must be integral
        kClampBit = 1 << 2,         // T must be integral
        kIsRestrictedBit = 1 << 3,  // T must be float or double
    }

    /// Mirrors v8::CTypeInfo
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct CTypeInfo {
        type_: CType,
        sequence_type_: SequenceType,
        flags_: Flags,
    }

    impl CTypeInfo {
        pub const fn new(type_: CType, flags_: Flags) -> Self {
            CTypeInfo {
                type_,
                sequence_type_: SequenceType::kScalar,
                flags_,
            }
        }

        pub const fn new_deprecated(type_: CType, sequence_type_: SequenceType, flags_: Flags) -> Self {
            CTypeInfo {
                type_,
                sequence_type_,
                flags_,
            }
        }

        pub type Identifier = u32;
        pub const fn new_with_identifier(identifier: Identifier) -> Self {
            CTypeInfo {
                type_: unsafe { mem::transmute((identifier >> 16) as u8) },
                sequence_type_: unsafe { mem::transmute(((identifier >> 8) & 255) as u8) },
                flags_: unsafe { mem::transmute((identifier & 255) as u8) },
            }
        }

        pub const fn get_id(self) -> Identifier {
            (self.type_ as u32) << 16 | (self.sequence_type_ as u32) << 8 | (self.flags_ as u32)
        }

        pub const fn get_type(self) -> CType {
            self.type_
        }

        pub const fn get_sequence_type(self) -> SequenceType {
            self.sequence_type_
        }

        pub const fn get_flags(self) -> Flags {
            self.flags_
        }
    }

    /// Mirrors v8::FastOneByteString
    #[derive(Debug)]
    pub struct FastOneByteString {
        pub data: *const char,
        pub length: u32,
    }

    /// Mirrors v8::CFunctionInfo
    pub struct CFunctionInfo {
        return_info_: CTypeInfo,
        repr_: Int64Representation,
        arg_count_: u32,
        arg_info_: Vec<CTypeInfo>, // Box<[CTypeInfo]>,
    }

    impl CFunctionInfo {
        pub fn new(
            return_info: CTypeInfo,
            arg_count: u32,
            arg_info: &[CTypeInfo],
            repr: Int64Representation,
        ) -> Self {
            CFunctionInfo {
                return_info,
                repr_: repr,
                arg_count_: arg_count,
                arg_info_: arg_info.to_vec(),
            }
        }

        pub fn return_info(&self) -> CTypeInfo {
            self.return_info_
        }

        pub fn argument_count(&self) -> u32 {
            if self.has_options() {
                self.arg_count_ - 1
            } else {
                self.arg_count_
            }
        }

        pub fn get_int64_representation(&self) -> Int64Representation {
            self.repr_
        }

        pub fn argument_info(&self, index: u32) -> CTypeInfo {
            self.arg_info_[index as usize]
        }

        pub fn has_options(&self) -> bool {
            if self.arg_count_ > 0 {
                self.arg_info_[(self.arg_count_ - 1) as usize].get_type() == CType::kCallbackOptionsType
            } else {
                false
            }
        }
    }

    /// Mirrors v8::CFunctionInfo::Int64Representation
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Int64Representation {
        kNumber = 0, // Use numbers to represent 64 bit integers.
        kBigInt = 1, // Use BigInts to represent 64 bit integers.
    }

    /// Mirrors v8::FastApiCallbackOptions
    #[derive(Debug)]
    pub struct FastApiCallbackOptions {
        pub isolate: *mut Isolate, // v8::Isolate*
        pub data: Local<'static, Value>, //v8::Local<v8::Value>
        pub fallback: AtomicBool,
    }

    impl FastApiCallbackOptions {
        pub fn create_for_testing(_isolate: *mut Isolate) -> Self {
            FastApiCallbackOptions {
                isolate: ptr::null_mut(),
                data: Local::empty(),
                fallback: AtomicBool::new(false),
            }
        }
    }

    // Mirrors v8::AnyCType
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub union AnyCType {
        pub bool_value: bool,
        pub int32_value: i32,
        pub uint32_value: u32,
        pub int64_value: i64,
        pub uint64_value: u64,
        pub float_value: f32,
        pub double_value: f64,
        pub pointer_value: *mut std::ffi::c_void,
        pub object_value: Local<'static, Object>,
        pub sequence_value: Local<'static, Array>,
        pub string_value: *const FastOneByteString,
        pub options_value: *mut FastApiCallbackOptions,
    }

    impl AnyCType {
        pub fn new() -> Self {
            AnyCType { int64_value: 0 }
        }
    }

    /// Mirrors v8::CFunction
    #[derive(Debug, Copy, Clone)]
    pub struct CFunction {
        address_: *const std::ffi::c_void,
        type_info_: *const CFunctionInfo,
    }

    impl CFunction {
        pub const fn new() -> Self {
            CFunction {
                address_: ptr::null(),
                type_info_: ptr::null(),
            }
        }

        pub fn return_info(&self) -> CTypeInfo {
            unsafe { (*self.type_info_).return_info() }
        }

        pub fn argument_info(&self, index: u32) -> CTypeInfo {
            unsafe { (*self.type_info_).argument_info(index) }
        }

        pub fn argument_count(&self) -> u32 {
            unsafe { (*self.type_info_).argument_count() }
        }

        pub fn get_address(&self) -> *const std::ffi::c_void {
            self.address_
        }

        pub fn get_int64_representation(&self) -> Int64Representation {
            unsafe { (*self.type_info_).get_int64_representation() }
        }

        pub fn get_type_info(&self) -> *const CFunctionInfo {
            self.type_info_
        }

        pub enum OverloadResolution {
            kImpossible,
            kAtRuntime,
            kAtCompileTime,
        }

        pub fn make<F>(func: *const F, int64_rep: Int64Representation) -> Self {
            ArgUnwrap::<*const F>::make(func, int64_rep)
        }

        pub fn make_with_patch<R, Args, R_Patch, Args_Patch>(
            func: extern "C" fn(Args) -> R,
            patching_func: extern "C" fn(Args_Patch) -> R_Patch,
            int64_rep: Int64Representation,
        ) -> Self {
            let mut c_func = ArgUnwrap::<extern "C" fn(Args) -> R>::make(func, int64_rep);
            c_func.address_ = patching_func as *const std::ffi::c_void;
            c_func
        }

        pub const fn new_with_address_and_type_info(
            address: *const std::ffi::c_void,
            type_info: *const CFunctionInfo,
        ) -> Self {
            CFunction {
                address_: address,
                type_info_: type_info,
            }
        }
    }

    pub(crate) trait ArgUnwrapTrait {
        fn make<F>(func: *const F, int64_rep: Int64Representation) -> CFunction;
    }

    #[derive(Debug)]
    pub struct ArgUnwrap<F> {
        _phantom: PhantomData<F>,
    }

    impl<F> ArgUnwrap<F> {
        // This static_assert is impossible to express in Rust, so omitting it.
        // static_assert(sizeof(F) != sizeof(F), "CFunction must be created from a function pointer.");
    }

    impl<R, Args> ArgUnwrapTrait for ArgUnwrap<extern "C" fn(Args) -> R> {
        fn make<F>(func: *const F, int64_rep: Int64Representation) -> CFunction {
            let func_ptr = func as *const std::ffi::c_void;
            if int64_rep == Int64Representation::kNumber {
                CFunctionBuilder::new().fn_with_function(func_ptr).build()
            } else {
                CFunctionBuilder::new()
                    .fn_with_function(func_ptr)
                    .build_with_representation::<{ Int64Representation::kBigInt }>()
            }
        }
    }

    /// Mirrors v8::CFunctionOverloads
    pub struct CFunctionOverloads {
        pub functions: Vec<CFunction>,
    }

    /// Mirrors v8::CFunctionBuilder
    pub struct CFunctionBuilder {
    }

    impl CFunctionBuilder {
        pub const fn new() -> Self {
            CFunctionBuilder {}
        }

        pub const fn fn_void<Args>(fn: extern "C" fn(Args)) -> CFunctionBuilderWithFunction<CTypeInfoBuilder<()>, CTypeInfoBuilder<Args>>{
            CFunctionBuilderWithFunction::new(fn as *const std::ffi::c_void)
        }

        pub const fn fn_with_function(fn: *const std::ffi::c_void) -> CFunctionBuilderWithFunction<CTypeInfoBuilder<()>>{
            CFunctionBuilderWithFunction::new(fn as *const std::ffi::c_void)
        }

        pub const fn fn<R, Args>(fn: extern "C" fn(Args) -> R) -> CFunctionBuilderWithFunction<CTypeInfoBuilder<R>, CTypeInfoBuilder<Args>> {
            CFunctionBuilderWithFunction::new(fn as *const std::ffi::c_void)
        }
    }

    /// Mirrors v8::CFunctionBuilderWithFunction
    pub struct CFunctionBuilderWithFunction<RetBuilder, ArgBuilders = ()> {
        fn_: *const std::ffi::c_void,
        _phantom_ret: PhantomData<RetBuilder>,
        _phantom_args: PhantomData<ArgBuilders>,
    }

    impl<RetBuilder> CFunctionBuilderWithFunction<RetBuilder, ()> {
        pub const fn new(fn: *const std::ffi::c_void) -> Self {
            CFunctionBuilderWithFunction {
                fn_: fn,
                _phantom_ret: PhantomData,
                _phantom_args: PhantomData,
            }
        }
    }

    impl<RetBuilder, ArgBuilders> CFunctionBuilderWithFunction<RetBuilder, ArgBuilders> {
        pub fn ret<Flags>(self) -> CFunctionBuilderWithFunction<CTypeInfoBuilder<Flags>, ArgBuilders> {
            CFunctionBuilderWithFunction {
                fn_: self.fn_,
                _phantom_ret: PhantomData,
                _phantom_args: PhantomData,
            }
        }

        pub fn arg<const N: usize, Flags>(self) -> Self {
            // Return a copy of the builder with the Nth arg builder merged with
            // template parameter pack Flags.
            self.arg_impl::<N, Flags>(std::array::from_fn(|i| i))
        }

        pub fn patch<Ret, Args>(self, patching_func: extern "C" fn(Args) -> Ret) -> Self {
            Self {
                fn_: patching_func as *const std::ffi::c_void,
                _phantom_ret: PhantomData,
                _phantom_args: PhantomData,
            }
        }

        pub fn build(self) -> CFunction {
            let instance =
                CFunctionInfoImpl::<{Int64Representation::kNumber}, RetBuilder, ArgBuilders>::new();
            CFunction::new_with_address_and_type_info(self.fn_, &instance)
        }

        pub fn build_with_representation<const Representation: u8>(self) -> CFunction {
            let representation: Int64Representation = unsafe { mem::transmute(Representation) };
            let instance = CFunctionInfoImpl::<{representation}, RetBuilder, ArgBuilders>::new();
            CFunction::new_with_address_and_type_info(self.fn_, &instance)
        }

        fn arg_impl<const N: usize, Flags, const I: usize>(self, _indices: [usize; 0]) -> Self {
            Self {
                fn_: self.fn_,
                _phantom_ret: PhantomData,
                _phantom_args: PhantomData,
            }
        }
    }

    /// Mirrors v8::CTypeInfoImpl
    struct CFunctionInfoImpl<const Representation: u8, RetBuilder, ArgBuilders = ()> {
        _phantom_ret: PhantomData<RetBuilder>,
        _phantom_args: PhantomData<ArgBuilders>,
        instance: CFunctionInfo,
    }

    impl<const Representation: u8, RetBuilder, ArgBuilders> CFunctionInfoImpl<Representation, RetBuilder, ArgBuilders> {
        const fn new() -> Self {
            let arg_info_storage: [CTypeInfo; 0] = [];

            let repr = unsafe { mem::transmute(Representation) };
            let return_info = CTypeInfoBuilder::<()>::build();
            let arg_count = 0;

            let instance = CFunctionInfo::new(return_info, arg_count, &arg_info_storage, repr);

            CFunctionInfoImpl {
                _phantom_ret: PhantomData,
                _phantom_args: PhantomData,
                instance: instance,
            }
        }
    }

    /// Mirrors v8::CTypeInfoBuilder
    pub struct CTypeInfoBuilder<T> {
        _phantom: PhantomData<T>,
    }

    impl CTypeInfoBuilder<()> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<()>::get_type(), TypeInfoHelper::<()>::get_flags())
        }
    }

    impl CTypeInfoBuilder<bool> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<bool>::get_type(), TypeInfoHelper::<bool>::get_flags())
        }
    }

    impl CTypeInfoBuilder<i32> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<i32>::get_type(), TypeInfoHelper::<i32>::get_flags())
        }
    }

    impl CTypeInfoBuilder<u32> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<u32>::get_type(), TypeInfoHelper::<u32>::get_flags())
        }
    }

    impl CTypeInfoBuilder<i64> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<i64>::get_type(), TypeInfoHelper::<i64>::get_flags())
        }
    }

    impl CTypeInfoBuilder<u64> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<u64>::get_type(), TypeInfoHelper::<u64>::get_flags())
        }
    }

    impl CTypeInfoBuilder<f32> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<f32>::get_type(), TypeInfoHelper::<f32>::get_flags())
        }
    }

    impl CTypeInfoBuilder<f64> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<f64>::get_type(), TypeInfoHelper::<f64>::get_flags())
        }
    }

    impl CTypeInfoBuilder<*mut std::ffi::c_void> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<*mut std::ffi::c_void>::get_type(), TypeInfoHelper::<*mut std::ffi::c_void>::get_flags())
        }
    }

    impl CTypeInfoBuilder<FastApiCallbackOptions> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<FastApiCallbackOptions>::get_type(), TypeInfoHelper::<FastApiCallbackOptions>::get_flags())
        }
    }

    impl CTypeInfoBuilder<FastApiCallbackOptionsAndRef> {
        pub const fn build() -> CTypeInfo {
            CTypeInfo::new(TypeInfoHelper::<FastApiCallbackOptionsAndRef>::get_type(), TypeInfoHelper::<FastApiCallbackOptionsAndRef>::get_flags())
        }
    }

    struct FastApiCallbackOptionsAndRef;

    struct TypeInfoHelper<()>;

    impl TypeInfoHelper<()> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kVoid
        }
    }

    struct TypeInfoHelper<bool>;

    impl TypeInfoHelper<bool> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kBool
        }
    }

    struct TypeInfoHelper<i32>;

    impl TypeInfoHelper<i32> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kInt32
        }
    }

    struct TypeInfoHelper<u32>;

    impl TypeInfoHelper<u32> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kUint32
        }
    }

    struct TypeInfoHelper<i64>;

    impl TypeInfoHelper<i64> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kInt64
        }
    }

    struct TypeInfoHelper<u64>;

    impl TypeInfoHelper<u64> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kUint64
        }
    }

    struct TypeInfoHelper<f32>;

    impl TypeInfoHelper<f32> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kFloat32
        }
    }

    struct TypeInfoHelper<f64>;

    impl TypeInfoHelper<f64> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kFloat64
        }
    }

    struct TypeInfoHelper<*mut std::ffi::c_void>;

    impl TypeInfoHelper<*mut std::ffi::c_void> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kPointer
        }
    }

    struct TypeInfoHelper<FastApiCallbackOptions>;

    impl TypeInfoHelper<FastApiCallbackOptions> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kCallbackOptionsType
        }
    }

    struct TypeInfoHelper<FastApiCallbackOptionsAndRef>;

    impl TypeInfoHelper<FastApiCallbackOptionsAndRef> {
        const fn get_flags() -> Flags {
            Flags::kNone
        }

        const fn get_type() -> CType {
            CType::kCallbackOptionsType
        }
    }

    /// Mirrors v8::TryToCopyAndConvertArrayToCppBuffer
    pub fn try_to_copy_and_convert_array_to_cpp_buffer<
        const TYPE_INFO_ID: CTypeInfo::Identifier,
        T,
    >(
        src: Local<Array>,
        dst: *mut T,
        max_length: u32,
    ) -> bool {
        // Placeholder implementation.  Needs actual V8 binding and array conversion logic.
        println!("try_to_copy_and_convert_array_to_cpp_buffer called with type_info_id: {}, dst: {:p}, max_length: {}", TYPE_INFO_ID, dst, max_length);
        true
    }

    // Explicit specializations for the C++ template function.
    pub fn try_to_copy_and_convert_array_to_cpp_buffer_i32(
        src: Local<Array>,
        dst: *mut i32,
        max_length: u32,
    ) -> bool {
        try_to_copy_and_convert_array_to_cpp_buffer::<{ CTypeInfoBuilder::<i32>::build().get_id() }, i32>(
            src, dst, max_length,
        )
    }

    pub fn try_to_copy_and_convert_array_to_cpp_buffer_u32(
        src: Local<Array>,
        dst: *mut u32,
        max_length: u32,
    ) -> bool {
        try_to_copy_and_convert_array_to_cpp_buffer::<{ CTypeInfoBuilder::<u32>::build().get_id() }, u32>(
            src, dst, max_length,
        )
    }

    pub fn try_to_copy_and_convert_array_to_cpp_buffer_f32(
        src: Local<Array>,
        dst: *mut f32,
        max_length: u32,
    ) -> bool {
        try_to_copy_and_convert_array_to_cpp_buffer::<{ CTypeInfoBuilder::<f32>::build().get_id() }, f32>(
            src, dst, max_length,
        )
    }

    pub fn try_to_copy_and_convert_array_to_cpp_buffer_f64(
        src: Local<Array>,
        dst: *mut f64,
        max_length: u32,
    ) -> bool {
        try_to_copy_and_convert_array_to_cpp_buffer::<{ CTypeInfoBuilder::<f64>::build().get_id() }, f64>(
            src, dst, max_length,
        )
    }
}