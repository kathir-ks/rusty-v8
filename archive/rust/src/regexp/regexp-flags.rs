// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt;
use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub};
use std::option::Option;

/// Represents the flags for a regular expression.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegExpFlags(u32);

impl RegExpFlags {
    pub const HAS_INDICES: Self = Self(1 << 7);
    pub const GLOBAL: Self = Self(1 << 0);
    pub const IGNORE_CASE: Self = Self(1 << 1);
    pub const LINEAR: Self = Self(1 << 6);
    pub const MULTILINE: Self = Self(1 << 2);
    pub const DOT_ALL: Self = Self(1 << 5);
    pub const UNICODE: Self = Self(1 << 4);
    pub const UNICODE_SETS: Self = Self(1 << 8);
    pub const STICKY: Self = Self(1 << 3);

    /// Checks if the `has_indices` flag is set.
    pub const fn is_has_indices(self) -> bool {
        (self & Self::HAS_INDICES) != Self(0)
    }

    /// Checks if the `global` flag is set.
    pub const fn is_global(self) -> bool {
        (self & Self::GLOBAL) != Self(0)
    }

    /// Checks if the `ignore_case` flag is set.
    pub const fn is_ignore_case(self) -> bool {
        (self & Self::IGNORE_CASE) != Self(0)
    }

    /// Checks if the `linear` flag is set.
    pub const fn is_linear(self) -> bool {
        (self & Self::LINEAR) != Self(0)
    }

    /// Checks if the `multiline` flag is set.
    pub const fn is_multiline(self) -> bool {
        (self & Self::MULTILINE) != Self(0)
    }

    /// Checks if the `dot_all` flag is set.
    pub const fn is_dot_all(self) -> bool {
        (self & Self::DOT_ALL) != Self(0)
    }

    /// Checks if the `unicode` flag is set.
    pub const fn is_unicode(self) -> bool {
        (self & Self::UNICODE) != Self(0)
    }

    /// Checks if the `unicode_sets` flag is set.
    pub const fn is_unicode_sets(self) -> bool {
        (self & Self::UNICODE_SETS) != Self(0)
    }

    /// Checks if the `sticky` flag is set.
    pub const fn is_sticky(self) -> bool {
        (self & Self::STICKY) != Self(0)
    }

    /// Checks if either the `unicode` or `unicode_sets` flag is set.
    pub const fn is_either_unicode(self) -> bool {
        self.is_unicode() || self.is_unicode_sets()
    }
}

impl BitAnd for RegExpFlags {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl BitOr for RegExpFlags {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for RegExpFlags {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl BitXor for RegExpFlags {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }
}

impl BitXorAssign for RegExpFlags {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl Not for RegExpFlags {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl Sub for RegExpFlags {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

impl fmt::Display for RegExpFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        if self.is_global() {
            s.push('g');
        }
        if self.is_ignore_case() {
            s.push('i');
        }
        if self.is_multiline() {
            s.push('m');
        }
        if self.is_sticky() {
            s.push('y');
        }
        if self.is_unicode() {
            s.push('u');
        }
        if self.is_dot_all() {
            s.push('s');
        }
        if self.is_linear() {
            s.push('l');
        }
        if self.is_has_indices() {
            s.push('d');
        }
        if self.is_unicode_sets() {
            s.push('v');
        }
        write!(f, "{}", s)
    }
}

/// Attempts to create a `RegExpFlags` from a character.
pub fn try_regexp_flag_from_char(c: char) -> Option<RegExpFlags> {
    match c {
        'd' => Some(RegExpFlags::HAS_INDICES),
        'g' => Some(RegExpFlags::GLOBAL),
        'i' => Some(RegExpFlags::IGNORE_CASE),
        'l' => Some(RegExpFlags::LINEAR),
        'm' => Some(RegExpFlags::MULTILINE),
        's' => Some(RegExpFlags::DOT_ALL),
        'u' => Some(RegExpFlags::UNICODE),
        'v' => Some(RegExpFlags::UNICODE_SETS),
        'y' => Some(RegExpFlags::STICKY),
        _ => None,
    }
}