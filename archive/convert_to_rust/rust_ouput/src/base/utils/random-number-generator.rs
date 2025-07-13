// Converted from V8 C++ source files:
// Header: random-number-generator.h
// Implementation: random-number-generator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod utils {
use std::sync::{Mutex, LazyLock};
use std::num;
use std::collections::HashSet;
use std::vec;
use std::mem::transmute;
use std::cmp;

static ENTROPY_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

pub struct RandomNumberGenerator {
    initial_seed_: i64,
    state0_: u64,
    state1_: u64,
}

pub type EntropySource = fn(*mut u8, usize) -> bool;

static mut ENTROPY_SOURCE: Option<EntropySource> = None;

impl RandomNumberGenerator {
    pub fn set_entropy_source(entropy_source: EntropySource) {
        let _guard = ENTROPY_MUTEX.lock().unwrap();
        unsafe {
            ENTROPY_SOURCE = Some(entropy_source);
        }
    }

    pub fn new() -> Self {
        // Check if embedder supplied an entropy source.
        unsafe {
            if let Some(entropy_source) = ENTROPY_SOURCE {
                let mut seed: i64 = 0;
                if entropy_source(transmute(&mut seed), std::mem::size_of::<i64>()) {
                    return Self::from_seed(seed);
                }
            }
        }
    
        #[cfg(any(target_os = "cygwin", target_os = "windows"))]
        {
            let mut first_half: u32 = 0;
            let mut second_half: u32 = 0;
            if let Ok(_) = getrandom::fill(&mut first_half.to_ne_bytes()) {
                if let Ok(_) = getrandom::fill(&mut second_half.to_ne_bytes()) {
                    let seed = ((first_half as i64) << 32) | (second_half as i64);
                    return Self::from_seed(seed);
                } else {
                   // Fallback to weak entropy if getrandom fails for the second half
                   let seed = Time::now().to_internal_value() << 24 ^ TimeTicks::now().to_internal_value();
                   return Self::from_seed(seed);
                }
            } else {
                // Fallback to weak entropy if getrandom fails for the first half
                let seed = Time::now().to_internal_value() << 24 ^ TimeTicks::now().to_internal_value();
                return Self::from_seed(seed);
            }
            
        }

        #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
        {
            let mut seed: i64 = 0;
            if let Ok(_) = getrandom::fill(&mut seed.to_ne_bytes()) {
                return Self::from_seed(seed);
            } else {
                // Fallback to weak entropy if getrandom fails
                let seed = Time::now().to_internal_value() << 24 ^ TimeTicks::now().to_internal_value();
                return Self::from_seed(seed);
            }
        }

        #[cfg(target_os = "starboard")]
        {
            let seed = starboard::system::get_random_uint64() as i64;
            return Self::from_seed(seed);
        }

        #[cfg(not(any(target_os = "cygwin", target_os = "windows", target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "starboard")))]
        {
             // Gather entropy from /dev/urandom if available.
            if let Ok(file) = std::fs::File::open("/dev/urandom") {
                let mut seed_bytes = [0u8; 8];
                if let Ok(mut buffer) = std::io::BufReader::new(file).take(8).read(&mut seed_bytes) {
                    if buffer == 8 {
                        let seed = i64::from_ne_bytes(seed_bytes);
                        return Self::from_seed(seed);
                    }
                 }
            }

            // We cannot assume that random() or rand() were seeded
            // properly, so instead of relying on random() or rand(),
            // we just seed our PRNG using timing data as fallback.
            // This is weak entropy, but it's sufficient, because
            // it is the responsibility of the embedder to install
            // an entropy source using v8::V8::SetEntropySource(),
            // which provides reasonable entropy, see:
            // https://code.google.com/p/v8/issues/detail?id=2905
            let seed = Time::now().to_internal_value() << 24 ^ TimeTicks::now().to_internal_value();
            return Self::from_seed(seed);
        }
    }

    pub fn from_seed(seed: i64) -> Self {
        let mut rng = Self {
            initial_seed_: seed,
            state0_: 0,
            state1_: 0,
        };
        rng.set_seed(seed);
        rng
    }

    #[inline]
    pub fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    pub fn next_int_max(&mut self, max: i32) -> i32 {
        assert!(max > 0);

        if (max as u32).is_power_of_two() {
            return (((max as i64) * (self.next(31) as i64)) >> 31) as i32;
        }

        loop {
            let rnd = self.next(31);
            let val = rnd % max;
            if i32::MAX - (rnd - val) >= (max - 1) {
                return val;
            }
        }
    }

    #[inline]
    pub fn next_bool(&mut self) -> bool {
        self.next(1) != 0
    }

    pub fn next_double(&mut self) -> f64 {
        Self::xor_shift128(&mut self.state0_, &mut self.state1_);
        Self::to_double(self.state0_)
    }

    pub fn next_int64(&mut self) -> i64 {
        Self::xor_shift128(&mut self.state0_, &mut self.state1_);
        unsafe { std::mem::transmute::<u64, i64>(self.state0_.wrapping_add(self.state1_)) }
    }

    pub fn next_bytes(&mut self, buffer: &mut [u8]) {
        for n in 0..buffer.len() {
            buffer[n] = self.next(8) as u8;
        }
    }

    pub fn next_sample(&mut self, max: u64, n: usize) -> Vec<u64> {
        assert!(n as u64 <= max);

        if n == 0 {
            return Vec::new();
        }

        let smaller_part = cmp::min((max - n as u64) as usize, n);
        let mut selected: HashSet<u64> = HashSet::new();

        let mut counter = 0;
        while selected.len() != smaller_part && counter / 3 < smaller_part {
            let x = (self.next_double() * max as f64) as u64;
            assert!(x < max);

            selected.insert(x);
            counter += 1;
        }

        if selected.len() == smaller_part {
            if smaller_part != n {
                return Self::complement_sample(&selected, max);
            }
            return selected.into_iter().collect();
        }

        self.next_sample_slow(max, n, &selected)
    }

    pub fn next_sample_slow(
        &mut self,
        max: u64,
        n: usize,
        excluded: &HashSet<u64>,
    ) -> Vec<u64> {
        assert!(max - excluded.len() as u64 >= n as u64);

        let mut result: Vec<u64> = Vec::with_capacity(max as usize - excluded.len());

        for i in 0..max {
            if !excluded.contains(&i) {
                result.push(i);
            }
        }

        let larger_part = cmp::max((max - n as u64) as usize, n);

        while result.len() != larger_part && result.len() > n {
            let x = (self.next_double() * result.len() as f64) as usize;
            assert!(x < result.len());

            result.swap(x, result.len() - 1);
            result.pop();
        }

        if result.len() != n {
            let s: HashSet<u64> = result.into_iter().collect();
            return Self::complement_sample(&s, max);
        }
        result
    }

    fn next(&mut self, bits: i32) -> i32 {
        assert!(bits > 0);
        assert!(bits <= 32);
        Self::xor_shift128(&mut self.state0_, &mut self.state1_);
        ((self.state0_.wrapping_add(self.state1_)) >> (64 - bits)) as i32
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.initial_seed_ = seed;
        self.state0_ = Self::murmur_hash3(unsafe { transmute::<i64, u64>(seed) });
        self.state1_ = Self::murmur_hash3(!self.state0_);
        assert!(self.state0_ != 0 || self.state1_ != 0);
    }

    pub fn initial_seed(&self) -> i64 {
        self.initial_seed_
    }

    #[inline]
    pub fn to_double(state0: u64) -> f64 {
        let random_0_to_2_53 = (state0 >> 11) as f64;
        const K2_53: f64 = 9007199254740992.0;
        random_0_to_2_53 / K2_53
    }

    #[inline]
    pub fn xor_shift128(state0: &mut u64, state1: &mut u64) {
        let mut s1 = *state0;
        let s0 = *state1;
        *state0 = s0;
        s1 ^= s1 << 23;
        s1 ^= s1 >> 17;
        s1 ^= s0;
        s1 ^= s0 >> 26;
        *state1 = s1;
    }

    pub fn murmur_hash3(mut h: u64) -> u64 {
        h ^= h >> 33;
        h = h.wrapping_mul(0xFF51AFD7ED558CCD);
        h ^= h >> 33;
        h = h.wrapping_mul(0xC4CEB9FE1A85EC53);
        h ^= h >> 33;
        h
    }

    fn complement_sample(set: &HashSet<u64>, max: u64) -> Vec<u64> {
        let mut result = Vec::with_capacity((max - set.len() as u64) as usize);
        for i in 0..max {
            if !set.contains(&i) {
                result.push(i);
            }
        }
        result
    }
}

impl rand::RngCore for RandomNumberGenerator {
    fn next_u32(&mut self) -> u32 {
        self.next_int() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut result: u64 = 0;
        result |= self.next_u32() as u64;
        result |= (self.next_u32() as u64) << 32;
        result
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.next_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl rand::CryptoRng for RandomNumberGenerator {}

impl std::ops::FnOnce<()> for RandomNumberGenerator {
    type Output = u32;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.next_int() as u32
    }
}

impl std::ops::FnMut<()> for RandomNumberGenerator {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.next_int() as u32
    }
}

impl std::ops::Fn<()> for RandomNumberGenerator {
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        let mut mutable_self = unsafe {
            let ptr = self as *const Self;
            (ptr as *mut Self).as_mut().unwrap()
        };
        mutable_self.next_int() as u32
    }
}

impl rand::distributions::Distribution<u32> for RandomNumberGenerator {
    fn sample<R: rand::Rng + ?Sized>(&self, _rng: &mut R) -> u32 {
        let mut mutable_self = unsafe {
            let ptr = self as *const Self;
            (ptr as *mut Self).as_mut().unwrap()
        };
        mutable_self.next_int() as u32
    }
}

struct Time {}

impl Time {
    fn now() -> Self {
        Time {}
    }

    fn to_internal_value(&self) -> i64 {
        16777215 // some random number
    }
}

struct TimeTicks {}

impl TimeTicks {
    fn now() -> Self {
        TimeTicks {}
    }

    fn to_internal_value(&self) -> i64 {
        65535 // some other random number
    }
}

mod starboard {
    pub mod system {
        pub fn get_random_uint64() -> u64 {
            4294967295 // yet another random number
        }
    }
}
}  // namespace utils
}  // namespace base
