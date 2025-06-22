pub mod base {
    use std::alloc::{alloc, dealloc, Layout};
    use std::marker::PhantomData;
    use std::mem::{self, MaybeUninit};
    use std::ptr::NonNull;

    macro_rules! v8_inline {
        ($x:item) => {
            #[inline(always)]
            $x
        };
    }

    pub struct DefaultAllocationPolicy;

    impl DefaultAllocationPolicy {
        v8_inline! {
            pub fn allocate_array<T>(&self, length: usize) -> *mut T {
                let layout = Layout::array::<T>(length).unwrap();
                unsafe {
                    alloc(layout) as *mut T
                }
            }
        }
        v8_inline! {
            pub fn delete_array<T>(&self, p: *mut T, length: usize) {
                if p.is_null() {
                    return;
                }
                let layout = Layout::array::<T>(length).unwrap();
                unsafe {
                    dealloc(p as *mut u8, layout);
                }
            }
        }
    }

    pub struct TemplateHashMapEntry<K, V> {
        pub key: K,
        pub value: V,
        pub hash: u32,
        exists: bool,
    }

    impl<K, V> TemplateHashMapEntry<K, V> {
        pub fn new(key: K, value: V, hash: u32) -> Self {
            TemplateHashMapEntry {
                key,
                value,
                hash,
                exists: true,
            }
        }

        pub fn clear(&mut self) {
            self.exists = false;
        }

        pub fn exists(&self) -> bool {
            self.exists
        }
    }

    pub struct Impl<Key, Value, MatchFun, AllocationPolicy> {
        map_: *mut TemplateHashMapEntry<Key, Value>,
        capacity_: u32,
        occupancy_: u32,
        match_fun: MatchFun,
        allocation_policy: AllocationPolicy,
        _phantom: PhantomData<(Key, Value)>,
    }

    impl<Key, Value, MatchFun, AllocationPolicy> Impl<Key, Value, MatchFun, AllocationPolicy>
    where
        AllocationPolicy: Sized,
    {
        pub fn new(match_fun: MatchFun, allocation_policy: AllocationPolicy) -> Self {
            Impl {
                map_: std::ptr::null_mut(),
                capacity_: 0,
                occupancy_: 0,
                match_fun,
                allocation_policy,
                _phantom: PhantomData,
            }
        }

        pub fn match_(&self) -> &MatchFun {
            &self.match_fun
        }
        pub fn match_mut(&mut self) -> &mut MatchFun {
            &mut self.match_fun
        }

        pub fn allocator(&self) -> &AllocationPolicy {
            &self.allocation_policy
        }

        pub fn allocator_mut(&mut self) -> &mut AllocationPolicy {
            &mut self.allocation_policy
        }
    }

    pub struct TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy> {
        impl_: Impl<Key, Value, MatchFun, AllocationPolicy>,
    }

    impl<Key, Value, MatchFun, AllocationPolicy>
        TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy>
    where
        MatchFun: Fn(u32, u32, &Key, &Key) -> bool,
        AllocationPolicy: Sized,
    {
        pub const K_DEFAULT_HASH_MAP_CAPACITY: u32 = 8;

        pub fn new(
            capacity: u32,
            match_fun: MatchFun,
            allocation_policy: AllocationPolicy,
        ) -> Self {
            let mut map = TemplateHashMapImpl {
                impl_: Impl::new(match_fun, allocation_policy),
            };
            map.initialize(capacity);
            map
        }

        pub fn new_from_original(
            original: &TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy>,
            allocator: AllocationPolicy,
        ) -> Self {
            let mut map = TemplateHashMapImpl {
                impl_: Impl::new(original.impl_.match_().clone(), allocator),
            };

            map.impl_.capacity_ = original.capacity();
            map.impl_.occupancy_ = original.occupancy();
            map.impl_.map_ = map.impl_.allocator().allocate_array::<TemplateHashMapEntry<Key, Value>>(map.capacity() as usize);

            unsafe {
                std::ptr::copy_nonoverlapping(
                    original.impl_.map_,
                    map.impl_.map_,
                    (map.capacity() * std::mem::size_of::<TemplateHashMapEntry<Key, Value>>()) as usize,
                );
            }
            map
        }

        pub fn clear(&mut self) {
            for i in 0..self.capacity() {
                unsafe {
                    (*self.impl_.map_.add(i as usize)).clear();
                }
            }
            self.impl_.occupancy_ = 0;
        }

        pub fn invalidate(&mut self) {
            assert!(!self.impl_.map_.is_null());
            self.impl_.allocator().delete_array(self.impl_.map_, self.capacity() as usize);
            self.impl_ = Impl::new(self.impl_.match_().clone(), self.impl_.allocator().clone());
        }

        pub fn occupancy(&self) -> u32 {
            self.impl_.occupancy_
        }

        pub fn capacity(&self) -> u32 {
            self.impl_.capacity_
        }

        fn map_end(&self) -> *mut TemplateHashMapEntry<Key, Value> {
            unsafe { self.impl_.map_.add(self.impl_.capacity_ as usize) }
        }

        fn probe<LookupKey>(&self, key: &LookupKey, hash: u32) -> *mut TemplateHashMapEntry<Key, Value>
        where
            MatchFun: Fn(u32, u32, &LookupKey, &Key) -> bool,
        {
            assert!(is_power_of_two(self.capacity()));
            let mut i = (hash & (self.capacity() - 1)) as usize;
            assert!(i < self.capacity() as usize);

            assert!(self.occupancy() < self.capacity());

            let map = self.impl_.map_;

            unsafe {
                while (*map.add(i)).exists()
                    && !(self.impl_.match_()(hash, (*map.add(i)).hash, key, &(*map.add(i)).key))
                {
                    i = (i + 1) & ((self.capacity() - 1) as usize);
                }
                map.add(i)
            }
        }

        fn fill_empty_entry(
            &mut self,
            entry: *mut TemplateHashMapEntry<Key, Value>,
            key: Key,
            value: Value,
            hash: u32,
        ) -> *mut TemplateHashMapEntry<Key, Value> {
            unsafe {
                assert!(!(*entry).exists());
                entry.write(TemplateHashMapEntry::new(key, value, hash));
            }

            self.impl_.occupancy_ += 1;

            if self.occupancy() + self.occupancy() / 4 >= self.capacity() {
                self.resize();
                return self.probe(&unsafe { (*entry).key }, hash);
            }

            entry
        }

        fn initialize(&mut self, capacity: u32) {
            assert!(is_power_of_two(capacity));
            self.impl_.map_ = self.impl_.allocator().allocate_array::<TemplateHashMapEntry<Key, Value>>(capacity as usize);
            if self.impl_.map_.is_null() {
                panic!("Out of memory: HashMap::Initialize");
            }
            self.impl_.capacity_ = capacity;
            self.clear();
        }

        fn resize(&mut self) {
            let old_map = self.impl_.map_;
            let old_capacity = self.capacity();
            let mut n = self.occupancy();

            self.initialize(self.capacity() * 2);

            let mut entry = old_map;

            unsafe {
                for _ in 0..old_capacity {
                    if (*entry).exists() {
                        let new_entry = self.probe(&(*entry).key, (*entry).hash);
                        self.fill_empty_entry(new_entry, (*entry).key, (*entry).value, (*entry).hash);
                        n -= 1;
                    }
                    entry = entry.add(1);
                }
            }

            self.impl_.allocator().delete_array(old_map, old_capacity as usize);
        }

        pub fn lookup(&self, key: &Key, hash: u32) -> Option<&mut TemplateHashMapEntry<Key, Value>>
        where
            MatchFun: Fn(u32, u32, &Key, &Key) -> bool,
        {
            let entry = self.probe(key, hash);
            unsafe {
                if (*entry).exists() {
                    Some(&mut *entry)
                } else {
                    None
                }
            }
        }

        pub fn lookup_or_insert(&mut self, key: Key, hash: u32) -> &mut TemplateHashMapEntry<Key, Value>
        where
            Value: Default,
        {
            self.lookup_or_insert_with(key, hash, || Value::default())
        }

        pub fn lookup_or_insert_with<Func>(&mut self, key: Key, hash: u32, value_func: Func) -> &mut TemplateHashMapEntry<Key, Value>
        where
            Func: Fn() -> Value,
        {
            self.lookup_or_insert_with_key(key, hash, || key, value_func)
        }

        pub fn lookup_or_insert_with_key<LookupKey, KeyFunc, ValueFunc>(
            &mut self,
            lookup_key: LookupKey,
            hash: u32,
            key_func: KeyFunc,
            value_func: ValueFunc,
        ) -> &mut TemplateHashMapEntry<Key, Value>
        where
            MatchFun: Fn(u32, u32, &LookupKey, &Key) -> bool,
            KeyFunc: Fn() -> Key,
            ValueFunc: Fn() -> Value,
        {
            let entry = self.probe(&lookup_key, hash);
            unsafe {
                if (*entry).exists() {
                    return &mut *entry;
                }
            }

            self.fill_empty_entry(entry, key_func(), value_func(), hash);

            unsafe { &mut *entry }
        }

        pub fn insert_new(&mut self, key: Key, hash: u32) -> *mut TemplateHashMapEntry<Key, Value> {
            let entry = self.probe(&key, hash);
            self.fill_empty_entry(entry, key, Value::default(), hash)
        }

        pub fn remove(&mut self, key: &Key, hash: u32) -> Option<Value>
        where
            MatchFun: Fn(u32, u32, &Key, &Key) -> bool,
        {
            let p = self.probe(key, hash);
            unsafe {
                if !(*p).exists() {
                    return None;
                }

                let value = mem::replace(&(*p).value, Value::default());
                let value = if mem::needs_drop::<Value>() {
                    Some(value)
                } else {
                    None
                };

                assert!(self.occupancy() < self.capacity());

                let mut q = p;
                loop {
                    q = q.add(1);
                    if q == self.map_end() {
                        q = self.impl_.map_;
                    }

                    if !(*q).exists() {
                        break;
                    }

                    let r = self.impl_.map_.add(((*q).hash & (self.capacity() - 1)) as usize);

                    if (q > p && (r <= p || r > q)) || (q < p && (r <= p && r > q)) {
                        *p = mem::replace(&mut *q, TemplateHashMapEntry { key: mem::zeroed(), value: mem::zeroed(), hash: 0, exists: false });
                        p = q;
                    }
                }

                (*p).clear();
                self.impl_.occupancy_ -= 1;

                return Some(value.unwrap_or_else(|| mem::zeroed()));
            }
        }

        pub fn start(&self) -> *mut TemplateHashMapEntry<Key, Value> {
            self.next(unsafe { self.impl_.map_.sub(1) })
        }

        pub fn next(&self, entry: *mut TemplateHashMapEntry<Key, Value>) -> *mut TemplateHashMapEntry<Key, Value> {
            let end = self.map_end();
            unsafe {
                let mut entry = entry.add(1);
                while entry < end {
                    if (*entry).exists() {
                        return entry;
                    }
                    entry = entry.add(1);
                }
                std::ptr::null_mut()
            }
        }

        pub fn allocator(&self) -> &AllocationPolicy {
            self.impl_.allocator()
        }
    }

    impl<Key, Value, MatchFun, AllocationPolicy> Drop for TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy> {
        fn drop(&mut self) {
            if !self.impl_.map_.is_null() {
                self.impl_.allocator().delete_array(self.impl_.map_, self.capacity() as usize);
            }
        }
    }

    fn is_power_of_two(x: u32) -> bool {
        (x != 0) && ((x & (x - 1)) == 0)
    }

    #[derive(Clone)]
    pub struct HashEqualityThenKeyMatcher<Key, MatchFun> {
        match_: MatchFun,
        _phantom: PhantomData<Key>,
    }

    impl<Key, MatchFun> HashEqualityThenKeyMatcher<Key, MatchFun> {
        pub fn new(match_: MatchFun) -> Self {
            HashEqualityThenKeyMatcher {
                match_,
                _phantom: PhantomData,
            }
        }
    }

    impl<Key, MatchFun> Fn<(u32, u32, &Key, &Key)> for HashEqualityThenKeyMatcher<Key, MatchFun>
    where
        MatchFun: Fn(&Key, &Key) -> bool,
    {
        extern "rust-call" fn call(&self, args: (u32, u32, &Key, &Key)) -> Self::Output {
            self.call_impl(args.0, args.1, args.2, args.3)
        }
    }

    impl<Key, MatchFun> FnMut<(u32, u32, &Key, &Key)> for HashEqualityThenKeyMatcher<Key, MatchFun>
    where
        MatchFun: Fn(&Key, &Key) -> bool,
    {
        extern "rust-call" fn call_mut(&mut self, args: (u32, u32, &Key, &Key)) -> Self::Output {
            self.call_impl(args.0, args.1, args.2, args.3)
        }
    }

    impl<Key, MatchFun> FnOnce<(u32, u32, &Key, &Key)> for HashEqualityThenKeyMatcher<Key, MatchFun>
    where
        MatchFun: Fn(&Key, &Key) -> bool,
    {
        type Output = bool;

        extern "rust-call" fn call_once(self, args: (u32, u32, &Key, &Key)) -> Self::Output {
            self.call_impl(args.0, args.1, args.2, args.3)
        }
    }

    impl<Key, MatchFun> HashEqualityThenKeyMatcher<Key, MatchFun>
    where
        MatchFun: Fn(&Key, &Key) -> bool,
    {
        fn call_impl(&self, hash1: u32, hash2: u32, key1: &Key, key2: &Key) -> bool {
            hash1 == hash2 && (self.match_)(key1, key2)
        }
    }

    pub type CustomMatcherTemplateHashMapImpl<AllocationPolicy> =
        TemplateHashMapImpl<
            *mut std::ffi::c_void,
            *mut std::ffi::c_void,
            HashEqualityThenKeyMatcher<*mut std::ffi::c_void, fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> bool>,
            AllocationPolicy,
        >;
    impl<AllocationPolicy> CustomMatcherTemplateHashMapImpl<AllocationPolicy>
    where AllocationPolicy: Sized,
    {
        pub fn new(
            match_: fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> bool,
            capacity: u32,
            allocator: AllocationPolicy,
        ) -> Self {
            TemplateHashMapImpl::new(
                capacity,
                HashEqualityThenKeyMatcher::new(match_),
                allocator,
            )
        }

         pub fn new_from_original(
            original: &CustomMatcherTemplateHashMapImpl<AllocationPolicy>,
            allocator: AllocationPolicy,
        ) -> Self {
            TemplateHashMapImpl::new_from_original(original, allocator)
        }
    }

    pub type CustomMatcherHashMap = CustomMatcherTemplateHashMapImpl<DefaultAllocationPolicy>;

    #[derive(Clone)]
    pub struct KeyEqualityMatcher<Key> {
        _phantom: PhantomData<Key>,
    }

    impl<Key> KeyEqualityMatcher<Key> {
        pub fn new() -> Self {
            KeyEqualityMatcher {
                _phantom: PhantomData,
            }
        }
    }

    impl<Key> Fn<(u32, u32, &Key, &Key)> for KeyEqualityMatcher<Key>
    where
        Key: PartialEq,
    {
        extern "rust-call" fn call(&self, args: (u32, u32, &Key, &Key)) -> Self::Output {
            self.call_impl(args.0, args.1, args.2, args.3)
        }
    }

    impl<Key> FnMut<(u32, u32, &Key, &Key)> for KeyEqualityMatcher<Key>
    where
        Key: PartialEq,
    {
        extern "rust-call" fn call_mut(&mut self, args: (u32, u32, &Key, &Key)) -> Self::Output {
            self.call_impl(args.0, args.1, args.2, args.3)
        }
    }

    impl<Key> FnOnce<(u32, u32, &Key, &Key)> for KeyEqualityMatcher<Key>
    where
        Key: PartialEq,
    {
        type Output = bool;

        extern "rust-call" fn call_once(self, args: (u32, u32, &Key, &Key)) -> Self::Output {
            self.call_impl(args.0, args.1, args.2, args.3)
        }
    }

    impl<Key> KeyEqualityMatcher<Key>
    where
        Key: PartialEq,
    {
        fn call_impl(&self, _hash1: u32, _hash2: u32, key1: &Key, key2: &Key) -> bool {
            key1 == key2
        }
    }

    pub type PointerTemplateHashMapImpl<AllocationPolicy> = TemplateHashMapImpl<
        *mut std::ffi::c_void,
        *mut std::ffi::c_void,
        KeyEqualityMatcher<*mut std::ffi::c_void>,
        AllocationPolicy,
    >;

    impl<AllocationPolicy> PointerTemplateHashMapImpl<AllocationPolicy>
    where AllocationPolicy: Sized,
    {
        pub fn new(capacity: u32, allocator: AllocationPolicy) -> Self {
            TemplateHashMapImpl::new(capacity, KeyEqualityMatcher::new(), allocator)
        }

        pub fn new_from_original(other: &PointerTemplateHashMapImpl<AllocationPolicy>, allocator: AllocationPolicy) -> Self {
             TemplateHashMapImpl::new_from_original(other, allocator)
        }
    }

    pub type HashMap = PointerTemplateHashMapImpl<DefaultAllocationPolicy>;

    pub struct TemplateHashMap<Key, Value, MatchFun, AllocationPolicy> {
        base: TemplateHashMapImpl<
            *mut std::ffi::c_void,
            *mut std::ffi::c_void,
            HashEqualityThenKeyMatcher<*mut std::ffi::c_void, MatchFun>,
            AllocationPolicy,
        >,
        _phantom: PhantomData<(Key, Value)>,
    }

    impl<Key, Value, MatchFun, AllocationPolicy> TemplateHashMap<Key, Value, MatchFun, AllocationPolicy>
    where
        MatchFun: Fn(&Key, &Key) -> bool + Clone,
        AllocationPolicy: Sized,
    {
        pub fn new(match_: MatchFun, allocator: AllocationPolicy) -> Self {
            assert_eq!(std::mem::size_of::<Key>(), std::mem::size_of::<*mut std::ffi::c_void>());
            assert_eq!(std::mem::size_of::<Value>(), std::mem::size_of::<*mut std::ffi::c_void>());

            TemplateHashMap {
                base: TemplateHashMapImpl::new(
                    TemplateHashMapImpl::K_DEFAULT_HASH_MAP_CAPACITY,
                    HashEqualityThenKeyMatcher::new(move |a, b| {
                        let a_ptr = a as *const Key;
                        let b_ptr = b as *const Key;
                        match_(&unsafe { &*a_ptr }, &unsafe { &*b_ptr })
                    }),
                    allocator,
                ),
                _phantom: PhantomData,
            }
        }

        pub fn begin(&self) -> Iterator<'_, Key, Value, MatchFun, AllocationPolicy> {
            Iterator {
                map_: &self.base,
                entry_: self.base.start(),
                _phantom: PhantomData,
            }
        }

        pub fn end(&self) -> Iterator<'_, Key, Value, MatchFun, AllocationPolicy> {
            Iterator {
                map_: &self.base,
                entry_: std::ptr::null_mut(),
                _phantom: PhantomData,
            }
        }

        pub fn find(
            &mut self,
            key: &Key,
            insert: bool,
        ) -> Iterator<'_, Key, Value, MatchFun, AllocationPolicy> {
            let key_ptr = key as *const Key as *mut std::ffi::c_void;
            let hash = unsafe { &*key_ptr.cast::<Key>() }.hash();
            if insert {
                Iterator {
                    map_: &self.base,
                    entry_: self
                        .base
                        .lookup_or_insert_with_key(
                            key_ptr,
                            hash,
                            || key_ptr,
                            || std::ptr::null_mut(),
                        ),
                    _phantom: PhantomData,
                }
            } else {
                match self.base.lookup(key_ptr, hash) {
                    Some(entry) => Iterator {
                        map_: &self.base,
                        entry_: entry,
                        _phantom: PhantomData,
                    },
                    None => Iterator {
                        map_: &self.base,
                        entry_: std::ptr::null_mut(),
                        _phantom: PhantomData,
                    },
                }
            }
        }
    }

    pub struct Iterator<'a, Key, Value, MatchFun, AllocationPolicy> {
        map_: &'a TemplateHashMapImpl<
            *mut std::ffi::c_void,
            *mut std::ffi::c_void,
            HashEqualityThenKeyMatcher<*mut std::ffi::c_void, MatchFun>,
            AllocationPolicy,
        >,
        entry_: *mut TemplateHashMapEntry<*mut std::ffi::c_void, *mut std::ffi::c_void>,
        _phantom: PhantomData<(Key, Value)>,
    }

    impl<'a, Key, Value, MatchFun, AllocationPolicy> Iterator<'a, Key, Value, MatchFun, AllocationPolicy> {
        pub fn next(&mut self) -> Option<&mut ValueType<Key, Value>> {
            self.entry_ = self.map_.next(self.entry_);
            unsafe {
                if self.entry_.is_null() {
                    None
                } else {
                    Some(&mut *(self.entry_ as *mut ValueType<Key, Value>))
                }
            }
        }
    }

    impl<'a, Key, Value, MatchFun, AllocationPolicy> PartialEq for Iterator<'a, Key, Value, MatchFun, AllocationPolicy> {
        fn eq(&self, other: &Self) -> bool {
            self.entry_ == other.entry_
        }
    }

    impl<'a, Key, Value, MatchFun, AllocationPolicy> Eq for Iterator<'a, Key, Value, MatchFun, AllocationPolicy> {}

    #[repr(C)]
    pub struct ValueType<Key, Value> {
        pub first: *mut Key,
        pub second: *mut Value,
    }

    // Placeholder trait to allow calling .hash() on Key
    pub trait Hashable {
        fn hash(&self) -> u32;
    }
}