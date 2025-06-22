// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/regexp/experimental/experimental.rs

//use std::optional::Optional;
//use std::vec::Vec;

//use crate::common::assert_scope::AssertScope;
//use crate::objects::js_regexp::JSRegExp;
//use crate::regexp::experimental::experimental_compiler::ExperimentalRegExpCompiler;
//use crate::regexp::experimental::experimental_interpreter::ExperimentalRegExpInterpreter;
//use crate::regexp::regexp_parser::RegExpParser;
//use crate::regexp::regexp_result_vector::RegExpResultVector;
//use crate::utils::ostreams::StdoutStream;

// Placeholder types and constants.  Replace with actual implementations.
mod regexp {
    pub mod regexp_result_vector {
        pub struct RegExpResultVector {}
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CallOrigin {
        kFromJs,
        kFromRuntime,
    }
    pub const kInternalRegExpRetry: i32 = -1;
    pub const kInternalRegExpException: i32 = -2;
}

mod objects {
    pub mod js_regexp {
        use crate::regexp::CallOrigin;
        
        pub struct JSRegExp {}

        impl JSRegExp {
            pub fn registers_for_capture_count(capture_count: i32) -> i32 {
                capture_count * 2 + 2 // Placeholder implementation
            }

             pub fn as_js_regexp_flags(flags: crate::regexp::RegExpFlags) -> i32 {
                // Placeholder for conversion
                flags as i32
            }
        }
    }
}

mod common {
    pub mod assert_scope {
        pub struct AssertScope {}
    }
}

mod utils {
    pub mod ostreams {
        use std::io::Write;

        pub struct StdoutStream {}

        impl StdoutStream {
            pub fn new() -> Self {
                StdoutStream {}
            }
        }

        impl Write for StdoutStream {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                print!("{}", String::from_utf8_lossy(buf));
                Ok(buf.len())
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
    }
}

mod regexp {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RegExpError {
        kStackOverflow,
        // Add other errors as needed
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RegExpFlags {
        pub global: bool,
        pub ignore_case: bool,
        pub multiline: bool,
        pub dot_all: bool,
        pub unicode: bool,
        pub sticky: bool,
        pub has_indices: bool,
    }

    impl RegExpFlags {
        pub fn new() -> Self {
            RegExpFlags {
                global: false,
                ignore_case: false,
                multiline: false,
                dot_all: false,
                unicode: false,
                sticky: false,
                has_indices: false,
            }
        }
    }

    pub struct RegExpCompileData {
        pub error: RegExpError,
        pub tree: *mut RegExpTree, // Raw pointer, needs proper handling
        pub named_captures: Vec<String>, // Placeholder.  Needs proper type.
    }

    impl RegExpCompileData {
        pub fn new() -> Self {
            RegExpCompileData {
                error: RegExpError::kStackOverflow, // Default initialization
                tree: std::ptr::null_mut(),       // Initialize with null
                named_captures: Vec::new(),
            }
    }
}
    pub fn throw_regexp_exception(
        isolate: &mut Isolate,
        flags: RegExpFlags,
        source: &String,
        error: RegExpError,
    ) -> Result<(), String> {
        // Placeholder implementation
        Err(format!("RegExp exception: {:?} for pattern '{}'", error, source))
    }
}

mod regexp_parser {
    //use super::*;
    use crate::regexp::*;
    use crate::isolate::*;
    //use crate::zone::Zone;

    pub fn parse_regexp_from_heap_string(
        isolate: &mut Isolate,
        zone: &mut Zone,
        source: &String,
        flags: RegExpFlags,
        result: &mut RegExpCompileData,
    ) -> bool {
        // Placeholder implementation
        // Needs actual regexp parsing logic
        if source.is_empty() {
             result.error = RegExpError::kStackOverflow;
            false
        } else {
             result.error = RegExpError::kStackOverflow;
            true // Simulate success
        }
    }
}

mod zone {
    // Placeholder for Zone allocator
    pub struct Zone {
       // allocator: Allocator //Needs allocation
    }

    impl Zone {
        pub fn new(isolate: &Isolate, zone_name: &str) -> Self {
            Zone {
                //allocator : Allocator::new(),
            }
        }
    }
}

mod isolate {
    //use crate::objects::js_regexp::JSRegExp;
    use crate::factory::Factory;
    use crate::regexp::RegExpFlags;
    //use crate::heap::Heap;

    pub struct Isolate {
        pub exception: Option<String>, // Placeholder for exceptions
        pub factory: Factory,
        //heap: Heap,
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                exception: None,
                factory: Factory::new(),
                //heap: Heap::new(),
            }
        }

        pub fn has_exception(&self) -> bool {
            self.exception.is_some()
        }
         
        pub fn set_regexp_experimental_data(
            &mut self,
            re: &mut JSRegExp,
            source: &String,
            flags: i32,
            capture_count: i32,
        ) {
            // Placeholder
            println!("Setting experimental regexp data for {:?}", re);
        }
    }

}

mod factory {
    //use crate::objects::js_regexp::JSRegExp;

    pub struct Factory {}
    impl Factory {
        pub fn new() -> Self {
            Factory {}
        }
        pub fn set_regexp_experimental_data(
            &mut self,
            re: &mut JSRegExp,
            source: &String,
            flags: i32,
            capture_count: i32,
        ) {
            // Placeholder implementation
            println!(
                "Factory: Setting experimental regexp data for {:?} with source: {} and flags: {} and capture_count: {}",
                re, source, flags, capture_count
            );
        }

        pub fn new_trusted_byte_array(&self, byte_length: i32) -> TrustedByteArray {
            // Placeholder. Needs actual byte array allocation.
            TrustedByteArray {
                data: vec![0; byte_length as usize],
            }
        }

         pub fn create_capture_name_map(&self, named_captures: Vec<String>) -> FixedArray {
             FixedArray {
                data: vec![],
            }
        }
    }
}

mod irregexpdata {
    use crate::isolate::Isolate;
    //use crate::trustedbytearray::TrustedByteArray;
    //use crate::fixedarray::FixedArray;
    //use crate::string::String;

    #[derive(Debug, PartialEq)]
    pub enum Type {
        EXPERIMENTAL,
    }

    pub struct IrRegExpData {
        type_tag: Type,
        source: String,
        flags: i32,
        capture_count: i32,
        bytecode: Option<TrustedByteArray>,
        capture_name_map: Option<FixedArray>,
    }

    impl IrRegExpData {
        pub fn new(type_tag: Type, source: String, flags: i32, capture_count: i32) -> Self {
            IrRegExpData {
                type_tag,
                source,
                flags,
                capture_count,
                bytecode: None,
                capture_name_map: None,
            }
        }
        pub fn type_tag(&self) -> &Type {
            &self.type_tag
        }

        pub fn source(&self) -> &String {
            &self.source
        }

        pub fn flags(&self) -> i32 {
            self.flags
        }

        pub fn capture_count(&self) -> i32 {
            self.capture_count
        }
        
        pub fn has_bytecode(&self, _is_latin1: bool) -> bool {
            self.bytecode.is_some()
        }

        pub fn bytecode(&self, _is_latin1: bool) -> &TrustedByteArray {
            self.bytecode.as_ref().unwrap() //Handle option
        }

        pub fn set_bytecode_for_experimental(&mut self, isolate: &mut Isolate, bytecode: TrustedByteArray) {
            self.bytecode = Some(bytecode);
        }

         pub fn set_capture_name_map(&mut self, capture_name_map: FixedArray) {
            self.capture_name_map = Some(capture_name_map);
        }

        #[cfg(debug_assertions)]
        pub fn ir_regexp_data_verify(&self, isolate: &mut Isolate) {
            //Verification Logic
        }
    }
}

mod trustedbytearray {
    #[derive(Clone)]
    pub struct TrustedByteArray {
        pub data: Vec<u8>,
    }

    impl TrustedByteArray {
        pub fn begin(&self) -> *const u8 {
            self.data.as_ptr()
        }

        pub fn length(&self) -> i32 {
            self.data.len() as i32
        }
    }
}

mod fixedarray {
    //use std::vec::Vec;

    pub struct FixedArray {
        pub data: Vec<i32>, // Placeholder
    }
}

mod string {
    // Placeholder String type.  Replace with actual String type if needed.
    #[derive(Debug)]
    pub struct String {
        pub content: std::string::String,
    }

    impl String {
        pub fn new(content: std::string::String) -> Self {
            String { content }
        }

        pub fn is_flat(&self) -> bool {
            true // Placeholder
        }
        
        pub fn flatten(isolate: &mut Isolate, string: &String) -> String {
            // Placeholder for flattening logic
            String::new(string.content.clone())
        }
    }
}

mod regexptree {
    // Placeholder for RegExpTree. Replace with actual implementation.
    pub struct RegExpTree {}
}

mod regexpinstruction {
    #[derive(Clone, Copy, Debug)]
    pub struct RegExpInstruction(pub u8);
}

mod experimentaregexpcompiler {
    use crate::regexp::RegExpFlags;
    use crate::regexptree::RegExpTree;
    use crate::regexpinstruction::RegExpInstruction;
    use crate::zone::Zone;

    pub fn can_be_handled(tree: *mut RegExpTree, flags: RegExpFlags, capture_count: i32) -> bool {
        // Placeholder
        true
    }

    pub fn compile(tree: *mut RegExpTree, flags: RegExpFlags, zone: &mut Zone) -> Vec<RegExpInstruction> {
        // Placeholder implementation. Returns a dummy bytecode vector.
        vec![RegExpInstruction(0), RegExpInstruction(1), RegExpInstruction(2)]
    }
}

mod experimentaregexpinterpreter {
    use crate::isolate::Isolate;
    use crate::string::String;
    use crate::trustedbytearray::TrustedByteArray;
    use crate::zone::Zone;
    use crate::regexpinstruction::RegExpInstruction;
    use crate::regexp::CallOrigin;

    pub fn find_matches(
        isolate: &mut Isolate,
        call_origin: CallOrigin,
        bytecode: &TrustedByteArray,
        register_count_per_match: i32,
        subject: &String,
        subject_index: i32,
        output_registers: *mut i32,
        output_register_count: i32,
        zone: &mut Zone,
    ) -> i32 {
        // Placeholder implementation
        println!("Executing bytecode: {:?}", bytecode.data);
        println!("Subject: {}", subject.content);
        println!("Subject index: {}", subject_index);

        // Simulate a match at index 0
        if subject.content.len() > subject_index as usize {
            unsafe {
                *output_registers = subject_index; // Match start
                *output_registers.offset(1) = subject.content.len() as i32; // Match end
            }
            1 // Number of matches
        } else {
            0 // No match
        }
    }
}

//use v8_flags; // Assuming this is a module for V8 flags

// Dummy V8 flags module
mod v8_flags {
    pub const enable_experimental_regexp_engine: bool = true;
    pub const enable_experimental_regexp_engine_on_excessive_backtracks: bool = true;
    pub const trace_experimental_regexp_engine: bool = true;
    pub const verify_heap: bool = true;
}

pub struct ExperimentalRegExp {}

impl ExperimentalRegExp {
    pub fn can_be_handled(
        tree: *mut RegExpTree,
        pattern: &String,
        flags: crate::regexp::RegExpFlags,
        capture_count: i32,
    ) -> bool {
        assert!(
            v8_flags::enable_experimental_regexp_engine
                || v8_flags::enable_experimental_regexp_engine_on_excessive_backtracks
        );
        let can_be_handled =
            experimentaregexpcompiler::can_be_handled(tree, flags, capture_count);
        if !can_be_handled && v8_flags::trace_experimental_regexp_engine {
            let mut stdout = utils::ostreams::StdoutStream::new();
            use std::io::Write;
            let _ = write!(
                stdout,
                "Pattern not supported by experimental engine: {}\n",
                pattern.content
            );
        }
        can_be_handled
    }

    pub fn initialize(
        isolate: &mut Isolate,
        re: &mut objects::js_regexp::JSRegExp,
        source: &String,
        flags: crate::regexp::RegExpFlags,
        capture_count: i32,
    ) {
        assert!(v8_flags::enable_experimental_regexp_engine);
        if v8_flags::trace_experimental_regexp_engine {
            let mut stdout = utils::ostreams::StdoutStream::new();
            use std::io::Write;
            let _ = write!(
                stdout,
                "Initializing experimental regexp {}\n",
                source.content
            );
        }

        isolate.set_regexp_experimental_data(
            re,
            source,
            objects::js_regexp::JSRegExp::as_js_regexp_flags(flags),
            capture_count,
        );
    }

    pub fn is_compiled(re_data: &irregexpdata::IrRegExpData, isolate: &mut Isolate) -> bool {
        assert!(v8_flags::enable_experimental_regexp_engine);
        assert_eq!(re_data.type_tag(), &irregexpdata::Type::EXPERIMENTAL);

        #[cfg(debug_assertions)]
        if v8_flags::verify_heap {
           // re_data.ir_regexp_data_verify(isolate);
        }

        const K_IS_LATIN1: bool = true;
        re_data.has_bytecode(K_IS_LATIN1)
    }

    fn vector_to_byte_array<T: Copy>(isolate: &mut Isolate, data: Vec<T>) -> TrustedByteArray {
        //static_assert(std::is_trivial_v<T>);

        let byte_length = std::mem::size_of::<T>() * data.len();
        let byte_array = isolate.factory.new_trusted_byte_array(byte_length as i32);
        //DisallowGarbageCollection no_gc;
        //MemCopy(byte_array->begin(), data.begin(), byte_length);
        
        let byte_array_len = byte_array.length() as usize;
        
        let mut byte_data = byte_array.data.clone();
        
        let data_ptr = data.as_ptr() as *const u8;
        let slice = unsafe { std::slice::from_raw_parts(data_ptr, byte_length) };

        byte_data.copy_from_slice(slice);

        TrustedByteArray {
            data: byte_data
        }
    }

    fn compile_impl(
        isolate: &mut Isolate,
        re_data: &mut irregexpdata::IrRegExpData,
    ) -> Result<CompilationResult, String> {
        let mut zone = Zone::new(isolate, "ZONE_NAME");

        let source = re_data.source().clone();

        // Parse and compile the regexp source.
        let mut parse_result = crate::regexp::RegExpCompileData::new();
        assert!(!isolate.has_exception());

        let flags = crate::regexp::RegExpFlags {
            global: false,
            ignore_case: false,
            multiline: false,
            dot_all: false,
            unicode: false,
            sticky: false,
            has_indices: false,
        }; //objects::js_regexp::JSRegExp::as_regexp_flags(re_data.flags());

        let parse_success = regexp_parser::parse_regexp_from_heap_string(
            isolate,
            &mut zone,
            &source,
            flags,
            &mut parse_result,
        );
        if !parse_success {
            // The pattern was already parsed successfully during initialization, so
            // the only way parsing can fail now is because of stack overflow.
           assert_eq!(parse_result.error, crate::regexp::RegExpError::kStackOverflow);
            let _ = crate::regexp::throw_regexp_exception(isolate, flags, &source, parse_result.error)?;
            return Err("Parsing failed".to_string());
        }

        let bytecode = experimentaregexpcompiler::compile(parse_result.tree, flags, &mut zone);

        let result = CompilationResult {
            bytecode: ExperimentalRegExp::vector_to_byte_array(isolate, bytecode),
            capture_name_map: isolate.factory.create_capture_name_map(parse_result.named_captures),
        };
        Ok(result)
    }

    pub fn compile(isolate: &mut Isolate, re_data: &mut irregexpdata::IrRegExpData) -> bool {
        assert!(v8_flags::enable_experimental_regexp_engine);
        assert_eq!(re_data.type_tag(), &irregexpdata::Type::EXPERIMENTAL);

        #[cfg(debug_assertions)]
        if v8_flags::verify_heap {
            //re_data.ir_regexp_data_verify(isolate);
        }

        let source = re_data.source().clone();
        if v8_flags::trace_experimental_regexp_engine {
            let mut stdout = utils::ostreams::StdoutStream::new();
            use std::io::Write;
            let _ = write!(stdout, "Compiling experimental regexp {}\n", source.content);
        }

        let compilation_result = ExperimentalRegExp::compile_impl(isolate, re_data);

        match compilation_result {
            Ok(compilation_result) => {
                re_data.set_bytecode_for_experimental(isolate, compilation_result.bytecode);
                re_data.set_capture_name_map(compilation_result.capture_name_map);

                true
            }
            Err(_e) => {
                assert!(isolate.has_exception());
                false
            }
        }
    }

    fn as_instruction_sequence(raw_bytes: &TrustedByteArray) -> Vec<RegExpInstruction> {
        //RegExpInstruction* inst_begin =
        //  reinterpret_cast<RegExpInstruction*>(raw_bytes->begin());
        //int inst_num = raw_bytes->length() / sizeof(RegExpInstruction);
        //DCHECK_EQ(sizeof(RegExpInstruction) * inst_num, raw_bytes->length());
        //return base::Vector<RegExpInstruction>(inst_begin, inst_num);

        let inst_num = raw_bytes.length() as usize / std::mem::size_of::<RegExpInstruction>();
        assert_eq!(
            std::mem::size_of::<RegExpInstruction>() * inst_num,
            raw_bytes.length() as usize
        );

        raw_bytes.data
            .chunks(std::mem::size_of::<RegExpInstruction>())
            .map(|chunk| {
                let instruction = RegExpInstruction(chunk[0]);
                instruction
            })
            .collect()
    }

    fn exec_raw_impl(
        isolate: &mut Isolate,
        call_origin: regexp::CallOrigin,
        bytecode: &TrustedByteArray,
        subject: &String,
        capture_count: i32,
        output_registers: *mut i32,
        output_register_count: i32,
        subject_index: i32,
    ) -> i32 {
        //DisallowGarbageCollection no_gc;
        // TODO(cbruni): remove once gcmole is fixed.
        //DisableGCMole no_gc_mole;

        let register_count_per_match =
            objects::js_regexp::JSRegExp::registers_for_capture_count(capture_count);

        //int32_t result;
        assert!(subject.is_flat());
        let mut zone = Zone::new(isolate, "ZONE_NAME");
        experimentaregexpinterpreter::find_matches(
            isolate,
            call_origin,
            bytecode,
            register_count_per_match,
            subject,
            subject_index,
            output_registers,
            output_register_count,
            &mut zone,
        )
    }

    // Returns the number of matches.
    pub fn exec_raw(
        isolate: &mut Isolate,
        call_origin: regexp::CallOrigin,
        regexp_data: &irregexpdata::IrRegExpData,
        subject: &String,
        output_registers: *mut i32,
        output_register_count: i32,
        subject_index: i32,
    ) -> i32 {
        assert!(v8_flags::enable_experimental_regexp_engine);
        //DisallowGarbageCollection no_gc;

        if v8_flags::trace_experimental_regexp_engine {
            let mut stdout = utils::ostreams::StdoutStream::new();
            use std::io::Write;
            let _ = write!(
                stdout,
                "Executing experimental regexp {}\n",
                regexp_data.source().content
            );
        }

        const K_IS_LATIN1: bool = true;
        let bytecode = regexp_data.bytecode(K_IS_LATIN1);

        ExperimentalRegExp::exec_raw_impl(
            isolate,
            call_origin,
            bytecode,
            subject,
            regexp_data.capture_count(),
            output_registers,
            output_register_count,
            subject_index,
        )
    }

    pub fn match_for_call_from_js(
        subject: usize,
        start_position: i32,
        input_start: usize,
        input_end: usize,
        output_registers: *mut i32,
        output_register_count: i32,
        call_origin: regexp::CallOrigin,
        isolate: &mut Isolate,
        regexp_data: usize,
    ) -> i32 {
        assert!(v8_flags::enable_experimental_regexp_engine);
        assert!(!isolate.factory.new_trusted_byte_array(10).data.is_empty());
        assert!(call_origin == regexp::CallOrigin::kFromJs);

        //DisallowGarbageCollection no_gc;
        //DisallowJavascriptExecution no_js(isolate);
        //DisallowHandleAllocation no_handles;
        //DisallowHandleDereference no_deref;

        let subject_string = unsafe { &*(subject as *const string::String) };

        let regexp_data_obj = unsafe { &*(regexp_data as *const irregexpdata::IrRegExpData) };

        ExperimentalRegExp::exec_raw(
            isolate,
            regexp::CallOrigin::kFromJs,
            regexp_data_obj,
            subject_string,
            output_registers,
            output_register_count,
            start_position,
        )
    }

    // static
    pub fn exec(
        isolate: &mut Isolate,
        regexp_data: &mut irregexpdata::IrRegExpData,
        subject: &mut String,
        index: i32,
        result_offsets_vector: *mut i32,
        result_offsets_vector_length: u32,
    ) -> Result<Option<i32>, String> {
        assert!(v8_flags::enable_experimental_regexp_engine);
        assert_eq!(regexp_data.type_tag(), &irregexpdata::Type::EXPERIMENTAL);

        #[cfg(debug_assertions)]
        if v8_flags::verify_heap {
            //regexp_data.ir_regexp_data_verify(isolate);
        }

        if !ExperimentalRegExp::is_compiled(regexp_data, isolate) {
            if !ExperimentalRegExp::compile(isolate, regexp_data) {
                assert!(isolate.has_exception());
                return Err("Compilation failed".to_string());
            }
        }

        assert!(ExperimentalRegExp::is_compiled(regexp_data, isolate));

        *subject = string::String::flatten(isolate, subject);

        assert!(
            result_offsets_vector_length
                >= objects::js_regexp::JSRegExp::registers_for_capture_count(
                    regexp_data.capture_count()
                ) as u32
        );

        loop {
            let num_matches = ExperimentalRegExp::exec_raw(
                isolate,
                regexp::CallOrigin::kFromRuntime,
                regexp_data,
                subject,
                result_offsets_vector,
                result_offsets_vector_length as i32,
                index,
            );

            if num_matches > 0 {
                assert!(
                    (num_matches * objects::js_regexp::JSRegExp::registers_for_capture_count(
                        regexp_data.capture_count()
                    )) <= result_offsets_vector_length as i32
                );
                return Ok(Some(num_matches));
            } else if num_matches == 0 {
                return Ok(Some(num_matches));
            } else {
                assert!(num_matches < 0);
                if num_matches == regexp::kInternalRegExpRetry {
                    // Re-run execution.
                    continue;
                }
                assert!(isolate.has_exception());
                return Err("Execution failed".to_string());
            }
        }
    }

    pub fn oneshot_exec_raw(
        isolate: &mut Isolate,
        regexp_data: &mut irregexpdata::IrRegExpData,
        subject: &String,
        output_registers: *mut i32,
        output_register_count: i32,
        subject_index: i32,
    ) -> i32 {
        assert!(v8_flags::enable_experimental_regexp_engine_on_excessive_backtracks);

        if v8_flags::trace_experimental_regexp_engine {
            let mut stdout = utils::ostreams::StdoutStream::new();
            use std::io::Write;
            let _ = write!(
                stdout,
                "Experimental execution (oneshot) of regexp {}\n",
                regexp_data.source().content
            );
        }

        let compilation_result = ExperimentalRegExp::compile_impl(isolate, regexp_data);
        match compilation_result {
            Ok(compilation_result) => {
                //DisallowGarbageCollection no_gc;
                ExperimentalRegExp::exec_raw_impl(
                    isolate,
                    regexp::CallOrigin::kFromRuntime,
                    &compilation_result.bytecode,
                    subject,
                    regexp_data.capture_count(),
                    output_registers,
                    output_register_count,
                    subject_index,
                )
            }
            Err(_e) => regexp::kInternalRegExpException,
        }
    }

    pub fn oneshot_exec(
        isolate: &mut Isolate,
        regexp_data: &mut irregexpdata::IrRegExpData,
        subject: &String,
        subject_index: i32,
        result_offsets_vector: *mut i32,
        result_offsets_vector_length: u32,
    ) -> Result<Option<i32>, String> {
        assert!(v8_flags::enable_experimental_regexp_engine_on_excessive_backtracks);

        loop {
            let num_matches = ExperimentalRegExp::oneshot_exec_raw(
                isolate,
                regexp_data,
                subject,
                result_offsets_vector,
                result_offsets_vector_length as i32,
                subject_index,
            );

            if num_matches > 0 {
                assert!(
                    (num_matches * objects::js_regexp::JSRegExp::registers_for_capture_count(
                        regexp_data.capture_count()
                    )) <= result_offsets_vector_length as i32
                );
                return Ok(Some(num_matches));
            } else if num_matches == 0 {
                return Ok(Some(num_matches));
            } else {
                assert!(num_matches < 0);
                if num_matches == regexp::kInternalRegExpRetry {
                    // Re-run execution.
                    continue;
                }
                assert!(isolate.has_exception());
                return Err("Oneshot execution failed".to_string());
            }
        }
    }
}

struct CompilationResult {
    bytecode: TrustedByteArray,
    capture_name_map: FixedArray,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isolate::Isolate;
    use crate::string::String;
    use crate::irregexpdata::{IrRegExpData, Type};

    #[test]
    fn test_experimental_regexp() {
        let mut isolate = Isolate::new();
        let source = String::new("test".to_string());
        let flags = 0;
        let capture_count = 0;
        let mut regexp_data = IrRegExpData::new(Type::EXPERIMENTAL, source, flags, capture_count);
        let subject = String::new("test".to_string());
        let mut result_offsets_vector: Vec<i32> = vec![0; 10];
        let result_offsets_vector_length = result_offsets_vector.len() as u32;

        let result = ExperimentalRegExp::exec(
            &mut isolate,
            &mut regexp_data,
            &mut String::new("test".to_string()),
            0,
            result_offsets_vector.as_mut_ptr(),
            result_offsets_vector_length,
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1));
    }

      #[test]
    fn test_match_for_call_from_js() {
        let mut isolate = Isolate::new();
        let source = String::new("test".to_string());
        let flags = 0;
        let capture_count = 0;
        let regexp_data = IrRegExpData::new(Type::EXPERIMENTAL, source, flags, capture_count);
        let subject = String::new("test".to_string());
        let mut result_offsets_vector: Vec<i32> = vec![0; 10];
        //let result_offsets_vector_length = result_offsets_vector.len() as u32;
        let subject_ptr = &subject as *const String as usize;
        let regexp_data_ptr = &regexp_data as *const IrRegExpData as usize;

         let result = ExperimentalRegExp::match_for_call_from_js(
                subject_ptr,
                0,
                0,
                0,
                result_offsets_vector.as_mut_ptr(),
                10,
                regexp::CallOrigin::kFromJs,
                &mut isolate,
                regexp_data_ptr,
            );

         assert_eq!(result, 1);
    }

}