// Converted from V8 C++ source files:
// Header: deopt-data.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct SmallVector<T, const SIZE: usize> {
        data: Vec<T>,
    }

    impl<T, const SIZE: usize> SmallVector<T, SIZE> {
        pub fn push_back(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn clear(&mut self) {
            self.data.clear();
        }
    }

    impl<T, const SIZE: usize> Default for SmallVector<T, SIZE> {
        fn default() -> Self {
            SmallVector { data: Vec::new() }
        }
    }

    pub struct Vector<'a, T> {
        slice: &'a [T],
    }

    impl<'a, T> Vector<'a, T> {
        pub fn of(slice: &'a [T]) -> Self {
            Vector { slice }
        }

        pub fn empty(&self) -> bool {
            self.slice.is_empty()
        }
    }
}

pub mod common {
    pub type Address = usize;
}

pub mod compiler {
    pub mod frame_states {
        pub struct FrameStateInfo {}
        pub struct FrameState {}
    }

    pub mod turboshaft {
        use crate::base;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct OpIndex {
            id: u32,
        }

        impl OpIndex {
            pub fn new(id: u32) -> Self {
                OpIndex { id }
            }

            pub fn id(&self) -> u32 {
                self.id
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum MachineType {
            Int32,
            Float64,
            Any,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CreateArgumentsType {
            MappedArguments,
            UnmappedArguments,
            RestArguments,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct FrameStateData {
            pub frame_state_info: FrameStateInfo,
            pub instructions: base::Vector<'static, Instr>,
            pub machine_types: base::Vector<'static, MachineType>,
            pub int_operands: base::Vector<'static, u32>,
        }

        impl PartialEq for FrameStateData {
            fn eq(&self, other: &Self) -> bool {
                self.instructions.slice == other.instructions.slice &&
                self.machine_types.slice == other.machine_types.slice &&
                self.int_operands.slice == other.int_operands.slice
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Instr {
            kInput,
            kUnusedRegister,
            kDematerializedObject,
            kDematerializedObjectReference,
            kArgumentsElements,
            kArgumentsLength,
            kRestLength,
            kDematerializedStringConcat,
            kDematerializedStringConcatReference,
        }

        pub struct Builder {
            instructions_: base::SmallVector<Instr, 32>,
            machine_types_: base::SmallVector<MachineType, 32>,
            int_operands_: base::SmallVector<u32, 16>,
            inputs_: base::SmallVector<OpIndex, 32>,
            inlined_: bool,
        }

        impl Builder {
            pub fn new() -> Self {
                Builder {
                    instructions_: base::SmallVector::default(),
                    machine_types_: base::SmallVector::default(),
                    int_operands_: base::SmallVector::default(),
                    inputs_: base::SmallVector::default(),
                    inlined_: false,
                }
            }

            pub fn add_parent_frame_state(&mut self, _parent: FrameState) {
                assert!(self.inputs_.empty());
                self.inlined_ = true;
            }
            pub fn add_input(&mut self, type_: MachineType, input: OpIndex) {
                self.instructions_.push_back(Instr::kInput);
                self.machine_types_.push_back(type_);
                self.inputs_.push_back(input);
            }

            pub fn add_unused_register(&mut self) {
                self.instructions_.push_back(Instr::kUnusedRegister);
            }

            pub fn add_dematerialized_object_reference(&mut self, id: u32) {
                self.instructions_.push_back(Instr::kDematerializedObjectReference);
                self.int_operands_.push_back(id);
            }

            pub fn add_dematerialized_object(&mut self, id: u32, field_count: u32) {
                self.instructions_.push_back(Instr::kDematerializedObject);
                self.int_operands_.push_back(id);
                self.int_operands_.push_back(field_count);
            }

            pub fn add_dematerialized_string_concat(&mut self, id: u32) {
                self.instructions_.push_back(Instr::kDematerializedStringConcat);
                self.int_operands_.push_back(id);
            }

            pub fn add_dematerialized_string_concat_reference(&mut self, id: u32) {
                self.instructions_.push_back(Instr::kDematerializedStringConcatReference);
                self.int_operands_.push_back(id);
            }

            pub fn add_arguments_elements(&mut self, type_: CreateArgumentsType) {
                self.instructions_.push_back(Instr::kArgumentsElements);
                self.int_operands_.push_back(type_ as i32 as u32);
            }

            pub fn add_arguments_length(&mut self) {
                self.instructions_.push_back(Instr::kArgumentsLength);
            }

            pub fn add_rest_length(&mut self) {
                self.instructions_.push_back(Instr::kRestLength);
            }

            pub fn allocate_frame_state_data(
                &self,
                info: &FrameStateInfo,
                zone: &Zone,
            ) -> FrameStateData {
                FrameStateData {
                    frame_state_info: *info,
                    instructions: base::Vector::of(zone.clone_vector(self.instructions_.data.as_slice())),
                    machine_types: base::Vector::of(zone.clone_vector(self.machine_types_.data.as_slice())),
                    int_operands: base::Vector::of(zone.clone_vector(self.int_operands_.data.as_slice())),
                }
            }

            pub fn inputs(&self) -> base::Vector<OpIndex> {
                 base::Vector::of(self.inputs_.data.as_slice())
            }
            pub fn inlined(&self) -> bool {
                self.inlined_
            }
        }

        #[derive(Debug)]
        pub struct Iterator<'a> {
            pub instructions: base::Vector<'a, Instr>,
            pub machine_types: base::Vector<'a, MachineType>,
            pub int_operands: base::Vector<'a, u32>,
            pub inputs: base::Vector<'a, OpIndex>,
        }

        impl<'a> Iterator<'a> {
            pub fn has_more(&self) -> bool {
                if self.instructions.slice.is_empty() {
                   assert!(self.machine_types.slice.is_empty());
                   assert!(self.int_operands.slice.is_empty());
                }
                !self.instructions.slice.is_empty()
            }

            pub fn current_instr(&self) -> Instr {
                self.instructions.slice[0]
            }

            pub fn consume_input(&mut self, machine_type: &mut MachineType, input: &mut OpIndex) {
                assert_eq!(self.instructions.slice[0], Instr::kInput);
                *machine_type = self.machine_types.slice[0];
                *input = self.inputs.slice[0];

                self.instructions.slice = &self.instructions.slice[1..];
                self.machine_types.slice = &self.machine_types.slice[1..];
                self.inputs.slice = &self.inputs.slice[1..];
            }
            pub fn consume_unused_register(&mut self) {
                assert_eq!(self.instructions.slice[0], Instr::kUnusedRegister);
                self.instructions.slice = &self.instructions.slice[1..];
            }
            pub fn consume_dematerialized_object(&mut self, id: &mut u32, field_count: &mut u32) {
                assert_eq!(self.instructions.slice[0], Instr::kDematerializedObject);

                *id = self.int_operands.slice[0];
                *field_count = self.int_operands.slice[1];
                self.instructions.slice = &self.instructions.slice[1..];
                self.int_operands.slice = &self.int_operands.slice[2..];
            }
            pub fn consume_dematerialized_object_reference(&mut self, id: &mut u32) {
                assert_eq!(self.instructions.slice[0], Instr::kDematerializedObjectReference);
                *id = self.int_operands.slice[0];
                self.instructions.slice = &self.instructions.slice[1..];
                self.int_operands.slice = &self.int_operands.slice[1..];
            }
            pub fn consume_dematerialized_string_concat(&mut self, id: &mut u32) {
                assert_eq!(self.instructions.slice[0], Instr::kDematerializedStringConcat);
                *id = self.int_operands.slice[0];
                 self.instructions.slice = &self.instructions.slice[1..];
                self.int_operands.slice = &self.int_operands.slice[1..];
            }
            pub fn consume_dematerialized_string_concat_reference(&mut self, id: &mut u32) {
                assert_eq!(self.instructions.slice[0], Instr::kDematerializedStringConcatReference);
                *id = self.int_operands.slice[0];
                 self.instructions.slice = &self.instructions.slice[1..];
                self.int_operands.slice = &self.int_operands.slice[1..];
            }
            pub fn consume_arguments_elements(&mut self, type_: &mut CreateArgumentsType) {
                assert_eq!(self.instructions.slice[0], Instr::kArgumentsElements);
                *type_ = match self.int_operands.slice[0] {
                    0 => CreateArgumentsType::MappedArguments,
                    1 => CreateArgumentsType::UnmappedArguments,
                    2 => CreateArgumentsType::RestArguments,
                    _ => panic!("Invalid CreateArgumentsType value"),
                };
                self.instructions.slice = &self.instructions.slice[1..];
                self.int_operands.slice = &self.int_operands.slice[1..];
            }
            pub fn consume_arguments_length(&mut self) {
                assert_eq!(self.instructions.slice[0], Instr::kArgumentsLength);
                self.instructions.slice = &self.instructions.slice[1..];
            }
            pub fn consume_rest_length(&mut self) {
                assert_eq!(self.instructions.slice[0], Instr::kRestLength);
                self.instructions.slice = &self.instructions.slice[1..];
            }
        }

        impl FrameStateData {
            pub fn iterator<'a>(&self, state_values: base::Vector<'a, OpIndex>) -> Iterator<'a> {
                Iterator {
                    instructions: base::Vector::of(self.instructions.slice),
                    machine_types: base::Vector::of(self.machine_types.slice),
                    int_operands: base::Vector::of(self.int_operands.slice),
                    inputs: state_values,
                }
            }
        }
    }
}

pub mod zone {
    pub struct Zone {}

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }

        pub fn clone_vector<T: Copy>(&self, slice: &[T]) -> &'static [T] {
            let boxed_slice = slice.to_vec().into_boxed_slice();
            let raw_ptr = Box::into_raw(boxed_slice);
            unsafe { &*raw_ptr }
        }
    }

}

pub mod v8 {
    pub mod internal {
        pub mod compiler {
            pub mod turboshaft {
                pub use super::super::super::turboshaft::{
                    FrameStateData,
                    Builder,
                    FrameStateInfo,
                    MachineType,
                    OpIndex,
                    CreateArgumentsType,
                };
            }
        }
    }
}
