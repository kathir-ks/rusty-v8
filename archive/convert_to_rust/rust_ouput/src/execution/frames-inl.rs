// Converted from V8 C++ source files:
// Header: frames-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod frames_inl {
use std::cell::RefCell;
use std::sync::atomic::AtomicU16;
use std::ptr;
use crate::execution::frame_constants::*;
use crate::execution::frames::*;
use crate::execution::isolate::*;
use crate::objects::objects_inl::*;
use crate::base::memory::Memory;
use crate::v8::extension::String_ExternalOneByteStringResource;
use crate::v8::Local;
use std::mem;
use std::sync::Mutex;
use std::sync::Arc;
use std::ops::Deref;
use std::ops::DerefMut;

  pub struct InnerPointerToCodeCacheEntry {
    pub inner_pointer: Address,
    pub code: Option<Tagged<GcSafeCode>>,
    pub safepoint_entry: SafepointEntry,
    pub maglev_safepoint_entry: MaglevSafepointEntry,
  }

  impl InnerPointerToCodeCacheEntry {
      pub fn new() -> Self {
        InnerPointerToCodeCacheEntry {
          inner_pointer: Address {},
          code: None,
          safepoint_entry: SafepointEntry {},
          maglev_safepoint_entry: MaglevSafepointEntry {},
        }
      }
  }

  pub struct InnerPointerToCodeCache<'a> {
    isolate_: &'a IsolateData,
    cache_: [InnerPointerToCodeCacheEntry; InnerPointerToCodeCache::kInnerPointerToCodeCacheSize],
    cache_mutex: Mutex<()>,
  }

  impl<'a> InnerPointerToCodeCache<'a> {
    const kInnerPointerToCodeCacheSize: usize = 1024;

    pub fn new(isolate: &'a IsolateData) -> Self {
      let mut cache_: [InnerPointerToCodeCacheEntry; InnerPointerToCodeCache::kInnerPointerToCodeCacheSize] = unsafe {
            mem::zeroed()
      };
      for i in 0..InnerPointerToCodeCache::kInnerPointerToCodeCacheSize {
        cache_[i] = InnerPointerToCodeCacheEntry::new();
      }

      let mut result = InnerPointerToCodeCache {
        isolate_: isolate,
        cache_: cache_,
        cache_mutex: Mutex::new(()),
      };
      result.Flush();
      result
    }

    pub fn Flush(&mut self) {
      let _lock = self.cache_mutex.lock().unwrap();
      for i in 0..InnerPointerToCodeCache::kInnerPointerToCodeCacheSize {
        self.cache_[i] = InnerPointerToCodeCacheEntry::new();
      }
    }

    fn cache(&mut self, index: usize) -> &mut InnerPointerToCodeCacheEntry {
      &mut self.cache_[index]
    }

    pub fn GetCacheEntry(&mut self, inner_pointer: Address) -> Option<&mut InnerPointerToCodeCacheEntry> {
      let _lock = self.cache_mutex.lock().unwrap();
      let index = (inner_pointer.0 as usize) % InnerPointerToCodeCache::kInnerPointerToCodeCacheSize;
      let entry = self.cache(index);
      if entry.inner_pointer.0 == inner_pointer.0 {
          Some(entry)
      } else {
        None
      }
    }
  }

  impl<'a> Drop for InnerPointerToCodeCache<'a> {
      fn drop(&mut self) {
      }
  }

  impl StackHandler {
    pub fn address(&self) -> Address {
      Address {0: self as *const StackHandler as usize}
    }

    pub fn next(&self) -> *mut StackHandler {
      let offset = StackHandlerConstants::kNextOffset;
      let address = self.address().0 + offset as usize;
      address as *mut StackHandler
    }

    pub fn next_address(&self) -> Address {
      let address = self.address().0 + StackHandlerConstants::kNextOffset as usize;
      Address{0: address}
    }

    pub fn FromAddress(address: Address) -> *mut StackHandler {
      address.0 as *mut StackHandler
    }
  }

  impl<'a> StackFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      StackFrame {
        iterator_: iterator,
        isolate_: iterator.isolate_ as *mut IsolateData,
      }
    }

    pub fn top_handler(&self) -> *mut StackHandler {
      self.iterator_.handler_ as *mut StackHandler
    }

    pub fn pc(&self) -> Address {
      self.ReadPC(self.pc_address())
    }

    pub fn unauthenticated_pc(&self) -> Address {
        self.unauthenticated_pc_internal(self.pc_address())
    }

    // static
    pub fn unauthenticated_pc_internal(pc_address: *mut Address) -> Address {
      let pc_address_value = unsafe { *pc_address };
      PointerAuthentication::StripPAC(pc_address_value)
    }

    pub fn maybe_unauthenticated_pc(&self) -> Address {
      if !self.InFastCCall() && !self.is_profiler_entry_frame() && !self.is_stack_exit_frame() {
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

    pub fn ReadPC(&self, pc_address: *mut Address) -> Address {
      let pc_address_value = unsafe { *pc_address };
      PointerAuthentication::AuthenticatePC(pc_address, kSystemPointerSize)
    }

    pub fn ResolveReturnAddressLocation(&self, pc_address: *mut Address) -> *mut Address {
      if self.return_address_location_resolver_.is_none() {
        pc_address
      } else {
        let resolver = self.return_address_location_resolver_.unwrap();
        resolver(pc_address as usize as u64) as *mut Address
      }
    }
  }

  impl<'a> TypedFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      TypedFrame {
        common_frame: CommonFrame::new(iterator)
      }
    }
  }

  impl<'a> CommonFrameWithJSLinkage<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      CommonFrameWithJSLinkage {
        common_frame: CommonFrame::new(iterator)
      }
    }
  }

  impl<'a> TypedFrameWithJSLinkage<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      TypedFrameWithJSLinkage {
        common_frame_with_js_linkage: CommonFrameWithJSLinkage::new(iterator)
      }
    }
  }

  impl<'a> NativeFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      NativeFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  impl<'a> EntryFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      EntryFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  impl<'a> ConstructEntryFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      ConstructEntryFrame {
        entry_frame: EntryFrame::new(iterator)
      }
    }
  }

  impl<'a> ExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      ExitFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  impl<'a> BuiltinExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      BuiltinExitFrame {
        exit_frame: ExitFrame::new(iterator)
      }
    }

    pub fn receiver_slot_object(&self) -> Tagged<Object> {
      let address = self.fp() + BuiltinExitFrameConstants::kReceiverOffset as usize;
      let receiver_ptr = address as *mut Address;
      let receiver_address = unsafe { *receiver_ptr };
      Tagged::<Object>::new(receiver_address)
    }

    pub fn argc_slot_object(&self) -> Tagged<Object> {
      let address = self.fp() + BuiltinExitFrameConstants::kArgcOffset as usize;
      let argc_ptr = address as *mut Address;
      let argc_address = unsafe { *argc_ptr };
      Tagged::<Object>::new(argc_address)
    }

    pub fn target_slot_object(&self) -> Tagged<Object> {
      let address = self.fp() + BuiltinExitFrameConstants::kTargetOffset as usize;
      let target_ptr = address as *mut Address;
      let target_address = unsafe { *target_ptr };
      Tagged::<Object>::new(target_address)
    }

    pub fn new_target_slot_object(&self) -> Tagged<Object> {
      let address = self.fp() + BuiltinExitFrameConstants::kNewTargetOffset as usize;
      let new_target_ptr = address as *mut Address;
      let new_target_address = unsafe { *new_target_ptr };
      Tagged::<Object>::new(new_target_address)
    }
  }

  impl<'a> ApiCallbackExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      ApiCallbackExitFrame {
        exit_frame: ExitFrame::new(iterator)
      }
    }

    pub fn context(&self) -> Tagged<Object> {
      let address = self.fp() + ApiCallbackExitFrameConstants::kContextOffset as usize;
      let context_ptr = address as *mut Address;
      let context_address = unsafe { *context_ptr };
      Tagged::<Object>::new(context_address)
    }

    pub fn target_slot(&self) -> FullObjectSlot {
      FullObjectSlot {
        address: self.fp() + ApiCallbackExitFrameConstants::kTargetOffset as usize
      }
    }

    pub fn receiver(&self) -> Tagged<Object> {
      let address = self.fp() + ApiCallbackExitFrameConstants::kReceiverOffset as usize;
      let receiver_ptr = address as *mut Address;
      let receiver_address = unsafe { *receiver_ptr };
      Tagged::<Object>::new(receiver_address)
    }

    pub fn target(&self) -> Tagged<HeapObject> {
      let function = *self.target_slot();
      assert!(is_jsfunction(function.ptr()) || is_function_template_info(function.ptr()));
      Tagged::<HeapObject>::unchecked_cast(function)
    }

    pub fn set_target(&self, function: Tagged<HeapObject>) {
        assert!(is_jsfunction(function.ptr()) || is_function_template_info(function.ptr()));
        self.target_slot().store(function);
    }

    pub fn ComputeParametersCount(&self) -> i32 {
      let argc_address = self.fp() + ApiCallbackExitFrameConstants::kFCIArgcOffset as usize;
      let argc_ptr = argc_address as *mut usize;
      let argc = unsafe {*argc_ptr};
      assert!(argc >= 0);
      argc as i32
    }

    pub fn GetParameter(&self, i: i32) -> Tagged<Object> {
      assert!(i >= 0 && i < self.ComputeParametersCount());
      let offset = ApiCallbackExitFrameConstants::kFirstArgumentOffset as i32 + i * kSystemPointerSize as i32;
      let address = self.fp() + offset as usize;
      let param_ptr = address as *mut Address;
      let param_address = unsafe { *param_ptr };
      Tagged::<Object>::new(param_address)
    }

    pub fn IsConstructor(&self) -> bool {
      let new_target_address = self.fp() + ApiCallbackExitFrameConstants::kNewTargetOffset as usize;
      let new_context_ptr = new_target_address as *mut Address;
      let new_context = unsafe { *new_context_ptr };
      !is_undefined(new_context.0, self.isolate())
    }
  }

  impl<'a> ApiAccessorExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      ApiAccessorExitFrame {
        exit_frame: ExitFrame::new(iterator)
      }
    }

    pub fn property_name_slot(&self) -> FullObjectSlot {
      FullObjectSlot {
        address: self.fp() + ApiAccessorExitFrameConstants::kPropertyNameOffset as usize
      }
    }

    pub fn receiver_slot(&self) -> FullObjectSlot {
      FullObjectSlot {
        address: self.fp() + ApiAccessorExitFrameConstants::kReceiverOffset as usize
      }
    }

    pub fn holder_slot(&self) -> FullObjectSlot {
      FullObjectSlot {
        address: self.fp() + ApiAccessorExitFrameConstants::kHolderOffset as usize
      }
    }

    pub fn property_name(&self) -> Tagged<Name> {
      Tagged::<Name>::unchecked_cast(*self.property_name_slot())
    }

    pub fn receiver(&self) -> Tagged<Object> {
      *self.receiver_slot()
    }

    pub fn holder(&self) -> Tagged<Object> {
      *self.holder_slot()
    }
  }

  impl<'a> CommonFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      CommonFrame {
        stack_frame: StackFrame::new(iterator)
      }
    }

    pub fn GetExpression(&self, index: i32) -> Tagged<Object> {
      let address = self.GetExpressionAddress(index);
      let expression_ptr = address as *mut Address;
      let expression_address = unsafe { *expression_ptr };
      Tagged::<Object>::new(expression_address)
    }

    pub fn SetExpression(&self, index: i32, value: Tagged<Object>) {
      let address = self.GetExpressionAddress(index);
      let expression_ptr = address as *mut Address;
      unsafe { *expression_ptr = value.ptr() };
    }

    pub fn caller_fp(&self) -> Address {
      let address = self.fp() + StandardFrameConstants::kCallerFPOffset as usize;
      let caller_fp_ptr = address as *mut Address;
      let caller_fp_address = unsafe { *caller_fp_ptr };
      caller_fp_address
    }

    pub fn caller_pc(&self) -> Address {
      let address = self.fp() + StandardFrameConstants::kCallerPCOffset as usize;
      let pc_address = address as *mut Address;
      self.ReadPC(pc_address)
    }
  }

  impl<'a> CommonFrameWithJSLinkage<'a> {
    pub fn IsConstructFrame(fp: Address) -> bool {
      let frame_type_address = fp.0 + TypedFrameConstants::kFrameTypeOffset as usize;
      let frame_type_ptr = frame_type_address as *mut usize;
      let frame_type = unsafe { *frame_type_ptr };
      frame_type == StackFrame::TypeToMarker(StackFrame::CONSTRUCT) as usize ||
           frame_type == StackFrame::TypeToMarker(StackFrame::FAST_CONSTRUCT) as usize
    }
  }

  impl<'a> JavaScriptFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      JavaScriptFrame {
        common_frame_with_js_linkage: CommonFrameWithJSLinkage::new(iterator)
      }
    }
  }

  impl<'a> CommonFrameWithJSLinkage<'a> {
    pub fn GetParameterSlot(&self, index: i32) -> Address {
      assert!(index >= -1);
      assert!(index < std::cmp::max(self.GetActualArgumentCount(), self.ComputeParametersCount()));
      let parameter_offset = (index + 1) * kSystemPointerSize as i32;
      Address{0: self.caller_sp().0 + parameter_offset as usize}
    }

    pub fn GetActualArgumentCount(&self) -> i32 {
      0
    }
  }

  impl<'a> JavaScriptFrame<'a> {
    pub fn set_receiver(&self, value: Tagged<Object>) {
      let address = self.GetParameterSlot(-1);
      let receiver_ptr = address.0 as *mut Address;
      unsafe { *receiver_ptr = value.ptr() };
    }

    pub fn function_slot_object(&self) -> Tagged<Object> {
      let offset = StandardFrameConstants::kFunctionOffset;
      let address = self.fp() + offset as usize;
      let function_ptr = address as *mut Address;
      let function_address = unsafe { *function_ptr };
      Tagged::<Object>::new(function_address)
    }
  }

  impl<'a> UnoptimizedJSFrame<'a> {
    pub fn SetFeedbackVector(&self, feedback_vector: Tagged<FeedbackVector>) {
        let offset = InterpreterFrameConstants::kFeedbackVectorFromFp;
        let address = self.fp() + offset as usize;
        let feedback_vector_ptr = address as *mut Address;
        unsafe {*feedback_vector_ptr = feedback_vector.ptr()};
    }
  }

  impl<'a> TurbofanStubWithContextFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      TurbofanStubWithContextFrame {
        common_frame: CommonFrame::new(iterator)
      }
    }
  }

  impl<'a> StubFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      StubFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  impl<'a> OptimizedJSFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      OptimizedJSFrame {
        java_script_frame: JavaScriptFrame::new(iterator)
      }
    }
  }

  impl<'a> UnoptimizedJSFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      UnoptimizedJSFrame {
        java_script_frame: JavaScriptFrame::new(iterator)
      }
    }
  }

  impl<'a> InterpretedFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      InterpretedFrame {
        unoptimized_js_frame: UnoptimizedJSFrame::new(iterator)
      }
    }
  }

  impl<'a> BaselineFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      BaselineFrame {
        unoptimized_js_frame: UnoptimizedJSFrame::new(iterator)
      }
    }
  }

  impl<'a> MaglevFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      MaglevFrame {
        optimized_js_frame: OptimizedJSFrame::new(iterator)
      }
    }
  }

  impl<'a> TurbofanJSFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      TurbofanJSFrame {
        optimized_js_frame: OptimizedJSFrame::new(iterator)
      }
    }
  }

  impl<'a> BuiltinFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      BuiltinFrame {
        typed_frame_with_js_linkage: TypedFrameWithJSLinkage::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> WasmFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      WasmFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> WasmSegmentStartFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      WasmSegmentStartFrame {
        wasm_frame: WasmFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> WasmExitFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      WasmExitFrame {
        wasm_frame: WasmFrame::new(iterator)
      }
    }
  }

  #[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_DRUMBRAKE))]
  impl<'a> WasmInterpreterEntryFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      WasmInterpreterEntryFrame {
        wasm_frame: WasmFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> WasmDebugBreakFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      WasmDebugBreakFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> WasmToJsFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      WasmToJsFrame {
        wasm_frame: WasmFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> WasmToJsFunctionFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      WasmToJsFunctionFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> JsToWasmFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      JsToWasmFrame {
        stub_frame: StubFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> StackSwitchFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      StackSwitchFrame {
        exit_frame: ExitFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> CWasmEntryFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      CWasmEntryFrame {
        stub_frame: StubFrame::new(iterator)
      }
    }
  }

  #[cfg(V8_ENABLE_WEBASSEMBLY)]
  impl<'a> WasmLiftoffSetupFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      WasmLiftoffSetupFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  impl<'a> InternalFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      InternalFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  impl<'a> ConstructFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      ConstructFrame {
        internal_frame: InternalFrame::new(iterator)
      }
    }
  }

  impl<'a> FastConstructFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      FastConstructFrame {
        internal_frame: InternalFrame::new(iterator)
      }
    }
  }

  impl<'a> BuiltinContinuationFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      BuiltinContinuationFrame {
        internal_frame: InternalFrame::new(iterator)
      }
    }
  }

  impl<'a> JavaScriptBuiltinContinuationFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      JavaScriptBuiltinContinuationFrame {
        typed_frame_with_js_linkage: TypedFrameWithJSLinkage::new(iterator)
      }
    }
  }

  impl<'a> JavaScriptBuiltinContinuationWithCatchFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      JavaScriptBuiltinContinuationWithCatchFrame {
        java_script_builtin_continuation_frame: JavaScriptBuiltinContinuationFrame::new(iterator)
      }
    }
  }

  impl<'a> IrregexpFrame<'a> {
    pub fn new(iterator: &'a mut StackFrameIteratorBase<'a>) -> Self {
      IrregexpFrame {
        typed_frame: TypedFrame::new(iterator)
      }
    }
  }

  impl<'a> DebuggableStackFrameIterator<'a> {
    pub fn frame(&self) -> *mut CommonFrame<'a> {
      let frame = self.iterator_.frame();
      #[cfg(V8_ENABLE_WEBASSEMBLY)]
      assert!(frame.is_javascript() || frame.is_wasm());
      #[cfg(not(V8_ENABLE_WEBASSEMBLY))]
      assert!(frame.is_javascript());
      frame as *mut StackFrame as *mut CommonFrame
    }

    pub fn Reframe(&mut self) -> *mut CommonFrame<'a> {
      self.iterator_.Reframe();
      self.frame()
    }

    pub fn is_javascript(&self) -> bool {
      unsafe {(*self.frame()).is_javascript()}
    }

    #[cfg(V8_ENABLE_WEBASSEMBLY)]
    pub fn is_wasm(&self) -> bool {
      unsafe {(*self.frame()).is_wasm()}
    }

    #[cfg(all(V8_ENABLE_WEBASSEMBLY, V8_ENABLE_DRUMBRAKE))]
    pub fn is_wasm_interpreter_entry(&self) -> bool {
      unsafe {(*self.frame()).is_wasm_interpreter_entry()}
    }

    pub fn javascript_frame(&self) -> *mut JavaScriptFrame<'a> {
      unsafe {JavaScriptFrame::cast((*self.frame()))}
    }
  }

  impl StackFrameIteratorForProfiler {
    pub fn IsValidFrameType(type_: StackFrame::Type) -> bool {
      #[cfg(V8_ENABLE_WEBASSEMBLY)]
      assert_ne!(type_, StackFrame::C_WASM_ENTRY);

      let is_valid = StackFrame::IsJavaScript(type_) ||
                    type_ == StackFrame::EXIT ||
                    type_ == StackFrame::BUILTIN_EXIT ||
                    type_ == StackFrame::API_ACCESSOR_EXIT ||
                    type_ == StackFrame::API_CALLBACK_EXIT ||
                    {
                      #[cfg(V8_ENABLE_WEBASSEMBLY)]
                        {
                          type_ == StackFrame::WASM ||
                          type_ == StackFrame::WASM_TO_JS ||
                          type_ == StackFrame::JS_TO_WASM ||
                          type_ == StackFrame::WASM_SEGMENT_START ||
                          {
                            #[cfg(V8_ENABLE_DRUMBRAKE)]
                            {
                              type_ == StackFrame::WASM_INTERPRETER_ENTRY
                            }
                            #[cfg(not(V8_ENABLE_DRUMBRAKE))]
                            {
                              false
                            }
                          }
                        }
                      #[cfg(not(V8_ENABLE_WEBASSEMBLY))]
                        {
                          false
                        }
                    };
      is_valid
    }

    pub fn frame(&self) -> *mut StackFrame {
        assert!(!self.done_);
        assert!(StackFrameIteratorForProfiler::IsValidFrameType(self.frame_.type_()));
        self.frame_ as *mut StackFrame
    }
  }

  fn is_jsfunction(object: usize) -> bool {
      true
  }

  fn is_function_template_info(object: usize) -> bool {
      true
  }

  fn is_undefined(address: usize, isolate_data: *mut IsolateData) -> bool {
    true
  }
}
