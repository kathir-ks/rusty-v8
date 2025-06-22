// TODO: Add necessary crate imports based on the functionality of the code.
// For example:
// use std::rc::Rc;
// use std::cell::RefCell;

mod compiler {
    use std::cell::Cell;
    //use std::rc::Rc;
    //use std::cell::RefCell;

    // Mock types and functions to represent the V8 codebase.
    pub type NodeId = usize;

    pub struct Node {
        id: NodeId,
        opcode: IrOpcode,
        inputs: Vec<*mut Node>,
    }

    impl Node {
        pub fn id(&self) -> NodeId {
            self.id
        }
        pub fn opcode(&self) -> IrOpcode {
            self.opcode
        }
        pub fn input_at(&self, index: usize) -> *mut Node {
            self.inputs[index]
        }
        pub fn replace_input(&mut self, index: usize, new_input: *mut Node) {
            self.inputs[index] = new_input;
        }
        pub fn insert_input(&mut self, zone: &Zone, index: usize, new_input: *mut Node) {
          self.inputs.insert(index, new_input);
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IrOpcode {
        kAllocate,
        kAllocateRaw,
        kLoadFromObject,
        kLoadImmutableFromObject,
        kLoadElement,
        kLoadField,
        kStoreToObject,
        kInitializeImmutableInObject,
        kStoreElement,
        kStoreField,
        kStore,
        kParameter,
        kBitcastWordToTagged,
        kHeapConstant,
        kBitcastTaggedToWord,
        kInt32Add,
        kInt64Add,
        kWordEqual,
        kWordAnd,
        kWordShl,
        kWordShr,
        kUintLessThan,
        kCall,
        kDebugBreak,
    }

    pub struct Operator {
        // Add operator fields as needed
    }

    pub struct CommonOperatorBuilder {}
    impl CommonOperatorBuilder {
      pub fn call(&self, _call_descriptor: CallDescriptor) -> &'static Operator {
        &Operator{}
      }
      pub fn number_constant(&self, _value: i32) -> *mut Node {
        std::ptr::null_mut()
      }
      pub fn int32_constant(&self, _value: i32) -> *mut Node {
          std::ptr::null_mut()
      }
      pub fn int64_constant(&self, _value: i64) -> *mut Node {
          std::ptr::null_mut()
      }
    }

    pub struct MachineOperatorBuilder {
        // Add machine operator builder fields as needed
    }
    impl MachineOperatorBuilder {
      pub fn load(&self, _type: MachineType) -> &'static Operator {
        &Operator{}
      }
      pub fn store(&self, _rep: StoreRepresentation) -> &'static Operator {
        &Operator{}
      }
      pub fn store_indirect_pointer(&self, _wb: WriteBarrierKind) -> &'static Operator {
        &Operator{}
      }
    }

    pub struct NodeProperties {}
    impl NodeProperties {
      pub fn change_op(_node: *mut Node, _op: &'static Operator) {}
      pub fn get_value_input(_node: *mut Node, _index: usize) -> *mut Node {
        std::ptr::null_mut()
      }
      pub fn get_effect_input(_node: *mut Node) -> *mut Node {
        std::ptr::null_mut()
      }
      pub fn get_control_input(_node: *mut Node) -> *mut Node {
        std::ptr::null_mut()
      }
    }

    pub struct JSGraph {
        graph: Graph,
        common: CommonOperatorBuilder,
        machine: MachineOperatorBuilder,
        isolate: *mut Isolate,
    }
    impl JSGraph {
      pub fn graph(&self) -> &Graph {
        &self.graph
      }
      pub fn common(&self) -> &CommonOperatorBuilder {
        &self.common
      }
      pub fn machine(&self) -> &MachineOperatorBuilder {
        &self.machine
      }
      pub fn isolate(&self) -> *mut Isolate {
        self.isolate
      }
    }

    pub struct JSGraphAssembler {}
    impl JSGraphAssembler {
      pub fn initialize_effect_control(&self, _effect: *mut Node, _control: *mut Node) {}
      pub fn effect(&self) -> *mut Node {
        std::ptr::null_mut()
      }
      pub fn control(&self) -> *mut Node {
        std::ptr::null_mut()
      }
      pub fn add_node(&self, _node: *mut Node) -> *mut Node {
        std::ptr::null_mut()
      }
    }

    pub struct Graph {
        // Add graph fields as needed
    }
    impl Graph {
        pub fn start(&self) -> *mut Node {
            std::ptr::null_mut()
        }
        pub fn new_node(&self, _op: *const Operator) -> *mut Node {
            std::ptr::null_mut()
        }
    }

    pub struct Zone {
        // Add zone fields as needed
    }
    impl Zone {
        pub fn new<T>(&self) -> Box<T>
        where T: Default {
            Box::new(T::default())
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum AllocationType {
        kYoung,
        kOld,
        kCode,
    }

    #[derive(PartialEq, Eq)]
    pub enum AllocationFolding {
        kDoAllocationFolding,
        kDontDoAllocationFolding
    }

    pub type WriteBarrierAssertFailedCallback =
        fn(node: *mut Node, object: *mut Node, function_debug_name: &str, zone: &Zone);

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum WriteBarrierKind {
        kNoWriteBarrier,
        kAssertNoWriteBarrier,
        // Add other write barrier kinds as necessary
    }

    pub struct MemoryLowering {
        isolate_: *mut Isolate,
        zone_: *mut Zone,
        graph_: *mut Graph,
        common_: *mut CommonOperatorBuilder,
        machine_: *mut MachineOperatorBuilder,
        graph_assembler_: *mut JSGraphAssembler,
        is_wasm_: bool,
        allocation_folding_: AllocationFolding,
        write_barrier_assert_failed_: WriteBarrierAssertFailedCallback,
        function_debug_name_: String,
        allocate_operator_: Cell<Option<&'static Operator>>,
        wasm_instance_node_: Cell<Option<*mut Node>>,
    }

    impl MemoryLowering {
        pub fn new(
            jsgraph: *mut JSGraph,
            zone: *mut Zone,
            graph_assembler: *mut JSGraphAssembler,
            is_wasm: bool,
            allocation_folding: AllocationFolding,
            write_barrier_assert_failed: WriteBarrierAssertFailedCallback,
            function_debug_name: &str,
        ) -> Self {
            unsafe {
              let jsgraph_ref = jsgraph.as_ref().unwrap();
              MemoryLowering {
                  isolate_: jsgraph_ref.isolate,
                  zone_: zone,
                  graph_: jsgraph_ref.graph as *mut Graph,
                  common_: &jsgraph_ref.common as *const CommonOperatorBuilder as *mut CommonOperatorBuilder,
                  machine_: &jsgraph_ref.machine as *const MachineOperatorBuilder as *mut MachineOperatorBuilder,
                  graph_assembler_: graph_assembler,
                  is_wasm_: is_wasm,
                  allocation_folding_: allocation_folding,
                  write_barrier_assert_failed_: write_barrier_assert_failed,
                  function_debug_name_: function_debug_name.to_string(),
                  allocate_operator_: Cell::new(None),
                  wasm_instance_node_: Cell::new(None),
              }
            }
        }

        pub fn graph_zone(&self) -> *mut Zone {
            unsafe {
                self.graph_.as_ref().unwrap(); // Dereference to check validity
                self.zone_
            }
        }

        pub fn reduce(&mut self, node: *mut Node) -> Reduction {
            unsafe {
              let node_ref = node.as_ref().unwrap();
              match node_ref.opcode() {
                  IrOpcode::kAllocate => {
                      unreachable!();
                  }
                  IrOpcode::kAllocateRaw => self.reduce_allocate_raw(node),
                  IrOpcode::kLoadFromObject | IrOpcode::kLoadImmutableFromObject => {
                      self.reduce_load_from_object(node)
                  }
                  IrOpcode::kLoadElement => self.reduce_load_element(node),
                  IrOpcode::kLoadField => self.reduce_load_field(node),
                  IrOpcode::kStoreToObject | IrOpcode::kInitializeImmutableInObject => {
                      self.reduce_store_to_object(node, std::ptr::null())
                  }
                  IrOpcode::kStoreElement => self.reduce_store_element(node, std::ptr::null()),
                  IrOpcode::kStoreField => self.reduce_store_field(node, std::ptr::null()),
                  IrOpcode::kStore => self.reduce_store(node, std::ptr::null()),
                  _ => Reduction::NoChange,
              }
            }
        }

        fn ensure_allocate_operator(&self) {
            if self.allocate_operator_.get().is_some() {
                return;
            }

            // Dummy implementation as the original C++ code uses V8 internals.
            self.allocate_operator_.set(Some(&Operator{}));
        }

        // #[cfg(V8_ENABLE_WEBASSEMBLY)]
        fn get_wasm_instance_node(&self) -> *mut Node {
            if self.wasm_instance_node_.get().is_some() {
                return self.wasm_instance_node_.get().unwrap();
            }

            unsafe {
              let graph_ref = self.graph_.as_ref().unwrap();
              for use_node in (*graph_ref).start().as_ref().unwrap().inputs.iter() {
                  let use_node_ref = use_node.as_ref().unwrap();
                  if use_node_ref.opcode() == IrOpcode::kParameter && parameter_index_of(use_node_ref as *const Node as *mut Node) == kWasmInstanceDataParameterIndex {
                      self.wasm_instance_node_.set(Some(*use_node));
                      return *use_node;
                  }
              }
            }
            unreachable!(); // The instance node must have been created before.
        }

        fn align_to_allocation_alignment(&self, value: *mut Node) -> *mut Node {
            // Simplified implementation as V8_COMPRESS_POINTERS_8GB_BOOL is not available.
            value
        }

        fn reduce_allocate_raw(&mut self, node: *mut Node) -> Reduction {
            self.reduce_allocate_raw_impl(node, AllocationType::kYoung, std::ptr::null_mut())
        }

        fn reduce_allocate_raw_impl(
            &mut self,
            node: *mut Node,
            mut allocation_type: AllocationType,
            state_ptr: *mut *const AllocationState,
        ) -> Reduction {
            unsafe {
              if v8_flags::single_generation && allocation_type == AllocationType::kYoung {
                  allocation_type = AllocationType::kOld;
              }
              // InstructionStream objects may have a maximum size smaller than
              // kMaxHeapObjectSize due to guard pages. If we need to support allocating
              // code here we would need to call
              // MemoryChunkLayout::MaxRegularCodeObjectSize() at runtime.
              if allocation_type == AllocationType::kCode {
                unreachable!();
              }

              let node_ref = node.as_ref().unwrap();
              let size = node_ref.input_at(0);
              let effect = node_ref.input_at(1);
              let control = node_ref.input_at(2);

              let gasm = self.graph_assembler_.as_mut().unwrap();
              gasm.initialize_effect_control(effect, control);

              let allocate_builtin = if !self.is_wasm_ {
                  if allocation_type == AllocationType::kYoung {
                      self.allocate_in_young_generation_stub_constant()
                  } else {
                      self.allocate_in_old_generation_stub_constant()
                  }
              } else {
                // WASM specific code
                std::ptr::null_mut()
              };

              // Determine the top/limit addresses.
              let top_address;
              let limit_address;

              if !self.isolate_.is_null() {
                // !self.is_wasm_
                top_address = self.external_constant(if allocation_type == AllocationType::kYoung {
                    ExternalReferenceType::NewSpaceAllocationTopAddress
                } else {
                    ExternalReferenceType::OldSpaceAllocationTopAddress
                });
                limit_address = self.external_constant(if allocation_type == AllocationType::kYoung {
                    ExternalReferenceType::NewSpaceAllocationLimitAddress
                } else {
                    ExternalReferenceType::OldSpaceAllocationLimitAddress
                });
              } else {
                // Wasm mode
                let instance_node = self.get_wasm_instance_node();

                let top_address_offset = if allocation_type == AllocationType::kYoung {
                  kWasmTrustedInstanceDataNewAllocationTopAddressOffset
                } else {
                  kWasmTrustedInstanceDataOldAllocationTopAddressOffset
                };
                let limit_address_offset = if allocation_type == AllocationType::kYoung {
                  kWasmTrustedInstanceDataNewAllocationLimitAddressOffset
                } else {
                  kWasmTrustedInstanceDataOldAllocationLimitAddressOffset
                };

                top_address = self.load(MachineType::Pointer, instance_node, top_address_offset as i64 - kHeapObjectTag as i64);
                limit_address = self.load(MachineType::Pointer, instance_node, limit_address_offset as i64 - kHeapObjectTag as i64);
              }
              
              let intptr_matcher = IntPtrMatcher{node: size};

              if intptr_matcher.is_in_range(0, kMaxRegularHeapObjectSize) && v8_flags::inline_new && self.allocation_folding_ == AllocationFolding::kDoAllocationFolding {
                let object_size = align_to_allocation_alignment(intptr_matcher.resolved_value());
                // state_ptr not null checked here due to Rust translation
                let state = if state_ptr.is_null() {
                  None
                } else {
                  state_ptr.as_ref().map(|ptr| ptr.as_ref()).flatten()
                };
                if state.is_some() {
                  let state_val = state.unwrap();
                  if state_val.size() <= kMaxRegularHeapObjectSize - object_size && state_val.group().allocation() == allocation_type {
                    let state_size = state_val.size() + object_size;

                    let group = state_val.group();
                    //TODO(bmeurer): Implement Int64/32 constant opcode functions
                    // if machine_ref.is_64() {

                    // } else {
                    //   if op_parameter::<i32>(group.size().op()) < state_size as i32 {
                    //     NodeProperties::change_op(group.size(), common_ref.int32_constant(state_size as i32));
                    //   }
                    // }

                  } else {

                  }
                } else {

                }
              }

              Reduction::Replace(std::ptr::null_mut())
            }
        }

        fn reduce_load_from_object(&self, node: *mut Node) -> Reduction {
            unsafe {
              let node_ref = node.as_ref().unwrap();
              let access = object_access_of(node_ref.opcode());
              let machine_type = access.machine_type;

              if machine_type.is_map_word() {
                  return self.reduce_load_map(node);
              }

              let rep = machine_type.representation();
              let load_op = if element_size_in_bytes(rep) > kTaggedSize as usize && !self.machine_().unaligned_load_supported(rep) {
                  //self.machine().unaligned_load(machine_type)
                  &Operator{}
              } else {
                  //self.machine().load(machine_type)
                  &Operator{}
              };
              NodeProperties::change_op(node, load_op);
              Reduction::Changed(node)
            }
        }

        fn reduce_load_element(&self, node: *mut Node) -> Reduction {
            unsafe {
              let node_ref = node.as_ref().unwrap();
              let access = element_access_of(node_ref.opcode());
              let index = node_ref.input_at(1);
              let mut_node = node as *mut Node;
              (*mut_node).replace_input(1, self.compute_index(access, index));

              let type_ = access.machine_type;
              if type_.is_map_word() {
                  unreachable!();
              }

              let machine_ = self.machine_.as_ref().unwrap();
              let load_op = machine_.load(type_);
              NodeProperties::change_op(node, load_op);
              Reduction::Changed(node)
            }
        }

        fn reduce_load_field(&self, node: *mut Node) -> Reduction {
            unsafe {
              let node_ref = node.as_ref().unwrap();
              let access = field_access_of(node_ref.opcode());

              let offset = (access.offset - access.tag()) as i64;
              let offset_node = self.intptr_constant(offset);
              let mut_node = node as *mut Node;
              (*mut_node).insert_input(self.graph_zone(), 1, offset_node);

              let type_ = access.machine_type;

              if type_.is_map_word() {
                  return self.reduce_load_map(node);
              }

              if access.type_.is_external_pointer() {
                  return self.reduce_load_external_pointer_field(node);
              }

              if access.is_bounded_size_access {
                  return self.reduce_load_bounded_size(node);
              }
              let machine_ = self.machine_.as_ref().unwrap();
              let load_op = machine_.load(type_);
              NodeProperties::change_op(node, load_op);
              Reduction::Changed(node)
            }
        }

        fn reduce_store_to_object(
            &self,
            node: *mut Node,
            state: *const AllocationState,
        ) -> Reduction {
            unsafe {
                let node_ref = node.as_ref().unwrap();
                let access = object_access_of(node_ref.opcode());
                let object = node_ref.input_at(0);
                let value = node_ref.input_at(2);

                let write_barrier_kind = self.compute_write_barrier_kind(
                    node,
                    object,
                    value,
                    if state.is_null() { None } else { state.as_ref() },
                    access.write_barrier_kind,
                );
                if access.machine_type.is_map_word() {
                    unreachable!();
                }
                let rep = access.machine_type.representation();
                let store_rep = StoreRepresentation::new(rep, write_barrier_kind);
                let store_op = if element_size_in_bytes(rep) > kTaggedSize as usize && !self.machine_().unaligned_store_supported(rep) {
                   //self.machine().unaligned_store(rep)
                   &Operator{}
                } else {
                    //self.machine().store(store_rep)
                    &Operator{}
                };
                NodeProperties::change_op(node, store_op);
                Reduction::Changed(node)
            }
        }

        fn reduce_store_element(
            &self,
            node: *mut Node,
            state: *const AllocationState,
        ) -> Reduction {
            unsafe {
                let node_ref = node.as_ref().unwrap();
                let access = element_access_of(node_ref.opcode());
                let object = node_ref.input_at(0);
                let index = node_ref.input_at(1);
                let value = node_ref.input_at(2);
                let mut_node = node as *mut Node;
                (*mut_node).replace_input(1, self.compute_index(access, index));

                let write_barrier_kind = self.compute_write_barrier_kind(
                    node,
                    object,
                    value,
                    if state.is_null() { None } else { state.as_ref() },
                    access.write_barrier_kind,
                );
                let machine_ = self.machine_.as_ref().unwrap();
                let store_op = machine_.store(StoreRepresentation::new(
                    access.machine_type.representation(),
                    write_barrier_kind,
                ));
                NodeProperties::change_op(node, store_op);
                Reduction::Changed(node)
            }
        }

        fn reduce_store_field(&self, node: *mut Node, state: *const AllocationState) -> Reduction {
            unsafe {
              let node_ref = node.as_ref().unwrap();
              let access = field_access_of(node_ref.opcode());
              if access.type_.is_external_pointer() && v8_flags::enable_sandbox {
                unreachable!();
              }
              if access.type_.is_sandboxed_pointer() {
                unreachable!();
              }
              if access.is_bounded_size_access {
                unreachable!();
              }
              let machine_type = access.machine_type;
              let object = node_ref.input_at(0);
              let value = node_ref.input_at(1);

              let effect = NodeProperties::get_effect_input(node);
              let control = NodeProperties::get_control_input(node);
              let gasm = self.graph_assembler_.as_mut().unwrap();
              gasm.initialize_effect_control(effect, control);

              let write_barrier_kind = self.compute_write_barrier_kind(
                  node,
                  object,
                  value,
                  if state.is_null() { None } else { state.as_ref() },
                  access.write_barrier_kind,
              );

              let offset = (access.offset - access.tag()) as i64;
              let offset_node = self.intptr_constant(offset);
              let mut_node = node as *mut Node;
              (*mut_node).insert_input(self.graph_zone(), 1, offset_node);

              let mut adjusted_machine_type = machine_type;
              let mut adjusted_value = value;
              if machine_type.is_map_word() {
                  adjusted_machine_type = MachineType::TaggedPointer;
                  // #[cfg(V8_MAP_PACKING)]
                  // {
                  //     adjusted_value = self.pack_map_word(TNode::<Map>::unchecked_cast(value));
                  //     (*mut_node).replace_input(2, adjusted_value);
                  // }
              }

              if adjusted_machine_type.representation() == MachineRepresentation::kIndirectPointer {
                  if access.indirect_pointer_tag == IndirectPointerTag::kIndirectPointerNullTag {
                      unreachable!();
                  }
                  let tag = self.intptr_constant(access.indirect_pointer_tag as i64);
                  (*mut_node).insert_input(self.graph_zone(), 3, tag);
                  let machine_ = self.machine_.as_ref().unwrap();
                  let store_op = machine_.store_indirect_pointer(write_barrier_kind);
                  NodeProperties::change_op(node, store_op);
              } else {
                let machine_ = self.machine_.as_ref().unwrap();
                let store_op = machine_.store(StoreRepresentation::new(
                    adjusted_machine_type.representation(),
                    write_barrier_kind,
                ));
                NodeProperties::change_op(node, store_op);
              }
              Reduction::Changed(node)
            }
        }

        fn reduce_store(&self, node: *mut Node, state: *const AllocationState) -> Reduction {
            unsafe {
                let node_ref = node.as_ref().unwrap();
                let representation = store_representation_of(node_ref.opcode());
                let object = node_ref.input_at(0);
                let value = node_ref.input_at(2);

                let write_barrier_kind = self.compute_write_barrier_kind(
                    node,
                    object,
                    value,
                    if state.is_null() { None } else { state.as_ref() },
                    representation.write_barrier_kind(),
                );

                if write_barrier_kind != representation.write_barrier_kind() {
                    let machine_ = self.machine_.as_ref().unwrap();
                    let store_op = machine_.store(StoreRepresentation::new(
                        representation.representation(),
                        write_barrier_kind,
                    ));
                    NodeProperties::change_op(node, store_op);
                    return Reduction::Changed(node);
                }

                Reduction::NoChange
            }
        }

        fn compute_index(&self, access: ElementAccess, index: *mut Node) -> *mut Node {
            unsafe {
              let element_size_shift = element_size_log2_of(access.machine_type.representation());
              let mut adjusted_index = index;

              if element_size_shift != 0 {
                  adjusted_index = self.word_shl(adjusted_index, element_size_shift as i64);
              }

              let fixed_offset = access.header_size - access.tag();
              if fixed_offset != 0 {
                  adjusted_index = self.int_add(adjusted_index, fixed_offset as i64);
              }

              adjusted_index
            }
        }

        fn compute_write_barrier_kind(
            &self,
            node: *mut Node,
            object: *mut Node,
            value: *mut Node,
            state: Option<&AllocationState>,
            mut write_barrier_kind: WriteBarrierKind,
        ) -> WriteBarrierKind {
            unsafe {
                if let Some(state) = state {
                    if state.is_young_generation_allocation() && state.group().contains(object) {
                        write_barrier_kind = WriteBarrierKind::kNoWriteBarrier;
                    }
                }
                let isolate_ref = self.isolate_.as_ref();

                if !value_needs_write_barrier(value, isolate_ref) {
                    write_barrier_kind = WriteBarrierKind::kNoWriteBarrier;
                }
                if v8_flags::disable_write_barriers {
                    write_barrier_kind = WriteBarrierKind::kNoWriteBarrier;
                }
                if write_barrier_kind == WriteBarrierKind::kAssertNoWriteBarrier {
                    (self.write_barrier_assert_failed_)(
                        node,
                        object,
                        &self.function_debug_name_,
                        self.zone_.as_ref().unwrap(),
                    );
                }
                write_barrier_kind
            }
        }

        // Mock functions for accessing internal data
        fn allocate_in_young_generation_stub_constant(&self) -> *mut Node {
            std::ptr::null_mut() // Replace with actual implementation
        }

        fn allocate_in_old_generation_stub_constant(&self) -> *mut Node {
            std::ptr::null_mut() // Replace with actual implementation
        }

        fn external_constant(&self, _type: ExternalReferenceType) -> *mut Node {
          std::ptr::null_mut()
        }

        fn intptr_constant(&self, value: i64) -> *mut Node {
          unsafe {
            let common_ref = self.common_.as_ref().unwrap();
            common_ref.int64_constant(value)
          }
        }

        fn word_shl(&self, node: *mut Node, shift: i64) -> *mut Node {
          unsafe {
            let common_ref = self.common_.as_ref().unwrap();
            common_ref.int64_constant(shift)
          }
        }
        
        fn int_add(&self, node: *mut Node, value: i64) -> *mut Node {
          unsafe {
            let common_ref = self.common_.as_ref().unwrap();
            common_ref.int64_constant(value)
          }
        }

        fn reduce_load_external_pointer_field(&self, _node: *mut Node) -> Reduction {
          unimplemented!()
        }

        fn reduce_load_bounded_size(&self, _node: *mut Node) -> Reduction {
          unimplemented!()
        }

        fn reduce_load_map(&self, _node: *mut Node) -> Reduction {
          unimplemented!()
        }

        fn load(&self, _mt: MachineType, _arg1: *mut Node, _arg2: i64) -> *mut Node {
          unimplemented!()
        }
    }

    // Implementations for helper structs.
    // #[derive(Default)]
    pub struct AllocationGroup {
        node_ids_: Vec<NodeId>,
        allocation_: AllocationType,
        size_: *mut Node,
    }

    impl Default for AllocationGroup {
      fn default() -> Self {
        AllocationGroup {
          node_ids_: Vec::new(),
          allocation_: AllocationType::kYoung,
          size_: std::ptr::null_mut(),
        }
      }
    }

    impl AllocationGroup {
        fn new(node: *mut Node, allocation: AllocationType, zone: *mut Zone) -> Self {
            let mut group = Self::default();
            group.allocation_ = Self::check_allocation_type(allocation);
            unsafe {
              group.node_ids_.push(node.as_ref().unwrap().id());
            }
            group
        }

        fn new_with_size(
            node: *mut Node,
            allocation: AllocationType,
            size: *mut Node,
            zone: *mut Zone,
        ) -> Self {
            let mut group = Self::default();
            group.allocation_ = Self::check_allocation_type(allocation);
            group.size_ = size;
            unsafe {
              group.node_ids_.push(node.as_ref().unwrap().id());
            }
            group
        }

        fn add(&mut self, node: *mut Node) {
            unsafe {
              self.node_ids_.push(node.as_ref().unwrap().id());
            }
        }

        fn contains(&self, node: *mut Node) -> bool {
            unsafe {
                let mut current_node = node;
                while self.node_ids_.iter().find(|&x| *x == current_node.as_ref().unwrap().id()).is_none() {
                    match current_node.as_ref().unwrap().opcode() {
                        IrOpcode::kBitcastTaggedToWord | IrOpcode::kBitcastWordToTagged
                        | IrOpcode::kInt32Add | IrOpcode::kInt64Add => {
                            current_node = (*current_node).input_at(0);
                        }
                        _ => return false,
                    }
                }
                true
            }
        }

        fn is_young_generation_allocation(&self) -> bool {
            self.allocation() == AllocationType::kYoung
        }

        fn allocation(&self) -> AllocationType {
            self.allocation_
        }

        fn size(&self) -> *mut Node {
            self.size_
        }

        fn check_allocation_type(allocation: AllocationType) -> AllocationType {
            if v8_flags::single_generation && allocation == AllocationType::kYoung {
                AllocationType::kOld
            } else {
                allocation
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Reduction {
        Changed(*mut Node),
        NoChange,
        Replace(*mut Node),
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MachineRepresentation {
        kWord8,
        kWord16,
        kWord32,
        kWord64,
        kFloat32,
        kFloat64,
        kSimd128,
        kTaggedSigned,
        kTaggedPointer,
        kTagged,
        kBit,
        kIndirectPointer,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MachineSemantic {
        kAny,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct MachineType {
        representation: MachineRepresentation,
        semantic: MachineSemantic,
    }

    impl MachineType {
        pub const WORD8: Self = MachineType {
            representation: MachineRepresentation::kWord8,
            semantic: MachineSemantic::kAny,
        };
        pub const WORD16: Self = MachineType {
            representation: MachineRepresentation::kWord16,
            semantic: MachineSemantic::kAny,
        };
        pub const WORD32: Self = MachineType {
            representation: MachineRepresentation::kWord32,
            semantic: MachineSemantic::kAny,
        };
        pub const WORD64: Self = MachineType {
            representation: MachineRepresentation::kWord64,
            semantic: MachineSemantic::kAny,
        };
        pub const FLOAT32: Self = MachineType {
            representation: MachineRepresentation::kFloat32,