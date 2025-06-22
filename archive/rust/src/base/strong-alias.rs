// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Slightly adapted for inclusion in V8.
// Copyright 2024 the V8 project authors. All rights reserved.

use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub mod base {
    use std::{
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
        marker::PhantomData,
        ops::{Deref, DerefMut},
    };

    /// A type-safe alternative for a typedef or a 'using' directive.
    ///
    /// ---
    /// This is a port of Chromium's base::StrongAlias, keeping the API and naming.
    /// https://source.chromium.org/chromium/chromium/src/+/main:base/types/strong_alias.h;drc=0e7afdb6498599a66ec246045a9accf26da66a2b
    /// ---
    ///
    /// C++ currently does not support type-safe typedefs, despite multiple proposals
    /// (ex. http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2013/n3515.pdf). The
    /// next best thing is to try and emulate them in library code.
    ///
    /// The motivation is to disallow several classes of errors:
    ///
    /// using Orange = int;
    /// using Apple = int;
    /// Apple apple(2);
    /// Orange orange = apple;  // Orange should not be able to become an Apple.
    /// Orange x = orange + apple;  // Shouldn't add Oranges and Apples.
    /// if (orange > apple);  // Shouldn't compare Apples to Oranges.
    /// void foo(Orange);
    /// void foo(Apple);  // Redefinition.
    /// etc.
    ///
    /// StrongAlias may instead be used as follows:
    ///
    /// using Orange = StrongAlias<class OrangeTag, int>;
    /// using Apple = StrongAlias<class AppleTag, int>;
    /// using Banana = StrongAlias<class BananaTag, std::string>;
    /// Apple apple(2);
    /// Banana banana("Hello");
    /// Orange orange = apple;  // Does not compile.
    /// Orange other_orange = orange;  // Compiles, types match.
    /// Orange x = orange + apple;  // Does not compile.
    /// Orange y = Orange(orange.value() + apple.value());  // Compiles.
    /// Orange z = Orange(banana->size() + *other_orange);  // Compiles.
    /// if (orange > apple);  // Does not compile.
    /// if (orange > other_orange);  // Compiles.
    /// void foo(Orange);
    /// void foo(Apple);  // Compiles into separate overload.
    ///
    /// StrongAlias is a zero-cost abstraction, it's compiled away.
    ///
    /// TagType is an empty tag class (also called "phantom type") that only serves
    /// the type system to differentiate between different instantiations of the
    /// template.
    /// UnderlyingType may be almost any value type. Note that some methods of the
    /// StrongAlias may be unavailable (ie. produce elaborate compilation errors when
    /// used) if UnderlyingType doesn't support them.
    ///
    /// StrongAlias only directly exposes comparison operators (for convenient use in
    /// ordered containers). It's impossible, without reflection, to expose all
    /// methods of the UnderlyingType in StrongAlias's interface. It's also
    /// potentially unwanted (ex. you don't want to be able to add two StrongAliases
    /// that represent socket handles). A getter and dereference operators are
    /// provided in case you need to access the UnderlyingType.
    #[derive(Copy, Clone, Debug, Default)]
    pub struct StrongAlias<TagType, UnderlyingType> {
        value_: UnderlyingType,
        _phantom: PhantomData<TagType>,
    }

    impl<TagType, UnderlyingType> StrongAlias<TagType, UnderlyingType> {
        pub fn new(v: UnderlyingType) -> Self {
            StrongAlias {
                value_: v,
                _phantom: PhantomData,
            }
        }

        pub fn value(&self) -> &UnderlyingType {
            &self.value_
        }

        pub fn value_mut(&mut self) -> &mut UnderlyingType {
            &mut self.value_
        }
    }

    impl<TagType, UnderlyingType> Deref for StrongAlias<TagType, UnderlyingType> {
        type Target = UnderlyingType;

        fn deref(&self) -> &Self::Target {
            &self.value_
        }
    }

    impl<TagType, UnderlyingType> DerefMut for StrongAlias<TagType, UnderlyingType> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value_
        }
    }

    impl<TagType, UnderlyingType: PartialEq> PartialEq for StrongAlias<TagType, UnderlyingType> {
        fn eq(&self, other: &Self) -> bool {
            self.value_.eq(&other.value_)
        }
    }

    impl<TagType, UnderlyingType: Eq> Eq for StrongAlias<TagType, UnderlyingType> {}

    impl<TagType, UnderlyingType: PartialOrd> PartialOrd for StrongAlias<TagType, UnderlyingType> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.value_.partial_cmp(&other.value_)
        }
    }

    impl<TagType, UnderlyingType: Ord> Ord for StrongAlias<TagType, UnderlyingType> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.value_.cmp(&other.value_)
        }
    }

    impl<TagType, UnderlyingType: Hash> Hash for StrongAlias<TagType, UnderlyingType> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value_.hash(state);
        }
    }

    impl<TagType, UnderlyingType: fmt::Display> fmt::Display for StrongAlias<TagType, UnderlyingType> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.value_, f)
        }
    }

    /// Hasher to use in std::unordered_map, std::unordered_set, etc.
    ///
    /// Example usage:
    ///     using MyType = base::StrongAlias<...>;
    ///     using MySet = std::unordered_set<MyType, typename MyType::Hasher>;
    pub struct StrongAliasHasher<TagType, UnderlyingType> {
        _phantom: PhantomData<(TagType, UnderlyingType)>,
    }

    impl<TagType, UnderlyingType> StrongAliasHasher<TagType, UnderlyingType> {
        pub fn new() -> Self {
            StrongAliasHasher {
                _phantom: PhantomData,
            }
        }
    }

    impl<TagType, UnderlyingType: Hash> Hasher for StrongAliasHasher<TagType, UnderlyingType> {
        fn finish(&self) -> u64 {
            0 // Dummy value, the hash is calculated in write().
        }

        fn write(&mut self, bytes: &[u8]) {
            // Dummy implementation, the hash is calculated in the Hash implementation
            // of the UnderlyingType.
            // This is never called as we implement Hash for StrongAlias directly.
            panic!("This should not be called");
        }
    }
}

impl<TagType, UnderlyingType: Hash> std::hash::Hash for base::StrongAlias<TagType, UnderlyingType> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value().hash(state);
    }
}