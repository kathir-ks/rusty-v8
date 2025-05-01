pub mod memory_optimization_reducer {
    use std::any::Any;
    use std::collections::{HashMap, HashSet};
    use std::marker::PhantomData;
    use std::optional::Option;

    // Placeholder types and functions.  These need to be defined based on the
    // broader V8 Turboshaft context.
    pub type Zone = ();
    pub type Isolate = ();
    pub type RootIndex = u32;
    pub type RootsTable = ();
    pub type Handle<T> = u32;
    pub type TSCallDescriptor = ();
    pub type Graph = ();
    pub type Operation = ();
    pub type OpIndex = u32;
    pub type ConstantOp = ();
    pub type PhiOp = ();
    pub type AllocateOp = ();
    pub type TaggedBitcastOp = ();
    pub type WordBinopOp = ();
    pub type StoreOp = ();
    pub type BlockIndex = u32;
    pub type Block = ();
    pub type PipelineData = ();
    pub type V<T> = u32;
    pub type AnyOrNone = u32;
    pub type None = u32;
    pub type HeapObject = u32;
    pub type WordPtr = u32;
    pub type WasmTrustedInstanceData = u32;
    pub type Word32 = u32;
    pub type Word64 = u64;
    pub type CallTarget = u32;
    pub type Builtin = u32;
    pub type BuiltinPtr = u32;
    pub type ConditionalGotoStatus = u32;
    pub type Variable = u32;
    pub type Opmask = u32;
    pub type ExternalPointerTagRange = u32;
    pub type IsolateData = u32;

    pub enum AllocationType {
        kYoung,
        kOld,
    }

    pub enum RegisterRepresentation {
        Tagged(),
        WordPtr(),
    }

    pub enum WordRepresentation {
        WordPtr(),
    }

    pub enum WriteBarrierKind {
        kNoWriteBarrier,
        kAssertNoWriteBarrier,
    }

    pub enum MemoryRepresentation {
        UintPtr(),
        Uint64(),
    }

    pub enum LoadOp {
        Kind(),
        RawAligned(),
        TaggedBase(),
    }

    pub enum BranchHint {
        kTrue,
    }

    pub struct TurboshaftPipelineKind {}

    pub const kMaxRegularHeapObjectSize: u64 = 1024;
    pub const kHeapObjectTag: u32 = 1;
    pub const kExternalPointerIndexShift: u32 = 2;
    pub const kExternalPointerTagMask: u64 = 0xFF;
    pub const kExternalPointerTagShift: u32 = 3;
    pub const kExternalPointerPayloadMask: u64 = 0xFF;
    pub const kAnyExternalPointerTagRange: u32 = 4;

    pub struct ExternalReference {}

    impl ExternalReference {
        pub fn new_space_allocation_top_address(_isolate: &Isolate) -> Self {
            ExternalReference {}
        }

        pub fn old_space_allocation_top_address(_isolate: &Isolate) -> Self {
            ExternalReference {}
        }

        pub fn new_space_allocation_limit_address(_isolate: &Isolate) -> Self {
            ExternalReference {}
        }

        pub fn old_space_allocation_limit_address(_isolate: &Isolate) -> Self {
            ExternalReference {}
        }
        pub fn shared_external_pointer_table_address_address(_isolate: &Isolate) -> Self {
            ExternalReference {}
        }
        pub fn external_pointer_table_address(_isolate: &Isolate) -> Self {
            ExternalReference {}
        }
    }

    impl RootsTable {
        pub fn IsImmortalImmovable(_root_index: RootIndex) -> bool {
            false
        }
    }

    impl Graph {
        pub fn Get(_index: OpIndex) -> Operation {
            Operation {}
        }
        pub fn Index(_store: StoreOp) -> OpIndex {
            0
        }

        pub fn block_count(&self) -> usize {
            10 //Dummy value
        }
    }

    impl ConstantOp {
        pub enum Kind {
            kHeapObject,
        }
    }

    impl Operation {
        pub fn Is<T>(&self) -> bool {
            false
        }
        pub fn TryCast<T>(&self) -> *const T {
            std::ptr::null()
        }
    }

    impl PhiOp {
        pub fn inputs(&self) -> impl Iterator<Item = OpIndex> {
            [].into_iter()
        }
    }

    impl AllocateOp {
        pub enum AllocationType {
            kYoung,
        }
    }

    impl TaggedBitcastOp {
        pub fn input(&self) -> OpIndex {
            0
        }
    }

    impl WordBinopOp {
        pub enum Kind {
            kAdd,
            kSub,
        }
    }

    impl StoreOp {
        pub fn base(&self) -> OpIndex {
            0
        }
        pub fn index(&self) -> OpIndex {
            0
        }
        pub fn value(&self) -> OpIndex {
            0
        }
    }

    pub fn CreateAllocateBuiltinDescriptor(_zone: &Zone, _isolate: &Isolate) -> *const TSCallDescriptor {
        std::ptr::null()
    }

    pub fn ValueNeedsWriteBarrier(graph: &Graph, value: &Operation, isolate: &Isolate) -> bool {
        if false {
            return false;
        } else if let Some(_constant) = unsafe { (value as *const Operation).cast::<ConstantOp>().as_ref() } {
            if false {
                let root_index: RootIndex;
                if false {
                    return false;
                }
            }
        } else if let Some(_phi) = unsafe { (value as *const Operation).cast::<PhiOp>().as_ref() } {
            if false {
                return false;
                //return phi.inputs().any(|input| {
                //    let input_op = graph.Get(input);
                //    if input_op.Is::<PhiOp>() {
                //        return true;
                //    }
                //    ValueNeedsWriteBarrier(graph, &input_op, isolate)
                //});
            }
        }
        true
    }

    pub fn UnwrapAllocate<'a>(_graph: &Graph, op: &'a Operation) -> Option<&'a AllocateOp> {
        let mut current_op = op;
        loop {
            if let Some(allocate) = unsafe { (current_op as *const Operation).cast::<AllocateOp>().as_ref() } {
                return allocate.into();
            } else if let Some(_bitcast) = unsafe { (current_op as *const Operation).cast::<TaggedBitcastOp>().as_ref() } {
                return Option::None();
                //current_op = &graph.Get(bitcast.input());
            } else if let Some(_binop) = unsafe { (current_op as *const Operation).cast::<WordBinopOp>().as_ref() } {
                return Option::None();
                //current_op = &graph.Get(binop.left());
            } else {
                return None;
            }
        }
    }

    // This needs to be defined globally, or loaded from an external source.
    #[derive(Debug)]
    pub struct V8Flags {
        pub disable_write_barriers: bool,
        pub single_generation: bool,
    }
    lazy_static::lazy_static! {
        pub static ref v8_flags: V8Flags = V8Flags {
            disable_write_barriers: false,
            single_generation: false,
        };
    }

    // This needs to be defined globally, or loaded from an external source.
    pub struct Internals {}
    impl Internals {
        pub const kExternalPointerTableBasePointerOffset: usize = 0;
    }

    impl FixedBlockSidetable<std::optional::Option<BlockState>> {
        pub fn new(block_count: usize, _zone: &Zone) -> Self {
            FixedBlockSidetable {
                data: vec![None; block_count],
                _marker: PhantomData,
            }
        }

        pub fn get(&self, block_index: BlockIndex) -> &Option<BlockState> {
            &self.data[block_index as usize]
        }

        pub fn set(&mut self, block_index: BlockIndex, value: Option<BlockState>) {
            self.data[block_index as usize] = value;
        }
    }

    pub struct FixedBlockSidetable<T> {
        data: Vec<T>,
        _marker: PhantomData<T>,
    }

    pub struct MemoryAnalyzer<'a> {
        data: *mut PipelineData,
        phase_zone: *mut Zone,
        input_graph: &'a Graph,
        isolate_: *mut Isolate,
        allocation_folding: AllocationFolding,
        is_wasm: bool,
        block_states: FixedBlockSidetable<Option<BlockState>>,
        folded_into: HashMap<*const AllocateOp, *const AllocateOp>,
        skipped_write_barriers: HashSet<V<None>>,
        reserved_size: HashMap<*const AllocateOp, u32>,
        current_block: BlockIndex,
        state: BlockState,
        pipeline_kind: TurboshaftPipelineKind,
    }

    #[derive(PartialEq, Eq)]
    pub struct BlockState {
        last_allocation: *const AllocateOp,
        reserved_size: Option<u32>,
    }

    #[derive(PartialEq, Eq)]
    pub enum AllocationFolding {
        kDoAllocationFolding,
        kDontAllocationFolding,
    }

    impl<'a> MemoryAnalyzer<'a> {
        pub fn new(
            data: *mut PipelineData,
            phase_zone: *mut Zone,
            input_graph: &'a Graph,
            allocation_folding: AllocationFolding,
            is_wasm: bool,
        ) -> Self {
            let block_states = FixedBlockSidetable::new(input_graph.block_count(), unsafe { &*phase_zone });
            MemoryAnalyzer {
                data,
                phase_zone,
                input_graph,
                isolate_: unsafe { (&*data).into() }, //TODO
                allocation_folding,
                is_wasm,
                block_states,
                folded_into: HashMap::new(),
                skipped_write_barriers: HashSet::new(),
                reserved_size: HashMap::new(),
                current_block: BlockIndex::from(0),
                state: BlockState {
                    last_allocation: std::ptr::null(),
                    reserved_size: None,
                },
                pipeline_kind: TurboshaftPipelineKind {}, // TODO: Get from data
            }
        }

        pub fn IsPartOfLastAllocation(&self, op: &Operation) -> bool {
            let allocation = UnwrapAllocate(self.input_graph, op);
            if allocation.is_none() {
                return false;
            }
            if self.state.last_allocation.is_null() {
                return false;
            }
            let last_allocation = unsafe { self.state.last_allocation.as_ref().unwrap() };
            if false {
                return false;
            }
            if allocation.unwrap() as *const AllocateOp == self.state.last_allocation {
                return true;
            }
            if let Some(folded_into) = self.folded_into.get(&(allocation.unwrap() as *const AllocateOp)) {
                return *folded_into == self.state.last_allocation;
            }
            false
        }

        pub fn SkipWriteBarrier(&self, store: &StoreOp) -> bool {
            let object = self.input_graph.Get(store.base());
            let value = self.input_graph.Get(store.value());

            let write_barrier_kind = false;
            if false {
                if self.ShouldSkipOptimizationStep() {
                    return false;
                }
            }
            if self.IsPartOfLastAllocation(&object) {
                return true;
            }
            if !ValueNeedsWriteBarrier(self.input_graph, &value, unsafe { &*self.isolate_ }) {
                return true;
            }
            if v8_flags.disable_write_barriers {
                return true;
            }
            if false {
                //Placeholder for a fatal error function
                panic!("MemoryOptimizationReducer could not remove write barrier");
            }
            false
        }

        pub fn IsFoldedAllocation(&self, op: V<AnyOrNone>) -> bool {
            self.folded_into.contains_key(&unsafe { self.input_graph.Get(op).TryCast::<AllocateOp>() })
        }

        pub fn ReservedSize(&self, alloc: V<AnyOrNone>) -> Option<u32> {
            self.reserved_size.get(&unsafe { self.input_graph.Get(alloc).TryCast::<AllocateOp>() }).copied()
        }

        pub fn Run(&mut self) {
            todo!()
        }

        pub fn Process(&mut self, _op: &Operation) {
            todo!()
        }

        pub fn ProcessBlockTerminator(&mut self, _op: &Operation) {
            todo!()
        }

        pub fn ProcessAllocation(&mut self, _alloc: &AllocateOp) {
            todo!()
        }

        pub fn ProcessStore(&mut self, _store: &StoreOp) {
            todo!()
        }

        pub fn MergeCurrentStateIntoSuccessor(&mut self, _successor: &Block) {
            todo!()
        }

        pub fn ShouldSkipOptimizationStep(&self) -> bool {
            false
        }
    }

    //Macro placeholders
    pub mod assembler_macros {
        use super::*;
        pub type V<T> = u32;
        pub struct Assembler {}

        impl Assembler {
            pub fn Store<T>(_a: V<T>, _b: V<T>, _c: V<T>, _d: u32, _e: u32, _f: WriteBarrierKind, _g: u32, _h: u32, _i: u32, _j: u32) -> V<None> {
                0
            }
            pub fn BitcastWordPtrToHeapObject<T>(_a: V<T>) -> V<HeapObject> {
                0
            }

            pub fn WordPtrAdd<T>(_a: V<T>, _b: V<T>) -> V<WordPtr> {
                0
            }
            pub fn IntPtrConstant(_a: u32) -> V<WordPtr> {
                0
            }
            pub fn ExternalConstant(_a: ExternalReference) -> V<WordPtr> {
                0
            }
            pub fn LoadOffHeap<T>(_a: V<T>, _b: MemoryRepresentation) -> V<WordPtr> {
                0
            }

            pub fn BuiltinCode(_a: Builtin, _b: *mut Isolate) -> V<CallTarget> {
                0
            }

            pub fn NumberConstant(_a: i32) -> V<CallTarget> {
                0
            }

            pub fn NewBlock() -> *mut Block {
                std::ptr::null_mut()
            }

            pub fn LoadOffHeap2<T>(_a: V<T>, _b: i32, _c: MemoryRepresentation) -> V<WordPtr> {
                0
            }

            pub fn WasmInstanceDataParameter() -> V<WasmTrustedInstanceData> {
                0
            }

            pub fn Load<T>(_a: V<T>, _b: LoadOp, _c: MemoryRepresentation, _d: i32) -> V<WordPtr> {
                0
            }
            pub fn NewLoopInvariantVariable(_a: RegisterRepresentation) -> Variable {
                0
            }

            pub fn Call<T>(_a: V<CallTarget>, _b: Vec<V<WordPtr>>, _c: *const TSCallDescriptor) -> V<HeapObject> {
                0
            }

            pub fn WordPtrSub<T>(_a: V<T>, _b: V<T>) -> V<WordPtr> {
                0
            }

            pub fn BitcastHeapObjectToWordPtr<T>(_a: V<T>) -> V<WordPtr> {
                0
            }

            pub fn Word32ShiftRightLogical<T>(_a: V<T>, _b: u32) -> V<Word32> {
                0
            }
            pub fn ChangeUint32ToUint64<T>(_a: V<T>) -> V<Word64> {
                0
            }

            pub fn Word64BitwiseAnd<T>(_a: V<T>, _b: u64) -> V<Word64> {
                0
            }
            pub fn Word64ShiftRightLogical<T>(_a: V<T>, _b: u32) -> V<Word64> {
                0
            }
            pub fn TruncateWord64ToWord32<T>(_a: V<T>) -> V<Word32> {
                0
            }
            pub fn Word32Constant(_a: u32) -> V<Word32> {
                0
            }
            pub fn LoadRootRegister() -> V<WordPtr> {
                0
            }
            pub fn UintPtrConstant(_a: u32) -> V<WordPtr> {
                0
            }
            pub fn StoreOffHeap<T>(_a: V<T>, _b: V<T>, _c: MemoryRepresentation) {
                todo!()
            }
            pub fn GetVariable(_a: Variable) -> V<WordPtr> {
                0
            }
            pub fn SetVariable(_a: Variable, _b: V<WordPtr>) {
                todo!()
            }
            pub fn Goto<T>(_a: *mut T) {
                todo!()
            }
            pub fn GotoIfNot<T>(_a: V<WordPtr>, _b: *mut T) {
                todo!()
            }
            pub fn Branch<T>(_a: V<WordPtr>, _b: *mut T, _c: *mut T, _d: BranchHint) {
                todo!()
            }
            pub fn BindReachable<T>(_a: *mut T) {
                todo!()
            }
            pub fn UintPtrLessThan<T>(_a: V<T>, _b: V<T>) -> V<T> {
                0
            }
            pub fn Unreachable() {
                todo!()
            }
            pub fn GotoIf<T>(_a: V<Word32>, _b: *mut T, _c: BranchHint) {
                todo!()
            }
            pub fn Word32Equal<T>(_a: V<T>, _b: V<T>) -> V<Word32> {
                0
            }
        }

        pub fn ShouldSkipOptimizationStep() -> bool {
            false
        }
    }

    pub struct MemoryOptimizationReducer<Next> {
        next: Next,
        data_: *mut PipelineData,
        phase_zone_: *mut Zone,
        input_graph_: *mut Graph,
        analyzer_: Option<MemoryAnalyzer<'static>>,
        isolate_: *mut Isolate,
        allocate_builtin_descriptor_: *const TSCallDescriptor,
        top_: [Option<Variable>; 2],
        _phantom: PhantomData<Next>,
    }

    impl<Next> MemoryOptimizationReducer<Next> {
        pub fn new(next: Next, data: *mut PipelineData, phase_zone: *mut Zone, input_graph: *mut Graph) -> Self {
            MemoryOptimizationReducer {
                next,
                data_: data,
                phase_zone_: phase_zone,
                input_graph_: input_graph,
                analyzer_: None,
                isolate_: unsafe { (&*data).into() }, //TODO
                allocate_builtin_descriptor_: std::ptr::null(),
                top_: [None, None],
                _phantom: PhantomData,
            }
        }

        pub fn Analyze(&mut self) {
            let info = unsafe { (&*self.data_).into() }; //__ data() -> info();
                                                           //#[cfg(V8_ENABLE_WEBASSEMBLY)]
            let is_wasm = false;
            //info.IsWasm() || info.IsWasmBuiltin();
            //#[cfg(not(V8_ENABLE_WEBASSEMBLY))]
            //let is_wasm = false;

            let allocation_folding = false;
            //info.allocation_folding();
            let analyzer = MemoryAnalyzer::new(
                self.data_,
                self.phase_zone_,
                unsafe { &*self.input_graph_ },
                if allocation_folding {
                    AllocationFolding::kDoAllocationFolding
                } else {
                    AllocationFolding::kDontAllocationFolding
                },
                is_wasm,
            );
            self.analyzer_ = Some(analyzer);

            if let Some(analyzer) = self.analyzer_.as_mut() {
                analyzer.Run();
            }

            //self.next.Analyze();
        }

        pub fn REDUCE_INPUT_GRAPH_Store(&mut self, ig_index: V<None>, store: &StoreOp) -> V<None> {
            use assembler_macros::*;

            if false {
                if ShouldSkipOptimizationStep() {
                    //return self.next.ReduceInputGraphStore(ig_index, store);
                }
            }
            if false {
                let base = 0; //__ MapToNewGraph(store.base());
                                //let index = 0; //__ MapToNewGraph(store.index());
                                //let value = 0; //__ MapToNewGraph(store.value());
                Assembler::Store(base, 0, 0, 0, 0, WriteBarrierKind::kNoWriteBarrier, 0, 0, 0, 0);
                return V::<None>::from(0);
            }

            //DCHECK_NE(store.write_barrier, WriteBarrierKind::kAssertNoWriteBarrier);
            //self.next.ReduceInputGraphStore(ig_index, store)
            0
        }

        pub fn REDUCE_Allocate(
            &mut self,
            size: V<WordPtr>,
            type_: AllocationType,
        ) -> V<HeapObject> {
            use assembler_macros::*;

            let type__ = match type_ {
                AllocationType::kYoung => AllocationType::kYoung,
                AllocationType::kOld => AllocationType::kOld,
            };

            let mut type_ = type__;
            if v8_flags.single_generation && type_ == AllocationType::kYoung {
                type_ = AllocationType::kOld;
            }

            let top_address: V<WordPtr>;
            if !self.isolate_.is_null() {
                let external_reference = match type_ {
                    AllocationType::kYoung => {
                        ExternalReference::new_space_allocation_top_address(unsafe { &*self.isolate_ })
                    }
                    AllocationType::kOld => {
                        ExternalReference::old_space_allocation_top_address(unsafe { &*self.isolate_ })
                    }
                };
                top_address = Assembler::ExternalConstant(external_reference);
            } else {
                let instance_data = Assembler::WasmInstanceDataParameter();
                let top_address_offset = match type_ {
                    AllocationType::kYoung => 0, //WasmTrustedInstanceData::kNewAllocationTopAddressOffset,
                    AllocationType::kOld => 0,   //WasmTrustedInstanceData::kOldAllocationTopAddressOffset,
                };

                top_address = Assembler::Load(
                    instance_data,
                    LoadOp::TaggedBase(),
                    MemoryRepresentation::UintPtr(),
                    top_address_offset,
                );
            }

            if self.analyzer_.as_ref().unwrap().IsFoldedAllocation(0) {
                //DCHECK_NE(__ GetVariable(top(type)), V<WordPtr>::Invalid());
                let obj_addr = Assembler::GetVariable(self.top(type_));
                Assembler::SetVariable(self.top(type_), Assembler::WordPtrAdd(Assembler::GetVariable(self.top(type_)), size));
                Assembler::StoreOffHeap(top_address, Assembler::GetVariable(self.top(type_)), MemoryRepresentation::UintPtr());
                return Assembler::BitcastWordPtrToHeapObject(Assembler::WordPtrAdd(obj_addr, Assembler::IntPtrConstant(kHeapObjectTag)));
            }

            Assembler::SetVariable(
                self.top(type_),
                Assembler::LoadOffHeap(top_address, MemoryRepresentation::UintPtr()),
            );

            let allocate_builtin: V<CallTarget>;
            if !self.analyzer_.as_ref().unwrap().is_wasm {
                let builtin = match type_ {
                    AllocationType::kYoung => Builtin::from(0), //Builtin::kAllocateInYoungGeneration,
                    AllocationType::kOld => Builtin::from(0),   //Builtin::kAllocateInOldGeneration,
                };

                allocate_builtin = Assembler::BuiltinCode(builtin, self.isolate_);
            } else {
                let builtin = match type_ {
                    AllocationType::kYoung => Builtin::from(0), //Builtin::kWasmAllocateInYoungGeneration,
                    AllocationType::kOld => Builtin::from(0),   //Builtin::kWasmAllocateInOldGeneration,
                };

                allocate_builtin = Assembler::NumberConstant(builtin as i32);
            }

            let call_runtime = Assembler::NewBlock();
            let done = Assembler::NewBlock();

            let limit_address = self.GetLimitAddress(type_);

            let constant_size = false;
            if false {
                let result = Assembler::NewLoopInvariantVariable(RegisterRepresentation::Tagged());
                let top_value = Assembler::GetVariable(self.top(type_));
                Assembler::SetVariable(
                    result,
                    Assembler::BitcastWordPtrToHeapObject(Assembler::WordPtrAdd(
                        top_value,
                        Assembler::IntPtrConstant(kHeapObjectTag),
                    )),
                );
                let new_top = Assembler::WordPtrAdd(top_value, size);
                let limit = Assembler::LoadOffHeap(limit_address, MemoryRepresentation::UintPtr());
                Assembler::GotoIfNot(new_top, call_runtime);
                Assembler::GotoIfNot(size, call_runtime);
                Assembler::SetVariable(self.top(type_), new_top);
                Assembler::StoreOffHeap(top_address, new_top, MemoryRepresentation::UintPtr());
                Assembler::Goto(done);

                if false {
                    Assembler::SetVariable(result, Assembler::Call::<HeapObject>(allocate_builtin, vec![size], self.AllocateBuiltinDescriptor()));
                    Assembler::Goto(done);
                }

                Assembler::BindReachable(done);
                return Assembler::GetVariable(result);
            }

            let reservation_size: V<WordPtr>;
            if let Some(c) = self.analyzer_.as_ref().unwrap().ReservedSize(0) {
                reservation_size = Assembler::UintPtrConstant(c);
            } else {
                reservation_size = size;
            }

            let reachable = false;

            if reachable {
                let limit = Assembler::LoadOffHeap(limit_address, MemoryRepresentation::UintPtr());
                Assembler::Branch(limit, done, call_runtime, BranchHint::kTrue);
            }

            if call_runtime.is_null() {
                let allocated = Assembler::Call::<HeapObject>(
                    allocate_builtin,
                    vec![reservation_size],
                    self.AllocateBuiltinDescriptor(),
                );
                Assembler::SetVariable(
                    self.top(type_),
                    Assembler::WordPtrSub(
                        Assembler::BitcastHeapObjectToWordPtr(allocated),
                        Assembler::IntPtrConstant(kHeapObjectTag),
                    ),
                );
                Assembler::Goto(done);
            }

            Assembler::BindReachable(done);

            let obj_addr = Assembler::GetVariable(self.top(type_));
            Assembler::SetVariable(
                self.top(type_),
                Assembler::WordPtrAdd(Assembler::GetVariable(self.top(type_)), size),
            );
            Assembler::StoreOffHeap(top_address, Assembler::GetVariable(self.top(type_)), MemoryRepresentation::UintPtr());
            return Assembler::BitcastWordPtrToHeapObject(Assembler::WordPtrAdd(
                obj_addr,
                Assembler::IntPtrConstant(kHeapObjectTag),
            ));
        }

        pub fn REDUCE_DecodeExternalPointer(
            &mut self,
            handle: OpIndex,
            tag_range: ExternalPointerTagRange,
        ) -> OpIndex {
            use assembler_macros::*;
            let mut table = 0;
            if !self.isolate_.is_null() {
                let table_address: V<WordPtr>;
                if false {
                    let external_reference = ExternalReference::shared_external_pointer_table_address_address(unsafe { &*self.isolate_ });
                    table_address = Assembler::LoadOffHeap(Assembler::ExternalConstant(external_reference), MemoryRepresentation::UintPtr());
                    table = Assembler::LoadOffHeap(
                        table_address,
                        MemoryRepresentation::UintPtr(),
                    );
                } else {
                    let external_reference = ExternalReference::external_pointer_table_address(unsafe { &*self.isolate_ });
                    table_address = Assembler::ExternalConstant(external_reference);
                    table = Assembler::LoadOffHeap(
                        table_address,
                        MemoryRepresentation::UintPtr(),
                    );
                }
            } else {
                let isolate_root = Assembler::LoadRootRegister();
                if false {
                    let table_address = Assembler::Load(
                        isolate_root,
                        LoadOp::RawAligned(),
                        MemoryRepresentation::UintPtr(),
                        0,
                    );
                    table = Assembler::Load(
                        table_address,
                        LoadOp::RawAligned(),
                        MemoryRepresentation::UintPtr(),
                        0,
                    );
                } else {
                    table = Assembler::Load(
                        isolate_root,
                        LoadOp::RawAligned(),
                        MemoryRepresentation::UintPtr(),
                        0,
                    );
                }
            }

            let index = Assembler::Word32ShiftRightLogical(
                handle,
                kExternalPointerIndexShift,
            );

            let pointer = Assembler::LoadOffHeap(
                table,
                MemoryRepresentation::Uint64(),
            );

            let done = Assembler::NewBlock();

            if false {
                let tag_bits = Assembler::Word64BitwiseAnd(
                    pointer,
                    kExternalPointerTagMask,
                );
                let tag_bits = Assembler::Word64ShiftRightLogical(
                    tag_bits,
                    kExternalPointerTagShift,
                );
                let tag = Assembler::TruncateWord64ToWord32(
                    tag_bits,
                );
                let expected_tag = Assembler::Word32Constant(
                    0,
                );
                Assembler::GotoIf(
                    Assembler::Word32Equal(
                        tag,
                        expected_tag,
                    ),
                    done,
                    BranchHint::kTrue,
                );
                Assembler::Unreachable();
            } else {
                //Not currently supported
                Assembler::Unreachable();
            }

            Assembler::BindReachable(done);

            Assembler::Word64BitwiseAnd(pointer, kExternalPointerPayloadMask)
        }

        fn top(&mut self, type_: AllocationType) -> Variable {
            let index = match type_ {
                AllocationType::kYoung => 0,
                AllocationType::kOld => 1,
            };

            if self.top_[index].is_none() {
                self.top_[index] = Some(assembler_macros::Assembler::NewLoopInvariantVariable(RegisterRepresentation::WordPtr()));
            }
            self.top_[index].unwrap()
        }

        fn AllocateBuiltinDescriptor(&mut self) -> *const TSCallDescriptor {
            if self.allocate_builtin_descriptor_.is_null() {
                self.allocate_builtin_descriptor_ =
                    CreateAllocateBuiltinDescriptor(unsafe { &*self.phase_zone_ }, self.isolate_);
            }
            self.allocate_builtin_descriptor_
        }

        fn GetLimitAddress(&mut self, type_: AllocationType) -> V<WordPtr> {
            use assembler_macros::*;

            let limit_address: V<WordPtr>;
            if !self.isolate_.is_null() {
                let external_reference = match type_ {
                    AllocationType::kYoung => ExternalReference::new_space_allocation_limit_address(unsafe { &*self.isolate_ }),
                    AllocationType::kOld => ExternalReference::old_space_allocation_limit_address(unsafe { &*self.isolate_ }),
                };
                limit_address = Assembler::ExternalConstant(external_reference);
            } else {
                let instance_node = Assembler::WasmInstanceDataParameter();
                let limit_address_offset = match type_ {
                    AllocationType::kYoung => 0, //WasmTrustedInstanceData::kNewAllocationLimitAddressOffset,
                    AllocationType::kOld => 0,   //WasmTrustedInstanceData::kOldAllocationLimitAddressOffset,
                };
                limit_address = Assembler::Load(
                    instance_node,
                    LoadOp::TaggedBase(),
                    MemoryRepresentation::UintPtr(),
                    limit_address_offset,
                );
            }
            limit_address
        }
    }
}