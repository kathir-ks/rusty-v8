// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod label {

    /// Labels represent pc locations; they are typically jump or call targets.
    /// After declaration, a label can be freely used to denote known or (yet)
    /// unknown pc location. Assembler::bind() is used to bind a label to the
    /// current pc. A label can be bound only once.
    #[derive(Default, Debug)]
    pub struct Label {
        pos_: i32,
        near_link_pos_: i32,
    }

    #[allow(dead_code)]
    impl Label {
        pub enum Distance {
            Near,  // near jump: 8 bit displacement (signed)
            Far    // far jump: 32 bit displacement (signed)
        }

        pub fn new() -> Self {
            Label { pos_: 0, near_link_pos_: 0 }
        }

        pub fn unuse(&mut self) {
            self.pos_ = 0;
        }

        pub fn unuse_near(&mut self) {
            self.near_link_pos_ = 0;
        }

        pub fn is_bound(&self) -> bool {
            self.pos_ < 0
        }

        pub fn is_unused(&self) -> bool {
            self.pos_ == 0 && self.near_link_pos_ == 0
        }

        pub fn is_linked(&self) -> bool {
            self.pos_ > 0
        }

        pub fn is_near_linked(&self) -> bool {
            self.near_link_pos_ > 0
        }

        // Returns the position of bound or linked labels. Cannot be used
        // for unused labels.
        pub fn pos(&self) -> i32 {
            if self.pos_ < 0 {
                return -self.pos_ - 1;
            }
            if self.pos_ > 0 {
                return self.pos_ - 1;
            }
            panic!("UNREACHABLE");
        }

        pub fn near_link_pos(&self) -> i32 {
            self.near_link_pos_ - 1
        }

        fn bind_to(&mut self, pos: i32) {
            self.pos_ = -pos - 1;
            debug_assert!(self.is_bound());
        }

        fn link_to(&mut self, pos: i32, distance: Distance) {
            match distance {
                Distance::Near => {
                    self.near_link_pos_ = pos + 1;
                    debug_assert!(self.is_near_linked());
                }
                Distance::Far => {
                    self.pos_ = pos + 1;
                    debug_assert!(self.is_linked());
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_label() {
            let mut label = Label::new();
            assert!(label.is_unused());
            assert!(!label.is_bound());
            assert!(!label.is_linked());
            assert!(!label.is_near_linked());

            label.bind_to(10);
            assert!(!label.is_unused());
            assert!(label.is_bound());
            assert!(!label.is_linked());
            assert!(!label.is_near_linked());
            assert_eq!(label.pos(), 10);

            let mut label2 = Label::new();
            label2.link_to(20, Label::Distance::Far);
            assert!(!label2.is_unused());
            assert!(!label2.is_bound());
            assert!(label2.is_linked());
            assert!(!label2.is_near_linked());
            assert_eq!(label2.pos(), 20);

            let mut label3 = Label::new();
            label3.link_to(30, Label::Distance::Near);
            assert!(!label3.is_unused());
            assert!(!label3.is_bound());
            assert!(!label3.is_linked());
            assert!(label3.is_near_linked());
            assert_eq!(label3.near_link_pos(), 30);

            label.unuse();
            label2.unuse();
            label3.unuse_near();

            assert!(label.is_unused());
            assert!(label2.is_unused());
            assert_eq!(label3.near_link_pos_, 0);
        }
    }
}