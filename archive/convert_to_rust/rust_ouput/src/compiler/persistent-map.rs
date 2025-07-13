// Converted from V8 C++ source files:
// Header: persistent-map.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod hashing {
        // A simple hash function for demonstration purposes.
        pub fn hash<T>(_value: &T) -> usize {
            42 // Replace with a real hashing algorithm
        }
    }
}

pub mod zone {
    pub mod zone_containers {
        use std::collections::HashMap;

        pub struct ZoneMap<K, V> {
            map: HashMap<K, V>,
        }

        impl<K: Eq + std::hash::Hash + Clone, V: Clone> ZoneMap<K, V> {
            pub fn new() -> Self {
                ZoneMap { map: HashMap::new() }
            }

            pub fn find(&self, key: &K) -> Option<&V> {
                self.map.get(key)
            }

            pub fn emplace(&mut self, key: K, value: V) {
                self.map.insert(key, value);
            }

            pub fn erase(&mut self, key: &K) {
                self.map.remove(key);
            }

            pub fn begin(&self) -> ZoneMapIterator<K, V> {
                ZoneMapIterator {
                    inner: self.map.iter(),
                }
            }

            pub fn end(&self) -> ZoneMapIterator<K, V> {
                ZoneMapIterator {
                    inner: self.map.iter(),
                }
            }
        }

        pub struct ZoneMapIterator<'a, K, V> {
            inner: std::collections::hash_map::Iter<'a, K, V>,
        }

        impl<'a, K, V> ZoneMapIterator<'a, K, V> {
            pub fn new(inner: std::collections::hash_map::Iter<'a, K, V>) -> Self {
                ZoneMapIterator { inner }
            }
        }
        
        impl<'a, K, V> Iterator for ZoneMapIterator<'a, K, V>
        where K: Clone, V: Clone
        {
            type Item = (K, V);

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next().map(|(k, v)| (k.clone(), v.clone()))
            }
        }
    }
}

use std::array;
use std::cmp::Ordering;
use std::hash::Hasher;
use std::mem::size_of;
use std::ptr::null;
use std::sync::Mutex;
use std::{fmt, is_void};
use zone::zone_containers::ZoneMap;
use std::collections::hash_map::Iter;

pub struct Zone {
    // Simplified Zone implementation for demonstration purposes.
    // In a real implementation, this would manage memory allocation.
    memory: Mutex<Vec<u8>>,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            memory: Mutex::new(Vec::new()),
        }
    }

    pub fn allocate<T>(&self, size: usize) -> *mut T {
        let mut mem = self.memory.lock().unwrap();
        let start = mem.len();
        mem.resize(start + size, 0);
        mem[start..].as_mut_ptr() as *mut T
    }

    pub fn allocate_type<T>(&self) -> *mut T {
      let size = size_of::<T>();
      self.allocate::<T>(size)
    }
}

struct may_be_unequal<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: PartialEq> may_be_unequal<T> {
    fn new() -> Self {
        may_be_unequal {
            _phantom: std::marker::PhantomData,
        }
    }

    fn call(&self, a: &T, b: &T) -> bool {
        a != b
    }
}

pub struct PersistentMap<Key, Value, Hasher = base::hashing::hash<Key>> {
    tree_: *const FocusedTree<Key, Value, Hasher>,
    def_value_: Value,
    zone_: *mut Zone,
    _phantom: std::marker::PhantomData<(Key, Hasher)>,
}

impl<Key, Value, Hasher> fmt::Debug for PersistentMap<Key, Value, Hasher>
where
    Key: fmt::Debug,
    Value: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PersistentMap")
            .field("def_value_", &self.def_value_)
            .field("last_depth()", &self.last_depth())
            .finish()
    }
}

#[derive(Debug)]
struct FocusedTree<Key, Value, Hasher> {
    key_value: KeyValue<Key, Value>,
    length: i8,
    key_hash: HashValue<Hasher>,
    more: *mut ZoneMap<Key, Value>,
    path_array: [u8; 0], // Flexible array member simulation
    _phantom: std::marker::PhantomData<Hasher>,
}

impl<Key, Value, Hasher> FocusedTree<Key, Value, Hasher> {
    // Helper function to access the path array as a slice of pointers
    #[allow(dead_code)]
    fn path(&self, i: usize) -> *const FocusedTree<Key, Value, Hasher> {
        assert!(i < self.length as usize);
        unsafe {
            let base_ptr = self as *const Self as *const u8;
            let path_array_offset =
                (std::mem::size_of::<KeyValue<Key, Value>>()
                    + std::mem::size_of::<i8>()
                    + std::mem::size_of::<HashValue<Hasher>>()
                    + std::mem::size_of::<*mut ZoneMap<Key, Value>>()) as isize;
            let ptr_size = std::mem::size_of::<*const FocusedTree<Key, Value, Hasher>>() as isize;
            let path_ptr = base_ptr.offset(path_array_offset + (i as isize) * ptr_size)
                as *const *const FocusedTree<Key, Value, Hasher>;
            *path_ptr
        }
    }

    #[allow(dead_code)]
    fn path_mut(&mut self, i: usize) -> *mut FocusedTree<Key, Value, Hasher> {
        assert!(i < self.length as usize);
        unsafe {
            let base_ptr = self as *mut Self as *mut u8;
            let path_array_offset =
                (std::mem::size_of::<KeyValue<Key, Value>>()
                    + std::mem::size_of::<i8>()
                    + std::mem::size_of::<HashValue<Hasher>>()
                    + std::mem::size_of::<*mut ZoneMap<Key, Value>>()) as isize;
            let ptr_size = std::mem::size_of::<*mut FocusedTree<Key, Value, Hasher>>() as isize;
            let path_ptr = base_ptr.offset(path_array_offset + (i as isize) * ptr_size)
                as *mut *mut FocusedTree<Key, Value, Hasher>;
            *path_ptr
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct KeyValue<Key, Value> {
    first: Key,
    second: Value,
}

impl<Key, Value> KeyValue<Key, Value> {
    fn new(first: Key, second: Value) -> Self {
        KeyValue { first, second }
    }

    fn key(&self) -> &Key {
        &self.first
    }

    fn value(&self) -> &Value {
        &self.second
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct HashValue<Hasher> {
    bits_: u32,
    _phantom: std::marker::PhantomData<Hasher>,
}

impl<Hasher> HashValue<Hasher> {
    fn new(hash: usize) -> Self {
        HashValue {
            bits_: hash as u32,
            _phantom: std::marker::PhantomData,
        }
    }

    fn get_bit(&self, pos: usize) -> Bit {
        assert!(pos < 32);
        if self.bits_ & (1 << (32 - pos - 1)) != 0 {
            Bit::kRight
        } else {
            Bit::kLeft
        }
    }

    fn operator_xor(&self, other: HashValue<Hasher>) -> HashValue<Hasher> {
        HashValue {
            bits_: self.bits_ ^ other.bits_,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Bit {
    kLeft = 0,
    kRight = 1,
}

impl<Key, Value, H: Fn(&Key) -> usize> PersistentMap<Key, Value, H> {
    const kHashBits: usize = 32;

    pub fn last_depth(&self) -> usize {
        if self.tree_.is_null() {
            0
        } else {
            unsafe { (*self.tree_).length as usize }
        }
    }

    pub fn get(&self, key: &Key) -> &Value {
        let key_hash = HashValue::<H>::new((self.hasher())(key));
        let tree = self.find_hash(key_hash);
        self.get_focused_value(tree, key)
    }

    pub fn set(&mut self, key: Key, value: Value)
    where
        Key: Clone + Eq + std::hash::Hash,
        Value: Clone,
    {
        self.modify(key, |v| *v = value);
    }

    pub fn modify<F>(&mut self, key: Key, f: F)
    where
        F: FnOnce(&mut Value),
        Key: Clone + Eq + std::hash::Hash,
        Value: Clone,
    {
        let key_hash = HashValue::<H>::new((self.hasher())(&key));
        let mut path: [Option<*const FocusedTree<Key, Value, H>>; Self::kHashBits] =
            [None; Self::kHashBits];
        let mut length = 0;
        let old = self.find_hash_with_path(key_hash, &mut path, &mut length);
        let mut more: *mut ZoneMap<Key, Value> = null_mut();
        let old_value = self.get_focused_value(old, &key);
        let mut new_value = old_value.clone();
        f(&mut new_value);

        if !may_be_unequal::<Value>::new().call(old_value, &new_value) {
            return;
        }

        if !old.is_null() {
            unsafe {
                if (*old).more == null_mut() && (*old).key_value.first == key {
                  // pass
                } else {
                  more = (*self.zone_).allocate_type::<ZoneMap<Key, Value>>();
                  more.write(ZoneMap::new());

                  if !(*old).more.is_null() {
                      *more = (*(*old).more).clone();
                  } else {
                      (*more).erase(&(*old).key_value.first);
                      (*more).emplace((*old).key_value.first.clone(), (*old).key_value.second.clone());
                  }
                  (*more).erase(&key);
                  (*more).emplace(key.clone(), new_value.clone());
                }
            }
        }

        let size = size_of::<FocusedTree<Key, Value, H>>()
            + std::cmp::max(0, length as usize - 1) * size_of::<*const FocusedTree<Key, Value, H>>();

        unsafe {
            let tree = (*self.zone_).allocate::<FocusedTree<Key, Value, H>>(size) as *mut FocusedTree<Key, Value, H>;
            tree.write(FocusedTree {
                key_value: KeyValue::new(key.clone(), new_value.clone()),
                length: length as i8,
                key_hash,
                more,
                path_array: [0u8; 0],
                _phantom: std::marker::PhantomData,
            });

            for i in 0..length as usize {
                if let Some(path_node) = path[i] {
                   let path_ptr = (*tree).path_mut(i);
                   path_ptr.write(path_node as *mut FocusedTree<Key, Value, H>);
                }
            }

            *self = PersistentMap::new(tree as *const FocusedTree<Key, Value, H>, self.zone_, self.def_value_.clone());
        }
    }

    fn operator_eq(&self, other: &Self) -> bool
    where
        Key: PartialEq,
        Value: PartialEq,
    {
        if self.tree_ == other.tree_ {
            return true;
        }
        if self.def_value_ != other.def_value_ {
            return false;
        }
        for triple in self.zip(other) {
            if std::get::<1>(triple) != std::get::<2>(triple) {
                return false;
            }
        }
        true
    }

    fn operator_ne(&self, other: &Self) -> bool
    where
        Key: PartialEq,
        Value: PartialEq,
    {
        !self.operator_eq(other)
    }

    fn new(tree: *const FocusedTree<Key, Value, H>, zone: *mut Zone, def_value: Value) -> Self {
        PersistentMap {
            tree_: tree,
            def_value_: def_value,
            zone_: zone,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_hasher(zone: *mut Zone, def_value: Value) -> Self {
        PersistentMap {
            tree_: null(),
            def_value_: def_value,
            zone_: zone,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn new_with_default(zone: *mut Zone) -> Self
    where
        Value: Default,
    {
        Self::with_hasher(zone, Value::default())
    }

    fn find_hash(&self, hash: HashValue<H>) -> *const FocusedTree<Key, Value, H>
    where
        Key: PartialEq,
        Value: PartialEq,
    {
        let mut tree = self.tree_;
        let mut level = 0;

        while !tree.is_null() {
            if hash == unsafe { (*tree).key_hash } {
                break;
            }

            let mut tmp_level = level;
            while (hash.operator_xor(unsafe { (*tree).key_hash }).get_bit(tmp_level) == Bit::kLeft) {
                tmp_level += 1;
            }
            level = tmp_level;

            if level < unsafe { (*tree).length } as usize {
                tree = unsafe { (*tree).path((*tree).length as usize - 1) };
            } else {
                tree = null();
            }

            level += 1;
        }
        tree
    }

    fn find_hash_with_path(
        &self,
        hash: HashValue<H>,
        path: &mut [Option<*const FocusedTree<Key, Value, H>>; Self::kHashBits],
        length: &mut usize,
    ) -> *const FocusedTree<Key, Value, H>
    where
        Key: PartialEq,
        Value: PartialEq,
    {
        let mut tree = self.tree_;
        let mut level = 0;

        while !tree.is_null() {
            if hash == unsafe { (*tree).key_hash } {
                break;
            }

            let mut tmp_level = level;
            while (hash.operator_xor(unsafe { (*tree).key_hash }).get_bit(tmp_level) == Bit::kLeft) {
                path[tmp_level] = if tmp_level < unsafe { (*tree).length } as usize {
                   Some(unsafe { (*tree).path(tmp_level) })
                } else {
                  None
                };
                tmp_level += 1;
            }
            level = tmp_level;

            path[level] = Some(tree);
            if level < unsafe { (*tree).length } as usize {
              tree = unsafe { (*tree).path((*tree).length as usize - 1) };
            } else {
              tree = null();
            }
            level += 1;
        }

        if !tree.is_null() {
            while level < unsafe { (*tree).length } as usize {
                path[level] = if level < unsafe { (*tree).length } as usize {
                    Some(unsafe { (*tree).path(level) })
                } else {
                   None
                };
                level += 1;
            }
        }

        *length = level;
        tree
    }

    fn get_focused_value(&self, tree: *const FocusedTree<Key, Value, H>, key: &Key) -> &Value
    where
        Key: PartialEq,
        Value: Clone,
    {
        unsafe {
            if tree.is_null() {
                return &self.def_value_;
            }

            if !(*tree).more.is_null() {
                let it = (*(*tree).more).find(key);
                if it.is_none() {
                    return &self.def_value_;
                } else {
                    return it.unwrap();
                }
            } else {
                if *key == (*tree).key_value.first {
                    return &(*tree).key_value.second;
                } else {
                    return &self.def_value_;
                }
            }
        }
    }

    fn get_child(tree: *const FocusedTree<Key, Value, H>, level: usize, bit: Bit) -> *const FocusedTree<Key, Value, H> {
        unsafe {
            if (*tree).key_hash.get_bit(level) == bit {
                return tree;
            } else if level < (*tree).length as usize {
                return (*tree).path(level);
            } else {
                return null();
            }
        }
    }

    fn find_leftmost(
        start: *const FocusedTree<Key, Value, H>,
        level: &mut usize,
        path: &mut [Option<*const FocusedTree<Key, Value, H>>; Self::kHashBits],
    ) -> *const FocusedTree<Key, Value, H> {
        let mut current = start;
        unsafe {
            while *level < (*current).length as usize {
                let left_child = Self::get_child(current, *level, Bit::kLeft);
                if !left_child.is_null() {
                    path[*level] = Self::get_child(current, *level, Bit::kRight).into();
                    current = left_child;
                    *level += 1;
                } else {
                    let right_child = Self::get_child(current, *level, Bit::kRight);
                    if !right_child.is_null() {
                        path[*level] = Self::get_child(current, *level, Bit::kLeft).into();
                        current = right_child;
                        *level += 1;
                    } else {
                        panic!("UNREACHABLE");
                    }
                }
            }
        }
        current
    }

    fn hasher(&self) -> H {
        // This is a placeholder.  Ideally, the hasher would be stored directly
        // in the struct.  Since the C++ code doesn't store it, we have to
        // create a fresh one on each call.  This assumes that the hasher type
        // is Copy, which is not ideal.
        //H::default()
        |_key: &Key| -> usize { 42 } // Replace with a real hashing algorithm
    }
}

impl<Key, Value, H: Fn(&Key) -> usize> PersistentMap<Key, Value, H>
where
    Key: Clone + Eq + std::hash::Hash + fmt::Debug,
    Value: Clone + fmt::Debug + PartialEq,
{
    pub fn begin(&self) -> iterator<Key, Value, H> {
        if self.tree_.is_null() {
            self.end()
        } else {
            iterator::begin(self.tree_, self.def_value_.clone())
        }
    }

    pub fn end(&self) -> iterator<Key, Value, H> {
        iterator::end(self.def_value_.clone())
    }

    pub fn zip(&self, other: &Self) -> ZipIterable<Key, Value, H> {
        ZipIterable {
            a: PersistentMap {
              tree_: self.tree_,
              def_value_: self.def_value_.clone(),
              zone_: self.zone_,
              _phantom: std::marker::PhantomData,
            },
            b: PersistentMap {
              tree_: other.tree_,
              def_value_: other.def_value_.clone(),
              zone_: other.zone_,
              _phantom: std::marker::PhantomData,
            },
        }
    }
}

use std::ptr::null_mut;

pub struct iterator<Key, Value, Hasher> {
    level_: usize,
    more_iter_: Option<zone::zone_containers::ZoneMapIterator<'static, Key, Value>>,
    current_: *const FocusedTree<Key, Value, Hasher>,
    path_: [Option<*const FocusedTree<Key, Value, Hasher>>; PersistentMap::<Key, Value, Hasher>::kHashBits],
    def_value_: Value,
    _phantom: std::marker::PhantomData<Hasher>,
}

impl<Key, Value, Hasher> Iterator for iterator<Key, Value, Hasher>
where
    Key: Clone + Eq + std::hash::Hash + fmt::Debug,
    Value: Clone + fmt::Debug + PartialEq,
{
    type Item = (Key, Value);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_.is_null() {
                return None;
            }

            unsafe {
                if let Some(ref mut more) = self.more_iter_ {
                    if let Some((key, value)) = more.next() {
                        return Some((key.clone(), value.clone()));
                    }
                }

                if self.level_ == 0 {
                    *self = Self::end(self.def_value_.clone());
                    return None;
                }

                self.level_ -= 1;
                while HashValue::<Hasher>::new(0).get_bit(self.level_) == Bit::kRight
                    || self.path_[self.level_].is_none()
                {
                    if self.level_ == 0 {
                        *self = Self::end(self.def_value_.clone());
                        return None;
                    }
                    self.level_ -= 1;
                }

                let first_right_alternative = self.path_[self.level_].unwrap();
                self.level_ += 1;
                self.current_ = PersistentMap::<Key, Value, Hasher>::find_leftmost(
                    first_right_alternative,
                    &mut self.level_,
                    &mut self.path_,
                );

                if !(*self.current_).more.is_null() {
                    self.more_iter_ = Some(((*(*self.current_).more)).begin());
                }
            }

            if self.is_end() {
                return None;
            }

            if self.second() != self.def_value_ {
                return Some((self.first(), self.second()));
            }
        }
    }
}

impl<Key, Value, Hasher> iterator<Key, Value, Hasher>
where
    Key: Clone + Eq + std::hash::Hash + fmt::Debug,
    Value: Clone + fmt::Debug + PartialEq,
{
    fn first(&self) -> Key {
        unsafe {
            if !self.current_.is_null() {
                (*self.current_).key_value.first.clone()
            } else {
                panic!("Iterator is end.");
            }
        }
    }

    fn second(&self) -> Value {
        unsafe {
            if !self.current_.is_null() {
                if let Some(ref mut more) = self.more_iter_ {
                    if let Some((_, value)) = more.next() {
                        return value.clone();
                    }
                }
                (*self.current_).key_value.second.clone()
            } else {
                self.def_value_.clone()
            }
        }
    }

    fn begin(tree: *const FocusedTree<Key, Value, Hasher>, def_value: Value) -> Self {
        let mut i = iterator {
            level_: 0,
            more_iter_: None,
            current_: null(),
            path_: [None; PersistentMap::<Key, Value, Hasher>::kHashBits],
            def_value_: def_value.clone(),
            _phantom: std::marker::PhantomData,
        };

        i.current_ = PersistentMap::<Key, Value, Hasher>::find_leftmost(
            tree,
            &mut i.level_,
            &mut i.path_,
        );

        unsafe {
            if !(*i.current_).more.is_null() {
                i.more_iter_ = Some(((*(*i.current_).more)).begin());
            }
        }

        // Skip entries with default value. PersistentMap iterators must never point
        // to a default value.
        let mut temp_iter = i;
        while !temp_iter.is_end() && temp_iter.second() == def_value {
            temp_iter.next();
        }

        i
    }

    fn end(def_value: Value) -> Self {
        iterator {
            level_: 0,
            more_iter_: None,
            current_: null(),
            path_: [None; PersistentMap::<Key, Value, Hasher>::kHashBits],
            def_value_: def_value,
            _phantom: std::marker::PhantomData,
        }
    }

    fn is_end(&self) -> bool {
        self.current_.is_null()
    }
}

impl<Key: PartialEq, Value: PartialEq, Hasher> PartialEq for iterator<Key, Value, Hasher> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_end() {
            return other.is_end();
        }
        if other.is_end() {
            return false;
        }
        unsafe {
            if (*self.current_).key_hash != (*other.current_).key_hash {
                return false;
            } else {
                return self.first() == other.first();
            }
        }
    }
}

impl<Key: PartialEq, Value: PartialEq, Hasher> PartialOrd for iterator<Key, Value, Hasher>
where
    Key: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_end() {
            return Some(Ordering::Less);
        }
        if other.is_end() {
            return Some(Ordering::Greater);
        }
        unsafe {
            if (*self.current_).key_hash == (*other.current_).key_hash {
                return self.first().partial_cmp(&other.first());
            } else {
                return (*self.current_).key_hash.partial_cmp(&(*other.current_).key_hash);
            }
        }
    }
}

impl<Key: PartialEq, Value: PartialEq, Hasher> Eq for iterator<Key, Value, Hasher> {}

impl<Key: PartialEq, Value: PartialEq, Hasher> Ord for iterator<Key, Value, Hasher>
where
    Key: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_end() {
            return Ordering::Less;
        }
        if other.is_end() {
            return Ordering::Greater;
        }
        unsafe {
            if (*self.current_).key_hash == (*other.current_).key_hash {
                return self.first().cmp(&other.first());
            } else {
                return (*self.current_).key_hash.cmp(&(*other.current_).key_hash);
            }
        }
    }
}

#[derive(Debug)]
pub struct ZipIterable<Key, Value, Hasher> {
    a: PersistentMap<Key, Value, Hasher>,
    b: PersistentMap<Key, Value, Hasher>,
}

impl<Key, Value, Hasher> ZipIterable<Key, Value, Hasher>
where
    Key: Clone + Eq + std::hash::Hash + fmt::Debug,
    Value: Clone + fmt::Debug + PartialEq,
{
    pub fn begin(&self) -> double_iterator<Key, Value, Hasher> {
        double_iterator::new(self.a.begin(), self.b.begin())
    }

    pub fn end(&self) -> double_iterator<Key, Value, Hasher> {
        double_iterator::new(self.a.end(), self.b.end())
    }
}

#[derive(Debug)]
pub struct double_iterator<Key, Value, Hasher> {
    first_: iterator<Key, Value, Hasher>,
    second_: iterator<Key, Value, Hasher>,
    first_current_: bool,
    second_current_: bool,
    _phantom: std::marker::PhantomData<Hasher>,
}

impl<Key, Value, Hasher> double_iterator<Key, Value, Hasher>
where
    Key: Clone + Eq + std::hash::Hash + fmt::Debug,
    Value: Clone + fmt::Debug + PartialEq,
{
    fn new(first: iterator<Key, Value, Hasher>, second: iterator<Key, Value, Hasher>) -> Self {
        let mut first_current_ = false;
        let mut second_current_ = false;

        if first == second {
            first_current_ = true;
            second_current_ = true;
        } else if first < second {
            first_current_ = true;
            second_current_ = false;
        } else {
            first_current_ = false;
            second_current_ = true;
        }

        double_iterator {
            first_: first,
            second_: second,
            first_current_: first_current_,
            second_current_: second_current_,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn is_end(&self) -> bool {
        self.first_.is_end() && self.second_.is_end()
    }
}

impl<Key, Value, Hasher> PartialEq for double_iterator<Key, Value, Hasher>
where
    Key: Clone + Eq + std::hash::Hash + fmt::Debug,
    Value: Clone + fmt::Debug + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.first_ == other.first_ && self.second_ == other.second_
    }
}

impl<Key, Value, Hasher> Iterator for double_iterator<Key, Value, Hasher>
where
    Key: Clone + Eq + std::hash::Hash + fmt::Debug,
    Value: Clone + fmt::Debug + PartialEq,
{
    type Item = (Key, Value, Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }

        let old_first = self.first_.clone();
        let old_second = self.second_.clone();

        let result = if self.first_current_ {
            let first_pair = (self.first_.first(), self.first_.second());
            let second_value = if self.second_current_ {
                self.second_.second()
            } else {
                self.second_.def_value_.clone()
            };
            Some((first_pair.0, first_pair.1, second_value))
        } else {
            let second_pair = (self.second_.first(), self.second_.second());
            let first_value = self.first_.def_value_.clone();
            Some((second_pair.0, first_value, second_pair.1))
        };

        if self.first_current_ {
            self.first_.next();
            assert!(old_first < self.first_);
        }
        if self.second_current_ {
            self.second_.next();
            assert!(old_second < self.second_);
        }

        let mut new_iter = double_iterator::new(self.first_.clone(), self.second_.clone());
        self.first_current_ = new_iter.first_current_;
        self.second_current_ = new_iter.second_current_;
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistent_map() {
        let zone = Zone::new();
        let mut map: PersistentMap<i32, String, _> =
            PersistentMap::new_with_default(&(*zone as *mut Zone));
        assert_eq!(map.last_depth(), 0);

        map.set(1, "one".to_string());
        assert_eq!(map.get(&1), "one");
        assert_eq!(map.last_depth(), 1);

        map.set(2, "two".to_string());
        assert_eq!(map.get(&2), "two");
        assert_eq!(map.last_depth(), 2);

        assert_eq!(map.get(&3), ""); // Default value
    }

    #[test]
    fn test_iterator() {
        let zone = Zone::new();
        let mut map: PersistentMap<i32, String, _> =
            PersistentMap
