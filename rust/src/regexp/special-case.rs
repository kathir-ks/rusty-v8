// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![cfg(feature = "intl")]

#[cfg(feature = "intl")]
pub mod regexp_case_folding {
    use icu::char::props::CaseV2;
    use icu::string::StringSearch;
    use icu::unicodeset::UnicodeSet;
    use icu::uchar;
    use icu::unorm::compose;
    use std::char;
    use std::convert::TryFrom;

    /// Sets of Unicode characters that need special handling under "i" mode

    /// For non-unicode ignoreCase matches (aka "i", not "iu"), ECMA 262
    /// defines slightly different case-folding rules than Unicode. An
    /// input character should match a pattern character if the result of
    /// the Canonicalize algorithm is the same for both characters.
    ///
    /// Roughly speaking, for "i" regexps, Canonicalize(c) is the same as
    /// c.toUpperCase(), unless a) c.toUpperCase() is a multi-character
    /// string, or b) c is non-ASCII, and c.toUpperCase() is ASCII. See
    /// https://tc39.es/ecma262/#sec-runtime-semantics-canonicalize-ch for
    /// the precise definition.
    ///
    /// While compiling such regular expressions, we need to compute the
    /// set of characters that should match a given input character. (See
    /// GetCaseIndependentLetters and CharacterRange::AddCaseEquivalents.)
    /// For almost all characters, this can be efficiently computed using
    /// UnicodeSet::closeOver(USET_CASE_INSENSITIVE). These sets represent
    /// the remaining special cases.
    ///
    /// For a character c, the rules are as follows:
    ///
    /// 1. If c is in neither IgnoreSet nor SpecialAddSet, then calling
    ///    UnicodeSet::closeOver(USET_CASE_INSENSITIVE) on a UnicodeSet
    ///    containing c will produce the set of characters that should
    ///    match /c/i (or /[c]/i), and only those characters.
    ///
    /// 2. If c is in IgnoreSet, then the only character it should match is
    ///    itself. However, closeOver will add additional incorrect
    ///    matches. For example, consider SHARP S: 'ß' (U+00DF) and 'ẞ'
    ///    (U+1E9E). Although closeOver('ß') = "ßẞ", uppercase('ß') is
    ///    "SS".  Step 3.e therefore requires that 'ß' canonicalizes to
    ///    itself, and should not match 'ẞ'. In these cases, we can skip
    ///    the closeOver entirely, because it will never add an equivalent
    ///    character.
    ///
    /// 3. If c is in SpecialAddSet, then it should match at least one
    ///    character other than itself. However, closeOver will add at
    ///    least one additional incorrect match. For example, consider the
    ///    letter 'k'. Closing over 'k' gives "kKK" (lowercase k, uppercase
    ///    K, U+212A KELVIN SIGN). However, because of step 3.g, KELVIN
    ///    SIGN should not match either of the other two characters. As a
    ///    result, "k" and "K" are in SpecialAddSet (and KELVIN SIGN is in
    ///    IgnoreSet). To find the correct matches for characters in
    ///    SpecialAddSet, we closeOver the original character, but filter
    ///    out the results that do not have the same canonical value.
    ///
    /// The contents of these sets are calculated at build time by
    /// src/regexp/gen-regexp-special-case.cc, which generates
    /// gen/src/regexp/special-case.cc. This is done by iterating over the
    /// result of closeOver for each BMP character, and finding sets for
    /// which at least one character has a different canonical value than
    /// another character. Characters that match no other characters in
    /// their equivalence class are added to IgnoreSet. Characters that
    /// match at least one other character are added to SpecialAddSet.

    pub struct RegExpCaseFolding {}

    impl RegExpCaseFolding {
        /// Placeholder for the IgnoreSet.  Needs to be initialized correctly.
        pub fn ignore_set() -> &'static UnicodeSet {
            static IGNORE_SET: UnicodeSet = UnicodeSet::new(); // Replace with correct initialization
            &IGNORE_SET
        }

        /// Placeholder for the SpecialAddSet. Needs to be initialized correctly.
        pub fn special_add_set() -> &'static UnicodeSet {
            static SPECIAL_ADD_SET: UnicodeSet = UnicodeSet::new(); // Replace with correct initialization
            &SPECIAL_ADD_SET
        }

        /// This implements ECMAScript 2020 21.2.2.8.2 (Runtime Semantics:
        /// Canonicalize) step 3, which is used to determine whether
        /// characters match when ignoreCase is true and unicode is false.
        pub fn canonicalize(ch: u32) -> u32 {
            // a. Assert: ch is a UTF-16 code unit.
            if ch > 0xffff {
              return ch;
            }

            // b. Let s be the String value consisting of the single code unit ch.
            let s = String::from_utf16_lossy(&[ch as u16]);

            // c. Let u be the same result produced as if by performing the algorithm
            // for String.prototype.toUpperCase using s as the this value.
            // d. Assert: Type(u) is String.
            let u = s.to_uppercase();

            // e. If u does not consist of a single code unit, return ch.
            if u.chars().count() != 1 {
              return ch;
            }

            // f. Let cu be u's single code unit element.
            let cu = u.chars().next().unwrap() as u32;

            // g. If the value of ch >= 128 and the value of cu < 128, return ch.
            if ch >= 128 && cu < 128 {
              return ch;
            }

            // h. Return cu.
            cu
        }
    }
}