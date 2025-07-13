// Converted from V8 C++ source files:
// Header: allocation-result.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub enum AllocationOrigin {
  kGeneratedCode = 0,
  kRuntime = 1,
  kGC = 2,
  kFirstAllocationOrigin = Self::kGeneratedCode,
  kLastAllocationOrigin = Self::kGC,
  kNumberOfAllocationOrigins = Self::kLastAllocationOrigin as isize + 1 as isize,
}

#[derive(Copy, Clone)]
pub struct AllocationResult {
  object_: Option<Tagged<HeapObject>>,
}

impl AllocationResult {
  pub fn Failure() -> Self {
    AllocationResult { object_: None }
  }

  pub fn FromObject(heap_object: Tagged<HeapObject>) -> Self {
    AllocationResult {
      object_: Some(heap_object),
    }
  }

  pub fn new() -> Self {
    AllocationResult { object_: None }
  }

  pub fn IsFailure(&self) -> bool {
    self.object_.is_none()
  }

  pub fn To<T>(&self) -> Result<Tagged<T>, String> {
    if self.IsFailure() {
      return Err("Allocation failed".to_string());
    }
    match self.object_ {
        Some(object_) => {
            //This cast may not be safe
            let casted_object = unsafe { std::mem::transmute::<Tagged<HeapObject>, Tagged<T>>(object_) };
            Ok(casted_object)
        }
        None => Err("Allocation failed".to_string()),
    }
  }

  pub fn ToObjectChecked(&self) -> Tagged<HeapObject> {
    if self.IsFailure() {
        panic!("Check failed: AllocationResult is failure");
    }

    match self.object_ {
        Some(object_) => {
            unsafe { std::mem::transmute::<Tagged<HeapObject>, Tagged<HeapObject>>(object_) }
        }
        None => panic!("Object is none despite check"),
    }
  }

  pub fn ToObject(&self) -> Tagged<HeapObject> {
    if self.IsFailure() {
        panic!("Check failed: AllocationResult is failure");
    }

    match self.object_ {
        Some(object_) => {
            unsafe { std::mem::transmute::<Tagged<HeapObject>, Tagged<HeapObject>>(object_) }
        }
        None => panic!("Object is none despite check"),
    }
  }

  pub fn ToAddress(&self) -> Address {
    if self.IsFailure() {
      panic!("Check failed: AllocationResult is failure");
    }
    match &self.object_ {
      Some(heap_object) => {
          let object: Tagged<HeapObject> = unsafe { std::mem::transmute_copy(heap_object) };
          object.address()
      },
      None => panic!("Object is none despite check")
    }
  }
}

impl From<AllocationResult> for bool {
  fn from(result: AllocationResult) -> Self {
    !result.IsFailure()
  }
}

impl AllocationResult {
    fn new_with_object(heap_object: Tagged<HeapObject>) -> Self {
        AllocationResult{ object_: Some(heap_object) }
    }
}
