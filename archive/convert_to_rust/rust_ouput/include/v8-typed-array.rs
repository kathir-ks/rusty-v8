// Converted from V8 C++ source files:
// Header: v8-typed-array.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
  use std::mem::size_of;
  use std::sync::{Arc, Mutex};

  pub struct V8_EXPORT {}

  pub struct ArrayBuffer {}
  impl ArrayBuffer {
    pub const kMaxByteLength: usize = 2147483647;
  }
  pub struct SharedArrayBuffer {}
  pub struct ArrayBufferView {}
  pub struct Value {}
  pub struct Object {}

  pub struct Local<'a, T> {
    pub ptr: Arc<Mutex<T>>,
    _marker: std::marker::PhantomData<&'a T>,
  }

  impl<'a, T> Local<'a, T> {
    pub fn new(value: T) -> Self {
      Local {
        ptr: Arc::new(Mutex::new(value)),
        _marker: std::marker::PhantomData,
      }
    }

    pub fn get(&self) -> std::sync::MutexGuard<'_, T> {
      self.ptr.lock().unwrap()
    }
  }
  pub struct TypedArray {}
  impl TypedArray {
    pub const kMaxByteLength: usize = ArrayBuffer::kMaxByteLength;

    pub fn length(&self) -> usize {
      0
    }

    pub fn cast<'a>(value: *mut Value) -> *mut TypedArray {
      value as *mut TypedArray
    }
  }

  pub struct Uint8Array {}

  impl Uint8Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<u8>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Uint8Array> {
      Local::new(Uint8Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Uint8Array> {
      Local::new(Uint8Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Uint8Array {
      value as *mut Uint8Array
    }
  }

  pub struct Uint8ClampedArray {}

  impl Uint8ClampedArray {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<u8>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Uint8ClampedArray> {
      Local::new(Uint8ClampedArray {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Uint8ClampedArray> {
      Local::new(Uint8ClampedArray {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Uint8ClampedArray {
      value as *mut Uint8ClampedArray
    }
  }

  pub struct Int8Array {}

  impl Int8Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<i8>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Int8Array> {
      Local::new(Int8Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Int8Array> {
      Local::new(Int8Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Int8Array {
      value as *mut Int8Array
    }
  }

  pub struct Uint16Array {}

  impl Uint16Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<u16>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Uint16Array> {
      Local::new(Uint16Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Uint16Array> {
      Local::new(Uint16Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Uint16Array {
      value as *mut Uint16Array
    }
  }

  pub struct Int16Array {}

  impl Int16Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<i16>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Int16Array> {
      Local::new(Int16Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Int16Array> {
      Local::new(Int16Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Int16Array {
      value as *mut Int16Array
    }
  }

  pub struct Uint32Array {}

  impl Uint32Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<u32>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Uint32Array> {
      Local::new(Uint32Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Uint32Array> {
      Local::new(Uint32Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Uint32Array {
      value as *mut Uint32Array
    }
  }

  pub struct Int32Array {}

  impl Int32Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<i32>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Int32Array> {
      Local::new(Int32Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Int32Array> {
      Local::new(Int32Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Int32Array {
      value as *mut Int32Array
    }
  }

  pub struct Float16Array {}

  impl Float16Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<u16>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Float16Array> {
      Local::new(Float16Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Float16Array> {
      Local::new(Float16Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Float16Array {
      value as *mut Float16Array
    }
  }

  pub struct Float32Array {}

  impl Float32Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<f32>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Float32Array> {
      Local::new(Float32Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Float32Array> {
      Local::new(Float32Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Float32Array {
      value as *mut Float32Array
    }
  }

  pub struct Float64Array {}

  impl Float64Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<f64>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Float64Array> {
      Local::new(Float64Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, Float64Array> {
      Local::new(Float64Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut Float64Array {
      value as *mut Float64Array
    }
  }

  pub struct BigInt64Array {}

  impl BigInt64Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<i64>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, BigInt64Array> {
      Local::new(BigInt64Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, BigInt64Array> {
      Local::new(BigInt64Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut BigInt64Array {
      value as *mut BigInt64Array
    }
  }

  pub struct BigUint64Array {}

  impl BigUint64Array {
    pub const kMaxLength: usize = TypedArray::kMaxByteLength / size_of::<u64>();

    pub fn new<'a>(
      array_buffer: Local<'a, ArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, BigUint64Array> {
      Local::new(BigUint64Array {})
    }

    pub fn new_shared<'a>(
      shared_array_buffer: Local<'a, SharedArrayBuffer>,
      byte_offset: usize,
      length: usize,
    ) -> Local<'a, BigUint64Array> {
      Local::new(BigUint64Array {})
    }

    pub fn cast<'a>(value: *mut Value) -> *mut BigUint64Array {
      value as *mut BigUint64Array
    }
  }
}
