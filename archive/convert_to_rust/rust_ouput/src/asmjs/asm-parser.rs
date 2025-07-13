// Converted from V8 C++ source files:
// Header: asm-parser.h
// Implementation: asm-parser.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::size_of;
use std::ptr;
use std::{cmp, f32, f64};

use crate::asmjs::asm_scanner::{AsmJsScanner, token_t};
use crate::asmjs::asm_js::AsmType;
use crate::base::enum_set::EnumSet;
use crate::flags::flags::v8_flags;
use crate::numbers::conversions_inl::DoubleToFloat32;
use crate::parsing::scanner::Utf16CharacterStream;
use crate::wasm::wasm_limits::kV8MaxWasmFunctionLocals;
use crate::wasm::wasm_limits::kV8MaxWasmFunctionParams;
use crate::wasm::wasm_module_builder::{WasmFunctionBuilder, WasmInitExpr, WasmModuleBuilder, ModuleTypeIndex, WasmElemSegment};
use crate::wasm::wasm_opcodes::{kExprBlock, kVoidCode, kExprEnd, kExprLoop, kExprIf, kExprElse, kExprReturn, kExprDrop, kExprI32Eqz, kExprBrIf, kExprBr, kExprI32Eq, kExprGlobalGet, kExprGlobalSet, kExprCallFunction, kExprCallIndirect, kExprI32ShrS, kExprI32Shl, kExprI32ShrU, kExprI32Mul, kExprF64Mul, kExprF32Mul, kExprF64Div, kExprF32Div, kExprI32AsmjsDivS, kExprI32AsmjsDivU, kExprF64Mod, kExprI32AsmjsRemS, kExprI32AsmjsRemU, kExprF64Add, kExprF32Add, kExprI32Add, kExprF64Sub, kExprF32Sub, kExprI32Sub, kExprI32And, kExprI32Xor, kExprI32Ior, kExprI32LtS, kExprI32LtU, kExprF64Lt, kExprF32Lt, kExprI32LeS, kExprI32LeU, kExprF64Le, kExprF32Le, kExprI32GtS, kExprI32GtU, kExprF64Gt, kExprF32Gt, kExprI32GeS, kExprI32GeU, kExprF64Ge, kExprF32Ge, kExprI32Eq, kExprI32Ne, kExprF64Eq, kExprF32Eq, kExprF64Ne, kExprF32Ne, kExprF64Abs, kExprF32Abs, kExprF32ConvertF64, kExprF64ConvertF32, kExprF32SConvertI32, kExprF32UConvertI32, kExprF64Min, kExprF64Max, kExprF32Min, kExprF32Max, kI32Code, kF64Code, kF32Code, kExprF64SConvertI32, kExprF64UConvertI32, kExprI32AsmjsSConvertF64, kExprI32AsmjsSConvertF32, kExprF64Const, kExprI32Const, kExprF32Const, kExprF32Const, kExprF32Const, kExprF64Const, kExprF32Const, kExprF64Const, kExprF32Const};
use crate::wasm::value_type::{ValueType, kWasmF64, kWasmF32, kWasmI32, kWasmFuncRef};
use crate::zone::zone::{Zone, ZoneVector, ZoneLinkedList, ZoneUnorderedMap};
use crate::base::vector::{Vector, CStrVector, VectorOf};

use crate::asmjs::asm_js::StandardMember;
use crate::asmjs::asm_js::STDLIB_ARRAY_TYPE_LIST;
use crate::asmjs::asm_js::STDLIB_MATH_VALUE_LIST;
use crate::asmjs::asm_js::STDLIB_MATH_FUNCTION_LIST;
use crate::asmjs::asm_js::AsmCallableType;
use crate::asmjs::asm_js::AsmJs;

use crate::base;
use crate::execution::isolate::Isolate;
use crate::deoptimizer::deoptimizer::Module;
use std::u32;
use std::mem::MaybeUninit;
use std::cmp::max;

#[derive(Debug)]
pub enum AsmJsParserError {
    UnexpectedToken,
    ExpectedIdentifier,
    RedefinitionOfVariable,
    CannotShadowParameters,
    BadVariableDeclaration,
    ExpectedArrayBufferView,
    InvalidMemberOfStdlibMath,
    InvalidMemberOfStdlib,
    IllegalExportName,
    ExpectedFunction,
    FunctionRedefined,
    FunctionTableNameCollides,
    UndefinedFunction,
    ExceededFunctionTableSize,
    FunctionTableSizeDoesNotMatchUses,
    FunctionTableRedefined,
    ExceededMaximumFunctionTableSize,
    ExpectedReturnAtEndOfNonVoidFunction,
    InvalidVoidReturnType,
    IllegalBreak,
    IllegalContinue,
    DoubleLabelUnsupported,
    ExpectedSignedForSwitchValue,
    ExpectedNumericLiteral,
    IntegerNumericLiteralOutOfRange,
    ExpectedType,
    UndefinedLocalVariable,
    UndefinedGlobalVariable,
    InvalidAssignmentTarget,
    IllegalTypeStoredToHeapView,
    ExpectedMutableVariableInAssignment,
    TypeMismatchInAssignment,
    ExpectedInt,
    ExpectedSignedUnsignedDoubleOrFloat,
    IntegerMultiplyOfExpectsInt,
    ConstantMultipleOutOfRange,
    IllegalTypesForAddition,
    ExpectedIntishForOperator,
    ConstantMultipleOutOfRange2,
    ExpectedShiftOfWordSize,
    ExpectedValidHeapAccessShift,
    ExpectedHeapAccessShiftToMatchHeapView,
    ExpectedShiftOfWordSize2,
    ExpectedSignedUnsignedDoubleOrFloat2,
    DuplicateParameterName,
    InitializationFromGlobalRequiresConstVariable,
    CanOnlyDefineImmutableVariablesWithOtherImmutables,
    ExpectedIntFloatDoubleOrFroundForGlobalDefinition,
    ExpectedFRoundOrConstGlobal,
    ExpectedVariableInitialValue,
    BadIntegerParameterAnnotation,
    BadFunctionArgumentType,
    StackOverflowWhileParsingAsmJsModule,
    FunctionUseDoesntMatchDefinition,
    ExpectedCallTable,
    MaskSizeMismatch,
    MoreThan2Power20AdditiveValues,
    NumberParametersExceedsInternalLimit,
    ExpectedPowerOf2Mask,
    BadLocalVariableDefinition,
    ExpectedCallTargetFunction,
    ImportedFunctionArgsMustBeTypeExtern,
    ImportedFunctionCantBeCalledAsFloat,
    ExpectedFround,
    IllegalConversionToFloat,
    NumericLiteralOutOfRange,
    ExpectedHeapAccessShift,
    ExpectedHeapView,
    HeapAccessOutOfRange,
    ExpectedZero,
    ExpectedExtern,
    UnexpectedType,
    ExpectedFround2,
    DuplicateLocalVariableName,
    ExpectedActualType,
    FunctionDefinitionDoesntMatchUse,
    ExpectedFround3,
    FunctionUseDoesntMatchDefinition2,
    NumberLocalVariablesExceedsInternalLimit,
    SizeOfFunctionBodyExceedsInternalLimit,
    ExpectedTableVariable,
    ExpectedMutableOrConst,
    ExpectedVar,
    NoParameterName,
    ExpectedMath,
    FunctionTableNameColldies,
    ExpectedParameterName,
}

impl std::fmt::Display for AsmJsParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AsmJsParserError {}

macro_rules! TRACE_ASM_PARSER {
    ($($arg:tt)*) => {
        if v8_flags.trace_asm_parser {
            print!($($arg)*);
        }
    };
}

pub struct AsmJsParser {
    zone_: *mut Zone,
    scanner_: AsmJsScanner,
    module_builder_: Box<WasmModuleBuilder>,
    current_function_builder_: *mut WasmFunctionBuilder,
    return_type_: *mut AsmType,
    stack_limit_: uintptr_t,
    stdlib_uses_: EnumSet<StandardMember, u64>,
    global_var_info_: Vector<VarInfo>,
    local_var_info_: Vector<VarInfo>,
    num_globals_: usize,
    cached_valuetype_vectors_: CachedVectors<ValueType>,
    cached_asm_type_p_vectors_: CachedVectors<*mut AsmType>,
    cached_token_t_vectors_: CachedVectors<token_t>,
    cached_int_vectors_: CachedVectors<i32>,
    function_temp_locals_offset_: i32,
    function_temp_locals_used_: i32,
    function_temp_locals_depth_: i32,
    failed_: bool,
    failure_message_: Option<String>,
    failure_location_: i32,
    stdlib_name_: token_t,
    foreign_name_: token_t,
    heap_name_: token_t,
    inside_heap_assignment_: bool,
    heap_access_type_: *mut AsmType,
    block_stack_: ZoneVector<BlockInfo>,
    stdlib_dq2d_: *mut AsmType,
    stdlib_dqdq2d_: *mut AsmType,
    stdlib_i2s_: *mut AsmType,
    stdlib_ii2s_: *mut AsmType,
    stdlib_minmax_: *mut AsmType,
    stdlib_abs_: *mut AsmType,
    stdlib_ceil_like_: *mut AsmType,
    stdlib_fround_: *mut AsmType,
    call_coercion_: *mut AsmType,
    call_coercion_position_: usize,
    call_coercion_deferred_: *mut AsmType,
    call_coercion_deferred_position_: usize,
    heap_access_shift_position_: usize,
    heap_access_shift_value_: u32,
    pending_label_: token_t,
    global_imports_: ZoneLinkedList<GlobalImport>,
}

impl AsmJsParser {
    const kTokenNone: token_t = 0;
    const kNoSourcePosition: i32 = -1;
    const kNoHeapAccessShift: usize = usize::MAX;

    pub fn new(zone: *mut Zone, stack_limit: uintptr_t, stream: *mut Utf16CharacterStream) -> AsmJsParser {
        let mut module_builder = Box::new(WasmModuleBuilder::new(unsafe { &mut *zone }));
        module_builder.add_memory(0);

        let mut parser = AsmJsParser {
            zone_: zone,
            scanner_: AsmJsScanner::new(unsafe { &mut *stream }),
            module_builder_: module_builder,
            current_function_builder_: ptr::null_mut(),
            return_type_: ptr::null_mut(),
            stack_limit_: stack_limit,
            stdlib_uses_: EnumSet::new(),
            global_var_info_: Vector::new(),
            local_var_info_: Vector::new(),
            num_globals_: 0,
            cached_valuetype_vectors_: CachedVectors::new(zone),
            cached_asm_type_p_vectors_: CachedVectors::new(zone),
            cached_token_t_vectors_: CachedVectors::new(zone),
            cached_int_vectors_: CachedVectors::new(zone),
            function_temp_locals_offset_: 0,
            function_temp_locals_used_: 0,
            function_temp_locals_depth_: 0,
            failed_: false,
            failure_message_: None,
            failure_location_: Self::kNoSourcePosition,
            stdlib_name_: Self::kTokenNone,
            foreign_name_: Self::kTokenNone,
            heap_name_: Self::kTokenNone,
            inside_heap_assignment_: false,
            heap_access_type_: ptr::null_mut(),
            block_stack_: ZoneVector::new(zone),
            stdlib_dq2d_: ptr::null_mut(),
            stdlib_dqdq2d_: ptr::null_mut(),
            stdlib_i2s_: ptr::null_mut(),
            stdlib_ii2s_: ptr::null_mut(),
            stdlib_minmax_: ptr::null_mut(),
            stdlib_abs_: ptr::null_mut(),
            stdlib_ceil_like_: ptr::null_mut(),
            stdlib_fround_: ptr::null_mut(),
            call_coercion_: ptr::null_mut(),
            call_coercion_position_: 0,
            call_coercion_deferred_: ptr::null_mut(),
            call_coercion_deferred_position_: 0,
            heap_access_shift_position_: Self::kNoHeapAccessShift,
            heap_access_shift_value_: 0,
            pending_label_: Self::kTokenNone,
            global_imports_: ZoneLinkedList::new(zone),
        };
        parser.initialize_stdlib_types();
        parser
    }

    fn initialize_stdlib_types(&mut self) {
        let zone = unsafe { &mut *self.zone_ };
        let d = AsmType::double();
        let dq = AsmType::doubleq();
        self.stdlib_dq2d_ = AsmType::function(zone, d);
        unsafe {
            (&mut *self.stdlib_dq2d_).as_function_type().add_argument(dq);
        }

        self.stdlib_dqdq2d_ = AsmType::function(zone, d);
        unsafe {
            (&mut *self.stdlib_dqdq2d_).as_function_type().add_argument(dq);
            (&mut *self.stdlib_dqdq2d_).as_function_type().add_argument(dq);
        }

        let f = AsmType::float();
        let fh = AsmType::floatish();
        let fq = AsmType::floatq();
        let fq2fh = AsmType::function(zone, fh);
        unsafe {
            (&mut *fq2fh).as_function_type().add_argument(fq);
        }

        let s = AsmType::signed();
        let u = AsmType::unsigned();
        let s2u = AsmType::function(zone, u);
        unsafe {
            (&mut *s2u).as_function_type().add_argument(s);
        }

        let i = AsmType::int();
        self.stdlib_i2s_ = AsmType::function(zone, s);
        unsafe {
            (&mut *self.stdlib_i2s_).as_function_type().add_argument(i);
        }

        self.stdlib_ii2s_ = AsmType::function(zone, s);
        unsafe {
            (&mut *self.stdlib_ii2s_).as_function_type().add_argument(i);
            (&mut *self.stdlib_ii2s_).as_function_type().add_argument(i);
        }

        let minmax_d = AsmType::min_max_type(zone, d, d);
        let minmax_f = AsmType::min_max_type(zone, f, f);
        let minmax_s = AsmType::min_max_type(zone, s, s);
        self.stdlib_minmax_ = AsmType::overloaded_function(zone);
        unsafe {
            (&mut *self.stdlib_minmax_).as_overloaded_function_type().add_overload(minmax_s);
            (&mut *self.stdlib_minmax_).as_overloaded_function_type().add_overload(minmax_f);
            (&mut *self.stdlib_minmax_).as_overloaded_function_type().add_overload(minmax_d);
        }

        self.stdlib_abs_ = AsmType::overloaded_function(zone);
        unsafe {
            (&mut *self.stdlib_abs_).as_overloaded_function_type().add_overload(s2u);
            (&mut *self.stdlib_abs_).as_overloaded_function_type().add_overload(AsmType::function(zone, d));
            (&mut *self.stdlib_abs_).as_overloaded_function_type().add_overload(AsmType::function(zone, fh));
        }

        self.stdlib_ceil_like_ = AsmType::overloaded_function(zone);
        unsafe {
            (&mut *self.stdlib_ceil_like_).as_overloaded_function_type().add_overload(AsmType::function(zone, d));
            (&mut *self.stdlib_ceil_like_).as_overloaded_function_type().add_overload(AsmType::function(zone, fh));
        }

        self.stdlib_fround_ = AsmType::fround_type(zone);
    }

    fn convert_signature(&mut self, return_type: *mut AsmType, params: &ZoneVector<*mut AsmType>) -> *mut FunctionSig {
        let zone = unsafe { &mut *self.zone_ };
        let mut sig_builder = FunctionSig::builder(
            zone,
            if unsafe { !(*return_type).is_a(AsmType::void()) } { 1 } else { 0 },
            params.len(),
        );
        for param in params.iter() {
            if unsafe { (*param).is_a(AsmType::double()) } {
                sig_builder.add_param(kWasmF64);
            } else if unsafe { (*param).is_a(AsmType::float()) } {
                sig_builder.add_param(kWasmF32);
            } else if unsafe { (*param).is_a(AsmType::int()) } {
                sig_builder.add_param(kWasmI32);
            } else {
                unreachable!();
            }
        }

        if unsafe { !(*return_type).is_a(AsmType::void()) } {
            if unsafe { (*return_type).is_a(AsmType::double()) } {
                sig_builder.add_return(kWasmF64);
            } else if unsafe { (*return_type).is_a(AsmType::float()) } {
                sig_builder.add_return(kWasmF32);
            } else if unsafe { (*return_type).is_a(AsmType::signed()) } {
                sig_builder.add_return(kWasmI32);
            } else {
                unreachable!();
            }
        }
        sig_builder.get()
    }

    pub fn run(&mut self) -> Result<(), AsmJsParserError> {
        self.validate_module()?;
        Ok(())
    }

    fn bare_begin(&mut self, kind: BlockKind, label: token_t) {
        let info = BlockInfo { kind, label };
        unsafe { (&mut *self.zone_).extend_lifetime(info) };
        self.block_stack_.push_back(info);
    }

    fn bare_end(&mut self) {
        debug_assert!(self.block_stack_.len() > 0);
        self.block_stack_.pop_back();
    }

    fn add_global_import(
        &mut self,
        name: Vector<*const i8>,
        type_: *mut AsmType,
        vtype: ValueType,
        mutable_variable: bool,
        info: *mut VarInfo,
    ) {
        self.declare_global(info, mutable_variable, type_, vtype, WasmInitExpr::default_value(vtype));

        let import = GlobalImport {
            import_name: name,
            value_type: vtype,
            var_info: info,
        };
        self.global_imports_.push_back(import);
    }

    fn declare_global(
        &mut self,
        info: *mut VarInfo,
        mutable_variable: bool,
        type_: *mut AsmType,
        vtype: ValueType,
        init: WasmInitExpr,
    ) {
        unsafe {
            (*info).kind = VarKind::kGlobal;
            (*info).type_ = type_;
            (*info).index = self.module_builder_.add_global(vtype, true, init);
            (*info).mutable_variable = mutable_variable;
        }
    }

    fn declare_stdlib_func(&mut self, info: *mut VarInfo, kind: VarKind, type_: *mut AsmType) {
        unsafe {
            (*info).kind = kind;
            (*info).type_ = type_;
            (*info).index = 0;
            (*info).mutable_variable = false;
        }
    }

    fn temp_variable(&mut self, index: i32) -> u32 {
        if index + 1 > self.function_temp_locals_used_ {
            self.function_temp_locals_used_ = index + 1;
        }
        (self.function_temp_locals_offset_ + index) as u32
    }

    fn copy_current_identifier_string(&mut self) -> Vector<*const i8> {
        unsafe { (&mut *self.zone_).clone_vector(VectorOf(self.scanner_.get_identifier_string())) }
    }

    fn var_index(&self, info: *mut VarInfo) -> u32 {
        debug_assert!(unsafe { (*info).kind == VarKind::kGlobal });
        (unsafe { (*info).index } + (self.global_imports_.len() as u32)) as u32
    }

    fn skip_semicolon(&mut self) -> Result<(), AsmJsParserError> {
        if self.check(';') {
            Ok(())
        } else if !self.peek('}') && !self.scanner_.is_preceded_by_newline() {
            self.fail("Expected ;")
        } else {
            Ok(())
        }
    }

    fn get_var_info(&mut self, token: token_t) -> *mut VarInfo {
        let is_global = self.scanner_.is_global(token);
        debug_assert!(is_global || self.scanner_.is_local(token));

        let var_info = if is_global {
            &mut self.global_var_info_
        } else {
            &mut self.local_var_info_
        };

        let index = if is_global {
            self.scanner_.global_index(token)
        } else {
            self.scanner_.local_index(token)
        };

        if is_global && (index + 1) as usize > self.num_globals_ {
            self.num_globals_ = (index + 1) as usize;
        }

        if (index + 1) as usize > var_info.len() {
            let new_size = max(2 * var_info.len(), (index + 1) as usize);
            let mut new_info: Vector<VarInfo> = Vector::new();
            new_info.resize(new_size, VarInfo::default());

            for i in 0..var_info.len() {
                new_info[i] = var_info[i].clone();
            }

            if is_global {
                self.global_var_info_ = new_info;
            } else {
                self.local_var_info_ = new_info;
            }
        }

        if is_global {
            unsafe { &mut self.global_var_info_[index as usize] }
        } else {
            unsafe { &mut self.local_var_info_[index as usize] }
        }
    }

    fn peek(&self, token: char) -> bool {
        self.scanner_.token() as u8 == token as u8
    }

    fn peek_for_zero(&self) -> bool {
        self.scanner_.is_unsigned() && self.scanner_.as_unsigned() == 0
    }

    fn check(&mut self, token: char) -> bool {
        if self.scanner_.token() as u8 == token as u8 {
            self.scanner_.next();
            true
        } else {
            false
        }
    }

    fn check_for_zero(&mut self) -> bool {
        if self.scanner_.is_unsigned() && self.scanner_.as_unsigned() == 0 {
            self.scanner_.next();
            true
        } else {
            false
        }
    }

    fn check_for_double(&mut self, value: &mut f64) -> bool {
        if self.scanner_.is_double() {
            *value = self.scanner_.as_double();
            self.scanner_.next();
            true
        } else {
            false
        }
    }

    fn check_for_unsigned(&mut self, value: &mut u32) -> bool {
        if self.scanner_.is_unsigned() {
            *value = self.scanner_.as_unsigned();
            self.scanner_.next();
            true
        } else {
            false
        }
    }

    fn check_for_unsigned_below(&mut self, limit: u32, value: &mut u32) -> bool {
        if self.scanner_.is_unsigned() && self.scanner_.as_unsigned() < limit {
            *value = self.scanner_.as_unsigned();
            self.scanner_.next();
            true
        } else {
            false
        }
    }

    fn consume(&mut self) -> token_t {
        let ret = self.scanner_.token();
        self.scanner_.next();
        ret
    }

    fn fail<T>(&mut self, msg: &str) -> Result<T, AsmJsParserError> {
        self.failed_ = true;
        self.failure_message_ = Some(msg.to_string());
        self.failure_location_ = self.scanner_.position() as i32;

        TRACE_ASM_PARSER!(
            "[asm.js failure: {}, token: '{}', see: {}:{}]\n",
            msg,
            self.scanner_.name(self.scanner_.token()),
            file!(),
            line!()
        );

        Err(match msg {
                "Unexpected token" => AsmJsParserError::UnexpectedToken,
                "Expected identifier" => AsmJsParserError::ExpectedIdentifier,
                "Redefinition of variable" => AsmJsParserError::RedefinitionOfVariable,
                "Cannot shadow parameters" => AsmJsParserError::CannotShadowParameters,
                "Bad variable declaration" => AsmJsParserError::BadVariableDeclaration,
                "Expected ArrayBuffer view" => AsmJsParserError::ExpectedArrayBufferView,
                "Invalid member of stdlib.Math" => AsmJsParserError::InvalidMemberOfStdlibMath,
                "Invalid member of stdlib" => AsmJsParserError::InvalidMemberOfStdlib,
                "Illegal export name" => AsmJsParserError::IllegalExportName,
                "Expected function" => AsmJsParserError::ExpectedFunction,
                "Function redefined" => AsmJsParserError::FunctionRedefined,
                "Function table name collides" => AsmJsParserError::FunctionTableNameCollides,
                "Undefined function" => AsmJsParserError::UndefinedFunction,
                "Exceeded function table size" => AsmJsParserError::ExceededFunctionTableSize,
                "Function table size does not match uses" => AsmJsParserError::FunctionTableSizeDoesNotMatchUses,
                "Number of parameters exceeds internal limit" => AsmJsParserError::NumberParametersExceedsInternalLimit,
                "More than 2^20 additive values" => AsmJsParserError::MoreThan2Power20AdditiveValues,
                "Stack overflow while parsing asm.js module." => AsmJsParserError::StackOverflowWhileParsingAsmJsModule,
                "Illegal break" => AsmJsParserError::IllegalBreak,
                "Illegal continue" => AsmJsParserError::IllegalContinue,
                "Expected fround or const global" => AsmJsParserError::ExpectedFRoundOrConstGlobal,
                "Expected variable initial value" => AsmJsParserError::ExpectedVariableInitialValue,
                "Duplicate parameter name" => AsmJsParserError::DuplicateParameterName,
                "Initialization from global requires const variable" => AsmJsParserError::InitializationFromGlobalRequiresConstVariable,
                "Can only define immutable variables with other immutables" => AsmJsParserError::CanOnlyDefineImmutableVariablesWithOtherImmutables,
                "Expected int, float, double, or fround for global definition" => AsmJsParserError::ExpectedIntFloatDoubleOrFroundForGlobalDefinition,
                "Function redefined" => AsmJsParserError::FunctionRedefined,
                "Bad function argument type" => AsmJsParserError::BadFunctionArgumentType,
                "Function use doesn't match definition" => AsmJsParserError::FunctionUseDoesntMatchDefinition,
                "Expected signed" => AsmJsParserError::ExpectedSignedForSwitchValue,
                _ => AsmJsParserError::UnexpectedToken,
        })
    }

    unsafe fn current_stack_position(&self) -> uintptr_t {
        0
    }

    fn begin(&mut self, label: token_t) {
        self.bare_begin(BlockKind::kRegular, label);
        unsafe {
            (&mut *self.current_function_builder_)
                .emit_with_u8(kExprBlock, kVoidCode);
        }
    }

    fn loop_(&mut self, label: token_t) {
        self.bare_begin(BlockKind::kLoop, label);
        let position = self.scanner_.position();
        unsafe {
            (&mut *self.current_function_builder_).add_asm_wasm_offset(position, position);
            (&mut *self.current_function_builder_).emit_with_u8(kExprLoop, kVoidCode);
        }
    }

    fn end(&mut self) {
        self.bare_end();
        unsafe {
            (&mut *self.current_function_builder_).emit(kExprEnd);
        }
    }

    fn find_continue_label_depth(&self, label: token_t) -> i32 {
        let mut count = 0;
        for it in self.block_stack_.iter().rev() {
            if it.kind == BlockKind::kLoop && (label == Self::kTokenNone || it.label == label) {
                return count;
            }
            count += 1;
        }
        -1
    }

    fn find_break_label_depth(&self, label: token_t) -> i32 {
        let mut count = 0;
        for it in self.block_stack_.iter().rev() {
            if (it.kind == BlockKind::kRegular && (label == Self::kTokenNone || it.label == label))
                || (it.kind == BlockKind::kNamed && it.label == label)
            {
                return count;
            }
            count += 1;
        }
        -1
    }

    fn validate_module(&mut self) -> Result<(), AsmJsParserError> {
        self.validate_module_parameters()?;
        self.expect_token('{')?;
        self.expect_token(AsmJsScanner::kToken_UseAsm)?;
        self.skip_semicolon()?;
        self.validate_module_vars()?;

        while self.peek(AsmJsScanner::kToken_function) {
            self.validate_function()?;
        }

        while self.peek(AsmJsScanner::kToken_var) {
            self.validate_function_table()?;
        }

        self.validate_export()?;
        self.skip_semicolon()?;
        self.expect_token('}')?;

        for i in 0..self.num_globals_ {
            let info = unsafe { &self.global_var_info_[i] };
            if info.kind == VarKind::kFunction && !info.function_defined {
                return self.fail("Undefined function");
            }
            if info.kind == VarKind::kTable && !info.function_defined {
                return self.fail("Undefined function table");
            }
            if info.kind == VarKind::kImportedFunction && !info.function_defined {
                let void_void_sig = FunctionSig::builder(unsafe { &mut *self.zone_ }, 0, 0).get();
                self.module_builder_.add_import(
                    unsafe { (*info.import).function_name },
                    void_void_sig,
                );
            }
        }

        let start = self.module_builder_.add_function();
        self.module_builder_.mark_start_function(start);

        for global_import in self.global_imports_.iter() {
            let import_index = self.module_builder_.add_global_import(
                global_import.import_name,
                global_import.value_type,
                false,
            );
            unsafe {
                (&mut *start).emit_with_u32v(kExprGlobalGet, import_index);
                (&mut *start).emit_with_u32v(kExprGlobalSet, self.var_index(global_import.var_info));
            }
        }

        unsafe { (&mut *start).emit(kExprEnd) };
        let mut b = FunctionSig::builder(unsafe { &mut *self.zone_ }, 0, 0);
        unsafe { (&mut *start).set_signature(b.get()) };

        Ok(())
    }

    fn validate_module_parameters(&mut self) -> Result<(), AsmJsParserError> {
        self.expect_token('(')?;

        self.stdlib_name_ = Self::kTokenNone;
        self.foreign_name_ = Self::kTokenNone;
        self.heap_name_ = Self::kTokenNone;

        if !self.peek(')') {
            
