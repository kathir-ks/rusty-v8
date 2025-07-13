// Converted from V8 C++ source files:
// Header: v8-fast-api-calls.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::marker::PhantomData;

use crate::v8::{
    ArrayBuffer, FunctionTemplate, Isolate, Local, Object, TypedArray, Value, V8,
};

pub struct FastOneByteString {
    pub data: *const char,
    pub length: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Int64Representation {
    kNumber = 0,
    kBigInt = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CTypeInfo_Type {
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
    kApiObject,
    kAny,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CTypeInfo_SequenceType {
    kScalar,
    kIsSequence,
    kIsArrayBuffer,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CTypeInfo_Flags {
    kNone = 0,
    kAllowSharedBit = 1 << 0,
    kEnforceRangeBit = 1 << 1,
    kClampBit = 1 << 2,
    kIsRestrictedBit = 1 << 3,
}

#[derive(Debug, Clone, Copy)]
pub struct CTypeInfo {
    type_: CTypeInfo_Type,
    sequence_type_: CTypeInfo_SequenceType,
    flags_: CTypeInfo_Flags,
}

impl CTypeInfo {
    pub const kCallbackOptionsType: CTypeInfo_Type = unsafe { std::mem::transmute(255u8) };

    pub const fn new(type_: CTypeInfo_Type, flags: CTypeInfo_Flags) -> Self {
        CTypeInfo {
            type_,
            sequence_type_: CTypeInfo_SequenceType::kScalar,
            flags_,
        }
    }

    pub const fn new_deprecated(
        type_: CTypeInfo_Type,
        sequence_type: CTypeInfo_SequenceType,
        flags: CTypeInfo_Flags,
    ) -> Self {
        CTypeInfo {
            type_,
            sequence_type_: sequence_type,
            flags_,
        }
    }

    pub type Identifier = u32;
    pub const fn new_identifier(identifier: CTypeInfo::Identifier) -> Self {
        CTypeInfo {
            type_: unsafe { std::mem::transmute((identifier >> 16) as u8) },
            sequence_type_: unsafe {
                std::mem::transmute(((identifier >> 8) & 255) as u8)
            },
            flags_: unsafe { std::mem::transmute((identifier & 255) as u8) },
        }
    }

    pub const fn get_id(&self) -> CTypeInfo::Identifier {
        (self.type_ as u8 as u32) << 16
            | (self.sequence_type_ as u8 as u32) << 8
            | (self.flags_ as u8 as u32)
    }

    pub const fn get_type(&self) -> CTypeInfo_Type {
        self.type_
    }

    pub const fn get_sequence_type(&self) -> CTypeInfo_SequenceType {
        self.sequence_type_
    }

    pub const fn get_flags(&self) -> CTypeInfo_Flags {
        self.flags_
    }

    pub const fn is_integral_type(type_: CTypeInfo_Type) -> bool {
        type_ == CTypeInfo_Type::kUint8
            || type_ == CTypeInfo_Type::kInt32
            || type_ == CTypeInfo_Type::kUint32
            || type_ == CTypeInfo_Type::kInt64
            || type_ == CTypeInfo_Type::kUint64
    }

    pub const fn is_floating_point_type(type_: CTypeInfo_Type) -> bool {
        type_ == CTypeInfo_Type::kFloat32 || type_ == CTypeInfo_Type::kFloat64
    }

    pub const fn is_primitive(type_: CTypeInfo_Type) -> bool {
        CTypeInfo::is_integral_type(type_)
            || CTypeInfo::is_floating_point_type(type_)
            || type_ == CTypeInfo_Type::kBool
    }
}

pub struct CFunctionInfo {
    return_info_: CTypeInfo,
    repr_: Int64Representation,
    arg_count_: u32,
    arg_info_: Vec<CTypeInfo>,
}

impl CFunctionInfo {
    pub fn new(
        return_info: CTypeInfo,
        arg_count: u32,
        arg_info: &[CTypeInfo],
        repr: Int64Representation,
    ) -> Self {
        CFunctionInfo {
            return_info_: return_info,
            repr_: repr,
            arg_count_: arg_count,
            arg_info_: arg_info.to_vec(),
        }
    }

    pub fn return_info(&self) -> &CTypeInfo {
        &self.return_info_
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

    pub fn argument_info(&self, index: u32) -> &CTypeInfo {
        &self.arg_info_[index as usize]
    }

    pub fn has_options(&self) -> bool {
        self.arg_count_ > 0
            && self.arg_info_[(self.arg_count_ - 1) as usize].get_type()
                == CTypeInfo::kCallbackOptionsType
    }
}

pub struct FastApiCallbackOptions {
    pub isolate: *mut Isolate,
    pub data: Local<'static, Value>,
}

impl FastApiCallbackOptions {
    pub fn create_for_testing(_isolate: *mut Isolate) -> Self {
        FastApiCallbackOptions {
            isolate: std::ptr::null_mut(),
            data: Local {
                _phantom: PhantomData,
            },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union AnyCType {
    pub bool_value: bool,
    pub int32_value: i32,
    pub uint32_value: u32,
    pub int64_value: i64,
    pub uint64_value: u64,
    pub float_value: f32,
    pub double_value: f64,
    pub pointer_value: *mut void,
    pub object_value: Local<'static, Object>,
    pub sequence_value: Local<'static, crate::v8::Array>,
    pub string_value: *const FastOneByteString,
    pub options_value: *mut FastApiCallbackOptions,
}

impl AnyCType {
    pub fn new() -> Self {
        AnyCType { int64_value: 0 }
    }
}

pub struct CFunction {
    address_: *const void,
    type_info_: *const CFunctionInfo,
}

impl CFunction {
    pub const fn new() -> Self {
        CFunction {
            address_: std::ptr::null(),
            type_info_: std::ptr::null(),
        }
    }

    pub fn return_info(&self) -> &CTypeInfo {
        unsafe { (*self.type_info_).return_info() }
    }

    pub fn argument_info(&self, index: u32) -> &CTypeInfo {
        unsafe { (*self.type_info_).argument_info(index) }
    }

    pub fn argument_count(&self) -> u32 {
        unsafe { (*self.type_info_).argument_count() }
    }

    pub fn get_address(&self) -> *const void {
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

    pub fn make<F>(func: *mut F, int64_rep: Int64Representation) -> Self {
        ArgUnwrap::<F>::make(func, int64_rep)
    }

    pub fn make_with_patch<R, Args, R_Patch, Args_Patch>(
        func: fn(Args...) -> R,
        patching_func: fn(Args_Patch...) -> R_Patch,
        int64_rep: Int64Representation,
    ) -> Self {
        let mut c_func: CFunction = ArgUnwrap::<fn(Args...) -> R>::make(func as *mut _, int64_rep);
        assert_eq!(
            std::mem::size_of::<Args_Patch>(),
            std::mem::size_of::<Args>(),
            "The patching function must have the same number of arguments."
        );
        c_func.address_ = patching_func as *const void;
        c_func
    }

    pub fn from_raw_parts(address: *const void, type_info: *const CFunctionInfo) -> Self {
        CFunction {
            address_: address,
            type_info_: type_info,
        }
    }
}

struct ArgUnwrap<F> {
    _phantom: PhantomData<F>,
}

impl<F> ArgUnwrap<F> {
    fn make(_func: *mut F, _int64_rep: Int64Representation) -> CFunction {
        panic!("CFunction must be created from a function pointer.");
    }
}

impl<R, Args> ArgUnwrap<fn(Args...) -> R> {
    fn make(func: *mut fn(Args...) -> R, int64_rep: Int64Representation) -> CFunction {
        if int64_rep == Int64Representation::kNumber {
            CFunctionBuilder::new().fn_build(func).build()
        } else {
            CFunctionBuilder::new()
                .fn_build(func)
                .build_with_representation::<{ Int64Representation::kBigInt as u8 }>()
        }
    }
}

struct TypeInfoHelper<T> {
    _phantom: PhantomData<T>,
}

impl<T> TypeInfoHelper<T> {
    const fn flags() -> CTypeInfo_Flags {
        panic!("This type is not supported")
    }
    const fn type_() -> CTypeInfo_Type {
        panic!("This type is not supported")
    }
}

macro_rules! specialize_get_type_info_helper_for {
    ($t:ty, $enum:ident) => {
        impl TypeInfoHelper<$t> {
            const fn flags() -> CTypeInfo_Flags {
                CTypeInfo_Flags::kNone
            }
            const fn type_() -> CTypeInfo_Type {
                CTypeInfo_Type::$enum
            }
        }
    };
}

specialize_get_type_info_helper_for!(bool, kBool);
specialize_get_type_info_helper_for!(u8, kUint8);
specialize_get_type_info_helper_for!(i32, kInt32);
specialize_get_type_info_helper_for!(u32, kUint32);
specialize_get_type_info_helper_for!(i64, kInt64);
specialize_get_type_info_helper_for!(u64, kUint64);
specialize_get_type_info_helper_for!(f32, kFloat32);
specialize_get_type_info_helper_for!(f64, kFloat64);
specialize_get_type_info_helper_for!(*mut void, kPointer);
specialize_get_type_info_helper_for!(Local<'static, Value>, kV8Value);
specialize_get_type_info_helper_for!(Local<'static, Object>, kV8Value);
specialize_get_type_info_helper_for!(Local<'static, crate::v8::Array>, kV8Value);

//TODO Implement AnyCType
//specialize_get_type_info_helper_for!(AnyCType, kAny);

impl TypeInfoHelper<FastApiCallbackOptions> {
    const fn flags() -> CTypeInfo_Flags {
        CTypeInfo_Flags::kNone
    }

    const fn type_() -> CTypeInfo_Type {
        CTypeInfo::kCallbackOptionsType
    }
}

impl TypeInfoHelper<FastOneByteString> {
    const fn flags() -> CTypeInfo_Flags {
        CTypeInfo_Flags::kNone
    }

    const fn type_() -> CTypeInfo_Type {
        CTypeInfo_Type::kSeqOneByteString
    }
}

macro_rules! static_assert_implies {
    ($cond:expr, $assertion:expr, $msg:expr) => {
        const _: () = if !$cond || $assertion {
        } else {
            panic!($msg);
        };
    };
}

pub struct CTypeInfoBuilder<T, const FLAGS: u8> {
    _phantom: PhantomData<T>,
}

impl<T, const FLAGS: u8> CTypeInfoBuilder<T, FLAGS> {
    pub const fn build() -> CTypeInfo {
        let k_flags: CTypeInfo_Flags = unsafe { std::mem::transmute(FLAGS) };
        let k_type: CTypeInfo_Type = TypeInfoHelper::<T>::type_();

        static_assert_implies!(
            (FLAGS & (CTypeInfo_Flags::kEnforceRangeBit as u8)) != 0,
            CTypeInfo::is_integral_type(k_type),
            "kEnforceRangeBit is only allowed for integral types."
        );
        static_assert_implies!(
            (FLAGS & (CTypeInfo_Flags::kClampBit as u8)) != 0,
            CTypeInfo::is_integral_type(k_type),
            "kClampBit is only allowed for integral types."
        );
        static_assert_implies!(
            (FLAGS & (CTypeInfo_Flags::kIsRestrictedBit as u8)) != 0,
            CTypeInfo::is_floating_point_type(k_type),
            "kIsRestrictedBit is only allowed for floating point types."
        );
        CTypeInfo::new(TypeInfoHelper::<T>::type_(), k_flags)
    }
}

struct CFunctionBuilderWithFunction<RetBuilder, Args> {
    fn_: *const void,
    _phantom: PhantomData<(RetBuilder, Args)>,
}

impl<RetBuilder, Args> CFunctionBuilderWithFunction<RetBuilder, Args> {
    const fn new(fn_: *const void) -> Self {
        CFunctionBuilderWithFunction {
            fn_: fn_,
            _phantom: PhantomData,
        }
    }

    const fn ret<const FLAGS: u8>(self) -> CFunctionBuilderWithFunction<
        CTypeInfoBuilder<
            <RetBuilder as RetBuilderTrait>::BaseType,
            { FLAGS },
        >,
        Args,
    >
    where
        RetBuilder: RetBuilderTrait,
    {
        CFunctionBuilderWithFunction::<
            CTypeInfoBuilder<<RetBuilder as RetBuilderTrait>::BaseType, { FLAGS }>,
            Args,
        >::new(self.fn_)
    }
}

trait RetBuilderTrait {
    type BaseType;
}

impl<T, const FLAGS: u8> RetBuilderTrait for CTypeInfoBuilder<T, FLAGS> {
    type BaseType = T;
}

macro_rules! define_primitive_c_types {
    ($($name:ident, $ctype:ty, $enum:ident;)*) => {
        $(
            pub const $name: CTypeInfo = CTypeInfo {
                type_: CTypeInfo_Type::$enum,
                sequence_type_: CTypeInfo_SequenceType::kScalar,
                flags_: CTypeInfo_Flags::kNone,
            };
        )*
    }
}

define_primitive_c_types! {
    K_TYPE_INFO_INT32, i32, kInt32;
    K_TYPE_INFO_FLOAT64, f64, kFloat64;
}

struct CFunctionBuilder {
}

impl CFunctionBuilder {
    const fn new() -> Self {
        CFunctionBuilder {}
    }
    const fn fn_build<R, Args>(self, fn_: *mut fn(Args...) -> R) -> CFunctionBuilderWithFunction<CTypeInfoBuilder<R, 0>, Args> {
        CFunctionBuilderWithFunction::<CTypeInfoBuilder<R, 0>, Args>::new(fn_ as *const void)
    }
}

trait BuildTrait {
    fn build(&self) -> CFunction;
}

impl<RetBuilder, Args> BuildTrait
    for CFunctionBuilderWithFunction<RetBuilder, Args>
where
    RetBuilder: RetBuilderTrait,
{
    fn build(&self) -> CFunction {
        todo!()
    }
}

impl<RetBuilder, Args> CFunctionBuilderWithFunction<RetBuilder, Args> {
    const fn build_with_representation<const REPRESENTATION: u8>(&self) -> CFunction {
        todo!()
    }
}

pub fn try_to_copy_and_convert_array_to_cpp_buffer<
    const TYPE_INFO_ID: u32,
    T,
>(
    src: Local<'static, crate::v8::Array>,
    dst: *mut T,
    max_length: u32,
) -> bool {
    false
}

pub fn try_to_copy_and_convert_array_to_cpp_buffer_int32(
    src: Local<'static, crate::v8::Array>,
    dst: *mut i32,
    max_length: u32,
) -> bool {
    false
}

pub fn try_to_copy_and_convert_array_to_cpp_buffer_uint32(
    src: Local<'static, crate::v8::Array>,
    dst: *mut u32,
    max_length: u32,
) -> bool {
    false
}

pub fn try_to_copy_and_convert_array_to_cpp_buffer_float(
    src: Local<'static, crate::v8::Array>,
    dst: *mut f32,
    max_length: u32,
) -> bool {
    false
}

pub fn try_to_copy_and_convert_array_to_cpp_buffer_double(
    src: Local<'static, crate::v8::Array>,
    dst: *mut f64,
    max_length: u32,
) -> bool {
    false
}
