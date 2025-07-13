// Converted from V8 C++ source files:
// Header: N/A
// Implementation: gen-regexp-special-case.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::fs::File;
use std::io::{Write, BufWriter};
//use icu::UnicodeSet;
//use icu::UErrorCode;
//use icu::USET_CASE_INSENSITIVE;
//use icu::ErrorCode;
//use icu::char::UChar32;
//use icu::normalizer::Normalizer;
//use icu::string::StringSearch;
//use icu::collator::Collator;
//use icu::pattern::RegexPattern;

mod base {
    pub type uc32 = u32;
}

mod regexp_case_folding {
    pub fn Canonicalize(i: u32) -> u32 {
        i
    }
}


fn print_set(out: &mut BufWriter<File>, name: &str) -> Result<(), std::io::Error> {
    writeln!(out, "/*icu::UnicodeSet Build{}() {{", name)?;
    writeln!(out, "  icu::UnicodeSet set;")?;
    /*for i in 0..set.get_range_count() {
        if set.get_range_start(i) == set.get_range_end(i) {
            writeln!(out, "  set.add(0x{:x});", set.get_range_start(i))?;
        } else {
            writeln!(
                out,
                "  set.add(0x{:x}, 0x{:x});",
                set.get_range_start(i),
                set.get_range_end(i)
            )?;
        }
    }*/
    writeln!(out, "  //set.freeze();")?;
    writeln!(out, "  //return set;")?;
    writeln!(out, "//}}")?;
    writeln!(out)?;

    writeln!(out, "/*struct {}Data {{", name)?;
    writeln!(out, "  {}Data() {{}}", name)?;
    writeln!(out, "  //const icu::UnicodeSet set;")?;
    writeln!(out, "//}};")?;
    writeln!(out)?;

    writeln!(out, "//static")?;
    writeln!(out, "/*const icu::UnicodeSet& RegExpCaseFolding::{}() {{", name)?;
    writeln!(out, "  //static base::LazyInstance<{}Data>::type set =", name)?;
    writeln!(out, "  //    LAZY_INSTANCE_INITIALIZER;")?;
    writeln!(out, "  //return set.Pointer()->set;")?;
    writeln!(out, "//}}*/")?;
    writeln!(out)?;
    Ok(())
}

fn print_special(out: &mut BufWriter<File>) -> Result<(), std::io::Error> {
    //let mut current = icu::UnicodeSet::new();
    //let mut special_add = icu::UnicodeSet::new();
    //let mut ignore = icu::UnicodeSet::new();
    //let mut status = UErrorCode::ZERO_ERROR;
   // let upper = icu::UnicodeSet::new_from_string("[\\p{Lu}]", &mut status).unwrap();
    //if !status.is_ok() {
     //   panic!("UnicodeSet::new_from_string failed: {:?}", status);
    //}

    // Iterate through all chars in BMP except surrogates.
    const K_SURROGATE_START: base::uc32 = 0xd800;
    const K_SURROGATE_END: base::uc32 = 0xdfff;
    const K_NON_BMP_START: base::uc32 = 0x10000;

    /*for i in 0..K_NON_BMP_START {
        if i >= K_SURROGATE_START && i <= K_SURROGATE_END {
            continue; // Ignore surrogate range
        }
        current.set(i, i);
        current.close_over(USET_CASE_INSENSITIVE, &mut status);

        // Check to see if all characters in the case-folding equivalence
        // class as defined by UnicodeSet::closeOver all map to the same
        // canonical value.
        let canonical = regexp_case_folding::Canonicalize(i);
        let mut class_has_matching_canonical_char = false;
        let mut class_has_non_matching_canonical_char = false;
        for j in 0..current.get_range_count() {
            for c in current.get_range_start(j)..=current.get_range_end(j) {
                if c == i {
                    continue;
                }
                let other_canonical = regexp_case_folding::Canonicalize(c);
                if canonical == other_canonical {
                    class_has_matching_canonical_char = true;
                } else {
                    class_has_non_matching_canonical_char = true;
                }
            }
        }
        // If any other character in i's equivalence class has a
        // different canonical value, then i needs special handling.  If
        // no other character shares a canonical value with i, we can
        // ignore i when adding alternatives for case-independent
        // comparison.  If at least one other character shares a
        // canonical value, then i needs special handling.
        if class_has_non_matching_canonical_char {
            if class_has_matching_canonical_char {
                special_add.add(i);
            } else {
                ignore.add(i);
            }
        }
    }

    // Verify that no Unicode equivalence class contains two non-trivial
    // JS equivalence classes. Every character in SpecialAddSet has the
    // same canonical value as every other non-IgnoreSet character in
    // its Unicode equivalence class. Therefore, if we call closeOver on
    // a set containing no IgnoreSet characters, the only characters
    // that must be removed from the result are in IgnoreSet. This fact
    // is used in CharacterRange::AddCaseEquivalents.
    for i in 0..special_add.get_range_count() {
        for c in special_add.get_range_start(i)..=special_add.get_range_end(i) {
            let canonical = regexp_case_folding::Canonicalize(c);
            current.set(c, c);
            current.close_over(USET_CASE_INSENSITIVE, &mut status);
            current.remove_all(&ignore);
            for j in 0..current.get_range_count() {
                for c2 in current.get_range_start(j)..=current.get_range_end(j) {
                    assert_eq!(canonical, regexp_case_folding::Canonicalize(c2));
                }
            }
        }
    }*/

    print_set(out, "IgnoreSet")?;
    print_set(out, "SpecialAddSet")?;
    Ok(())
}

fn write_header(header_filename: &str) -> Result<(), std::io::Error> {
    let file = File::create(header_filename)?;
    let mut out = BufWriter::new(file);

    writeln!(out, "// Copyright 2020 the V8 project authors. All rights reserved.")?;
    writeln!(out, "// Use of this source code is governed by a BSD-style license that")?;
    writeln!(out, "// can be found in the LICENSE file.")?;
    writeln!(out)?;
    writeln!(out, "// Automatically generated by regexp/gen-regexp-special-case.cc")?;
    writeln!(out)?;
    writeln!(out, "// The following functions are used to build UnicodeSets")?;
    writeln!(out, "// for special cases where the case-folding algorithm used by")?;
    writeln!(out, "// UnicodeSet::closeOver(USET_CASE_INSENSITIVE) does not match")?;
    writeln!(out, "// the algorithm defined in ECMAScript 2020 21.2.2.8.2 (Runtime")?;
    writeln!(out, "// Semantics: Canonicalize) step 3.")?;
    writeln!(out)?;
    writeln!(out, "#[cfg(feature = \"intl\")]")?;
    writeln!(out, "mod base {{")?;
    writeln!(out, "   pub type uc32 = u32;")?;
    writeln!(out, "}}")?;
    writeln!(out, "#[cfg(feature = \"intl\")]")?;
    writeln!(out, "mod regexp_case_folding {{")?;
    writeln!(out, "   pub fn Canonicalize(i: u32) -> u32 {{")?;
    writeln!(out, "      i")?;
    writeln!(out, "   }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "#[cfg(feature = \"intl\")]")?;
    writeln!(out, "mod icu {{")?;
    writeln!(out, "   pub struct UnicodeSet {{}}")?;
    writeln!(out, "   impl UnicodeSet {{")?;
    writeln!(out, "       pub fn new() -> Self {{ UnicodeSet {{}} }}")?;
    writeln!(out, "       pub fn add(&mut self, _c: u32) {{}}")?;
    writeln!(out, "       pub fn close_over(&mut self, _uset_case_insensitive: i32, _status: &mut i32) {{}}")?;
    writeln!(out, "       pub fn get_range_count(&self) -> i32 {{ 0 }}")?;
    writeln!(out, "       pub fn get_range_start(&self, _i: i32) -> u32 {{ 0 }}")?;
    writeln!(out, "       pub fn get_range_end(&self, _i: i32) -> u32 {{ 0 }}")?;
    writeln!(out, "       pub fn remove_all(&mut self, _other: &UnicodeSet) {{}}")?;
    writeln!(out, "   }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "#[cfg(feature = \"intl\")]")?;
    writeln!(out, "use icu::UnicodeSet;")?;
    writeln!(out, "#[cfg(feature = \"intl\")]")?;
    writeln!(out, "pub mod regexp_special_case {{")?;
    writeln!(out)?;

    print_special(&mut out)?;

    writeln!(out)?;
    writeln!(out, "}}  // namespace regexp_special_case")?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <output filename>", args[0]);
        std::process::exit(1);
    }
    write_header(&args[1])?;

    Ok(())
}
