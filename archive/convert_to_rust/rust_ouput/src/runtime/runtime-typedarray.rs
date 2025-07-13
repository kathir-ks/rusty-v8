// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-typedarray.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod typedarray {
  use crate::v8::internal::{HandleScope, V8};
  use std::cmp::Ordering;
  use std::mem::size_of;
  use std::ptr;
  use std::sync::{Arc, Mutex};

  pub struct Isolate {
    pub array_buffer_allocator: Arc<Mutex<ArrayBufferAllocator>>,
    pub factory: Factory,
  }

  impl Isolate {
    pub fn new(array_buffer_allocator: Arc<Mutex<ArrayBufferAllocator>>) -> Self {
      Self {
        array_buffer_allocator,
        factory: Factory::new(),
      }
    }
  }

  pub struct Factory {}

  impl Factory {
    pub fn new() -> Self {
      Factory {}
    }
    pub fn NewNumber(&self, value: f64) -> Box<Number> {
      Box::new(Number { value })
    }
    pub fn NewByteArray(&self, length: i32) -> Box<ByteArray> {
      Box::new(ByteArray {
        data: vec![0u8; length as usize],
      })
    }

    pub fn NewNumberFromSize(&self, size: usize) -> Box<Number> {
      Box::new(Number { value: size as f64 })
    }
  }

  pub struct Number {
    pub value: f64,
  }

  pub struct ByteArray {
    pub data: Vec<u8>,
  }
  impl ByteArray {
    pub fn begin(&mut self) -> *mut u8 {
      self.data.as_mut_ptr()
    }

    pub fn LengthFor(size: usize) -> usize {
      size
    }
  }
  pub struct ReadOnlyRoots {
    pub undefined_value: i32,
    pub exception: i32,
  }

  impl ReadOnlyRoots {
    pub fn new(undefined_value: i32, exception: i32) -> Self {
      ReadOnlyRoots {
        undefined_value,
        exception,
      }
    }
  }

  pub struct JSArrayBuffer {
    pub backing_store: Vec<u8>,
    pub is_shared: bool,
    pub detach_key: i32,
    pub byte_length : usize
  }

  impl JSArrayBuffer {
    pub fn new(byte_length: usize, is_shared: bool) -> Self {
      JSArrayBuffer {
        backing_store: vec![0; byte_length],
        is_shared,
        detach_key: 0,
        byte_length
      }
    }
    pub fn Detach(
      &mut self,
      force: bool,
      key: Option<i32>,
    ) -> Result<(), DetachError> {
      if self.is_shared && !force {
        return Err(DetachError::Shared);
      }
      if key.is_some() && self.detach_key != key.unwrap() {
        return Err(DetachError::InvalidKey);
      }
      self.backing_store.clear();
      Ok(())
    }

    pub fn is_shared(&self) -> bool {
      self.is_shared
    }

    pub fn byte_length(&self) -> usize {
      self.backing_store.len()
    }

    pub fn GetBackingStore(&self) -> &Vec<u8> {
      &self.backing_store
    }

    pub fn set_detach_key(&mut self, key: i32) {
      self.detach_key = key;
    }
  }

  #[derive(Debug)]
  pub enum DetachError {
    Shared,
    InvalidKey,
  }

  pub struct JSTypedArray {
    pub buffer: Box<JSArrayBuffer>,
    pub byte_offset: usize,
    pub length: usize,
    pub element_size: usize,
    pub external_array_type: ExternalArrayType,
  }

  impl JSTypedArray {
    pub fn new(
      buffer: Box<JSArrayBuffer>,
      byte_offset: usize,
      length: usize,
      element_size: usize,
      external_array_type: ExternalArrayType,
    ) -> Self {
      JSTypedArray {
        buffer,
        byte_offset,
        length,
        element_size,
        external_array_type,
      }
    }

    pub fn GetBuffer(&self) -> &Box<JSArrayBuffer> {
      &self.buffer
    }

    pub fn GetByteLength(&self) -> usize {
      self.length * self.element_size
    }

    pub fn GetLength(&self) -> usize {
      self.length
    }

    pub fn DataPtr(&mut self) -> *mut u8 {
      self.buffer.backing_store[self.byte_offset..].as_mut_ptr()
    }
    pub fn type_(&self) -> ExternalArrayType {
      self.external_array_type
    }
    pub fn WasDetached(&self) -> bool {
      self.buffer.backing_store.is_empty()
    }
    pub fn IsOutOfBounds(&self) -> bool {
      self.byte_offset + (self.length * self.element_size) > self.buffer.backing_store.len()
    }

    pub fn GetElementsAccessor(&self) -> Box<ElementsAccessor> {
      Box::new(ElementsAccessor {})
    }
  }

  #[derive(PartialEq, Eq, Debug, Clone, Copy)]
  pub enum ExternalArrayType {
    kExternalInt8Array,
    kExternalUint8Array,
    kExternalInt16Array,
    kExternalUint16Array,
    kExternalInt32Array,
    kExternalUint32Array,
    kExternalFloat32Array,
    kExternalFloat64Array,
    kExternalUint8ClampedArray,
    kExternalBigInt64Array,
    kExternalBigUint64Array,
    kExternalFloat16Array,
  }

  pub struct ElementsAccessor {}

  impl ElementsAccessor {
    pub fn CopyElements(
      &self,
      source: Box<JSAny>,
      target: &mut JSTypedArray,
      length: usize,
      offset: usize,
    ) -> i32 {
      match *source {
        JSAny::JSTypedArray(ref src) => {
          unsafe {
            let src_ptr = src.buffer.backing_store.as_ptr().add(src.byte_offset);
            let dst_ptr = target.buffer.backing_store.as_mut_ptr().add(target.byte_offset + offset * target.element_size);

            ptr::copy_nonoverlapping(src_ptr, dst_ptr, length * target.element_size);
          }
          0
        }
        _ => -1,
      }
    }
  }
  pub struct Arguments {
    pub args: Vec<Box<JSAny>>,
  }

  impl Arguments {
    pub fn length(&self) -> usize {
      self.args.len()
    }
    pub fn at<T>(&self, index: usize) -> Box<T>
    where
      T: 'static,
    {
      match self.args[index].as_any().downcast_ref::<T>() {
        Some(x) => unsafe { Box::from_raw(Box::into_raw(Box::new(x.clone())) as *mut T) }, // VERY unsafe
        None => panic!("wrong type!"),
      }
    }
    pub fn at_(&self, index: usize) -> &Box<JSAny> {
      &self.args[index]
    }

    pub fn atOrUndefined(&self, isolate: &Isolate, index: usize) -> Option<i32> {
      if index < self.args.len() {
        Some(1)
      } else {
        None
      }
    }
  }

  pub enum JSAny {
    JSArrayBuffer(JSArrayBuffer),
    JSTypedArray(JSTypedArray),
    Number(Number),
  }

  impl JSAny {
    pub fn as_any(&self) -> &dyn std::any::Any {
      match self {
        JSAny::JSArrayBuffer(x) => x,
        JSAny::JSTypedArray(x) => x,
        JSAny::Number(x) => x,
      }
    }
  }

  pub fn IsJSArrayBuffer(obj: &Box<JSAny>) -> bool {
    match **obj {
      JSAny::JSArrayBuffer(_) => true,
      _ => false,
    }
  }

  pub fn IsJSTypedArray(obj: &Box<JSAny>) -> bool {
    match **obj {
      JSAny::JSTypedArray(_) => true,
      _ => false,
    }
  }

  pub fn Cast<T>(obj: &Box<JSAny>) -> &T
  where
    T: 'static,
  {
    match **obj {
      JSAny::JSArrayBuffer(ref x) => unsafe { &*(x as *const JSArrayBuffer as *const T) },
      JSAny::JSTypedArray(ref x) => unsafe { &*(x as *const JSTypedArray as *const T) },
      _ => panic!("oh no"),
    }
  }

  pub fn TryNumberToSize(obj: &Box<JSAny>, out: &mut usize) -> bool {
    match **obj {
      JSAny::Number(ref n) => {
        *out = n.value as usize;
        true
      }
      _ => false,
    }
  }

  pub fn NewTypeError(template: MessageTemplate) -> TypeError {
    TypeError { template }
  }

  pub struct TypeError {
    pub template: MessageTemplate,
  }

  #[derive(Debug)]
  pub enum MessageTemplate {
    kNotTypedArray,
  }

  pub fn ThrowNewErrorReturnFailure<T>(isolate: &Isolate, error: TypeError) -> Result<T, TypeError> {
    Err(error)
  }

  pub struct SaveAndClearThreadInWasmFlag {}

  impl SaveAndClearThreadInWasmFlag {
    pub fn new(_isolate: &Isolate) -> Self {
      SaveAndClearThreadInWasmFlag {}
    }
  }

  pub struct ArrayBufferAllocator {
    pub max_allocation_size: usize,
  }

  impl ArrayBufferAllocator {
    pub fn new(max_allocation_size: usize) -> Self {
      ArrayBufferAllocator { max_allocation_size }
    }

    pub fn MaxAllocationSize(&self) -> usize {
      self.max_allocation_size
    }
  }

  pub const KMaxRegularHeapObjectSize: usize = 2147483647;

  pub fn fp16_ieee_to_fp32_value(half: u16) -> f32 {
    let float16 = half as u32;
    let sign = (float16 >> 15) & 1;
    let exponent = (float16 >> 10) & 0x1F;
    let fraction = float16 & 0x3FF;

    if exponent == 0 {
      if fraction == 0 {
        return f32::from_bits((sign << 31) as u32);
      } else {
        let mut f = fraction as u32;
        while (f & 0x400) == 0 {
          f <<= 1;
          exponent -= 1;
        }
      }
    } else if exponent == 0x1F {
      if fraction == 0 {
        return if sign == 0 {
          f32::INFINITY
        } else {
          f32::NEG_INFINITY
        };
      } else {
        return f32::NAN;
      }
    }
    let sign = sign as u32;
    let exponent = (exponent as i32 - 15 + 127) as u32;
    let fraction = fraction as u32 << 13;

    let result = (sign << 31) | (exponent << 23) | fraction;
    f32::from_bits(result)
  }
}

mod runtime {
  use crate::typedarray::*;
  use std::cmp::Ordering;
  use std::sync::{Arc, Mutex};

  pub fn Runtime_ArrayBufferDetach(
    isolate: &Isolate,
    args: Arguments,
  ) -> Result<i32, TypeError> {
    if args.length() < 1 || !IsJSArrayBuffer(args.at_(&0)) {
      return ThrowNewErrorReturnFailure(
        isolate,
        NewTypeError(MessageTemplate::kNotTypedArray),
      );
    }
    let mut array_buffer = args.at::<JSArrayBuffer>(0);
    let force_for_wasm_memory = false;
    match JSArrayBuffer::Detach(
      &mut array_buffer,
      force_for_wasm_memory,
      args.atOrUndefined(isolate, 1),
    ) {
      Ok(_) => Ok(ReadOnlyRoots::new(0, 0).undefined_value),
      Err(_) => {
        //ReadOnlyRoots(isolate).exception()
        Ok(ReadOnlyRoots::new(0, 0).exception)
      }
    }
  }

  pub fn Runtime_ArrayBufferSetDetachKey(
    isolate: &Isolate,
    args: Arguments,
  ) -> Result<i32, TypeError> {
    if args.length() != 2 {
      panic!("expected 2 arguments");
    }
    let argument = args.at_(&0);
    let key = args.at_(&1);

    if !IsJSArrayBuffer(argument) {
      return ThrowNewErrorReturnFailure(
        isolate,
        NewTypeError(MessageTemplate::kNotTypedArray),
      );
    }
    let mut array_buffer = args.at::<JSArrayBuffer>(0);
    array_buffer.set_detach_key(1);
    Ok(ReadOnlyRoots::new(0, 0).undefined_value)
  }

  pub fn Runtime_TypedArrayCopyElements(
    isolate: &Isolate,
    args: Arguments,
  ) -> i32 {
    let mut target = args.at::<JSTypedArray>(0);
    let source = args.at::<JSAny>(1);
    let mut length: usize = 0;
    if !TryNumberToSize(args.at_(&2), &mut length) {
      return -1;
    }
    let accessor = target.GetElementsAccessor();
    accessor.CopyElements(source, &mut target, length, 0)
  }

  pub fn Runtime_TypedArrayGetBuffer(
    _isolate: &Isolate,
    args: Arguments,
  ) -> Box<JSAny> {
    let holder = args.at::<JSTypedArray>(0);
    let buffer = holder.GetBuffer();
    Box::new(JSAny::JSArrayBuffer(JSArrayBuffer {
      backing_store: buffer.backing_store.clone(),
      is_shared: buffer.is_shared,
      detach_key: buffer.detach_key,
      byte_length: buffer.byte_length,
    }))
  }

  pub fn Runtime_GrowableSharedArrayBufferByteLength(
    isolate: &Isolate,
    args: Arguments,
  ) -> Box<Number> {
    let array_buffer = args.at::<JSArrayBuffer>(0);
    let _clear_wasm_flag = SaveAndClearThreadInWasmFlag::new(isolate);

    assert_eq!(0, array_buffer.byte_length());
    let byte_length = array_buffer.GetBackingStore().len();
    isolate.factory.NewNumberFromSize(byte_length)
  }

  fn compare_num<T: PartialOrd>(x: T, y: T) -> bool {
    if x < y {
      return true;
    } else if x > y {
      return false;
    } else {
      return false;
    }
  }

  fn less_than_float16_raw_bits(x: u16, y: u16) -> bool {
    compare_num(fp16_ieee_to_fp32_value(x), fp16_ieee_to_fp32_value(y))
  }
  pub fn Runtime_TypedArraySortFast(_isolate: &Isolate, args: Arguments) -> Box<JSAny> {
    let array = args.at::<JSTypedArray>(0);
    assert!(!array.WasDetached());
    assert!(!array.IsOutOfBounds());

    let byte_length = array.GetByteLength();
    let buffer = args.at::<JSTypedArray>(0).GetBuffer();
    let copy_data = buffer.is_shared();

    let mut array_copy: Option<Box<ByteArray>> = None;
    let mut offheap_copy: Vec<u8> = Vec::new();
    let mut data_copy_ptr: *mut u8 = std::ptr::null_mut();

    if copy_data {
      if byte_length <= ByteArray::LengthFor(KMaxRegularHeapObjectSize) {
        let mut arr = ByteArray {
          data: vec![0u8; byte_length],
        };
        data_copy_ptr = arr.data.as_mut_ptr();
        array_copy = Some(Box::new(arr));
      } else {
        offheap_copy.resize(byte_length);
        data_copy_ptr = offheap_copy.as_mut_ptr();
      }
      unsafe {
        std::ptr::copy_nonoverlapping(
          array.buffer.backing_store.as_ptr(),
          data_copy_ptr,
          byte_length,
        );
      }
    }
    let length = array.GetLength();
    assert!(length > 1);

    match array.type_() {
      ExternalArrayType::kExternalFloat64Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut f64, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut f64,
              length,
            )
          }
        };
        data.sort_by(|a, b| {
          if a.is_nan() && !b.is_nan() {
            Ordering::Greater
          } else if !a.is_nan() && b.is_nan() {
            Ordering::Less
          } else {
            a.partial_cmp(b).unwrap_or(Ordering::Equal)
          }
        });
      }
      ExternalArrayType::kExternalFloat32Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut f32, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut f32,
              length,
            )
          }
        };
        data.sort_by(|a, b| {
          if a.is_nan() && !b.is_nan() {
            Ordering::Greater
          } else if !a.is_nan() && b.is_nan() {
            Ordering::Less
          } else {
            a.partial_cmp(b).unwrap_or(Ordering::Equal)
          }
        });
      }
      ExternalArrayType::kExternalInt8Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut i8, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut i8,
              length,
            )
          }
        };
        data.sort();
      }
      ExternalArrayType::kExternalUint8Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut u8, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut u8,
              length,
            )
          }
        };
        data.sort();
      }
      ExternalArrayType::kExternalInt16Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut i16, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut i16,
              length,
            )
          }
        };
        data.sort();
      }
      ExternalArrayType::kExternalUint16Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut u16, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut u16,
              length,
            )
          }
        };
        data.sort();
      }
      ExternalArrayType::kExternalInt32Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut i32, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut i32,
              length,
            )
          }
        };
        data.sort();
      }
      ExternalArrayType::kExternalUint32Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut u32, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut u32,
              length,
            )
          }
        };
        data.sort();
      }
      ExternalArrayType::kExternalFloat16Array => {
        let mut data = if copy_data {
          unsafe { std::slice::from_raw_parts_mut(data_copy_ptr as *mut u16, length) }
        } else {
          unsafe {
            std::slice::from_raw_parts_mut(
              array.buffer.backing_store.as_mut_ptr() as *mut u16,
              length,
            )
          }
        };
        data.sort_by(|a, b| {
          if less_than_float16_raw_bits(*a, *b) {
            Ordering::Less
          } else {
            Ordering::Greater
          }
        });
      }
      _ => {
        println!("not implemented");
      }
    }

    if copy_data {
      unsafe {
        std::ptr::copy_nonoverlapping(
          data_copy_ptr,
          array.buffer.backing_store.as_mut_ptr(),
          byte_length,
        );
      }
    }
    Box::new(JSAny::JSTypedArray(JSTypedArray {
      buffer: array.buffer,
      byte_offset: array.byte_offset,
      length: array.length,
      element_size: array.element_size,
      external_array_type: array.external_array_type,
    }))
  }

  pub fn Runtime_TypedArraySet(
    isolate: &Isolate,
    args: Arguments,
  ) -> i32 {
    let mut target = args.at::<JSTypedArray>(0);
    let source = args.at::<JSAny>(1);
    let mut length: usize = 0;
    if !TryNumberToSize(args.at_(&2), &mut length) {
      return -1;
    }
    let mut offset: usize = 0;
    if !TryNumberToSize(args.at_(&3), &mut offset) {
      return -1;
    }
    let accessor = target.GetElementsAccessor();
    accessor.CopyElements(source, &mut target, length, offset)
  }

  pub fn Runtime_ArrayBufferMaxByteLength(isolate: &Isolate, _args: Arguments) -> Box<Number> {
    let heap_max = isolate.array_buffer_allocator.lock().unwrap().MaxAllocationSize();
    isolate.factory.NewNumber(heap_max as f64)
  }
}
