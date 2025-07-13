// Converted from V8 C++ source files:
// Header: N/A
// Implementation: instruction-selector-ia32.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::any::Any;
use std::mem;
//use std::collections::HashMap;

use crate::base::{bits::WraparoundAdd32, Flags, Iterator, Vector};
use crate::codegen::ia32::{
    assembler_ia32::Assembler, register_ia32::Register, AddressingMode,
};
use crate::codegen::machine_type::MachineType;
use crate::codegen::macro_assembler_base::MacroAssemblerBase;
use crate::common::globals::FLAG_enable_unconditional_write_barriers;
use crate::compiler::backend::{
    instruction::{AddressingModeField, InstructionCode, InstructionOperand},
    instruction_codes::RecordWriteModeField,
    instruction_selector_adapter::TurboshaftAdapter,
    instruction_selector_impl::FlagsContinuationT,
    instruction_selector::InstructionSelector,
    write_barrier_kind::{WriteBarrierKind, WriteBarrierKindToRecordWriteMode},
};
use crate::compiler::{
    backend::instruction::Instruction, turboshaft, Frame, turboshaft::SwitchInfo,
};
use crate::compiler::{
    linkage::LinkageLocation, MachineOperatorBuilder, MachineRepresentation,
};
use crate::compiler::wasm_gc_operator_reducer::Wasm;
use crate::execution::isolate::Isolate;
use crate::flags::flags;
use crate::objects::heap_number::HeapNumber;
use crate::objects::smi::Smi;
use crate::roots::roots::RootIndex;
use crate::utils::utils::is_int32;
use crate::wasm::{simd_shuffle::{CanonicalizeShuffle, Pack4Lanes, PackBlend4, SwapShuffleInputs, SimdShuffle, Self}, simd_shuffle};

use super::code_generator_ia32::{CodeGenerator, DisplacementMode};
use self::turboshaft::{LoadOp, ConstantOp, WordBinopOp, ShiftOp, Operation, WordRepresentation, RegisterRepresentation, Float32Sqrt, Float64Sqrt, MemoryBarrierOp, Float64FromWord32PairOp, BitcastWord32PairToFloat64Op, Simd128ConstantOp, FlagsContinuation, ComparisonOp, RegisterUseKind, I8x16Shl, AtomicMemoryOrder, simdd128::MemoryRepresentation, Simd128ExtractLaneOp, StoreOp, ProtectedLoadOp, ExternalReference, OverflowCheckedBinopOp, ProjectionOp, StackPointerGreaterThanOp, StackSlotOp, LoadRootRegisterOp, Float64Ieee754Unop, Simd128ReplaceLaneOp, Simd128BinopOp, Float64ToUint32};

// Define architectural opcodes for IA32
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArchOpcode {
    kIA32Movl,
    kIA32Movb,
    kIA32Movw,
    kIA32Movsxbl,
    kIA32Movzxbl,
    kIA32Movsxwl,
    kIA32Movzxwl,
    kIA32Movss,
    kIA32Movsd,
    kIA32And,
    kIA32Or,
    kIA32Xor,
    kIA32Add,
    kIA32Sub,
    kIA32Cmp,
    kIA32Test,
    kIA32Cmp16,
    kIA32Test16,
    kIA32Cmp8,
    kIA32Test8,
    kIA32Shl,
    kIA32Shr,
    kIA32Sar,
    kIA32Imul,
    kIA32Float32Cmp,
    kIA32Float64Cmp,
    kArchStackSlot,
    kIA32Push,
    kArchPrepareCallCFunction,
    kIA32Poke,
    kArchAbortCSADcheck,
    kArchSetStackPointer,
    kIA32Udiv,
    kIA32Idiv,
    kIA32ImulHigh,
    kIA32UmulHigh,
    kIA32Idiv,
    kIA32Udiv,
    kIA32Lzcnt,
    kIA32Tzcnt,
    kIA32Popcnt,
    kIA32Float32Sqrt,
    kIA32Float64Sqrt,
    kSSEInt32ToFloat32,
    kSSEInt32ToFloat64,
    kIA32Float32ToInt32,
    kIA32Float64ToFloat32,
    kIA32BitcastFI,
    kIA32BitcastIF,
    kIA32Float64ExtractLowWord32,
    kIA32Float64ExtractHighWord32,
    kIA32Float64ToInt32,
    kIA32Float32ToFloat64,
    kIA32Uint32ToFloat64,
    kIA32Float64ToUint32,
    kIA32Sha1Update,
    kIA32Ldexp,
    kIA32Float32Round,
    kIA32Float64Round,
    kIA32Float64Mod,
    kIA32Float32Max,
    kIA32Float64Max,
    kIA32Float32Min,
    kIA32Float64Min,
	kIA32Lea,
	kIA32Neg,
	kIA32Call,
    kIA32EnterFrame,
    kIA32LeaveFrame,
    kIA32Testl,
    kIA32Testb,
    kIA32Movdqu,
    kArchStoreWithWriteBarrier,
	kArchTableSwitch,
	kArchBinarySearchSwitch,
	kIA32Peek,
	kIA32S128Zero,
	kIA32Not,
	kIA32Bswap,
	kSSE4_2,
	kAVX,

	kSSEF32x4Shuffle,
    kIA32F32x4Splat,
	kAVXF32x4Splat,

	kIA32Blendvps,
	kBlendvps,
	
	kIA32I8x16Shuffle,
    kIA32S16x8Blend,

    kIA32F64x2UnpackLow,
    kIA32S64x2UnpackLow,

	kIA32MulPair,
	kIA32ShlPair,
	kIA32ShrPair,
	kIA32SarPair,
	kSSES16x8UnzipLow,
    kAVXS16x8UnzipLow,
	kSSES16x8UnzipHigh,
    kAVXS16x8UnzipHigh,
	kSSES8x16UnzipLow,
    kAVXS8x16UnzipLow,
	kSSES8x16UnzipHigh,
    kAVXS8x16UnzipHigh,
	kAVXI64x2GtS,
    kIA32I64x2GtS,
    kAVXI64x2GeS,
    kIA32I64x2GeS,

	kSSEI32x4UConvertF32x4,
    kAVXI32x4UConvertF32x4,

	kS128AllOnes,
	kIA32I16x8Dup,
    kSSES8x16TransposeLow,
    kAVXS8x16TransposeLow,
    kSSES8x16TransposeHigh,
    kAVXS8x16TransposeHigh,

    kIA32Cvttpd2dq,
    kIA32Cvttps2dq,

    kIA32Word32ReleasePairStore,
	kIA32Word32SeqCstPairStore,
    kArchTruncateDoubleToI,
    kF64x2Sqrt,
	kIA32Sha256Compress,

    kIA32Float64FromWord32Pair,
    kAtomicCompareExchangeWord32,
	kAtomicCompareExchangeInt8,
	kAtomicCompareExchangeInt16,
	kAtomicCompareExchangeUint8,
	kAtomicCompareExchangeUint16,
    kAtomicExchangeWord32,
	kAtomicExchangeInt8,
	kAtomicExchangeInt16,
	kAtomicExchangeUint8,
	kAtomicExchangeUint16,
    kAtomicAddWord32,
    kAtomicAddInt8,
    kAtomicAddInt16,
    kAtomicAddUint8,
    kAtomicAddUint16,
    kAtomicSubWord32,
    kAtomicSubInt8,
    kAtomicSubInt16,
    kAtomicSubUint8,
    kAtomicSubUint16,
    kAtomicAndWord32,
    kAtomicAndInt8,
    kAtomicAndInt16,
    kAtomicAndUint8,
    kAtomicAndUint16,
    kAtomicOrWord32,
    kAtomicOrInt8,
    kAtomicOrInt16,
    kAtomicOrUint8,
    kAtomicOrUint16,
    kAtomicXorWord32,
    kAtomicXorInt8,
    kAtomicXorInt16,
    kAtomicXorUint8,
    kAtomicXorUint16,

    kAtomicExchangeWord32,
    kArchStackPointerGreaterThan,
    kStackPointerGreaterThanCondition,
    kArchNop,
    kIA32I8x16ShrU,

    kAtomicExchangeInt8,
    kAtomicExchangeInt16,
    kAtomicExchangeWord32,
    kArchWord32AtomicPairLoad,
    kIA32I8x16Dup,
    kSSES8x8Reverse,
    kAVXS8x8Reverse,

    kIA32I8x16Popcnt,
    kS8x16TransposeLow,
    kS8x16TransposeHigh,
	S64x2,
    kAVXS8x2Reverse,
    kSSES8x2Reverse,
    kAVXS8x4Reverse,
    kSSES8x4Reverse,
    kAtomicCompareExchangeInt16,
    kAVXI32x4ExtAddPairwiseI16x8S,
    kAtomicCompareExchangeUint8,
    kAtomicCompareExchangeUint16,
    kIA32Float64SilenceNaN,
    kS128Not,
    kS8x16UnpackLow,
    kIA32I8x16ExtractLaneS,
    kIA32F64x2ExtractLane,

    kIA32Pextrb,

    kIA32Pextrw,
    kAVXS16x8HalfShuffle1,
    kAVXS16x8HalfShuffle2,

    kSSES32x4UnpackLow,
    kAVXS32x4UnpackLow,

    kIA32Shl,
    kIA32MFence,

    kIA32I16x8ExtAddPairwiseI8x16U,
    kI32x4TruncF32x4S,
    kBlendvps,
    I8x16,
    kF32x4Qfma,
    kF32x4Qfms,
	kF64x2Qfma,
	kF64x2Qfms,
	S128,
	F16x8,
}
#[derive(Debug)]
struct LoadStoreView {
    base: OpIndex,
    index: OptionOpIndex,
    offset: i32,
}

impl LoadStoreView {
    fn new(op: &Operation) -> Self {
        if let Some(load) = op.try_cast::<LoadOp>() {
            LoadStoreView {
                base: load.base(),
                index: load.index(),
                offset: load.offset,
            }
        } else if let Some(store) = op.try_cast::<StoreOp>() {
            LoadStoreView {
                base: store.base(),
                index: store.index(),
                offset: store.offset,
            }
        } else {
            panic!("Operation must be LoadOp or StoreOp");
        }
    }
}

struct ScaledIndexMatch {
    base: OpIndex,
    index: OpIndex,
    scale: i32,
}

struct BaseWithScaledIndexAndDisplacementMatch {
    base: OpIndex,
    index: OpIndex,
    scale: i32,
    displacement: i32,
    displacement_mode: DisplacementMode,
}

impl BaseWithScaledIndexAndDisplacementMatch {
    fn new() -> Self {
        BaseWithScaledIndexAndDisplacementMatch {
            base: OpIndex { index: 0 },
            index: OpIndex { index: 0 },
            scale: 0,
            displacement: 0,
            displacement_mode: DisplacementMode::kPositiveDisplacement,
        }
    }
}

impl InstructionSelector {
	fn CanAddressRelativeToRootsRegister(&self, reference: ExternalReference) -> bool {
		true
	}

	fn RootRegisterOffsetForExternalReference(&self, isolate: &dyn Any, reference: ExternalReference) -> i64 {
		0
	}
}

impl<'a> IA32OperandGeneratorT<'a> {

	fn UseByteRegister(&self, node: OpIndex) -> InstructionOperand {
		InstructionOperand::UseFixed(node, Register {code: 1}) //edx
	}
}

struct IA32OperandGeneratorT<'a> {
    selector: &'a InstructionSelector,
	root: u32,
}

impl<'a> IA32OperandGeneratorT<'a> {
    fn new(selector: &'a InstructionSelector) -> Self {
        IA32OperandGeneratorT { selector, root: 0 }
    }

    fn DefineAsRegister(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand::DefineAsRegister(node)
    }

    fn UseRegister(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand::UseRegister(node)
    }
    fn UseUniqueRegister(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand::UseUniqueRegister(node)
    }

    fn UseFixed(&self, node: OpIndex, reg: Register) -> InstructionOperand {
        InstructionOperand::UseFixed(node, reg)
    }

    fn Use(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand::Use(node)
    }

    fn UseImmediate(&self, value: i32) -> InstructionOperand {
        InstructionOperand::UseImmediate(value)
    }
    
    fn TempImmediate(&self, value: i32) -> InstructionOperand {
        InstructionOperand::Immediate(value)
    }

    fn NoOutput(&self) -> InstructionOperand {
        InstructionOperand::None
    }
    
    fn Get(&self, node: OpIndex) -> &Operation {
		self.selector.Get(node)
	}

    fn load_view(&self, node: OpIndex) -> TurboshaftAdapter::LoadView<'_> {
		self.selector.load_view(node)
	}

    fn store_view(&self, node: OpIndex) -> TurboshaftAdapter::StoreView<'_> {
		self.selector.store_view(node)
	}
    
    fn MatchExternalConstant(&self, node: OpIndex, reference: &mut ExternalReference) -> bool{
		false
	}

	fn IsExternalConstant(&self, node: OpIndex) -> bool {
		false
	}
	
	fn IsLoadOrLoadImmutable(&self, node: OpIndex) -> bool {
		self.selector.IsLoadOrLoadImmutable(node)
	}

	fn MatchIntegralWord32Constant(&self, node: OpIndex, value: &mut i32) -> bool{
		let op = self.selector.Get(node);
		if let Some(c) = op.try_cast::<ConstantOp>() {
			if c.kind == ConstantOp::Kind::kWord32 {
				*value = c.word32() as i32;
				return true;
			}
		}
		false
	}
	fn TempRegister(&self) -> InstructionOperand {
        InstructionOperand::TempRegister()
    }

	fn UseUniqueRegisterOrSlotOrConstant(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand::UseUniqueRegisterOrSlotOrConstant(node)
    }

	fn TempSimd128Register(&self) -> InstructionOperand {
        InstructionOperand::TempSimd128Register()
    }
    
	fn value(&self, index: OpIndex) -> OpIndex {
		self.selector.value(index)
	}

    fn GenerateMemoryOperandInputs(
        &self,
        index: OptionalOpIndex,
        scale: i32,
        base: OpIndex,
        displacement: i32,
        displacement_mode: DisplacementMode,
        inputs: &mut [InstructionOperand],
        input_count: &mut usize,
        register_mode: RegisterMode,
    ) -> AddressingMode {
        let mut mode = AddressingMode::kMode_MRI;
        let mut displacement = displacement;
        if displacement_mode == DisplacementMode::kNegativeDisplacement {
            displacement = WraparoundAdd32(0, -displacement);
        }
        if base.index != 0 {
            if let Some(constant) = self.Get(base).try_cast::<ConstantOp>() {
                if constant.kind == ConstantOp::Kind::kWord32 {
                    displacement = WraparoundAdd32(displacement, constant.word32() as i32);
                } else if constant.kind == ConstantOp::Kind::kSmi {
                    if let Some(smi) = Smi::try_from(constant.smi()) {
                        displacement = WraparoundAdd32(displacement, smi.value() as i32);
                    }
                }
            }
        }
        if base.index != 0 {
            inputs[*input_count] = self.UseRegisterWithMode(base, register_mode);
            *input_count += 1;
            if let Some(index_op) = index.into_option() {
                inputs[*input_count] = self.UseRegisterWithMode(self.value(index_op), register_mode);
                *input_count += 1;
                if displacement != 0 {
                    inputs[*input_count] = self.TempImmediate(displacement);
                    *input_count += 1;
                    static const MRnI_MODES: [AddressingMode; 4] = [
                        AddressingMode::kMode_MR1I,
                        AddressingMode::kMode_MR2I,
                        AddressingMode::kMode_MR4I,
                        AddressingMode::kMode_MR8I,
                    ];
                    mode = MRnI_MODES[scale as usize];
                } else {
                    static const MRn_MODES: [AddressingMode; 4] = [
                        AddressingMode::kMode_MR1,
                        AddressingMode::kMode_MR2,
                        AddressingMode::kMode_MR4,
                        AddressingMode::kMode_MR8,
                    ];
                    mode = MRn_MODES[scale as usize];
                }
            } else {
                if displacement == 0 {
                    mode = AddressingMode::kMode_MR;
                } else {
                    inputs[*input_count] = self.TempImmediate(displacement);
                    *input_count += 1;
                    mode = AddressingMode::kMode_MRI;
                }
            }
        } else {
            if let Some(index_op) = index.into_option() {
                inputs[*input_count] = self.UseRegisterWithMode(self.value(index_op), register_mode);
                *input_count += 1;
                if displacement != 0 {
                    inputs[*input_count] = self.TempImmediate(displacement);
                    *input_count += 1;
                    static const MnI_MODES: [AddressingMode; 4] = [
                        AddressingMode::kMode_MRI,
                        AddressingMode::kMode_M2I,
                        AddressingMode::kMode_M4I,
                        AddressingMode::kMode_M8I,
                    ];
                    mode = MnI_MODES[scale as usize];
                } else {
                    static const Mn_MODES: [AddressingMode; 4] = [
                        AddressingMode::kMode_MR,
                        AddressingMode::kMode_M2,
                        AddressingMode::kMode_M4,
                        AddressingMode::kMode_M8,
                    ];
                    mode = Mn_MODES[scale as usize];
                }
            } else {
                inputs[*input_count] = self.TempImmediate(displacement);
                *input_count += 1;
                return AddressingMode::kMode_MI;
            }
        }
        mode
    }

    fn UseRegisterWithMode(&self, node: OpIndex, register_mode: RegisterMode) -> InstructionOperand {
        InstructionOperand::UseRegisterWithMode(node, register_mode)
    }

	fn GetEffectiveIndexOperand(&self, index: OpIndex, mode: &mut AddressingMode) -> InstructionOperand {
		if self.CanBeImmediate(index) {
			*mode = AddressingMode::kMode_MRI;
			return self.UseImmediate(index);
		} else {
			*mode = AddressingMode::kMode_MR1;
			return self.UseUniqueRegister(index);
		}
	}
    
	fn UseAny(&self, node: OpIndex) -> InstructionOperand {
		InstructionOperand::UseAny(node)
	}
	
    fn DefineSameAsFirst(&self, node: OpIndex) -> InstructionOperand {
		InstructionOperand::DefineSameAsFirst(node)
	}

	fn TempRegister(&self, reg: Register) -> InstructionOperand {
		InstructionOperand::TempRegisterWithHint(reg)
	}

	fn CanBeImmediate(&self, node: OpIndex) -> bool {
		if self.IsExternalConstant(node) {
			return true;
		}
		if let Some(constant) = self.Get(node).try_cast::<ConstantOp>() {
			match constant.kind {
				ConstantOp::Kind::kWord32 => true,
				ConstantOp::Kind::kRelocatableWasmCall => true,
				ConstantOp::Kind::kRelocatableWasmStubCall => true,
				ConstantOp::Kind::kSmi => true,
				ConstantOp::Kind::kNumber => {
					let number = constant.number();
					number.to_bits() == 0
				}
				_ => false,
			}
		} else {
			false
		}
	}
    
	fn IsCommutative(&self, node: OpIndex) -> bool {
		self.selector.IsCommutative(node)
	}

	fn GetEffectiveAddressMemoryOperand(&self, node: OpIndex, inputs: &mut [InstructionOperand], input_count: &mut usize, register_mode: RegisterMode) -> AddressingMode {
		let op = self.Get(node);

		if let Some(load) = op.try_cast::<LoadOp>() {

			let mut reference : ExternalReference = ExternalReference { index: 0, canonical_name: ""};
			if self.MatchExternalConstant(load.base(), &mut reference) && !load.index().is_some() {
				if self.selector.CanAddressRelativeToRootsRegister(reference) {
					//TODO
					
				}
			}
		}
		if let Some(m) = TryMatchBaseWithScaledIndexAndDisplacement(&self, node) {
			if m.base.index != 0 && self.Get(m.base).try_cast::<LoadRootRegisterOp>().is_some() {
				return AddressingMode::kMode_None;
			}
			if self.ValueFitsIntoImmediate(m.displacement) {
				self.GenerateMemoryOperandInputs(m.index, m.scale, m.base, m.displacement, m.displacement_mode, inputs, input_count, register_mode)
			} else {
				
				inputs[*input_count] = self.UseRegisterWithMode(m.base, register_mode);
				*input_count+=1;
				inputs[*input_count] = self.UseRegisterWithMode(m.index, register_mode);
				*input_count+=1;
				AddressingMode::kMode_MR1

			}
		} else {
			AddressingMode::kMode_None
		}
	}

	fn ValueFitsIntoImmediate(&self, value: i32) -> bool {
        value >= (i32::MIN + 1) && value <= i32::MAX
    }
}

impl InstructionSelector {
    fn Get(&self, node: OpIndex) -> &Operation {
        self.Get(node)
    }

    fn DefineSameAsFirst(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand::DefineSameAsFirst(node)
    }

    fn UseRegister(&self, node: OpIndex) -> InstructionOperand {
        InstructionOperand::UseRegister(node)
    }
    
    fn IsLoadOrLoadImmutable(&self, node: OpIndex) -> bool {
		let op = self.Get(node);
		op.is::<LoadOp>()
	}

    fn Emit(&mut self, opcode: InstructionCode, output_count: usize, outputs: &[InstructionOperand], input_count: usize, inputs: &[InstructionOperand], temp_count: usize, temps: &[InstructionOperand]) {
        self.Emit(opcode, output_count, outputs, input_count, inputs, temp_count, temps)
    }
	
    fn IsCommutative(&self, node: OpIndex) -> bool {
		false
	}

	fn CanCover(&self, user: OpIndex, value: OpIndex) -> bool {
		true
	}

	fn GetEffectLevel(&self, node: OpIndex) -> i32 {
		0
	}
	
	fn TryCast<T>(&self, node: OpIndex) -> Option<&T>
	where
		T: Any,
	{
		let op = self.Get(node);
		op.try_cast::<T>()
	}
	
	fn value(&self, index: OpIndex) -> OpIndex {
		index
	}

    fn IsSupported(&self, feature: CpuFeature) -> bool {
        true
    }

    fn EmitWithContinuation(&mut self, opcode: InstructionCode, output_count: usize, outputs: &[InstructionOperand], input_count: usize, inputs: &[InstructionOperand], temp_count: usize, temps: &[InstructionOperand], cont: &FlagsContinuationT) {
        self.EmitWithContinuation(opcode, output_count, outputs, input_count, inputs, temp_count, temps, cont)
    }
}
pub enum CpuFeature{
    POPCNT,
    SSE4_1,
    SSE4_2,
    AVX,
    IA32,
    ARM64
}
pub enum RegisterMode{
    kRegister,
    kUniqueRegister
}
fn TryMatchBaseWithScaledIndexAndDisplacement(selector: &IA32OperandGeneratorT, node: OpIndex) -> Option<BaseWithScaledIndexAndDisplacementMatch> {
	todo!()
}
fn TryMatchBaseWithScaledIndexAndDisplacementForWordBinop(selector: &IA32OperandGeneratorT, left: OpIndex, right: OpIndex) -> Option<BaseWithScaledIndexAndDisplacementMatch>{
	todo!()
}

impl InstructionOperand {
	fn UseRegisterWithMode(node: OpIndex, register_mode: RegisterMode) -> Self {
		InstructionOperand::UseRegister(node)
	}

	fn UseUniqueRegisterOrSlotOrConstant(node: OpIndex) -> Self {
		InstructionOperand::Use(node)
	}
}
