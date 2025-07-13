// Converted from V8 C++ source files:
// Header: macros.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub use crate::v8_config::*;
    pub use crate::base::compiler_specific::*;
    pub use crate::base::logging::*;
    pub mod macros {
        use std::mem;
        use std::marker::PhantomData;
        use std::fmt::Debug;
        use std::any::Any;
        use std::ptr;
        use crate::base::OS;
        // No-op macro which is used to work around MSVC's funky VA_ARGS support.
        macro_rules! EXPAND {
            ($x:expr) => {
                $x
            };
        }
        // This macro does nothing. That's all.
        macro_rules! NOTHING {
            ($($arg:tt)*) => {};
        }
        macro_rules! CONCAT_ {
            ($a:ident, $($b:tt)*) => {
                concat_idents!($a, $($b)*)
            };
        }
        macro_rules! CONCAT {
            ($a:ident, $($b:tt)*) => {
                CONCAT_!($a, $($b)*)
            };
        }
        // Creates an unique identifier. Useful for scopes to avoid shadowing names.
        macro_rules! UNIQUE_IDENTIFIER {
            ($base:ident) => {
                concat_idents!($base, __COUNTER__)
            };
        }
        // COUNT_MACRO_ARGS(...) returns the number of arguments passed. Currently, up
        // to 8 arguments are supported.
        macro_rules! COUNT_MACRO_ARGS {
            ($($args:tt),*) => {
                COUNT_MACRO_ARGS_IMPL!($($args,)* 8, 7, 6, 5, 4, 3, 2, 1, 0)
            };
        }
        macro_rules! COUNT_MACRO_ARGS_IMPL {
            ($arg1:tt, $arg2:tt, $arg3:tt, $arg4:tt, $arg5:tt, $arg6:tt, $arg7:tt, $arg8:tt, $n:tt, $($rest:tt)*) => {
                $n
            };
        }
        // GET_NTH_ARG(N, ...) returns the Nth argument in the list of arguments
        // following. Currently, up to N=8 is supported.
        macro_rules! GET_NTH_ARG {
            ($n:tt, $($args:tt),*) => {
                concat_idents!(GET_NTH_ARG_IMPL_, $n)($($args),*)
            };
        }
        macro_rules! GET_NTH_ARG_IMPL_0 {
            ($arg0:tt, $($args:tt)*) => {
                $arg0
            };
        }
        macro_rules! GET_NTH_ARG_IMPL_1 {
            ($arg0:tt, $arg1:tt, $($args:tt)*) => {
                $arg1
            };
        }
        macro_rules! GET_NTH_ARG_IMPL_2 {
            ($arg0:tt, $arg1:tt, $arg2:tt, $($args:tt)*) => {
                $arg2
            };
        }
        macro_rules! GET_NTH_ARG_IMPL_3 {
            ($arg0:tt, $arg1:tt, $arg2:tt, $arg3:tt, $($args:tt)*) => {
                $arg3
            };
        }
        macro_rules! GET_NTH_ARG_IMPL_4 {
            ($arg0:tt, $arg1:tt, $arg2:tt, $arg3:tt, $arg4:tt, $($args:tt)*) => {
                $arg4
            };
        }
        macro_rules! GET_NTH_ARG_IMPL_5 {
            ($arg0:tt, $arg1:tt, $arg2:tt, $arg3:tt, $arg4:tt, $arg5:tt, $($args:tt)*) => {
                $arg5
            };
        }
        macro_rules! GET_NTH_ARG_IMPL_6 {
            ($arg0:tt, $arg1:tt, $arg2:tt, $arg3:tt, $arg4:tt, $arg5:tt, $arg6:tt, $($args:tt)*) => {
                $arg6
            };
        }
        macro_rules! GET_NTH_ARG_IMPL_7 {
            ($arg0:tt, $arg1:tt, $arg2:tt, $arg3:tt, $arg4:tt, $arg5:tt, $arg6:tt, $arg7:tt, $($args:tt)*) => {
                $arg7
            };
        }
        // UNPAREN(x) removes a layer of nested parentheses on x, if any. This means
        // that both UNPAREN(x) and UNPAREN((x)) expand to x. This is helpful for macros
        // that want to support multi argument templates with commas, e.g.
        //
        //   #define FOO(Type, Name) UNPAREN(Type) Name;
        //
        // will work with both
        //
        //   FOO(int, x);
        //   FOO((Foo<int, double, float>), x);
        macro_rules! UNPAREN {
            ($x:tt) => {
                concat_idents!(DROP_, UNPAREN_ $x)
            };
        }
        macro_rules! UNPAREN_ {
            ($($args:tt)*) => {
                UNPAREN_ $($args)*
            };
        }
        macro_rules! DROP_UNPAREN_ {
            () => {}
        }
        macro_rules! OFFSET_OF {
            ($type:ty, $field:tt) => {
                unsafe {
                    let field = &(*(ptr::null::<$type>())).$field;
                    (field as *const _ as usize) - (ptr::null::<$type>() as *const _ as usize)
                }
            };
        }
        // A comma, to be used in macro arguments where it would otherwise be
        // interpreted as separator of arguments.
        const LITERAL_COMMA: &str = ",";
        // The arraysize(arr) macro returns the # of elements in an array arr.
        // The expression is a compile-time constant, and therefore can be
        // used in defining new arrays, for example.  If you use arraysize on
        // a pointer by mistake, you will get a compile-time error.
        macro_rules! arraysize {
            ($array:expr) => {
                ArraySizeHelper($array).len()
            };
        }
        // This template function declaration is used in defining arraysize.
        // Note that the function doesn't need an implementation, as we only
        // use its type.
        fn ArraySizeHelper<T, const N: usize>(array: &[T; N]) -> &[T; N] {
            array
        }
        // This is an equivalent to C++20's std::bit_cast<>(), but with additional
        // warnings. It morally does what `*reinterpret_cast<Dest*>(&source)` does, but
        // the cast/deref pair is undefined behavior, while bit_cast<>() isn't.
        //
        // This is not a magic "get out of UB free" card. This must only be used on
        // values, not on references or pointers. For pointers, use
        // reinterpret_cast<>(), or static_cast<>() when casting between void* and other
        // pointers, and then look at https://eel.is/c++draft/basic.lval#11 as that's
        // probably UB also.
        pub mod v8_base {
            use std::mem;
            use std::any::type_name;
            pub fn bit_cast<Dest: Sized + Copy + Debug, Source: Sized + Copy + Debug>(source: Source) -> Dest {
                if std::any::TypeId::of::<Source>() == std::any::TypeId::of::<*mut ()>() ||
                    std::any::TypeId::of::<Dest>() == std::any::TypeId::of::<*mut ()>() {
                    panic!("bit_cast must not be used on pointer types");
                }
                if std::any::TypeId::of::<Dest>() == std::any::TypeId::of::<&()>() {
                    panic!("bit_cast must not be used on reference types");
                }
                if mem::size_of::<Dest>() != mem::size_of::<Source>() {
                    panic!("bit_cast requires source and destination types to be the same size. Source: {}, Dest: {}", mem::size_of::<Source>(), mem::size_of::<Dest>());
                }
                if !std::intrinsics::needs_drop::<Source>() {
                } else {
                    panic!("bit_cast requires the source type to be trivially copyable: {}", type_name::<Source>());
                }
                if !std::intrinsics::needs_drop::<Dest>() {
                } else {
                    panic!("bit_cast requires the destination type to be trivially copyable: {}", type_name::<Dest>());
                }
                unsafe {
                    let mut dest: Dest = mem::uninitialized();
                    std::ptr::copy_nonoverlapping(&source as *const Source as *const u8, &mut dest as *mut Dest as *mut u8, mem::size_of::<Dest>());
                    dest
                }
            }
        }  // namespace v8::base
        // Explicitly declare the assignment operator as deleted.
        // Note: This macro is deprecated and will be removed soon. Please explicitly
        // delete the assignment operator instead.
        macro_rules! DISALLOW_ASSIGN {
            ($TypeName:ident) => {
                impl $TypeName {
                    #[inline(always)]
                    fn operator_assign(&mut self, _other: &Self) {
                      panic!("Assignment operator is deleted for type {}", stringify!($TypeName));
                    }
                }
            };
        }
        // Explicitly declare all implicit constructors as deleted, namely the
        // default constructor, copy constructor and operator= functions.
        // This is especially useful for classes containing only static methods.
        macro_rules! DISALLOW_IMPLICIT_CONSTRUCTORS {
            ($TypeName:ident) => {
                impl $TypeName {
                  #[inline(always)]
                    fn new() {
                      panic!("Default constructor is deleted for type {}", stringify!($TypeName));
                    }
                  #[inline(always)]
                    fn copy(&self) {
                      panic!("Copy constructor is deleted for type {}", stringify!($TypeName));
                    }
                }
                DISALLOW_ASSIGN!($TypeName);
            };
        }
        // Disallow copying a type, but provide default construction, move construction
        // and move assignment. Especially useful for move-only structs.
        macro_rules! MOVE_ONLY_WITH_DEFAULT_CONSTRUCTORS {
            ($TypeName:ident) => {
                impl $TypeName {
                    #[inline(always)]
                    fn new() -> Self {
                       Default::default()
                    }
                }
                MOVE_ONLY_NO_DEFAULT_CONSTRUCTOR!($TypeName);
            };
        }
        // Disallow copying a type, and only provide move construction and move
        // assignment. Especially useful for move-only structs.
        macro_rules! MOVE_ONLY_NO_DEFAULT_CONSTRUCTOR {
            ($TypeName:ident) => {
                impl $TypeName {
                    // Move constructor
                    #[inline(always)]
                    fn move_construct(self) -> Self {
                        self
                    }
                    // Move assignment operator
                    #[inline(always)]
                    fn move_assign(&mut self, mut other: Self) {
                        std::mem::swap(self, &mut other);
                    }
                  #[inline(always)]
                    fn copy(&self) {
                      panic!("Copy constructor is deleted for type {}", stringify!($TypeName));
                    }
                }
                DISALLOW_ASSIGN!($TypeName);
            };
        }
        // A macro to disallow the dynamic allocation.
        // This should be used in the private: declarations for a class
        // Declaring operator new and delete as deleted is not spec compliant.
        // Extract from 3.2.2 of C++11 spec:
        //  [...] A non-placement deallocation function for a class is
        //  odr-used by the definition of the destructor of that class, [...]
        macro_rules! DISALLOW_NEW_AND_DELETE {
            () => {
                compile_error!("Dynamic allocation is disallowed.");
            };
        }
        // Define V8_USE_ADDRESS_SANITIZER macro.
        // Define V8_USE_HWADDRESS_SANITIZER macro.
        // Define V8_USE_MEMORY_SANITIZER macro.
        // Define V8_USE_UNDEFINED_BEHAVIOR_SANITIZER macro.
        // Define V8_USE_SAFE_STACK macro.
        // DISABLE_CFI_PERF -- Disable Control Flow Integrity checks for Perf reasons.
        macro_rules! DISABLE_CFI_PERF {
            () => {
                /*V8_CLANG_NO_SANITIZE!("cfi")*/
            };
        }
        // DISABLE_CFI_ICALL -- Disable Control Flow Integrity indirect call checks,
        // useful because calls into JITed code can not be CFI verified. Same for
        // UBSan's function pointer type checks.
        macro_rules! DISABLE_CFI_ICALL {
            () => {
                /*V8_CLANG_NO_SANITIZE!("cfi-icall")
                V8_CLANG_NO_SANITIZE!("function")
                __declspec(guard(nocf))*/
            };
        }
        // V8_PRETTY_FUNCTION_VALUE_OR(ELSE) emits a pretty function value, if
        // available for this compiler, otherwise it emits ELSE.
        macro_rules! V8_PRETTY_FUNCTION_VALUE_OR {
            ($ELSE:expr) => {
                function!()
            };
        }
        pub mod v8 {
            pub mod base {
                use std::marker::PhantomData;
                use std::mem;
                use std::fmt::Debug;
                use std::any::type_name;
                pub struct is_trivially_copyable<T> {
                    _phantom: PhantomData<T>,
                }
                impl<T> is_trivially_copyable<T> {
                    pub const value: bool = !std::mem::needs_drop::<T>();
                }
                macro_rules! ASSERT_TRIVIALLY_COPYABLE {
                  ($t:ty) => {
                    const _: () = assert!(::v8::base::is_trivially_copyable::<$t>::value, concat!(stringify!($t), " should be trivially copyable"));
                  };
                }
                pub(crate) use ASSERT_TRIVIALLY_COPYABLE;
                macro_rules! ASSERT_NOT_TRIVIALLY_COPYABLE {
                  ($t:ty) => {
                    const _: () = assert!(!::v8::base::is_trivially_copyable::<$t>::value, concat!(stringify!($t), " should not be trivially copyable"));
                  };
                }
                pub(crate) use ASSERT_NOT_TRIVIALLY_COPYABLE;
                pub struct is_trivially_destructible<T>(PhantomData<T>);
                impl<T> is_trivially_destructible<T> {
                    pub const value: bool = !std::mem::needs_drop::<T>();
                }
                macro_rules! ASSERT_TRIVIALLY_DESTRUCTIBLE {
                  ($t:ty) => {
                    const _: () = assert!(::v8::base::is_trivially_destructible::<$t>::value, concat!(stringify!($t), " should be trivially destructible"));
                  };
                }
                pub(crate) use ASSERT_TRIVIALLY_DESTRUCTIBLE;
                macro_rules! ASSERT_NOT_TRIVIALLY_DESTRUCTIBLE {
                  ($t:ty) => {
                    const _: () = assert!(!::v8::base::is_trivially_destructible::<$t>::value, concat!(stringify!($t), " should not be trivially destructible"));
                  };
                }
                pub(crate) use ASSERT_NOT_TRIVIALLY_DESTRUCTIBLE;
                // The USE(x, ...) template is used to silence C++ compiler warnings
                // issued for (yet) unused variables (typically parameters).
                // The arguments are guaranteed to be evaluated from left to right.
                pub struct Use {}
                impl Use {
                    pub fn new<T>(_value: T) -> Self {
                        Use {}
                    }
                }
                macro_rules! USE {
                    ($($arg:expr),*) => {
                        {
                            #[allow(unused_mut)]
                            let mut unused_tmp_array_for_use_macro = [$(::v8::base::Use::new($arg)),*];
                            let _ = &unused_tmp_array_for_use_macro;
                        }
                    };
                }
                pub(crate) use USE;
            }  // namespace base
        }  // namespace v8
        // implicit_cast<A>(x) triggers an implicit cast from {x} to type {A}. This is
        // useful in situations where static_cast<A>(x) would do too much.
        // Only use this for cheap-to-copy types, or use move semantics explicitly.
        #[inline]
        pub fn implicit_cast<A>(x: A) -> A {
            x
        }
        // Define our own macros for writing 64-bit constants.  This is less fragile
        // than defining __STDC_CONSTANT_MACROS before including <stdint.h>, and it
        // works on compilers that don't have it (like MSVC).
        //ptr prefix
        // Hex digits
        // ptrdiff
        //uint64 from uint32
        #[inline]
        pub fn make_uint64(high: u32, low: u32) -> u64 {
            ((high as u64) << 32) + low as u64
        }
        // Return the largest multiple of m which is <= x.
        #[inline]
        pub const fn round_down<T>(x: T, m: isize) -> T
        where
            T: std::ops::BitAnd<Output = T> + std::ops::Neg + std::convert::From<isize> + std::marker::Copy,
        {
            // m must be a power of two.
            //DEBUG_ASSERT(m != 0 && ((m & (m - 1)) == 0));
            x & ((-m).into())
        }
        // Return the smallest multiple of m which is >= x.
        #[inline]
        pub const fn round_up<T>(x: T, m: isize) -> T
        where
            T: std::ops::Add<Output = T>
                + std::ops::Sub<Output = T>
                + std::ops::BitAnd<Output = T>
                + std::ops::Neg
                + std::convert::From<isize>
                + std::marker::Copy
                + PartialOrd,
        {
            //DEBUG_ASSERT(x >= 0);
            //DEBUG_ASSERT(std::u64::MAX - (x as u64) >= (m - 1) as u64); // Overflow check.
            round_down((x + (m - 1).into()), m)
        }
        #[inline]
        pub const fn is_aligned<T, U>(value: T, alignment: U) -> bool
        where
            T: std::ops::BitAnd<U, Output = T> + PartialEq + std::convert::From<i32>,
            U: std::ops::Sub<i32, Output = U> + Copy,
        {
            (value & (alignment - 1)) == 0.into()
        }
        #[inline]
        pub fn aligned_address(address: *mut std::ffi::c_void, alignment: usize) -> *mut std::ffi::c_void {
            let address_int = address as usize;
            let rounded_down = round_down(address_int, alignment as isize);
            rounded_down as *mut std::ffi::c_void
        }
        #[inline]
        pub fn round_up_address(address: *mut std::ffi::c_void, alignment: usize) -> *mut std::ffi::c_void {
            let address_int = address as usize;
            let rounded_up = round_up(address_int, alignment as isize);
            rounded_up as *mut std::ffi::c_void
        }
        // Bounds checks for float to integer conversions, which does truncation. Hence,
        // the range of legal values is (min - 1, max + 1).
        pub fn is_inbounds<int_t, float_t, biggest_int_t>(v: float_t) -> bool
        where
            int_t: std::convert::From<i8>,
            float_t: std::convert::From<f32> + PartialOrd,
            biggest_int_t: std::convert::From<i64>,
        {
            let kLowerBound: float_t = std::convert::From::from(std::i8::MIN as i32 as f32) - std::convert::From::from(1.0 as f32);
            let kUpperBound: float_t = std::convert::From::from(std::i8::MAX as i32 as f32) + std::convert::From::from(1.0 as f32);
            let kLowerBoundIsMin: bool =
                biggest_int_t::from(kLowerBound) == biggest_int_t::from(std::i8::MIN as i32 as i64);
            let kUpperBoundIsMax: bool =
                biggest_int_t::from(kUpperBound) == biggest_int_t::from(std::i8::MAX as i32 as i64);
            v8::base::macros::v8::base::USE!(kLowerBoundIsMin);
            v8::base::macros::v8::base::USE!(kUpperBoundIsMax);
            if kLowerBoundIsMin {
                if kLowerBound <= v {
                    if kUpperBoundIsMax {
                        v <= kUpperBound
                    } else {
                        v < kUpperBound
                    }
                } else {
                    false
                }
            } else {
                if kLowerBound < v {
                    if kUpperBoundIsMax {
                        v <= kUpperBound
                    } else {
                        v < kUpperBound
                    }
                } else {
                    false
                }
            }
        }
        // Setup for Windows shared library export.
        // Setup for Linux shared library export.
        // Defines IF_WASM, to be used in macro lists for elements that should only be
        // there if WebAssembly is enabled.
        macro_rules! IF_WASM {
            ($V:ident, $($args:tt)*) => {
            };
        }
        // Defines IF_TSAN, to be used in macro lists for elements that should only be
        // there if TSAN is enabled.
        macro_rules! IF_TSAN {
            ($V:ident, $($args:tt)*) => {
            };
        }
        // Defines IF_INTL, to be used in macro lists for elements that should only be
        // there if INTL is enabled.
        macro_rules! IF_INTL {
            ($V:ident, $($args:tt)*) => {
            };
        }
        // Defines IF_SHADOW_STACK, to be used in macro lists for elements that should
        // only be there if CET shadow stack is enabled.
        macro_rules! IF_SHADOW_STACK {
            ($V:ident, $($args:tt)*) => {
            };
        }
        // Defines IF_TARGET_ARCH_64_BIT, to be used in macro lists for elements that
        // should only be there if the target architecture is a 64-bit one.
        macro_rules! IF_TARGET_ARCH_64_BIT {
            ($V:ident, $($args:tt)*) => {
            };
        }
        // Defines IF_V8_WASM_RANDOM_FUZZERS and IF_NO_V8_WASM_RANDOM_FUZZERS, to be
        // used in macro lists for elements that should only be there/absent when
        // building the Wasm fuzzers.
        macro_rules! IF_V8_WASM_RANDOM_FUZZERS {
            ($V:ident, $($args:tt)*) => {
            };
        }
        macro_rules! IF_NO_V8_WASM_RANDOM_FUZZERS {
            ($V:ident, $($args:tt)*) => {
            };
        }
        // Disable FRIEND_TEST macro in Google3.
        macro_rules! FRIEND_TEST {
            ($test_case_name:ident, $test_name:ident) => {};
        }
    }
}

mod v8_config {
    pub const V8_CC_MSVC: bool = false;
    pub const V8_CC_GNU: bool = false;
    pub const V8_CC_MINGW64: bool = false;
    pub const V8_HOST_ARCH_64_BIT: bool = cfg!(target_pointer_width = "64");
    pub const V8_TARGET_ARCH_64_BIT: bool = cfg!(target_pointer_width = "64");
    pub const V8_OS_DARWIN: bool = cfg!(target_os = "macos") || cfg!(target_os = "ios");
    pub const V8_OS_AIX: bool = false;
    pub const V8_ENABLE_WEBASSEMBLY: bool = false;
    pub const V8_IS_TSAN: bool = false;
    pub const V8_INTL_SUPPORT: bool = false;
    pub const V8_ENABLE_CET_SHADOW_STACK: bool = false;
    pub const V8_OS_WIN: bool = cfg!(target_os = "windows");
    pub const V8_HAS_ATTRIBUTE_VISIBILITY: bool = true;
    pub const V8_HAS_BUILTIN_BIT_CAST: bool = true;
    pub const V8_ENABLE_DRUMBRAKE: bool = false;
    pub const V8_DRUMBRAKE_BOUNDS_CHECKS: bool = false;
}

mod base {
  pub mod compiler_specific {
    #[macro_export]
    macro_rules! V8_INLINE {
        () => {
            #[inline]
        };
    }
    #[macro_export]
    macro_rules! V8_NOEXCEPT {
        () => {
            //#[noexcept]
        };
    }
      #[macro_export]
      macro_rules! V8_CLANG_NO_SANITIZE {
          ($arg:expr) => {
              //#[cfg(feature = $arg)]
              //#[no_sanitize($arg)]
          }
      }

  }
    pub mod logging {
      #[macro_export]
      macro_rules! DCHECK {
          ($condition:expr) => {
              if !$condition {
                  panic!("DCHECK failed: {}", stringify!($condition));
              }
          };
      }

      #[macro_export]
      macro_rules! DCHECK_GE {
          ($a:expr, $b:expr) => {
              if $a < $b {
                  panic!("DCHECK_GE failed: {} >= {}", stringify!($a), stringify!($b));
              }
          };
      }
    }
    pub mod OS {
        pub fn Abort() {
            std::process::abort();
        }
    }
}
