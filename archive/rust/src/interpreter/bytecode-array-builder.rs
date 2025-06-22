// src/interpreter/bytecode_array_builder.rs

use std::{
    convert::TryFrom,
    marker::PhantomData,
    num::TryFromIntError,
    ops::{Deref, DerefMut},
    rc::Rc,
};

// Placeholder for v8::internal namespace
mod internal {
    // Placeholder for v8::internal::interpreter namespace
    pub mod interpreter {
        // Dummy types and functions to satisfy the C++ code.  These would need
        // proper implementations to fully replace the C++ code.

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Register(i32);

        impl Register {
            pub fn invalid() -> Self {
                Register(i32::MIN) // Some invalid value
            }

            pub fn is_valid(&self) -> bool {
                self.0 != i32::MIN
            }

            pub fn is_current_context(&self) -> bool {
                self.0 == -1000 // Dummy check, replace with real logic
            }

            pub fn is_function_closure(&self) -> bool {
                self.0 == -2000 // Dummy check, replace with real logic
            }

            pub fn is_parameter(&self) -> bool {
                self.0 < 0 && self.0 > -1000
            }

            pub fn to_parameter_index(&self) -> i32 {
                -self.0
            }

            pub fn index(&self) -> i32 {
                self.0.abs()
            }

            pub fn to_operand(&self) -> u32 {
                self.0 as u32
            }

            pub fn try_to_short_star(&self) -> Option<Bytecode> {
                if self.0 >= 0 && self.0 <= 5 {
                    match self.0 {
                        0 => Some(Bytecode::Star0),
                        1 => Some(Bytecode::Star1),
                        2 => Some(Bytecode::Star2),
                        3 => Some(Bytecode::Star3),
                        4 => Some(Bytecode::Star4),
                        5 => Some(Bytecode::Star5),
                        _ => None,
                    }
                } else {
                    None
                }
            }

            pub fn from_parameter_index(index: i32) -> Self {
                Register(-index)
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct RegisterList {
            first_register: Register,
            register_count: i32,
        }

        impl RegisterList {
            pub fn new(first_register: Register, register_count: i32) -> Self {
                RegisterList {
                    first_register,
                    register_count,
                }
            }

            pub fn register_count(&self) -> i32 {
                self.register_count
            }

            pub fn first_register(&self) -> Register {
                self.first_register
            }

            pub fn get(&self, index: usize) -> Register {
                if index < self.register_count as usize {
                    Register(self.first_register.0 + index as i32)
                } else {
                    Register::invalid()
                }
            }

            pub fn get_mut(&mut self, index: usize) -> Option<&mut Register> {
                if index < self.register_count as usize {
                    Some(&mut Register(self.first_register.0 + index as i32))
                } else {
                    None
                }
            }
        }

        impl std::ops::Index<usize> for RegisterList {
            type Output = Register;

            fn index(&self, index: usize) -> &Self::Output {
                assert!(index < self.register_count as usize);
                let reg = Register(self.first_register.0 + index as i32);
                &reg
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Bytecode {
            Nop,
            Ldar,
            Star,
            Mov,
            LdaZero,
            LdaSmi,
            LdaUndefined,
            LdaNull,
            LdaTheHole,
            LdaTrue,
            LdaFalse,
            LdaConstant,
            Star0,
            Star1,
            Star2,
            Star3,
            Star4,
            Star5,
            Add,
            Sub,
            Mul,
            Div,
            Mod,
            Exp,
            BitwiseOr,
            BitwiseXor,
            BitwiseAnd,
            ShiftLeft,
            ShiftRight,
            ShiftRightLogical,
            AddSmi,
            SubSmi,
            MulSmi,
            DivSmi,
            ModSmi,
            ExpSmi,
            BitwiseOrSmi,
            BitwiseXorSmi,
            BitwiseAndSmi,
            ShiftLeftSmi,
            ShiftRightSmi,
            ShiftRightLogicalSmi,
            Inc,
            Dec,
            ToNumber,
            Negate,
            BitwiseNot,
            LogicalNot,
            ToBooleanLogicalNot,
            TypeOf,
            GetSuperConstructor,
            FindNonDefaultConstructorOrConstruct,
            TestEqual,
            TestEqualStrict,
            TestLessThan,
            TestGreaterThan,
            TestLessThanOrEqual,
            TestGreaterThanOrEqual,
            TestInstanceOf,
            TestIn,
            TestReferenceEqual,
            TestUndetectable,
            TestUndefined,
            TestNull,
            TestTypeOf,
            LdaGlobal,
            LdaGlobalInsideTypeof,
            StaGlobal,
            LdaImmutableCurrentContextSlot,
            LdaImmutableContextSlot,
            LdaCurrentScriptContextSlot,
            LdaScriptContextSlot,
            LdaCurrentContextSlot,
            LdaContextSlot,
            StaCurrentScriptContextSlot,
            StaScriptContextSlot,
            StaCurrentContextSlot,
            StaContextSlot,
            LdaLookupSlot,
            LdaLookupSlotInsideTypeof,
            LdaLookupScriptContextSlotInsideTypeof,
            LdaLookupContextSlotInsideTypeof,
            LdaLookupScriptContextSlot,
            LdaLookupContextSlot,
            LdaLookupGlobalSlotInsideTypeof,
            LdaLookupGlobalSlot,
            StaLookupSlot,
            GetNamedProperty,
            GetNamedPropertyFromSuper,
            GetKeyedProperty,
            GetEnumeratedKeyedProperty,
            GetIterator,
            DefineKeyedOwnPropertyInLiteral,
            SetNamedProperty,
            DefineNamedOwnProperty,
            SetKeyedProperty,
            DefineKeyedOwnProperty,
            StaInArrayLiteral,
            GetTemplateObject,
            CreateClosure,
            CreateBlockContext,
            CreateCatchContext,
            CreateFunctionContext,
            CreateEvalContext,
            CreateWithContext,
            CreateMappedArguments,
            CreateUnmappedArguments,
            CreateRestParameter,
            CreateRegExpLiteral,
            CreateEmptyArrayLiteral,
            CreateArrayLiteral,
            CreateArrayFromIterable,
            CreateObjectLiteral,
            CreateEmptyObjectLiteral,
            CloneObject,
            PushContext,
            PopContext,
            ToObject,
            ToName,
            ToString,
            ToBoolean,
            ToNumeric,
            Jump,
            JumpIfTrue,
            JumpIfFalse,
            JumpIfNull,
            JumpIfNotNull,
            JumpIfUndefined,
            JumpIfUndefinedOrNull,
            JumpIfNotUndefined,
            JumpIfJSReceiver,
            JumpIfForInDone,
            JumpLoop,
            SwitchOnSmiNoFeedback,
            SetPendingMessage,
            Throw,
            ReThrow,
            Abort,
            Return,
            ThrowReferenceErrorIfHole,
            ThrowSuperNotCalledIfHole,
            ThrowSuperAlreadyCalledIfNotHole,
            ThrowIfNotSuperConstructor,
            Debugger,
            IncBlockCounter,
            ForInEnumerate,
            ForInPrepare,
            ForInNext,
            ForInStep,
            StaModuleVariable,
            LdaModuleVariable,
            SuspendGenerator,
            SwitchOnGeneratorState,
            ResumeGenerator,
            CallProperty0,
            CallProperty1,
            CallProperty2,
            CallProperty,
            CallUndefinedReceiver0,
            CallUndefinedReceiver1,
            CallUndefinedReceiver2,
            CallUndefinedReceiver,
            CallAnyReceiver,
            CallWithSpread,
            Construct,
            ConstructWithSpread,
            ConstructForwardAllArgs,
            CallRuntime,
            CallRuntimeForPair,
            CallJSRuntime,
            DeletePropertySloppy,
            DeletePropertyStrict,
            OutputTypeOf, // Added this as it's referenced
        }

        pub mod Bytecodes {
            use super::Bytecode;
            pub const kMaxOperands: usize = 3;
            pub fn is_forward_jump(_bytecode: Bytecode) -> bool {
                false
            }
            pub fn size_for_unsigned_operand<T>(_value: T) -> OperandSize {
                OperandSize::Byte
            }
            pub fn is_without_external_side_effects(_bytecode: Bytecode) -> bool {
                false
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum OperandSize {
            Byte,
            Short,
            Long,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum OperandType {
            kImm,
            kReg,
            kRegList,
            kRegPair,
            kRegOut,
            kRegOutList,
            kRegOutPair,
            kRegOutTriple,
            kRegInOut,
            kUByte,
            kUShort,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ImplicitRegisterUse {
            None,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ContextKind {
            kAnyContext,
            kScriptContext,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum LanguageMode {
            Sloppy,
            Strict,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum LookupHoistingMode {
            Normal,
        }

        pub mod StoreLookupSlotFlags {
            use super::LanguageMode;
            use super::LookupHoistingMode;
            pub fn encode(
                _language_mode: LanguageMode,
                _lookup_hoisting_mode: LookupHoistingMode,
            ) -> u8 {
                0
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum NilValue {
            kNullValue,
            kUndefinedValue,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TypeofMode {
            kInside,
            kNotInside,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum DefineKeyedOwnPropertyInLiteralFlags {}
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum DefineKeyedOwnPropertyFlags {}

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CreateArgumentsType {
            kMappedArguments,
            kUnmappedArguments,
            kRestParameter,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ToBooleanMode {
            kAlreadyBoolean,
            kConvertToBoolean,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct SourcePositionTableBuilder {
            recording_mode: RecordingMode,
        }

        impl SourcePositionTableBuilder {
            pub fn new(recording_mode: RecordingMode) -> Self {
                SourcePositionTableBuilder { recording_mode }
            }

            pub enum RecordingMode {
                AllSourcePositions,
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct AbortReason {
            _private: i32,
        }

        impl AbortReason {
            pub const kNoReason: AbortReason = AbortReason { _private: 0 };
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum FeedbackSlotKind {
            kLoadGlobalIC,
            kStoreGlobalIC,
            kDefineNamedOwn,
            kLoadPropertyIC,
            kStorePropertyIC,
            kCallIC,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct FeedbackVectorSpec {}

        impl FeedbackVectorSpec {
            pub fn get_kind(&self, _slot: FeedbackVector::Slot) -> FeedbackSlotKind {
                FeedbackSlotKind::kLoadGlobalIC
            }
        }

        pub mod FeedbackVector {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct Slot {}
            pub fn to_slot(_index: i32) -> Slot {
                Slot {}
            }
        }

        pub fn get_language_mode_from_slot_kind(
            _kind: FeedbackSlotKind,
        ) -> LanguageMode {
            LanguageMode::Sloppy
        }
        pub fn get_typeof_mode_from_slot_kind(_kind: FeedbackSlotKind) -> TypeofMode {
            TypeofMode::kNotInside
        }

        pub struct BytecodeArray(Vec<u8>);
        impl BytecodeArray {
            pub fn new(size: usize) -> Self {
                BytecodeArray(vec![0; size])
            }
        }

        pub struct TrustedByteArray(Vec<u8>);
        impl TrustedByteArray {
            pub fn new(size: usize) -> Self {
                TrustedByteArray(vec![0; size])
            }
        }

        pub struct FeedbackVector(_private: i32); // Dummy

        pub struct AstRawString {
            _private: i32,
        }
        pub struct AstConsString {
            _private: i32,
        }
        pub struct Scope {
            _private: i32,
        }
        pub struct AstBigInt {
            _private: i32,
        }

        pub struct Isolate {}

        pub mod Runtime {
            pub struct FunctionId {}
            pub fn function_for_id(_id: FunctionId) -> FunctionInfo {
                FunctionInfo { result_size: 1 }
            }

            pub struct FunctionInfo {
                pub result_size: i32,
            }
        }

        pub mod IntrinsicsHelper {
            pub fn is_supported(_id: super::Runtime::FunctionId) -> bool {
                false
            }
            pub fn from_runtime_id(_id: super::Runtime::FunctionId) -> IntrinsicId {
                IntrinsicId {}
            }

            pub struct IntrinsicId {}
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TestTypeOfFlags {
            kOther,
        }

        impl TestTypeOfFlags {
            pub fn encode(_flag: TestTypeOfFlags::LiteralFlag) -> i32 {
                0
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum LiteralFlag {
                kOther,
            }
        }

        pub fn double_to_smi_integer(_value: f64, _smi: &mut i32) -> bool {
            false
        }

        pub struct Smi {
            value: i32,
        }
        impl Smi {
            pub fn from_int(value: i32) -> Self {
                Smi { value }
            }

            pub fn value(&self) -> i32 {
                self.value
            }
        }

        pub struct Variable {
            index: i32,
            mode: VariableMode,
            scope: ScopeRef,
        }

        impl Variable {
            pub fn index(&self) -> i32 {
                self.index
            }
            pub fn mode(&self) -> VariableMode {
                self.mode
            }
            pub fn scope(&self) -> &ScopeRef {
                &self.scope
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum VariableMode {
            kLet,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ContextSlotMutability {
            kMutableSlot,
            kImmutableSlot,
        }

        pub struct DirectHandle<T>(PhantomData<T>); // Dummy

        pub struct LocalIsolate {}
        pub type Handle<T> = Rc<T>;

        pub struct Flags {
            pub ignition_reo: bool,
            pub ignition_filter_expression_positions: bool,
            pub script_context_mutable_heap_number: bool,
            pub const_tracking_let: bool,
        }

        pub struct ScopeRef {
            id: i32,
        }
        impl ScopeRef {
            pub fn is_script_scope(&self) -> bool {
                self.id > 0
            }
        }

        // Forward declarations
        pub struct BytecodeArrayBuilder;

        // Dummy RegisterTransferWriter. Needs implementation if register optimization is used.
        struct RegisterTransferWriter {
            builder: *mut BytecodeArrayBuilder,
        }

        impl RegisterTransferWriter {
            fn new(builder: *mut BytecodeArrayBuilder) -> Self {
                RegisterTransferWriter { builder }
            }

            fn emit_ldar(&mut self, _input: Register) {
                unsafe {
                    (*self.builder).output_ldar_raw(_input);
                }
            }

            fn emit_star(&mut self, _output: Register) {
                unsafe {
                    (*self.builder).output_star_raw(_output);
                }
            }

            fn emit_mov(&mut self, _input: Register, _output: Register) {
                unsafe {
                    (*self.builder).output_mov_raw(_input, _output);
                }
            }
        }

        pub struct BytecodeRegisterOptimizer {
            _private: i32, // Dummy for now.
        }

        impl BytecodeRegisterOptimizer {
            fn new(
                _zone: &Zone,
                _register_allocator: &RegisterAllocator,
                _fixed_register_count: i32,
                _parameter_count: i32,
                _writer: &RegisterTransferWriter,
            ) -> Self {
                BytecodeRegisterOptimizer { _private: 0 }
            }
            fn flush(&mut self) {}
            fn maxiumum_register_index(&self) -> i32 {
                0
            }
            fn do_ldar(&mut self, _reg: Register) {}
            fn do_star(&mut self, _reg: Register) {}
            fn do_mov(&mut self, _from: Register, _to: Register) {}
            fn prepare_for_bytecode<const BYTECODE: Bytecode, const IMPLICIT_REGISTER_USE: ImplicitRegisterUse>(
                &mut self,
            ) {
            }
            fn get_input_register(&mut self, reg: Register) -> Register {
                reg
            }
            fn prepare_output_register(&mut self, reg: Register) {}
            fn ensure_all_registers_are_flushed(&self) -> bool {
                false
            }
            fn is_accumulator_reset(&self) -> bool {
                false
            }
            fn get_input_register_list(&mut self, reg_list: RegisterList) -> RegisterList {
                reg_list
            }
            fn prepare_output_register_list(&mut self, reg_list: RegisterList) {}
            fn reset_type_hint_for_accumulator(&mut self) {}
        }

        pub struct RegisterAllocator {
            _private: i32,
        }
        impl RegisterAllocator {
            fn new(_fixed_register_count: i32) -> Self {
                RegisterAllocator { _private: 0 }
            }

            fn register_is_live(&self, _reg: Register) -> bool {
                false
            }
        }

        pub struct ConstantArrayBuilder {
            _private: i32,
        }

        impl ConstantArrayBuilder {
            fn new(_zone: &Zone) -> Self {
                ConstantArrayBuilder { _private: 0 }
            }

            fn insert(&mut self, _raw_string: &AstRawString) -> usize {
                0
            }

            fn insert(&mut self, _cons_string: &AstConsString) -> usize {
                0
            }

            fn insert(&mut self, _bigint: AstBigInt) -> usize {
                0
            }

            fn insert(&mut self, _scope: &Scope) -> usize {
                0
            }

            fn insert(&mut self, _number: f64) -> usize {
                0
            }

            fn insert_jump_table(&mut self, _size: usize) -> usize {
                0
            }

            fn insert_deferred(&mut self) -> usize {
                0
            }

            fn set_deferred_at(&mut self, _entry: usize, _object: Rc<dyn std::any::Any>) {}

            fn insert_iterator_symbol(&mut self) -> usize {
                0
            }
            fn insert_async_iterator_symbol(&mut self) -> usize {
                0
            }
            fn insert_class_fields_symbol(&mut self) -> usize {
                0
            }
        }

        pub struct HandlerTableBuilder {
            _private: i32,
        }

        impl HandlerTableBuilder {
            fn new(_zone: &Zone) -> Self {
                HandlerTableBuilder { _private: 0 }
            }

            fn to_handler_table(_self: &HandlerTableBuilder) -> DirectHandle<TrustedByteArray> {
                DirectHandle(PhantomData)
            }

            fn set_prediction(&mut self, _handler_id: i32, _catch_prediction: CatchPrediction) {}

            fn set_context_register(&mut self, _handler_id: i32, _context: Register) {}
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CatchPrediction {
            kCatch,
        }

        pub struct BytecodeLabel {
            _private: i32, // Dummy
        }

        impl BytecodeLabel {
            pub fn new() -> Self {
                BytecodeLabel { _private: 0 }
            }

            pub fn is_bound(&self) -> bool {
                false
            }

            pub fn has_referrer_jump(&self) -> bool {
                false
            }
        }

        pub struct BytecodeLoopHeader {
            _private: i32, // Dummy
        }

        impl BytecodeLoopHeader {
            pub fn new() -> Self {
                BytecodeLoopHeader { _private: 0 }
            }
        }

        pub struct BytecodeJumpTable {
            constant_pool_index: usize,
            size: i32,
            case_value_base: i32,
            _private: i32, // Dummy
        }

        impl BytecodeJumpTable {
            pub fn new(constant_pool_index: usize, size: i32, case_value_base: i32) -> Self {
                BytecodeJumpTable {
                    constant_pool_index,
                    size,
                    case_value_base,
                    _private: 0,
                }
            }

            pub fn constant_pool_index(&self) -> usize {
                self.constant_pool_index
            }

            pub fn size(&self) -> i32 {
                self.size
            }

            pub fn case_value_base(&self) -> i32 {
                self.case_value_base
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct BytecodeSourceInfo {
            source_position: i32,
            is_statement: bool,
            is_valid: bool,
        }

        impl BytecodeSourceInfo {
            pub fn new() -> Self {
                BytecodeSourceInfo {
                    source_position: 0,
                    is_statement: false,
                    is_valid: false,
                }
            }
            pub fn is_valid(&self) -> bool {
                self.is_valid
            }
            pub fn set_invalid(&mut self) {
                self.is_valid = false;
            }

            pub fn is_statement(&self) -> bool {
                self.is_statement
            }

            pub fn is_expression(&self) -> bool {
                !self.is_statement
            }

            pub fn make_statement_position(&mut self, source_position: i32) {
                self.source_position = source_position;
                self.is_statement = true;
            }

            pub fn force_expression_position(&mut self, position: i32) {
                self.source_position = position;
                self.is_statement = false;
                self.is_valid = true;
            }

            pub fn source_position(&self) -> i32 {
                self.source_position
            }
        }

        pub struct BytecodeNode {
            bytecode: Bytecode,
            operands: Vec<u32>,
            source_info: BytecodeSourceInfo,
        }

        impl BytecodeNode {
            pub fn new(bytecode: Bytecode) -> Self {
                BytecodeNode {
                    bytecode,
                    operands: Vec::new(),
                    source_info: BytecodeSourceInfo::new(),
                }
            }
            pub fn ldar(source_info: BytecodeSourceInfo, operand: u32) -> Self {
                BytecodeNode {
                    bytecode: Bytecode::Ldar,
                    operands: vec![operand],
                    source_info,
                }
            }

            pub fn star(source_info: BytecodeSourceInfo, operand: u32) -> Self {
                BytecodeNode {
                    bytecode: Bytecode::Star,
                    operands: vec![operand],
                    source_info,
                }
            }
            pub fn mov(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32) -> Self {
                BytecodeNode {
                    bytecode: Bytecode::Mov,
                    operands: vec![operand0, operand1],
                    source_info,
                }
            }

            pub fn create<
                const BYTECODE: Bytecode,
                const IMPLICIT_REGISTER_USE: ImplicitRegisterUse,
                const OPERAND_TYPES: (),
            >(
                source_info: BytecodeSourceInfo,
            ) -> Self {
                BytecodeNode {
                    bytecode: BYTECODE,
                    operands: Vec::new(),
                    source_info,
                }
            }

            pub fn create<
                const BYTECODE: Bytecode,
                const IMPLICIT_REGISTER_USE: ImplicitRegisterUse,
                const OPERAND_TYPES: (OperandType, OperandType),
            >(
                source_info: BytecodeSourceInfo,
                operand0: u32,
                operand1: u32,
            ) -> Self {
                BytecodeNode {
                    bytecode: BYTECODE,
                    operands: vec![operand0, operand1],
                    source_info,
                }
            }

            pub fn create<
                const BYTECODE: Bytecode,
                const IMPLICIT_REGISTER_USE: ImplicitRegisterUse,
                const OPERAND_TYPES: (OperandType, OperandType, OperandType),
            >(
                source_info: BytecodeSourceInfo,
                operand0: u32,
                operand1: u32,
                operand2: u32,
            ) -> Self {
                BytecodeNode {
                    bytecode: BYTECODE,
                    operands: vec![operand0, operand1, operand2],
                    source_info,
                }
            }

            pub fn set_source_info(&mut self, source_info: BytecodeSourceInfo) {
                self.source_info = source_info;
            }

            pub fn source_info(&self) -> BytecodeSourceInfo {
                self.source_info
            }
        }

        pub struct BytecodeArrayWriter {
            constant_array_builder: *mut ConstantArrayBuilder,
            source_position_mode: SourcePositionTableBuilder::RecordingMode,
            _private: i32, // Dummy
        }

        impl BytecodeArrayWriter {
            fn new(
                _zone: &Zone,
                constant_array_builder: *mut ConstantArrayBuilder,
                source_position_mode: SourcePositionTableBuilder::RecordingMode,
            ) -> Self {
                BytecodeArrayWriter {
                    constant_array_builder,
                    source_position_mode,
                    _private: 0,
                }
            }

            fn write(&mut self, _node: &BytecodeNode) {}

            fn write_jump(&mut self, _node: &BytecodeNode, _label: &BytecodeLabel) {}

            fn write_jump_loop(
                &mut self,
                _node: &BytecodeNode,
                _loop_header: &BytecodeLoopHeader,
            ) {
            }

            fn write_switch(&mut self, _node: &BytecodeNode, _jump_table: &BytecodeJumpTable) {}

            fn bind_label(&mut self, _label: &BytecodeLabel) {}

            fn bind_loop_header(&mut self, _loop_header: &BytecodeLoopHeader) {}

            fn bind_jump_table_entry(&mut self, _jump_table: &BytecodeJumpTable, _case_value: i32) {}

            fn bind_handler_target(
                &mut self,
                _handler_table_builder: *mut HandlerTableBuilder,
                _handler_id: i32,
            ) {
            }

            fn bind_try_region_start(
                &mut self,
                _handler_table_builder: *mut HandlerTableBuilder,
                _handler_id: i32,
            ) {
            }

            fn bind_try_region_end(
                &mut self,
                _handler_table_builder: *mut HandlerTableBuilder,
                _handler_id: i32,
            ) {
            }

            fn to_bytecode_array(
                &mut self,
                _isolate: &Isolate,
                _register_count: i32,
                _parameter_count: i32,
                _max_arguments: i32,
                _handler_table: DirectHandle<TrustedByteArray>,
            ) -> Rc<BytecodeArray> {
                Rc::new(BytecodeArray::new(0))
            }

            fn to_bytecode_array(
                &mut self,
                _isolate: &LocalIsolate,
                _register_count: i32,
                _parameter_count: i32,
                _max_arguments: i32,
                _handler_table: DirectHandle<TrustedByteArray>,
            ) -> Rc<BytecodeArray> {
                Rc::new(BytecodeArray::new(0))
            }

            fn to_source_position_table(_isolate: &Isolate) -> DirectHandle<TrustedByteArray> {
                DirectHandle(PhantomData)
            }

            fn to_source_position_table(_isolate: &LocalIsolate) -> DirectHandle<TrustedByteArray> {
                DirectHandle(PhantomData)
            }

            fn set_function_entry_source_position(&mut self, _position: i32) {}

            #[cfg(debug_assertions)]
            fn check_bytecode_matches(&self, _bytecode: &BytecodeArray) -> i32 {
                0
            }
        }

        pub struct Zone {}

        impl Zone {
            pub fn new() -> Self {
                Zone {}
            }

            pub fn new_bytecode_register_optimizer(
                &self,
                _register_allocator: &RegisterAllocator,
                _fixed_register_count: i32,
                _parameter_count: i32,
                _writer: &RegisterTransferWriter,
            ) -> BytecodeRegisterOptimizer {
                BytecodeRegisterOptimizer { _private: 0 }
            }

            pub fn new_bytecode_jump_table(
                &self,
                _constant_pool_index: usize,
                _size: i32,
                _case_value_base: i32,
            ) -> BytecodeJumpTable {
                BytecodeJumpTable {
                    constant_pool_index: 0,
                    size: 0,
                    case_value_base: 0,
                    _private: 0,
                }
            }
        }

        lazy_static::lazy_static! {
            pub static ref FLAGS: Flags = Flags {
                ignition_reo: false,
                ignition_filter_expression_positions: false,
                script_context_mutable_heap_number: false,
                const_tracking_let: false
            };
        }

        impl BytecodeArrayBuilder {
            pub fn new(
                zone: &Zone,
                parameter_count: i32,
                locals_count: i32,
                feedback_vector_spec: &FeedbackVectorSpec,
                source_position_mode: SourcePositionTableBuilder::RecordingMode,
            ) -> Self {
                let register_allocator = RegisterAllocator::new(Self::fixed_register_count());
                let constant_array_builder = ConstantArrayBuilder::new(zone);
                let mut bytecode_array_writer = BytecodeArrayWriter::new(
                    zone,
                    &mut constant_array_builder as *mut ConstantArrayBuilder,
                    source_position_mode,
                );

                let handler_table_builder = HandlerTableBuilder::new(zone);
                let register_optimizer = if FLAGS.ignition_reo {
                    // SAFETY: 'static lifetime is acceptable in this case as flags is
                    // lazily initialised once.
                    let writer =
                        RegisterTransferWriter::new(unsafe { std::mem::transmute(0usize) }); // FIXME: This is a dummy
                    Some(BytecodeRegisterOptimizer::new(
                        zone,
                        &register_allocator,
                        Self::fixed_register_count(),
                        parameter_count,
                        &writer,
                    ))
                } else {
                    None
                };

                BytecodeArrayBuilder {
                    zone,
                    feedback_vector_spec,
                    bytecode_generated: false,
                    constant_array_builder,
                    handler_table_builder,
                    parameter_count,
                    max_arguments: 0,
                    local_register_count: locals_count,
                    register_allocator,
                    bytecode_array_writer,
                    register_optimizer,
                    latest_source_info_: BytecodeSourceInfo::new(),
                    deferred_source_info_: BytecodeSourceInfo::new(),
