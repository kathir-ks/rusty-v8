// Converted from V8 C++ source files:
// Header: zone-containers.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod zone_containers {
    use std::{
        cmp::min,
        collections::{
            BTreeMap, BTreeSet, LinkedList, VecDeque, HashMap, HashSet, BinaryHeap, Vec
        },
        fmt::Debug,
        hash::{Hash, Hasher},
        iter::Iterator,
        mem::MaybeUninit,
        ops::{Deref, DerefMut},
        ptr,
        rc::Rc,
        sync::Mutex,
    };
    use crate::{
        base::{
            hashing,
            intrusive_set::IntrusiveSetIndex,
            small_map::SmallMap,
            small_vector::SmallVector,
        },
        src::zone::zone_allocator::ZoneAllocator,
        src::zone::accounting_allocator::Zone,
    };
    use super::*;

    pub struct ZoneVector<T> {
        zone_: *mut Zone,
        data_: *mut T,
        end_: *mut T,
        capacity_: *mut T,
    }

    impl<T> ZoneVector<T> {
        pub type iterator = *mut T;
        pub type const_iterator = *const T;
        pub type reverse_iterator = std::vec::IntoIter<T>;
        pub type const_reverse_iterator = std::vec::IntoIter<T>;
        pub type value_type = T;
        pub type reference = T;
        pub type const_reference = T;
        pub type size_type = usize;

        // Constructs an empty vector.
        pub fn new(zone: *mut Zone) -> Self {
            ZoneVector {
                zone_: zone,
                data_: ptr::null_mut(),
                end_: ptr::null_mut(),
                capacity_: ptr::null_mut(),
            }
        }

        // Constructs a new vector and fills it with {size} elements, each
        // constructed via the default constructor.
        pub fn with_size(size: usize, zone: *mut Zone) -> Self
        where
            T: Default + Copy,
        {
            let mut vec = ZoneVector::new(zone);
            if size > 0 {
                unsafe {
                    vec.data_ = (*zone).allocate_array::<T>(size);
                    vec.end_ = vec.data_.add(size);
                    vec.capacity_ = vec.data_.add(size);
                    for i in 0..size {
                        let ptr = vec.data_.add(i);
                        ptr::write(ptr, T::default()); // Use ptr::write for placement
                    }
                }
            }
            vec
        }

        // Constructs a new vector and fills it with {size} elements, each
        // having the value {def}.
        pub fn with_size_and_default(size: usize, def: T, zone: *mut Zone) -> Self
        where
            T: Copy,
        {
            let mut vec = ZoneVector::new(zone);
            if size > 0 {
                unsafe {
                    vec.data_ = (*zone).allocate_array::<T>(size);
                    vec.end_ = vec.data_.add(size);
                    vec.capacity_ = vec.data_.add(size);
                    for i in 0..size {
                        let ptr = vec.data_.add(i);
                        ptr::write(ptr, def);
                    }
                }
            }
            vec
        }

        // Constructs a new vector and fills it with the contents of the given
        // initializer list.
        pub fn from_initializer_list(list: &[T], zone: *mut Zone) -> Self
        where
            T: Copy,
        {
            let size = list.len();
            let mut vec = ZoneVector::new(zone);
            if size > 0 {
                unsafe {
                    vec.data_ = (*zone).allocate_array::<T>(size);
                    vec.end_ = vec.data_.add(size);
                    vec.capacity_ = vec.data_.add(size);
                    for i in 0..size {
                        let ptr = vec.data_.add(i);
                        ptr::write(ptr, list[i]);
                    }
                }
            }
            vec
        }

        // Constructs a new vector and fills it with the contents of the range
        // [first, last).
        pub fn from_iter<It>(first: It, last: It, zone: *mut Zone) -> Self
        where
            It: Iterator<Item = T> + Clone,
            T: Copy,
        {
            let mut vec = ZoneVector::new(zone);
            let mut current = first.clone();
            while current.clone().ne(&last) {
                vec.push_back(current.next().unwrap());
            }
            vec
        }

        pub fn copy(other: &ZoneVector<T>) -> Self
        where
            T: Copy,
        {
            let mut new_vec = ZoneVector::new(other.zone_);
            new_vec = other.clone();
            new_vec
        }

        pub fn move_from(other: &mut ZoneVector<T>) -> Self {
            let mut new_vec = ZoneVector::new(other.zone_);
            new_vec = std::mem::take(other);
            new_vec
        }

        pub fn assign_from_copy(&mut self, other: &ZoneVector<T>)
        where
            T: Copy,
        {
            // Self-assignment check (though likely a bug if it occurs)
            if self as *const _ == other as *const _ {
                return;
            }

            let src = other.data_;
            unsafe {
                if self.capacity() >= other.size() && self.zone_ == other.zone_ {
                    let mut dst = self.data_;
                    if std::mem::needs_drop::<T>() {
                        while dst < self.end_ && src < other.end_ {
                            *dst = *src;
                            dst = dst.add(1);
                            src = src.add(1);
                        }
                        while src < other.end_ {
                            self.emplace(dst, *src);
                            dst = dst.add(1);
                            src = src.add(1);
                        }
                        let mut old_end = self.end_;
                        self.end_ = dst;
                        while self.end_ < old_end {
                            self.end_.drop_in_place();
                            self.end_ = self.end_.add(1);
                        }
                    } else {
                        let size = other.size();
                        if size > 0 {
                            ptr::copy_nonoverlapping(src, dst, size * std::mem::size_of::<T>());
                        }
                        self.end_ = dst.add(size);
                    }
                } else {
                    self.clear();
                    if self.data_ != ptr::null_mut() {
                        (*self.zone_).delete_array(self.data_, self.capacity());
                    }
                    let new_cap = other.capacity();
                    if new_cap > 0 {
                        self.data_ = (*self.zone_).allocate_array::<T>(new_cap);
                        self.copy_to_new_storage(self.data_, other.data_, other.end_);
                    } else {
                        self.data_ = ptr::null_mut();
                    }
                    self.capacity_ = self.data_.add(new_cap);
                    self.end_ = self.data_.add(other.size());
                }
            }
        }

        pub fn assign_from_move(&mut self, other: &mut ZoneVector<T>) {
            // Self-assignment check (though likely a bug if it occurs)
            if self as *const _ == other as *const _ {
                return;
            }

            // Different zone move-assignment check (unexpected lifetime semantics)
            if self.zone_ == ptr::null_mut() {
                self.zone_ = other.zone_;
            } else {
                assert_eq!(self.zone_, other.zone_);
            }

            unsafe {
                for p in self.data_..self.end_ {
                    p.drop_in_place();
                }
                if self.data_ != ptr::null_mut() {
                    (*self.zone_).delete_array(self.data_, self.capacity());
                }

                self.data_ = other.data_;
                self.end_ = other.end_;
                self.capacity_ = other.capacity_;

                other.data_ = ptr::null_mut();
                other.end_ = ptr::null_mut();
                other.capacity_ = ptr::null_mut();
            }
        }

        pub fn assign_from_initializer_list(&mut self, ilist: &[T])
        where
            T: Copy,
        {
            self.clear();
            self.ensure_capacity(ilist.len());
            self.copy_to_new_storage(self.data_, ilist.as_ptr(), unsafe {
                ilist.as_ptr().add(ilist.len())
            });
            unsafe {
                self.end_ = self.data_.add(ilist.len());
            }
        }

        pub fn swap(&mut self, other: &mut ZoneVector<T>) {
            assert_eq!(self.zone_, other.zone_);
            std::mem::swap(&mut self.data_, &mut other.data_);
            std::mem::swap(&mut self.end_, &mut other.end_);
            std::mem::swap(&mut self.capacity_, &mut other.capacity_);
        }

        pub fn resize(&mut self, new_size: usize)
        where
            T: Default + Copy,
        {
            self.ensure_capacity(new_size);
            unsafe {
                let mut new_end = self.data_.add(new_size);
                while self.end_ < new_end {
                    self.emplace(self.end_, T::default());
                    self.end_ = self.end_.add(1);
                }
                while new_end < self.end_ {
                    self.end_.drop_in_place();
                    self.end_ = self.end_.sub(1);
                }
            }
        }

        pub fn resize_with_value(&mut self, new_size: usize, value: &T)
        where
            T: Copy,
        {
            self.ensure_capacity(new_size);
            unsafe {
                let mut new_end = self.data_.add(new_size);
                while self.end_ < new_end {
                    self.emplace(self.end_, *value);
                    self.end_ = self.end_.add(1);
                }
                while new_end < self.end_ {
                    self.end_.drop_in_place();
                    self.end_ = self.end_.sub(1);
                }
            }
        }

        pub fn assign(&mut self, new_size: usize, value: &T)
        where
            T: Copy,
        {
            unsafe {
                if self.capacity() >= new_size {
                    let new_end = self.data_.add(new_size);
                    let assignable = self.data_.add(min(self.size(), new_size));

                    let mut p = self.data_;
                    while p < assignable {
                        self.copying_overwrite(p, value);
                        p = p.add(1);
                    }

                    let mut p = assignable;
                    while p < new_end {
                        self.copy_to_new_storage(p, value);
                        p = p.add(1);
                    }

                    let mut p = new_end;
                    while p < self.end_ {
                        p.drop_in_place();
                        p = p.add(1);
                    }

                    self.end_ = new_end;
                } else {
                    self.clear();
                    self.ensure_capacity(new_size);
                    let new_end = self.data_.add(new_size);
                    let mut p = self.data_;

                    while p < new_end {
                        self.emplace(p, *value);
                        p = p.add(1);
                    }

                    self.end_ = new_end;
                }
            }
        }

        pub fn clear(&mut self) {
            unsafe {
                while self.data_ < self.end_ {
                    self.end_.drop_in_place();
                    self.end_ = self.end_.sub(1);
                }
                self.end_ = self.data_;
            }
        }

        pub fn size(&self) -> usize {
            if self.data_ == ptr::null_mut() || self.end_ == ptr::null_mut() {
                return 0;
            }
            unsafe { self.end_.offset_from(self.data_) as usize }
        }

        pub fn empty(&self) -> bool {
            self.end_ == self.data_
        }

        pub fn capacity(&self) -> usize {
            if self.data_ == ptr::null_mut() || self.capacity_ == ptr::null_mut() {
                return 0;
            }
            unsafe { self.capacity_.offset_from(self.data_) as usize }
        }

        pub fn reserve(&mut self, new_cap: usize) {
            self.ensure_capacity(new_cap);
        }

        pub fn data(&mut self) -> *mut T {
            self.data_
        }

        pub fn data_const(&self) -> *const T {
            self.data_ as *const T
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone_
        }

        pub fn at(&mut self, pos: usize) -> &mut T {
            assert!(pos < self.size());
            unsafe { &mut *self.data_.add(pos) }
        }

        pub fn at_const(&self, pos: usize) -> &T {
            assert!(pos < self.size());
            unsafe { &*self.data_.add(pos) }
        }

        pub fn get_mut(&mut self, pos: usize) -> Option<&mut T> {
            if pos < self.size() {
                unsafe { Some(&mut *self.data_.add(pos)) }
            } else {
                None
            }
        }

        pub fn get(&self, pos: usize) -> Option<&T> {
            if pos < self.size() {
                unsafe { Some(&*self.data_.add(pos)) }
            } else {
                None
            }
        }

        pub fn front(&mut self) -> &mut T {
            assert!(self.end_ > self.data_);
            unsafe { &mut *self.data_ }
        }

        pub fn front_const(&self) -> &T {
            assert!(self.end_ > self.data_);
            unsafe { &*self.data_ }
        }

        pub fn back(&mut self) -> &mut T {
            assert!(self.end_ > self.data_);
            unsafe { &mut *self.end_.sub(1) }
        }

        pub fn back_const(&self) -> &T {
            assert!(self.end_ > self.data_);
            unsafe { &*self.end_.sub(1) }
        }

        pub fn begin(&mut self) -> *mut T {
            self.data_
        }

        pub fn begin_const(&self) -> *const T {
            self.data_ as *const T
        }

        pub fn end(&mut self) -> *mut T {
            self.end_
        }

        pub fn end_const(&self) -> *const T {
            self.end_ as *const T
        }

        pub fn push_back(&mut self, value: T)
        where
            T: Copy,
        {
            self.ensure_one_more_capacity();
            unsafe {
                self.emplace(self.end_, value);
                self.end_ = self.end_.add(1);
            }
        }

        pub fn push_back_move(&mut self, value: T) {
            self.emplace_back(value);
        }

        pub fn pop_back(&mut self) {
            assert!(self.end_ > self.data_);
            unsafe {
                self.end_ = self.end_.sub(1);
                self.end_.drop_in_place();
            }
        }

        pub fn emplace_back(&mut self, args: T)
        where
            T: Copy,
        {
            self.ensure_one_more_capacity();
            unsafe {
                let ptr = self.end_;
                ptr::write(ptr, args);
                self.end_ = self.end_.add(1);
            }
        }

        pub fn insert<It>(
            &mut self,
            pos: *const T,
            first: It,
            last: It,
        ) -> *mut T
        where
            It: Iterator<Item = T> + Clone,
            T: Copy,
        {
            unsafe {
                let position;
                let mut first_mut = first.clone();
                if true {
                    // std::is_base_of_v<std::random_access_iterator_tag,
                    // typename std::iterator_traits<It>::iterator_category>
                    let mut count = 0;
                    let mut temp = first_mut.clone();
                    while temp.clone().ne(&last) {
                        count += 1;
                        temp.next();
                    }

                    let mut assignable = 0;
                    position = self.prepare_for_insertion(pos, count, &mut assignable);
                    if std::mem::needs_drop::<T>() {
                        self.copying_overwrite(position, first_mut.clone(), last);
                    } else {
                        if count > 0 {
                            let mut i = 0;
                            while first_mut.clone().ne(&last) {
                                let ptr = position.add(i);
                                ptr::write(ptr, first_mut.next().unwrap());
                                i += 1;
                            }
                        }
                    }
                } else if pos == self.end() as *const T {
                    position = self.end_;
                    while first_mut.clone().ne(&last) {
                        self.ensure_one_more_capacity();
                        self.emplace(self.end_, first_mut.next().unwrap());
                        self.end_ = self.end_.add(1);
                    }
                } else {
                    panic!("Unimplemented");
                }
                return position;
            }
        }

        pub fn insert_with_value(&mut self, pos: *const T, count: usize, value: &T) -> *mut T
        where
            T: Copy,
        {
            unsafe {
                let mut assignable = 0;
                let position = self.prepare_for_insertion(pos, count, &mut assignable);
                let mut dst = position;
                let stop = position.add(assignable);
                while dst < stop {
                    self.copying_overwrite(dst, value);
                    dst = dst.add(1);
                }

                let mut dst = position.add(assignable);
                let stop = position.add(count);
                while dst < stop {
                    self.emplace(dst, *value);
                    dst = dst.add(1);
                }

                return position;
            }
        }

        pub fn erase(&mut self, pos: *const T) -> *mut T {
            assert!(self.data_ as *const T <= pos && pos <= self.end_ as *const T);
            if pos == self.end_ as *const T {
                return pos as *mut T;
            }
            return self.erase_range(pos, 1);
        }

        pub fn erase_range(&mut self, first: *const T, last: *const T) -> *mut T {
            assert!(
                self.data_ as *const T <= first
                    && first <= last
                    && last <= self.end_ as *const T
            );
            if first == last {
                return first as *mut T;
            }
            return self.erase_range(first, unsafe { last.offset_from(first) as usize });
        }

        const MIN_CAPACITY: usize = 2;

        fn new_capacity(&self, minimum: usize) -> usize {
            let new_capacity = if self.data_ == self.capacity_ {
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
            if self.end_ < self.capacity_ {
                return;
            }
            self.grow(self.capacity() + 1);
        }

        #[inline]
        fn ensure_capacity(&mut self, minimum: usize) {
            if minimum <= self.capacity() {
                return;
            }
            self.grow(minimum);
        }

        #[inline]
        unsafe fn copy_to_new_storage(&mut self, dst: *mut T, src: *const T)
        where
            T: Copy,
        {
            self.emplace(dst, *src);
        }

        #[inline]
        unsafe fn move_to_new_storage(&mut self, dst: *mut T, src: *mut T)
        where
            T: Copy,
        {
            if true {
                //std::is_move_constructible_v::<T>()
                self.emplace(dst, *src);
            } else {
                self.copy_to_new_storage(dst, src);
            }
        }

        #[inline]
        unsafe fn copying_overwrite(&mut self, dst: *mut T, src: *const T)
        where
            T: Copy,
        {
            if true {
                //std::is_copy_assignable_v::<T>()
                *dst = *src;
            } else {
                dst.drop_in_place();
                self.copy_to_new_storage(dst, src);
            }
        }

        #[inline]
        unsafe fn moving_overwrite(&mut self, dst: *mut T, src: *mut T)
        where
            T: Copy,
        {
            if true {
                //std::is_move_assignable_v::<T>()
                *dst = *src;
            } else {
                self.copying_overwrite(dst, src);
            }
        }

        unsafe fn copy_to_new_storage_range(&mut self, dst: *mut T, src: *const T, src_end: *const T)
        where
            T: Copy,
        {
            if std::mem::needs_drop::<T>() {
                let mut current_src = src;
                let mut current_dst = dst;
                while current_src < src_end {
                    self.copy_to_new_storage(current_dst, current_src);
                    current_src = current_src.add(1);
                    current_dst = current_dst.add(1);
                }
            } else {
                let count = src_end.offset_from(src) as usize;
                if count > 0 {
                    ptr::copy_nonoverlapping(src, dst, count * std::mem::size_of::<T>());
                }
            }
        }

        unsafe fn move_to_new_storage_range(&mut self, dst: *mut T, src: *mut T, src_end: *const T)
        where
            T: Copy,
        {
            if std::mem::needs_drop::<T>() {
                let mut current_src = src;
                let mut current_dst = dst;
                while current_src < src_end as *mut T {
                    self.move_to_new_storage(current_dst, current_src);
                    current_src.drop_in_place();
                    current_src = current_src.add(1);
                    current_dst = current_dst.add(1);
                }
            } else {
                let count = src_end.offset_from(src) as usize;
                if count > 0 {
                    ptr::copy_nonoverlapping(src, dst, count * std::mem::size_of::<T>());
                }
            }
        }

        unsafe fn copying_overwrite_range(&mut self, dst: *mut T, src: *const T, src_end: *const T)
        where
            T: Copy,
        {
            if std::mem::needs_drop::<T>() {
                let mut current_src = src;
                let mut current_dst = dst;
                while current_src < src_end {
                    self.copying_overwrite(current_dst, current_src);
                    current_src = current_src.add(1);
                    current_dst = current_dst.add(1);
                }
            } else {
                let count = src_end.offset_from(src) as usize;
                if count > 0 {
                    ptr::copy_nonoverlapping(src, dst, count * std::mem::size_of::<T>());
                }
            }
        }

        unsafe fn moving_overwrite_range(&mut self, dst: *mut T, src: *mut T, src_end: *const T)
        where
            T: Copy,
        {
            if std::mem::needs_drop::<T>() {
                let mut current_src = src;
                let mut current_dst = dst;
                while current_src < src_end as *mut T {
                    self.moving_overwrite(current_dst, current_src);
                    current_src = current_src.add(1);
                    current_dst = current_dst.add(1);
                }
            } else {
                let count = src_end.offset_from(src) as usize;
                if count > 0 {
                    ptr::copy_nonoverlapping(src, dst, count * std::mem::size_of::<T>());
                }
            }
        }

        #[no_mangle]
        #[inline(never)]
        unsafe fn grow(&mut self, minimum: usize)
        where
            T: Copy,
        {
            let old_data = self.data_;
            let old_end = self.end_;
            let old_size = self.size();
            let new_capacity = self.new_capacity(minimum);
            self.data_ = (*self.zone_).allocate_array::<T>(new_capacity);
            self.end_ = self.data_.add(old_size);
            if old_data != ptr::null_mut() {
                self.move_to_new_storage_range(self.data_, old_data, old_end as *const T);
                (*self.zone_).delete_array(old_data, self.capacity_ as usize);
            }
            self.capacity_ = self.data_.add(new_capacity);
        }

        fn prepare_for_insertion(&mut self, pos: *const T, count: usize, assignable: &mut usize) -> *mut T
        where
            T: Copy,
        {
            assert!(self.data_ as *const T <= pos && pos <= self.end_ as *const T);
            assert!(usize::MAX - self.size() >= count);
            unsafe {
                let index = pos.offset_from(self.data_ as *const T) as usize;
                let to_shift = self.end().offset_from(pos) as usize;
                assert_eq!(index + to_shift, self.size());

                if self.capacity() < self.size() + count {
                    *assignable = 0;
                    let old_data = self.data_;
                    let old_end = self.end_;
                    let old_size = self.size();
                    let new_capacity = self.new_capacity(old_size + count);
                    self.data_ = (*self.zone_).allocate_array::<T>(new_capacity);
                    self.end_ = self.data_.add(old_size + count);

                    if old_data != ptr::null_mut() {
                        self.move_to_new_storage_range(self.data_, old_data, pos);
                        self.move_to_new_storage_range(
                            self.data_.add(index + count),
                            pos as *mut T,
                            old_end as *const T,
                        );
                        (*self.zone_).delete_array(old_data, self.capacity() as usize);
                    }
                    self.capacity_ = self.data_.add(new_capacity);
                } else {
                    let assignable_slots = min(to_shift, count);
                    *assignable = assignable_slots;
                    if !std::mem::needs_drop::<T>() {
                        if to_shift > 0 {
                            ptr::copy(
                                pos,
                                self.data_.add(index + count),
                                to_shift * std::mem::size_of::<T>(),
                            );
                        }
                        self.end_ = self.end_.add(count);
                        return self.data_.add(index);
                    }

                    let mut dst = self.end_.add(count);
                    let mut src = self.end_;
                    let mut stop = dst.sub(assignable_slots);
                    while dst > stop {
                        src = src.sub(1);
                        dst = dst.sub(1);
                        self.move_to_new_storage(dst, src);
                    }

                    let mut dst = self.end_.add(count) ;
                    let mut src = self.end_ ;

                    while src > pos as *mut T {
                        src = src.sub(1);
                        dst = dst.sub(1);
                        self.moving_overwrite(dst, src);
                    }
                    self.end_ = self.end_.add(count);
                }
                return self.data_.add(index);
            }
        }

        fn erase_range(&mut self, first: *const T, count: usize) -> *mut T
        where
            T: Copy,
        {
            assert!(self.data_ as *const T <= first && first <= self.end_ as *const T);
            assert!(count <= unsafe { self.end().offset_from(first) } as usize);

            let position = first as *mut T;
            unsafe {
                self.moving_overwrite_range(position, position.add(count), self.end_);
                let old_end = self.end_;
                self.end_ = self.end_.sub(count);
                let mut p = self.end_;

                while p < old_end {
                    p.drop_in_place();
                    p = p.add(1);
                }
            }
            return position;
        }

        #[inline]
        unsafe fn emplace(&mut self, target: *mut T, args: T)
        where
            T: Copy,
        {
            ptr::write(target, args);
        }
    }

    impl<T: Copy> Clone for ZoneVector<T> {
        fn clone(&self) -> Self {
            let mut new_vec = ZoneVector::new(self.zone_);
             unsafe {
                if self.size() > 0 {
                    new_vec.data_ = (*self.zone_).allocate_array::<T>(self.size());
                    new_vec.end_ = new_vec.data_.add(self.size());
                    new_vec.capacity_ = new_vec.data_.add(self.size());
                    for i in 0..self.size() {
                        let ptr = new_vec.data_.add(i);
                        ptr::write(ptr, *self.data_.add(i));
                    }
                }
            }
            new_vec
        }
    }

    impl<T> Drop for ZoneVector<T> {
        fn drop(&mut self) {
            unsafe {
                if !self.data_.is_null() {
                    for i in 0..self.size() {
                        self.data_.add(i).drop_in_place();
                    }
                    (*self.zone_).delete_array(self.data_, self.capacity());
                    self.data_ = ptr::null_mut();
                    self.end_ = ptr::null_mut();
                    self.capacity_ = ptr::null_mut();
                }
            }
        }
    }

    impl<T: PartialEq> PartialEq for ZoneVector<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.size() != other.size() {
                return false;
            }
            unsafe {
                for i in 0..self.size() {
                    if *self.data_.add(i) != *other.data_.add(i) {
                        return false;
                    }
                }
            }
            true
        }
    }

    impl<T: PartialOrd> PartialOrd for ZoneVector<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let mut i = 0;
            while i < self.size() && i < other.size() {
                unsafe {
                    match (*self.data_.add(i)).partial_cmp(&*other.data_.add(i)) {
                        Some(std::cmp::Ordering::Equal) => {}
                        other => return other,
                    }
                }
                i += 1;
            }
            if self.size() < other.size() {
                return Some(std::cmp::Ordering::Less);
            } else if self.size() > other.size() {
                return Some(std::cmp::Ordering::Greater);
            }
            Some(std::cmp::Ordering::Equal)
        }
    }

    pub struct ZoneIntrusiveSet<T, GetIntrusiveSetIndex> {
        inner: base::IntrusiveSet<T, GetIntrusiveSetIndex, ZoneVector<T>>,
    }

    impl<T, GetIntrusiveSetIndex> ZoneIntrusiveSet<T, GetIntrusiveSetIndex>
    where
        GetIntrusiveSetIndex: IntrusiveSetIndex<T>,
    {
        pub fn new(zone: *mut Zone
