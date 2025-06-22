// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use regex::Regex;

pub mod protocol {
    pub mod Debugger {
        #[derive(Debug, Clone)]
        pub struct SearchMatch {
            pub line_number: i32,
            pub line_content: String,
        }

        impl SearchMatch {
            pub fn create() -> SearchMatchBuilder {
                SearchMatchBuilder::new()
            }
        }

        pub struct SearchMatchBuilder {
            line_number: Option<i32>,
            line_content: Option<String>,
        }

        impl SearchMatchBuilder {
            pub fn new() -> Self {
                SearchMatchBuilder {
                    line_number: None,
                    line_content: None,
                }
            }

            pub fn set_line_number(mut self, line_number: i32) -> Self {
                self.line_number = Some(line_number);
                self
            }

            pub fn set_line_content(mut self, line_content: String) -> Self {
                self.line_content = Some(line_content);
                self
            }

            pub fn build(self) -> Option<SearchMatch> {
                Some(SearchMatch {
                    line_number: self.line_number?,
                    line_content: self.line_content?,
                })
            }
        }
    }
}

pub trait V8InspectorSession {
    fn inspector(&self) -> &dyn V8Inspector;
}

pub trait V8Inspector {
    // Define necessary methods from V8Inspector here
    // that V8Regex needs.
}

pub struct V8Regex {
    regex: Regex,
}

impl V8Regex {
    pub fn new(_inspector: &dyn V8Inspector, pattern: String, case_sensitive: bool) -> Self {
        let regex_string = if case_sensitive {
            pattern
        } else {
            format!("(?i){}", pattern)
        };

        let regex = Regex::new(&regex_string).unwrap();
        V8Regex { regex }
    }

    pub fn match_(&self, text: &str, start_index: usize) -> Option<usize> {
        self.regex.find_at(text, start_index).map(|m| m.start())
    }
}

pub struct V8InspectorImpl {}

impl V8Inspector for V8InspectorImpl {}

pub struct V8InspectorSessionImpl {
    inspector: V8InspectorImpl,
}

impl V8InspectorSessionImpl {
    pub fn new(inspector: V8InspectorImpl) -> Self {
        V8InspectorSessionImpl { inspector }
    }
}

impl V8InspectorSession for V8InspectorSessionImpl {
    fn inspector(&self) -> &dyn V8Inspector {
        &self.inspector
    }
}

mod string16 {
    use std::string::FromUtf16Error;

    #[derive(Debug, Clone, PartialEq)]
    pub struct String16 {
        data: Vec<u16>,
    }

    impl String16 {
        pub const K_NOT_FOUND: usize = usize::MAX;

        pub fn from_str(s: &str) -> Self {
            String16 {
                data: s.encode_utf16().collect(),
            }
        }

        pub fn from_utf16(data: Vec<u16>) -> Result<Self, FromUtf16Error> {
            String::from_utf16(&data)?;
            Ok(String16 { data })
        }

        pub fn to_string(&self) -> String {
            String::from_utf16_lossy(&self.data)
        }

        pub fn length(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn find(&self, needle: &str, start: usize) -> usize {
            let needle_utf16: Vec<u16> = needle.encode_utf16().collect();
            if needle_utf16.is_empty() {
                return start;
            }

            let mut i = start;
            while i + needle_utf16.len() <= self.data.len() {
                if self.data[i..i + needle_utf16.len()] == needle_utf16[..] {
                    return i;
                }
                i += 1;
            }

            String16::K_NOT_FOUND
        }

        pub fn reverse_find(&self, needle: &str, end: usize) -> usize {
            let needle_utf16: Vec<u16> = needle.encode_utf16().collect();
            if needle_utf16.is_empty() {
                return end;
            }

            let mut i = end.min(self.data.len());
            while i >= needle_utf16.len() {
                if self.data[i - needle_utf16.len()..i] == needle_utf16[..] {
                    return i - needle_utf16.len();
                }
                i -= 1;
            }

            String16::K_NOT_FOUND
        }

        pub fn substring(&self, start: usize, end: usize) -> String16 {
            String16 {
                data: self.data[start..end].to_vec(),
            }
        }

        pub fn char_at(&self, index: usize) -> Option<u16> {
            self.data.get(index).copied()
        }

        pub fn strip_white_space(&self) -> String16 {
            let mut start = 0;
            while start < self.data.len() && self.data[start].is_ascii_whitespace() {
                start += 1;
            }

            let mut end = self.data.len();
            while end > start && self.data[end - 1].is_ascii_whitespace() {
                end -= 1;
            }

            self.substring(start, end)
        }

        // Simulate indexing behavior: return a u16 at a given index.
        pub fn get(&self, index: usize) -> Option<u16> {
            self.data.get(index).copied()
        }
    }

    impl std::ops::Index<usize> for String16 {
        type Output = u16;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }
}

use string16::String16;

mod string16_builder {
    use super::string16::String16;

    pub struct String16Builder {
        data: Vec<u16>,
    }

    impl String16Builder {
        pub fn new() -> String16Builder {
            String16Builder { data: Vec::new() }
        }

        pub fn append(&mut self, c: u16) {
            self.data.push(c);
        }

        pub fn append_str(&mut self, s: &str) {
            self.data.extend(s.encode_utf16());
        }

        pub fn to_string(self) -> String16 {
            String16 { data: self.data }
        }
    }
}

use string16_builder::String16Builder;

mod search_util {
    use super::*;
    use string16::String16;

    fn find_magic_comment(content: &String16, name: &str, multiline: bool) -> String16 {
        assert!(!name.contains("="));
        let length = content.length();
        let name_length = name.len();
        let name16 = String16::from_str(name);

        let mut pos = length;
        let mut equal_sign_pos = 0;
        let mut closing_comment_pos = 0;

        loop {
            pos = content.reverse_find(name, pos);
            if pos == String16::K_NOT_FOUND {
                return String16::from_str("");
            }

            if pos < 4 {
                return String16::from_str("");
            }
            let pos_usize = pos as usize;
            if content.get(pos_usize) != Some('/' as u16) {
                continue;
            }

            if (content.get(pos_usize + 1) != Some('/' as u16) || multiline)
                && (content.get(pos_usize + 1) != Some('*' as u16) || !multiline)
            {
                continue;
            }
            if content.get(pos_usize + 2) != Some('#' as u16) && content.get(pos_usize + 2) != Some('@' as u16) {
                continue;
            }
            if content.get(pos_usize + 3) != Some(' ' as u16) && content.get(pos_usize + 3) != Some('\t' as u16) {
                continue;
            }

            equal_sign_pos = pos + 4 + name_length;
            if equal_sign_pos >= length {
                continue;
            }
            if content.get(equal_sign_pos) != Some('=' as u16) {
                continue;
            }

            if multiline {
                closing_comment_pos = content.find("*/", equal_sign_pos + 1);
                if closing_comment_pos == String16::K_NOT_FOUND {
                    return String16::from_str("");
                }
            }
            break;
        }

        assert!(equal_sign_pos > 0);
        assert!(equal_sign_pos < length);
        assert!(!multiline || closing_comment_pos > 0);

        let url_pos = equal_sign_pos + 1;
        let match_str = if multiline {
            content.substring(url_pos, closing_comment_pos)
        } else {
            content.substring(url_pos, content.length())
        };

        let new_line = match_str.find("\n", 0);
        let mut match_str = match_str;
        if new_line != String16::K_NOT_FOUND {
            match_str = match_str.substring(0, new_line);
        }
        let match_str = match_str.strip_white_space();

        for i in 0..match_str.length() {
            let c = match_str.get(i).unwrap();
            if c == '"' as u16 || c == '\'' as u16 || c == ' ' as u16 || c == '\t' as u16 {
                return String16::from_str("");
            }
        }

        match_str
    }

    fn create_search_regex_source(text: &String16) -> String16 {
        let mut result = String16Builder::new();

        for i in 0..text.length() {
            let c = text.get(i).unwrap();
            match c {
                '[' as u16 | ']' as u16 | '(' as u16 | ')' as u16 | '{' as u16 | '}' as u16
                | '+' as u16 | '-' as u16 | '*' as u16 | '.' as u16 | ',' as u16 | '?' as u16
                | '\\' as u16 | '^' as u16 | '$' as u16 | '|' as u16 => {
                    result.append('\\' as u16);
                }
                _ => {}
            }
            result.append(c);
        }

        result.to_string()
    }

    fn line_endings(text: &String16) -> Vec<usize> {
        let line_end_string = String16::from_str("\n");
        let mut result = Vec::new();
        let mut start = 0;

        while start < text.length() {
            let line_end = text.find("\n", start);
            if line_end == String16::K_NOT_FOUND {
                break;
            }

            result.push(line_end);
            start = line_end + 1;
        }
        result.push(text.length());
        result
    }

    fn script_regexp_matches_by_lines(regex: &V8Regex, text: &String16) -> Vec<(i32, String)> {
        let mut result = Vec::new();
        if text.is_empty() {
            return result;
        }

        let endings = line_endings(text);
        let size = endings.len();
        let mut start = 0;

        for line_number in 0..size {
            let line_end = endings[line_number];
            let mut line = text.substring(start, line_end);

            if line.length() > 0 && line.get(line.length() - 1) == Some('\r' as u16) {
                line = line.substring(0, line.length() - 1);
            }

            if regex.match_(&line.to_string(), 0).is_some() {
                result.push((line_number as i32, line.to_string()));
            }

            start = line_end + 1;
        }
        result
    }

    fn build_object_for_search_match(line_number: i32, line_content: &String) -> Option<protocol::Debugger::SearchMatch> {
        protocol::Debugger::SearchMatch::create()
            .set_line_number(line_number)
            .set_line_content(line_content.clone())
            .build()
    }

    fn create_search_regex(inspector: &dyn V8Inspector, query: &String16, case_sensitive: bool, is_regex: bool) -> V8Regex {
        let regex_source = if is_regex {
            query.clone()
        } else {
            create_search_regex_source(query)
        };
        V8Regex::new(inspector, regex_source.to_string(), case_sensitive)
    }

    pub fn search_in_text_by_lines_impl(session: &dyn V8InspectorSession, text: &String16, query: &String16, case_sensitive: bool, is_regex: bool) -> Vec<protocol::Debugger::SearchMatch> {
        let inspector = session.inspector();
        let regex = create_search_regex(inspector, query, case_sensitive, is_regex);
        let matches = script_regexp_matches_by_lines(&regex, text);

        let mut result = Vec::with_capacity(matches.len());
        for match_ in matches {
            if let Some(obj) = build_object_for_search_match(match_.0, &match_.1) {
                result.push(obj);
            }
        }
        result
    }

    pub fn find_source_url(content: &String16, multiline: bool) -> String16 {
        find_magic_comment(content, "sourceURL", multiline)
    }

    pub fn find_source_map_url(content: &String16, multiline: bool) -> String16 {
        find_magic_comment(content, "sourceMappingURL", multiline)
    }
}

pub use search_util::*;