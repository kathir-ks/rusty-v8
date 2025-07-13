// Converted from V8 C++ source files:
// Header: label.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

namespace! {
  v8 {
    namespace! {
      internal {
        #[derive(Default)]
        pub struct Label {
          pos_: i32,
          near_link_pos_: i32,
        }

        impl Label {
          pub enum Distance {
            kNear,
            kFar,
          }

          pub fn new() -> Self {
            Label {
              pos_: 0,
              near_link_pos_: 0,
            }
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

          pub fn pos(&self) -> i32 {
            if self.pos_ < 0 {
              -self.pos_ - 1
            } else if self.pos_ > 0 {
              self.pos_ - 1
            } else {
              panic!("Unreachable");
            }
          }

          pub fn near_link_pos(&self) -> i32 {
            self.near_link_pos_ - 1
          }

          fn bind_to(&mut self, pos: i32) {
            self.pos_ = -pos - 1;
          }

          fn link_to(&mut self, pos: i32, distance: Label::Distance) {
            match distance {
              Label::Distance::kNear => {
                self.near_link_pos_ = pos + 1;
              }
              Label::Distance::kFar => {
                self.pos_ = pos + 1;
              }
            }
          }
        }

        #[cfg(debug_assertions)]
        impl Drop for Label {
          fn drop(&mut self) {
            assert!(!self.is_linked());
            assert!(!self.is_near_linked());
          }
        }
      }
    }
  }
}
