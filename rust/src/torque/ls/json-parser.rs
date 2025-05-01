// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;

mod earley_parser;
use earley_parser::*;

mod source_file_map;
use source_file_map::*;

mod torque_messages;
use torque_messages::*;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(JsonArray),
    Object(JsonObject),
}

impl JsonValue {
    fn from_bool(value: bool) -> Self {
        JsonValue::Bool(value)
    }

    fn from_number(value: f64) -> Self {
        JsonValue::Number(value)
    }

    fn from_string(value: String) -> Self {
        JsonValue::String(value)
    }

    fn from_array(value: JsonArray) -> Self {
        JsonValue::Array(value)
    }

    fn from_object(value: JsonObject) -> Self {
        JsonValue::Object(value)
    }

    fn from(value: bool) -> Self {
        JsonValue::Bool(value)
    }

    fn from_f64(value: f64) -> Self {
        JsonValue::Number(value)
    }
    fn from(value: f64) -> Self {
        JsonValue::from_f64(value)
    }

    fn from(value: String) -> Self {
        JsonValue::from_string(value)
    }

    fn from(value: JsonArray) -> Self {
        JsonValue::from_array(value)
    }

    fn from(value: JsonObject) -> Self {
        JsonValue::from_object(value)
    }
}

pub type JsonArray = Vec<JsonValue>;
pub type JsonObject = HashMap<String, JsonValue>;

// ParseResultTypeId is not used in the provided C++ code
// so there is no equivalent in this converted Rust code.

mod ls {
    use super::*;

    pub type JsonMember = (String, JsonValue);

    fn make_bool_literal(child_results: &mut ParseResultIterator) -> Option<ParseResult> {
        if let Some(ParseResult { value }) = child_results.next() {
            if let Some(bool_val) = value.downcast_ref::<bool>() {
                return Some(ParseResult {
                    value: Box::new(JsonValue::from_bool(*bool_val)),
                });
            }
        }
        None
    }

    fn make_null_literal(_child_results: &mut ParseResultIterator) -> Option<ParseResult> {
        Some(ParseResult {
            value: Box::new(JsonValue::Null),
        })
    }

    fn make_number_literal(child_results: &mut ParseResultIterator) -> Option<ParseResult> {
        if let Some(ParseResult { value }) = child_results.next() {
            if let Some(number_str) = value.downcast_ref::<String>() {
                if let Ok(d) = number_str.parse::<f64>() {
                    return Some(ParseResult {
                        value: Box::new(JsonValue::from_number(d)),
                    });
                }
            }
        }
        None
    }

    fn make_string_literal(child_results: &mut ParseResultIterator) -> Option<ParseResult> {
        if let Some(ParseResult { value }) = child_results.next() {
            if let Some(literal) = value.downcast_ref::<String>() {
                let unquoted = string_literal_unquote(literal.clone());
                return Some(ParseResult {
                    value: Box::new(JsonValue::from_string(unquoted)),
                });
            }
        }
        None
    }

    fn make_array(child_results: &mut ParseResultIterator) -> Option<ParseResult> {
        if let Some(ParseResult { value }) = child_results.next() {
            if let Some(array) = value.downcast_ref::<JsonArray>() {
                return Some(ParseResult {
                    value: Box::new(JsonValue::from_array(array.clone())),
                });
            }
        }
        None
    }

    fn make_member(child_results: &mut ParseResultIterator) -> Option<ParseResult> {
        let mut results = child_results;
        let key_result = results.next();
        let value_result = results.next();

        if let (Some(ParseResult { value: Some(key_box) }), Some(ParseResult { value: Some(value_box) })) = (key_result, value_result) {
            if let (Some(key), Some(value)) = (key_box.downcast_ref::<String>(), value_box.downcast_ref::<JsonValue>()) {
                let unquoted_key = string_literal_unquote(key.clone());
                let result = (unquoted_key, value.clone());
                return Some(ParseResult { value: Box::new(result) });
            }
        }
        None
    }

    fn make_object(child_results: &mut ParseResultIterator) -> Option<ParseResult> {
        if let Some(ParseResult { value }) = child_results.next() {
            if let Some(members) = value.downcast_ref::<Vec<JsonMember>>() {
                let mut object = JsonObject::new();
                for (key, value) in members {
                    object.insert(key.clone(), value.clone());
                }
                return Some(ParseResult {
                    value: Box::new(JsonValue::from_object(object)),
                });
            }
        }
        None
    }

    struct JsonGrammar {
        grammar: Grammar,
        true_literal: Symbol,
        false_literal: Symbol,
        null_literal: Symbol,
        decimal_literal: Symbol,
        string_literal: Symbol,
        element_list: Symbol,
        array: Symbol,
        member: Symbol,
        member_list: Symbol,
        object: Symbol,
        value: Symbol,
        file: Symbol,
    }

    impl JsonGrammar {
        fn new() -> Self {
            let mut grammar = Grammar::new(&Symbol::new(Rule::new(vec![]))); // Dummy value, will be replaced later.
            grammar.set_whitespace(Self::match_whitespace);

            let true_literal = Symbol::new(Rule::new(vec![Token::new("true")]));
            let false_literal = Symbol::new(Rule::new(vec![Token::new("false")]));
            let null_literal = Symbol::new(Rule::new(vec![Token::new("null")]));

            let decimal_literal = Symbol::new(Rule::new(vec![Pattern(Self::match_decimal_literal), YieldMatchedInput]));
            let decimal_literal2 = Symbol::new(Rule::new(vec![Pattern(Self::match_hex_literal), YieldMatchedInput]));

            let string_literal = Symbol::new(Rule::new(vec![Pattern(Self::match_string_literal), YieldMatchedInput]));

            let value = Symbol::new(Rule::new(vec![]));
            let element_list = Grammar::list::<JsonValue>(&value, Token::new(","));
            let array = Symbol::new(Rule::new(vec![Token::new("["), element_list.clone(), Token::new("]") ]));

            let member = Symbol::new(Rule::new(vec![string_literal.clone(), Token::new(":"), value.clone()], Some(make_member)));
            let member_list = Grammar::list::<JsonMember>(&member, Token::new(","));
            let object = Symbol::new(Rule::new(vec![Token::new("{"), member_list.clone(), Token::new("}")]));

            let true_literal_rule = Rule::new(vec![true_literal.clone()], Some(|child_results: &mut ParseResultIterator| {
                Some(ParseResult {
                    value: Box::new(true),
                })
            }));

            let false_literal_rule = Rule::new(vec![false_literal.clone()], Some(|child_results: &mut ParseResultIterator| {
                Some(ParseResult {
                    value: Box::new(false),
                })
            }));

            let null_literal_rule = Rule::new(vec![null_literal.clone()], Some(make_null_literal));
            let decimal_literal_rule = Rule::new(vec![decimal_literal.clone()], Some(make_number_literal));
            let decimal_literal_rule2 = Rule::new(vec![decimal_literal2.clone()], Some(make_number_literal));
            let string_literal_rule = Rule::new(vec![string_literal.clone()], Some(make_string_literal));
            let object_rule = Rule::new(vec![object.clone()], Some(make_object));
            let array_rule = Rule::new(vec![array.clone()], Some(make_array));
            let value_rules = vec![
                true_literal_rule,
                false_literal_rule,
                null_literal_rule,
                decimal_literal_rule,
                decimal_literal_rule2,
                string_literal_rule,
                object_rule,
                array_rule,
            ];

            let mut value_symbol = Symbol::new(Rule::new(vec![]));
            for rule in value_rules{
                value_symbol.add_rule(rule);
            }
            let file = Symbol::new(Rule::new(vec![value_symbol.clone()]));

            grammar.start_symbol = &file;

            Self {
                grammar,
                true_literal,
                false_literal,
                null_literal,
                decimal_literal,
                string_literal,
                element_list,
                array,
                member,
                member_list,
                object,
                value: value_symbol,
                file,
            }
        }

        fn match_whitespace(pos: &mut InputPosition) -> bool {
            while pos.peek().map_or(false, |c| c.is_whitespace()) {
                pos.next();
            }
            true
        }

        fn match_string_literal(pos: &mut InputPosition) -> bool {
            let mut current = pos.clone();
            if Grammar::match_string("\"", &mut current) {
                while Grammar::match_string("\\", &mut current) && Grammar::match_any_char(&mut current)
                    || current.peek().map_or(false, |c| c != '"' && c != '\n') && Grammar::match_any_char(&mut current)
                {
                    // Intentionally empty loop body
                }
                if Grammar::match_string("\"", &mut current) {
                    *pos = current;
                    return true;
                }
            }

            current = pos.clone();
            if Grammar::match_string("'", &mut current) {
                while Grammar::match_string("\\", &mut current) && Grammar::match_any_char(&mut current)
                    || current.peek().map_or(false, |c| c != '\'' && c != '\n') && Grammar::match_any_char(&mut current)
                {
                    // Intentionally empty loop body
                }
                if Grammar::match_string("'", &mut current) {
                    *pos = current;
                    return true;
                }
            }

            false
        }

        fn match_hex_literal(pos: &mut InputPosition) -> bool {
            let mut current = pos.clone();
            Grammar::match_string("-", &mut current);
            if Grammar::match_string("0x", &mut current) && current.peek().map_or(false, |c| c.is_ascii_hexdigit()) && Grammar::match_any_char(&mut current){
                while current.peek().map_or(false, |c| c.is_ascii_hexdigit()) && Grammar::match_any_char(&mut current) {
                    // Intentionally empty loop body
                }
                *pos = current;
                return true;
            }
            false
        }

        fn match_decimal_literal(pos: &mut InputPosition) -> bool {
            let mut current = pos.clone();
            let mut found_digit = false;

            Grammar::match_string("-", &mut current);

            while current.peek().map_or(false, |c| c.is_ascii_digit()) && Grammar::match_any_char(&mut current) {
                found_digit = true;
            }

            Grammar::match_string(".", &mut current);

            while current.peek().map_or(false, |c| c.is_ascii_digit()) && Grammar::match_any_char(&mut current) {
                found_digit = true;
            }

            if !found_digit {
                return false;
            }

            *pos = current.clone();

            if (Grammar::match_string("e", &mut current) || Grammar::match_string("E", &mut current))
                && (Grammar::match_string("+", &mut current) || Grammar::match_string("-", &mut current) || true)
                && current.peek().map_or(false, |c| c.is_ascii_digit()) && Grammar::match_any_char(&mut current)
            {
                while current.peek().map_or(false, |c| c.is_ascii_digit()) && Grammar::match_any_char(&mut current) {
                    // Intentionally empty loop body
                }
                *pos = current;
                return true;
            }

            true
        }
    }

    pub struct JsonParserResult {
        pub value: Option<JsonValue>,
        pub error: Option<TorqueMessage>,
    }

    pub fn parse_json(input: &str) -> JsonParserResult {
        let source_map_scope = SourceFileMap::scope("<json>".to_string());
        let messages_scope = TorqueMessages::scope();
        let unknown_file = CurrentSourceFile::scope(SourceFileMap::add_source("<json>".to_string()));

        let mut result = JsonParserResult {
            value: None,
            error: None,
        };

        let grammar = JsonGrammar::new();
        match grammar.grammar.parse(input) {
            Ok(parse_result) => {
                if let Some(boxed_value) = parse_result.value {
                    if let Some(json_value) = boxed_value.downcast_ref::<JsonValue>() {
                        result.value = Some(json_value.clone());
                    }
                }
            },
            Err(_) => {
                if !TorqueMessages::get().is_empty() {
                    result.error = TorqueMessages::get().first().cloned();
                }
            }
        }

        result
    }
}

fn string_literal_unquote(literal: String) -> String {
    let mut result = String::new();
    let mut chars = literal.chars();

    match chars.next() {
        Some('"') | Some('\'') => {},
        _ => return literal, // Not a quoted string literal
    };
    let quote_char = literal.chars().next().unwrap();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                match chars.next() {
                    Some('"' ) => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('/') => result.push('/'),
                    Some('b') => result.push('\x08'),
                    Some('f') => result.push('\x0C'),
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some(other) => {
                        result.push('\\');
                        result.push(other);
                    }
                    None => {
                        result.push('\\');
                        break;
                    }
                }
            }
            c if c == quote_char => break,
            _ => result.push(c),
        }
    }
    result
}