// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::max;
use std::convert::TryInto;
use std::f64;
use std::mem;
use std::num::Wrapping;

use lazy_static::lazy_static;

//use crate::base::overflowing_math; // Assuming a corresponding base crate
//use crate::flags::flags; // Assuming a corresponding flags crate
//use crate::numbers::conversions_inl; // Assuming a corresponding numbers crate
//use crate::parsing::scanner; // Assuming a corresponding parsing crate
//use crate::wasm::wasm_limits; // Assuming a corresponding wasm crate
//use crate::wasm::wasm_opcodes; // Assuming a corresponding wasm crate

// Placeholder imports; replace with actual implementations.
mod asm_js;
mod asm_types;
mod wasm_module_builder;
mod asm_scanner;

use asm_js::*;
use asm_types::*;
use wasm_module_builder::*;
use asm_scanner::*;

lazy_static! {
    static ref V8_FLAGS_TRACE_ASM_PARSER: bool = false; // replace with actual flag retrieval if available
}

macro_rules! trace_asm_parser {
    ($($arg:tt)*) => {
        if *V8_FLAGS_TRACE_ASM_PARSER {
            println!($($arg)*);
        }
    };
}

// Error type for parsing failures.
#[derive(Debug, Clone)]
pub struct AsmJsParserError {
    message: String,
    location: usize,
}

type AsmJsParserResult<T> = Result<T, AsmJsParserError>;

macro_rules! fail_and_return {
    ($ret:expr, $msg:expr, $scanner:expr, $file:expr, $line:expr) => {
        {
            let msg = $msg.to_string();
            trace_asm_parser!("[asm.js failure: {}, token: '{}', see: {}:{} ]",
                msg,
                $scanner.name($scanner.token()),
                $file,
                $line);
            return Err(AsmJsParserError {
                message: msg,
                location: $scanner.position() as usize,
            });
        }
    };
}

macro_rules! fail {
    ($msg:expr, $scanner:expr) => {
        fail_and_return!((), $msg, $scanner, file!(), line!())
    };
}

macro_rules! failn {
    ($msg:expr, $scanner:expr) => {
        fail_and_return!(None, $msg, $scanner, file!(), line!())
    };
}

macro_rules! expect_token_or_return {
    ($ret:expr, $token:expr, $scanner:expr) => {
        if $scanner.token() != $token {
            fail_and_return!($ret, "Unexpected token", $scanner, file!(), line!());
        }
        $scanner.next();
    };
}

macro_rules! expect_token {
    ($token:expr, $scanner:expr) => {
        expect_token_or_return!((), $token, $scanner)
    };
}

macro_rules! expect_tokenn {
    ($token:expr, $scanner:expr) => {
        expect_token_or_return!(None, $token, $scanner)
    };
}

macro_rules! recurse_or_return {
    ($ret:expr, $call:expr, $parser:expr, $stack_limit:expr) => {
        {
            if $parser.failed_.is_some() {
                return $ret;
            }

            if $parser.get_current_stack_position() < *$stack_limit {
                fail_and_return!($ret, "Stack overflow while parsing asm.js module.",
                    $parser.scanner_, file!(), line!());
            }
            $call;
            if $parser.failed_.is_some() {
                return $ret;
            }
        }
    };
}

macro_rules! recurse {
    ($call:expr, $parser:expr, $stack_limit:expr) => {
        recurse_or_return!((), $call, $parser, $stack_limit)
    };
}

macro_rules! recursen {
    ($call:expr, $parser:expr, $stack_limit:expr) => {
        recurse_or_return!(None, $call, $parser, $stack_limit)
    };
}

// Helper macro for creating tokens
macro_rules! tok {
    ($name:ident) => {
        AsmJsScannerToken::KToken_##$name
    };
}

pub struct AsmJsParser<'a> {
    zone_: &'a Zone,
    scanner_: AsmJsScanner<'a>,
    module_builder_: WasmModuleBuilder<'a>,
    stack_limit_: usize,
    block_stack_: Vec<BlockInfo>,
    global_imports_: Vec<GlobalImport>,
    stdlib_name_: AsmJsScannerToken,
    foreign_name_: AsmJsScannerToken,
    heap_name_: AsmJsScannerToken,
    failed_: Option<AsmJsParserError>,
    failure_message_: String,
    failure_location_: usize,
    num_globals_: usize,
    global_var_info_: Vec<VarInfo>,
    local_var_info_: Vec<VarInfo>,
    stdlib_dq2d_: Option<&'a AsmType>,
    stdlib_dqdq2d_: Option<&'a AsmType>,
    stdlib_i2s_: Option<&'a AsmType>,
    stdlib_ii2s_: Option<&'a AsmType>,
    stdlib_minmax_: Option<&'a AsmType>,
    stdlib_abs_: Option<&'a AsmType>,
    stdlib_ceil_like_: Option<&'a AsmType>,
    stdlib_fround_: Option<&'a AsmType>,
    return_type_: Option<&'a AsmType>,
    current_function_builder_: Option<&'a WasmFunctionBuilder<'a>>,
    call_coercion_: Option<&'a AsmType>,
    call_coercion_position_: usize,
    call_coercion_deferred_: Option<&'a AsmType>,
    call_coercion_deferred_position_: usize,
    heap_access_type_: Option<&'a AsmType>,
    heap_access_shift_position_: usize,
    heap_access_shift_value_: u32,
    inside_heap_assignment_: bool,
    pending_label_: AsmJsScannerToken,
    function_temp_locals_offset_: u32,
    function_temp_locals_used_: i32,
    function_temp_locals_depth_: i32,
    stdlib_uses_: StandardMembers,

    //Cached vectors, might be possible to make this a generic argument
    cached_asm_type_p_vectors_: ZoneCachedVector<&'a AsmType>,
    cached_valuetype_vectors_: ZoneCachedVector<ValueType>,
    cached_token_t_vectors_: ZoneCachedVector<AsmJsScannerToken>,
    cached_int_vectors_: ZoneCachedVector<i32>,
}

impl<'a> AsmJsParser<'a> {
    pub fn new(zone: &'a Zone, stack_limit: usize, stream: &'a str) -> Self {
        let mut parser = AsmJsParser {
            zone_: zone,
            scanner_: AsmJsScanner::new(zone, stream),
            module_builder_: WasmModuleBuilder::new(zone),
            stack_limit_: stack_limit,
            block_stack_: Vec::new(),
            global_imports_: Vec::new(),
            stdlib_name_: AsmJsScannerToken::KTokenNone,
            foreign_name_: AsmJsScannerToken::KTokenNone,
            heap_name_: AsmJsScannerToken::KTokenNone,
            failed_: None,
            failure_message_: String::new(),
            failure_location_: 0,
            num_globals_: 0,
            global_var_info_: Vec::new(),
            local_var_info_: Vec::new(),
            stdlib_dq2d_: None,
            stdlib_dqdq2d_: None,
            stdlib_i2s_: None,
            stdlib_ii2s_: None,
            stdlib_minmax_: None,
            stdlib_abs_: None,
            stdlib_ceil_like_: None,
            stdlib_fround_: None,
            return_type_: None,
            current_function_builder_: None,
            call_coercion_: None,
            call_coercion_position_: 0,
            call_coercion_deferred_: None,
            call_coercion_deferred_position_: 0,
            heap_access_type_: None,
            heap_access_shift_position_: 0,
            heap_access_shift_value_: 0,
            inside_heap_assignment_: false,
            pending_label_: AsmJsScannerToken::KTokenNone,
            function_temp_locals_offset_: 0,
            function_temp_locals_used_: 0,
            function_temp_locals_depth_: 0,
            stdlib_uses_: StandardMembers::new(),
            cached_asm_type_p_vectors_: ZoneCachedVector::new(zone),
            cached_valuetype_vectors_: ZoneCachedVector::new(zone),
            cached_token_t_vectors_: ZoneCachedVector::new(zone),
            cached_int_vectors_: ZoneCachedVector::new(zone),
        };
        parser.module_builder_.add_memory(0);
        parser.initialize_stdlib_types();
        parser
    }

    fn initialize_stdlib_types(&mut self) {
        let d = AsmType::Double();
        let dq = AsmType::DoubleQ();
        self.stdlib_dq2d_ = Some(AsmType::Function(self.zone_, d));
        self.stdlib_dq2d_.unwrap().as_function_type().add_argument(dq);

        self.stdlib_dqdq2d_ = Some(AsmType::Function(self.zone_, d));
        self.stdlib_dqdq2d_.unwrap().as_function_type().add_argument(dq);
        self.stdlib_dqdq2d_.unwrap().as_function_type().add_argument(dq);

        let f = AsmType::Float();
        let fh = AsmType::Floatish();
        let fq = AsmType::FloatQ();
        let fq2fh = AsmType::Function(self.zone_, fh);
        fq2fh.as_function_type().add_argument(fq);

        let s = AsmType::Signed();
        let u = AsmType::Unsigned();
        let s2u = AsmType::Function(self.zone_, u);
        s2u.as_function_type().add_argument(s);

        let i = AsmType::Int();
        self.stdlib_i2s_ = Some(AsmType::Function(self.zone_, s));
        self.stdlib_i2s_.unwrap().as_function_type().add_argument(i);

        self.stdlib_ii2s_ = Some(AsmType::Function(self.zone_, s));
        self.stdlib_ii2s_.unwrap().as_function_type().add_argument(i);
        self.stdlib_ii2s_.unwrap().as_function_type().add_argument(i);

        let minmax_d = AsmType::MinMaxType(self.zone_, d, d);
        let minmax_f = AsmType::MinMaxType(self.zone_, f, f);
        let minmax_s = AsmType::MinMaxType(self.zone_, s, s);
        self.stdlib_minmax_ = Some(AsmType::OverloadedFunction(self.zone_));
        self.stdlib_minmax_.unwrap().as_overloaded_function_type().add_overload(minmax_s);
        self.stdlib_minmax_.unwrap().as_overloaded_function_type().add_overload(minmax_f);
        self.stdlib_minmax_.unwrap().as_overloaded_function_type().add_overload(minmax_d);

        self.stdlib_abs_ = Some(AsmType::OverloadedFunction(self.zone_));
        self.stdlib_abs_.unwrap().as_overloaded_function_type().add_overload(s2u);
        self.stdlib_abs_.unwrap().as_overloaded_function_type().add_overload(self.stdlib_dq2d_.unwrap());
        self.stdlib_abs_.unwrap().as_overloaded_function_type().add_overload(fq2fh);

        self.stdlib_ceil_like_ = Some(AsmType::OverloadedFunction(self.zone_));
        self.stdlib_ceil_like_.unwrap().as_overloaded_function_type().add_overload(self.stdlib_dq2d_.unwrap());
        self.stdlib_ceil_like_.unwrap().as_overloaded_function_type().add_overload(fq2fh);

        self.stdlib_fround_ = Some(AsmType::FroundType(self.zone_));
    }

    fn convert_signature(
        &self,
        return_type: &AsmType,
        params: &ZoneCachedVector<&'a AsmType>,
    ) -> FunctionSig<'a> {
        let mut sig_builder = FunctionSig::builder(
            self.zone(),
            if !return_type.is_a(AsmType::Void()) { 1 } else { 0 },
            params.len(),
        );
        for param in params.iter() {
            if param.is_a(AsmType::Double()) {
                sig_builder.add_param(ValueType::WasmF64);
            } else if param.is_a(AsmType::Float()) {
                sig_builder.add_param(ValueType::WasmF32);
            } else if param.is_a(AsmType::Int()) {
                sig_builder.add_param(ValueType::WasmI32);
            } else {
                unreachable!();
            }
        }
        if !return_type.is_a(AsmType::Void()) {
            if return_type.is_a(AsmType::Double()) {
                sig_builder.add_return(ValueType::WasmF64);
            } else if return_type.is_a(AsmType::Float()) {
                sig_builder.add_return(ValueType::WasmF32);
            } else if return_type.is_a(AsmType::Signed()) {
                sig_builder.add_return(ValueType::WasmI32);
            } else {
                unreachable!();
            }
        }
        sig_builder.get()
    }

    pub fn run(&mut self) -> bool {
        match self.validate_module() {
            Ok(_) => !self.failed_.is_some(),
            Err(_) => !self.failed_.is_some(),
        }
    }

    struct TemporaryVariableScope<'b, 'a> {
        parser_: &'b mut AsmJsParser<'a>,
        local_depth_: i32,
    }

    impl<'b, 'a> TemporaryVariableScope<'b, 'a> {
        fn new(parser: &'b mut AsmJsParser<'a>) -> Self {
            let local_depth_ = parser.function_temp_locals_depth_;
            parser.function_temp_locals_depth_ += 1;
            TemporaryVariableScope {
                parser_: parser,
                local_depth_: local_depth_,
            }
        }

        fn get(&self) -> u32 {
            self.parser_.temp_variable(self.local_depth_)
        }
    }

    impl<'b, 'a> Drop for TemporaryVariableScope<'b, 'a> {
        fn drop(&mut self) {
            debug_assert_eq!(self.local_depth_, self.parser_.function_temp_locals_depth_ - 1);
            self.parser_.function_temp_locals_depth_ -= 1;
        }
    }

    fn get_var_info(&mut self, token: AsmJsScannerToken) -> &mut VarInfo {
        let is_global = AsmJsScanner::is_global(token);
        debug_assert!(is_global || AsmJsScanner::is_local(token));

        let var_info = if is_global {
            &mut self.global_var_info_
        } else {
            &mut self.local_var_info_
        };

        let old_capacity = var_info.len();
        let index = if is_global {
            AsmJsScanner::global_index(token)
        } else {
            AsmJsScanner::local_index(token)
        };

        if is_global && index as usize + 1 > self.num_globals_ {
            self.num_globals_ = index as usize + 1;
        }

        if index as usize + 1 > old_capacity {
            let new_size = max(2 * old_capacity, index as usize + 1);
            var_info.resize(new_size, VarInfo::default()); //Assuming VarInfo can be default constructed
        }
        &mut var_info[index as usize]
    }

    fn var_index(&self, info: &VarInfo) -> u32 {
        debug_assert_eq!(info.kind, VarKind::KGlobal);
        info.index + self.global_imports_.len() as u32
    }

    fn add_global_import(
        &mut self,
        name: Vec<char>,
        type_: &'a AsmType,
        vtype: ValueType,
        mutable_variable: bool,
        info: &mut VarInfo,
    ) {
        // Allocate a separate variable for the import.
        self.declare_global(info, mutable_variable, type_, vtype, WasmInitExpr::default_value(vtype));

        // Record the need to initialize the global from the import.
        self.global_imports_.push(GlobalImport {
            import_name: name,
            value_type: vtype,
            var_info: info,
        });
    }

    fn declare_global(
        &mut self,
        info: &mut VarInfo,
        mutable_variable: bool,
        type_: &'a AsmType,
        vtype: ValueType,
        init: WasmInitExpr,
    ) {
        info.kind = VarKind::KGlobal;
        info.type_ = type_;
        info.index = self.module_builder_.add_global(vtype, true, init);
        info.mutable_variable = mutable_variable;
    }

    fn declare_stdlib_func(&mut self, info: &mut VarInfo, kind: VarKind, type_: &'a AsmType) {
        info.kind = kind;
        info.type_ = type_;
        info.index = 0; // unused
        info.mutable_variable = false;
    }

    fn temp_variable(&mut self, index: i32) -> u32 {
        if index + 1 > self.function_temp_locals_used_ {
            self.function_temp_locals_used_ = index + 1;
        }
        (self.function_temp_locals_offset_ as i32 + index) as u32
    }

    fn copy_current_identifier_string(&self) -> Vec<char> {
        self.zone().clone_vector(self.scanner_.get_identifier_string())
    }

    fn skip_semicolon(&mut self) {
        if self.check(';') {
            // Had a semicolon.
        } else if !self.peek('}') && !self.scanner_.is_preceded_by_newline() {
            fail!("Expected ;", &mut self.scanner_);
        }
    }

    fn begin(&mut self, label: AsmJsScannerToken) {
        self.bare_begin(BlockKind::KRegular, label);
        self.current_function_builder_.unwrap().emit_with_u8(WasmOpcode::kExprBlock, ValueType::Void.to_byte());
    }

    fn loop_(&mut self, label: AsmJsScannerToken) {
        self.bare_begin(BlockKind::KLoop, label);
        let position = self.scanner_.position();
        self.current_function_builder_.unwrap().add_asm_wasm_offset(position, position);
        self.current_function_builder_.unwrap().emit_with_u8(WasmOpcode::kExprLoop, ValueType::Void.to_byte());
    }

    fn end(&mut self) {
        self.bare_end();
        self.current_function_builder_.unwrap().emit(WasmOpcode::kExprEnd);
    }

    fn bare_begin(&mut self, kind: BlockKind, label: AsmJsScannerToken) {
        let info = BlockInfo { kind, label };
        self.block_stack_.push(info);
    }

    fn bare_end(&mut self) {
        debug_assert!(self.block_stack_.len() > 0);
        self.block_stack_.pop();
    }

    fn find_continue_label_depth(&self, label: AsmJsScannerToken) -> i32 {
        let mut count = 0;
        for it in self.block_stack_.iter().rev() {
            // A 'continue' statement targets ...
            //  - The innermost {kLoop} block if no label is given.
            //  - The matching {kLoop} block (when a label is provided).
            if it.kind == BlockKind::KLoop && (label == AsmJsScannerToken::KTokenNone || it.label == label) {
                return count;
            }
            count += 1;
        }
        -1
    }

    fn find_break_label_depth(&self, label: AsmJsScannerToken) -> i32 {
        let mut count = 0;
        for it in self.block_stack_.iter().rev() {
            // A 'break' statement targets ...
            //  - The innermost {kRegular} block if no label is given.
            //  - The matching {kRegular} or {kNamed} block (when a label is provided).
            if (it.kind == BlockKind::KRegular && (label == AsmJsScannerToken::KTokenNone || it.label == label))
                || (it.kind == BlockKind::KNamed && it.label == label)
            {
                return count;
            }
            count += 1;
        }
        -1
    }

    // 6.1 ValidateModule
    fn validate_module(&mut self) -> AsmJsParserResult<()> {
        let stack_limit = &self.stack_limit_;
        recurse!(self.validate_module_parameters(), self, stack_limit);
        expect_token!('{', &mut self.scanner_);
        expect_token!(tok!(UseAsm), &mut self.scanner_);
        recurse!(self.skip_semicolon(), self, stack_limit);
        recurse!(self.validate_module_vars(), self, stack_limit);
        while self.peek(tok!(function)) {
            recurse!(self.validate_function(), self, stack_limit);
        }
        while self.peek(tok!(var)) {
            recurse!(self.validate_function_table(), self, stack_limit);
        }
        recurse!(self.validate_export(), self, stack_limit);
        recurse!(self.skip_semicolon(), self, stack_limit);
        expect_token!('}', &mut self.scanner_);

        // Check that all functions were eventually defined.
        for info in self.global_var_info_.iter().take(self.num_globals_) {
            if info.kind == VarKind::KFunction && !info.function_defined {
                fail!("Undefined function", &mut self.scanner_);
            }
            if info.kind == VarKind::KTable && !info.function_defined {
                fail!("Undefined function table", &mut self.scanner_);
            }
            if info.kind == VarKind::KImportedFunction && !info.function_defined {
                // For imported functions without a single call site, we insert a dummy
                // import here to preserve the fact that there actually was an import.
                let void_void_sig = FunctionSig::builder(self.zone(), 0, 0).get();
                self.module_builder_.add_import(info.import.as_ref().unwrap().function_name.clone(), void_void_sig);
            }
        }

        // Add start function to initialize things.
        let start = self.module_builder_.add_function();
        self.module_builder_.mark_start_function(start);
        for global_import in &self.global_imports_ {
            let import_index = self.module_builder_.add_global_import(
                global_import.import_name.clone(),
                global_import.value_type,
                false, /* mutability */
            );
            start.emit_with_u32v(WasmOpcode::kExprGlobalGet, import_index);
            start.emit_with_u32v(WasmOpcode::kExprGlobalSet, self.var_index(global_import.var_info));
        }
        start.emit(WasmOpcode::kExprEnd);
        let b = FunctionSig::builder(self.zone(), 0, 0);
        start.set_signature(b.get());

        Ok(())
    }

    // 6.1 ValidateModule - parameters
    fn validate_module_parameters(&mut self) -> AsmJsParserResult<()> {
        expect_token!('(', &mut self.scanner_);
        self.stdlib_name_ = AsmJsScannerToken::KTokenNone;
        self.foreign_name_ = AsmJsScannerToken::KTokenNone;
        self.heap_name_ = AsmJsScannerToken::KTokenNone;

        if !self.peek(')') {
            if !self.scanner_.is_global() {
                fail!("Expected stdlib parameter", &mut self.scanner_);
            }
            self.stdlib_name_ = self.consume();
            if !self.peek(')') {
                expect_token!(',', &mut self.scanner_);
                if !self.scanner_.is_global() {
                    fail!("Expected foreign parameter", &mut self.scanner_);
                }
                self.foreign_name_ = self.consume();
                if self.stdlib_name_ == self.foreign_name_ {
                    fail!("Duplicate parameter name", &mut self.scanner_);
                }
                if !self.peek(')') {
                    expect_token!(',', &mut self.scanner_);
                    if !self.scanner_.is_global() {
                        fail!("Expected heap parameter", &mut self.scanner_);
                    }
                    self.heap_name_ = self.consume();
                    if self.heap_name_ == self.stdlib_name_ || self.heap_name_ == self.foreign_name_ {
                        fail!("Duplicate parameter name", &mut self.scanner_);
                    }
                }
            }
        }

        expect_token!(')', &mut self.scanner_);
        Ok(())
    }

    // 6.1 ValidateModule - variables
    fn validate_module_vars(&mut self) -> AsmJsParserResult<()> {
        while self.peek(tok!(var)) || self.peek(tok!(const)) {
            let mut mutable_variable = true;
            if self.check(tok!(var)) {
                // Had a var.
            } else {
                expect_token!(tok!(const), &mut self.scanner_);
                mutable_variable = false;
            }

            loop {
                self.validate_module_var(mutable_variable)?;
                if self.check(',') {
                    continue;
                }
                break;
            }
            self.skip_semicolon();
        }

        Ok(())
    }

    // 6.1 ValidateModule - one variable
    fn validate_module_var(&mut self, mutable_variable: bool) -> AsmJsParserResult<()> {
        if !self.scanner_.is_global() {
            fail!("Expected identifier", &mut self.scanner_);
        }

        let identifier = self.consume();
        if identifier == self.stdlib_name_ || identifier == self.foreign_name_ || identifier == self.heap_name_ {
            fail!("Cannot shadow parameters", &mut self.scanner_);
        }

        let mut info = self.get_var_info(identifier);
        if info.kind != VarKind::KUnused {
            fail!("Redefinition of variable", &mut self.scanner_);
        }

        expect_token!('=', &mut self.scanner_);

        let mut dvalue = 0.0;
        let mut uvalue = 0;
        if self.check_for_double(&mut dvalue) {
            self.declare_global(&mut info, mutable_variable, AsmType::Double(), ValueType::WasmF64, WasmInitExpr::new(dvalue));
        } else if self.check_for_unsigned(&mut uvalue) {
            if uvalue > 0x7FFFFFFF {
                fail!("Numeric literal out of range", &mut self.scanner_);
            }
            self.declare_global(
                &mut info,
                mutable_variable,
                if mutable_variable { AsmType::Int() } else { AsmType::Signed() },
                ValueType::WasmI32,
                WasmInitExpr::new(uvalue as i32),
            );
        } else if self.check('-') {
            if self.check_for_double(&mut dvalue) {
                self.declare_global(&mut info, mutable_variable, AsmType::Double(), ValueType::WasmF64, WasmInitExpr::new(-dvalue));
            } else if self.check_for_unsigned(&mut uvalue) {
                if uvalue > 0x7FFFFFFF {
                    fail!("Numeric literal out of range", &mut self.scanner_);
                }
                if uvalue == 0 {
                    // '-0' is treated as float.
                    self.declare_global(&mut info, mutable_variable, AsmType::Float(), ValueType::WasmF32, WasmInitExpr::new(-0.0f32));
                } else {
                    self.declare_global(
                        &mut info,
                        mutable_variable,
                        if mutable_variable { AsmType::Int() } else { AsmType::Signed() },
                        ValueType::WasmI32,
                        WasmInitExpr::new(-(uvalue as i32)),
                    );
                }
            } else {
                fail!("Expected numeric literal", &mut self.scanner_);
            }
        } else if self.check(tok!(new)) {
            self.validate_module_var_new_stdlib(&mut info)?;
        } else if self.check(self.stdlib_name_) {
            expect_token!('.', &mut self.scanner_);
            self.validate_module_var_stdlib(&mut info)?;
        } else if self.peek(self.foreign_name_) || self.peek('+') {
            self.validate_module_var_import(&mut info, mutable_variable)?;
        } else if self.scanner_.is_global() {
            self.validate_module_var_from_global(&mut info, mutable_variable)?;
        } else {
            fail!("Bad variable declaration", &mut self.scanner_);
        }

        Ok(())
    }

    // 6.1 ValidateModule - global float declaration
    fn validate_module_var_from_global(&mut self, info: &mut VarInfo, mutable_variable: bool) -> AsmJsParserResult<()> {
        let src_info = self.get_var_info(self.consume());
        if !src_info.type_.is_a(self.stdlib_fround_.unwrap()) {
            if src_info.mutable_variable {
                fail!("Can only use immutable variables in global definition", &mut self.scanner_);
            }
            if mutable_variable {
                fail!("Can only define immutable variables with other immutables", &mut self.scanner_);
            }
            if !src_info.type_.is_a(AsmType::Int())
                && !src_info.type_.is_a(AsmType::Float())
                && !src_info.type_.is_a(AsmType::Double())
            {
                fail!("Expected int, float, double, or fround for global definition", &mut self.scanner_);
            }
            info.kind = VarKind::KGlobal;
            info.type_ = src_info.type_;
            info.index = src_info.index;
            info.mutable_variable = false;
            return Ok(());
        }

        expect_token!('(', &mut self.scanner_);
        let mut negate = false;
        if self.check('-') {
            negate = true;
        }

        let mut dvalue = 0.0;
        let mut uvalue = 0;
        if self.check_for_double(&mut dvalue) {
            if negate {
                dvalue = -dvalue;
            }
            self.declare_global(&mut info, mutable_variable, AsmType::Float(), ValueType::WasmF32, WasmInitExpr::new(double_to_float32(dvalue)));
        } else if self.check_for_unsigned(&mut uvalue) {
            dvalue = uvalue as f64;
            if negate {
                dvalue = -dvalue;
            }
            self.declare_global(&mut info, mutable_variable, AsmType::Float(), ValueType::WasmF32, WasmInitExpr::new(dvalue as f32));
        } else {
            fail!("Expected numeric literal", &mut self.scanner_);
        }

        expect_token!(')', &mut self.scanner_);
        Ok(())
    }

    // 6.1 ValidateModule - foreign imports
    fn validate_module_var_import(&mut self, info: &mut VarInfo, mutable_