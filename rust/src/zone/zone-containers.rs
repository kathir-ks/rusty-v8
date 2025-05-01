pub mod zone_containers {
    use std::alloc::{Allocator, Layout};
    use std::cmp::Ordering;
    use std::collections::{LinkedList, VecDeque};
    use std::fmt;
    use std::iter::FromIterator;
    use std::marker::PhantomData;
    use std::mem::{self, MaybeUninit};
    use std::ops::{Deref, DerefMut, Index, IndexMut};
    use std::ptr;
    use std::vec::Vec;

    // Placeholder for Zone allocator.  Need to define a proper Zone in Rust.
    pub struct Zone {
        // Implementation detail
    }

    impl Zone {
        pub fn allocate_array<T>(&self, size: usize) -> *mut T {
            let layout = Layout::array::<T>(size).unwrap();
            unsafe {
                let ptr = std::alloc::alloc(layout) as *mut T;
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }
                ptr
            }
        }

        pub fn delete_array<T>(&self, ptr: *mut T, _capacity: usize) {
            unsafe {
                let layout = Layout::array::<T>(_capacity).unwrap();
                std::alloc::dealloc(ptr as *mut u8, layout);
            }
        }
    }

    // A drop-in replacement for std::vector that uses a Zone for its allocations.
    pub struct ZoneVector<T> {
        zone: *mut Zone, // Raw pointer, needs careful management
        data: *mut T,    // Raw pointer to the start of the data
        end: *mut T,     // Raw pointer to the end of the valid data
        capacity: *mut T, // Raw pointer to the end of the allocated capacity
    }

    impl<T> ZoneVector<T> {
        pub type Iterator<'a> = std::slice::Iter<'a, T>;
        pub type ConstIterator<'a> = std::slice::Iter<'a, T>;
        pub type ReverseIterator<'a> = std::slice::Iter<'a, T>;
        pub type ConstReverseIterator<'a> = std::slice::Iter<'a, T>;
        pub type ValueType = T;
        pub type Reference<'a> = &'a T;
        pub type ConstReference<'a> = &'a T;
        pub type SizeType = usize;

        const MIN_CAPACITY: usize = 2;

        /// Constructs an empty vector.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneVector {
                zone,
                data: ptr::null_mut(),
                end: ptr::null_mut(),
                capacity: ptr::null_mut(),
            }
        }

        /// Constructs a new vector and fills it with {size} elements, each
        /// constructed via the default constructor.
        pub fn with_size(size: usize, zone: *mut Zone) -> Self
        where
            T: Default,
        {
            let mut vec = ZoneVector::new(zone);
            vec.resize(size);
            vec
        }

        /// Constructs a new vector and fills it with {size} elements, each
        /// having the value {def}.
        pub fn with_value(size: usize, def: T, zone: *mut Zone) -> Self
        where
            T: Clone,
        {
            let mut vec = ZoneVector::new(zone);
            vec.resize_with(size, || def.clone());
            vec
        }

        /// Constructs a new vector and fills it with the contents of the given
        /// initializer list.
        pub fn from_initializer_list(list: &[T], zone: *mut Zone) -> Self
        where
            T: Clone,
        {
            let mut vec = ZoneVector::new(zone);
            vec.extend_from_slice(list);
            vec
        }

        /// Constructs a new vector and fills it with the contents of the range
        /// [first, last).
        pub fn from_iter<I>(iter: I, zone: *mut Zone) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let mut vec = ZoneVector::new(zone);
            vec.extend(iter);
            vec
        }

        // Assignment operators.
        pub fn assign(&mut self, other: &Self)
        where
            T: Clone,
        {
            if self as *const _ == other {
                panic!("Self-assignment is not allowed");
            }

            if self.capacity() >= other.size() && self.zone == other.zone {
                let dst = self.data;
                let mut dst_ptr = dst;
                let src = other.data;
                let mut src_ptr = src;

                if mem::needs_drop::<T>() {
                    unsafe {
                        while dst_ptr < self.end && src_ptr < other.end {
                            dst_ptr.write(src_ptr.read().clone());
                            dst_ptr = dst_ptr.add(1);
                            src_ptr = src_ptr.add(1);
                        }
                        while src_ptr < other.end {
                            self.emplace_back(src_ptr.read().clone());
                            src_ptr = src_ptr.add(1);
                        }
                        let old_end = self.end;
                        self.end = dst_ptr;

                        for p in (self.end as usize..old_end as usize).step_by(mem::size_of::<T>()) {
                            (p as *mut T).drop_in_place();
                        }
                    }
                } else {
                    unsafe {
                        let size = other.size();
                        if size > 0 {
                            ptr::copy_nonoverlapping(src, dst, size);
                            // ptr::copy(src, dst, size); //copy could cause issues due to overlapping
                        }
                        self.end = dst.add(size);
                    }
                }
            } else {
                self.clear();
                if self.data != std::ptr::null_mut() {
                    unsafe {
                        (*(self.zone)).delete_array(self.data, self.capacity());
                    }
                }

                let new_cap = other.capacity();
                if new_cap > 0 {
                    self.data = unsafe { (*(self.zone)).allocate_array::<T>(new_cap) };
                    unsafe {
                        ptr::copy_nonoverlapping(other.data, self.data, other.size());
                    }
                } else {
                    self.data = ptr::null_mut();
                }

                self.capacity = unsafe { self.data.add(new_cap) };
                self.end = unsafe { self.data.add(other.size()) };
            }
        }

        pub fn assign_from_initializer_list(&mut self, ilist: &[T])
        where
            T: Clone,
        {
            self.clear();
            self.ensure_capacity(ilist.len());
            unsafe {
                ptr::copy_nonoverlapping(ilist.as_ptr(), self.data, ilist.len());
                self.end = self.data.add(ilist.len());
            }
        }

        pub fn swap(&mut self, other: &mut Self) {
            if self.zone != other.zone {
                panic!("Zones must be equal for swap");
            }
            std::mem::swap(&mut self.data, &mut other.data);
            std::mem::swap(&mut self.end, &mut other.end);
            std::mem::swap(&mut self.capacity, &mut other.capacity);
        }

        pub fn resize(&mut self, new_size: usize)
        where
            T: Default,
        {
            self.ensure_capacity(new_size);
            unsafe {
                let mut new_end = self.data.add(new_size);
                while self.end < new_end {
                    ptr::write(self.end, T::default());
                    self.end = self.end.add(1);
                }

                while new_end < self.end {
                    self.end = self.end.sub(1);
                    ptr::drop_in_place(self.end);
                }
            }
        }

        pub fn resize_with<F: FnMut() -> T>(&mut self, new_size: usize, f: F) {
            self.ensure_capacity(new_size);
            unsafe {
                let mut new_end = self.data.add(new_size);
                while self.end < new_end {
                    ptr::write(self.end, f());
                    self.end = self.end.add(1);
                }

                while new_end < self.end {
                    self.end = self.end.sub(1);
                    ptr::drop_in_place(self.end);
                }
            }
        }

        pub fn clear(&mut self) {
            if mem::needs_drop::<T>() {
                unsafe {
                    let mut current = self.data;
                    while current < self.end {
                        ptr::drop_in_place(current);
                        current = current.add(1);
                    }
                }
            }
            self.end = self.data;
        }

        pub fn size(&self) -> usize {
            if self.data.is_null() || self.end.is_null() {
                return 0;
            }
            unsafe { self.end.offset_from(self.data) as usize }
        }

        pub fn empty(&self) -> bool {
            self.size() == 0
        }

        pub fn capacity(&self) -> usize {
            if self.data.is_null() || self.capacity.is_null() {
                return 0;
            }
            unsafe { self.capacity.offset_from(self.data) as usize }
        }

        pub fn reserve(&mut self, new_cap: usize) {
            self.ensure_capacity(new_cap);
        }

        pub fn data(&self) -> *mut T {
            self.data
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone
        }

        pub fn at(&self, pos: usize) -> &T {
            if pos >= self.size() {
                panic!("Index out of bounds");
            }
            unsafe { &*self.data.add(pos) }
        }

        pub fn at_mut(&mut self, pos: usize) -> &mut T {
            if pos >= self.size() {
                panic!("Index out of bounds");
            }
            unsafe { &mut *self.data.add(pos) }
        }

        pub fn front(&self) -> &T {
            if self.empty() {
                panic!("Vector is empty");
            }
            unsafe { &*self.data }
        }

        pub fn front_mut(&mut self) -> &mut T {
            if self.empty() {
                panic!("Vector is empty");
            }
            unsafe { &mut *self.data }
        }

        pub fn back(&self) -> &T {
            if self.empty() {
                panic!("Vector is empty");
            }
            unsafe { &*self.end.sub(1) }
        }

        pub fn back_mut(&mut self) -> &mut T {
            if self.empty() {
                panic!("Vector is empty");
            }
            unsafe { &mut *self.end.sub(1) }
        }

        pub fn begin(&self) -> *mut T {
            self.data
        }

        pub fn end(&self) -> *mut T {
            self.end
        }

        pub fn push_back(&mut self, value: T) {
            self.ensure_one_more_capacity();
            unsafe {
                ptr::write(self.end, value);
                self.end = self.end.add(1);
            }
        }

        pub fn pop_back(&mut self) {
            if self.end == self.data {
                panic!("Vector is empty");
            }
            unsafe {
                self.end = self.end.sub(1);
                ptr::drop_in_place(self.end);
            }
        }

        pub fn emplace_back(&mut self, value: T) -> &mut T {
            self.ensure_one_more_capacity();
            unsafe {
                let ptr = self.end;
                ptr::write(ptr, value);
                self.end = self.end.add(1);
                &mut *ptr
            }
        }

        fn new_capacity(&self, minimum: usize) -> usize {
            let new_capacity = if self.data == self.capacity {
                Self::MIN_CAPACITY
            } else {
                self.capacity() * 2
            };
            if new_capacity < minimum {
                minimum
            } else {
                new_capacity
            }
        }

        #[inline]
        fn ensure_one_more_capacity(&mut self) {
            if self.end < self.capacity {
                return;
            }
            let new_capacity = if self.data == self.capacity {
                Self::MIN_CAPACITY
            } else {
                self.capacity() * 2
            };

            self.grow(new_capacity);
        }

        #[inline]
        fn ensure_capacity(&mut self, minimum: usize) {
            if minimum <= self.capacity() {
                return;
            }
            self.grow(minimum);
        }

        #[cold]
        #[inline(never)]
        fn grow(&mut self, minimum: usize) {
            unsafe {
                let old_data = self.data;
                let old_end = self.end;
                let old_size = self.size();
                let new_capacity = self.new_capacity(minimum);
                self.data = (*(self.zone)).allocate_array::<T>(new_capacity);
                self.end = self.data.add(old_size);

                if old_data != ptr::null_mut() {
                    ptr::copy_nonoverlapping(old_data, self.data, old_size);
                    (*(self.zone)).delete_array(old_data, self.capacity());
                }

                self.capacity = self.data.add(new_capacity);
            }
        }

        pub fn insert(&mut self, pos: *const T, value: T) -> *mut T
        where
            T: Clone,
        {
            unsafe {
                let index = pos.offset_from(self.data) as usize;
                self.ensure_one_more_capacity();
                let p: *mut T = self.data.add(index);

                ptr::copy(p, self.data.add(index + 1), self.size() - index);
                ptr::write(self.data.add(index), value);
                self.end = self.end.add(1);
                self.data
            }
        }

        // TODO : Implement the other insert and erase methods
        // Need to ensure safety of pointer manipulation in Rust
        pub fn erase(&mut self, pos: *const T) -> *mut T {
            if pos < self.data || pos > self.end {
                panic!("Pointer not within vector's bounds");
            }

            unsafe {
                let index = pos.offset_from(self.data) as usize;
                if index as usize == self.size() {
                    return self.end as *mut T;
                }
                return self.erase_range(pos, 1);
            }
        }

        pub fn erase_range(&mut self, first: *const T, count: usize) -> *mut T {
            if first < self.data || first > self.end {
                panic!("Pointer not within vector's bounds");
            }
            unsafe {
                let position = first as *mut T;
                let first_ptr = first as *mut T;
                ptr::copy(first_ptr.add(count), first_ptr, self.size() - (first_ptr.offset_from(self.data) as usize + count));
                // MovingOverwrite(position, position + count, end());
                let old_end = self.end;
                self.end = self.end.sub(count);

                for p in (self.end as usize..old_end as usize).step_by(mem::size_of::<T>()) {
                    (p as *mut T).drop_in_place();
                }
                return position;
            }
        }
    }

    impl<T> Drop for ZoneVector<T> {
        fn drop(&mut self) {
            if self.data != ptr::null_mut() {
                if mem::needs_drop::<T>() {
                    unsafe {
                        let mut current = self.data;
                        while current < self.end {
                            ptr::drop_in_place(current);
                            current = current.add(1);
                        }
                    }
                }
                unsafe {
                    (*(self.zone)).delete_array(self.data, self.capacity());
                }
            }
            self.data = ptr::null_mut();
            self.end = ptr::null_mut();
            self.capacity = ptr::null_mut();
        }
    }

    impl<T: PartialEq> PartialEq for ZoneVector<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.size() != other.size() {
                return false;
            }
            unsafe {
                for i in 0..self.size() {
                    if *self.data.add(i) != *other.data.add(i) {
                        return false;
                    }
                }
            }
            true
        }
    }

    impl<T: PartialOrd> PartialOrd for ZoneVector<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let mut i = 0;
            while i < self.size() && i < other.size() {
                unsafe {
                    match self.data.add(i).partial_cmp(&other.data.add(i)) {
                        Some(Ordering::Equal) => (),
                        other => return other,
                    }
                }
                i += 1;
            }

            if self.size() < other.size() {
                Some(Ordering::Less)
            } else if self.size() > other.size() {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }

    impl<T: Clone> Clone for ZoneVector<T> {
        fn clone(&self) -> Self {
            let mut new_vec = ZoneVector::new(self.zone);
            new_vec.assign(self);
            new_vec
        }
    }

    impl<T> Index<usize> for ZoneVector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            self.at(index)
        }
    }

    impl<T> IndexMut<usize> for ZoneVector<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.at_mut(index)
        }
    }

    impl<T> Extend<T> for ZoneVector<T>
    where
        T: Clone,
    {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            for elem in iter {
                self.push_back(elem);
            }
        }
    }

    impl<T> ZoneVector<T> {
        pub fn extend_from_slice(&mut self, slice: &[T])
        where
            T: Clone,
        {
            self.ensure_capacity(self.size() + slice.len());
            unsafe {
                for elem in slice {
                    ptr::write(self.end, elem.clone());
                    self.end = self.end.add(1);
                }
            }
        }
    }
    //------------------------------------------------------------------------------------------------------
    // Need to implement IntrusiveSet properly
    // Implementation requires careful handling of raw pointers and intrusive structures.
    //------------------------------------------------------------------------------------------------------
    pub struct ZoneIntrusiveSet<T, GetIntrusiveSetIndex, V = ZoneVector<T>> {
        data: V,
        index_functor: GetIntrusiveSetIndex,
        _marker: PhantomData<T>,
    }

    impl<T, GetIntrusiveSetIndex> ZoneIntrusiveSet<T, GetIntrusiveSetIndex>
    where
        GetIntrusiveSetIndex: Fn(&T) -> usize,
    {
        pub fn new(zone: *mut Zone, index_functor: GetIntrusiveSetIndex) -> Self {
            ZoneIntrusiveSet {
                data: ZoneVector::new(zone),
                index_functor,
                _marker: PhantomData,
            }
        }
    }

    pub struct IntrusiveSetIndex;

    // A wrapper subclass for std::deque to make it easy to construct one
    // that uses a zone allocator.
    pub struct ZoneDeque<T> {
        deque: VecDeque<T>,
        zone: *mut Zone, // Raw pointer
    }

    impl<T> ZoneDeque<T> {
        /// Constructs an empty deque.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneDeque {
                deque: VecDeque::new(),
                zone,
            }
        }
    }

    impl<T> Deref for ZoneDeque<T> {
        type Target = VecDeque<T>;

        fn deref(&self) -> &Self::Target {
            &self.deque
        }
    }

    impl<T> DerefMut for ZoneDeque<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.deque
        }
    }

    // A wrapper subclass for std::list to make it easy to construct one
    // that uses a zone allocator.
    pub struct ZoneLinkedList<T> {
        list: LinkedList<T>,
        zone: *mut Zone, // Raw pointer
    }

    impl<T> ZoneLinkedList<T> {
        /// Constructs an empty list.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneLinkedList {
                list: LinkedList::new(),
                zone,
            }
        }
    }

    impl<T> Deref for ZoneLinkedList<T> {
        type Target = LinkedList<T>;

        fn deref(&self) -> &Self::Target {
            &self.list
        }
    }

    impl<T> DerefMut for ZoneLinkedList<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.list
        }
    }

    // A wrapper subclass for std::forward_list to make it easy to construct one
    // that uses a zone allocator.
    pub struct ZoneForwardList<T> {
        list: LinkedList<T>,
        zone: *mut Zone, // Raw pointer
    }

    impl<T> ZoneForwardList<T> {
        /// Constructs an empty list.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneForwardList {
                list: LinkedList::new(),
                zone,
            }
        }
    }

    // A wrapper subclass for std::priority_queue to make it easy to construct one
    // that uses a zone allocator.
    pub struct ZonePriorityQueue<T, Compare = std::collections::BinaryHeap<T>>
    where
        Compare: Ord,
    {
        queue: std::collections::BinaryHeap<T>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<Compare>,
    }

    impl<T: Ord> ZonePriorityQueue<T> {
        /// Constructs an empty list.
        pub fn new(zone: *mut Zone) -> Self {
            ZonePriorityQueue {
                queue: std::collections::BinaryHeap::new(),
                zone,
                _marker: PhantomData,
            }
        }
    }

    // A wrapper subclass for std::queue to make it easy to construct one
    // that uses a zone allocator.
    pub struct ZoneQueue<T> {
        queue: VecDeque<T>,
        zone: *mut Zone, // Raw pointer
    }

    impl<T> ZoneQueue<T> {
        /// Constructs an empty queue.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneQueue {
                queue: VecDeque::new(),
                zone,
            }
        }
    }

    // A wrapper subclass for std::stack to make it easy to construct one that uses
    // a zone allocator.
    pub struct ZoneStack<T> {
        stack: Vec<T>,
        zone: *mut Zone, // Raw pointer
    }

    impl<T> ZoneStack<T> {
        /// Constructs an empty stack.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneStack {
                stack: Vec::new(),
                zone,
            }
        }
    }

    // A wrapper subclass for std::set to make it easy to construct one that uses
    // a zone allocator.
    pub struct ZoneSet<K, Compare = std::cmp::PartialEq> {
        set: std::collections::HashSet<K>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<Compare>,
    }

    impl<K: Eq + std::hash::Hash> ZoneSet<K> {
        /// Constructs an empty set.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneSet {
                set: std::collections::HashSet::new(),
                zone,
                _marker: PhantomData,
            }
        }
    }

    // A wrapper subclass for std::multiset to make it easy to construct one that
    // uses a zone allocator.
    pub struct ZoneMultiset<K, Compare = std::cmp::PartialEq> {
        set: Vec<K>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<Compare>,
    }

    impl<K: Eq + std::hash::Hash + Clone> ZoneMultiset<K> {
        /// Constructs an empty multiset.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneMultiset {
                set: Vec::new(),
                zone,
                _marker: PhantomData,
            }
        }

        pub fn insert(&mut self, value: K) {
            self.set.push(value);
        }
    }

    // A wrapper subclass for std::map to make it easy to construct one that uses
    // a zone allocator.
    pub struct ZoneMap<K, V, Compare = std::cmp::PartialEq> {
        map: std::collections::HashMap<K, V>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<Compare>,
    }

    impl<K: Eq + std::hash::Hash, V> ZoneMap<K, V> {
        /// Constructs an empty map.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneMap {
                map: std::collections::HashMap::new(),
                zone,
                _marker: PhantomData,
            }
        }
    }

    // A wrapper subclass for std::unordered_map to make it easy to construct one
    // that uses a zone allocator.
    pub struct ZoneUnorderedMap<K, V, Hash = std::collections::hash_map::DefaultHasher, KeyEqual = std::cmp::PartialEq> {
        map: std::collections::HashMap<K, V>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<(Hash, KeyEqual)>,
    }

    impl<K: Eq + std::hash::Hash, V> ZoneUnorderedMap<K, V> {
        /// Constructs an empty map.
        pub fn new(zone: *mut Zone, bucket_count: usize) -> Self {
            let mut map = std::collections::HashMap::new();
            //map.reserve(bucket_count);
            ZoneUnorderedMap {
                map,
                zone,
                _marker: PhantomData,
            }
        }
    }

    // A wrapper subclass for std::unordered_set to make it easy to construct one
    // that uses a zone allocator.
    pub struct ZoneUnorderedSet<K, Hash = std::collections::hash_map::DefaultHasher, KeyEqual = std::cmp::PartialEq> {
        set: std::collections::HashSet<K>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<(Hash, KeyEqual)>,
    }

    impl<K: Eq + std::hash::Hash> ZoneUnorderedSet<K> {
        /// Constructs an empty set.
        pub fn new(zone: *mut Zone, bucket_count: usize) -> Self {
            let mut set = std::collections::HashSet::new();
            //set.reserve(bucket_count);
            ZoneUnorderedSet {
                set,
                zone,
                _marker: PhantomData,
            }
        }
    }

    // A wrapper subclass for std::multimap to make it easy to construct one that
    // uses a zone allocator.
    pub struct ZoneMultimap<K, V, Compare = std::cmp::PartialEq> {
        map: std::collections::BTreeMap<K, Vec<V>>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<Compare>,
    }

    impl<K: Ord + Clone, V: Clone> ZoneMultimap<K, V> {
        /// Constructs an empty multimap.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneMultimap {
                map: std::collections::BTreeMap::new(),
                zone,
                _marker: PhantomData,
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            self.map.entry(key.clone()).or_insert_with(Vec::new).push(value);
        }
    }

    // A wrapper subclass for base::SmallVector to make it easy to construct one
    // that uses a zone allocator.
    pub struct SmallZoneVector<T, const SIZE: usize> {
        vec: Vec<T>,
        zone: *mut Zone, // Raw pointer
    }

    impl<T, const SIZE: usize> SmallZoneVector<T, SIZE> {
        /// Constructs an empty small vector.
        pub fn new(zone: *mut Zone) -> Self {
            SmallZoneVector {
                vec: Vec::new(),
                zone,
            }
        }

        pub fn with_size(size: usize, zone: *mut Zone) -> Self
        where
            T: Default,
        {
            SmallZoneVector {
                vec: vec![T::default(); size],
                zone,
            }
        }
    }

    // Used by SmallZoneMap below. Essentially a closure around placement-new of
    // the "full" fallback ZoneMap. Called once SmallMap grows beyond kArraySize.
    struct ZoneMapInit<'a, ZoneMap> {
        zone: &'a Zone,
        _marker: PhantomData<ZoneMap>,
    }

    impl<'a, ZoneMap> ZoneMapInit<'a, ZoneMap> {
        fn new(zone: &'a Zone) -> Self {
            ZoneMapInit {
                zone,
                _marker: PhantomData,
            }
        }
    }

    // A wrapper subclass for base::SmallMap to make it easy to construct one that
    // uses a zone-allocated std::map as the fallback once the SmallMap outgrows
    // its inline storage.
    pub struct SmallZoneMap<K, V, const ARRAYSIZE: usize, Compare = std::cmp::PartialEq, KeyEqual = std::cmp::PartialEq> {
        map: std::collections::HashMap<K, V>, // In real implementation SmallMap would have inline storage
        zone: *mut Zone,                      // Raw pointer
        _marker: PhantomData<(Compare, KeyEqual)>,
    }

    impl<K: Eq + std::hash::Hash, V, const ARRAYSIZE: usize> SmallZoneMap<K, V, ARRAYSIZE> {
        pub fn new(zone: *mut Zone) -> Self {
            SmallZoneMap {
                map: std::collections::HashMap::new(),
                zone,
                _marker: PhantomData,
            }
        }
    }

    // A wrapper subclass for absl::flat_hash_map to make it easy to construct one
    // that uses a zone allocator. If you want to use a user-defined type as key
    // (K), you'll need to define a AbslHashValue function for it (see
    // https://abseil.io/docs/cpp/guides/hash).
    pub struct ZoneAbslFlatHashMap<K, V, Hash = std::collections::hash_map::DefaultHasher, KeyEqual = std::cmp::PartialEq> {
        map: std::collections::HashMap<K, V>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<(Hash, KeyEqual)>,
    }

    impl<K: Eq + std::hash::Hash, V> ZoneAbslFlatHashMap<K, V> {
        /// Constructs an empty map.
        pub fn new(zone: *mut Zone, bucket_count: usize) -> Self {
            let mut map = std::collections::HashMap::new();
            //map.reserve(bucket_count);
            ZoneAbslFlatHashMap {
                map,
                zone,
                _marker: PhantomData,
            }
        }
    }

    // A wrapper subclass for absl::flat_hash_set to make it easy to construct one
    // that uses a zone allocator. If you want to use a user-defined type as key
    // (K), you'll need to define a AbslHashValue function for it (see
    // https://abseil.io/docs/cpp/guides/hash).
    pub struct ZoneAbslFlatHashSet<K, Hash = std::collections::hash_map::DefaultHasher, KeyEqual = std::cmp::PartialEq> {
        set: std::collections::HashSet<K>,
        zone: *mut Zone, // Raw pointer
        _marker: PhantomData<(Hash, KeyEqual)>,
    }

    impl<K: Eq + std::hash::Hash> ZoneAbslFlatHashSet<K> {
        /// Constructs an empty set.
        pub fn new(zone: *mut Zone, bucket_count: usize) ->