// Converted from V8 C++ source files:
// Header: types.h
// Implementation: types.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::{max, min};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;

pub struct Factory {}

impl Factory {
    pub fn NewTurboshaftWord32RangeType(
        &self,
        from: u32,
        to: u32,
        allocation_type: AllocationType,
    ) -> Handle<TurboshaftType> {
        Handle::new(TurboshaftType::Word32Range { from, to })
    }
    pub fn NewTurboshaftWord32SetType(&self, size: i32, allocation_type: AllocationType) -> Handle<TurboshaftType> {
        Handle::new(TurboshaftType::Word32Set { size: 0, elements: Vec::new() })
    }

    pub fn NewTurboshaftWord64RangeType(
        &self,
        from_high: u32,
        from_low: u32,
        to_high: u32,
        to_low: u32,
        allocation_type: AllocationType,
    ) -> Handle<TurboshaftType> {
        Handle::new(TurboshaftType::Word64Range {
            from_high,
            from_low,
            to_high,
            to_low,
        })
    }
    pub fn NewTurboshaftWord64SetType(&self, size: i32, allocation_type: AllocationType) -> Handle<TurboshaftType> {
        Handle::new(TurboshaftType::Word64Set { size: 0, elements_high: Vec::new(), elements_low: Vec::new() })
    }
    pub fn NewTurboshaftFloat64RangeType(
        &self,
        special_values: u32,
        padding: u32,
        min: f64,
        max: f64,
        allocation_type: AllocationType,
    ) -> Handle<TurboshaftType> {
        Handle::new(TurboshaftType::Float64Range {
            special_values,
            padding,
            min,
            max,
        })
    }
    pub fn NewTurboshaftFloat64SetType(
        &self,
        special_values: u32,
        size: i32,
        allocation_type: AllocationType,
    ) -> Handle<TurboshaftType> {
        Handle::new(TurboshaftType::Float64Set { special_values, size: 0, elements: Vec::new() })
    }
}

pub struct StdoutStream {}

impl std::io::Write for StdoutStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        print!("{}", String::from_utf8_lossy(buf));
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AllocationType {
    Young,
}

#[derive(Clone)]
pub struct Handle<T> {
    ptr: Rc<T>,
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Handle { ptr: Rc::new(value) }
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

#[derive(Debug, Clone)]
pub enum TurboshaftType {
    Word32Range { from: u32, to: u32 },
    Word32Set { size: i32, elements: Vec<u32> },
    Word64Range {
        from_high: u32,
        from_low: u32,
        to_high: u32,
        to_low: u32,
    },
    Word64Set { size: i32, elements_high: Vec<u32>, elements_low: Vec<u32> },
    Float64Range {
        special_values: u32,
        padding: u32,
        min: f64,
        max: f64,
    },
    Float64Set { special_values: u32, size: i32, elements: Vec<f64> },
}

#[macro_export]
macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}

#[macro_export]
macro_rules! DCHECK_EQ {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!(
                "DCHECK_EQ failed: {} != {}",
                stringify!($left),
                stringify!($right)
            );
        }
    };
}

#[macro_export]
macro_rules! DCHECK_NE {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!(
                "DCHECK_NE failed: {} == {}",
                stringify!($left),
                stringify!($right)
            );
        }
    };
}

#[macro_export]
macro_rules! DCHECK_LT {
    ($left:expr, $right:expr) => {
        if !($left < $right) {
            panic!(
                "DCHECK_LT failed: {} < {}",
                stringify!($left),
                stringify!($right)
            );
        }
    };
}

#[macro_export]
macro_rules! DCHECK_LE {
    ($left:expr, $right:expr) => {
        if !($left <= $right) {
            panic!(
                "DCHECK_LE failed: {} <= {}",
                stringify!($left),
                stringify!($right)
            );
        }
    };
}

#[macro_export]
macro_rules! DCHECK_GT {
    ($left:expr, $right:expr) => {
        if !($left > $right) {
            panic!(
                "DCHECK_GT failed: {} > {}",
                stringify!($left),
                stringify!($right)
            );
        }
    };
}

#[macro_export]
macro_rules! DCHECK_GE {
    ($left:expr, $right:expr) => {
        if !($left >= $right) {
            panic!(
                "DCHECK_GE failed: {} >= {}",
                stringify!($left),
                stringify!($right)
            );
        }
    };
}

#[macro_export]
macro_rules! DCHECK_IMPLIES {
    ($condition:expr, $implication:expr) => {
        if $condition && !$implication {
            panic!(
                "DCHECK_IMPLIES failed: ({}) implies ({})",
                stringify!($condition),
                stringify!($implication)
            );
        }
    };
}

#[macro_export]
macro_rules! UNREACHABLE {
    () => {
        panic!("UNREACHABLE");
    };
}

#[macro_export]
macro_rules! UNIMPLEMENTED {
    () => {
        panic!("UNIMPLEMENTED");
    };
}

mod detail {
    pub fn is_unique_and_sorted<T: Ord>(container: &[T]) -> bool {
        if container.len() <= 1 {
            return true;
        }
        let mut cur = container.iter();
        let mut next = container.iter();
        next.next();

        for (cur, next) in cur.zip(next) {
            if !(cur < next) {
                return false;
            }
        }
        true
    }

    pub fn is_minus_zero<T>(value: T) -> bool
    where
        T: std::convert::Into<f64>,
    {
        let value_f64: f64 = value.into();
        value_f64 == 0.0 && value_f64.is_sign_negative()
    }

    pub fn is_float_special_value<T>(value: T) -> bool
    where
        T: std::convert::Into<f64>,
    {
        let value_f64: f64 = value.into();
        value_f64.is_nan() || is_minus_zero(value_f64)
    }

    pub struct TypeForBits<const BITS: usize>;

    impl TypeForBits<32> {
        pub type uint_type = u32;
        pub type float_type = f32;
        pub const nan: Self::float_type = f32::NAN;
    }

    impl TypeForBits<64> {
        pub type uint_type = u64;
        pub type float_type = f64;
        pub const nan: Self::float_type = f64::NAN;
    }

    #[derive(Clone, Copy)]
    pub struct Payload_Empty {
        pub dummy: u8,
    }

    impl Default for Payload_Empty {
        fn default() -> Self {
            Payload_Empty { dummy: 0 }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Payload_Range<T> {
        pub min: T,
        pub max: T,
    }

    #[derive(Clone, Copy)]
    pub struct Payload_InlineSet<T> {
        pub elements: [T; 2],
    }

    #[derive(Clone)]
    pub struct Payload_OutlineSet<T> {
        pub array: Vec<T>,
    }
}

pub fn next_smaller<T>(v: T) -> T
where
    T: std::fmt::Debug + std::ops::Sub<Output = T> + From<i32> + Copy + PartialOrd,
{
    let one: T = 1.into();
    DCHECK!(!(std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() && v.to_string() == "NaN"));
    DCHECK_LT!(T::from(i32::min_value()), v);
    v - one
}

pub fn next_larger<T>(v: T) -> T
where
    T: std::fmt::Debug + std::ops::Add<Output = T> + From<i32> + Copy + PartialOrd,
{
    let one: T = 1.into();
    DCHECK!(!(std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() && v.to_string() == "NaN"));
    DCHECK_LT!(v, T::from(i32::max_value()));
    v + one
}

pub type uint_type<const BITS: usize> =
    <detail::TypeForBits<BITS> as detail::TypeForBitsTrait>::uint_type;
pub type float_type<const BITS: usize> =
    <detail::TypeForBits<BITS> as detail::TypeForBitsTrait>::float_type;
pub const nan_v: f64 = f64::NAN;

trait detail::TypeForBitsTrait {
    type uint_type;
    type float_type;
}

impl detail::TypeForBitsTrait for detail::TypeForBits<32> {
    type uint_type = u32;
    type float_type = f32;
}

impl detail::TypeForBitsTrait for detail::TypeForBits<64> {
    type uint_type = u64;
    type float_type = f64;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    kind_: Kind,
    sub_kind_: u8,
    set_size_: u8,
    reserved_: u8,
    bitfield_: u32,
    payload_: [u64; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    kInvalid,
    kNone,
    kWord32,
    kWord64,
    kFloat32,
    kFloat64,
    kTuple,
    kAny,
}

impl Type {
    pub fn new(kind: Kind) -> Self {
        Type {
            kind_: kind,
            sub_kind_: 0,
            set_size_: 0,
            reserved_: 0,
            bitfield_: 0,
            payload_: [0; 2],
        }
    }

    pub fn invalid() -> Self {
        Type::new(Kind::kInvalid)
    }

    pub fn none() -> Self {
        Type::new(Kind::kNone)
    }

    pub fn any() -> Self {
        Type::new(Kind::kAny)
    }

    pub fn kind(&self) -> Kind {
        self.kind_
    }

    pub fn is_invalid(&self) -> bool {
        self.kind_ == Kind::kInvalid
    }

    pub fn is_none(&self) -> bool {
        self.kind_ == Kind::kNone
    }

    pub fn is_word32(&self) -> bool {
        self.kind_ == Kind::kWord32
    }

    pub fn is_word64(&self) -> bool {
        self.kind_ == Kind::kWord64
    }

    pub fn is_float32(&self) -> bool {
        self.kind_ == Kind::kFloat32
    }

    pub fn is_float64(&self) -> bool {
        self.kind_ == Kind::kFloat64
    }

    pub fn is_tuple(&self) -> bool {
        self.kind_ == Kind::kTuple
    }

    pub fn is_any(&self) -> bool {
        self.kind_ == Kind::kAny
    }

    pub fn is_word<const B: usize>(&self) -> bool {
        if B == 32 {
            self.is_word32()
        } else if B == 64 {
            self.is_word64()
        } else {
            panic!("B must be 32 or 64");
        }
    }

    pub fn is_float<const B: usize>(&self) -> bool {
        if B == 32 {
            self.is_float32()
        } else if B == 64 {
            self.is_float64()
        } else {
            panic!("B must be 32 or 64");
        }
    }

    pub fn as_word32(&self) -> &Word32Type {
        DCHECK!(self.is_word32());
        unsafe { &*(self as *const Type as *const Word32Type) }
    }

    pub fn as_word64(&self) -> &Word64Type {
        DCHECK!(self.is_word64());
        unsafe { &*(self as *const Type as *const Word64Type) }
    }

    pub fn as_float32(&self) -> &Float32Type {
        DCHECK!(self.is_float32());
        unsafe { &*(self as *const Type as *const Float32Type) }
    }

    pub fn as_float64(&self) -> &Float64Type {
        DCHECK!(self.is_float64());
        unsafe { &*(self as *const Type as *const Float64Type) }
    }

    pub fn as_tuple(&self) -> &TupleType {
        DCHECK!(self.is_tuple());
        unsafe { &*(self as *const Type as *const TupleType) }
    }

    pub fn as_word<const B: usize>(&self) -> &WordType<B> {
        if B == 32 {
            self.as_word32() as &WordType<B>
        } else if B == 64 {
            self.as_word64() as &WordType<B>
        } else {
            panic!("B must be 32 or 64");
        }
    }

    pub fn as_float<const B: usize>(&self) -> &FloatType<B> {
        if B == 32 {
            self.as_float32() as &FloatType<B>
        } else if B == 64 {
            self.as_float64() as &FloatType<B>
        } else {
            panic!("B must be 32 or 64");
        }
    }

    pub fn equals(&self, other: &Type) -> bool {
        DCHECK!(!self.is_invalid());
        DCHECK!(!other.is_invalid());

        if self.kind_ != other.kind_ {
            return false;
        }
        match self.kind_ {
            Kind::kInvalid => {
                UNREACHABLE!();
                false
            }
            Kind::kNone => true,
            Kind::kWord32 => self.as_word32().equals(other.as_word32()),
            Kind::kWord64 => self.as_word64().equals(other.as_word64()),
            Kind::kFloat32 => self.as_float32().equals(other.as_float32()),
            Kind::kFloat64 => self.as_float64().equals(other.as_float64()),
            Kind::kTuple => self.as_tuple().equals(other.as_tuple()),
            Kind::kAny => true,
        }
    }

    pub fn is_subtype_of(&self, other: &Type) -> bool {
        DCHECK!(!self.is_invalid());
        DCHECK!(!other.is_invalid());

        if other.is_any() || self.is_none() {
            return true;
        }
        if self.kind_ != other.kind_ {
            return false;
        }

        match self.kind_ {
            Kind::kInvalid | Kind::kNone => {
                UNREACHABLE!();
                false
            }
            Kind::kWord32 => self.as_word32().is_subtype_of(other.as_word32()),
            Kind::kWord64 => self.as_word64().is_subtype_of(other.as_word64()),
            Kind::kFloat32 => self.as_float32().is_subtype_of(other.as_float32()),
            Kind::kFloat64 => self.as_float64().is_subtype_of(other.as_float64()),
            Kind::kTuple => self.as_tuple().is_subtype_of(other.as_tuple()),
            Kind::kAny => {
                UNREACHABLE!();
                false
            }
        }
    }

    pub fn print_to(&self, stream: &mut impl std::io::Write) -> std::io::Result<()> {
        match self.kind_ {
            Kind::kInvalid => {
                UNREACHABLE!();
                Ok(())
            }
            Kind::kNone => {
                write!(stream, "None")
            }
            Kind::kWord32 => {
                self.as_word32().print_to(stream)
            }
            Kind::kWord64 => {
                self.as_word64().print_to(stream)
            }
            Kind::kFloat32 => {
                self.as_float32().print_to(stream)
            }
            Kind::kFloat64 => {
                self.as_float64().print_to(stream)
            }
            Kind::kTuple => {
                self.as_tuple().print_to(stream)
            }
            Kind::kAny => {
                write!(stream, "Any")
            }
        }
    }

    pub fn print(&self) {
        let mut os = StdoutStream {};
        self.print_to(&mut os).unwrap();
        println!();
    }

    pub fn to_string(&self) -> String {
        let mut stream = Vec::new();
        self.print_to(&mut stream).unwrap();
        String::from_utf8(stream).unwrap()
    }

    pub fn least_upper_bound(lhs: &Type, rhs: &Type, zone: *mut Zone) -> Type {
        if lhs.is_any() || rhs.is_any() {
            return Type::any();
        }
        if lhs.is_none() {
            return rhs.clone();
        }
        if rhs.is_none() {
            return lhs.clone();
        }

        if lhs.kind() != rhs.kind() {
            return Type::any();
        }

        match lhs.kind() {
            Kind::kInvalid | Kind::kNone | Kind::kAny => {
                UNREACHABLE!();
                Type::invalid()
            }
            Kind::kWord32 => Word32Type::least_upper_bound(lhs.as_word32(), rhs.as_word32(), zone),
            Kind::kWord64 => Word64Type::least_upper_bound(lhs.as_word64(), rhs.as_word64(), zone),
            Kind::kFloat32 => {
                Float32Type::least_upper_bound(lhs.as_float32(), rhs.as_float32(), zone)
            }
            Kind::kFloat64 => {
                Float64Type::least_upper_bound(lhs.as_float64(), rhs.as_float64(), zone)
            }
            Kind::kTuple => TupleType::least_upper_bound(lhs.as_tuple(), rhs.as_tuple(), zone),
        }
    }

    pub fn parse_from_string(str: &str, zone: *mut Zone) -> Option<Type> {
        let parser = TypeParser {
            input: str.to_string(),
            pos: 0,
            zone,
        };
        parser.parse()
    }

    pub fn allocate_on_heap(&self, factory: *mut Factory) -> Handle<TurboshaftType> {
        let factory = unsafe { &*factory };
        match self.kind_ {
            Kind::kInvalid => {
                UNREACHABLE!();
                Handle::new(TurboshaftType::Word32Range { from: 0, to: 0 }) // Dummy handle
            }
            Kind::kNone => {
                UNIMPLEMENTED!();
                Handle::new(TurboshaftType::Word32Range { from: 0, to: 0 }) // Dummy handle
            }
            Kind::kWord32 => self.as_word32().allocate_on_heap(factory),
            Kind::kWord64 => self.as_word64().allocate_on_heap(factory),
            Kind::kFloat32 => self.as_float32().allocate_on_heap(factory),
            Kind::kFloat64 => self.as_float64().allocate_on_heap(factory),
            Kind::kTuple => {
                UNIMPLEMENTED!();
                Handle::new(TurboshaftType::Word32Range { from: 0, to: 0 }) // Dummy handle
            }
            Kind::kAny => {
                UNIMPLEMENTED!();
                Handle::new(TurboshaftType::Word32Range { from: 0, to: 0 }) // Dummy handle
            }
        }
    }
    fn new_with_payload<Payload: Copy>(
        kind: Kind,
        sub_kind: u8,
        set_size: u8,
        bitfield: u32,
        reserved: u8,
        payload: Payload,
    ) -> Self {
        let mut payload_bytes: [u8; 16] = [0; 16];
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload as *const Payload as *const u8,
                payload_bytes.as_mut_ptr(),
                std::mem::size_of::<Payload>(),
            );
        }

        if std::mem::size_of::<Payload>() < 16 {
            for i in std::mem::size_of::<Payload>()..16 {
                payload_bytes[i] = 0x00;
            }
        }

        Type {
            kind_: kind,
            sub_kind_: sub_kind,
            set_size_: set_size,
            reserved_: reserved,
            bitfield_: bitfield,
            payload_: unsafe { std::mem::transmute(payload_bytes) },
        }
    }
    fn get_payload<Payload: Copy>(&self) -> Payload {
        unsafe {
            let mut payload: Payload = std::mem::zeroed();
            std::ptr::copy_nonoverlapping(
                self.payload_.as_ptr() as *const u8,
                &mut payload as *mut Payload as *mut u8,
                std::mem::size_of::<Payload>(),
            );
            payload
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Kind::kInvalid => write!(f, "Invalid"),
            Kind::kNone => write!(f, "None"),
            Kind::kWord32 => write!(f, "Word32"),
            Kind::kWord64 => write!(f, "Word64"),
            Kind::kFloat32 => write!(f, "Float32"),
            Kind::kFloat64 => write!(f, "Float64"),
            Kind::kTuple => write!(f, "Tuple"),
            Kind::kAny => write!(f, "Any"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.print_to(&mut FmtWrite { fmt: f }).map_err(|_| fmt::Error)
    }
}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind_.hash(state);
        self.sub_kind_.hash(state);
        self.set_size_.hash(state);
        self.reserved_.hash(state);
        self.bitfield_.hash(state);
        self.payload_.hash(state);
    }
}

impl PartialEq for TurboshaftType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TurboshaftType::Word32Range { from: f1, to: t1 }, TurboshaftType::Word32Range { from: f2, to: t2 }) => {
                f1 == f2 && t1 == t2
            }
            _ => false,
        }
    }
}

impl Eq for TurboshaftType {}

struct FmtWrite<'a, 'b> {
    fmt: &'a mut Formatter<'b>,
}

impl<'a, 'b> std::io::Write for FmtWrite<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf).map_err(|_| std::io::ErrorKind::InvalidData)?;
        self.fmt.write_str(s).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "fmt error"))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

const fn is_minus_zero(value: f64) -> bool {
    value == 0.0 && value.is_sign_negative()
}

mod base {
    pub use std::cmp::max;
    pub use std::cmp::min;
    pub use std::vec::Vec;

    pub fn sort<T: Ord>(vec: &mut Vec<T>) {
        vec.sort();
    }

    pub fn vector_append<T: Copy>(dest: &mut Vec<T>, src: &[T]) {
        dest.extend_from_slice(src);
    }

    pub fn none_of<T, F: Fn(&T) -> bool>(elements: &[T], f: F) -> bool {
        !elements.iter().any(f)
    }

    pub struct SmallVector<T, const N: usize> {
        data: Vec<T>,
    }

    impl<T, const N: usize> SmallVector<T, const N> {
        pub fn new() -> Self {
            SmallVector { data: Vec::new() }
        }

        pub fn push_back(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn pop_back(&mut self, n: usize) {
            self.data.truncate(self.data.len() - n);
        }

        pub fn begin(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }

        pub fn end(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    impl<T, const N: usize> FromIterator<T> for SmallVector<T, N> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut v = SmallVector::new();
            for i in iter {
                v.push_back(i);
            }
            v
        }
    }

    pub struct Vector<'a, T> {
        data: &'a [T],
    }

    impl<'a, T> Vector<'a, T> {
        pub fn of(slice: &'a [T]) -> Self {
            Vector { data: slice }
        }

        pub fn size(&self) -> usize {
            self.data.len()
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }
    }

    pub fn VectorOf<T>(elements: &[T]) -> Vector<'_, T> {
        Vector { data: elements }
    }

    impl<'a, T> std::ops::Index<usize> for Vector<'a, T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ResolutionMode {
    kPreciseOrInvalid,
    kOverApproximate,
    kGreatestLowerBound,
}

#[derive(Clone, Debug)]
pub struct WordType<const Bits: usize> {
    base: Type,
}

impl<const Bits: usize> WordType<Bits> {
    const KIND: Kind = if Bits == 32 { Kind::kWord32 } else { Kind::kWord64 };
    const KMAXINLINESETSIZE: usize = 2;
    pub const KMAXSETSIZE: usize = 8;

    pub fn any(zone: *mut Zone) -> Self {
        Self::range(0, std::u64::MAX, zone)
    }

    pub fn range(from: u64, to: u64, zone: *mut Zone) -> Self {
        if to >= from {
            if to - from <= Self::KMAXSETSIZE as u64 - 1 {
                let mut elements: Vec<u64> = Vec::new();
                for i in from..=to {
                    elements.push(i);
                }
                return Self::set(&elements, zone);
            }
        } else {
            if (std::u64::MAX - from + to) <= Self::KMAXSETSIZE as u64 - 2 {
                let mut elements: Vec<u64> = Vec::new();
                for i in from..=std::u64::MAX {
                    elements.push(i);
                }
                for i in 0..=to {
                    elements.push(i);
                }
                elements.sort();
                return Self::set(&elements, zone);
            }
        }

        let payload = detail::Payload_Range { min: from, max: to };
        WordType {
            base: Type::new_with_payload(Self::KIND, SubKind::kRange as u8, 0, 0, 0, payload),
        }
    }

    pub fn set(elements: &[u64], zone: *mut Zone) -> Self {
        DCHECK!(detail::is_unique_and_sorted(elements));
        DCHECK_IMPLIES!(
            elements.len() > Self::KMAXINLINESETSIZE,
            zone != std::ptr::null_mut()
        );
        DCHECK_GT!(elements.len(), 0);
        DCHECK_LE!(elements.len(), Self::KMAXSETSIZE);

        if elements.len() <= Self::KMAXINLINESETSIZE {
            let mut p = detail::Payload_InlineSet { elements: [0, 0] };
            DCHECK_LT!(0, elements.len());
            p.elements[0] = elements[0];
            if elements.len() > 1 {
                p.elements[1] = elements[1];
            }
            WordType {
                base: Type::new_with_payload(
                    Self::KIND,
                    SubKind::kSet as u8,
                    elements.len() as u8,
                    0,
                    0,
                    p,
                ),
            }
        } else {
            if zone.is_null() {
                return WordType::any(zone);
            }
            let mut p = detail::Payload_Outline
