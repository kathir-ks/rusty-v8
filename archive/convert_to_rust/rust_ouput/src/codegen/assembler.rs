// Converted from V8 C++ source files:
// Header: assembler.h
// Implementation: assembler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod memory {
        pub type Address = usize;
    }
}

pub mod codegen {
    pub mod code_comments {
        pub struct CodeCommentsWriter {}
        impl CodeCommentsWriter {
            pub fn Add(&mut self, _offset: i32, _comment: String) {}
            pub fn Emit(&mut self, _assembler: &mut AssemblerBase) {}
            pub fn section_size(&self) -> i32 {0}
            pub fn entry_count(&self) -> i32 {0}
        }
    }
    pub mod cpu_features {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum CpuFeature {
            SSE2,
            SSE3,
            SSSE3,
            SSE4_1,
            SSE4_2,
            AVX,
            F16C,
            RDTSCP,
            MOVBE,
            POPCNT,
            LZCNT,
            TZCNT,
            ERMS,
            MPX,
            AVX2,
            AVX512F,
            AVX512DQ,
            AVX512BW,
            AVX512VL,
            AVX512IFMA,
            AVX512VBMI,
            AVX512VNNI,
            AVX512BF16,
            AVX512VBMI2,
            AVX512VP2INTERSECT,
            AMX_TILE,
            AMX_INT8,
            AMX_BF16,
            FMA3,
            BMI1,
            BMI2,
            SAES,
            AESNI,
            TNI,
            VNNI,
            BF16,
            WBNOINVD,
            RDPID,
            CLZERO,
            SMX,
            Serialize,
            CodeSerialization,
            WasmSimd128,
            CETSS,
            ARMv7,
            ARMv8,
            NEON,
            VFP3,
            IDIV,
            Atomics,
            Mips,
            Mips64,
            FPU,
            Msa,
            LoongArch,
            RiscV,
        }

        pub struct CpuFeatures {}
        impl CpuFeatures {
            pub fn IsSupported(_f: CpuFeature) -> bool {
                true
            }
        }

    }
    pub mod external_reference {
        pub struct ExternalReference {}
    }
    pub mod label {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Label {
            id: usize,
            bound: bool,
        }

        impl Label {
            pub fn new(id: usize) -> Self {
                Label { id, bound: false }
            }

            pub fn bind(&mut self) {
                self.bound = true;
            }

            pub fn is_bound(&self) -> bool {
                self.bound
            }
        }
    }
    pub mod reglist {
        pub struct RegList {}
    }
    pub mod reloc_info {
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum Mode {
            NoInfo,
            Literal,
            ConstPool,
            CodeTarget,
            EmbeddedObject,
            ExternalReference,
            InternalReference,
            OffHeapTarget,
            RuntimeEntry,
            DebugBreakSlot,
            SourcePosition,
            None,
            DeoptId,
            DeoptReason,
            DeoptScriptOffset,
            DeoptInliningId,
            DeoptNodeId,
            GcSafePoint,
            UnwindingEntry,
            WasmImportCallTarget,
            ForSerialize,
            WasmDataSectionOffset,
            RelativeCodeTarget,
            ModeMask
        }
        impl Mode {
            pub fn IsNoInfo(mode: Mode) -> bool {
                mode == Mode::NoInfo
            }
            pub fn IsOnlyForSerializer(mode: Mode) -> bool {
                mode == Mode::ForSerialize
            }
        }

        pub struct RelocInfoWriter {}

        impl RelocInfoWriter {
            pub const kMaxSize: usize = 16;
        }
    }
}

pub mod common {
    pub mod globals {
        pub type Address = usize;
        pub const KB: i32 = 1024;
    }
}

pub mod deoptimizer {
    pub mod deoptimize_reason {
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum DeoptimizeReason {
            Generic,
        }
    }
}

pub mod flags {
    pub mod flags {
        pub struct Flag<T> {
            value: T,
            comment: &'static str,
        }

        impl<T> Flag<T> {
            pub fn value(&self) -> &T {
                &self.value
            }
            pub fn comment(&self) -> &str {
                self.comment
            }
        }

        pub struct Flags {
            pub code_comments: Flag<bool>,
            pub debug_code: Flag<bool>,
            pub slow_debug_code: Flag<bool>,
            pub target_is_simulator: Flag<bool>,
        }

        impl Flags {
            pub const fn new() -> Self {
                Flags {
                    code_comments: Flag { value: false, comment: "" },
                    debug_code: Flag { value: false, comment: "" },
                    slow_debug_code: Flag { value: false, comment: "" },
                    target_is_simulator: Flag { value: false, comment: "" },
                }
            }
        }

        pub static mut v8_flags: Flags = Flags::new();
    }
}

pub mod handles {
    pub struct Handle<T> {
        address: usize,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Handle<T> {
        pub fn new(address: usize) -> Self {
            Handle {
                address,
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn address(&self) -> usize {
            self.address
        }
    }

    pub struct IndirectHandle<T> {
        address: usize,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> IndirectHandle<T> {
        pub fn new(address: usize) -> Self {
            IndirectHandle {
                address,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn is_null(&self) -> bool {
            self.address == 0
        }
        pub fn address(&self) -> usize {
            self.address
        }
        
        pub fn null() -> Self {
            IndirectHandle {
                address: 0,
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn hash(&self) -> usize {
            self.address
        }
        pub fn equal_to(&self) -> usize {
            self.address
        }
    }
        
    pub type Local<'a, T> = Handle<T>;
}

pub mod objects {
    pub struct HeapObject {}
    pub struct Code {}
    pub struct String {}
    pub struct Script {}
}

pub mod sandbox {
    pub mod indirect_pointer_tag {
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum IndirectPointerTag {
            kIndirectPointerNullTag,
            kCodeIndirectPointerTag,
        }
    }
}

pub mod utils {
    pub mod ostreams {
        pub struct StdoutStream {}

        impl StdoutStream {
            pub fn new() -> Self {
                StdoutStream {}
            }
        }
    }
}

pub mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! DCHECK {
            ($condition:expr) => {
                if !$condition {
                    panic!("DCHECK failed: {}", stringify!($condition));
                }
            };
        }

        #[macro_export]
        macro_rules! UNREACHABLE {
            () => {
                panic!("UNREACHABLE");
            };
        }
        #[macro_export]
        macro_rules! FATAL {
            ($message:expr) => {
                panic!("FATAL: {}", $message);
            };
        }
        #[macro_export]
        macro_rules! V8_INLINE {
            () => {};
        }
        #[macro_export]
        macro_rules! V8_NODISCARD {
            () => {};
        }
        #[macro_export]
        macro_rules! V8_WARN_UNUSED_RESULT {
            () => {};
        }
        #[macro_export]
        macro_rules! CHECK_IMPLIES {
            ($condition:expr, $message:expr) => {
                if $condition && !$message {
                    panic!("CHECK_IMPLIES failed: {} implies {}", stringify!($condition), stringify!($message));
                }
            };
        }
    }
    pub mod memory {
        use std::alloc::{GlobalAlloc, Layout, System};
        use std::ptr::NonNull;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Address(pub usize);

        impl Address {
            pub const fn from_usize(address: usize) -> Self {
                Address(address)
            }

            pub fn to_usize(&self) -> usize {
                self.0
            }

            pub const fn is_null(&self) -> bool {
                self.0 == 0
            }
        }

        impl From<usize> for Address {
            fn from(address: usize) -> Self {
                Address(address)
            }
        }

        impl From<Address> for usize {
            fn from(address: Address) -> Self {
                address.0
            }
        }
        pub fn ReadUnalignedValue<T: Copy>(address: Address) -> T {
            unsafe {
                std::ptr::read_unaligned(address.0 as *const T)
            }
        }
        pub fn WriteUnalignedValue<T: Copy>(address: Address, value: T) {
            unsafe {
                std::ptr::write_unaligned(address.0 as *mut T, value)
            }
        }
        
        #[derive(Debug)]
        pub struct OwnedVector<T> {
            ptr: NonNull<T>,
            len: usize,
            capacity: usize,
        }
        
        impl<T> OwnedVector<T> {
            pub fn NewForOverwrite(size: usize) -> Self {
                let layout = Layout::array::<T>(size).unwrap();
                
                let ptr = unsafe {
                    System.alloc(layout)
                };
        
                let ptr = match ptr {
                    std::ptr::null_mut() => std::alloc::handle_alloc_error(layout),
                    ptr => ptr,
                };

                let ptr = unsafe { NonNull::new_unchecked(ptr as *mut T) };
                
                OwnedVector {
                    ptr,
                    len: 0,
                    capacity: size,
                }
            }
            
            pub fn begin(&self) -> *mut u8 {
                self.ptr.as_ptr() as *mut u8
            }

            pub fn size(&self) -> usize {
                self.capacity
            }
        }

        impl<T> Drop for OwnedVector<T> {
            fn drop(&mut self) {
                let layout = Layout::array::<T>(self.capacity).unwrap();

                unsafe {
                    System.dealloc(self.ptr.as_ptr() as *mut u8, layout);
                }
            }
        }
    }
}

pub mod execution {
    pub struct Isolate {}
}

pub mod heap {
    pub struct Heap {}

    impl Heap {
        pub fn code_range_base(&self) -> usize {
            0
        }
    }
}

pub mod snapshot {
    pub mod embedded {
        pub struct EmbeddedData {}
    }
}

pub mod diagnostics {
    pub mod disassembler {
        use crate::execution::Isolate;
        use crate::utils::ostreams::StdoutStream;
        pub fn Decode(_isolate: *mut Isolate, _os: StdoutStream, _start: *mut u8, _end: *mut u8) {}
    }
}

pub mod src {
    pub mod codegen {
        pub mod assembler {
            use std::any::Any;
            use std::collections::HashMap;
            use std::forward_list::ForwardList;
            use std::hash::{Hash, Hasher};
            use std::mem::MaybeUninit;
            use std::ptr::null_mut;

            use crate::base::macros::{DCHECK, FATAL, UNREACHABLE, V8_INLINE, V8_NODISCARD, V8_WARN_UNUSED_RESULT, CHECK_IMPLIES};
            use crate::base::memory::{Address, OwnedVector, ReadUnalignedValue, WriteUnalignedValue};
            use crate::codegen::code_comments::CodeCommentsWriter;
            use crate::codegen::cpu_features::{CpuFeature, CpuFeatures};
            use crate::codegen::label::Label;
            use crate::codegen::reloc_info::{Mode, RelocInfoWriter};
            use crate::common::globals::{Address as GlobalAddress, KB};
            use crate::deoptimizer::deoptimize_reason::DeoptimizeReason;
            use crate::execution::Isolate;
            use crate::flags::flags::v8_flags;
            use crate::handles::{Handle, IndirectHandle};
            use crate::heap::Heap;
            use crate::objects::{Code, HeapObject, Script, String};
            use crate::sandbox::indirect_pointer_tag::IndirectPointerTag;
            use std::marker::PhantomData;
            use std::cell::RefCell;
            use std::rc::Rc;
            use std::sync::Mutex;

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub struct SourcePosition {
                script_offset: i32,
                inlining_id: i32,
            }
            impl SourcePosition {
                pub fn new(script_offset: i32, inlining_id: i32) -> Self {
                    SourcePosition { script_offset, inlining_id }
                }
                pub fn ScriptOffset(&self) -> i32 {
                    self.script_offset
                }
                
                pub fn InliningId(&self) -> i32 {
                    self.inlining_id
                }
                pub fn IsKnown(&self) -> bool {
                    true
                }
                pub fn Current() -> Self{
                    SourcePosition{script_offset: 0, inlining_id: 0}
                }
                pub fn ToString(&self) -> String {
                    String::from("SourcePosition::ToString Placeholder")
                }
                pub fn FileName(&self) -> Option<String>{
                    None
                }
            }
            pub struct JumpOptimizationInfo {
                pub stage: i32,
                pub hash_code: usize,
                pub align_pos_size: HashMap<i32, i32>,
                pub farjmp_num: i32,
                pub farjmps: Vec<JumpInfo>,
                pub optimizable: bool,
                pub may_optimizable_farjmp: HashMap<i32, JumpInfo>,
                pub label_farjmp_maps: HashMap<Label, Vec<i32>>,
            }

            impl JumpOptimizationInfo {
                pub struct JumpInfo {
                    pub pos: i32,
                    pub opcode_size: i32,
                    pub distance: i32,
                }

                pub fn new() -> Self {
                    JumpOptimizationInfo {
                        stage: 0,
                        hash_code: 0,
                        align_pos_size: HashMap::new(),
                        farjmp_num: 0,
                        farjmps: Vec::new(),
                        optimizable: false,
                        may_optimizable_farjmp: HashMap::new(),
                        label_farjmp_maps: HashMap::new(),
                    }
                }

                pub fn is_collecting(&self) -> bool {
                    self.stage == 0
                }

                pub fn is_optimizing(&self) -> bool {
                    self.stage == 1
                }

                pub fn set_optimizing(&mut self) {
                    DCHECK!(self.is_optimizable());
                    self.stage = 1;
                }

                pub fn is_optimizable(&self) -> bool {
                    self.optimizable
                }

                pub fn set_optimizable(&mut self) {
                    DCHECK!(self.is_collecting());
                    self.optimizable = true;
                }

                pub fn MaxAlignInRange(&self, _from: i32, _to: i32) -> i32 {
                    0
                }

                pub fn Print(&self) {}
            }

            pub struct HeapNumberRequest {
                value_: f64,
                offset_: i32,
            }
            impl HeapNumberRequest {
                 pub fn new(heap_number: f64, offset: i32) -> Self{
                    HeapNumberRequest{
                        value_: heap_number,
                        offset_: offset,
                    }
                }
                pub fn heap_number(&self) -> f64 {
                    self.value_
                }
                pub fn offset(&self) -> i32 {
                    DCHECK!(self.offset_ >= 0);
                    self.offset_
                }
                pub fn set_offset(&mut self, offset: i32) {
                    DCHECK!(self.offset_ < 0);
                    self.offset_ = offset;
                    DCHECK!(self.offset_ >= 0);
                }
            }
            
            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum CodeObjectRequired {
                kNo,
                kYes,
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum BuiltinCallJumpMode {
                kAbsolute,
                kPCRelative,
                kIndirect,
                kForMksnapshot,
            }

            #[derive(Debug)]
            pub struct AssemblerOptions {
                pub record_reloc_info_for_serialization: bool,
                pub enable_root_relative_access: bool,
                pub enable_simulator_code: bool,
                pub isolate_independent_code: bool,
                pub builtin_call_jump_mode: BuiltinCallJumpMode,
                pub use_pc_relative_calls_and_jumps_for_mksnapshot: bool,
                pub code_range_base: GlobalAddress,
                pub collect_win64_unwind_info: bool,
                pub emit_code_comments: bool,
                pub is_wasm: bool,
            }
            impl AssemblerOptions {
                pub fn Default(isolate: *mut Isolate) -> Self {
                    AssemblerOptions {
                        record_reloc_info_for_serialization: false,
                        enable_root_relative_access: false,
                        enable_simulator_code: false,
                        isolate_independent_code: false,
                        builtin_call_jump_mode: BuiltinCallJumpMode::kAbsolute,
                        use_pc_relative_calls_and_jumps_for_mksnapshot: false,
                        code_range_base: GlobalAddress::from_usize(0),
                        collect_win64_unwind_info: false,
                        emit_code_comments: unsafe { v8_flags.code_comments.value() },
                        is_wasm: false,
                    }
                }
            }
            
            pub type MaybeAssemblerZone = Box<dyn Any>;

            pub trait AssemblerBuffer {
                fn start(&self) -> *mut u8;
                fn size(&self) -> i32;
                fn Grow(&mut self, new_size: i32) -> Result<(), String>;
            }
            
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum IndirectPointerTag {
                kIndirectPointerNullTag,
                kCodeIndirectPointerTag,
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct SlotDescriptor {
                indirect_pointer_tag_: IndirectPointerTag,
            }

            impl SlotDescriptor {
                pub fn contains_direct_pointer(&self) -> bool {
                    self.indirect_pointer_tag_ == IndirectPointerTag::kIndirectPointerNullTag
                }

                pub fn contains_indirect_pointer(&self) -> bool {
                    self.indirect_pointer_tag_ != IndirectPointerTag::kIndirectPointerNullTag
                }

                pub fn indirect_pointer_tag(&self) -> IndirectPointerTag {
                    DCHECK!(self.contains_indirect_pointer());
                    self.indirect_pointer_tag_
                }

                pub fn ForDirectPointerSlot() -> Self {
                    SlotDescriptor {
                        indirect_pointer_tag_: IndirectPointerTag::kIndirectPointerNullTag,
                    }
                }

                pub fn ForIndirectPointerSlot(tag: IndirectPointerTag) -> Self {
                    SlotDescriptor {
                        indirect_pointer_tag_: tag,
                    }
                }

                pub fn ForTrustedPointerSlot(tag: IndirectPointerTag) -> Self {
                    #[cfg(feature = "v8_enable_sandbox")]
                    {
                        Self::ForIndirectPointerSlot(tag)
                    }
                    #[cfg(not(feature = "v8_enable_sandbox"))]
                    {
                        Self::ForDirectPointerSlot()
                    }
                }

                pub fn ForCodePointerSlot() -> Self {
                    Self::ForTrustedPointerSlot(IndirectPointerTag::kCodeIndirectPointerTag)
                }
            }
            
            struct DefaultAssemblerBuffer {
                buffer_: OwnedVector<u8>,
            }
            impl DefaultAssemblerBuffer {
                fn new(size: i32) -> Self {
                    DefaultAssemblerBuffer {
                        buffer_: OwnedVector::NewForOverwrite(size.max(AssemblerBase::kMinimalBufferSize) as usize),
                    }
                }
            }
            impl AssemblerBuffer for DefaultAssemblerBuffer {
                fn start(&self) -> *mut u8 {
                    self.buffer_.begin()
                }

                fn size(&self) -> i32 {
                    self.buffer_.size() as i32
                }

                fn Grow(&mut self, new_size: i32) -> Result<(), String> {
                    if self.size() >= new_size {
                        return Err("Cannot grow external assembler buffer".to_string());
                    }
                    Ok(())
                }
            }
            
            struct ExternalAssemblerBufferImpl {
                start_: *mut u8,
                size_: i32,
            }
            impl AssemblerBuffer for ExternalAssemblerBufferImpl {
                fn start(&self) -> *mut u8 {
                    self.start_
                }

                fn size(&self) -> i32 {
                    self.size_
                }

                fn Grow(&mut self, new_size: i32) -> Result<(), String> {
                    Err("Cannot grow external assembler buffer".to_string())
                }
            }
            impl ExternalAssemblerBufferImpl {
                unsafe fn new(start: *mut u8, size: i32) -> Self {
                    ExternalAssemblerBufferImpl { start_: start, size_: size }
                }
            }

            pub fn ExternalAssemblerBuffer(buffer: *mut u8, size: i32) -> Box<dyn AssemblerBuffer> {
                unsafe {
                    Box::new(ExternalAssemblerBufferImpl::new(buffer, size))
                }
            }

            pub fn NewAssemblerBuffer(size: i32) -> Box<dyn AssemblerBuffer> {
                Box::new(DefaultAssemblerBuffer::new(size))
            }

            pub struct AssemblerBase {
                options_: AssemblerOptions,
                enabled_cpu_features_: u64,
                predictable_code_size_: bool,
                constant_pool_available_: bool,
                jump_optimization_info_: *mut JumpOptimizationInfo,
                buffer_: Box<dyn AssemblerBuffer>,
                buffer_start_: *mut u8,
                pc_: *mut u8,
                heap_number_requests_: Vec<HeapNumberRequest>,
                code_comments_writer_: CodeCommentsWriter,
                code_targets_: Vec<IndirectHandle<Code>>,
                embedded_objects_: Vec<IndirectHandle<HeapObject>>,
                embedded_objects_map_: HashMap<usize, usize>,
            }
            impl AssemblerBase {
                const kMinimalBufferSize: i32 = 128;
                const kDefaultBufferSize: i32 = 4 * KB;

                pub fn new(options: AssemblerOptions, buffer: Box<dyn AssemblerBuffer>) -> Self {
                    let mut assembler = AssemblerBase {
                        options_: options,
                        enabled_cpu_features_: 0,
                        predictable_code_size_: false,
                        constant_pool_available_: false,
                        jump_optimization_info_: null_mut(),
                        buffer_: buffer,
                        buffer_start_: null_mut(),
                        pc_: null_mut(),
                        heap_number_requests_: Vec::new(),
                        code_comments_writer_: CodeCommentsWriter {},
                        code_targets_: Vec::new(),
                        embedded_objects_: Vec::new(),
                        embedded_objects_map_: HashMap::new(),
                    };
                    assembler.buffer_start_ = assembler.buffer_.start();
                    assembler.pc_ = assembler.buffer_.start();
                    assembler
                }

                pub fn ReleaseBuffer(mut self) -> Box<dyn AssemblerBuffer> {
                    let buffer = std::mem::replace(&mut self.buffer_, NewAssemblerBuffer(0));
                    self.buffer_start_ = null_mut();
                    self.pc_ = null_mut();
                    buffer
                }

                pub fn options(&self) -> &AssemblerOptions {
                    &self.options_
                }
                pub fn predictable_code_size(&self) -> bool {
                    self.predictable_code_size_
                }

                pub fn set_predictable_code_size(&mut self, value: bool) {
                    self.predictable_code_size_ = value;
                }

                pub fn enabled_cpu_features(&self) -> u64 {
                    self.enabled_cpu_features_
                }

                pub fn set_enabled_cpu_features(&mut self, features: u64) {
                    self.enabled_cpu_features_ = features;
                }

                pub fn IsEnabled(&self, f: CpuFeature) -> bool {
                    (self.enabled_cpu_features_ & (1 << f as u64)) != 0
                }

                pub fn EnableCpuFeature(&mut self, f: CpuFeature) {
                    self.enabled_cpu_features_ |= 1 << (f as u64);
                }
                pub fn is_constant_pool_available(&self) -> bool {
                    false
                }

                pub fn jump_optimization_info(&mut self) -> *mut JumpOptimizationInfo {
                    self.jump_optimization_info_
                }

                pub fn set_jump_optimization_info(&mut self, jump_opt: *mut JumpOptimizationInfo) {
                    self.jump_optimization_info_ = jump_opt;
                }

                pub fn FinalizeJumpOptimizationInfo(&mut self) {}
                pub fn QuietNaN(_nan: Handle<HeapObject>) {}

                pub fn pc_offset(&self) -> i32 {
                    unsafe { self.pc_.offset_from(self.buffer_start_) as i32 }
                }
                pub fn pc_offset_for_safepoint(&self) -> i32 {
                    self.pc_offset()
                }

                pub fn buffer_start(&self) -> *mut u8 {
                    self.buffer_start_
                }

                pub fn buffer_size(&self) -> i32 {
                    self.buffer_.size()
                }

                pub fn instruction_size(&self) -> i32 {
                    self.pc_offset()
                }

                pub fn AbortedCodeGeneration(&mut self) {}

                pub fn Print(&mut self, _isolate: *mut Isolate) {}

                pub fn RecordComment(&mut self, comment: String, loc : SourcePosition) {
                    if unsafe { !v8_flags.code_comments.value() } {
                        return;
                    }
                    if self.options().emit_code_comments {
                        let mut comment_str = comment;
                        if loc.FileName().is_some(){
                            comment_str += &(" - ".to_string() + &loc.ToString());
                        }
                        self.code_comments_writer_.Add(self.pc_offset(), comment_str);
                    }
                }
                
                fn RequestHeapNumber(&mut self, request: HeapNumberRequest) {
                    let mut mutable_request = request;
                    mutable_request.set_offset(self.pc_offset());
                    self.heap_number_requests_.push(mutable_request);
                }
                fn AddCodeTarget(&mut self, target: IndirectHandle<Code>) -> i32 {
                    let current = self.code_targets_.len() as i32;
                    if current > 0 && !target.is_null()
                        && self.code_targets_.last().map(|x| x.address() == target.address()).unwrap_or(false) {
                        return current - 1;
                    } else {
                        self.code_targets_.push(target);
                        return current;
                    }
                }
                
                fn GetCodeTarget(&self, code_target_index: isize) -> IndirectHandle<Code> {
                    DCHECK!(code_target_index >= 0);
                    DCHECK!(code_target_index < self.code_targets_.len() as isize);
                    self.code_targets_[code_target_index as usize].clone()
                }
                
                fn AddEmbeddedObject(&mut self, object: IndirectHandle<HeapObject>) -> usize {
                    let current = self.embedded_objects_.len();
                    if !object.is_null() {
                        if let Some(&index) = self.embedded_objects_map_.get(&object.address()){
                            return index;
                        }
                        self.embedded_objects_map_.insert(object.address(), current);
                    }
                    self.embedded_objects_.push(object);
                    current
                }

                fn GetEmbeddedObject(&self, index: usize) -> IndirectHandle<HeapObject> {
                    DCHECK!(index < self.embedded_objects_.len());
                    self.embedded_objects_[index].clone()
                }
                
                fn ShouldRecordRelocInfo(&self, rmode: Mode) -> bool {
                    DCHECK!(rmode != Mode::NoInfo);
                    if Mode::IsOnlyForSerializer(rmode) &&
                        !self.options().record_reloc_info_for_serialization &&
                        unsafe { !v8_flags.debug_code.value() } &&
                        unsafe { !v8_flags.slow_debug_code.value() } {
                        return false;
                    }
                    return true;
                }
                
                pub fn WriteCodeComments(&mut self) -> i32 {
                    if unsafe { !v8_flags.code_comments.value() } {
                        return 0;
                    }
                    CHECK_IMPLIES!(self.code_comments_writer_.entry_count() > 0, self.options().emit_code_comments);
                    if self.code_comments_writer_.entry_count() == 0 {
                        return 0;
                    }
                    let offset = self.pc_offset();
                    self.code_comments_writer_.Emit(self);
                    let size = self.pc_offset() - offset;
                    DCHECK!(size == self.code_comments_writer_.section_size());
                    return size;
                }
                

            }
            impl Drop for AssemblerBase {
                fn drop(&mut self) {}
            }
            pub struct Assembler {
                base: AssemblerBase,
            }

            impl Assembler {
                pub fn new(options: AssemblerOptions, buffer: Box<dyn AssemblerBuffer>) -> Self {
                    Assembler {
                        base: AssemblerBase::new(options, buffer),
                    }
                }
                
                pub fn base(&mut self) -> &mut AssemblerBase {
                    &mut self.base
                }

                pub fn EnsureSpace(&mut self, _space_needed: usize) {
                }

                pub fn RecordRelocInfo(&mut self, rmode: Mode, data: i32) {
                    if self.base.ShouldRecordRelocInfo(rmode) {
                    }
                }

                pub fn RecordDeoptReason(&mut self, reason: DeoptimizeReason, node_id: u32, position: SourcePosition, id: i32) {
                    if RelocInfoWriter::kMaxSize * 2 <= Assembler::kGap as usize {
                        self.EnsureSpace(Assembler::kGap as usize);
                        DCHECK!(position.IsKnown());
                        self.RecordRelocInfo(Mode::DeoptScriptOffset, position.ScriptOffset());
                        self.RecordRelocInfo(Mode::DeoptInliningId, position.InliningId());
                    }
                    self.EnsureSpace(Assembler::kGap as usize);
                    self.RecordRelocInfo(Mode::DeoptReason, reason as i32);
                    self.RecordRelocInfo(Mode::DeoptId, id);
                    self.EnsureSpace(Assembler::kGap as usize);
                    self.RecordRelocInfo(Mode::DeoptNodeId, node_id as i32);
                }

                pub fn DataAlign(&mut self, m: i32) {
                    DCHECK!(m >= 2 && (m & (m - 1)) == 0);
                    while (self.base.pc_offset() & (m - 1)) != 0 {
                        self.db(0xcc);
                    }
                }
                pub fn db(&mut self, byte: i8){
                
                    unsafe{
                        let mut pc = self.base.pc_;
                        std::ptr::write(pc, byte as u8);
                        self.base.pc_ = pc.offset(1);
                    }
                }
                
                pub fn pc_offset(&self) -> i32 {
                    self.base.pc_offset()
                }

            }
            
            impl Drop for Assembler {
                fn drop(&mut self) {}
            }

            impl Assembler {
                const kGap: i32 = 3
