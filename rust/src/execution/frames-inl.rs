// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a translation of v8/src/execution/frames-inl.h to Rust.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem;
use std::option::Option;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

//use crate::base::memory::Memory; // Assuming Memory is a custom type in src/base
//use crate::execution::frame_constants::*; // Assuming frame_constants are defined in src/execution
//use crate::execution::frames::*; // Assuming frames are defined in src/execution
//use crate::execution::isolate::*; // Assuming Isolate is defined in src/execution
//use crate::execution::pointer_authentication::*; // Assuming PointerAuthentication is defined in src/execution
//use crate::objects::objects::*; // Assuming objects are defined in src/objects

// Mock definitions for types that are not available.
// Replace these with actual definitions when available.
pub type Address = usize;

#[derive(Debug, Copy, Clone)]
pub struct Isolate {}

#[derive(Debug, Copy, Clone)]
pub struct Tagged<T> {
    ptr: Address,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn new(ptr: Address) -> Self {
        Tagged {
            ptr,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn ptr(&self) -> Address {
        self.ptr
    }
}

impl From<Address> for Tagged<Object> {
    fn from(ptr: Address) -> Self {
        Tagged::new(ptr)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Object {}
#[derive(Debug, Copy, Clone)]
pub struct GcSafeCode {}
#[derive(Debug, Copy, Clone)]
pub struct SafepointEntry {}
#[derive(Debug, Copy, Clone)]
pub struct MaglevSafepointEntry {}
#[derive(Debug, Copy, Clone)]
pub struct Name {}
#[derive(Debug, Copy, Clone)]
pub struct HeapObject {}
#[derive(Debug, Copy, Clone)]
pub struct JSFunction {}
#[derive(Debug, Copy, Clone)]
pub struct FunctionTemplateInfo {}
#[derive(Debug, Copy, Clone)]
pub struct FeedbackVector {}

pub fn IsJSFunction(_obj: Tagged<Object>) -> bool {
    true // dummy
}

pub fn IsFunctionTemplateInfo(_obj: Tagged<Object>) -> bool {
    true // dummy
}

pub fn IsUndefined(_obj: Tagged<Object>, _isolate: &Isolate) -> bool {
    false // dummy
}

#[derive(Debug, Copy, Clone)]
pub struct FullObjectSlot {
    address: Address,
}

impl FullObjectSlot {
    pub fn store(&self, _obj: Tagged<HeapObject>) {}

    pub fn read(&self) -> Tagged<Object> {
        Tagged::new(self.address) // placeholder
    }
}

pub fn Cast<T>(_obj: Tagged<Object>) -> Tagged<T> {
    Tagged {
        ptr: 0, // placeholder
        phantom: std::marker::PhantomData,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PointerAuthentication {}
impl PointerAuthentication {
    pub fn StripPAC(address: Address) -> Address {
        address
    }

    pub fn AuthenticatePC(pc_address: *mut Address, _size: usize) -> Address {
        unsafe { *pc_address }
    }
}

const kSystemPointerSize: usize = 8;

pub mod StackFrame {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum Type {
        ENTRY,
        CONSTRUCT,
        FAST_CONSTRUCT,
        EXIT,
        BUILTIN_EXIT,
        API_CALLBACK_EXIT,
        API_ACCESSOR_EXIT,
        JAVA_SCRIPT,
        WASM,
        WASM_TO_JS,
        JS_TO_WASM,
        WASM_SEGMENT_START,
        WASM_INTERPRETER_ENTRY,
        STUB,
        INTERNAL,
        IRREXP,
        BUILTIN_CONTINUATION,
    }
    pub fn TypeToMarker(_type: Type) -> usize {
        0 //dummy
    }
    pub fn IsJavaScript(type_: Type) -> bool {
        match type_ {
            Type::JAVA_SCRIPT => true,
            _ => false,
        }
    }
}

pub mod StackHandlerConstants {
    pub const kNextOffset: usize = 8;
}

pub mod StandardFrameConstants {
    pub const kCallerFPOffset: usize = 8;
    pub const kCallerPCOffset: usize = 16;
    pub const kFunctionOffset: usize = 24;
}

pub mod InterpreterFrameConstants {
    pub const kFeedbackVectorFromFp: usize = 32;
}

pub mod TypedFrameConstants {
    pub const kFrameTypeOffset: usize = 0;
}

pub mod BuiltinExitFrameConstants {
    pub const kReceiverOffset: usize = 0;
    pub const kArgcOffset: usize = 8;
    pub const kTargetOffset: usize = 16;
    pub const kNewTargetOffset: usize = 24;
}

pub mod ApiCallbackExitFrameConstants {
    pub const kContextOffset: usize = 0;
    pub const kTargetOffset: usize = 8;
    pub const kReceiverOffset: usize = 16;
    pub const kFCIArgcOffset: usize = 24;
    pub const kNewTargetOffset: usize = 32;
    pub const kFirstArgumentOffset: usize = 40;
}

pub mod ApiAccessorExitFrameConstants {
    pub const kPropertyNameOffset: usize = 0;
    pub const kReceiverOffset: usize = 8;
    pub const kHolderOffset: usize = 16;
}

mod base {
    pub struct Memory<T> {
        address: Address,
        phantom: std::marker::PhantomData<T>,
    }

    impl<T> Memory<T> {
        pub fn new(address: Address) -> Self {
            Memory {
                address,
                phantom: std::marker::PhantomData,
            }
        }

        pub fn read(&self) -> T
        where
            T: Copy,
        {
            unsafe { (self.address as *const T).read_unaligned() }
        }
    }
    impl<T> std::ops::Add<usize> for Memory<T> {
        type Output = Self;

        fn add(self, offset: usize) -> Self {
            Memory {
                address: self.address + offset,
                phantom: self.phantom,
            }
        }
    }

    impl<T> Memory<T>
    where
        T: Copy,
    {
        pub fn write(&mut self, value: T) {
            unsafe {
                (self.address as *mut T).write_unaligned(value);
            }
        }
    }

    impl<T> From<Address> for Memory<T> {
        fn from(address: Address) -> Self {
            Memory {
                address,
                phantom: std::marker::PhantomData,
            }
        }
    }
}

pub struct InnerPointerToCodeCache {
    isolate_: *mut Isolate, // Using raw pointer as in the original C++
    cache_: [InnerPointerToCodeCacheEntry; kInnerPointerToCodeCacheSize],
}

impl InnerPointerToCodeCache {
    pub struct InnerPointerToCodeCacheEntry {
        inner_pointer: Address,
        code: Option<Tagged<GcSafeCode>>,
        safepoint_entry: SafepointUnion,
    }

    union SafepointUnion {
        safepoint_entry: SafepointEntry,
        maglev_safepoint_entry: MaglevSafepointEntry,
    }

    const kInnerPointerToCodeCacheSize: usize = 1024;

    pub fn new(isolate: *mut Isolate) -> Self {
        let mut cache_: [InnerPointerToCodeCacheEntry;
            InnerPointerToCodeCache::kInnerPointerToCodeCacheSize] = unsafe {
            std::mem::zeroed()
        };
        for i in 0..InnerPointerToCodeCache::kInnerPointerToCodeCacheSize {
            cache_[i] = InnerPointerToCodeCacheEntry {
                inner_pointer: 0,
                code: None,
                safepoint_entry: SafepointUnion {
                    safepoint_entry: SafepointEntry {},
                },
            }
        }
        InnerPointerToCodeCache {
            isolate_: isolate,
            cache_: cache_,
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            ptr::write_bytes(
                self.cache_.as_mut_ptr() as *mut u8,
                0,
                mem::size_of_val(&self.cache_),
            );
        }
    }

    pub fn get_cache_entry(&mut self, inner_pointer: Address) -> *mut InnerPointerToCodeCacheEntry {
        // Placeholder logic
        let index = (inner_pointer % Self::kInnerPointerToCodeCacheSize as usize) as usize;
        &mut self.cache_[index] as *mut InnerPointerToCodeCacheEntry
    }

    fn cache(&mut self, index: usize) -> *mut InnerPointerToCodeCacheEntry {
        &mut self.cache_[index] as *mut InnerPointerToCodeCacheEntry
    }
}

pub struct StackHandler {
    // Fields representing the StackHandler
    address: Address,
}

impl StackHandler {
    pub fn address(&self) -> Address {
        self.address
    }

    pub fn next(&self) -> *mut StackHandler {
        let offset = StackHandlerConstants::kNextOffset;
        Self::from_address(base::Memory::<Address>::from(self.address() + offset).read())
    }

    pub fn next_address(&self) -> Address {
        base::Memory::<Address>::from(self.address() + StackHandlerConstants::kNextOffset).read()
    }

    pub fn from_address(address: Address) -> *mut StackHandler {
        address as *mut StackHandler
    }
}

pub struct StackFrame<'a> {
    iterator_: &'a mut StackFrameIteratorBase<'a>,
    isolate_: *mut Isolate,
    return_address_location_resolver_: Option<fn(uintptr_t) -> uintptr_t>,
}

type uintptr_t = usize;

impl<'a> StackFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        StackFrame {
            iterator_: iterator,
            isolate_: iterator.isolate(),
            return_address_location_resolver_: None,
        }
    }

    pub fn top_handler(&self) -> *mut StackHandler {
        self.iterator_.handler()
    }

    pub fn pc(&self) -> Address {
        self.read_pc(self.pc_address())
    }

    pub fn unauthenticated_pc(&self) -> Address {
        self.unauthenticated_pc_addr(self.pc_address())
    }

    // static
    pub fn unauthenticated_pc_addr(pc_address: Address) -> Address {
        PointerAuthentication::StripPAC(pc_address)
    }

    pub fn maybe_unauthenticated_pc(&self) -> Address {
        if !self.in_fast_c_call() && !self.is_profiler_entry_frame() && !self.is_stack_exit_frame() {
            // Here the pc_address() is on the stack and properly authenticated.
            self.pc()
        } else {
            // For fast C calls pc_address() points into IsolateData and the pc in there
            // is unauthenticated. For the profiler, the pc_address of the first visited
            // frame is also not written by a call instruction.
            // For wasm stacks, the exit frame's pc is stored in the jump buffer
            // unsigned.
            self.unauthenticated_pc()
        }
    }

    pub fn read_pc(&self, pc_address: Address) -> Address {
        unsafe {
            PointerAuthentication::AuthenticatePC(pc_address as *mut Address, kSystemPointerSize)
        }
    }

    pub fn resolve_return_address_location(&self, pc_address: Address) -> Address {
        match self.return_address_location_resolver_ {
            Some(resolver) => resolver(pc_address),
            None => pc_address,
        }
    }

    // The following methods need to be overriden in subclasses
    pub fn pc_address(&self) -> Address {
        0 // Placeholder
    }

    pub fn in_fast_c_call(&self) -> bool {
        false // Placeholder
    }

    pub fn is_profiler_entry_frame(&self) -> bool {
        false // Placeholder
    }

    pub fn is_stack_exit_frame(&self) -> bool {
        false // Placeholder
    }

    pub fn fp(&self) -> Address {
        0 // Placeholder
    }
}

pub struct TypedFrame<'a> {
    common_frame: CommonFrame<'a>,
}

impl<'a> TypedFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        TypedFrame {
            common_frame: CommonFrame::new(iterator),
        }
    }
}

pub struct CommonFrameWithJSLinkage<'a> {
    common_frame: CommonFrame<'a>,
}

impl<'a> CommonFrameWithJSLinkage<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        CommonFrameWithJSLinkage {
            common_frame: CommonFrame::new(iterator),
        }
    }

    pub fn is_construct_frame(fp: Address) -> bool {
        let frame_type = base::Memory::<usize>::from(fp + TypedFrameConstants::kFrameTypeOffset).read();
        frame_type == StackFrame::TypeToMarker(StackFrame::Type::CONSTRUCT) as usize ||
        frame_type == StackFrame::TypeToMarker(StackFrame::Type::FAST_CONSTRUCT) as usize
    }
}

pub struct TypedFrameWithJSLinkage<'a> {
    common_frame_with_js_linkage: CommonFrameWithJSLinkage<'a>,
}

impl<'a> TypedFrameWithJSLinkage<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        TypedFrameWithJSLinkage {
            common_frame_with_js_linkage: CommonFrameWithJSLinkage::new(iterator),
        }
    }
}

pub struct NativeFrame<'a> {
    typed_frame: TypedFrame<'a>,
}

impl<'a> NativeFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        NativeFrame {
            typed_frame: TypedFrame::new(iterator),
        }
    }
}

pub struct EntryFrame<'a> {
    typed_frame: TypedFrame<'a>,
}

impl<'a> EntryFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        EntryFrame {
            typed_frame: TypedFrame::new(iterator),
        }
    }
}

pub struct ConstructEntryFrame<'a> {
    entry_frame: EntryFrame<'a>,
}

impl<'a> ConstructEntryFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        ConstructEntryFrame {
            entry_frame: EntryFrame::new(iterator),
        }
    }
}

pub struct ExitFrame<'a> {
    typed_frame: TypedFrame<'a>,
}

impl<'a> ExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        ExitFrame {
            typed_frame: TypedFrame::new(iterator),
        }
    }

    fn fp(&self) -> Address {
        self.typed_frame.common_frame.iterator_.frame().fp()
    }
}

pub struct BuiltinExitFrame<'a> {
    exit_frame: ExitFrame<'a>,
}

impl<'a> BuiltinExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        BuiltinExitFrame {
            exit_frame: ExitFrame::new(iterator),
        }
    }

    pub fn receiver_slot_object(&self) -> Tagged<Object> {
        Tagged::<Object>::from(base::Memory::<Address>::from(self.exit_frame.fp() + BuiltinExitFrameConstants::kReceiverOffset).read())
    }

    pub fn argc_slot_object(&self) -> Tagged<Object> {
        Tagged::<Object>::from(base::Memory::<Address>::from(self.exit_frame.fp() + BuiltinExitFrameConstants::kArgcOffset).read())
    }

    pub fn target_slot_object(&self) -> Tagged<Object> {
        Tagged::<Object>::from(base::Memory::<Address>::from(self.exit_frame.fp() + BuiltinExitFrameConstants::kTargetOffset).read())
    }

    pub fn new_target_slot_object(&self) -> Tagged<Object> {
        Tagged::<Object>::from(base::Memory::<Address>::from(self.exit_frame.fp() + BuiltinExitFrameConstants::kNewTargetOffset).read())
    }
}

pub struct ApiCallbackExitFrame<'a> {
    exit_frame: ExitFrame<'a>,
}

impl<'a> ApiCallbackExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        ApiCallbackExitFrame {
            exit_frame: ExitFrame::new(iterator),
        }
    }

    pub fn context(&self) -> Tagged<Object> {
        Tagged::<Object>::from(base::Memory::<Address>::from(self.exit_frame.fp() + ApiCallbackExitFrameConstants::kContextOffset).read())
    }

    pub fn target_slot(&self) -> FullObjectSlot {
        FullObjectSlot {
            address: self.exit_frame.fp() + ApiCallbackExitFrameConstants::kTargetOffset,
        }
    }

    pub fn receiver(&self) -> Tagged<Object> {
        Tagged::<Object>::from(base::Memory::<Address>::from(self.exit_frame.fp() + ApiCallbackExitFrameConstants::kReceiverOffset).read())
    }

    pub fn target(&self) -> Tagged<HeapObject> {
        let function = self.target_slot().read();
        assert!(IsJSFunction(function) || IsFunctionTemplateInfo(function));
        Cast::<HeapObject>(function)
    }

    pub fn set_target(&self, function: Tagged<HeapObject>) {
        assert!(IsJSFunction(function) || IsFunctionTemplateInfo(function));
        self.target_slot().store(function);
    }

    pub fn compute_parameters_count(&self) -> i32 {
        let argc = base::Memory::<Address>::from(self.exit_frame.fp() + ApiCallbackExitFrameConstants::kFCIArgcOffset).read() as i32;
        assert!(argc >= 0);
        argc
    }

    pub fn get_parameter(&self, i: i32) -> Tagged<Object> {
        assert!(i >= 0 && i < self.compute_parameters_count());
        let offset = ApiCallbackExitFrameConstants::kFirstArgumentOffset + (i as usize) * kSystemPointerSize;
        Tagged::<Object>::from(base::Memory::<Address>::from(self.exit_frame.fp() + offset).read())
    }

    pub fn is_constructor(&self) -> bool {
        let new_context = Tagged::<Object>::from(base::Memory::<Address>::from(
            self.exit_frame.fp() + ApiCallbackExitFrameConstants::kNewTargetOffset).read());
        !IsUndefined(new_context, self.exit_frame.typed_frame.common_frame.iterator_.isolate_ref())
    }
}

pub struct ApiAccessorExitFrame<'a> {
    exit_frame: ExitFrame<'a>,
}

impl<'a> ApiAccessorExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        ApiAccessorExitFrame {
            exit_frame: ExitFrame::new(iterator),
        }
    }

    pub fn property_name_slot(&self) -> FullObjectSlot {
        FullObjectSlot {
            address: self.exit_frame.fp() + ApiAccessorExitFrameConstants::kPropertyNameOffset,
        }
    }

    pub fn receiver_slot(&self) -> FullObjectSlot {
        FullObjectSlot {
            address: self.exit_frame.fp() + ApiAccessorExitFrameConstants::kReceiverOffset,
        }
    }

    pub fn holder_slot(&self) -> FullObjectSlot {
        FullObjectSlot {
            address: self.exit_frame.fp() + ApiAccessorExitFrameConstants::kHolderOffset,
        }
    }

    pub fn property_name(&self) -> Tagged<Name> {
        Cast::<Name>(self.property_name_slot().read())
    }

    pub fn receiver(&self) -> Tagged<Object> {
        self.receiver_slot().read()
    }

    pub fn holder(&self) -> Tagged<Object> {
        self.holder_slot().read()
    }
}

pub struct CommonFrame<'a> {
    stack_frame: StackFrame<'a>,
}

impl<'a> CommonFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        CommonFrame {
            stack_frame: StackFrame::new(iterator),
        }
    }

    pub fn get_expression(&self, index: i32) -> Tagged<Object> {
        Tagged::<Object>::from(base::Memory::<Address>::from(self.get_expression_address(index)).read())
    }

    pub fn set_expression(&self, index: i32, value: Tagged<Object>) {
        base::Memory::<Address>::from(self.get_expression_address(index)).write(value.ptr());
    }

    pub fn caller_fp(&self) -> Address {
        base::Memory::<Address>::from(self.fp() + StandardFrameConstants::kCallerFPOffset).read()
    }

    pub fn caller_pc(&self) -> Address {
        unsafe {
            self.stack_frame.read_pc((self.fp() + StandardFrameConstants::kCallerPCOffset) as *mut Address)
        }
    }

    // The following methods need to be overriden in subclasses
    pub fn get_expression_address(&self, _index: i32) -> Address {
        0 // Placeholder
    }

    fn fp(&self) -> Address {
        self.stack_frame.fp()
    }

    fn isolate_ref(&self) -> *mut Isolate {
        self.stack_frame.isolate_
    }
}

pub struct JavaScriptFrame<'a> {
    common_frame_with_js_linkage: CommonFrameWithJSLinkage<'a>,
}

impl<'a> JavaScriptFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        JavaScriptFrame {
            common_frame_with_js_linkage: CommonFrameWithJSLinkage::new(iterator),
        }
    }
}

impl<'a> CommonFrameWithJSLinkage<'a> {
    fn get_parameter_slot(&self, index: i32) -> Address {
        assert!(index <= -1);
        assert!(index < std::cmp::max(self.get_actual_argument_count(), self.compute_parameters_count()));
        let parameter_offset = (index + 1) as usize * kSystemPointerSize;
        self.caller_sp() + parameter_offset
    }

    fn get_actual_argument_count(&self) -> i32 {
        0 // Placeholder
    }

    // Placeholder
    fn compute_parameters_count(&self) -> i32 {
        0
    }

    // Placeholder
    fn caller_sp(&self) -> Address {
        0
    }
}

impl<'a> JavaScriptFrame<'a> {
    pub fn set_receiver(&self, value: Tagged<Object>) {
        base::Memory::<Address>::from(self.common_frame_with_js_linkage.get_parameter_slot(-1)).write(value.ptr());
    }
}

pub struct UnoptimizedJSFrame<'a> {
    java_script_frame: JavaScriptFrame<'a>,
}

impl<'a> UnoptimizedJSFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        UnoptimizedJSFrame {
            java_script_frame: JavaScriptFrame::new(iterator),
        }
    }
}

impl<'a> UnoptimizedJSFrame<'a> {
    pub fn set_feedback_vector(&self, feedback_vector: Tagged<FeedbackVector>) {
        let offset = InterpreterFrameConstants::kFeedbackVectorFromFp;
        base::Memory::<Address>::from(self.fp() + offset).write(feedback_vector.ptr());
    }
}

impl<'a> JavaScriptFrame<'a> {
    pub fn function_slot_object(&self) -> Tagged<Object> {
        let offset = StandardFrameConstants::kFunctionOffset;
        Tagged::<Object>::from(base::Memory::<Address>::from(self.fp() + offset).read())
    }

    //Placeholder
    fn fp(&self) -> Address {
        0
    }
}

pub struct TurbofanStubWithContextFrame<'a> {
    common_frame: CommonFrame<'a>,
}

impl<'a> TurbofanStubWithContextFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        TurbofanStubWithContextFrame {
            common_frame: CommonFrame::new(iterator),
        }
    }
}

pub struct StubFrame<'a> {
    typed_frame: TypedFrame<'a>,
}

impl<'a> StubFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        StubFrame {
            typed_frame: TypedFrame::new(iterator),
        }
    }
}

pub struct OptimizedJSFrame<'a> {
    java_script_frame: JavaScriptFrame<'a>,
}

impl<'a> OptimizedJSFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        OptimizedJSFrame {
            java_script_frame: JavaScriptFrame::new(iterator),
        }
    }
}

pub struct InterpretedFrame<'a> {
    unoptimized_js_frame: UnoptimizedJSFrame<'a>,
}

impl<'a> InterpretedFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        InterpretedFrame {
            unoptimized_js_frame: UnoptimizedJSFrame::new(iterator),
        }
    }
}

pub struct BaselineFrame<'a> {
    unoptimized_js_frame: UnoptimizedJSFrame<'a>,
}

impl<'a> BaselineFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        BaselineFrame {
            unoptimized_js_frame: UnoptimizedJSFrame::new(iterator),
        }
    }
}

pub struct MaglevFrame<'a> {
    optimized_js_frame: OptimizedJSFrame<'a>,
}

impl<'a> MaglevFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        MaglevFrame {
            optimized_js_frame: OptimizedJSFrame::new(iterator),
        }
    }
}

pub struct TurbofanJSFrame<'a> {
    optimized_js_frame: OptimizedJSFrame<'a>,
}

impl<'a> TurbofanJSFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        TurbofanJSFrame {
            optimized_js_frame: OptimizedJSFrame::new(iterator),
        }
    }
}

pub struct BuiltinFrame<'a> {
    typed_frame_with_js_linkage: TypedFrameWithJSLinkage<'a>,
}

impl<'a> BuiltinFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        BuiltinFrame {
            typed_frame_with_js_linkage: TypedFrameWithJSLinkage::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct WasmFrame<'a> {
    typed_frame: TypedFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> WasmFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        WasmFrame {
            typed_frame: TypedFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct WasmSegmentStartFrame<'a> {
    wasm_frame: WasmFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> WasmSegmentStartFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        WasmSegmentStartFrame {
            wasm_frame: WasmFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct WasmExitFrame<'a> {
    wasm_frame: WasmFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> WasmExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        WasmExitFrame {
            wasm_frame: WasmFrame::new(iterator),
        }
    }
}

#[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_DRUMBRAKE))]
pub struct WasmInterpreterEntryFrame<'a> {
    wasm_frame: WasmFrame<'a>,
}

#[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_DRUMBRAKE))]
impl<'a> WasmInterpreterEntryFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        WasmInterpreterEntryFrame {
            wasm_frame: WasmFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct WasmDebugBreakFrame<'a> {
    typed_frame: TypedFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> WasmDebugBreakFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        WasmDebugBreakFrame {
            typed_frame: TypedFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct WasmToJsFrame<'a> {
    wasm_frame: WasmFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> WasmToJsFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        WasmToJsFrame {
            wasm_frame: WasmFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct WasmToJsFunctionFrame<'a> {
    typed_frame: TypedFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> WasmToJsFunctionFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        WasmToJsFunctionFrame {
            typed_frame: TypedFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct JsToWasmFrame<'a> {
    stub_frame: StubFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> JsToWasmFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        JsToWasmFrame {
            stub_frame: StubFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct StackSwitchFrame<'a> {
    exit_frame: ExitFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> StackSwitchFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        StackSwitchFrame {
            exit_frame: ExitFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct CWasmEntryFrame<'a> {
    stub_frame: StubFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> CWasmEntryFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
        CWasmEntryFrame {
            stub_frame: StubFrame::new(iterator),
        }
    }
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
pub struct WasmLiftoffSetupFrame<'a> {
    typed_frame: TypedFrame<'a>,
}

#[cfg(V8_ENABLE_WEBASSEMBLY)]
impl<'a> WasmLiftoffSetupFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'