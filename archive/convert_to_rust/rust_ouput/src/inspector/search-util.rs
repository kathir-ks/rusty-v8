// Converted from V8 C++ source files:
// Header: search-util.h
// Implementation: search-util.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    pub mod Debugger {
        #[derive(Debug, Clone)]
        pub struct SearchMatch {
            line_number: i32,
            line_content: String,
        }

        impl SearchMatch {
            pub fn create() -> SearchMatchBuilder {
                SearchMatchBuilder::new()
            }

            pub fn line_number(&self) -> i32 {
                self.line_number
            }

            pub fn line_content(&self) -> &str {
                &self.line_content
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

            pub fn setLineNumber(mut self, line_number: i32) -> Self {
                self.line_number = Some(line_number);
                self
            }

            pub fn setLineContent(mut self, line_content: String) -> Self {
                self.line_content = Some(line_content);
                self
            }

            pub fn build(self) -> Box<SearchMatch> {
                Box::new(SearchMatch {
                    line_number: self.line_number.unwrap_or(0),
                    line_content: self.line_content.unwrap_or_default(),
                })
            }
        }
    }
}

mod string_util {
    pub const kNotFound: usize = usize::MAX;

    pub trait String16 {
        fn len(&self) -> usize;
        fn is_empty(&self) -> bool {
            self.len() == 0
        }
        fn find(&self, needle: &str, start: usize) -> usize;
        fn reverse_find(&self, needle: &str, start: usize) -> usize;
        fn substring(&self, start: usize, end: usize) -> String;
        fn strip_whitespace(&self) -> String;
        fn as_str(&self) -> &str;
        fn char_at(&self, index: usize) -> Option<char>;
    }

    impl String16 for String {
        fn len(&self) -> usize {
            self.len()
        }

        fn find(&self, needle: &str, start: usize) -> usize {
            if start >= self.len() {
                return kNotFound;
            }
            match self[start..].find(needle) {
                Some(index) => start + index,
                None => kNotFound,
            }
        }

        fn reverse_find(&self, needle: &str, start: usize) -> usize {
            if start >= self.len() {
                return kNotFound;
            }

            let mut pos = kNotFound;
            let mut current_pos = 0;

            while let Some(index) = self[current_pos..start].find(needle) {
                pos = current_pos + index;
                current_pos += index + needle.len();
            }

            pos
        }


        fn substring(&self, start: usize, end: usize) -> String {
            if start >= self.len() || end > self.len() || start > end {
                return String::new();
            }
            self[start..end].to_string()
        }

        fn strip_whitespace(&self) -> String {
            self.trim().to_string()
        }

        fn as_str(&self) -> &str {
            self.as_str()
        }

        fn char_at(&self, index: usize) -> Option<char> {
            self.chars().nth(index)
        }
    }

    pub struct String16Builder {
        result: String,
    }

    impl String16Builder {
        pub fn new() -> Self {
            String16Builder { result: String::new() }
        }

        pub fn append(&mut self, c: char) {
            self.result.push(c);
        }

        pub fn toString(&self) -> String {
            self.result.clone()
        }
    }
}

use string_util::{String16, String16Builder, kNotFound};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SearchError {
    RegexError(String),
    GenericError(String),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchError::RegexError(e) => write!(f, "Regex error: {}", e),
            SearchError::GenericError(e) => write!(f, "Generic error: {}", e),
        }
    }
}

impl Error for SearchError {}

pub struct V8InspectorSession {}

impl V8InspectorSession {
    pub fn new() -> Self {
        V8InspectorSession {}
    }
}

pub fn findSourceURL(content: &String, multiline: bool) -> String {
    findMagicComment(content, "sourceURL", multiline)
}

pub fn findSourceMapURL(content: &String, multiline: bool) -> String {
    findMagicComment(content, "sourceMappingURL", multiline)
}

fn findMagicComment(content: &String, name: &str, multiline: bool) -> String {
    if name.contains("=") {
        return String::new();
    }

    let length = content.len();
    let name_length = name.len();

    let mut pos = length;
    let mut equal_sign_pos = 0;
    let mut closing_comment_pos = 0;

    loop {
        pos = content.reverse_find(name, pos);
        if pos == kNotFound {
            return String::new();
        }

        if pos < 4 {
            return String::new();
        }
        let pos_minus_4 = pos - 4;

        if content.char_at(pos_minus_4) != Some('/') {
            continue;
        }
        if (content.char_at(pos_minus_4 + 1) != Some('/') || multiline) &&
           (content.char_at(pos_minus_4 + 1) != Some('*') || !multiline) {
            continue;
        }

        if content.char_at(pos_minus_4 + 2) != Some('#') && content.char_at(pos_minus_4 + 2) != Some('@') {
            continue;
        }

        if content.char_at(pos_minus_4 + 3) != Some(' ') && content.char_at(pos_minus_4 + 3) != Some('\t') {
            continue;
        }

        equal_sign_pos = pos_minus_4 + 4 + name_length;

        if equal_sign_pos >= length {
            continue;
        }

        if content.char_at(equal_sign_pos) != Some('=') {
            continue;
        }

        if multiline {
            closing_comment_pos = content.find("*/", equal_sign_pos + 1);
            if closing_comment_pos == kNotFound {
                return String::new();
            }
        }
        break;
    }

    let url_pos = equal_sign_pos + 1;

    let mut match_str = if multiline {
        content.substring(url_pos, closing_comment_pos)
    } else {
        content.substring(url_pos, length)
    };

    let new_line = match_str.find("\n", 0);
    if new_line != kNotFound {
        match_str = match_str.substring(0, new_line);
    }

    match_str = match_str.strip_whitespace();

    for i in 0..match_str.len() {
        match match_str.char_at(i) {
            Some('"') | Some('\'') | Some(' ') | Some('\t') => return String::new(),
            _ => {}
        }
    }

    match_str
}

fn createSearchRegexSource(text: &String) -> String {
    let mut result = String16Builder::new();

    for c in text.chars() {
        match c {
            '[' | ']' | '(' | ')' | '{' | '}' | '+' | '-' | '*' | '.' | ',' | '?' | '\\' | '^' | '$' | '|' => {
                result.append('\\');
            }
            _ => {}
        }
        result.append(c);
    }

    result.toString()
}

fn lineEndings(text: &String) -> Vec<usize> {
    let mut result = Vec::new();
    let line_end_string = "\n";
    let mut start = 0;

    while start < text.len() {
        let line_end = text.find(line_end_string, start);
        if line_end == kNotFound {
            break;
        }

        result.push(line_end);
        start = line_end + 1;
    }

    result.push(text.len());
    result
}

struct V8Regex {
    regex: regex::Regex,
}

impl V8Regex {
    fn new(inspector: &V8InspectorImpl, source: String, case_sensitive: bool) -> Result<Self, SearchError> {
        let regex_string = if case_sensitive {
            source
        } else {
            format!("(?i){}", source)
        };

        let regex = regex::Regex::new(&regex_string).map_err(|e| SearchError::RegexError(e.to_string()))?;
        Ok(V8Regex { regex })
    }

    fn match_(&self, text: &str, start: usize) -> Option<usize> {
        self.regex.find_at(text, start).map(|m| m.start())
    }

    fn match_length(&self, text: &str) -> Option<usize> {
        self.regex.find(text).map(|m| m.len())
    }
}

fn scriptRegexpMatchesByLines(regex: &V8Regex, text: &String) -> Vec<(i32, String)> {
    let mut result = Vec::new();
    if text.is_empty() {
        return result;
    }

    let endings = lineEndings(text);
    let size = endings.len();
    let mut start = 0;

    for line_number in 0..size {
        let line_end = endings[line_number];
        let mut line = text.substring(start, line_end);

        if !line.is_empty() && line.char_at(line.len() - 1) == Some('\r') {
            line = line.substring(0, line.len() - 1);
        }

        if regex.match_length(&line).is_some() {
            result.push((line_number as i32, line));
        }

        start = line_end + 1;
    }

    result
}

fn buildObjectForSearchMatch(line_number: i32, line_content: String) -> Box<protocol::Debugger::SearchMatch> {
    protocol::Debugger::SearchMatch::create()
        .setLineNumber(line_number)
        .setLineContent(line_content)
        .build()
}

struct V8InspectorImpl {}

impl V8InspectorImpl {
    fn new() -> Self {
        V8InspectorImpl {}
    }
}

struct V8InspectorSessionImpl {
    inspector: V8InspectorImpl,
}

impl V8InspectorSessionImpl {
    fn new() -> Self {
        V8InspectorSessionImpl {
            inspector: V8InspectorImpl::new(),
        }
    }

    fn inspector(&self) -> &V8InspectorImpl {
        &self.inspector
    }
}

fn createSearchRegex(inspector: &V8InspectorImpl, query: &String, case_sensitive: bool, is_regex: bool) -> Result<V8Regex, SearchError> {
    let regex_source = if is_regex {
        query.clone()
    } else {
        createSearchRegexSource(query)
    };

    V8Regex::new(inspector, regex_source, case_sensitive)
}

pub fn searchInTextByLinesImpl(session: &V8InspectorSession, text: &String, query: &String, case_sensitive: bool, is_regex: bool) -> Vec<Box<protocol::Debugger::SearchMatch>> {
    let session_impl = V8InspectorSessionImpl::new();
    let regex_result = createSearchRegex(&session_impl.inspector(), query, case_sensitive, is_regex);

    match regex_result {
        Ok(regex) => {
            let matches = scriptRegexpMatchesByLines(&regex, text);
            let mut result = Vec::with_capacity(matches.len());

            for match_item in matches {
                result.push(buildObjectForSearchMatch(match_item.0, match_item.1));
            }

            result
        }
        Err(_) => Vec::new(),
    }
}
