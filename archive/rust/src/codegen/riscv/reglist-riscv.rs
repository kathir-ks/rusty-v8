// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod register_arch {
    // Placeholder for src/codegen/register-arch.h
    // Assuming it defines a Register and DoubleRegister type.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Register(pub u32);

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct DoubleRegister(pub u32);

    pub const t0: Register = Register(0);
    pub const t1: Register = Register(1);
    pub const t2: Register = Register(2);
    pub const a0: Register = Register(3);
    pub const a1: Register = Register(4);
    pub const a2: Register = Register(5);
    pub const a3: Register = Register(6);
    pub const a4: Register = Register(7);
    pub const a5: Register = Register(8);
    pub const a6: Register = Register(9);
    pub const a7: Register = Register(10);
    pub const t4: Register = Register(11);
    pub const fp: Register = Register(12);
    pub const s1: Register = Register(13);
    pub const s2: Register = Register(14);
    pub const s3: Register = Register(15);
    pub const s4: Register = Register(16);
    pub const s5: Register = Register(17);
    pub const s6: Register = Register(18);
    pub const s7: Register = Register(19);
    pub const s8: Register = Register(20);
    pub const s9: Register = Register(21);
    pub const s10: Register = Register(22);
    pub const s11: Register = Register(23);

    pub const ft0: DoubleRegister = DoubleRegister(0);
    pub const ft1: DoubleRegister = DoubleRegister(1);
    pub const ft2: DoubleRegister = DoubleRegister(2);
    pub const ft3: DoubleRegister = DoubleRegister(3);
    pub const ft4: DoubleRegister = DoubleRegister(4);
    pub const ft5: DoubleRegister = DoubleRegister(5);
    pub const ft6: DoubleRegister = DoubleRegister(6);
    pub const ft7: DoubleRegister = DoubleRegister(7);
    pub const fa0: DoubleRegister = DoubleRegister(8);
    pub const fa1: DoubleRegister = DoubleRegister(9);
    pub const fa2: DoubleRegister = DoubleRegister(10);
    pub const fa3: DoubleRegister = DoubleRegister(11);
    pub const fa4: DoubleRegister = DoubleRegister(12);
    pub const fa5: DoubleRegister = DoubleRegister(13);
    pub const fa6: DoubleRegister = DoubleRegister(14);
    pub const fa7: DoubleRegister = DoubleRegister(15);
    pub const ft8: DoubleRegister = DoubleRegister(16);
    pub const ft9: DoubleRegister = DoubleRegister(17);
    pub const ft10: DoubleRegister = DoubleRegister(18);
    pub const ft11: DoubleRegister = DoubleRegister(19);

    pub const fs0: DoubleRegister = DoubleRegister(20);
    pub const fs1: DoubleRegister = DoubleRegister(21);
    pub const fs2: DoubleRegister = DoubleRegister(22);
    pub const fs3: DoubleRegister = DoubleRegister(23);
    pub const fs4: DoubleRegister = DoubleRegister(24);
    pub const fs5: DoubleRegister = DoubleRegister(25);
    pub const fs6: DoubleRegister = DoubleRegister(26);
    pub const fs7: DoubleRegister = DoubleRegister(27);
    pub const fs8: DoubleRegister = DoubleRegister(28);
    pub const fs9: DoubleRegister = DoubleRegister(29);
    pub const fs10: DoubleRegister = DoubleRegister(30);
    pub const fs11: DoubleRegister = DoubleRegister(31);
}

mod reglist_base {
    use std::marker::Copy;
    use std::ops::BitOr;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RegListBase<T: Copy>(pub Vec<T>);

    impl<T: Copy> RegListBase<T> {
        pub const fn new(registers: Vec<T>) -> Self {
            RegListBase(registers)
        }

        pub const fn Count(&self) -> usize {
            self.0.len()
        }
    }

    impl<T: Copy + Clone> BitOr for RegListBase<T> {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            let mut result = self.0.clone();
            result.extend(other.0);
            RegListBase(result)
        }
    }
}

pub mod internal {
    use crate::register_arch::*;
    use crate::reglist_base::*;

    pub type RegList = RegListBase<Register>;
    pub type DoubleRegList = RegListBase<DoubleRegister>;

    //TODO: Add proper compile time assertions, e.g., using static_assertions crate

    pub const kJSCallerSaved: RegList = RegList::new(vec![
        t0, t1, t2, a0, a1, a2, a3, a4, a5, a6, a7, t4,
    ]);

    pub const kNumJSCallerSaved: usize = 12;

    // Callee-saved registers preserved when switching from C to JavaScript.
    pub const kCalleeSaved: RegList = RegList::new(vec![
        fp, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11,
    ]);

    pub const kNumCalleeSaved: usize = 12;

    pub const kCalleeSavedFPU: DoubleRegList = RegList::new(vec![
        fs0, fs1, fs2, fs3, fs4, fs5, fs6, fs7, fs8, fs9, fs10, fs11,
    ]);

    pub const kNumCalleeSavedFPU: usize = kCalleeSavedFPU.Count();

    pub const kCallerSavedFPU: DoubleRegList = RegList::new(vec![
        ft0, ft1, ft2, ft3, ft4, ft5, ft6, ft7, fa0, fa1, fa2, fa3, fa4, fa5, fa6, fa7, ft8, ft9,
        ft10, ft11,
    ]);

    pub const kNumCallerSavedFPU: usize = kCallerSavedFPU.Count();

    // Number of registers for which space is reserved in safepoints. Must be a
    // multiple of 8.
    pub const kNumSafepointRegisters: usize = 32;

    // Define the list of registers actually saved at safepoints.
    // Note that the number of saved registers may be smaller than the reserved
    // space, i.e. kNumSafepointSavedRegisters <= kNumSafepointRegisters.
    pub const kSafepointSavedRegisters: RegList = {
        let mut combined = kJSCallerSaved.0.clone();
        combined.extend(kCalleeSaved.0.clone());
        RegList::new(combined)
    };

    pub const kNumSafepointSavedRegisters: usize = kNumJSCallerSaved + kNumCalleeSaved;
}