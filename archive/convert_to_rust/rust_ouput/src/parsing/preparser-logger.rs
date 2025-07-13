// Converted from V8 C++ source files:
// Header: preparser-logger.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#[allow(dead_code)]
pub struct PreParserLogger {
    end: i32,
    num_parameters: i32,
    function_length: i32,
    num_inner_infos: i32,
}

impl PreParserLogger {
    pub fn new() -> Self {
        PreParserLogger {
            end: -1,
            num_parameters: -1,
            function_length: -1,
            num_inner_infos: -1,
        }
    }

    pub fn log_function(&mut self, end: i32, num_parameters: i32, function_length: i32, num_inner_infos: i32) {
        self.end = end;
        self.num_parameters = num_parameters;
        self.function_length = function_length;
        self.num_inner_infos = num_inner_infos;
    }

    pub fn end(&self) -> i32 {
        self.end
    }

    pub fn num_parameters(&self) -> i32 {
        self.num_parameters
    }

    pub fn function_length(&self) -> i32 {
        self.function_length
    }

    pub fn num_inner_infos(&self) -> i32 {
        self.num_inner_infos
    }
}
