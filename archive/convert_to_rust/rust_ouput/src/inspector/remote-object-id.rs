// Converted from V8 C++ source files:
// Header: remote-object-id.h
// Implementation: remote-object-id.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    pub struct Response {
        success: bool,
        error_message: Option<String>,
    }

    impl Response {
        pub fn Success() -> Response {
            Response {
                success: true,
                error_message: None,
            }
        }

        pub fn ServerError(error_message: &str) -> Response {
            Response {
                success: false,
                error_message: Some(error_message.to_string()),
            }
        }

        pub fn ok(&self) -> bool {
            self.success
        }
    }
}

use std::string::FromUtf16Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String16 {
    data: Vec<u16>,
}

impl String16 {
    pub const kNotFound: usize = usize::MAX;

    pub fn from_str(s: &str) -> String16 {
        String16 {
            data: s.encode_utf16().collect(),
        }
    }

    pub fn from_integer(i: i32) -> String16 {
        String16::from_str(&i.to_string())
    }

    pub fn from_integer64(i: i64) -> String16 {
        String16::from_str(&i.to_string())
    }

    pub fn concat(a: String16, b: &str, c: String16) -> String16 {
        let mut result = a.data;
        result.extend(b.encode_utf16());
        result.extend(c.data);
        String16 { data: result }
    }

    pub fn find(&self, pattern: char) -> usize {
        self.data.iter().position(|&c| c == pattern as u16).unwrap_or(String16::kNotFound)
    }

    pub fn find_from(&self, pattern: char, start_pos: usize) -> usize {
         self.data[start_pos..].iter().position(|&c| c == pattern as u16).map(|i| i + start_pos).unwrap_or(String16::kNotFound)
    }

    pub fn substring(&self, start: usize) -> String16 {
        String16 {
            data: self.data[start..].to_vec(),
        }
    }

     pub fn substring(&self, start: usize, end: usize) -> String16 {
        String16 {
            data: self.data[start..end].to_vec(),
        }
    }

    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(&self.data)
    }

    pub fn to_integer(&self) -> Result<i32, std::num::ParseIntError> {
        self.to_string().unwrap().parse::<i32>()
    }

    pub fn to_integer64(&self, ok: &mut bool) -> i64 {
        match self.to_string().unwrap().parse::<i64>() {
            Ok(value) => {
                *ok = true;
                value
            }
            Err(_) => {
                *ok = false;
                0
            }
        }
    }
}

mod remote_object_id {
    use super::protocol::Response;
    use super::String16;

    fn serialize_id(isolate_id: u64, injected_script_id: i32, id: i32) -> String16 {
        String16::concat(
            String16::from_integer64(isolate_id as i64),
            ".",
            String16::from_integer(injected_script_id),
            ".",
            String16::from_integer(id),
        )
    }

    pub struct RemoteObjectIdBase {
        m_isolate_id: u64,
        m_injected_script_id: i32,
        m_id: i32,
    }

    impl RemoteObjectIdBase {
        pub fn isolate_id(&self) -> u64 {
            self.m_isolate_id
        }
        pub fn context_id(&self) -> i32 {
            self.m_injected_script_id
        }

        pub fn new() -> Self {
            RemoteObjectIdBase {
                m_isolate_id: 0,
                m_injected_script_id: 0,
                m_id: 0,
            }
        }

        fn parse_id(&mut self, object_id: &String16) -> bool {
            let dot = '.';
            let first_dot_pos = object_id.find(dot);
            if first_dot_pos == String16::kNotFound {
                return false;
            }
            let mut ok = false;
            let isolate_id = object_id.substring(0, first_dot_pos).to_integer64(&mut ok);
            if !ok {
                return false;
            }

            let first_dot_pos = first_dot_pos + 1;
            let second_dot_pos = object_id.find_from(dot, first_dot_pos);
            if second_dot_pos == String16::kNotFound {
                return false;
            }
            let injected_script_id = object_id.substring(first_dot_pos, second_dot_pos).to_integer().unwrap();
            if injected_script_id < 0 {
                return false;
            }

            let second_dot_pos = second_dot_pos + 1;
            let id = object_id.substring(second_dot_pos).to_integer().unwrap();
            if id < 0 {
                return false;
            }

            self.m_isolate_id = isolate_id as u64;
            self.m_injected_script_id = injected_script_id;
            self.m_id = id;
            true
        }
    }

    pub struct RemoteObjectId {
        base: RemoteObjectIdBase,
        m_id: i32,
    }

    impl RemoteObjectId {
        pub fn parse(object_id: &String16) -> Result<Box<RemoteObjectId>, Response> {
            let mut remote_object_id = RemoteObjectId {
                base: RemoteObjectIdBase::new(),
                m_id: 0,
            };
            if !remote_object_id.base.parse_id(object_id) {
                return Err(Response::ServerError("Invalid remote object id"));
            }
            remote_object_id.m_id = remote_object_id.base.m_id;
            Ok(Box::new(remote_object_id))
        }

        pub fn id(&self) -> i32 {
            self.m_id
        }

        pub fn serialize(isolate_id: u64, injected_script_id: i32, id: i32) -> String16 {
            serialize_id(isolate_id, injected_script_id, id)
        }
    }

    pub struct RemoteCallFrameId {
        base: RemoteObjectIdBase,
        m_id: i32,
    }

    impl RemoteCallFrameId {
        pub fn parse(object_id: &String16) -> Result<Box<RemoteCallFrameId>, Response> {
            let mut remote_call_frame_id = RemoteCallFrameId {
                base: RemoteObjectIdBase::new(),
                m_id: 0,
            };
            if !remote_call_frame_id.base.parse_id(object_id) {
                return Err(Response::ServerError("Invalid call frame id"));
            }
             remote_call_frame_id.m_id = remote_call_frame_id.base.m_id;
            Ok(Box::new(remote_call_frame_id))
        }

        pub fn frame_ordinal(&self) -> i32 {
            self.m_id
        }

        pub fn serialize(isolate_id: u64, injected_script_id: i32, frame_ordinal: i32) -> String16 {
            serialize_id(isolate_id, injected_script_id, frame_ordinal)
        }
    }
}

pub use remote_object_id::{RemoteObjectId, RemoteCallFrameId};
