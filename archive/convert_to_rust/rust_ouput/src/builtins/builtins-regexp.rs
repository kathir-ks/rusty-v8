// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-regexp.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::string::String as StdString;
use std::vec::Vec;
use std::{ptr, fmt};
use std::convert::TryInto;
use std::fmt::Write;
use std::error::Error;
use crate::v8::V8;

pub struct Isolate {
    regexp_last_match_info_: Box<RegExpMatchInfo>,
}

impl Isolate {
    pub fn regexp_function(&self) -> &RegExpFunction {
        &RegExpFunction {}
    }
    pub fn factory(&self) -> Factory {
      Factory {}
    }
    pub fn CountUsage(&self, _usage: v8::Isolate::Usage) {}
    pub fn regexp_last_match_info(&self) -> &RegExpMatchInfo {
      &self.regexp_last_match_info_
    }
}

pub mod v8 {
    pub use crate::v8::V8;
    pub mod Isolate {
        pub enum Usage {
            kRegExpPrototypeToString,
        }
    }
}

pub struct Factory {}
impl Factory {
  pub fn source_string(&self) -> String {
    String {}
  }
  pub fn flags_string(&self) -> String {
    String {}
  }
  pub fn NewSubString(&self, _string: &String, _start: i32, _len: i32) -> String {
    String {}
  }
}

pub struct String {}
impl String {
  pub fn Flatten(_isolate: &Isolate, string: &String) -> String {
    String {}
  }
  pub fn IsOneByteRepresentation(&self) -> bool {
    true
  }
  pub fn GetFlatContent<'a>(&'a self, _no_gc: DisallowGarbageCollection) -> FlatStringContents<'a> {
    FlatStringContents{one_byte : true, string : StdString::from("dummy")}
  }
  pub fn length(&self) -> i32 {
    0
  }
}

pub struct FlatStringContents<'a>{
  one_byte : bool,
  string : StdString,
}

impl <'a> FlatStringContents<'a>{
    pub fn ToOneByteVector(&self) -> Vec<u8> {
      self.string.as_bytes().to_vec()
    }
    pub fn ToUC16Vector(&self) -> Vec<base::uc16> {
        let mut vec = Vec::new();
        for c in self.string.encode_utf16() {
            vec.push(base::uc16 { value: c });
        }
        vec
    }
}

pub struct Object {}
impl Object{
  pub fn ToString(_isolate: &Isolate, _object: &Handle<Object>) -> Result<String, Box<dyn Error>> {
    Ok(String{})
  }
}

pub struct JSReceiver {}
impl JSReceiver{
  pub fn GetProperty(_isolate: &Isolate, _receiver: &Handle<JSReceiver>, _name: String) -> Result<Handle<Object>, Box<dyn Error>>{
    Ok(Handle{value : Object{}})
  }
}

struct Handle<T> {
    value: T,
}

impl<T> Handle<T> {
    fn new(value: T) -> Self {
        Handle { value }
    }
}

pub struct ReadOnlyRoots {}

impl ReadOnlyRoots {
    pub fn empty_string(&self) -> String {
        String {}
    }
    pub fn undefined_value(&self) -> String {
      String {}
    }
}

pub struct RegExpFunction {}
impl RegExpFunction {
    pub fn prototype(&self) -> RegExpFunction {
        RegExpFunction {}
    }
}

pub struct RegExpMatchInfo {
    last_input_: String,
}

impl RegExpMatchInfo {
    pub fn last_input(&self) -> &String {
        &self.last_input_
    }
    pub fn set_last_input(&mut self, value: String) {
        self.last_input_ = value;
    }
    pub fn number_of_capture_registers(&self) -> i32 {
        0
    }
    pub fn capture(&self, _index: i32) -> i32 {
        0
    }
    pub fn last_subject(&self) -> &String {
      &String {}
    }
}

mod base {
  pub struct OwnedVector<T> {
    data : Vec<T>
  }

  impl <T : Copy> OwnedVector<T> {
    pub fn new(vec : Vec<T>) -> Self{
      OwnedVector{data : vec}
    }
    pub fn get(&self, index : usize) -> T {
      self.data[index]
    }
    pub fn size(&self) -> usize {
      self.data.len()
    }
  }

  pub struct ArrayVector<T> {
    data: Vec<T>,
  }
  impl<T> ArrayVector<T> {
      pub fn new(data: Vec<T>) -> Self {
          ArrayVector { data }
      }
  }

  pub struct uc16{
    value : u16,
  }
}

pub fn IsUndefined(_object: &Object, _isolate: &Isolate) -> bool {
    false
}

pub fn Cast<T>(_object: &Object) -> &T {
    unsafe { &*(ptr::null() as *const T) }
}

pub fn NewTypeError(_message_template: MessageTemplate, _string: String) -> TypeError {
    TypeError {}
}

pub enum MessageTemplate {
    kArgumentIsNonString,
}

pub struct TypeError {}

pub struct IncrementalStringBuilder {
    data: StdString,
}

impl IncrementalStringBuilder {
    pub fn new(_isolate: &Isolate) -> IncrementalStringBuilder {
        IncrementalStringBuilder { data: StdString::new() }
    }

    pub fn AppendCharacter(&mut self, c: char) {
        self.data.push(c);
    }

    pub fn AppendString(&mut self, string: &String) {
        //self.data.push_str(string.as_str());
    }

    pub fn Finish(&self) -> Result<String, Box<dyn Error>> {
        Ok(String{})
    }
    pub fn ChangeEncoding(&mut self) {}
    pub fn AppendCStringLiteral(&mut self, literal: &str) {
        self.data.push_str(literal);
    }
    pub fn Append<CharT, CharT0>(&mut self, cp: CharT0)
    where
        CharT: fmt::Display,
    {
        self.data.push_str(&cp.to_string());
    }
}

pub mod RegExpUtils {
    use super::*;
    use std::error::Error;

    pub fn GenericCaptureGetter(
        _isolate: &Isolate,
        _match_info: &RegExpMatchInfo,
        _index: i32,
    ) -> Result<String, Box<dyn Error>> {
        Ok(String {})
    }
}

pub fn IsString(_value: &Object) -> bool {
    true
}

pub fn DoubleToRadixStringView<T>(_value: T, _radix: i32, _buffer: base::ArrayVector<char>) -> StdString
where T : Into<i64>{
  StdString::from("dummy")
}

pub fn IsAscii(_c: base::uc16) -> bool {
    true
}

pub fn IsWhiteSpaceOrLineTerminator(_cp: u32) -> bool {
    false
}

pub struct DisallowGarbageCollection {}
impl DisallowGarbageCollection{
  pub fn new() -> Self{
    DisallowGarbageCollection{}
  }
}

pub fn THROW_NEW_ERROR_RETURN_FAILURE<T>(_isolate : &Isolate, error : TypeError) -> Result<T, Box<dyn Error>> {
  Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")))
}

pub fn ASSIGN_RETURN_FAILURE_ON_EXCEPTION<T>(_isolate : &Isolate, _str : String, result : Result<String, Box<dyn Error>>) -> Result<(), Box<dyn Error>>{
  match result{
    Ok(_)=>{
      Ok(())
    },
    Err(e)=>{
      return Err(e);
    }
  }
}

impl <T : Copy> base::OwnedVector<T> {
  pub fn OwnedCopyOf(vec : Vec<T>) -> Self {
    base::OwnedVector{data : vec}
  }
}

impl base::OwnedVector<const u8> {
    
}

mod unibrow {
  pub mod Utf16{
    pub fn IsLeadSurrogate(_c : super::super::base::uc16) -> bool{
      false
    }
    pub fn IsTrailSurrogate(_c : super::super::base::uc16) -> bool{
      false
    }
    pub fn CombineSurrogatePair(_c : super::super::base::uc16, _d : super::super::base::uc16) -> u32{
      0
    }
    pub fn IsSurrogatePair(_c : super::super::base::uc16, _d : super::super::base::uc16) -> bool{
      false
    }
  }
}

fn RegExpPrototypeToString(isolate: &Isolate, recv: &JSReceiver) -> Result<String, Box<dyn Error>> {
    let scope = HandleScope {};
    check_receiver(recv, "RegExp.prototype.toString")?;

    if ptr::eq(recv, unsafe { ptr::read(isolate.regexp_function() as *const _ as *const JSReceiver) }) {
        isolate.CountUsage(v8::Isolate::Usage::kRegExpPrototypeToString);
    }

    let mut builder = IncrementalStringBuilder::new(isolate);

    builder.AppendCharacter('/');

    let source: Handle<Object> = {
        let source_result = JSReceiver::GetProperty(isolate, &Handle::new(*recv), isolate.factory().source_string());
        match source_result {
            Ok(source) => source,
            Err(e) => return Err(e),
        }
    };
    
    let source_str_result = Object::ToString(isolate, &source);
    match source_str_result {
      Ok(source_str) => builder.AppendString(&source_str),
      Err(e) => return Err(e),
    };

    builder.AppendCharacter('/');

    let flags: Handle<Object> = {
        let flags_result = JSReceiver::GetProperty(isolate, &Handle::new(*recv), isolate.factory().flags_string());
        match flags_result {
            Ok(flags) => flags,
            Err(e) => return Err(e),
        }
    };
    let flags_str_result = Object::ToString(isolate, &flags);
    match flags_str_result {
      Ok(flags_str) => builder.AppendString(&flags_str),
      Err(e) => return Err(e),
    };

    match builder.Finish() {
      Ok(r) => Ok(r),
      Err(e) => Err(e),
    }
}

fn check_receiver(_recv: &JSReceiver, _method_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

macro_rules! define_capture_getter {
    ($i:expr) => {
        fn RegExpCaptureGetter(_isolate: &Isolate) -> Result<String, Box<dyn Error>> {
            let _scope = HandleScope {};
            match RegExpUtils::GenericCaptureGetter(_isolate, &_isolate.regexp_last_match_info(), $i) {
              Ok(r) => Ok(r),
              Err(e) => Err(e),
            }
        }
    };
}

define_capture_getter!(1);
define_capture_getter!(2);
define_capture_getter!(3);
define_capture_getter!(4);
define_capture_getter!(5);
define_capture_getter!(6);
define_capture_getter!(7);
define_capture_getter!(8);
define_capture_getter!(9);

fn RegExpInputGetter(isolate: &Isolate) -> Result<String, Box<dyn Error>> {
    let scope = HandleScope {};
    let obj = Handle::new(*isolate.regexp_last_match_info().last_input());
    if IsUndefined(&obj.value, isolate) {
        Ok(ReadOnlyRoots {}.empty_string())
    } else {
        Ok(Cast::<String>(&obj.value).clone())
    }
}

fn RegExpInputSetter(isolate: &mut Isolate, args: &Arguments) -> Result<String, Box<dyn Error>> {
    let scope = HandleScope {};
    let value = args.atOrUndefined(isolate, 1);
    let str_result = Object::ToString(isolate, &value);
    match str_result {
      Ok(str) => {
        isolate.regexp_last_match_info_.set_last_input(str);
        Ok(ReadOnlyRoots {}.undefined_value())
      },
      Err(e) => Err(e),
    }
}

fn RegExpLastMatchGetter(isolate: &Isolate) -> Result<String, Box<dyn Error>> {
    let scope = HandleScope {};
    match RegExpUtils::GenericCaptureGetter(isolate, &isolate.regexp_last_match_info(), 0) {
      Ok(r) => Ok(r),
      Err(e) => Err(e),
    }
}

fn RegExpLastParenGetter(isolate: &Isolate) -> Result<String, Box<dyn Error>> {
    let scope = HandleScope {};
    let match_info = isolate.regexp_last_match_info();
    let length = match_info.number_of_capture_registers();
    if length <= 2 {
        return Ok(ReadOnlyRoots {}.empty_string());
    }

    if length % 2 != 0 {
        panic!();
    }
    let last_capture = (length / 2) - 1;
    match RegExpUtils::GenericCaptureGetter(isolate, match_info, last_capture) {
      Ok(r) => Ok(r),
      Err(e) => Err(e),
    }
}

fn RegExpLeftContextGetter(isolate: &Isolate) -> Result<String, Box<dyn Error>> {
    let scope = HandleScope {};
    let match_info = isolate.regexp_last_match_info();
    let start_index = match_info.capture(0);
    let last_subject = Handle::new(*match_info.last_subject());
    Ok(isolate.factory().NewSubString(&last_subject.value, 0, start_index))
}

fn RegExpRightContextGetter(isolate: &Isolate) -> Result<String, Box<dyn Error>> {
    let scope = HandleScope {};
    let match_info = isolate.regexp_last_match_info();
    let start_index = match_info.capture(1);
    let last_subject = Handle::new(*match_info.last_subject());
    let len = last_subject.value.length();
    Ok(isolate.factory().NewSubString(&last_subject.value, start_index, len))
}

const K_NO_ESCAPE: u8 = 0;
const K_ESCAPE_TO_HEX: u8 = std::u8::MAX;

const fn get_ascii_escape(c: char) -> u8 {
    match c {
        '^' | '$' | '\\' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '/' => c as u8,
        '\f' => 'f' as u8,
        '\n' => 'n' as u8,
        '\r' => 'r' as u8,
        '\t' => 't' as u8,
        '\v' => 'v' as u8,
        ',' | '-' | '=' | '<' | '>' | '#' | '&' | '!' | '%' | ':' | ';' | '@' | '~' | '\'' | '`' | '"' | ' ' => K_ESCAPE_TO_HEX,
        _ => K_NO_ESCAPE,
    }
}

const K_ASCII_ESCAPES: [u8; 128] = {
    let mut escapes = [0u8; 128];
    let mut i = 0;
    while i < 128 {
        escapes[i] = get_ascii_escape(i as u8 as char);
        i += 1;
    }
    escapes
};

fn RegExpEscapeImpl<CharT>(isolate: &Isolate, source: base::OwnedVector<CharT>) -> Result<String, Box<dyn Error>>
where
    CharT: Copy + PartialEq + TryInto<u32> + Into<i64>,
    <CharT as TryInto<u32>>::Error: std::fmt::Debug,
{
    let mut double_to_radix_chars = [0u8; 100];
    let double_to_radix_buffer = base::ArrayVector::new(double_to_radix_chars.to_vec());
    let mut escaped_builder = IncrementalStringBuilder::new(isolate);

    if std::mem::size_of::<CharT>() == 2 {
        escaped_builder.ChangeEncoding();
    }

    let first_c = source.get(0);
    let mut start;
    if is_alpha_numeric(first_c) {
        start = 1;
        escaped_builder.AppendCStringLiteral("\\x");
        let hex = DoubleToRadixStringView(first_c.into(), 16, double_to_radix_buffer);
        escaped_builder.AppendString(&String {});
    } else {
        start = 0;
    }

    for i in start..source.size() {
        let cu = source.get(i);
        let mut cp: u32 = match cu.try_into() {
            Ok(c) => c,
            Err(e) => {
                println!("conversion error: {:?}", e);
                0
            }
        };
        let mut cmd = K_NO_ESCAPE;

        if (cu as u32) < 128 {
            cmd = K_ASCII_ESCAPES[cu as usize];
        } else {
            if std::mem::size_of::<CharT>() == 2 {
                if is_lead_surrogate(cu) {
                    if i + 1 < source.size() && is_trail_surrogate(source.get(i + 1)) {
                        cp = combine_surrogate_pair(cu, source.get(i + 1));
                    } else {
                        cmd = K_ESCAPE_TO_HEX;
                    }
                } else if is_trail_surrogate(cu) {
                    cmd = K_ESCAPE_TO_HEX;
                }
            }
            if is_white_space_or_line_terminator(cp) {
                cmd = K_ESCAPE_TO_HEX;
            }
        }

        if cmd == K_NO_ESCAPE {
            if cp == (cu as u32) {
                escaped_builder.Append::<CharT, CharT>(cu);
            } else {
                if is_surrogate_pair(cu, source.get(i)) {
                    cp = combine_surrogate_pair(cu, source.get(i));
                    escaped_builder.Append::<CharT, CharT>(cu);
                    escaped_builder.Append::<CharT, CharT>(source.get(i));
                }
            }
        } else if cmd == K_ESCAPE_TO_HEX {
            escaped_builder.AppendCStringLiteral(if cp <= 0xFF { "\\x" } else { "\\u" });
            let hex = DoubleToRadixStringView(cp.into(), 16, double_to_radix_buffer);
            escaped_builder.AppendString(&String {});
        } else {
            escaped_builder.AppendCharacter('\\');
            escaped_builder.AppendCharacter(cmd as char);
        }
    }

    match escaped_builder.Finish() {
      Ok(r) => Ok(r),
      Err(e) => Err(e),
    }
}

fn is_alpha_numeric<CharT>(_c: CharT) -> bool {
    false
}

fn is_lead_surrogate<CharT>(_c: CharT) -> bool {
    false
}

fn is_trail_surrogate<CharT>(_c: CharT) -> bool {
    false
}

fn combine_surrogate_pair<CharT>(_c: CharT, _d: CharT) -> u32 {
    0
}

fn is_surrogate_pair<CharT>(_c: CharT, _d: CharT) -> bool {
    false
}

fn is_white_space_or_line_terminator(_cp: u32) -> bool {
    false
}

fn RegExpEscape(isolate: &Isolate, args: &Arguments) -> Result<String, Box<dyn Error>> {
    let scope = HandleScope {};
    let value = args.atOrUndefined(isolate, 1);

    isolate.CountUsage(v8::Isolate::Usage::kRegExpEscape);

    if !IsString(&value) {
      match THROW_NEW_ERROR_RETURN_FAILURE(isolate, NewTypeError(MessageTemplate::kArgumentIsNonString, isolate.factory().source_string())) {
        Ok(_) => unreachable!(),
        Err(e) => return Err(e),
      }
    }
    let str_handle = Cast::<String>(&value);
    let str = str_handle;

    if str.length() == 0 {
        return Ok(ReadOnlyRoots {}.empty_string());
    }

    let escaped: String;

    let str_flattened = String::Flatten(isolate, str);
    if str_flattened.IsOneByteRepresentation() {
        let copy: base::OwnedVector<u8>;
        {
            let _no_gc = DisallowGarbageCollection::new();
            copy = base::OwnedVector::OwnedCopyOf(StdString::from("dummy").as_bytes().to_vec());
        }
        match RegExpEscapeImpl(isolate, copy) {
          Ok(r) => escaped = r,
          Err(e) => return Err(e),
        }
    } else {
        let copy: base::OwnedVector<base::uc16>;
        {
            let _no_gc = DisallowGarbageCollection::new();
            copy = base::OwnedVector::OwnedCopyOf(StdString::from("dummy").encode_utf16().map(|x| base::uc16{value : x}).collect());
        }
        match RegExpEscapeImpl(isolate, copy) {
          Ok(r) => escaped = r,
          Err(e) => return Err(e),
        }
    }

    Ok(escaped)
}

pub struct Arguments {}
impl Arguments{
  pub fn atOrUndefined(&self, _isolate: &Isolate, _index : i32) -> Object{
    Object{}
  }
}

pub struct HandleScope {}
