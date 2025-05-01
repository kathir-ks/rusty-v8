// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod sandbox {
    pub mod check {
        use cfg_if::cfg_if;

        cfg_if! {
            if #[cfg(feature = "v8_enable_sandbox")] {
                use super::hardware_support::SandboxHardwareSupport;

                macro_rules! block_sandbox_access_in_debug_mode {
                    () => {
                        #[cfg(debug_assertions)]
                        let _block_access = SandboxHardwareSupport::maybe_block_access();
                        #[cfg(not(debug_assertions))]
                        {};
                    };
                }

                macro_rules! sbxcheck {
                    ($condition:expr) => {
                        {
                            block_sandbox_access_in_debug_mode!();
                            assert!($condition);
                        }
                    };
                }

                macro_rules! sbxcheck_wrapped {
                    ($condition:ident, $lhs:expr, $rhs:expr) => {
                        {
                            block_sandbox_access_in_debug_mode!();
                            match stringify!($condition) {
                                "EQ" => assert_eq!($lhs, $rhs),
                                "NE" => assert_ne!($lhs, $rhs),
                                "GT" => assert!($lhs > $rhs),
                                "GE" => assert!($lhs >= $rhs),
                                "LT" => assert!($lhs < $rhs),
                                "LE" => assert!($lhs <= $rhs),
                                "BOUNDS" => assert!($lhs < $rhs),
                                "IMPLIES" => if $lhs { assert!($rhs); },
                                _ => panic!("Unknown SBXCHECK type"),
                            }
                        }
                    };
                }

                macro_rules! sbxcheck_eq {
                    ($lhs:expr, $rhs:expr) => {
                        sbxcheck_wrapped!(EQ, $lhs, $rhs)
                    };
                }

                macro_rules! sbxcheck_ne {
                    ($lhs:expr, $rhs:expr) => {
                        sbxcheck_wrapped!(NE, $lhs, $rhs)
                    };
                }

                macro_rules! sbxcheck_gt {
                    ($lhs:expr, $rhs:expr) => {
                        sbxcheck_wrapped!(GT, $lhs, $rhs)
                    };
                }

                macro_rules! sbxcheck_ge {
                    ($lhs:expr, $rhs:expr) => {
                        sbxcheck_wrapped!(GE, $lhs, $rhs)
                    };
                }

                macro_rules! sbxcheck_lt {
                    ($lhs:expr, $rhs:expr) => {
                        sbxcheck_wrapped!(LT, $lhs, $rhs)
                    };
                }

                macro_rules! sbxcheck_le {
                    ($lhs:expr, $rhs:expr) => {
                        sbxcheck_wrapped!(LE, $lhs, $rhs)
                    };
                }

                macro_rules! sbxcheck_bounds {
                    ($index:expr, $limit:expr) => {
                        sbxcheck_wrapped!(BOUNDS, $index, $limit)
                    };
                }

                macro_rules! sbxcheck_implies {
                    ($when:expr, $then:expr) => {
                        sbxcheck_wrapped!(IMPLIES, $when, $then)
                    };
                }

                pub(crate) use sbxcheck;
                pub(crate) use sbxcheck_eq;
                pub(crate) use sbxcheck_ne;
                pub(crate) use sbxcheck_gt;
                pub(crate) use sbxcheck_ge;
                pub(crate) use sbxcheck_lt;
                pub(crate) use sbxcheck_le;
                pub(crate) use sbxcheck_bounds;
                pub(crate) use sbxcheck_implies;
            } else {
                macro_rules! sbxcheck {
                    ($condition:expr) => {
                        debug_assert!($condition);
                    };
                }

                macro_rules! sbxcheck_eq {
                    ($lhs:expr, $rhs:expr) => {
                        debug_assert_eq!($lhs, $rhs);
                    };
                }

                macro_rules! sbxcheck_ne {
                    ($lhs:expr, $rhs:expr) => {
                        debug_assert_ne!($lhs, $rhs);
                    };
                }

                macro_rules! sbxcheck_gt {
                    ($lhs:expr, $rhs:expr) => {
                        debug_assert!($lhs > $rhs);
                    };
                }

                macro_rules! sbxcheck_ge {
                    ($lhs:expr, $rhs:expr) => {
                        debug_assert!($lhs >= $rhs);
                    };
                }

                macro_rules! sbxcheck_lt {
                    ($lhs:expr, $rhs:expr) => {
                        debug_assert!($lhs < $rhs);
                    };
                }

                macro_rules! sbxcheck_le {
                    ($lhs:expr, $rhs:expr) => {
                        debug_assert!($lhs <= $rhs);
                    };
                }

                macro_rules! sbxcheck_bounds {
                    ($index:expr, $limit:expr) => {
                        debug_assert!($index < $limit);
                    };
                }

                macro_rules! sbxcheck_implies {
                    ($when:expr, $then:expr) => {
                        if $when {
                            debug_assert!($then);
                        }
                    };
                }

                pub(crate) use sbxcheck;
                pub(crate) use sbxcheck_eq;
                pub(crate) use sbxcheck_ne;
                pub(crate) use sbxcheck_gt;
                pub(crate) use sbxcheck_ge;
                pub(crate) use sbxcheck_lt;
                pub(crate) use sbxcheck_le;
                pub(crate) use sbxcheck_bounds;
                pub(crate) use sbxcheck_implies;
            }
        }
    }

    pub mod hardware_support {
        pub struct SandboxHardwareSupport;

        impl SandboxHardwareSupport {
            pub fn maybe_block_access() {}
        }
    }
}