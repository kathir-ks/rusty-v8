// Converted from V8 C++ source files:
// Header: fast-hash.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]

use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;

pub mod base {
    use std::hash::{Hash, Hasher};

    pub fn hash<T: Hash>() -> impl Fn(&T) -> usize {
        |t: &T| {
            let mut s = std::collections::hash_map::DefaultHasher::new();
            t.hash(&mut s);
            s.finish() as usize
        }
    }

    #[derive(Debug, Clone)]
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new() -> Self {
            Vector { data: Vec::new() }
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn begin(&self) -> std::slice::Iter<T> {
            self.data.iter()
        }

        pub fn end(&self) -> std::slice::Iter<T> {
            self.data.iter()
        }

        pub fn iter(&self) -> std::slice::Iter<T> {
            self.data.iter()
        }

        pub fn clear(&mut self) {
            self.data.clear();
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
        pub fn extend_from_slice(&mut self, other: &[T])
        where
            T: Copy,
        {
            self.data.extend_from_slice(other);
        }

        pub fn resize(&mut self, new_len: usize, value: T)
        where
            T: Clone,
        {
            self.data.resize(new_len, value);
        }
    }
    impl<T> Deref for Vector<T> {
        type Target = Vec<T>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
    impl<T> From<Vec<T>> for Vector<T> {
        fn from(data: Vec<T>) -> Self {
            Vector { data }
        }
    }
}

pub mod internal {
    pub mod compiler {
        pub mod turboshaft {
            pub fn fast_hash_combine() -> usize {
                0
            }

            pub fn fast_hash_combine1(acc: usize) -> usize {
                acc
            }

            pub fn fast_hash_combine2(acc: usize, value: usize) -> usize {
                17 * acc + value
            }

            pub fn fast_hash_combine<T>(v: &T) -> usize
            where
                T: std::hash::Hash,
            {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                v.hash(&mut hasher);
                hasher.finish() as usize
            }

            pub fn fast_hash_range<Iterator, T>(mut first: Iterator, last: Iterator) -> usize
            where
                Iterator: Iterator<Item = T>,
                T: std::hash::Hash,
            {
                let mut acc = 0;
                for item in first {
                   acc = fast_hash_combine2(acc, fast_hash_combine(&item));
                }
                acc
            }
            
            pub struct FastHash<T>(std::marker::PhantomData<T>);

            impl<T> FastHash<T> {
                pub fn new() -> Self {
                    FastHash(std::marker::PhantomData)
                }
            }
            impl<T> Default for FastHash<T> {
                fn default() -> Self {
                    Self::new()
                }
            }
            impl<T> FastHash<T> {
                pub fn call(&self, v: &T) -> usize
                where
                    T: std::hash::Hash,
                {
                   fast_hash_combine(v)
                }
            }
        }
    }
}

use internal::compiler::turboshaft::*;

pub fn fast_hash_combine<T, Ts>(v: &T, vs: Ts) -> usize
where
    T: Hash,
    Ts: Hash
{
    fast_hash_combine2(fast_hash_combine(&vs), fast_hash_combine(v))
}

pub struct fast_hash<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> fast_hash<T> {
    pub fn new() -> Self {
        fast_hash {
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<T> Default for fast_hash<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> fast_hash<T> {
    pub fn call(&self, v: &T) -> usize
    where
        T: std::hash::Hash,
    {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish() as usize
    }
}

impl<T1, T2> fast_hash<std::pair::Pair<T1, T2>> {
    pub fn call(&self, v: &std::pair::Pair<T1, T2>) -> usize
    where
        T1: Hash,
        T2: Hash,
    {
        fast_hash_combine(&v.first, &v.second)
    }
}

impl<Ts> fast_hash<std::tuple::Tuple<Ts>> {
    pub fn call(&self, v: &std::tuple::Tuple<Ts>) -> usize
    where
        Ts: Hash
    {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish() as usize
    }
}
impl<T> fast_hash<base::Vector<T>> {
    pub fn call(&self, v: &base::Vector<T>) -> usize
    where
        T: Hash,
    {
        fast_hash_range(v.begin(), v.end())
    }
}

pub mod std {
    pub mod pair {
        #[derive(Debug, Clone)]
        pub struct Pair<T1, T2> {
            pub first: T1,
            pub second: T2,
        }
    }
    pub mod tuple {
        use std::hash::{Hash, Hasher};

        #[derive(Debug, Clone)]
        pub struct Tuple<Ts> {
            _phantom: std::marker::PhantomData<Ts>,
            data: Vec<usize>,
        }

        impl<Ts> Tuple<Ts> {
            pub fn new() -> Self {
                Tuple {
                    _phantom: std::marker::PhantomData,
                    data: Vec::new(),
                }
            }
        }

        impl<Ts> Hash for Tuple<Ts> {
            fn hash<H: Hasher>(&self, state: &mut H) {
               for val in &self.data {
                 val.hash(state);
               }
            }
        }
    }
}
