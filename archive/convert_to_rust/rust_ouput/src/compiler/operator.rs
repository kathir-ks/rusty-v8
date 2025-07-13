// Converted from V8 C++ source files:
// Header: operator.h
// Implementation: operator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod flags {
        use std::ops::{BitAnd, BitOr, BitXor, Not};

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub struct Flags<E, T> {
            bits: T,
            _phantom: std::marker::PhantomData<E>,
        }

        impl<E, T> Flags<E, T> {
            pub const fn new(bits: T) -> Self {
                Flags {
                    bits,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn empty() -> Self
            where
                T: Default,
            {
                Flags::new(T::default())
            }

            pub fn contains(&self, other: Flags<E, T>) -> bool
            where
                T: BitAnd<Output = T> + PartialEq,
            {
                (self.bits & other.bits) == other.bits
            }

            pub fn insert(&mut self, other: Flags<E, T>)
            where
                T: BitOr<Output = T> + Copy,
            {
                self.bits = (self.bits | other.bits);
            }

            pub fn remove(&mut self, other: Flags<E, T>)
            where
                T: BitAnd<Output = T> + Not<Output = T> + Copy,
            {
                self.bits = (self.bits & (!other.bits));
            }

            pub fn toggle(&mut self, other: Flags<E, T>)
            where
                T: BitXor<Output = T> + Copy,
            {
                self.bits = (self.bits ^ other.bits);
            }

            pub fn bits(&self) -> &T {
                &self.bits
            }

            pub fn from_bits(bits: T) -> Self {
                Flags {
                    bits,
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<E, T: Copy> Copy for Flags<E, T> {}

        impl<E, T: Clone> Clone for Flags<E, T> {
            fn clone(&self) -> Self {
                Flags {
                    bits: self.bits.clone(),
                    _phantom: self._phantom,
                }
            }
        }

        impl<E, T: BitOr<Output = T> + Copy> BitOr for Flags<E, T> {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Flags::new(self.bits | rhs.bits)
            }
        }

        impl<E, T: BitAnd<Output = T> + Copy> BitAnd for Flags<E, T> {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Flags::new(self.bits & rhs.bits)
            }
        }

        impl<E, T: BitXor<Output = T> + Copy> BitXor for Flags<E, T> {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Flags::new(self.bits ^ rhs.bits)
            }
        }

        impl<E, T: Not<Output = T> + Copy> Not for Flags<E, T> {
            type Output = Self;

            fn not(self) -> Self::Output {
                Flags::new(!self.bits)
            }
        }

        impl<E, T: Default> Default for Flags<E, T> {
            fn default() -> Self {
                Flags::new(T::default())
            }
        }
    }
    pub mod hashing {
        use std::hash::{Hash, Hasher};

        pub fn hash_combine<T: Hash>(seed: u64, value: T) -> u64 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            hasher.write_u64(seed);
            value.hash(&mut hasher);
            hasher.finish()
        }

        pub struct bit_equal_to<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> bit_equal_to<T> {
            pub fn new() -> Self {
                bit_equal_to {
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<T> Default for bit_equal_to<T> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl PartialEq<f32> for bit_equal_to<f32> {
            fn eq(&self, a: &f32, b: &f32) -> bool {
                a.to_bits() == b.to_bits()
            }
        }

        impl PartialEq<f64> for bit_equal_to<f64> {
            fn eq(&self, a: &f64, b: &f64) -> bool {
                a.to_bits() == b.to_bits()
            }
        }

        pub struct bit_hash<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> bit_hash<T> {
            pub fn new() -> Self {
                bit_hash {
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<T> Default for bit_hash<T> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Hash for bit_hash<f32> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                std::any::TypeId::of::<f32>().hash(state);
            }
        }

        impl Hash for bit_hash<f64> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                std::any::TypeId::of::<f64>().hash(state);
            }
        }

        impl bit_hash<f32> {
            pub fn call(&self, value: &f32) -> u64 {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                value.to_bits().hash(&mut hasher);
                hasher.finish()
            }
        }

        impl bit_hash<f64> {
            pub fn call(&self, value: &f64) -> u64 {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                value.to_bits().hash(&mut hasher);
                hasher.finish()
            }
        }

        pub fn hash<T: Hash>() -> impl Fn(&T) -> u64 {
            |value: &T| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                value.hash(&mut hasher);
                hasher.finish()
            }
        }
    }
}
pub mod common {
    pub mod globals {
        pub const kMaxInt: i32 = i32::max_value();
    }
}
pub mod handles {
    pub struct Handles {}
}
pub mod zone {
    pub struct Zone {}
}

pub mod compiler {
    use crate::base::flags::Flags;
    use crate::base::hashing;
    use crate::common::globals::kMaxInt;
    use crate::handles::Handles;
    use crate::zone::Zone;
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::limits;
    use std::mem::size_of;
    use std::ops::BitAnd;

    pub trait ZoneObject {}

    macro_rules! define_operators_for_flags {
        ($flags_type:ty) => {
            impl std::fmt::Display for $flags_type {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        };
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Operator {
        mnemonic_: &'static str,
        opcode_: Opcode,
        properties_: Properties,
        value_in_: u32,
        effect_in_: u32,
        control_in_: u32,
        value_out_: u32,
        effect_out_: u8,
        control_out_: u32,
    }

    impl fmt::Display for Operator {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.PrintTo(f, PrintVerbosity::kVerbose);
            Ok(())
        }
    }

    impl Operator {
        pub type Opcode = u16;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Property {
            kNoProperties = 0,
            kCommutative = 1 << 0,
            kAssociative = 1 << 1,
            kIdempotent = 1 << 2,
            kNoRead = 1 << 3,
            kNoWrite = 1 << 4,
            kNoThrow = 1 << 5,
            kNoDeopt = 1 << 6,
            kFoldable = Property::kNoRead as isize | Property::kNoWrite as isize,
            kEliminatable = Property::kNoDeopt as isize
                | Property::kNoWrite as isize
                | Property::kNoThrow as isize,
            kKontrol = Property::kNoDeopt as isize
                | Property::kFoldable as isize
                | Property::kNoThrow as isize,
            kPure = Property::kKontrol as isize | Property::kIdempotent as isize,
        }

        macro_rules! operator_property_list {
            ($v:ident) => {
                $v!(Commutative);
                $v!(Associative);
                $v!(Idempotent);
                $v!(NoRead);
                $v!(NoWrite);
                $v!(NoThrow);
                $v!(NoDeopt);
            };
        }

        pub type Properties = Flags<Property, u8>;
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum PrintVerbosity {
            kVerbose,
            kSilent,
        }

        pub fn new(
            opcode: Opcode,
            properties: Properties,
            mnemonic: &'static str,
            value_in: usize,
            effect_in: usize,
            control_in: usize,
            value_out: usize,
            effect_out: usize,
            control_out: usize,
        ) -> Self {
            Operator {
                mnemonic_: mnemonic,
                opcode_: opcode,
                properties_: properties,
                value_in_: Operator::check_range::<u32>(value_in),
                effect_in_: Operator::check_range::<u32>(effect_in),
                control_in_: Operator::check_range::<u32>(control_in),
                value_out_: Operator::check_range::<u32>(value_out),
                effect_out_: Operator::check_range::<u8>(effect_out),
                control_out_: Operator::check_range::<u32>(control_out),
            }
        }

        pub const fn opcode(&self) -> Opcode {
            self.opcode_
        }

        pub fn mnemonic(&self) -> &'static str {
            self.mnemonic_
        }

        pub fn equals(&self, that: &Operator) -> bool {
            self.opcode() == that.opcode()
        }

        pub fn hash_code(&self) -> u64 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            self.opcode().hash(&mut hasher);
            hasher.finish()
        }

        pub fn has_property(&self, property: Property) -> bool {
            (self.properties() & Flags::new(property as u8)) == Flags::new(property as u8)
        }

        pub fn properties(&self) -> Properties {
            self.properties_
        }

        pub fn value_input_count(&self) -> i32 {
            self.value_in_ as i32
        }
        pub fn effect_input_count(&self) -> i32 {
            self.effect_in_ as i32
        }
        pub fn control_input_count(&self) -> i32 {
            self.control_in_ as i32
        }

        pub fn value_output_count(&self) -> i32 {
            self.value_out_ as i32
        }
        pub fn effect_output_count(&self) -> i32 {
            self.effect_out_ as i32
        }
        pub fn control_output_count(&self) -> i32 {
            self.control_out_ as i32
        }

        pub fn zero_if_eliminatable(properties: Properties) -> usize {
            if (properties & Flags::new(Property::kEliminatable as u8))
                == Flags::new(Property::kEliminatable as u8)
            {
                0
            } else {
                1
            }
        }

        pub fn zero_if_no_throw(properties: Properties) -> usize {
            if (properties & Flags::new(Property::kNoThrow as u8))
                == Flags::new(Property::kNoThrow as u8)
            {
                0
            } else {
                2
            }
        }

        pub fn zero_if_pure(properties: Properties) -> usize {
            if (properties & Flags::new(Property::kPure as u8)) == Flags::new(Property::kPure as u8)
            {
                0
            } else {
                1
            }
        }

        pub fn PrintTo(&self, os: &mut fmt::Formatter, verbose: PrintVerbosity) {
            self.print_to_impl(os, verbose);
        }

        pub fn PrintPropsTo(&self, os: &mut fmt::Formatter) -> fmt::Result {
            let mut separator = String::new();

            macro_rules! print_prop_if_set {
                ($name:ident) => {
                    if self.has_property(Operator::Property::k##$name) {
                        write!(os, "{}", separator)?;
                        write!(os, "{}", stringify!($name))?;
                        separator = ", ".to_string();
                    }
                };
            }

            operator_property_list!(print_prop_if_set);
            Ok(())
        }

        fn print_to_impl(&self, os: &mut fmt::Formatter, _verbose: PrintVerbosity) {
            write!(os, "{}", self.mnemonic()).unwrap();
        }

        fn check_range<N>(val: usize) -> N
        where
            N: num_traits::PrimInt,
        {
            if val as u64 > std::cmp::min(N::max_value() as u64, kMaxInt as u64) {
                panic!(
                    "Value {} exceeds the maximum allowed value for type {}",
                    val,
                    std::any::type_name::<N>()
                );
            }
            num_traits::cast(val).unwrap()
        }
    }

    define_operators_for_flags!(Operator::Properties);

    pub fn operator_fmt(os: &mut fmt::Formatter, op: &Operator) -> fmt::Result {
        op.PrintTo(os, Operator::PrintVerbosity::kVerbose);
        Ok(())
    }

    pub trait OpParameterTrait<T> {
        fn parameter(&self) -> &T;
        fn print_parameter(&self, os: &mut fmt::Formatter, verbose: PrintVerbosity) -> fmt::Result;
    }

    pub struct Operator1<T, Pred, Hash>
    where
        Pred: Fn(&T, &T) -> bool,
        Hash: Fn(&T) -> u64,
    {
        base: Operator,
        parameter_: T,
        pred_: Pred,
        hash_: Hash,
    }

    impl<T, Pred, Hash> Operator1<T, Pred, Hash>
    where
        Pred: Fn(&T, &T) -> bool,
        Hash: Fn(&T) -> u64,
        T: Clone + fmt::Display,
    {
        pub fn new(
            opcode: Operator::Opcode,
            properties: Operator::Properties,
            mnemonic: &'static str,
            value_in: usize,
            effect_in: usize,
            control_in: usize,
            value_out: usize,
            effect_out: usize,
            control_out: usize,
            parameter: T,
            pred: Pred,
            hash: Hash,
        ) -> Self {
            Operator1 {
                base: Operator::new(
                    opcode,
                    properties,
                    mnemonic,
                    value_in,
                    effect_in,
                    control_in,
                    value_out,
                    effect_out,
                    control_out,
                ),
                parameter_: parameter,
                pred_: pred,
                hash_: hash,
            }
        }

        pub fn parameter(&self) -> &T {
            &self.parameter_
        }

        pub fn equals(&self, other: &Operator) -> bool {
            if self.base.opcode() != other.opcode() {
                return false;
            }
            // Attempt to downcast the `other` Operator to Operator1<T, Pred, Hash>.
            // If the downcast fails, it means `other` is not of the same type as `self`,
            // and therefore they cannot be equal.
            if let Some(that) = other.downcast_ref::<Self>() {
                return (self.pred_)(&self.parameter_, &that.parameter_);
            }
            false
        }
        pub fn hash_code(&self) -> u64 {
            hashing::hash_combine(self.base.opcode() as u64, (self.hash_)(&self.parameter_))
        }

        pub fn print_parameter(
            &self,
            os: &mut fmt::Formatter,
            verbose: Operator::PrintVerbosity,
        ) -> fmt::Result {
            write!(os, "[{}]", self.parameter())
        }

        pub fn print_to_impl(
            &self,
            os: &mut fmt::Formatter,
            verbose: Operator::PrintVerbosity,
        ) -> fmt::Result {
            write!(os, "{}", self.base.mnemonic())?;
            self.print_parameter(os, verbose)?;
            Ok(())
        }
    }

    impl<T, Pred, Hash> Operator1<T, Pred, Hash>
    where
        Pred: Fn(&T, &T) -> bool,
        Hash: Fn(&T) -> u64,
    {
        pub fn opcode(&self) -> Operator::Opcode {
            self.base.opcode()
        }
    }

    impl<T, Pred, Hash> std::ops::Deref for Operator1<T, Pred, Hash>
    where
        Pred: Fn(&T, &T) -> bool,
        Hash: Fn(&T) -> u64,
    {
        type Target = Operator;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    pub trait OperatorExt {
        fn downcast_ref<T>(&self) -> Option<&T>;
    }

    impl OperatorExt for Operator {
        fn downcast_ref<T>(&self) -> Option<&T> {
            // This is a placeholder. In a real implementation, you would need to use
            // runtime type information (e.g., `std::any::Any`) to safely downcast.
            // Since we don't have the actual Operator1 instances here, we can't
            // implement a correct downcast.
            None
        }
    }
}
