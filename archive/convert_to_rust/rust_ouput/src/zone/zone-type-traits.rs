// Converted from V8 C++ source files:
// Header: zone-type-traits.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod zone_type_traits {
    #[cfg(feature = "V8_COMPRESS_ZONES")]
    use super::compressed_zone_ptr::CompressedZonePtr;

    pub struct ZoneList<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    pub type ZonePtrList<T> = ZoneList<*mut T>;

    pub type FullZonePtr<T> = *mut T;

    pub struct ZoneTypeTraits<const ENABLE_COMPRESSION: bool>;

    impl ZoneTypeTraits<false> {
        pub type Ptr<T> = FullZonePtr<T>;
    }

    impl ZoneTypeTraits<true> {
        pub type Ptr<T> = CompressedZonePtr<T>;
    }

    pub struct IsCompressedPointer<T>(std::marker::PhantomData<T>);

    impl<T> IsCompressedPointer<T> {
        pub const VALUE: bool = false;
    }

    impl<T> IsCompressedPointer<CompressedZonePtr<T>> {
        pub const VALUE: bool = true;
    }

    impl<T> IsCompressedPointer<*const CompressedZonePtr<T>> {
        pub const VALUE: bool = true;
    }
}
