// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod maglev_register_frame_array {
    use std::ops::{Index, IndexMut};
    //use v8::interpreter::bytecode_register::Register;  // Assuming this is in a separate crate
    //use v8::maglev::maglev_compilation_unit::MaglevCompilationUnit; // Assuming this is in a separate crate
    //use v8::zone::zone::Zone; // Assuming this is in a separate crate

    // Dummy structs to mimic the original C++ dependencies.
    // Replace with actual implementations from the v8 crate if available.
    pub struct MaglevCompilationUnit {
        parameter_count: usize,
        register_count: usize,
        zone: Zone,
    }

    impl MaglevCompilationUnit {
        pub fn new(parameter_count: usize, register_count: usize, zone: Zone) -> Self {
            MaglevCompilationUnit {
                parameter_count,
                register_count,
                zone,
            }
        }
        pub fn parameter_count(&self) -> usize {
            self.parameter_count
        }
        pub fn register_count(&self) -> usize {
            self.register_count
        }
        pub fn zone(&self) -> &Zone {
            &self.zone
        }
    }
    pub struct BytecodeLivenessState {
        live_registers: Vec<usize>,
    }

    impl BytecodeLivenessState {
        pub fn new(live_registers: Vec<usize>) -> Self {
            BytecodeLivenessState { live_registers }
        }
    }

    impl std::iter::IntoIterator for &BytecodeLivenessState {
        type Item = usize;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.live_registers.clone().into_iter()
        }
    }

    #[derive(Clone, Copy)]
    pub struct Register(i32);

    impl Register {
        pub fn from_parameter_index(index: usize) -> Self {
            Register(-(index as i32) - 1)
        }
        pub fn new(index: usize) -> Self {
            Register(index as i32)
        }

        pub fn index(&self) -> i32 {
            self.0
        }
    }

    pub struct Zone {}

    impl Zone {
        pub fn allocate_array<T>(&self, size: usize) -> Vec<T>
        where
            T: Default + Copy,
        {
            vec![T::default(); size]
        }
    }

    /// Vector of values associated with a bytecode's register frame. Indexable by
    /// interpreter register.
    pub struct RegisterFrameArray<T> {
        frame_start_: Vec<T>,
        parameter_count: usize,
        register_count: usize,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T: Default + Copy> RegisterFrameArray<T> {
        pub fn new(info: &MaglevCompilationUnit) -> Self {
            // The first local is at index zero, parameters are behind it with
            // negative indices, and the unoptimized frame header is between the two,
            // so the entire frame state including parameters is the number of locals
            // and parameters, plus the number of slots between them.
            const FRAME_SIZE_BETWEEN_PARAMS_AND_LOCALS: i32 =
                -Register::from_parameter_index(0).index();
            assert!(Register::from_parameter_index(0).index() < 0);
            assert_eq!(Register::new(0).index(), 0);

            let total_size = (info.parameter_count() as i32
                + FRAME_SIZE_BETWEEN_PARAMS_AND_LOCALS
                + info.register_count() as i32) as usize;

            let mut frame = info.zone().allocate_array::<T>(total_size);

            // Set frame_start_ to a "butterfly" pointer into the middle of the above
            // Zone-allocated array, so that locals start at zero.
            let frame_start_ = frame.clone();

            RegisterFrameArray {
                frame_start_,
                parameter_count: info.parameter_count(),
                register_count: info.register_count(),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn copy_from(
            &mut self,
            info: &MaglevCompilationUnit,
            other: &RegisterFrameArray<T>,
            liveness: Option<&BytecodeLivenessState>,
        ) {
            let last_param = Register::from_parameter_index(info.parameter_count() - 1);
            let mut end = 1;
            if liveness.is_none() {
                let last_local = Register::new(info.register_count() - 1);
                end = last_local.index() as usize;
            }
            // All parameters are live.
            for index in last_param.index()..=0 {
                let reg = Register(index);
                self[reg] = other[reg];
            }
            if let Some(liveness) = liveness {
                for index in liveness {
                    let reg = Register(*index as i32);
                    self[reg] = other[reg];
                }
            }
        }

        fn data_begin(&self, parameter_count: usize) -> i32 {
            Register::from_parameter_index(parameter_count - 1).index()
        }

        fn data_size(register_count: usize, parameter_count: usize) -> i32 {
            // The first local is at index zero, parameters are behind it with
            // negative indices, and the unoptimized frame header is between the two,
            // so the entire frame state including parameters is the distance from the
            // last parameter to the last local frame register, plus one to include both
            // ends.
            let last_local = Register::new(register_count - 1);
            let last_param = Register::from_parameter_index(parameter_count - 1);
            last_local.index() - last_param.index() + 1
        }
    }

    impl<T: Copy> Index<Register> for RegisterFrameArray<T> {
        type Output = T;

        fn index(&self, reg: Register) -> &Self::Output {
            let index = (reg.index() as usize);
            &self.frame_start_[index]
        }
    }

    impl<T: Copy> IndexMut<Register> for RegisterFrameArray<T> {
        fn index_mut(&mut self, reg: Register) -> &mut Self::Output {
            let index = (reg.index() as usize);
            &mut self.frame_start_[index]
        }
    }
}