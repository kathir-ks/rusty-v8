// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-internal.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
  use std::ffi::CString;
  use std::mem::MaybeUninit;
  use std::sync::Mutex;

  pub struct V8 {}
  pub struct Isolate {
    heap_: Heap,
    stack_guard_: StackGuard,
    counters_: Counters,
    context_: *mut Context, //TODO - replace with correct type
    exceptions_: Exceptions,
    factory_: Factory,
    tiering_manager_: TieringManager,
  }

  impl Isolate {
    pub fn heap(&mut self) -> &mut Heap {
      &mut self.heap_
    }

    pub fn stack_guard(&self) -> &StackGuard {
      &self.stack_guard_
    }
    pub fn has_exception(&self) -> bool {
      true // Placeholder
    }
    pub fn set_exception(&mut self, _exception: Object) {}
    pub fn clear_exception(&mut self) {}
    pub fn native_context(&self) -> &NativeContext {
      unsafe { &*self.context_ }.native_context() // Assuming Context has native_context method
    }
    pub fn context(&self) -> &Context {
      unsafe { &*self.context_ } // Assuming Context has native_context method
    }
    pub fn terminate_execution(&self) -> Address {
      Address { dummy: 0 }
    }
    pub fn tiering_manager(&self) -> &TieringManager{
      &self.tiering_manager_
    }
    pub fn factory(&self) -> &Factory{
      &self.factory_
    }

    pub fn report_failed_access_check(&self, _object: DirectHandle<JSObject>) -> Box<dyn std::error::Error> {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Access check failed"))
    }

    pub fn may_access(&self, _context: &NativeContext, _object: DirectHandle<JSObject>) -> bool {
        true
    }
    pub fn stack_overflow(&self) -> Address {
      Address { dummy: 0 } // Placeholder
    }

    pub fn unwind_and_find_handler(&self) -> Address {
      Address { dummy: 0 } // Placeholder
    }
    pub fn throw(&self, _arg: Address) -> Address {
      Address { dummy: 0 }
    }
    pub fn re_throw(&self, _arg: Address) -> Address {
      Address { dummy: 0 }
    }
    pub fn re_throw_with_message(&self, _arg1: Address, _arg2: Address) -> Address {
      Address { dummy: 0 }
    }
    pub fn create_message_or_abort(&self, _exception: &DirectHandle<Object>, _no_location: *mut MessageLocation) -> DirectHandle<JSMessageObject> {
      DirectHandle{dummy : 0}
    }
    pub fn exceptions(&mut self) -> &mut Exceptions {
      &mut self.exceptions_
    }
    pub fn count_usage(&self, _counter: v8::Isolate::UseCounterFeature) {}
    pub fn counters(&self) -> &Counters{
      &self.counters_
    }
  }

  pub struct Heap {
    // dummy field
    dummy: i32,
  }

  impl Heap {
    pub fn fatal_process_out_of_memory(&mut self, message: &str) {
      eprintln!("FatalProcessOutOfMemory: {}", message);
      std::process::abort();
    }
  }

  pub struct StackGuard {
    // dummy field
    dummy: i32,
  }

  impl StackGuard {
    pub fn handle_interrupts(&self) -> Address {
      Address { dummy: 0 }
    }
    pub fn handle_interrupts_level(&self, _level: InterruptLevel) -> Address {
      Address { dummy: 0 }
    }
  }

  pub enum InterruptLevel {
    kAnyEffect,
    kNoHeapWrites,
  }

  pub struct Counters {
    worker_thread_runtime_call_stats_: Mutex<WorkerThreadRuntimeCallStats>,
    runtime_call_stats_: RuntimeCallStats,
  }

  impl Counters {
    pub fn worker_thread_runtime_call_stats(&self) -> &Mutex<WorkerThreadRuntimeCallStats> {
      &self.worker_thread_runtime_call_stats_
    }
    pub fn runtime_call_stats(&mut self) -> &mut RuntimeCallStats {
      &mut self.runtime_call_stats_
    }
  }

  pub struct WorkerThreadRuntimeCallStats {}
  impl WorkerThreadRuntimeCallStats {
    pub fn add_to_main_table(&self, _runtime_call_stats: &mut RuntimeCallStats) {}
  }

  pub struct RuntimeCallStats {}
  impl RuntimeCallStats {
    pub fn print(&self, _stream: &mut std::stringstream) {}
    pub fn reset(&mut self) {}
  }

  #[derive(Debug)]
  pub struct Address {
    dummy: i32,
  }

  pub struct HandleScope {}

  impl HandleScope {
    pub fn new(_isolate: &Isolate) -> Self {
      HandleScope {}
    }
  }

  pub struct SealHandleScope {}

  impl SealHandleScope {
    pub fn new(_isolate: &Isolate) -> Self {
      SealHandleScope {}
    }
  }

  pub struct RuntimeArguments {
    length_: i32,
    values_: Vec<Address>,
  }

  impl RuntimeArguments {
    pub fn length(&self) -> i32 {
      self.length_
    }

    pub fn at<T>(&self, index: i32) -> DirectHandle<T> {
      DirectHandle { dummy: 0 } // Placeholder
    }

    pub fn smi_value_at(&self, index: i32) -> i32 {
      0 // Placeholder
    }
    pub fn positive_smi_value_at(&self, index: i32) -> u32 {
      0 // Placeholder
    }
    pub fn number_value_at(&self, index: i32) -> f64 {
      0.0 // Placeholder
    }
    pub fn get(&self, i: usize) -> Address {
      self.values_[i].clone()
    }
  }

  pub struct DirectHandle<T> {
    dummy: i32,
  }

  pub struct MaybeDirectHandle<T> {}

  pub struct JSObject {}

  pub struct JSFunction {}
  impl JSFunction {
    pub fn global_proxy(&self) -> &JSObject {
      &JSObject {} // Placeholder
    }
    pub fn trace_optimization_status(&self, _s: &str, _codekindtostring: &str){}
  }

  pub struct String {}
  impl String {
    pub fn print_on(&self, _f: *mut std::ffi::c_void){}
    pub fn to_cstring(&self) -> std::result::Result<CString, std::ffi::NulError> {
      CString::new("dummy") // Placeholder
    }
  }

  pub struct Factory {
      //dummy field
      dummy: i32
  }

  impl Factory{
    pub fn new_string_from_ascii_checked(&self, _str: &str) -> DirectHandle<String> {
      DirectHandle{dummy : 0}
    }
    pub fn new_cons_string(&self, _str1 : DirectHandle<String>, _str2 : DirectHandle<String>) -> std::result::Result<DirectHandle<String>, Box<dyn std::error::Error>>{
      Ok(DirectHandle{dummy : 0})
    }
    pub fn object_string(&self) -> DirectHandle<String>{
      DirectHandle{dummy : 0}
    }
    pub fn new_error(&self, _message_template: MessageTemplate, _arg0: DirectHandle<Object>) -> DirectHandle<Object> {
      DirectHandle{dummy : 0}
    }
    pub fn new_error_function(&self) -> DirectHandle<JSFunction> {
      DirectHandle{dummy : 0}
    }
    pub fn range_error_function(&self) -> DirectHandle<JSFunction> {
      DirectHandle{dummy : 0}
    }
    pub fn type_error_function(&self) -> DirectHandle<JSFunction> {
      DirectHandle{dummy : 0}
    }
    pub fn new_reference_error(&self, _message_template: MessageTemplate, _arg0: DirectHandle<Object>) -> DirectHandle<Object> {
      DirectHandle{dummy : 0}
    }
    pub fn undefined_value(&self) -> Address{
      Address{dummy: 0}
    }
    pub fn to_boolean(&self, _b: bool) -> Address{
      Address{dummy: 0}
    }
    pub fn new_byte_array(&self, _length: i32) -> DirectHandle<ByteArray> {
      DirectHandle{dummy : 0}
    }
    pub fn new_js_async_from_sync_iterator(&self, _sync_iterator: DirectHandle<JSReceiver>, _next: DirectHandle<Object>) -> DirectHandle<JSAsyncFromSyncIterator> {
      DirectHandle{dummy : 0}
    }
    pub fn class_fields_symbol(&self) -> DirectHandle<Symbol> {
      DirectHandle{dummy : 0}
    }
    pub fn new_filler_object(&self, _size: i32, _alignment: AllocationAlignment, _allocation_type: AllocationType, _allocation_origin: AllocationOrigin) -> DirectHandle<FillerObject>{
      DirectHandle{dummy : 0}
    }
  }

  pub struct JSReceiver {}
  impl JSReceiver {
    pub fn get_data_property(_isolate: &Isolate, _constructor: DirectHandle<JSReceiver>, _key: DirectHandle<Symbol>) -> DirectHandle<Object>{
      DirectHandle{dummy : 0}
    }
  }

  pub struct JSAsyncFromSyncIterator {}

  pub struct TemplateObjectDescription {}
  impl TemplateObjectDescription{
    pub fn get_template_object(_isolate: &Isolate, _native_context: DirectHandle<NativeContext>, _description: DirectHandle<TemplateObjectDescription>, _shared_info: DirectHandle<SharedFunctionInfo>, _slot_id: i32) -> DirectHandle<Object>{
      DirectHandle{dummy : 0}
    }
  }

  pub struct NativeContext {}

  pub struct SharedFunctionInfo {}

  pub struct Symbol {}

  pub struct Object {}
  impl Object {
    pub fn create_list_from_array_like(
      _isolate: &Isolate,
      _object: DirectHandle<Object>,
      _element_types: ElementTypes,
    ) -> std::result::Result<Address, Box<dyn std::error::Error>> {
      Ok(Address { dummy: 0 }) // Placeholder
    }
    pub fn type_of(_isolate: &Isolate, _object: DirectHandle<Object>) -> DirectHandle<String>{
      DirectHandle{dummy : 0}
    }
    pub fn share_slow(_isolate: &Isolate, _value : Handle<HeapObject>, _kthrowonerror : i32) -> std::result::Result<DirectHandle<Object>, Box<dyn std::error::Error>> {
      Ok(DirectHandle{dummy : 0}) // Placeholder
    }
    pub fn ordinary_has_instance(_isolate: &Isolate, _callable: DirectHandle<JSAny>, _object: DirectHandle<JSAny>) -> std::result::Result<Address, Box<dyn std::error::Error>> {
      Ok(Address { dummy: 0 }) // Placeholder
    }
    pub fn get_property(_isolate: &Isolate, _receiver: DirectHandle<JSReceiver>, _name: DirectHandle<String>) -> std::result::Result<DirectHandle<Object>, Box<dyn std::error::Error>> {
      Ok(DirectHandle{dummy : 0}) // Placeholder
    }
    pub fn to_int32(_object: Address, _radix: *mut i32) -> bool {
        true
    }
  }

  pub struct ByteArray {}

  pub struct FillerObject {}

  pub struct Handle<T> {
    // dummy field
    dummy: i32,
    _phantom: std::marker::PhantomData<T>,
  }
  impl<T> Handle<T>{
    pub fn new() -> Self {
      Handle{dummy: 0, _phantom: std::marker::PhantomData}
    }
  }

  pub struct JSAny {}

  pub enum MessageTemplate {
    kSymbolAsyncIteratorInvalid,
    kBigIntTooBig,
    kIteratorResultNotAnObject,
    kThrowMethodMissing,
    kSymbolIteratorInvalid,
    kNoAccess,
    kNotConstructor,
    kApplyNonFunction,
    kInvalid,
    kNotDefined,
    kAccessedUninitializedVariable,
    kDerivedConstructorReturnedNonObject,
    kInvalidTypedArrayAlignment
  }

  fn message_template_from_int(message_id_smi: i32) -> MessageTemplate {
    match message_id_smi {
      _ => MessageTemplate::kSymbolAsyncIteratorInvalid, // Placeholder
    }
  }

  pub struct ErrorUtils {}

  impl ErrorUtils {
    pub fn new_iterator_error(_isolate: &Isolate, _object: DirectHandle<Object>) -> Handle<Object> {
      Handle::new() // Placeholder
    }
    pub fn throw_spread_arg_error(_isolate: &Isolate, _message_id: MessageTemplate, _object: DirectHandle<Object>) -> Address{
      Address{dummy: 0}
    }
    pub fn new_called_non_callable_error(_isolate: &Isolate, _object: DirectHandle<Object>) -> Handle<Object> {
      Handle::new() // Placeholder
    }
    pub fn new_constructed_non_constructable(_isolate: &Isolate, _object: DirectHandle<Object>) -> Handle<Object> {
      Handle::new() // Placeholder
    }
    pub fn throw_load_from_null_or_undefined(_isolate: &Isolate, _object: DirectHandle<Object>, _maybe_directhandle: MaybeDirectHandle<Object>) -> Address{
      Address{dummy: 0}
    }
  }

  pub struct StackLimitCheck {
    isolate: *mut Isolate,
  }

  impl StackLimitCheck {
    pub fn new(isolate: *mut Isolate) -> Self {
      StackLimitCheck { isolate }
    }

    pub fn js_has_overflowed(&self) -> bool {
      true // Placeholder
    }
    pub fn interrupt_requested(&self) -> bool {
      true // Placeholder
    }
    pub fn js_has_overflowed_gap(&self, _gap: u32) -> bool {
      true // Placeholder
    }
  }

  pub struct TieringManager {}
  impl TieringManager{
    pub fn on_interrupt_tick(&self, _function: DirectHandle<JSFunction>, _code_kind: CodeKind){}
  }

  pub enum CodeKind {
    INTERPRETED_FUNCTION,
    BASELINE,
    MAGLEV
  }
  pub fn code_kind_to_string(_code_kind: CodeKind) -> &'static str {
    "INTERPRETED_FUNCTION" // Placeholder
  }

  pub struct SaveAndClearThreadInWasmFlag {
    _isolate: *mut Isolate,
  }
  impl SaveAndClearThreadInWasmFlag {
    pub fn new(_isolate: *mut Isolate) -> Self {
      SaveAndClearThreadInWasmFlag { _isolate }
    }
  }

  pub enum AllocationAlignment {
    kDoubleAligned,
    kTaggedAligned,
  }

  pub enum AllocationType {
    kYoung,
    kOld,
  }

  pub enum AllocationOrigin {
    kGeneratedCode,
  }

  pub enum ElementTypes {
    kAll
  }

  pub mod v8 {
    pub enum UseCounterFeature {
      // Placeholder
      V8_USE_COUNTER_LAST_FEATURE,
    }
  }

  pub struct BasicBlockProfiler {}

  impl BasicBlockProfiler {
    pub fn get() -> &'static BasicBlockProfiler {
      unsafe {
        static mut INSTANCE: MaybeUninit<BasicBlockProfiler> = MaybeUninit::uninit();
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
          INSTANCE.as_mut_ptr().write(BasicBlockProfiler {});
        });
        INSTANCE.assume_init_ref()
      }
    }
    pub fn has_data(&self, _isolate: &Isolate) -> bool {
      true
    }
    pub fn log(&self, _isolate: &Isolate, _stats_stream: &mut std::stringstream) {}
    pub fn reset_counts(&self, _isolate: &Isolate) {}
  }

  pub struct OFStream<'a> {
    f: *mut std::ffi::c_void,
    _phantom: std::marker::PhantomData<&'a ()>,
  }

  impl<'a> OFStream<'a> {
    pub fn new(f: *mut std::ffi::c_void) -> Self {
      OFStream {
        f: f,
        _phantom: std::marker::PhantomData,
      }
    }
  }
  pub struct Exceptions{}

  #[link(wasm_import_module = "foo")]
  extern "C" {
    fn abort();
  }

  #[no_mangle]
  pub extern "C" fn FatalInvalidSize(){
    unsafe{abort()};
  }

  macro_rules! return_failure_on_exception {
      ($isolate:expr, $value:expr) => {
          return Address { dummy: 0 };
      };
  }
  pub(crate) use return_failure_on_exception;

  macro_rules! throw_new_error_return_failure {
    ($isolate:expr, $value:expr) => {
        return Address { dummy: 0 };
    };
  }
  pub(crate) use throw_new_error_return_failure;

  macro_rules! check {
    ($e:expr) => {
        if !($e) {
            unsafe{abort()};
        }
    };
  }
  pub(crate) use check;

  macro_rules! dcheck_eq {
    ($e1:expr, $e2:expr) => {
        if $e1 != $e2 {
            unsafe{abort()};
        }
    };
  }
  pub(crate) use dcheck_eq;

  macro_rules! dcheck_le {
    ($e1:expr, $e2:expr) => {
        if $e1 > $e2 {
            unsafe{abort()};
        }
    };
  }
  pub(crate) use dcheck_le;

  macro_rules! dcheck_lt {
    ($e1:expr, $e2:expr) => {
        if $e1 >= $e2 {
            unsafe{abort()};
        }
    };
  }
  pub(crate) use dcheck_lt;

  macro_rules! dcheck_not_null {
    ($e:expr) => {
        if ($e).is_null() {
            unsafe{abort()};
        }
    };
  }
  pub(crate) use dcheck_not_null;

  macro_rules! align_to_allocation_alignment {
    ($size:expr) => {
        $size
    };
  }
  pub(crate) use align_to_allocation_alignment;

  macro_rules! is_aligned {
    ($size:expr, $alignment:expr) => {
        true
    };
  }
  pub(crate) use is_aligned;

  macro_rules! check_gt {
    ($e1:expr, $e2:expr) => {
        if !($e1 > $e2) {
            unsafe{abort()};
        }
    };
  }
  pub(crate) use check_gt;

  macro_rules! assign_return_failure_on_exception {
      ($isolate:expr, $var:expr, $expr:expr) => {
          match $expr {
              Ok(value) => $var = value,
              Err(_e) => return Address { dummy: 0 },
          }
      };
  }
  pub(crate) use assign_return_failure_on_exception;

  macro_rules! return_result_or_failure {
    ($isolate:expr, $result:expr) => {
      match $result {
        Ok(value) => return value,
        Err(_e) => return Address{dummy: 0}
      }
    };
  }
  pub(crate) use return_result_or_failure;

  const V8_RUNTIME_CALL_STATS: bool = true;
  const KDOUBLETORADIXMAXCHARS: usize = 128;
}

use internal::*;

#[no_mangle]
pub extern "C" fn Runtime_AccessCheck(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(1, args.length());
  let object: DirectHandle<JSObject> = args.at(0);
  if !_isolate.may_access(_isolate.native_context(), object) {
    return_failure_on_exception!(_isolate, _isolate.report_failed_access_check(object));
    unreachable!();
  }
  ReadOnlyRoots(_isolate).undefined_value()
}

#[no_mangle]
pub extern "C" fn Runtime_FatalProcessOutOfMemoryInAllocateRaw(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(0, args.length());
  _isolate
    .heap()
    .fatal_process_out_of_memory("CodeStubAssembler::AllocateRaw");
  unreachable!();
}

#[no_mangle]
pub extern "C" fn Runtime_FatalProcessOutOfMemoryInvalidArrayLength(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(0, args.length());
  _isolate
    .heap()
    .fatal_process_out_of_memory("invalid array length");
  unreachable!();
}

#[no_mangle]
pub extern "C" fn Runtime_FatalInvalidSize(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(0, args.length());
  FatalInvalidSize();
  unreachable!();
}

#[no_mangle]
pub extern "C" fn Runtime_Throw(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(1, args.length());
  _isolate.throw(args.get(0))
}

#[no_mangle]
pub extern "C" fn Runtime_ReThrow(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(1, args.length());
  _isolate.re_throw(args.get(0))
}

#[no_mangle]
pub extern "C" fn Runtime_ReThrowWithMessage(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(2, args.length());
  _isolate.re_throw_with_message(args.get(0), args.get(1))
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowStackOverflow(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let _isolate = unsafe { &mut *isolate };
  let shs = SealHandleScope::new(_isolate);
  dcheck_le!(0, args.length());
  _isolate.stack_overflow()
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowSymbolAsyncIteratorInvalid(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(0, args.length());
  throw_new_error_return_failure!(
    _isolate,
    NewTypeError(_isolate, MessageTemplate::kSymbolAsyncIteratorInvalid)
  );
}

#[no_mangle]
pub extern "C" fn Runtime_TerminateExecution(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_eq!(0, args.length());
  _isolate.terminate_execution()
}

fn new_error(
  isolate: *mut Isolate,
  args: RuntimeArguments,
  constructor_fn: fn(&mut Isolate) -> DirectHandle<JSFunction>,
) -> Address {
  let mut _isolate = unsafe { &mut *isolate };
  let scope = HandleScope::new(_isolate);
  dcheck_le!(1, args.length());
  let message_id_smi = args.smi_value_at(0);

  const KMAXMESSAGEARGS: usize = 3;
  let mut message_args: [DirectHandle<Object>; KMAXMESSAGEARGS] = [
    DirectHandle { dummy: 0 },
    DirectHandle { dummy: 0 },
    DirectHandle { dummy: 0 },
  ];
  let mut num_message_args = 0;
  while num_message_args < KMAXMESSAGEARGS && args.length() > num_message_args + 1 {
    message_args[num_message_args] = args.at(num_message_args + 1);
    num_message_args += 1;
  }

  let message_id = message_template_from_int(message_id_smi);

  Address{dummy:0} //Placeholder
  //*_isolate.factory().new_error(
  //    constructor_fn(&mut _isolate),
  //    message_id,
  //    message_args[..num_message_args].into(),
  //)
}

fn throw_error(
  isolate: *mut Isolate,
  args: RuntimeArguments,
  constructor_fn: fn(&mut Isolate) -> DirectHandle<JSFunction>,
) -> Address {
  let mut _isolate = unsafe { &mut *isolate };
  let error_address = new_error(isolate, args, constructor_fn);
  _isolate.throw(error_address)
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowRangeError(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let mut _isolate = unsafe { &mut *isolate };

  if false {
    dcheck_le!(1, args.length());
    let message_id_smi = args.smi_value_at(0);

    if message_template_from_int(message_id_smi) == MessageTemplate::kBigIntTooBig {
      unsafe {
        abort();
      }
    }
  }

  throw_error(isolate, args, |isolate| isolate.factory().range_error_function())
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowTypeError(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let isolate = unsafe { &mut *isolate };
  throw_error(isolate as *mut Isolate, args, |isolate| isolate.factory().type_error_function())
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowTypeErrorIfStrict(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let isolate = unsafe { &mut *isolate };
  if GetShouldThrow(isolate, Nothing::new()) == ShouldThrow::kDontThrow {
    return ReadOnlyRoots(isolate).undefined_value();
  }
  throw_error(isolate as *mut Isolate, args, |isolate| isolate.factory().type_error_function())
}

fn elements_kind_to_type(fixed_elements_kind: ElementsKind) -> &'static str {
  match fixed_elements_kind {
    ElementsKind::kUint8Elements => "Uint8Array",
    ElementsKind::kInt8Elements => "Int8Array",
    ElementsKind::kUint16Elements => "Uint16Array",
    ElementsKind::kInt16Elements => "Int16Array",
    ElementsKind::kUint32Elements => "Uint32Array",
    ElementsKind::kInt32Elements => "Int32Array",
    ElementsKind::kFloat32Elements => "Float32Array",
    ElementsKind::kFloat64Elements => "Float64Array",
    ElementsKind::kUint8ClampedElements => "Uint8ClampedArray",
    _ => unreachable!(),
  }
}

#[no_mangle]
pub extern "C" fn Runtime_ThrowInvalidTypedArrayAlignment(
  args_length: i32,
  args_object: *mut Address,
  isolate: *mut Isolate,
) -> Address {
  let args = unsafe {
    RuntimeArguments {
      length_: args_length,
      values_: Vec::from_raw_parts(
        args_object,
        args_length as usize,
        args_length as usize,
      ),
    }
  };
  let isolate = unsafe { &mut *isolate };
  let
