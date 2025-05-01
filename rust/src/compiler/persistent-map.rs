// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::array;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::{BitAnd, BitXor, Deref, DerefMut};
use std::rc::Rc;
use std::{fmt, mem, tuple};

// Placeholder for Zone and ZoneMap. In V8, these are custom memory management
// solutions. In Rust, we'll use standard memory management for simplicity.
// In a real port, these would need careful consideration.

// A fast and possibly incomplete equality check. If it returns false, the
// values are certainly not equal, otherwise we do not know. The template is
// intended to be specialized for types with expensive equality checks.
struct MayBeUnequal<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: PartialEq> MayBeUnequal<T> {
    fn new() -> Self {
        MayBeUnequal {
            _phantom: std::marker::PhantomData,
        }
    }
    fn may_be_unequal(&self, a: &T, b: &T) -> bool {
        a != b
    }
}

// PersistentMap is a persistent map datastructure based on hash trees (a binary
// tree using the bits of a hash value as addresses). The map is a conceptually
// infinite: All keys are initially mapped to a default value, values are
// deleted by overwriting them with the default value. The iterators produce
// exactly the keys that are not the default value. The hash values should have
// high variance in their high bits, so dense integers are a bad choice.
// Complexity:
// - Copy and assignment: O(1)
// - access: O(log n)
// - update: O(log n) time and space
// - iteration: amortized O(1) per step
// - Zip: O(n)
// - equality check: O(n)
// TODO(turbofan): Cache map transitions to avoid re-allocation of the same map.
// TODO(turbofan): Implement an O(1) equality check based on hash consing or
//              something similar.
pub struct PersistentMap<K, V, H = DefaultHasher> {
    tree: Option<Rc<FocusedTree<K, V, H>>>,
    def_value: V,
    //zone: Zone, // Using standard memory management instead
    _phantom: std::marker::PhantomData<H>,
}

impl<K, V, H> Clone for PersistentMap<K, V, H>
where
    K: Clone,
    V: Clone,
    H: Clone,
{
    fn clone(&self) -> Self {
        PersistentMap {
            tree: self.tree.clone(),
            def_value: self.def_value.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<K, V, H> PartialEq for PersistentMap<K, V, H>
where
    K: Eq + Hash + Clone,
    V: Eq + Clone,
    H: Hasher + Default + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        if self.tree == other.tree {
            return true;
        }
        if self.def_value != other.def_value {
            return false;
        }
        for triple in self.zip(other) {
            if tuple::get::<1>(&triple) != tuple::get::<2>(&triple) {
                return false;
            }
        }
        true
    }
}

impl<K, V, H> PersistentMap<K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone,
    H: Hasher + Default + Clone,
{
    pub type KeyType = K;
    pub type MappedType = V;
    pub type ValueType = (K, V);

    const K_HASH_BITS: usize = 32;

    /// Depth of the last added element. This is a cheap estimate for the size of
    /// the hash tree.
    pub fn last_depth(&self) -> usize {
        self.tree.as_ref().map_or(0, |tree| tree.length as usize)
    }

    pub fn get(&self, key: &K) -> &V {
        let key_hash = HashValue::new::<K, H>(key);
        let tree = self.find_hash(key_hash);
        self.get_focused_value(tree, key)
    }

    /// Add or overwrite an existing key-value pair.
    pub fn set(&mut self, key: K, value: V) {
        self.modify(key, |v| *v = value);
    }

    /// Modify an entry in-place, avoiding repeated search.
    /// `F` is a functional that expects a `Value*` parameter to modify it.
    pub fn modify<F>(&mut self, key: K, f: F)
    where
        F: FnOnce(&mut V),
    {
        let key_hash = HashValue::new::<K, H>(&key);
        let mut path: array::[Option<Rc<FocusedTree<K, V, H>>>;
                           PersistentMap::<K, V, H>::K_HASH_BITS] =
            [None; PersistentMap::<K, V, H>::K_HASH_BITS];
        let mut length = 0;
        let old = self.find_hash_with_path(key_hash, &mut path, &mut length);

        let mut more: Option<ZoneMap<K, V>> = None;
        let mut new_value = self.get_focused_value(old.as_ref(), &key).clone();
        f(&mut new_value);

        if !MayBeUnequal::<V>::new().may_be_unequal(&self.get_focused_value(old.as_ref(), &key), &new_value) {
            return;
        }
        if let Some(old_tree) = old.as_ref() {
            if old_tree.more.is_none() && old_tree.key_value.0 != key {
                more = Some(ZoneMap::new());

                if let Some(ref old_more) = old_tree.more {
                    // This case cannot happen because more = None
                } else {
                    more.as_mut().unwrap().insert(old_tree.key_value.0.clone(), old_tree.key_value.1.clone());
                }

                more.as_mut().unwrap().remove(&key);
                more.as_mut().unwrap().insert(key.clone(), new_value.clone());
            }
        }

        let mut path_vec = Vec::new();
        for item in path.iter().take(length) {
            path_vec.push(item.clone());
        }

        let tree = Rc::new(FocusedTree::new(
            (key.clone(), new_value.clone()),
            length as i8,
            key_hash,
            more,
            path_vec,
        ));

        self.tree = Some(tree);
    }

    pub fn zip(&self, other: &Self) -> ZipIterable<K, V, H> {
        ZipIterable {
            a: self.clone(),
            b: other.clone(),
        }
    }

    pub fn new(def_value: V) -> Self {
        PersistentMap {
            tree: None,
            def_value,
            _phantom: std::marker::PhantomData,
        }
    }

    // Find the {FocusedTree} that contains a key-value pair with key hash {hash}.
    fn find_hash(&self, hash: HashValue) -> Option<Rc<FocusedTree<K, V, H>>>> {
        let mut tree = self.tree.clone();
        let mut level = 0;
        while let Some(ref current_tree) = tree {
            if hash == current_tree.key_hash {
                return Some(current_tree.clone());
            }

            let mut current_level = level;
            while (hash ^ current_tree.key_hash).bitand(HashValue::mask(current_level)) == HashValue::mask(current_level) {
                current_level += 1;
            }
            
            if current_level < current_tree.length as usize {
                tree = current_tree.path.get(current_level).and_then(|x| x.clone());
            } else {
                return None;
            }
            level += 1;
        }
        None
    }

    fn find_hash_with_path(
        &self,
        hash: HashValue,
        path: &mut array::[Option<Rc<FocusedTree<K, V, H>>>;
                           PersistentMap::<K, V, H>::K_HASH_BITS],
        length: &mut usize,
    ) -> Option<Rc<FocusedTree<K, V, H>>> {
        let mut tree = self.tree.clone();
        let mut level = 0;
        while let Some(ref current_tree) = tree {
            if hash == current_tree.key_hash {
                while level < current_tree.length as usize {
                    path[level] = current_tree.path.get(level).and_then(|x| x.clone());
                    level += 1;
                }
                *length = level;
                return Some(current_tree.clone());
            }

            let mut current_level = level;
            while (hash ^ current_tree.key_hash).bitand(HashValue::mask(current_level)) == HashValue::mask(current_level) {
                path[current_level] = if current_level < current_tree.length as usize {
                    current_tree.path.get(current_level).and_then(|x| x.clone())
                } else {
                    None
                };
                current_level += 1;
            }

            path[level] = Some(current_tree.clone());
            tree = if level < current_tree.length as usize {
                current_tree.path.get(level).and_then(|x| x.clone())
            } else {
                None
            };
            level += 1;
        }
        *length = level;
        None
    }

    fn get_focused_value(&self, tree: Option<&Rc<FocusedTree<K, V, H>>>, key: &K) -> &V {
        match tree {
            None => &self.def_value,
            Some(node) => {
                if let Some(ref more) = node.more {
                    if let Some(value) = more.get(key) {
                        value
                    } else {
                        &self.def_value
                    }
                } else {
                    if &node.key_value.0 == key {
                        &node.key_value.1
                    } else {
                        &self.def_value
                    }
                }
            }
        }
    }

    fn get_child(tree: &Rc<FocusedTree<K, V, H>>, level: usize, bit: Bit) -> Option<Rc<FocusedTree<K, V, H>>> {
        if tree.key_hash[level] == bit {
            Some(tree.clone())
        } else if level < tree.length as usize {
            tree.path.get(level).and_then(|x| x.clone())
        } else {
            None
        }
    }

    fn find_leftmost(start: &Rc<FocusedTree<K, V, H>>, level: &mut usize, path: &mut array::[Option<Rc<FocusedTree<K, V, H>>>;
                           PersistentMap::<K, V, H>::K_HASH_BITS]) -> Rc<FocusedTree<K, V, H>> {
        let mut current = start.clone();
        while *level < current.length as usize {
            if let Some(left_child) = PersistentMap::<K, V, H>::get_child(&current, *level, Bit::Left) {
                path[*level] = PersistentMap::<K, V, H>::get_child(&current, *level, Bit::Right);
                current = left_child;
                *level += 1;
            } else if let Some(right_child) = PersistentMap::<K, V, H>::get_child(&current, *level, Bit::Right) {
                path[*level] = PersistentMap::<K, V, H>::get_child(&current, *level, Bit::Left);
                current = right_child;
                *level += 1;
            } else {
                panic!("Unreachable");
            }
        }
        current
    }

    pub fn begin(&self) -> Iterator<K, V, H> {
        match &self.tree {
            None => self.end(),
            Some(tree) => Iterator::begin(tree, self.def_value.clone())
        }
    }

    pub fn end(&self) -> Iterator<K, V, H> {
        Iterator::end(self.def_value.clone())
    }
}

impl<K, V, H> fmt::Display for PersistentMap<K, V, H>
where
    K: fmt::Display + Eq + Hash + Clone,
    V: fmt::Display + Clone,
    H: Hasher + Default + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for pair in self.begin() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{}: {}", pair.0, pair.1)?;
        }
        write!(f, "}}")
    }
}

#[derive(PartialEq)]
struct HashValue {
    bits: u32,
}

impl HashValue {
    fn new<K, H: Hasher + Default>(key: &K) -> Self
    where
        K: Hash,
    {
        let mut hasher = H::default();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        HashValue {
            bits: hash as u32,
        }
    }

    fn mask(pos: usize) -> u32 {
        1 << (PersistentMap::<(),(),DefaultHasher>::K_HASH_BITS - pos - 1)
    }
}

impl BitAnd for HashValue {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        HashValue { bits: self.bits & other.bits }
    }
}

impl BitXor for HashValue {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        HashValue { bits: self.bits ^ other.bits }
    }
}

impl Eq for HashValue {}

impl std::ops::Index<usize> for HashValue {
    type Output = Bit;

    fn index(&self, pos: usize) -> &Self::Output {
        if self.bits & (1 << (PersistentMap::<(),(),DefaultHasher>::K_HASH_BITS - pos - 1)) != 0 {
            &Bit::Right
        } else {
            &Bit::Left
        }
    }
}

impl PartialOrd for HashValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.bits.cmp(&other.bits))
    }
}

impl Ord for HashValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bits.cmp(&other.bits)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Bit {
    Left = 0,
    Right = 1,
}

type KeyValue<K, V> = (K, V);

struct FocusedTree<K, V, H> {
    key_value: KeyValue<K, V>,
    length: i8,
    key_hash: HashValue,
    more: Option<ZoneMap<K, V>>,
    path: Vec<Option<Rc<FocusedTree<K, V, H>>>>,
}

impl<K, V, H> FocusedTree<K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone,
    H: Hasher + Default + Clone,
{
    fn new(
        key_value: KeyValue<K, V>,
        length: i8,
        key_hash: HashValue,
        more: Option<ZoneMap<K, V>>,
        path: Vec<Option<Rc<FocusedTree<K, V, H>>>>,
    ) -> Self {
        FocusedTree {
            key_value,
            length,
            key_hash,
            more,
            path,
        }
    }
}

pub struct Iterator<'a, K, V, H> {
    level: usize,
    more_iter: Option<<ZoneMap<K, V> as IntoIterator>::IntoIter>,
    current: Option<Rc<FocusedTree<K, V, H>>>,
    path: array::[Option<Rc<FocusedTree<K, V, H>>>;
                PersistentMap::<K, V, H>::K_HASH_BITS],
    def_value: V,
    _phantom: std::marker::PhantomData<&'a H>
}

impl<'a, K, V, H> Iterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    fn begin(tree: &Rc<FocusedTree<K, V, H>>, def_value: V) -> Self {
        let mut i = Iterator {
            level: 0,
            more_iter: None,
            current: None,
            path: [None; PersistentMap::<K, V, H>::K_HASH_BITS],
            def_value,
            _phantom: std::marker::PhantomData
        };
        i.current = Some(PersistentMap::<K, V, H>::find_leftmost(tree, &mut i.level, &mut i.path));
        if let Some(ref current) = i.current {
            i.more_iter = current.more.clone().map(|more| more.into_iter());
        }

        // Skip entries with default value. PersistentMap iterators must never point
        // to a default value.
        let mut mutable_self = i;
        while !mutable_self.is_end() && mutable_self.deref().second() == mutable_self.def_value {
            mutable_self = Iterator::next(mutable_self).unwrap();
        }
        mutable_self
    }

    fn end(def_value: V) -> Self {
        Iterator {
            level: 0,
            more_iter: None,
            current: None,
            path: [None; PersistentMap::<K, V, H>::K_HASH_BITS],
            def_value,
            _phantom: std::marker::PhantomData
        }
    }

    fn is_end(&self) -> bool {
        self.current.is_none()
    }

    fn def_value(&self) -> &V {
        &self.def_value
    }
}

impl<'a, K, V, H> Deref for Iterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    type Target = KeyValue<K, V>;

    fn deref(&self) -> &Self::Target {
        if let Some(ref current) = self.current {
            if let Some(ref more_iter) = self.more_iter {
                if let Some(ref next) = more_iter.clone().next() {
                    return next;
                }
            }
            return &current.key_value;
        }
        panic!("Dereferencing end iterator");
    }
}

impl<'a, K, V, H> DerefMut for Iterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some(ref mut current) = self.current {
            if let Some(ref mut more_iter) = self.more_iter {
                if let Some(ref mut next) = more_iter.clone().next() {
                    return next;
                }
            }
            return &mut current.key_value;
        }
        panic!("Dereferencing end iterator");
    }
}

impl<'a, K, V, H> PartialEq for Iterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        if self.is_end() {
            return other.is_end();
        }
        if other.is_end() {
            return false;
        }
        if self.current.as_ref().unwrap().key_hash != other.current.as_ref().unwrap().key_hash {
            return false;
        }
        self.deref().0 == other.deref().0
    }
}

impl<'a, K, V, H> Iterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    fn next(mut self) -> Option<Self> {
        loop {
            if self.current.is_none() {
                // Iterator is past the end.
                return None;
            }

            if let Some(ref mut more_iter) = self.more_iter {
                if let Some(_next) = more_iter.clone().next() {
                    //self.more_iter = Some(more_iter.clone().skip(1));
                    return Some(self);
                }
            }

            if self.level == 0 {
                return Some(Iterator::end(self.def_value.clone()));
            }

            self.level -= 1;
            while self.current.as_ref().unwrap().key_hash[self.level] == Bit::Right
                || self.path[self.level].is_none()
            {
                if self.level == 0 {
                    return Some(Iterator::end(self.def_value.clone()));
                }
                self.level -= 1;
            }

            let first_right_alternative = self.path[self.level].clone().unwrap();
            self.level += 1;
            self.current = Some(PersistentMap::<K, V, H>::find_leftmost(&first_right_alternative, &mut self.level, &mut self.path));
            if let Some(ref current) = self.current {
                self.more_iter = current.more.clone().map(|more| more.into_iter());
            }
            if self.deref().second() != self.def_value {
                return Some(self);
            }
        }
    }
}

impl<'a, K, V, H> std::iter::Iterator for Iterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    type Item = KeyValue<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_iter = Iterator::next(Iterator {
            level: self.level,
            more_iter: self.more_iter.clone(),
            current: self.current.clone(),
            path: self.path.clone(),
            def_value: self.def_value.clone(),
            _phantom: std::marker::PhantomData
        });

        match next_iter {
            None => None,
            Some(new_iter) => {
                self.level = new_iter.level;
                self.more_iter = new_iter.more_iter;
                self.current = new_iter.current;
                self.path = new_iter.path;
                Some((self.deref().0.clone(), self.deref().1.clone()))
            }
        }
    }
}

struct DoubleIterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    first: Iterator<'a, K, V, H>,
    second: Iterator<'a, K, V, H>,
    first_current: bool,
    second_current: bool,
}

impl<'a, K, V, H> DoubleIterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    fn new(first: Iterator<'a, K, V, H>, second: Iterator<'a, K, V, H>) -> Self {
        let first_current;
        let second_current;
        if first == second {
            first_current = true;
            second_current = true;
        } else if first < second {
            first_current = true;
            second_current = false;
        } else {
            first_current = false;
            second_current = true;
        }

        DoubleIterator {
            first,
            second,
            first_current,
            second_current,
        }
    }

    fn is_end(&self) -> bool {
        self.first.is_end() && self.second.is_end()
    }
}

impl<'a, K, V, H> std::iter::Iterator for DoubleIterator<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    type Item = (K, V, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }

        let result;
        if self.first_current {
            let pair = self.first.deref();
            result = Some((
                pair.0.clone(),
                pair.1.clone(),
                if self.second_current {
                    self.second.deref().1.clone()
                } else {
                    self.second.def_value().clone()
                },
            ));
            if let Some(new_iter) = Iterator::next(Iterator {
                level: self.first.level,
                more_iter: self.first.more_iter.clone(),
                current: self.first.current.clone(),
                path: self.first.path.clone(),
                def_value: self.first.def_value.clone(),
                _phantom: std::marker::PhantomData
            })
             {
                self.first = new_iter;
            } else {
                self.first = Iterator::end(self.first.def_value.clone());
            }
        } else {
            let pair = self.second.deref();
            result = Some((
                pair.0.clone(),
                self.first.def_value().clone(),
                pair.1.clone(),
            ));
            if let Some(new_iter) = Iterator::next(Iterator {
                level: self.second.level,
                more_iter: self.second.more_iter.clone(),
                current: self.second.current.clone(),
                path: self.second.path.clone(),
                def_value: self.second.def_value.clone(),
                _phantom: std::marker::PhantomData
            }) {
                self.second = new_iter;
            } else {
                self.second = Iterator::end(self.second.def_value.clone());
            }
        }

        //Advance iterators according to relative positions
        if self.first == self.second {
            self.first_current = true;
            self.second_current = true;
        } else if self.first < self.second {
            self.first_current = true;
            self.second_current = false;
        } else {
            self.first_current = false;
            self.second_current = true;
        }

        result
    }
}

struct ZipIterable<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    a: PersistentMap<K, V, H>,
    b: PersistentMap<K, V, H>,
}

impl<'a, K, V, H> ZipIterable<'a, K, V, H>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq,
    H: Hasher + Default + Clone,
{
    fn begin(&self) -> DoubleIterator<K, V, H> {
        DoubleIterator::new(self.a.begin(), self.b.begin())
    }

    fn end(&self) -> DoubleIterator<K, V, H> {
        DoubleIterator::new(self.a.end(), self.b.end())
    }
}

// Placeholder for ZoneMap. In V8, this is a custom memory management
// solution. In Rust, we'll use standard memory management for simplicity.
// In a real port, this would need careful consideration.
use std::collections::BTreeMap;

type ZoneMap<K, V> = BTreeMap<K, V>;