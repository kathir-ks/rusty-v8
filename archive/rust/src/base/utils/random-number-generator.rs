pub mod base {
    pub mod utils {
        use std::collections::HashSet;
        use std::convert::TryInto;
        use std::limits;
        use std::mem;
        use std::slice;

        /// Represents a source of entropy for the random number generator.
        pub type EntropySource = fn(*mut u8, usize) -> bool;

        static mut ENTROPY_SOURCE: Option<EntropySource> = None;

        /// Sets the global entropy source for the random number generator.
        pub fn set_entropy_source(entropy_source: EntropySource) {
            unsafe {
                ENTROPY_SOURCE = Some(entropy_source);
            }
        }

        /// A pseudo-random number generator using xorshift128+.
        ///
        /// This struct generates a stream of pseudo-random numbers with a period
        /// length of 2^128-1.  It uses a 64-bit seed, which is passed through
        /// MurmurHash3 to create two 64-bit state values. This pair of state
        /// values is then used in xorshift128+.
        ///
        /// NOTE: Any changes to the algorithm must be tested against TestU01.
        ///       Please find instructions for this in the internal repository.
        ///
        /// This class is neither reentrant nor threadsafe.
        pub struct RandomNumberGenerator {
            initial_seed_: i64,
            state0_: u64,
            state1_: u64,
        }

        impl RandomNumberGenerator {
            const MULTIPLIER: i64 = 0x5_deec_e66d;
            const ADDEND: i64 = 0xb;
            const MASK: i64 = 0xffff_ffff_ffff;

            /// Creates a new `RandomNumberGenerator` with a seed derived from a weak entropy source.
            pub fn new() -> Self {
                let mut rng = Self {
                    initial_seed_: 0,
                    state0_: 0,
                    state1_: 0,
                };
                rng.set_seed(Self::generate_seed());
                rng
            }

            fn generate_seed() -> i64 {
                let mut buffer: [u8; 8] = [0; 8];
                let buffer_slice = buffer.as_mut_ptr();

                unsafe {
                    if let Some(entropy_source) = ENTROPY_SOURCE {
                        if entropy_source(buffer_slice, 8) {
                            return i64::from_ne_bytes(buffer);
                        }
                    }
                }

                // Fallback to a less secure method if entropy source is not set or fails.
                // This is similar to the C++ code, which relies on the embedder setting a
                // proper entropy source.
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0) as i64;
                timestamp.wrapping_mul(Self::MULTIPLIER).wrapping_add(Self::ADDEND) & Self::MASK
            }

            /// Creates a new `RandomNumberGenerator` with the specified seed.
            pub fn new_with_seed(seed: i64) -> Self {
                let mut rng = Self {
                    initial_seed_: 0,
                    state0_: 0,
                    state1_: 0,
                };
                rng.set_seed(seed);
                rng
            }

            /// Returns the next pseudorandom, uniformly distributed `i32` value
            /// from this random number generator's sequence.
            pub fn next_int(&mut self) -> i32 {
                self.next(32) as i32
            }

            /// Returns a pseudorandom, uniformly distributed `i32` value between
            /// 0 (inclusive) and the specified `max` value (exclusive).
            pub fn next_int_max(&mut self, max: i32) -> i32 {
                if max <= 0 {
                    panic!("max must be positive");
                }
                (self.next_double() * (max as f64)) as i32
            }

            /// Returns the next pseudorandom, uniformly distributed `bool` value
            /// from this random number generator's sequence.
            pub fn next_bool(&mut self) -> bool {
                self.next(1) != 0
            }

            /// Returns the next pseudorandom, uniformly distributed `f64` value
            /// between 0.0 (inclusive) and 1.0 (exclusive) from this random
            /// number generator's sequence.
            pub fn next_double(&mut self) -> f64 {
                Self::to_double(self.state0_)
            }

            /// Returns the next pseudorandom, uniformly distributed `i64` value
            /// from this random number generator's sequence.
            pub fn next_int64(&mut self) -> i64 {
                let high = self.next(32) as i64;
                let low = self.next(32) as i64;
                (high << 32) | low
            }

            /// Fills the elements of a specified array of bytes with random numbers.
            pub fn next_bytes(&mut self, buffer: &mut [u8]) {
                let mut i = 0;
                while i + 7 < buffer.len() {
                    let value = self.next_int64();
                    buffer[i..i + 8].copy_from_slice(&value.to_ne_bytes());
                    i += 8;
                }
                let remaining = buffer.len() - i;
                if remaining > 0 {
                    let value = self.next_int64();
                    let bytes = value.to_ne_bytes();
                    buffer[i..].copy_from_slice(&bytes[..remaining]);
                }
            }

            /// Returns the next pseudorandom set of `n` unique `u64` values smaller
            /// than `max`.
            ///
            /// `n` must be less or equal to `max`.
            pub fn next_sample(&mut self, max: u64, n: usize) -> Vec<u64> {
                if n > max as usize {
                    panic!("n must be less or equal to max");
                }
                self.next_sample_slow(max, n, &HashSet::new())
            }

            /// Returns the next pseudorandom set of `n` unique `u64` values smaller
            /// than `max`.
            ///
            /// `n` must be less or equal to `max`.
            /// `max - excluded` must be less or equal to `n`.
            ///
            /// Generates list of all possible values and removes random values from
            /// it until size reaches n.
            pub fn next_sample_slow(
                &mut self,
                max: u64,
                n: usize,
                excluded: &HashSet<u64>,
            ) -> Vec<u64> {
                if n > max as usize {
                    panic!("n must be less or equal to max");
                }

                let mut candidates: Vec<u64> = (0..max).filter(|&x| !excluded.contains(&x)).collect();

                if candidates.len() < n {
                  return candidates; // Cannot generate n unique values.
                }

                let mut result: Vec<u64> = Vec::with_capacity(n);
                for _ in 0..n {
                    let index = self.next_int_max(candidates.len() as i32) as usize;
                    result.push(candidates.remove(index));
                }
                result
            }

            /// Overrides the current seed.
            pub fn set_seed(&mut self, seed: i64) {
                self.initial_seed_ = seed;
                let hash = Self::murmur_hash3(seed as u64);
                self.state0_ = hash;
                self.state1_ = hash ^ 0xdeadbeef;
            }

            /// Returns the initial seed used to initialize this generator.
            pub fn initial_seed(&self) -> i64 {
                self.initial_seed_
            }

            /// Converts a u64 state to a double in the range [0.0, 1.0).
            pub fn to_double(state0: u64) -> f64 {
                // Get a random [0,2**53) integer value (up to MAX_SAFE_INTEGER) by dropping
                // 11 bits of the state.
                let random_0_to_2_53 = (state0 >> 11) as f64;
                // Map this to [0,1) by division with 2**53.
                const K2_53: f64 = (1_u64 << 53) as f64;
                random_0_to_2_53 / K2_53
            }

            /// Performs the xorshift128+ operation.
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

            /// MurmurHash3 algorithm for generating initial state.
            pub fn murmur_hash3(mut k: u64) -> u64 {
                k ^= k >> 33;
                k = k.wrapping_mul(0xff51afd7ed558ccd);
                k ^= k >> 33;
                k = k.wrapping_mul(0xc2b2ae353253864f);
                k ^= k >> 33;
                k
            }

            fn next(&mut self, bits: i32) -> u32 {
                Self::xor_shift128(&mut self.state0_, &mut self.state1_);
                (self.state0_ >> (64 - bits)) as u32
            }
        }

        impl std::ops::FnMut<()> for RandomNumberGenerator {
            extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
                self.next_int() as Self::Output
            }
        }

        impl std::ops::FnOnce<()> for RandomNumberGenerator {
            type Output = u32;

            extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
                (self.call_mut)(args)
            }
        }

        impl std::ops::Fn<()> for RandomNumberGenerator {
            extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
                panic!("cannot call RandomNumberGenerator::call without a mutable reference");
            }
        }

        impl RandomNumberGenerator {
            pub const fn min() -> u32 {
                0
            }
            pub const fn max() -> u32 {
                std::u32::MAX
            }
        }
    }
}