// Converted from V8 C++ source files:
// Header: linkage.h
// Implementation: linkage.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub use std::ops::BitOr;
pub use std::ops::BitAnd;
pub use std::fmt;

    pub struct Flags<T> {
        flags: u32,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Flags<T> {
        pub fn new(flags: u32) -> Self {
            Flags {
                flags,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn empty() -> Self {
            Flags {
                flags: 0,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn contains(&self, other: Self) -> bool {
            (self.flags & other.flags) == other.flags
        }

        pub fn insert(&mut self, other: Self) {
            self.flags |= other.flags;
        }

        pub fn remove(&mut self, other: Self) {
            self.flags &= !other.flags;
        }

        pub fn bits(&self) -> u32 {
            self.flags
        }
    }

    impl<T> BitOr for Flags<T> {
        type Output = Self;

        fn bitor(self, other: Self) -> Self {
            Flags::new(self.flags | other.flags)
        }
    }

    impl<T> BitAnd for Flags<T> {
        type Output = Self;

        fn bitand(self, other: Self) -> Self {
            Flags::new(self.flags & other.flags)
        }
    }

    impl<T: fmt::Debug> fmt::Debug for Flags<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Flags")
                .field("flags", &self.flags)
                .finish()
        }
    }
}

pub mod codegen {
    pub struct LinkageLocation {
        location: i32,
        type_: MachineType,
        kind: LinkageLocationKind,
        size: i32
    }

    #[derive(PartialEq, Eq)]
    enum LinkageLocationKind {
        None,
        Register,
        CallerFrameSlot,
        CalleeFrameSlot,
        AnyRegister
    }

    impl LinkageLocation {
        pub fn ForRegister(code: i32, type_: MachineType) -> Self {
            LinkageLocation {
                location: code,
                type_,
                kind: LinkageLocationKind::Register,
                size: 1
            }
        }

         pub fn ForCallerFrameSlot(slot: i32, type_: MachineType) -> Self {
            LinkageLocation {
                location: slot,
                type_,
                kind: LinkageLocationKind::CallerFrameSlot,
                size: 1
            }
        }

        pub fn ForCalleeFrameSlot(slot: i32, type_: MachineType) -> Self {
            LinkageLocation {
                location: slot,
                type_,
                kind: LinkageLocationKind::CalleeFrameSlot,
                size: 1
            }
        }

        pub fn ForAnyRegister(type_: MachineType) -> Self {
             LinkageLocation {
                location: 0,
                type_,
                kind: LinkageLocationKind::AnyRegister,
                size: 1
            }
        }
       pub fn ForSavedCallerFunction() -> Self {
            LinkageLocation {
                location: 0,
                type_: MachineType::AnyTagged(),
                kind: LinkageLocationKind::AnyRegister,
                size: 1
            }
        }
      

        pub fn GetLocation(&self) -> i32 {
            self.location
        }

        pub fn GetType(&self) -> MachineType {
            self.type_
        }

        pub fn GetSizeInPointers(&self) -> i32 {
            self.size
        }

        pub fn IsRegister(&self) -> bool {
            self.kind == LinkageLocationKind::Register
        }
          pub fn IsAnyRegister(&self) -> bool {
            self.kind == LinkageLocationKind::AnyRegister
        }

        pub fn AsRegister(&self) -> i32 {
            if self.IsRegister() {
                self.location
            } else {
                panic!("Not a register location")
            }
        }

        pub fn IsCallerFrameSlot(&self) -> bool {
            self.kind == LinkageLocationKind::CallerFrameSlot
        }

        pub fn AsCallerFrameSlot(&self) -> i32 {
            if self.IsCallerFrameSlot() {
                self.location
            } else {
                panic!("Not a caller frame slot")
            }
        }

        pub fn IsSameLocation(a: &LinkageLocation, b: &LinkageLocation) -> bool {
            a.location == b.location && a.type_ == b.type_ && a.kind == b.kind
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum MachineRepresentation {
         ওয়ার্ড8,
        ওয়ার্ড16,
        ওয়ার্ড32,
        ওয়ার্ড64,
        ফ্লোট32,
        ফ্লোট64,
        বিট,
        পয়েন্টার,
        স্ট্রাকচার,
        কোড,
        ট্যাগড
    }
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct MachineType {
        representation: MachineRepresentation,
    }

    impl MachineType {
        pub fn AnyTagged() -> Self {
            MachineType {
                representation: MachineRepresentation::ট্যাগড,
            }
        }
        pub fn Pointer() -> Self {
             MachineType {
                representation: MachineRepresentation::পয়েন্টার,
            }
        }

        pub fn Int32() -> Self {
             MachineType {
                representation: MachineRepresentation::ওয়ার্ড32,
            }
        }

        pub fn Int64() -> Self {
            MachineType {
                representation: MachineRepresentation::ওয়ার্ড64,
            }
        }

        pub fn Float64() -> Self {
             MachineType {
                representation: MachineRepresentation::ফ্লোট64,
            }
        }

        pub fn representation(&self) -> MachineRepresentation {
            self.representation
        }

        pub fn IsTagged(&self) -> bool {
            self.representation == MachineRepresentation::ট্যাগড
        }
    }
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum StackArgumentOrder {
        kDefault,
        kJS
    }
}

pub mod common {
    pub const kInvalidWasmSignatureHash: u64 = 0;
}

pub mod compiler {
    use std::fmt;
    use std::ops::{BitAnd, BitOr};
    use crate::codegen::{LinkageLocation, MachineType, StackArgumentOrder};
    use crate::base::Flags;
    use crate::common::kInvalidWasmSignatureHash;

    pub struct ZoneObject {}

    pub struct CallInterfaceDescriptor {}

    pub struct OptimizedCompilationInfo {}

    pub struct MachineSignature {
    return_count: usize,
    parameter_count: usize,
    types: Vec<MachineType>
    }

    impl MachineSignature{
        pub fn new(return_count: usize, parameter_count: usize, types: Vec<MachineType>) -> Self{
            MachineSignature{
                return_count,
                parameter_count,
                types
            }
        }
    }

    pub enum CodeEntrypointTag {
        kDefaultCodeEntrypointTag,
        kJSEntrypointTag,
        kBytecodeHandlerEntrypointTag
    }

    impl CodeEntrypointTag {
        pub fn shifted_tag(&self) -> u32 {
            match self {
                CodeEntrypointTag::kDefaultCodeEntrypointTag => 0,
                CodeEntrypointTag::kJSEntrypointTag => 0,
                CodeEntrypointTag::kBytecodeHandlerEntrypointTag => 0,
            }
        }
    }

    pub const kDefaultCodeEntrypointTag: CodeEntrypointTag = CodeEntrypointTag::kDefaultCodeEntrypointTag;
    pub const kJSEntrypointTag: CodeEntrypointTag = CodeEntrypointTag::kJSEntrypointTag;
    pub const kBytecodeHandlerEntrypointTag: CodeEntrypointTag = CodeEntrypointTag::kBytecodeHandlerEntrypointTag;
    pub const kCodeEntrypointTagShift: i32 = 32;

    pub enum OperatorProperties {
        kNoProperties,
        kNoThrow
    }
    pub const Operator::kNoThrow: OperatorProperties = OperatorProperties::kNoThrow;
    pub mod Operator{
        pub const kNoProperties: super::OperatorProperties = super::OperatorProperties::kNoProperties;
    }

    pub enum LazyDeoptOnThrow {
        kNo,
        kYes
    }

    pub enum StubCallMode {
        kCallCodeObject,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct RegList{
         bits: u64
    }

    impl RegList{
        pub fn is_empty(&self) -> bool{
            self.bits == 0
        }
    }
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct DoubleRegList{
        bits: u64
    }

    pub const kNoCalleeSaved: RegList = RegList { bits: 0 };
    pub const kNoCalleeSavedFp: DoubleRegList = DoubleRegList { bits: 0 };

    pub struct CallDescriptor {
        kind_: Kind,
        tag_: CodeEntrypointTag,
        target_type_: MachineType,
        target_loc_: LinkageLocation,
        location_sig_: Box<LocationSignature>,
        param_slot_count_: usize,
        return_slot_count_: usize,
        properties_: OperatorProperties,
        callee_saved_registers_: RegList,
        callee_saved_fp_registers_: DoubleRegList,
        allocatable_registers_: RegList,
        flags_: Flags<Flag>,
        stack_order_: StackArgumentOrder,
        debug_name_: String,
        signature_hash_: u64,
        gp_param_count_: std::cell::RefCell<Option<usize>>,
        fp_param_count_: std::cell::RefCell<Option<usize>>
    }

    impl fmt::Display for CallDescriptor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}:{}:r{}s{}i{}f{}", self.kind_, self.debug_name_, self.ReturnCount(), self.ParameterSlotCount(), self.InputCount(), self.FrameStateCount())
        }
    }

    impl CallDescriptor {
        pub const kFlagsBitsEncodedInInstructionCode: i32 = 10;

        pub fn ToEncodedCSignature(&self) -> EncodedCSignature{
            EncodedCSignature{
                parameter_types: vec![],
                return_type: None
            }
        }

        pub fn GetMachineSignature(&self, zone: &Zone) -> MachineSignature{
            let mut types:Vec<MachineType> = Vec::new();
            for i in 0..self.ReturnCount(){
                types.push(self.GetReturnType(i));
            }
            for i in 0..self.ParameterCount(){
                types.push(self.GetParameterType(i));
            }
            MachineSignature::new(self.ReturnCount(),self.ParameterCount(),types)
        }

        pub fn CalleeSavedRegisters(&self) -> RegList {
            self.callee_saved_registers_
        }

        pub fn flags(&self) -> Flags<Flag> {
            self.flags_.clone()
        }
         pub fn RequiresFrameAsIncoming(&self) -> bool {
            if self.IsCFunctionCall() || self.IsJSFunctionCall() {
                return true;
            }
            if self.CalleeSavedRegisters() != kNoCalleeSaved {
                return true;
            }
            return false;
        }
        pub fn IsCFunctionCall(&self) -> bool {
            self.kind_ == Kind::kCallAddress
        }
        pub fn IsJSFunctionCall(&self) -> bool {
            self.kind_ == Kind::kCallJSFunction
        }
        pub fn RequiresEntrypointTagForCall(&self) -> bool {
            self.IsCodeObjectCall()
        }
         pub fn GetStackArgumentOrder(&self) -> StackArgumentOrder {
            self.stack_order_
        }
        pub fn AllocatableRegisters(&self) -> RegList {
            self.allocatable_registers_
        }
       pub fn IsCodeObjectCall(&self) -> bool {
            self.kind_ == Kind::kCallCodeObject
        }
        pub fn GetStackParameterDelta(
            &self,
            tail_caller: &CallDescriptor,
        ) -> i32 {
            if self.IsTailCallForTierUp() {
                return 0;
            }
            let callee_slots_above_sp = self.AddArgumentPaddingSlots(self.GetOffsetToReturns());
            let tail_caller_slots_above_sp = tail_caller.AddArgumentPaddingSlots(tail_caller.GetOffsetToReturns());
            let stack_param_delta = callee_slots_above_sp - tail_caller_slots_above_sp;
            if !self.ShouldPadArguments(stack_param_delta) {
                stack_param_delta
            } else {
                 stack_param_delta
            }
        }

        fn ShouldPadArguments(&self, stack_param_delta: i32) -> bool {
            stack_param_delta != 0
        }
        fn AddArgumentPaddingSlots(&self, offset_to_returns: i32) -> i32 {
            offset_to_returns
        }
         pub fn IsTailCallForTierUp(&self) -> bool {
            self.flags().contains(Flag::kIsTailCallForTierUp)
        }

        pub fn ParameterCount(&self) -> usize {
            self.location_sig_.parameter_count()
        }
          pub fn ParameterSlotCount(&self) -> usize {
            self.param_slot_count_
        }
        pub fn GPParameterCount(&self) -> usize {
            if self.gp_param_count_.borrow().is_none() {
                self.ComputeParamCounts();
            }
            self.gp_param_count_.borrow().unwrap()
        }

        pub fn FPParameterCount(&self) -> usize {
            if self.fp_param_count_.borrow().is_none() {
                self.ComputeParamCounts();
            }
            self.fp_param_count_.borrow().unwrap()
        }

        pub fn GetParameterType(&self, index: usize) -> MachineType {
            self.location_sig_.GetParam(index).GetType()
        }

         pub fn GetInputType(&self, index: usize) -> MachineType {
            if index == 0 {
                self.target_type_
            } else {
                self.location_sig_.GetParam(index - 1).GetType()
            }
        }

         pub fn ReturnCount(&self) -> usize {
            self.location_sig_.return_count()
        }

        pub fn GetReturnType(&self, index: usize) -> MachineType {
            self.location_sig_.GetReturn(index).GetType()
        }

         pub fn InputCount(&self) -> usize {
            1 + self.location_sig_.parameter_count()
        }

         pub fn GetInputLocation(&self, index: usize) -> LinkageLocation {
            if index == 0 {
                self.target_loc_
            } else {
                self.location_sig_.GetParam(index - 1)
            }
        }
          pub fn GetReturnLocation(&self, index: usize) -> LinkageLocation {
            self.location_sig_.GetReturn(index)
        }

        pub fn FrameStateCount(&self) -> usize {
            if self.NeedsFrameState() {
                1
            } else {
                0
            }
        }
          pub fn NeedsFrameState(&self) -> bool {
            self.flags().contains(Flag::kNeedsFrameState)
        }
           pub fn GetOffsetToReturns(&self) -> i32 {
                let mut offset = 0;
                for i in 0..self.ReturnCount(){
                    let operand = self.GetReturnLocation(i);
                    if !operand.IsRegister() {
                       if !operand.IsCallerFrameSlot() {
                         continue;
                        }
                        let slot_offset = -operand.GetLocation();
                        offset = std::cmp::min(offset,slot_offset);
                    }
                }
                if offset != 0{
                    return offset -1
                }
                 let last_argument_slot = self.GetOffsetToFirstUnusedStackSlot() - 1;
                offset = self.AddArgumentPaddingSlots(last_argument_slot);
                 return offset;
            }
            pub fn GetOffsetToFirstUnusedStackSlot(&self) -> i32 {
                let mut offset = 1;
                 for i in 0..self.InputCount() {
                    let operand = self.GetInputLocation(i);
                    if !operand.IsRegister() {
                        if !operand.IsCallerFrameSlot(){
                            continue
                        }
                         let slot_offset = -operand.GetLocation();
                         offset = std::cmp::max(offset, slot_offset + operand.GetSizeInPointers());
                    }
                }
                return offset;
            }
        pub fn IsDirectWasmFunctionCall(&self) -> bool {
            self.kind_ == Kind::kCallWasmFunction
        }
         pub fn IsIndirectWasmFunctionCall(&self) -> bool {
            self.kind_ == Kind::kCallWasmFunctionIndirect
        }
         pub fn signature_hash(&self) -> u64 {
            self.signature_hash_
        }
         pub fn NeedsCallerSavedRegisters(&self) -> bool {
            self.flags().contains(Flag::kCallerSavedRegisters)
        }
          fn CanTailCall(&self, callee: &CallDescriptor) -> bool {
            if self.ReturnCount() != callee.ReturnCount() {
                return false;
            }
            let stack_returns_delta = self.GetOffsetToReturns() - callee.GetOffsetToReturns();
            for i in 0..self.ReturnCount() {
                if self.GetReturnLocation(i).IsCallerFrameSlot() &&
                    callee.GetReturnLocation(i).IsCallerFrameSlot() {
                    if self.GetReturnLocation(i).AsCallerFrameSlot() + stack_returns_delta !=
                        callee.GetReturnLocation(i).AsCallerFrameSlot() {
                        return false;
                    }
                } else if !LinkageLocation::IsSameLocation(&self.GetReturnLocation(i), &callee.GetReturnLocation(i)) {
                    return false;
                }
            }
            true
        }

        fn CalculateFixedFrameSize(&self, code_kind: CodeKind) -> i32 {
            match self.kind_ {
                Kind::kCallJSFunction => 10,
                 Kind::kCallAddress => 2,
                Kind::kCallCodeObject => 3,
                Kind::kCallBuiltinPointer => 3,
                _ => 0
            }
        }

        fn HasRestrictedAllocatableRegisters(&self) -> bool {
            !self.allocatable_registers_.is_empty()
        }

        fn ComputeParamCounts(&self) {
             *self.gp_param_count_.borrow_mut() = Some(0);
            *self.fp_param_count_.borrow_mut() = Some(0);
            for i in 0..self.ParameterCount() {
                if self.IsFloatingPoint(self.GetParameterType(i).representation()) {
                     *self.fp_param_count_.borrow_mut() = Some(self.fp_param_count_.borrow().unwrap() + 1);
                } else {
                    *self.gp_param_count_.borrow_mut() = Some(self.gp_param_count_.borrow().unwrap() + 1);
                }
            }
        }
        fn IsFloatingPoint(&self, representation: MachineRepresentation) -> bool {
            representation == MachineRepresentation::ফ্লোট32 || representation == MachineRepresentation::ফ্লোট64
        }

        pub fn new(
            kind_: Kind,
            tag_: CodeEntrypointTag,
            target_type_: MachineType,
            target_loc_: LinkageLocation,
            location_sig_: LocationSignature,
            param_slot_count_: usize,
            return_slot_count_: usize,
            properties_: OperatorProperties,
            callee_saved_registers_: RegList,
            callee_saved_fp_registers_: DoubleRegList,
            allocatable_registers_: RegList,
            flags_: Flags<Flag>,
            stack_order_: StackArgumentOrder,
            debug_name_: String,
            signature_hash_: u64,
        ) -> Self {
            CallDescriptor {
                kind_: kind_,
                tag_: tag_,
                target_type_: target_type_,
                target_loc_: target_loc_,
                location_sig_: Box::new(location_sig_),
                param_slot_count_: param_slot_count_,
                return_slot_count_: return_slot_count_,
                properties_: properties_,
                callee_saved_registers_: callee_saved_registers_,
                callee_saved_fp_registers_: callee_saved_fp_registers_,
                allocatable_registers_: allocatable_registers_,
                flags_: flags_,
                stack_order_: stack_order_,
                debug_name_: debug_name_,
                signature_hash_: signature_hash_,
                 gp_param_count_: std::cell::RefCell::new(None),
                fp_param_count_: std::cell::RefCell::new(None)
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Kind {
        kCallCodeObject,
        kCallJSFunction,
        kCallAddress,
        kCallWasmCapiFunction,
        kCallWasmFunction,
        kCallWasmFunctionIndirect,
        kCallWasmImportWrapper,
        kCallBuiltinPointer,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    pub enum Flag {
        kNoFlags,
        kNeedsFrameState,
        kHasExceptionHandler,
        kCanUseRoots,
        kInitializeRootRegister,
        kNoAllocate,
        kFixedTargetRegister,
        kCallerSavedRegisters,
        kCallerSavedFPRegisters,
        kIsTailCallForTierUp,
        kNoFunctionDescriptor,
    }

    impl From<Flag> for u32 {
        fn from(flag: Flag) -> Self {
            match flag {
                Flag::kNoFlags => 0,
                Flag::kNeedsFrameState => 1 << 0,
                Flag::kHasExceptionHandler => 1 << 1,
                Flag::kCanUseRoots => 1 << 2,
                Flag::kInitializeRootRegister => 1 << 3,
                Flag::kNoAllocate => 1 << 4,
                Flag::kFixedTargetRegister => 1 << 5,
                Flag::kCallerSavedRegisters => 1 << 6,
                Flag::kCallerSavedFPRegisters => 1 << 7,
                Flag::kIsTailCallForTierUp => 1 << 8,
                Flag::kNoFunctionDescriptor => 1 << 9,
            }
        }
    }
    impl From<u32> for Flag {
        fn from(bits: u32) -> Self {
            match bits {
                0 => Flag::kNoFlags,
                1 => Flag::kNeedsFrameState,
                2 => Flag::kHasExceptionHandler,
                4 => Flag::kCanUseRoots,
                8 => Flag::kInitializeRootRegister,
                16 => Flag::kNoAllocate,
                32 => Flag::kFixedTargetRegister,
                64 => Flag::kCallerSavedRegisters,
                128 => Flag::kCallerSavedFPRegisters,
                256 => Flag::kIsTailCallForTierUp,
                512 => Flag::kNoFunctionDescriptor,
                _ => Flag::kNoFlags,
            }
        }
    }

    pub type FlagsType = Flags<Flag>;

    impl Kind {
        pub fn to_str(&self) -> &str {
            match self {
                Kind::kCallCodeObject => "Code",
                Kind::kCallJSFunction => "JS",
                Kind::kCallAddress => "Addr",
                Kind::kCallWasmCapiFunction => "WasmExit",
                Kind::kCallWasmFunction => "WasmFunction",
                Kind::kCallWasmFunctionIndirect => "WasmFunctionIndirect",
                Kind::kCallWasmImportWrapper => "WasmImportWrapper",
                Kind::kCallBuiltinPointer => "BuiltinPointer",
            }
        }
    }

    impl fmt::Debug for Kind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_str())
        }
    }

    pub struct Linkage {
        incoming_: Box<CallDescriptor>,
    }

    impl Linkage {
        pub fn new(incoming: CallDescriptor) -> Self {
            Linkage {
                incoming_: Box::new(incoming),
            }
        }

        pub fn GetIncomingDescriptor(&self) -> &CallDescriptor {
            &self.incoming_
        }

        pub fn GetParameterLocation(&self, index: i32) -> LinkageLocation {
            self.incoming_.GetInputLocation((index + 1) as usize)
        }

        pub fn GetParameterType(&self, index: i32) -> MachineType {
            self.incoming_.GetInputType((index + 1) as usize)
        }
         pub fn GetReturnLocation(&self, index: usize) -> LinkageLocation {
            self.incoming_.GetReturnLocation(index)
        }

        pub fn GetReturnType(&self, index: usize) -> MachineType {
            self.incoming_.GetReturnType(index)
        }

        pub fn NeedsFrameStateInput(function: RuntimeFunctionId) -> bool {
        match function {
            RuntimeFunctionId::kAbort => false,
             RuntimeFunctionId::kAllocateInOldGeneration => false,
            RuntimeFunctionId::kCreateIterResultObject => false,
            RuntimeFunctionId::kGrowableSharedArrayBufferByteLength => false,
            RuntimeFunctionId::kIncBlockCounter => false,
            RuntimeFunctionId::kNewClosure => false,
            RuntimeFunctionId::kNewClosure_Tenured => false,
            RuntimeFunctionId::kNewFunctionContext => false,
            RuntimeFunctionId::kPushBlockContext => false,
            RuntimeFunctionId::kPushCatchContext => false,
            RuntimeFunctionId::kStringEqual => false,
            RuntimeFunctionId::kStringLessThan => false,
            RuntimeFunctionId::kStringLessThanOrEqual => false,
            RuntimeFunctionId::kStringGreaterThan => false,
            RuntimeFunctionId::kStringGreaterThanOrEqual => false,
             RuntimeFunctionId::kToFastProperties => false,
             RuntimeFunctionId::kTraceEnter => false,
            RuntimeFunctionId::kTraceExit => false,
            RuntimeFunctionId::kInlineCreateIterResultObject => false,
            RuntimeFunctionId::kInlineIncBlockCounter => false,
            RuntimeFunctionId::kInlineGeneratorClose => false,
            RuntimeFunctionId::kInlineGeneratorGetResumeMode => false,
            RuntimeFunctionId::kInlineCreateJSGeneratorObject => false,

            _ => true,
        }
    }
        pub fn GetCEntryStubCallDescriptor(
            zone: &Zone,
            return_count: i32,
            js_parameter_count: i32,
            debug_name: &str,
            properties: OperatorProperties,
            flags: Flags<Flag>,
            stack_order: StackArgumentOrder,
        ) -> CallDescriptor {
        let function_count = 1;
        let num_args_count = 1;
        let context_count = 1;
        let parameter_count = function_count + js_parameter_count + num_args_count + context_count;

        let mut locations_builder = LocationSignatureBuilder::new(return_count as usize, parameter_count as usize);

        if locations_builder.return_count_ > 0 {
            locations_builder.AddReturn(LinkageLocation::ForRegister(0, MachineType::AnyTagged()));
        }
        if locations_builder.return_count_ > 1 {
            locations_builder.AddReturn(LinkageLocation::ForRegister(1, MachineType::AnyTagged()));
        }
        if locations_builder.return_count_ > 2 {
            locations_builder.AddReturn(LinkageLocation::ForRegister(2, MachineType::AnyTagged()));
        }
        for i in 0..js_parameter_count {
            locations_builder.AddParam(LinkageLocation::ForCallerFrameSlot(i - js_parameter_count, MachineType::AnyTagged()));
        }
        locations_builder.AddParam(LinkageLocation::ForRegister(3, MachineType::Pointer()));
        locations_builder.AddParam(LinkageLocation::ForRegister(4, MachineType::Int32()));
        locations_builder.AddParam(LinkageLocation::ForRegister(5, MachineType::AnyTagged()));

         let target_type = MachineType::AnyTagged();
        let target_loc = LinkageLocation::ForAnyRegister(MachineType::AnyTagged());

        CallDescriptor::new(
            Kind::kCallCodeObject,
            kDefaultCodeEntrypointTag,
            target_type,
            target_loc,
            locations_builder.Build(),
            js_parameter_count as usize,
             return_count as usize,
            properties,
            kNoCalleeSaved,
            kNoCalleeSavedFp,
            flags,
            debug_name.to_string(),
            stack_order,
         )
    }

        pub fn GetJSCallDescriptor(
            zone: &Zone,
            is_osr: bool,
            js_parameter_count: i32,
            flags: Flags<Flag>,
            properties: OperatorProperties,
        ) -> CallDescriptor {
            let return_count = 1;
            let context_count = 1;
            let new_target_count = 1;
            let num_args_count = 1;
            let dispatch_handle_count = 0;
            let parameter_count = js_parameter_count + new_target_count + num_args_count + dispatch_handle_count + context_count;

            let mut locations_builder = LocationSignatureBuilder::new(return_count as usize, parameter_count as usize);

            locations_builder.AddReturn(LinkageLocation::ForRegister(0, MachineType::AnyTagged()));

            for i in 0..js_parameter_count {
                let spill_slot_index = -i - 1;
                locations_builder.AddParam(LinkageLocation::ForCallerFrameSlot(spill_slot_index, MachineType::AnyTagged()));
            }

            locations_builder.AddParam(LinkageLocation::ForRegister(1, MachineType::AnyTagged()));
            locations_builder.AddParam(LinkageLocation::ForRegister(2, MachineType::Int32()));
            locations_builder.AddParam(LinkageLocation::ForRegister(5, MachineType::AnyTagged()));

            let target_type = MachineType::AnyTagged();
            let target_loc = if is_osr {
                LinkageLocation::ForSavedCallerFunction()
            } else {
                LinkageLocation::ForRegister(6, target_type)
            };
            let descriptor_kind = Kind::kCallJSFunction;

             CallDescriptor::new(
                descriptor_kind,
                kJSEntrypointTag,
                target_type,
                target_loc,
                locations_builder.Build(),
                js_parameter_count as usize,
                 return_count as usize,
                properties,
                kNoCalleeSaved,
                kNoCalleeSavedFp,
                flags,
                "js-call".to_string(),
                0,
            )
        }
    }
     #[derive(Clone)]
    pub struct LocationSignature {
        return_count: usize,
        parameter_count: usize,
        returns: Vec<LinkageLocation>,
        params: Vec<LinkageLocation>,
    }

    impl LocationSignature {
        pub fn new(
            return_count: usize,
            parameter_count: usize,
            returns: Vec<LinkageLocation>,
            params: Vec<LinkageLocation>,
        ) -> Self {
            LocationSignature {
                return_count,
                parameter_count,
                returns,
                params,
            }
        }

        pub fn return_count(&self) -> usize {
            self.return_count
        }

        pub fn parameter_count(&self) -> usize {
            self.parameter_count
        }

        pub fn GetReturn(&self, index: usize) -> LinkageLocation {
            self.returns[index].clone()
        }

        pub fn GetParam(&self, index: usize) -> LinkageLocation {
            self.params[index].clone()
        }
    }
    struct LocationSignatureBuilder {
        return_count_: usize,
        parameter_count_: usize,
        returns_: Vec<LinkageLocation>,
        params_: Vec<LinkageLocation>,
    }

    impl LocationSignatureBuilder {
         fn new(return_count: usize, parameter_count: usize) -> Self {
            LocationSignatureBuilder {
                return_count_: return_count,
                parameter_count_: parameter_count,
                returns_: Vec::new(),
                params_: Vec::new(),
            }
        }
        fn AddReturn(&mut self, location: LinkageLocation) {
            self.returns_.push(location);
        }

        fn AddParam(&mut self, location: LinkageLocation) {
            self.params_.push(location);
        }
        fn Build(&self) -> LocationSignature {
             LocationSignature {
                return_count: self.return_count_,
                parameter_count: self.parameter_count_,
                returns: self.returns_.clone(),
                params: self.params_.clone(),
            }
        }
    }

}

pub mod execution {
    pub enum RuntimeFunctionId {
        kAbort,
         kAllocateInOldGeneration,
        kCreateIterResultObject,
        kGrowableSharedArrayBufferByteLength,
        kIncBlockCounter,
        kNew
