// Converted from V8 C++ source files:
// Header: dataview-lowering-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod dataview_lowering_reducer {
    use crate::compiler::turboshaft::assembler::*;
    use crate::compiler::turboshaft::int64_lowering_reducer::MemoryRepresentation;
    use crate::compiler::turboshaft::copying_phase::Variable;
    use crate::compiler::turboshaft::deopt_data::OpIndex;
    use crate::compiler::turboshaft::deopt_data::MachineType;
    use std::rc::Rc;

    pub enum ExternalArrayType {
        kExternalInt8Array,
        kExternalUint8Array,
        kExternalUint8ClampedArray,
        kExternalInt16Array,
        kExternalFloat16Array,
        kExternalUint16Array,
        kExternalInt32Array,
        kExternalUint32Array,
        kExternalFloat32Array,
        kExternalFloat64Array,
        kExternalBigInt64Array,
        kExternalBigUint64Array,
    }

    pub struct AccessBuilder {}

    impl AccessBuilder {
        pub fn ForTypedArrayElement(element_type: ExternalArrayType, arg: bool) -> AccessBuilder {
            AccessBuilder {}
        }
        pub fn machine_type(&self) -> MachineType {
            MachineType::Any
        }
    }

    pub trait DataViewLoweringReducerNext {
        fn asm(&self) -> &Assembler;
        fn asm_mut(&mut self) -> &mut Assembler;
    }

    pub struct DataViewLoweringReducer<Next: DataViewLoweringReducerNext> {
        next: Next,
    }

    impl<Next: DataViewLoweringReducerNext> DataViewLoweringReducer<Next> {
        pub fn new(next: Next) -> Self {
            DataViewLoweringReducer { next }
        }

        fn asm(&self) -> &Assembler {
            self.next.asm()
        }

        fn asm_mut(&mut self) -> &mut Assembler {
            self.next.asm_mut()
        }

        fn build_reverse_bytes(&mut self, array_type: ExternalArrayType, value: OpIndex) -> OpIndex {
            match array_type {
                ExternalArrayType::kExternalInt8Array |
                ExternalArrayType::kExternalUint8Array |
                ExternalArrayType::kExternalUint8ClampedArray => value,
                ExternalArrayType::kExternalInt16Array => {
                    self.word32_shift_right_arithmetic(self.word32_reverse_bytes(value), 16)
                }
                ExternalArrayType::kExternalFloat16Array |
                ExternalArrayType::kExternalUint16Array => {
                    self.word32_shift_right_logical(self.word32_reverse_bytes(value), 16)
                }
                ExternalArrayType::kExternalInt32Array |
                ExternalArrayType::kExternalUint32Array => self.word32_reverse_bytes(value),
                ExternalArrayType::kExternalFloat32Array => {
                    let bytes = self.bitcast_float32_to_word32(value);
                    let reversed = self.word32_reverse_bytes(bytes);
                    self.bitcast_word32_to_float32(reversed)
                }
                ExternalArrayType::kExternalFloat64Array => {
                    if cfg!(target_pointer_width = "64") {
                        let bytes = self.bitcast_float64_to_word64(value);
                        let reversed = self.word64_reverse_bytes(bytes);
                        self.bitcast_word64_to_float64(reversed)
                    } else {
                        let reversed_lo = self.word32_reverse_bytes(self.float64_extract_low_word32(value));
                        let reversed_hi = self.word32_reverse_bytes(self.float64_extract_high_word32(value));
                        self.bitcast_word32_pair_to_float64(reversed_lo, reversed_hi)
                    }
                }
                ExternalArrayType::kExternalBigInt64Array |
                ExternalArrayType::kExternalBigUint64Array => self.word64_reverse_bytes(value),
            }
        }

        fn reduce_load_data_view_element(
            &mut self,
            object: OpIndex,
            storage: OpIndex,
            index: OpIndex,
            is_little_endian: OpIndex,
            element_type: ExternalArrayType,
        ) -> OpIndex {
            let machine_type = AccessBuilder::ForTypedArrayElement(element_type, true).machine_type();
            let memory_rep = MemoryRepresentation::FromMachineType(machine_type);

            let value = self.load(storage, index, LoadOp::Kind::MaybeUnaligned(memory_rep).NotLoadEliminable(), memory_rep);

            let mut result = self.asm_mut().new_loop_invariant_variable(RegisterRepresentationForArrayType(element_type));

            if self.is_true(is_little_endian) {
                #[cfg(target_endian = "little")]
                self.asm_mut().set_variable(&mut result, value);
                #[cfg(not(target_endian = "little"))]
                self.asm_mut().set_variable(&mut result, self.build_reverse_bytes(element_type, value));
            } else {
                #[cfg(target_endian = "little")]
                self.asm_mut().set_variable(&mut result, self.build_reverse_bytes(element_type, value));
                #[cfg(not(target_endian = "little"))]
                self.asm_mut().set_variable(&mut result, value);
            }

            // We need to keep the {object} (either the JSArrayBuffer or the JSDataView)
            // alive so that the GC will not release the JSArrayBuffer (if there's any)
            // as long as we are still operating on it.
            self.retain(object);
            self.asm().get_variable(&result)
        }

        fn reduce_store_data_view_element(
            &mut self,
            object: OpIndex,
            storage: OpIndex,
            index: OpIndex,
            value: OpIndex,
            is_little_endian: OpIndex,
            element_type: ExternalArrayType,
        ) -> OpIndex {
            let machine_type = AccessBuilder::ForTypedArrayElement(element_type, true).machine_type();

            let mut value_to_store = self.asm_mut().new_loop_invariant_variable(RegisterRepresentationForArrayType(element_type));
            if self.is_true(is_little_endian) {
                #[cfg(target_endian = "little")]
                self.asm_mut().set_variable(&mut value_to_store, value);
                #[cfg(not(target_endian = "little"))]
                self.asm_mut().set_variable(&mut value_to_store, self.build_reverse_bytes(element_type, value));
            } else {
                #[cfg(target_endian = "little")]
                self.asm_mut().set_variable(&mut value_to_store, self.build_reverse_bytes(element_type, value));
                #[cfg(not(target_endian = "little"))]
                self.asm_mut().set_variable(&mut value_to_store, value);
            }

            let memory_rep = MemoryRepresentation::FromMachineType(machine_type);
            self.store(storage, index, self.asm().get_variable(&value_to_store), StoreOp::Kind::MaybeUnaligned(memory_rep).NotLoadEliminable(), memory_rep, WriteBarrierKind::kNoWriteBarrier);

            // We need to keep the {object} (either the JSArrayBuffer or the JSDataView)
            // alive so that the GC will not release the JSArrayBuffer (if there's any)
            // as long as we are still operating on it.
            self.retain(object);
            OpIndex{} // Return empty OpIndex for void return
        }

        fn is_true(&self, is_little_endian: OpIndex) -> bool {
            true
        }

        fn retain(&mut self, object: OpIndex) {}

        fn load(&mut self, storage: OpIndex, index: OpIndex, kind: LoadOp::Kind, memory_rep: MemoryRepresentation) -> OpIndex {
            OpIndex{}
        }

        fn store(&mut self, storage: OpIndex, index: OpIndex, value: OpIndex, kind: StoreOp::Kind, memory_rep: MemoryRepresentation, write_barrier_kind: WriteBarrierKind) {}

        fn word32_shift_right_arithmetic(&mut self, value: OpIndex, shift: i32) -> OpIndex {
            OpIndex{}
        }

        fn word32_reverse_bytes(&mut self, value: OpIndex) -> OpIndex {
            OpIndex{}
        }

        fn word32_shift_right_logical(&mut self, value: OpIndex, shift: i32) -> OpIndex {
            OpIndex{}
        }

        fn bitcast_float32_to_word32(&mut self, value: OpIndex) -> OpIndex {
            OpIndex{}
        }

        fn bitcast_word32_to_float32(&mut self, value: OpIndex) -> OpIndex {
            OpIndex{}
        }

        fn bitcast_float64_to_word64(&mut self, value: OpIndex) -> OpIndex {
            OpIndex{}
        }

        fn word64_reverse_bytes(&mut self, value: OpIndex) -> OpIndex {
            OpIndex{}
        }

        fn bitcast_word64_to_float64(&mut self, value: OpIndex) -> OpIndex {
            OpIndex{}
        }

        fn float64_extract_low_word32(&mut self, value: OpIndex) -> OpIndex {
            OpIndex{}
        }

        fn float64_extract_high_word32(&mut self, value: OpIndex) -> OpIndex {
            OpIndex{}
        }

        fn bitcast_word32_pair_to_float64(&mut self, reversed_lo: OpIndex, reversed_hi: OpIndex) -> OpIndex {
            OpIndex{}
        }
    }

    pub enum RegisterRepresentation {
        Word32,
        Word64,
        Float32,
        Float64,
    }

    fn RegisterRepresentationForArrayType(element_type: ExternalArrayType) -> RegisterRepresentation {
        match element_type {
            ExternalArrayType::kExternalInt8Array |
            ExternalArrayType::kExternalUint8Array |
            ExternalArrayType::kExternalUint8ClampedArray |
            ExternalArrayType::kExternalInt16Array |
            ExternalArrayType::kExternalUint16Array |
            ExternalArrayType::kExternalInt32Array |
            ExternalArrayType::kExternalUint32Array => RegisterRepresentation::Word32,
            ExternalArrayType::kExternalFloat32Array => RegisterRepresentation::Float32,
            ExternalArrayType::kExternalFloat64Array => RegisterRepresentation::Float64,
            ExternalArrayType::kExternalBigInt64Array |
            ExternalArrayType::kExternalBigUint64Array => RegisterRepresentation::Word64,
        }
    }

    pub struct LoadOp {}
    impl LoadOp {
        pub struct Kind {
            kind: i32
        }

        impl Kind {
            pub fn MaybeUnaligned(memory_rep: MemoryRepresentation) -> Self {
                Kind {kind: 0}
            }
            pub fn NotLoadEliminable(self) -> Self {
                Kind { kind: 0 }
            }
        }
    }
    pub struct StoreOp {}
    impl StoreOp {
        pub struct Kind {
            kind: i32
        }

        impl Kind {
            pub fn MaybeUnaligned(memory_rep: MemoryRepresentation) -> Self {
                Kind {kind: 0}
            }
            pub fn NotLoadEliminable(self) -> Self {
                Kind {kind: 0}
            }
        }
    }

    pub enum WriteBarrierKind {
        kNoWriteBarrier
    }
}
