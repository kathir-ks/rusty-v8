// Converted from V8 C++ source files:
// Header: region-allocator.h
// Implementation: region-allocator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::cmp::{max, min};
    use std::collections::HashSet;
    use std::fmt;
    use std::mem::MaybeUninit;
    use std::ops::Deref;
    use std::ptr;
    use std::sync::{Arc, Mutex, RwLock};
    use std::{
        cell::{Ref, RefCell, RefMut},
        rc::Rc,
    };

    use crate::base::bits::IsPowerOfTwo;
    use crate::base::logging::CHECK_LT;
    use crate::base::macros::{CHECK, CHECK_EQ, CHECK_GE, CHECK_NE, DCHECK};
    use crate::base::utils::RandomNumberGenerator;

    pub struct RegionAllocator {
        whole_region_: Region,
        region_size_in_pages_: usize,
        max_load_for_randomization_: usize,
        free_size_: usize,
        page_size_: usize,
        all_regions_: Mutex<AllRegionsSet>,
        free_regions_: Mutex<FreeRegionsSet>,
        on_split_: Option<SplitMergeCallback>,
        on_merge_: Option<SplitMergeCallback>,
    }

    type SplitMergeCallback = Box<dyn Fn(usize, usize) + Send + Sync>;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum RegionState {
        kFree,
        kExcluded,
        kAllocated,
    }

    impl fmt::Display for RegionState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RegionState::kFree => write!(f, "free"),
                RegionState::kExcluded => write!(f, "excluded"),
                RegionState::kAllocated => write!(f, "used"),
            }
        }
    }

    const K_MAX_LOAD_FACTOR_FOR_RANDOMIZATION: f64 = 0.40;
    const K_MAX_RANDOMIZATION_ATTEMPTS: i32 = 3;

    impl RegionAllocator {
        pub const K_ALLOCATION_FAILURE: usize = usize::MAX;

        pub fn new(address: usize, size: usize, page_size: usize) -> RegionAllocator {
            CHECK_LT!(address, address + size);
            CHECK!(IsPowerOfTwo(page_size));
            CHECK!(is_aligned(size, page_size));
            CHECK!(is_aligned(address, page_size));
            let whole_region_ = Region::new(address, size, RegionState::kFree);
            let region_size_in_pages_ = size / page_size;
            let max_load_for_randomization_ =
                (size as f64 * K_MAX_LOAD_FACTOR_FOR_RANDOMIZATION) as usize;
            let free_size_ = 0;
            let page_size_ = page_size;
            let region = Box::new(Region::new(
                whole_region_.address,
                whole_region_.size,
                whole_region_.state,
            ));
            let mut all_regions_set = AllRegionsSet::new();
            all_regions_set.insert(Box::into_raw(region) as *mut Region);
            let all_regions_ = Mutex::new(all_regions_set);

            let mut free_regions_set = FreeRegionsSet::new();
            let region = Box::new(Region::new(
                whole_region_.address,
                whole_region_.size,
                whole_region_.state,
            ));
            free_regions_set.insert(Box::into_raw(region) as *mut Region);
            let free_regions_ = Mutex::new(free_regions_set);

            let mut region_allocator = RegionAllocator {
                whole_region_: whole_region_,
                region_size_in_pages_: region_size_in_pages_,
                max_load_for_randomization_: max_load_for_randomization_,
                free_size_: free_size_,
                page_size_: page_size_,
                all_regions_: all_regions_,
                free_regions_: free_regions_,
                on_split_: None,
                on_merge_: None,
            };

            let region = region_allocator
                .all_regions_
                .lock()
                .unwrap()
                .iter()
                .next()
                .cloned();
            if let Some(r) = region {
                region_allocator.FreeListAddRegion(unsafe { &mut *r });
            }

            region_allocator
        }

        pub fn set_on_split_callback(&mut self, callback: SplitMergeCallback) {
            self.on_split_ = Some(callback);
        }

        pub fn set_on_merge_callback(&mut self, callback: SplitMergeCallback) {
            self.on_merge_ = Some(callback);
        }

        pub fn AllocateRegion(&mut self, size: usize) -> usize {
            DCHECK_NE!(size, 0);
            DCHECK!(is_aligned(size, self.page_size_));

            let region = self.FreeListFindRegion(size);
            if region.is_none() {
                return RegionAllocator::K_ALLOCATION_FAILURE;
            }

            let mut region = unsafe { &mut *region.unwrap() };

            if region.size() != size {
                self.Split(region, size);

                let region = self.FreeListFindRegion(size);
                if region.is_none() {
                    return RegionAllocator::K_ALLOCATION_FAILURE;
                }
                let mut region = unsafe { &mut *region.unwrap() };

                DCHECK!(is_aligned(region.begin(), self.page_size_));
                DCHECK_EQ!(region.size(), size);

                self.FreeListRemoveRegion(region);
                region.set_state(RegionState::kAllocated);
                return region.begin();
            }

            DCHECK!(is_aligned(region.begin(), self.page_size_));
            DCHECK_EQ!(region.size(), size);

            self.FreeListRemoveRegion(region);
            region.set_state(RegionState::kAllocated);
            region.begin()
        }

        pub fn AllocateRegion_random(
            &mut self,
            rng: &mut RandomNumberGenerator,
            size: usize,
        ) -> usize {
            if self.free_size() >= self.max_load_for_randomization_ {
                let mut random: [u8; 8] = [0; 8];

                for _i in 0..K_MAX_RANDOMIZATION_ATTEMPTS {
                    rng.NextBytes(&mut random, 8);
                    let random_offset =
                        self.page_size_ * ((random[0] as usize) % self.region_size_in_pages_);
                    let address = self.begin() + random_offset;

                    if self.AllocateRegionAt(address, size, RegionState::kAllocated) {
                        return address;
                    }
                }
            }

            self.AllocateRegion(size)
        }

        pub fn AllocateRegionAt(
            &mut self,
            requested_address: usize,
            size: usize,
            region_state: RegionState,
        ) -> bool {
            DCHECK!(is_aligned(requested_address, self.page_size_));
            DCHECK_NE!(size, 0);
            DCHECK!(is_aligned(size, self.page_size_));
            DCHECK_NE!(region_state, RegionState::kFree);

            let requested_end = requested_address + size;
            DCHECK_LE!(requested_end, self.end());

            let region_iter = self.FindRegion(requested_address);
            if region_iter.is_none() {
                return false;
            }

            let region_iter = region_iter.unwrap();
            let region = unsafe { &mut *region_iter };

            if !region.is_free() || region.end() < requested_end {
                return false;
            }

            if region.begin() != requested_address {
                let new_size = requested_address - region.begin();
                DCHECK!(is_aligned(new_size, self.page_size_));
                self.Split(region, new_size);

                let region_iter = self.FindRegion(requested_address);
                if region_iter.is_none() {
                    return false;
                }
                let region_iter = region_iter.unwrap();
                let region = unsafe { &mut *region_iter };
            }

            if region.end() != requested_end {
                self.Split(region, size);
            }

            DCHECK_EQ!(region.begin(), requested_address);
            DCHECK_EQ!(region.size(), size);

            self.FreeListRemoveRegion(region);
            region.set_state(region_state);
            true
        }

        pub fn AllocateAlignedRegion(&mut self, size: usize, alignment: usize) -> usize {
            DCHECK!(is_aligned(size, self.page_size_));
            DCHECK!(is_aligned(alignment, self.page_size_));
            DCHECK_GE!(alignment, self.page_size_);

            let padded_size = size + alignment - self.page_size_;
            let region = self.FreeListFindRegion(padded_size);

            let mut region = match region {
                Some(r) => unsafe { &mut *r },
                None => {
                    let region = self.FreeListFindRegion(size);
                    match region {
                        Some(r) => unsafe { &mut *r },
                        None => return RegionAllocator::K_ALLOCATION_FAILURE,
                    }
                }
            };

            if !is_aligned(region.begin(), alignment) {
                let start = round_up(region.begin(), alignment);
                if start + size > region.end() {
                    return RegionAllocator::K_ALLOCATION_FAILURE;
                }
                self.Split(region, start - region.begin());
                DCHECK_EQ!(region.begin(), start);
                DCHECK!(is_aligned(region.begin(), alignment));
            }

            if region.size() != size {
                self.Split(region, size);
            }

            DCHECK!(is_aligned(region.begin(), alignment));
            DCHECK_EQ!(region.size(), size);

            self.FreeListRemoveRegion(region);
            region.set_state(RegionState::kAllocated);
            region.begin()
        }

        pub fn AllocateRegion_hint(
            &mut self,
            hint: usize,
            size: usize,
            alignment: usize,
        ) -> usize {
            DCHECK!(is_aligned(alignment, self.page_size()));
            DCHECK!(is_aligned(hint, alignment));

            if hint != 0 && self.contains(hint, size) {
                if self.AllocateRegionAt(hint, size, RegionState::kAllocated) {
                    return hint;
                }
            }

            if alignment <= self.page_size() {
                return self.AllocateRegion(size);
            } else {
                return self.AllocateAlignedRegion(size, alignment);
            }
        }

        pub fn FreeRegion(&mut self, address: usize) -> usize {
            self.TrimRegion(address, 0)
        }

        pub fn TrimRegion(&mut self, address: usize, new_size: usize) -> usize {
            DCHECK!(is_aligned(new_size, self.page_size_));

            let region_iter = self.FindRegion(address);

            if region_iter.is_none() {
                return 0;
            }

            let mut region_iter = region_iter.unwrap();
            let region = unsafe { &mut *region_iter };

            if region.begin() != address || !region.is_allocated() {
                return 0;
            }

            let locked_free_regions = self.free_regions_.lock().unwrap();
            if locked_free_regions.contains(region) {
                unreachable!();
            }
            drop(locked_free_regions);

            if new_size > 0 {
                self.Split(region, new_size);
                let region_iter = self.FindRegion(address);

                if region_iter.is_none() {
                    return 0;
                }

                let mut region_iter = region_iter.unwrap();
                let region = unsafe { &mut *region_iter };

                region_iter = self.FindRegion(address).unwrap();
            }

            let size = region.size();
            region.set_state(RegionState::kFree);

            let next_iter = {
                let mut all_regions_guard = self.all_regions_.lock().unwrap();
                let mut iter = all_regions_guard.iter();
                let mut found = false;
                let mut next_region = None;

                for r in iter {
                    if *r == region_iter {
                        found = true;
                        continue;
                    }
                    if found {
                        next_region = Some(*r);
                        break;
                    }
                }
                next_region
            };

            if region.end() != self.whole_region_.end() {
                if next_iter.is_none() {
                    unreachable!();
                }
                let next_iter = next_iter.unwrap();
                let next = unsafe { &mut *next_iter };

                if next.is_free() {
                    self.FreeListRemoveRegion(next);
                    let region = unsafe { &mut *region_iter };
                    self.Merge(region_iter, next_iter);
                }
            }

            if new_size == 0 && region.begin() != self.whole_region_.begin() {
                let prev_iter = {
                    let mut all_regions_guard = self.all_regions_.lock().unwrap();
                    let mut iter = all_regions_guard.iter();
                    let mut prev_region = None;

                    for r in iter {
                        if *r == region_iter {
                            break;
                        }
                        prev_region = Some(*r);
                    }
                    prev_region
                };

                if prev_iter.is_none() {
                    unreachable!();
                }

                let prev_iter = prev_iter.unwrap();
                let prev = unsafe { &mut *prev_iter };

                if prev.is_free() {
                    self.FreeListRemoveRegion(prev);
                    let region = unsafe { &mut *region_iter };
                    self.Merge(prev_iter, region_iter);

                    self.FreeListAddRegion(unsafe { &mut *prev_iter });
                    return size;
                }
            }

            self.FreeListAddRegion(region);
            size
        }

        pub fn CheckRegion(&self, address: usize) -> usize {
            let region_iter = self.FindRegion(address);

            if region_iter.is_none() {
                return 0;
            }

            let mut region_iter = region_iter.unwrap();
            let region = unsafe { &mut *region_iter };

            if region.begin() != address || region.is_free() {
                return 0;
            }
            region.size()
        }

        pub fn IsFree(&self, address: usize, size: usize) -> bool {
            CHECK!(self.contains(address, size));
            let region_iter = self.FindRegion(address);

            if region_iter.is_none() {
                return true;
            }

            let mut region_iter = region_iter.unwrap();
            let region = unsafe { &mut *region_iter };

            region.is_free() && region.contains(address, size)
        }

        pub fn begin(&self) -> usize {
            self.whole_region_.begin()
        }

        pub fn end(&self) -> usize {
            self.whole_region_.end()
        }

        pub fn size(&self) -> usize {
            self.whole_region_.size()
        }

        pub fn contains(&self, address: usize) -> bool {
            self.whole_region_.contains(address)
        }

        pub fn contains_size(&self, address: usize, size: usize) -> bool {
            self.whole_region_.contains_size(address, size)
        }

        pub fn free_size(&self) -> usize {
            self.free_size_
        }

        pub fn page_size(&self) -> usize {
            self.page_size_
        }

        fn FindRegion(&self, address: usize) -> Option<*mut Region> {
            if !self.whole_region_.contains(address) {
                return None;
            }
            let key = Region::new(address, 0, RegionState::kFree);
            let all_regions_guard = self.all_regions_.lock().unwrap();
            for region in all_regions_guard.iter() {
                let region_ptr = *region;
                let r = unsafe { &mut *region_ptr };
                if r.contains(address) {
                    return Some(*region);
                }
            }
            None
        }

        fn FreeListAddRegion(&mut self, region: *mut Region) {
            let region_ref = unsafe { &mut *region };
            self.free_size_ += region_ref.size();
            let mut locked_free_regions = self.free_regions_.lock().unwrap();
            locked_free_regions.insert(region);
        }

        fn FreeListFindRegion(&self, size: usize) -> Option<*mut Region> {
            let key = Region::new(0, size, RegionState::kFree);
            let locked_free_regions = self.free_regions_.lock().unwrap();
            for region in locked_free_regions.iter() {
                let region_ptr = *region;
                let r = unsafe { &mut *region_ptr };
                if r.size() >= size {
                    return Some(*region);
                }
            }
            None
        }

        fn FreeListRemoveRegion(&mut self, region: *mut Region) {
            let region_ref = unsafe { &mut *region };
            DCHECK!(region_ref.is_free());
            let mut locked_free_regions = self.free_regions_.lock().unwrap();

            if !locked_free_regions.contains(region) {
                unreachable!()
            }

            DCHECK_LE!(region_ref.size(), self.free_size_);
            self.free_size_ -= region_ref.size();
            locked_free_regions.remove(region);
        }

        fn Split(&mut self, region: *mut Region, new_size: usize) -> *mut Region {
            let region_ref = unsafe { &mut *region };
            DCHECK!(is_aligned(new_size, self.page_size_));
            DCHECK_NE!(new_size, 0);
            DCHECK_GT!(region_ref.size(), new_size);

            if let Some(ref on_split) = self.on_split_ {
                on_split(region_ref.begin(), new_size);
            }

            DCHECK!(!region_ref.is_excluded());
            let state = region_ref.state();

            let new_region = Box::new(Region::new(
                region_ref.begin() + new_size,
                region_ref.size() - new_size,
                state,
            ));

            if state == RegionState::kFree {
                self.FreeListRemoveRegion(region);
            }

            region_ref.set_size(new_size);

            let new_region_ptr = Box::into_raw(new_region) as *mut Region;
            let mut locked_all_regions = self.all_regions_.lock().unwrap();
            locked_all_regions.insert(new_region_ptr);

            if state == RegionState::kFree {
                self.FreeListAddRegion(region);
                self.FreeListAddRegion(new_region_ptr);
            }

            new_region_ptr
        }

        fn Merge(&mut self, prev_iter: *mut Region, next_iter: *mut Region) {
            let prev = unsafe { &mut *prev_iter };
            let next = unsafe { &mut *next_iter };

            DCHECK_EQ!(prev.end(), next.begin());

            if let Some(ref on_merge) = self.on_merge_ {
                on_merge(prev.begin(), prev.size() + next.size());
            }

            prev.set_size(prev.size() + next.size());

            let mut locked_all_regions = self.all_regions_.lock().unwrap();
            if !locked_all_regions.contains(&next_iter) {
                unreachable!()
            }
            locked_all_regions.remove(&next_iter);

            let mut locked_free_regions = self.free_regions_.lock().unwrap();
            if locked_free_regions.contains(next) {
                unreachable!()
            }

            unsafe {
                drop(Box::from_raw(next_iter));
            };
        }

        pub fn Print(&self, os: &mut std::fmt::Write) -> fmt::Result {
            let flags = 0;
            writeln!(os, "RegionAllocator: [0x{:x}, 0x{:x})", self.begin(), self.end())?;
            writeln!(os, "size: {}", self.size())?;
            writeln!(os, "free_size: {}", self.free_size())?;
            writeln!(os, "page_size: {}", self.page_size())?;

            writeln!(os, "all regions: ")?;
            let locked_all_regions = self.all_regions_.lock().unwrap();
            for region in locked_all_regions.iter() {
                let region_ptr = *region;
                let r = unsafe { &mut *region_ptr };
                write!(os, "  ")?;
                r.Print(os)?;
                writeln!(os)?;
            }
            drop(locked_all_regions);

            writeln!(os, "free regions: ")?;
            let locked_free_regions = self.free_regions_.lock().unwrap();
            for region in locked_free_regions.iter() {
                let region_ptr = *region;
                let r = unsafe { &mut *region_ptr };
                write!(os, "  ")?;
                r.Print(os)?;
                writeln!(os)?;
            }
            drop(locked_free_regions);

            writeln!(os)?;
            Ok(())
        }
    }

    impl Drop for RegionAllocator {
        fn drop(&mut self) {
            let mut all_regions_guard = self.all_regions_.lock().unwrap();
            for region in all_regions_guard.drain() {
                unsafe {
                    drop(Box::from_raw(region));
                }
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    struct Region {
        address: usize,
        size: usize,
        state: RegionState,
    }

    impl Region {
        fn new(address: usize, size: usize, state: RegionState) -> Region {
            Region {
                address: address,
                size: size,
                state: state,
            }
        }

        fn is_free(&self) -> bool {
            self.state == RegionState::kFree
        }

        fn is_allocated(&self) -> bool {
            self.state == RegionState::kAllocated
        }

        fn is_excluded(&self) -> bool {
            self.state == RegionState::kExcluded
        }

        fn state(&self) -> RegionState {
            self.state
        }

        fn set_state(&mut self, state: RegionState) {
            self.state = state;
        }

        fn begin(&self) -> usize {
            self.address
        }

        fn end(&self) -> usize {
            self.address + self.size
        }

        fn size(&self) -> usize {
            self.size
        }

        fn set_size(&mut self, size: usize) {
            self.size = size;
        }

        fn contains(&self, address: usize) -> bool {
            address >= self.address && address < self.address + self.size
        }

        fn contains_size(&self, address: usize, size: usize) -> bool {
            address >= self.address && (address + size) <= (self.address + self.size)
        }

        fn Print(&self, os: &mut std::fmt::Write) -> fmt::Result {
            writeln!(
                os,
                "[0x{:x}, 0x{:x}), size: {}, {}",
                self.begin(),
                self.end(),
                self.size(),
                self.state()
            )
        }
    }

    impl PartialEq for Region {
        fn eq(&self, other: &Self) -> bool {
            self.address == other.address && self.size == other.size
        }
    }

    impl Eq for Region {}

    #[derive(Default)]
    struct AddressEndOrder {}

    impl AddressEndOrder {
        fn new() -> Self {
            AddressEndOrder {}
        }

        fn compare(&self, a: &Region, b: &Region) -> std::cmp::Ordering {
            a.end().cmp(&b.end())
        }
    }

    type AllRegionsSet = HashSet<*mut Region>;

    #[derive(Default)]
    struct SizeAddressOrder {}

    impl SizeAddressOrder {
        fn new() -> Self {
            SizeAddressOrder {}
        }

        fn compare(&self, a: &Region, b: &Region) -> std::cmp::Ordering {
            if a.size() != b.size() {
                return a.size().cmp(&b.size());
            }
            return a.begin().cmp(&b.begin());
        }
    }

    type FreeRegionsSet = HashSet<*mut Region>;

    fn is_aligned(value: usize, alignment: usize) -> bool {
        value & (alignment - 1) == 0
    }

    fn round_up(value: usize, alignment: usize) -> usize {
        (value + alignment - 1) & !(alignment - 1)
    }

    pub mod bits {
        pub fn IsPowerOfTwo(x: usize) -> bool {
            (x != 0) && ((x & (x - 1)) == 0)
        }
    }

    pub mod logging {
        #[macro_export]
        macro_rules! CHECK_LT {
            ($x:expr, $y:expr) => {
                if !($x < $y) {
                    panic!("Check failed: {} < {}", $x, $y);
                }
            };
        }

        #[macro_export]
        macro_rules! CHECK {
            ($x:expr) => {
                if !($x) {
                    panic!("Check failed: {}", stringify!($x));
                }
            };
        }
        #[macro_export]
        macro_rules! CHECK_EQ {
            ($x:expr, $y:expr) => {
                if !($x == $y) {
                    panic!("Check failed: {} == {}", $x, $y);
                }
            };
        }
        #[macro_export]
        macro_rules! CHECK_GE {
            ($x:expr, $y:expr) => {
                if !($x >= $y) {
                    panic!("Check failed: {} >= {}", $x, $y);
                }
            };
        }
        #[macro_export]
        macro_rules! CHECK_NE {
            ($x:expr, $y:expr) => {
                if !($x != $y) {
                    panic!("Check failed: {} != {}", $x, $y);
                }
            };
        }

        #[macro_export]
        macro_rules! DCHECK {
            ($x:expr) => {
                if cfg!(debug_assertions) {
                    if !($x) {
                        panic!("DCheck failed: {}", stringify!($x));
                    }
                }
            };
        }
    }

    pub mod macros {
        #[macro_export]
        macro_rules! UNREACHABLE {
            () => {
                panic!("UNREACHABLE");
            };
        }
    }

    pub mod utils {
        pub struct RandomNumberGenerator {
            seed: u64,
        }

        impl RandomNumberGenerator {
            pub fn new() -> Self {
                RandomNumberGenerator { seed: 0 }
            }

            pub fn NextBytes(&mut self, buffer: &mut [u8], size: usize) {
                for i in 0..size {
                    self.seed = self.seed.wrapping_mul(6364136223846793005);
                    buffer[i] = (self.seed >> 58) as u8;
                }
            }
        }
    }
}
